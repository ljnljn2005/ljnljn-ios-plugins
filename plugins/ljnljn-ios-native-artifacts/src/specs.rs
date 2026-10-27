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

pub static IOS_SPECS: &[&SqlSpec] = &[
    &IOS_MESSAGES,
    &IOS_CALLS,
    &IOS_CONTACTS,
    &IOS_SAFARI,
    &IOS_WHATSAPP,
    &IOS_PHOTOS,
    &IOS_CALENDAR,
];
