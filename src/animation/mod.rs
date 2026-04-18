use serde::{Serialize, Deserialize};

/// Animation instance.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    /// 'Animation''s id.

    pub id: String,
    /// 'Animation''s name.

    pub name: String,
    /// 'Animation''s internal paused state.

    pub pausedState: bool,
    /// 'Animation''s play state.

    pub playState: String,
    /// 'Animation''s playback rate.

    pub playbackRate: f64,
    /// 'Animation''s start time.
    /// Milliseconds for time based animations and
    /// percentage \[0 - 100\] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).

    pub startTime: f64,
    /// 'Animation''s current time.

    pub currentTime: f64,
    /// Animation type of 'Animation'.

    #[serde(rename = "type")]
    pub type_: String,
    /// 'Animation''s source animation node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AnimationEffect>,
    /// A unique ID for 'Animation' representing the sources that triggered this CSS
    /// animation/transition.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssId: Option<String>,
    /// View or scroll timeline

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewOrScrollTimeline: Option<ViewOrScrollTimeline>,
}

/// Timeline instance

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViewOrScrollTimeline {
    /// Scroll container node

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceNodeId: Option<crate::dom::BackendNodeId>,
    /// Represents the starting scroll position of the timeline
    /// as a length offset in pixels from scroll origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub startOffset: Option<f64>,
    /// Represents the ending scroll position of the timeline
    /// as a length offset in pixels from scroll origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endOffset: Option<f64>,
    /// The element whose principal box's visibility in the
    /// scrollport defined the progress of the timeline.
    /// Does not exist for animations with ScrollTimeline

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjectNodeId: Option<crate::dom::BackendNodeId>,
    /// Orientation of the scroll

    pub axis: crate::dom::ScrollOrientation,
}

/// AnimationEffect instance

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AnimationEffect {
    /// 'AnimationEffect''s delay.

    pub delay: f64,
    /// 'AnimationEffect''s end delay.

    pub endDelay: f64,
    /// 'AnimationEffect''s iteration start.

    pub iterationStart: f64,
    /// 'AnimationEffect''s iterations. Omitted if the value is infinite.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<f64>,
    /// 'AnimationEffect''s iteration duration.
    /// Milliseconds for time based animations and
    /// percentage \[0 - 100\] for scroll driven animations
    /// (i.e. when viewOrScrollTimeline exists).

    pub duration: f64,
    /// 'AnimationEffect''s playback direction.

    pub direction: String,
    /// 'AnimationEffect''s fill mode.

    pub fill: String,
    /// 'AnimationEffect''s target node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
    /// 'AnimationEffect''s keyframes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframesRule: Option<KeyframesRule>,
    /// 'AnimationEffect''s timing function.

    pub easing: String,
}

/// Keyframes Rule

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyframesRule {
    /// CSS keyframed animation's name.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of animation keyframes.

    pub keyframes: Vec<KeyframeStyle>,
}

/// Keyframe Style

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyframeStyle {
    /// Keyframe's time offset.

    pub offset: String,
    /// 'AnimationEffect''s timing function.

    pub easing: String,
}

/// Returns the current time of the an animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentTimeParams {
    /// Id of animation.

    pub id: String,
}

/// Returns the current time of the an animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentTimeReturns {
    /// Current time of the page.

    pub currentTime: f64,
}

/// Gets the playback rate of the document timeline.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaybackRateReturns {
    /// Playback rate for animations on page.

    pub playbackRate: f64,
}

/// Releases a set of animations to no longer be manipulated.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAnimationsParams {
    /// List of animation ids to seek.

    pub animations: Vec<String>,
}

/// Gets the remote object of the Animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveAnimationParams {
    /// Animation id.

    pub animationId: String,
}

/// Gets the remote object of the Animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveAnimationReturns {
    /// Corresponding remote object.

    pub remoteObject: crate::runtime::RemoteObject,
}

/// Seek a set of animations to a particular time within each animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SeekAnimationsParams {
    /// List of animation ids to seek.

    pub animations: Vec<String>,
    /// Set the current time of each animation.

    pub currentTime: f64,
}

/// Sets the paused state of a set of animations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPausedParams {
    /// Animations to set the pause state of.

    pub animations: Vec<String>,
    /// Paused state to set to.

    pub paused: bool,
}

/// Sets the playback rate of the document timeline.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPlaybackRateParams {
    /// Playback rate for animations on page

    pub playbackRate: f64,
}

/// Sets the timing of an animation node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTimingParams {
    /// Animation id.

    pub animationId: String,
    /// Duration of the animation.

    pub duration: f64,
    /// Delay of the animation.

    pub delay: f64,
}
