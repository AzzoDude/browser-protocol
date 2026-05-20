use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams {
    pub fn builder() -> DisableParamsBuilder {
        DisableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct DisableParamsBuilder {}

impl DisableParamsBuilder {
    pub fn build(self) -> DisableParams {
        DisableParams {}
    }
}

impl DisableParams { pub const METHOD: &'static str = "Inspector.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Inspector.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct EnableParamsBuilder {}

impl EnableParamsBuilder {
    pub fn build(self) -> EnableParams {
        EnableParams {}
    }
}

impl EnableParams { pub const METHOD: &'static str = "Inspector.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Inspector.enable";
    type Response = crate::EmptyReturns;
}
