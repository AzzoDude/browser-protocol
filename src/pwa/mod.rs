//! This domain allows interacting with the browser to control PWAs.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// The following types are the replica of
/// https://crsrc.org/c/chrome/browser/web_applications/proto/web_app_os_integration_state.proto;drc=9910d3be894c8f142c977ba1023f30a656bc13fc;l=67

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileHandlerAccept<'a> {
    /// New name of the mimetype according to
    /// https://www.iana.org/assignments/media-types/media-types.xhtml
    mediaType: Cow<'a, str>,
    fileExtensions: Vec<Cow<'a, str>>,
}

impl<'a> FileHandlerAccept<'a> {
    pub fn builder(mediaType: impl Into<Cow<'a, str>>, fileExtensions: Vec<Cow<'a, str>>) -> FileHandlerAcceptBuilder<'a> {
        FileHandlerAcceptBuilder {
            mediaType: mediaType.into(),
            fileExtensions: fileExtensions,
        }
    }
    pub fn mediaType(&self) -> &str { self.mediaType.as_ref() }
    pub fn fileExtensions(&self) -> &[Cow<'a, str>] { &self.fileExtensions }
}


pub struct FileHandlerAcceptBuilder<'a> {
    mediaType: Cow<'a, str>,
    fileExtensions: Vec<Cow<'a, str>>,
}

