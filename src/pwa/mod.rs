//! This domain allows interacting with the browser to control PWAs.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// The following types are the replica of
/// <https://crsrc.org/c/chrome/browser/web_applications/proto/web_app_os_integration_state.proto;drc=9910d3be894c8f142c977ba1023f30a656bc13fc;l=67>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileHandlerAccept<'a> {
    /// New name of the mimetype according to
    /// <https://www.iana.org/assignments/media-types/media-types.xhtml>
    #[serde(rename = "mediaType")]
    media_type: Cow<'a, str>,
    #[serde(rename = "fileExtensions")]
    file_extensions: Vec<Cow<'a, str>>,
}

impl<'a> FileHandlerAccept<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `media_type`: New name of the mimetype according to <https://www.iana.org/assignments/media-types/media-types.xhtml>
    /// * `file_extensions`: 
    pub fn builder(media_type: impl Into<Cow<'a, str>>, file_extensions: Vec<Cow<'a, str>>) -> FileHandlerAcceptBuilder<'a> {
        FileHandlerAcceptBuilder {
            media_type: media_type.into(),
            file_extensions: file_extensions,
        }
    }
    /// New name of the mimetype according to
    /// <https://www.iana.org/assignments/media-types/media-types.xhtml>
    pub fn media_type(&self) -> &str { self.media_type.as_ref() }
    pub fn file_extensions(&self) -> &[Cow<'a, str>] { &self.file_extensions }
}


pub struct FileHandlerAcceptBuilder<'a> {
    media_type: Cow<'a, str>,
    file_extensions: Vec<Cow<'a, str>>,
}

impl<'a> FileHandlerAcceptBuilder<'a> {
    pub fn build(self) -> FileHandlerAccept<'a> {
        FileHandlerAccept {
            media_type: self.media_type,
            file_extensions: self.file_extensions,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileHandler<'a> {
    action: Cow<'a, str>,
    accepts: Vec<FileHandlerAccept<'a>>,
    #[serde(rename = "displayName")]
    display_name: Cow<'a, str>,
}

impl<'a> FileHandler<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `action`: 
    /// * `accepts`: 
    /// * `display_name`: 
    pub fn builder(action: impl Into<Cow<'a, str>>, accepts: Vec<FileHandlerAccept<'a>>, display_name: impl Into<Cow<'a, str>>) -> FileHandlerBuilder<'a> {
        FileHandlerBuilder {
            action: action.into(),
            accepts: accepts,
            display_name: display_name.into(),
        }
    }
    pub fn action(&self) -> &str { self.action.as_ref() }
    pub fn accepts(&self) -> &[FileHandlerAccept<'a>] { &self.accepts }
    pub fn display_name(&self) -> &str { self.display_name.as_ref() }
}


pub struct FileHandlerBuilder<'a> {
    action: Cow<'a, str>,
    accepts: Vec<FileHandlerAccept<'a>>,
    display_name: Cow<'a, str>,
}

impl<'a> FileHandlerBuilder<'a> {
    pub fn build(self) -> FileHandler<'a> {
        FileHandler {
            action: self.action,
            accepts: self.accepts,
            display_name: self.display_name,
        }
    }
}

/// If user prefers opening the app in browser or an app window.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DisplayMode {
    #[default]
    #[serde(rename = "standalone")]
    Standalone,
    #[serde(rename = "browser")]
    Browser,
}

/// Returns the following OS state for the given manifest id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOsAppStateParams<'a> {
    /// The id from the webapp's manifest file, commonly it's the url of the
    /// site installing the webapp. See
    /// <https://web.dev/learn/pwa/web-app-manifest>.
    #[serde(rename = "manifestId")]
    manifest_id: Cow<'a, str>,
}

impl<'a> GetOsAppStateParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `manifest_id`: The id from the webapp's manifest file, commonly it's the url of the site installing the webapp. See <https://web.dev/learn/pwa/web-app-manifest>.
    pub fn builder(manifest_id: impl Into<Cow<'a, str>>) -> GetOsAppStateParamsBuilder<'a> {
        GetOsAppStateParamsBuilder {
            manifest_id: manifest_id.into(),
        }
    }
    /// The id from the webapp's manifest file, commonly it's the url of the
    /// site installing the webapp. See
    /// <https://web.dev/learn/pwa/web-app-manifest>.
    pub fn manifest_id(&self) -> &str { self.manifest_id.as_ref() }
}


pub struct GetOsAppStateParamsBuilder<'a> {
    manifest_id: Cow<'a, str>,
}

