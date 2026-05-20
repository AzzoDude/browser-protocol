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
    pub fn builder() -> EventListenerBuilder<'a> { EventListenerBuilder::default() }
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

#[derive(Default)]
pub struct EventListenerBuilder<'a> {
    type_: Option<Cow<'a, str>>,
    useCapture: Option<bool>,
    passive: Option<bool>,
    once: Option<bool>,
    scriptId: Option<crate::runtime::ScriptId<'a>>,
    lineNumber: Option<i64>,
    columnNumber: Option<i64>,
    handler: Option<crate::runtime::RemoteObject>,
    originalHandler: Option<crate::runtime::RemoteObject>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> EventListenerBuilder<'a> {
    /// 'EventListener''s type.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    /// 'EventListener''s useCapture.
    pub fn useCapture(mut self, useCapture: bool) -> Self { self.useCapture = Some(useCapture); self }
    /// 'EventListener''s passive flag.
    pub fn passive(mut self, passive: bool) -> Self { self.passive = Some(passive); self }
    /// 'EventListener''s once flag.
    pub fn once(mut self, once: bool) -> Self { self.once = Some(once); self }
    /// Script id of the handler code.
    pub fn scriptId(mut self, scriptId: crate::runtime::ScriptId<'a>) -> Self { self.scriptId = Some(scriptId); self }
    /// Line number in the script (0-based).
    pub fn lineNumber(mut self, lineNumber: i64) -> Self { self.lineNumber = Some(lineNumber); self }
    /// Column number in the script (0-based).
    pub fn columnNumber(mut self, columnNumber: i64) -> Self { self.columnNumber = Some(columnNumber); self }
    /// Event handler function value.
    pub fn handler(mut self, handler: crate::runtime::RemoteObject) -> Self { self.handler = Some(handler); self }
    /// Event original handler function value.
    pub fn originalHandler(mut self, originalHandler: crate::runtime::RemoteObject) -> Self { self.originalHandler = Some(originalHandler); self }
    /// Node the listener is added to (if any).
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    pub fn build(self) -> EventListener<'a> {
        EventListener {
            type_: self.type_.unwrap_or_default(),
            useCapture: self.useCapture.unwrap_or_default(),
            passive: self.passive.unwrap_or_default(),
            once: self.once.unwrap_or_default(),
            scriptId: self.scriptId.unwrap_or_default(),
            lineNumber: self.lineNumber.unwrap_or_default(),
            columnNumber: self.columnNumber.unwrap_or_default(),
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
    pub fn builder() -> GetEventListenersParamsBuilder<'a> { GetEventListenersParamsBuilder::default() }
    pub fn objectId(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.objectId }
    pub fn depth(&self) -> Option<i64> { self.depth }
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}

#[derive(Default)]
pub struct GetEventListenersParamsBuilder<'a> {
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    depth: Option<i64>,
    pierce: Option<bool>,
}

