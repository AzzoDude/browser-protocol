use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! This domain allows detailed inspection of media elements.

/// Players will get an ID that is unique within the agent context.

pub type PlayerId = String;


pub type Timestamp = f64;

/// Have one type per entry in MediaLogRecord::Type
/// Corresponds to kMessage

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMessage {
    /// Keep in sync with MediaLogMessageLevel
    /// We are currently keeping the message level 'error' separate from the
    /// PlayerError type because right now they represent different things,
    /// this one being a DVLOG(ERROR) style log message that gets printed
    /// based on what log level is selected in the UI, and the other is a
    /// representation of a media::PipelineStatus object. Soon however we're
    /// going to be moving away from using PipelineStatus for errors and
    /// introducing a new error type which should hopefully let us integrate
    /// the error log level into the PlayerError type.

    pub level: String,

    pub message: String,
}

/// Corresponds to kMediaPropertyChange

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProperty {

    pub name: String,

    pub value: String,
}

/// Corresponds to kMediaEventTriggered

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerEvent {

    pub timestamp: Timestamp,

    pub value: String,
}

/// Represents logged source line numbers reported in an error.
/// NOTE: file and line are from chromium c++ implementation code, not js.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerErrorSourceLocation {

    pub file: String,

    pub line: i64,
}

/// Corresponds to kMediaError

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerError {

    pub errorType: String,
    /// Code is the numeric enum entry for a specific set of error codes, such
    /// as PipelineStatusCodes in media/base/pipeline_status.h

    pub code: i64,
    /// A trace of where this error was caused / where it passed through.

    pub stack: Vec<PlayerErrorSourceLocation>,
    /// Errors potentially have a root cause error, ie, a DecoderError might be
    /// caused by an WindowsError

    pub cause: Vec<PlayerError>,
    /// Extra data attached to an error, such as an HRESULT, Video Codec, etc.

    pub data: serde_json::Map<String, JsonValue>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Player {

    pub playerId: PlayerId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domNodeId: Option<crate::dom::BackendNodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Media.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "Media.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Media.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "Media.disable";
    type Response = crate::EmptyReturns;
}
