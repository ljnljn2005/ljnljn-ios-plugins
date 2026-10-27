//! Fixed SQL definitions for the iOS core artifacts.
//!
//! Each unit has an ordered variant list. Variants express real schema
//! differences between iOS releases; they are not user input and never
//! interpolate text from evidence.

use mobile_core::{SqlSpec, SqlUnit, SqlVariant, TimeKind};

static MESSAGES: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["message", "handle", "chat", "chat_message_join"],
        sql: r#"
SELECT m.ROWID AS message_id, m.guid AS guid, m.text AS text,
       m.attributedBody AS attributed_body, m.handle_id AS handle_id,
       h.id AS handle, m.service AS service, m.date AS date,
       m.date_read AS date_read, m.date_delivered AS date_delivered,
       m.is_from_me AS is_from_me, m.cache_has_attachments AS has_attachments,
       c.chat_identifier AS chat
FROM message m
LEFT JOIN handle h ON h.ROWID = m.handle_id
LEFT JOIN chat_message_join cmj ON cmj.message_id = m.ROWID
LEFT JOIN chat c ON c.ROWID = cmj.chat_id
ORDER BY m.date ASC
"#,
    },
    SqlVariant {
        required_tables: &["message", "handle"],
        sql: r#"
SELECT m.ROWID AS message_id, m.guid AS guid, m.text AS text,
       m.attributedBody AS attributed_body, m.handle_id AS handle_id,
       h.id AS handle, m.service AS service, m.date AS date,
       m.date_read AS date_read, m.date_delivered AS date_delivered,
       m.is_from_me AS is_from_me, m.cache_has_attachments AS has_attachments,
       NULL AS chat
FROM message m
LEFT JOIN handle h ON h.ROWID = m.handle_id
ORDER BY m.date ASC
"#,
    },
    SqlVariant {
        required_tables: &["message"],
        sql: r#"
SELECT ROWID AS message_id, guid AS guid, text AS text,
       attributedBody AS attributed_body, handle_id AS handle_id,
       NULL AS handle, service AS service, date AS date,
       date_read AS date_read, date_delivered AS date_delivered,
       is_from_me AS is_from_me, cache_has_attachments AS has_attachments,
       NULL AS chat
FROM message ORDER BY date ASC
"#,
    },
    SqlVariant {
        required_tables: &["message"],
        sql: r#"
SELECT ROWID AS message_id, guid AS guid, text AS text,
       NULL AS attributed_body, handle_id AS handle_id, NULL AS handle,
       service AS service, date AS date, NULL AS date_read,
       NULL AS date_delivered, is_from_me AS is_from_me,
       NULL AS has_attachments, NULL AS chat
FROM message ORDER BY date ASC
"#,
    },
    SqlVariant {
        required_tables: &["message"],
        sql: r#"
SELECT ROWID AS message_id, guid AS guid, text AS text,
       NULL AS attributed_body, handle_id AS handle_id, NULL AS handle,
       NULL AS service, date AS date, NULL AS date_read,
       NULL AS date_delivered, is_from_me AS is_from_me,
       NULL AS has_attachments, NULL AS chat
FROM message ORDER BY date ASC
"#,
    },
];

static CALLS_COREDATA: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["ZCALLRECORD"],
        sql: r#"
SELECT Z_PK AS call_id, ZADDRESS AS address, ZDATE AS date,
       ZDURATION AS duration, ZORIGINATED AS originated,
       ZANSWERED AS answered, ZCALLTYPE AS call_type,
       ZSERVICE_PROVIDER AS service_provider
FROM ZCALLRECORD ORDER BY ZDATE ASC
"#,
    },
    SqlVariant {
        required_tables: &["ZCALLRECORD"],
        sql: r#"
SELECT Z_PK AS call_id, ZADDRESS AS address, ZDATE AS date,
       ZDURATION AS duration, NULL AS originated, NULL AS answered,
       NULL AS call_type, NULL AS service_provider
FROM ZCALLRECORD ORDER BY ZDATE ASC
"#,
    },
];

