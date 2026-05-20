use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Animation instance.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Animation<'a> {
    /// 'Animation''s id.
    id: Cow<'a, str>,
    /// 'Animation''s name.
    name: Cow<'a, str>,
    /// 'Animation''s internal paused state.
    pausedState: bool,
    /// 'Animation''s play state.
    playState: Cow<'a, str>,
    /// 'Animation''s playback rate.
    playbackRate: f64,
    /// 'Animation''s start time.
    /// Milliseconds for time based animations and
    /// percentage [0 - 100] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    startTime: f64,
    /// 'Animation''s current time.
    currentTime: f64,
    /// Animation type of 'Animation'.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// 'Animation''s source animation node.
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<AnimationEffect<'a>>,
    /// A unique ID for 'Animation' representing the sources that triggered this CSS
    /// animation/transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    cssId: Option<Cow<'a, str>>,
    /// View or scroll timeline
    #[serde(skip_serializing_if = "Option::is_none")]
    viewOrScrollTimeline: Option<ViewOrScrollTimeline>,
}

impl<'a> Animation<'a> {
    pub fn builder() -> AnimationBuilder<'a> { AnimationBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn pausedState(&self) -> bool { self.pausedState }
    pub fn playState(&self) -> &str { self.playState.as_ref() }
    pub fn playbackRate(&self) -> f64 { self.playbackRate }
    pub fn startTime(&self) -> f64 { self.startTime }
    pub fn currentTime(&self) -> f64 { self.currentTime }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn source(&self) -> Option<&AnimationEffect<'a>> { self.source.as_ref() }
    pub fn cssId(&self) -> Option<&str> { self.cssId.as_deref() }
    pub fn viewOrScrollTimeline(&self) -> Option<&ViewOrScrollTimeline> { self.viewOrScrollTimeline.as_ref() }
}

#[derive(Default)]
pub struct AnimationBuilder<'a> {
    id: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    pausedState: Option<bool>,
    playState: Option<Cow<'a, str>>,
    playbackRate: Option<f64>,
    startTime: Option<f64>,
    currentTime: Option<f64>,
    type_: Option<Cow<'a, str>>,
    source: Option<AnimationEffect<'a>>,
    cssId: Option<Cow<'a, str>>,
    viewOrScrollTimeline: Option<ViewOrScrollTimeline>,
}

impl<'a> AnimationBuilder<'a> {
    /// 'Animation''s id.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    /// 'Animation''s name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// 'Animation''s internal paused state.
    pub fn pausedState(mut self, pausedState: bool) -> Self { self.pausedState = Some(pausedState); self }
    /// 'Animation''s play state.
    pub fn playState(mut self, playState: impl Into<Cow<'a, str>>) -> Self { self.playState = Some(playState.into()); self }
    /// 'Animation''s playback rate.
    pub fn playbackRate(mut self, playbackRate: f64) -> Self { self.playbackRate = Some(playbackRate); self }
    /// 'Animation''s start time.
    /// Milliseconds for time based animations and
    /// percentage [0 - 100] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    pub fn startTime(mut self, startTime: f64) -> Self { self.startTime = Some(startTime); self }
    /// 'Animation''s current time.
    pub fn currentTime(mut self, currentTime: f64) -> Self { self.currentTime = Some(currentTime); self }
    /// Animation type of 'Animation'.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    /// 'Animation''s source animation node.
    pub fn source(mut self, source: AnimationEffect<'a>) -> Self { self.source = Some(source); self }
    /// A unique ID for 'Animation' representing the sources that triggered this CSS
    /// animation/transition.
    pub fn cssId(mut self, cssId: impl Into<Cow<'a, str>>) -> Self { self.cssId = Some(cssId.into()); self }
    /// View or scroll timeline
    pub fn viewOrScrollTimeline(mut self, viewOrScrollTimeline: ViewOrScrollTimeline) -> Self { self.viewOrScrollTimeline = Some(viewOrScrollTimeline); self }
    pub fn build(self) -> Animation<'a> {
        Animation {
            id: self.id.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            pausedState: self.pausedState.unwrap_or_default(),
            playState: self.playState.unwrap_or_default(),
            playbackRate: self.playbackRate.unwrap_or_default(),
            startTime: self.startTime.unwrap_or_default(),
            currentTime: self.currentTime.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
            source: self.source,
            cssId: self.cssId,
            viewOrScrollTimeline: self.viewOrScrollTimeline,
        }
    }
}

/// Timeline instance

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViewOrScrollTimeline {
    /// Scroll container node
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceNodeId: Option<crate::dom::BackendNodeId>,
    /// Represents the starting scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    startOffset: Option<f64>,
    /// Represents the ending scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    endOffset: Option<f64>,
    /// The element whose principal box's visibility in the
    /// scrollport defined the progress of the timeline.
    /// Does not exist for animations with ScrollTimeline
    #[serde(skip_serializing_if = "Option::is_none")]
    subjectNodeId: Option<crate::dom::BackendNodeId>,
    /// Orientation of the scroll
    axis: crate::dom::ScrollOrientation,
}

impl ViewOrScrollTimeline {
    pub fn builder() -> ViewOrScrollTimelineBuilder { ViewOrScrollTimelineBuilder::default() }
    pub fn sourceNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.sourceNodeId.as_ref() }
    pub fn startOffset(&self) -> Option<f64> { self.startOffset }
    pub fn endOffset(&self) -> Option<f64> { self.endOffset }
    pub fn subjectNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.subjectNodeId.as_ref() }
    pub fn axis(&self) -> &crate::dom::ScrollOrientation { &self.axis }
}

