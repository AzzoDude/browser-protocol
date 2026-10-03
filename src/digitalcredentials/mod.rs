//! This domain allows interacting with the Digital Credentials API for automation.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// The type of virtual wallet action.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum VirtualWalletAction {
    #[default]
    #[serde(rename = "respond")]
    Respond,
    #[serde(rename = "decline")]
    Decline,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "clear")]
    Clear,
}

/// Sets the behavior of the virtual wallet for digital credential requests
/// issued from this frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DigitalCredentials.setVirtualWalletBehavior")]
pub struct SetVirtualWalletBehaviorParams<'a> {
    /// The action of the virtual wallet.
    pub action: VirtualWalletAction,
    /// The protocol identifier (e.g. "openid4vp"). Required when |action| is
    /// "respond", forbidden otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Cow<'a, str>>,
    /// The response data object returned by the wallet.
    /// Required when |action| is "respond", forbidden otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Map<String, JsonValue>>,
    /// The frame to scope the virtual wallet behavior to.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
}