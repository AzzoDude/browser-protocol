//! This domain allows inspection of Web Audio API.
//! https://webaudio.github.io/web-audio-api/


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
    currentTime: f64,
    /// The time spent on rendering graph divided by render quantum duration,
    /// and multiplied by 100. 100 means the audio renderer reached the full
    /// capacity and glitch may occur.
    renderCapacity: f64,
    /// A running mean of callback interval.
    callbackIntervalMean: f64,
    /// A running variance of callback interval.
    callbackIntervalVariance: f64,
}

impl ContextRealtimeData {
    pub fn builder(currentTime: f64, renderCapacity: f64, callbackIntervalMean: f64, callbackIntervalVariance: f64) -> ContextRealtimeDataBuilder {
        ContextRealtimeDataBuilder {
            currentTime: currentTime,
            renderCapacity: renderCapacity,
            callbackIntervalMean: callbackIntervalMean,
            callbackIntervalVariance: callbackIntervalVariance,
        }
    }
    pub fn currentTime(&self) -> f64 { self.currentTime }
    pub fn renderCapacity(&self) -> f64 { self.renderCapacity }
    pub fn callbackIntervalMean(&self) -> f64 { self.callbackIntervalMean }
    pub fn callbackIntervalVariance(&self) -> f64 { self.callbackIntervalVariance }
}


pub struct ContextRealtimeDataBuilder {
    currentTime: f64,
    renderCapacity: f64,
    callbackIntervalMean: f64,
    callbackIntervalVariance: f64,
}

impl ContextRealtimeDataBuilder {
    pub fn build(self) -> ContextRealtimeData {
        ContextRealtimeData {
            currentTime: self.currentTime,
            renderCapacity: self.renderCapacity,
            callbackIntervalMean: self.callbackIntervalMean,
            callbackIntervalVariance: self.callbackIntervalVariance,
        }
    }
}

/// Protocol object for BaseAudioContext

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BaseAudioContext<'a> {
    contextId: GraphObjectId<'a>,
    contextType: ContextType,
    contextState: ContextState,
    #[serde(skip_serializing_if = "Option::is_none")]
    realtimeData: Option<ContextRealtimeData>,
    /// Platform-dependent callback buffer size.
    callbackBufferSize: f64,
    /// Number of output channels supported by audio hardware in use.
    maxOutputChannelCount: f64,
    /// Context sample rate.
    sampleRate: f64,
}

impl<'a> BaseAudioContext<'a> {
    pub fn builder(contextId: impl Into<GraphObjectId<'a>>, contextType: impl Into<ContextType>, contextState: impl Into<ContextState>, callbackBufferSize: f64, maxOutputChannelCount: f64, sampleRate: f64) -> BaseAudioContextBuilder<'a> {
        BaseAudioContextBuilder {
            contextId: contextId.into(),
            contextType: contextType.into(),
            contextState: contextState.into(),
            realtimeData: None,
            callbackBufferSize: callbackBufferSize,
            maxOutputChannelCount: maxOutputChannelCount,
            sampleRate: sampleRate,
        }
    }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
    pub fn contextType(&self) -> &ContextType { &self.contextType }
    pub fn contextState(&self) -> &ContextState { &self.contextState }
    pub fn realtimeData(&self) -> Option<&ContextRealtimeData> { self.realtimeData.as_ref() }
    pub fn callbackBufferSize(&self) -> f64 { self.callbackBufferSize }
    pub fn maxOutputChannelCount(&self) -> f64 { self.maxOutputChannelCount }
    pub fn sampleRate(&self) -> f64 { self.sampleRate }
}


pub struct BaseAudioContextBuilder<'a> {
    contextId: GraphObjectId<'a>,
    contextType: ContextType,
    contextState: ContextState,
    realtimeData: Option<ContextRealtimeData>,
    callbackBufferSize: f64,
    maxOutputChannelCount: f64,
    sampleRate: f64,
}

