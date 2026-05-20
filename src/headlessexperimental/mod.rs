//! This domain provides experimental commands only supported in headless mode.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Encoding options for a screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotParams<'a> {
    /// Image compression format (defaults to png).
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<Cow<'a, str>>,
    /// Compression quality from range \[0..100\] (jpeg and webp only).
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<i64>,
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    #[serde(skip_serializing_if = "Option::is_none", rename = "optimizeForSpeed")]
    optimize_for_speed: Option<bool>,
}

impl<'a> ScreenshotParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> ScreenshotParamsBuilder<'a> {
        ScreenshotParamsBuilder {
            format: None,
            quality: None,
            optimize_for_speed: None,
        }
    }
    /// Image compression format (defaults to png).
    pub fn format(&self) -> Option<&str> { self.format.as_deref() }
    /// Compression quality from range \[0..100\] (jpeg and webp only).
    pub fn quality(&self) -> Option<i64> { self.quality }
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    pub fn optimize_for_speed(&self) -> Option<bool> { self.optimize_for_speed }
}

#[derive(Default)]
pub struct ScreenshotParamsBuilder<'a> {
    format: Option<Cow<'a, str>>,
    quality: Option<i64>,
    optimize_for_speed: Option<bool>,
}

impl<'a> ScreenshotParamsBuilder<'a> {
    /// Image compression format (defaults to png).
    pub fn format(mut self, format: impl Into<Cow<'a, str>>) -> Self { self.format = Some(format.into()); self }
    /// Compression quality from range \[0..100\] (jpeg and webp only).
    pub fn quality(mut self, quality: i64) -> Self { self.quality = Some(quality); self }
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    pub fn optimize_for_speed(mut self, optimize_for_speed: bool) -> Self { self.optimize_for_speed = Some(optimize_for_speed); self }
    pub fn build(self) -> ScreenshotParams<'a> {
        ScreenshotParams {
            format: self.format,
            quality: self.quality,
            optimize_for_speed: self.optimize_for_speed,
        }
    }
}

/// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
/// screenshot from the resulting frame. Requires that the target was created with enabled
/// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
/// <https://goo.gle/chrome-headless-rendering> for more background.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BeginFrameParams<'a> {
    /// Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
    /// the current time will be used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameTimeTicks")]
    frame_time_ticks: Option<f64>,
    /// The interval between BeginFrames that is reported to the compositor, in milliseconds.
    /// Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<f64>,
    /// Whether updates should not be committed and drawn onto the display. False by default. If
    /// true, only side effects of the BeginFrame will be run, such as layout and animations, but
    /// any visual updates may not be visible on the display or in screenshots.
    #[serde(skip_serializing_if = "Option::is_none", rename = "noDisplayUpdates")]
    no_display_updates: Option<bool>,
    /// If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
    /// no screenshot will be captured. Note that capturing a screenshot can fail, for example,
    /// during renderer initialization. In such a case, no screenshot data will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    screenshot: Option<ScreenshotParams<'a>>,
}

impl<'a> BeginFrameParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> BeginFrameParamsBuilder<'a> {
        BeginFrameParamsBuilder {
            frame_time_ticks: None,
            interval: None,
            no_display_updates: None,
            screenshot: None,
        }
    }
    /// Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
    /// the current time will be used.
    pub fn frame_time_ticks(&self) -> Option<f64> { self.frame_time_ticks }
    /// The interval between BeginFrames that is reported to the compositor, in milliseconds.
    /// Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.
    pub fn interval(&self) -> Option<f64> { self.interval }
    /// Whether updates should not be committed and drawn onto the display. False by default. If
    /// true, only side effects of the BeginFrame will be run, such as layout and animations, but
    /// any visual updates may not be visible on the display or in screenshots.
    pub fn no_display_updates(&self) -> Option<bool> { self.no_display_updates }
    /// If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
    /// no screenshot will be captured. Note that capturing a screenshot can fail, for example,
    /// during renderer initialization. In such a case, no screenshot data will be returned.
    pub fn screenshot(&self) -> Option<&ScreenshotParams<'a>> { self.screenshot.as_ref() }
}

