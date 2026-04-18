use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Memory pressure level.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PressureLevel {
    #[default]
    Moderate,
    Critical,
}

/// Heap profile sample.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingProfileNode {
    /// Size of the sampled allocation.

    pub size: f64,
    /// Total bytes attributed to this sample.

    pub total: f64,
    /// Execution stack at the point of allocation.

    pub stack: Vec<String>,
}

/// Array of heap profile samples.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingProfile {

    pub samples: Vec<SamplingProfileNode>,

    pub modules: Vec<Module>,
}

/// Executable module information

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    /// Name of the module.

    pub name: String,
    /// UUID of the module.

    pub uuid: String,
    /// Base address where the module is loaded into memory. Encoded as a decimal
    /// or hexadecimal (0x prefixed) string.

    pub baseAddress: String,
    /// Size of the module in bytes.

    pub size: f64,
}

/// DOM object counter data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DOMCounter {
    /// Object name. Note: object names should be presumed volatile and clients should not expect
    /// the returned names to be consistent across runs.

    pub name: String,
    /// Object count.

    pub count: u64,
}

/// Retruns current DOM object counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMCountersReturns {

    pub documents: i64,

    pub nodes: i64,

    pub jsEventListeners: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDOMCountersParams {}

impl GetDOMCountersParams { pub const METHOD: &'static str = "Memory.getDOMCounters"; }

impl crate::CdpCommand for GetDOMCountersParams {
    const METHOD: &'static str = "Memory.getDOMCounters";
    type Response = GetDOMCountersReturns;
}

/// Retruns DOM object counters after preparing renderer for leak detection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMCountersForLeakDetectionReturns {
    /// DOM object counters.

    pub counters: Vec<DOMCounter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDOMCountersForLeakDetectionParams {}

impl GetDOMCountersForLeakDetectionParams { pub const METHOD: &'static str = "Memory.getDOMCountersForLeakDetection"; }

impl crate::CdpCommand for GetDOMCountersForLeakDetectionParams {
    const METHOD: &'static str = "Memory.getDOMCountersForLeakDetection";
    type Response = GetDOMCountersForLeakDetectionReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrepareForLeakDetectionParams {}

impl PrepareForLeakDetectionParams { pub const METHOD: &'static str = "Memory.prepareForLeakDetection"; }

impl crate::CdpCommand for PrepareForLeakDetectionParams {
    const METHOD: &'static str = "Memory.prepareForLeakDetection";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForciblyPurgeJavaScriptMemoryParams {}

impl ForciblyPurgeJavaScriptMemoryParams { pub const METHOD: &'static str = "Memory.forciblyPurgeJavaScriptMemory"; }

impl crate::CdpCommand for ForciblyPurgeJavaScriptMemoryParams {
    const METHOD: &'static str = "Memory.forciblyPurgeJavaScriptMemory";
    type Response = crate::EmptyReturns;
}

/// Enable/disable suppressing memory pressure notifications in all processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureNotificationsSuppressedParams {
    /// If true, memory pressure notifications will be suppressed.

    pub suppressed: bool,
}

impl SetPressureNotificationsSuppressedParams { pub const METHOD: &'static str = "Memory.setPressureNotificationsSuppressed"; }

impl crate::CdpCommand for SetPressureNotificationsSuppressedParams {
    const METHOD: &'static str = "Memory.setPressureNotificationsSuppressed";
    type Response = crate::EmptyReturns;
}

/// Simulate a memory pressure notification in all processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePressureNotificationParams {
    /// Memory pressure level of the notification.

    pub level: PressureLevel,
}

impl SimulatePressureNotificationParams { pub const METHOD: &'static str = "Memory.simulatePressureNotification"; }

impl crate::CdpCommand for SimulatePressureNotificationParams {
    const METHOD: &'static str = "Memory.simulatePressureNotification";
    type Response = crate::EmptyReturns;
}

/// Start collecting native memory profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartSamplingParams {
    /// Average number of bytes between samples.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samplingInterval: Option<i64>,
    /// Do not randomize intervals between samples.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressRandomness: Option<bool>,
}

impl StartSamplingParams { pub const METHOD: &'static str = "Memory.startSampling"; }

impl crate::CdpCommand for StartSamplingParams {
    const METHOD: &'static str = "Memory.startSampling";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopSamplingParams {}

impl StopSamplingParams { pub const METHOD: &'static str = "Memory.stopSampling"; }

impl crate::CdpCommand for StopSamplingParams {
    const METHOD: &'static str = "Memory.stopSampling";
    type Response = crate::EmptyReturns;
}

/// Retrieve native memory allocations profile
/// collected since renderer process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTimeSamplingProfileReturns {

    pub profile: SamplingProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAllTimeSamplingProfileParams {}

impl GetAllTimeSamplingProfileParams { pub const METHOD: &'static str = "Memory.getAllTimeSamplingProfile"; }

impl crate::CdpCommand for GetAllTimeSamplingProfileParams {
    const METHOD: &'static str = "Memory.getAllTimeSamplingProfile";
    type Response = GetAllTimeSamplingProfileReturns;
}

/// Retrieve native memory allocations profile
/// collected since browser process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserSamplingProfileReturns {

    pub profile: SamplingProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBrowserSamplingProfileParams {}

impl GetBrowserSamplingProfileParams { pub const METHOD: &'static str = "Memory.getBrowserSamplingProfile"; }

impl crate::CdpCommand for GetBrowserSamplingProfileParams {
    const METHOD: &'static str = "Memory.getBrowserSamplingProfile";
    type Response = GetBrowserSamplingProfileReturns;
}

/// Retrieve native memory allocations profile collected since last
/// 'startSampling' call.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfileReturns {

    pub profile: SamplingProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSamplingProfileParams {}

impl GetSamplingProfileParams { pub const METHOD: &'static str = "Memory.getSamplingProfile"; }

impl crate::CdpCommand for GetSamplingProfileParams {
    const METHOD: &'static str = "Memory.getSamplingProfile";
    type Response = GetSamplingProfileReturns;
}
