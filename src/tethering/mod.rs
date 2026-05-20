//! The Tethering domain defines methods and events for browser port binding.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Request browser port binding.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindParams {
    /// Port number to bind.
    port: i64,
}

impl BindParams {
    pub fn builder() -> BindParamsBuilder { BindParamsBuilder::default() }
    pub fn port(&self) -> i64 { self.port }
}

#[derive(Default)]
pub struct BindParamsBuilder {
    port: Option<i64>,
}

impl BindParamsBuilder {
    /// Port number to bind.
    pub fn port(mut self, port: i64) -> Self { self.port = Some(port); self }
    pub fn build(self) -> BindParams {
        BindParams {
            port: self.port.unwrap_or_default(),
        }
    }
}

impl BindParams { pub const METHOD: &'static str = "Tethering.bind"; }

impl<'a> crate::CdpCommand<'a> for BindParams {
    const METHOD: &'static str = "Tethering.bind";
    type Response = crate::EmptyReturns;
}

/// Request browser port unbinding.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnbindParams {
    /// Port number to unbind.
    port: i64,
}

impl UnbindParams {
    pub fn builder() -> UnbindParamsBuilder { UnbindParamsBuilder::default() }
    pub fn port(&self) -> i64 { self.port }
}

#[derive(Default)]
pub struct UnbindParamsBuilder {
    port: Option<i64>,
}

impl UnbindParamsBuilder {
    /// Port number to unbind.
    pub fn port(mut self, port: i64) -> Self { self.port = Some(port); self }
    pub fn build(self) -> UnbindParams {
        UnbindParams {
            port: self.port.unwrap_or_default(),
        }
    }
}

impl UnbindParams { pub const METHOD: &'static str = "Tethering.unbind"; }

impl<'a> crate::CdpCommand<'a> for UnbindParams {
    const METHOD: &'static str = "Tethering.unbind";
    type Response = crate::EmptyReturns;
}
