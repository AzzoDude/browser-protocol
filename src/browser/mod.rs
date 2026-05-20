//! The Browser domain defines methods and events for browser managing.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


pub type BrowserContextID<'a> = Cow<'a, str>;


pub type WindowID = i64;

/// The state of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum WindowState {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "minimized")]
    Minimized,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "fullscreen")]
    Fullscreen,
}

/// Browser window bounds information

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Bounds {
    /// The offset from the left edge of the screen to the window in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<i64>,
    /// The offset from the top edge of the screen to the window in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<i64>,
    /// The window width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u64>,
    /// The window height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
    /// The window state. Default to normal.
    #[serde(skip_serializing_if = "Option::is_none")]
    windowState: Option<WindowState>,
}

impl Bounds {
    pub fn builder() -> BoundsBuilder {
        BoundsBuilder {
            left: None,
            top: None,
            width: None,
            height: None,
            windowState: None,
        }
    }
    pub fn left(&self) -> Option<i64> { self.left }
    pub fn top(&self) -> Option<i64> { self.top }
    pub fn width(&self) -> Option<u64> { self.width }
    pub fn height(&self) -> Option<i64> { self.height }
    pub fn windowState(&self) -> Option<&WindowState> { self.windowState.as_ref() }
}

#[derive(Default)]
pub struct BoundsBuilder {
    left: Option<i64>,
    top: Option<i64>,
    width: Option<u64>,
    height: Option<i64>,
    windowState: Option<WindowState>,
}

impl BoundsBuilder {
    /// The offset from the left edge of the screen to the window in pixels.
    pub fn left(mut self, left: i64) -> Self { self.left = Some(left); self }
    /// The offset from the top edge of the screen to the window in pixels.
    pub fn top(mut self, top: i64) -> Self { self.top = Some(top); self }
    /// The window width in pixels.
    pub fn width(mut self, width: u64) -> Self { self.width = Some(width); self }
    /// The window height in pixels.
    pub fn height(mut self, height: i64) -> Self { self.height = Some(height); self }
    /// The window state. Default to normal.
    pub fn windowState(mut self, windowState: WindowState) -> Self { self.windowState = Some(windowState); self }
    pub fn build(self) -> Bounds {
        Bounds {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            windowState: self.windowState,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionType {
    #[default]
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "audioCapture")]
    AudioCapture,
    #[serde(rename = "automaticFullscreen")]
    AutomaticFullscreen,
    #[serde(rename = "backgroundFetch")]
    BackgroundFetch,
    #[serde(rename = "backgroundSync")]
    BackgroundSync,
    #[serde(rename = "cameraPanTiltZoom")]
    CameraPanTiltZoom,
    #[serde(rename = "capturedSurfaceControl")]
    CapturedSurfaceControl,
    #[serde(rename = "clipboardReadWrite")]
    ClipboardReadWrite,
    #[serde(rename = "clipboardSanitizedWrite")]
    ClipboardSanitizedWrite,
    #[serde(rename = "displayCapture")]
    DisplayCapture,
    #[serde(rename = "durableStorage")]
    DurableStorage,
    #[serde(rename = "geolocation")]
    Geolocation,
    #[serde(rename = "handTracking")]
    HandTracking,
    #[serde(rename = "idleDetection")]
    IdleDetection,
    #[serde(rename = "keyboardLock")]
    KeyboardLock,
    #[serde(rename = "localFonts")]
    LocalFonts,
    #[serde(rename = "localNetwork")]
    LocalNetwork,
    #[serde(rename = "localNetworkAccess")]
    LocalNetworkAccess,
    #[serde(rename = "loopbackNetwork")]
    LoopbackNetwork,
    #[serde(rename = "midi")]
    Midi,
    #[serde(rename = "midiSysex")]
    MidiSysex,
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "paymentHandler")]
    PaymentHandler,
    #[serde(rename = "periodicBackgroundSync")]
    PeriodicBackgroundSync,
    #[serde(rename = "pointerLock")]
    PointerLock,
    #[serde(rename = "protectedMediaIdentifier")]
    ProtectedMediaIdentifier,
    #[serde(rename = "sensors")]
    Sensors,
    #[serde(rename = "smartCard")]
    SmartCard,
    #[serde(rename = "speakerSelection")]
    SpeakerSelection,
    #[serde(rename = "storageAccess")]
    StorageAccess,
    #[serde(rename = "topLevelStorageAccess")]
    TopLevelStorageAccess,
    #[serde(rename = "videoCapture")]
    VideoCapture,
    #[serde(rename = "vr")]
    Vr,
    #[serde(rename = "wakeLockScreen")]
    WakeLockScreen,
    #[serde(rename = "wakeLockSystem")]
    WakeLockSystem,
    #[serde(rename = "webAppInstallation")]
    WebAppInstallation,
    #[serde(rename = "webPrinting")]
    WebPrinting,
    #[serde(rename = "windowManagement")]
    WindowManagement,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionSetting {
    #[default]
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "prompt")]
    Prompt,
}

/// Definition of PermissionDescriptor defined in the Permissions API:
/// https://w3c.github.io/permissions/#dom-permissiondescriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDescriptor<'a> {
    /// Name of permission.
    /// See https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl for valid permission names.
    name: Cow<'a, str>,
    /// For "midi" permission, may also specify sysex control.
    #[serde(skip_serializing_if = "Option::is_none")]
    sysex: Option<bool>,
    /// For "push" permission, may specify userVisibleOnly.
    /// Note that userVisibleOnly = true is the only currently supported type.
    #[serde(skip_serializing_if = "Option::is_none")]
    userVisibleOnly: Option<bool>,
    /// For "clipboard" permission, may specify allowWithoutSanitization.
    #[serde(skip_serializing_if = "Option::is_none")]
    allowWithoutSanitization: Option<bool>,
    /// For "fullscreen" permission, must specify allowWithoutGesture:true.
    #[serde(skip_serializing_if = "Option::is_none")]
    allowWithoutGesture: Option<bool>,
    /// For "camera" permission, may specify panTiltZoom.
    #[serde(skip_serializing_if = "Option::is_none")]
    panTiltZoom: Option<bool>,
}

