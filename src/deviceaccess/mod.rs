use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Device request id.

pub type RequestId = String;

/// A device id.

pub type DeviceId = String;

/// Device information displayed in a user prompt to select a device.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PromptDevice {

    pub id: DeviceId,
    /// Display name as it appears in a device request user prompt.

    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "DeviceAccess.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "DeviceAccess.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "DeviceAccess.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "DeviceAccess.disable";
    type Response = crate::EmptyReturns;
}

/// Select a device in response to a DeviceAccess.deviceRequestPrompted event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectPromptParams {

    pub id: RequestId,

    pub deviceId: DeviceId,
}

impl SelectPromptParams { pub const METHOD: &'static str = "DeviceAccess.selectPrompt"; }

impl crate::CdpCommand for SelectPromptParams {
    const METHOD: &'static str = "DeviceAccess.selectPrompt";
    type Response = crate::EmptyReturns;
}

/// Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelPromptParams {

    pub id: RequestId,
}

impl CancelPromptParams { pub const METHOD: &'static str = "DeviceAccess.cancelPrompt"; }

impl crate::CdpCommand for CancelPromptParams {
    const METHOD: &'static str = "DeviceAccess.cancelPrompt";
    type Response = crate::EmptyReturns;
}
