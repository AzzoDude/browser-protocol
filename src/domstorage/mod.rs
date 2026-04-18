//! Query and modify DOM storage.

use serde::{Serialize, Deserialize};


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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDOMStorageItemParams {

    pub storageId: StorageId,

    pub key: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDOMStorageItemParams {

    pub storageId: StorageId,

    pub key: String,

    pub value: String,
}
