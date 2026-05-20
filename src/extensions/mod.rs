//! Defines commands and events for browser extensions.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Storage areas.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StorageArea {
    #[default]
    #[serde(rename = "session")]
    Session,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "sync")]
    Sync,
    #[serde(rename = "managed")]
    Managed,
}

/// Detailed information about an extension.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionInfo<'a> {
    /// Extension id.
    id: Cow<'a, str>,
    /// Extension name.
    name: Cow<'a, str>,
    /// Extension version.
    version: Cow<'a, str>,
    /// The path from which the extension was loaded.
    path: Cow<'a, str>,
    /// Extension enabled status.
    enabled: bool,
}

impl<'a> ExtensionInfo<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: Extension id.
    /// * `name`: Extension name.
    /// * `version`: Extension version.
    /// * `path`: The path from which the extension was loaded.
    /// * `enabled`: Extension enabled status.
    pub fn builder(id: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, version: impl Into<Cow<'a, str>>, path: impl Into<Cow<'a, str>>, enabled: bool) -> ExtensionInfoBuilder<'a> {
        ExtensionInfoBuilder {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            path: path.into(),
            enabled: enabled,
        }
    }
    /// Extension id.
    pub fn id(&self) -> &str { self.id.as_ref() }
    /// Extension name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Extension version.
    pub fn version(&self) -> &str { self.version.as_ref() }
    /// The path from which the extension was loaded.
    pub fn path(&self) -> &str { self.path.as_ref() }
    /// Extension enabled status.
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct ExtensionInfoBuilder<'a> {
    id: Cow<'a, str>,
    name: Cow<'a, str>,
    version: Cow<'a, str>,
    path: Cow<'a, str>,
    enabled: bool,
}

impl<'a> ExtensionInfoBuilder<'a> {
    pub fn build(self) -> ExtensionInfo<'a> {
        ExtensionInfo {
            id: self.id,
            name: self.name,
            version: self.version,
            path: self.path,
            enabled: self.enabled,
        }
    }
}

/// Runs an extension default action.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerActionParams<'a> {
    /// Extension id.
    id: Cow<'a, str>,
    /// A tab target ID to trigger the default extension action on.
    #[serde(rename = "targetId")]
    target_id: Cow<'a, str>,
}

impl<'a> TriggerActionParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: Extension id.
    /// * `target_id`: A tab target ID to trigger the default extension action on.
    pub fn builder(id: impl Into<Cow<'a, str>>, target_id: impl Into<Cow<'a, str>>) -> TriggerActionParamsBuilder<'a> {
        TriggerActionParamsBuilder {
            id: id.into(),
            target_id: target_id.into(),
        }
    }
    /// Extension id.
    pub fn id(&self) -> &str { self.id.as_ref() }
    /// A tab target ID to trigger the default extension action on.
    pub fn target_id(&self) -> &str { self.target_id.as_ref() }
}


pub struct TriggerActionParamsBuilder<'a> {
    id: Cow<'a, str>,
    target_id: Cow<'a, str>,
}

impl<'a> TriggerActionParamsBuilder<'a> {
    pub fn build(self) -> TriggerActionParams<'a> {
        TriggerActionParams {
            id: self.id,
            target_id: self.target_id,
        }
    }
}

impl<'a> TriggerActionParams<'a> { pub const METHOD: &'static str = "Extensions.triggerAction"; }

impl<'a> crate::CdpCommand<'a> for TriggerActionParams<'a> {
    const METHOD: &'static str = "Extensions.triggerAction";
    type Response = crate::EmptyReturns;
}

/// Installs an unpacked extension from the filesystem similar to
/// --load-extension CLI flags. Returns extension ID once the extension
/// has been installed.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadUnpackedParams<'a> {
    /// Absolute file path.
    path: Cow<'a, str>,
    /// Enable the extension in incognito
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableInIncognito")]
    enable_in_incognito: Option<bool>,
}

impl<'a> LoadUnpackedParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `path`: Absolute file path.
    pub fn builder(path: impl Into<Cow<'a, str>>) -> LoadUnpackedParamsBuilder<'a> {
        LoadUnpackedParamsBuilder {
            path: path.into(),
            enable_in_incognito: None,
        }
    }
    /// Absolute file path.
    pub fn path(&self) -> &str { self.path.as_ref() }
    /// Enable the extension in incognito
    pub fn enable_in_incognito(&self) -> Option<bool> { self.enable_in_incognito }
}


