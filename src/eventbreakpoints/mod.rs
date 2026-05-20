//! EventBreakpoints permits setting JavaScript breakpoints on operations and events
//! occurring in native code invoked from JavaScript. Once breakpoint is hit, it is
//! reported through Debugger domain, similarly to regular breakpoints being hit.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

impl<'a> SetInstrumentationBreakpointParams<'a> { pub const METHOD: &'static str = "EventBreakpoints.setInstrumentationBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for SetInstrumentationBreakpointParams<'a> {
    const METHOD: &'static str = "EventBreakpoints.setInstrumentationBreakpoint";
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

impl<'a> RemoveInstrumentationBreakpointParams<'a> { pub const METHOD: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for RemoveInstrumentationBreakpointParams<'a> {
    const METHOD: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "EventBreakpoints.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "EventBreakpoints.disable";
    type Response = crate::EmptyReturns;
}
