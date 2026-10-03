//! This domain provides experimental commands only supported in headless mode.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Encoding options for a screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotParams<'a> {
    /// Image compression format (defaults to png).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Cow<'a, str>>,
    /// Compression quality from range \[0..100\] (jpeg and webp only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    #[serde(skip_serializing_if = "Option::is_none", rename = "optimizeForSpeed")]
    pub optimize_for_speed: Option<bool>,
}
/// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
/// screenshot from the resulting frame. Requires that the target was created with enabled
/// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
/// <https://goo.gle/chrome-headless-rendering> for more background.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeadlessExperimental.beginFrame", response = "BeginFrameReturns<'a>")]
pub struct BeginFrameParams<'a> {
    /// Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
    /// the current time will be used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameTimeTicks")]
    pub frame_time_ticks: Option<f64>,
    /// The interval between BeginFrames that is reported to the compositor, in milliseconds.
    /// Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
    /// Whether updates should not be committed and drawn onto the display. False by default. If
    /// true, only side effects of the BeginFrame will be run, such as layout and animations, but
    /// any visual updates may not be visible on the display or in screenshots.
    #[serde(skip_serializing_if = "Option::is_none", rename = "noDisplayUpdates")]
    pub no_display_updates: Option<bool>,
    /// If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
    /// no screenshot will be captured. Note that capturing a screenshot can fail, for example,
    /// during renderer initialization. In such a case, no screenshot data will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<ScreenshotParams<'a>>,
}
/// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
/// screenshot from the resulting frame. Requires that the target was created with enabled
/// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
/// <https://goo.gle/chrome-headless-rendering> for more background.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BeginFrameReturns<'a> {
    /// Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the
    /// display. Reported for diagnostic uses, may be removed in the future.
    #[serde(rename = "hasDamage")]
    pub has_damage: bool,
    /// Base64-encoded image data of the screenshot, if one was requested and successfully taken. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenshotData")]
    pub screenshot_data: Option<Cow<'a, str>>,
}
/// Disables headless events for the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeadlessExperimental.disable")]
pub struct DisableParams {

}
/// Enables headless events for the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeadlessExperimental.enable")]
pub struct EnableParams {

}