//! Reporting of performance timeline events, as specified in
//! <https://w3c.github.io/performance-timeline/#dom-performanceobserver.>

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// See <https://github.com/WICG/LargestContentfulPaint> and largest_contentful_paint.idl

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LargestContentfulPaint {

    pub renderTime: crate::network::TimeSinceEpoch,

    pub loadTime: crate::network::TimeSinceEpoch,
    /// The number of pixels being painted.

    pub size: f64,
    /// The id attribute of the element, if available.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub elementId: Option<String>,
    /// The URL of the image (may be trimmed).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::BackendNodeId>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutShiftAttribution {

    pub previousRect: crate::dom::Rect,

    pub currentRect: crate::dom::Rect,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::BackendNodeId>,
}

/// See <https://wicg.github.io/layout-instability/#sec-layout-shift> and layout_shift.idl

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutShift {
    /// Score increment produced by this event.

    pub value: f64,

    pub hadRecentInput: bool,

    pub lastInputTime: crate::network::TimeSinceEpoch,

    pub sources: Vec<LayoutShiftAttribution>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEvent {
    /// Identifies the frame that this event is related to. Empty for non-frame targets.

    pub frameId: crate::page::FrameId,
    /// The event type, as specified in <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>
    /// This determines which of the optional "details" fields is present.

    #[serde(rename = "type")]
    pub type_: String,
    /// Name may be empty depending on the type.

    pub name: String,
    /// Time in seconds since Epoch, monotonically increasing within document lifetime.

    pub time: crate::network::TimeSinceEpoch,
    /// Event duration, if applicable.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcpDetails: Option<LargestContentfulPaint>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layoutShiftDetails: Option<LayoutShift>,
}

/// Previously buffered events would be reported before method returns.
/// See also: timelineEventAdded

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// The types of event to report, as specified in
    /// <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>
    /// The specified filter overrides any previous filters, passing empty
    /// filter disables recording.
    /// Note that not all types exposed to the web platform are currently supported.

    pub eventTypes: Vec<String>,
}
