//! PList-based Safari and Photos artifacts.
//!
//! These are independent plists rather than SQLite databases. Only known
//! top-level keys and arrays are read; archived represented objects in Photos
//! recent searches are not decoded (their titles are still reported).

use std::io::Cursor;
use std::time::UNIX_EPOCH;

use lovelycarve_plugin::{Json, Result, Run, Timestamp, Value};
use mobile_core::{ArtifactFile, ArtifactRecord};
use plist::Value as PlistValue;

const MAX_BYTES: u64 = 32 * 1024 * 1024;

struct Row {
    key: String,
    title: String,
    time: Option<Timestamp>,
    data: Vec<(String, Value)>,
}

pub fn safari_downloads(run: &Run) -> Result<()> {
    evidence(
        run,
        "ios_safari_downloads",
        &["**/Safari/Downloads/Downloads.plist"],
        downloads,
    )
}

pub fn safari_recent_searches(run: &Run) -> Result<()> {
    evidence(
        run,
        "ios_safari_recent_searches",
        &["**/Library/Preferences/com.apple.mobilesafari.plist"],
        recent_searches,
    )
}

pub fn photos_recent_searches(run: &Run) -> Result<()> {
    evidence(
        run,
        "ios_photos_recent_searches",
        &["**/PhotoData/Caches/search/RecentSearches.plist"],
        photos_searches,
    )
}

pub fn bluetooth_registry(run: &Run) -> Result<()> {
    evidence(
        run,
        "ios_bluetooth_reg",
        &["**/com.apple.MobileBluetooth.devices.plist"],
        bluetooth_devices,
    )
}

fn evidence(
    run: &Run,
    artifact: &str,
    patterns: &[&str],
    read: fn(&PlistValue) -> Vec<Row>,
) -> Result<()> {
    let view = run.require_view()?;
    let mut expanded = Vec::new();
    for mount in view.mounts() {
        let prefix = mount.path.trim_matches('/');
        for pattern in patterns {
            expanded.push(if prefix.is_empty() {
                (*pattern).to_owned()
            } else {
                format!("{prefix}/{pattern}")
            });
        }
    }
    expanded.sort();
    expanded.dedup();
    let refs: Vec<&str> = expanded.iter().map(String::as_str).collect();
    let paths = match view.find(&refs) {
        Ok(paths) => paths,
        Err(error) => {
            run.warn(&error.code, error.message)?;
            return Ok(());
        }
    };
    let mut files = 0usize;
    let mut rows = 0usize;
    for path in paths {
        run.check_cancelled()?;
        let Some(file) = view.open(&path)? else {
            continue;
        };
        let source = file
            .view_path()
            .map(str::to_owned)
            .unwrap_or_else(|| file.name().to_owned());
        let bytes = match file.read_all(MAX_BYTES) {
            Ok(bytes) => bytes,
            Err(error) => {
                run.warn_file(&file, &error.code, error.message)?;
                continue;
            }
        };
        let plist = match PlistValue::from_reader(Cursor::new(&bytes)) {
            Ok(plist) => plist,
            Err(error) => {
                run.warn_file(
                    &file,
                    "PLIST_PARSE",
                    format!("{source}: 不是可解析的 plist：{error}"),
                )?;
                continue;
            }
        };
        let entries = read(&plist);
        for (index, entry) in entries.iter().enumerate() {
            run.check_cancelled()?;
            let data = entry
                .data
                .iter()
                .map(|(name, value)| (Value::from(name.as_str()), value.clone()))
                .collect();
            run.record(&ArtifactRecord {
                platform: "ios".to_owned(),
                artifact: artifact.to_owned(),
                source: source.clone(),
                record_key: format!("{}:{}", entry.key, index + 1),
                time: entry.time.clone(),
                title: entry.title.clone(),
                data: Json(Value::Map(data)),
            })
            .from(&file)
            .note(format!("{artifact} #{}", index + 1))
            .emit()?;
        }
        rows += entries.len();
        files += 1;
        run.record(&ArtifactFile {
            platform: "ios".to_owned(),
            artifact: artifact.to_owned(),
            source,
            status: if entries.is_empty() { "empty" } else { "ok" }.to_owned(),
            rows: entries.len() as i64,
            detail: "plist".to_owned(),
        })
        .from(&file)
        .emit()?;
    }
    if files == 0 {
        run.warn("PLIST_ARTIFACT_NOT_FOUND", format!("未找到 {artifact} 的 plist"))?;
    }
    run.set_summary(format!("{artifact}：{files} 个文件，{rows} 条记录"));
    Ok(())
}

fn downloads(plist: &PlistValue) -> Vec<Row> {
    let Some(history) = plist
        .as_dictionary()
        .and_then(|dict| dict.get("DownloadHistory"))
        .and_then(PlistValue::as_array)
    else {
        return Vec::new();
    };
    history
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            let dict = entry.as_dictionary()?;
            let text = |key: &str| {
                dict.get(key)
                    .and_then(PlistValue::as_string)
                    .unwrap_or("")
                    .to_owned()
            };
            let value = |key: &str| {
                dict.get(key)
                    .map(to_json)
                    .unwrap_or(Value::Nil)
            };
            let url = text("DownloadEntryURL");
            let path = text("DownloadEntryPath");
            let identifier = text("DownloadEntryIdentifier");
            let title = if !url.is_empty() { url.clone() } else { path.clone() };
            Some(Row {
                key: if identifier.is_empty() {
                    format!("download-{}", index + 1)
                } else {
                    identifier.clone()
                },
                title,
                time: dict
                    .get("DownloadEntryDateAddedKey")
                    .and_then(date_time),
                data: vec![
                    ("url".to_owned(), Value::from(url.as_str())),
                    ("path".to_owned(), Value::from(path.as_str())),
                    (
                        "bytes_so_far".to_owned(),
                        value("DownloadEntryProgressBytesSoFar"),
                    ),
                    (
                        "total_bytes".to_owned(),
                        value("DownloadEntryProgressTotalToLoad"),
                    ),
                    (
                        "identifier".to_owned(),
                        Value::from(identifier.as_str()),
                    ),
                    (
                        "sandbox_identifier".to_owned(),
                        value("DownloadEntrySandboxIdentifier"),
                    ),
                    (
                        "remove_when_done".to_owned(),
                        value("DownloadEntryRemoveWhenDoneKey"),
                    ),
                ],
            })
        })
        .collect()
}

