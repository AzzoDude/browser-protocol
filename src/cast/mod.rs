//! A domain for interacting with Cast, Presentation API, and Remote Playback API
//! functionalities.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sink<'a> {
    name: Cow<'a, str>,
    id: Cow<'a, str>,
    /// Text describing the current session. Present only if there is an active
    /// session on the sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<Cow<'a, str>>,
}

impl<'a> Sink<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: 
    /// * `id`: 
    pub fn builder(name: impl Into<Cow<'a, str>>, id: impl Into<Cow<'a, str>>) -> SinkBuilder<'a> {
        SinkBuilder {
            name: name.into(),
            id: id.into(),
            session: None,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    /// Text describing the current session. Present only if there is an active
    /// session on the sink.
    pub fn session(&self) -> Option<&str> { self.session.as_deref() }
}


pub struct SinkBuilder<'a> {
    name: Cow<'a, str>,
    id: Cow<'a, str>,
    session: Option<Cow<'a, str>>,
}

impl<'a> SinkBuilder<'a> {
    /// Text describing the current session. Present only if there is an active
    /// session on the sink.
    pub fn session(mut self, session: impl Into<Cow<'a, str>>) -> Self { self.session = Some(session.into()); self }
    pub fn build(self) -> Sink<'a> {
        Sink {
            name: self.name,
            id: self.id,
            session: self.session,
        }
    }
}

/// Starts observing for sinks that can be used for tab mirroring, and if set,
/// sinks compatible with |presentationUrl| as well. When sinks are found, a
/// |sinksUpdated| event is fired.
/// Also starts observing for issue messages. When an issue is added or removed,
/// an |issueUpdated| event is fired.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "presentationUrl")]
    presentation_url: Option<Cow<'a, str>>,
}

impl<'a> EnableParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> EnableParamsBuilder<'a> {
        EnableParamsBuilder {
            presentation_url: None,
        }
    }
    pub fn presentation_url(&self) -> Option<&str> { self.presentation_url.as_deref() }
}

#[derive(Default)]
pub struct EnableParamsBuilder<'a> {
    presentation_url: Option<Cow<'a, str>>,
}

impl<'a> EnableParamsBuilder<'a> {
    pub fn presentation_url(mut self, presentation_url: impl Into<Cow<'a, str>>) -> Self { self.presentation_url = Some(presentation_url.into()); self }
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            presentation_url: self.presentation_url,
        }
    }
}

impl<'a> EnableParams<'a> { pub const METHOD: &'static str = "Cast.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams<'a> {
    const METHOD: &'static str = "Cast.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Cast.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Cast.disable";
    type Response = crate::EmptyReturns;
}

/// Sets a sink to be used when the web page requests the browser to choose a
/// sink via Presentation API, Remote Playback API, or Cast SDK.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSinkToUseParams<'a> {
    #[serde(rename = "sinkName")]
    sink_name: Cow<'a, str>,
}

impl<'a> SetSinkToUseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `sink_name`: 
    pub fn builder(sink_name: impl Into<Cow<'a, str>>) -> SetSinkToUseParamsBuilder<'a> {
        SetSinkToUseParamsBuilder {
            sink_name: sink_name.into(),
        }
    }
    pub fn sink_name(&self) -> &str { self.sink_name.as_ref() }
}


pub struct SetSinkToUseParamsBuilder<'a> {
    sink_name: Cow<'a, str>,
}

impl<'a> SetSinkToUseParamsBuilder<'a> {
    pub fn build(self) -> SetSinkToUseParams<'a> {
        SetSinkToUseParams {
            sink_name: self.sink_name,
        }
    }
}

impl<'a> SetSinkToUseParams<'a> { pub const METHOD: &'static str = "Cast.setSinkToUse"; }

impl<'a> crate::CdpCommand<'a> for SetSinkToUseParams<'a> {
    const METHOD: &'static str = "Cast.setSinkToUse";
    type Response = crate::EmptyReturns;
}

/// Starts mirroring the desktop to the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartDesktopMirroringParams<'a> {
    #[serde(rename = "sinkName")]
    sink_name: Cow<'a, str>,
}

impl<'a> StartDesktopMirroringParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `sink_name`: 
    pub fn builder(sink_name: impl Into<Cow<'a, str>>) -> StartDesktopMirroringParamsBuilder<'a> {
        StartDesktopMirroringParamsBuilder {
            sink_name: sink_name.into(),
        }
    }
    pub fn sink_name(&self) -> &str { self.sink_name.as_ref() }
}


pub struct StartDesktopMirroringParamsBuilder<'a> {
    sink_name: Cow<'a, str>,
}

impl<'a> StartDesktopMirroringParamsBuilder<'a> {
    pub fn build(self) -> StartDesktopMirroringParams<'a> {
        StartDesktopMirroringParams {
            sink_name: self.sink_name,
        }
    }
}

impl<'a> StartDesktopMirroringParams<'a> { pub const METHOD: &'static str = "Cast.startDesktopMirroring"; }

impl<'a> crate::CdpCommand<'a> for StartDesktopMirroringParams<'a> {
    const METHOD: &'static str = "Cast.startDesktopMirroring";
    type Response = crate::EmptyReturns;
}

/// Starts mirroring the tab to the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTabMirroringParams<'a> {
    #[serde(rename = "sinkName")]
    sink_name: Cow<'a, str>,
}

impl<'a> StartTabMirroringParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `sink_name`: 
    pub fn builder(sink_name: impl Into<Cow<'a, str>>) -> StartTabMirroringParamsBuilder<'a> {
        StartTabMirroringParamsBuilder {
            sink_name: sink_name.into(),
        }
    }
    pub fn sink_name(&self) -> &str { self.sink_name.as_ref() }
}


pub struct StartTabMirroringParamsBuilder<'a> {
    sink_name: Cow<'a, str>,
}

impl<'a> StartTabMirroringParamsBuilder<'a> {
    pub fn build(self) -> StartTabMirroringParams<'a> {
        StartTabMirroringParams {
            sink_name: self.sink_name,
        }
    }
}

impl<'a> StartTabMirroringParams<'a> { pub const METHOD: &'static str = "Cast.startTabMirroring"; }

impl<'a> crate::CdpCommand<'a> for StartTabMirroringParams<'a> {
    const METHOD: &'static str = "Cast.startTabMirroring";
    type Response = crate::EmptyReturns;
}

/// Stops the active Cast session on the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopCastingParams<'a> {
    #[serde(rename = "sinkName")]
    sink_name: Cow<'a, str>,
}

impl<'a> StopCastingParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `sink_name`: 
    pub fn builder(sink_name: impl Into<Cow<'a, str>>) -> StopCastingParamsBuilder<'a> {
        StopCastingParamsBuilder {
            sink_name: sink_name.into(),
        }
    }
    pub fn sink_name(&self) -> &str { self.sink_name.as_ref() }
}


pub struct StopCastingParamsBuilder<'a> {
    sink_name: Cow<'a, str>,
}

impl<'a> StopCastingParamsBuilder<'a> {
    pub fn build(self) -> StopCastingParams<'a> {
        StopCastingParams {
            sink_name: self.sink_name,
        }
    }
}

impl<'a> StopCastingParams<'a> { pub const METHOD: &'static str = "Cast.stopCasting"; }

impl<'a> crate::CdpCommand<'a> for StopCastingParams<'a> {
    const METHOD: &'static str = "Cast.stopCasting";
    type Response = crate::EmptyReturns;
}