#[derive(Default)]
pub struct ViewOrScrollTimelineBuilder {
    sourceNodeId: Option<crate::dom::BackendNodeId>,
    startOffset: Option<f64>,
    endOffset: Option<f64>,
    subjectNodeId: Option<crate::dom::BackendNodeId>,
    axis: Option<crate::dom::ScrollOrientation>,
}

impl ViewOrScrollTimelineBuilder {
    /// Scroll container node
    pub fn sourceNodeId(mut self, sourceNodeId: crate::dom::BackendNodeId) -> Self { self.sourceNodeId = Some(sourceNodeId); self }
    /// Represents the starting scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    pub fn startOffset(mut self, startOffset: f64) -> Self { self.startOffset = Some(startOffset); self }
    /// Represents the ending scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    pub fn endOffset(mut self, endOffset: f64) -> Self { self.endOffset = Some(endOffset); self }
    /// The element whose principal box's visibility in the
    /// scrollport defined the progress of the timeline.
    /// Does not exist for animations with ScrollTimeline
    pub fn subjectNodeId(mut self, subjectNodeId: crate::dom::BackendNodeId) -> Self { self.subjectNodeId = Some(subjectNodeId); self }
    /// Orientation of the scroll
    pub fn axis(mut self, axis: crate::dom::ScrollOrientation) -> Self { self.axis = Some(axis); self }
    pub fn build(self) -> ViewOrScrollTimeline {
        ViewOrScrollTimeline {
            sourceNodeId: self.sourceNodeId,
            startOffset: self.startOffset,
            endOffset: self.endOffset,
            subjectNodeId: self.subjectNodeId,
            axis: self.axis.unwrap_or_default(),
        }
    }
}

/// AnimationEffect instance

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AnimationEffect<'a> {
    /// 'AnimationEffect''s delay.
    delay: f64,
    /// 'AnimationEffect''s end delay.
    endDelay: f64,
    /// 'AnimationEffect''s iteration start.
    iterationStart: f64,
    /// 'AnimationEffect''s iterations. Omitted if the value is infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    iterations: Option<f64>,
    /// 'AnimationEffect''s iteration duration.
    /// Milliseconds for time based animations and
    /// percentage [0 - 100] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    duration: f64,
    /// 'AnimationEffect''s playback direction.
    direction: Cow<'a, str>,
    /// 'AnimationEffect''s fill mode.
    fill: Cow<'a, str>,
    /// 'AnimationEffect''s target node.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
    /// 'AnimationEffect''s keyframes.
    #[serde(skip_serializing_if = "Option::is_none")]
    keyframesRule: Option<KeyframesRule<'a>>,
    /// 'AnimationEffect''s timing function.
    easing: Cow<'a, str>,
}