impl<'a> PermissionDescriptor<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>) -> PermissionDescriptorBuilder<'a> {
        PermissionDescriptorBuilder {
            name: name.into(),
            sysex: None,
            userVisibleOnly: None,
            allowWithoutSanitization: None,
            allowWithoutGesture: None,
            panTiltZoom: None,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn sysex(&self) -> Option<bool> { self.sysex }
    pub fn userVisibleOnly(&self) -> Option<bool> { self.userVisibleOnly }
    pub fn allowWithoutSanitization(&self) -> Option<bool> { self.allowWithoutSanitization }
    pub fn allowWithoutGesture(&self) -> Option<bool> { self.allowWithoutGesture }
    pub fn panTiltZoom(&self) -> Option<bool> { self.panTiltZoom }
}


pub struct PermissionDescriptorBuilder<'a> {
    name: Cow<'a, str>,
    sysex: Option<bool>,
    userVisibleOnly: Option<bool>,
    allowWithoutSanitization: Option<bool>,
    allowWithoutGesture: Option<bool>,
    panTiltZoom: Option<bool>,
}

impl<'a> PermissionDescriptorBuilder<'a> {
    /// For "midi" permission, may also specify sysex control.
    pub fn sysex(mut self, sysex: bool) -> Self { self.sysex = Some(sysex); self }
    /// For "push" permission, may specify userVisibleOnly.
    /// Note that userVisibleOnly = true is the only currently supported type.
    pub fn userVisibleOnly(mut self, userVisibleOnly: bool) -> Self { self.userVisibleOnly = Some(userVisibleOnly); self }
    /// For "clipboard" permission, may specify allowWithoutSanitization.
    pub fn allowWithoutSanitization(mut self, allowWithoutSanitization: bool) -> Self { self.allowWithoutSanitization = Some(allowWithoutSanitization); self }
    /// For "fullscreen" permission, must specify allowWithoutGesture:true.
    pub fn allowWithoutGesture(mut self, allowWithoutGesture: bool) -> Self { self.allowWithoutGesture = Some(allowWithoutGesture); self }
    /// For "camera" permission, may specify panTiltZoom.
    pub fn panTiltZoom(mut self, panTiltZoom: bool) -> Self { self.panTiltZoom = Some(panTiltZoom); self }
    pub fn build(self) -> PermissionDescriptor<'a> {
        PermissionDescriptor {
            name: self.name,
            sysex: self.sysex,
            userVisibleOnly: self.userVisibleOnly,
            allowWithoutSanitization: self.allowWithoutSanitization,
            allowWithoutGesture: self.allowWithoutGesture,
            panTiltZoom: self.panTiltZoom,
        }
    }
}

/// Browser command ids used by executeBrowserCommand.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BrowserCommandId {
    #[default]
    #[serde(rename = "openTabSearch")]
    OpenTabSearch,
    #[serde(rename = "closeTabSearch")]
    CloseTabSearch,
    #[serde(rename = "openGlic")]
    OpenGlic,
}

/// Chrome histogram bucket.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    /// Minimum value (inclusive).
    low: i64,
    /// Maximum value (exclusive).
    high: i64,
    /// Number of samples.
    count: u64,
}

impl Bucket {
    pub fn builder(low: i64, high: i64, count: u64) -> BucketBuilder {
        BucketBuilder {
            low: low,
            high: high,
            count: count,
        }
    }
    pub fn low(&self) -> i64 { self.low }
    pub fn high(&self) -> i64 { self.high }
    pub fn count(&self) -> u64 { self.count }
}


pub struct BucketBuilder {
    low: i64,
    high: i64,
    count: u64,
}

impl BucketBuilder {
    pub fn build(self) -> Bucket {
        Bucket {
            low: self.low,
            high: self.high,
            count: self.count,
        }
    }
}

/// Chrome histogram.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Histogram<'a> {
    /// Name.
    name: Cow<'a, str>,
    /// Sum of sample values.
    sum: i64,
    /// Total number of samples.
    count: u64,
    /// Buckets.
    buckets: Vec<Bucket>,
}

impl<'a> Histogram<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, sum: i64, count: u64, buckets: Vec<Bucket>) -> HistogramBuilder<'a> {
        HistogramBuilder {
            name: name.into(),
            sum: sum,
            count: count,
            buckets: buckets,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn sum(&self) -> i64 { self.sum }
    pub fn count(&self) -> u64 { self.count }
    pub fn buckets(&self) -> &[Bucket] { &self.buckets }
}


pub struct HistogramBuilder<'a> {
    name: Cow<'a, str>,
    sum: i64,
    count: u64,
    buckets: Vec<Bucket>,
}

