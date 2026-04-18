use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! This domain emulates different environments for the page.


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SafeAreaInsets {
    /// Overrides safe-area-inset-top.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    /// Overrides safe-area-max-inset-top.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topMax: Option<i64>,
    /// Overrides safe-area-inset-left.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i64>,
    /// Overrides safe-area-max-inset-left.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leftMax: Option<i64>,
    /// Overrides safe-area-inset-bottom.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<i64>,
    /// Overrides safe-area-max-inset-bottom.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottomMax: Option<i64>,
    /// Overrides safe-area-inset-right.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<i64>,
    /// Overrides safe-area-max-inset-right.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rightMax: Option<i64>,
}

/// Screen orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreenOrientation {
    /// Orientation type.

    #[serde(rename = "type")]
    pub type_: String,
    /// Orientation angle.

    pub angle: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisplayFeature {
    /// Orientation of a display feature in relation to screen

    pub orientation: String,
    /// The offset from the screen origin in either the x (for vertical
    /// orientation) or y (for horizontal orientation) direction.

    pub offset: i32,
    /// A display feature may mask content such that it is not physically
    /// displayed - this length along with the offset describes this area.
    /// A display feature that only splits content will have a 0 mask_length.

    pub maskLength: u64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DevicePosture {
    /// Current posture of the device

    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaFeature {

    pub name: String,

    pub value: String,
}

/// advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to
/// allow the next delayed task (if any) to run; pause: The virtual time base may not advance;
/// pauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending
/// resource fetches.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum VirtualTimePolicy {
    #[default]
    Advance,
    Pause,
    PauseIfNetworkFetchesPending,
}

/// Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentBrandVersion {

    pub brand: String,

    pub version: String,
}

/// Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints
/// Missing optional values will be filled in by the target with what it would normally use.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentMetadata {
    /// Brands appearing in Sec-CH-UA.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands: Option<Vec<UserAgentBrandVersion>>,
    /// Brands appearing in Sec-CH-UA-Full-Version-List.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullVersionList: Option<Vec<UserAgentBrandVersion>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullVersion: Option<String>,

    pub platform: String,

    pub platformVersion: String,

    pub architecture: String,

    pub model: String,

    pub mobile: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitness: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wow64: Option<bool>,
    /// Used to specify User Agent form-factor values.
    /// See https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors

    #[serde(skip_serializing_if = "Option::is_none")]
    pub formFactors: Option<Vec<String>>,
}