impl<'a> GetEventListenersParamsBuilder<'a> {
    /// Identifier of the object to return listeners for.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    /// The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false). Reports listeners for all contexts if pierce is enabled.
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> GetEventListenersParams<'a> {
        GetEventListenersParams {
            objectId: self.objectId.unwrap_or_default(),
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
    pub fn builder() -> GetEventListenersReturnsBuilder<'a> { GetEventListenersReturnsBuilder::default() }
    pub fn listeners(&self) -> &[EventListener<'a>] { &self.listeners }
}

#[derive(Default)]
pub struct GetEventListenersReturnsBuilder<'a> {
    listeners: Option<Vec<EventListener<'a>>>,
}

impl<'a> GetEventListenersReturnsBuilder<'a> {
    /// Array of relevant listeners.
    pub fn listeners(mut self, listeners: Vec<EventListener<'a>>) -> Self { self.listeners = Some(listeners); self }
    pub fn build(self) -> GetEventListenersReturns<'a> {
        GetEventListenersReturns {
            listeners: self.listeners.unwrap_or_default(),
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
    pub fn builder() -> RemoveDOMBreakpointParamsBuilder { RemoveDOMBreakpointParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
    pub fn type_(&self) -> &DOMBreakpointType { &self.type_ }
}

#[derive(Default)]
pub struct RemoveDOMBreakpointParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
    type_: Option<DOMBreakpointType>,
}

impl RemoveDOMBreakpointParamsBuilder {
    /// Identifier of the node to remove breakpoint from.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Type of the breakpoint to remove.
    pub fn type_(mut self, type_: DOMBreakpointType) -> Self { self.type_ = Some(type_); self }
    pub fn build(self) -> RemoveDOMBreakpointParams {
        RemoveDOMBreakpointParams {
            nodeId: self.nodeId.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
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
    pub fn builder() -> RemoveEventListenerBreakpointParamsBuilder<'a> { RemoveEventListenerBreakpointParamsBuilder::default() }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
    pub fn targetName(&self) -> Option<&str> { self.targetName.as_deref() }
}

#[derive(Default)]
pub struct RemoveEventListenerBreakpointParamsBuilder<'a> {
    eventName: Option<Cow<'a, str>>,
    targetName: Option<Cow<'a, str>>,
}

impl<'a> RemoveEventListenerBreakpointParamsBuilder<'a> {
    /// Event name.
    pub fn eventName(mut self, eventName: impl Into<Cow<'a, str>>) -> Self { self.eventName = Some(eventName.into()); self }
    /// EventTarget interface name.
    pub fn targetName(mut self, targetName: impl Into<Cow<'a, str>>) -> Self { self.targetName = Some(targetName.into()); self }
    pub fn build(self) -> RemoveEventListenerBreakpointParams<'a> {
        RemoveEventListenerBreakpointParams {
            eventName: self.eventName.unwrap_or_default(),
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
    pub fn builder() -> RemoveInstrumentationBreakpointParamsBuilder<'a> { RemoveInstrumentationBreakpointParamsBuilder::default() }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
}

#[derive(Default)]
pub struct RemoveInstrumentationBreakpointParamsBuilder<'a> {
    eventName: Option<Cow<'a, str>>,
}

impl<'a> RemoveInstrumentationBreakpointParamsBuilder<'a> {
    /// Instrumentation name to stop on.
    pub fn eventName(mut self, eventName: impl Into<Cow<'a, str>>) -> Self { self.eventName = Some(eventName.into()); self }
    pub fn build(self) -> RemoveInstrumentationBreakpointParams<'a> {
        RemoveInstrumentationBreakpointParams {
            eventName: self.eventName.unwrap_or_default(),
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
    pub fn builder() -> RemoveXHRBreakpointParamsBuilder<'a> { RemoveXHRBreakpointParamsBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}

#[derive(Default)]
pub struct RemoveXHRBreakpointParamsBuilder<'a> {
    url: Option<Cow<'a, str>>,
}

impl<'a> RemoveXHRBreakpointParamsBuilder<'a> {
    /// Resource URL substring.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> RemoveXHRBreakpointParams<'a> {
        RemoveXHRBreakpointParams {
            url: self.url.unwrap_or_default(),
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
    pub fn builder() -> SetBreakOnCSPViolationParamsBuilder { SetBreakOnCSPViolationParamsBuilder::default() }
    pub fn violationTypes(&self) -> &[CSPViolationType] { &self.violationTypes }
}

#[derive(Default)]
pub struct SetBreakOnCSPViolationParamsBuilder {
    violationTypes: Option<Vec<CSPViolationType>>,
}

impl SetBreakOnCSPViolationParamsBuilder {
    /// CSP Violations to stop upon.
    pub fn violationTypes(mut self, violationTypes: Vec<CSPViolationType>) -> Self { self.violationTypes = Some(violationTypes); self }
    pub fn build(self) -> SetBreakOnCSPViolationParams {
        SetBreakOnCSPViolationParams {
            violationTypes: self.violationTypes.unwrap_or_default(),
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
    pub fn builder() -> SetDOMBreakpointParamsBuilder { SetDOMBreakpointParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
    pub fn type_(&self) -> &DOMBreakpointType { &self.type_ }
}

#[derive(Default)]
pub struct SetDOMBreakpointParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
    type_: Option<DOMBreakpointType>,
}

impl SetDOMBreakpointParamsBuilder {
    /// Identifier of the node to set breakpoint on.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Type of the operation to stop upon.
    pub fn type_(mut self, type_: DOMBreakpointType) -> Self { self.type_ = Some(type_); self }
    pub fn build(self) -> SetDOMBreakpointParams {
        SetDOMBreakpointParams {
            nodeId: self.nodeId.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
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
    pub fn builder() -> SetEventListenerBreakpointParamsBuilder<'a> { SetEventListenerBreakpointParamsBuilder::default() }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
    pub fn targetName(&self) -> Option<&str> { self.targetName.as_deref() }
}

#[derive(Default)]
pub struct SetEventListenerBreakpointParamsBuilder<'a> {
    eventName: Option<Cow<'a, str>>,
    targetName: Option<Cow<'a, str>>,
}

impl<'a> SetEventListenerBreakpointParamsBuilder<'a> {
    /// DOM Event name to stop on (any DOM event will do).
    pub fn eventName(mut self, eventName: impl Into<Cow<'a, str>>) -> Self { self.eventName = Some(eventName.into()); self }
    /// EventTarget interface name to stop on. If equal to '"*"' or not provided, will stop on any
    /// EventTarget.
    pub fn targetName(mut self, targetName: impl Into<Cow<'a, str>>) -> Self { self.targetName = Some(targetName.into()); self }
    pub fn build(self) -> SetEventListenerBreakpointParams<'a> {
        SetEventListenerBreakpointParams {
            eventName: self.eventName.unwrap_or_default(),
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
    pub fn builder() -> SetInstrumentationBreakpointParamsBuilder<'a> { SetInstrumentationBreakpointParamsBuilder::default() }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
}

#[derive(Default)]
pub struct SetInstrumentationBreakpointParamsBuilder<'a> {
    eventName: Option<Cow<'a, str>>,
}

impl<'a> SetInstrumentationBreakpointParamsBuilder<'a> {
    /// Instrumentation name to stop on.
    pub fn eventName(mut self, eventName: impl Into<Cow<'a, str>>) -> Self { self.eventName = Some(eventName.into()); self }
    pub fn build(self) -> SetInstrumentationBreakpointParams<'a> {
        SetInstrumentationBreakpointParams {
            eventName: self.eventName.unwrap_or_default(),
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
    pub fn builder() -> SetXHRBreakpointParamsBuilder<'a> { SetXHRBreakpointParamsBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}

#[derive(Default)]
pub struct SetXHRBreakpointParamsBuilder<'a> {
    url: Option<Cow<'a, str>>,
}

impl<'a> SetXHRBreakpointParamsBuilder<'a> {
    /// Resource URL substring. All XHRs having this substring in the URL will get stopped upon.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> SetXHRBreakpointParams<'a> {
        SetXHRBreakpointParams {
            url: self.url.unwrap_or_default(),
        }
    }
}

impl<'a> SetXHRBreakpointParams<'a> { pub const METHOD: &'static str = "DOMDebugger.setXHRBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for SetXHRBreakpointParams<'a> {
    const METHOD: &'static str = "DOMDebugger.setXHRBreakpoint";
    type Response = crate::EmptyReturns;
}
