use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Memory pressure level.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PressureLevel {
    #[default]
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "critical")]
    Critical,
}

/// Heap profile sample.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingProfileNode<'a> {
    /// Size of the sampled allocation.
    size: f64,
    /// Total bytes attributed to this sample.
    total: f64,
    /// Execution stack at the point of allocation.
    stack: Vec<Cow<'a, str>>,
}

impl<'a> SamplingProfileNode<'a> {
    pub fn builder(size: f64, total: f64, stack: Vec<Cow<'a, str>>) -> SamplingProfileNodeBuilder<'a> {
        SamplingProfileNodeBuilder {
            size: size,
            total: total,
            stack: stack,
        }
    }
    pub fn size(&self) -> f64 { self.size }
    pub fn total(&self) -> f64 { self.total }
    pub fn stack(&self) -> &[Cow<'a, str>] { &self.stack }
}


pub struct SamplingProfileNodeBuilder<'a> {
    size: f64,
    total: f64,
    stack: Vec<Cow<'a, str>>,
}

impl<'a> SamplingProfileNodeBuilder<'a> {
    pub fn build(self) -> SamplingProfileNode<'a> {
        SamplingProfileNode {
            size: self.size,
            total: self.total,
            stack: self.stack,
        }
    }
}

/// Array of heap profile samples.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingProfile<'a> {
    samples: Vec<SamplingProfileNode<'a>>,
    modules: Vec<Module<'a>>,
}

impl<'a> SamplingProfile<'a> {
    pub fn builder(samples: Vec<SamplingProfileNode<'a>>, modules: Vec<Module<'a>>) -> SamplingProfileBuilder<'a> {
        SamplingProfileBuilder {
            samples: samples,
            modules: modules,
        }
    }
    pub fn samples(&self) -> &[SamplingProfileNode<'a>] { &self.samples }
    pub fn modules(&self) -> &[Module<'a>] { &self.modules }
}


pub struct SamplingProfileBuilder<'a> {
    samples: Vec<SamplingProfileNode<'a>>,
    modules: Vec<Module<'a>>,
}

impl<'a> SamplingProfileBuilder<'a> {
    pub fn build(self) -> SamplingProfile<'a> {
        SamplingProfile {
            samples: self.samples,
            modules: self.modules,
        }
    }
}

/// Executable module information

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Module<'a> {
    /// Name of the module.
    name: Cow<'a, str>,
    /// UUID of the module.
    uuid: Cow<'a, str>,
    /// Base address where the module is loaded into memory. Encoded as a decimal
    /// or hexadecimal (0x prefixed) string.
    baseAddress: Cow<'a, str>,
    /// Size of the module in bytes.
    size: f64,
}

impl<'a> Module<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, uuid: impl Into<Cow<'a, str>>, baseAddress: impl Into<Cow<'a, str>>, size: f64) -> ModuleBuilder<'a> {
        ModuleBuilder {
            name: name.into(),
            uuid: uuid.into(),
            baseAddress: baseAddress.into(),
            size: size,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn uuid(&self) -> &str { self.uuid.as_ref() }
    pub fn baseAddress(&self) -> &str { self.baseAddress.as_ref() }
    pub fn size(&self) -> f64 { self.size }
}


pub struct ModuleBuilder<'a> {
    name: Cow<'a, str>,
    uuid: Cow<'a, str>,
    baseAddress: Cow<'a, str>,
    size: f64,
}

impl<'a> ModuleBuilder<'a> {
    pub fn build(self) -> Module<'a> {
        Module {
            name: self.name,
            uuid: self.uuid,
            baseAddress: self.baseAddress,
            size: self.size,
        }
    }
}

/// DOM object counter data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DOMCounter<'a> {
    /// Object name. Note: object names should be presumed volatile and clients should not expect
    /// the returned names to be consistent across runs.
    name: Cow<'a, str>,
    /// Object count.
    count: u64,
}

impl<'a> DOMCounter<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, count: u64) -> DOMCounterBuilder<'a> {
        DOMCounterBuilder {
            name: name.into(),
            count: count,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn count(&self) -> u64 { self.count }
}


pub struct DOMCounterBuilder<'a> {
    name: Cow<'a, str>,
    count: u64,
}