static CALLS_LEGACY: &[SqlVariant] = &[SqlVariant {
    required_tables: &["call"],
    sql: r#"
SELECT rowid AS call_id, address AS address, date AS date, duration AS duration,
       flags AS call_type, NULL AS originated, NULL AS answered,
       NULL AS service_provider
FROM call ORDER BY date ASC
"#,
}];

static CONTACTS: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["ABPerson", "ABMultiValue"],
        sql: r#"
SELECT p.ROWID AS person_id, p.First AS first, p.Last AS last,
       p.Organization AS organization, p.JobTitle AS job_title,
       (SELECT group_concat(value, '; ') FROM ABMultiValue m
         WHERE m.record_id = p.ROWID AND m.property = 3) AS phones,
       (SELECT group_concat(value, '; ') FROM ABMultiValue m
         WHERE m.record_id = p.ROWID AND m.property = 4) AS emails
FROM ABPerson p ORDER BY p.ROWID ASC
"#,
    },
    SqlVariant {
        required_tables: &["ABPerson"],
        sql: r#"
SELECT ROWID AS person_id, First AS first, Last AS last,
       Organization AS organization, JobTitle AS job_title,
       NULL AS phones, NULL AS emails
FROM ABPerson ORDER BY ROWID ASC
"#,
    },
    SqlVariant {
        required_tables: &["ABPerson"],
        sql: r#"
SELECT ROWID AS person_id, First AS first, Last AS last,
       NULL AS organization, NULL AS job_title, NULL AS phones, NULL AS emails
FROM ABPerson ORDER BY ROWID ASC
"#,
    },
];

static SAFARI: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["history_visits", "history_items"],
        sql: r#"
SELECT v.id AS visit_id, v.visit_time AS visit_time, v.title AS visit_title,
       i.url AS url, i.title AS item_title, i.visit_count AS visit_count,
       v.load_successful AS load_successful
FROM history_visits v
JOIN history_items i ON i.id = v.history_item
ORDER BY v.visit_time ASC
"#,
    },
    SqlVariant {
        required_tables: &["history_visits", "history_items"],
        sql: r#"
SELECT v.id AS visit_id, v.visit_time AS visit_time, v.title AS visit_title,
       i.url AS url, i.title AS item_title, i.visit_count AS visit_count,
       NULL AS load_successful
FROM history_visits v
JOIN history_items i ON i.id = v.history_item
ORDER BY v.visit_time ASC
"#,
    },
];

static WHATSAPP: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["ZWAMESSAGE", "ZWACHATSESSION"],
        sql: r#"
SELECT m.Z_PK AS message_id, m.ZMESSAGEDATE AS date,
       m.ZISFROMME AS is_from_me, m.ZFROMJID AS from_jid,
       m.ZTOJID AS to_jid, m.ZTEXT AS text, m.ZMEDIAITEM AS media_item,
       m.ZMESSAGETYPE AS message_type, s.ZPARTNERNAME AS chat_name,
       s.ZCONTACTJID AS chat_jid
FROM ZWAMESSAGE m
LEFT JOIN ZWACHATSESSION s ON s.Z_PK = m.ZCHATSESSION
ORDER BY m.ZMESSAGEDATE ASC
"#,
    },
    SqlVariant {
        required_tables: &["ZWAMESSAGE"],
        sql: r#"
SELECT Z_PK AS message_id, ZMESSAGEDATE AS date, ZISFROMME AS is_from_me,
       ZFROMJID AS from_jid, ZTOJID AS to_jid, ZTEXT AS text,
       ZMEDIAITEM AS media_item, ZMESSAGETYPE AS message_type,
       NULL AS chat_name, NULL AS chat_jid
FROM ZWAMESSAGE ORDER BY ZMESSAGEDATE ASC
"#,
    },
    SqlVariant {
        required_tables: &["ZWAMESSAGE"],
        sql: r#"
SELECT Z_PK AS message_id, ZMESSAGEDATE AS date, ZISFROMME AS is_from_me,
       ZFROMJID AS from_jid, ZTOJID AS to_jid, ZTEXT AS text,
       NULL AS media_item, NULL AS message_type, NULL AS chat_name,
       NULL AS chat_jid
FROM ZWAMESSAGE ORDER BY ZMESSAGEDATE ASC
"#,
    },
];

