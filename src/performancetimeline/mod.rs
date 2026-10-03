//! Reporting of performance timeline events, as specified in
//! <https://w3c.github.io/performance-timeline/#dom-performanceobserver>.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// See <https://github.com/WICG/LargestContentfulPaint> and largest_contentful_paint.idl

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LargestContentfulPaint<'a> {
    #[serde(rename = "renderTime")]
    pub render_time: crate::network::TimeSinceEpoch,
    #[serde(rename = "loadTime")]
    pub load_time: crate::network::TimeSinceEpoch,
    /// The number of pixels being painted.
    pub size: f64,
    /// The id attribute of the element, if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "elementId")]
    pub element_id: Option<Cow<'a, str>>,
    /// The URL of the image (may be trimmed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::BackendNodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LayoutShiftAttribution {
    #[serde(rename = "previousRect")]
    pub previous_rect: crate::dom::Rect,
    #[serde(rename = "currentRect")]
    pub current_rect: crate::dom::Rect,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::BackendNodeId>,
}
/// See <https://wicg.github.io/layout-instability/#sec-layout-shift> and layout_shift.idl

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LayoutShift {
    /// Score increment produced by this event.
    pub value: f64,
    #[serde(rename = "hadRecentInput")]
    pub had_recent_input: bool,
    #[serde(rename = "lastInputTime")]
    pub last_input_time: crate::network::TimeSinceEpoch,
    pub sources: Vec<LayoutShiftAttribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEvent<'a> {
    /// Identifies the frame that this event is related to. Empty for non-frame targets.
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
    /// The event type, as specified in <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>
    /// This determines which of the optional "details" fields is present.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Name may be empty depending on the type.
    pub name: Cow<'a, str>,
    /// Time in seconds since Epoch, monotonically increasing within document lifetime.
    pub time: crate::network::TimeSinceEpoch,
    /// Event duration, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lcpDetails")]
    pub lcp_details: Option<LargestContentfulPaint<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "layoutShiftDetails")]
    pub layout_shift_details: Option<LayoutShift>,
}
/// Previously buffered events would be reported before method returns.
/// See also: timelineEventAdded

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "PerformanceTimeline.enable")]
pub struct EnableParams<'a> {
    /// The types of event to report, as specified in
    /// <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>
    /// The specified filter overrides any previous filters, passing empty
    /// filter disables recording.
    /// Note that not all types exposed to the web platform are currently supported.
    #[serde(rename = "eventTypes")]
    pub event_types: Vec<Cow<'a, str>>,
}
/// Sent when a performance timeline event is added. See reportPerformanceTimeline method.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "PerformanceTimeline.timelineEventAdded")]
pub struct TimelineEventAdded<'a> {
    pub event: TimelineEvent<'a>,
}