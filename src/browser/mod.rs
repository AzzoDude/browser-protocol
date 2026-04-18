use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! The Browser domain defines methods and events for browser managing.


pub type BrowserContextID = String;


pub type WindowID = i64;

/// The state of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum WindowState {
    #[default]
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

/// Browser window bounds information

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Bounds {
    /// The offset from the left edge of the screen to the window in pixels.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i64>,
    /// The offset from the top edge of the screen to the window in pixels.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    /// The window width in pixels.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    /// The window height in pixels.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The window state. Default to normal.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowState: Option<WindowState>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionType {
    #[default]
    Ar,
    AudioCapture,
    AutomaticFullscreen,
    BackgroundFetch,
    BackgroundSync,
    CameraPanTiltZoom,
    CapturedSurfaceControl,
    ClipboardReadWrite,
    ClipboardSanitizedWrite,
    DisplayCapture,
    DurableStorage,
    Geolocation,
    HandTracking,
    IdleDetection,
    KeyboardLock,
    LocalFonts,
    LocalNetwork,
    LocalNetworkAccess,
    LoopbackNetwork,
    Midi,
    MidiSysex,
    Nfc,
    Notifications,
    PaymentHandler,
    PeriodicBackgroundSync,
    PointerLock,
    ProtectedMediaIdentifier,
    Sensors,
    SmartCard,
    SpeakerSelection,
    StorageAccess,
    TopLevelStorageAccess,
    VideoCapture,
    Vr,
    WakeLockScreen,
    WakeLockSystem,
    WebAppInstallation,
    WebPrinting,
    WindowManagement,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionSetting {
    #[default]
    Granted,
    Denied,
    Prompt,
}

/// Definition of PermissionDescriptor defined in the Permissions API:
/// https://w3c.github.io/permissions/#dom-permissiondescriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDescriptor {
    /// Name of permission.
    /// See https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl for valid permission names.

    pub name: String,
    /// For "midi" permission, may also specify sysex control.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysex: Option<bool>,
    /// For "push" permission, may specify userVisibleOnly.
    /// Note that userVisibleOnly = true is the only currently supported type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userVisibleOnly: Option<bool>,
    /// For "clipboard" permission, may specify allowWithoutSanitization.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowWithoutSanitization: Option<bool>,
    /// For "fullscreen" permission, must specify allowWithoutGesture:true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowWithoutGesture: Option<bool>,
    /// For "camera" permission, may specify panTiltZoom.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub panTiltZoom: Option<bool>,
}

/// Browser command ids used by executeBrowserCommand.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BrowserCommandId {
    #[default]
    OpenTabSearch,
    CloseTabSearch,
    OpenGlic,
}

/// Chrome histogram bucket.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    /// Minimum value (inclusive).

    pub low: i64,
    /// Maximum value (exclusive).

    pub high: i64,
    /// Number of samples.

    pub count: u64,
}

/// Chrome histogram.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    /// Name.

    pub name: String,
    /// Sum of sample values.

    pub sum: i64,
    /// Total number of samples.

    pub count: u64,
    /// Buckets.

    pub buckets: Vec<Bucket>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PrivacySandboxAPI {
    #[default]
    BiddingAndAuctionServices,
    TrustedKeyValue,
}

/// Set permission settings for given embedding and embedded origins.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPermissionParams {
    /// Descriptor of permission to override.

    pub permission: PermissionDescriptor,
    /// Setting of the permission.

    pub setting: PermissionSetting,
    /// Embedding origin the permission applies to, all origins if not specified.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Embedded origin the permission applies to. It is ignored unless the embedding origin is
    /// present and valid. If the embedding origin is provided but the embedded origin isn't, the
    /// embedding origin is used as the embedded origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddedOrigin: Option<String>,
    /// Context to override. When omitted, default browser context is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<BrowserContextID>,
}

impl SetPermissionParams { pub const METHOD: &'static str = "Browser.setPermission"; }

impl crate::CdpCommand for SetPermissionParams {
    const METHOD: &'static str = "Browser.setPermission";
    type Response = crate::EmptyReturns;
}

/// Grant specific permissions to the given origin and reject all others. Deprecated. Use
/// setPermission instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GrantPermissionsParams {

    pub permissions: Vec<PermissionType>,
    /// Origin the permission applies to, all origins if not specified.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// BrowserContext to override permissions. When omitted, default browser context is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<BrowserContextID>,
}

impl GrantPermissionsParams { pub const METHOD: &'static str = "Browser.grantPermissions"; }

impl crate::CdpCommand for GrantPermissionsParams {
    const METHOD: &'static str = "Browser.grantPermissions";
    type Response = crate::EmptyReturns;
}

/// Reset all permission management for all origins.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetPermissionsParams {
    /// BrowserContext to reset permissions. When omitted, default browser context is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<BrowserContextID>,
}

