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
    pub fn builder() -> CrashReportContextEntryBuilder<'a> { CrashReportContextEntryBuilder::default() }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
}

#[derive(Default)]
pub struct CrashReportContextEntryBuilder<'a> {
    key: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> CrashReportContextEntryBuilder<'a> {
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// The ID of the frame where the key-value pair was set.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> CrashReportContextEntry<'a> {
        CrashReportContextEntry {
            key: self.key.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            frameId: self.frameId.unwrap_or_default(),
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
    pub fn builder() -> GetEntriesReturnsBuilder<'a> { GetEntriesReturnsBuilder::default() }
    pub fn entries(&self) -> &[CrashReportContextEntry<'a>] { &self.entries }
}

#[derive(Default)]
pub struct GetEntriesReturnsBuilder<'a> {
    entries: Option<Vec<CrashReportContextEntry<'a>>>,
}

impl<'a> GetEntriesReturnsBuilder<'a> {
    pub fn entries(mut self, entries: Vec<CrashReportContextEntry<'a>>) -> Self { self.entries = Some(entries); self }
    pub fn build(self) -> GetEntriesReturns<'a> {
        GetEntriesReturns {
            entries: self.entries.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEntriesParams {}

impl GetEntriesParams {
    pub fn builder() -> GetEntriesParamsBuilder {
        GetEntriesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetEntriesParamsBuilder {}

impl GetEntriesParamsBuilder {
    pub fn build(self) -> GetEntriesParams {
        GetEntriesParams {}
    }
}

impl GetEntriesParams { pub const METHOD: &'static str = "CrashReportContext.getEntries"; }

impl<'a> crate::CdpCommand<'a> for GetEntriesParams {
    const METHOD: &'static str = "CrashReportContext.getEntries";
    type Response = GetEntriesReturns<'a>;
}
