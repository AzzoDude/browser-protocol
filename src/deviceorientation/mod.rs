use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Clears the overridden Device Orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DeviceOrientation.clearDeviceOrientationOverride")]
pub struct ClearDeviceOrientationOverrideParams {

}
/// Overrides the Device Orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DeviceOrientation.setDeviceOrientationOverride")]
pub struct SetDeviceOrientationOverrideParams {
    /// Mock alpha
    pub alpha: f64,
    /// Mock beta
    pub beta: f64,
    /// Mock gamma
    pub gamma: f64,
}