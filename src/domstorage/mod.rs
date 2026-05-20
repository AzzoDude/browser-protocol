//! Query and modify DOM storage.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


pub type SerializedStorageKey<'a> = Cow<'a, str>;

/// DOM Storage identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageId<'a> {
    /// Security origin for the storage.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    security_origin: Option<Cow<'a, str>>,
    /// Represents a key by which DOM Storage keys its CachedStorageAreas
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    storage_key: Option<SerializedStorageKey<'a>>,
    /// Whether the storage is local storage (not session storage).
    #[serde(rename = "isLocalStorage")]
    is_local_storage: bool,
}

impl<'a> StorageId<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `is_local_storage`: Whether the storage is local storage (not session storage).
    pub fn builder(is_local_storage: bool) -> StorageIdBuilder<'a> {
        StorageIdBuilder {
            security_origin: None,
            storage_key: None,
            is_local_storage: is_local_storage,
        }
    }
    /// Security origin for the storage.
    pub fn security_origin(&self) -> Option<&str> { self.security_origin.as_deref() }
    /// Represents a key by which DOM Storage keys its CachedStorageAreas
    pub fn storage_key(&self) -> Option<&SerializedStorageKey<'a>> { self.storage_key.as_ref() }
    /// Whether the storage is local storage (not session storage).
    pub fn is_local_storage(&self) -> bool { self.is_local_storage }
}


pub struct StorageIdBuilder<'a> {
    security_origin: Option<Cow<'a, str>>,
    storage_key: Option<SerializedStorageKey<'a>>,
    is_local_storage: bool,
}

impl<'a> StorageIdBuilder<'a> {
    /// Security origin for the storage.
    pub fn security_origin(mut self, security_origin: impl Into<Cow<'a, str>>) -> Self { self.security_origin = Some(security_origin.into()); self }
    /// Represents a key by which DOM Storage keys its CachedStorageAreas
    pub fn storage_key(mut self, storage_key: impl Into<SerializedStorageKey<'a>>) -> Self { self.storage_key = Some(storage_key.into()); self }
    pub fn build(self) -> StorageId<'a> {
        StorageId {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            is_local_storage: self.is_local_storage,
        }
    }
}

/// DOM Storage item.

pub type Item<'a> = Vec<Cow<'a, str>>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearParams<'a> {
    #[serde(rename = "storageId")]
    storage_id: StorageId<'a>,
}

impl<'a> ClearParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_id`: 
    pub fn builder(storage_id: StorageId<'a>) -> ClearParamsBuilder<'a> {
        ClearParamsBuilder {
            storage_id: storage_id,
        }
    }
    pub fn storage_id(&self) -> &StorageId<'a> { &self.storage_id }
}


pub struct ClearParamsBuilder<'a> {
    storage_id: StorageId<'a>,
}

impl<'a> ClearParamsBuilder<'a> {
    pub fn build(self) -> ClearParams<'a> {
        ClearParams {
            storage_id: self.storage_id,
        }
    }
}

impl<'a> ClearParams<'a> { pub const METHOD: &'static str = "DOMStorage.clear"; }

impl<'a> crate::CdpCommand<'a> for ClearParams<'a> {
    const METHOD: &'static str = "DOMStorage.clear";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "DOMStorage.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "DOMStorage.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "DOMStorage.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "DOMStorage.enable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMStorageItemsParams<'a> {
    #[serde(rename = "storageId")]
    storage_id: StorageId<'a>,
}

impl<'a> GetDOMStorageItemsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_id`: 
    pub fn builder(storage_id: StorageId<'a>) -> GetDOMStorageItemsParamsBuilder<'a> {
        GetDOMStorageItemsParamsBuilder {
            storage_id: storage_id,
        }
    }
    pub fn storage_id(&self) -> &StorageId<'a> { &self.storage_id }
}


pub struct GetDOMStorageItemsParamsBuilder<'a> {
    storage_id: StorageId<'a>,
}

