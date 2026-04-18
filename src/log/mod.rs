//! Provides access to log entries.

use serde::{Serialize, Deserialize};

/// Log entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    /// Log entry source.

    pub source: String,
    /// Log entry severity.

    pub level: String,
    /// Logged text.

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Timestamp when this entry was added.

    pub timestamp: crate::runtime::Timestamp,
    /// URL of the resource if known.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Line number in the resource.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineNumber: Option<i64>,
    /// JavaScript stack trace.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackTrace: Option<crate::runtime::StackTrace>,
    /// Identifier of the network request associated with this entry.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub networkRequestId: Option<crate::network::RequestId>,
    /// Identifier of the worker associated with this entry.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workerId: Option<String>,
    /// Call arguments.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<crate::runtime::RemoteObject>>,
}

/// Violation configuration setting.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViolationSetting {
    /// Violation type.

    pub name: String,
    /// Time threshold to trigger upon.

    pub threshold: f64,
}

/// start violation reporting.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartViolationsReportParams {
    /// Configuration for violations.

    pub config: Vec<ViolationSetting>,
}
