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
    useCapture: bool,
    /// 'EventListener''s passive flag.
    passive: bool,
    /// 'EventListener''s once flag.
    once: bool,
    /// Script id of the handler code.
    scriptId: crate::runtime::ScriptId<'a>,
    /// Line number in the script (0-based).
    lineNumber: i64,
    /// Column number in the script (0-based).
    columnNumber: i64,
    /// Event handler function value.
    #[serde(skip_serializing_if = "Option::is_none")]
    handler: Option<crate::runtime::RemoteObject>,
    /// Event original handler function value.
    #[serde(skip_serializing_if = "Option::is_none")]
    originalHandler: Option<crate::runtime::RemoteObject>,
    /// Node the listener is added to (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> EventListener<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>, useCapture: bool, passive: bool, once: bool, scriptId: crate::runtime::ScriptId<'a>, lineNumber: i64, columnNumber: i64) -> EventListenerBuilder<'a> {
        EventListenerBuilder {
            type_: type_.into(),
            useCapture: useCapture,
            passive: passive,
            once: once,
            scriptId: scriptId,
            lineNumber: lineNumber,
            columnNumber: columnNumber,
            handler: None,
            originalHandler: None,
            backendNodeId: None,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn useCapture(&self) -> bool { self.useCapture }
    pub fn passive(&self) -> bool { self.passive }
    pub fn once(&self) -> bool { self.once }
    pub fn scriptId(&self) -> &crate::runtime::ScriptId<'a> { &self.scriptId }
    pub fn lineNumber(&self) -> i64 { self.lineNumber }
    pub fn columnNumber(&self) -> i64 { self.columnNumber }
    pub fn handler(&self) -> Option<&crate::runtime::RemoteObject> { self.handler.as_ref() }
    pub fn originalHandler(&self) -> Option<&crate::runtime::RemoteObject> { self.originalHandler.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
}


pub struct EventListenerBuilder<'a> {
    type_: Cow<'a, str>,
    useCapture: bool,
    passive: bool,
    once: bool,
    scriptId: crate::runtime::ScriptId<'a>,
    lineNumber: i64,
    columnNumber: i64,
    handler: Option<crate::runtime::RemoteObject>,
    originalHandler: Option<crate::runtime::RemoteObject>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> EventListenerBuilder<'a> {
    /// Event handler function value.
    pub fn handler(mut self, handler: crate::runtime::RemoteObject) -> Self { self.handler = Some(handler); self }
    /// Event original handler function value.
    pub fn originalHandler(mut self, originalHandler: crate::runtime::RemoteObject) -> Self { self.originalHandler = Some(originalHandler); self }
    /// Node the listener is added to (if any).
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    pub fn build(self) -> EventListener<'a> {
        EventListener {
            type_: self.type_,
            useCapture: self.useCapture,
            passive: self.passive,
            once: self.once,
            scriptId: self.scriptId,
            lineNumber: self.lineNumber,
            columnNumber: self.columnNumber,
            handler: self.handler,
            originalHandler: self.originalHandler,
            backendNodeId: self.backendNodeId,
        }
    }
}

/// Returns event listeners of the given object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEventListenersParams<'a> {
    /// Identifier of the object to return listeners for.
    objectId: crate::runtime::RemoteObjectId<'a>,
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
    pub fn builder(objectId: crate::runtime::RemoteObjectId<'a>) -> GetEventListenersParamsBuilder<'a> {
        GetEventListenersParamsBuilder {
            objectId: objectId,
            depth: None,
            pierce: None,
        }
    }
    pub fn objectId(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.objectId }
    pub fn depth(&self) -> Option<i64> { self.depth }
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}


pub struct GetEventListenersParamsBuilder<'a> {
    objectId: crate::runtime::RemoteObjectId<'a>,
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
            objectId: self.objectId,
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
    pub fn builder(listeners: Vec<EventListener<'a>>) -> GetEventListenersReturnsBuilder<'a> {
        GetEventListenersReturnsBuilder {
            listeners: listeners,
        }
    }
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
    nodeId: crate::dom::NodeId,
    /// Type of the breakpoint to remove.
    #[serde(rename = "type")]
    type_: DOMBreakpointType,
}

impl RemoveDOMBreakpointParams {
    pub fn builder(nodeId: crate::dom::NodeId, type_: impl Into<DOMBreakpointType>) -> RemoveDOMBreakpointParamsBuilder {
        RemoveDOMBreakpointParamsBuilder {
            nodeId: nodeId,
            type_: type_.into(),
        }
    }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
    pub fn type_(&self) -> &DOMBreakpointType { &self.type_ }
}


pub struct RemoveDOMBreakpointParamsBuilder {
    nodeId: crate::dom::NodeId,
    type_: DOMBreakpointType,
}

