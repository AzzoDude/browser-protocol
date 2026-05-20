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
    pub fn builder() -> PlayerMessageBuilder<'a> { PlayerMessageBuilder::default() }
    pub fn level(&self) -> &str { self.level.as_ref() }
    pub fn message(&self) -> &str { self.message.as_ref() }
}

#[derive(Default)]
pub struct PlayerMessageBuilder<'a> {
    level: Option<Cow<'a, str>>,
    message: Option<Cow<'a, str>>,
}

impl<'a> PlayerMessageBuilder<'a> {
    /// Keep in sync with MediaLogMessageLevel
    /// We are currently keeping the message level 'error' separate from the
    /// PlayerError type because right now they represent different things,
    /// this one being a DVLOG(ERROR) style log message that gets printed
    /// based on what log level is selected in the UI, and the other is a
    /// representation of a media::PipelineStatus object. Soon however we're
    /// going to be moving away from using PipelineStatus for errors and
    /// introducing a new error type which should hopefully let us integrate
    /// the error log level into the PlayerError type.
    pub fn level(mut self, level: impl Into<Cow<'a, str>>) -> Self { self.level = Some(level.into()); self }
    pub fn message(mut self, message: impl Into<Cow<'a, str>>) -> Self { self.message = Some(message.into()); self }
    pub fn build(self) -> PlayerMessage<'a> {
        PlayerMessage {
            level: self.level.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
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
    pub fn builder() -> PlayerPropertyBuilder<'a> { PlayerPropertyBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct PlayerPropertyBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> PlayerPropertyBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> PlayerProperty<'a> {
        PlayerProperty {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
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
    pub fn builder() -> PlayerEventBuilder<'a> { PlayerEventBuilder::default() }
    pub fn timestamp(&self) -> &Timestamp { &self.timestamp }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct PlayerEventBuilder<'a> {
    timestamp: Option<Timestamp>,
    value: Option<Cow<'a, str>>,
}

impl<'a> PlayerEventBuilder<'a> {
    pub fn timestamp(mut self, timestamp: Timestamp) -> Self { self.timestamp = Some(timestamp); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> PlayerEvent<'a> {
        PlayerEvent {
            timestamp: self.timestamp.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
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
    pub fn builder() -> PlayerErrorSourceLocationBuilder<'a> { PlayerErrorSourceLocationBuilder::default() }
    pub fn file(&self) -> &str { self.file.as_ref() }
    pub fn line(&self) -> i64 { self.line }
}

#[derive(Default)]
pub struct PlayerErrorSourceLocationBuilder<'a> {
    file: Option<Cow<'a, str>>,
    line: Option<i64>,
}

impl<'a> PlayerErrorSourceLocationBuilder<'a> {
    pub fn file(mut self, file: impl Into<Cow<'a, str>>) -> Self { self.file = Some(file.into()); self }
    pub fn line(mut self, line: i64) -> Self { self.line = Some(line); self }
    pub fn build(self) -> PlayerErrorSourceLocation<'a> {
        PlayerErrorSourceLocation {
            file: self.file.unwrap_or_default(),
            line: self.line.unwrap_or_default(),
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
    pub fn builder() -> PlayerErrorBuilder<'a> { PlayerErrorBuilder::default() }
    pub fn errorType(&self) -> &str { self.errorType.as_ref() }
    pub fn code(&self) -> i64 { self.code }
    pub fn stack(&self) -> &[PlayerErrorSourceLocation<'a>] { &self.stack }
    pub fn cause(&self) -> &[Box<PlayerError<'a>>] { &self.cause }
    pub fn data(&self) -> &serde_json::Map<String, JsonValue> { &self.data }
}

#[derive(Default)]
pub struct PlayerErrorBuilder<'a> {
    errorType: Option<Cow<'a, str>>,
    code: Option<i64>,
    stack: Option<Vec<PlayerErrorSourceLocation<'a>>>,
    cause: Option<Vec<Box<PlayerError<'a>>>>,
    data: Option<serde_json::Map<String, JsonValue>>,
}

impl<'a> PlayerErrorBuilder<'a> {
    pub fn errorType(mut self, errorType: impl Into<Cow<'a, str>>) -> Self { self.errorType = Some(errorType.into()); self }
    /// Code is the numeric enum entry for a specific set of error codes, such
    /// as PipelineStatusCodes in media/base/pipeline_status.h
    pub fn code(mut self, code: i64) -> Self { self.code = Some(code); self }
    /// A trace of where this error was caused / where it passed through.
    pub fn stack(mut self, stack: Vec<PlayerErrorSourceLocation<'a>>) -> Self { self.stack = Some(stack); self }
    /// Errors potentially have a root cause error, ie, a DecoderError might be
    /// caused by an WindowsError
    pub fn cause(mut self, cause: Vec<Box<PlayerError<'a>>>) -> Self { self.cause = Some(cause); self }
    /// Extra data attached to an error, such as an HRESULT, Video Codec, etc.
    pub fn data(mut self, data: serde_json::Map<String, JsonValue>) -> Self { self.data = Some(data); self }
    pub fn build(self) -> PlayerError<'a> {
        PlayerError {
            errorType: self.errorType.unwrap_or_default(),
            code: self.code.unwrap_or_default(),
            stack: self.stack.unwrap_or_default(),
            cause: self.cause.unwrap_or_default(),
            data: self.data.unwrap_or_default(),
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
    pub fn builder() -> PlayerBuilder<'a> { PlayerBuilder::default() }
    pub fn playerId(&self) -> &PlayerId<'a> { &self.playerId }
    pub fn domNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.domNodeId.as_ref() }
}

#[derive(Default)]
pub struct PlayerBuilder<'a> {
    playerId: Option<PlayerId<'a>>,
    domNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> PlayerBuilder<'a> {
    pub fn playerId(mut self, playerId: PlayerId<'a>) -> Self { self.playerId = Some(playerId); self }
    pub fn domNodeId(mut self, domNodeId: crate::dom::BackendNodeId) -> Self { self.domNodeId = Some(domNodeId); self }
    pub fn build(self) -> Player<'a> {
        Player {
            playerId: self.playerId.unwrap_or_default(),
            domNodeId: self.domNodeId,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct EnableParamsBuilder {}

impl EnableParamsBuilder {
    pub fn build(self) -> EnableParams {
        EnableParams {}
    }
}

impl EnableParams { pub const METHOD: &'static str = "Media.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Media.enable";
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

impl DisableParams { pub const METHOD: &'static str = "Media.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Media.disable";
    type Response = crate::EmptyReturns;
}
