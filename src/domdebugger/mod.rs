use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! DOM debugging allows setting breakpoints on particular DOM operations and events. JavaScript
//! execution will stop on these operations as if there was a regular breakpoint set.

/// DOM breakpoint type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DOMBreakpointType {
    #[default]
    SubtreeModified,
    AttributeModified,
    NodeRemoved,
}

/// CSP Violation type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CSPViolationType {
    #[default]
    TrustedtypeSinkViolation,
    TrustedtypePolicyViolation,
}

/// Object event listener.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventListener {
    /// 'EventListener''s type.

    #[serde(rename = "type")]
    pub type_: String,
    /// 'EventListener''s useCapture.

    pub useCapture: bool,
    /// 'EventListener''s passive flag.

    pub passive: bool,
    /// 'EventListener''s once flag.

    pub once: bool,
    /// Script id of the handler code.

    pub scriptId: crate::runtime::ScriptId,
    /// Line number in the script (0-based).

    pub lineNumber: i64,
    /// Column number in the script (0-based).

    pub columnNumber: i64,
    /// Event handler function value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<crate::runtime::RemoteObject>,
    /// Event original handler function value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub originalHandler: Option<crate::runtime::RemoteObject>,
    /// Node the listener is added to (if any).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
}

/// Returns event listeners of the given object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEventListenersParams {
    /// Identifier of the object to return listeners for.

    pub objectId: crate::runtime::RemoteObjectId,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEventListenersReturns {
    /// Array of relevant listeners.

    pub listeners: Vec<EventListener>,
}

impl GetEventListenersParams { pub const METHOD: &'static str = "DOMDebugger.getEventListeners"; }

impl crate::CdpCommand for GetEventListenersParams {
    const METHOD: &'static str = "DOMDebugger.getEventListeners";
    type Response = GetEventListenersReturns;
}

/// Removes DOM breakpoint that was set using 'setDOMBreakpoint'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDOMBreakpointParams {
    /// Identifier of the node to remove breakpoint from.

    pub nodeId: crate::dom::NodeId,
    /// Type of the breakpoint to remove.

    #[serde(rename = "type")]
    pub type_: DOMBreakpointType,
}

impl RemoveDOMBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.removeDOMBreakpoint"; }

impl crate::CdpCommand for RemoveDOMBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.removeDOMBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Removes breakpoint on particular DOM event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEventListenerBreakpointParams {
    /// Event name.

    pub eventName: String,
    /// EventTarget interface name.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetName: Option<String>,
}

impl RemoveEventListenerBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.removeEventListenerBreakpoint"; }

impl crate::CdpCommand for RemoveEventListenerBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.removeEventListenerBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Removes breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveInstrumentationBreakpointParams {
    /// Instrumentation name to stop on.

    pub eventName: String,
}

impl RemoveInstrumentationBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.removeInstrumentationBreakpoint"; }

impl crate::CdpCommand for RemoveInstrumentationBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.removeInstrumentationBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Removes breakpoint from XMLHttpRequest.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveXHRBreakpointParams {
    /// Resource URL substring.

    pub url: String,
}

impl RemoveXHRBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.removeXHRBreakpoint"; }

impl crate::CdpCommand for RemoveXHRBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.removeXHRBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on particular CSP violations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakOnCSPViolationParams {
    /// CSP Violations to stop upon.

    pub violationTypes: Vec<CSPViolationType>,
}

impl SetBreakOnCSPViolationParams { pub const METHOD: &'static str = "DOMDebugger.setBreakOnCSPViolation"; }

impl crate::CdpCommand for SetBreakOnCSPViolationParams {
    const METHOD: &'static str = "DOMDebugger.setBreakOnCSPViolation";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on particular operation with DOM.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDOMBreakpointParams {
    /// Identifier of the node to set breakpoint on.

    pub nodeId: crate::dom::NodeId,
    /// Type of the operation to stop upon.

    #[serde(rename = "type")]
    pub type_: DOMBreakpointType,
}

impl SetDOMBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.setDOMBreakpoint"; }

impl crate::CdpCommand for SetDOMBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.setDOMBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on particular DOM event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEventListenerBreakpointParams {
    /// DOM Event name to stop on (any DOM event will do).

    pub eventName: String,
    /// EventTarget interface name to stop on. If equal to '"*"' or not provided, will stop on any
    /// EventTarget.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetName: Option<String>,
}

impl SetEventListenerBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.setEventListenerBreakpoint"; }

impl crate::CdpCommand for SetEventListenerBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.setEventListenerBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointParams {
    /// Instrumentation name to stop on.

    pub eventName: String,
}

impl SetInstrumentationBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.setInstrumentationBreakpoint"; }

impl crate::CdpCommand for SetInstrumentationBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.setInstrumentationBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on XMLHttpRequest.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetXHRBreakpointParams {
    /// Resource URL substring. All XHRs having this substring in the URL will get stopped upon.

    pub url: String,
}

impl SetXHRBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.setXHRBreakpoint"; }

impl crate::CdpCommand for SetXHRBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.setXHRBreakpoint";
    type Response = crate::EmptyReturns;
}
