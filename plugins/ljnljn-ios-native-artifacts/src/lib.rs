//! Native iOS artifact readers (LPP v2).
//!
//! This plugin is the direct-read counterpart of the iLEAPP TSV importer. It
//! opens the extraction itself and runs fixed read-only SQL against the core
//! iOS stores; it never starts Python or executes evidence-supplied SQL.
//!
//! Coverage in this first native batch: device/backup metadata, SMS/iMessage,
//! call history, contacts, Safari history, WhatsApp messages, Photos metadata
//! and Calendar events. Long-tail iLEAPP modules remain available through the
//! report-import plugin until they are ported.

mod device;
mod specs;

#[cfg(test)]
mod tests;

use lovelycarve_plugin::{export_plugin, Plugin, Result, Run};
use mobile_core::{run_evidence, ArtifactFile, ArtifactRecord};

fn device_info(run: &Run) -> Result<()> {
    device::run_evidence(run)
}

fn messages(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_MESSAGES)
}

fn calls(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_CALLS)
}

fn contacts(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_CONTACTS)
}

fn safari(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_SAFARI)
}

fn whatsapp(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_WHATSAPP)
}

fn photos(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_PHOTOS)
}

fn calendar(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_CALENDAR)
}

fn notes(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_NOTES)
}

fn safari_bookmarks(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_SAFARI_BOOKMARKS)
}

fn safari_tabs(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_SAFARI_TABS)
}

fn keyboard(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_KEYBOARD)
}

fn voicemail(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_VOICEMAIL)
}

fn wifi_networks(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_WIFI_NETWORKS)
}

fn bluetooth(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_BLUETOOTH)
}

fn screen_time(run: &Run) -> Result<()> {
    run_evidence(run, &specs::IOS_SCREEN_TIME)
}

fn selected_databases(run: &Run) -> Result<()> {
    mobile_core::run_files(run, specs::IOS_SPECS)
}

fn selected_plist(run: &Run) -> Result<()> {
    device::run_selected(run)
}

fn plugin() -> Plugin {
    Plugin::new("org.ljnljn.ios-native-artifacts")
        .record::<ArtifactRecord>()
        .record::<ArtifactFile>()
        .analyzer("ios_device_info", device_info)
        .analyzer("ios_messages", messages)
        .analyzer("ios_calls", calls)
        .analyzer("ios_contacts", contacts)
        .analyzer("ios_safari_history", safari)
        .analyzer("ios_whatsapp", whatsapp)
        .analyzer("ios_photos", photos)
        .analyzer("ios_calendar", calendar)
        .analyzer("ios_notes", notes)
        .analyzer("ios_safari_bookmarks", safari_bookmarks)
        .analyzer("ios_safari_tabs", safari_tabs)
        .analyzer("ios_keyboard", keyboard)
        .analyzer("ios_voicemail", voicemail)
        .analyzer("ios_wifi_networks", wifi_networks)
        .analyzer("ios_bluetooth", bluetooth)
        .analyzer("ios_screen_time", screen_time)
        .analyzer("ios_selected_databases", selected_databases)
        .analyzer("ios_selected_plist", selected_plist)
}

export_plugin!(plugin);