impl<'a> AnimationEffect<'a> {
    pub fn builder() -> AnimationEffectBuilder<'a> { AnimationEffectBuilder::default() }
    pub fn delay(&self) -> f64 { self.delay }
    pub fn endDelay(&self) -> f64 { self.endDelay }
    pub fn iterationStart(&self) -> f64 { self.iterationStart }
    pub fn iterations(&self) -> Option<f64> { self.iterations }
    pub fn duration(&self) -> f64 { self.duration }
    pub fn direction(&self) -> &str { self.direction.as_ref() }
    pub fn fill(&self) -> &str { self.fill.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn keyframesRule(&self) -> Option<&KeyframesRule<'a>> { self.keyframesRule.as_ref() }
    pub fn easing(&self) -> &str { self.easing.as_ref() }
}

#[derive(Default)]
pub struct AnimationEffectBuilder<'a> {
    delay: Option<f64>,
    endDelay: Option<f64>,
    iterationStart: Option<f64>,
    iterations: Option<f64>,
    duration: Option<f64>,
    direction: Option<Cow<'a, str>>,
    fill: Option<Cow<'a, str>>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    keyframesRule: Option<KeyframesRule<'a>>,
    easing: Option<Cow<'a, str>>,
}

impl<'a> AnimationEffectBuilder<'a> {
    /// 'AnimationEffect''s delay.
    pub fn delay(mut self, delay: f64) -> Self { self.delay = Some(delay); self }
    /// 'AnimationEffect''s end delay.
    pub fn endDelay(mut self, endDelay: f64) -> Self { self.endDelay = Some(endDelay); self }
    /// 'AnimationEffect''s iteration start.
    pub fn iterationStart(mut self, iterationStart: f64) -> Self { self.iterationStart = Some(iterationStart); self }
    /// 'AnimationEffect''s iterations. Omitted if the value is infinite.
    pub fn iterations(mut self, iterations: f64) -> Self { self.iterations = Some(iterations); self }
    /// 'AnimationEffect''s iteration duration.
    /// Milliseconds for time based animations and
    /// percentage [0 - 100] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    pub fn duration(mut self, duration: f64) -> Self { self.duration = Some(duration); self }
    /// 'AnimationEffect''s playback direction.
    pub fn direction(mut self, direction: impl Into<Cow<'a, str>>) -> Self { self.direction = Some(direction.into()); self }
    /// 'AnimationEffect''s fill mode.
    pub fn fill(mut self, fill: impl Into<Cow<'a, str>>) -> Self { self.fill = Some(fill.into()); self }
    /// 'AnimationEffect''s target node.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// 'AnimationEffect''s keyframes.
    pub fn keyframesRule(mut self, keyframesRule: KeyframesRule<'a>) -> Self { self.keyframesRule = Some(keyframesRule); self }
    /// 'AnimationEffect''s timing function.
    pub fn easing(mut self, easing: impl Into<Cow<'a, str>>) -> Self { self.easing = Some(easing.into()); self }
    pub fn build(self) -> AnimationEffect<'a> {
        AnimationEffect {
            delay: self.delay.unwrap_or_default(),
            endDelay: self.endDelay.unwrap_or_default(),
            iterationStart: self.iterationStart.unwrap_or_default(),
            iterations: self.iterations,
            duration: self.duration.unwrap_or_default(),
            direction: self.direction.unwrap_or_default(),
            fill: self.fill.unwrap_or_default(),
            backendNodeId: self.backendNodeId,
            keyframesRule: self.keyframesRule,
            easing: self.easing.unwrap_or_default(),
        }
    }
}

/// Keyframes Rule

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyframesRule<'a> {
    /// CSS keyframed animation's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    /// List of animation keyframes.
    keyframes: Vec<KeyframeStyle<'a>>,
}

impl<'a> KeyframesRule<'a> {
    pub fn builder() -> KeyframesRuleBuilder<'a> { KeyframesRuleBuilder::default() }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn keyframes(&self) -> &[KeyframeStyle<'a>] { &self.keyframes }
}

#[derive(Default)]
pub struct KeyframesRuleBuilder<'a> {
    name: Option<Cow<'a, str>>,
    keyframes: Option<Vec<KeyframeStyle<'a>>>,
}

