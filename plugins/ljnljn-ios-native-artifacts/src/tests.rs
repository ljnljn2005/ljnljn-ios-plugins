use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use lovelycarve_plugin::testing::{Fixture, Harness, TestRecord, TestRun};
use lovelycarve_plugin::Value;
use rusqlite::Connection;

const COCOA_2024: i64 = 725_760_000;

struct Root {
    path: PathBuf,
}

impl Root {
    fn new(label: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "ljnljn-ios-native-{label}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn write(&self, relative: &str, content: &str) -> PathBuf {
        let path = self.path.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, content).unwrap();
        path
    }

    fn database(&self, relative: &str, sql: &str) -> PathBuf {
        let path = self.path.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let connection = Connection::open(&path).unwrap();
        connection.execute_batch(sql).unwrap();
        path
    }

    fn evidence(&self, analyzer: &str) -> TestRun {
        Harness::new(super::plugin)
            .run_with(
                analyzer,
                Fixture::evidence(&[("part:1", &self.path)]),
                BTreeMap::new(),
                BTreeMap::new(),
            )
            .expect("run")
    }

    fn selected(&self, analyzer: &str, path: &Path) -> TestRun {
        Harness::new(super::plugin)
            .run_with(
                analyzer,
                Fixture::files(&[path]),
                BTreeMap::new(),
                BTreeMap::new(),
            )
            .expect("run")
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn data<'a>(record: &'a TestRecord, key: &str) -> Option<&'a Value> {
    let Value::Map(map) = record.get("data")? else {
        return None;
    };
    map.iter()
        .find(|(name, _)| name.as_str() == Some(key))
        .map(|(_, value)| value)
}

fn message_fixture(root: &Root) -> PathBuf {
    root.database(
        "data/sms.db",
        &format!(
            "CREATE TABLE message(ROWID INTEGER PRIMARY KEY, guid TEXT, text TEXT, \
             attributedBody BLOB, handle_id INTEGER, service TEXT, date INTEGER, \
             date_read INTEGER, date_delivered INTEGER, is_from_me INTEGER, \
             cache_has_attachments INTEGER);\
             CREATE TABLE handle(ROWID INTEGER PRIMARY KEY, id TEXT);\
             CREATE TABLE chat(ROWID INTEGER PRIMARY KEY, chat_identifier TEXT, display_name TEXT);\
             CREATE TABLE chat_message_join(message_id INTEGER, chat_id INTEGER);\
             INSERT INTO handle VALUES(1, '+8613800000000');\
             INSERT INTO chat VALUES(1, '+8613800000000', 'Alice');\
             INSERT INTO message VALUES(1, 'guid-1', 'hello', NULL, 1, 'iMessage', {COCOA_2024}, 0, 0, 1, 0);\
             INSERT INTO chat_message_join VALUES(1, 1);"
        ),
    )
}

#[test]
fn messages_read_text_handle_and_cocoa_time() {
    let root = Root::new("messages");
    message_fixture(&root);
    let run = root.evidence("ios_messages");

    assert_eq!(run.count("mobile.artifact"), 1);
    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("artifact"), Some("ios_messages"));
    assert_eq!(record.text("time"), Some("2024-01-01T00:00:00Z"));
    assert_eq!(
        data(record, "text").and_then(Value::as_str),
        Some("hello")
    );
    assert_eq!(
        data(record, "handle").and_then(Value::as_str),
        Some("+8613800000000")
    );
    let file = run.of_type("mobile.artifact.file")[0];
    assert_eq!(file.text("status"), Some("ok"));
    assert_eq!(file.int("rows"), Some(1));
}