impl<'a> GetOsAppStateParamsBuilder<'a> {
    pub fn build(self) -> GetOsAppStateParams<'a> {
        GetOsAppStateParams {
            manifest_id: self.manifest_id,
        }
    }
}

/// Returns the following OS state for the given manifest id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOsAppStateReturns<'a> {
    #[serde(rename = "badgeCount")]
    badge_count: u64,
    #[serde(rename = "fileHandlers")]
    file_handlers: Vec<FileHandler<'a>>,
}

impl<'a> GetOsAppStateReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `badge_count`: 
    /// * `file_handlers`: 
    pub fn builder(badge_count: u64, file_handlers: Vec<FileHandler<'a>>) -> GetOsAppStateReturnsBuilder<'a> {
        GetOsAppStateReturnsBuilder {
            badge_count: badge_count,
            file_handlers: file_handlers,
        }
    }
    pub fn badge_count(&self) -> u64 { self.badge_count }
    pub fn file_handlers(&self) -> &[FileHandler<'a>] { &self.file_handlers }
}


pub struct GetOsAppStateReturnsBuilder<'a> {
    badge_count: u64,
    file_handlers: Vec<FileHandler<'a>>,
}

impl<'a> GetOsAppStateReturnsBuilder<'a> {
    pub fn build(self) -> GetOsAppStateReturns<'a> {
        GetOsAppStateReturns {
            badge_count: self.badge_count,
            file_handlers: self.file_handlers,
        }
    }
}

impl<'a> GetOsAppStateParams<'a> { pub const METHOD: &'static str = "PWA.getOsAppState"; }

impl<'a> crate::CdpCommand<'a> for GetOsAppStateParams<'a> {
    const METHOD: &'static str = "PWA.getOsAppState";
    type Response = GetOsAppStateReturns<'a>;
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
pub struct InstallParams<'a> {
    #[serde(rename = "manifestId")]
    manifest_id: Cow<'a, str>,
    /// The location of the app or bundle overriding the one derived from the
    /// manifestId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "installUrlOrBundleUrl")]
    install_url_or_bundle_url: Option<Cow<'a, str>>,
}

impl<'a> InstallParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `manifest_id`: 
    pub fn builder(manifest_id: impl Into<Cow<'a, str>>) -> InstallParamsBuilder<'a> {
        InstallParamsBuilder {
            manifest_id: manifest_id.into(),
            install_url_or_bundle_url: None,
        }
    }
    pub fn manifest_id(&self) -> &str { self.manifest_id.as_ref() }
    /// The location of the app or bundle overriding the one derived from the
    /// manifestId.
    pub fn install_url_or_bundle_url(&self) -> Option<&str> { self.install_url_or_bundle_url.as_deref() }
}


pub struct InstallParamsBuilder<'a> {
    manifest_id: Cow<'a, str>,
    install_url_or_bundle_url: Option<Cow<'a, str>>,
}

impl<'a> InstallParamsBuilder<'a> {
    /// The location of the app or bundle overriding the one derived from the
    /// manifestId.
    pub fn install_url_or_bundle_url(mut self, install_url_or_bundle_url: impl Into<Cow<'a, str>>) -> Self { self.install_url_or_bundle_url = Some(install_url_or_bundle_url.into()); self }
    pub fn build(self) -> InstallParams<'a> {
        InstallParams {
            manifest_id: self.manifest_id,
            install_url_or_bundle_url: self.install_url_or_bundle_url,
        }
    }
}

impl<'a> InstallParams<'a> { pub const METHOD: &'static str = "PWA.install"; }

impl<'a> crate::CdpCommand<'a> for InstallParams<'a> {
    const METHOD: &'static str = "PWA.install";
    type Response = crate::EmptyReturns;
}

/// Uninstalls the given manifest_id and closes any opened app windows.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UninstallParams<'a> {
    #[serde(rename = "manifestId")]
    manifest_id: Cow<'a, str>,
}

impl<'a> UninstallParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `manifest_id`: 
    pub fn builder(manifest_id: impl Into<Cow<'a, str>>) -> UninstallParamsBuilder<'a> {
        UninstallParamsBuilder {
            manifest_id: manifest_id.into(),
        }
    }
    pub fn manifest_id(&self) -> &str { self.manifest_id.as_ref() }
}


pub struct UninstallParamsBuilder<'a> {
    manifest_id: Cow<'a, str>,
}

