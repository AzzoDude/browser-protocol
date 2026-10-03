//! A domain for interacting with Cast, Presentation API, and Remote Playback API
//! functionalities.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Sink<'a> {
    pub name: Cow<'a, str>,
    pub id: Cow<'a, str>,
    /// Text describing the current session. Present only if there is an active
    /// session on the sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Cow<'a, str>>,
}
/// Starts observing for sinks that can be used for tab mirroring, and if set,
/// sinks compatible with |presentationUrl| as well. When sinks are found, a
/// |sinksUpdated| event is fired.
/// Also starts observing for issue messages. When an issue is added or removed,
/// an |issueUpdated| event is fired.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Cast.enable")]
pub struct EnableParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "presentationUrl")]
    pub presentation_url: Option<Cow<'a, str>>,
}
/// Stops observing for sinks and issues.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Cast.disable")]
pub struct DisableParams {

}
/// Sets a sink to be used when the web page requests the browser to choose a
/// sink via Presentation API, Remote Playback API, or Cast SDK.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Cast.setSinkToUse")]
pub struct SetSinkToUseParams<'a> {
    #[serde(rename = "sinkName")]
    pub sink_name: Cow<'a, str>,
}
/// Starts mirroring the desktop to the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Cast.startDesktopMirroring")]
pub struct StartDesktopMirroringParams<'a> {
    #[serde(rename = "sinkName")]
    pub sink_name: Cow<'a, str>,
}
/// Starts mirroring the tab to the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Cast.startTabMirroring")]
pub struct StartTabMirroringParams<'a> {
    #[serde(rename = "sinkName")]
    pub sink_name: Cow<'a, str>,
}
/// Stops the active Cast session on the sink.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Cast.stopCasting")]
pub struct StopCastingParams<'a> {
    #[serde(rename = "sinkName")]
    pub sink_name: Cow<'a, str>,
}
/// This is fired whenever the list of available sinks changes. A sink is a
/// device or a software surface that you can cast to.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Cast.sinksUpdated")]
pub struct SinksUpdated<'a> {
    pub sinks: Vec<Sink<'a>>,
}
/// This is fired whenever the outstanding issue/error message changes.
/// |issueMessage| is empty if there is no issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Cast.issueUpdated")]
pub struct IssueUpdated<'a> {
    #[serde(rename = "issueMessage")]
    pub issue_message: Cow<'a, str>,
}