impl<'a> KeyframesRuleBuilder<'a> {
    /// CSS keyframed animation's name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// List of animation keyframes.
    pub fn keyframes(mut self, keyframes: Vec<KeyframeStyle<'a>>) -> Self { self.keyframes = Some(keyframes); self }
    pub fn build(self) -> KeyframesRule<'a> {
        KeyframesRule {
            name: self.name,
            keyframes: self.keyframes.unwrap_or_default(),
        }
    }
}

/// Keyframe Style

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyframeStyle<'a> {
    /// Keyframe's time offset.
    offset: Cow<'a, str>,
    /// 'AnimationEffect''s timing function.
    easing: Cow<'a, str>,
}

impl<'a> KeyframeStyle<'a> {
    pub fn builder() -> KeyframeStyleBuilder<'a> { KeyframeStyleBuilder::default() }
    pub fn offset(&self) -> &str { self.offset.as_ref() }
    pub fn easing(&self) -> &str { self.easing.as_ref() }
}

#[derive(Default)]
pub struct KeyframeStyleBuilder<'a> {
    offset: Option<Cow<'a, str>>,
    easing: Option<Cow<'a, str>>,
}

impl<'a> KeyframeStyleBuilder<'a> {
    /// Keyframe's time offset.
    pub fn offset(mut self, offset: impl Into<Cow<'a, str>>) -> Self { self.offset = Some(offset.into()); self }
    /// 'AnimationEffect''s timing function.
    pub fn easing(mut self, easing: impl Into<Cow<'a, str>>) -> Self { self.easing = Some(easing.into()); self }
    pub fn build(self) -> KeyframeStyle<'a> {
        KeyframeStyle {
            offset: self.offset.unwrap_or_default(),
            easing: self.easing.unwrap_or_default(),
        }
    }
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

impl DisableParams { pub const METHOD: &'static str = "Animation.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Animation.disable";
    type Response = crate::EmptyReturns;
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

impl EnableParams { pub const METHOD: &'static str = "Animation.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Animation.enable";
    type Response = crate::EmptyReturns;
}

/// Returns the current time of the an animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentTimeParams<'a> {
    /// Id of animation.
    id: Cow<'a, str>,
}

impl<'a> GetCurrentTimeParams<'a> {
    pub fn builder() -> GetCurrentTimeParamsBuilder<'a> { GetCurrentTimeParamsBuilder::default() }
    pub fn id(&self) -> &str { self.id.as_ref() }
}

#[derive(Default)]
pub struct GetCurrentTimeParamsBuilder<'a> {
    id: Option<Cow<'a, str>>,
}

impl<'a> GetCurrentTimeParamsBuilder<'a> {
    /// Id of animation.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    pub fn build(self) -> GetCurrentTimeParams<'a> {
        GetCurrentTimeParams {
            id: self.id.unwrap_or_default(),
        }
    }
}

/// Returns the current time of the an animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentTimeReturns {
    /// Current time of the page.
    currentTime: f64,
}

impl GetCurrentTimeReturns {
    pub fn builder() -> GetCurrentTimeReturnsBuilder { GetCurrentTimeReturnsBuilder::default() }
    pub fn currentTime(&self) -> f64 { self.currentTime }
}

#[derive(Default)]
pub struct GetCurrentTimeReturnsBuilder {
    currentTime: Option<f64>,
}

