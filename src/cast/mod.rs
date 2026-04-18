use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! A domain for interacting with Cast, Presentation API, and Remote Playback API
//! functionalities.


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sink {

    pub name: String,

    pub id: String,
    /// Text describing the current session. Present only if there is an active
    /// session on the sink.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

/// Starts observing for sinks that can be used for tab mirroring, and if set,
/// sinks compatible with |presentationUrl| as well. When sinks are found, a
/// |sinksUpdated| event is fired.
/// Also starts observing for issue messages. When an issue is added or removed,
/// an |issueUpdated| event is fired.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentationUrl: Option<String>,
}

impl EnableParams { pub const METHOD: &'static str = "Cast.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "Cast.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Cast.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "Cast.disable";
    type Response = crate::EmptyReturns;
}

/// Sets a sink to be used when the web page requests the browser to choose a
/// sink via Presentation API, Remote Playback API, or Cast SDK.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSinkToUseParams {

    pub sinkName: String,
}

impl SetSinkToUseParams { pub const METHOD: &'static str = "Cast.setSinkToUse"; }

impl crate::CdpCommand for SetSinkToUseParams {
    const METHOD: &'static str = "Cast.setSinkToUse";
    type Response = crate::EmptyReturns;
}

/// Starts mirroring the desktop to the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartDesktopMirroringParams {

    pub sinkName: String,
}

impl StartDesktopMirroringParams { pub const METHOD: &'static str = "Cast.startDesktopMirroring"; }

impl crate::CdpCommand for StartDesktopMirroringParams {
    const METHOD: &'static str = "Cast.startDesktopMirroring";
    type Response = crate::EmptyReturns;
}

/// Starts mirroring the tab to the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTabMirroringParams {

    pub sinkName: String,
}

impl StartTabMirroringParams { pub const METHOD: &'static str = "Cast.startTabMirroring"; }

impl crate::CdpCommand for StartTabMirroringParams {
    const METHOD: &'static str = "Cast.startTabMirroring";
    type Response = crate::EmptyReturns;
}

/// Stops the active Cast session on the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopCastingParams {

    pub sinkName: String,
}

impl StopCastingParams { pub const METHOD: &'static str = "Cast.stopCasting"; }

impl crate::CdpCommand for StopCastingParams {
    const METHOD: &'static str = "Cast.stopCasting";
    type Response = crate::EmptyReturns;
}
