use serde::{Serialize, Deserialize};

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
