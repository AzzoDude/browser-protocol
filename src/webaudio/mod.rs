use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! This domain allows inspection of Web Audio API.
//! https://webaudio.github.io/web-audio-api/

/// An unique ID for a graph object (AudioContext, AudioNode, AudioParam) in Web Audio API

pub type GraphObjectId = String;

/// Enum of BaseAudioContext types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContextType {
    #[default]
    Realtime,
    Offline,
}

/// Enum of AudioContextState from the spec

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContextState {
    #[default]
    Suspended,
    Running,
    Closed,
    Interrupted,
}

/// Enum of AudioNode types

pub type NodeType = String;

/// Enum of AudioNode::ChannelCountMode from the spec

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ChannelCountMode {
    #[default]
    ClampedMax,
    Explicit,
    Max,
}

/// Enum of AudioNode::ChannelInterpretation from the spec

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ChannelInterpretation {
    #[default]
    Discrete,
    Speakers,
}

/// Enum of AudioParam types

pub type ParamType = String;

/// Enum of AudioParam::AutomationRate from the spec

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AutomationRate {
    #[default]
    ARate,
    KRate,
}

/// Fields in AudioContext that change in real-time.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContextRealtimeData {
    /// The current context time in second in BaseAudioContext.

    pub currentTime: f64,
    /// The time spent on rendering graph divided by render quantum duration,
    /// and multiplied by 100. 100 means the audio renderer reached the full
    /// capacity and glitch may occur.

    pub renderCapacity: f64,
    /// A running mean of callback interval.

    pub callbackIntervalMean: f64,
    /// A running variance of callback interval.

    pub callbackIntervalVariance: f64,
}

/// Protocol object for BaseAudioContext

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BaseAudioContext {

    pub contextId: GraphObjectId,

    pub contextType: ContextType,

    pub contextState: ContextState,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtimeData: Option<ContextRealtimeData>,
    /// Platform-dependent callback buffer size.

    pub callbackBufferSize: f64,
    /// Number of output channels supported by audio hardware in use.

    pub maxOutputChannelCount: f64,
    /// Context sample rate.

    pub sampleRate: f64,
}

/// Protocol object for AudioListener

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioListener {

    pub listenerId: GraphObjectId,

    pub contextId: GraphObjectId,
}

/// Protocol object for AudioNode

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioNode {

    pub nodeId: GraphObjectId,

    pub contextId: GraphObjectId,

    pub nodeType: NodeType,

    pub numberOfInputs: f64,

    pub numberOfOutputs: f64,

    pub channelCount: f64,

    pub channelCountMode: ChannelCountMode,

    pub channelInterpretation: ChannelInterpretation,
}

/// Protocol object for AudioParam

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioParam {

    pub paramId: GraphObjectId,

    pub nodeId: GraphObjectId,

    pub contextId: GraphObjectId,

    pub paramType: ParamType,

    pub rate: AutomationRate,

    pub defaultValue: f64,

    pub minValue: f64,

    pub maxValue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "WebAudio.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "WebAudio.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "WebAudio.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "WebAudio.disable";
    type Response = crate::EmptyReturns;
}

/// Fetch the realtime data from the registered contexts.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRealtimeDataParams {

    pub contextId: GraphObjectId,
}

/// Fetch the realtime data from the registered contexts.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRealtimeDataReturns {

    pub realtimeData: ContextRealtimeData,
}

impl GetRealtimeDataParams { pub const METHOD: &'static str = "WebAudio.getRealtimeData"; }

impl crate::CdpCommand for GetRealtimeDataParams {
    const METHOD: &'static str = "WebAudio.getRealtimeData";
    type Response = GetRealtimeDataReturns;
}
