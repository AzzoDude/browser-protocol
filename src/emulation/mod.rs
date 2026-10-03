//! This domain emulates different environments for the page.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SafeAreaInsets {
    /// Overrides safe-area-inset-top.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    /// Overrides safe-area-max-inset-top.
    #[serde(skip_serializing_if = "Option::is_none", rename = "topMax")]
    pub top_max: Option<i64>,
    /// Overrides safe-area-inset-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i64>,
    /// Overrides safe-area-max-inset-left.
    #[serde(skip_serializing_if = "Option::is_none", rename = "leftMax")]
    pub left_max: Option<i64>,
    /// Overrides safe-area-inset-bottom.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<i64>,
    /// Overrides safe-area-max-inset-bottom.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bottomMax")]
    pub bottom_max: Option<i64>,
    /// Overrides safe-area-inset-right.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<i64>,
    /// Overrides safe-area-max-inset-right.
    #[serde(skip_serializing_if = "Option::is_none", rename = "rightMax")]
    pub right_max: Option<i64>,
}
/// Screen orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScreenOrientation<'a> {
    /// Orientation type.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Orientation angle.
    pub angle: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DisplayFeature<'a> {
    /// Orientation of a display feature in relation to screen
    pub orientation: Cow<'a, str>,
    /// The offset from the screen origin in either the x (for vertical
    /// orientation) or y (for horizontal orientation) direction.
    pub offset: i32,
    /// A display feature may mask content such that it is not physically
    /// displayed - this length along with the offset describes this area.
    /// A display feature that only splits content will have a 0 mask_length.
    #[serde(rename = "maskLength")]
    pub mask_length: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DevicePosture<'a> {
    /// Current posture of the device
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MediaFeature<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}
/// advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to
/// allow the next delayed task (if any) to run; pause: The virtual time base may not advance;
/// pauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending
/// resource fetches.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum VirtualTimePolicy {
    #[default]
    #[serde(rename = "advance")]
    Advance,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "pauseIfNetworkFetchesPending")]
    PauseIfNetworkFetchesPending,
}

/// Used to specify User Agent Client Hints to emulate. See <https://wicg.github.io/ua-client-hints>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentBrandVersion<'a> {
    pub brand: Cow<'a, str>,
    pub version: Cow<'a, str>,
}
/// Used to specify User Agent Client Hints to emulate. See <https://wicg.github.io/ua-client-hints>
/// Missing optional values will be filled in by the target with what it would normally use.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentMetadata<'a> {
    /// Brands appearing in Sec-CH-UA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands: Option<Vec<UserAgentBrandVersion<'a>>>,
    /// Brands appearing in Sec-CH-UA-Full-Version-List.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fullVersionList")]
    pub full_version_list: Option<Vec<UserAgentBrandVersion<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fullVersion")]
    pub full_version: Option<Cow<'a, str>>,
    pub platform: Cow<'a, str>,
    #[serde(rename = "platformVersion")]
    pub platform_version: Cow<'a, str>,
    pub architecture: Cow<'a, str>,
    pub model: Cow<'a, str>,
    pub mobile: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitness: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wow64: Option<bool>,
    /// Used to specify User Agent form-factor values.
    /// See <https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors>
    #[serde(skip_serializing_if = "Option::is_none", rename = "formFactors")]
    pub form_factors: Option<Vec<Cow<'a, str>>>,
}
/// Used to specify sensor types to emulate.
/// See <https://w3c.github.io/sensors/#automation> for more information.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SensorType {
    #[default]
    #[serde(rename = "absolute-orientation")]
    AbsoluteOrientation,
    #[serde(rename = "accelerometer")]
    Accelerometer,
    #[serde(rename = "ambient-light")]
    AmbientLight,
    #[serde(rename = "gravity")]
    Gravity,
    #[serde(rename = "gyroscope")]
    Gyroscope,
    #[serde(rename = "linear-acceleration")]
    LinearAcceleration,
    #[serde(rename = "magnetometer")]
    Magnetometer,
    #[serde(rename = "relative-orientation")]
    RelativeOrientation,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SensorMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "minimumFrequency")]
    pub minimum_frequency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maximumFrequency")]
    pub maximum_frequency: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingSingle {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingXYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
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
    #[serde(rename = "cpu")]
    Cpu,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PressureState {
    #[default]
    #[serde(rename = "nominal")]
    Nominal,
    #[serde(rename = "fair")]
    Fair,
    #[serde(rename = "serious")]
    Serious,
    #[serde(rename = "critical")]
    Critical,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PressureMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
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

pub type ScreenId<'a> = Cow<'a, str>;

/// Screen information similar to the one returned by window.getScreenDetails() method,
/// see <https://w3c.github.io/window-management/#screendetailed>.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScreenInfo<'a> {
    /// Offset of the left edge of the screen.
    pub left: i64,
    /// Offset of the top edge of the screen.
    pub top: i64,
    /// Width of the screen.
    pub width: u64,
    /// Height of the screen.
    pub height: i64,
    /// Offset of the left edge of the available screen area.
    #[serde(rename = "availLeft")]
    pub avail_left: i64,
    /// Offset of the top edge of the available screen area.
    #[serde(rename = "availTop")]
    pub avail_top: i64,
    /// Width of the available screen area.
    #[serde(rename = "availWidth")]
    pub avail_width: u64,
    /// Height of the available screen area.
    #[serde(rename = "availHeight")]
    pub avail_height: i64,
    /// Specifies the screen's device pixel ratio.
    #[serde(rename = "devicePixelRatio")]
    pub device_pixel_ratio: f64,
    /// Specifies the screen's orientation.
    pub orientation: ScreenOrientation<'a>,
    /// Specifies the screen's color depth in bits.
    #[serde(rename = "colorDepth")]
    pub color_depth: i64,
    /// Indicates whether the device has multiple screens.
    #[serde(rename = "isExtended")]
    pub is_extended: bool,
    /// Indicates whether the screen is internal to the device or external, attached to the device.
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    /// Indicates whether the screen is set as the the operating system primary screen.
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
    /// Specifies the descriptive label for the screen.
    pub label: Cow<'a, str>,
    /// Specifies the unique identifier of the screen.
    pub id: ScreenId<'a>,
}
/// Enum of image types that can be disabled.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DisabledImageType {
    #[default]
    #[serde(rename = "avif")]
    Avif,
    #[serde(rename = "jxl")]
    Jxl,
    #[serde(rename = "webp")]
    Webp,
}

