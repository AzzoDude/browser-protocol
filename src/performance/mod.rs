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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Metric name.
    /// * `value`: Metric value.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: f64) -> MetricBuilder<'a> {
        MetricBuilder {
            name: name.into(),
            value: value,
        }
    }
    /// Metric name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Metric value.
    pub fn value(&self) -> f64 { self.value }
}


pub struct MetricBuilder<'a> {
    name: Cow<'a, str>,
    value: f64,
}

impl<'a> MetricBuilder<'a> {
    pub fn build(self) -> Metric<'a> {
        Metric {
            name: self.name,
            value: self.value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

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
    #[serde(skip_serializing_if = "Option::is_none", rename = "timeDomain")]
    time_domain: Option<Cow<'a, str>>,
}

impl<'a> EnableParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> EnableParamsBuilder<'a> {
        EnableParamsBuilder {
            time_domain: None,
        }
    }
    /// Time domain to use for collecting and reporting duration metrics.
    pub fn time_domain(&self) -> Option<&str> { self.time_domain.as_deref() }
}

#[derive(Default)]
pub struct EnableParamsBuilder<'a> {
    time_domain: Option<Cow<'a, str>>,
}

impl<'a> EnableParamsBuilder<'a> {
    /// Time domain to use for collecting and reporting duration metrics.
    pub fn time_domain(mut self, time_domain: impl Into<Cow<'a, str>>) -> Self { self.time_domain = Some(time_domain.into()); self }
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            time_domain: self.time_domain,
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
    #[serde(rename = "timeDomain")]
    time_domain: Cow<'a, str>,
}

impl<'a> SetTimeDomainParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `time_domain`: Time domain
    pub fn builder(time_domain: impl Into<Cow<'a, str>>) -> SetTimeDomainParamsBuilder<'a> {
        SetTimeDomainParamsBuilder {
            time_domain: time_domain.into(),
        }
    }
    /// Time domain
    pub fn time_domain(&self) -> &str { self.time_domain.as_ref() }
}


pub struct SetTimeDomainParamsBuilder<'a> {
    time_domain: Cow<'a, str>,
}

impl<'a> SetTimeDomainParamsBuilder<'a> {
    pub fn build(self) -> SetTimeDomainParams<'a> {
        SetTimeDomainParams {
            time_domain: self.time_domain,
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
    /// Creates a builder for this type with the required parameters:
    /// * `metrics`: Current values for run-time metrics.
    pub fn builder(metrics: Vec<Metric<'a>>) -> GetMetricsReturnsBuilder<'a> {
        GetMetricsReturnsBuilder {
            metrics: metrics,
        }
    }
    /// Current values for run-time metrics.
    pub fn metrics(&self) -> &[Metric<'a>] { &self.metrics }
}


pub struct GetMetricsReturnsBuilder<'a> {
    metrics: Vec<Metric<'a>>,
}

impl<'a> GetMetricsReturnsBuilder<'a> {
    pub fn build(self) -> GetMetricsReturns<'a> {
        GetMetricsReturns {
            metrics: self.metrics,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMetricsParams {}

impl GetMetricsParams { pub const METHOD: &'static str = "Performance.getMetrics"; }

impl<'a> crate::CdpCommand<'a> for GetMetricsParams {
    const METHOD: &'static str = "Performance.getMetrics";
    type Response = GetMetricsReturns<'a>;
}
