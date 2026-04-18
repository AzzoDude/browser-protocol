use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! EventBreakpoints permits setting JavaScript breakpoints on operations and events
//! occurring in native code invoked from JavaScript. Once breakpoint is hit, it is
//! reported through Debugger domain, similarly to regular breakpoints being hit.

/// Sets breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointParams {
    /// Instrumentation name to stop on.

    pub eventName: String,
}

impl SetInstrumentationBreakpointParams { pub const METHOD: &'static str = "EventBreakpoints.setInstrumentationBreakpoint"; }

impl crate::CdpCommand for SetInstrumentationBreakpointParams {
    const METHOD: &'static str = "EventBreakpoints.setInstrumentationBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Removes breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveInstrumentationBreakpointParams {
    /// Instrumentation name to stop on.

    pub eventName: String,
}

impl RemoveInstrumentationBreakpointParams { pub const METHOD: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint"; }

impl crate::CdpCommand for RemoveInstrumentationBreakpointParams {
    const METHOD: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "EventBreakpoints.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "EventBreakpoints.disable";
    type Response = crate::EmptyReturns;
}