impl<'a> HistogramBuilder<'a> {
    pub fn build(self) -> Histogram<'a> {
        Histogram {
            name: self.name,
            sum: self.sum,
            count: self.count,
            buckets: self.buckets,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PrivacySandboxAPI {
    #[default]
    #[serde(rename = "BiddingAndAuctionServices")]
    BiddingAndAuctionServices,
    #[serde(rename = "TrustedKeyValue")]
    TrustedKeyValue,
}

/// Set permission settings for given embedding and embedded origins.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPermissionParams<'a> {
    /// Descriptor of permission to override.
    permission: PermissionDescriptor<'a>,
    /// Setting of the permission.
    setting: PermissionSetting,
    /// Embedding origin the permission applies to, all origins if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<Cow<'a, str>>,
    /// Embedded origin the permission applies to. It is ignored unless the embedding origin is
    /// present and valid. If the embedding origin is provided but the embedded origin isn't, the
    /// embedding origin is used as the embedded origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    embeddedOrigin: Option<Cow<'a, str>>,
    /// Context to override. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> SetPermissionParams<'a> {
    pub fn builder(permission: PermissionDescriptor<'a>, setting: PermissionSetting) -> SetPermissionParamsBuilder<'a> {
        SetPermissionParamsBuilder {
            permission: permission,
            setting: setting,
            origin: None,
            embeddedOrigin: None,
            browserContextId: None,
        }
    }
    pub fn permission(&self) -> &PermissionDescriptor<'a> { &self.permission }
    pub fn setting(&self) -> &PermissionSetting { &self.setting }
    pub fn origin(&self) -> Option<&str> { self.origin.as_deref() }
    pub fn embeddedOrigin(&self) -> Option<&str> { self.embeddedOrigin.as_deref() }
    pub fn browserContextId(&self) -> Option<&BrowserContextID<'a>> { self.browserContextId.as_ref() }
}


pub struct SetPermissionParamsBuilder<'a> {
    permission: PermissionDescriptor<'a>,
    setting: PermissionSetting,
    origin: Option<Cow<'a, str>>,
    embeddedOrigin: Option<Cow<'a, str>>,
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> SetPermissionParamsBuilder<'a> {
    /// Embedding origin the permission applies to, all origins if not specified.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// Embedded origin the permission applies to. It is ignored unless the embedding origin is
    /// present and valid. If the embedding origin is provided but the embedded origin isn't, the
    /// embedding origin is used as the embedded origin.
    pub fn embeddedOrigin(mut self, embeddedOrigin: impl Into<Cow<'a, str>>) -> Self { self.embeddedOrigin = Some(embeddedOrigin.into()); self }
    /// Context to override. When omitted, default browser context is used.
    pub fn browserContextId(mut self, browserContextId: BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> SetPermissionParams<'a> {
        SetPermissionParams {
            permission: self.permission,
            setting: self.setting,
            origin: self.origin,
            embeddedOrigin: self.embeddedOrigin,
            browserContextId: self.browserContextId,
        }
    }
}

impl<'a> SetPermissionParams<'a> { pub const METHOD: &'static str = "Browser.setPermission"; }

impl<'a> crate::CdpCommand<'a> for SetPermissionParams<'a> {
    const METHOD: &'static str = "Browser.setPermission";
    type Response = crate::EmptyReturns;
}

/// Grant specific permissions to the given origin and reject all others. Deprecated. Use
/// setPermission instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GrantPermissionsParams<'a> {
    permissions: Vec<PermissionType>,
    /// Origin the permission applies to, all origins if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<Cow<'a, str>>,
    /// BrowserContext to override permissions. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> GrantPermissionsParams<'a> {
    pub fn builder(permissions: Vec<PermissionType>) -> GrantPermissionsParamsBuilder<'a> {
        GrantPermissionsParamsBuilder {
            permissions: permissions,
            origin: None,
            browserContextId: None,
        }
    }
    pub fn permissions(&self) -> &[PermissionType] { &self.permissions }
    pub fn origin(&self) -> Option<&str> { self.origin.as_deref() }
    pub fn browserContextId(&self) -> Option<&BrowserContextID<'a>> { self.browserContextId.as_ref() }
}


pub struct GrantPermissionsParamsBuilder<'a> {
    permissions: Vec<PermissionType>,
    origin: Option<Cow<'a, str>>,
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> GrantPermissionsParamsBuilder<'a> {
    /// Origin the permission applies to, all origins if not specified.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// BrowserContext to override permissions. When omitted, default browser context is used.
    pub fn browserContextId(mut self, browserContextId: BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> GrantPermissionsParams<'a> {
        GrantPermissionsParams {
            permissions: self.permissions,
            origin: self.origin,
            browserContextId: self.browserContextId,
        }
    }
}

impl<'a> GrantPermissionsParams<'a> { pub const METHOD: &'static str = "Browser.grantPermissions"; }

impl<'a> crate::CdpCommand<'a> for GrantPermissionsParams<'a> {
    const METHOD: &'static str = "Browser.grantPermissions";
    type Response = crate::EmptyReturns;
}

/// Reset all permission management for all origins.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetPermissionsParams<'a> {
    /// BrowserContext to reset permissions. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> ResetPermissionsParams<'a> {
    pub fn builder() -> ResetPermissionsParamsBuilder<'a> {
        ResetPermissionsParamsBuilder {
            browserContextId: None,
        }
    }
    pub fn browserContextId(&self) -> Option<&BrowserContextID<'a>> { self.browserContextId.as_ref() }
}

#[derive(Default)]
pub struct ResetPermissionsParamsBuilder<'a> {
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> ResetPermissionsParamsBuilder<'a> {
    /// BrowserContext to reset permissions. When omitted, default browser context is used.
    pub fn browserContextId(mut self, browserContextId: BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> ResetPermissionsParams<'a> {
        ResetPermissionsParams {
            browserContextId: self.browserContextId,
        }
    }
}