#[test]
fn calls_read_coredata_direction_and_duration() {
    let root = Root::new("calls");
    root.database(
        "data/CallHistory.storedata",
        &format!(
            "CREATE TABLE ZCALLRECORD(Z_PK INTEGER PRIMARY KEY, ZADDRESS TEXT, ZDATE INTEGER, \
             ZDURATION INTEGER, ZORIGINATED INTEGER, ZANSWERED INTEGER, ZCALLTYPE INTEGER, \
             ZSERVICE_PROVIDER TEXT);\
             INSERT INTO ZCALLRECORD VALUES(1, '+8613900000000', {COCOA_2024}, 65, 1, 1, 1, 'Telephony');"
        ),
    );
    let run = root.evidence("ios_calls");

    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("artifact"), Some("ios_calls"));
    assert_eq!(record.text("time"), Some("2024-01-01T00:00:00Z"));
    assert_eq!(
        data(record, "duration").and_then(Value::as_i64),
        Some(65)
    );
    assert_eq!(
        data(record, "address").and_then(Value::as_str),
        Some("+8613900000000")
    );
}

#[test]
fn contacts_join_phone_and_email_multivalues() {
    let root = Root::new("contacts");
    root.database(
        "data/AddressBook.sqlitedb",
        "CREATE TABLE ABPerson(ROWID INTEGER PRIMARY KEY, First TEXT, Last TEXT, \
         Organization TEXT, JobTitle TEXT);\
         CREATE TABLE ABMultiValue(record_id INTEGER, property INTEGER, value TEXT);\
         INSERT INTO ABPerson VALUES(1, 'Alice', 'Wang', 'ACME', 'Engineer');\
         INSERT INTO ABMultiValue VALUES(1, 3, '+8613800000000');\
         INSERT INTO ABMultiValue VALUES(1, 4, 'alice@example.com');",
    );
    let run = root.evidence("ios_contacts");

    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(
        data(record, "phones").and_then(Value::as_str),
        Some("+8613800000000")
    );
    assert_eq!(
        data(record, "emails").and_then(Value::as_str),
        Some("alice@example.com")
    );
    assert_eq!(record.text("title"), Some("Alice · Wang · ACME"));
}

#[test]
fn safari_history_time_is_cocoa_or_unix() {
    let root = Root::new("safari");
    root.database(
        "data/Safari/History.db",
        &format!(
            "CREATE TABLE history_items(id INTEGER PRIMARY KEY, url TEXT, title TEXT, visit_count INTEGER);\
             CREATE TABLE history_visits(id INTEGER PRIMARY KEY, history_item INTEGER, visit_time INTEGER, \
             title TEXT, load_successful INTEGER);\
             INSERT INTO history_items VALUES(1, 'https://example.com', 'Example', 3);\
             INSERT INTO history_visits VALUES(1, 1, {COCOA_2024}, 'Example', 1);"
        ),
    );
    let run = root.evidence("ios_safari_history");

    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("time"), Some("2024-01-01T00:00:00Z"));
    assert_eq!(
        data(record, "url").and_then(Value::as_str),
        Some("https://example.com")
    );
    assert_eq!(
        data(record, "visit_count").and_then(Value::as_i64),
        Some(3)
    );
}

#[test]
fn whatsapp_messages_join_chat_name() {
    let root = Root::new("whatsapp");
    root.database(
        "data/ChatStorage.sqlite",
        &format!(
            "CREATE TABLE ZWACHATSESSION(Z_PK INTEGER PRIMARY KEY, ZPARTNERNAME TEXT, ZCONTACTJID TEXT);\
             CREATE TABLE ZWAMESSAGE(Z_PK INTEGER PRIMARY KEY, ZMESSAGEDATE INTEGER, ZISFROMME INTEGER, \
             ZFROMJID TEXT, ZTOJID TEXT, ZTEXT TEXT, ZMEDIAITEM INTEGER, ZMESSAGETYPE INTEGER, ZCHATSESSION INTEGER);\
             INSERT INTO ZWACHATSESSION VALUES(1, 'Alice', '8613800000000@s.whatsapp.net');\
             INSERT INTO ZWAMESSAGE VALUES(1, {COCOA_2024}, 0, '8613800000000@s.whatsapp.net', 'me', 'hi', NULL, 0, 1);"
        ),
    );
    let run = root.evidence("ios_whatsapp");

    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("time"), Some("2024-01-01T00:00:00Z"));
    assert_eq!(
        data(record, "chat_name").and_then(Value::as_str),
        Some("Alice")
    );
    assert_eq!(data(record, "text").and_then(Value::as_str), Some("hi"));
}

