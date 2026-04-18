use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! This domain exposes the current state of the CrashReportContext API.

/// Key-value pair in CrashReportContext.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrashReportContextEntry {

    pub key: String,

    pub value: String,
    /// The ID of the frame where the key-value pair was set.

    pub frameId: crate::page::FrameId,
}

/// Returns all entries in the CrashReportContext across all frames in the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEntriesReturns {

    pub entries: Vec<CrashReportContextEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEntriesParams {}

impl GetEntriesParams { pub const METHOD: &'static str = "CrashReportContext.getEntries"; }

impl crate::CdpCommand for GetEntriesParams {
    const METHOD: &'static str = "CrashReportContext.getEntries";
    type Response = GetEntriesReturns;
}
