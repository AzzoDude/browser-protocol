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
    #[serde(rename = "pausedState")]
    paused_state: bool,
    /// 'Animation''s play state.
    #[serde(rename = "playState")]
    play_state: Cow<'a, str>,
    /// 'Animation''s playback rate.
    #[serde(rename = "playbackRate")]
    playback_rate: f64,
    /// 'Animation''s start time.
    /// Milliseconds for time based animations and
    /// percentage \[0 - 100\] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    #[serde(rename = "startTime")]
    start_time: f64,
    /// 'Animation''s current time.
    #[serde(rename = "currentTime")]
    current_time: f64,
    /// Animation type of 'Animation'.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// 'Animation''s source animation node.
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<AnimationEffect<'a>>,
    /// A unique ID for 'Animation' representing the sources that triggered this CSS
    /// animation/transition.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssId")]
    css_id: Option<Cow<'a, str>>,
    /// View or scroll timeline
    #[serde(skip_serializing_if = "Option::is_none", rename = "viewOrScrollTimeline")]
    view_or_scroll_timeline: Option<ViewOrScrollTimeline>,
}

impl<'a> Animation<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: `Animation`'s id.
    /// * `name`: `Animation`'s name.
    /// * `paused_state`: `Animation`'s internal paused state.
    /// * `play_state`: `Animation`'s play state.
    /// * `playback_rate`: `Animation`'s playback rate.
    /// * `start_time`: `Animation`'s start time. Milliseconds for time based animations and percentage \[0 - 100\] for scroll driven animations (i.e. when viewOrScrollTimeline exists).
    /// * `current_time`: `Animation`'s current time.
    /// * `type_`: Animation type of `Animation`.
    pub fn builder(id: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, paused_state: bool, play_state: impl Into<Cow<'a, str>>, playback_rate: f64, start_time: f64, current_time: f64, type_: impl Into<Cow<'a, str>>) -> AnimationBuilder<'a> {
        AnimationBuilder {
            id: id.into(),
            name: name.into(),
            paused_state: paused_state,
            play_state: play_state.into(),
            playback_rate: playback_rate,
            start_time: start_time,
            current_time: current_time,
            type_: type_.into(),
            source: None,
            css_id: None,
            view_or_scroll_timeline: None,
        }
    }
    /// 'Animation''s id.
    pub fn id(&self) -> &str { self.id.as_ref() }
    /// 'Animation''s name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// 'Animation''s internal paused state.
    pub fn paused_state(&self) -> bool { self.paused_state }
    /// 'Animation''s play state.
    pub fn play_state(&self) -> &str { self.play_state.as_ref() }
    /// 'Animation''s playback rate.
    pub fn playback_rate(&self) -> f64 { self.playback_rate }
    /// 'Animation''s start time.
    /// Milliseconds for time based animations and
    /// percentage \[0 - 100\] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    pub fn start_time(&self) -> f64 { self.start_time }
    /// 'Animation''s current time.
    pub fn current_time(&self) -> f64 { self.current_time }
    /// Animation type of 'Animation'.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// 'Animation''s source animation node.
    pub fn source(&self) -> Option<&AnimationEffect<'a>> { self.source.as_ref() }
    /// A unique ID for 'Animation' representing the sources that triggered this CSS
    /// animation/transition.
    pub fn css_id(&self) -> Option<&str> { self.css_id.as_deref() }
    /// View or scroll timeline
    pub fn view_or_scroll_timeline(&self) -> Option<&ViewOrScrollTimeline> { self.view_or_scroll_timeline.as_ref() }
}


pub struct AnimationBuilder<'a> {
    id: Cow<'a, str>,
    name: Cow<'a, str>,
    paused_state: bool,
    play_state: Cow<'a, str>,
    playback_rate: f64,
    start_time: f64,
    current_time: f64,
    type_: Cow<'a, str>,
    source: Option<AnimationEffect<'a>>,
    css_id: Option<Cow<'a, str>>,
    view_or_scroll_timeline: Option<ViewOrScrollTimeline>,
}