#[test]
fn photos_metadata_includes_filename_and_coordinates() {
    let root = Root::new("photos");
    root.database(
        "data/PhotoData/Photos.sqlite",
        &format!(
            "CREATE TABLE ZASSET(Z_PK INTEGER PRIMARY KEY, ZDATECREATED INTEGER, ZADDEDDATE INTEGER, \
             ZMODIFICATIONDATE INTEGER, ZFILENAME TEXT, ZDIRECTORY TEXT, ZUNIFORMTYPEIDENTIFIER TEXT, \
             ZWIDTH INTEGER, ZHEIGHT INTEGER, ZLATITUDE REAL, ZLONGITUDE REAL);\
             CREATE TABLE ZADDITIONALASSETATTRIBUTES(ZASSET INTEGER, ZORIGINALFILENAME TEXT);\
             INSERT INTO ZASSET VALUES(1, {COCOA_2024}, {COCOA_2024}, {COCOA_2024}, 'IMG_0001.JPG', \
             'DCIM', 'public.jpeg', 4032, 3024, 31.2304, 121.4737);\
             INSERT INTO ZADDITIONALASSETATTRIBUTES VALUES(1, 'IMG_0001.JPG');"
        ),
    );
    let run = root.evidence("ios_photos");

    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(
        data(record, "filename").and_then(Value::as_str),
        Some("IMG_0001.JPG")
    );
    assert_eq!(
        data(record, "latitude").and_then(Value::as_f64),
        Some(31.2304)
    );
}

#[test]
fn calendar_event_reads_summary_and_calendar() {
    let root = Root::new("calendar");
    root.database(
        "data/Calendar.sqlitedb",
        &format!(
            "CREATE TABLE Calendar(ROWID INTEGER PRIMARY KEY, title TEXT);\
             CREATE TABLE CalendarItem(ROWID INTEGER PRIMARY KEY, summary TEXT, start_date INTEGER, \
             end_date INTEGER, all_day INTEGER, location TEXT, description TEXT, creation_date INTEGER, \
             last_modified INTEGER, calendar_id INTEGER);\
             INSERT INTO Calendar VALUES(1, 'Work');\
             INSERT INTO CalendarItem VALUES(1, 'Meeting', {COCOA_2024}, {COCOA_2024} + 3600, 0, 'Room 1', '', {COCOA_2024}, {COCOA_2024}, 1);"
        ),
    );
    let run = root.evidence("ios_calendar");

    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("time"), Some("2024-01-01T00:00:00Z"));
    assert_eq!(
        data(record, "summary").and_then(Value::as_str),
        Some("Meeting")
    );
    assert_eq!(
        data(record, "calendar").and_then(Value::as_str),
        Some("Work")
    );
}

#[test]
fn device_info_reads_system_version_plist() {
    let root = Root::new("device");
    root.write(
        "System/Library/CoreServices/SystemVersion.plist",
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>ProductName</key><string>iPhone OS</string>
<key>ProductVersion</key><string>17.1</string>
<key>Device Name</key><string>Test iPhone</string>
</dict></plist>
"#,
    );
    let run = root.evidence("ios_device_info");

    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("artifact"), Some("ios_device_info"));
    assert_eq!(record.text("title"), Some("Test iPhone"));
    assert_eq!(
        data(record, "ProductVersion").and_then(Value::as_str),
        Some("17.1")
    );
}