impl<'a> BaseAudioContextBuilder<'a> {
    pub fn realtimeData(mut self, realtimeData: ContextRealtimeData) -> Self { self.realtimeData = Some(realtimeData); self }
    pub fn build(self) -> BaseAudioContext<'a> {
        BaseAudioContext {
            contextId: self.contextId,
            contextType: self.contextType,
            contextState: self.contextState,
            realtimeData: self.realtimeData,
            callbackBufferSize: self.callbackBufferSize,
            maxOutputChannelCount: self.maxOutputChannelCount,
            sampleRate: self.sampleRate,
        }
    }
}

/// Protocol object for AudioListener

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioListener<'a> {
    listenerId: GraphObjectId<'a>,
    contextId: GraphObjectId<'a>,
}

impl<'a> AudioListener<'a> {
    pub fn builder(listenerId: impl Into<GraphObjectId<'a>>, contextId: impl Into<GraphObjectId<'a>>) -> AudioListenerBuilder<'a> {
        AudioListenerBuilder {
            listenerId: listenerId.into(),
            contextId: contextId.into(),
        }
    }
    pub fn listenerId(&self) -> &GraphObjectId<'a> { &self.listenerId }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
}


pub struct AudioListenerBuilder<'a> {
    listenerId: GraphObjectId<'a>,
    contextId: GraphObjectId<'a>,
}

impl<'a> AudioListenerBuilder<'a> {
    pub fn build(self) -> AudioListener<'a> {
        AudioListener {
            listenerId: self.listenerId,
            contextId: self.contextId,
        }
    }
}

/// Protocol object for AudioNode

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioNode<'a> {
    nodeId: GraphObjectId<'a>,
    contextId: GraphObjectId<'a>,
    nodeType: NodeType<'a>,
    numberOfInputs: f64,
    numberOfOutputs: f64,
    channelCount: f64,
    channelCountMode: ChannelCountMode,
    channelInterpretation: ChannelInterpretation,
}

impl<'a> AudioNode<'a> {
    pub fn builder(nodeId: impl Into<GraphObjectId<'a>>, contextId: impl Into<GraphObjectId<'a>>, nodeType: impl Into<NodeType<'a>>, numberOfInputs: f64, numberOfOutputs: f64, channelCount: f64, channelCountMode: impl Into<ChannelCountMode>, channelInterpretation: impl Into<ChannelInterpretation>) -> AudioNodeBuilder<'a> {
        AudioNodeBuilder {
            nodeId: nodeId.into(),
            contextId: contextId.into(),
            nodeType: nodeType.into(),
            numberOfInputs: numberOfInputs,
            numberOfOutputs: numberOfOutputs,
            channelCount: channelCount,
            channelCountMode: channelCountMode.into(),
            channelInterpretation: channelInterpretation.into(),
        }
    }
    pub fn nodeId(&self) -> &GraphObjectId<'a> { &self.nodeId }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
    pub fn nodeType(&self) -> &NodeType<'a> { &self.nodeType }
    pub fn numberOfInputs(&self) -> f64 { self.numberOfInputs }
    pub fn numberOfOutputs(&self) -> f64 { self.numberOfOutputs }
    pub fn channelCount(&self) -> f64 { self.channelCount }
    pub fn channelCountMode(&self) -> &ChannelCountMode { &self.channelCountMode }
    pub fn channelInterpretation(&self) -> &ChannelInterpretation { &self.channelInterpretation }
}


pub struct AudioNodeBuilder<'a> {
    nodeId: GraphObjectId<'a>,
    contextId: GraphObjectId<'a>,
    nodeType: NodeType<'a>,
    numberOfInputs: f64,
    numberOfOutputs: f64,
    channelCount: f64,
    channelCountMode: ChannelCountMode,
    channelInterpretation: ChannelInterpretation,
}

impl<'a> AudioNodeBuilder<'a> {
    pub fn build(self) -> AudioNode<'a> {
        AudioNode {
            nodeId: self.nodeId,
            contextId: self.contextId,
            nodeType: self.nodeType,
            numberOfInputs: self.numberOfInputs,
            numberOfOutputs: self.numberOfOutputs,
            channelCount: self.channelCount,
            channelCountMode: self.channelCountMode,
            channelInterpretation: self.channelInterpretation,
        }
    }
}