impl<'a> UninstallParamsBuilder<'a> {
    pub fn build(self) -> UninstallParams<'a> {
        UninstallParams {
            manifest_id: self.manifest_id,
        }
    }
}

impl<'a> UninstallParams<'a> { pub const METHOD: &'static str = "PWA.uninstall"; }

impl<'a> crate::CdpCommand<'a> for UninstallParams<'a> {
    const METHOD: &'static str = "PWA.uninstall";
    type Response = crate::EmptyReturns;
}

/// Launches the installed web app, or an url in the same web app instead of the
/// default start url if it is provided. Returns a page Target.TargetID which
/// can be used to attach to via Target.attachToTarget or similar APIs.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchParams<'a> {
    #[serde(rename = "manifestId")]
    manifest_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
}

impl<'a> LaunchParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `manifest_id`: 
    pub fn builder(manifest_id: impl Into<Cow<'a, str>>) -> LaunchParamsBuilder<'a> {
        LaunchParamsBuilder {
            manifest_id: manifest_id.into(),
            url: None,
        }
    }
    pub fn manifest_id(&self) -> &str { self.manifest_id.as_ref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
}


pub struct LaunchParamsBuilder<'a> {
    manifest_id: Cow<'a, str>,
    url: Option<Cow<'a, str>>,
}

impl<'a> LaunchParamsBuilder<'a> {
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> LaunchParams<'a> {
        LaunchParams {
            manifest_id: self.manifest_id,
            url: self.url,
        }
    }
}

/// Launches the installed web app, or an url in the same web app instead of the
/// default start url if it is provided. Returns a page Target.TargetID which
/// can be used to attach to via Target.attachToTarget or similar APIs.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchReturns<'a> {
    /// ID of the tab target created as a result.
    #[serde(rename = "targetId")]
    target_id: crate::target::TargetID<'a>,
}

impl<'a> LaunchReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: ID of the tab target created as a result.
    pub fn builder(target_id: crate::target::TargetID<'a>) -> LaunchReturnsBuilder<'a> {
        LaunchReturnsBuilder {
            target_id: target_id,
        }
    }
    /// ID of the tab target created as a result.
    pub fn target_id(&self) -> &crate::target::TargetID<'a> { &self.target_id }
}


pub struct LaunchReturnsBuilder<'a> {
    target_id: crate::target::TargetID<'a>,
}

impl<'a> LaunchReturnsBuilder<'a> {
    pub fn build(self) -> LaunchReturns<'a> {
        LaunchReturns {
            target_id: self.target_id,
        }
    }
}

impl<'a> LaunchParams<'a> { pub const METHOD: &'static str = "PWA.launch"; }

impl<'a> crate::CdpCommand<'a> for LaunchParams<'a> {
    const METHOD: &'static str = "PWA.launch";
    type Response = LaunchReturns<'a>;
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
pub struct LaunchFilesInAppParams<'a> {
    #[serde(rename = "manifestId")]
    manifest_id: Cow<'a, str>,
    files: Vec<Cow<'a, str>>,
}

impl<'a> LaunchFilesInAppParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `manifest_id`: 
    /// * `files`: 
    pub fn builder(manifest_id: impl Into<Cow<'a, str>>, files: Vec<Cow<'a, str>>) -> LaunchFilesInAppParamsBuilder<'a> {
        LaunchFilesInAppParamsBuilder {
            manifest_id: manifest_id.into(),
            files: files,
        }
    }
    pub fn manifest_id(&self) -> &str { self.manifest_id.as_ref() }
    pub fn files(&self) -> &[Cow<'a, str>] { &self.files }
}


pub struct LaunchFilesInAppParamsBuilder<'a> {
    manifest_id: Cow<'a, str>,
    files: Vec<Cow<'a, str>>,
}

impl<'a> LaunchFilesInAppParamsBuilder<'a> {
    pub fn build(self) -> LaunchFilesInAppParams<'a> {
        LaunchFilesInAppParams {
            manifest_id: self.manifest_id,
            files: self.files,
        }
    }
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
pub struct LaunchFilesInAppReturns<'a> {
    /// IDs of the tab targets created as the result.
    #[serde(rename = "targetIds")]
    target_ids: Vec<crate::target::TargetID<'a>>,
}

