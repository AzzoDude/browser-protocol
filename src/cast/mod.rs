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
    pub fn builder() -> SinkBuilder<'a> { SinkBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn session(&self) -> Option<&str> { self.session.as_deref() }
}

#[derive(Default)]
pub struct SinkBuilder<'a> {
    name: Option<Cow<'a, str>>,
    id: Option<Cow<'a, str>>,
    session: Option<Cow<'a, str>>,
}

impl<'a> SinkBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    /// Text describing the current session. Present only if there is an active
    /// session on the sink.
    pub fn session(mut self, session: impl Into<Cow<'a, str>>) -> Self { self.session = Some(session.into()); self }
    pub fn build(self) -> Sink<'a> {
        Sink {
            name: self.name.unwrap_or_default(),
            id: self.id.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    presentationUrl: Option<Cow<'a, str>>,
}

impl<'a> EnableParams<'a> {
    pub fn builder() -> EnableParamsBuilder<'a> { EnableParamsBuilder::default() }
    pub fn presentationUrl(&self) -> Option<&str> { self.presentationUrl.as_deref() }
}

#[derive(Default)]
pub struct EnableParamsBuilder<'a> {
    presentationUrl: Option<Cow<'a, str>>,
}

impl<'a> EnableParamsBuilder<'a> {
    pub fn presentationUrl(mut self, presentationUrl: impl Into<Cow<'a, str>>) -> Self { self.presentationUrl = Some(presentationUrl.into()); self }
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            presentationUrl: self.presentationUrl,
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
    sinkName: Cow<'a, str>,
}

impl<'a> SetSinkToUseParams<'a> {
    pub fn builder() -> SetSinkToUseParamsBuilder<'a> { SetSinkToUseParamsBuilder::default() }
    pub fn sinkName(&self) -> &str { self.sinkName.as_ref() }
}

#[derive(Default)]
pub struct SetSinkToUseParamsBuilder<'a> {
    sinkName: Option<Cow<'a, str>>,
}

impl<'a> SetSinkToUseParamsBuilder<'a> {
    pub fn sinkName(mut self, sinkName: impl Into<Cow<'a, str>>) -> Self { self.sinkName = Some(sinkName.into()); self }
    pub fn build(self) -> SetSinkToUseParams<'a> {
        SetSinkToUseParams {
            sinkName: self.sinkName.unwrap_or_default(),
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
    sinkName: Cow<'a, str>,
}

impl<'a> StartDesktopMirroringParams<'a> {
    pub fn builder() -> StartDesktopMirroringParamsBuilder<'a> { StartDesktopMirroringParamsBuilder::default() }
    pub fn sinkName(&self) -> &str { self.sinkName.as_ref() }
}

#[derive(Default)]
pub struct StartDesktopMirroringParamsBuilder<'a> {
    sinkName: Option<Cow<'a, str>>,
}

impl<'a> StartDesktopMirroringParamsBuilder<'a> {
    pub fn sinkName(mut self, sinkName: impl Into<Cow<'a, str>>) -> Self { self.sinkName = Some(sinkName.into()); self }
    pub fn build(self) -> StartDesktopMirroringParams<'a> {
        StartDesktopMirroringParams {
            sinkName: self.sinkName.unwrap_or_default(),
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
    sinkName: Cow<'a, str>,
}

impl<'a> StartTabMirroringParams<'a> {
    pub fn builder() -> StartTabMirroringParamsBuilder<'a> { StartTabMirroringParamsBuilder::default() }
    pub fn sinkName(&self) -> &str { self.sinkName.as_ref() }
}

#[derive(Default)]
pub struct StartTabMirroringParamsBuilder<'a> {
    sinkName: Option<Cow<'a, str>>,
}

impl<'a> StartTabMirroringParamsBuilder<'a> {
    pub fn sinkName(mut self, sinkName: impl Into<Cow<'a, str>>) -> Self { self.sinkName = Some(sinkName.into()); self }
    pub fn build(self) -> StartTabMirroringParams<'a> {
        StartTabMirroringParams {
            sinkName: self.sinkName.unwrap_or_default(),
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
    sinkName: Cow<'a, str>,
}

impl<'a> StopCastingParams<'a> {
    pub fn builder() -> StopCastingParamsBuilder<'a> { StopCastingParamsBuilder::default() }
    pub fn sinkName(&self) -> &str { self.sinkName.as_ref() }
}

#[derive(Default)]
pub struct StopCastingParamsBuilder<'a> {
    sinkName: Option<Cow<'a, str>>,
}

impl<'a> StopCastingParamsBuilder<'a> {
    pub fn sinkName(mut self, sinkName: impl Into<Cow<'a, str>>) -> Self { self.sinkName = Some(sinkName.into()); self }
    pub fn build(self) -> StopCastingParams<'a> {
        StopCastingParams {
            sinkName: self.sinkName.unwrap_or_default(),
        }
    }
}

impl<'a> StopCastingParams<'a> { pub const METHOD: &'static str = "Cast.stopCasting"; }

impl<'a> crate::CdpCommand<'a> for StopCastingParams<'a> {
    const METHOD: &'static str = "Cast.stopCasting";
    type Response = crate::EmptyReturns;
}