pub struct LoadUnpackedParamsBuilder<'a> {
    path: Cow<'a, str>,
    enable_in_incognito: Option<bool>,
}

impl<'a> LoadUnpackedParamsBuilder<'a> {
    /// Enable the extension in incognito
    pub fn enable_in_incognito(mut self, enable_in_incognito: bool) -> Self { self.enable_in_incognito = Some(enable_in_incognito); self }
    pub fn build(self) -> LoadUnpackedParams<'a> {
        LoadUnpackedParams {
            path: self.path,
            enable_in_incognito: self.enable_in_incognito,
        }
    }
}

/// Installs an unpacked extension from the filesystem similar to
/// --load-extension CLI flags. Returns extension ID once the extension
/// has been installed.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadUnpackedReturns<'a> {
    /// Extension id.
    id: Cow<'a, str>,
}

impl<'a> LoadUnpackedReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: Extension id.
    pub fn builder(id: impl Into<Cow<'a, str>>) -> LoadUnpackedReturnsBuilder<'a> {
        LoadUnpackedReturnsBuilder {
            id: id.into(),
        }
    }
    /// Extension id.
    pub fn id(&self) -> &str { self.id.as_ref() }
}


pub struct LoadUnpackedReturnsBuilder<'a> {
    id: Cow<'a, str>,
}

impl<'a> LoadUnpackedReturnsBuilder<'a> {
    pub fn build(self) -> LoadUnpackedReturns<'a> {
        LoadUnpackedReturns {
            id: self.id,
        }
    }
}

impl<'a> LoadUnpackedParams<'a> { pub const METHOD: &'static str = "Extensions.loadUnpacked"; }

impl<'a> crate::CdpCommand<'a> for LoadUnpackedParams<'a> {
    const METHOD: &'static str = "Extensions.loadUnpacked";
    type Response = LoadUnpackedReturns<'a>;
}

/// Gets a list of all unpacked extensions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetExtensionsReturns<'a> {
    extensions: Vec<ExtensionInfo<'a>>,
}

impl<'a> GetExtensionsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `extensions`: 
    pub fn builder(extensions: Vec<ExtensionInfo<'a>>) -> GetExtensionsReturnsBuilder<'a> {
        GetExtensionsReturnsBuilder {
            extensions: extensions,
        }
    }
    pub fn extensions(&self) -> &[ExtensionInfo<'a>] { &self.extensions }
}


pub struct GetExtensionsReturnsBuilder<'a> {
    extensions: Vec<ExtensionInfo<'a>>,
}

impl<'a> GetExtensionsReturnsBuilder<'a> {
    pub fn build(self) -> GetExtensionsReturns<'a> {
        GetExtensionsReturns {
            extensions: self.extensions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetExtensionsParams {}

impl GetExtensionsParams { pub const METHOD: &'static str = "Extensions.getExtensions"; }

impl<'a> crate::CdpCommand<'a> for GetExtensionsParams {
    const METHOD: &'static str = "Extensions.getExtensions";
    type Response = GetExtensionsReturns<'a>;
}

/// Uninstalls an unpacked extension (others not supported) from the profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UninstallParams<'a> {
    /// Extension id.
    id: Cow<'a, str>,
}

impl<'a> UninstallParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: Extension id.
    pub fn builder(id: impl Into<Cow<'a, str>>) -> UninstallParamsBuilder<'a> {
        UninstallParamsBuilder {
            id: id.into(),
        }
    }
    /// Extension id.
    pub fn id(&self) -> &str { self.id.as_ref() }
}


pub struct UninstallParamsBuilder<'a> {
    id: Cow<'a, str>,
}

impl<'a> UninstallParamsBuilder<'a> {
    pub fn build(self) -> UninstallParams<'a> {
        UninstallParams {
            id: self.id,
        }
    }
}

impl<'a> UninstallParams<'a> { pub const METHOD: &'static str = "Extensions.uninstall"; }

impl<'a> crate::CdpCommand<'a> for UninstallParams<'a> {
    const METHOD: &'static str = "Extensions.uninstall";
    type Response = crate::EmptyReturns;
}