#[test]
fn notes_read_title_folder_and_modified_time() {
    let root = Root::new("notes");
    root.database(
        "data/NoteStore.sqlite",
        &format!(
            "CREATE TABLE ZICCLOUDSYNCINGOBJECT(Z_PK INTEGER PRIMARY KEY, ZTITLE1 TEXT, \
             ZSNIPPET TEXT, ZFOLDER INTEGER, ZMODIFICATIONDATE1 INTEGER, \
             ZISPASSWORDPROTECTED INTEGER, ZMARKEDFORDELETION INTEGER, ZISPINNED INTEGER, \
             ZFILESIZE INTEGER, ZTYPEUTI TEXT, ZNOTE INTEGER);\
             CREATE TABLE ZICNOTEDATA(ZNOTE INTEGER, ZDATA BLOB);\
             INSERT INTO ZICCLOUDSYNCINGOBJECT VALUES(1, 'Shopping', 'milk', 0, {COCOA_2024}, 0, 0, 0, 0, NULL, 1);\
             INSERT INTO ZICNOTEDATA VALUES(1, X'00');"
        ),
    );
    let run = root.evidence("ios_notes");
    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("artifact"), Some("ios_notes"));
    assert_eq!(record.text("time"), Some("2024-01-01T00:00:00Z"));
    assert_eq!(
        data(record, "title").and_then(Value::as_str),
        Some("Shopping")
    );
}

#[test]
fn safari_bookmarks_read_url_and_title() {
    let root = Root::new("safari-bookmarks");
    root.database(
        "data/Safari/Bookmarks.db",
        "CREATE TABLE bookmarks(title TEXT, url TEXT, hidden INTEGER);\
         INSERT INTO bookmarks VALUES('Example', 'https://example.com', 0);",
    );
    let run = root.evidence("ios_safari_bookmarks");
    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("artifact"), Some("ios_safari_bookmarks"));
    assert_eq!(
        data(record, "url").and_then(Value::as_str),
        Some("https://example.com")
    );
}

#[test]
fn safari_tabs_skip_deleted_rows() {
    let root = Root::new("safari-tabs");
    root.database(
        "data/Safari/SafariTabs.db",
        "CREATE TABLE bookmarks(title TEXT, url TEXT, hidden INTEGER, deleted INTEGER);\
         INSERT INTO bookmarks VALUES('Open', 'https://open.example', 0, 0);\
         INSERT INTO bookmarks VALUES('Gone', 'https://gone.example', 0, 1);",
    );
    let run = root.evidence("ios_safari_tabs");
    assert_eq!(run.count("mobile.artifact"), 1);
    assert_eq!(
        data(run.of_type("mobile.artifact")[0], "title").and_then(Value::as_str),
        Some("Open")
    );
}

#[test]
fn keyboard_usage_reads_key_and_value() {
    let root = Root::new("keyboard");
    root.database(
        "data/Keyboard/user_model_database.sqlite",
        &format!(
            "CREATE TABLE usermodeldurablerecords(creation_date INTEGER, last_update_date INTEGER, \
             key TEXT, value TEXT);\
             INSERT INTO usermodeldurablerecords VALUES({}, {COCOA_2024}, 'typed', '42');",
            1_704_067_200
        ),
    );
    let run = root.evidence("ios_keyboard");
    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("time"), Some("2024-01-01T00:00:00Z"));
    assert_eq!(data(record, "key").and_then(Value::as_str), Some("typed"));
}

#[test]
fn voicemail_reads_sender_and_duration() {
    let root = Root::new("voicemail");
    root.database(
        "data/Voicemail/voicemail.db",
        "CREATE TABLE voicemail(date INTEGER, sender TEXT, receiver TEXT, duration INTEGER, \
         trashed_date INTEGER, flags INTEGER);\
         INSERT INTO voicemail VALUES(1704067200, '+8613800000000', '+8613900000000', 30, 0, 0);",
    );
    let run = root.evidence("ios_voicemail");
    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(record.text("time"), Some("2024-01-01T00:00:00Z"));
    assert_eq!(
        data(record, "sender").and_then(Value::as_str),
        Some("+8613800000000")
    );
}

