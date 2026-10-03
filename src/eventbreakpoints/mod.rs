//! EventBreakpoints permits setting JavaScript breakpoints on operations and events
//! occurring in native code invoked from JavaScript. Once breakpoint is hit, it is
//! reported through Debugger domain, similarly to regular breakpoints being hit.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Sets breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "EventBreakpoints.setInstrumentationBreakpoint")]
pub struct SetInstrumentationBreakpointParams<'a> {
    /// Instrumentation name to stop on.
    #[serde(rename = "eventName")]
    pub event_name: Cow<'a, str>,
}
/// Removes breakpoint on particular native event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "EventBreakpoints.removeInstrumentationBreakpoint")]
pub struct RemoveInstrumentationBreakpointParams<'a> {
    /// Instrumentation name to stop on.
    #[serde(rename = "eventName")]
    pub event_name: Cow<'a, str>,
}
/// Removes all breakpoints

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "EventBreakpoints.disable")]
pub struct DisableParams {

}