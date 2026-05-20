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
    pub fn builder() -> ExtensionInfoBuilder<'a> { ExtensionInfoBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn version(&self) -> &str { self.version.as_ref() }
    pub fn path(&self) -> &str { self.path.as_ref() }
    pub fn enabled(&self) -> bool { self.enabled }
}

#[derive(Default)]
pub struct ExtensionInfoBuilder<'a> {
    id: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    version: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    enabled: Option<bool>,
}

impl<'a> ExtensionInfoBuilder<'a> {
    /// Extension id.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    /// Extension name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Extension version.
    pub fn version(mut self, version: impl Into<Cow<'a, str>>) -> Self { self.version = Some(version.into()); self }
    /// The path from which the extension was loaded.
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    /// Extension enabled status.
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    pub fn build(self) -> ExtensionInfo<'a> {
        ExtensionInfo {
            id: self.id.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            version: self.version.unwrap_or_default(),
            path: self.path.unwrap_or_default(),
            enabled: self.enabled.unwrap_or_default(),
        }
    }
}

/// Runs an extension default action.
/// Available if the client is connected using the --remote-debugging-pipe
/// flag and the --enable-unsafe-extension-debugging flag is set.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerActionParams<'a> {
    /// Extension id.
    id: Cow<'a, str>,
    /// A tab target ID to trigger the default extension action on.
    targetId: Cow<'a, str>,
}

impl<'a> TriggerActionParams<'a> {
    pub fn builder() -> TriggerActionParamsBuilder<'a> { TriggerActionParamsBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn targetId(&self) -> &str { self.targetId.as_ref() }
}

#[derive(Default)]
pub struct TriggerActionParamsBuilder<'a> {
    id: Option<Cow<'a, str>>,
    targetId: Option<Cow<'a, str>>,
}

impl<'a> TriggerActionParamsBuilder<'a> {
    /// Extension id.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    /// A tab target ID to trigger the default extension action on.
    pub fn targetId(mut self, targetId: impl Into<Cow<'a, str>>) -> Self { self.targetId = Some(targetId.into()); self }
    pub fn build(self) -> TriggerActionParams<'a> {
        TriggerActionParams {
            id: self.id.unwrap_or_default(),
            targetId: self.targetId.unwrap_or_default(),
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
/// has been installed. Available if the client is connected using the
/// --remote-debugging-pipe flag and the --enable-unsafe-extension-debugging
/// flag is set.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadUnpackedParams<'a> {
    /// Absolute file path.
    path: Cow<'a, str>,
    /// Enable the extension in incognito
    #[serde(skip_serializing_if = "Option::is_none")]
    enableInIncognito: Option<bool>,
}

impl<'a> LoadUnpackedParams<'a> {
    pub fn builder() -> LoadUnpackedParamsBuilder<'a> { LoadUnpackedParamsBuilder::default() }
    pub fn path(&self) -> &str { self.path.as_ref() }
    pub fn enableInIncognito(&self) -> Option<bool> { self.enableInIncognito }
}

#[derive(Default)]
pub struct LoadUnpackedParamsBuilder<'a> {
    path: Option<Cow<'a, str>>,
    enableInIncognito: Option<bool>,
}

impl<'a> LoadUnpackedParamsBuilder<'a> {
    /// Absolute file path.
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    /// Enable the extension in incognito
    pub fn enableInIncognito(mut self, enableInIncognito: bool) -> Self { self.enableInIncognito = Some(enableInIncognito); self }
    pub fn build(self) -> LoadUnpackedParams<'a> {
        LoadUnpackedParams {
            path: self.path.unwrap_or_default(),
            enableInIncognito: self.enableInIncognito,
        }
    }
}

/// Installs an unpacked extension from the filesystem similar to
/// --load-extension CLI flags. Returns extension ID once the extension
/// has been installed. Available if the client is connected using the
/// --remote-debugging-pipe flag and the --enable-unsafe-extension-debugging
/// flag is set.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadUnpackedReturns<'a> {
    /// Extension id.
    id: Cow<'a, str>,
}

impl<'a> LoadUnpackedReturns<'a> {
    pub fn builder() -> LoadUnpackedReturnsBuilder<'a> { LoadUnpackedReturnsBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
}

#[derive(Default)]
pub struct LoadUnpackedReturnsBuilder<'a> {
    id: Option<Cow<'a, str>>,
}

