//! Reporting of performance timeline events, as specified in
//! <https://w3c.github.io/performance-timeline/#dom-performanceobserver>.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// See <https://github.com/WICG/LargestContentfulPaint> and largest_contentful_paint.idl

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LargestContentfulPaint<'a> {
    #[serde(rename = "renderTime")]
    render_time: crate::network::TimeSinceEpoch,
    #[serde(rename = "loadTime")]
    load_time: crate::network::TimeSinceEpoch,
    /// The number of pixels being painted.
    size: f64,
    /// The id attribute of the element, if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "elementId")]
    element_id: Option<Cow<'a, str>>,
    /// The URL of the image (may be trimmed).
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::BackendNodeId>,
}

impl<'a> LargestContentfulPaint<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `render_time`: 
    /// * `load_time`: 
    /// * `size`: The number of pixels being painted.
    pub fn builder(render_time: crate::network::TimeSinceEpoch, load_time: crate::network::TimeSinceEpoch, size: f64) -> LargestContentfulPaintBuilder<'a> {
        LargestContentfulPaintBuilder {
            render_time: render_time,
            load_time: load_time,
            size: size,
            element_id: None,
            url: None,
            node_id: None,
        }
    }
    pub fn render_time(&self) -> &crate::network::TimeSinceEpoch { &self.render_time }
    pub fn load_time(&self) -> &crate::network::TimeSinceEpoch { &self.load_time }
    /// The number of pixels being painted.
    pub fn size(&self) -> f64 { self.size }
    /// The id attribute of the element, if available.
    pub fn element_id(&self) -> Option<&str> { self.element_id.as_deref() }
    /// The URL of the image (may be trimmed).
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.node_id.as_ref() }
}


pub struct LargestContentfulPaintBuilder<'a> {
    render_time: crate::network::TimeSinceEpoch,
    load_time: crate::network::TimeSinceEpoch,
    size: f64,
    element_id: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    node_id: Option<crate::dom::BackendNodeId>,
}