static PHOTOS: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["ZASSET", "ZADDITIONALASSETATTRIBUTES"],
        sql: r#"
SELECT a.Z_PK AS asset_id, a.ZDATECREATED AS date_created,
       a.ZADDEDDATE AS date_added, a.ZMODIFICATIONDATE AS date_modified,
       a.ZFILENAME AS filename, a.ZDIRECTORY AS directory,
       a.ZUNIFORMTYPEIDENTIFIER AS uniform_type, a.ZWIDTH AS width,
       a.ZHEIGHT AS height, a.ZLATITUDE AS latitude, a.ZLONGITUDE AS longitude,
       aa.ZORIGINALFILENAME AS original_filename
FROM ZASSET a
LEFT JOIN ZADDITIONALASSETATTRIBUTES aa ON aa.ZASSET = a.Z_PK
ORDER BY a.ZDATECREATED ASC
"#,
    },
    SqlVariant {
        required_tables: &["ZASSET"],
        sql: r#"
SELECT Z_PK AS asset_id, ZDATECREATED AS date_created,
       ZADDEDDATE AS date_added, ZMODIFICATIONDATE AS date_modified,
       ZFILENAME AS filename, ZDIRECTORY AS directory,
       ZUNIFORMTYPEIDENTIFIER AS uniform_type, ZWIDTH AS width,
       ZHEIGHT AS height, ZLATITUDE AS latitude, ZLONGITUDE AS longitude,
       NULL AS original_filename
FROM ZASSET ORDER BY ZDATECREATED ASC
"#,
    },
    SqlVariant {
        required_tables: &["ZASSET"],
        sql: r#"
SELECT Z_PK AS asset_id, ZDATECREATED AS date_created,
       NULL AS date_added, NULL AS date_modified, ZFILENAME AS filename,
       NULL AS directory, NULL AS uniform_type, NULL AS width, NULL AS height,
       NULL AS latitude, NULL AS longitude, NULL AS original_filename
FROM ZASSET ORDER BY ZDATECREATED ASC
"#,
    },
];

static PHOTOS_GENERIC: &[SqlVariant] = &[SqlVariant {
    required_tables: &["ZGENERICASSET"],
    sql: r#"
SELECT Z_PK AS asset_id, ZDATECREATED AS date_created,
       ZADDEDDATE AS date_added, ZMODIFICATIONDATE AS date_modified,
       ZFILENAME AS filename, ZDIRECTORY AS directory,
       NULL AS uniform_type, ZWIDTH AS width, ZHEIGHT AS height,
       ZLATITUDE AS latitude, ZLONGITUDE AS longitude,
       NULL AS original_filename
FROM ZGENERICASSET ORDER BY ZDATECREATED ASC
"#,
}];

static CALENDAR: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["CalendarItem", "Calendar"],
        sql: r#"
SELECT ci.ROWID AS event_id, ci.summary AS summary,
       ci.start_date AS start_date, ci.end_date AS end_date,
       ci.all_day AS all_day, ci.location AS location,
       ci.description AS description, ci.creation_date AS creation_date,
       ci.last_modified AS last_modified, c.title AS calendar
FROM CalendarItem ci
LEFT JOIN Calendar c ON c.ROWID = ci.calendar_id
ORDER BY ci.start_date ASC
"#,
    },
    SqlVariant {
        required_tables: &["CalendarItem"],
        sql: r#"
SELECT ROWID AS event_id, summary AS summary, start_date AS start_date,
       end_date AS end_date, all_day AS all_day, location AS location,
       description AS description, creation_date AS creation_date,
       last_modified AS last_modified, NULL AS calendar
FROM CalendarItem ORDER BY start_date ASC
"#,
    },
    SqlVariant {
        required_tables: &["CalendarItem"],
        sql: r#"
SELECT ROWID AS event_id, summary AS summary, start_date AS start_date,
       end_date AS end_date, NULL AS all_day, location AS location,
       NULL AS description, NULL AS creation_date, NULL AS last_modified,
       NULL AS calendar
FROM CalendarItem ORDER BY start_date ASC
"#,
    },
];