impl<'a> ResetPermissionsParams<'a> { pub const METHOD: &'static str = "Browser.resetPermissions"; }

impl<'a> crate::CdpCommand<'a> for ResetPermissionsParams<'a> {
    const METHOD: &'static str = "Browser.resetPermissions";
    type Response = crate::EmptyReturns;
}

/// Set the behavior when downloading a file.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDownloadBehaviorParams<'a> {
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny). |allowAndName| allows download and names files according to
    /// their download guids.
    behavior: Cow<'a, str>,
    /// BrowserContext to set download behavior. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<BrowserContextID<'a>>,
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    /// or 'allowAndName'.
    #[serde(skip_serializing_if = "Option::is_none")]
    downloadPath: Option<Cow<'a, str>>,
    /// Whether to emit download events (defaults to false).
    #[serde(skip_serializing_if = "Option::is_none")]
    eventsEnabled: Option<bool>,
}

impl<'a> SetDownloadBehaviorParams<'a> {
    pub fn builder(behavior: impl Into<Cow<'a, str>>) -> SetDownloadBehaviorParamsBuilder<'a> {
        SetDownloadBehaviorParamsBuilder {
            behavior: behavior.into(),
            browserContextId: None,
            downloadPath: None,
            eventsEnabled: None,
        }
    }
    pub fn behavior(&self) -> &str { self.behavior.as_ref() }
    pub fn browserContextId(&self) -> Option<&BrowserContextID<'a>> { self.browserContextId.as_ref() }
    pub fn downloadPath(&self) -> Option<&str> { self.downloadPath.as_deref() }
    pub fn eventsEnabled(&self) -> Option<bool> { self.eventsEnabled }
}


pub struct SetDownloadBehaviorParamsBuilder<'a> {
    behavior: Cow<'a, str>,
    browserContextId: Option<BrowserContextID<'a>>,
    downloadPath: Option<Cow<'a, str>>,
    eventsEnabled: Option<bool>,
}

impl<'a> SetDownloadBehaviorParamsBuilder<'a> {
    /// BrowserContext to set download behavior. When omitted, default browser context is used.
    pub fn browserContextId(mut self, browserContextId: BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    /// or 'allowAndName'.
    pub fn downloadPath(mut self, downloadPath: impl Into<Cow<'a, str>>) -> Self { self.downloadPath = Some(downloadPath.into()); self }
    /// Whether to emit download events (defaults to false).
    pub fn eventsEnabled(mut self, eventsEnabled: bool) -> Self { self.eventsEnabled = Some(eventsEnabled); self }
    pub fn build(self) -> SetDownloadBehaviorParams<'a> {
        SetDownloadBehaviorParams {
            behavior: self.behavior,
            browserContextId: self.browserContextId,
            downloadPath: self.downloadPath,
            eventsEnabled: self.eventsEnabled,
        }
    }
}

impl<'a> SetDownloadBehaviorParams<'a> { pub const METHOD: &'static str = "Browser.setDownloadBehavior"; }

impl<'a> crate::CdpCommand<'a> for SetDownloadBehaviorParams<'a> {
    const METHOD: &'static str = "Browser.setDownloadBehavior";
    type Response = crate::EmptyReturns;
}

/// Cancel a download if in progress

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelDownloadParams<'a> {
    /// Global unique identifier of the download.
    guid: Cow<'a, str>,
    /// BrowserContext to perform the action in. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> CancelDownloadParams<'a> {
    pub fn builder(guid: impl Into<Cow<'a, str>>) -> CancelDownloadParamsBuilder<'a> {
        CancelDownloadParamsBuilder {
            guid: guid.into(),
            browserContextId: None,
        }
    }
    pub fn guid(&self) -> &str { self.guid.as_ref() }
    pub fn browserContextId(&self) -> Option<&BrowserContextID<'a>> { self.browserContextId.as_ref() }
}


pub struct CancelDownloadParamsBuilder<'a> {
    guid: Cow<'a, str>,
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> CancelDownloadParamsBuilder<'a> {
    /// BrowserContext to perform the action in. When omitted, default browser context is used.
    pub fn browserContextId(mut self, browserContextId: BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> CancelDownloadParams<'a> {
        CancelDownloadParams {
            guid: self.guid,
            browserContextId: self.browserContextId,
        }
    }
}