impl GetCurrentTimeReturnsBuilder {
    /// Current time of the page.
    pub fn currentTime(mut self, currentTime: f64) -> Self { self.currentTime = Some(currentTime); self }
    pub fn build(self) -> GetCurrentTimeReturns {
        GetCurrentTimeReturns {
            currentTime: self.currentTime.unwrap_or_default(),
        }
    }
}

impl<'a> GetCurrentTimeParams<'a> { pub const METHOD: &'static str = "Animation.getCurrentTime"; }

impl<'a> crate::CdpCommand<'a> for GetCurrentTimeParams<'a> {
    const METHOD: &'static str = "Animation.getCurrentTime";
    type Response = GetCurrentTimeReturns;
}

/// Gets the playback rate of the document timeline.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaybackRateReturns {
    /// Playback rate for animations on page.
    playbackRate: f64,
}

impl GetPlaybackRateReturns {
    pub fn builder() -> GetPlaybackRateReturnsBuilder { GetPlaybackRateReturnsBuilder::default() }
    pub fn playbackRate(&self) -> f64 { self.playbackRate }
}

#[derive(Default)]
pub struct GetPlaybackRateReturnsBuilder {
    playbackRate: Option<f64>,
}

impl GetPlaybackRateReturnsBuilder {
    /// Playback rate for animations on page.
    pub fn playbackRate(mut self, playbackRate: f64) -> Self { self.playbackRate = Some(playbackRate); self }
    pub fn build(self) -> GetPlaybackRateReturns {
        GetPlaybackRateReturns {
            playbackRate: self.playbackRate.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetPlaybackRateParams {}

impl GetPlaybackRateParams {
    pub fn builder() -> GetPlaybackRateParamsBuilder {
        GetPlaybackRateParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetPlaybackRateParamsBuilder {}

impl GetPlaybackRateParamsBuilder {
    pub fn build(self) -> GetPlaybackRateParams {
        GetPlaybackRateParams {}
    }
}

impl GetPlaybackRateParams { pub const METHOD: &'static str = "Animation.getPlaybackRate"; }

impl<'a> crate::CdpCommand<'a> for GetPlaybackRateParams {
    const METHOD: &'static str = "Animation.getPlaybackRate";
    type Response = GetPlaybackRateReturns;
}

/// Releases a set of animations to no longer be manipulated.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAnimationsParams<'a> {
    /// List of animation ids to seek.
    animations: Vec<Cow<'a, str>>,
}

impl<'a> ReleaseAnimationsParams<'a> {
    pub fn builder() -> ReleaseAnimationsParamsBuilder<'a> { ReleaseAnimationsParamsBuilder::default() }
    pub fn animations(&self) -> &[Cow<'a, str>] { &self.animations }
}

#[derive(Default)]
pub struct ReleaseAnimationsParamsBuilder<'a> {
    animations: Option<Vec<Cow<'a, str>>>,
}

impl<'a> ReleaseAnimationsParamsBuilder<'a> {
    /// List of animation ids to seek.
    pub fn animations(mut self, animations: Vec<Cow<'a, str>>) -> Self { self.animations = Some(animations); self }
    pub fn build(self) -> ReleaseAnimationsParams<'a> {
        ReleaseAnimationsParams {
            animations: self.animations.unwrap_or_default(),
        }
    }
}

impl<'a> ReleaseAnimationsParams<'a> { pub const METHOD: &'static str = "Animation.releaseAnimations"; }

impl<'a> crate::CdpCommand<'a> for ReleaseAnimationsParams<'a> {
    const METHOD: &'static str = "Animation.releaseAnimations";
    type Response = crate::EmptyReturns;
}

/// Gets the remote object of the Animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveAnimationParams<'a> {
    /// Animation id.
    animationId: Cow<'a, str>,
}

impl<'a> ResolveAnimationParams<'a> {
    pub fn builder() -> ResolveAnimationParamsBuilder<'a> { ResolveAnimationParamsBuilder::default() }
    pub fn animationId(&self) -> &str { self.animationId.as_ref() }
}

#[derive(Default)]
pub struct ResolveAnimationParamsBuilder<'a> {
    animationId: Option<Cow<'a, str>>,
}

impl<'a> ResolveAnimationParamsBuilder<'a> {
    /// Animation id.
    pub fn animationId(mut self, animationId: impl Into<Cow<'a, str>>) -> Self { self.animationId = Some(animationId.into()); self }
    pub fn build(self) -> ResolveAnimationParams<'a> {
        ResolveAnimationParams {
            animationId: self.animationId.unwrap_or_default(),
        }
    }
}

/// Gets the remote object of the Animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveAnimationReturns {
    /// Corresponding remote object.
    remoteObject: crate::runtime::RemoteObject,
}

impl ResolveAnimationReturns {
    pub fn builder() -> ResolveAnimationReturnsBuilder { ResolveAnimationReturnsBuilder::default() }
    pub fn remoteObject(&self) -> &crate::runtime::RemoteObject { &self.remoteObject }
}

#[derive(Default)]
pub struct ResolveAnimationReturnsBuilder {
    remoteObject: Option<crate::runtime::RemoteObject>,
}

impl ResolveAnimationReturnsBuilder {
    /// Corresponding remote object.
    pub fn remoteObject(mut self, remoteObject: crate::runtime::RemoteObject) -> Self { self.remoteObject = Some(remoteObject); self }
    pub fn build(self) -> ResolveAnimationReturns {
        ResolveAnimationReturns {
            remoteObject: self.remoteObject.unwrap_or_default(),
        }
    }
}

impl<'a> ResolveAnimationParams<'a> { pub const METHOD: &'static str = "Animation.resolveAnimation"; }

impl<'a> crate::CdpCommand<'a> for ResolveAnimationParams<'a> {
    const METHOD: &'static str = "Animation.resolveAnimation";
    type Response = ResolveAnimationReturns;
}

/// Seek a set of animations to a particular time within each animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SeekAnimationsParams<'a> {
    /// List of animation ids to seek.
    animations: Vec<Cow<'a, str>>,
    /// Set the current time of each animation.
    currentTime: f64,
}

impl<'a> SeekAnimationsParams<'a> {
    pub fn builder() -> SeekAnimationsParamsBuilder<'a> { SeekAnimationsParamsBuilder::default() }
    pub fn animations(&self) -> &[Cow<'a, str>] { &self.animations }
    pub fn currentTime(&self) -> f64 { self.currentTime }
}

#[derive(Default)]
pub struct SeekAnimationsParamsBuilder<'a> {
    animations: Option<Vec<Cow<'a, str>>>,
    currentTime: Option<f64>,
}

impl<'a> SeekAnimationsParamsBuilder<'a> {
    /// List of animation ids to seek.
    pub fn animations(mut self, animations: Vec<Cow<'a, str>>) -> Self { self.animations = Some(animations); self }
    /// Set the current time of each animation.
    pub fn currentTime(mut self, currentTime: f64) -> Self { self.currentTime = Some(currentTime); self }
    pub fn build(self) -> SeekAnimationsParams<'a> {
        SeekAnimationsParams {
            animations: self.animations.unwrap_or_default(),
            currentTime: self.currentTime.unwrap_or_default(),
        }
    }
}

impl<'a> SeekAnimationsParams<'a> { pub const METHOD: &'static str = "Animation.seekAnimations"; }

impl<'a> crate::CdpCommand<'a> for SeekAnimationsParams<'a> {
    const METHOD: &'static str = "Animation.seekAnimations";
    type Response = crate::EmptyReturns;
}

/// Sets the paused state of a set of animations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPausedParams<'a> {
    /// Animations to set the pause state of.
    animations: Vec<Cow<'a, str>>,
    /// Paused state to set to.
    paused: bool,
}