fn recent_searches(plist: &PlistValue) -> Vec<Row> {
    let Some(items) = plist
        .as_dictionary()
        .and_then(|dict| dict.get("RecentWebSearches"))
        .and_then(PlistValue::as_array)
    else {
        return Vec::new();
    };
    items
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            let dict = entry.as_dictionary()?;
            let search = dict
                .get("SearchString")
                .and_then(PlistValue::as_string)
                .unwrap_or("")
                .to_owned();
            Some(Row {
                key: format!("search-{}", index + 1),
                title: search.clone(),
                time: dict.get("Date").and_then(date_time),
                data: vec![("search".to_owned(), Value::from(search.as_str()))],
            })
        })
        .collect()
}

fn bluetooth_devices(plist: &PlistValue) -> Vec<Row> {
    let Some(dict) = plist.as_dictionary() else {
        return Vec::new();
    };
    dict.iter()
        .filter_map(|(mac, value)| {
            let entry = value.as_dictionary()?;
            let text = |key: &str| {
                entry
                    .get(key)
                    .and_then(PlistValue::as_string)
                    .unwrap_or("")
                    .to_owned()
            };
            let name = text("Name");
            let default_name = text("DefaultName");
            let title = if !name.is_empty() {
                name.clone()
            } else if !default_name.is_empty() {
                default_name.clone()
            } else {
                mac.clone()
            };
            let last_seen = entry
                .get("LastSeenTime")
                .and_then(plist_integer)
                .map(Timestamp::from_unix);
            Some(Row {
                key: mac.clone(),
                title,
                time: last_seen,
                data: vec![
                    ("mac".to_owned(), Value::from(mac.as_str())),
                    ("name".to_owned(), Value::from(name.as_str())),
                    (
                        "default_name".to_owned(),
                        Value::from(default_name.as_str()),
                    ),
                    (
                        "user_name_key".to_owned(),
                        Value::from(text("UserNameKey").as_str()),
                    ),
                    (
                        "device_product".to_owned(),
                        Value::from(text("DeviceIdProduct").as_str()),
                    ),
                ],
            })
        })
        .collect()
}

fn plist_integer(value: &PlistValue) -> Option<i64> {
    match value {
        PlistValue::Integer(integer) => integer
            .as_signed()
            .or_else(|| integer.as_unsigned().map(|value| value as i64)),
        PlistValue::Real(real) => Some(real.round() as i64),
        _ => None,
    }
}

fn photos_searches(plist: &PlistValue) -> Vec<Row> {    let Some(items) = plist.as_array() else {
        return Vec::new();
    };
    items
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            let dict = entry.as_dictionary()?;
            let search = dict
                .get("SearchText")
                .and_then(PlistValue::as_string)
                .unwrap_or("")
                .to_owned();
            let objects = dict
                .get("RepresentedObjects")
                .and_then(PlistValue::as_array)
                .cloned()
                .unwrap_or_default();
            let titles: Vec<String> = objects
                .iter()
                .filter_map(|object| object.as_dictionary())
                .filter_map(|object| object.get("title"))
                .map(|title| match title {
                    PlistValue::String(text) => text.clone(),
                    other => format!("{other:?}"),
                })
                .collect();
            Some(Row {
                key: format!("photo-search-{}", index + 1),
                title: search.clone(),
                time: None,
                data: vec![
                    ("search".to_owned(), Value::from(search.as_str())),
                    (
                        "represented_object_count".to_owned(),
                        Value::from(objects.len() as u64),
                    ),
                    (
                        "represented_object_titles".to_owned(),
                        Value::from(titles.join("; ").as_str()),
                    ),
                ],
            })
        })
        .collect()
}

fn date_time(value: &PlistValue) -> Option<Timestamp> {
    let PlistValue::Date(date) = value else {
        return None;
    };
    let system: std::time::SystemTime = (*date).into();
    system
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| Timestamp::from_unix(duration.as_secs() as i64))
}

fn to_json(value: &PlistValue) -> Value {
    match value {
        PlistValue::String(text) => Value::from(text.as_str()),
        PlistValue::Integer(integer) => integer
            .as_signed()
            .map(Value::from)
            .or_else(|| integer.as_unsigned().map(Value::from))
            .unwrap_or(Value::Nil),
        PlistValue::Real(real) => Value::F64(*real),
        PlistValue::Boolean(boolean) => Value::Boolean(*boolean),
        PlistValue::Date(_) => date_time(value)
            .map(|time| Value::from(time.as_str()))
            .unwrap_or(Value::Nil),
        PlistValue::Data(bytes) => {
            Value::from(format!("<data {} bytes>", bytes.len()).as_str())
        }
        PlistValue::Array(items) => Value::Array(items.iter().map(to_json).collect()),
        PlistValue::Dictionary(dict) => Value::Map(
            dict.iter()
                .map(|(key, value)| (Value::from(key.as_str()), to_json(value)))
                .collect(),
        ),
        _ => Value::Nil,
    }
}
