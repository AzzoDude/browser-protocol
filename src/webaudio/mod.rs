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
    pub fn builder() -> ContextRealtimeDataBuilder { ContextRealtimeDataBuilder::default() }
    pub fn currentTime(&self) -> f64 { self.currentTime }
    pub fn renderCapacity(&self) -> f64 { self.renderCapacity }
    pub fn callbackIntervalMean(&self) -> f64 { self.callbackIntervalMean }
    pub fn callbackIntervalVariance(&self) -> f64 { self.callbackIntervalVariance }
}

#[derive(Default)]
pub struct ContextRealtimeDataBuilder {
    currentTime: Option<f64>,
    renderCapacity: Option<f64>,
    callbackIntervalMean: Option<f64>,
    callbackIntervalVariance: Option<f64>,
}

impl ContextRealtimeDataBuilder {
    /// The current context time in second in BaseAudioContext.
    pub fn currentTime(mut self, currentTime: f64) -> Self { self.currentTime = Some(currentTime); self }
    /// The time spent on rendering graph divided by render quantum duration,
    /// and multiplied by 100. 100 means the audio renderer reached the full
    /// capacity and glitch may occur.
    pub fn renderCapacity(mut self, renderCapacity: f64) -> Self { self.renderCapacity = Some(renderCapacity); self }
    /// A running mean of callback interval.
    pub fn callbackIntervalMean(mut self, callbackIntervalMean: f64) -> Self { self.callbackIntervalMean = Some(callbackIntervalMean); self }
    /// A running variance of callback interval.
    pub fn callbackIntervalVariance(mut self, callbackIntervalVariance: f64) -> Self { self.callbackIntervalVariance = Some(callbackIntervalVariance); self }
    pub fn build(self) -> ContextRealtimeData {
        ContextRealtimeData {
            currentTime: self.currentTime.unwrap_or_default(),
            renderCapacity: self.renderCapacity.unwrap_or_default(),
            callbackIntervalMean: self.callbackIntervalMean.unwrap_or_default(),
            callbackIntervalVariance: self.callbackIntervalVariance.unwrap_or_default(),
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
    pub fn builder() -> BaseAudioContextBuilder<'a> { BaseAudioContextBuilder::default() }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
    pub fn contextType(&self) -> &ContextType { &self.contextType }
    pub fn contextState(&self) -> &ContextState { &self.contextState }
    pub fn realtimeData(&self) -> Option<&ContextRealtimeData> { self.realtimeData.as_ref() }
    pub fn callbackBufferSize(&self) -> f64 { self.callbackBufferSize }
    pub fn maxOutputChannelCount(&self) -> f64 { self.maxOutputChannelCount }
    pub fn sampleRate(&self) -> f64 { self.sampleRate }
}

#[derive(Default)]
pub struct BaseAudioContextBuilder<'a> {
    contextId: Option<GraphObjectId<'a>>,
    contextType: Option<ContextType>,
    contextState: Option<ContextState>,
    realtimeData: Option<ContextRealtimeData>,
    callbackBufferSize: Option<f64>,
    maxOutputChannelCount: Option<f64>,
    sampleRate: Option<f64>,
}

impl<'a> BaseAudioContextBuilder<'a> {
    pub fn contextId(mut self, contextId: GraphObjectId<'a>) -> Self { self.contextId = Some(contextId); self }
    pub fn contextType(mut self, contextType: ContextType) -> Self { self.contextType = Some(contextType); self }
    pub fn contextState(mut self, contextState: ContextState) -> Self { self.contextState = Some(contextState); self }
    pub fn realtimeData(mut self, realtimeData: ContextRealtimeData) -> Self { self.realtimeData = Some(realtimeData); self }
    /// Platform-dependent callback buffer size.
    pub fn callbackBufferSize(mut self, callbackBufferSize: f64) -> Self { self.callbackBufferSize = Some(callbackBufferSize); self }
    /// Number of output channels supported by audio hardware in use.
    pub fn maxOutputChannelCount(mut self, maxOutputChannelCount: f64) -> Self { self.maxOutputChannelCount = Some(maxOutputChannelCount); self }
    /// Context sample rate.
    pub fn sampleRate(mut self, sampleRate: f64) -> Self { self.sampleRate = Some(sampleRate); self }
    pub fn build(self) -> BaseAudioContext<'a> {
        BaseAudioContext {
            contextId: self.contextId.unwrap_or_default(),
            contextType: self.contextType.unwrap_or_default(),
            contextState: self.contextState.unwrap_or_default(),
            realtimeData: self.realtimeData,
            callbackBufferSize: self.callbackBufferSize.unwrap_or_default(),
            maxOutputChannelCount: self.maxOutputChannelCount.unwrap_or_default(),
            sampleRate: self.sampleRate.unwrap_or_default(),
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
    pub fn builder() -> AudioListenerBuilder<'a> { AudioListenerBuilder::default() }
    pub fn listenerId(&self) -> &GraphObjectId<'a> { &self.listenerId }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
}

#[derive(Default)]
pub struct AudioListenerBuilder<'a> {
    listenerId: Option<GraphObjectId<'a>>,
    contextId: Option<GraphObjectId<'a>>,
}

impl<'a> AudioListenerBuilder<'a> {
    pub fn listenerId(mut self, listenerId: GraphObjectId<'a>) -> Self { self.listenerId = Some(listenerId); self }
    pub fn contextId(mut self, contextId: GraphObjectId<'a>) -> Self { self.contextId = Some(contextId); self }
    pub fn build(self) -> AudioListener<'a> {
        AudioListener {
            listenerId: self.listenerId.unwrap_or_default(),
            contextId: self.contextId.unwrap_or_default(),
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
    pub fn builder() -> AudioNodeBuilder<'a> { AudioNodeBuilder::default() }
    pub fn nodeId(&self) -> &GraphObjectId<'a> { &self.nodeId }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
    pub fn nodeType(&self) -> &NodeType<'a> { &self.nodeType }
    pub fn numberOfInputs(&self) -> f64 { self.numberOfInputs }
    pub fn numberOfOutputs(&self) -> f64 { self.numberOfOutputs }
    pub fn channelCount(&self) -> f64 { self.channelCount }
    pub fn channelCountMode(&self) -> &ChannelCountMode { &self.channelCountMode }
    pub fn channelInterpretation(&self) -> &ChannelInterpretation { &self.channelInterpretation }
}

#[derive(Default)]
pub struct AudioNodeBuilder<'a> {
    nodeId: Option<GraphObjectId<'a>>,
    contextId: Option<GraphObjectId<'a>>,
    nodeType: Option<NodeType<'a>>,
    numberOfInputs: Option<f64>,
    numberOfOutputs: Option<f64>,
    channelCount: Option<f64>,
    channelCountMode: Option<ChannelCountMode>,
    channelInterpretation: Option<ChannelInterpretation>,
}

impl<'a> AudioNodeBuilder<'a> {
    pub fn nodeId(mut self, nodeId: GraphObjectId<'a>) -> Self { self.nodeId = Some(nodeId); self }
    pub fn contextId(mut self, contextId: GraphObjectId<'a>) -> Self { self.contextId = Some(contextId); self }
    pub fn nodeType(mut self, nodeType: NodeType<'a>) -> Self { self.nodeType = Some(nodeType); self }
    pub fn numberOfInputs(mut self, numberOfInputs: f64) -> Self { self.numberOfInputs = Some(numberOfInputs); self }
    pub fn numberOfOutputs(mut self, numberOfOutputs: f64) -> Self { self.numberOfOutputs = Some(numberOfOutputs); self }
    pub fn channelCount(mut self, channelCount: f64) -> Self { self.channelCount = Some(channelCount); self }
    pub fn channelCountMode(mut self, channelCountMode: ChannelCountMode) -> Self { self.channelCountMode = Some(channelCountMode); self }
    pub fn channelInterpretation(mut self, channelInterpretation: ChannelInterpretation) -> Self { self.channelInterpretation = Some(channelInterpretation); self }
    pub fn build(self) -> AudioNode<'a> {
        AudioNode {
            nodeId: self.nodeId.unwrap_or_default(),
            contextId: self.contextId.unwrap_or_default(),
            nodeType: self.nodeType.unwrap_or_default(),
            numberOfInputs: self.numberOfInputs.unwrap_or_default(),
            numberOfOutputs: self.numberOfOutputs.unwrap_or_default(),
            channelCount: self.channelCount.unwrap_or_default(),
            channelCountMode: self.channelCountMode.unwrap_or_default(),
            channelInterpretation: self.channelInterpretation.unwrap_or_default(),
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
    pub fn builder() -> AudioParamBuilder<'a> { AudioParamBuilder::default() }
    pub fn paramId(&self) -> &GraphObjectId<'a> { &self.paramId }
    pub fn nodeId(&self) -> &GraphObjectId<'a> { &self.nodeId }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
    pub fn paramType(&self) -> &ParamType<'a> { &self.paramType }
    pub fn rate(&self) -> &AutomationRate { &self.rate }
    pub fn defaultValue(&self) -> f64 { self.defaultValue }
    pub fn minValue(&self) -> f64 { self.minValue }
    pub fn maxValue(&self) -> f64 { self.maxValue }
}

#[derive(Default)]
pub struct AudioParamBuilder<'a> {
    paramId: Option<GraphObjectId<'a>>,
    nodeId: Option<GraphObjectId<'a>>,
    contextId: Option<GraphObjectId<'a>>,
    paramType: Option<ParamType<'a>>,
    rate: Option<AutomationRate>,
    defaultValue: Option<f64>,
    minValue: Option<f64>,
    maxValue: Option<f64>,
}

impl<'a> AudioParamBuilder<'a> {
    pub fn paramId(mut self, paramId: GraphObjectId<'a>) -> Self { self.paramId = Some(paramId); self }
    pub fn nodeId(mut self, nodeId: GraphObjectId<'a>) -> Self { self.nodeId = Some(nodeId); self }
    pub fn contextId(mut self, contextId: GraphObjectId<'a>) -> Self { self.contextId = Some(contextId); self }
    pub fn paramType(mut self, paramType: ParamType<'a>) -> Self { self.paramType = Some(paramType); self }
    pub fn rate(mut self, rate: AutomationRate) -> Self { self.rate = Some(rate); self }
    pub fn defaultValue(mut self, defaultValue: f64) -> Self { self.defaultValue = Some(defaultValue); self }
    pub fn minValue(mut self, minValue: f64) -> Self { self.minValue = Some(minValue); self }
    pub fn maxValue(mut self, maxValue: f64) -> Self { self.maxValue = Some(maxValue); self }
    pub fn build(self) -> AudioParam<'a> {
        AudioParam {
            paramId: self.paramId.unwrap_or_default(),
            nodeId: self.nodeId.unwrap_or_default(),
            contextId: self.contextId.unwrap_or_default(),
            paramType: self.paramType.unwrap_or_default(),
            rate: self.rate.unwrap_or_default(),
            defaultValue: self.defaultValue.unwrap_or_default(),
            minValue: self.minValue.unwrap_or_default(),
            maxValue: self.maxValue.unwrap_or_default(),
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

impl EnableParams { pub const METHOD: &'static str = "WebAudio.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "WebAudio.enable";
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
    pub fn builder() -> GetRealtimeDataParamsBuilder<'a> { GetRealtimeDataParamsBuilder::default() }
    pub fn contextId(&self) -> &GraphObjectId<'a> { &self.contextId }
}

#[derive(Default)]
pub struct GetRealtimeDataParamsBuilder<'a> {
    contextId: Option<GraphObjectId<'a>>,
}

impl<'a> GetRealtimeDataParamsBuilder<'a> {
    pub fn contextId(mut self, contextId: GraphObjectId<'a>) -> Self { self.contextId = Some(contextId); self }
    pub fn build(self) -> GetRealtimeDataParams<'a> {
        GetRealtimeDataParams {
            contextId: self.contextId.unwrap_or_default(),
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
    pub fn builder() -> GetRealtimeDataReturnsBuilder { GetRealtimeDataReturnsBuilder::default() }
    pub fn realtimeData(&self) -> &ContextRealtimeData { &self.realtimeData }
}

#[derive(Default)]
pub struct GetRealtimeDataReturnsBuilder {
    realtimeData: Option<ContextRealtimeData>,
}

impl GetRealtimeDataReturnsBuilder {
    pub fn realtimeData(mut self, realtimeData: ContextRealtimeData) -> Self { self.realtimeData = Some(realtimeData); self }
    pub fn build(self) -> GetRealtimeDataReturns {
        GetRealtimeDataReturns {
            realtimeData: self.realtimeData.unwrap_or_default(),
        }
    }
}

impl<'a> GetRealtimeDataParams<'a> { pub const METHOD: &'static str = "WebAudio.getRealtimeData"; }

impl<'a> crate::CdpCommand<'a> for GetRealtimeDataParams<'a> {
    const METHOD: &'static str = "WebAudio.getRealtimeData";
    type Response = GetRealtimeDataReturns;
}