impl<'a> LargestContentfulPaintBuilder<'a> {
    /// The id attribute of the element, if available.
    pub fn element_id(mut self, element_id: impl Into<Cow<'a, str>>) -> Self { self.element_id = Some(element_id.into()); self }
    /// The URL of the image (may be trimmed).
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn node_id(mut self, node_id: crate::dom::BackendNodeId) -> Self { self.node_id = Some(node_id); self }
    pub fn build(self) -> LargestContentfulPaint<'a> {
        LargestContentfulPaint {
            render_time: self.render_time,
            load_time: self.load_time,
            size: self.size,
            element_id: self.element_id,
            url: self.url,
            node_id: self.node_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutShiftAttribution {
    #[serde(rename = "previousRect")]
    previous_rect: crate::dom::Rect,
    #[serde(rename = "currentRect")]
    current_rect: crate::dom::Rect,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::BackendNodeId>,
}

impl LayoutShiftAttribution {
    /// Creates a builder for this type with the required parameters:
    /// * `previous_rect`: 
    /// * `current_rect`: 
    pub fn builder(previous_rect: crate::dom::Rect, current_rect: crate::dom::Rect) -> LayoutShiftAttributionBuilder {
        LayoutShiftAttributionBuilder {
            previous_rect: previous_rect,
            current_rect: current_rect,
            node_id: None,
        }
    }
    pub fn previous_rect(&self) -> &crate::dom::Rect { &self.previous_rect }
    pub fn current_rect(&self) -> &crate::dom::Rect { &self.current_rect }
    pub fn node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.node_id.as_ref() }
}


pub struct LayoutShiftAttributionBuilder {
    previous_rect: crate::dom::Rect,
    current_rect: crate::dom::Rect,
    node_id: Option<crate::dom::BackendNodeId>,
}

impl LayoutShiftAttributionBuilder {
    pub fn node_id(mut self, node_id: crate::dom::BackendNodeId) -> Self { self.node_id = Some(node_id); self }
    pub fn build(self) -> LayoutShiftAttribution {
        LayoutShiftAttribution {
            previous_rect: self.previous_rect,
            current_rect: self.current_rect,
            node_id: self.node_id,
        }
    }
}

/// See <https://wicg.github.io/layout-instability/#sec-layout-shift> and layout_shift.idl

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutShift {
    /// Score increment produced by this event.
    value: f64,
    #[serde(rename = "hadRecentInput")]
    had_recent_input: bool,
    #[serde(rename = "lastInputTime")]
    last_input_time: crate::network::TimeSinceEpoch,
    sources: Vec<LayoutShiftAttribution>,
}

impl LayoutShift {
    /// Creates a builder for this type with the required parameters:
    /// * `value`: Score increment produced by this event.
    /// * `had_recent_input`: 
    /// * `last_input_time`: 
    /// * `sources`: 
    pub fn builder(value: f64, had_recent_input: bool, last_input_time: crate::network::TimeSinceEpoch, sources: Vec<LayoutShiftAttribution>) -> LayoutShiftBuilder {
        LayoutShiftBuilder {
            value: value,
            had_recent_input: had_recent_input,
            last_input_time: last_input_time,
            sources: sources,
        }
    }
    /// Score increment produced by this event.
    pub fn value(&self) -> f64 { self.value }
    pub fn had_recent_input(&self) -> bool { self.had_recent_input }
    pub fn last_input_time(&self) -> &crate::network::TimeSinceEpoch { &self.last_input_time }
    pub fn sources(&self) -> &[LayoutShiftAttribution] { &self.sources }
}


pub struct LayoutShiftBuilder {
    value: f64,
    had_recent_input: bool,
    last_input_time: crate::network::TimeSinceEpoch,
    sources: Vec<LayoutShiftAttribution>,
}

impl LayoutShiftBuilder {
    pub fn build(self) -> LayoutShift {
        LayoutShift {
            value: self.value,
            had_recent_input: self.had_recent_input,
            last_input_time: self.last_input_time,
            sources: self.sources,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEvent<'a> {
    /// Identifies the frame that this event is related to. Empty for non-frame targets.
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
    /// The event type, as specified in <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "lcpDetails")]
    lcp_details: Option<LargestContentfulPaint<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "layoutShiftDetails")]
    layout_shift_details: Option<LayoutShift>,
}

impl<'a> TimelineEvent<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: Identifies the frame that this event is related to. Empty for non-frame targets.
    /// * `type_`: The event type, as specified in <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype> This determines which of the optional "details" fields is present.
    /// * `name`: Name may be empty depending on the type.
    /// * `time`: Time in seconds since Epoch, monotonically increasing within document lifetime.
    pub fn builder(frame_id: crate::page::FrameId<'a>, type_: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, time: crate::network::TimeSinceEpoch) -> TimelineEventBuilder<'a> {
        TimelineEventBuilder {
            frame_id: frame_id,
            type_: type_.into(),
            name: name.into(),
            time: time,
            duration: None,
            lcp_details: None,
            layout_shift_details: None,
        }
    }
    /// Identifies the frame that this event is related to. Empty for non-frame targets.
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
    /// The event type, as specified in <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>
    /// This determines which of the optional "details" fields is present.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Name may be empty depending on the type.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Time in seconds since Epoch, monotonically increasing within document lifetime.
    pub fn time(&self) -> &crate::network::TimeSinceEpoch { &self.time }
    /// Event duration, if applicable.
    pub fn duration(&self) -> Option<f64> { self.duration }
    pub fn lcp_details(&self) -> Option<&LargestContentfulPaint<'a>> { self.lcp_details.as_ref() }
    pub fn layout_shift_details(&self) -> Option<&LayoutShift> { self.layout_shift_details.as_ref() }
}


pub struct TimelineEventBuilder<'a> {
    frame_id: crate::page::FrameId<'a>,
    type_: Cow<'a, str>,
    name: Cow<'a, str>,
    time: crate::network::TimeSinceEpoch,
    duration: Option<f64>,
    lcp_details: Option<LargestContentfulPaint<'a>>,
    layout_shift_details: Option<LayoutShift>,
}

impl<'a> TimelineEventBuilder<'a> {
    /// Event duration, if applicable.
    pub fn duration(mut self, duration: f64) -> Self { self.duration = Some(duration); self }
    pub fn lcp_details(mut self, lcp_details: LargestContentfulPaint<'a>) -> Self { self.lcp_details = Some(lcp_details); self }
    pub fn layout_shift_details(mut self, layout_shift_details: LayoutShift) -> Self { self.layout_shift_details = Some(layout_shift_details); self }
    pub fn build(self) -> TimelineEvent<'a> {
        TimelineEvent {
            frame_id: self.frame_id,
            type_: self.type_,
            name: self.name,
            time: self.time,
            duration: self.duration,
            lcp_details: self.lcp_details,
            layout_shift_details: self.layout_shift_details,
        }
    }
}

/// Previously buffered events would be reported before method returns.
/// See also: timelineEventAdded

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams<'a> {
    /// The types of event to report, as specified in
    /// <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>
    /// The specified filter overrides any previous filters, passing empty
    /// filter disables recording.
    /// Note that not all types exposed to the web platform are currently supported.
    #[serde(rename = "eventTypes")]
    event_types: Vec<Cow<'a, str>>,
}

impl<'a> EnableParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `event_types`: The types of event to report, as specified in <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype> The specified filter overrides any previous filters, passing empty filter disables recording. Note that not all types exposed to the web platform are currently supported.
    pub fn builder(event_types: Vec<Cow<'a, str>>) -> EnableParamsBuilder<'a> {
        EnableParamsBuilder {
            event_types: event_types,
        }
    }
    /// The types of event to report, as specified in
    /// <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>
    /// The specified filter overrides any previous filters, passing empty
    /// filter disables recording.
    /// Note that not all types exposed to the web platform are currently supported.
    pub fn event_types(&self) -> &[Cow<'a, str>] { &self.event_types }
}


pub struct EnableParamsBuilder<'a> {
    event_types: Vec<Cow<'a, str>>,
}

impl<'a> EnableParamsBuilder<'a> {
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            event_types: self.event_types,
        }
    }
}

impl<'a> EnableParams<'a> { pub const METHOD: &'static str = "PerformanceTimeline.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams<'a> {
    const METHOD: &'static str = "PerformanceTimeline.enable";
    type Response = crate::EmptyReturns;
}