impl<'a> GetDOMStorageItemsParamsBuilder<'a> {
    pub fn build(self) -> GetDOMStorageItemsParams<'a> {
        GetDOMStorageItemsParams {
            storage_id: self.storage_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMStorageItemsReturns<'a> {
    entries: Vec<Item<'a>>,
}

impl<'a> GetDOMStorageItemsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `entries`: 
    pub fn builder(entries: Vec<Item<'a>>) -> GetDOMStorageItemsReturnsBuilder<'a> {
        GetDOMStorageItemsReturnsBuilder {
            entries: entries,
        }
    }
    pub fn entries(&self) -> &[Item<'a>] { &self.entries }
}


pub struct GetDOMStorageItemsReturnsBuilder<'a> {
    entries: Vec<Item<'a>>,
}

impl<'a> GetDOMStorageItemsReturnsBuilder<'a> {
    pub fn build(self) -> GetDOMStorageItemsReturns<'a> {
        GetDOMStorageItemsReturns {
            entries: self.entries,
        }
    }
}

impl<'a> GetDOMStorageItemsParams<'a> { pub const METHOD: &'static str = "DOMStorage.getDOMStorageItems"; }

impl<'a> crate::CdpCommand<'a> for GetDOMStorageItemsParams<'a> {
    const METHOD: &'static str = "DOMStorage.getDOMStorageItems";
    type Response = GetDOMStorageItemsReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDOMStorageItemParams<'a> {
    #[serde(rename = "storageId")]
    storage_id: StorageId<'a>,
    key: Cow<'a, str>,
}

impl<'a> RemoveDOMStorageItemParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_id`: 
    /// * `key`: 
    pub fn builder(storage_id: StorageId<'a>, key: impl Into<Cow<'a, str>>) -> RemoveDOMStorageItemParamsBuilder<'a> {
        RemoveDOMStorageItemParamsBuilder {
            storage_id: storage_id,
            key: key.into(),
        }
    }
    pub fn storage_id(&self) -> &StorageId<'a> { &self.storage_id }
    pub fn key(&self) -> &str { self.key.as_ref() }
}


pub struct RemoveDOMStorageItemParamsBuilder<'a> {
    storage_id: StorageId<'a>,
    key: Cow<'a, str>,
}

impl<'a> RemoveDOMStorageItemParamsBuilder<'a> {
    pub fn build(self) -> RemoveDOMStorageItemParams<'a> {
        RemoveDOMStorageItemParams {
            storage_id: self.storage_id,
            key: self.key,
        }
    }
}

impl<'a> RemoveDOMStorageItemParams<'a> { pub const METHOD: &'static str = "DOMStorage.removeDOMStorageItem"; }

impl<'a> crate::CdpCommand<'a> for RemoveDOMStorageItemParams<'a> {
    const METHOD: &'static str = "DOMStorage.removeDOMStorageItem";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDOMStorageItemParams<'a> {
    #[serde(rename = "storageId")]
    storage_id: StorageId<'a>,
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SetDOMStorageItemParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_id`: 
    /// * `key`: 
    /// * `value`: 
    pub fn builder(storage_id: StorageId<'a>, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> SetDOMStorageItemParamsBuilder<'a> {
        SetDOMStorageItemParamsBuilder {
            storage_id: storage_id,
            key: key.into(),
            value: value.into(),
        }
    }
    pub fn storage_id(&self) -> &StorageId<'a> { &self.storage_id }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct SetDOMStorageItemParamsBuilder<'a> {
    storage_id: StorageId<'a>,
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SetDOMStorageItemParamsBuilder<'a> {
    pub fn build(self) -> SetDOMStorageItemParams<'a> {
        SetDOMStorageItemParams {
            storage_id: self.storage_id,
            key: self.key,
            value: self.value,
        }
    }
}

impl<'a> SetDOMStorageItemParams<'a> { pub const METHOD: &'static str = "DOMStorage.setDOMStorageItem"; }

impl<'a> crate::CdpCommand<'a> for SetDOMStorageItemParams<'a> {
    const METHOD: &'static str = "DOMStorage.setDOMStorageItem";
    type Response = crate::EmptyReturns;
}
