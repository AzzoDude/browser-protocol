//! Provides access to log entries.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Log entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry<'a> {
    /// Log entry source.
    pub source: Cow<'a, str>,
    /// Log entry severity.
    pub level: Cow<'a, str>,
    /// Logged text.
    pub text: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Cow<'a, str>>,
    /// Timestamp when this entry was added.
    pub timestamp: crate::runtime::Timestamp,
    /// URL of the resource if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// Line number in the resource.
    #[serde(skip_serializing_if = "Option::is_none", rename = "lineNumber")]
    pub line_number: Option<i64>,
    /// JavaScript stack trace.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackTrace")]
    pub stack_trace: Option<crate::runtime::StackTrace>,
    /// Identifier of the network request associated with this entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "networkRequestId")]
    pub network_request_id: Option<crate::network::RequestId<'a>>,
    /// Identifier of the worker associated with this entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "workerId")]
    pub worker_id: Option<Cow<'a, str>>,
    /// Call arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<crate::runtime::RemoteObject>>,
}
/// Violation configuration setting.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ViolationSetting<'a> {
    /// Violation type.
    pub name: Cow<'a, str>,
    /// Time threshold to trigger upon.
    pub threshold: f64,
}
/// Clears the log.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Log.clear")]
pub struct ClearParams {

}
/// Disables log domain, prevents further log entries from being reported to the client.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Log.disable")]
pub struct DisableParams {

}
/// Enables log domain, sends the entries collected so far to the client by means of the
/// 'entryAdded' notification.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Log.enable")]
pub struct EnableParams {

}
/// start violation reporting.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Log.startViolationsReport")]
pub struct StartViolationsReportParams<'a> {
    /// Configuration for violations.
    pub config: Vec<ViolationSetting<'a>>,
}
/// Stop violation reporting.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Log.stopViolationsReport")]
pub struct StopViolationsReportParams {

}
/// Issued when new message was logged.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Log.entryAdded")]
pub struct EntryAdded<'a> {
    /// The entry.
    pub entry: LogEntry<'a>,
}