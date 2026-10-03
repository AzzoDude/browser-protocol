//! Query and modify DOM storage.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


pub type SerializedStorageKey<'a> = Cow<'a, str>;

/// DOM Storage identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StorageId<'a> {
    /// Security origin for the storage.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    pub security_origin: Option<Cow<'a, str>>,
    /// Represents a key by which DOM Storage keys its CachedStorageAreas
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    pub storage_key: Option<SerializedStorageKey<'a>>,
    /// Whether the storage is local storage (not session storage).
    #[serde(rename = "isLocalStorage")]
    pub is_local_storage: bool,
}
/// DOM Storage item.

pub type Item<'a> = Vec<Cow<'a, str>>;


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.clear")]
pub struct ClearParams<'a> {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId<'a>,
}
/// Disables storage tracking, prevents storage events from being sent to the client.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.disable")]
pub struct DisableParams {

}
/// Enables storage tracking, storage events will now be delivered to the client.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.enable")]
pub struct EnableParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.getDOMStorageItems", response = "GetDOMStorageItemsReturns<'a>")]
pub struct GetDOMStorageItemsParams<'a> {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMStorageItemsReturns<'a> {
    pub entries: Vec<Item<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.removeDOMStorageItem")]
pub struct RemoveDOMStorageItemParams<'a> {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId<'a>,
    pub key: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.setDOMStorageItem")]
pub struct SetDOMStorageItemParams<'a> {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId<'a>,
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.domStorageItemAdded")]
pub struct DomStorageItemAdded<'a> {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId<'a>,
    pub key: Cow<'a, str>,
    #[serde(rename = "newValue")]
    pub new_value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.domStorageItemRemoved")]
pub struct DomStorageItemRemoved<'a> {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId<'a>,
    pub key: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.domStorageItemUpdated")]
pub struct DomStorageItemUpdated<'a> {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId<'a>,
    pub key: Cow<'a, str>,
    #[serde(rename = "oldValue")]
    pub old_value: Cow<'a, str>,
    #[serde(rename = "newValue")]
    pub new_value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMStorage.domStorageItemsCleared")]
pub struct DomStorageItemsCleared<'a> {
    #[serde(rename = "storageId")]
    pub storage_id: StorageId<'a>,
}