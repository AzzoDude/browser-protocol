//! This domain allows interacting with the browser to control PWAs.

use serde::{Serialize, Deserialize};

/// The following types are the replica of
/// <https://crsrc.org/c/chrome/browser/web_applications/proto/web_app_os_integration_state.proto;drc=9910d3be894c8f142c977ba1023f30a656bc13fc;l=67>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileHandlerAccept {
    /// New name of the mimetype according to
    /// <https://www.iana.org/assignments/media-types/media-types.xhtml>

    pub mediaType: String,

    pub fileExtensions: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileHandler {

    pub action: String,

    pub accepts: Vec<FileHandlerAccept>,

    pub displayName: String,
}

/// If user prefers opening the app in browser or an app window.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DisplayMode {
    #[default]
    Standalone,
    Browser,
}

/// Returns the following OS state for the given manifest id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOsAppStateParams {
    /// The id from the webapp's manifest file, commonly it's the url of the
    /// site installing the webapp. See
    /// <https://web.dev/learn/pwa/web-app-manifest.>

    pub manifestId: String,
}

/// Returns the following OS state for the given manifest id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOsAppStateReturns {

    pub badgeCount: u64,

    pub fileHandlers: Vec<FileHandler>,
}

/// Installs the given manifest identity, optionally using the given installUrlOrBundleUrl
/// 
/// IWA-specific install description:
/// manifestId corresponds to isolated-app:// + web_package::SignedWebBundleId
/// 
/// File installation mode:
/// The installUrlOrBundleUrl can be either file:// or http(s):// pointing
/// to a signed web bundle (.swbn). In this case SignedWebBundleId must correspond to
/// The .swbn file's signing key.
/// 
/// Dev proxy installation mode:
/// installUrlOrBundleUrl must be http(s):// that serves dev mode IWA.
/// web_package::SignedWebBundleId must be of type dev proxy.
/// 
/// The advantage of dev proxy mode is that all changes to IWA
/// automatically will be reflected in the running app without
/// reinstallation.
/// 
/// To generate bundle id for proxy mode:
/// 1. Generate 32 random bytes.
/// 2. Add a specific suffix at the end following the documentation
/// <https://github.com/WICG/isolated-web-apps/blob/main/Scheme.md#suffix>
/// 3. Encode the entire sequence using Base32 without padding.
/// 
/// If Chrome is not in IWA dev
/// mode, the installation will fail, regardless of the state of the allowlist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallParams {

    pub manifestId: String,
    /// The location of the app or bundle overriding the one derived from the
    /// manifestId.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub installUrlOrBundleUrl: Option<String>,
}

/// Uninstalls the given manifest_id and closes any opened app windows.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UninstallParams {

    pub manifestId: String,
}

/// Launches the installed web app, or an url in the same web app instead of the
/// default start url if it is provided. Returns a page Target.TargetID which
/// can be used to attach to via Target.attachToTarget or similar APIs.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchParams {

    pub manifestId: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Launches the installed web app, or an url in the same web app instead of the
/// default start url if it is provided. Returns a page Target.TargetID which
/// can be used to attach to via Target.attachToTarget or similar APIs.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchReturns {
    /// ID of the tab target created as a result.

    pub targetId: crate::target::TargetID,
}

/// Opens one or more local files from an installed web app identified by its
/// manifestId. The web app needs to have file handlers registered to process
/// the files. The API returns one or more page Target.TargetIDs which can be
/// used to attach to via Target.attachToTarget or similar APIs.
/// If some files in the parameters cannot be handled by the web app, they will
/// be ignored. If none of the files can be handled, this API returns an error.
/// If no files are provided as the parameter, this API also returns an error.
/// 
/// According to the definition of the file handlers in the manifest file, one
/// Target.TargetID may represent a page handling one or more files. The order
/// of the returned Target.TargetIDs is not guaranteed.
/// 
/// TODO(crbug.com/339454034): Check the existences of the input files.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchFilesInAppParams {

    pub manifestId: String,

    pub files: Vec<String>,
}

/// Opens one or more local files from an installed web app identified by its
/// manifestId. The web app needs to have file handlers registered to process
/// the files. The API returns one or more page Target.TargetIDs which can be
/// used to attach to via Target.attachToTarget or similar APIs.
/// If some files in the parameters cannot be handled by the web app, they will
/// be ignored. If none of the files can be handled, this API returns an error.
/// If no files are provided as the parameter, this API also returns an error.
/// 
/// According to the definition of the file handlers in the manifest file, one
/// Target.TargetID may represent a page handling one or more files. The order
/// of the returned Target.TargetIDs is not guaranteed.
/// 
/// TODO(crbug.com/339454034): Check the existences of the input files.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchFilesInAppReturns {
    /// IDs of the tab targets created as the result.

    pub targetIds: Vec<crate::target::TargetID>,
}

/// Opens the current page in its web app identified by the manifest id, needs
/// to be called on a page target. This function returns immediately without
/// waiting for the app to finish loading.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenCurrentPageInAppParams {

    pub manifestId: String,
}

/// Changes user settings of the web app identified by its manifestId. If the
/// app was not installed, this command returns an error. Unset parameters will
/// be ignored; unrecognized values will cause an error.
/// 
/// Unlike the ones defined in the manifest files of the web apps, these
/// settings are provided by the browser and controlled by the users, they
/// impact the way the browser handling the web apps.
/// 
/// See the comment of each parameter.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAppUserSettingsParams {

    pub manifestId: String,
    /// If user allows the links clicked on by the user in the app's scope, or
    /// extended scope if the manifest has scope extensions and the flags
    /// 'DesktopPWAsLinkCapturingWithScopeExtensions' and
    /// 'WebAppEnableScopeExtensions' are enabled.
    /// 
    /// Note, the API does not support resetting the linkCapturing to the
    /// initial value, uninstalling and installing the web app again will reset
    /// it.
    /// 
    /// TODO(crbug.com/339453269): Setting this value on ChromeOS is not
    /// supported yet.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkCapturing: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayMode: Option<DisplayMode>,
}
