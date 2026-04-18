use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Inspector.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "Inspector.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Inspector.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "Inspector.enable";
    type Response = crate::EmptyReturns;
}
