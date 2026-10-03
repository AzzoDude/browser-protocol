//! This domain allows inspection of Web Audio API.
//! <https://webaudio.github.io/web-audio-api/>


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// An unique ID for a graph object (AudioContext, AudioNode, AudioParam) in Web Audio API

pub type GraphObjectId<'a> = Cow<'a, str>;

/// Enum of BaseAudioContext types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContextType {
    #[default]
    #[serde(rename = "realtime")]
    Realtime,
    #[serde(rename = "offline")]
    Offline,
}

/// Enum of AudioContextState from the spec

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContextState {
    #[default]
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "interrupted")]
    Interrupted,
}

/// Enum of AudioNode types

pub type NodeType<'a> = Cow<'a, str>;

/// Enum of AudioNode::ChannelCountMode from the spec

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ChannelCountMode {
    #[default]
    #[serde(rename = "clamped-max")]
    ClampedMax,
    #[serde(rename = "explicit")]
    Explicit,
    #[serde(rename = "max")]
    Max,
}

/// Enum of AudioNode::ChannelInterpretation from the spec

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ChannelInterpretation {
    #[default]
    #[serde(rename = "discrete")]
    Discrete,
    #[serde(rename = "speakers")]
    Speakers,
}

/// Enum of AudioParam types

pub type ParamType<'a> = Cow<'a, str>;

/// Enum of AudioParam::AutomationRate from the spec

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AutomationRate {
    #[default]
    #[serde(rename = "a-rate")]
    ARate,
    #[serde(rename = "k-rate")]
    KRate,
}

