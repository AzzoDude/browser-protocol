//! Reporting of performance timeline events, as specified in
//! https://w3c.github.io/performance-timeline/#dom-performanceobserver.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LargestContentfulPaint<'a> {
    renderTime: crate::network::TimeSinceEpoch,
    loadTime: crate::network::TimeSinceEpoch,
    /// The number of pixels being painted.
    size: f64,
    /// The id attribute of the element, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    elementId: Option<Cow<'a, str>>,
    /// The URL of the image (may be trimmed).
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> LargestContentfulPaint<'a> {
    pub fn builder(renderTime: crate::network::TimeSinceEpoch, loadTime: crate::network::TimeSinceEpoch, size: f64) -> LargestContentfulPaintBuilder<'a> {
        LargestContentfulPaintBuilder {
            renderTime: renderTime,
            loadTime: loadTime,
            size: size,
            elementId: None,
            url: None,
            nodeId: None,
        }
    }
    pub fn renderTime(&self) -> &crate::network::TimeSinceEpoch { &self.renderTime }
    pub fn loadTime(&self) -> &crate::network::TimeSinceEpoch { &self.loadTime }
    pub fn size(&self) -> f64 { self.size }
    pub fn elementId(&self) -> Option<&str> { self.elementId.as_deref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn nodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.nodeId.as_ref() }
}