impl ResetPermissionsParams { pub const METHOD: &'static str = "Browser.resetPermissions"; }

impl crate::CdpCommand for ResetPermissionsParams {
    const METHOD: &'static str = "Browser.resetPermissions";
    type Response = crate::EmptyReturns;
}

/// Set the behavior when downloading a file.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDownloadBehaviorParams {
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny). |allowAndName| allows download and names files according to
    /// their download guids.

    pub behavior: String,
    /// BrowserContext to set download behavior. When omitted, default browser context is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<BrowserContextID>,
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    /// or 'allowAndName'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloadPath: Option<String>,
    /// Whether to emit download events (defaults to false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventsEnabled: Option<bool>,
}

impl SetDownloadBehaviorParams { pub const METHOD: &'static str = "Browser.setDownloadBehavior"; }

impl crate::CdpCommand for SetDownloadBehaviorParams {
    const METHOD: &'static str = "Browser.setDownloadBehavior";
    type Response = crate::EmptyReturns;
}

/// Cancel a download if in progress

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelDownloadParams {
    /// Global unique identifier of the download.

    pub guid: String,
    /// BrowserContext to perform the action in. When omitted, default browser context is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<BrowserContextID>,
}

impl CancelDownloadParams { pub const METHOD: &'static str = "Browser.cancelDownload"; }

impl crate::CdpCommand for CancelDownloadParams {
    const METHOD: &'static str = "Browser.cancelDownload";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloseParams {}

impl CloseParams { pub const METHOD: &'static str = "Browser.close"; }

impl crate::CdpCommand for CloseParams {
    const METHOD: &'static str = "Browser.close";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrashParams {}

impl CrashParams { pub const METHOD: &'static str = "Browser.crash"; }

impl crate::CdpCommand for CrashParams {
    const METHOD: &'static str = "Browser.crash";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrashGpuProcessParams {}

impl CrashGpuProcessParams { pub const METHOD: &'static str = "Browser.crashGpuProcess"; }

impl crate::CdpCommand for CrashGpuProcessParams {
    const METHOD: &'static str = "Browser.crashGpuProcess";
    type Response = crate::EmptyReturns;
}

/// Returns version information.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVersionReturns {
    /// Protocol version.

    pub protocolVersion: String,
    /// Product name.

    pub product: String,
    /// Product revision.

    pub revision: String,
    /// User-Agent.

    pub userAgent: String,
    /// V8 version.

    pub jsVersion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetVersionParams {}

impl GetVersionParams { pub const METHOD: &'static str = "Browser.getVersion"; }

impl crate::CdpCommand for GetVersionParams {
    const METHOD: &'static str = "Browser.getVersion";
    type Response = GetVersionReturns;
}

/// Returns the command line switches for the browser process if, and only if
/// --enable-automation is on the commandline.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserCommandLineReturns {
    /// Commandline parameters

    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBrowserCommandLineParams {}

impl GetBrowserCommandLineParams { pub const METHOD: &'static str = "Browser.getBrowserCommandLine"; }

impl crate::CdpCommand for GetBrowserCommandLineParams {
    const METHOD: &'static str = "Browser.getBrowserCommandLine";
    type Response = GetBrowserCommandLineReturns;
}

/// Get Chrome histograms.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramsParams {
    /// Requested substring in name. Only histograms which have query as a
    /// substring in their name are extracted. An empty or absent query returns
    /// all histograms.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// If true, retrieve delta since last delta call.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<bool>,
}

/// Get Chrome histograms.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramsReturns {
    /// Histograms.

    pub histograms: Vec<Histogram>,
}

impl GetHistogramsParams { pub const METHOD: &'static str = "Browser.getHistograms"; }

impl crate::CdpCommand for GetHistogramsParams {
    const METHOD: &'static str = "Browser.getHistograms";
    type Response = GetHistogramsReturns;
}

/// Get a Chrome histogram by name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramParams {
    /// Requested histogram name.

    pub name: String,
    /// If true, retrieve delta since last delta call.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<bool>,
}

/// Get a Chrome histogram by name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramReturns {
    /// Histogram.

    pub histogram: Histogram,
}

impl GetHistogramParams { pub const METHOD: &'static str = "Browser.getHistogram"; }

impl crate::CdpCommand for GetHistogramParams {
    const METHOD: &'static str = "Browser.getHistogram";
    type Response = GetHistogramReturns;
}

/// Get position and size of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowBoundsParams {
    /// Browser window id.

