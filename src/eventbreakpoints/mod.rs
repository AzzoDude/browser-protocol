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
