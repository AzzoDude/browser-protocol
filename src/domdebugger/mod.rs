//! DOM debugging allows setting breakpoints on particular DOM operations and events. JavaScript
//! execution will stop on these operations as if there was a regular breakpoint set.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventListener<'a> {
    /// 'EventListener''s type.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// 'EventListener''s useCapture.
    #[serde(rename = "useCapture")]
    use_capture: bool,
    /// 'EventListener''s passive flag.
    passive: bool,
    /// 'EventListener''s once flag.
    once: bool,
    /// Script id of the handler code.
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
    /// Line number in the script (0-based).
    #[serde(rename = "lineNumber")]
    line_number: i64,
    /// Column number in the script (0-based).
    #[serde(rename = "columnNumber")]
    column_number: i64,
    /// Event handler function value.
    #[serde(skip_serializing_if = "Option::is_none")]
    handler: Option<crate::runtime::RemoteObject>,
    /// Event original handler function value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originalHandler")]
    original_handler: Option<crate::runtime::RemoteObject>,
    /// Node the listener is added to (if any).
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
}

impl<'a> EventListener<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: `EventListener`'s type.
    /// * `use_capture`: `EventListener`'s useCapture.
    /// * `passive`: `EventListener`'s passive flag.
    /// * `once`: `EventListener`'s once flag.
    /// * `script_id`: Script id of the handler code.
    /// * `line_number`: Line number in the script (0-based).
    /// * `column_number`: Column number in the script (0-based).
    pub fn builder(type_: impl Into<Cow<'a, str>>, use_capture: bool, passive: bool, once: bool, script_id: crate::runtime::ScriptId<'a>, line_number: i64, column_number: i64) -> EventListenerBuilder<'a> {
        EventListenerBuilder {
            type_: type_.into(),
            use_capture: use_capture,
            passive: passive,
            once: once,
            script_id: script_id,
            line_number: line_number,
            column_number: column_number,
            handler: None,
            original_handler: None,
            backend_node_id: None,
        }
    }
    /// 'EventListener''s type.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// 'EventListener''s useCapture.
    pub fn use_capture(&self) -> bool { self.use_capture }
    /// 'EventListener''s passive flag.
    pub fn passive(&self) -> bool { self.passive }
    /// 'EventListener''s once flag.
    pub fn once(&self) -> bool { self.once }
    /// Script id of the handler code.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
    /// Line number in the script (0-based).
    pub fn line_number(&self) -> i64 { self.line_number }
    /// Column number in the script (0-based).
    pub fn column_number(&self) -> i64 { self.column_number }
    /// Event handler function value.
    pub fn handler(&self) -> Option<&crate::runtime::RemoteObject> { self.handler.as_ref() }
    /// Event original handler function value.
    pub fn original_handler(&self) -> Option<&crate::runtime::RemoteObject> { self.original_handler.as_ref() }
    /// Node the listener is added to (if any).
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
}


pub struct EventListenerBuilder<'a> {
    type_: Cow<'a, str>,
    use_capture: bool,
    passive: bool,
    once: bool,
    script_id: crate::runtime::ScriptId<'a>,
    line_number: i64,
    column_number: i64,
    handler: Option<crate::runtime::RemoteObject>,
    original_handler: Option<crate::runtime::RemoteObject>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
}

impl<'a> EventListenerBuilder<'a> {
    /// Event handler function value.
    pub fn handler(mut self, handler: crate::runtime::RemoteObject) -> Self { self.handler = Some(handler); self }
    /// Event original handler function value.
    pub fn original_handler(mut self, original_handler: crate::runtime::RemoteObject) -> Self { self.original_handler = Some(original_handler); self }
    /// Node the listener is added to (if any).
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    pub fn build(self) -> EventListener<'a> {
        EventListener {
            type_: self.type_,
            use_capture: self.use_capture,
            passive: self.passive,
            once: self.once,
            script_id: self.script_id,
            line_number: self.line_number,
            column_number: self.column_number,
            handler: self.handler,
            original_handler: self.original_handler,
            backend_node_id: self.backend_node_id,
        }
    }
}