impl RemoveDOMBreakpointParamsBuilder {
    pub fn build(self) -> RemoveDOMBreakpointParams {
        RemoveDOMBreakpointParams {
            nodeId: self.nodeId,
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
    eventName: Cow<'a, str>,
    /// EventTarget interface name.
    #[serde(skip_serializing_if = "Option::is_none")]
    targetName: Option<Cow<'a, str>>,
}

impl<'a> RemoveEventListenerBreakpointParams<'a> {
    pub fn builder(eventName: impl Into<Cow<'a, str>>) -> RemoveEventListenerBreakpointParamsBuilder<'a> {
        RemoveEventListenerBreakpointParamsBuilder {
            eventName: eventName.into(),
            targetName: None,
        }
    }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
    pub fn targetName(&self) -> Option<&str> { self.targetName.as_deref() }
}


pub struct RemoveEventListenerBreakpointParamsBuilder<'a> {
    eventName: Cow<'a, str>,
    targetName: Option<Cow<'a, str>>,
}

impl<'a> RemoveEventListenerBreakpointParamsBuilder<'a> {
    /// EventTarget interface name.
    pub fn targetName(mut self, targetName: impl Into<Cow<'a, str>>) -> Self { self.targetName = Some(targetName.into()); self }
    pub fn build(self) -> RemoveEventListenerBreakpointParams<'a> {
        RemoveEventListenerBreakpointParams {
            eventName: self.eventName,
            targetName: self.targetName,
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
    eventName: Cow<'a, str>,
}

impl<'a> RemoveInstrumentationBreakpointParams<'a> {
    pub fn builder(eventName: impl Into<Cow<'a, str>>) -> RemoveInstrumentationBreakpointParamsBuilder<'a> {
        RemoveInstrumentationBreakpointParamsBuilder {
            eventName: eventName.into(),
        }
    }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
}


pub struct RemoveInstrumentationBreakpointParamsBuilder<'a> {
    eventName: Cow<'a, str>,
}

impl<'a> RemoveInstrumentationBreakpointParamsBuilder<'a> {
    pub fn build(self) -> RemoveInstrumentationBreakpointParams<'a> {
        RemoveInstrumentationBreakpointParams {
            eventName: self.eventName,
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
    pub fn builder(url: impl Into<Cow<'a, str>>) -> RemoveXHRBreakpointParamsBuilder<'a> {
        RemoveXHRBreakpointParamsBuilder {
            url: url.into(),
        }
    }
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
    violationTypes: Vec<CSPViolationType>,
}

impl SetBreakOnCSPViolationParams {
    pub fn builder(violationTypes: Vec<CSPViolationType>) -> SetBreakOnCSPViolationParamsBuilder {
        SetBreakOnCSPViolationParamsBuilder {
            violationTypes: violationTypes,
        }
    }
    pub fn violationTypes(&self) -> &[CSPViolationType] { &self.violationTypes }
}


pub struct SetBreakOnCSPViolationParamsBuilder {
    violationTypes: Vec<CSPViolationType>,
}

impl SetBreakOnCSPViolationParamsBuilder {
    pub fn build(self) -> SetBreakOnCSPViolationParams {
        SetBreakOnCSPViolationParams {
            violationTypes: self.violationTypes,
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
    nodeId: crate::dom::NodeId,
    /// Type of the operation to stop upon.
    #[serde(rename = "type")]
    type_: DOMBreakpointType,
}

impl SetDOMBreakpointParams {
    pub fn builder(nodeId: crate::dom::NodeId, type_: impl Into<DOMBreakpointType>) -> SetDOMBreakpointParamsBuilder {
        SetDOMBreakpointParamsBuilder {
            nodeId: nodeId,
            type_: type_.into(),
        }
    }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
    pub fn type_(&self) -> &DOMBreakpointType { &self.type_ }
}


pub struct SetDOMBreakpointParamsBuilder {
    nodeId: crate::dom::NodeId,
    type_: DOMBreakpointType,
}

impl SetDOMBreakpointParamsBuilder {
    pub fn build(self) -> SetDOMBreakpointParams {
        SetDOMBreakpointParams {
            nodeId: self.nodeId,
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
    eventName: Cow<'a, str>,
    /// EventTarget interface name to stop on. If equal to '"*"' or not provided, will stop on any
    /// EventTarget.
    #[serde(skip_serializing_if = "Option::is_none")]
    targetName: Option<Cow<'a, str>>,
}

impl<'a> SetEventListenerBreakpointParams<'a> {
    pub fn builder(eventName: impl Into<Cow<'a, str>>) -> SetEventListenerBreakpointParamsBuilder<'a> {
        SetEventListenerBreakpointParamsBuilder {
            eventName: eventName.into(),
            targetName: None,
        }
    }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
    pub fn targetName(&self) -> Option<&str> { self.targetName.as_deref() }
}


pub struct SetEventListenerBreakpointParamsBuilder<'a> {
    eventName: Cow<'a, str>,
    targetName: Option<Cow<'a, str>>,
}

impl<'a> SetEventListenerBreakpointParamsBuilder<'a> {
    /// EventTarget interface name to stop on. If equal to '"*"' or not provided, will stop on any
    /// EventTarget.
    pub fn targetName(mut self, targetName: impl Into<Cow<'a, str>>) -> Self { self.targetName = Some(targetName.into()); self }
    pub fn build(self) -> SetEventListenerBreakpointParams<'a> {
        SetEventListenerBreakpointParams {
            eventName: self.eventName,
            targetName: self.targetName,
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
    eventName: Cow<'a, str>,
}

impl<'a> SetInstrumentationBreakpointParams<'a> {
    pub fn builder(eventName: impl Into<Cow<'a, str>>) -> SetInstrumentationBreakpointParamsBuilder<'a> {
        SetInstrumentationBreakpointParamsBuilder {
            eventName: eventName.into(),
        }
    }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
}


pub struct SetInstrumentationBreakpointParamsBuilder<'a> {
    eventName: Cow<'a, str>,
}

impl<'a> SetInstrumentationBreakpointParamsBuilder<'a> {
    pub fn build(self) -> SetInstrumentationBreakpointParams<'a> {
        SetInstrumentationBreakpointParams {
            eventName: self.eventName,
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
    pub fn builder(url: impl Into<Cow<'a, str>>) -> SetXHRBreakpointParamsBuilder<'a> {
        SetXHRBreakpointParamsBuilder {
            url: url.into(),
        }
    }
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