impl<'a> AnimationBuilder<'a> {
    /// 'Animation''s source animation node.
    pub fn source(mut self, source: AnimationEffect<'a>) -> Self { self.source = Some(source); self }
    /// A unique ID for 'Animation' representing the sources that triggered this CSS
    /// animation/transition.
    pub fn css_id(mut self, css_id: impl Into<Cow<'a, str>>) -> Self { self.css_id = Some(css_id.into()); self }
    /// View or scroll timeline
    pub fn view_or_scroll_timeline(mut self, view_or_scroll_timeline: ViewOrScrollTimeline) -> Self { self.view_or_scroll_timeline = Some(view_or_scroll_timeline); self }
    pub fn build(self) -> Animation<'a> {
        Animation {
            id: self.id,
            name: self.name,
            paused_state: self.paused_state,
            play_state: self.play_state,
            playback_rate: self.playback_rate,
            start_time: self.start_time,
            current_time: self.current_time,
            type_: self.type_,
            source: self.source,
            css_id: self.css_id,
            view_or_scroll_timeline: self.view_or_scroll_timeline,
        }
    }
}

/// Timeline instance

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViewOrScrollTimeline {
    /// Scroll container node
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceNodeId")]
    source_node_id: Option<crate::dom::BackendNodeId>,
    /// Represents the starting scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "startOffset")]
    start_offset: Option<f64>,
    /// Represents the ending scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "endOffset")]
    end_offset: Option<f64>,
    /// The element whose principal box's visibility in the
    /// scrollport defined the progress of the timeline.
    /// Does not exist for animations with ScrollTimeline
    #[serde(skip_serializing_if = "Option::is_none", rename = "subjectNodeId")]
    subject_node_id: Option<crate::dom::BackendNodeId>,
    /// Orientation of the scroll
    axis: crate::dom::ScrollOrientation,
}

impl ViewOrScrollTimeline {
    /// Creates a builder for this type with the required parameters:
    /// * `axis`: Orientation of the scroll
    pub fn builder(axis: crate::dom::ScrollOrientation) -> ViewOrScrollTimelineBuilder {
        ViewOrScrollTimelineBuilder {
            source_node_id: None,
            start_offset: None,
            end_offset: None,
            subject_node_id: None,
            axis: axis,
        }
    }
    /// Scroll container node
    pub fn source_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.source_node_id.as_ref() }
    /// Represents the starting scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    pub fn start_offset(&self) -> Option<f64> { self.start_offset }
    /// Represents the ending scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    pub fn end_offset(&self) -> Option<f64> { self.end_offset }
    /// The element whose principal box's visibility in the
    /// scrollport defined the progress of the timeline.
    /// Does not exist for animations with ScrollTimeline
    pub fn subject_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.subject_node_id.as_ref() }
    /// Orientation of the scroll
    pub fn axis(&self) -> &crate::dom::ScrollOrientation { &self.axis }
}


pub struct ViewOrScrollTimelineBuilder {
    source_node_id: Option<crate::dom::BackendNodeId>,
    start_offset: Option<f64>,
    end_offset: Option<f64>,
    subject_node_id: Option<crate::dom::BackendNodeId>,
    axis: crate::dom::ScrollOrientation,
}

