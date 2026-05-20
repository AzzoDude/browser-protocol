//! This domain allows inspection of Web Audio API.
//! <https://webaudio.github.io/web-audio-api/>


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContextRealtimeData {
    /// The current context time in second in BaseAudioContext.
    #[serde(rename = "currentTime")]
    current_time: f64,
    /// The time spent on rendering graph divided by render quantum duration,
    /// and multiplied by 100. 100 means the audio renderer reached the full
    /// capacity and glitch may occur.
    #[serde(rename = "renderCapacity")]
    render_capacity: f64,
    /// A running mean of callback interval.
    #[serde(rename = "callbackIntervalMean")]
    callback_interval_mean: f64,
    /// A running variance of callback interval.
    #[serde(rename = "callbackIntervalVariance")]
    callback_interval_variance: f64,
}

impl ContextRealtimeData {
    /// Creates a builder for this type with the required parameters:
    /// * `current_time`: The current context time in second in BaseAudioContext.
    /// * `render_capacity`: The time spent on rendering graph divided by render quantum duration, and multiplied by 100. 100 means the audio renderer reached the full capacity and glitch may occur.
    /// * `callback_interval_mean`: A running mean of callback interval.
    /// * `callback_interval_variance`: A running variance of callback interval.
    pub fn builder(current_time: f64, render_capacity: f64, callback_interval_mean: f64, callback_interval_variance: f64) -> ContextRealtimeDataBuilder {
        ContextRealtimeDataBuilder {
            current_time: current_time,
            render_capacity: render_capacity,
            callback_interval_mean: callback_interval_mean,
            callback_interval_variance: callback_interval_variance,
        }
    }
    /// The current context time in second in BaseAudioContext.
    pub fn current_time(&self) -> f64 { self.current_time }
    /// The time spent on rendering graph divided by render quantum duration,
    /// and multiplied by 100. 100 means the audio renderer reached the full
    /// capacity and glitch may occur.
    pub fn render_capacity(&self) -> f64 { self.render_capacity }
    /// A running mean of callback interval.
    pub fn callback_interval_mean(&self) -> f64 { self.callback_interval_mean }
    /// A running variance of callback interval.
    pub fn callback_interval_variance(&self) -> f64 { self.callback_interval_variance }
}


pub struct ContextRealtimeDataBuilder {
    current_time: f64,
    render_capacity: f64,
    callback_interval_mean: f64,
    callback_interval_variance: f64,
}

impl ContextRealtimeDataBuilder {
    pub fn build(self) -> ContextRealtimeData {
        ContextRealtimeData {
            current_time: self.current_time,
            render_capacity: self.render_capacity,
            callback_interval_mean: self.callback_interval_mean,
            callback_interval_variance: self.callback_interval_variance,
        }
    }
}

/// Protocol object for BaseAudioContext

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BaseAudioContext<'a> {
    #[serde(rename = "contextId")]
    context_id: GraphObjectId<'a>,
    #[serde(rename = "contextType")]
    context_type: ContextType,
    #[serde(rename = "contextState")]
    context_state: ContextState,
    #[serde(skip_serializing_if = "Option::is_none", rename = "realtimeData")]
    realtime_data: Option<ContextRealtimeData>,
    /// Platform-dependent callback buffer size.
    #[serde(rename = "callbackBufferSize")]
    callback_buffer_size: f64,
    /// Number of output channels supported by audio hardware in use.
    #[serde(rename = "maxOutputChannelCount")]
    max_output_channel_count: f64,
    /// Context sample rate.
    #[serde(rename = "sampleRate")]
    sample_rate: f64,
}