/// Returns event listeners of the given object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEventListenersParams<'a> {
    /// Identifier of the object to return listeners for.
    #[serde(rename = "objectId")]
    object_id: crate::runtime::RemoteObjectId<'a>,
    /// The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false). Reports listeners for all contexts if pierce is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pierce: Option<bool>,
}

impl<'a> GetEventListenersParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `object_id`: Identifier of the object to return listeners for.
    pub fn builder(object_id: crate::runtime::RemoteObjectId<'a>) -> GetEventListenersParamsBuilder<'a> {
        GetEventListenersParamsBuilder {
            object_id: object_id,
            depth: None,
            pierce: None,
        }
    }
    /// Identifier of the object to return listeners for.
    pub fn object_id(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.object_id }
    /// The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(&self) -> Option<i64> { self.depth }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false). Reports listeners for all contexts if pierce is enabled.
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}


pub struct GetEventListenersParamsBuilder<'a> {
    object_id: crate::runtime::RemoteObjectId<'a>,
    depth: Option<i64>,
    pierce: Option<bool>,
}

impl<'a> GetEventListenersParamsBuilder<'a> {
    /// The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false). Reports listeners for all contexts if pierce is enabled.
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> GetEventListenersParams<'a> {
        GetEventListenersParams {
            object_id: self.object_id,
            depth: self.depth,
            pierce: self.pierce,
        }
    }
}

/// Returns event listeners of the given object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEventListenersReturns<'a> {
    /// Array of relevant listeners.
    listeners: Vec<EventListener<'a>>,
}

impl<'a> GetEventListenersReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `listeners`: Array of relevant listeners.
    pub fn builder(listeners: Vec<EventListener<'a>>) -> GetEventListenersReturnsBuilder<'a> {
        GetEventListenersReturnsBuilder {
            listeners: listeners,
        }
    }
    /// Array of relevant listeners.
    pub fn listeners(&self) -> &[EventListener<'a>] { &self.listeners }
}


pub struct GetEventListenersReturnsBuilder<'a> {
    listeners: Vec<EventListener<'a>>,
}

impl<'a> GetEventListenersReturnsBuilder<'a> {
    pub fn build(self) -> GetEventListenersReturns<'a> {
        GetEventListenersReturns {
            listeners: self.listeners,
        }
    }
}

impl<'a> GetEventListenersParams<'a> { pub const METHOD: &'static str = "DOMDebugger.getEventListeners"; }

impl<'a> crate::CdpCommand<'a> for GetEventListenersParams<'a> {
    const METHOD: &'static str = "DOMDebugger.getEventListeners";
    type Response = GetEventListenersReturns<'a>;
}

/// Removes DOM breakpoint that was set using 'setDOMBreakpoint'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDOMBreakpointParams {
    /// Identifier of the node to remove breakpoint from.
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
    /// Type of the breakpoint to remove.
    #[serde(rename = "type")]
    type_: DOMBreakpointType,
}

impl RemoveDOMBreakpointParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Identifier of the node to remove breakpoint from.
    /// * `type_`: Type of the breakpoint to remove.
    pub fn builder(node_id: crate::dom::NodeId, type_: impl Into<DOMBreakpointType>) -> RemoveDOMBreakpointParamsBuilder {
        RemoveDOMBreakpointParamsBuilder {
            node_id: node_id,
            type_: type_.into(),
        }
    }
    /// Identifier of the node to remove breakpoint from.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
    /// Type of the breakpoint to remove.
    pub fn type_(&self) -> &DOMBreakpointType { &self.type_ }
}


pub struct RemoveDOMBreakpointParamsBuilder {
    node_id: crate::dom::NodeId,
    type_: DOMBreakpointType,
}

impl RemoveDOMBreakpointParamsBuilder {
    pub fn build(self) -> RemoveDOMBreakpointParams {
        RemoveDOMBreakpointParams {
            node_id: self.node_id,
            type_: self.type_,
        }
    }
}

