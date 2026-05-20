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
    pub fn builder(id: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, pausedState: bool, playState: impl Into<Cow<'a, str>>, playbackRate: f64, startTime: f64, currentTime: f64, type_: impl Into<Cow<'a, str>>) -> AnimationBuilder<'a> {
        AnimationBuilder {
            id: id.into(),
            name: name.into(),
            pausedState: pausedState,
            playState: playState.into(),
            playbackRate: playbackRate,
            startTime: startTime,
            currentTime: currentTime,
            type_: type_.into(),
            source: None,
            cssId: None,
            viewOrScrollTimeline: None,
        }
    }
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


pub struct AnimationBuilder<'a> {
    id: Cow<'a, str>,
    name: Cow<'a, str>,
    pausedState: bool,
    playState: Cow<'a, str>,
    playbackRate: f64,
    startTime: f64,
    currentTime: f64,
    type_: Cow<'a, str>,
    source: Option<AnimationEffect<'a>>,
    cssId: Option<Cow<'a, str>>,
    viewOrScrollTimeline: Option<ViewOrScrollTimeline>,
}

impl<'a> AnimationBuilder<'a> {
    /// 'Animation''s source animation node.
    pub fn source(mut self, source: AnimationEffect<'a>) -> Self { self.source = Some(source); self }
    /// A unique ID for 'Animation' representing the sources that triggered this CSS
    /// animation/transition.
    pub fn cssId(mut self, cssId: impl Into<Cow<'a, str>>) -> Self { self.cssId = Some(cssId.into()); self }
    /// View or scroll timeline
    pub fn viewOrScrollTimeline(mut self, viewOrScrollTimeline: ViewOrScrollTimeline) -> Self { self.viewOrScrollTimeline = Some(viewOrScrollTimeline); self }
    pub fn build(self) -> Animation<'a> {
        Animation {
            id: self.id,
            name: self.name,
            pausedState: self.pausedState,
            playState: self.playState,
            playbackRate: self.playbackRate,
            startTime: self.startTime,
            currentTime: self.currentTime,
            type_: self.type_,
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
    pub fn builder(axis: crate::dom::ScrollOrientation) -> ViewOrScrollTimelineBuilder {
        ViewOrScrollTimelineBuilder {
            sourceNodeId: None,
            startOffset: None,
            endOffset: None,
            subjectNodeId: None,
            axis: axis,
        }
    }
    pub fn sourceNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.sourceNodeId.as_ref() }
    pub fn startOffset(&self) -> Option<f64> { self.startOffset }
    pub fn endOffset(&self) -> Option<f64> { self.endOffset }
    pub fn subjectNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.subjectNodeId.as_ref() }
    pub fn axis(&self) -> &crate::dom::ScrollOrientation { &self.axis }
}


pub struct ViewOrScrollTimelineBuilder {
    sourceNodeId: Option<crate::dom::BackendNodeId>,
    startOffset: Option<f64>,
    endOffset: Option<f64>,
    subjectNodeId: Option<crate::dom::BackendNodeId>,
    axis: crate::dom::ScrollOrientation,
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
    pub fn build(self) -> ViewOrScrollTimeline {
        ViewOrScrollTimeline {
            sourceNodeId: self.sourceNodeId,
            startOffset: self.startOffset,
            endOffset: self.endOffset,
            subjectNodeId: self.subjectNodeId,
            axis: self.axis,
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
    pub fn builder(delay: f64, endDelay: f64, iterationStart: f64, duration: f64, direction: impl Into<Cow<'a, str>>, fill: impl Into<Cow<'a, str>>, easing: impl Into<Cow<'a, str>>) -> AnimationEffectBuilder<'a> {
        AnimationEffectBuilder {
            delay: delay,
            endDelay: endDelay,
            iterationStart: iterationStart,
            iterations: None,
            duration: duration,
            direction: direction.into(),
            fill: fill.into(),
            backendNodeId: None,
            keyframesRule: None,
            easing: easing.into(),
        }
    }
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


pub struct AnimationEffectBuilder<'a> {
    delay: f64,
    endDelay: f64,
    iterationStart: f64,
    iterations: Option<f64>,
    duration: f64,
    direction: Cow<'a, str>,
    fill: Cow<'a, str>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    keyframesRule: Option<KeyframesRule<'a>>,
    easing: Cow<'a, str>,
}

impl<'a> AnimationEffectBuilder<'a> {
    /// 'AnimationEffect''s iterations. Omitted if the value is infinite.
    pub fn iterations(mut self, iterations: f64) -> Self { self.iterations = Some(iterations); self }
    /// 'AnimationEffect''s target node.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// 'AnimationEffect''s keyframes.
    pub fn keyframesRule(mut self, keyframesRule: KeyframesRule<'a>) -> Self { self.keyframesRule = Some(keyframesRule); self }
    pub fn build(self) -> AnimationEffect<'a> {
        AnimationEffect {
            delay: self.delay,
            endDelay: self.endDelay,
            iterationStart: self.iterationStart,
            iterations: self.iterations,
            duration: self.duration,
            direction: self.direction,
            fill: self.fill,
            backendNodeId: self.backendNodeId,
            keyframesRule: self.keyframesRule,
            easing: self.easing,
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
    pub fn builder(keyframes: Vec<KeyframeStyle<'a>>) -> KeyframesRuleBuilder<'a> {
        KeyframesRuleBuilder {
            name: None,
            keyframes: keyframes,
        }
    }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn keyframes(&self) -> &[KeyframeStyle<'a>] { &self.keyframes }
}


pub struct KeyframesRuleBuilder<'a> {
    name: Option<Cow<'a, str>>,
    keyframes: Vec<KeyframeStyle<'a>>,
}

impl<'a> KeyframesRuleBuilder<'a> {
    /// CSS keyframed animation's name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> KeyframesRule<'a> {
        KeyframesRule {
            name: self.name,
            keyframes: self.keyframes,
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
    pub fn builder(offset: impl Into<Cow<'a, str>>, easing: impl Into<Cow<'a, str>>) -> KeyframeStyleBuilder<'a> {
        KeyframeStyleBuilder {
            offset: offset.into(),
            easing: easing.into(),
        }
    }
    pub fn offset(&self) -> &str { self.offset.as_ref() }
    pub fn easing(&self) -> &str { self.easing.as_ref() }
}


pub struct KeyframeStyleBuilder<'a> {
    offset: Cow<'a, str>,
    easing: Cow<'a, str>,
}

impl<'a> KeyframeStyleBuilder<'a> {
    pub fn build(self) -> KeyframeStyle<'a> {
        KeyframeStyle {
            offset: self.offset,
            easing: self.easing,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Animation.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Animation.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

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
    pub fn builder(id: impl Into<Cow<'a, str>>) -> GetCurrentTimeParamsBuilder<'a> {
        GetCurrentTimeParamsBuilder {
            id: id.into(),
        }
    }
    pub fn id(&self) -> &str { self.id.as_ref() }
}


pub struct GetCurrentTimeParamsBuilder<'a> {
    id: Cow<'a, str>,
}

impl<'a> GetCurrentTimeParamsBuilder<'a> {
    pub fn build(self) -> GetCurrentTimeParams<'a> {
        GetCurrentTimeParams {
            id: self.id,
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
    pub fn builder(currentTime: f64) -> GetCurrentTimeReturnsBuilder {
        GetCurrentTimeReturnsBuilder {
            currentTime: currentTime,
        }
    }
    pub fn currentTime(&self) -> f64 { self.currentTime }
}


pub struct GetCurrentTimeReturnsBuilder {
    currentTime: f64,
}

impl GetCurrentTimeReturnsBuilder {
    pub fn build(self) -> GetCurrentTimeReturns {
        GetCurrentTimeReturns {
            currentTime: self.currentTime,
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
    pub fn builder(playbackRate: f64) -> GetPlaybackRateReturnsBuilder {
        GetPlaybackRateReturnsBuilder {
            playbackRate: playbackRate,
        }
    }
    pub fn playbackRate(&self) -> f64 { self.playbackRate }
}


pub struct GetPlaybackRateReturnsBuilder {
    playbackRate: f64,
}

impl GetPlaybackRateReturnsBuilder {
    pub fn build(self) -> GetPlaybackRateReturns {
        GetPlaybackRateReturns {
            playbackRate: self.playbackRate,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetPlaybackRateParams {}

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
    pub fn builder(animations: Vec<Cow<'a, str>>) -> ReleaseAnimationsParamsBuilder<'a> {
        ReleaseAnimationsParamsBuilder {
            animations: animations,
        }
    }
    pub fn animations(&self) -> &[Cow<'a, str>] { &self.animations }
}


pub struct ReleaseAnimationsParamsBuilder<'a> {
    animations: Vec<Cow<'a, str>>,
}

impl<'a> ReleaseAnimationsParamsBuilder<'a> {
    pub fn build(self) -> ReleaseAnimationsParams<'a> {
        ReleaseAnimationsParams {
            animations: self.animations,
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
    pub fn builder(animationId: impl Into<Cow<'a, str>>) -> ResolveAnimationParamsBuilder<'a> {
        ResolveAnimationParamsBuilder {
            animationId: animationId.into(),
        }
    }
    pub fn animationId(&self) -> &str { self.animationId.as_ref() }
}


pub struct ResolveAnimationParamsBuilder<'a> {
    animationId: Cow<'a, str>,
}

impl<'a> ResolveAnimationParamsBuilder<'a> {
    pub fn build(self) -> ResolveAnimationParams<'a> {
        ResolveAnimationParams {
            animationId: self.animationId,
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
    pub fn builder(remoteObject: crate::runtime::RemoteObject) -> ResolveAnimationReturnsBuilder {
        ResolveAnimationReturnsBuilder {
            remoteObject: remoteObject,
        }
    }
    pub fn remoteObject(&self) -> &crate::runtime::RemoteObject { &self.remoteObject }
}


pub struct ResolveAnimationReturnsBuilder {
    remoteObject: crate::runtime::RemoteObject,
}

impl ResolveAnimationReturnsBuilder {
    pub fn build(self) -> ResolveAnimationReturns {
        ResolveAnimationReturns {
            remoteObject: self.remoteObject,
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
    pub fn builder(animations: Vec<Cow<'a, str>>, currentTime: f64) -> SeekAnimationsParamsBuilder<'a> {
        SeekAnimationsParamsBuilder {
            animations: animations,
            currentTime: currentTime,
        }
    }
    pub fn animations(&self) -> &[Cow<'a, str>] { &self.animations }
    pub fn currentTime(&self) -> f64 { self.currentTime }
}


pub struct SeekAnimationsParamsBuilder<'a> {
    animations: Vec<Cow<'a, str>>,
    currentTime: f64,
}

impl<'a> SeekAnimationsParamsBuilder<'a> {
    pub fn build(self) -> SeekAnimationsParams<'a> {
        SeekAnimationsParams {
            animations: self.animations,
            currentTime: self.currentTime,
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
    pub fn builder(animations: Vec<Cow<'a, str>>, paused: bool) -> SetPausedParamsBuilder<'a> {
        SetPausedParamsBuilder {
            animations: animations,
            paused: paused,
        }
    }
    pub fn animations(&self) -> &[Cow<'a, str>] { &self.animations }
    pub fn paused(&self) -> bool { self.paused }
}


pub struct SetPausedParamsBuilder<'a> {
    animations: Vec<Cow<'a, str>>,
    paused: bool,
}

impl<'a> SetPausedParamsBuilder<'a> {
    pub fn build(self) -> SetPausedParams<'a> {
        SetPausedParams {
            animations: self.animations,
            paused: self.paused,
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
    pub fn builder(playbackRate: f64) -> SetPlaybackRateParamsBuilder {
        SetPlaybackRateParamsBuilder {
            playbackRate: playbackRate,
        }
    }
    pub fn playbackRate(&self) -> f64 { self.playbackRate }
}


pub struct SetPlaybackRateParamsBuilder {
    playbackRate: f64,
}

impl SetPlaybackRateParamsBuilder {
    pub fn build(self) -> SetPlaybackRateParams {
        SetPlaybackRateParams {
            playbackRate: self.playbackRate,
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
    pub fn builder(animationId: impl Into<Cow<'a, str>>, duration: f64, delay: f64) -> SetTimingParamsBuilder<'a> {
        SetTimingParamsBuilder {
            animationId: animationId.into(),
            duration: duration,
            delay: delay,
        }
    }
    pub fn animationId(&self) -> &str { self.animationId.as_ref() }
    pub fn duration(&self) -> f64 { self.duration }
    pub fn delay(&self) -> f64 { self.delay }
}


pub struct SetTimingParamsBuilder<'a> {
    animationId: Cow<'a, str>,
    duration: f64,
    delay: f64,
}

impl<'a> SetTimingParamsBuilder<'a> {
    pub fn build(self) -> SetTimingParams<'a> {
        SetTimingParams {
            animationId: self.animationId,
            duration: self.duration,
            delay: self.delay,
        }
    }
}

impl<'a> SetTimingParams<'a> { pub const METHOD: &'static str = "Animation.setTiming"; }

impl<'a> crate::CdpCommand<'a> for SetTimingParams<'a> {
    const METHOD: &'static str = "Animation.setTiming";
    type Response = crate::EmptyReturns;
}