/// Tells whether emulation is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.canEmulate", response = "CanEmulateReturns")]
pub struct CanEmulateParams {

}
/// Tells whether emulation is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulateReturns {
    /// True if emulation is supported.
    pub result: bool,
}
/// Clears the overridden device metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.clearDeviceMetricsOverride")]
pub struct ClearDeviceMetricsOverrideParams {

}
/// Clears the overridden Geolocation Position and Error.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.clearGeolocationOverride")]
pub struct ClearGeolocationOverrideParams {

}
/// Requests that page scale factor is reset to initial values.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.resetPageScaleFactor")]
pub struct ResetPageScaleFactorParams {

}
/// Enables or disables simulating a focused and active page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setFocusEmulationEnabled")]
pub struct SetFocusEmulationEnabledParams {
    /// Whether to enable to disable focus emulation.
    pub enabled: bool,
}
/// Automatically render all web contents using a dark theme.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setAutoDarkModeOverride")]
pub struct SetAutoDarkModeOverrideParams {
    /// Whether to enable or disable automatic dark mode.
    /// If not specified, any existing override will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
/// Enables CPU throttling to emulate slow CPUs.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setCPUThrottlingRate")]
pub struct SetCPUThrottlingRateParams {
    /// Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc).
    pub rate: f64,
}
/// Sets or clears an override of the default background color of the frame. This override is used
/// if the content does not specify one.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setDefaultBackgroundColorOverride")]
pub struct SetDefaultBackgroundColorOverrideParams {
    /// RGBA of the default background color. If not specified, any existing override will be
    /// cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::dom::RGBA>,
}
/// Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the
/// respective variables to be undefined, even if previously overridden.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setSafeAreaInsetsOverride")]
pub struct SetSafeAreaInsetsOverrideParams {
    pub insets: SafeAreaInsets,
}
/// Overrides virtual keyboard geometry in CSS pixels, relative to the top-level viewport. The
/// provided rect is used for navigator.virtualKeyboard.boundingRect, geometrychange events, and
/// env(keyboard-inset-*) values on the inspected frame. The override applies independently of
/// navigator.virtualKeyboard.overlaysContent so clients can preview overlay geometry without
/// mutating page state. Values are rounded to the nearest CSS pixel. Omitting the rect clears the
/// override.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setVirtualKeyboardGeometryOverride")]
pub struct SetVirtualKeyboardGeometryOverrideParams {
    #[serde(skip_serializing_if = "Option::is_none", rename = "keyboardRect")]
    pub keyboard_rect: Option<crate::dom::Rect>,
}
/// Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
/// window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
/// query results).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setDeviceMetricsOverride")]
pub struct SetDeviceMetricsOverrideParams<'a> {
    /// Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub width: u64,
    /// Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub height: i64,
    /// Overriding device scale factor value. 0 disables the override.
    #[serde(rename = "deviceScaleFactor")]
    pub device_scale_factor: f64,
    /// Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    /// autosizing and more.
    pub mobile: bool,
    /// Scale to apply to resulting view image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Overriding screen width value in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenWidth")]
    pub screen_width: Option<u64>,
    /// Overriding screen height value in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenHeight")]
    pub screen_height: Option<i64>,
    /// Overriding view X position on screen in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionX")]
    pub position_x: Option<i64>,
    /// Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionY")]
    pub position_y: Option<i64>,
    /// Do not set visible view size, rely upon explicit setVisibleSize call.
    #[serde(skip_serializing_if = "Option::is_none", rename = "dontSetVisibleSize")]
    pub dont_set_visible_size: Option<bool>,
    /// Screen orientation override.
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenOrientation")]
    pub screen_orientation: Option<ScreenOrientation<'a>>,
    /// If set, the visible area of the page will be overridden to this viewport. This viewport
    /// change is not observed by the page, e.g. viewport-relative elements do not change positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<crate::page::Viewport>,
    /// If set, the display feature of a multi-segment screen. If not set, multi-segment support
    /// is turned-off.
    /// Deprecated, use Emulation.setDisplayFeaturesOverride.
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayFeature")]
    pub display_feature: Option<DisplayFeature<'a>>,
    /// If set, the posture of a foldable device. If not set the posture is set
    /// to continuous.
    /// Deprecated, use Emulation.setDevicePostureOverride.
    #[serde(skip_serializing_if = "Option::is_none", rename = "devicePosture")]
    pub device_posture: Option<DevicePosture<'a>>,
    /// Scrollbar type. Default: 'default'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollbarType")]
    pub scrollbar_type: Option<Cow<'a, str>>,
    /// If set to true, enables screen orientation lock emulation, which
    /// intercepts screen.orientation.lock() calls from the page and reports
    /// orientation changes via screenOrientationLockChanged events. This is
    /// useful for emulating mobile device orientation lock behavior in
    /// responsive design mode.
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenOrientationLockEmulation")]
    pub screen_orientation_lock_emulation: Option<bool>,
    /// Viewport meta tag behavior. Default: 'default'. Note: if 'mobile' is 'true',
    /// the viewport meta tag is always enabled.
    #[serde(skip_serializing_if = "Option::is_none", rename = "viewportMeta")]
    pub viewport_meta: Option<Cow<'a, str>>,
    /// Text layout mode. Default: 'default'. Note: if 'mobile' is 'true',
    /// mobile text layout mode (text autosizing) is always enabled.
    #[serde(skip_serializing_if = "Option::is_none", rename = "textLayoutMode")]
    pub text_layout_mode: Option<Cow<'a, str>>,
}
/// Start reporting the given posture value to the Device Posture API.
/// This override can also be set in setDeviceMetricsOverride().

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setDevicePostureOverride")]
pub struct SetDevicePostureOverrideParams<'a> {
    pub posture: DevicePosture<'a>,
}
/// Clears a device posture override set with either setDeviceMetricsOverride()
/// or setDevicePostureOverride() and starts using posture information from the
/// platform again.
/// Does nothing if no override is set.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.clearDevicePostureOverride")]
pub struct ClearDevicePostureOverrideParams {

}
/// Start using the given display features to pupulate the Viewport Segments API.
/// This override can also be set in setDeviceMetricsOverride().

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setDisplayFeaturesOverride")]
pub struct SetDisplayFeaturesOverrideParams<'a> {
    pub features: Vec<DisplayFeature<'a>>,
}
/// Clears the display features override set with either setDeviceMetricsOverride()
/// or setDisplayFeaturesOverride() and starts using display features from the
/// platform again.
/// Does nothing if no override is set.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.clearDisplayFeaturesOverride")]
pub struct ClearDisplayFeaturesOverrideParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setScrollbarsHidden")]
pub struct SetScrollbarsHiddenParams {
    /// Whether scrollbars should be always hidden.
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setDocumentCookieDisabled")]
pub struct SetDocumentCookieDisabledParams {
    /// Whether document.coookie API should be disabled.
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setEmitTouchEventsForMouse")]
pub struct SetEmitTouchEventsForMouseParams<'a> {
    /// Whether touch emulation based on mouse input should be enabled.
    pub enabled: bool,
    /// Touch/gesture events configuration. Default: current platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Cow<'a, str>>,
}
/// Emulates the given media type or media feature for CSS media queries.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setEmulatedMedia")]
pub struct SetEmulatedMediaParams<'a> {
    /// Media type to emulate. Empty string disables the override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Cow<'a, str>>,
    /// Media features to emulate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<MediaFeature<'a>>>,
}
/// Emulates the given vision deficiency.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setEmulatedVisionDeficiency")]
pub struct SetEmulatedVisionDeficiencyParams<'a> {
    /// Vision deficiency to emulate. Order: best-effort emulations come first, followed by any
    /// physiologically accurate emulations for medically recognized color vision deficiencies.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
}
/// Emulates the given OS text scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setEmulatedOSTextScale")]
pub struct SetEmulatedOSTextScaleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}
/// Overrides the Geolocation Position or Error. Omitting latitude, longitude or
/// accuracy emulates position unavailable.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setGeolocationOverride")]
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "altitudeAccuracy")]
    pub altitude_accuracy: Option<f64>,
    /// Mock heading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<f64>,
    /// Mock speed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.getOverriddenSensorInformation", response = "GetOverriddenSensorInformationReturns")]