/// Protocol object for AudioParam

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioParam<'a> {
    paramId: GraphObjectId<'a>,
    nodeId: GraphObjectId<'a>,
    contextId: GraphObjectId<'a>,
    paramType: ParamType<'a>,
    rate: AutomationRate,
    defaultValue: f64,
    minValue: f64,
    maxValue: f64,
}

impl<'a> AudioParam<'a> {
    pub fn builder(paramId: impl Into<GraphObjectId<'a>>, nodeId: impl Into<GraphObjectId<'a>>, contextId: impl Into<GraphObjectId<'a>>, paramType: impl Into<ParamType<'a>>, rate: impl Into<AutomationRate>, defaultValue: f64, minValue: f64, maxValue: f64) -> AudioParamBuilder<'a> {
        AudioParamBuilder {
            paramId: paramId.into(),
            nodeId: nodeId.into(),
            contextId: contextId.into(),
            paramType: paramType.into(),
            rate: rate.into(),
            defaultValue: defaultValue,
            minValue: minValue,
            maxValue: maxValue,
        }
    }
    pub fn paramId(&self) -> &GraphObjectId<'a> { &self.paramId }
    pub fn nodeId(&self) -> &GraphObjectId<'a> { &self.nodeId }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
    pub fn paramType(&self) -> &ParamType<'a> { &self.paramType }
    pub fn rate(&self) -> &AutomationRate { &self.rate }
    pub fn defaultValue(&self) -> f64 { self.defaultValue }
    pub fn minValue(&self) -> f64 { self.minValue }
    pub fn maxValue(&self) -> f64 { self.maxValue }
}


pub struct AudioParamBuilder<'a> {
    paramId: GraphObjectId<'a>,
    nodeId: GraphObjectId<'a>,
    contextId: GraphObjectId<'a>,
    paramType: ParamType<'a>,
    rate: AutomationRate,
    defaultValue: f64,
    minValue: f64,
    maxValue: f64,
}

impl<'a> AudioParamBuilder<'a> {
    pub fn build(self) -> AudioParam<'a> {
        AudioParam {
            paramId: self.paramId,
            nodeId: self.nodeId,
            contextId: self.contextId,
            paramType: self.paramType,
            rate: self.rate,
            defaultValue: self.defaultValue,
            minValue: self.minValue,
            maxValue: self.maxValue,
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
    contextId: GraphObjectId<'a>,
}

impl<'a> GetRealtimeDataParams<'a> {
    pub fn builder(contextId: impl Into<GraphObjectId<'a>>) -> GetRealtimeDataParamsBuilder<'a> {
        GetRealtimeDataParamsBuilder {
            contextId: contextId.into(),
        }
    }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
}


pub struct GetRealtimeDataParamsBuilder<'a> {
    contextId: GraphObjectId<'a>,
}

impl<'a> GetRealtimeDataParamsBuilder<'a> {
    pub fn build(self) -> GetRealtimeDataParams<'a> {
        GetRealtimeDataParams {
            contextId: self.contextId,
        }
    }
}

/// Fetch the realtime data from the registered contexts.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRealtimeDataReturns {
    realtimeData: ContextRealtimeData,
}

impl GetRealtimeDataReturns {
    pub fn builder(realtimeData: ContextRealtimeData) -> GetRealtimeDataReturnsBuilder {
        GetRealtimeDataReturnsBuilder {
            realtimeData: realtimeData,
        }
    }
    pub fn realtimeData(&self) -> &ContextRealtimeData { &self.realtimeData }
}


pub struct GetRealtimeDataReturnsBuilder {
    realtimeData: ContextRealtimeData,
}

impl GetRealtimeDataReturnsBuilder {
    pub fn build(self) -> GetRealtimeDataReturns {
        GetRealtimeDataReturns {
            realtimeData: self.realtimeData,
        }
    }
}

impl<'a> GetRealtimeDataParams<'a> { pub const METHOD: &'static str = "WebAudio.getRealtimeData"; }

impl<'a> crate::CdpCommand<'a> for GetRealtimeDataParams<'a> {
    const METHOD: &'static str = "WebAudio.getRealtimeData";
    type Response = GetRealtimeDataReturns;
}
