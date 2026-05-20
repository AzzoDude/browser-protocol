//! Provides access to log entries.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Log entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry<'a> {
    /// Log entry source.
    source: Cow<'a, str>,
    /// Log entry severity.
    level: Cow<'a, str>,
    /// Logged text.
    text: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<Cow<'a, str>>,
    /// Timestamp when this entry was added.
    timestamp: crate::runtime::Timestamp,
    /// URL of the resource if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// Line number in the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    lineNumber: Option<i64>,
    /// JavaScript stack trace.
    #[serde(skip_serializing_if = "Option::is_none")]
    stackTrace: Option<crate::runtime::StackTrace>,
    /// Identifier of the network request associated with this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    networkRequestId: Option<crate::network::RequestId<'a>>,
    /// Identifier of the worker associated with this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    workerId: Option<Cow<'a, str>>,
    /// Call arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<Vec<crate::runtime::RemoteObject>>,
}

impl<'a> LogEntry<'a> {
    pub fn builder() -> LogEntryBuilder<'a> { LogEntryBuilder::default() }
    pub fn source(&self) -> &str { self.source.as_ref() }
    pub fn level(&self) -> &str { self.level.as_ref() }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn category(&self) -> Option<&str> { self.category.as_deref() }
    pub fn timestamp(&self) -> &crate::runtime::Timestamp { &self.timestamp }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn lineNumber(&self) -> Option<i64> { self.lineNumber }
    pub fn stackTrace(&self) -> Option<&crate::runtime::StackTrace> { self.stackTrace.as_ref() }
    pub fn networkRequestId(&self) -> Option<&crate::network::RequestId<'a>> { self.networkRequestId.as_ref() }
    pub fn workerId(&self) -> Option<&str> { self.workerId.as_deref() }
    pub fn args(&self) -> Option<&[crate::runtime::RemoteObject]> { self.args.as_deref() }
}

#[derive(Default)]
pub struct LogEntryBuilder<'a> {
    source: Option<Cow<'a, str>>,
    level: Option<Cow<'a, str>>,
    text: Option<Cow<'a, str>>,
    category: Option<Cow<'a, str>>,
    timestamp: Option<crate::runtime::Timestamp>,
    url: Option<Cow<'a, str>>,
    lineNumber: Option<i64>,
    stackTrace: Option<crate::runtime::StackTrace>,
    networkRequestId: Option<crate::network::RequestId<'a>>,
    workerId: Option<Cow<'a, str>>,
    args: Option<Vec<crate::runtime::RemoteObject>>,
}

