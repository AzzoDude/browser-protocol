use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Device request id.

pub type RequestId<'a> = Cow<'a, str>;

/// A device id.

pub type DeviceId<'a> = Cow<'a, str>;

/// Device information displayed in a user prompt to select a device.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PromptDevice<'a> {
    id: DeviceId<'a>,
    /// Display name as it appears in a device request user prompt.
    name: Cow<'a, str>,
}

impl<'a> PromptDevice<'a> {
    pub fn builder() -> PromptDeviceBuilder<'a> { PromptDeviceBuilder::default() }
    pub fn id(&self) -> &DeviceId<'a> { &self.id }
    pub fn name(&self) -> &str { self.name.as_ref() }
}

#[derive(Default)]
pub struct PromptDeviceBuilder<'a> {
    id: Option<DeviceId<'a>>,
    name: Option<Cow<'a, str>>,
}

impl<'a> PromptDeviceBuilder<'a> {
    pub fn id(mut self, id: DeviceId<'a>) -> Self { self.id = Some(id); self }
    /// Display name as it appears in a device request user prompt.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> PromptDevice<'a> {
        PromptDevice {
            id: self.id.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
        }
    }
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

impl EnableParams { pub const METHOD: &'static str = "DeviceAccess.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "DeviceAccess.enable";
    type Response = crate::EmptyReturns;
}

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

impl DisableParams { pub const METHOD: &'static str = "DeviceAccess.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "DeviceAccess.disable";
    type Response = crate::EmptyReturns;
}

/// Select a device in response to a DeviceAccess.deviceRequestPrompted event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectPromptParams<'a> {
    id: RequestId<'a>,
    deviceId: DeviceId<'a>,
}

impl<'a> SelectPromptParams<'a> {
    pub fn builder() -> SelectPromptParamsBuilder<'a> { SelectPromptParamsBuilder::default() }
    pub fn id(&self) -> &RequestId<'a> { &self.id }
    pub fn deviceId(&self) -> &DeviceId<'a> { &self.deviceId }
}

#[derive(Default)]
pub struct SelectPromptParamsBuilder<'a> {
    id: Option<RequestId<'a>>,
    deviceId: Option<DeviceId<'a>>,
}

impl<'a> SelectPromptParamsBuilder<'a> {
    pub fn id(mut self, id: RequestId<'a>) -> Self { self.id = Some(id); self }
    pub fn deviceId(mut self, deviceId: DeviceId<'a>) -> Self { self.deviceId = Some(deviceId); self }
    pub fn build(self) -> SelectPromptParams<'a> {
        SelectPromptParams {
            id: self.id.unwrap_or_default(),
            deviceId: self.deviceId.unwrap_or_default(),
        }
    }
}

impl<'a> SelectPromptParams<'a> { pub const METHOD: &'static str = "DeviceAccess.selectPrompt"; }

impl<'a> crate::CdpCommand<'a> for SelectPromptParams<'a> {
    const METHOD: &'static str = "DeviceAccess.selectPrompt";
    type Response = crate::EmptyReturns;
}

/// Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelPromptParams<'a> {
    id: RequestId<'a>,
}

impl<'a> CancelPromptParams<'a> {
    pub fn builder() -> CancelPromptParamsBuilder<'a> { CancelPromptParamsBuilder::default() }
    pub fn id(&self) -> &RequestId<'a> { &self.id }
}

#[derive(Default)]
pub struct CancelPromptParamsBuilder<'a> {
    id: Option<RequestId<'a>>,
}

impl<'a> CancelPromptParamsBuilder<'a> {
    pub fn id(mut self, id: RequestId<'a>) -> Self { self.id = Some(id); self }
    pub fn build(self) -> CancelPromptParams<'a> {
        CancelPromptParams {
            id: self.id.unwrap_or_default(),
        }
    }
}

impl<'a> CancelPromptParams<'a> { pub const METHOD: &'static str = "DeviceAccess.cancelPrompt"; }

impl<'a> crate::CdpCommand<'a> for CancelPromptParams<'a> {
    const METHOD: &'static str = "DeviceAccess.cancelPrompt";
    type Response = crate::EmptyReturns;
}