/// Gets data from extension storage in the given 'storageArea'. If 'keys' is
/// specified, these are used to filter the result.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageItemsParams<'a> {
    /// ID of extension.
    id: Cow<'a, str>,
    /// StorageArea to retrieve data from.
    #[serde(rename = "storageArea")]
    storage_area: StorageArea,
    /// Keys to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    keys: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GetStorageItemsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: ID of extension.
    /// * `storage_area`: StorageArea to retrieve data from.
    pub fn builder(id: impl Into<Cow<'a, str>>, storage_area: impl Into<StorageArea>) -> GetStorageItemsParamsBuilder<'a> {
        GetStorageItemsParamsBuilder {
            id: id.into(),
            storage_area: storage_area.into(),
            keys: None,
        }
    }
    /// ID of extension.
    pub fn id(&self) -> &str { self.id.as_ref() }
    /// StorageArea to retrieve data from.
    pub fn storage_area(&self) -> &StorageArea { &self.storage_area }
    /// Keys to retrieve.
    pub fn keys(&self) -> Option<&[Cow<'a, str>]> { self.keys.as_deref() }
}


pub struct GetStorageItemsParamsBuilder<'a> {
    id: Cow<'a, str>,
    storage_area: StorageArea,
    keys: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GetStorageItemsParamsBuilder<'a> {
    /// Keys to retrieve.
    pub fn keys(mut self, keys: Vec<Cow<'a, str>>) -> Self { self.keys = Some(keys); self }
    pub fn build(self) -> GetStorageItemsParams<'a> {
        GetStorageItemsParams {
            id: self.id,
            storage_area: self.storage_area,
            keys: self.keys,
        }
    }
}

/// Gets data from extension storage in the given 'storageArea'. If 'keys' is
/// specified, these are used to filter the result.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageItemsReturns {
    data: serde_json::Map<String, JsonValue>,
}

impl GetStorageItemsReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `data`: 
    pub fn builder(data: serde_json::Map<String, JsonValue>) -> GetStorageItemsReturnsBuilder {
        GetStorageItemsReturnsBuilder {
            data: data,
        }
    }
    pub fn data(&self) -> &serde_json::Map<String, JsonValue> { &self.data }
}


pub struct GetStorageItemsReturnsBuilder {
    data: serde_json::Map<String, JsonValue>,
}

impl GetStorageItemsReturnsBuilder {
    pub fn build(self) -> GetStorageItemsReturns {
        GetStorageItemsReturns {
            data: self.data,
        }
    }
}

impl<'a> GetStorageItemsParams<'a> { pub const METHOD: &'static str = "Extensions.getStorageItems"; }

impl<'a> crate::CdpCommand<'a> for GetStorageItemsParams<'a> {
    const METHOD: &'static str = "Extensions.getStorageItems";
    type Response = GetStorageItemsReturns;
}

/// Removes 'keys' from extension storage in the given 'storageArea'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveStorageItemsParams<'a> {
    /// ID of extension.
    id: Cow<'a, str>,
    /// StorageArea to remove data from.
    #[serde(rename = "storageArea")]
    storage_area: StorageArea,
    /// Keys to remove.
    keys: Vec<Cow<'a, str>>,
}

impl<'a> RemoveStorageItemsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: ID of extension.
    /// * `storage_area`: StorageArea to remove data from.
    /// * `keys`: Keys to remove.
    pub fn builder(id: impl Into<Cow<'a, str>>, storage_area: impl Into<StorageArea>, keys: Vec<Cow<'a, str>>) -> RemoveStorageItemsParamsBuilder<'a> {
        RemoveStorageItemsParamsBuilder {
            id: id.into(),
            storage_area: storage_area.into(),
            keys: keys,
        }
    }
    /// ID of extension.
    pub fn id(&self) -> &str { self.id.as_ref() }
    /// StorageArea to remove data from.
    pub fn storage_area(&self) -> &StorageArea { &self.storage_area }
    /// Keys to remove.
    pub fn keys(&self) -> &[Cow<'a, str>] { &self.keys }
}


pub struct RemoveStorageItemsParamsBuilder<'a> {
    id: Cow<'a, str>,
    storage_area: StorageArea,
    keys: Vec<Cow<'a, str>>,
}