impl<'a> LogEntryBuilder<'a> {
    /// Log entry source.
    pub fn source(mut self, source: impl Into<Cow<'a, str>>) -> Self { self.source = Some(source.into()); self }
    /// Log entry severity.
    pub fn level(mut self, level: impl Into<Cow<'a, str>>) -> Self { self.level = Some(level.into()); self }
    /// Logged text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn category(mut self, category: impl Into<Cow<'a, str>>) -> Self { self.category = Some(category.into()); self }
    /// Timestamp when this entry was added.
    pub fn timestamp(mut self, timestamp: crate::runtime::Timestamp) -> Self { self.timestamp = Some(timestamp); self }
    /// URL of the resource if known.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Line number in the resource.
    pub fn lineNumber(mut self, lineNumber: i64) -> Self { self.lineNumber = Some(lineNumber); self }
    /// JavaScript stack trace.
    pub fn stackTrace(mut self, stackTrace: crate::runtime::StackTrace) -> Self { self.stackTrace = Some(stackTrace); self }
    /// Identifier of the network request associated with this entry.
    pub fn networkRequestId(mut self, networkRequestId: crate::network::RequestId<'a>) -> Self { self.networkRequestId = Some(networkRequestId); self }
    /// Identifier of the worker associated with this entry.
    pub fn workerId(mut self, workerId: impl Into<Cow<'a, str>>) -> Self { self.workerId = Some(workerId.into()); self }
    /// Call arguments.
    pub fn args(mut self, args: Vec<crate::runtime::RemoteObject>) -> Self { self.args = Some(args); self }
    pub fn build(self) -> LogEntry<'a> {
        LogEntry {
            source: self.source.unwrap_or_default(),
            level: self.level.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
            category: self.category,
            timestamp: self.timestamp.unwrap_or_default(),
            url: self.url,
            lineNumber: self.lineNumber,
            stackTrace: self.stackTrace,
            networkRequestId: self.networkRequestId,
            workerId: self.workerId,
            args: self.args,
        }
    }
}

/// Violation configuration setting.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViolationSetting<'a> {
    /// Violation type.
    name: Cow<'a, str>,
    /// Time threshold to trigger upon.
    threshold: f64,
}

impl<'a> ViolationSetting<'a> {
    pub fn builder() -> ViolationSettingBuilder<'a> { ViolationSettingBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn threshold(&self) -> f64 { self.threshold }
}

#[derive(Default)]
pub struct ViolationSettingBuilder<'a> {
    name: Option<Cow<'a, str>>,
    threshold: Option<f64>,
}

impl<'a> ViolationSettingBuilder<'a> {
    /// Violation type.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Time threshold to trigger upon.
    pub fn threshold(mut self, threshold: f64) -> Self { self.threshold = Some(threshold); self }
    pub fn build(self) -> ViolationSetting<'a> {
        ViolationSetting {
            name: self.name.unwrap_or_default(),
            threshold: self.threshold.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearParams {}

impl ClearParams {
    pub fn builder() -> ClearParamsBuilder {
        ClearParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ClearParamsBuilder {}

impl ClearParamsBuilder {
    pub fn build(self) -> ClearParams {
        ClearParams {}
    }
}

impl ClearParams { pub const METHOD: &'static str = "Log.clear"; }

impl<'a> crate::CdpCommand<'a> for ClearParams {
    const METHOD: &'static str = "Log.clear";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams {
    pub fn builder() -> DisableParamsBuilder {
        DisableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct DisableParamsBuilder {}

impl DisableParamsBuilder {
    pub fn build(self) -> DisableParams {
        DisableParams {}
    }
}

impl DisableParams { pub const METHOD: &'static str = "Log.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Log.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct EnableParamsBuilder {}

impl EnableParamsBuilder {
    pub fn build(self) -> EnableParams {
        EnableParams {}
    }
}

impl EnableParams { pub const METHOD: &'static str = "Log.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Log.enable";
    type Response = crate::EmptyReturns;
}

/// start violation reporting.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartViolationsReportParams<'a> {
    /// Configuration for violations.
    config: Vec<ViolationSetting<'a>>,
}

impl<'a> StartViolationsReportParams<'a> {
    pub fn builder() -> StartViolationsReportParamsBuilder<'a> { StartViolationsReportParamsBuilder::default() }
    pub fn config(&self) -> &[ViolationSetting<'a>] { &self.config }
}

#[derive(Default)]
pub struct StartViolationsReportParamsBuilder<'a> {
    config: Option<Vec<ViolationSetting<'a>>>,
}

impl<'a> StartViolationsReportParamsBuilder<'a> {
    /// Configuration for violations.
    pub fn config(mut self, config: Vec<ViolationSetting<'a>>) -> Self { self.config = Some(config); self }
    pub fn build(self) -> StartViolationsReportParams<'a> {
        StartViolationsReportParams {
            config: self.config.unwrap_or_default(),
        }
    }
}

impl<'a> StartViolationsReportParams<'a> { pub const METHOD: &'static str = "Log.startViolationsReport"; }

impl<'a> crate::CdpCommand<'a> for StartViolationsReportParams<'a> {
    const METHOD: &'static str = "Log.startViolationsReport";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopViolationsReportParams {}

impl StopViolationsReportParams {
    pub fn builder() -> StopViolationsReportParamsBuilder {
        StopViolationsReportParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct StopViolationsReportParamsBuilder {}

impl StopViolationsReportParamsBuilder {
    pub fn build(self) -> StopViolationsReportParams {
        StopViolationsReportParams {}
    }
}

impl StopViolationsReportParams { pub const METHOD: &'static str = "Log.stopViolationsReport"; }

impl<'a> crate::CdpCommand<'a> for StopViolationsReportParams {
    const METHOD: &'static str = "Log.stopViolationsReport";
    type Response = crate::EmptyReturns;
}
