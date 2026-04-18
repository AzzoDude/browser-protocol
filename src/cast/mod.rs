//! A domain for interacting with Cast, Presentation API, and Remote Playback API
//! functionalities.

use serde::{Serialize, Deserialize};


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

/// Sets a sink to be used when the web page requests the browser to choose a
/// sink via Presentation API, Remote Playback API, or Cast SDK.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSinkToUseParams {

    pub sinkName: String,
}

/// Starts mirroring the desktop to the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartDesktopMirroringParams {

    pub sinkName: String,
}

/// Starts mirroring the tab to the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTabMirroringParams {

    pub sinkName: String,
}

/// Stops the active Cast session on the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopCastingParams {

    pub sinkName: String,
}