impl RemoveDOMBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.removeDOMBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for RemoveDOMBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.removeDOMBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Removes breakpoint on particular DOM event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEventListenerBreakpointParams<'a> {
    /// Event name.
    #[serde(rename = "eventName")]
    event_name: Cow<'a, str>,
    /// EventTarget interface name.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetName")]
    target_name: Option<Cow<'a, str>>,
}

impl<'a> RemoveEventListenerBreakpointParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `event_name`: Event name.
    pub fn builder(event_name: impl Into<Cow<'a, str>>) -> RemoveEventListenerBreakpointParamsBuilder<'a> {
        RemoveEventListenerBreakpointParamsBuilder {
            event_name: event_name.into(),
            target_name: None,
        }
    }
    /// Event name.
    pub fn event_name(&self) -> &str { self.event_name.as_ref() }
    /// EventTarget interface name.
    pub fn target_name(&self) -> Option<&str> { self.target_name.as_deref() }
}


pub struct RemoveEventListenerBreakpointParamsBuilder<'a> {
    event_name: Cow<'a, str>,
    target_name: Option<Cow<'a, str>>,
}

impl<'a> RemoveEventListenerBreakpointParamsBuilder<'a> {
    /// EventTarget interface name.
    pub fn target_name(mut self, target_name: impl Into<Cow<'a, str>>) -> Self { self.target_name = Some(target_name.into()); self }
    pub fn build(self) -> RemoveEventListenerBreakpointParams<'a> {
        RemoveEventListenerBreakpointParams {
            event_name: self.event_name,
            target_name: self.target_name,
        }
    }
}

impl<'a> RemoveEventListenerBreakpointParams<'a> { pub const METHOD: &'static str = "DOMDebugger.removeEventListenerBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for RemoveEventListenerBreakpointParams<'a> {
    const METHOD: &'static str = "DOMDebugger.removeEventListenerBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Removes breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveInstrumentationBreakpointParams<'a> {
    /// Instrumentation name to stop on.
    #[serde(rename = "eventName")]
    event_name: Cow<'a, str>,
}

impl<'a> RemoveInstrumentationBreakpointParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `event_name`: Instrumentation name to stop on.
    pub fn builder(event_name: impl Into<Cow<'a, str>>) -> RemoveInstrumentationBreakpointParamsBuilder<'a> {
        RemoveInstrumentationBreakpointParamsBuilder {
            event_name: event_name.into(),
        }
    }
    /// Instrumentation name to stop on.
    pub fn event_name(&self) -> &str { self.event_name.as_ref() }
}


pub struct RemoveInstrumentationBreakpointParamsBuilder<'a> {
    event_name: Cow<'a, str>,
}

impl<'a> RemoveInstrumentationBreakpointParamsBuilder<'a> {
    pub fn build(self) -> RemoveInstrumentationBreakpointParams<'a> {
        RemoveInstrumentationBreakpointParams {
            event_name: self.event_name,
        }
    }
}

impl<'a> RemoveInstrumentationBreakpointParams<'a> { pub const METHOD: &'static str = "DOMDebugger.removeInstrumentationBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for RemoveInstrumentationBreakpointParams<'a> {
    const METHOD: &'static str = "DOMDebugger.removeInstrumentationBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Removes breakpoint from XMLHttpRequest.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveXHRBreakpointParams<'a> {
    /// Resource URL substring.
    url: Cow<'a, str>,
}

impl<'a> RemoveXHRBreakpointParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: Resource URL substring.
    pub fn builder(url: impl Into<Cow<'a, str>>) -> RemoveXHRBreakpointParamsBuilder<'a> {
        RemoveXHRBreakpointParamsBuilder {
            url: url.into(),
        }
    }
    /// Resource URL substring.
    pub fn url(&self) -> &str { self.url.as_ref() }
}


pub struct RemoveXHRBreakpointParamsBuilder<'a> {
    url: Cow<'a, str>,
}

impl<'a> RemoveXHRBreakpointParamsBuilder<'a> {
    pub fn build(self) -> RemoveXHRBreakpointParams<'a> {
        RemoveXHRBreakpointParams {
            url: self.url,
        }
    }
}