pub static IOS_MESSAGES: SqlSpec = SqlSpec {
    artifact: "ios_messages",
    platform: "ios",
    paths: &["**/sms.db"],
    units: &[SqlUnit {
        label: "message",
        variants: MESSAGES,
        time_column: Some("date"),
        time_kind: TimeKind::Cocoa,
        key_columns: &["message_id", "guid"],
        title_columns: &["text", "handle", "chat"],
    }],
};

pub static IOS_CALLS: SqlSpec = SqlSpec {
    artifact: "ios_calls",
    platform: "ios",
    paths: &[
        "**/CallHistory.storedata",
        "**/CallHistoryTemp.storedata",
        "**/call_history.db",
    ],
    units: &[
        SqlUnit {
            label: "coredata",
            variants: CALLS_COREDATA,
            time_column: Some("date"),
            time_kind: TimeKind::Cocoa,
            key_columns: &["call_id"],
            title_columns: &["address"],
        },
        SqlUnit {
            label: "legacy",
            variants: CALLS_LEGACY,
            time_column: Some("date"),
            time_kind: TimeKind::UnixSeconds,
            key_columns: &["call_id"],
            title_columns: &["address"],
        },
    ],
};

pub static IOS_CONTACTS: SqlSpec = SqlSpec {
    artifact: "ios_contacts",
    platform: "ios",
    paths: &["**/AddressBook.sqlitedb"],
    units: &[SqlUnit {
        label: "person",
        variants: CONTACTS,
        time_column: None,
        time_kind: TimeKind::UnixSeconds,
        key_columns: &["person_id"],
        title_columns: &["first", "last", "organization"],
    }],
};

pub static IOS_SAFARI: SqlSpec = SqlSpec {
    artifact: "ios_safari_history",
    platform: "ios",
    paths: &["**/Safari/History.db", "**/History.db"],
    units: &[SqlUnit {
        label: "visit",
        variants: SAFARI,
        time_column: Some("visit_time"),
        time_kind: TimeKind::CocoaOrUnix,
        key_columns: &["visit_id"],
        title_columns: &["visit_title", "item_title", "url"],
    }],
};

pub static IOS_WHATSAPP: SqlSpec = SqlSpec {
    artifact: "ios_whatsapp",
    platform: "ios",
    paths: &["**/ChatStorage.sqlite"],
    units: &[SqlUnit {
        label: "message",
        variants: WHATSAPP,
        time_column: Some("date"),
        time_kind: TimeKind::Cocoa,
        key_columns: &["message_id"],
        title_columns: &["chat_name", "text", "from_jid"],
    }],
};

pub static IOS_PHOTOS: SqlSpec = SqlSpec {
    artifact: "ios_photos",
    platform: "ios",
    paths: &["**/PhotoData/Photos.sqlite", "**/Photos.sqlite"],
    units: &[
        SqlUnit {
            label: "asset",
            variants: PHOTOS,
            time_column: Some("date_created"),
            time_kind: TimeKind::Cocoa,
            key_columns: &["asset_id"],
            title_columns: &["filename", "original_filename"],
        },
        SqlUnit {
            label: "generic_asset",
            variants: PHOTOS_GENERIC,
            time_column: Some("date_created"),
            time_kind: TimeKind::Cocoa,
            key_columns: &["asset_id"],
            title_columns: &["filename"],
        },
    ],
};

pub static IOS_CALENDAR: SqlSpec = SqlSpec {
    artifact: "ios_calendar",
    platform: "ios",
    paths: &["**/Calendar.sqlitedb"],
    units: &[SqlUnit {
        label: "event",
        variants: CALENDAR,
        time_column: Some("start_date"),
        time_kind: TimeKind::Cocoa,
        key_columns: &["event_id"],
        title_columns: &["summary", "location", "calendar"],
    }],
};

