use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDeviceOrientationOverrideParams {}

impl ClearDeviceOrientationOverrideParams { pub const METHOD: &'static str = "DeviceOrientation.clearDeviceOrientationOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearDeviceOrientationOverrideParams {
    const METHOD: &'static str = "DeviceOrientation.clearDeviceOrientationOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides the Device Orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceOrientationOverrideParams {
    /// Mock alpha
    alpha: f64,
    /// Mock beta
    beta: f64,
    /// Mock gamma
    gamma: f64,
}

impl SetDeviceOrientationOverrideParams {
    /// Creates a builder for this type with the required parameters:
    /// * `alpha`: Mock alpha
    /// * `beta`: Mock beta
    /// * `gamma`: Mock gamma
    pub fn builder(alpha: f64, beta: f64, gamma: f64) -> SetDeviceOrientationOverrideParamsBuilder {
        SetDeviceOrientationOverrideParamsBuilder {
            alpha: alpha,
            beta: beta,
            gamma: gamma,
        }
    }
    /// Mock alpha
    pub fn alpha(&self) -> f64 { self.alpha }
    /// Mock beta
    pub fn beta(&self) -> f64 { self.beta }
    /// Mock gamma
    pub fn gamma(&self) -> f64 { self.gamma }
}


pub struct SetDeviceOrientationOverrideParamsBuilder {
    alpha: f64,
    beta: f64,
    gamma: f64,
}

impl SetDeviceOrientationOverrideParamsBuilder {
    pub fn build(self) -> SetDeviceOrientationOverrideParams {
        SetDeviceOrientationOverrideParams {
            alpha: self.alpha,
            beta: self.beta,
            gamma: self.gamma,
        }
    }
}

impl SetDeviceOrientationOverrideParams { pub const METHOD: &'static str = "DeviceOrientation.setDeviceOrientationOverride"; }

impl<'a> crate::CdpCommand<'a> for SetDeviceOrientationOverrideParams {
    const METHOD: &'static str = "DeviceOrientation.setDeviceOrientationOverride";
    type Response = crate::EmptyReturns;
}
