use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! The Tethering domain defines methods and events for browser port binding.

/// Request browser port binding.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindParams {
    /// Port number to bind.

    pub port: i64,
}

impl BindParams { pub const METHOD: &'static str = "Tethering.bind"; }

impl crate::CdpCommand for BindParams {
    const METHOD: &'static str = "Tethering.bind";
    type Response = crate::EmptyReturns;
}

/// Request browser port unbinding.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnbindParams {
    /// Port number to unbind.

    pub port: i64,
}

impl UnbindParams { pub const METHOD: &'static str = "Tethering.unbind"; }

impl crate::CdpCommand for UnbindParams {
    const METHOD: &'static str = "Tethering.unbind";
    type Response = crate::EmptyReturns;
}
