//! Defines commands and events for browser extensions.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionInfo<'a> {
    /// Extension id.
    pub id: Cow<'a, str>,
    /// Extension name.
    pub name: Cow<'a, str>,
    /// Extension version.
    pub version: Cow<'a, str>,
    /// The path from which the extension was loaded.
    pub path: Cow<'a, str>,
    /// Extension enabled status.
    pub enabled: bool,
}
/// Runs an extension default action.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Extensions.triggerAction")]
pub struct TriggerActionParams<'a> {
    /// Extension id.
    pub id: Cow<'a, str>,
    /// A tab target ID to trigger the default extension action on.
    #[serde(rename = "targetId")]
    pub target_id: Cow<'a, str>,
}
/// Installs an unpacked extension from the filesystem similar to
/// --load-extension CLI flags. Returns extension ID once the extension
/// has been installed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Extensions.loadUnpacked", response = "LoadUnpackedReturns<'a>")]
pub struct LoadUnpackedParams<'a> {
    /// Absolute file path.
    pub path: Cow<'a, str>,
    /// Enable the extension in incognito
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableInIncognito")]
    pub enable_in_incognito: Option<bool>,
}
/// Installs an unpacked extension from the filesystem similar to
/// --load-extension CLI flags. Returns extension ID once the extension
/// has been installed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LoadUnpackedReturns<'a> {
    /// Extension id.
    pub id: Cow<'a, str>,
}
/// Gets a list of all unpacked extensions.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Extensions.getExtensions", response = "GetExtensionsReturns<'a>")]
pub struct GetExtensionsParams {

}
/// Gets a list of all unpacked extensions.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetExtensionsReturns<'a> {
    pub extensions: Vec<ExtensionInfo<'a>>,
}
/// Uninstalls an unpacked extension (others not supported) from the profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Extensions.uninstall")]
pub struct UninstallParams<'a> {
    /// Extension id.
    pub id: Cow<'a, str>,
}
/// Gets data from extension storage in the given 'storageArea'. If 'keys' is
/// specified, these are used to filter the result.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Extensions.getStorageItems", response = "GetStorageItemsReturns")]
pub struct GetStorageItemsParams<'a> {
    /// ID of extension.
    pub id: Cow<'a, str>,
    /// StorageArea to retrieve data from.
    #[serde(rename = "storageArea")]
    pub storage_area: StorageArea,
    /// Keys to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<Cow<'a, str>>>,
}
/// Gets data from extension storage in the given 'storageArea'. If 'keys' is
/// specified, these are used to filter the result.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageItemsReturns {
    pub data: serde_json::Map<String, JsonValue>,
}
/// Removes 'keys' from extension storage in the given 'storageArea'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Extensions.removeStorageItems")]
pub struct RemoveStorageItemsParams<'a> {
    /// ID of extension.
    pub id: Cow<'a, str>,
    /// StorageArea to remove data from.
    #[serde(rename = "storageArea")]
    pub storage_area: StorageArea,
    /// Keys to remove.
    pub keys: Vec<Cow<'a, str>>,
}
/// Clears extension storage in the given 'storageArea'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Extensions.clearStorageItems")]
pub struct ClearStorageItemsParams<'a> {
    /// ID of extension.
    pub id: Cow<'a, str>,
    /// StorageArea to remove data from.
    #[serde(rename = "storageArea")]
    pub storage_area: StorageArea,
}
/// Sets 'values' in extension storage in the given 'storageArea'. The provided 'values'
/// will be merged with existing values in the storage area.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Extensions.setStorageItems")]
pub struct SetStorageItemsParams<'a> {
    /// ID of extension.
    pub id: Cow<'a, str>,
    /// StorageArea to set data in.
    #[serde(rename = "storageArea")]
    pub storage_area: StorageArea,
    /// Values to set.
    pub values: serde_json::Map<String, JsonValue>,
}