impl<'a> CancelDownloadParams<'a> { pub const METHOD: &'static str = "Browser.cancelDownload"; }

impl<'a> crate::CdpCommand<'a> for CancelDownloadParams<'a> {
    const METHOD: &'static str = "Browser.cancelDownload";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloseParams {}

impl CloseParams { pub const METHOD: &'static str = "Browser.close"; }

impl<'a> crate::CdpCommand<'a> for CloseParams {
    const METHOD: &'static str = "Browser.close";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrashParams {}

impl CrashParams { pub const METHOD: &'static str = "Browser.crash"; }

impl<'a> crate::CdpCommand<'a> for CrashParams {
    const METHOD: &'static str = "Browser.crash";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrashGpuProcessParams {}

impl CrashGpuProcessParams { pub const METHOD: &'static str = "Browser.crashGpuProcess"; }

impl<'a> crate::CdpCommand<'a> for CrashGpuProcessParams {
    const METHOD: &'static str = "Browser.crashGpuProcess";
    type Response = crate::EmptyReturns;
}

/// Returns version information.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVersionReturns<'a> {
    /// Protocol version.
    protocolVersion: Cow<'a, str>,
    /// Product name.
    product: Cow<'a, str>,
    /// Product revision.
    revision: Cow<'a, str>,
    /// User-Agent.
    userAgent: Cow<'a, str>,
    /// V8 version.
    jsVersion: Cow<'a, str>,
}

impl<'a> GetVersionReturns<'a> {
    pub fn builder(protocolVersion: impl Into<Cow<'a, str>>, product: impl Into<Cow<'a, str>>, revision: impl Into<Cow<'a, str>>, userAgent: impl Into<Cow<'a, str>>, jsVersion: impl Into<Cow<'a, str>>) -> GetVersionReturnsBuilder<'a> {
        GetVersionReturnsBuilder {
            protocolVersion: protocolVersion.into(),
            product: product.into(),
            revision: revision.into(),
            userAgent: userAgent.into(),
            jsVersion: jsVersion.into(),
        }
    }
    pub fn protocolVersion(&self) -> &str { self.protocolVersion.as_ref() }
    pub fn product(&self) -> &str { self.product.as_ref() }
    pub fn revision(&self) -> &str { self.revision.as_ref() }
    pub fn userAgent(&self) -> &str { self.userAgent.as_ref() }
    pub fn jsVersion(&self) -> &str { self.jsVersion.as_ref() }
}


pub struct GetVersionReturnsBuilder<'a> {
    protocolVersion: Cow<'a, str>,
    product: Cow<'a, str>,
    revision: Cow<'a, str>,
    userAgent: Cow<'a, str>,
    jsVersion: Cow<'a, str>,
}

impl<'a> GetVersionReturnsBuilder<'a> {
    pub fn build(self) -> GetVersionReturns<'a> {
        GetVersionReturns {
            protocolVersion: self.protocolVersion,
            product: self.product,
            revision: self.revision,
            userAgent: self.userAgent,
            jsVersion: self.jsVersion,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetVersionParams {}

impl GetVersionParams { pub const METHOD: &'static str = "Browser.getVersion"; }

impl<'a> crate::CdpCommand<'a> for GetVersionParams {
    const METHOD: &'static str = "Browser.getVersion";
    type Response = GetVersionReturns<'a>;
}

/// Returns the command line switches for the browser process if, and only if
/// --enable-automation is on the commandline.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserCommandLineReturns<'a> {
    /// Commandline parameters
    arguments: Vec<Cow<'a, str>>,
}

impl<'a> GetBrowserCommandLineReturns<'a> {
    pub fn builder(arguments: Vec<Cow<'a, str>>) -> GetBrowserCommandLineReturnsBuilder<'a> {
        GetBrowserCommandLineReturnsBuilder {
            arguments: arguments,
        }
    }
    pub fn arguments(&self) -> &[Cow<'a, str>] { &self.arguments }
}


pub struct GetBrowserCommandLineReturnsBuilder<'a> {
    arguments: Vec<Cow<'a, str>>,
}

impl<'a> GetBrowserCommandLineReturnsBuilder<'a> {
    pub fn build(self) -> GetBrowserCommandLineReturns<'a> {
        GetBrowserCommandLineReturns {
            arguments: self.arguments,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBrowserCommandLineParams {}

impl GetBrowserCommandLineParams { pub const METHOD: &'static str = "Browser.getBrowserCommandLine"; }

impl<'a> crate::CdpCommand<'a> for GetBrowserCommandLineParams {
    const METHOD: &'static str = "Browser.getBrowserCommandLine";
    type Response = GetBrowserCommandLineReturns<'a>;
}

/// Get Chrome histograms.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramsParams<'a> {
    /// Requested substring in name. Only histograms which have query as a
    /// substring in their name are extracted. An empty or absent query returns
    /// all histograms.
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<Cow<'a, str>>,
    /// If true, retrieve delta since last delta call.
    #[serde(skip_serializing_if = "Option::is_none")]
    delta: Option<bool>,
}

impl<'a> GetHistogramsParams<'a> {
    pub fn builder() -> GetHistogramsParamsBuilder<'a> {
        GetHistogramsParamsBuilder {
            query: None,
            delta: None,
        }
    }
    pub fn query(&self) -> Option<&str> { self.query.as_deref() }
    pub fn delta(&self) -> Option<bool> { self.delta }
}

#[derive(Default)]
pub struct GetHistogramsParamsBuilder<'a> {
    query: Option<Cow<'a, str>>,
    delta: Option<bool>,
}

impl<'a> GetHistogramsParamsBuilder<'a> {
    /// Requested substring in name. Only histograms which have query as a
    /// substring in their name are extracted. An empty or absent query returns
    /// all histograms.
    pub fn query(mut self, query: impl Into<Cow<'a, str>>) -> Self { self.query = Some(query.into()); self }
    /// If true, retrieve delta since last delta call.
    pub fn delta(mut self, delta: bool) -> Self { self.delta = Some(delta); self }
    pub fn build(self) -> GetHistogramsParams<'a> {
        GetHistogramsParams {
            query: self.query,
            delta: self.delta,
        }
    }
}

/// Get Chrome histograms.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramsReturns<'a> {
    /// Histograms.
    histograms: Vec<Histogram<'a>>,
}

impl<'a> GetHistogramsReturns<'a> {
    pub fn builder(histograms: Vec<Histogram<'a>>) -> GetHistogramsReturnsBuilder<'a> {
        GetHistogramsReturnsBuilder {
            histograms: histograms,
        }
    }
    pub fn histograms(&self) -> &[Histogram<'a>] { &self.histograms }
}


pub struct GetHistogramsReturnsBuilder<'a> {
    histograms: Vec<Histogram<'a>>,
}

impl<'a> GetHistogramsReturnsBuilder<'a> {
    pub fn build(self) -> GetHistogramsReturns<'a> {
        GetHistogramsReturns {
            histograms: self.histograms,
        }
    }
}

impl<'a> GetHistogramsParams<'a> { pub const METHOD: &'static str = "Browser.getHistograms"; }

impl<'a> crate::CdpCommand<'a> for GetHistogramsParams<'a> {
    const METHOD: &'static str = "Browser.getHistograms";
    type Response = GetHistogramsReturns<'a>;
}

/// Get a Chrome histogram by name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramParams<'a> {
    /// Requested histogram name.
    name: Cow<'a, str>,
    /// If true, retrieve delta since last delta call.
    #[serde(skip_serializing_if = "Option::is_none")]
    delta: Option<bool>,
}

impl<'a> GetHistogramParams<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>) -> GetHistogramParamsBuilder<'a> {
        GetHistogramParamsBuilder {
            name: name.into(),
            delta: None,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn delta(&self) -> Option<bool> { self.delta }
}


pub struct GetHistogramParamsBuilder<'a> {
    name: Cow<'a, str>,
    delta: Option<bool>,
}

impl<'a> GetHistogramParamsBuilder<'a> {
    /// If true, retrieve delta since last delta call.
    pub fn delta(mut self, delta: bool) -> Self { self.delta = Some(delta); self }
    pub fn build(self) -> GetHistogramParams<'a> {
        GetHistogramParams {
            name: self.name,
            delta: self.delta,
        }
    }
}

/// Get a Chrome histogram by name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramReturns<'a> {
    /// Histogram.
    histogram: Histogram<'a>,
}

impl<'a> GetHistogramReturns<'a> {
    pub fn builder(histogram: Histogram<'a>) -> GetHistogramReturnsBuilder<'a> {
        GetHistogramReturnsBuilder {
            histogram: histogram,
        }
    }
    pub fn histogram(&self) -> &Histogram<'a> { &self.histogram }
}


pub struct GetHistogramReturnsBuilder<'a> {
    histogram: Histogram<'a>,
}

impl<'a> GetHistogramReturnsBuilder<'a> {
    pub fn build(self) -> GetHistogramReturns<'a> {
        GetHistogramReturns {
            histogram: self.histogram,
        }
    }
}

impl<'a> GetHistogramParams<'a> { pub const METHOD: &'static str = "Browser.getHistogram"; }

impl<'a> crate::CdpCommand<'a> for GetHistogramParams<'a> {
    const METHOD: &'static str = "Browser.getHistogram";
    type Response = GetHistogramReturns<'a>;
}

/// Get position and size of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowBoundsParams {
    /// Browser window id.
    windowId: WindowID,
}

impl GetWindowBoundsParams {
    pub fn builder(windowId: WindowID) -> GetWindowBoundsParamsBuilder {
        GetWindowBoundsParamsBuilder {
            windowId: windowId,
        }
    }
    pub fn windowId(&self) -> &WindowID { &self.windowId }
}


pub struct GetWindowBoundsParamsBuilder {
    windowId: WindowID,
}

impl GetWindowBoundsParamsBuilder {
    pub fn build(self) -> GetWindowBoundsParams {
        GetWindowBoundsParams {
            windowId: self.windowId,
        }
    }
}

/// Get position and size of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowBoundsReturns {
    /// Bounds information of the window. When window state is 'minimized', the restored window
    /// position and size are returned.
    bounds: Bounds,
}

impl GetWindowBoundsReturns {
    pub fn builder(bounds: Bounds) -> GetWindowBoundsReturnsBuilder {
        GetWindowBoundsReturnsBuilder {
            bounds: bounds,
        }
    }
    pub fn bounds(&self) -> &Bounds { &self.bounds }
}


pub struct GetWindowBoundsReturnsBuilder {
    bounds: Bounds,
}

impl GetWindowBoundsReturnsBuilder {
    pub fn build(self) -> GetWindowBoundsReturns {
        GetWindowBoundsReturns {
            bounds: self.bounds,
        }
    }
}

impl GetWindowBoundsParams { pub const METHOD: &'static str = "Browser.getWindowBounds"; }

impl<'a> crate::CdpCommand<'a> for GetWindowBoundsParams {
    const METHOD: &'static str = "Browser.getWindowBounds";
    type Response = GetWindowBoundsReturns;
}

/// Get the browser window that contains the devtools target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowForTargetParams<'a> {
    /// Devtools agent host id. If called as a part of the session, associated targetId is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    targetId: Option<crate::target::TargetID<'a>>,
}

impl<'a> GetWindowForTargetParams<'a> {
    pub fn builder() -> GetWindowForTargetParamsBuilder<'a> {
        GetWindowForTargetParamsBuilder {
            targetId: None,
        }
    }
    pub fn targetId(&self) -> Option<&crate::target::TargetID<'a>> { self.targetId.as_ref() }
}