/// Fields in AudioContext that change in real-time.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ContextRealtimeData {
    /// The current context time in second in BaseAudioContext.
    #[serde(rename = "currentTime")]
    pub current_time: f64,
    /// The time spent on rendering graph divided by render quantum duration,
    /// and multiplied by 100. 100 means the audio renderer reached the full
    /// capacity and glitch may occur.
    #[serde(rename = "renderCapacity")]
    pub render_capacity: f64,
    /// A running mean of callback interval.
    #[serde(rename = "callbackIntervalMean")]
    pub callback_interval_mean: f64,
    /// A running variance of callback interval.
    #[serde(rename = "callbackIntervalVariance")]
    pub callback_interval_variance: f64,
}
/// Protocol object for BaseAudioContext

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BaseAudioContext<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "contextType")]
    pub context_type: ContextType,
    #[serde(rename = "contextState")]
    pub context_state: ContextState,
    #[serde(skip_serializing_if = "Option::is_none", rename = "realtimeData")]
    pub realtime_data: Option<ContextRealtimeData>,
    /// Platform-dependent callback buffer size.
    #[serde(rename = "callbackBufferSize")]
    pub callback_buffer_size: f64,
    /// Number of output channels supported by audio hardware in use.
    #[serde(rename = "maxOutputChannelCount")]
    pub max_output_channel_count: f64,
    /// Context sample rate.
    #[serde(rename = "sampleRate")]
    pub sample_rate: f64,
    #[serde(rename = "renderQuantumSize")]
    pub render_quantum_size: f64,
}
/// Protocol object for AudioListener

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AudioListener<'a> {
    #[serde(rename = "listenerId")]
    pub listener_id: GraphObjectId<'a>,
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
}
/// Protocol object for AudioNode

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AudioNode<'a> {
    #[serde(rename = "nodeId")]
    pub node_id: GraphObjectId<'a>,
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "nodeType")]
    pub node_type: NodeType<'a>,
    #[serde(rename = "numberOfInputs")]
    pub number_of_inputs: f64,
    #[serde(rename = "numberOfOutputs")]
    pub number_of_outputs: f64,
    #[serde(rename = "channelCount")]
    pub channel_count: f64,
    #[serde(rename = "channelCountMode")]
    pub channel_count_mode: ChannelCountMode,
    #[serde(rename = "channelInterpretation")]
    pub channel_interpretation: ChannelInterpretation,
}
/// Protocol object for AudioParam

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AudioParam<'a> {
    #[serde(rename = "paramId")]
    pub param_id: GraphObjectId<'a>,
    #[serde(rename = "nodeId")]
    pub node_id: GraphObjectId<'a>,
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "paramType")]
    pub param_type: ParamType<'a>,
    pub rate: AutomationRate,
    #[serde(rename = "defaultValue")]
    pub default_value: f64,
    #[serde(rename = "minValue")]
    pub min_value: f64,
    #[serde(rename = "maxValue")]
    pub max_value: f64,
}
/// Enables the WebAudio domain and starts sending context lifetime events.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.enable")]
pub struct EnableParams {

}
/// Disables the WebAudio domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.disable")]
pub struct DisableParams {

}
/// Fetch the realtime data from the registered contexts.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.getRealtimeData", response = "GetRealtimeDataReturns")]
pub struct GetRealtimeDataParams<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
}
/// Fetch the realtime data from the registered contexts.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetRealtimeDataReturns {
    #[serde(rename = "realtimeData")]
    pub realtime_data: ContextRealtimeData,
}
/// Notifies that a new BaseAudioContext has been created.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.contextCreated")]
pub struct ContextCreated<'a> {
    pub context: BaseAudioContext<'a>,
}
/// Notifies that an existing BaseAudioContext will be destroyed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.contextWillBeDestroyed")]
pub struct ContextWillBeDestroyed<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
}
/// Notifies that existing BaseAudioContext has changed some properties (id stays the same)..

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.contextChanged")]
pub struct ContextChanged<'a> {
    pub context: BaseAudioContext<'a>,
}
/// Notifies that the construction of an AudioListener has finished.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.audioListenerCreated")]
pub struct AudioListenerCreated<'a> {
    pub listener: AudioListener<'a>,
}
/// Notifies that a new AudioListener has been created.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.audioListenerWillBeDestroyed")]
pub struct AudioListenerWillBeDestroyed<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "listenerId")]
    pub listener_id: GraphObjectId<'a>,
}
/// Notifies that a new AudioNode has been created.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.audioNodeCreated")]
pub struct AudioNodeCreated<'a> {
    pub node: AudioNode<'a>,
}
/// Notifies that an existing AudioNode has been destroyed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.audioNodeWillBeDestroyed")]
pub struct AudioNodeWillBeDestroyed<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "nodeId")]
    pub node_id: GraphObjectId<'a>,
}
/// Notifies that a new AudioParam has been created.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.audioParamCreated")]
pub struct AudioParamCreated<'a> {
    pub param: AudioParam<'a>,
}
/// Notifies that an existing AudioParam has been destroyed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.audioParamWillBeDestroyed")]
pub struct AudioParamWillBeDestroyed<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "nodeId")]
    pub node_id: GraphObjectId<'a>,
    #[serde(rename = "paramId")]
    pub param_id: GraphObjectId<'a>,
}
/// Notifies that two AudioNodes are connected.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.nodesConnected")]
pub struct NodesConnected<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "sourceId")]
    pub source_id: GraphObjectId<'a>,
    #[serde(rename = "destinationId")]
    pub destination_id: GraphObjectId<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceOutputIndex")]
    pub source_output_index: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "destinationInputIndex")]
    pub destination_input_index: Option<f64>,
}
/// Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.nodesDisconnected")]
pub struct NodesDisconnected<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "sourceId")]
    pub source_id: GraphObjectId<'a>,
    #[serde(rename = "destinationId")]
    pub destination_id: GraphObjectId<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceOutputIndex")]
    pub source_output_index: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "destinationInputIndex")]
    pub destination_input_index: Option<f64>,
}
/// Notifies that an AudioNode is connected to an AudioParam.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.nodeParamConnected")]
pub struct NodeParamConnected<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "sourceId")]
    pub source_id: GraphObjectId<'a>,
    #[serde(rename = "destinationId")]
    pub destination_id: GraphObjectId<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceOutputIndex")]
    pub source_output_index: Option<f64>,
}
/// Notifies that an AudioNode is disconnected to an AudioParam.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAudio.nodeParamDisconnected")]
pub struct NodeParamDisconnected<'a> {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId<'a>,
    #[serde(rename = "sourceId")]
    pub source_id: GraphObjectId<'a>,
    #[serde(rename = "destinationId")]
    pub destination_id: GraphObjectId<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceOutputIndex")]
    pub source_output_index: Option<f64>,
}