use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Device request id.

pub type RequestId<'a> = Cow<'a, str>;

/// A device id.

pub type DeviceId<'a> = Cow<'a, str>;

/// Device information displayed in a user prompt to select a device.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PromptDevice<'a> {
    pub id: DeviceId<'a>,
    /// Display name as it appears in a device request user prompt.
    pub name: Cow<'a, str>,
}
/// Enable events in this domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DeviceAccess.enable")]
pub struct EnableParams {

}
/// Disable events in this domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DeviceAccess.disable")]
pub struct DisableParams {

}
/// Select a device in response to a DeviceAccess.deviceRequestPrompted event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DeviceAccess.selectPrompt")]
pub struct SelectPromptParams<'a> {
    pub id: RequestId<'a>,
    #[serde(rename = "deviceId")]
    pub device_id: DeviceId<'a>,
}
/// Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DeviceAccess.cancelPrompt")]
pub struct CancelPromptParams<'a> {
    pub id: RequestId<'a>,
}
/// A device request opened a user prompt to select a device. Respond with the
/// selectPrompt or cancelPrompt command.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DeviceAccess.deviceRequestPrompted")]
pub struct DeviceRequestPrompted<'a> {
    pub id: RequestId<'a>,
    pub devices: Vec<PromptDevice<'a>>,
}