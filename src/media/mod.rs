//! This domain allows detailed inspection of media elements.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Players will get an ID that is unique within the agent context.

pub type PlayerId<'a> = Cow<'a, str>;


pub type Timestamp = f64;

/// Have one type per entry in MediaLogRecord::Type
/// Corresponds to kMessage

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    level: Cow<'a, str>,
    message: Cow<'a, str>,
}

impl<'a> PlayerMessage<'a> {
    pub fn builder(level: impl Into<Cow<'a, str>>, message: impl Into<Cow<'a, str>>) -> PlayerMessageBuilder<'a> {
        PlayerMessageBuilder {
            level: level.into(),
            message: message.into(),
        }
    }
    pub fn level(&self) -> &str { self.level.as_ref() }
    pub fn message(&self) -> &str { self.message.as_ref() }
}


pub struct PlayerMessageBuilder<'a> {
    level: Cow<'a, str>,
    message: Cow<'a, str>,
}

impl<'a> PlayerMessageBuilder<'a> {
    pub fn build(self) -> PlayerMessage<'a> {
        PlayerMessage {
            level: self.level,
            message: self.message,
        }
    }
}

/// Corresponds to kMediaPropertyChange

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProperty<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> PlayerProperty<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> PlayerPropertyBuilder<'a> {
        PlayerPropertyBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct PlayerPropertyBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> PlayerPropertyBuilder<'a> {
    pub fn build(self) -> PlayerProperty<'a> {
        PlayerProperty {
            name: self.name,
            value: self.value,
        }
    }
}

/// Corresponds to kMediaEventTriggered

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerEvent<'a> {
    timestamp: Timestamp,
    value: Cow<'a, str>,
}

impl<'a> PlayerEvent<'a> {
    pub fn builder(timestamp: Timestamp, value: impl Into<Cow<'a, str>>) -> PlayerEventBuilder<'a> {
        PlayerEventBuilder {
            timestamp: timestamp,
            value: value.into(),
        }
    }
    pub fn timestamp(&self) -> &Timestamp { &self.timestamp }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct PlayerEventBuilder<'a> {
    timestamp: Timestamp,
    value: Cow<'a, str>,
}

impl<'a> PlayerEventBuilder<'a> {
    pub fn build(self) -> PlayerEvent<'a> {
        PlayerEvent {
            timestamp: self.timestamp,
            value: self.value,
        }
    }
}

/// Represents logged source line numbers reported in an error.
/// NOTE: file and line are from chromium c++ implementation code, not js.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerErrorSourceLocation<'a> {
    file: Cow<'a, str>,
    line: i64,
}

impl<'a> PlayerErrorSourceLocation<'a> {
    pub fn builder(file: impl Into<Cow<'a, str>>, line: i64) -> PlayerErrorSourceLocationBuilder<'a> {
        PlayerErrorSourceLocationBuilder {
            file: file.into(),
            line: line,
        }
    }
    pub fn file(&self) -> &str { self.file.as_ref() }
    pub fn line(&self) -> i64 { self.line }
}


pub struct PlayerErrorSourceLocationBuilder<'a> {
    file: Cow<'a, str>,
    line: i64,
}

impl<'a> PlayerErrorSourceLocationBuilder<'a> {
    pub fn build(self) -> PlayerErrorSourceLocation<'a> {
        PlayerErrorSourceLocation {
            file: self.file,
            line: self.line,
        }
    }
}

/// Corresponds to kMediaError

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerError<'a> {
    errorType: Cow<'a, str>,
    /// Code is the numeric enum entry for a specific set of error codes, such
    /// as PipelineStatusCodes in media/base/pipeline_status.h
    code: i64,
    /// A trace of where this error was caused / where it passed through.
    stack: Vec<PlayerErrorSourceLocation<'a>>,
    /// Errors potentially have a root cause error, ie, a DecoderError might be
    /// caused by an WindowsError
    cause: Vec<Box<PlayerError<'a>>>,
    /// Extra data attached to an error, such as an HRESULT, Video Codec, etc.
    data: serde_json::Map<String, JsonValue>,
}

impl<'a> PlayerError<'a> {
    pub fn builder(errorType: impl Into<Cow<'a, str>>, code: i64, stack: Vec<PlayerErrorSourceLocation<'a>>, cause: Vec<Box<PlayerError<'a>>>, data: serde_json::Map<String, JsonValue>) -> PlayerErrorBuilder<'a> {
        PlayerErrorBuilder {
            errorType: errorType.into(),
            code: code,
            stack: stack,
            cause: cause,
            data: data,
        }
    }
    pub fn errorType(&self) -> &str { self.errorType.as_ref() }
    pub fn code(&self) -> i64 { self.code }
    pub fn stack(&self) -> &[PlayerErrorSourceLocation<'a>] { &self.stack }
    pub fn cause(&self) -> &[Box<PlayerError<'a>>] { &self.cause }
    pub fn data(&self) -> &serde_json::Map<String, JsonValue> { &self.data }
}


pub struct PlayerErrorBuilder<'a> {
    errorType: Cow<'a, str>,
    code: i64,
    stack: Vec<PlayerErrorSourceLocation<'a>>,
    cause: Vec<Box<PlayerError<'a>>>,
    data: serde_json::Map<String, JsonValue>,
}

impl<'a> PlayerErrorBuilder<'a> {
    pub fn build(self) -> PlayerError<'a> {
        PlayerError {
            errorType: self.errorType,
            code: self.code,
            stack: self.stack,
            cause: self.cause,
            data: self.data,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Player<'a> {
    playerId: PlayerId<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> Player<'a> {
    pub fn builder(playerId: impl Into<PlayerId<'a>>) -> PlayerBuilder<'a> {
        PlayerBuilder {
            playerId: playerId.into(),
            domNodeId: None,
        }
    }
    pub fn playerId(&self) -> &PlayerId<'a> { &self.playerId }
    pub fn domNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.domNodeId.as_ref() }
}


pub struct PlayerBuilder<'a> {
    playerId: PlayerId<'a>,
    domNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> PlayerBuilder<'a> {
    pub fn domNodeId(mut self, domNodeId: crate::dom::BackendNodeId) -> Self { self.domNodeId = Some(domNodeId); self }
    pub fn build(self) -> Player<'a> {
        Player {
            playerId: self.playerId,
            domNodeId: self.domNodeId,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Media.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Media.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Media.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Media.disable";
    type Response = crate::EmptyReturns;
}