#[derive(Default)]
pub struct BeginFrameParamsBuilder<'a> {
    frame_time_ticks: Option<f64>,
    interval: Option<f64>,
    no_display_updates: Option<bool>,
    screenshot: Option<ScreenshotParams<'a>>,
}

impl<'a> BeginFrameParamsBuilder<'a> {
    /// Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
    /// the current time will be used.
    pub fn frame_time_ticks(mut self, frame_time_ticks: f64) -> Self { self.frame_time_ticks = Some(frame_time_ticks); self }
    /// The interval between BeginFrames that is reported to the compositor, in milliseconds.
    /// Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.
    pub fn interval(mut self, interval: f64) -> Self { self.interval = Some(interval); self }
    /// Whether updates should not be committed and drawn onto the display. False by default. If
    /// true, only side effects of the BeginFrame will be run, such as layout and animations, but
    /// any visual updates may not be visible on the display or in screenshots.
    pub fn no_display_updates(mut self, no_display_updates: bool) -> Self { self.no_display_updates = Some(no_display_updates); self }
    /// If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
    /// no screenshot will be captured. Note that capturing a screenshot can fail, for example,
    /// during renderer initialization. In such a case, no screenshot data will be returned.
    pub fn screenshot(mut self, screenshot: ScreenshotParams<'a>) -> Self { self.screenshot = Some(screenshot); self }
    pub fn build(self) -> BeginFrameParams<'a> {
        BeginFrameParams {
            frame_time_ticks: self.frame_time_ticks,
            interval: self.interval,
            no_display_updates: self.no_display_updates,
            screenshot: self.screenshot,
        }
    }
}

/// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
/// screenshot from the resulting frame. Requires that the target was created with enabled
/// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
/// <https://goo.gle/chrome-headless-rendering> for more background.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BeginFrameReturns<'a> {
    /// Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the
    /// display. Reported for diagnostic uses, may be removed in the future.
    #[serde(rename = "hasDamage")]
    has_damage: bool,
    /// Base64-encoded image data of the screenshot, if one was requested and successfully taken. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenshotData")]
    screenshot_data: Option<Cow<'a, str>>,
}

impl<'a> BeginFrameReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `has_damage`: Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the display. Reported for diagnostic uses, may be removed in the future.
    pub fn builder(has_damage: bool) -> BeginFrameReturnsBuilder<'a> {
        BeginFrameReturnsBuilder {
            has_damage: has_damage,
            screenshot_data: None,
        }
    }
    /// Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the
    /// display. Reported for diagnostic uses, may be removed in the future.
    pub fn has_damage(&self) -> bool { self.has_damage }
    /// Base64-encoded image data of the screenshot, if one was requested and successfully taken. (Encoded as a base64 string when passed over JSON)
    pub fn screenshot_data(&self) -> Option<&str> { self.screenshot_data.as_deref() }
}


pub struct BeginFrameReturnsBuilder<'a> {
    has_damage: bool,
    screenshot_data: Option<Cow<'a, str>>,
}

impl<'a> BeginFrameReturnsBuilder<'a> {
    /// Base64-encoded image data of the screenshot, if one was requested and successfully taken. (Encoded as a base64 string when passed over JSON)
    pub fn screenshot_data(mut self, screenshot_data: impl Into<Cow<'a, str>>) -> Self { self.screenshot_data = Some(screenshot_data.into()); self }
    pub fn build(self) -> BeginFrameReturns<'a> {
        BeginFrameReturns {
            has_damage: self.has_damage,
            screenshot_data: self.screenshot_data,
        }
    }
}

impl<'a> BeginFrameParams<'a> { pub const METHOD: &'static str = "HeadlessExperimental.beginFrame"; }

impl<'a> crate::CdpCommand<'a> for BeginFrameParams<'a> {
    const METHOD: &'static str = "HeadlessExperimental.beginFrame";
    type Response = BeginFrameReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "HeadlessExperimental.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "HeadlessExperimental.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "HeadlessExperimental.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "HeadlessExperimental.enable";
    type Response = crate::EmptyReturns;
}
