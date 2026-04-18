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

/// Retruns DOM object counters after preparing renderer for leak detection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMCountersForLeakDetectionReturns {
    /// DOM object counters.

    pub counters: Vec<DOMCounter>,
}

/// Enable/disable suppressing memory pressure notifications in all processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureNotificationsSuppressedParams {
    /// If true, memory pressure notifications will be suppressed.

    pub suppressed: bool,
}

/// Simulate a memory pressure notification in all processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePressureNotificationParams {
    /// Memory pressure level of the notification.

    pub level: PressureLevel,
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

/// Retrieve native memory allocations profile
/// collected since renderer process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTimeSamplingProfileReturns {

    pub profile: SamplingProfile,
}

/// Retrieve native memory allocations profile
/// collected since browser process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserSamplingProfileReturns {

    pub profile: SamplingProfile,
}

/// Retrieve native memory allocations profile collected since last
/// 'startSampling' call.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfileReturns {

    pub profile: SamplingProfile,
}
