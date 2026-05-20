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
    #[serde(skip_serializing_if = "Option::is_none")]
    securityOrigin: Option<Cow<'a, str>>,
    /// Represents a key by which DOM Storage keys its CachedStorageAreas
    #[serde(skip_serializing_if = "Option::is_none")]
    storageKey: Option<SerializedStorageKey<'a>>,
    /// Whether the storage is local storage (not session storage).
    isLocalStorage: bool,
}

impl<'a> StorageId<'a> {
    pub fn builder() -> StorageIdBuilder<'a> { StorageIdBuilder::default() }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&SerializedStorageKey<'a>> { self.storageKey.as_ref() }
    pub fn isLocalStorage(&self) -> bool { self.isLocalStorage }
}

#[derive(Default)]
pub struct StorageIdBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<SerializedStorageKey<'a>>,
    isLocalStorage: Option<bool>,
}

impl<'a> StorageIdBuilder<'a> {
    /// Security origin for the storage.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Represents a key by which DOM Storage keys its CachedStorageAreas
    pub fn storageKey(mut self, storageKey: SerializedStorageKey<'a>) -> Self { self.storageKey = Some(storageKey); self }
    /// Whether the storage is local storage (not session storage).
    pub fn isLocalStorage(mut self, isLocalStorage: bool) -> Self { self.isLocalStorage = Some(isLocalStorage); self }
    pub fn build(self) -> StorageId<'a> {
        StorageId {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            isLocalStorage: self.isLocalStorage.unwrap_or_default(),
        }
    }
}

/// DOM Storage item.

pub type Item<'a> = Vec<Cow<'a, str>>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearParams<'a> {
    storageId: StorageId<'a>,
}

impl<'a> ClearParams<'a> {
    pub fn builder() -> ClearParamsBuilder<'a> { ClearParamsBuilder::default() }
    pub fn storageId(&self) -> &StorageId<'a> { &self.storageId }
}

#[derive(Default)]
pub struct ClearParamsBuilder<'a> {
    storageId: Option<StorageId<'a>>,
}

impl<'a> ClearParamsBuilder<'a> {
    pub fn storageId(mut self, storageId: StorageId<'a>) -> Self { self.storageId = Some(storageId); self }
    pub fn build(self) -> ClearParams<'a> {
        ClearParams {
            storageId: self.storageId.unwrap_or_default(),
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

impl DisableParams { pub const METHOD: &'static str = "DOMStorage.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "DOMStorage.disable";
    type Response = crate::EmptyReturns;
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

impl EnableParams { pub const METHOD: &'static str = "DOMStorage.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "DOMStorage.enable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMStorageItemsParams<'a> {
    storageId: StorageId<'a>,
}

impl<'a> GetDOMStorageItemsParams<'a> {
    pub fn builder() -> GetDOMStorageItemsParamsBuilder<'a> { GetDOMStorageItemsParamsBuilder::default() }
    pub fn storageId(&self) -> &StorageId<'a> { &self.storageId }
}

#[derive(Default)]
pub struct GetDOMStorageItemsParamsBuilder<'a> {
    storageId: Option<StorageId<'a>>,
}

impl<'a> GetDOMStorageItemsParamsBuilder<'a> {
    pub fn storageId(mut self, storageId: StorageId<'a>) -> Self { self.storageId = Some(storageId); self }
    pub fn build(self) -> GetDOMStorageItemsParams<'a> {
        GetDOMStorageItemsParams {
            storageId: self.storageId.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDOMStorageItemsReturns<'a> {
    entries: Vec<Item<'a>>,
}

impl<'a> GetDOMStorageItemsReturns<'a> {
    pub fn builder() -> GetDOMStorageItemsReturnsBuilder<'a> { GetDOMStorageItemsReturnsBuilder::default() }
    pub fn entries(&self) -> &[Item<'a>] { &self.entries }
}

#[derive(Default)]
pub struct GetDOMStorageItemsReturnsBuilder<'a> {
    entries: Option<Vec<Item<'a>>>,
}

impl<'a> GetDOMStorageItemsReturnsBuilder<'a> {
    pub fn entries(mut self, entries: Vec<Item<'a>>) -> Self { self.entries = Some(entries); self }
    pub fn build(self) -> GetDOMStorageItemsReturns<'a> {
        GetDOMStorageItemsReturns {
            entries: self.entries.unwrap_or_default(),
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
    storageId: StorageId<'a>,
    key: Cow<'a, str>,
}

impl<'a> RemoveDOMStorageItemParams<'a> {
    pub fn builder() -> RemoveDOMStorageItemParamsBuilder<'a> { RemoveDOMStorageItemParamsBuilder::default() }
    pub fn storageId(&self) -> &StorageId<'a> { &self.storageId }
    pub fn key(&self) -> &str { self.key.as_ref() }
}

#[derive(Default)]
pub struct RemoveDOMStorageItemParamsBuilder<'a> {
    storageId: Option<StorageId<'a>>,
    key: Option<Cow<'a, str>>,
}

impl<'a> RemoveDOMStorageItemParamsBuilder<'a> {
    pub fn storageId(mut self, storageId: StorageId<'a>) -> Self { self.storageId = Some(storageId); self }
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    pub fn build(self) -> RemoveDOMStorageItemParams<'a> {
        RemoveDOMStorageItemParams {
            storageId: self.storageId.unwrap_or_default(),
            key: self.key.unwrap_or_default(),
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
    storageId: StorageId<'a>,
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SetDOMStorageItemParams<'a> {
    pub fn builder() -> SetDOMStorageItemParamsBuilder<'a> { SetDOMStorageItemParamsBuilder::default() }
    pub fn storageId(&self) -> &StorageId<'a> { &self.storageId }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct SetDOMStorageItemParamsBuilder<'a> {
    storageId: Option<StorageId<'a>>,
    key: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> SetDOMStorageItemParamsBuilder<'a> {
    pub fn storageId(mut self, storageId: StorageId<'a>) -> Self { self.storageId = Some(storageId); self }
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> SetDOMStorageItemParams<'a> {
        SetDOMStorageItemParams {
            storageId: self.storageId.unwrap_or_default(),
            key: self.key.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}

impl<'a> SetDOMStorageItemParams<'a> { pub const METHOD: &'static str = "DOMStorage.setDOMStorageItem"; }

impl<'a> crate::CdpCommand<'a> for SetDOMStorageItemParams<'a> {
    const METHOD: &'static str = "DOMStorage.setDOMStorageItem";
    type Response = crate::EmptyReturns;
}