#[test]
fn wifi_network_store_reads_ssid() {
    let root = Root::new("wifi");
    root.database(
        "data/Library/Application Support/WiFiNetworkStoreModel.sqlite",
        "CREATE TABLE Z_PRIMARYKEY(Z_ENT INTEGER, Z_NAME TEXT);\
         CREATE TABLE ZNETWORK(Z_PK INTEGER PRIMARY KEY, Z_ENT INTEGER, ZSSID TEXT, ZAUTHFLAG INTEGER);\
         INSERT INTO Z_PRIMARYKEY VALUES(1, 'Network');\
         INSERT INTO ZNETWORK VALUES(1, 1, 'HomeWiFi', 4);",
    );
    let run = root.evidence("ios_wifi_networks");
    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(data(record, "ssid").and_then(Value::as_str), Some("HomeWiFi"));
}

#[test]
fn bluetooth_paired_reads_device_name() {
    let root = Root::new("bluetooth");
    root.database(
        "data/com.apple.MobileBluetooth.ledevices.paired.db",
        "CREATE TABLE PairedDevices(Uuid TEXT, Name TEXT, NameOrigin TEXT, Address TEXT, \
         ResolvedAddress TEXT, LastSeenTime INTEGER, LastConnectionTime INTEGER);\
         INSERT INTO PairedDevices VALUES('uuid-1', 'Watch', 'user', 'AA:BB', 'AA:BB', 100, 200);",
    );
    let run = root.evidence("ios_bluetooth");
    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(data(record, "name").and_then(Value::as_str), Some("Watch"));
}

#[test]
fn screen_time_settings_read_flags() {
    let root = Root::new("screen-time");
    root.database(
        "data/RMAdminStore-Local.sqlite",
        "CREATE TABLE ZSCREENTIMESETTINGS(Z_PK INTEGER PRIMARY KEY, Z_OPT INTEGER, \
         ZSCREENTIMEENABLED INTEGER, ZCLOUDSYNCENABLED INTEGER, ZFAMILYSETUPCOMPLETE INTEGER);\
         INSERT INTO ZSCREENTIMESETTINGS VALUES(1, 3, 1, 0, 0);",
    );
    let run = root.evidence("ios_screen_time");
    let record = run.of_type("mobile.artifact")[0];
    assert_eq!(data(record, "enabled").and_then(Value::as_i64), Some(1));
}

#[test]
fn selected_database_uses_the_files_scope() {
    let root = Root::new("selected");
    let path = message_fixture(&root);
    let run = root.selected("ios_selected_databases", &path);

    assert_eq!(run.count("mobile.artifact"), 1);
    assert_eq!(
        run.of_type("mobile.artifact")[0].text("artifact"),
        Some("ios_messages")
    );
}

#[test]
fn selected_plist_uses_the_files_scope() {
    let root = Root::new("selected-plist");
    let path = root.write(
        "SystemVersion.plist",
        r#"<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0"><dict>
<key>ProductVersion</key><string>16.7</string>
</dict></plist>
"#,
    );
    let run = root.selected("ios_selected_plist", &path);

    assert_eq!(run.count("mobile.artifact"), 1);
    assert_eq!(
        data(run.of_type("mobile.artifact")[0], "ProductVersion").and_then(Value::as_str),
        Some("16.7")
    );
}

#[test]
fn missing_database_is_a_warning() {
    let root = Root::new("missing");
    let run = root.evidence("ios_messages");

    assert_eq!(run.count("mobile.artifact"), 0);
    assert!(run
        .warnings
        .iter()
        .any(|(code, _, _)| code == "ARTIFACT_NOT_FOUND"));
}
