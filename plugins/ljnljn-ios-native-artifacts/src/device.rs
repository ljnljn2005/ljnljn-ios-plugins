//! Device and backup metadata from property lists.
//!
//! `SystemVersion.plist` is read from a full filesystem extraction and
//! `Manifest.plist` from an iTunes/Finder backup. Binary and XML plists are
//! both accepted; the original structure is kept in the record JSON.

use std::io::Cursor;
use std::time::UNIX_EPOCH;

use lovelycarve_plugin::{FileRef, Json, Result, Run, Timestamp, Value};
use mobile_core::{ArtifactFile, ArtifactRecord};
use plist::Value as PlistValue;

const MAX_PLIST_BYTES: u64 = 16 * 1024 * 1024;
const PATTERNS: &[&str] = &["**/Manifest.plist", "**/SystemVersion.plist"];

pub fn analyze(run: &Run, file: &FileRef) -> Result<bool> {
    let bytes = match file.read_all(MAX_PLIST_BYTES) {
        Ok(bytes) => bytes,
        Err(error) => {
            run.warn_file(file, &error.code, error.message)?;
            return Ok(false);
        }
    };
    let plist = match PlistValue::from_reader(Cursor::new(&bytes)) {
        Ok(plist) => plist,
        Err(error) => {
            run.warn_file(
                file,
                "PLIST_PARSE",
                format!("{}: 不是可解析的 plist：{error}", source(file)),
            )?;
            return Ok(false);
        }
    };
    let source = source(file);
    let title = find_string(
        &plist,
        &["Device Name", "ProductName", "ProductVersion", "Product Version"],
    )
    .unwrap_or_else(|| file.name().to_owned());
    let time = find_date(&plist, &["Last Backup Date", "Date"]);
    run.record(&ArtifactRecord {
        platform: "ios".to_owned(),
        artifact: "ios_device_info".to_owned(),
        source: source.clone(),
        record_key: source.clone(),
        time,
        title,
        data: Json(to_json(&plist)),
    })
    .from(file)
    .note(file.name())
    .emit()?;
    run.record(&ArtifactFile {
        platform: "ios".to_owned(),
        artifact: "ios_device_info".to_owned(),
        source,
        status: "ok".to_owned(),
        rows: 1,
        detail: "plist".to_owned(),
    })
    .from(file)
    .emit()?;
    Ok(true)
}

pub fn run_evidence(run: &Run) -> Result<()> {
    let view = run.require_view()?;
    let mut patterns = Vec::new();
    for mount in view.mounts() {
        let prefix = mount.path.trim_matches('/');
        for pattern in PATTERNS {
            patterns.push(if prefix.is_empty() {
                (*pattern).to_owned()
            } else {
                format!("{prefix}/{pattern}")
            });
        }
    }
    patterns.sort();
    patterns.dedup();
    let refs: Vec<&str> = patterns.iter().map(String::as_str).collect();
    let paths = match view.find(&refs) {
        Ok(paths) => paths,
        Err(error) => {
            run.warn(&error.code, error.message)?;
            return Ok(());
        }
    };
    let mut count = 0usize;
    for path in paths {
        run.check_cancelled()?;
        let Some(file) = view.open(&path)? else {
            continue;
        };
        if analyze(run, &file)? {
            count += 1;
        }
    }
    if count == 0 {
        run.warn(
            "DEVICE_INFO_NOT_FOUND",
            "未找到 Manifest.plist 或 SystemVersion.plist",
        )?;
    }
    run.set_summary(format!("iOS 设备/备份信息：{count} 个 plist"));
    Ok(())
}

pub fn run_selected(run: &Run) -> Result<()> {
    let mut count = 0usize;
    for file in run.inputs() {
        run.check_cancelled()?;
        if analyze(run, file)? {
            count += 1;
        }
    }
    if count == 0 {
        run.warn("PLIST_PARSE", "所选文件不是可解析的 plist")?;
    }
    run.set_summary(format!("所选 plist：{count} 个"));
    Ok(())
}

fn source(file: &FileRef) -> String {
    file.view_path()
        .map(str::to_owned)
        .unwrap_or_else(|| file.name().to_owned())
}

fn find_string(value: &PlistValue, keys: &[&str]) -> Option<String> {
    if let PlistValue::Dictionary(dict) = value {
        for key in keys {
            if let Some(PlistValue::String(text)) = dict.get(*key) {
                if !text.trim().is_empty() {
                    return Some(text.clone());
                }
            }
        }
        for child in dict.values() {
            if let Some(found) = find_string(child, keys) {
                return Some(found);
            }
        }
    } else if let PlistValue::Array(items) = value {
        for child in items {
            if let Some(found) = find_string(child, keys) {
                return Some(found);
            }
        }
    }
    None
}

fn find_date(value: &PlistValue, keys: &[&str]) -> Option<Timestamp> {
    if let PlistValue::Dictionary(dict) = value {
        for key in keys {
            if let Some(PlistValue::Date(date)) = dict.get(*key) {
                let system: std::time::SystemTime = (*date).into();
                if let Ok(duration) = system.duration_since(UNIX_EPOCH) {
                    return Some(Timestamp::from_unix(duration.as_secs() as i64));
                }
            }
        }
        for child in dict.values() {
            if let Some(found) = find_date(child, keys) {
                return Some(found);
            }
        }
    } else if let PlistValue::Array(items) = value {
        for child in items {
            if let Some(found) = find_date(child, keys) {
                return Some(found);
            }
        }
    }
    None
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
        PlistValue::Date(date) => {
            let system: std::time::SystemTime = (*date).into();
            match system.duration_since(UNIX_EPOCH) {
                Ok(duration) => {
                    let text = Timestamp::from_unix(duration.as_secs() as i64);
                    Value::from(text.as_str())
                }
                Err(_) => Value::Nil,
            }
        }
        PlistValue::Data(bytes) => Value::from(format!("<data {} bytes>", bytes.len()).as_str()),
        PlistValue::Array(items) => Value::Array(items.iter().map(to_json).collect()),
        PlistValue::Dictionary(dict) => Value::Map(
            dict.iter()
                .map(|(key, value)| (Value::from(key.as_str()), to_json(value)))
                .collect(),
        ),
        _ => Value::Nil,
    }
}
