//! This domain provides experimental commands only supported in headless mode.

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Encoding options for a screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotParams {
    /// Image compression format (defaults to png).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Compression quality from range \[0..100\] (jpeg and webp only).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
    /// Optimize image encoding for speed, not for resulting size (defaults to false)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizeForSpeed: Option<bool>,
}

/// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
/// screenshot from the resulting frame. Requires that the target was created with enabled
/// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
/// <https://goo.gle/chrome-headless-rendering> for more background.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BeginFrameParams {
    /// Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
    /// the current time will be used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameTimeTicks: Option<f64>,
    /// The interval between BeginFrames that is reported to the compositor, in milliseconds.
    /// Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
    /// Whether updates should not be committed and drawn onto the display. False by default. If
    /// true, only side effects of the BeginFrame will be run, such as layout and animations, but
    /// any visual updates may not be visible on the display or in screenshots.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub noDisplayUpdates: Option<bool>,
    /// If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
    /// no screenshot will be captured. Note that capturing a screenshot can fail, for example,
    /// during renderer initialization. In such a case, no screenshot data will be returned.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<ScreenshotParams>,
}

/// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
/// screenshot from the resulting frame. Requires that the target was created with enabled
/// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
/// <https://goo.gle/chrome-headless-rendering> for more background.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BeginFrameReturns {
    /// Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the
    /// display. Reported for diagnostic uses, may be removed in the future.

    pub hasDamage: bool,
    /// Base64-encoded image data of the screenshot, if one was requested and successfully taken. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshotData: Option<String>,
}
