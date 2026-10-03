//! The Tethering domain defines methods and events for browser port binding.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Request browser port binding.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tethering.bind")]
pub struct BindParams {
    /// Port number to bind.
    pub port: i64,
}
/// Request browser port unbinding.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tethering.unbind")]
pub struct UnbindParams {
    /// Port number to unbind.
    pub port: i64,
}
/// Informs that port was successfully bound and got a specified connection id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tethering.accepted")]
pub struct Accepted<'a> {
    /// Port number that was successfully bound.
    pub port: i64,
    /// Connection id to be used.
    #[serde(rename = "connectionId")]
    pub connection_id: Cow<'a, str>,
}