impl<'a> FileHandlerAcceptBuilder<'a> {
    pub fn build(self) -> FileHandlerAccept<'a> {
        FileHandlerAccept {
            mediaType: self.mediaType,
            fileExtensions: self.fileExtensions,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileHandler<'a> {
    action: Cow<'a, str>,
    accepts: Vec<FileHandlerAccept<'a>>,
    displayName: Cow<'a, str>,
}

impl<'a> FileHandler<'a> {
    pub fn builder(action: impl Into<Cow<'a, str>>, accepts: Vec<FileHandlerAccept<'a>>, displayName: impl Into<Cow<'a, str>>) -> FileHandlerBuilder<'a> {
        FileHandlerBuilder {
            action: action.into(),
            accepts: accepts,
            displayName: displayName.into(),
        }
    }
    pub fn action(&self) -> &str { self.action.as_ref() }
    pub fn accepts(&self) -> &[FileHandlerAccept<'a>] { &self.accepts }
    pub fn displayName(&self) -> &str { self.displayName.as_ref() }
}


pub struct FileHandlerBuilder<'a> {
    action: Cow<'a, str>,
    accepts: Vec<FileHandlerAccept<'a>>,
    displayName: Cow<'a, str>,
}

impl<'a> FileHandlerBuilder<'a> {
    pub fn build(self) -> FileHandler<'a> {
        FileHandler {
            action: self.action,
            accepts: self.accepts,
            displayName: self.displayName,
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
    /// https://web.dev/learn/pwa/web-app-manifest.
    manifestId: Cow<'a, str>,
}

impl<'a> GetOsAppStateParams<'a> {
    pub fn builder(manifestId: impl Into<Cow<'a, str>>) -> GetOsAppStateParamsBuilder<'a> {
        GetOsAppStateParamsBuilder {
            manifestId: manifestId.into(),
        }
    }
    pub fn manifestId(&self) -> &str { self.manifestId.as_ref() }
}


pub struct GetOsAppStateParamsBuilder<'a> {
    manifestId: Cow<'a, str>,
}

impl<'a> GetOsAppStateParamsBuilder<'a> {
    pub fn build(self) -> GetOsAppStateParams<'a> {
        GetOsAppStateParams {
            manifestId: self.manifestId,
        }
    }
}

/// Returns the following OS state for the given manifest id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOsAppStateReturns<'a> {
    badgeCount: u64,
    fileHandlers: Vec<FileHandler<'a>>,
}

impl<'a> GetOsAppStateReturns<'a> {
    pub fn builder(badgeCount: u64, fileHandlers: Vec<FileHandler<'a>>) -> GetOsAppStateReturnsBuilder<'a> {
        GetOsAppStateReturnsBuilder {
            badgeCount: badgeCount,
            fileHandlers: fileHandlers,
        }
    }
    pub fn badgeCount(&self) -> u64 { self.badgeCount }
    pub fn fileHandlers(&self) -> &[FileHandler<'a>] { &self.fileHandlers }
}


pub struct GetOsAppStateReturnsBuilder<'a> {
    badgeCount: u64,
    fileHandlers: Vec<FileHandler<'a>>,
}

impl<'a> GetOsAppStateReturnsBuilder<'a> {
    pub fn build(self) -> GetOsAppStateReturns<'a> {
        GetOsAppStateReturns {
            badgeCount: self.badgeCount,
            fileHandlers: self.fileHandlers,
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
/// https://github.com/WICG/isolated-web-apps/blob/main/Scheme.md#suffix
/// 3. Encode the entire sequence using Base32 without padding.
/// 
/// If Chrome is not in IWA dev
/// mode, the installation will fail, regardless of the state of the allowlist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallParams<'a> {
    manifestId: Cow<'a, str>,
    /// The location of the app or bundle overriding the one derived from the
    /// manifestId.
    #[serde(skip_serializing_if = "Option::is_none")]
    installUrlOrBundleUrl: Option<Cow<'a, str>>,
}

impl<'a> InstallParams<'a> {
    pub fn builder(manifestId: impl Into<Cow<'a, str>>) -> InstallParamsBuilder<'a> {
        InstallParamsBuilder {
            manifestId: manifestId.into(),
            installUrlOrBundleUrl: None,
        }
    }
    pub fn manifestId(&self) -> &str { self.manifestId.as_ref() }
    pub fn installUrlOrBundleUrl(&self) -> Option<&str> { self.installUrlOrBundleUrl.as_deref() }
}


pub struct InstallParamsBuilder<'a> {
    manifestId: Cow<'a, str>,
    installUrlOrBundleUrl: Option<Cow<'a, str>>,
}

impl<'a> InstallParamsBuilder<'a> {
    /// The location of the app or bundle overriding the one derived from the
    /// manifestId.
    pub fn installUrlOrBundleUrl(mut self, installUrlOrBundleUrl: impl Into<Cow<'a, str>>) -> Self { self.installUrlOrBundleUrl = Some(installUrlOrBundleUrl.into()); self }
    pub fn build(self) -> InstallParams<'a> {
        InstallParams {
            manifestId: self.manifestId,
            installUrlOrBundleUrl: self.installUrlOrBundleUrl,
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
    manifestId: Cow<'a, str>,
}

impl<'a> UninstallParams<'a> {
    pub fn builder(manifestId: impl Into<Cow<'a, str>>) -> UninstallParamsBuilder<'a> {
        UninstallParamsBuilder {
            manifestId: manifestId.into(),
        }
    }
    pub fn manifestId(&self) -> &str { self.manifestId.as_ref() }
}


pub struct UninstallParamsBuilder<'a> {
    manifestId: Cow<'a, str>,
}

impl<'a> UninstallParamsBuilder<'a> {
    pub fn build(self) -> UninstallParams<'a> {
        UninstallParams {
            manifestId: self.manifestId,
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
    manifestId: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
}

impl<'a> LaunchParams<'a> {
    pub fn builder(manifestId: impl Into<Cow<'a, str>>) -> LaunchParamsBuilder<'a> {
        LaunchParamsBuilder {
            manifestId: manifestId.into(),
            url: None,
        }
    }
    pub fn manifestId(&self) -> &str { self.manifestId.as_ref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
}


pub struct LaunchParamsBuilder<'a> {
    manifestId: Cow<'a, str>,
    url: Option<Cow<'a, str>>,
}

impl<'a> LaunchParamsBuilder<'a> {
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> LaunchParams<'a> {
        LaunchParams {
            manifestId: self.manifestId,
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
    targetId: crate::target::TargetID<'a>,
}

impl<'a> LaunchReturns<'a> {
    pub fn builder(targetId: crate::target::TargetID<'a>) -> LaunchReturnsBuilder<'a> {
        LaunchReturnsBuilder {
            targetId: targetId,
        }
    }
    pub fn targetId(&self) -> &crate::target::TargetID<'a> { &self.targetId }
}


pub struct LaunchReturnsBuilder<'a> {
    targetId: crate::target::TargetID<'a>,
}

impl<'a> LaunchReturnsBuilder<'a> {
    pub fn build(self) -> LaunchReturns<'a> {
        LaunchReturns {
            targetId: self.targetId,
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
    manifestId: Cow<'a, str>,
    files: Vec<Cow<'a, str>>,
}

impl<'a> LaunchFilesInAppParams<'a> {
    pub fn builder(manifestId: impl Into<Cow<'a, str>>, files: Vec<Cow<'a, str>>) -> LaunchFilesInAppParamsBuilder<'a> {
        LaunchFilesInAppParamsBuilder {
            manifestId: manifestId.into(),
            files: files,
        }
    }
    pub fn manifestId(&self) -> &str { self.manifestId.as_ref() }
    pub fn files(&self) -> &[Cow<'a, str>] { &self.files }
}


pub struct LaunchFilesInAppParamsBuilder<'a> {
    manifestId: Cow<'a, str>,
    files: Vec<Cow<'a, str>>,
}

impl<'a> LaunchFilesInAppParamsBuilder<'a> {
    pub fn build(self) -> LaunchFilesInAppParams<'a> {
        LaunchFilesInAppParams {
            manifestId: self.manifestId,
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
    targetIds: Vec<crate::target::TargetID<'a>>,
}

impl<'a> LaunchFilesInAppReturns<'a> {
    pub fn builder(targetIds: Vec<crate::target::TargetID<'a>>) -> LaunchFilesInAppReturnsBuilder<'a> {
        LaunchFilesInAppReturnsBuilder {
            targetIds: targetIds,
        }
    }
    pub fn targetIds(&self) -> &[crate::target::TargetID<'a>] { &self.targetIds }
}


pub struct LaunchFilesInAppReturnsBuilder<'a> {
    targetIds: Vec<crate::target::TargetID<'a>>,
}

impl<'a> LaunchFilesInAppReturnsBuilder<'a> {
    pub fn build(self) -> LaunchFilesInAppReturns<'a> {
        LaunchFilesInAppReturns {
            targetIds: self.targetIds,
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
    manifestId: Cow<'a, str>,
}

impl<'a> OpenCurrentPageInAppParams<'a> {
    pub fn builder(manifestId: impl Into<Cow<'a, str>>) -> OpenCurrentPageInAppParamsBuilder<'a> {
        OpenCurrentPageInAppParamsBuilder {
            manifestId: manifestId.into(),
        }
    }
    pub fn manifestId(&self) -> &str { self.manifestId.as_ref() }
}


pub struct OpenCurrentPageInAppParamsBuilder<'a> {
    manifestId: Cow<'a, str>,
}

impl<'a> OpenCurrentPageInAppParamsBuilder<'a> {
    pub fn build(self) -> OpenCurrentPageInAppParams<'a> {
        OpenCurrentPageInAppParams {
            manifestId: self.manifestId,
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
    manifestId: Cow<'a, str>,
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
    linkCapturing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    displayMode: Option<DisplayMode>,
}

impl<'a> ChangeAppUserSettingsParams<'a> {
    pub fn builder(manifestId: impl Into<Cow<'a, str>>) -> ChangeAppUserSettingsParamsBuilder<'a> {
        ChangeAppUserSettingsParamsBuilder {
            manifestId: manifestId.into(),
            linkCapturing: None,
            displayMode: None,
        }
    }
    pub fn manifestId(&self) -> &str { self.manifestId.as_ref() }
    pub fn linkCapturing(&self) -> Option<bool> { self.linkCapturing }
    pub fn displayMode(&self) -> Option<&DisplayMode> { self.displayMode.as_ref() }
}


pub struct ChangeAppUserSettingsParamsBuilder<'a> {
    manifestId: Cow<'a, str>,
    linkCapturing: Option<bool>,
    displayMode: Option<DisplayMode>,
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
    pub fn linkCapturing(mut self, linkCapturing: bool) -> Self { self.linkCapturing = Some(linkCapturing); self }
    pub fn displayMode(mut self, displayMode: impl Into<DisplayMode>) -> Self { self.displayMode = Some(displayMode.into()); self }
    pub fn build(self) -> ChangeAppUserSettingsParams<'a> {
        ChangeAppUserSettingsParams {
            manifestId: self.manifestId,
            linkCapturing: self.linkCapturing,
            displayMode: self.displayMode,
        }
    }
}

impl<'a> ChangeAppUserSettingsParams<'a> { pub const METHOD: &'static str = "PWA.changeAppUserSettings"; }

impl<'a> crate::CdpCommand<'a> for ChangeAppUserSettingsParams<'a> {
    const METHOD: &'static str = "PWA.changeAppUserSettings";
    type Response = crate::EmptyReturns;
}