impl<'a> SetPausedParams<'a> {
    pub fn builder() -> SetPausedParamsBuilder<'a> { SetPausedParamsBuilder::default() }
    pub fn animations(&self) -> &[Cow<'a, str>] { &self.animations }
    pub fn paused(&self) -> bool { self.paused }
}

#[derive(Default)]
pub struct SetPausedParamsBuilder<'a> {
    animations: Option<Vec<Cow<'a, str>>>,
    paused: Option<bool>,
}

impl<'a> SetPausedParamsBuilder<'a> {
    /// Animations to set the pause state of.
    pub fn animations(mut self, animations: Vec<Cow<'a, str>>) -> Self { self.animations = Some(animations); self }
    /// Paused state to set to.
    pub fn paused(mut self, paused: bool) -> Self { self.paused = Some(paused); self }
    pub fn build(self) -> SetPausedParams<'a> {
        SetPausedParams {
            animations: self.animations.unwrap_or_default(),
            paused: self.paused.unwrap_or_default(),
        }
    }
}

impl<'a> SetPausedParams<'a> { pub const METHOD: &'static str = "Animation.setPaused"; }

impl<'a> crate::CdpCommand<'a> for SetPausedParams<'a> {
    const METHOD: &'static str = "Animation.setPaused";
    type Response = crate::EmptyReturns;
}

