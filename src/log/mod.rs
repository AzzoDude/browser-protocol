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
    #[serde(skip_serializing_if = "Option::is_none", rename = "lineNumber")]
    line_number: Option<i64>,
    /// JavaScript stack trace.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackTrace")]
    stack_trace: Option<crate::runtime::StackTrace>,
    /// Identifier of the network request associated with this entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "networkRequestId")]
    network_request_id: Option<crate::network::RequestId<'a>>,
    /// Identifier of the worker associated with this entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "workerId")]
    worker_id: Option<Cow<'a, str>>,
    /// Call arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<Vec<crate::runtime::RemoteObject>>,
}

impl<'a> LogEntry<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `source`: Log entry source.
    /// * `level`: Log entry severity.
    /// * `text`: Logged text.
    /// * `timestamp`: Timestamp when this entry was added.
    pub fn builder(source: impl Into<Cow<'a, str>>, level: impl Into<Cow<'a, str>>, text: impl Into<Cow<'a, str>>, timestamp: crate::runtime::Timestamp) -> LogEntryBuilder<'a> {
        LogEntryBuilder {
            source: source.into(),
            level: level.into(),
            text: text.into(),
            category: None,
            timestamp: timestamp,
            url: None,
            line_number: None,
            stack_trace: None,
            network_request_id: None,
            worker_id: None,
            args: None,
        }
    }
    /// Log entry source.
    pub fn source(&self) -> &str { self.source.as_ref() }
    /// Log entry severity.
    pub fn level(&self) -> &str { self.level.as_ref() }
    /// Logged text.
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn category(&self) -> Option<&str> { self.category.as_deref() }
    /// Timestamp when this entry was added.
    pub fn timestamp(&self) -> &crate::runtime::Timestamp { &self.timestamp }
    /// URL of the resource if known.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// Line number in the resource.
    pub fn line_number(&self) -> Option<i64> { self.line_number }
    /// JavaScript stack trace.
    pub fn stack_trace(&self) -> Option<&crate::runtime::StackTrace> { self.stack_trace.as_ref() }
    /// Identifier of the network request associated with this entry.
    pub fn network_request_id(&self) -> Option<&crate::network::RequestId<'a>> { self.network_request_id.as_ref() }
    /// Identifier of the worker associated with this entry.
    pub fn worker_id(&self) -> Option<&str> { self.worker_id.as_deref() }
    /// Call arguments.
    pub fn args(&self) -> Option<&[crate::runtime::RemoteObject]> { self.args.as_deref() }
}


pub struct LogEntryBuilder<'a> {
    source: Cow<'a, str>,
    level: Cow<'a, str>,
    text: Cow<'a, str>,
    category: Option<Cow<'a, str>>,
    timestamp: crate::runtime::Timestamp,
    url: Option<Cow<'a, str>>,
    line_number: Option<i64>,
    stack_trace: Option<crate::runtime::StackTrace>,
    network_request_id: Option<crate::network::RequestId<'a>>,
    worker_id: Option<Cow<'a, str>>,
    args: Option<Vec<crate::runtime::RemoteObject>>,
}

impl<'a> LogEntryBuilder<'a> {
    pub fn category(mut self, category: impl Into<Cow<'a, str>>) -> Self { self.category = Some(category.into()); self }
    /// URL of the resource if known.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Line number in the resource.
    pub fn line_number(mut self, line_number: i64) -> Self { self.line_number = Some(line_number); self }
    /// JavaScript stack trace.
    pub fn stack_trace(mut self, stack_trace: crate::runtime::StackTrace) -> Self { self.stack_trace = Some(stack_trace); self }
    /// Identifier of the network request associated with this entry.
    pub fn network_request_id(mut self, network_request_id: crate::network::RequestId<'a>) -> Self { self.network_request_id = Some(network_request_id); self }
    /// Identifier of the worker associated with this entry.
    pub fn worker_id(mut self, worker_id: impl Into<Cow<'a, str>>) -> Self { self.worker_id = Some(worker_id.into()); self }
    /// Call arguments.
    pub fn args(mut self, args: Vec<crate::runtime::RemoteObject>) -> Self { self.args = Some(args); self }
    pub fn build(self) -> LogEntry<'a> {
        LogEntry {
            source: self.source,
            level: self.level,
            text: self.text,
            category: self.category,
            timestamp: self.timestamp,
            url: self.url,
            line_number: self.line_number,
            stack_trace: self.stack_trace,
            network_request_id: self.network_request_id,
            worker_id: self.worker_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Violation type.
    /// * `threshold`: Time threshold to trigger upon.
    pub fn builder(name: impl Into<Cow<'a, str>>, threshold: f64) -> ViolationSettingBuilder<'a> {
        ViolationSettingBuilder {
            name: name.into(),
            threshold: threshold,
        }
    }
    /// Violation type.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Time threshold to trigger upon.
    pub fn threshold(&self) -> f64 { self.threshold }
}


pub struct ViolationSettingBuilder<'a> {
    name: Cow<'a, str>,
    threshold: f64,
}

impl<'a> ViolationSettingBuilder<'a> {
    pub fn build(self) -> ViolationSetting<'a> {
        ViolationSetting {
            name: self.name,
            threshold: self.threshold,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearParams {}

impl ClearParams { pub const METHOD: &'static str = "Log.clear"; }

impl<'a> crate::CdpCommand<'a> for ClearParams {
    const METHOD: &'static str = "Log.clear";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Log.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Log.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

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
    /// Creates a builder for this type with the required parameters:
    /// * `config`: Configuration for violations.
    pub fn builder(config: Vec<ViolationSetting<'a>>) -> StartViolationsReportParamsBuilder<'a> {
        StartViolationsReportParamsBuilder {
            config: config,
        }
    }
    /// Configuration for violations.
    pub fn config(&self) -> &[ViolationSetting<'a>] { &self.config }
}


pub struct StartViolationsReportParamsBuilder<'a> {
    config: Vec<ViolationSetting<'a>>,
}

impl<'a> StartViolationsReportParamsBuilder<'a> {
    pub fn build(self) -> StartViolationsReportParams<'a> {
        StartViolationsReportParams {
            config: self.config,
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

impl StopViolationsReportParams { pub const METHOD: &'static str = "Log.stopViolationsReport"; }

impl<'a> crate::CdpCommand<'a> for StopViolationsReportParams {
    const METHOD: &'static str = "Log.stopViolationsReport";
    type Response = crate::EmptyReturns;
}