impl<'a> RemoveXHRBreakpointParams<'a> { pub const METHOD: &'static str = "DOMDebugger.removeXHRBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for RemoveXHRBreakpointParams<'a> {
    const METHOD: &'static str = "DOMDebugger.removeXHRBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on particular CSP violations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakOnCSPViolationParams {
    /// CSP Violations to stop upon.
    #[serde(rename = "violationTypes")]
    violation_types: Vec<CSPViolationType>,
}

impl SetBreakOnCSPViolationParams {
    /// Creates a builder for this type with the required parameters:
    /// * `violation_types`: CSP Violations to stop upon.
    pub fn builder(violation_types: Vec<CSPViolationType>) -> SetBreakOnCSPViolationParamsBuilder {
        SetBreakOnCSPViolationParamsBuilder {
            violation_types: violation_types,
        }
    }
    /// CSP Violations to stop upon.
    pub fn violation_types(&self) -> &[CSPViolationType] { &self.violation_types }
}


pub struct SetBreakOnCSPViolationParamsBuilder {
    violation_types: Vec<CSPViolationType>,
}

impl SetBreakOnCSPViolationParamsBuilder {
    pub fn build(self) -> SetBreakOnCSPViolationParams {
        SetBreakOnCSPViolationParams {
            violation_types: self.violation_types,
        }
    }
}

impl SetBreakOnCSPViolationParams { pub const METHOD: &'static str = "DOMDebugger.setBreakOnCSPViolation"; }

impl<'a> crate::CdpCommand<'a> for SetBreakOnCSPViolationParams {
    const METHOD: &'static str = "DOMDebugger.setBreakOnCSPViolation";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on particular operation with DOM.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDOMBreakpointParams {
    /// Identifier of the node to set breakpoint on.
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
    /// Type of the operation to stop upon.
    #[serde(rename = "type")]
    type_: DOMBreakpointType,
}

impl SetDOMBreakpointParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Identifier of the node to set breakpoint on.
    /// * `type_`: Type of the operation to stop upon.
    pub fn builder(node_id: crate::dom::NodeId, type_: impl Into<DOMBreakpointType>) -> SetDOMBreakpointParamsBuilder {
        SetDOMBreakpointParamsBuilder {
            node_id: node_id,
            type_: type_.into(),
        }
    }
    /// Identifier of the node to set breakpoint on.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
    /// Type of the operation to stop upon.
    pub fn type_(&self) -> &DOMBreakpointType { &self.type_ }
}


pub struct SetDOMBreakpointParamsBuilder {
    node_id: crate::dom::NodeId,
    type_: DOMBreakpointType,
}

impl SetDOMBreakpointParamsBuilder {
    pub fn build(self) -> SetDOMBreakpointParams {
        SetDOMBreakpointParams {
            node_id: self.node_id,
            type_: self.type_,
        }
    }
}

impl SetDOMBreakpointParams { pub const METHOD: &'static str = "DOMDebugger.setDOMBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for SetDOMBreakpointParams {
    const METHOD: &'static str = "DOMDebugger.setDOMBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on particular DOM event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEventListenerBreakpointParams<'a> {
    /// DOM Event name to stop on (any DOM event will do).
    #[serde(rename = "eventName")]
    event_name: Cow<'a, str>,
    /// EventTarget interface name to stop on. If equal to '"*"' or not provided, will stop on any
    /// EventTarget.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetName")]
    target_name: Option<Cow<'a, str>>,
}

impl<'a> SetEventListenerBreakpointParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `event_name`: DOM Event name to stop on (any DOM event will do).
    pub fn builder(event_name: impl Into<Cow<'a, str>>) -> SetEventListenerBreakpointParamsBuilder<'a> {
        SetEventListenerBreakpointParamsBuilder {
            event_name: event_name.into(),
            target_name: None,
        }
    }
    /// DOM Event name to stop on (any DOM event will do).
    pub fn event_name(&self) -> &str { self.event_name.as_ref() }
    /// EventTarget interface name to stop on. If equal to '"*"' or not provided, will stop on any
    /// EventTarget.
    pub fn target_name(&self) -> Option<&str> { self.target_name.as_deref() }
}


pub struct SetEventListenerBreakpointParamsBuilder<'a> {
    event_name: Cow<'a, str>,
    target_name: Option<Cow<'a, str>>,
}

impl<'a> SetEventListenerBreakpointParamsBuilder<'a> {
    /// EventTarget interface name to stop on. If equal to '"*"' or not provided, will stop on any
    /// EventTarget.
    pub fn target_name(mut self, target_name: impl Into<Cow<'a, str>>) -> Self { self.target_name = Some(target_name.into()); self }
    pub fn build(self) -> SetEventListenerBreakpointParams<'a> {
        SetEventListenerBreakpointParams {
            event_name: self.event_name,
            target_name: self.target_name,
        }
    }
}

impl<'a> SetEventListenerBreakpointParams<'a> { pub const METHOD: &'static str = "DOMDebugger.setEventListenerBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for SetEventListenerBreakpointParams<'a> {
    const METHOD: &'static str = "DOMDebugger.setEventListenerBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointParams<'a> {
    /// Instrumentation name to stop on.
    #[serde(rename = "eventName")]
    event_name: Cow<'a, str>,
}

impl<'a> SetInstrumentationBreakpointParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `event_name`: Instrumentation name to stop on.
    pub fn builder(event_name: impl Into<Cow<'a, str>>) -> SetInstrumentationBreakpointParamsBuilder<'a> {
        SetInstrumentationBreakpointParamsBuilder {
            event_name: event_name.into(),
        }
    }
    /// Instrumentation name to stop on.
    pub fn event_name(&self) -> &str { self.event_name.as_ref() }
}


pub struct SetInstrumentationBreakpointParamsBuilder<'a> {
    event_name: Cow<'a, str>,
}

impl<'a> SetInstrumentationBreakpointParamsBuilder<'a> {
    pub fn build(self) -> SetInstrumentationBreakpointParams<'a> {
        SetInstrumentationBreakpointParams {
            event_name: self.event_name,
        }
    }
}

impl<'a> SetInstrumentationBreakpointParams<'a> { pub const METHOD: &'static str = "DOMDebugger.setInstrumentationBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for SetInstrumentationBreakpointParams<'a> {
    const METHOD: &'static str = "DOMDebugger.setInstrumentationBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Sets breakpoint on XMLHttpRequest.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetXHRBreakpointParams<'a> {
    /// Resource URL substring. All XHRs having this substring in the URL will get stopped upon.
    url: Cow<'a, str>,
}

impl<'a> SetXHRBreakpointParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: Resource URL substring. All XHRs having this substring in the URL will get stopped upon.
    pub fn builder(url: impl Into<Cow<'a, str>>) -> SetXHRBreakpointParamsBuilder<'a> {
        SetXHRBreakpointParamsBuilder {
            url: url.into(),
        }
    }
    /// Resource URL substring. All XHRs having this substring in the URL will get stopped upon.
    pub fn url(&self) -> &str { self.url.as_ref() }
}


pub struct SetXHRBreakpointParamsBuilder<'a> {
    url: Cow<'a, str>,
}

impl<'a> SetXHRBreakpointParamsBuilder<'a> {
    pub fn build(self) -> SetXHRBreakpointParams<'a> {
        SetXHRBreakpointParams {
            url: self.url,
        }
    }
}

impl<'a> SetXHRBreakpointParams<'a> { pub const METHOD: &'static str = "DOMDebugger.setXHRBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for SetXHRBreakpointParams<'a> {
    const METHOD: &'static str = "DOMDebugger.setXHRBreakpoint";
    type Response = crate::EmptyReturns;
}