#[derive(Default)]
pub struct GetWindowForTargetParamsBuilder<'a> {
    targetId: Option<crate::target::TargetID<'a>>,
}

impl<'a> GetWindowForTargetParamsBuilder<'a> {
    /// Devtools agent host id. If called as a part of the session, associated targetId is used.
    pub fn targetId(mut self, targetId: crate::target::TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> GetWindowForTargetParams<'a> {
        GetWindowForTargetParams {
            targetId: self.targetId,
        }
    }
}

/// Get the browser window that contains the devtools target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowForTargetReturns {
    /// Browser window id.
    windowId: WindowID,
    /// Bounds information of the window. When window state is 'minimized', the restored window
    /// position and size are returned.
    bounds: Bounds,
}

impl GetWindowForTargetReturns {
    pub fn builder(windowId: WindowID, bounds: Bounds) -> GetWindowForTargetReturnsBuilder {
        GetWindowForTargetReturnsBuilder {
            windowId: windowId,
            bounds: bounds,
        }
    }
    pub fn windowId(&self) -> &WindowID { &self.windowId }
    pub fn bounds(&self) -> &Bounds { &self.bounds }
}


pub struct GetWindowForTargetReturnsBuilder {
    windowId: WindowID,
    bounds: Bounds,
}

impl GetWindowForTargetReturnsBuilder {
    pub fn build(self) -> GetWindowForTargetReturns {
        GetWindowForTargetReturns {
            windowId: self.windowId,
            bounds: self.bounds,
        }
    }
}