impl<'a> RemoveStorageItemsParamsBuilder<'a> {
    pub fn build(self) -> RemoveStorageItemsParams<'a> {
        RemoveStorageItemsParams {
            id: self.id,
            storage_area: self.storage_area,
            keys: self.keys,
        }
    }
}

impl<'a> RemoveStorageItemsParams<'a> { pub const METHOD: &'static str = "Extensions.removeStorageItems"; }

impl<'a> crate::CdpCommand<'a> for RemoveStorageItemsParams<'a> {
    const METHOD: &'static str = "Extensions.removeStorageItems";
    type Response = crate::EmptyReturns;
}

/// Clears extension storage in the given 'storageArea'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearStorageItemsParams<'a> {
    /// ID of extension.
    id: Cow<'a, str>,
    /// StorageArea to remove data from.
    #[serde(rename = "storageArea")]
    storage_area: StorageArea,
}

impl<'a> ClearStorageItemsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: ID of extension.
    /// * `storage_area`: StorageArea to remove data from.
    pub fn builder(id: impl Into<Cow<'a, str>>, storage_area: impl Into<StorageArea>) -> ClearStorageItemsParamsBuilder<'a> {
        ClearStorageItemsParamsBuilder {
            id: id.into(),
            storage_area: storage_area.into(),
        }
    }
    /// ID of extension.
    pub fn id(&self) -> &str { self.id.as_ref() }
    /// StorageArea to remove data from.
    pub fn storage_area(&self) -> &StorageArea { &self.storage_area }
}


pub struct ClearStorageItemsParamsBuilder<'a> {
    id: Cow<'a, str>,
    storage_area: StorageArea,
}

impl<'a> ClearStorageItemsParamsBuilder<'a> {
    pub fn build(self) -> ClearStorageItemsParams<'a> {
        ClearStorageItemsParams {
            id: self.id,
            storage_area: self.storage_area,
        }
    }
}

impl<'a> ClearStorageItemsParams<'a> { pub const METHOD: &'static str = "Extensions.clearStorageItems"; }

impl<'a> crate::CdpCommand<'a> for ClearStorageItemsParams<'a> {
    const METHOD: &'static str = "Extensions.clearStorageItems";
    type Response = crate::EmptyReturns;
}

/// Sets 'values' in extension storage in the given 'storageArea'. The provided 'values'
/// will be merged with existing values in the storage area.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStorageItemsParams<'a> {
    /// ID of extension.
    id: Cow<'a, str>,
    /// StorageArea to set data in.
    #[serde(rename = "storageArea")]
    storage_area: StorageArea,
    /// Values to set.
    values: serde_json::Map<String, JsonValue>,
}

impl<'a> SetStorageItemsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: ID of extension.
    /// * `storage_area`: StorageArea to set data in.
    /// * `values`: Values to set.
    pub fn builder(id: impl Into<Cow<'a, str>>, storage_area: impl Into<StorageArea>, values: serde_json::Map<String, JsonValue>) -> SetStorageItemsParamsBuilder<'a> {
        SetStorageItemsParamsBuilder {
            id: id.into(),
            storage_area: storage_area.into(),
            values: values,
        }
    }
    /// ID of extension.
    pub fn id(&self) -> &str { self.id.as_ref() }
    /// StorageArea to set data in.
    pub fn storage_area(&self) -> &StorageArea { &self.storage_area }
    /// Values to set.
    pub fn values(&self) -> &serde_json::Map<String, JsonValue> { &self.values }
}


pub struct SetStorageItemsParamsBuilder<'a> {
    id: Cow<'a, str>,
    storage_area: StorageArea,
    values: serde_json::Map<String, JsonValue>,
}

impl<'a> SetStorageItemsParamsBuilder<'a> {
    pub fn build(self) -> SetStorageItemsParams<'a> {
        SetStorageItemsParams {
            id: self.id,
            storage_area: self.storage_area,
            values: self.values,
        }
    }
}

impl<'a> SetStorageItemsParams<'a> { pub const METHOD: &'static str = "Extensions.setStorageItems"; }

impl<'a> crate::CdpCommand<'a> for SetStorageItemsParams<'a> {
    const METHOD: &'static str = "Extensions.setStorageItems";
    type Response = crate::EmptyReturns;
}
