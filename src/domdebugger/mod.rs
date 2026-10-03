//! DOM debugging allows setting breakpoints on particular DOM operations and events. JavaScript
//! execution will stop on these operations as if there was a regular breakpoint set.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// DOM breakpoint type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DOMBreakpointType {
    #[default]
    #[serde(rename = "subtree-modified")]
    SubtreeModified,
    #[serde(rename = "attribute-modified")]
    AttributeModified,
    #[serde(rename = "node-removed")]
    NodeRemoved,
}

/// CSP Violation type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CSPViolationType {
    #[default]
    #[serde(rename = "trustedtype-sink-violation")]
    TrustedtypeSinkViolation,
    #[serde(rename = "trustedtype-policy-violation")]
    TrustedtypePolicyViolation,
}

/// Object event listener.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct EventListener<'a> {
    /// 'EventListener''s type.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// 'EventListener''s useCapture.
    #[serde(rename = "useCapture")]
    pub use_capture: bool,
    /// 'EventListener''s passive flag.
    pub passive: bool,
    /// 'EventListener''s once flag.
    pub once: bool,
    /// Script id of the handler code.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// Line number in the script (0-based).
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    /// Column number in the script (0-based).
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
    /// Event handler function value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<crate::runtime::RemoteObject>,
    /// Event original handler function value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originalHandler")]
    pub original_handler: Option<crate::runtime::RemoteObject>,
    /// Node the listener is added to (if any).
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
}
/// Returns event listeners of the given object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.getEventListeners", response = "GetEventListenersReturns<'a>")]
pub struct GetEventListenersParams<'a> {
    /// Identifier of the object to return listeners for.
    #[serde(rename = "objectId")]
    pub object_id: crate::runtime::RemoteObjectId<'a>,
    /// The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false). Reports listeners for all contexts if pierce is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}
/// Returns event listeners of the given object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetEventListenersReturns<'a> {
    /// Array of relevant listeners.
    pub listeners: Vec<EventListener<'a>>,
}
/// Removes DOM breakpoint that was set using 'setDOMBreakpoint'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.removeDOMBreakpoint")]
pub struct RemoveDOMBreakpointParams {
    /// Identifier of the node to remove breakpoint from.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    /// Type of the breakpoint to remove.
    #[serde(rename = "type")]
    pub type_: DOMBreakpointType,
}
/// Removes breakpoint on particular DOM event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.removeEventListenerBreakpoint")]
pub struct RemoveEventListenerBreakpointParams<'a> {
    /// Event name.
    #[serde(rename = "eventName")]
    pub event_name: Cow<'a, str>,
    /// EventTarget interface name.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetName")]
    pub target_name: Option<Cow<'a, str>>,
}
/// Removes breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.removeInstrumentationBreakpoint")]
pub struct RemoveInstrumentationBreakpointParams<'a> {
    /// Instrumentation name to stop on.
    #[serde(rename = "eventName")]
    pub event_name: Cow<'a, str>,
}
/// Removes breakpoint from XMLHttpRequest.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.removeXHRBreakpoint")]
pub struct RemoveXHRBreakpointParams<'a> {
    /// Resource URL substring.
    pub url: Cow<'a, str>,
}
/// Sets breakpoint on particular CSP violations.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.setBreakOnCSPViolation")]
pub struct SetBreakOnCSPViolationParams {
    /// CSP Violations to stop upon.
    #[serde(rename = "violationTypes")]
    pub violation_types: Vec<CSPViolationType>,
}
/// Sets breakpoint on particular operation with DOM.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.setDOMBreakpoint")]
pub struct SetDOMBreakpointParams {
    /// Identifier of the node to set breakpoint on.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    /// Type of the operation to stop upon.
    #[serde(rename = "type")]
    pub type_: DOMBreakpointType,
}
/// Sets breakpoint on particular DOM event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.setEventListenerBreakpoint")]
pub struct SetEventListenerBreakpointParams<'a> {
    /// DOM Event name to stop on (any DOM event will do).
    #[serde(rename = "eventName")]
    pub event_name: Cow<'a, str>,
    /// EventTarget interface name to stop on. If equal to '"*"' or not provided, will stop on any
    /// EventTarget.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetName")]
    pub target_name: Option<Cow<'a, str>>,
}
/// Sets breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.setInstrumentationBreakpoint")]
pub struct SetInstrumentationBreakpointParams<'a> {
    /// Instrumentation name to stop on.
    #[serde(rename = "eventName")]
    pub event_name: Cow<'a, str>,
}
/// Sets breakpoint on XMLHttpRequest.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMDebugger.setXHRBreakpoint")]
pub struct SetXHRBreakpointParams<'a> {
    /// Resource URL substring. All XHRs having this substring in the URL will get stopped upon.
    pub url: Cow<'a, str>,
}