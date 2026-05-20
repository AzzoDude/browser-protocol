//! This domain exposes the current state of the CrashReportContext API.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Key-value pair in CrashReportContext.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrashReportContextEntry<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
    /// The ID of the frame where the key-value pair was set.
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
}

impl<'a> CrashReportContextEntry<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `key`: 
    /// * `value`: 
    /// * `frame_id`: The ID of the frame where the key-value pair was set.
    pub fn builder(key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>, frame_id: crate::page::FrameId<'a>) -> CrashReportContextEntryBuilder<'a> {
        CrashReportContextEntryBuilder {
            key: key.into(),
            value: value.into(),
            frame_id: frame_id,
        }
    }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    /// The ID of the frame where the key-value pair was set.
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
}


pub struct CrashReportContextEntryBuilder<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
    frame_id: crate::page::FrameId<'a>,
}

impl<'a> CrashReportContextEntryBuilder<'a> {
    pub fn build(self) -> CrashReportContextEntry<'a> {
        CrashReportContextEntry {
            key: self.key,
            value: self.value,
            frame_id: self.frame_id,
        }
    }
}

/// Returns all entries in the CrashReportContext across all frames in the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEntriesReturns<'a> {
    entries: Vec<CrashReportContextEntry<'a>>,
}

impl<'a> GetEntriesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `entries`: 
    pub fn builder(entries: Vec<CrashReportContextEntry<'a>>) -> GetEntriesReturnsBuilder<'a> {
        GetEntriesReturnsBuilder {
            entries: entries,
        }
    }
    pub fn entries(&self) -> &[CrashReportContextEntry<'a>] { &self.entries }
}


pub struct GetEntriesReturnsBuilder<'a> {
    entries: Vec<CrashReportContextEntry<'a>>,
}

impl<'a> GetEntriesReturnsBuilder<'a> {
    pub fn build(self) -> GetEntriesReturns<'a> {
        GetEntriesReturns {
            entries: self.entries,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEntriesParams {}

impl GetEntriesParams { pub const METHOD: &'static str = "CrashReportContext.getEntries"; }

impl<'a> crate::CdpCommand<'a> for GetEntriesParams {
    const METHOD: &'static str = "CrashReportContext.getEntries";
    type Response = GetEntriesReturns<'a>;
}