impl<'a> BaseAudioContext<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `context_id`: 
    /// * `context_type`: 
    /// * `context_state`: 
    /// * `callback_buffer_size`: Platform-dependent callback buffer size.
    /// * `max_output_channel_count`: Number of output channels supported by audio hardware in use.
    /// * `sample_rate`: Context sample rate.
    pub fn builder(context_id: impl Into<GraphObjectId<'a>>, context_type: impl Into<ContextType>, context_state: impl Into<ContextState>, callback_buffer_size: f64, max_output_channel_count: f64, sample_rate: f64) -> BaseAudioContextBuilder<'a> {
        BaseAudioContextBuilder {
            context_id: context_id.into(),
            context_type: context_type.into(),
            context_state: context_state.into(),
            realtime_data: None,
            callback_buffer_size: callback_buffer_size,
            max_output_channel_count: max_output_channel_count,
            sample_rate: sample_rate,
        }
    }
    pub fn context_id(&self) -> &GraphObjectId<'a> { &self.context_id }
    pub fn context_type(&self) -> &ContextType { &self.context_type }
    pub fn context_state(&self) -> &ContextState { &self.context_state }
    pub fn realtime_data(&self) -> Option<&ContextRealtimeData> { self.realtime_data.as_ref() }
    /// Platform-dependent callback buffer size.
    pub fn callback_buffer_size(&self) -> f64 { self.callback_buffer_size }
    /// Number of output channels supported by audio hardware in use.
    pub fn max_output_channel_count(&self) -> f64 { self.max_output_channel_count }
    /// Context sample rate.
    pub fn sample_rate(&self) -> f64 { self.sample_rate }
}


pub struct BaseAudioContextBuilder<'a> {
    context_id: GraphObjectId<'a>,
    context_type: ContextType,
    context_state: ContextState,
    realtime_data: Option<ContextRealtimeData>,
    callback_buffer_size: f64,
    max_output_channel_count: f64,
    sample_rate: f64,
}

impl<'a> BaseAudioContextBuilder<'a> {
    pub fn realtime_data(mut self, realtime_data: ContextRealtimeData) -> Self { self.realtime_data = Some(realtime_data); self }
    pub fn build(self) -> BaseAudioContext<'a> {
        BaseAudioContext {
            context_id: self.context_id,
            context_type: self.context_type,
            context_state: self.context_state,
            realtime_data: self.realtime_data,
            callback_buffer_size: self.callback_buffer_size,
            max_output_channel_count: self.max_output_channel_count,
            sample_rate: self.sample_rate,
        }
    }
}

/// Protocol object for AudioListener

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioListener<'a> {
    #[serde(rename = "listenerId")]
    listener_id: GraphObjectId<'a>,
    #[serde(rename = "contextId")]
    context_id: GraphObjectId<'a>,
}

impl<'a> AudioListener<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `listener_id`: 
    /// * `context_id`: 
    pub fn builder(listener_id: impl Into<GraphObjectId<'a>>, context_id: impl Into<GraphObjectId<'a>>) -> AudioListenerBuilder<'a> {
        AudioListenerBuilder {
            listener_id: listener_id.into(),
            context_id: context_id.into(),
        }
    }
    pub fn listener_id(&self) -> &GraphObjectId<'a> { &self.listener_id }
    pub fn context_id(&self) -> &GraphObjectId<'a> { &self.context_id }
}


pub struct AudioListenerBuilder<'a> {
    listener_id: GraphObjectId<'a>,
    context_id: GraphObjectId<'a>,
}

impl<'a> AudioListenerBuilder<'a> {
    pub fn build(self) -> AudioListener<'a> {
        AudioListener {
            listener_id: self.listener_id,
            context_id: self.context_id,
        }
    }
}

/// Protocol object for AudioNode

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioNode<'a> {
    #[serde(rename = "nodeId")]
    node_id: GraphObjectId<'a>,
    #[serde(rename = "contextId")]
    context_id: GraphObjectId<'a>,
    #[serde(rename = "nodeType")]
    node_type: NodeType<'a>,
    #[serde(rename = "numberOfInputs")]
    number_of_inputs: f64,
    #[serde(rename = "numberOfOutputs")]
    number_of_outputs: f64,
    #[serde(rename = "channelCount")]
    channel_count: f64,
    #[serde(rename = "channelCountMode")]
    channel_count_mode: ChannelCountMode,
    #[serde(rename = "channelInterpretation")]
    channel_interpretation: ChannelInterpretation,
}

