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
        .analyzer("ios_selected_databases", selected_databases)
        .analyzer("ios_selected_plist", selected_plist)
}

export_plugin!(plugin);
