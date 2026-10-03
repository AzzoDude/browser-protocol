//! This domain allows detailed inspection of media elements.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Players will get an ID that is unique within the agent context.

pub type PlayerId<'a> = Cow<'a, str>;


pub type Timestamp = f64;

/// Have one type per entry in MediaLogRecord::Type
/// Corresponds to kMessage

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMessage<'a> {
    /// Keep in sync with MediaLogMessageLevel
    /// We are currently keeping the message level 'error' separate from the
    /// PlayerError type because right now they represent different things,
    /// this one being a DVLOG(ERROR) style log message that gets printed
    /// based on what log level is selected in the UI, and the other is a
    /// representation of a media::PipelineStatus object. Soon however we're
    /// going to be moving away from using PipelineStatus for errors and
    /// introducing a new error type which should hopefully let us integrate
    /// the error log level into the PlayerError type.
    pub level: Cow<'a, str>,
    pub message: Cow<'a, str>,
}
/// Corresponds to kMediaPropertyChange

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProperty<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}
/// Corresponds to kMediaEventTriggered

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PlayerEvent<'a> {
    pub timestamp: Timestamp,
    pub value: Cow<'a, str>,
}
/// Represents logged source line numbers reported in an error.
/// NOTE: file and line are from chromium c++ implementation code, not js.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PlayerErrorSourceLocation<'a> {
    pub file: Cow<'a, str>,
    pub line: i64,
}
/// Corresponds to kMediaError

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PlayerError<'a> {
    #[serde(rename = "errorType")]
    pub error_type: Cow<'a, str>,
    /// Code is the numeric enum entry for a specific set of error codes, such
    /// as PipelineStatusCodes in media/base/pipeline_status.h
    pub code: i64,
    /// A trace of where this error was caused / where it passed through.
    pub stack: Vec<PlayerErrorSourceLocation<'a>>,
    /// Errors potentially have a root cause error, ie, a DecoderError might be
    /// caused by an WindowsError
    pub cause: Vec<Box<PlayerError<'a>>>,
    /// Extra data attached to an error, such as an HRESULT, Video Codec, etc.
    pub data: serde_json::Map<String, JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Player<'a> {
    #[serde(rename = "playerId")]
    pub player_id: PlayerId<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "domNodeId")]
    pub dom_node_id: Option<crate::dom::BackendNodeId>,
}
/// Enables the Media domain

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Media.enable")]
pub struct EnableParams {

}
/// Disables the Media domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Media.disable")]
pub struct DisableParams {

}
/// This can be called multiple times, and can be used to set / override /
/// remove player properties. A null propValue indicates removal.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Media.playerPropertiesChanged")]
pub struct PlayerPropertiesChanged<'a> {
    #[serde(rename = "playerId")]
    pub player_id: PlayerId<'a>,
    pub properties: Vec<PlayerProperty<'a>>,
}
/// Send events as a list, allowing them to be batched on the browser for less
/// congestion. If batched, events must ALWAYS be in chronological order.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Media.playerEventsAdded")]
pub struct PlayerEventsAdded<'a> {
    #[serde(rename = "playerId")]
    pub player_id: PlayerId<'a>,
    pub events: Vec<PlayerEvent<'a>>,
}
/// Send a list of any messages that need to be delivered.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Media.playerMessagesLogged")]
pub struct PlayerMessagesLogged<'a> {
    #[serde(rename = "playerId")]
    pub player_id: PlayerId<'a>,
    pub messages: Vec<PlayerMessage<'a>>,
}
/// Send a list of any errors that need to be delivered.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Media.playerErrorsRaised")]
pub struct PlayerErrorsRaised<'a> {
    #[serde(rename = "playerId")]
    pub player_id: PlayerId<'a>,
    pub errors: Vec<PlayerError<'a>>,
}
/// Called whenever a player is created, or when a new agent joins and receives
/// a list of active players. If an agent is restored, it will receive one
/// event for each active player.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Media.playerCreated")]
pub struct PlayerCreated<'a> {
    pub player: Player<'a>,
}