impl<'a> AudioNode<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: 
    /// * `context_id`: 
    /// * `node_type`: 
    /// * `number_of_inputs`: 
    /// * `number_of_outputs`: 
    /// * `channel_count`: 
    /// * `channel_count_mode`: 
    /// * `channel_interpretation`: 
    pub fn builder(node_id: impl Into<GraphObjectId<'a>>, context_id: impl Into<GraphObjectId<'a>>, node_type: impl Into<NodeType<'a>>, number_of_inputs: f64, number_of_outputs: f64, channel_count: f64, channel_count_mode: impl Into<ChannelCountMode>, channel_interpretation: impl Into<ChannelInterpretation>) -> AudioNodeBuilder<'a> {
        AudioNodeBuilder {
            node_id: node_id.into(),
            context_id: context_id.into(),
            node_type: node_type.into(),
            number_of_inputs: number_of_inputs,
            number_of_outputs: number_of_outputs,
            channel_count: channel_count,
            channel_count_mode: channel_count_mode.into(),
            channel_interpretation: channel_interpretation.into(),
        }
    }
    pub fn node_id(&self) -> &GraphObjectId<'a> { &self.node_id }
    pub fn context_id(&self) -> &GraphObjectId<'a> { &self.context_id }
    pub fn node_type(&self) -> &NodeType<'a> { &self.node_type }
    pub fn number_of_inputs(&self) -> f64 { self.number_of_inputs }
    pub fn number_of_outputs(&self) -> f64 { self.number_of_outputs }
    pub fn channel_count(&self) -> f64 { self.channel_count }
    pub fn channel_count_mode(&self) -> &ChannelCountMode { &self.channel_count_mode }
    pub fn channel_interpretation(&self) -> &ChannelInterpretation { &self.channel_interpretation }
}


pub struct AudioNodeBuilder<'a> {
    node_id: GraphObjectId<'a>,
    context_id: GraphObjectId<'a>,
    node_type: NodeType<'a>,
    number_of_inputs: f64,
    number_of_outputs: f64,
    channel_count: f64,
    channel_count_mode: ChannelCountMode,
    channel_interpretation: ChannelInterpretation,
}

impl<'a> AudioNodeBuilder<'a> {
    pub fn build(self) -> AudioNode<'a> {
        AudioNode {
            node_id: self.node_id,
            context_id: self.context_id,
            node_type: self.node_type,
            number_of_inputs: self.number_of_inputs,
            number_of_outputs: self.number_of_outputs,
            channel_count: self.channel_count,
            channel_count_mode: self.channel_count_mode,
            channel_interpretation: self.channel_interpretation,
        }
    }
}

/// Protocol object for AudioParam

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioParam<'a> {
    #[serde(rename = "paramId")]
    param_id: GraphObjectId<'a>,
    #[serde(rename = "nodeId")]
    node_id: GraphObjectId<'a>,
    #[serde(rename = "contextId")]
    context_id: GraphObjectId<'a>,
    #[serde(rename = "paramType")]
    param_type: ParamType<'a>,
    rate: AutomationRate,
    #[serde(rename = "defaultValue")]
    default_value: f64,
    #[serde(rename = "minValue")]
    min_value: f64,
    #[serde(rename = "maxValue")]
    max_value: f64,
}