/// Used to specify sensor types to emulate.
/// See https://w3c.github.io/sensors/#automation for more information.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SensorType {
    #[default]
    AbsoluteOrientation,
    Accelerometer,
    AmbientLight,
    Gravity,
    Gyroscope,
    LinearAcceleration,
    Magnetometer,
    RelativeOrientation,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorMetadata {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimumFrequency: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximumFrequency: Option<f64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingSingle {

    pub value: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingXYZ {

    pub x: f64,

    pub y: f64,

    pub z: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingQuaternion {

    pub x: f64,

    pub y: f64,

    pub z: f64,

    pub w: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorReading {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub single: Option<SensorReadingSingle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xyz: Option<SensorReadingXYZ>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quaternion: Option<SensorReadingQuaternion>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PressureSource {
    #[default]
    Cpu,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PressureState {
    #[default]
    Nominal,
    Fair,
    Serious,
    Critical,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PressureMetadata {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorkAreaInsets {
    /// Work area top inset in pixels. Default is 0;

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    /// Work area left inset in pixels. Default is 0;

    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i64>,
    /// Work area bottom inset in pixels. Default is 0;

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<i64>,
    /// Work area right inset in pixels. Default is 0;

    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<i64>,
}


pub type ScreenId = String;

/// Screen information similar to the one returned by window.getScreenDetails() method,
/// see https://w3c.github.io/window-management/#screendetailed.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreenInfo {
    /// Offset of the left edge of the screen.

    pub left: i64,
    /// Offset of the top edge of the screen.

    pub top: i64,
    /// Width of the screen.

    pub width: u64,
    /// Height of the screen.

    pub height: i64,
    /// Offset of the left edge of the available screen area.

    pub availLeft: i64,
    /// Offset of the top edge of the available screen area.

    pub availTop: i64,
    /// Width of the available screen area.

    pub availWidth: u64,
    /// Height of the available screen area.

    pub availHeight: i64,
    /// Specifies the screen's device pixel ratio.

    pub devicePixelRatio: f64,
    /// Specifies the screen's orientation.

    pub orientation: ScreenOrientation,
    /// Specifies the screen's color depth in bits.

    pub colorDepth: i64,
    /// Indicates whether the device has multiple screens.

    pub isExtended: bool,
    /// Indicates whether the screen is internal to the device or external, attached to the device.

    pub isInternal: bool,
    /// Indicates whether the screen is set as the the operating system primary screen.

    pub isPrimary: bool,
    /// Specifies the descriptive label for the screen.

    pub label: String,
    /// Specifies the unique identifier of the screen.

    pub id: ScreenId,
}

/// Enum of image types that can be disabled.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DisabledImageType {
    #[default]
    Avif,
    Jxl,
    Webp,
}

/// Tells whether emulation is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulateReturns {
    /// True if emulation is supported.

    pub result: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanEmulateParams {}

impl CanEmulateParams { pub const METHOD: &'static str = "Emulation.canEmulate"; }

impl crate::CdpCommand for CanEmulateParams {
    const METHOD: &'static str = "Emulation.canEmulate";
    type Response = CanEmulateReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDeviceMetricsOverrideParams {}

impl ClearDeviceMetricsOverrideParams { pub const METHOD: &'static str = "Emulation.clearDeviceMetricsOverride"; }

impl crate::CdpCommand for ClearDeviceMetricsOverrideParams {
    const METHOD: &'static str = "Emulation.clearDeviceMetricsOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearGeolocationOverrideParams {}

impl ClearGeolocationOverrideParams { pub const METHOD: &'static str = "Emulation.clearGeolocationOverride"; }

impl crate::CdpCommand for ClearGeolocationOverrideParams {
    const METHOD: &'static str = "Emulation.clearGeolocationOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResetPageScaleFactorParams {}

impl ResetPageScaleFactorParams { pub const METHOD: &'static str = "Emulation.resetPageScaleFactor"; }

impl crate::CdpCommand for ResetPageScaleFactorParams {
    const METHOD: &'static str = "Emulation.resetPageScaleFactor";
    type Response = crate::EmptyReturns;
}

/// Enables or disables simulating a focused and active page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFocusEmulationEnabledParams {
    /// Whether to enable to disable focus emulation.

    pub enabled: bool,
}

impl SetFocusEmulationEnabledParams { pub const METHOD: &'static str = "Emulation.setFocusEmulationEnabled"; }

impl crate::CdpCommand for SetFocusEmulationEnabledParams {
    const METHOD: &'static str = "Emulation.setFocusEmulationEnabled";
    type Response = crate::EmptyReturns;
}

/// Automatically render all web contents using a dark theme.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoDarkModeOverrideParams {
    /// Whether to enable or disable automatic dark mode.
    /// If not specified, any existing override will be cleared.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl SetAutoDarkModeOverrideParams { pub const METHOD: &'static str = "Emulation.setAutoDarkModeOverride"; }

impl crate::CdpCommand for SetAutoDarkModeOverrideParams {
    const METHOD: &'static str = "Emulation.setAutoDarkModeOverride";
    type Response = crate::EmptyReturns;
}

/// Enables CPU throttling to emulate slow CPUs.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCPUThrottlingRateParams {
    /// Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc).

    pub rate: f64,
}

impl SetCPUThrottlingRateParams { pub const METHOD: &'static str = "Emulation.setCPUThrottlingRate"; }

impl crate::CdpCommand for SetCPUThrottlingRateParams {
    const METHOD: &'static str = "Emulation.setCPUThrottlingRate";
    type Response = crate::EmptyReturns;
}

/// Sets or clears an override of the default background color of the frame. This override is used
/// if the content does not specify one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultBackgroundColorOverrideParams {
    /// RGBA of the default background color. If not specified, any existing override will be
    /// cleared.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::dom::RGBA>,
}

impl SetDefaultBackgroundColorOverrideParams { pub const METHOD: &'static str = "Emulation.setDefaultBackgroundColorOverride"; }

impl crate::CdpCommand for SetDefaultBackgroundColorOverrideParams {
    const METHOD: &'static str = "Emulation.setDefaultBackgroundColorOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the
/// respective variables to be undefined, even if previously overridden.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSafeAreaInsetsOverrideParams {

    pub insets: SafeAreaInsets,
}

impl SetSafeAreaInsetsOverrideParams { pub const METHOD: &'static str = "Emulation.setSafeAreaInsetsOverride"; }

impl crate::CdpCommand for SetSafeAreaInsetsOverrideParams {
    const METHOD: &'static str = "Emulation.setSafeAreaInsetsOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
/// window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
/// query results).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceMetricsOverrideParams {
    /// Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.

    pub width: u64,
    /// Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.

    pub height: i64,
    /// Overriding device scale factor value. 0 disables the override.

    pub deviceScaleFactor: f64,
    /// Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    /// autosizing and more.

    pub mobile: bool,
    /// Scale to apply to resulting view image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Overriding screen width value in pixels (minimum 0, maximum 10000000).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenWidth: Option<u64>,
    /// Overriding screen height value in pixels (minimum 0, maximum 10000000).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenHeight: Option<i64>,
    /// Overriding view X position on screen in pixels (minimum 0, maximum 10000000).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub positionX: Option<i64>,
    /// Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub positionY: Option<i64>,
    /// Do not set visible view size, rely upon explicit setVisibleSize call.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dontSetVisibleSize: Option<bool>,
    /// Screen orientation override.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenOrientation: Option<ScreenOrientation>,
    /// If set, the visible area of the page will be overridden to this viewport. This viewport
    /// change is not observed by the page, e.g. viewport-relative elements do not change positions.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<crate::page::Viewport>,
    /// If set, the display feature of a multi-segment screen. If not set, multi-segment support
    /// is turned-off.
    /// Deprecated, use Emulation.setDisplayFeaturesOverride.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayFeature: Option<DisplayFeature>,
    /// If set, the posture of a foldable device. If not set the posture is set
    /// to continuous.
    /// Deprecated, use Emulation.setDevicePostureOverride.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicePosture: Option<DevicePosture>,
    /// Scrollbar type. Default: 'default'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollbarType: Option<String>,
    /// If set to true, enables screen orientation lock emulation, which
    /// intercepts screen.orientation.lock() calls from the page and reports
    /// orientation changes via screenOrientationLockChanged events. This is
    /// useful for emulating mobile device orientation lock behavior in
    /// responsive design mode.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenOrientationLockEmulation: Option<bool>,
}

impl SetDeviceMetricsOverrideParams { pub const METHOD: &'static str = "Emulation.setDeviceMetricsOverride"; }

impl crate::CdpCommand for SetDeviceMetricsOverrideParams {
    const METHOD: &'static str = "Emulation.setDeviceMetricsOverride";
    type Response = crate::EmptyReturns;
}

/// Start reporting the given posture value to the Device Posture API.
/// This override can also be set in setDeviceMetricsOverride().

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDevicePostureOverrideParams {

    pub posture: DevicePosture,
}

impl SetDevicePostureOverrideParams { pub const METHOD: &'static str = "Emulation.setDevicePostureOverride"; }

impl crate::CdpCommand for SetDevicePostureOverrideParams {
    const METHOD: &'static str = "Emulation.setDevicePostureOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDevicePostureOverrideParams {}

impl ClearDevicePostureOverrideParams { pub const METHOD: &'static str = "Emulation.clearDevicePostureOverride"; }

impl crate::CdpCommand for ClearDevicePostureOverrideParams {
    const METHOD: &'static str = "Emulation.clearDevicePostureOverride";
    type Response = crate::EmptyReturns;
}

/// Start using the given display features to pupulate the Viewport Segments API.
/// This override can also be set in setDeviceMetricsOverride().

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayFeaturesOverrideParams {

    pub features: Vec<DisplayFeature>,
}

impl SetDisplayFeaturesOverrideParams { pub const METHOD: &'static str = "Emulation.setDisplayFeaturesOverride"; }

impl crate::CdpCommand for SetDisplayFeaturesOverrideParams {
    const METHOD: &'static str = "Emulation.setDisplayFeaturesOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDisplayFeaturesOverrideParams {}

impl ClearDisplayFeaturesOverrideParams { pub const METHOD: &'static str = "Emulation.clearDisplayFeaturesOverride"; }

impl crate::CdpCommand for ClearDisplayFeaturesOverrideParams {
    const METHOD: &'static str = "Emulation.clearDisplayFeaturesOverride";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScrollbarsHiddenParams {
    /// Whether scrollbars should be always hidden.

    pub hidden: bool,
}

impl SetScrollbarsHiddenParams { pub const METHOD: &'static str = "Emulation.setScrollbarsHidden"; }

impl crate::CdpCommand for SetScrollbarsHiddenParams {
    const METHOD: &'static str = "Emulation.setScrollbarsHidden";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentCookieDisabledParams {
    /// Whether document.coookie API should be disabled.

    pub disabled: bool,
}

impl SetDocumentCookieDisabledParams { pub const METHOD: &'static str = "Emulation.setDocumentCookieDisabled"; }

impl crate::CdpCommand for SetDocumentCookieDisabledParams {
    const METHOD: &'static str = "Emulation.setDocumentCookieDisabled";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEmitTouchEventsForMouseParams {
    /// Whether touch emulation based on mouse input should be enabled.

    pub enabled: bool,
    /// Touch/gesture events configuration. Default: current platform.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
}

impl SetEmitTouchEventsForMouseParams { pub const METHOD: &'static str = "Emulation.setEmitTouchEventsForMouse"; }

impl crate::CdpCommand for SetEmitTouchEventsForMouseParams {
    const METHOD: &'static str = "Emulation.setEmitTouchEventsForMouse";
    type Response = crate::EmptyReturns;
}

/// Emulates the given media type or media feature for CSS media queries.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedMediaParams {
    /// Media type to emulate. Empty string disables the override.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<String>,
    /// Media features to emulate.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<MediaFeature>>,
}

impl SetEmulatedMediaParams { pub const METHOD: &'static str = "Emulation.setEmulatedMedia"; }

impl crate::CdpCommand for SetEmulatedMediaParams {
    const METHOD: &'static str = "Emulation.setEmulatedMedia";
    type Response = crate::EmptyReturns;
}

/// Emulates the given vision deficiency.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedVisionDeficiencyParams {
    /// Vision deficiency to emulate. Order: best-effort emulations come first, followed by any
    /// physiologically accurate emulations for medically recognized color vision deficiencies.

    #[serde(rename = "type")]
    pub type_: String,
}

impl SetEmulatedVisionDeficiencyParams { pub const METHOD: &'static str = "Emulation.setEmulatedVisionDeficiency"; }

impl crate::CdpCommand for SetEmulatedVisionDeficiencyParams {
    const METHOD: &'static str = "Emulation.setEmulatedVisionDeficiency";
    type Response = crate::EmptyReturns;
}

/// Emulates the given OS text scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedOSTextScaleParams {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}

impl SetEmulatedOSTextScaleParams { pub const METHOD: &'static str = "Emulation.setEmulatedOSTextScale"; }

impl crate::CdpCommand for SetEmulatedOSTextScaleParams {
    const METHOD: &'static str = "Emulation.setEmulatedOSTextScale";
    type Response = crate::EmptyReturns;
}

/// Overrides the Geolocation Position or Error. Omitting latitude, longitude or
/// accuracy emulates position unavailable.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetGeolocationOverrideParams {
    /// Mock latitude

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Mock longitude

    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Mock accuracy

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    /// Mock altitude

    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    /// Mock altitudeAccuracy

    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitudeAccuracy: Option<f64>,
    /// Mock heading

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<f64>,
    /// Mock speed

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
}

impl SetGeolocationOverrideParams { pub const METHOD: &'static str = "Emulation.setGeolocationOverride"; }

impl crate::CdpCommand for SetGeolocationOverrideParams {
    const METHOD: &'static str = "Emulation.setGeolocationOverride";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOverriddenSensorInformationParams {

    #[serde(rename = "type")]
    pub type_: SensorType,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOverriddenSensorInformationReturns {

    pub requestedSamplingFrequency: f64,
}

impl GetOverriddenSensorInformationParams { pub const METHOD: &'static str = "Emulation.getOverriddenSensorInformation"; }

impl crate::CdpCommand for GetOverriddenSensorInformationParams {
    const METHOD: &'static str = "Emulation.getOverriddenSensorInformation";
    type Response = GetOverriddenSensorInformationReturns;
}

/// Overrides a platform sensor of a given type. If |enabled| is true, calls to
/// Sensor.start() will use a virtual sensor as backend rather than fetching
/// data from a real hardware sensor. Otherwise, existing virtual
/// sensor-backend Sensor objects will fire an error event and new calls to
/// Sensor.start() will attempt to use a real sensor instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSensorOverrideEnabledParams {

    pub enabled: bool,

    #[serde(rename = "type")]
    pub type_: SensorType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SensorMetadata>,
}

impl SetSensorOverrideEnabledParams { pub const METHOD: &'static str = "Emulation.setSensorOverrideEnabled"; }

impl crate::CdpCommand for SetSensorOverrideEnabledParams {
    const METHOD: &'static str = "Emulation.setSensorOverrideEnabled";
    type Response = crate::EmptyReturns;
}

/// Updates the sensor readings reported by a sensor type previously overridden
/// by setSensorOverrideEnabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSensorOverrideReadingsParams {

    #[serde(rename = "type")]
    pub type_: SensorType,

    pub reading: SensorReading,
}

impl SetSensorOverrideReadingsParams { pub const METHOD: &'static str = "Emulation.setSensorOverrideReadings"; }

impl crate::CdpCommand for SetSensorOverrideReadingsParams {
    const METHOD: &'static str = "Emulation.setSensorOverrideReadings";
    type Response = crate::EmptyReturns;
}

/// Overrides a pressure source of a given type, as used by the Compute
/// Pressure API, so that updates to PressureObserver.observe() are provided
/// via setPressureStateOverride instead of being retrieved from
/// platform-provided telemetry data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureSourceOverrideEnabledParams {

    pub enabled: bool,

    pub source: PressureSource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PressureMetadata>,
}

impl SetPressureSourceOverrideEnabledParams { pub const METHOD: &'static str = "Emulation.setPressureSourceOverrideEnabled"; }

impl crate::CdpCommand for SetPressureSourceOverrideEnabledParams {
    const METHOD: &'static str = "Emulation.setPressureSourceOverrideEnabled";
    type Response = crate::EmptyReturns;
}

/// TODO: OBSOLETE: To remove when setPressureDataOverride is merged.
/// Provides a given pressure state that will be processed and eventually be
/// delivered to PressureObserver users. |source| must have been previously
/// overridden by setPressureSourceOverrideEnabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureStateOverrideParams {

    pub source: PressureSource,

    pub state: PressureState,
}

impl SetPressureStateOverrideParams { pub const METHOD: &'static str = "Emulation.setPressureStateOverride"; }

impl crate::CdpCommand for SetPressureStateOverrideParams {
    const METHOD: &'static str = "Emulation.setPressureStateOverride";
    type Response = crate::EmptyReturns;
}

/// Provides a given pressure data set that will be processed and eventually be
/// delivered to PressureObserver users. |source| must have been previously
/// overridden by setPressureSourceOverrideEnabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureDataOverrideParams {

    pub source: PressureSource,

    pub state: PressureState,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownContributionEstimate: Option<f64>,
}

impl SetPressureDataOverrideParams { pub const METHOD: &'static str = "Emulation.setPressureDataOverride"; }

impl crate::CdpCommand for SetPressureDataOverrideParams {
    const METHOD: &'static str = "Emulation.setPressureDataOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides the Idle state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetIdleOverrideParams {
    /// Mock isUserActive

    pub isUserActive: bool,
    /// Mock isScreenUnlocked

    pub isScreenUnlocked: bool,
}

impl SetIdleOverrideParams { pub const METHOD: &'static str = "Emulation.setIdleOverride"; }

impl crate::CdpCommand for SetIdleOverrideParams {
    const METHOD: &'static str = "Emulation.setIdleOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearIdleOverrideParams {}

impl ClearIdleOverrideParams { pub const METHOD: &'static str = "Emulation.clearIdleOverride"; }

impl crate::CdpCommand for ClearIdleOverrideParams {
    const METHOD: &'static str = "Emulation.clearIdleOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides value returned by the javascript navigator object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNavigatorOverridesParams {
    /// The platform navigator.platform should return.

    pub platform: String,
}

impl SetNavigatorOverridesParams { pub const METHOD: &'static str = "Emulation.setNavigatorOverrides"; }

impl crate::CdpCommand for SetNavigatorOverridesParams {
    const METHOD: &'static str = "Emulation.setNavigatorOverrides";
    type Response = crate::EmptyReturns;
}

/// Sets a specified page scale factor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPageScaleFactorParams {
    /// Page scale factor.

    pub pageScaleFactor: f64,
}

impl SetPageScaleFactorParams { pub const METHOD: &'static str = "Emulation.setPageScaleFactor"; }

impl crate::CdpCommand for SetPageScaleFactorParams {
    const METHOD: &'static str = "Emulation.setPageScaleFactor";
    type Response = crate::EmptyReturns;
}

/// Switches script execution in the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScriptExecutionDisabledParams {
    /// Whether script execution should be disabled in the page.

    pub value: bool,
}

impl SetScriptExecutionDisabledParams { pub const METHOD: &'static str = "Emulation.setScriptExecutionDisabled"; }

impl crate::CdpCommand for SetScriptExecutionDisabledParams {
    const METHOD: &'static str = "Emulation.setScriptExecutionDisabled";
    type Response = crate::EmptyReturns;
}

/// Enables touch on platforms which do not support them.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTouchEmulationEnabledParams {
    /// Whether the touch event emulation should be enabled.

    pub enabled: bool,
    /// Maximum touch points supported. Defaults to one.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxTouchPoints: Option<i64>,
}

impl SetTouchEmulationEnabledParams { pub const METHOD: &'static str = "Emulation.setTouchEmulationEnabled"; }

impl crate::CdpCommand for SetTouchEmulationEnabledParams {
    const METHOD: &'static str = "Emulation.setTouchEmulationEnabled";
    type Response = crate::EmptyReturns;
}

/// Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
/// the current virtual time policy.  Note this supersedes any previous time budget.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVirtualTimePolicyParams {

    pub policy: VirtualTimePolicy,
    /// If set, after this many virtual milliseconds have elapsed virtual time will be paused and a
    /// virtualTimeBudgetExpired event is sent.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<f64>,
    /// If set this specifies the maximum number of tasks that can be run before virtual is forced
    /// forwards to prevent deadlock.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxVirtualTimeTaskStarvationCount: Option<u64>,
    /// If set, base::Time::Now will be overridden to initially return this value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialVirtualTime: Option<crate::network::TimeSinceEpoch>,
}

/// Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
/// the current virtual time policy.  Note this supersedes any previous time budget.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVirtualTimePolicyReturns {
    /// Absolute timestamp at which virtual time was first enabled (up time in milliseconds).

    pub virtualTimeTicksBase: f64,
}

impl SetVirtualTimePolicyParams { pub const METHOD: &'static str = "Emulation.setVirtualTimePolicy"; }

impl crate::CdpCommand for SetVirtualTimePolicyParams {
    const METHOD: &'static str = "Emulation.setVirtualTimePolicy";
    type Response = SetVirtualTimePolicyReturns;
}

/// Overrides default host system locale with the specified one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetLocaleOverrideParams {
    /// ICU style C locale (e.g. "en_US"). If not specified or empty, disables the override and
    /// restores default host system locale.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

impl SetLocaleOverrideParams { pub const METHOD: &'static str = "Emulation.setLocaleOverride"; }

impl crate::CdpCommand for SetLocaleOverrideParams {
    const METHOD: &'static str = "Emulation.setLocaleOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides default host system timezone with the specified one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTimezoneOverrideParams {
    /// The timezone identifier. List of supported timezones:
    /// https://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt
    /// If empty, disables the override and restores default host system timezone.

    pub timezoneId: String,
}

impl SetTimezoneOverrideParams { pub const METHOD: &'static str = "Emulation.setTimezoneOverride"; }

impl crate::CdpCommand for SetTimezoneOverrideParams {
    const METHOD: &'static str = "Emulation.setTimezoneOverride";
    type Response = crate::EmptyReturns;
}

/// Resizes the frame/viewport of the page. Note that this does not affect the frame's container
/// (e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
/// on Android.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVisibleSizeParams {
    /// Frame width (DIP).

    pub width: u64,
    /// Frame height (DIP).

    pub height: i64,
}

impl SetVisibleSizeParams { pub const METHOD: &'static str = "Emulation.setVisibleSize"; }

impl crate::CdpCommand for SetVisibleSizeParams {
    const METHOD: &'static str = "Emulation.setVisibleSize";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDisabledImageTypesParams {
    /// Image types to disable.

    pub imageTypes: Vec<DisabledImageType>,
}

impl SetDisabledImageTypesParams { pub const METHOD: &'static str = "Emulation.setDisabledImageTypes"; }

impl crate::CdpCommand for SetDisabledImageTypesParams {
    const METHOD: &'static str = "Emulation.setDisabledImageTypes";
    type Response = crate::EmptyReturns;
}

/// Override the value of navigator.connection.saveData

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDataSaverOverrideParams {
    /// Override value. Omitting the parameter disables the override.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataSaverEnabled: Option<bool>,
}

impl SetDataSaverOverrideParams { pub const METHOD: &'static str = "Emulation.setDataSaverOverride"; }

impl crate::CdpCommand for SetDataSaverOverrideParams {
    const METHOD: &'static str = "Emulation.setDataSaverOverride";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetHardwareConcurrencyOverrideParams {
    /// Hardware concurrency to report

    pub hardwareConcurrency: i64,
}

impl SetHardwareConcurrencyOverrideParams { pub const METHOD: &'static str = "Emulation.setHardwareConcurrencyOverride"; }

impl crate::CdpCommand for SetHardwareConcurrencyOverrideParams {
    const METHOD: &'static str = "Emulation.setHardwareConcurrencyOverride";
    type Response = crate::EmptyReturns;
}

/// Allows overriding user agent with the given string.
/// 'userAgentMetadata' must be set for Client Hint headers to be sent.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAgentOverrideParams {
    /// User agent to use.

    pub userAgent: String,
    /// Browser language to emulate.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptLanguage: Option<String>,
    /// The platform navigator.platform should return.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userAgentMetadata: Option<UserAgentMetadata>,
}

impl SetUserAgentOverrideParams { pub const METHOD: &'static str = "Emulation.setUserAgentOverride"; }

impl crate::CdpCommand for SetUserAgentOverrideParams {
    const METHOD: &'static str = "Emulation.setUserAgentOverride";
    type Response = crate::EmptyReturns;
}

/// Allows overriding the automation flag.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAutomationOverrideParams {
    /// Whether the override should be enabled.

    pub enabled: bool,
}

impl SetAutomationOverrideParams { pub const METHOD: &'static str = "Emulation.setAutomationOverride"; }

impl crate::CdpCommand for SetAutomationOverrideParams {
    const METHOD: &'static str = "Emulation.setAutomationOverride";
    type Response = crate::EmptyReturns;
}

/// Allows overriding the difference between the small and large viewport sizes, which determine the
/// value of the 'svh' and 'lvh' unit, respectively. Only supported for top-level frames.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSmallViewportHeightDifferenceOverrideParams {
    /// This will cause an element of size 100svh to be 'difference' pixels smaller than an element
    /// of size 100lvh.

    pub difference: i64,
}

impl SetSmallViewportHeightDifferenceOverrideParams { pub const METHOD: &'static str = "Emulation.setSmallViewportHeightDifferenceOverride"; }

impl crate::CdpCommand for SetSmallViewportHeightDifferenceOverrideParams {
    const METHOD: &'static str = "Emulation.setSmallViewportHeightDifferenceOverride";
    type Response = crate::EmptyReturns;
}

/// Returns device's screen configuration. In headful mode, the physical screens configuration is returned,
/// whereas in headless mode, a virtual headless screen configuration is provided instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetScreenInfosReturns {

    pub screenInfos: Vec<ScreenInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetScreenInfosParams {}

impl GetScreenInfosParams { pub const METHOD: &'static str = "Emulation.getScreenInfos"; }

impl crate::CdpCommand for GetScreenInfosParams {
    const METHOD: &'static str = "Emulation.getScreenInfos";
    type Response = GetScreenInfosReturns;
}

/// Add a new screen to the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScreenParams {
    /// Offset of the left edge of the screen in pixels.

    pub left: i64,
    /// Offset of the top edge of the screen in pixels.

    pub top: i64,
    /// The width of the screen in pixels.

    pub width: u64,
    /// The height of the screen in pixels.

    pub height: i64,
    /// Specifies the screen's work area. Default is entire screen.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workAreaInsets: Option<WorkAreaInsets>,
    /// Specifies the screen's device pixel ratio. Default is 1.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicePixelRatio: Option<f64>,
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<i64>,
    /// Specifies the screen's color depth in bits. Default is 24.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorDepth: Option<i64>,
    /// Specifies the descriptive label for the screen. Default is none.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isInternal: Option<bool>,
}

/// Add a new screen to the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScreenReturns {

    pub screenInfo: ScreenInfo,
}

impl AddScreenParams { pub const METHOD: &'static str = "Emulation.addScreen"; }

impl crate::CdpCommand for AddScreenParams {
    const METHOD: &'static str = "Emulation.addScreen";
    type Response = AddScreenReturns;
}

/// Updates specified screen parameters. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScreenParams {
    /// Target screen identifier.

    pub screenId: ScreenId,
    /// Offset of the left edge of the screen in pixels.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i64>,
    /// Offset of the top edge of the screen in pixels.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    /// The width of the screen in pixels.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    /// The height of the screen in pixels.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Specifies the screen's work area.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workAreaInsets: Option<WorkAreaInsets>,
    /// Specifies the screen's device pixel ratio.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicePixelRatio: Option<f64>,
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<i64>,
    /// Specifies the screen's color depth in bits.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorDepth: Option<i64>,
    /// Specifies the descriptive label for the screen.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isInternal: Option<bool>,
}

/// Updates specified screen parameters. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScreenReturns {

    pub screenInfo: ScreenInfo,
}

impl UpdateScreenParams { pub const METHOD: &'static str = "Emulation.updateScreen"; }

impl crate::CdpCommand for UpdateScreenParams {
    const METHOD: &'static str = "Emulation.updateScreen";
    type Response = UpdateScreenReturns;
}

/// Remove screen from the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScreenParams {

    pub screenId: ScreenId,
}

impl RemoveScreenParams { pub const METHOD: &'static str = "Emulation.removeScreen"; }

impl crate::CdpCommand for RemoveScreenParams {
    const METHOD: &'static str = "Emulation.removeScreen";
    type Response = crate::EmptyReturns;
}

/// Set primary screen. Only supported in headless mode.
/// Note that this changes the coordinate system origin to the top-left
/// of the new primary screen, updating the bounds and work areas
/// of all existing screens accordingly.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPrimaryScreenParams {

    pub screenId: ScreenId,
}

impl SetPrimaryScreenParams { pub const METHOD: &'static str = "Emulation.setPrimaryScreen"; }

impl crate::CdpCommand for SetPrimaryScreenParams {
    const METHOD: &'static str = "Emulation.setPrimaryScreen";
    type Response = crate::EmptyReturns;
}
