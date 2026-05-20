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
    /// Compression quality from range [0..100] (jpeg and webp only).
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<i64>,
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    #[serde(skip_serializing_if = "Option::is_none")]
    optimizeForSpeed: Option<bool>,
}

impl<'a> ScreenshotParams<'a> {
    pub fn builder() -> ScreenshotParamsBuilder<'a> {
        ScreenshotParamsBuilder {
            format: None,
            quality: None,
            optimizeForSpeed: None,
        }
    }
    pub fn format(&self) -> Option<&str> { self.format.as_deref() }
    pub fn quality(&self) -> Option<i64> { self.quality }
    pub fn optimizeForSpeed(&self) -> Option<bool> { self.optimizeForSpeed }
}

#[derive(Default)]
pub struct ScreenshotParamsBuilder<'a> {
    format: Option<Cow<'a, str>>,
    quality: Option<i64>,
    optimizeForSpeed: Option<bool>,
}

impl<'a> ScreenshotParamsBuilder<'a> {
    /// Image compression format (defaults to png).
    pub fn format(mut self, format: impl Into<Cow<'a, str>>) -> Self { self.format = Some(format.into()); self }
    /// Compression quality from range [0..100] (jpeg and webp only).
    pub fn quality(mut self, quality: i64) -> Self { self.quality = Some(quality); self }
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    pub fn optimizeForSpeed(mut self, optimizeForSpeed: bool) -> Self { self.optimizeForSpeed = Some(optimizeForSpeed); self }
    pub fn build(self) -> ScreenshotParams<'a> {
        ScreenshotParams {
            format: self.format,
            quality: self.quality,
            optimizeForSpeed: self.optimizeForSpeed,
        }
    }
}

/// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
/// screenshot from the resulting frame. Requires that the target was created with enabled
/// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
/// https://goo.gle/chrome-headless-rendering for more background.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BeginFrameParams<'a> {
    /// Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
    /// the current time will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameTimeTicks: Option<f64>,
    /// The interval between BeginFrames that is reported to the compositor, in milliseconds.
    /// Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<f64>,
    /// Whether updates should not be committed and drawn onto the display. False by default. If
    /// true, only side effects of the BeginFrame will be run, such as layout and animations, but
    /// any visual updates may not be visible on the display or in screenshots.
    #[serde(skip_serializing_if = "Option::is_none")]
    noDisplayUpdates: Option<bool>,
    /// If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
    /// no screenshot will be captured. Note that capturing a screenshot can fail, for example,
    /// during renderer initialization. In such a case, no screenshot data will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    screenshot: Option<ScreenshotParams<'a>>,
}

impl<'a> BeginFrameParams<'a> {
    pub fn builder() -> BeginFrameParamsBuilder<'a> {
        BeginFrameParamsBuilder {
            frameTimeTicks: None,
            interval: None,
            noDisplayUpdates: None,
            screenshot: None,
        }
    }
    pub fn frameTimeTicks(&self) -> Option<f64> { self.frameTimeTicks }
    pub fn interval(&self) -> Option<f64> { self.interval }
    pub fn noDisplayUpdates(&self) -> Option<bool> { self.noDisplayUpdates }
    pub fn screenshot(&self) -> Option<&ScreenshotParams<'a>> { self.screenshot.as_ref() }
}

#[derive(Default)]
pub struct BeginFrameParamsBuilder<'a> {
    frameTimeTicks: Option<f64>,
    interval: Option<f64>,
    noDisplayUpdates: Option<bool>,
    screenshot: Option<ScreenshotParams<'a>>,
}

impl<'a> BeginFrameParamsBuilder<'a> {
    /// Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
    /// the current time will be used.
    pub fn frameTimeTicks(mut self, frameTimeTicks: f64) -> Self { self.frameTimeTicks = Some(frameTimeTicks); self }
    /// The interval between BeginFrames that is reported to the compositor, in milliseconds.
    /// Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.
    pub fn interval(mut self, interval: f64) -> Self { self.interval = Some(interval); self }
    /// Whether updates should not be committed and drawn onto the display. False by default. If
    /// true, only side effects of the BeginFrame will be run, such as layout and animations, but
    /// any visual updates may not be visible on the display or in screenshots.
    pub fn noDisplayUpdates(mut self, noDisplayUpdates: bool) -> Self { self.noDisplayUpdates = Some(noDisplayUpdates); self }
    /// If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
    /// no screenshot will be captured. Note that capturing a screenshot can fail, for example,
    /// during renderer initialization. In such a case, no screenshot data will be returned.
    pub fn screenshot(mut self, screenshot: ScreenshotParams<'a>) -> Self { self.screenshot = Some(screenshot); self }
    pub fn build(self) -> BeginFrameParams<'a> {
        BeginFrameParams {
            frameTimeTicks: self.frameTimeTicks,
            interval: self.interval,
            noDisplayUpdates: self.noDisplayUpdates,
            screenshot: self.screenshot,
        }
    }
}

/// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
/// screenshot from the resulting frame. Requires that the target was created with enabled
/// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
/// https://goo.gle/chrome-headless-rendering for more background.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BeginFrameReturns<'a> {
    /// Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the
    /// display. Reported for diagnostic uses, may be removed in the future.
    hasDamage: bool,
    /// Base64-encoded image data of the screenshot, if one was requested and successfully taken. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    screenshotData: Option<Cow<'a, str>>,
}

impl<'a> BeginFrameReturns<'a> {
    pub fn builder(hasDamage: bool) -> BeginFrameReturnsBuilder<'a> {
        BeginFrameReturnsBuilder {
            hasDamage: hasDamage,
            screenshotData: None,
        }
    }
    pub fn hasDamage(&self) -> bool { self.hasDamage }
    pub fn screenshotData(&self) -> Option<&str> { self.screenshotData.as_deref() }
}


pub struct BeginFrameReturnsBuilder<'a> {
    hasDamage: bool,
    screenshotData: Option<Cow<'a, str>>,
}

impl<'a> BeginFrameReturnsBuilder<'a> {
    /// Base64-encoded image data of the screenshot, if one was requested and successfully taken. (Encoded as a base64 string when passed over JSON)
    pub fn screenshotData(mut self, screenshotData: impl Into<Cow<'a, str>>) -> Self { self.screenshotData = Some(screenshotData.into()); self }
    pub fn build(self) -> BeginFrameReturns<'a> {
        BeginFrameReturns {
            hasDamage: self.hasDamage,
            screenshotData: self.screenshotData,
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
