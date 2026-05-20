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
    pub fn builder(id: impl Into<DeviceId<'a>>, name: impl Into<Cow<'a, str>>) -> PromptDeviceBuilder<'a> {
        PromptDeviceBuilder {
            id: id.into(),
            name: name.into(),
        }
    }
    pub fn id(&self) -> &DeviceId<'a> { &self.id }
    pub fn name(&self) -> &str { self.name.as_ref() }
}


pub struct PromptDeviceBuilder<'a> {
    id: DeviceId<'a>,
    name: Cow<'a, str>,
}

impl<'a> PromptDeviceBuilder<'a> {
    pub fn build(self) -> PromptDevice<'a> {
        PromptDevice {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "DeviceAccess.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "DeviceAccess.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

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
    pub fn builder(id: impl Into<RequestId<'a>>, deviceId: impl Into<DeviceId<'a>>) -> SelectPromptParamsBuilder<'a> {
        SelectPromptParamsBuilder {
            id: id.into(),
            deviceId: deviceId.into(),
        }
    }
    pub fn id(&self) -> &RequestId<'a> { &self.id }
    pub fn deviceId(&self) -> &DeviceId<'a> { &self.deviceId }
}


pub struct SelectPromptParamsBuilder<'a> {
    id: RequestId<'a>,
    deviceId: DeviceId<'a>,
}

impl<'a> SelectPromptParamsBuilder<'a> {
    pub fn build(self) -> SelectPromptParams<'a> {
        SelectPromptParams {
            id: self.id,
            deviceId: self.deviceId,
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
    pub fn builder(id: impl Into<RequestId<'a>>) -> CancelPromptParamsBuilder<'a> {
        CancelPromptParamsBuilder {
            id: id.into(),
        }
    }
    pub fn id(&self) -> &RequestId<'a> { &self.id }
}


pub struct CancelPromptParamsBuilder<'a> {
    id: RequestId<'a>,
}

impl<'a> CancelPromptParamsBuilder<'a> {
    pub fn build(self) -> CancelPromptParams<'a> {
        CancelPromptParams {
            id: self.id,
        }
    }
}

impl<'a> CancelPromptParams<'a> { pub const METHOD: &'static str = "DeviceAccess.cancelPrompt"; }

impl<'a> crate::CdpCommand<'a> for CancelPromptParams<'a> {
    const METHOD: &'static str = "DeviceAccess.cancelPrompt";
    type Response = crate::EmptyReturns;
}
