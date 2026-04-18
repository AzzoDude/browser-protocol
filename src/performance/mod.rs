use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Run-time execution metric.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    /// Metric name.

    pub name: String,
    /// Metric value.

    pub value: f64,
}

/// Enable collecting and reporting metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Time domain to use for collecting and reporting duration metrics.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeDomain: Option<String>,
}

/// Sets time domain to use for collecting and reporting duration metrics.
/// Note that this must be called before enabling metrics collection. Calling
/// this method while metrics collection is enabled returns an error.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTimeDomainParams {
    /// Time domain

    pub timeDomain: String,
}

/// Retrieve current values of run-time metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMetricsReturns {
    /// Current values for run-time metrics.

    pub metrics: Vec<Metric>,
}