    pub windowId: WindowID,
}

/// Get position and size of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowBoundsReturns {
    /// Bounds information of the window. When window state is 'minimized', the restored window
    /// position and size are returned.

    pub bounds: Bounds,
}

impl GetWindowBoundsParams { pub const METHOD: &'static str = "Browser.getWindowBounds"; }

impl crate::CdpCommand for GetWindowBoundsParams {
    const METHOD: &'static str = "Browser.getWindowBounds";
    type Response = GetWindowBoundsReturns;
}

/// Get the browser window that contains the devtools target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowForTargetParams {
    /// Devtools agent host id. If called as a part of the session, associated targetId is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetId: Option<crate::target::TargetID>,
}

/// Get the browser window that contains the devtools target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowForTargetReturns {
    /// Browser window id.

    pub windowId: WindowID,
    /// Bounds information of the window. When window state is 'minimized', the restored window
    /// position and size are returned.

    pub bounds: Bounds,
}

impl GetWindowForTargetParams { pub const METHOD: &'static str = "Browser.getWindowForTarget"; }

impl crate::CdpCommand for GetWindowForTargetParams {
    const METHOD: &'static str = "Browser.getWindowForTarget";
    type Response = GetWindowForTargetReturns;
}

/// Set position and/or size of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetWindowBoundsParams {
    /// Browser window id.

    pub windowId: WindowID,
    /// New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined
    /// with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.

    pub bounds: Bounds,
}

impl SetWindowBoundsParams { pub const METHOD: &'static str = "Browser.setWindowBounds"; }

impl crate::CdpCommand for SetWindowBoundsParams {
    const METHOD: &'static str = "Browser.setWindowBounds";
    type Response = crate::EmptyReturns;
}

/// Set size of the browser contents resizing browser window as necessary.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContentsSizeParams {
    /// Browser window id.

    pub windowId: WindowID,
    /// The window contents width in DIP. Assumes current width if omitted.
    /// Must be specified if 'height' is omitted.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    /// The window contents height in DIP. Assumes current height if omitted.
    /// Must be specified if 'width' is omitted.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
}

impl SetContentsSizeParams { pub const METHOD: &'static str = "Browser.setContentsSize"; }

impl crate::CdpCommand for SetContentsSizeParams {
    const METHOD: &'static str = "Browser.setContentsSize";
    type Response = crate::EmptyReturns;
}

/// Set dock tile details, platform-specific.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDockTileParams {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub badgeLabel: Option<String>,
    /// Png encoded image. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

impl SetDockTileParams { pub const METHOD: &'static str = "Browser.setDockTile"; }

impl crate::CdpCommand for SetDockTileParams {
    const METHOD: &'static str = "Browser.setDockTile";
    type Response = crate::EmptyReturns;
}

/// Invoke custom browser commands used by telemetry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteBrowserCommandParams {

    pub commandId: BrowserCommandId,
}

impl ExecuteBrowserCommandParams { pub const METHOD: &'static str = "Browser.executeBrowserCommand"; }

impl crate::CdpCommand for ExecuteBrowserCommandParams {
    const METHOD: &'static str = "Browser.executeBrowserCommand";
    type Response = crate::EmptyReturns;
}

/// Allows a site to use privacy sandbox features that require enrollment
/// without the site actually being enrolled. Only supported on page targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddPrivacySandboxEnrollmentOverrideParams {

    pub url: String,
}

impl AddPrivacySandboxEnrollmentOverrideParams { pub const METHOD: &'static str = "Browser.addPrivacySandboxEnrollmentOverride"; }

impl crate::CdpCommand for AddPrivacySandboxEnrollmentOverrideParams {
    const METHOD: &'static str = "Browser.addPrivacySandboxEnrollmentOverride";
    type Response = crate::EmptyReturns;
}

/// Configures encryption keys used with a given privacy sandbox API to talk
/// to a trusted coordinator.  Since this is intended for test automation only,
/// coordinatorOrigin must be a .test domain. No existing coordinator
/// configuration for the origin may exist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddPrivacySandboxCoordinatorKeyConfigParams {

    pub api: PrivacySandboxAPI,

    pub coordinatorOrigin: String,

    pub keyConfig: String,
    /// BrowserContext to perform the action in. When omitted, default browser
    /// context is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<BrowserContextID>,
}

impl AddPrivacySandboxCoordinatorKeyConfigParams { pub const METHOD: &'static str = "Browser.addPrivacySandboxCoordinatorKeyConfig"; }

impl crate::CdpCommand for AddPrivacySandboxCoordinatorKeyConfigParams {
    const METHOD: &'static str = "Browser.addPrivacySandboxCoordinatorKeyConfig";
    type Response = crate::EmptyReturns;
}