pub struct GetOverriddenSensorInformationParams {
    #[serde(rename = "type")]
    pub type_: SensorType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetOverriddenSensorInformationReturns {
    #[serde(rename = "requestedSamplingFrequency")]
    pub requested_sampling_frequency: f64,
}
/// Overrides a platform sensor of a given type. If |enabled| is true, calls to
/// Sensor.start() will use a virtual sensor as backend rather than fetching
/// data from a real hardware sensor. Otherwise, existing virtual
/// sensor-backend Sensor objects will fire an error event and new calls to
/// Sensor.start() will attempt to use a real sensor instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setSensorOverrideEnabled")]
pub struct SetSensorOverrideEnabledParams {
    pub enabled: bool,
    #[serde(rename = "type")]
    pub type_: SensorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SensorMetadata>,
}
/// Updates the sensor readings reported by a sensor type previously overridden
/// by setSensorOverrideEnabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setSensorOverrideReadings")]
pub struct SetSensorOverrideReadingsParams {
    #[serde(rename = "type")]
    pub type_: SensorType,
    pub reading: SensorReading,
}
/// Overrides a pressure source of a given type, as used by the Compute
/// Pressure API, so that updates to PressureObserver.observe() are provided
/// via setPressureStateOverride instead of being retrieved from
/// platform-provided telemetry data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setPressureSourceOverrideEnabled")]
pub struct SetPressureSourceOverrideEnabledParams {
    pub enabled: bool,
    pub source: PressureSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PressureMetadata>,
}
/// Provides a given pressure state that will be processed and eventually be
/// delivered to PressureObserver users. |source| must have been previously
/// overridden by setPressureSourceOverrideEnabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setPressureStateOverride")]
pub struct SetPressureStateOverrideParams {
    pub source: PressureSource,
    pub state: PressureState,
}
/// Overrides the Idle state.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setIdleOverride")]
pub struct SetIdleOverrideParams {
    /// Mock isUserActive
    #[serde(rename = "isUserActive")]
    pub is_user_active: bool,
    /// Mock isScreenUnlocked
    #[serde(rename = "isScreenUnlocked")]
    pub is_screen_unlocked: bool,
}
/// Clears Idle state overrides.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.clearIdleOverride")]
pub struct ClearIdleOverrideParams {

}
/// Overrides value returned by the javascript navigator object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setNavigatorOverrides")]
pub struct SetNavigatorOverridesParams<'a> {
    /// The platform navigator.platform should return.
    pub platform: Cow<'a, str>,
}
/// Sets a specified page scale factor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setPageScaleFactor")]
pub struct SetPageScaleFactorParams {
    /// Page scale factor.
    #[serde(rename = "pageScaleFactor")]
    pub page_scale_factor: f64,
}
/// Switches script execution in the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setScriptExecutionDisabled")]
pub struct SetScriptExecutionDisabledParams {
    /// Whether script execution should be disabled in the page.
    pub value: bool,
}
/// Enables touch on platforms which do not support them.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setTouchEmulationEnabled")]
pub struct SetTouchEmulationEnabledParams {
    /// Whether the touch event emulation should be enabled.
    pub enabled: bool,
    /// Maximum touch points supported. Defaults to one.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxTouchPoints")]
    pub max_touch_points: Option<i64>,
}
/// Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
/// the current virtual time policy.  Note this supersedes any previous time budget.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setVirtualTimePolicy", response = "SetVirtualTimePolicyReturns")]
pub struct SetVirtualTimePolicyParams {
    pub policy: VirtualTimePolicy,
    /// If set, after this many virtual milliseconds have elapsed virtual time will be paused and a
    /// virtualTimeBudgetExpired event is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<f64>,
    /// If set this specifies the maximum number of tasks that can be run before virtual is forced
    /// forwards to prevent deadlock.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxVirtualTimeTaskStarvationCount")]
    pub max_virtual_time_task_starvation_count: Option<u64>,
    /// If set, base::Time::Now will be overridden to initially return this value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "initialVirtualTime")]
    pub initial_virtual_time: Option<crate::network::TimeSinceEpoch>,
}
/// Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
/// the current virtual time policy.  Note this supersedes any previous time budget.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetVirtualTimePolicyReturns {
    /// Absolute timestamp at which virtual time was first enabled (up time in milliseconds).
    #[serde(rename = "virtualTimeTicksBase")]
    pub virtual_time_ticks_base: f64,
}
/// Overrides default host system locale with the specified one.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setLocaleOverride")]
pub struct SetLocaleOverrideParams<'a> {
    /// ICU style C locale (e.g. "en_US"). If not specified or empty, disables the override and
    /// restores default host system locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<Cow<'a, str>>,
}
/// Overrides default host system timezone with the specified one.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setTimezoneOverride")]
pub struct SetTimezoneOverrideParams<'a> {
    /// The timezone identifier. List of supported timezones:
    /// <https://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt>
    /// If empty, disables the override and restores default host system timezone.
    #[serde(rename = "timezoneId")]
    pub timezone_id: Cow<'a, str>,
}
/// Resizes the frame/viewport of the page. Note that this does not affect the frame's container
/// (e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
/// on Android.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setVisibleSize")]
pub struct SetVisibleSizeParams {
    /// Frame width (DIP).
    pub width: u64,
    /// Frame height (DIP).
    pub height: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setDisabledImageTypes")]
pub struct SetDisabledImageTypesParams {
    /// Image types to disable.
    #[serde(rename = "imageTypes")]
    pub image_types: Vec<DisabledImageType>,
}
/// Override the value of navigator.connection.saveData

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setDataSaverOverride")]
pub struct SetDataSaverOverrideParams {
    /// Override value. Omitting the parameter disables the override.
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataSaverEnabled")]
    pub data_saver_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setHardwareConcurrencyOverride")]
pub struct SetHardwareConcurrencyOverrideParams {
    /// Hardware concurrency to report
    #[serde(rename = "hardwareConcurrency")]
    pub hardware_concurrency: i64,
}
/// Overrides the value of navigator.cpuPerformance

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setCPUPerformanceOverride")]
pub struct SetCPUPerformanceOverrideParams<'a> {
    /// Override value. Omitting the parameter disables the override.
    #[serde(skip_serializing_if = "Option::is_none", rename = "performanceTier")]
    pub performance_tier: Option<Cow<'a, str>>,
}
/// Allows overriding user agent with the given string.
/// 'userAgentMetadata' must be set for Client Hint headers to be sent.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setUserAgentOverride")]
pub struct SetUserAgentOverrideParams<'a> {
    /// User agent to use.
    #[serde(rename = "userAgent")]
    pub user_agent: Cow<'a, str>,
    /// Browser language to emulate.
    #[serde(skip_serializing_if = "Option::is_none", rename = "acceptLanguage")]
    pub accept_language: Option<Cow<'a, str>>,
    /// The platform navigator.platform should return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Cow<'a, str>>,
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    #[serde(skip_serializing_if = "Option::is_none", rename = "userAgentMetadata")]
    pub user_agent_metadata: Option<UserAgentMetadata<'a>>,
}
/// Allows overriding the automation flag.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setAutomationOverride")]
pub struct SetAutomationOverrideParams {
    /// Whether the override should be enabled.
    pub enabled: bool,
}
/// Allows overriding the difference between the small and large viewport sizes, which determine the
/// value of the 'svh' and 'lvh' unit, respectively. Only supported for top-level frames.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setSmallViewportHeightDifferenceOverride")]
pub struct SetSmallViewportHeightDifferenceOverrideParams {
    /// This will cause an element of size 100svh to be 'difference' pixels smaller than an element
    /// of size 100lvh.
    pub difference: i64,
}
/// Returns device's screen configuration. In headful mode, the physical screens configuration is returned,
/// whereas in headless mode, a virtual headless screen configuration is provided instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.getScreenInfos", response = "GetScreenInfosReturns<'a>")]
pub struct GetScreenInfosParams {

}
/// Returns device's screen configuration. In headful mode, the physical screens configuration is returned,
/// whereas in headless mode, a virtual headless screen configuration is provided instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetScreenInfosReturns<'a> {
    #[serde(rename = "screenInfos")]
    pub screen_infos: Vec<ScreenInfo<'a>>,
}
/// Add a new screen to the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.addScreen", response = "AddScreenReturns<'a>")]
pub struct AddScreenParams<'a> {
    /// Offset of the left edge of the screen in pixels.
    pub left: i64,
    /// Offset of the top edge of the screen in pixels.
    pub top: i64,
    /// The width of the screen in pixels.
    pub width: u64,
    /// The height of the screen in pixels.
    pub height: i64,
    /// Specifies the screen's work area. Default is entire screen.
    #[serde(skip_serializing_if = "Option::is_none", rename = "workAreaInsets")]
    pub work_area_insets: Option<WorkAreaInsets>,
    /// Specifies the screen's device pixel ratio. Default is 1.
    #[serde(skip_serializing_if = "Option::is_none", rename = "devicePixelRatio")]
    pub device_pixel_ratio: Option<f64>,
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<i64>,
    /// Specifies the screen's color depth in bits. Default is 24.
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorDepth")]
    pub color_depth: Option<i64>,
    /// Specifies the descriptive label for the screen. Default is none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Cow<'a, str>>,
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isInternal")]
    pub is_internal: Option<bool>,
}
/// Add a new screen to the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddScreenReturns<'a> {
    #[serde(rename = "screenInfo")]
    pub screen_info: ScreenInfo<'a>,
}
/// Updates specified screen parameters. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.updateScreen", response = "UpdateScreenReturns<'a>")]
pub struct UpdateScreenParams<'a> {
    /// Target screen identifier.
    #[serde(rename = "screenId")]
    pub screen_id: ScreenId<'a>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "workAreaInsets")]
    pub work_area_insets: Option<WorkAreaInsets>,
    /// Specifies the screen's device pixel ratio.
    #[serde(skip_serializing_if = "Option::is_none", rename = "devicePixelRatio")]
    pub device_pixel_ratio: Option<f64>,
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<i64>,
    /// Specifies the screen's color depth in bits.
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorDepth")]
    pub color_depth: Option<i64>,
    /// Specifies the descriptive label for the screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Cow<'a, str>>,
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isInternal")]
    pub is_internal: Option<bool>,
}
/// Updates specified screen parameters. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScreenReturns<'a> {
    #[serde(rename = "screenInfo")]
    pub screen_info: ScreenInfo<'a>,
}
/// Remove screen from the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.removeScreen")]
pub struct RemoveScreenParams<'a> {
    #[serde(rename = "screenId")]
    pub screen_id: ScreenId<'a>,
}
/// Set primary screen. Only supported in headless mode.
/// Note that this changes the coordinate system origin to the top-left
/// of the new primary screen, updating the bounds and work areas
/// of all existing screens accordingly.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.setPrimaryScreen")]
pub struct SetPrimaryScreenParams<'a> {
    #[serde(rename = "screenId")]
    pub screen_id: ScreenId<'a>,
}
/// Notification sent after the virtual time budget for the current VirtualTimePolicy has run out.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.virtualTimeBudgetExpired")]
pub struct VirtualTimeBudgetExpired {

}
/// Fired when a page calls screen.orientation.lock() or screen.orientation.unlock()
/// while device emulation is enabled. This allows the DevTools frontend to update the
/// emulated device orientation accordingly.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Emulation.screenOrientationLockChanged")]
pub struct ScreenOrientationLockChanged<'a> {
    /// Whether the screen orientation is currently locked.
    pub locked: bool,
    /// The orientation lock type requested by the page. Only set when locked is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<ScreenOrientation<'a>>,
}