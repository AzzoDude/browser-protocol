use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Animation instance.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Animation<'a> {
    /// 'Animation''s id.
    pub id: Cow<'a, str>,
    /// 'Animation''s name.
    pub name: Cow<'a, str>,
    /// 'Animation''s internal paused state.
    #[serde(rename = "pausedState")]
    pub paused_state: bool,
    /// 'Animation''s play state.
    #[serde(rename = "playState")]
    pub play_state: Cow<'a, str>,
    /// 'Animation''s playback rate.
    #[serde(rename = "playbackRate")]
    pub playback_rate: f64,
    /// 'Animation''s start time.
    /// Milliseconds for time based animations and
    /// percentage \[0 - 100\] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// 'Animation''s current time.
    #[serde(rename = "currentTime")]
    pub current_time: f64,
    /// Animation type of 'Animation'.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// 'Animation''s source animation node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AnimationEffect<'a>>,
    /// A unique ID for 'Animation' representing the sources that triggered this CSS
    /// animation/transition.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssId")]
    pub css_id: Option<Cow<'a, str>>,
    /// View or scroll timeline
    #[serde(skip_serializing_if = "Option::is_none", rename = "viewOrScrollTimeline")]
    pub view_or_scroll_timeline: Option<ViewOrScrollTimeline>,
}
/// Timeline instance

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ViewOrScrollTimeline {
    /// Scroll container node
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceNodeId")]
    pub source_node_id: Option<crate::dom::BackendNodeId>,
    /// Represents the starting scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "startOffset")]
    pub start_offset: Option<f64>,
    /// Represents the ending scroll position of the timeline
    /// as a length offset in pixels from scroll origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "endOffset")]
    pub end_offset: Option<f64>,
    /// The element whose principal box's visibility in the
    /// scrollport defined the progress of the timeline.
    /// Does not exist for animations with ScrollTimeline
    #[serde(skip_serializing_if = "Option::is_none", rename = "subjectNodeId")]
    pub subject_node_id: Option<crate::dom::BackendNodeId>,
    /// Orientation of the scroll
    pub axis: crate::dom::ScrollOrientation,
}
/// AnimationEffect instance

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AnimationEffect<'a> {
    /// 'AnimationEffect''s delay.
    pub delay: f64,
    /// 'AnimationEffect''s end delay.
    #[serde(rename = "endDelay")]
    pub end_delay: f64,
    /// 'AnimationEffect''s iteration start.
    #[serde(rename = "iterationStart")]
    pub iteration_start: f64,
    /// 'AnimationEffect''s iterations. Omitted if the value is infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<f64>,
    /// 'AnimationEffect''s iteration duration.
    /// Milliseconds for time based animations and
    /// percentage \[0 - 100\] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).
    pub duration: f64,
    /// 'AnimationEffect''s playback direction.
    pub direction: Cow<'a, str>,
    /// 'AnimationEffect''s fill mode.
    pub fill: Cow<'a, str>,
    /// 'AnimationEffect''s target node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
    /// 'AnimationEffect''s keyframes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "keyframesRule")]
    pub keyframes_rule: Option<KeyframesRule<'a>>,
    /// 'AnimationEffect''s timing function.
    pub easing: Cow<'a, str>,
}
/// Keyframes Rule

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct KeyframesRule<'a> {
    /// CSS keyframed animation's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    /// List of animation keyframes.
    pub keyframes: Vec<KeyframeStyle<'a>>,
}
/// Keyframe Style

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct KeyframeStyle<'a> {
    /// Keyframe's time offset.
    pub offset: Cow<'a, str>,
    /// 'AnimationEffect''s timing function.
    pub easing: Cow<'a, str>,
}
/// Disables animation domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.disable")]
pub struct DisableParams {

}
/// Enables animation domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.enable")]
pub struct EnableParams {

}
/// Returns the current time of the an animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.getCurrentTime", response = "GetCurrentTimeReturns")]
pub struct GetCurrentTimeParams<'a> {
    /// Id of animation.
    pub id: Cow<'a, str>,
}
/// Returns the current time of the an animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentTimeReturns {
    /// Current time of the page.
    #[serde(rename = "currentTime")]
    pub current_time: f64,
}
/// Gets the playback rate of the document timeline.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.getPlaybackRate", response = "GetPlaybackRateReturns")]
pub struct GetPlaybackRateParams {

}
/// Gets the playback rate of the document timeline.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaybackRateReturns {
    /// Playback rate for animations on page.
    #[serde(rename = "playbackRate")]
    pub playback_rate: f64,
}
/// Releases a set of animations to no longer be manipulated.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.releaseAnimations")]
pub struct ReleaseAnimationsParams<'a> {
    /// List of animation ids to seek.
    pub animations: Vec<Cow<'a, str>>,
}
/// Gets the remote object of the Animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.resolveAnimation", response = "ResolveAnimationReturns")]
pub struct ResolveAnimationParams<'a> {
    /// Animation id.
    #[serde(rename = "animationId")]
    pub animation_id: Cow<'a, str>,
}
/// Gets the remote object of the Animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ResolveAnimationReturns {
    /// Corresponding remote object.
    #[serde(rename = "remoteObject")]
    pub remote_object: crate::runtime::RemoteObject,
}
/// Seek a set of animations to a particular time within each animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.seekAnimations")]
pub struct SeekAnimationsParams<'a> {
    /// List of animation ids to seek.
    pub animations: Vec<Cow<'a, str>>,
    /// Set each animation to the same time.
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentTime")]
    pub current_time: Option<f64>,
    /// Set each animation to a different time. If set, should have the same
    /// length as animations. Exactly one of currentTime or currentTimes should
    /// be set.
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentTimes")]
    pub current_times: Option<Vec<f64>>,
}
/// Sets the paused state of a set of animations.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.setPaused")]
pub struct SetPausedParams<'a> {
    /// Animations to set the pause state of.
    pub animations: Vec<Cow<'a, str>>,
    /// Paused state to set to.
    pub paused: bool,
}
/// Sets the playback rate of the document timeline.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.setPlaybackRate")]
pub struct SetPlaybackRateParams {
    /// Playback rate for animations on page
    #[serde(rename = "playbackRate")]
    pub playback_rate: f64,
}
/// Sets the timing of an animation node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.setTiming")]
pub struct SetTimingParams<'a> {
    /// Animation id.
    #[serde(rename = "animationId")]
    pub animation_id: Cow<'a, str>,
    /// Duration of the animation.
    pub duration: f64,
    /// Delay of the animation.
    pub delay: f64,
}
/// Event for when an animation has been cancelled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.animationCanceled")]
pub struct AnimationCanceled<'a> {
    /// Id of the animation that was cancelled.
    pub id: Cow<'a, str>,
}
/// Event for each animation that has been created.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.animationCreated")]
pub struct AnimationCreated<'a> {
    /// Id of the animation that was created.
    pub id: Cow<'a, str>,
}
/// Event for animation that has been started.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.animationStarted")]
pub struct AnimationStarted<'a> {
    /// Animation that was started.
    pub animation: Animation<'a>,
}
/// Event for animation that has been updated.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Animation.animationUpdated")]
pub struct AnimationUpdated<'a> {
    /// Animation that was updated.
    pub animation: Animation<'a>,
}