impl<'a> LoadUnpackedReturnsBuilder<'a> {
    /// Extension id.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    pub fn build(self) -> LoadUnpackedReturns<'a> {
        LoadUnpackedReturns {
            id: self.id.unwrap_or_default(),
        }
    }
}

impl<'a> LoadUnpackedParams<'a> { pub const METHOD: &'static str = "Extensions.loadUnpacked"; }

impl<'a> crate::CdpCommand<'a> for LoadUnpackedParams<'a> {
    const METHOD: &'static str = "Extensions.loadUnpacked";
    type Response = LoadUnpackedReturns<'a>;
}

/// Gets a list of all unpacked extensions.
/// Available if the client is connected using the --remote-debugging-pipe flag
/// and the --enable-unsafe-extension-debugging flag is set.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetExtensionsReturns<'a> {
    extensions: Vec<ExtensionInfo<'a>>,
}

impl<'a> GetExtensionsReturns<'a> {
    pub fn builder() -> GetExtensionsReturnsBuilder<'a> { GetExtensionsReturnsBuilder::default() }
    pub fn extensions(&self) -> &[ExtensionInfo<'a>] { &self.extensions }
}

#[derive(Default)]
pub struct GetExtensionsReturnsBuilder<'a> {
    extensions: Option<Vec<ExtensionInfo<'a>>>,
}