impl<'a> GetWindowForTargetParams<'a> { pub const METHOD: &'static str = "Browser.getWindowForTarget"; }

impl<'a> crate::CdpCommand<'a> for GetWindowForTargetParams<'a> {
    const METHOD: &'static str = "Browser.getWindowForTarget";
    type Response = GetWindowForTargetReturns;
}

/// Set position and/or size of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetWindowBoundsParams {
    /// Browser window id.
    windowId: WindowID,
    /// New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined
    /// with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.
    bounds: Bounds,
}

impl SetWindowBoundsParams {
    pub fn builder(windowId: WindowID, bounds: Bounds) -> SetWindowBoundsParamsBuilder {
        SetWindowBoundsParamsBuilder {
            windowId: windowId,
            bounds: bounds,
        }
    }
    pub fn windowId(&self) -> &WindowID { &self.windowId }
    pub fn bounds(&self) -> &Bounds { &self.bounds }
}


pub struct SetWindowBoundsParamsBuilder {
    windowId: WindowID,
    bounds: Bounds,
}

impl SetWindowBoundsParamsBuilder {
    pub fn build(self) -> SetWindowBoundsParams {
        SetWindowBoundsParams {
            windowId: self.windowId,
            bounds: self.bounds,
        }
    }
}

impl SetWindowBoundsParams { pub const METHOD: &'static str = "Browser.setWindowBounds"; }

impl<'a> crate::CdpCommand<'a> for SetWindowBoundsParams {
    const METHOD: &'static str = "Browser.setWindowBounds";
    type Response = crate::EmptyReturns;
}

/// Set size of the browser contents resizing browser window as necessary.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContentsSizeParams {
    /// Browser window id.
    windowId: WindowID,
    /// The window contents width in DIP. Assumes current width if omitted.
    /// Must be specified if 'height' is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u64>,
    /// The window contents height in DIP. Assumes current height if omitted.
    /// Must be specified if 'width' is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
}

impl SetContentsSizeParams {
    pub fn builder(windowId: WindowID) -> SetContentsSizeParamsBuilder {
        SetContentsSizeParamsBuilder {
            windowId: windowId,
            width: None,
            height: None,
        }
    }
    pub fn windowId(&self) -> &WindowID { &self.windowId }
    pub fn width(&self) -> Option<u64> { self.width }
    pub fn height(&self) -> Option<i64> { self.height }
}


pub struct SetContentsSizeParamsBuilder {
    windowId: WindowID,
    width: Option<u64>,
    height: Option<i64>,
}

impl SetContentsSizeParamsBuilder {
    /// The window contents width in DIP. Assumes current width if omitted.
    /// Must be specified if 'height' is omitted.
    pub fn width(mut self, width: u64) -> Self { self.width = Some(width); self }
    /// The window contents height in DIP. Assumes current height if omitted.
    /// Must be specified if 'width' is omitted.
    pub fn height(mut self, height: i64) -> Self { self.height = Some(height); self }
    pub fn build(self) -> SetContentsSizeParams {
        SetContentsSizeParams {
            windowId: self.windowId,
            width: self.width,
            height: self.height,
        }
    }
}

impl SetContentsSizeParams { pub const METHOD: &'static str = "Browser.setContentsSize"; }

impl<'a> crate::CdpCommand<'a> for SetContentsSizeParams {
    const METHOD: &'static str = "Browser.setContentsSize";
    type Response = crate::EmptyReturns;
}

/// Set dock tile details, platform-specific.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDockTileParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    badgeLabel: Option<Cow<'a, str>>,
    /// Png encoded image. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Cow<'a, str>>,
}

impl<'a> SetDockTileParams<'a> {
    pub fn builder() -> SetDockTileParamsBuilder<'a> {
        SetDockTileParamsBuilder {
            badgeLabel: None,
            image: None,
        }
    }
    pub fn badgeLabel(&self) -> Option<&str> { self.badgeLabel.as_deref() }
    pub fn image(&self) -> Option<&str> { self.image.as_deref() }
}

#[derive(Default)]
pub struct SetDockTileParamsBuilder<'a> {
    badgeLabel: Option<Cow<'a, str>>,
    image: Option<Cow<'a, str>>,
}

impl<'a> SetDockTileParamsBuilder<'a> {
    pub fn badgeLabel(mut self, badgeLabel: impl Into<Cow<'a, str>>) -> Self { self.badgeLabel = Some(badgeLabel.into()); self }
    /// Png encoded image. (Encoded as a base64 string when passed over JSON)
    pub fn image(mut self, image: impl Into<Cow<'a, str>>) -> Self { self.image = Some(image.into()); self }
    pub fn build(self) -> SetDockTileParams<'a> {
        SetDockTileParams {
            badgeLabel: self.badgeLabel,
            image: self.image,
        }
    }
}

impl<'a> SetDockTileParams<'a> { pub const METHOD: &'static str = "Browser.setDockTile"; }

impl<'a> crate::CdpCommand<'a> for SetDockTileParams<'a> {
    const METHOD: &'static str = "Browser.setDockTile";
    type Response = crate::EmptyReturns;
}

/// Invoke custom browser commands used by telemetry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteBrowserCommandParams {
    commandId: BrowserCommandId,
}

