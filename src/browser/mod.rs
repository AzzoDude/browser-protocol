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
    #[serde(skip_serializing_if = "Option::is_none", rename = "windowState")]
    window_state: Option<WindowState>,
}

impl Bounds {
    /// Creates a builder for this type.
    pub fn builder() -> BoundsBuilder {
        BoundsBuilder {
            left: None,
            top: None,
            width: None,
            height: None,
            window_state: None,
        }
    }
    /// The offset from the left edge of the screen to the window in pixels.
    pub fn left(&self) -> Option<i64> { self.left }
    /// The offset from the top edge of the screen to the window in pixels.
    pub fn top(&self) -> Option<i64> { self.top }
    /// The window width in pixels.
    pub fn width(&self) -> Option<u64> { self.width }
    /// The window height in pixels.
    pub fn height(&self) -> Option<i64> { self.height }
    /// The window state. Default to normal.
    pub fn window_state(&self) -> Option<&WindowState> { self.window_state.as_ref() }
}

#[derive(Default)]
pub struct BoundsBuilder {
    left: Option<i64>,
    top: Option<i64>,
    width: Option<u64>,
    height: Option<i64>,
    window_state: Option<WindowState>,
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
    pub fn window_state(mut self, window_state: impl Into<WindowState>) -> Self { self.window_state = Some(window_state.into()); self }
    pub fn build(self) -> Bounds {
        Bounds {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            window_state: self.window_state,
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
/// <https://w3c.github.io/permissions/#dom-permissiondescriptor>.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDescriptor<'a> {
    /// Name of permission.
    /// See <https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl> for valid permission names.
    name: Cow<'a, str>,
    /// For "midi" permission, may also specify sysex control.
    #[serde(skip_serializing_if = "Option::is_none")]
    sysex: Option<bool>,
    /// For "push" permission, may specify userVisibleOnly.
    /// Note that userVisibleOnly = true is the only currently supported type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "userVisibleOnly")]
    user_visible_only: Option<bool>,
    /// For "clipboard" permission, may specify allowWithoutSanitization.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowWithoutSanitization")]
    allow_without_sanitization: Option<bool>,
    /// For "fullscreen" permission, must specify allowWithoutGesture:true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowWithoutGesture")]
    allow_without_gesture: Option<bool>,
    /// For "camera" permission, may specify panTiltZoom.
    #[serde(skip_serializing_if = "Option::is_none", rename = "panTiltZoom")]
    pan_tilt_zoom: Option<bool>,
}

impl<'a> PermissionDescriptor<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Name of permission. See <https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl> for valid permission names.
    pub fn builder(name: impl Into<Cow<'a, str>>) -> PermissionDescriptorBuilder<'a> {
        PermissionDescriptorBuilder {
            name: name.into(),
            sysex: None,
            user_visible_only: None,
            allow_without_sanitization: None,
            allow_without_gesture: None,
            pan_tilt_zoom: None,
        }
    }
    /// Name of permission.
    /// See <https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl> for valid permission names.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// For "midi" permission, may also specify sysex control.
    pub fn sysex(&self) -> Option<bool> { self.sysex }
    /// For "push" permission, may specify userVisibleOnly.
    /// Note that userVisibleOnly = true is the only currently supported type.
    pub fn user_visible_only(&self) -> Option<bool> { self.user_visible_only }
    /// For "clipboard" permission, may specify allowWithoutSanitization.
    pub fn allow_without_sanitization(&self) -> Option<bool> { self.allow_without_sanitization }
    /// For "fullscreen" permission, must specify allowWithoutGesture:true.
    pub fn allow_without_gesture(&self) -> Option<bool> { self.allow_without_gesture }
    /// For "camera" permission, may specify panTiltZoom.
    pub fn pan_tilt_zoom(&self) -> Option<bool> { self.pan_tilt_zoom }
}


pub struct PermissionDescriptorBuilder<'a> {
    name: Cow<'a, str>,
    sysex: Option<bool>,
    user_visible_only: Option<bool>,
    allow_without_sanitization: Option<bool>,
    allow_without_gesture: Option<bool>,
    pan_tilt_zoom: Option<bool>,
}

impl<'a> PermissionDescriptorBuilder<'a> {
    /// For "midi" permission, may also specify sysex control.
    pub fn sysex(mut self, sysex: bool) -> Self { self.sysex = Some(sysex); self }
    /// For "push" permission, may specify userVisibleOnly.
    /// Note that userVisibleOnly = true is the only currently supported type.
    pub fn user_visible_only(mut self, user_visible_only: bool) -> Self { self.user_visible_only = Some(user_visible_only); self }
    /// For "clipboard" permission, may specify allowWithoutSanitization.
    pub fn allow_without_sanitization(mut self, allow_without_sanitization: bool) -> Self { self.allow_without_sanitization = Some(allow_without_sanitization); self }
    /// For "fullscreen" permission, must specify allowWithoutGesture:true.
    pub fn allow_without_gesture(mut self, allow_without_gesture: bool) -> Self { self.allow_without_gesture = Some(allow_without_gesture); self }
    /// For "camera" permission, may specify panTiltZoom.
    pub fn pan_tilt_zoom(mut self, pan_tilt_zoom: bool) -> Self { self.pan_tilt_zoom = Some(pan_tilt_zoom); self }
    pub fn build(self) -> PermissionDescriptor<'a> {
        PermissionDescriptor {
            name: self.name,
            sysex: self.sysex,
            user_visible_only: self.user_visible_only,
            allow_without_sanitization: self.allow_without_sanitization,
            allow_without_gesture: self.allow_without_gesture,
            pan_tilt_zoom: self.pan_tilt_zoom,
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
    /// Creates a builder for this type with the required parameters:
    /// * `low`: Minimum value (inclusive).
    /// * `high`: Maximum value (exclusive).
    /// * `count`: Number of samples.
    pub fn builder(low: i64, high: i64, count: u64) -> BucketBuilder {
        BucketBuilder {
            low: low,
            high: high,
            count: count,
        }
    }
    /// Minimum value (inclusive).
    pub fn low(&self) -> i64 { self.low }
    /// Maximum value (exclusive).
    pub fn high(&self) -> i64 { self.high }
    /// Number of samples.
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Name.
    /// * `sum`: Sum of sample values.
    /// * `count`: Total number of samples.
    /// * `buckets`: Buckets.
    pub fn builder(name: impl Into<Cow<'a, str>>, sum: i64, count: u64, buckets: Vec<Bucket>) -> HistogramBuilder<'a> {
        HistogramBuilder {
            name: name.into(),
            sum: sum,
            count: count,
            buckets: buckets,
        }
    }
    /// Name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Sum of sample values.
    pub fn sum(&self) -> i64 { self.sum }
    /// Total number of samples.
    pub fn count(&self) -> u64 { self.count }
    /// Buckets.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "embeddedOrigin")]
    embedded_origin: Option<Cow<'a, str>>,
    /// Context to override. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> SetPermissionParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `permission`: Descriptor of permission to override.
    /// * `setting`: Setting of the permission.
    pub fn builder(permission: PermissionDescriptor<'a>, setting: impl Into<PermissionSetting>) -> SetPermissionParamsBuilder<'a> {
        SetPermissionParamsBuilder {
            permission: permission,
            setting: setting.into(),
            origin: None,
            embedded_origin: None,
            browser_context_id: None,
        }
    }
    /// Descriptor of permission to override.
    pub fn permission(&self) -> &PermissionDescriptor<'a> { &self.permission }
    /// Setting of the permission.
    pub fn setting(&self) -> &PermissionSetting { &self.setting }
    /// Embedding origin the permission applies to, all origins if not specified.
    pub fn origin(&self) -> Option<&str> { self.origin.as_deref() }
    /// Embedded origin the permission applies to. It is ignored unless the embedding origin is
    /// present and valid. If the embedding origin is provided but the embedded origin isn't, the
    /// embedding origin is used as the embedded origin.
    pub fn embedded_origin(&self) -> Option<&str> { self.embedded_origin.as_deref() }
    /// Context to override. When omitted, default browser context is used.
    pub fn browser_context_id(&self) -> Option<&BrowserContextID<'a>> { self.browser_context_id.as_ref() }
}


pub struct SetPermissionParamsBuilder<'a> {
    permission: PermissionDescriptor<'a>,
    setting: PermissionSetting,
    origin: Option<Cow<'a, str>>,
    embedded_origin: Option<Cow<'a, str>>,
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> SetPermissionParamsBuilder<'a> {
    /// Embedding origin the permission applies to, all origins if not specified.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// Embedded origin the permission applies to. It is ignored unless the embedding origin is
    /// present and valid. If the embedding origin is provided but the embedded origin isn't, the
    /// embedding origin is used as the embedded origin.
    pub fn embedded_origin(mut self, embedded_origin: impl Into<Cow<'a, str>>) -> Self { self.embedded_origin = Some(embedded_origin.into()); self }
    /// Context to override. When omitted, default browser context is used.
    pub fn browser_context_id(mut self, browser_context_id: impl Into<BrowserContextID<'a>>) -> Self { self.browser_context_id = Some(browser_context_id.into()); self }
    pub fn build(self) -> SetPermissionParams<'a> {
        SetPermissionParams {
            permission: self.permission,
            setting: self.setting,
            origin: self.origin,
            embedded_origin: self.embedded_origin,
            browser_context_id: self.browser_context_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> GrantPermissionsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `permissions`: 
    pub fn builder(permissions: Vec<PermissionType>) -> GrantPermissionsParamsBuilder<'a> {
        GrantPermissionsParamsBuilder {
            permissions: permissions,
            origin: None,
            browser_context_id: None,
        }
    }
    pub fn permissions(&self) -> &[PermissionType] { &self.permissions }
    /// Origin the permission applies to, all origins if not specified.
    pub fn origin(&self) -> Option<&str> { self.origin.as_deref() }
    /// BrowserContext to override permissions. When omitted, default browser context is used.
    pub fn browser_context_id(&self) -> Option<&BrowserContextID<'a>> { self.browser_context_id.as_ref() }
}


pub struct GrantPermissionsParamsBuilder<'a> {
    permissions: Vec<PermissionType>,
    origin: Option<Cow<'a, str>>,
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> GrantPermissionsParamsBuilder<'a> {
    /// Origin the permission applies to, all origins if not specified.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// BrowserContext to override permissions. When omitted, default browser context is used.
    pub fn browser_context_id(mut self, browser_context_id: impl Into<BrowserContextID<'a>>) -> Self { self.browser_context_id = Some(browser_context_id.into()); self }
    pub fn build(self) -> GrantPermissionsParams<'a> {
        GrantPermissionsParams {
            permissions: self.permissions,
            origin: self.origin,
            browser_context_id: self.browser_context_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> ResetPermissionsParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> ResetPermissionsParamsBuilder<'a> {
        ResetPermissionsParamsBuilder {
            browser_context_id: None,
        }
    }
    /// BrowserContext to reset permissions. When omitted, default browser context is used.
    pub fn browser_context_id(&self) -> Option<&BrowserContextID<'a>> { self.browser_context_id.as_ref() }
}

#[derive(Default)]
pub struct ResetPermissionsParamsBuilder<'a> {
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> ResetPermissionsParamsBuilder<'a> {
    /// BrowserContext to reset permissions. When omitted, default browser context is used.
    pub fn browser_context_id(mut self, browser_context_id: impl Into<BrowserContextID<'a>>) -> Self { self.browser_context_id = Some(browser_context_id.into()); self }
    pub fn build(self) -> ResetPermissionsParams<'a> {
        ResetPermissionsParams {
            browser_context_id: self.browser_context_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<BrowserContextID<'a>>,
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    /// or 'allowAndName'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "downloadPath")]
    download_path: Option<Cow<'a, str>>,
    /// Whether to emit download events (defaults to false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "eventsEnabled")]
    events_enabled: Option<bool>,
}

impl<'a> SetDownloadBehaviorParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `behavior`: Whether to allow all or deny all download requests, or use default Chrome behavior if available (otherwise deny). |allowAndName| allows download and names files according to their download guids.
    pub fn builder(behavior: impl Into<Cow<'a, str>>) -> SetDownloadBehaviorParamsBuilder<'a> {
        SetDownloadBehaviorParamsBuilder {
            behavior: behavior.into(),
            browser_context_id: None,
            download_path: None,
            events_enabled: None,
        }
    }
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny). |allowAndName| allows download and names files according to
    /// their download guids.
    pub fn behavior(&self) -> &str { self.behavior.as_ref() }
    /// BrowserContext to set download behavior. When omitted, default browser context is used.
    pub fn browser_context_id(&self) -> Option<&BrowserContextID<'a>> { self.browser_context_id.as_ref() }
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    /// or 'allowAndName'.
    pub fn download_path(&self) -> Option<&str> { self.download_path.as_deref() }
    /// Whether to emit download events (defaults to false).
    pub fn events_enabled(&self) -> Option<bool> { self.events_enabled }
}


pub struct SetDownloadBehaviorParamsBuilder<'a> {
    behavior: Cow<'a, str>,
    browser_context_id: Option<BrowserContextID<'a>>,
    download_path: Option<Cow<'a, str>>,
    events_enabled: Option<bool>,
}

impl<'a> SetDownloadBehaviorParamsBuilder<'a> {
    /// BrowserContext to set download behavior. When omitted, default browser context is used.
    pub fn browser_context_id(mut self, browser_context_id: impl Into<BrowserContextID<'a>>) -> Self { self.browser_context_id = Some(browser_context_id.into()); self }
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    /// or 'allowAndName'.
    pub fn download_path(mut self, download_path: impl Into<Cow<'a, str>>) -> Self { self.download_path = Some(download_path.into()); self }
    /// Whether to emit download events (defaults to false).
    pub fn events_enabled(mut self, events_enabled: bool) -> Self { self.events_enabled = Some(events_enabled); self }
    pub fn build(self) -> SetDownloadBehaviorParams<'a> {
        SetDownloadBehaviorParams {
            behavior: self.behavior,
            browser_context_id: self.browser_context_id,
            download_path: self.download_path,
            events_enabled: self.events_enabled,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> CancelDownloadParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `guid`: Global unique identifier of the download.
    pub fn builder(guid: impl Into<Cow<'a, str>>) -> CancelDownloadParamsBuilder<'a> {
        CancelDownloadParamsBuilder {
            guid: guid.into(),
            browser_context_id: None,
        }
    }
    /// Global unique identifier of the download.
    pub fn guid(&self) -> &str { self.guid.as_ref() }
    /// BrowserContext to perform the action in. When omitted, default browser context is used.
    pub fn browser_context_id(&self) -> Option<&BrowserContextID<'a>> { self.browser_context_id.as_ref() }
}


pub struct CancelDownloadParamsBuilder<'a> {
    guid: Cow<'a, str>,
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> CancelDownloadParamsBuilder<'a> {
    /// BrowserContext to perform the action in. When omitted, default browser context is used.
    pub fn browser_context_id(mut self, browser_context_id: impl Into<BrowserContextID<'a>>) -> Self { self.browser_context_id = Some(browser_context_id.into()); self }
    pub fn build(self) -> CancelDownloadParams<'a> {
        CancelDownloadParams {
            guid: self.guid,
            browser_context_id: self.browser_context_id,
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
    #[serde(rename = "protocolVersion")]
    protocol_version: Cow<'a, str>,
    /// Product name.
    product: Cow<'a, str>,
    /// Product revision.
    revision: Cow<'a, str>,
    /// User-Agent.
    #[serde(rename = "userAgent")]
    user_agent: Cow<'a, str>,
    /// V8 version.
    #[serde(rename = "jsVersion")]
    js_version: Cow<'a, str>,
}

impl<'a> GetVersionReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `protocol_version`: Protocol version.
    /// * `product`: Product name.
    /// * `revision`: Product revision.
    /// * `user_agent`: User-Agent.
    /// * `js_version`: V8 version.
    pub fn builder(protocol_version: impl Into<Cow<'a, str>>, product: impl Into<Cow<'a, str>>, revision: impl Into<Cow<'a, str>>, user_agent: impl Into<Cow<'a, str>>, js_version: impl Into<Cow<'a, str>>) -> GetVersionReturnsBuilder<'a> {
        GetVersionReturnsBuilder {
            protocol_version: protocol_version.into(),
            product: product.into(),
            revision: revision.into(),
            user_agent: user_agent.into(),
            js_version: js_version.into(),
        }
    }
    /// Protocol version.
    pub fn protocol_version(&self) -> &str { self.protocol_version.as_ref() }
    /// Product name.
    pub fn product(&self) -> &str { self.product.as_ref() }
    /// Product revision.
    pub fn revision(&self) -> &str { self.revision.as_ref() }
    /// User-Agent.
    pub fn user_agent(&self) -> &str { self.user_agent.as_ref() }
    /// V8 version.
    pub fn js_version(&self) -> &str { self.js_version.as_ref() }
}


pub struct GetVersionReturnsBuilder<'a> {
    protocol_version: Cow<'a, str>,
    product: Cow<'a, str>,
    revision: Cow<'a, str>,
    user_agent: Cow<'a, str>,
    js_version: Cow<'a, str>,
}

impl<'a> GetVersionReturnsBuilder<'a> {
    pub fn build(self) -> GetVersionReturns<'a> {
        GetVersionReturns {
            protocol_version: self.protocol_version,
            product: self.product,
            revision: self.revision,
            user_agent: self.user_agent,
            js_version: self.js_version,
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
    /// Creates a builder for this type with the required parameters:
    /// * `arguments`: Commandline parameters
    pub fn builder(arguments: Vec<Cow<'a, str>>) -> GetBrowserCommandLineReturnsBuilder<'a> {
        GetBrowserCommandLineReturnsBuilder {
            arguments: arguments,
        }
    }
    /// Commandline parameters
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
    /// Creates a builder for this type.
    pub fn builder() -> GetHistogramsParamsBuilder<'a> {
        GetHistogramsParamsBuilder {
            query: None,
            delta: None,
        }
    }
    /// Requested substring in name. Only histograms which have query as a
    /// substring in their name are extracted. An empty or absent query returns
    /// all histograms.
    pub fn query(&self) -> Option<&str> { self.query.as_deref() }
    /// If true, retrieve delta since last delta call.
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
    /// Creates a builder for this type with the required parameters:
    /// * `histograms`: Histograms.
    pub fn builder(histograms: Vec<Histogram<'a>>) -> GetHistogramsReturnsBuilder<'a> {
        GetHistogramsReturnsBuilder {
            histograms: histograms,
        }
    }
    /// Histograms.
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Requested histogram name.
    pub fn builder(name: impl Into<Cow<'a, str>>) -> GetHistogramParamsBuilder<'a> {
        GetHistogramParamsBuilder {
            name: name.into(),
            delta: None,
        }
    }
    /// Requested histogram name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// If true, retrieve delta since last delta call.
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
    /// Creates a builder for this type with the required parameters:
    /// * `histogram`: Histogram.
    pub fn builder(histogram: Histogram<'a>) -> GetHistogramReturnsBuilder<'a> {
        GetHistogramReturnsBuilder {
            histogram: histogram,
        }
    }
    /// Histogram.
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
    #[serde(rename = "windowId")]
    window_id: WindowID,
}

impl GetWindowBoundsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `window_id`: Browser window id.
    pub fn builder(window_id: WindowID) -> GetWindowBoundsParamsBuilder {
        GetWindowBoundsParamsBuilder {
            window_id: window_id,
        }
    }
    /// Browser window id.
    pub fn window_id(&self) -> &WindowID { &self.window_id }
}


pub struct GetWindowBoundsParamsBuilder {
    window_id: WindowID,
}

impl GetWindowBoundsParamsBuilder {
    pub fn build(self) -> GetWindowBoundsParams {
        GetWindowBoundsParams {
            window_id: self.window_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `bounds`: Bounds information of the window. When window state is 'minimized', the restored window position and size are returned.
    pub fn builder(bounds: Bounds) -> GetWindowBoundsReturnsBuilder {
        GetWindowBoundsReturnsBuilder {
            bounds: bounds,
        }
    }
    /// Bounds information of the window. When window state is 'minimized', the restored window
    /// position and size are returned.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    target_id: Option<crate::target::TargetID<'a>>,
}

impl<'a> GetWindowForTargetParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetWindowForTargetParamsBuilder<'a> {
        GetWindowForTargetParamsBuilder {
            target_id: None,
        }
    }
    /// Devtools agent host id. If called as a part of the session, associated targetId is used.
    pub fn target_id(&self) -> Option<&crate::target::TargetID<'a>> { self.target_id.as_ref() }
}

#[derive(Default)]
pub struct GetWindowForTargetParamsBuilder<'a> {
    target_id: Option<crate::target::TargetID<'a>>,
}

impl<'a> GetWindowForTargetParamsBuilder<'a> {
    /// Devtools agent host id. If called as a part of the session, associated targetId is used.
    pub fn target_id(mut self, target_id: crate::target::TargetID<'a>) -> Self { self.target_id = Some(target_id); self }
    pub fn build(self) -> GetWindowForTargetParams<'a> {
        GetWindowForTargetParams {
            target_id: self.target_id,
        }
    }
}

/// Get the browser window that contains the devtools target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowForTargetReturns {
    /// Browser window id.
    #[serde(rename = "windowId")]
    window_id: WindowID,
    /// Bounds information of the window. When window state is 'minimized', the restored window
    /// position and size are returned.
    bounds: Bounds,
}

impl GetWindowForTargetReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `window_id`: Browser window id.
    /// * `bounds`: Bounds information of the window. When window state is 'minimized', the restored window position and size are returned.
    pub fn builder(window_id: WindowID, bounds: Bounds) -> GetWindowForTargetReturnsBuilder {
        GetWindowForTargetReturnsBuilder {
            window_id: window_id,
            bounds: bounds,
        }
    }
    /// Browser window id.
    pub fn window_id(&self) -> &WindowID { &self.window_id }
    /// Bounds information of the window. When window state is 'minimized', the restored window
    /// position and size are returned.
    pub fn bounds(&self) -> &Bounds { &self.bounds }
}


pub struct GetWindowForTargetReturnsBuilder {
    window_id: WindowID,
    bounds: Bounds,
}

impl GetWindowForTargetReturnsBuilder {
    pub fn build(self) -> GetWindowForTargetReturns {
        GetWindowForTargetReturns {
            window_id: self.window_id,
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
    #[serde(rename = "windowId")]
    window_id: WindowID,
    /// New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined
    /// with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.
    bounds: Bounds,
}

impl SetWindowBoundsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `window_id`: Browser window id.
    /// * `bounds`: New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.
    pub fn builder(window_id: WindowID, bounds: Bounds) -> SetWindowBoundsParamsBuilder {
        SetWindowBoundsParamsBuilder {
            window_id: window_id,
            bounds: bounds,
        }
    }
    /// Browser window id.
    pub fn window_id(&self) -> &WindowID { &self.window_id }
    /// New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined
    /// with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.
    pub fn bounds(&self) -> &Bounds { &self.bounds }
}


pub struct SetWindowBoundsParamsBuilder {
    window_id: WindowID,
    bounds: Bounds,
}

impl SetWindowBoundsParamsBuilder {
    pub fn build(self) -> SetWindowBoundsParams {
        SetWindowBoundsParams {
            window_id: self.window_id,
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
    #[serde(rename = "windowId")]
    window_id: WindowID,
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
    /// Creates a builder for this type with the required parameters:
    /// * `window_id`: Browser window id.
    pub fn builder(window_id: WindowID) -> SetContentsSizeParamsBuilder {
        SetContentsSizeParamsBuilder {
            window_id: window_id,
            width: None,
            height: None,
        }
    }
    /// Browser window id.
    pub fn window_id(&self) -> &WindowID { &self.window_id }
    /// The window contents width in DIP. Assumes current width if omitted.
    /// Must be specified if 'height' is omitted.
    pub fn width(&self) -> Option<u64> { self.width }
    /// The window contents height in DIP. Assumes current height if omitted.
    /// Must be specified if 'width' is omitted.
    pub fn height(&self) -> Option<i64> { self.height }
}


pub struct SetContentsSizeParamsBuilder {
    window_id: WindowID,
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
            window_id: self.window_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "badgeLabel")]
    badge_label: Option<Cow<'a, str>>,
    /// Png encoded image. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Cow<'a, str>>,
}

impl<'a> SetDockTileParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> SetDockTileParamsBuilder<'a> {
        SetDockTileParamsBuilder {
            badge_label: None,
            image: None,
        }
    }
    pub fn badge_label(&self) -> Option<&str> { self.badge_label.as_deref() }
    /// Png encoded image. (Encoded as a base64 string when passed over JSON)
    pub fn image(&self) -> Option<&str> { self.image.as_deref() }
}

#[derive(Default)]
pub struct SetDockTileParamsBuilder<'a> {
    badge_label: Option<Cow<'a, str>>,
    image: Option<Cow<'a, str>>,
}

impl<'a> SetDockTileParamsBuilder<'a> {
    pub fn badge_label(mut self, badge_label: impl Into<Cow<'a, str>>) -> Self { self.badge_label = Some(badge_label.into()); self }
    /// Png encoded image. (Encoded as a base64 string when passed over JSON)
    pub fn image(mut self, image: impl Into<Cow<'a, str>>) -> Self { self.image = Some(image.into()); self }
    pub fn build(self) -> SetDockTileParams<'a> {
        SetDockTileParams {
            badge_label: self.badge_label,
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
    #[serde(rename = "commandId")]
    command_id: BrowserCommandId,
}

impl ExecuteBrowserCommandParams {
    /// Creates a builder for this type with the required parameters:
    /// * `command_id`: 
    pub fn builder(command_id: impl Into<BrowserCommandId>) -> ExecuteBrowserCommandParamsBuilder {
        ExecuteBrowserCommandParamsBuilder {
            command_id: command_id.into(),
        }
    }
    pub fn command_id(&self) -> &BrowserCommandId { &self.command_id }
}


pub struct ExecuteBrowserCommandParamsBuilder {
    command_id: BrowserCommandId,
}

impl ExecuteBrowserCommandParamsBuilder {
    pub fn build(self) -> ExecuteBrowserCommandParams {
        ExecuteBrowserCommandParams {
            command_id: self.command_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `url`: 
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
    #[serde(rename = "coordinatorOrigin")]
    coordinator_origin: Cow<'a, str>,
    #[serde(rename = "keyConfig")]
    key_config: Cow<'a, str>,
    /// BrowserContext to perform the action in. When omitted, default browser
    /// context is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> AddPrivacySandboxCoordinatorKeyConfigParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `api`: 
    /// * `coordinator_origin`: 
    /// * `key_config`: 
    pub fn builder(api: impl Into<PrivacySandboxAPI>, coordinator_origin: impl Into<Cow<'a, str>>, key_config: impl Into<Cow<'a, str>>) -> AddPrivacySandboxCoordinatorKeyConfigParamsBuilder<'a> {
        AddPrivacySandboxCoordinatorKeyConfigParamsBuilder {
            api: api.into(),
            coordinator_origin: coordinator_origin.into(),
            key_config: key_config.into(),
            browser_context_id: None,
        }
    }
    pub fn api(&self) -> &PrivacySandboxAPI { &self.api }
    pub fn coordinator_origin(&self) -> &str { self.coordinator_origin.as_ref() }
    pub fn key_config(&self) -> &str { self.key_config.as_ref() }
    /// BrowserContext to perform the action in. When omitted, default browser
    /// context is used.
    pub fn browser_context_id(&self) -> Option<&BrowserContextID<'a>> { self.browser_context_id.as_ref() }
}


pub struct AddPrivacySandboxCoordinatorKeyConfigParamsBuilder<'a> {
    api: PrivacySandboxAPI,
    coordinator_origin: Cow<'a, str>,
    key_config: Cow<'a, str>,
    browser_context_id: Option<BrowserContextID<'a>>,
}

impl<'a> AddPrivacySandboxCoordinatorKeyConfigParamsBuilder<'a> {
    /// BrowserContext to perform the action in. When omitted, default browser
    /// context is used.
    pub fn browser_context_id(mut self, browser_context_id: impl Into<BrowserContextID<'a>>) -> Self { self.browser_context_id = Some(browser_context_id.into()); self }
    pub fn build(self) -> AddPrivacySandboxCoordinatorKeyConfigParams<'a> {
        AddPrivacySandboxCoordinatorKeyConfigParams {
            api: self.api,
            coordinator_origin: self.coordinator_origin,
            key_config: self.key_config,
            browser_context_id: self.browser_context_id,
        }
    }
}

impl<'a> AddPrivacySandboxCoordinatorKeyConfigParams<'a> { pub const METHOD: &'static str = "Browser.addPrivacySandboxCoordinatorKeyConfig"; }

impl<'a> crate::CdpCommand<'a> for AddPrivacySandboxCoordinatorKeyConfigParams<'a> {
    const METHOD: &'static str = "Browser.addPrivacySandboxCoordinatorKeyConfig";
    type Response = crate::EmptyReturns;
}
