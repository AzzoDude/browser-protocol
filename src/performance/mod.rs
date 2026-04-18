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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Performance.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "Performance.disable";
    type Response = crate::EmptyReturns;
}

/// Enable collecting and reporting metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Time domain to use for collecting and reporting duration metrics.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeDomain: Option<String>,
}

impl EnableParams { pub const METHOD: &'static str = "Performance.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "Performance.enable";
    type Response = crate::EmptyReturns;
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

impl SetTimeDomainParams { pub const METHOD: &'static str = "Performance.setTimeDomain"; }

impl crate::CdpCommand for SetTimeDomainParams {
    const METHOD: &'static str = "Performance.setTimeDomain";
    type Response = crate::EmptyReturns;
}

/// Retrieve current values of run-time metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMetricsReturns {
    /// Current values for run-time metrics.

    pub metrics: Vec<Metric>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMetricsParams {}

impl GetMetricsParams { pub const METHOD: &'static str = "Performance.getMetrics"; }

impl crate::CdpCommand for GetMetricsParams {
    const METHOD: &'static str = "Performance.getMetrics";
    type Response = GetMetricsReturns;
}