impl<'a> AudioParam<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `param_id`: 
    /// * `node_id`: 
    /// * `context_id`: 
    /// * `param_type`: 
    /// * `rate`: 
    /// * `default_value`: 
    /// * `min_value`: 
    /// * `max_value`: 
    pub fn builder(param_id: impl Into<GraphObjectId<'a>>, node_id: impl Into<GraphObjectId<'a>>, context_id: impl Into<GraphObjectId<'a>>, param_type: impl Into<ParamType<'a>>, rate: impl Into<AutomationRate>, default_value: f64, min_value: f64, max_value: f64) -> AudioParamBuilder<'a> {
        AudioParamBuilder {
            param_id: param_id.into(),
            node_id: node_id.into(),
            context_id: context_id.into(),
            param_type: param_type.into(),
            rate: rate.into(),
            default_value: default_value,
            min_value: min_value,
            max_value: max_value,
        }
    }
    pub fn param_id(&self) -> &GraphObjectId<'a> { &self.param_id }
    pub fn node_id(&self) -> &GraphObjectId<'a> { &self.node_id }
    pub fn context_id(&self) -> &GraphObjectId<'a> { &self.context_id }
    pub fn param_type(&self) -> &ParamType<'a> { &self.param_type }
    pub fn rate(&self) -> &AutomationRate { &self.rate }
    pub fn default_value(&self) -> f64 { self.default_value }
    pub fn min_value(&self) -> f64 { self.min_value }
    pub fn max_value(&self) -> f64 { self.max_value }
}


pub struct AudioParamBuilder<'a> {
    param_id: GraphObjectId<'a>,
    node_id: GraphObjectId<'a>,
    context_id: GraphObjectId<'a>,
    param_type: ParamType<'a>,
    rate: AutomationRate,
    default_value: f64,
    min_value: f64,
    max_value: f64,
}

impl<'a> AudioParamBuilder<'a> {
    pub fn build(self) -> AudioParam<'a> {
        AudioParam {
            param_id: self.param_id,
            node_id: self.node_id,
            context_id: self.context_id,
            param_type: self.param_type,
            rate: self.rate,
            default_value: self.default_value,
            min_value: self.min_value,
            max_value: self.max_value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "WebAudio.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "WebAudio.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "WebAudio.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "WebAudio.disable";
    type Response = crate::EmptyReturns;
}

/// Fetch the realtime data from the registered contexts.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRealtimeDataParams<'a> {
    #[serde(rename = "contextId")]
    context_id: GraphObjectId<'a>,
}

impl<'a> GetRealtimeDataParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `context_id`: 
    pub fn builder(context_id: impl Into<GraphObjectId<'a>>) -> GetRealtimeDataParamsBuilder<'a> {
        GetRealtimeDataParamsBuilder {
            context_id: context_id.into(),
        }
    }
    pub fn context_id(&self) -> &GraphObjectId<'a> { &self.context_id }
}


pub struct GetRealtimeDataParamsBuilder<'a> {
    context_id: GraphObjectId<'a>,
}

impl<'a> GetRealtimeDataParamsBuilder<'a> {
    pub fn build(self) -> GetRealtimeDataParams<'a> {
        GetRealtimeDataParams {
            context_id: self.context_id,
        }
    }
}

/// Fetch the realtime data from the registered contexts.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRealtimeDataReturns {
    #[serde(rename = "realtimeData")]
    realtime_data: ContextRealtimeData,
}

impl GetRealtimeDataReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `realtime_data`: 
    pub fn builder(realtime_data: ContextRealtimeData) -> GetRealtimeDataReturnsBuilder {
        GetRealtimeDataReturnsBuilder {
            realtime_data: realtime_data,
        }
    }
    pub fn realtime_data(&self) -> &ContextRealtimeData { &self.realtime_data }
}


pub struct GetRealtimeDataReturnsBuilder {
    realtime_data: ContextRealtimeData,
}

impl GetRealtimeDataReturnsBuilder {
    pub fn build(self) -> GetRealtimeDataReturns {
        GetRealtimeDataReturns {
            realtime_data: self.realtime_data,
        }
    }
}

impl<'a> GetRealtimeDataParams<'a> { pub const METHOD: &'static str = "WebAudio.getRealtimeData"; }

impl<'a> crate::CdpCommand<'a> for GetRealtimeDataParams<'a> {
    const METHOD: &'static str = "WebAudio.getRealtimeData";
    type Response = GetRealtimeDataReturns;
}