impl<'a> GetExtensionsReturnsBuilder<'a> {
    pub fn extensions(mut self, extensions: Vec<ExtensionInfo<'a>>) -> Self { self.extensions = Some(extensions); self }
    pub fn build(self) -> GetExtensionsReturns<'a> {
        GetExtensionsReturns {
            extensions: self.extensions.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetExtensionsParams {}

impl GetExtensionsParams {
    pub fn builder() -> GetExtensionsParamsBuilder {
        GetExtensionsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetExtensionsParamsBuilder {}

impl GetExtensionsParamsBuilder {
    pub fn build(self) -> GetExtensionsParams {
        GetExtensionsParams {}
    }
}

impl GetExtensionsParams { pub const METHOD: &'static str = "Extensions.getExtensions"; }

impl<'a> crate::CdpCommand<'a> for GetExtensionsParams {
    const METHOD: &'static str = "Extensions.getExtensions";
    type Response = GetExtensionsReturns<'a>;
}

/// Uninstalls an unpacked extension (others not supported) from the profile.
/// Available if the client is connected using the --remote-debugging-pipe flag
/// and the --enable-unsafe-extension-debugging.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UninstallParams<'a> {
    /// Extension id.
    id: Cow<'a, str>,
}

impl<'a> UninstallParams<'a> {
    pub fn builder() -> UninstallParamsBuilder<'a> { UninstallParamsBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
}

#[derive(Default)]
pub struct UninstallParamsBuilder<'a> {
    id: Option<Cow<'a, str>>,
}

impl<'a> UninstallParamsBuilder<'a> {
    /// Extension id.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    pub fn build(self) -> UninstallParams<'a> {
        UninstallParams {
            id: self.id.unwrap_or_default(),
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
    storageArea: StorageArea,
    /// Keys to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    keys: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GetStorageItemsParams<'a> {
    pub fn builder() -> GetStorageItemsParamsBuilder<'a> { GetStorageItemsParamsBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn storageArea(&self) -> &StorageArea { &self.storageArea }
    pub fn keys(&self) -> Option<&[Cow<'a, str>]> { self.keys.as_deref() }
}

#[derive(Default)]
pub struct GetStorageItemsParamsBuilder<'a> {
    id: Option<Cow<'a, str>>,
    storageArea: Option<StorageArea>,
    keys: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GetStorageItemsParamsBuilder<'a> {
    /// ID of extension.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    /// StorageArea to retrieve data from.
    pub fn storageArea(mut self, storageArea: StorageArea) -> Self { self.storageArea = Some(storageArea); self }
    /// Keys to retrieve.
    pub fn keys(mut self, keys: Vec<Cow<'a, str>>) -> Self { self.keys = Some(keys); self }
    pub fn build(self) -> GetStorageItemsParams<'a> {
        GetStorageItemsParams {
            id: self.id.unwrap_or_default(),
            storageArea: self.storageArea.unwrap_or_default(),
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
    pub fn builder() -> GetStorageItemsReturnsBuilder { GetStorageItemsReturnsBuilder::default() }
    pub fn data(&self) -> &serde_json::Map<String, JsonValue> { &self.data }
}

#[derive(Default)]
pub struct GetStorageItemsReturnsBuilder {
    data: Option<serde_json::Map<String, JsonValue>>,
}

impl GetStorageItemsReturnsBuilder {
    pub fn data(mut self, data: serde_json::Map<String, JsonValue>) -> Self { self.data = Some(data); self }
    pub fn build(self) -> GetStorageItemsReturns {
        GetStorageItemsReturns {
            data: self.data.unwrap_or_default(),
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
    storageArea: StorageArea,
    /// Keys to remove.
    keys: Vec<Cow<'a, str>>,
}

impl<'a> RemoveStorageItemsParams<'a> {
    pub fn builder() -> RemoveStorageItemsParamsBuilder<'a> { RemoveStorageItemsParamsBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn storageArea(&self) -> &StorageArea { &self.storageArea }
    pub fn keys(&self) -> &[Cow<'a, str>] { &self.keys }
}

#[derive(Default)]
pub struct RemoveStorageItemsParamsBuilder<'a> {
    id: Option<Cow<'a, str>>,
    storageArea: Option<StorageArea>,
    keys: Option<Vec<Cow<'a, str>>>,
}

impl<'a> RemoveStorageItemsParamsBuilder<'a> {
    /// ID of extension.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    /// StorageArea to remove data from.
    pub fn storageArea(mut self, storageArea: StorageArea) -> Self { self.storageArea = Some(storageArea); self }
    /// Keys to remove.
    pub fn keys(mut self, keys: Vec<Cow<'a, str>>) -> Self { self.keys = Some(keys); self }
    pub fn build(self) -> RemoveStorageItemsParams<'a> {
        RemoveStorageItemsParams {
            id: self.id.unwrap_or_default(),
            storageArea: self.storageArea.unwrap_or_default(),
            keys: self.keys.unwrap_or_default(),
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
    storageArea: StorageArea,
}

impl<'a> ClearStorageItemsParams<'a> {
    pub fn builder() -> ClearStorageItemsParamsBuilder<'a> { ClearStorageItemsParamsBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn storageArea(&self) -> &StorageArea { &self.storageArea }
}

#[derive(Default)]
pub struct ClearStorageItemsParamsBuilder<'a> {
    id: Option<Cow<'a, str>>,
    storageArea: Option<StorageArea>,
}

impl<'a> ClearStorageItemsParamsBuilder<'a> {
    /// ID of extension.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    /// StorageArea to remove data from.
    pub fn storageArea(mut self, storageArea: StorageArea) -> Self { self.storageArea = Some(storageArea); self }
    pub fn build(self) -> ClearStorageItemsParams<'a> {
        ClearStorageItemsParams {
            id: self.id.unwrap_or_default(),
            storageArea: self.storageArea.unwrap_or_default(),
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
    storageArea: StorageArea,
    /// Values to set.
    values: serde_json::Map<String, JsonValue>,
}

impl<'a> SetStorageItemsParams<'a> {
    pub fn builder() -> SetStorageItemsParamsBuilder<'a> { SetStorageItemsParamsBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn storageArea(&self) -> &StorageArea { &self.storageArea }
    pub fn values(&self) -> &serde_json::Map<String, JsonValue> { &self.values }
}

#[derive(Default)]
pub struct SetStorageItemsParamsBuilder<'a> {
    id: Option<Cow<'a, str>>,
    storageArea: Option<StorageArea>,
    values: Option<serde_json::Map<String, JsonValue>>,
}

impl<'a> SetStorageItemsParamsBuilder<'a> {
    /// ID of extension.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    /// StorageArea to set data in.
    pub fn storageArea(mut self, storageArea: StorageArea) -> Self { self.storageArea = Some(storageArea); self }
    /// Values to set.
    pub fn values(mut self, values: serde_json::Map<String, JsonValue>) -> Self { self.values = Some(values); self }
    pub fn build(self) -> SetStorageItemsParams<'a> {
        SetStorageItemsParams {
            id: self.id.unwrap_or_default(),
            storageArea: self.storageArea.unwrap_or_default(),
            values: self.values.unwrap_or_default(),
        }
    }
}

impl<'a> SetStorageItemsParams<'a> { pub const METHOD: &'static str = "Extensions.setStorageItems"; }

impl<'a> crate::CdpCommand<'a> for SetStorageItemsParams<'a> {
    const METHOD: &'static str = "Extensions.setStorageItems";
    type Response = crate::EmptyReturns;
}
