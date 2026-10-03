use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SamplingProfileNode<'a> {
    /// Size of the sampled allocation.
    pub size: f64,
    /// Total bytes attributed to this sample.
    pub total: f64,
    /// Execution stack at the point of allocation.
    pub stack: Vec<Cow<'a, str>>,
}
/// Array of heap profile samples.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SamplingProfile<'a> {
    pub samples: Vec<SamplingProfileNode<'a>>,
    pub modules: Vec<Module<'a>>,
}
/// Executable module information

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Module<'a> {
    /// Name of the module.
    pub name: Cow<'a, str>,
    /// UUID of the module.
    pub uuid: Cow<'a, str>,
    /// Base address where the module is loaded into memory. Encoded as a decimal
    /// or hexadecimal (0x prefixed) string.
    #[serde(rename = "baseAddress")]
    pub base_address: Cow<'a, str>,
    /// Size of the module in bytes.
    pub size: f64,
}
/// DOM object counter data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DOMCounter<'a> {
    /// Object name. Note: object names should be presumed volatile and clients should not expect
    /// the returned names to be consistent across runs.
    pub name: Cow<'a, str>,
    /// Object count.
    pub count: u64,
}
/// Retruns current DOM object counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.getDOMCounters", response = "GetDOMCountersReturns")]
pub struct GetDOMCountersParams {

}
/// Retruns current DOM object counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMCountersReturns {
    pub documents: i64,
    pub nodes: i64,
    #[serde(rename = "jsEventListeners")]
    pub js_event_listeners: i64,
}
/// Retruns DOM object counters after preparing renderer for leak detection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.getDOMCountersForLeakDetection", response = "GetDOMCountersForLeakDetectionReturns<'a>")]
pub struct GetDOMCountersForLeakDetectionParams {

}
/// Retruns DOM object counters after preparing renderer for leak detection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMCountersForLeakDetectionReturns<'a> {
    /// DOM object counters.
    pub counters: Vec<DOMCounter<'a>>,
}
/// Prepares for leak detection by terminating workers, stopping spellcheckers,
/// dropping non-essential internal caches, running garbage collections, etc.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.prepareForLeakDetection")]
pub struct PrepareForLeakDetectionParams {

}
/// Simulate OomIntervention by purging V8 memory.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.forciblyPurgeJavaScriptMemory")]
pub struct ForciblyPurgeJavaScriptMemoryParams {

}
/// Enable/disable suppressing memory pressure notifications in all processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.setPressureNotificationsSuppressed")]
pub struct SetPressureNotificationsSuppressedParams {
    /// If true, memory pressure notifications will be suppressed.
    pub suppressed: bool,
}
/// Simulate a memory pressure notification in all processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.simulatePressureNotification")]
pub struct SimulatePressureNotificationParams {
    /// Memory pressure level of the notification.
    pub level: PressureLevel,
}
/// Start collecting native memory profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.startSampling")]
pub struct StartSamplingParams {
    /// Average number of bytes between samples.
    #[serde(skip_serializing_if = "Option::is_none", rename = "samplingInterval")]
    pub sampling_interval: Option<i64>,
    /// Do not randomize intervals between samples.
    #[serde(skip_serializing_if = "Option::is_none", rename = "suppressRandomness")]
    pub suppress_randomness: Option<bool>,
}
/// Stop collecting native memory profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.stopSampling")]
pub struct StopSamplingParams {

}
/// Retrieve native memory allocations profile
/// collected since renderer process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.getAllTimeSamplingProfile", response = "GetAllTimeSamplingProfileReturns<'a>")]
pub struct GetAllTimeSamplingProfileParams {

}
/// Retrieve native memory allocations profile
/// collected since renderer process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTimeSamplingProfileReturns<'a> {
    pub profile: SamplingProfile<'a>,
}
/// Retrieve native memory allocations profile
/// collected since browser process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.getBrowserSamplingProfile", response = "GetBrowserSamplingProfileReturns<'a>")]
pub struct GetBrowserSamplingProfileParams {

}
/// Retrieve native memory allocations profile
/// collected since browser process startup.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserSamplingProfileReturns<'a> {
    pub profile: SamplingProfile<'a>,
}
/// Retrieve native memory allocations profile collected since last
/// 'startSampling' call.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Memory.getSamplingProfile", response = "GetSamplingProfileReturns<'a>")]
pub struct GetSamplingProfileParams {

}
/// Retrieve native memory allocations profile collected since last
/// 'startSampling' call.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfileReturns<'a> {
    pub profile: SamplingProfile<'a>,
}