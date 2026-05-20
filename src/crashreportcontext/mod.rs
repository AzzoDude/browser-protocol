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
    frameId: crate::page::FrameId<'a>,
}

impl<'a> CrashReportContextEntry<'a> {
    pub fn builder(key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>, frameId: crate::page::FrameId<'a>) -> CrashReportContextEntryBuilder<'a> {
        CrashReportContextEntryBuilder {
            key: key.into(),
            value: value.into(),
            frameId: frameId,
        }
    }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
}


pub struct CrashReportContextEntryBuilder<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
    frameId: crate::page::FrameId<'a>,
}

impl<'a> CrashReportContextEntryBuilder<'a> {
    pub fn build(self) -> CrashReportContextEntry<'a> {
        CrashReportContextEntry {
            key: self.key,
            value: self.value,
            frameId: self.frameId,
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