static NOTES: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["ZICCLOUDSYNCINGOBJECT", "ZICNOTEDATA"],
        sql: r#"
SELECT a.Z_PK AS note_id, a.ZTITLE1 AS title, a.ZSNIPPET AS snippet,
       b.ZTITLE2 AS folder, a.ZMODIFICATIONDATE1 AS modified,
       a.ZISPASSWORDPROTECTED AS password_protected,
       a.ZMARKEDFORDELETION AS marked_deleted, a.ZISPINNED AS pinned,
       d.ZFILESIZE AS attachment_size, d.ZTYPEUTI AS attachment_type
FROM ZICCLOUDSYNCINGOBJECT a
LEFT JOIN ZICCLOUDSYNCINGOBJECT b ON a.ZFOLDER = b.Z_PK
LEFT JOIN ZICCLOUDSYNCINGOBJECT d ON a.Z_PK = d.ZNOTE
INNER JOIN ZICNOTEDATA f ON f.ZNOTE = a.Z_PK
ORDER BY a.ZMODIFICATIONDATE1 ASC
"#,
    },
    SqlVariant {
        required_tables: &["ZICCLOUDSYNCINGOBJECT", "ZICNOTEDATA"],
        sql: r#"
SELECT a.Z_PK AS note_id, a.ZTITLE1 AS title, a.ZSNIPPET AS snippet,
       NULL AS folder, a.ZMODIFICATIONDATE1 AS modified,
       NULL AS password_protected, NULL AS marked_deleted, NULL AS pinned,
       NULL AS attachment_size, NULL AS attachment_type
FROM ZICCLOUDSYNCINGOBJECT a
INNER JOIN ZICNOTEDATA f ON f.ZNOTE = a.Z_PK
ORDER BY a.ZMODIFICATIONDATE1 ASC
"#,
    },
];

static SAFARI_BOOKMARKS: &[SqlVariant] = &[SqlVariant {
    required_tables: &["bookmarks"],
    sql: r#"
SELECT ROWID AS bookmark_id, title AS title, url AS url, hidden AS hidden
FROM bookmarks ORDER BY ROWID ASC
"#,
}];

static SAFARI_TABS: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["bookmarks"],
        sql: r#"
SELECT ROWID AS tab_id, title AS title, url AS url, hidden AS hidden,
       deleted AS deleted
FROM bookmarks
WHERE url IS NOT NULL AND (deleted IS NULL OR deleted = 0)
ORDER BY ROWID ASC
"#,
    },
    SqlVariant {
        required_tables: &["bookmarks"],
        sql: r#"
SELECT ROWID AS tab_id, title AS title, url AS url, hidden AS hidden,
       NULL AS deleted
FROM bookmarks WHERE url IS NOT NULL ORDER BY ROWID ASC
"#,
    },
];

static KEYBOARD: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["usermodeldurablerecords"],
        sql: r#"
SELECT ROWID AS record_id, creation_date AS creation_date,
       last_update_date AS last_update_date, key AS key, value AS value
FROM usermodeldurablerecords ORDER BY last_update_date ASC
"#,
    },
    SqlVariant {
        required_tables: &["usermodeldurablerecords"],
        sql: r#"
SELECT ROWID AS record_id, NULL AS creation_date,
       last_update_date AS last_update_date, key AS key, value AS value
FROM usermodeldurablerecords ORDER BY last_update_date ASC
"#,
    },
];

static VOICEMAIL: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["voicemail"],
        sql: r#"
SELECT ROWID AS voicemail_id, date AS date, sender AS sender,
       receiver AS receiver, duration AS duration,
       trashed_date AS trashed_date, flags AS flags
FROM voicemail ORDER BY date ASC
"#,
    },
    SqlVariant {
        required_tables: &["voicemail"],
        sql: r#"
SELECT ROWID AS voicemail_id, date AS date, sender AS sender,
       callback_num AS receiver, duration AS duration,
       trashed_date AS trashed_date, flags AS flags
FROM voicemail ORDER BY date ASC
"#,
    },
];

