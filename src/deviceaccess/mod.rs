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

/// Select a device in response to a DeviceAccess.deviceRequestPrompted event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectPromptParams {

    pub id: RequestId,

    pub deviceId: DeviceId,
}

/// Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelPromptParams {

    pub id: RequestId,
}
