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

impl<'a> RemoveInstrumentationBreakpointParams<'a> { pub const METHOD: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for RemoveInstrumentationBreakpointParams<'a> {
    const METHOD: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams {
    pub fn builder() -> DisableParamsBuilder {
        DisableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct DisableParamsBuilder {}

impl DisableParamsBuilder {
    pub fn build(self) -> DisableParams {
        DisableParams {}
    }
}

impl DisableParams { pub const METHOD: &'static str = "EventBreakpoints.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "EventBreakpoints.disable";
    type Response = crate::EmptyReturns;
}