/// Sets the playback rate of the document timeline.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPlaybackRateParams {
    /// Playback rate for animations on page
    playbackRate: f64,
}

impl SetPlaybackRateParams {
    pub fn builder() -> SetPlaybackRateParamsBuilder { SetPlaybackRateParamsBuilder::default() }
    pub fn playbackRate(&self) -> f64 { self.playbackRate }
}

#[derive(Default)]
pub struct SetPlaybackRateParamsBuilder {
    playbackRate: Option<f64>,
}

impl SetPlaybackRateParamsBuilder {
    /// Playback rate for animations on page
    pub fn playbackRate(mut self, playbackRate: f64) -> Self { self.playbackRate = Some(playbackRate); self }
    pub fn build(self) -> SetPlaybackRateParams {
        SetPlaybackRateParams {
            playbackRate: self.playbackRate.unwrap_or_default(),
        }
    }
}

impl SetPlaybackRateParams { pub const METHOD: &'static str = "Animation.setPlaybackRate"; }

impl<'a> crate::CdpCommand<'a> for SetPlaybackRateParams {
    const METHOD: &'static str = "Animation.setPlaybackRate";
    type Response = crate::EmptyReturns;
}

/// Sets the timing of an animation node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTimingParams<'a> {
    /// Animation id.
    animationId: Cow<'a, str>,
    /// Duration of the animation.
    duration: f64,
    /// Delay of the animation.
    delay: f64,
}

impl<'a> SetTimingParams<'a> {
    pub fn builder() -> SetTimingParamsBuilder<'a> { SetTimingParamsBuilder::default() }
    pub fn animationId(&self) -> &str { self.animationId.as_ref() }
    pub fn duration(&self) -> f64 { self.duration }
    pub fn delay(&self) -> f64 { self.delay }
}

#[derive(Default)]
pub struct SetTimingParamsBuilder<'a> {
    animationId: Option<Cow<'a, str>>,
    duration: Option<f64>,
    delay: Option<f64>,
}

impl<'a> SetTimingParamsBuilder<'a> {
    /// Animation id.
    pub fn animationId(mut self, animationId: impl Into<Cow<'a, str>>) -> Self { self.animationId = Some(animationId.into()); self }
    /// Duration of the animation.
    pub fn duration(mut self, duration: f64) -> Self { self.duration = Some(duration); self }
    /// Delay of the animation.
    pub fn delay(mut self, delay: f64) -> Self { self.delay = Some(delay); self }
    pub fn build(self) -> SetTimingParams<'a> {
        SetTimingParams {
            animationId: self.animationId.unwrap_or_default(),
            duration: self.duration.unwrap_or_default(),
            delay: self.delay.unwrap_or_default(),
        }
    }
}

impl<'a> SetTimingParams<'a> { pub const METHOD: &'static str = "Animation.setTiming"; }

impl<'a> crate::CdpCommand<'a> for SetTimingParams<'a> {
    const METHOD: &'static str = "Animation.setTiming";
    type Response = crate::EmptyReturns;
}
