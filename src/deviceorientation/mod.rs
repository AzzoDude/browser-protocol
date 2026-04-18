use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDeviceOrientationOverrideParams {}

impl ClearDeviceOrientationOverrideParams { pub const METHOD: &'static str = "DeviceOrientation.clearDeviceOrientationOverride"; }

impl crate::CdpCommand for ClearDeviceOrientationOverrideParams {
    const METHOD: &'static str = "DeviceOrientation.clearDeviceOrientationOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides the Device Orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceOrientationOverrideParams {
    /// Mock alpha

    pub alpha: f64,
    /// Mock beta

    pub beta: f64,
    /// Mock gamma

    pub gamma: f64,
}

impl SetDeviceOrientationOverrideParams { pub const METHOD: &'static str = "DeviceOrientation.setDeviceOrientationOverride"; }

impl crate::CdpCommand for SetDeviceOrientationOverrideParams {
    const METHOD: &'static str = "DeviceOrientation.setDeviceOrientationOverride";
    type Response = crate::EmptyReturns;
}