static WIFI_NETWORKS: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["ZNETWORK", "Z_PRIMARYKEY"],
        sql: r#"
SELECT n.Z_PK AS network_id, p.Z_NAME AS entity, n.ZSSID AS ssid,
       n.ZAUTHFLAG AS auth_flag
FROM ZNETWORK n
LEFT JOIN Z_PRIMARYKEY p ON p.Z_ENT = n.Z_ENT
ORDER BY n.Z_PK ASC
"#,
    },
    SqlVariant {
        required_tables: &["ZNETWORK"],
        sql: r#"
SELECT Z_PK AS network_id, NULL AS entity, ZSSID AS ssid,
       ZAUTHFLAG AS auth_flag
FROM ZNETWORK ORDER BY Z_PK ASC
"#,
    },
];

static BLUETOOTH_PAIRED: &[SqlVariant] = &[SqlVariant {
    required_tables: &["PairedDevices"],
    sql: r#"
SELECT Uuid AS uuid, Name AS name, NameOrigin AS name_origin,
       Address AS address, ResolvedAddress AS resolved_address,
       LastSeenTime AS last_seen, LastConnectionTime AS last_connection
FROM PairedDevices ORDER BY LastConnectionTime ASC
"#,
}];

static SCREEN_TIME: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["ZSCREENTIMESETTINGS"],
        sql: r#"
SELECT Z_PK AS row_id, Z_OPT AS row_version, ZSCREENTIMEENABLED AS enabled,
       ZCLOUDSYNCENABLED AS cloud_sync, ZFAMILYSETUPCOMPLETE AS family_setup
FROM ZSCREENTIMESETTINGS ORDER BY Z_PK ASC
"#,
    },
    SqlVariant {
        required_tables: &["ZSCREENTIMESETTINGS"],
        sql: r#"
SELECT Z_PK AS row_id, Z_OPT AS row_version, ZSCREENTIMEENABLED AS enabled,
       ZCLOUDSYNCENABLED AS cloud_sync, NULL AS family_setup
FROM ZSCREENTIMESETTINGS ORDER BY Z_PK ASC
"#,
    },
];

pub static IOS_NOTES: SqlSpec = SqlSpec {
    artifact: "ios_notes",
    platform: "ios",
    paths: &["**/NoteStore.sqlite"],
    units: &[SqlUnit {
        label: "note",
        variants: NOTES,
        time_column: Some("modified"),
        time_kind: TimeKind::Cocoa,
        key_columns: &["note_id"],
        title_columns: &["title", "snippet", "folder"],
    }],
};

pub static IOS_SAFARI_BOOKMARKS: SqlSpec = SqlSpec {
    artifact: "ios_safari_bookmarks",
    platform: "ios",
    paths: &["**/Safari/Bookmarks.db"],
    units: &[SqlUnit {
        label: "bookmark",
        variants: SAFARI_BOOKMARKS,
        time_column: None,
        time_kind: TimeKind::UnixSeconds,
        key_columns: &["bookmark_id"],
        title_columns: &["title", "url"],
    }],
};

pub static IOS_SAFARI_TABS: SqlSpec = SqlSpec {
    artifact: "ios_safari_tabs",
    platform: "ios",
    paths: &["**/Safari/SafariTabs.db"],
    units: &[SqlUnit {
        label: "tab",
        variants: SAFARI_TABS,
        time_column: None,
        time_kind: TimeKind::UnixSeconds,
        key_columns: &["tab_id"],
        title_columns: &["title", "url"],
    }],
};

pub static IOS_KEYBOARD: SqlSpec = SqlSpec {
    artifact: "ios_keyboard",
    platform: "ios",
    paths: &["**/Keyboard/user_model_database.sqlite"],
    units: &[SqlUnit {
        label: "usage",
        variants: KEYBOARD,
        time_column: Some("creation_date"),
        time_kind: TimeKind::UnixSeconds,
        key_columns: &["record_id"],
        title_columns: &["key", "value"],
    }],
};