impl ViewOrScrollTimelineBuilder {
    /// Scroll container node
    pub fn source_node_id(mut self, source_node_id: crate::dom::BackendNodeId) -> Self { self.source_node_id = Some(source_node_id); self }
    /// Represents the starting scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    pub fn start_offset(mut self, start_offset: f64) -> Self { self.start_offset = Some(start_offset); self }
    /// Represents the ending scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    pub fn end_offset(mut self, end_offset: f64) -> Self { self.end_offset = Some(end_offset); self }
    /// The element whose principal box's visibility in the
    /// scrollport defined the progress of the timeline.
    /// Does not exist for animations with ScrollTimeline
    pub fn subject_node_id(mut self, subject_node_id: crate::dom::BackendNodeId) -> Self { self.subject_node_id = Some(subject_node_id); self }
    pub fn build(self) -> ViewOrScrollTimeline {
        ViewOrScrollTimeline {
            source_node_id: self.source_node_id,
            start_offset: self.start_offset,
            end_offset: self.end_offset,
            subject_node_id: self.subject_node_id,
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
    #[serde(rename = "endDelay")]
    end_delay: f64,
    /// 'AnimationEffect''s iteration start.
    #[serde(rename = "iterationStart")]
    iteration_start: f64,
    /// 'AnimationEffect''s iterations. Omitted if the value is infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    iterations: Option<f64>,
    /// 'AnimationEffect''s iteration duration.
    /// Milliseconds for time based animations and
    /// percentage \[0 - 100\] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    duration: f64,
    /// 'AnimationEffect''s playback direction.
    direction: Cow<'a, str>,
    /// 'AnimationEffect''s fill mode.
    fill: Cow<'a, str>,
    /// 'AnimationEffect''s target node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
    /// 'AnimationEffect''s keyframes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "keyframesRule")]
    keyframes_rule: Option<KeyframesRule<'a>>,
    /// 'AnimationEffect''s timing function.
    easing: Cow<'a, str>,
}

impl<'a> AnimationEffect<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `delay`: `AnimationEffect`'s delay.
    /// * `end_delay`: `AnimationEffect`'s end delay.
    /// * `iteration_start`: `AnimationEffect`'s iteration start.
    /// * `duration`: `AnimationEffect`'s iteration duration. Milliseconds for time based animations and percentage \[0 - 100\] for scroll driven animations (i.e. when viewOrScrollTimeline exists).
    /// * `direction`: `AnimationEffect`'s playback direction.
    /// * `fill`: `AnimationEffect`'s fill mode.
    /// * `easing`: `AnimationEffect`'s timing function.
    pub fn builder(delay: f64, end_delay: f64, iteration_start: f64, duration: f64, direction: impl Into<Cow<'a, str>>, fill: impl Into<Cow<'a, str>>, easing: impl Into<Cow<'a, str>>) -> AnimationEffectBuilder<'a> {
        AnimationEffectBuilder {
            delay: delay,
            end_delay: end_delay,
            iteration_start: iteration_start,
            iterations: None,
            duration: duration,
            direction: direction.into(),
            fill: fill.into(),
            backend_node_id: None,
            keyframes_rule: None,
            easing: easing.into(),
        }
    }
    /// 'AnimationEffect''s delay.
    pub fn delay(&self) -> f64 { self.delay }
    /// 'AnimationEffect''s end delay.
    pub fn end_delay(&self) -> f64 { self.end_delay }
    /// 'AnimationEffect''s iteration start.
    pub fn iteration_start(&self) -> f64 { self.iteration_start }
    /// 'AnimationEffect''s iterations. Omitted if the value is infinite.
    pub fn iterations(&self) -> Option<f64> { self.iterations }
    /// 'AnimationEffect''s iteration duration.
    /// Milliseconds for time based animations and
    /// percentage \[0 - 100\] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    pub fn duration(&self) -> f64 { self.duration }
    /// 'AnimationEffect''s playback direction.
    pub fn direction(&self) -> &str { self.direction.as_ref() }
    /// 'AnimationEffect''s fill mode.
    pub fn fill(&self) -> &str { self.fill.as_ref() }
    /// 'AnimationEffect''s target node.
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
    /// 'AnimationEffect''s keyframes.
    pub fn keyframes_rule(&self) -> Option<&KeyframesRule<'a>> { self.keyframes_rule.as_ref() }
    /// 'AnimationEffect''s timing function.
    pub fn easing(&self) -> &str { self.easing.as_ref() }
}


pub struct AnimationEffectBuilder<'a> {
    delay: f64,
    end_delay: f64,
    iteration_start: f64,
    iterations: Option<f64>,
    duration: f64,
    direction: Cow<'a, str>,
    fill: Cow<'a, str>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
    keyframes_rule: Option<KeyframesRule<'a>>,
    easing: Cow<'a, str>,
}

impl<'a> AnimationEffectBuilder<'a> {
    /// 'AnimationEffect''s iterations. Omitted if the value is infinite.
    pub fn iterations(mut self, iterations: f64) -> Self { self.iterations = Some(iterations); self }
    /// 'AnimationEffect''s target node.
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// 'AnimationEffect''s keyframes.
    pub fn keyframes_rule(mut self, keyframes_rule: KeyframesRule<'a>) -> Self { self.keyframes_rule = Some(keyframes_rule); self }
    pub fn build(self) -> AnimationEffect<'a> {
        AnimationEffect {
            delay: self.delay,
            end_delay: self.end_delay,
            iteration_start: self.iteration_start,
            iterations: self.iterations,
            duration: self.duration,
            direction: self.direction,
            fill: self.fill,
            backend_node_id: self.backend_node_id,
            keyframes_rule: self.keyframes_rule,
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
    /// Creates a builder for this type with the required parameters:
    /// * `keyframes`: List of animation keyframes.
    pub fn builder(keyframes: Vec<KeyframeStyle<'a>>) -> KeyframesRuleBuilder<'a> {
        KeyframesRuleBuilder {
            name: None,
            keyframes: keyframes,
        }
    }
    /// CSS keyframed animation's name.
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    /// List of animation keyframes.
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
    /// Creates a builder for this type with the required parameters:
    /// * `offset`: Keyframe's time offset.
    /// * `easing`: `AnimationEffect`'s timing function.
    pub fn builder(offset: impl Into<Cow<'a, str>>, easing: impl Into<Cow<'a, str>>) -> KeyframeStyleBuilder<'a> {
        KeyframeStyleBuilder {
            offset: offset.into(),
            easing: easing.into(),
        }
    }
    /// Keyframe's time offset.
    pub fn offset(&self) -> &str { self.offset.as_ref() }
    /// 'AnimationEffect''s timing function.
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
    /// Creates a builder for this type with the required parameters:
    /// * `id`: Id of animation.
    pub fn builder(id: impl Into<Cow<'a, str>>) -> GetCurrentTimeParamsBuilder<'a> {
        GetCurrentTimeParamsBuilder {
            id: id.into(),
        }
    }
    /// Id of animation.
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
    #[serde(rename = "currentTime")]
    current_time: f64,
}

impl GetCurrentTimeReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `current_time`: Current time of the page.
    pub fn builder(current_time: f64) -> GetCurrentTimeReturnsBuilder {
        GetCurrentTimeReturnsBuilder {
            current_time: current_time,
        }
    }
    /// Current time of the page.
    pub fn current_time(&self) -> f64 { self.current_time }
}


pub struct GetCurrentTimeReturnsBuilder {
    current_time: f64,
}

impl GetCurrentTimeReturnsBuilder {
    pub fn build(self) -> GetCurrentTimeReturns {
        GetCurrentTimeReturns {
            current_time: self.current_time,
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
    #[serde(rename = "playbackRate")]
    playback_rate: f64,
}

impl GetPlaybackRateReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `playback_rate`: Playback rate for animations on page.
    pub fn builder(playback_rate: f64) -> GetPlaybackRateReturnsBuilder {
        GetPlaybackRateReturnsBuilder {
            playback_rate: playback_rate,
        }
    }
    /// Playback rate for animations on page.
    pub fn playback_rate(&self) -> f64 { self.playback_rate }
}


pub struct GetPlaybackRateReturnsBuilder {
    playback_rate: f64,
}

impl GetPlaybackRateReturnsBuilder {
    pub fn build(self) -> GetPlaybackRateReturns {
        GetPlaybackRateReturns {
            playback_rate: self.playback_rate,
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
    /// Creates a builder for this type with the required parameters:
    /// * `animations`: List of animation ids to seek.
    pub fn builder(animations: Vec<Cow<'a, str>>) -> ReleaseAnimationsParamsBuilder<'a> {
        ReleaseAnimationsParamsBuilder {
            animations: animations,
        }
    }
    /// List of animation ids to seek.
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
    #[serde(rename = "animationId")]
    animation_id: Cow<'a, str>,
}

impl<'a> ResolveAnimationParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `animation_id`: Animation id.
    pub fn builder(animation_id: impl Into<Cow<'a, str>>) -> ResolveAnimationParamsBuilder<'a> {
        ResolveAnimationParamsBuilder {
            animation_id: animation_id.into(),
        }
    }
    /// Animation id.
    pub fn animation_id(&self) -> &str { self.animation_id.as_ref() }
}


pub struct ResolveAnimationParamsBuilder<'a> {
    animation_id: Cow<'a, str>,
}

impl<'a> ResolveAnimationParamsBuilder<'a> {
    pub fn build(self) -> ResolveAnimationParams<'a> {
        ResolveAnimationParams {
            animation_id: self.animation_id,
        }
    }
}

/// Gets the remote object of the Animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveAnimationReturns {
    /// Corresponding remote object.
    #[serde(rename = "remoteObject")]
    remote_object: crate::runtime::RemoteObject,
}

impl ResolveAnimationReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `remote_object`: Corresponding remote object.
    pub fn builder(remote_object: crate::runtime::RemoteObject) -> ResolveAnimationReturnsBuilder {
        ResolveAnimationReturnsBuilder {
            remote_object: remote_object,
        }
    }
    /// Corresponding remote object.
    pub fn remote_object(&self) -> &crate::runtime::RemoteObject { &self.remote_object }
}


pub struct ResolveAnimationReturnsBuilder {
    remote_object: crate::runtime::RemoteObject,
}

impl ResolveAnimationReturnsBuilder {
    pub fn build(self) -> ResolveAnimationReturns {
        ResolveAnimationReturns {
            remote_object: self.remote_object,
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
    #[serde(rename = "currentTime")]
    current_time: f64,
}

impl<'a> SeekAnimationsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `animations`: List of animation ids to seek.
    /// * `current_time`: Set the current time of each animation.
    pub fn builder(animations: Vec<Cow<'a, str>>, current_time: f64) -> SeekAnimationsParamsBuilder<'a> {
        SeekAnimationsParamsBuilder {
            animations: animations,
            current_time: current_time,
        }
    }
    /// List of animation ids to seek.
    pub fn animations(&self) -> &[Cow<'a, str>] { &self.animations }
    /// Set the current time of each animation.
    pub fn current_time(&self) -> f64 { self.current_time }
}


pub struct SeekAnimationsParamsBuilder<'a> {
    animations: Vec<Cow<'a, str>>,
    current_time: f64,
}

impl<'a> SeekAnimationsParamsBuilder<'a> {
    pub fn build(self) -> SeekAnimationsParams<'a> {
        SeekAnimationsParams {
            animations: self.animations,
            current_time: self.current_time,
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
    /// Creates a builder for this type with the required parameters:
    /// * `animations`: Animations to set the pause state of.
    /// * `paused`: Paused state to set to.
    pub fn builder(animations: Vec<Cow<'a, str>>, paused: bool) -> SetPausedParamsBuilder<'a> {
        SetPausedParamsBuilder {
            animations: animations,
            paused: paused,
        }
    }
    /// Animations to set the pause state of.
    pub fn animations(&self) -> &[Cow<'a, str>] { &self.animations }
    /// Paused state to set to.
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
    #[serde(rename = "playbackRate")]
    playback_rate: f64,
}

impl SetPlaybackRateParams {
    /// Creates a builder for this type with the required parameters:
    /// * `playback_rate`: Playback rate for animations on page
    pub fn builder(playback_rate: f64) -> SetPlaybackRateParamsBuilder {
        SetPlaybackRateParamsBuilder {
            playback_rate: playback_rate,
        }
    }
    /// Playback rate for animations on page
    pub fn playback_rate(&self) -> f64 { self.playback_rate }
}


pub struct SetPlaybackRateParamsBuilder {
    playback_rate: f64,
}

impl SetPlaybackRateParamsBuilder {
    pub fn build(self) -> SetPlaybackRateParams {
        SetPlaybackRateParams {
            playback_rate: self.playback_rate,
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
    #[serde(rename = "animationId")]
    animation_id: Cow<'a, str>,
    /// Duration of the animation.
    duration: f64,
    /// Delay of the animation.
    delay: f64,
}

impl<'a> SetTimingParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `animation_id`: Animation id.
    /// * `duration`: Duration of the animation.
    /// * `delay`: Delay of the animation.
    pub fn builder(animation_id: impl Into<Cow<'a, str>>, duration: f64, delay: f64) -> SetTimingParamsBuilder<'a> {
        SetTimingParamsBuilder {
            animation_id: animation_id.into(),
            duration: duration,
            delay: delay,
        }
    }
    /// Animation id.
    pub fn animation_id(&self) -> &str { self.animation_id.as_ref() }
    /// Duration of the animation.
    pub fn duration(&self) -> f64 { self.duration }
    /// Delay of the animation.
    pub fn delay(&self) -> f64 { self.delay }
}


pub struct SetTimingParamsBuilder<'a> {
    animation_id: Cow<'a, str>,
    duration: f64,
    delay: f64,
}

impl<'a> SetTimingParamsBuilder<'a> {
    pub fn build(self) -> SetTimingParams<'a> {
        SetTimingParams {
            animation_id: self.animation_id,
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
