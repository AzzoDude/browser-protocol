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
    pub fn builder() -> SamplingProfileNodeBuilder<'a> { SamplingProfileNodeBuilder::default() }
    pub fn size(&self) -> f64 { self.size }
    pub fn total(&self) -> f64 { self.total }
    pub fn stack(&self) -> &[Cow<'a, str>] { &self.stack }
}

#[derive(Default)]
pub struct SamplingProfileNodeBuilder<'a> {
    size: Option<f64>,
    total: Option<f64>,
    stack: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SamplingProfileNodeBuilder<'a> {
    /// Size of the sampled allocation.
    pub fn size(mut self, size: f64) -> Self { self.size = Some(size); self }
    /// Total bytes attributed to this sample.
    pub fn total(mut self, total: f64) -> Self { self.total = Some(total); self }
    /// Execution stack at the point of allocation.
    pub fn stack(mut self, stack: Vec<Cow<'a, str>>) -> Self { self.stack = Some(stack); self }
    pub fn build(self) -> SamplingProfileNode<'a> {
        SamplingProfileNode {
            size: self.size.unwrap_or_default(),
            total: self.total.unwrap_or_default(),
            stack: self.stack.unwrap_or_default(),
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
    pub fn builder() -> SamplingProfileBuilder<'a> { SamplingProfileBuilder::default() }
    pub fn samples(&self) -> &[SamplingProfileNode<'a>] { &self.samples }
    pub fn modules(&self) -> &[Module<'a>] { &self.modules }
}

#[derive(Default)]
pub struct SamplingProfileBuilder<'a> {
    samples: Option<Vec<SamplingProfileNode<'a>>>,
    modules: Option<Vec<Module<'a>>>,
}

impl<'a> SamplingProfileBuilder<'a> {
    pub fn samples(mut self, samples: Vec<SamplingProfileNode<'a>>) -> Self { self.samples = Some(samples); self }
    pub fn modules(mut self, modules: Vec<Module<'a>>) -> Self { self.modules = Some(modules); self }
    pub fn build(self) -> SamplingProfile<'a> {
        SamplingProfile {
            samples: self.samples.unwrap_or_default(),
            modules: self.modules.unwrap_or_default(),
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
    pub fn builder() -> ModuleBuilder<'a> { ModuleBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn uuid(&self) -> &str { self.uuid.as_ref() }
    pub fn baseAddress(&self) -> &str { self.baseAddress.as_ref() }
    pub fn size(&self) -> f64 { self.size }
}

#[derive(Default)]
pub struct ModuleBuilder<'a> {
    name: Option<Cow<'a, str>>,
    uuid: Option<Cow<'a, str>>,
    baseAddress: Option<Cow<'a, str>>,
    size: Option<f64>,
}

impl<'a> ModuleBuilder<'a> {
    /// Name of the module.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// UUID of the module.
    pub fn uuid(mut self, uuid: impl Into<Cow<'a, str>>) -> Self { self.uuid = Some(uuid.into()); self }
    /// Base address where the module is loaded into memory. Encoded as a decimal
    /// or hexadecimal (0x prefixed) string.
    pub fn baseAddress(mut self, baseAddress: impl Into<Cow<'a, str>>) -> Self { self.baseAddress = Some(baseAddress.into()); self }
    /// Size of the module in bytes.
    pub fn size(mut self, size: f64) -> Self { self.size = Some(size); self }
    pub fn build(self) -> Module<'a> {
        Module {
            name: self.name.unwrap_or_default(),
            uuid: self.uuid.unwrap_or_default(),
            baseAddress: self.baseAddress.unwrap_or_default(),
            size: self.size.unwrap_or_default(),
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
    pub fn builder() -> DOMCounterBuilder<'a> { DOMCounterBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn count(&self) -> u64 { self.count }
}

#[derive(Default)]
pub struct DOMCounterBuilder<'a> {
    name: Option<Cow<'a, str>>,
    count: Option<u64>,
}

impl<'a> DOMCounterBuilder<'a> {
    /// Object name. Note: object names should be presumed volatile and clients should not expect
    /// the returned names to be consistent across runs.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Object count.
    pub fn count(mut self, count: u64) -> Self { self.count = Some(count); self }
    pub fn build(self) -> DOMCounter<'a> {
        DOMCounter {
            name: self.name.unwrap_or_default(),
            count: self.count.unwrap_or_default(),
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
    pub fn builder() -> GetDOMCountersReturnsBuilder { GetDOMCountersReturnsBuilder::default() }
    pub fn documents(&self) -> i64 { self.documents }
    pub fn nodes(&self) -> i64 { self.nodes }
    pub fn jsEventListeners(&self) -> i64 { self.jsEventListeners }
}

#[derive(Default)]
pub struct GetDOMCountersReturnsBuilder {
    documents: Option<i64>,
    nodes: Option<i64>,
    jsEventListeners: Option<i64>,
}

impl GetDOMCountersReturnsBuilder {
    pub fn documents(mut self, documents: i64) -> Self { self.documents = Some(documents); self }
    pub fn nodes(mut self, nodes: i64) -> Self { self.nodes = Some(nodes); self }
    pub fn jsEventListeners(mut self, jsEventListeners: i64) -> Self { self.jsEventListeners = Some(jsEventListeners); self }
    pub fn build(self) -> GetDOMCountersReturns {
        GetDOMCountersReturns {
            documents: self.documents.unwrap_or_default(),
            nodes: self.nodes.unwrap_or_default(),
            jsEventListeners: self.jsEventListeners.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDOMCountersParams {}

impl GetDOMCountersParams {
    pub fn builder() -> GetDOMCountersParamsBuilder {
        GetDOMCountersParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetDOMCountersParamsBuilder {}

impl GetDOMCountersParamsBuilder {
    pub fn build(self) -> GetDOMCountersParams {
        GetDOMCountersParams {}
    }
}

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
    pub fn builder() -> GetDOMCountersForLeakDetectionReturnsBuilder<'a> { GetDOMCountersForLeakDetectionReturnsBuilder::default() }
    pub fn counters(&self) -> &[DOMCounter<'a>] { &self.counters }
}

#[derive(Default)]
pub struct GetDOMCountersForLeakDetectionReturnsBuilder<'a> {
    counters: Option<Vec<DOMCounter<'a>>>,
}

impl<'a> GetDOMCountersForLeakDetectionReturnsBuilder<'a> {
    /// DOM object counters.
    pub fn counters(mut self, counters: Vec<DOMCounter<'a>>) -> Self { self.counters = Some(counters); self }
    pub fn build(self) -> GetDOMCountersForLeakDetectionReturns<'a> {
        GetDOMCountersForLeakDetectionReturns {
            counters: self.counters.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDOMCountersForLeakDetectionParams {}

impl GetDOMCountersForLeakDetectionParams {
    pub fn builder() -> GetDOMCountersForLeakDetectionParamsBuilder {
        GetDOMCountersForLeakDetectionParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetDOMCountersForLeakDetectionParamsBuilder {}

impl GetDOMCountersForLeakDetectionParamsBuilder {
    pub fn build(self) -> GetDOMCountersForLeakDetectionParams {
        GetDOMCountersForLeakDetectionParams {}
    }
}

impl GetDOMCountersForLeakDetectionParams { pub const METHOD: &'static str = "Memory.getDOMCountersForLeakDetection"; }

impl<'a> crate::CdpCommand<'a> for GetDOMCountersForLeakDetectionParams {
    const METHOD: &'static str = "Memory.getDOMCountersForLeakDetection";
    type Response = GetDOMCountersForLeakDetectionReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrepareForLeakDetectionParams {}

impl PrepareForLeakDetectionParams {
    pub fn builder() -> PrepareForLeakDetectionParamsBuilder {
        PrepareForLeakDetectionParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct PrepareForLeakDetectionParamsBuilder {}

impl PrepareForLeakDetectionParamsBuilder {
    pub fn build(self) -> PrepareForLeakDetectionParams {
        PrepareForLeakDetectionParams {}
    }
}

impl PrepareForLeakDetectionParams { pub const METHOD: &'static str = "Memory.prepareForLeakDetection"; }

impl<'a> crate::CdpCommand<'a> for PrepareForLeakDetectionParams {
    const METHOD: &'static str = "Memory.prepareForLeakDetection";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForciblyPurgeJavaScriptMemoryParams {}

impl ForciblyPurgeJavaScriptMemoryParams {
    pub fn builder() -> ForciblyPurgeJavaScriptMemoryParamsBuilder {
        ForciblyPurgeJavaScriptMemoryParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ForciblyPurgeJavaScriptMemoryParamsBuilder {}

impl ForciblyPurgeJavaScriptMemoryParamsBuilder {
    pub fn build(self) -> ForciblyPurgeJavaScriptMemoryParams {
        ForciblyPurgeJavaScriptMemoryParams {}
    }
}

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
    pub fn builder() -> SetPressureNotificationsSuppressedParamsBuilder { SetPressureNotificationsSuppressedParamsBuilder::default() }
    pub fn suppressed(&self) -> bool { self.suppressed }
}

#[derive(Default)]
pub struct SetPressureNotificationsSuppressedParamsBuilder {
    suppressed: Option<bool>,
}

impl SetPressureNotificationsSuppressedParamsBuilder {
    /// If true, memory pressure notifications will be suppressed.
    pub fn suppressed(mut self, suppressed: bool) -> Self { self.suppressed = Some(suppressed); self }
    pub fn build(self) -> SetPressureNotificationsSuppressedParams {
        SetPressureNotificationsSuppressedParams {
            suppressed: self.suppressed.unwrap_or_default(),
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
    pub fn builder() -> SimulatePressureNotificationParamsBuilder { SimulatePressureNotificationParamsBuilder::default() }
    pub fn level(&self) -> &PressureLevel { &self.level }
}

#[derive(Default)]
pub struct SimulatePressureNotificationParamsBuilder {
    level: Option<PressureLevel>,
}

impl SimulatePressureNotificationParamsBuilder {
    /// Memory pressure level of the notification.
    pub fn level(mut self, level: PressureLevel) -> Self { self.level = Some(level); self }
    pub fn build(self) -> SimulatePressureNotificationParams {
        SimulatePressureNotificationParams {
            level: self.level.unwrap_or_default(),
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
    pub fn builder() -> StartSamplingParamsBuilder { StartSamplingParamsBuilder::default() }
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

impl StopSamplingParams {
    pub fn builder() -> StopSamplingParamsBuilder {
        StopSamplingParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct StopSamplingParamsBuilder {}

impl StopSamplingParamsBuilder {
    pub fn build(self) -> StopSamplingParams {
        StopSamplingParams {}
    }
}

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
    pub fn builder() -> GetAllTimeSamplingProfileReturnsBuilder<'a> { GetAllTimeSamplingProfileReturnsBuilder::default() }
    pub fn profile(&self) -> &SamplingProfile<'a> { &self.profile }
}

#[derive(Default)]
pub struct GetAllTimeSamplingProfileReturnsBuilder<'a> {
    profile: Option<SamplingProfile<'a>>,
}

impl<'a> GetAllTimeSamplingProfileReturnsBuilder<'a> {
    pub fn profile(mut self, profile: SamplingProfile<'a>) -> Self { self.profile = Some(profile); self }
    pub fn build(self) -> GetAllTimeSamplingProfileReturns<'a> {
        GetAllTimeSamplingProfileReturns {
            profile: self.profile.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAllTimeSamplingProfileParams {}

impl GetAllTimeSamplingProfileParams {
    pub fn builder() -> GetAllTimeSamplingProfileParamsBuilder {
        GetAllTimeSamplingProfileParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetAllTimeSamplingProfileParamsBuilder {}

impl GetAllTimeSamplingProfileParamsBuilder {
    pub fn build(self) -> GetAllTimeSamplingProfileParams {
        GetAllTimeSamplingProfileParams {}
    }
}

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
    pub fn builder() -> GetBrowserSamplingProfileReturnsBuilder<'a> { GetBrowserSamplingProfileReturnsBuilder::default() }
    pub fn profile(&self) -> &SamplingProfile<'a> { &self.profile }
}

#[derive(Default)]
pub struct GetBrowserSamplingProfileReturnsBuilder<'a> {
    profile: Option<SamplingProfile<'a>>,
}

impl<'a> GetBrowserSamplingProfileReturnsBuilder<'a> {
    pub fn profile(mut self, profile: SamplingProfile<'a>) -> Self { self.profile = Some(profile); self }
    pub fn build(self) -> GetBrowserSamplingProfileReturns<'a> {
        GetBrowserSamplingProfileReturns {
            profile: self.profile.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBrowserSamplingProfileParams {}

impl GetBrowserSamplingProfileParams {
    pub fn builder() -> GetBrowserSamplingProfileParamsBuilder {
        GetBrowserSamplingProfileParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetBrowserSamplingProfileParamsBuilder {}

impl GetBrowserSamplingProfileParamsBuilder {
    pub fn build(self) -> GetBrowserSamplingProfileParams {
        GetBrowserSamplingProfileParams {}
    }
}

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
    pub fn builder() -> GetSamplingProfileReturnsBuilder<'a> { GetSamplingProfileReturnsBuilder::default() }
    pub fn profile(&self) -> &SamplingProfile<'a> { &self.profile }
}

#[derive(Default)]
pub struct GetSamplingProfileReturnsBuilder<'a> {
    profile: Option<SamplingProfile<'a>>,
}

impl<'a> GetSamplingProfileReturnsBuilder<'a> {
    pub fn profile(mut self, profile: SamplingProfile<'a>) -> Self { self.profile = Some(profile); self }
    pub fn build(self) -> GetSamplingProfileReturns<'a> {
        GetSamplingProfileReturns {
            profile: self.profile.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSamplingProfileParams {}

impl GetSamplingProfileParams {
    pub fn builder() -> GetSamplingProfileParamsBuilder {
        GetSamplingProfileParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetSamplingProfileParamsBuilder {}

impl GetSamplingProfileParamsBuilder {
    pub fn build(self) -> GetSamplingProfileParams {
        GetSamplingProfileParams {}
    }
}

impl GetSamplingProfileParams { pub const METHOD: &'static str = "Memory.getSamplingProfile"; }

impl<'a> crate::CdpCommand<'a> for GetSamplingProfileParams {
    const METHOD: &'static str = "Memory.getSamplingProfile";
    type Response = GetSamplingProfileReturns<'a>;
}
