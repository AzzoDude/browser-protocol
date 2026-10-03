use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Run-time execution metric.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Metric<'a> {
    /// Metric name.
    pub name: Cow<'a, str>,
    /// Metric value.
    pub value: f64,
}
/// Disable collecting and reporting metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Performance.disable")]
pub struct DisableParams {

}
/// Enable collecting and reporting metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Performance.enable")]
pub struct EnableParams<'a> {
    /// Time domain to use for collecting and reporting duration metrics.
    #[serde(skip_serializing_if = "Option::is_none", rename = "timeDomain")]
    pub time_domain: Option<Cow<'a, str>>,
}
/// Sets time domain to use for collecting and reporting duration metrics.
/// Note that this must be called before enabling metrics collection. Calling
/// this method while metrics collection is enabled returns an error.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Performance.setTimeDomain")]
pub struct SetTimeDomainParams<'a> {
    /// Time domain
    #[serde(rename = "timeDomain")]
    pub time_domain: Cow<'a, str>,
}
/// Retrieve current values of run-time metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Performance.getMetrics", response = "GetMetricsReturns<'a>")]
pub struct GetMetricsParams {

}
/// Retrieve current values of run-time metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetMetricsReturns<'a> {
    /// Current values for run-time metrics.
    pub metrics: Vec<Metric<'a>>,
}
/// Current values of the metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Performance.metrics")]
pub struct Metrics<'a> {
    /// Current values of the metrics.
    pub metrics: Vec<Metric<'a>>,
    /// Timestamp title.
    pub title: Cow<'a, str>,
}