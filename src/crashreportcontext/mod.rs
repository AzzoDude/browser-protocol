//! This domain exposes the current state of the CrashReportContext API.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Key-value pair in CrashReportContext.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CrashReportContextEntry<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
    /// The ID of the frame where the key-value pair was set.
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
}
/// Returns all entries in the CrashReportContext across all frames in the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CrashReportContext.getEntries", response = "GetEntriesReturns<'a>")]
pub struct GetEntriesParams {

}
/// Returns all entries in the CrashReportContext across all frames in the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetEntriesReturns<'a> {
    pub entries: Vec<CrashReportContextEntry<'a>>,
}