impl<'a> LaunchFilesInAppReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_ids`: IDs of the tab targets created as the result.
    pub fn builder(target_ids: Vec<crate::target::TargetID<'a>>) -> LaunchFilesInAppReturnsBuilder<'a> {
        LaunchFilesInAppReturnsBuilder {
            target_ids: target_ids,
        }
    }
    /// IDs of the tab targets created as the result.
    pub fn target_ids(&self) -> &[crate::target::TargetID<'a>] { &self.target_ids }
}


pub struct LaunchFilesInAppReturnsBuilder<'a> {
    target_ids: Vec<crate::target::TargetID<'a>>,
}

impl<'a> LaunchFilesInAppReturnsBuilder<'a> {
    pub fn build(self) -> LaunchFilesInAppReturns<'a> {
        LaunchFilesInAppReturns {
            target_ids: self.target_ids,
        }
    }
}

impl<'a> LaunchFilesInAppParams<'a> { pub const METHOD: &'static str = "PWA.launchFilesInApp"; }

impl<'a> crate::CdpCommand<'a> for LaunchFilesInAppParams<'a> {
    const METHOD: &'static str = "PWA.launchFilesInApp";
    type Response = LaunchFilesInAppReturns<'a>;
}

/// Opens the current page in its web app identified by the manifest id, needs
/// to be called on a page target. This function returns immediately without
/// waiting for the app to finish loading.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenCurrentPageInAppParams<'a> {
    #[serde(rename = "manifestId")]
    manifest_id: Cow<'a, str>,
}

impl<'a> OpenCurrentPageInAppParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `manifest_id`: 
    pub fn builder(manifest_id: impl Into<Cow<'a, str>>) -> OpenCurrentPageInAppParamsBuilder<'a> {
        OpenCurrentPageInAppParamsBuilder {
            manifest_id: manifest_id.into(),
        }
    }
    pub fn manifest_id(&self) -> &str { self.manifest_id.as_ref() }
}


pub struct OpenCurrentPageInAppParamsBuilder<'a> {
    manifest_id: Cow<'a, str>,
}

impl<'a> OpenCurrentPageInAppParamsBuilder<'a> {
    pub fn build(self) -> OpenCurrentPageInAppParams<'a> {
        OpenCurrentPageInAppParams {
            manifest_id: self.manifest_id,
        }
    }
}

impl<'a> OpenCurrentPageInAppParams<'a> { pub const METHOD: &'static str = "PWA.openCurrentPageInApp"; }

impl<'a> crate::CdpCommand<'a> for OpenCurrentPageInAppParams<'a> {
    const METHOD: &'static str = "PWA.openCurrentPageInApp";
    type Response = crate::EmptyReturns;
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
pub struct ChangeAppUserSettingsParams<'a> {
    #[serde(rename = "manifestId")]
    manifest_id: Cow<'a, str>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "linkCapturing")]
    link_capturing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayMode")]
    display_mode: Option<DisplayMode>,
}

impl<'a> ChangeAppUserSettingsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `manifest_id`: 
    pub fn builder(manifest_id: impl Into<Cow<'a, str>>) -> ChangeAppUserSettingsParamsBuilder<'a> {
        ChangeAppUserSettingsParamsBuilder {
            manifest_id: manifest_id.into(),
            link_capturing: None,
            display_mode: None,
        }
    }
    pub fn manifest_id(&self) -> &str { self.manifest_id.as_ref() }
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
    pub fn link_capturing(&self) -> Option<bool> { self.link_capturing }
    pub fn display_mode(&self) -> Option<&DisplayMode> { self.display_mode.as_ref() }
}


pub struct ChangeAppUserSettingsParamsBuilder<'a> {
    manifest_id: Cow<'a, str>,
    link_capturing: Option<bool>,
    display_mode: Option<DisplayMode>,
}

impl<'a> ChangeAppUserSettingsParamsBuilder<'a> {
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
    pub fn link_capturing(mut self, link_capturing: bool) -> Self { self.link_capturing = Some(link_capturing); self }
    pub fn display_mode(mut self, display_mode: impl Into<DisplayMode>) -> Self { self.display_mode = Some(display_mode.into()); self }
    pub fn build(self) -> ChangeAppUserSettingsParams<'a> {
        ChangeAppUserSettingsParams {
            manifest_id: self.manifest_id,
            link_capturing: self.link_capturing,
            display_mode: self.display_mode,
        }
    }
}

impl<'a> ChangeAppUserSettingsParams<'a> { pub const METHOD: &'static str = "PWA.changeAppUserSettings"; }

impl<'a> crate::CdpCommand<'a> for ChangeAppUserSettingsParams<'a> {
    const METHOD: &'static str = "PWA.changeAppUserSettings";
    type Response = crate::EmptyReturns;
}