pub struct LargestContentfulPaintBuilder<'a> {
    renderTime: crate::network::TimeSinceEpoch,
    loadTime: crate::network::TimeSinceEpoch,
    size: f64,
    elementId: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    nodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> LargestContentfulPaintBuilder<'a> {
    /// The id attribute of the element, if available.
    pub fn elementId(mut self, elementId: impl Into<Cow<'a, str>>) -> Self { self.elementId = Some(elementId.into()); self }
    /// The URL of the image (may be trimmed).
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn nodeId(mut self, nodeId: crate::dom::BackendNodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> LargestContentfulPaint<'a> {
        LargestContentfulPaint {
            renderTime: self.renderTime,
            loadTime: self.loadTime,
            size: self.size,
            elementId: self.elementId,
            url: self.url,
            nodeId: self.nodeId,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutShiftAttribution {
    previousRect: crate::dom::Rect,
    currentRect: crate::dom::Rect,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<crate::dom::BackendNodeId>,
}

impl LayoutShiftAttribution {
    pub fn builder(previousRect: crate::dom::Rect, currentRect: crate::dom::Rect) -> LayoutShiftAttributionBuilder {
        LayoutShiftAttributionBuilder {
            previousRect: previousRect,
            currentRect: currentRect,
            nodeId: None,
        }
    }
    pub fn previousRect(&self) -> &crate::dom::Rect { &self.previousRect }
    pub fn currentRect(&self) -> &crate::dom::Rect { &self.currentRect }
    pub fn nodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.nodeId.as_ref() }
}


pub struct LayoutShiftAttributionBuilder {
    previousRect: crate::dom::Rect,
    currentRect: crate::dom::Rect,
    nodeId: Option<crate::dom::BackendNodeId>,
}

impl LayoutShiftAttributionBuilder {
    pub fn nodeId(mut self, nodeId: crate::dom::BackendNodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> LayoutShiftAttribution {
        LayoutShiftAttribution {
            previousRect: self.previousRect,
            currentRect: self.currentRect,
            nodeId: self.nodeId,
        }
    }
}

/// See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutShift {
    /// Score increment produced by this event.
    value: f64,
    hadRecentInput: bool,
    lastInputTime: crate::network::TimeSinceEpoch,
    sources: Vec<LayoutShiftAttribution>,
}

impl LayoutShift {
    pub fn builder(value: f64, hadRecentInput: bool, lastInputTime: crate::network::TimeSinceEpoch, sources: Vec<LayoutShiftAttribution>) -> LayoutShiftBuilder {
        LayoutShiftBuilder {
            value: value,
            hadRecentInput: hadRecentInput,
            lastInputTime: lastInputTime,
            sources: sources,
        }
    }
    pub fn value(&self) -> f64 { self.value }
    pub fn hadRecentInput(&self) -> bool { self.hadRecentInput }
    pub fn lastInputTime(&self) -> &crate::network::TimeSinceEpoch { &self.lastInputTime }
    pub fn sources(&self) -> &[LayoutShiftAttribution] { &self.sources }
}


pub struct LayoutShiftBuilder {
    value: f64,
    hadRecentInput: bool,
    lastInputTime: crate::network::TimeSinceEpoch,
    sources: Vec<LayoutShiftAttribution>,
}

impl LayoutShiftBuilder {
    pub fn build(self) -> LayoutShift {
        LayoutShift {
            value: self.value,
            hadRecentInput: self.hadRecentInput,
            lastInputTime: self.lastInputTime,
            sources: self.sources,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEvent<'a> {
    /// Identifies the frame that this event is related to. Empty for non-frame targets.
    frameId: crate::page::FrameId<'a>,
    /// The event type, as specified in https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype
    /// This determines which of the optional "details" fields is present.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Name may be empty depending on the type.
    name: Cow<'a, str>,
    /// Time in seconds since Epoch, monotonically increasing within document lifetime.
    time: crate::network::TimeSinceEpoch,
    /// Event duration, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lcpDetails: Option<LargestContentfulPaint<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layoutShiftDetails: Option<LayoutShift>,
}

impl<'a> TimelineEvent<'a> {
    pub fn builder(frameId: crate::page::FrameId<'a>, type_: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, time: crate::network::TimeSinceEpoch) -> TimelineEventBuilder<'a> {
        TimelineEventBuilder {
            frameId: frameId,
            type_: type_.into(),
            name: name.into(),
            time: time,
            duration: None,
            lcpDetails: None,
            layoutShiftDetails: None,
        }
    }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn time(&self) -> &crate::network::TimeSinceEpoch { &self.time }
    pub fn duration(&self) -> Option<f64> { self.duration }
    pub fn lcpDetails(&self) -> Option<&LargestContentfulPaint<'a>> { self.lcpDetails.as_ref() }
    pub fn layoutShiftDetails(&self) -> Option<&LayoutShift> { self.layoutShiftDetails.as_ref() }
}


pub struct TimelineEventBuilder<'a> {
    frameId: crate::page::FrameId<'a>,
    type_: Cow<'a, str>,
    name: Cow<'a, str>,
    time: crate::network::TimeSinceEpoch,
    duration: Option<f64>,
    lcpDetails: Option<LargestContentfulPaint<'a>>,
    layoutShiftDetails: Option<LayoutShift>,
}

impl<'a> TimelineEventBuilder<'a> {
    /// Event duration, if applicable.
    pub fn duration(mut self, duration: f64) -> Self { self.duration = Some(duration); self }
    pub fn lcpDetails(mut self, lcpDetails: LargestContentfulPaint<'a>) -> Self { self.lcpDetails = Some(lcpDetails); self }
    pub fn layoutShiftDetails(mut self, layoutShiftDetails: LayoutShift) -> Self { self.layoutShiftDetails = Some(layoutShiftDetails); self }
    pub fn build(self) -> TimelineEvent<'a> {
        TimelineEvent {
            frameId: self.frameId,
            type_: self.type_,
            name: self.name,
            time: self.time,
            duration: self.duration,
            lcpDetails: self.lcpDetails,
            layoutShiftDetails: self.layoutShiftDetails,
        }
    }
}

/// Previously buffered events would be reported before method returns.
/// See also: timelineEventAdded

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams<'a> {
    /// The types of event to report, as specified in
    /// https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype
    /// The specified filter overrides any previous filters, passing empty
    /// filter disables recording.
    /// Note that not all types exposed to the web platform are currently supported.
    eventTypes: Vec<Cow<'a, str>>,
}

impl<'a> EnableParams<'a> {
    pub fn builder(eventTypes: Vec<Cow<'a, str>>) -> EnableParamsBuilder<'a> {
        EnableParamsBuilder {
            eventTypes: eventTypes,
        }
    }
    pub fn eventTypes(&self) -> &[Cow<'a, str>] { &self.eventTypes }
}


pub struct EnableParamsBuilder<'a> {
    eventTypes: Vec<Cow<'a, str>>,
}

impl<'a> EnableParamsBuilder<'a> {
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            eventTypes: self.eventTypes,
        }
    }
}

impl<'a> EnableParams<'a> { pub const METHOD: &'static str = "PerformanceTimeline.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams<'a> {
    const METHOD: &'static str = "PerformanceTimeline.enable";
    type Response = crate::EmptyReturns;
}
