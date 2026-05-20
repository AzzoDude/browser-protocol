use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDeviceOrientationOverrideParams {}

impl ClearDeviceOrientationOverrideParams {
    pub fn builder() -> ClearDeviceOrientationOverrideParamsBuilder {
        ClearDeviceOrientationOverrideParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ClearDeviceOrientationOverrideParamsBuilder {}

impl ClearDeviceOrientationOverrideParamsBuilder {
    pub fn build(self) -> ClearDeviceOrientationOverrideParams {
        ClearDeviceOrientationOverrideParams {}
    }
}

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
    pub fn builder() -> SetDeviceOrientationOverrideParamsBuilder { SetDeviceOrientationOverrideParamsBuilder::default() }
    pub fn alpha(&self) -> f64 { self.alpha }
    pub fn beta(&self) -> f64 { self.beta }
    pub fn gamma(&self) -> f64 { self.gamma }
}

#[derive(Default)]
pub struct SetDeviceOrientationOverrideParamsBuilder {
    alpha: Option<f64>,
    beta: Option<f64>,
    gamma: Option<f64>,
}

impl SetDeviceOrientationOverrideParamsBuilder {
    /// Mock alpha
    pub fn alpha(mut self, alpha: f64) -> Self { self.alpha = Some(alpha); self }
    /// Mock beta
    pub fn beta(mut self, beta: f64) -> Self { self.beta = Some(beta); self }
    /// Mock gamma
    pub fn gamma(mut self, gamma: f64) -> Self { self.gamma = Some(gamma); self }
    pub fn build(self) -> SetDeviceOrientationOverrideParams {
        SetDeviceOrientationOverrideParams {
            alpha: self.alpha.unwrap_or_default(),
            beta: self.beta.unwrap_or_default(),
            gamma: self.gamma.unwrap_or_default(),
        }
    }
}

impl SetDeviceOrientationOverrideParams { pub const METHOD: &'static str = "DeviceOrientation.setDeviceOrientationOverride"; }

impl<'a> crate::CdpCommand<'a> for SetDeviceOrientationOverrideParams {
    const METHOD: &'static str = "DeviceOrientation.setDeviceOrientationOverride";
    type Response = crate::EmptyReturns;
}