pub static IOS_VOICEMAIL: SqlSpec = SqlSpec {
    artifact: "ios_voicemail",
    platform: "ios",
    paths: &["**/Voicemail/voicemail.db"],
    units: &[SqlUnit {
        label: "voicemail",
        variants: VOICEMAIL,
        time_column: Some("date"),
        time_kind: TimeKind::UnixSeconds,
        key_columns: &["voicemail_id"],
        title_columns: &["sender", "receiver"],
    }],
};

pub static IOS_WIFI_NETWORKS: SqlSpec = SqlSpec {
    artifact: "ios_wifi_networks",
    platform: "ios",
    paths: &["**/WiFiNetworkStoreModel.sqlite"],
    units: &[SqlUnit {
        label: "network",
        variants: WIFI_NETWORKS,
        time_column: None,
        time_kind: TimeKind::Cocoa,
        key_columns: &["network_id"],
        title_columns: &["ssid", "entity"],
    }],
};

pub static IOS_BLUETOOTH: SqlSpec = SqlSpec {
    artifact: "ios_bluetooth",
    platform: "ios",
    paths: &["**/com.apple.MobileBluetooth.ledevices.paired.db"],
    units: &[SqlUnit {
        label: "paired",
        variants: BLUETOOTH_PAIRED,
        time_column: None,
        time_kind: TimeKind::Cocoa,
        key_columns: &["uuid", "address"],
        title_columns: &["name", "address"],
    }],
};

pub static IOS_SCREEN_TIME: SqlSpec = SqlSpec {
    artifact: "ios_screen_time",
    platform: "ios",
    paths: &["**/RMAdminStore-*.sqlite"],
    units: &[SqlUnit {
        label: "settings",
        variants: SCREEN_TIME,
        time_column: None,
        time_kind: TimeKind::Cocoa,
        key_columns: &["row_id"],
        title_columns: &["row_id", "row_version"],
    }],
};

static SMS_REACTIONS: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["message", "handle", "chat", "chat_message_join"],
        sql: r#"
SELECT m.ROWID AS message_id, m.guid AS guid, m.date AS date,
       m.is_from_me AS is_from_me,
       CASE WHEN m.is_from_me = 0 THEN h.id END AS sender,
       c.chat_identifier AS chat, m.associated_message_type AS reaction_type,
       m.associated_message_guid AS target_guid, m.service AS service
FROM message m
LEFT JOIN handle h ON h.ROWID = m.handle_id
LEFT JOIN chat_message_join cmj ON cmj.message_id = m.ROWID
LEFT JOIN chat c ON c.ROWID = cmj.chat_id
WHERE m.associated_message_type = 1000
   OR m.associated_message_type BETWEEN 2000 AND 3007
ORDER BY m.date ASC, m.ROWID ASC
"#,
    },
    SqlVariant {
        required_tables: &["message"],
        sql: r#"
SELECT ROWID AS message_id, guid AS guid, date AS date,
       is_from_me AS is_from_me, NULL AS sender, NULL AS chat,
       associated_message_type AS reaction_type,
       associated_message_guid AS target_guid, service AS service
FROM message
WHERE associated_message_type = 1000
   OR associated_message_type BETWEEN 2000 AND 3007
ORDER BY date ASC, ROWID ASC
"#,
    },
];

static SMS_REPLIES: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["message", "handle", "chat", "chat_message_join"],
        sql: r#"
SELECT m.ROWID AS message_id, m.guid AS guid, m.date AS date,
       m.is_from_me AS is_from_me, m.text AS text,
       m.thread_originator_guid AS target_guid, h.id AS sender,
       c.chat_identifier AS chat, m.service AS service
