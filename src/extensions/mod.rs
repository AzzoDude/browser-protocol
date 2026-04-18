use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! Defines commands and events for browser extensions.

/// Storage areas.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StorageArea {
    #[default]
    Session,
    Local,
    Sync,
    Managed,
}

/// Detailed information about an extension.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionInfo {
    /// Extension id.

    pub id: String,
    /// Extension name.

    pub name: String,
    /// Extension version.

    pub version: String,
    /// The path from which the extension was loaded.

    pub path: String,
    /// Extension enabled status.

    pub enabled: bool,
}

/// Runs an extension default action.
/// Available if the client is connected using the --remote-debugging-pipe
/// flag and the --enable-unsafe-extension-debugging flag is set.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerActionParams {
    /// Extension id.

    pub id: String,
    /// A tab target ID to trigger the default extension action on.

    pub targetId: String,
}

impl TriggerActionParams { pub const METHOD: &'static str = "Extensions.triggerAction"; }

impl crate::CdpCommand for TriggerActionParams {
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
pub struct LoadUnpackedParams {
    /// Absolute file path.

    pub path: String,
    /// Enable the extension in incognito

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enableInIncognito: Option<bool>,
}

/// Installs an unpacked extension from the filesystem similar to
/// --load-extension CLI flags. Returns extension ID once the extension
/// has been installed. Available if the client is connected using the
/// --remote-debugging-pipe flag and the --enable-unsafe-extension-debugging
/// flag is set.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadUnpackedReturns {
    /// Extension id.

    pub id: String,
}

impl LoadUnpackedParams { pub const METHOD: &'static str = "Extensions.loadUnpacked"; }

impl crate::CdpCommand for LoadUnpackedParams {
    const METHOD: &'static str = "Extensions.loadUnpacked";
    type Response = LoadUnpackedReturns;
}

/// Gets a list of all unpacked extensions.
/// Available if the client is connected using the --remote-debugging-pipe flag
/// and the --enable-unsafe-extension-debugging flag is set.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetExtensionsReturns {

    pub extensions: Vec<ExtensionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetExtensionsParams {}

impl GetExtensionsParams { pub const METHOD: &'static str = "Extensions.getExtensions"; }

impl crate::CdpCommand for GetExtensionsParams {
    const METHOD: &'static str = "Extensions.getExtensions";
    type Response = GetExtensionsReturns;
}

/// Uninstalls an unpacked extension (others not supported) from the profile.
/// Available if the client is connected using the --remote-debugging-pipe flag
/// and the --enable-unsafe-extension-debugging.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UninstallParams {
    /// Extension id.

    pub id: String,
}

impl UninstallParams { pub const METHOD: &'static str = "Extensions.uninstall"; }

impl crate::CdpCommand for UninstallParams {
    const METHOD: &'static str = "Extensions.uninstall";
    type Response = crate::EmptyReturns;
}

/// Gets data from extension storage in the given 'storageArea'. If 'keys' is
/// specified, these are used to filter the result.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageItemsParams {
    /// ID of extension.

    pub id: String,
    /// StorageArea to retrieve data from.

    pub storageArea: StorageArea,
    /// Keys to retrieve.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Gets data from extension storage in the given 'storageArea'. If 'keys' is
/// specified, these are used to filter the result.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageItemsReturns {

    pub data: serde_json::Map<String, JsonValue>,
}

impl GetStorageItemsParams { pub const METHOD: &'static str = "Extensions.getStorageItems"; }

impl crate::CdpCommand for GetStorageItemsParams {
    const METHOD: &'static str = "Extensions.getStorageItems";
    type Response = GetStorageItemsReturns;
}

/// Removes 'keys' from extension storage in the given 'storageArea'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveStorageItemsParams {
    /// ID of extension.

    pub id: String,
    /// StorageArea to remove data from.

    pub storageArea: StorageArea,
    /// Keys to remove.

    pub keys: Vec<String>,
}

impl RemoveStorageItemsParams { pub const METHOD: &'static str = "Extensions.removeStorageItems"; }

impl crate::CdpCommand for RemoveStorageItemsParams {
    const METHOD: &'static str = "Extensions.removeStorageItems";
    type Response = crate::EmptyReturns;
}

/// Clears extension storage in the given 'storageArea'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearStorageItemsParams {
    /// ID of extension.

    pub id: String,
    /// StorageArea to remove data from.

    pub storageArea: StorageArea,
}

impl ClearStorageItemsParams { pub const METHOD: &'static str = "Extensions.clearStorageItems"; }

impl crate::CdpCommand for ClearStorageItemsParams {
    const METHOD: &'static str = "Extensions.clearStorageItems";
    type Response = crate::EmptyReturns;
}

/// Sets 'values' in extension storage in the given 'storageArea'. The provided 'values'
/// will be merged with existing values in the storage area.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStorageItemsParams {
    /// ID of extension.

    pub id: String,
    /// StorageArea to set data in.

    pub storageArea: StorageArea,
    /// Values to set.

    pub values: serde_json::Map<String, JsonValue>,
}

impl SetStorageItemsParams { pub const METHOD: &'static str = "Extensions.setStorageItems"; }

impl crate::CdpCommand for SetStorageItemsParams {
    const METHOD: &'static str = "Extensions.setStorageItems";
    type Response = crate::EmptyReturns;
}
