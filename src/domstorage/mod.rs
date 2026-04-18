use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! Query and modify DOM storage.


pub type SerializedStorageKey = String;

/// DOM Storage identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageId {
    /// Security origin for the storage.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityOrigin: Option<String>,
    /// Represents a key by which DOM Storage keys its CachedStorageAreas

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageKey: Option<SerializedStorageKey>,
    /// Whether the storage is local storage (not session storage).

    pub isLocalStorage: bool,
}

/// DOM Storage item.

pub type Item = Vec<String>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearParams {

    pub storageId: StorageId,
}

impl ClearParams { pub const METHOD: &'static str = "DOMStorage.clear"; }

impl crate::CdpCommand for ClearParams {
    const METHOD: &'static str = "DOMStorage.clear";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "DOMStorage.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "DOMStorage.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "DOMStorage.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "DOMStorage.enable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMStorageItemsParams {

    pub storageId: StorageId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMStorageItemsReturns {

    pub entries: Vec<Item>,
}

impl GetDOMStorageItemsParams { pub const METHOD: &'static str = "DOMStorage.getDOMStorageItems"; }

impl crate::CdpCommand for GetDOMStorageItemsParams {
    const METHOD: &'static str = "DOMStorage.getDOMStorageItems";
    type Response = GetDOMStorageItemsReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDOMStorageItemParams {

    pub storageId: StorageId,

    pub key: String,
}

impl RemoveDOMStorageItemParams { pub const METHOD: &'static str = "DOMStorage.removeDOMStorageItem"; }

impl crate::CdpCommand for RemoveDOMStorageItemParams {
    const METHOD: &'static str = "DOMStorage.removeDOMStorageItem";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDOMStorageItemParams {

    pub storageId: StorageId,

    pub key: String,

    pub value: String,
}

impl SetDOMStorageItemParams { pub const METHOD: &'static str = "DOMStorage.setDOMStorageItem"; }

impl crate::CdpCommand for SetDOMStorageItemParams {
    const METHOD: &'static str = "DOMStorage.setDOMStorageItem";
    type Response = crate::EmptyReturns;
}