FROM message m
LEFT JOIN handle h ON h.ROWID = m.handle_id
LEFT JOIN chat_message_join cmj ON cmj.message_id = m.ROWID
LEFT JOIN chat c ON c.ROWID = cmj.chat_id
WHERE m.thread_originator_guid IS NOT NULL AND m.thread_originator_guid != ''
ORDER BY m.date ASC, m.ROWID ASC
"#,
    },
    SqlVariant {
        required_tables: &["message"],
        sql: r#"
SELECT ROWID AS message_id, guid AS guid, date AS date,
       is_from_me AS is_from_me, text AS text,
       thread_originator_guid AS target_guid, NULL AS sender,
       NULL AS chat, service AS service
FROM message
WHERE thread_originator_guid IS NOT NULL AND thread_originator_guid != ''
ORDER BY date ASC, ROWID ASC
"#,
    },
];

static SMS_RECENTLY_DELETED: &[SqlVariant] = &[
    SqlVariant {
        required_tables: &["chat_recoverable_message_join", "message", "handle", "chat"],
        sql: r#"
SELECT r.delete_date AS deleted_date, m.ROWID AS message_id, m.guid AS guid,
       m.date AS date, m.text AS text, m.is_from_me AS is_from_me,
       h.id AS sender, c.chat_identifier AS chat, r.ck_sync_state AS sync_state
FROM chat_recoverable_message_join r
JOIN message m ON m.ROWID = r.message_id
LEFT JOIN handle h ON h.ROWID = m.handle_id
LEFT JOIN chat c ON c.ROWID = r.chat_id
ORDER BY r.delete_date ASC, m.ROWID ASC
"#,
    },
    SqlVariant {
        required_tables: &["chat_recoverable_message_join", "message"],
        sql: r#"
SELECT r.delete_date AS deleted_date, m.ROWID AS message_id, m.guid AS guid,
       m.date AS date, m.text AS text, m.is_from_me AS is_from_me,
       NULL AS sender, NULL AS chat, NULL AS sync_state
FROM chat_recoverable_message_join r
JOIN message m ON m.ROWID = r.message_id
ORDER BY r.delete_date ASC, m.ROWID ASC
"#,
    },
];

pub static IOS_SMS_REACTIONS: SqlSpec = SqlSpec {
    artifact: "ios_sms_reactions",
    platform: "ios",
    paths: &["**/sms.db"],
    units: &[SqlUnit {
        label: "reaction",
        variants: SMS_REACTIONS,
        time_column: Some("date"),
        time_kind: TimeKind::Cocoa,
        key_columns: &["message_id", "guid"],
        title_columns: &["chat", "sender", "reaction_type"],
    }],
};

pub static IOS_SMS_REPLIES: SqlSpec = SqlSpec {
    artifact: "ios_sms_replies",
    platform: "ios",
    paths: &["**/sms.db"],
    units: &[SqlUnit {
        label: "reply",
        variants: SMS_REPLIES,
        time_column: Some("date"),
        time_kind: TimeKind::Cocoa,
        key_columns: &["message_id", "guid"],
        title_columns: &["text", "chat"],
    }],
};

pub static IOS_SMS_RECENTLY_DELETED: SqlSpec = SqlSpec {
    artifact: "ios_sms_recently_deleted",
    platform: "ios",
    paths: &["**/sms.db"],
    units: &[SqlUnit {
        label: "deleted",
        variants: SMS_RECENTLY_DELETED,
        time_column: Some("deleted_date"),
        time_kind: TimeKind::Cocoa,
        key_columns: &["message_id", "guid"],
        title_columns: &["text", "chat"],
    }],
};

pub static IOS_SPECS: &[&SqlSpec] = &[
    &IOS_MESSAGES,
    &IOS_CALLS,
    &IOS_CONTACTS,
    &IOS_SAFARI,
    &IOS_WHATSAPP,
    &IOS_PHOTOS,
    &IOS_CALENDAR,
    &IOS_NOTES,
    &IOS_SAFARI_BOOKMARKS,
    &IOS_SAFARI_TABS,
    &IOS_KEYBOARD,
    &IOS_VOICEMAIL,
    &IOS_WIFI_NETWORKS,
    &IOS_BLUETOOTH,
    &IOS_SCREEN_TIME,
    &IOS_SMS_REACTIONS,
    &IOS_SMS_REPLIES,
    &IOS_SMS_RECENTLY_DELETED,
];
