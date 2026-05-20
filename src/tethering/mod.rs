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
    /// Creates a builder for this type with the required parameters:
    /// * `port`: Port number to bind.
    pub fn builder(port: i64) -> BindParamsBuilder {
        BindParamsBuilder {
            port: port,
        }
    }
    /// Port number to bind.
    pub fn port(&self) -> i64 { self.port }
}


pub struct BindParamsBuilder {
    port: i64,
}

impl BindParamsBuilder {
    pub fn build(self) -> BindParams {
        BindParams {
            port: self.port,
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
    /// Creates a builder for this type with the required parameters:
    /// * `port`: Port number to unbind.
    pub fn builder(port: i64) -> UnbindParamsBuilder {
        UnbindParamsBuilder {
            port: port,
        }
    }
    /// Port number to unbind.
    pub fn port(&self) -> i64 { self.port }
}


pub struct UnbindParamsBuilder {
    port: i64,
}

impl UnbindParamsBuilder {
    pub fn build(self) -> UnbindParams {
        UnbindParams {
            port: self.port,
        }
    }
}

impl UnbindParams { pub const METHOD: &'static str = "Tethering.unbind"; }

impl<'a> crate::CdpCommand<'a> for UnbindParams {
    const METHOD: &'static str = "Tethering.unbind";
    type Response = crate::EmptyReturns;
}