impl<'a> DOMCounterBuilder<'a> {
    pub fn build(self) -> DOMCounter<'a> {
        DOMCounter {
            name: self.name,
            count: self.count,
        }
    }
}

/// Retruns current DOM object counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMCountersReturns {
    documents: i64,
    nodes: i64,
    jsEventListeners: i64,
}

impl GetDOMCountersReturns {
    pub fn builder(documents: i64, nodes: i64, jsEventListeners: i64) -> GetDOMCountersReturnsBuilder {
        GetDOMCountersReturnsBuilder {
            documents: documents,
            nodes: nodes,
            jsEventListeners: jsEventListeners,
        }
    }
    pub fn documents(&self) -> i64 { self.documents }
    pub fn nodes(&self) -> i64 { self.nodes }
    pub fn jsEventListeners(&self) -> i64 { self.jsEventListeners }
}


pub struct GetDOMCountersReturnsBuilder {
    documents: i64,
    nodes: i64,
    jsEventListeners: i64,
}

impl GetDOMCountersReturnsBuilder {
    pub fn build(self) -> GetDOMCountersReturns {
        GetDOMCountersReturns {
            documents: self.documents,
            nodes: self.nodes,
            jsEventListeners: self.jsEventListeners,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDOMCountersParams {}

impl GetDOMCountersParams { pub const METHOD: &'static str = "Memory.getDOMCounters"; }

impl<'a> crate::CdpCommand<'a> for GetDOMCountersParams {
    const METHOD: &'static str = "Memory.getDOMCounters";
    type Response = GetDOMCountersReturns;
}

/// Retruns DOM object counters after preparing renderer for leak detection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMCountersForLeakDetectionReturns<'a> {
    /// DOM object counters.
    counters: Vec<DOMCounter<'a>>,
}

impl<'a> GetDOMCountersForLeakDetectionReturns<'a> {
    pub fn builder(counters: Vec<DOMCounter<'a>>) -> GetDOMCountersForLeakDetectionReturnsBuilder<'a> {
        GetDOMCountersForLeakDetectionReturnsBuilder {
            counters: counters,
        }
    }
    pub fn counters(&self) -> &[DOMCounter<'a>] { &self.counters }
}


pub struct GetDOMCountersForLeakDetectionReturnsBuilder<'a> {
    counters: Vec<DOMCounter<'a>>,
}

impl<'a> GetDOMCountersForLeakDetectionReturnsBuilder<'a> {
    pub fn build(self) -> GetDOMCountersForLeakDetectionReturns<'a> {
        GetDOMCountersForLeakDetectionReturns {
            counters: self.counters,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDOMCountersForLeakDetectionParams {}

impl GetDOMCountersForLeakDetectionParams { pub const METHOD: &'static str = "Memory.getDOMCountersForLeakDetection"; }

impl<'a> crate::CdpCommand<'a> for GetDOMCountersForLeakDetectionParams {
    const METHOD: &'static str = "Memory.getDOMCountersForLeakDetection";
    type Response = GetDOMCountersForLeakDetectionReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrepareForLeakDetectionParams {}

impl PrepareForLeakDetectionParams { pub const METHOD: &'static str = "Memory.prepareForLeakDetection"; }

impl<'a> crate::CdpCommand<'a> for PrepareForLeakDetectionParams {
    const METHOD: &'static str = "Memory.prepareForLeakDetection";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForciblyPurgeJavaScriptMemoryParams {}

impl ForciblyPurgeJavaScriptMemoryParams { pub const METHOD: &'static str = "Memory.forciblyPurgeJavaScriptMemory"; }

impl<'a> crate::CdpCommand<'a> for ForciblyPurgeJavaScriptMemoryParams {
    const METHOD: &'static str = "Memory.forciblyPurgeJavaScriptMemory";
    type Response = crate::EmptyReturns;
}

/// Enable/disable suppressing memory pressure notifications in all processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureNotificationsSuppressedParams {
    /// If true, memory pressure notifications will be suppressed.
    suppressed: bool,
}

impl SetPressureNotificationsSuppressedParams {
    pub fn builder(suppressed: bool) -> SetPressureNotificationsSuppressedParamsBuilder {
        SetPressureNotificationsSuppressedParamsBuilder {
            suppressed: suppressed,
        }
    }
    pub fn suppressed(&self) -> bool { self.suppressed }
}


pub struct SetPressureNotificationsSuppressedParamsBuilder {
    suppressed: bool,
}

impl SetPressureNotificationsSuppressedParamsBuilder {
    pub fn build(self) -> SetPressureNotificationsSuppressedParams {
        SetPressureNotificationsSuppressedParams {
            suppressed: self.suppressed,
        }
    }
}

impl SetPressureNotificationsSuppressedParams { pub const METHOD: &'static str = "Memory.setPressureNotificationsSuppressed"; }

impl<'a> crate::CdpCommand<'a> for SetPressureNotificationsSuppressedParams {
    const METHOD: &'static str = "Memory.setPressureNotificationsSuppressed";
    type Response = crate::EmptyReturns;
}

/// Simulate a memory pressure notification in all processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePressureNotificationParams {
    /// Memory pressure level of the notification.
    level: PressureLevel,
}

impl SimulatePressureNotificationParams {
    pub fn builder(level: PressureLevel) -> SimulatePressureNotificationParamsBuilder {
        SimulatePressureNotificationParamsBuilder {
            level: level,
        }
    }
    pub fn level(&self) -> &PressureLevel { &self.level }
}


pub struct SimulatePressureNotificationParamsBuilder {
    level: PressureLevel,
}

impl SimulatePressureNotificationParamsBuilder {
    pub fn build(self) -> SimulatePressureNotificationParams {
        SimulatePressureNotificationParams {
            level: self.level,
        }
    }
}

impl SimulatePressureNotificationParams { pub const METHOD: &'static str = "Memory.simulatePressureNotification"; }

impl<'a> crate::CdpCommand<'a> for SimulatePressureNotificationParams {
    const METHOD: &'static str = "Memory.simulatePressureNotification";
    type Response = crate::EmptyReturns;
}

/// Start collecting native memory profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartSamplingParams {
    /// Average number of bytes between samples.
    #[serde(skip_serializing_if = "Option::is_none")]
    samplingInterval: Option<i64>,
    /// Do not randomize intervals between samples.
    #[serde(skip_serializing_if = "Option::is_none")]
    suppressRandomness: Option<bool>,
}

impl StartSamplingParams {
    pub fn builder() -> StartSamplingParamsBuilder {
        StartSamplingParamsBuilder {
            samplingInterval: None,
            suppressRandomness: None,
        }
    }
    pub fn samplingInterval(&self) -> Option<i64> { self.samplingInterval }
    pub fn suppressRandomness(&self) -> Option<bool> { self.suppressRandomness }
}

#[derive(Default)]
pub struct StartSamplingParamsBuilder {
    samplingInterval: Option<i64>,
    suppressRandomness: Option<bool>,
}

impl StartSamplingParamsBuilder {
    /// Average number of bytes between samples.
    pub fn samplingInterval(mut self, samplingInterval: i64) -> Self { self.samplingInterval = Some(samplingInterval); self }
    /// Do not randomize intervals between samples.
    pub fn suppressRandomness(mut self, suppressRandomness: bool) -> Self { self.suppressRandomness = Some(suppressRandomness); self }
    pub fn build(self) -> StartSamplingParams {
        StartSamplingParams {
            samplingInterval: self.samplingInterval,
            suppressRandomness: self.suppressRandomness,
        }
    }
}

impl StartSamplingParams { pub const METHOD: &'static str = "Memory.startSampling"; }

impl<'a> crate::CdpCommand<'a> for StartSamplingParams {
    const METHOD: &'static str = "Memory.startSampling";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopSamplingParams {}

impl StopSamplingParams { pub const METHOD: &'static str = "Memory.stopSampling"; }

impl<'a> crate::CdpCommand<'a> for StopSamplingParams {
    const METHOD: &'static str = "Memory.stopSampling";
    type Response = crate::EmptyReturns;
}

/// Retrieve native memory allocations profile
/// collected since renderer process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTimeSamplingProfileReturns<'a> {
    profile: SamplingProfile<'a>,
}

impl<'a> GetAllTimeSamplingProfileReturns<'a> {
    pub fn builder(profile: SamplingProfile<'a>) -> GetAllTimeSamplingProfileReturnsBuilder<'a> {
        GetAllTimeSamplingProfileReturnsBuilder {
            profile: profile,
        }
    }
    pub fn profile(&self) -> &SamplingProfile<'a> { &self.profile }
}


pub struct GetAllTimeSamplingProfileReturnsBuilder<'a> {
    profile: SamplingProfile<'a>,
}

impl<'a> GetAllTimeSamplingProfileReturnsBuilder<'a> {
    pub fn build(self) -> GetAllTimeSamplingProfileReturns<'a> {
        GetAllTimeSamplingProfileReturns {
            profile: self.profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAllTimeSamplingProfileParams {}

impl GetAllTimeSamplingProfileParams { pub const METHOD: &'static str = "Memory.getAllTimeSamplingProfile"; }

impl<'a> crate::CdpCommand<'a> for GetAllTimeSamplingProfileParams {
    const METHOD: &'static str = "Memory.getAllTimeSamplingProfile";
    type Response = GetAllTimeSamplingProfileReturns<'a>;
}

/// Retrieve native memory allocations profile
/// collected since browser process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserSamplingProfileReturns<'a> {
    profile: SamplingProfile<'a>,
}

impl<'a> GetBrowserSamplingProfileReturns<'a> {
    pub fn builder(profile: SamplingProfile<'a>) -> GetBrowserSamplingProfileReturnsBuilder<'a> {
        GetBrowserSamplingProfileReturnsBuilder {
            profile: profile,
        }
    }
    pub fn profile(&self) -> &SamplingProfile<'a> { &self.profile }
}


pub struct GetBrowserSamplingProfileReturnsBuilder<'a> {
    profile: SamplingProfile<'a>,
}

impl<'a> GetBrowserSamplingProfileReturnsBuilder<'a> {
    pub fn build(self) -> GetBrowserSamplingProfileReturns<'a> {
        GetBrowserSamplingProfileReturns {
            profile: self.profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBrowserSamplingProfileParams {}

impl GetBrowserSamplingProfileParams { pub const METHOD: &'static str = "Memory.getBrowserSamplingProfile"; }

impl<'a> crate::CdpCommand<'a> for GetBrowserSamplingProfileParams {
    const METHOD: &'static str = "Memory.getBrowserSamplingProfile";
    type Response = GetBrowserSamplingProfileReturns<'a>;
}

/// Retrieve native memory allocations profile collected since last
/// 'startSampling' call.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfileReturns<'a> {
    profile: SamplingProfile<'a>,
}

impl<'a> GetSamplingProfileReturns<'a> {
    pub fn builder(profile: SamplingProfile<'a>) -> GetSamplingProfileReturnsBuilder<'a> {
        GetSamplingProfileReturnsBuilder {
            profile: profile,
        }
    }
    pub fn profile(&self) -> &SamplingProfile<'a> { &self.profile }
}


pub struct GetSamplingProfileReturnsBuilder<'a> {
    profile: SamplingProfile<'a>,
}

impl<'a> GetSamplingProfileReturnsBuilder<'a> {
    pub fn build(self) -> GetSamplingProfileReturns<'a> {
        GetSamplingProfileReturns {
            profile: self.profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSamplingProfileParams {}

impl GetSamplingProfileParams { pub const METHOD: &'static str = "Memory.getSamplingProfile"; }

impl<'a> crate::CdpCommand<'a> for GetSamplingProfileParams {
    const METHOD: &'static str = "Memory.getSamplingProfile";
    type Response = GetSamplingProfileReturns<'a>;
}
