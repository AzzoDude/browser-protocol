use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Unique Layer identifier.

pub type LayerId = String;

/// Unique snapshot identifier.

pub type SnapshotId = String;

/// Rectangle where scrolling happens on the main thread.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollRect {
    /// Rectangle itself.

    pub rect: crate::dom::Rect,
    /// Reason for rectangle to force scrolling on the main thread

    #[serde(rename = "type")]
    pub type_: String,
}

/// Sticky position constraints.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StickyPositionConstraint {
    /// Layout rectangle of the sticky element before being shifted

    pub stickyBoxRect: crate::dom::Rect,
    /// Layout rectangle of the containing block of the sticky element

    pub containingBlockRect: crate::dom::Rect,
    /// The nearest sticky layer that shifts the sticky box

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearestLayerShiftingStickyBox: Option<LayerId>,
    /// The nearest sticky layer that shifts the containing block

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearestLayerShiftingContainingBlock: Option<LayerId>,
}

/// Serialized fragment of layer picture along with its offset within the layer.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PictureTile {
    /// Offset from owning layer left boundary

    pub x: f64,
    /// Offset from owning layer top boundary

    pub y: f64,
    /// Base64-encoded snapshot data. (Encoded as a base64 string when passed over JSON)

    pub picture: String,
}

/// Information about a compositing layer.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    /// The unique id for this layer.

    pub layerId: LayerId,
    /// The id of parent (not present for root).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentLayerId: Option<LayerId>,
    /// The backend id for the node associated with this layer.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
    /// Offset from parent layer, X coordinate.

    pub offsetX: f64,
    /// Offset from parent layer, Y coordinate.

    pub offsetY: f64,
    /// Layer width.

    pub width: f64,
    /// Layer height.

    pub height: f64,
    /// Transformation matrix for layer, default is identity matrix

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<f64>>,
    /// Transform anchor point X, absent if no transform specified

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchorX: Option<f64>,
    /// Transform anchor point Y, absent if no transform specified

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchorY: Option<f64>,
    /// Transform anchor point Z, absent if no transform specified

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchorZ: Option<f64>,
    /// Indicates how many time this layer has painted.

    pub paintCount: u64,
    /// Indicates whether this layer hosts any content, rather than being used for
    /// transform/scrolling purposes only.

    pub drawsContent: bool,
    /// Set if layer is not visible.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invisible: Option<bool>,
    /// Rectangles scrolling on main thread only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollRects: Option<Vec<ScrollRect>>,
    /// Sticky position constraint information

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickyPositionConstraint: Option<StickyPositionConstraint>,
}

/// Array of timings, one per paint step.

pub type PaintProfile = Vec<f64>;

/// Provides the reasons why the given layer was composited.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompositingReasonsParams {
    /// The id of the layer for which we want to get the reasons it was composited.

    pub layerId: LayerId,
}

/// Provides the reasons why the given layer was composited.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompositingReasonsReturns {
    /// A list of strings specifying reasons for the given layer to become composited.

    pub compositingReasons: Vec<String>,
    /// A list of strings specifying reason IDs for the given layer to become composited.

    pub compositingReasonIds: Vec<String>,
}

impl CompositingReasonsParams { pub const METHOD: &'static str = "LayerTree.compositingReasons"; }

impl crate::CdpCommand for CompositingReasonsParams {
    const METHOD: &'static str = "LayerTree.compositingReasons";
    type Response = CompositingReasonsReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "LayerTree.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "LayerTree.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "LayerTree.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "LayerTree.enable";
    type Response = crate::EmptyReturns;
}

/// Returns the snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadSnapshotParams {
    /// An array of tiles composing the snapshot.

    pub tiles: Vec<PictureTile>,
}

/// Returns the snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadSnapshotReturns {
    /// The id of the snapshot.

    pub snapshotId: SnapshotId,
}

impl LoadSnapshotParams { pub const METHOD: &'static str = "LayerTree.loadSnapshot"; }

impl crate::CdpCommand for LoadSnapshotParams {
    const METHOD: &'static str = "LayerTree.loadSnapshot";
    type Response = LoadSnapshotReturns;
}

/// Returns the layer snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MakeSnapshotParams {
    /// The id of the layer.

    pub layerId: LayerId,
}

/// Returns the layer snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MakeSnapshotReturns {
    /// The id of the layer snapshot.

    pub snapshotId: SnapshotId,
}

impl MakeSnapshotParams { pub const METHOD: &'static str = "LayerTree.makeSnapshot"; }

impl crate::CdpCommand for MakeSnapshotParams {
    const METHOD: &'static str = "LayerTree.makeSnapshot";
    type Response = MakeSnapshotReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSnapshotParams {
    /// The id of the layer snapshot.

    pub snapshotId: SnapshotId,
    /// The maximum number of times to replay the snapshot (1, if not specified).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minRepeatCount: Option<u64>,
    /// The minimum duration (in seconds) to replay the snapshot.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minDuration: Option<f64>,
    /// The clip rectangle to apply when replaying the snapshot.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clipRect: Option<crate::dom::Rect>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSnapshotReturns {
    /// The array of paint profiles, one per run.

    pub timings: Vec<PaintProfile>,
}

impl ProfileSnapshotParams { pub const METHOD: &'static str = "LayerTree.profileSnapshot"; }

impl crate::CdpCommand for ProfileSnapshotParams {
    const METHOD: &'static str = "LayerTree.profileSnapshot";
    type Response = ProfileSnapshotReturns;
}

/// Releases layer snapshot captured by the back-end.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseSnapshotParams {
    /// The id of the layer snapshot.

    pub snapshotId: SnapshotId,
}

impl ReleaseSnapshotParams { pub const METHOD: &'static str = "LayerTree.releaseSnapshot"; }

impl crate::CdpCommand for ReleaseSnapshotParams {
    const METHOD: &'static str = "LayerTree.releaseSnapshot";
    type Response = crate::EmptyReturns;
}

/// Replays the layer snapshot and returns the resulting bitmap.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplaySnapshotParams {
    /// The id of the layer snapshot.

    pub snapshotId: SnapshotId,
    /// The first step to replay from (replay from the very start if not specified).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromStep: Option<i64>,
    /// The last step to replay to (replay till the end if not specified).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub toStep: Option<i64>,
    /// The scale to apply while replaying (defaults to 1).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}

/// Replays the layer snapshot and returns the resulting bitmap.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplaySnapshotReturns {
    /// A data: URL for resulting image.

    pub dataURL: String,
}

impl ReplaySnapshotParams { pub const METHOD: &'static str = "LayerTree.replaySnapshot"; }

impl crate::CdpCommand for ReplaySnapshotParams {
    const METHOD: &'static str = "LayerTree.replaySnapshot";
    type Response = ReplaySnapshotReturns;
}

/// Replays the layer snapshot and returns canvas log.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotCommandLogParams {
    /// The id of the layer snapshot.

    pub snapshotId: SnapshotId,
}

/// Replays the layer snapshot and returns canvas log.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotCommandLogReturns {
    /// The array of canvas function calls.

    pub commandLog: Vec<serde_json::Map<String, JsonValue>>,
}

impl SnapshotCommandLogParams { pub const METHOD: &'static str = "LayerTree.snapshotCommandLog"; }

impl crate::CdpCommand for SnapshotCommandLogParams {
    const METHOD: &'static str = "LayerTree.snapshotCommandLog";
    type Response = SnapshotCommandLogReturns;
}