impl ExecuteBrowserCommandParams {
    pub fn builder(commandId: BrowserCommandId) -> ExecuteBrowserCommandParamsBuilder {
        ExecuteBrowserCommandParamsBuilder {
            commandId: commandId,
        }
    }
    pub fn commandId(&self) -> &BrowserCommandId { &self.commandId }
}


pub struct ExecuteBrowserCommandParamsBuilder {
    commandId: BrowserCommandId,
}

impl ExecuteBrowserCommandParamsBuilder {
    pub fn build(self) -> ExecuteBrowserCommandParams {
        ExecuteBrowserCommandParams {
            commandId: self.commandId,
        }
    }
}

impl ExecuteBrowserCommandParams { pub const METHOD: &'static str = "Browser.executeBrowserCommand"; }

impl<'a> crate::CdpCommand<'a> for ExecuteBrowserCommandParams {
    const METHOD: &'static str = "Browser.executeBrowserCommand";
    type Response = crate::EmptyReturns;
}

/// Allows a site to use privacy sandbox features that require enrollment
/// without the site actually being enrolled. Only supported on page targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddPrivacySandboxEnrollmentOverrideParams<'a> {
    url: Cow<'a, str>,
}

impl<'a> AddPrivacySandboxEnrollmentOverrideParams<'a> {
    pub fn builder(url: impl Into<Cow<'a, str>>) -> AddPrivacySandboxEnrollmentOverrideParamsBuilder<'a> {
        AddPrivacySandboxEnrollmentOverrideParamsBuilder {
            url: url.into(),
        }
    }
    pub fn url(&self) -> &str { self.url.as_ref() }
}


pub struct AddPrivacySandboxEnrollmentOverrideParamsBuilder<'a> {
    url: Cow<'a, str>,
}

impl<'a> AddPrivacySandboxEnrollmentOverrideParamsBuilder<'a> {
    pub fn build(self) -> AddPrivacySandboxEnrollmentOverrideParams<'a> {
        AddPrivacySandboxEnrollmentOverrideParams {
            url: self.url,
        }
    }
}

impl<'a> AddPrivacySandboxEnrollmentOverrideParams<'a> { pub const METHOD: &'static str = "Browser.addPrivacySandboxEnrollmentOverride"; }

impl<'a> crate::CdpCommand<'a> for AddPrivacySandboxEnrollmentOverrideParams<'a> {
    const METHOD: &'static str = "Browser.addPrivacySandboxEnrollmentOverride";
    type Response = crate::EmptyReturns;
}

/// Configures encryption keys used with a given privacy sandbox API to talk
/// to a trusted coordinator.  Since this is intended for test automation only,
/// coordinatorOrigin must be a .test domain. No existing coordinator
/// configuration for the origin may exist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddPrivacySandboxCoordinatorKeyConfigParams<'a> {
    api: PrivacySandboxAPI,
    coordinatorOrigin: Cow<'a, str>,
    keyConfig: Cow<'a, str>,
    /// BrowserContext to perform the action in. When omitted, default browser
    /// context is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> AddPrivacySandboxCoordinatorKeyConfigParams<'a> {
    pub fn builder(api: PrivacySandboxAPI, coordinatorOrigin: impl Into<Cow<'a, str>>, keyConfig: impl Into<Cow<'a, str>>) -> AddPrivacySandboxCoordinatorKeyConfigParamsBuilder<'a> {
        AddPrivacySandboxCoordinatorKeyConfigParamsBuilder {
            api: api,
            coordinatorOrigin: coordinatorOrigin.into(),
            keyConfig: keyConfig.into(),
            browserContextId: None,
        }
    }
    pub fn api(&self) -> &PrivacySandboxAPI { &self.api }
    pub fn coordinatorOrigin(&self) -> &str { self.coordinatorOrigin.as_ref() }
    pub fn keyConfig(&self) -> &str { self.keyConfig.as_ref() }
    pub fn browserContextId(&self) -> Option<&BrowserContextID<'a>> { self.browserContextId.as_ref() }
}


pub struct AddPrivacySandboxCoordinatorKeyConfigParamsBuilder<'a> {
    api: PrivacySandboxAPI,
    coordinatorOrigin: Cow<'a, str>,
    keyConfig: Cow<'a, str>,
    browserContextId: Option<BrowserContextID<'a>>,
}

impl<'a> AddPrivacySandboxCoordinatorKeyConfigParamsBuilder<'a> {
    /// BrowserContext to perform the action in. When omitted, default browser
    /// context is used.
    pub fn browserContextId(mut self, browserContextId: BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> AddPrivacySandboxCoordinatorKeyConfigParams<'a> {
        AddPrivacySandboxCoordinatorKeyConfigParams {
            api: self.api,
            coordinatorOrigin: self.coordinatorOrigin,
            keyConfig: self.keyConfig,
            browserContextId: self.browserContextId,
        }
    }
}

impl<'a> AddPrivacySandboxCoordinatorKeyConfigParams<'a> { pub const METHOD: &'static str = "Browser.addPrivacySandboxCoordinatorKeyConfig"; }

impl<'a> crate::CdpCommand<'a> for AddPrivacySandboxCoordinatorKeyConfigParams<'a> {
    const METHOD: &'static str = "Browser.addPrivacySandboxCoordinatorKeyConfig";
    type Response = crate::EmptyReturns;
}
