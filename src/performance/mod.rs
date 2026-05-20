use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Run-time execution metric.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Metric<'a> {
    /// Metric name.
    name: Cow<'a, str>,
    /// Metric value.
    value: f64,
}

impl<'a> Metric<'a> {
    pub fn builder() -> MetricBuilder<'a> { MetricBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> f64 { self.value }
}

#[derive(Default)]
pub struct MetricBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<f64>,
}

impl<'a> MetricBuilder<'a> {
    /// Metric name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Metric value.
    pub fn value(mut self, value: f64) -> Self { self.value = Some(value); self }
    pub fn build(self) -> Metric<'a> {
        Metric {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
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

impl DisableParams { pub const METHOD: &'static str = "Performance.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Performance.disable";
    type Response = crate::EmptyReturns;
}

/// Enable collecting and reporting metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams<'a> {
    /// Time domain to use for collecting and reporting duration metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    timeDomain: Option<Cow<'a, str>>,
}

impl<'a> EnableParams<'a> {
    pub fn builder() -> EnableParamsBuilder<'a> { EnableParamsBuilder::default() }
    pub fn timeDomain(&self) -> Option<&str> { self.timeDomain.as_deref() }
}

#[derive(Default)]
pub struct EnableParamsBuilder<'a> {
    timeDomain: Option<Cow<'a, str>>,
}

impl<'a> EnableParamsBuilder<'a> {
    /// Time domain to use for collecting and reporting duration metrics.
    pub fn timeDomain(mut self, timeDomain: impl Into<Cow<'a, str>>) -> Self { self.timeDomain = Some(timeDomain.into()); self }
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            timeDomain: self.timeDomain,
        }
    }
}

impl<'a> EnableParams<'a> { pub const METHOD: &'static str = "Performance.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams<'a> {
    const METHOD: &'static str = "Performance.enable";
    type Response = crate::EmptyReturns;
}

/// Sets time domain to use for collecting and reporting duration metrics.
/// Note that this must be called before enabling metrics collection. Calling
/// this method while metrics collection is enabled returns an error.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTimeDomainParams<'a> {
    /// Time domain
    timeDomain: Cow<'a, str>,
}

impl<'a> SetTimeDomainParams<'a> {
    pub fn builder() -> SetTimeDomainParamsBuilder<'a> { SetTimeDomainParamsBuilder::default() }
    pub fn timeDomain(&self) -> &str { self.timeDomain.as_ref() }
}

#[derive(Default)]
pub struct SetTimeDomainParamsBuilder<'a> {
    timeDomain: Option<Cow<'a, str>>,
}

impl<'a> SetTimeDomainParamsBuilder<'a> {
    /// Time domain
    pub fn timeDomain(mut self, timeDomain: impl Into<Cow<'a, str>>) -> Self { self.timeDomain = Some(timeDomain.into()); self }
    pub fn build(self) -> SetTimeDomainParams<'a> {
        SetTimeDomainParams {
            timeDomain: self.timeDomain.unwrap_or_default(),
        }
    }
}

impl<'a> SetTimeDomainParams<'a> { pub const METHOD: &'static str = "Performance.setTimeDomain"; }

impl<'a> crate::CdpCommand<'a> for SetTimeDomainParams<'a> {
    const METHOD: &'static str = "Performance.setTimeDomain";
    type Response = crate::EmptyReturns;
}

/// Retrieve current values of run-time metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMetricsReturns<'a> {
    /// Current values for run-time metrics.
    metrics: Vec<Metric<'a>>,
}

impl<'a> GetMetricsReturns<'a> {
    pub fn builder() -> GetMetricsReturnsBuilder<'a> { GetMetricsReturnsBuilder::default() }
    pub fn metrics(&self) -> &[Metric<'a>] { &self.metrics }
}

#[derive(Default)]
pub struct GetMetricsReturnsBuilder<'a> {
    metrics: Option<Vec<Metric<'a>>>,
}

impl<'a> GetMetricsReturnsBuilder<'a> {
    /// Current values for run-time metrics.
    pub fn metrics(mut self, metrics: Vec<Metric<'a>>) -> Self { self.metrics = Some(metrics); self }
    pub fn build(self) -> GetMetricsReturns<'a> {
        GetMetricsReturns {
            metrics: self.metrics.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMetricsParams {}

impl GetMetricsParams {
    pub fn builder() -> GetMetricsParamsBuilder {
        GetMetricsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetMetricsParamsBuilder {}

impl GetMetricsParamsBuilder {
    pub fn build(self) -> GetMetricsParams {
        GetMetricsParams {}
    }
}

impl GetMetricsParams { pub const METHOD: &'static str = "Performance.getMetrics"; }

impl<'a> crate::CdpCommand<'a> for GetMetricsParams {
    const METHOD: &'static str = "Performance.getMetrics";
    type Response = GetMetricsReturns<'a>;
}
