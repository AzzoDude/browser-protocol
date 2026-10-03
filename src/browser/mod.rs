//! The Browser domain defines methods and events for browser managing.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "windowState")]
    pub window_state: Option<WindowState>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDescriptor<'a> {
    /// Name of permission.
    /// See <https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl> for valid permission names.
    pub name: Cow<'a, str>,
    /// For "midi" permission, may also specify sysex control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysex: Option<bool>,
    /// For "push" permission, may specify userVisibleOnly.
    /// Note that userVisibleOnly = true is the only currently supported type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "userVisibleOnly")]
    pub user_visible_only: Option<bool>,
    /// For "clipboard" permission, may specify allowWithoutSanitization.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowWithoutSanitization")]
    pub allow_without_sanitization: Option<bool>,
    /// For "fullscreen" permission, must specify allowWithoutGesture:true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowWithoutGesture")]
    pub allow_without_gesture: Option<bool>,
    /// For "camera" permission, may specify panTiltZoom.
    #[serde(skip_serializing_if = "Option::is_none", rename = "panTiltZoom")]
    pub pan_tilt_zoom: Option<bool>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Histogram<'a> {
    /// Name.
    pub name: Cow<'a, str>,
    /// Sum of sample values.
    pub sum: i64,
    /// Total number of samples.
    pub count: u64,
    /// Buckets.
    pub buckets: Vec<Bucket>,
}
/// Set permission settings for given embedding and embedded origins.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.setPermission")]
pub struct SetPermissionParams<'a> {
    /// Descriptor of permission to override.
    pub permission: PermissionDescriptor<'a>,
    /// Setting of the permission.
    pub setting: PermissionSetting,
    /// Embedding origin the permission applies to, all origins if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Cow<'a, str>>,
    /// Embedded origin the permission applies to. It is ignored unless the embedding origin is
    /// present and valid. If the embedding origin is provided but the embedded origin isn't, the
    /// embedding origin is used as the embedded origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "embeddedOrigin")]
    pub embedded_origin: Option<Cow<'a, str>>,
    /// Context to override. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextID<'a>>,
}
/// Grant specific permissions to the given origin and reject all others. Deprecated. Use
/// setPermission instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.grantPermissions")]
pub struct GrantPermissionsParams<'a> {
    pub permissions: Vec<PermissionType>,
    /// Origin the permission applies to, all origins if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Cow<'a, str>>,
    /// BrowserContext to override permissions. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextID<'a>>,
}
/// Reset all permission management for all origins.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.resetPermissions")]
pub struct ResetPermissionsParams<'a> {
    /// BrowserContext to reset permissions. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextID<'a>>,
}
/// Set the behavior when downloading a file.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.setDownloadBehavior")]
pub struct SetDownloadBehaviorParams<'a> {
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny). |allowAndName| allows download and names files according to
    /// their download guids.
    pub behavior: Cow<'a, str>,
    /// BrowserContext to set download behavior. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextID<'a>>,
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    /// or 'allowAndName'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "downloadPath")]
    pub download_path: Option<Cow<'a, str>>,
    /// Whether to emit download events (defaults to false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "eventsEnabled")]
    pub events_enabled: Option<bool>,
}
/// Cancel a download if in progress

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.cancelDownload")]
pub struct CancelDownloadParams<'a> {
    /// Global unique identifier of the download.
    pub guid: Cow<'a, str>,
    /// BrowserContext to perform the action in. When omitted, default browser context is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<BrowserContextID<'a>>,
}
/// Close browser gracefully.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.close")]
pub struct CloseParams {

}
/// Crashes browser on the main thread.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.crash")]
pub struct CrashParams {

}
/// Crashes GPU process.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.crashGpuProcess")]
pub struct CrashGpuProcessParams {

}
/// Returns version information.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.getVersion", response = "GetVersionReturns<'a>")]
pub struct GetVersionParams {

}
/// Returns version information.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetVersionReturns<'a> {
    /// Protocol version.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: Cow<'a, str>,
    /// Product name.
    pub product: Cow<'a, str>,
    /// Product revision.
    pub revision: Cow<'a, str>,
    /// User-Agent.
    #[serde(rename = "userAgent")]
    pub user_agent: Cow<'a, str>,
    /// V8 version.
    #[serde(rename = "jsVersion")]
    pub js_version: Cow<'a, str>,
}
/// Returns the command line switches for the browser process if, and only if
/// --enable-automation is on the commandline.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.getBrowserCommandLine", response = "GetBrowserCommandLineReturns<'a>")]
pub struct GetBrowserCommandLineParams {

}
/// Returns the command line switches for the browser process if, and only if
/// --enable-automation is on the commandline.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserCommandLineReturns<'a> {
    /// Commandline parameters
    pub arguments: Vec<Cow<'a, str>>,
}
/// Adds or updates a mock camera in the shared video capture device list for
/// test automation. The mock camera is not scoped to a particular page or
/// frame and is removed when the DevTools session that created it disconnects.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.addMockCamera")]
pub struct AddMockCameraParams<'a> {
    /// Required non-empty identifier for the mock camera. This is mapped to an
    /// internal virtual-device identifier and is not the MediaDeviceInfo.deviceId
    /// exposed to the page.
    #[serde(rename = "deviceId")]
    pub device_id: Cow<'a, str>,
}
/// Get Chrome histograms.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.getHistograms", response = "GetHistogramsReturns<'a>")]
pub struct GetHistogramsParams<'a> {
    /// Requested substring in name. Only histograms which have query as a
    /// substring in their name are extracted. An empty or absent query returns
    /// all histograms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Cow<'a, str>>,
    /// If true, retrieve delta since last delta call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<bool>,
}
/// Get Chrome histograms.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramsReturns<'a> {
    /// Histograms.
    pub histograms: Vec<Histogram<'a>>,
}
/// Get a Chrome histogram by name.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.getHistogram", response = "GetHistogramReturns<'a>")]
pub struct GetHistogramParams<'a> {
    /// Requested histogram name.
    pub name: Cow<'a, str>,
    /// If true, retrieve delta since last delta call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<bool>,
}
/// Get a Chrome histogram by name.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetHistogramReturns<'a> {
    /// Histogram.
    pub histogram: Histogram<'a>,
}
/// Get position and size of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.getWindowBounds", response = "GetWindowBoundsReturns")]
pub struct GetWindowBoundsParams {
    /// Browser window id.
    #[serde(rename = "windowId")]
    pub window_id: WindowID,
}
/// Get position and size of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowBoundsReturns {
    /// Bounds information of the window. When window state is 'minimized', the restored window
    /// position and size are returned.
    pub bounds: Bounds,
}
/// Get the browser window that contains the devtools target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.getWindowForTarget", response = "GetWindowForTargetReturns")]
pub struct GetWindowForTargetParams<'a> {
    /// Devtools agent host id. If called as a part of the session, associated targetId is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    pub target_id: Option<crate::target::TargetID<'a>>,
}
/// Get the browser window that contains the devtools target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowForTargetReturns {
    /// Browser window id.
    #[serde(rename = "windowId")]
    pub window_id: WindowID,
    /// Bounds information of the window. When window state is 'minimized', the restored window
    /// position and size are returned.
    pub bounds: Bounds,
}
/// Set position and/or size of the browser window.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.setWindowBounds")]
pub struct SetWindowBoundsParams {
    /// Browser window id.
    #[serde(rename = "windowId")]
    pub window_id: WindowID,
    /// New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined
    /// with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.
    pub bounds: Bounds,
}
/// Set size of the browser contents resizing browser window as necessary.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.setContentsSize")]
pub struct SetContentsSizeParams {
    /// Browser window id.
    #[serde(rename = "windowId")]
    pub window_id: WindowID,
    /// The window contents width in DIP. Assumes current width if omitted.
    /// Must be specified if 'height' is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    /// The window contents height in DIP. Assumes current height if omitted.
    /// Must be specified if 'width' is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
}
/// Set dock tile details, platform-specific.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.setDockTile")]
pub struct SetDockTileParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "badgeLabel")]
    pub badge_label: Option<Cow<'a, str>>,
    /// Png encoded image. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Cow<'a, str>>,
}
/// Invoke custom browser commands used by telemetry.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.executeBrowserCommand")]
pub struct ExecuteBrowserCommandParams {
    #[serde(rename = "commandId")]
    pub command_id: BrowserCommandId,
}
/// Allows a site to use privacy sandbox features that require enrollment
/// without the site actually being enrolled. Only supported on page targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.addPrivacySandboxEnrollmentOverride")]
pub struct AddPrivacySandboxEnrollmentOverrideParams<'a> {
    pub url: Cow<'a, str>,
}
/// Gets the current globally-applied privacy control status
/// See <https://www.w3.org/TR/gpc/#get-global-privacy-control>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.getGlobalPrivacyControl", response = "GetGlobalPrivacyControlReturns")]
pub struct GetGlobalPrivacyControlParams {

}
/// Gets the current globally-applied privacy control status
/// See <https://www.w3.org/TR/gpc/#get-global-privacy-control>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetGlobalPrivacyControlReturns {
    pub gpc: bool,
}
/// Sets and then gets the current globally-applied privacy control status
/// See <https://www.w3.org/TR/gpc/#set-global-privacy-control>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.setGlobalPrivacyControl", response = "SetGlobalPrivacyControlReturns")]
pub struct SetGlobalPrivacyControlParams {
    pub gpc: bool,
}
/// Sets and then gets the current globally-applied privacy control status
/// See <https://www.w3.org/TR/gpc/#set-global-privacy-control>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetGlobalPrivacyControlReturns {
    pub gpc: bool,
}
/// Fired when page is about to start a download.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.downloadWillBegin")]
pub struct DownloadWillBegin<'a> {
    /// Id of the frame that caused the download to begin.
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
    /// Global unique identifier of the download.
    pub guid: Cow<'a, str>,
    /// URL of the resource being downloaded.
    pub url: Cow<'a, str>,
    /// Suggested file name of the resource (the actual name of the file saved on disk may differ).
    #[serde(rename = "suggestedFilename")]
    pub suggested_filename: Cow<'a, str>,
}
/// Fired when download makes progress. Last call has |done| == true.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Browser.downloadProgress")]
pub struct DownloadProgress<'a> {
    /// Global unique identifier of the download.
    pub guid: Cow<'a, str>,
    /// Total expected bytes to download.
    #[serde(rename = "totalBytes")]
    pub total_bytes: f64,
    /// Total bytes received.
    #[serde(rename = "receivedBytes")]
    pub received_bytes: f64,
    /// Download status.
    pub state: Cow<'a, str>,
    /// If download is "completed", provides the path of the downloaded file.
    /// Depending on the platform, it is not guaranteed to be set, nor the file
    /// is guaranteed to exist.
    #[serde(skip_serializing_if = "Option::is_none", rename = "filePath")]
    pub file_path: Option<Cow<'a, str>>,
}