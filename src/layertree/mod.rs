use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Unique Layer identifier.

pub type LayerId<'a> = Cow<'a, str>;

/// Unique snapshot identifier.

pub type SnapshotId<'a> = Cow<'a, str>;

/// Rectangle where scrolling happens on the main thread.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScrollRect<'a> {
    /// Rectangle itself.
    pub rect: crate::dom::Rect,
    /// Reason for rectangle to force scrolling on the main thread
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
}
/// Sticky position constraints.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StickyPositionConstraint<'a> {
    /// Layout rectangle of the sticky element before being shifted
    #[serde(rename = "stickyBoxRect")]
    pub sticky_box_rect: crate::dom::Rect,
    /// Layout rectangle of the containing block of the sticky element
    #[serde(rename = "containingBlockRect")]
    pub containing_block_rect: crate::dom::Rect,
    /// The nearest sticky layer that shifts the sticky box
    #[serde(skip_serializing_if = "Option::is_none", rename = "nearestLayerShiftingStickyBox")]
    pub nearest_layer_shifting_sticky_box: Option<LayerId<'a>>,
    /// The nearest sticky layer that shifts the containing block
    #[serde(skip_serializing_if = "Option::is_none", rename = "nearestLayerShiftingContainingBlock")]
    pub nearest_layer_shifting_containing_block: Option<LayerId<'a>>,
}
/// Serialized fragment of layer picture along with its offset within the layer.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PictureTile<'a> {
    /// Offset from owning layer left boundary
    pub x: f64,
    /// Offset from owning layer top boundary
    pub y: f64,
    /// Base64-encoded snapshot data. (Encoded as a base64 string when passed over JSON)
    pub picture: Cow<'a, str>,
}
/// Information about a compositing layer.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Layer<'a> {
    /// The unique id for this layer.
    #[serde(rename = "layerId")]
    pub layer_id: LayerId<'a>,
    /// The id of parent (not present for root).
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentLayerId")]
    pub parent_layer_id: Option<LayerId<'a>>,
    /// The backend id for the node associated with this layer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
    /// Offset from parent layer, X coordinate.
    #[serde(rename = "offsetX")]
    pub offset_x: f64,
    /// Offset from parent layer, Y coordinate.
    #[serde(rename = "offsetY")]
    pub offset_y: f64,
    /// Layer width.
    pub width: f64,
    /// Layer height.
    pub height: f64,
    /// Transformation matrix for layer, default is identity matrix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<f64>>,
    /// Transform anchor point X, absent if no transform specified
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorX")]
    pub anchor_x: Option<f64>,
    /// Transform anchor point Y, absent if no transform specified
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorY")]
    pub anchor_y: Option<f64>,
    /// Transform anchor point Z, absent if no transform specified
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorZ")]
    pub anchor_z: Option<f64>,
    /// Indicates how many time this layer has painted.
    #[serde(rename = "paintCount")]
    pub paint_count: u64,
    /// Indicates whether this layer hosts any content, rather than being used for
    /// transform/scrolling purposes only.
    #[serde(rename = "drawsContent")]
    pub draws_content: bool,
    /// Set if layer is not visible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invisible: Option<bool>,
    /// Rectangles scrolling on main thread only.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollRects")]
    pub scroll_rects: Option<Vec<ScrollRect<'a>>>,
    /// Sticky position constraint information
    #[serde(skip_serializing_if = "Option::is_none", rename = "stickyPositionConstraint")]
    pub sticky_position_constraint: Option<StickyPositionConstraint<'a>>,
}
/// Array of timings, one per paint step.

pub type PaintProfile = Vec<f64>;

/// Provides the reasons why the given layer was composited.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.compositingReasons", response = "CompositingReasonsReturns<'a>")]
pub struct CompositingReasonsParams<'a> {
    /// The id of the layer for which we want to get the reasons it was composited.
    #[serde(rename = "layerId")]
    pub layer_id: LayerId<'a>,
}
/// Provides the reasons why the given layer was composited.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CompositingReasonsReturns<'a> {
    /// A list of strings specifying reasons for the given layer to become composited.
    #[serde(rename = "compositingReasons")]
    pub compositing_reasons: Vec<Cow<'a, str>>,
    /// A list of strings specifying reason IDs for the given layer to become composited.
    #[serde(rename = "compositingReasonIds")]
    pub compositing_reason_ids: Vec<Cow<'a, str>>,
}
/// Disables compositing tree inspection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.disable")]
pub struct DisableParams {

}
/// Enables compositing tree inspection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.enable")]
pub struct EnableParams {

}
/// Returns the snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.loadSnapshot", response = "LoadSnapshotReturns<'a>")]
pub struct LoadSnapshotParams<'a> {
    /// An array of tiles composing the snapshot.
    pub tiles: Vec<PictureTile<'a>>,
}
/// Returns the snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LoadSnapshotReturns<'a> {
    /// The id of the snapshot.
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId<'a>,
}
/// Returns the layer snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.makeSnapshot", response = "MakeSnapshotReturns<'a>")]
pub struct MakeSnapshotParams<'a> {
    /// The id of the layer.
    #[serde(rename = "layerId")]
    pub layer_id: LayerId<'a>,
}
/// Returns the layer snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MakeSnapshotReturns<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.profileSnapshot", response = "ProfileSnapshotReturns")]
pub struct ProfileSnapshotParams<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId<'a>,
    /// The maximum number of times to replay the snapshot (1, if not specified).
    #[serde(skip_serializing_if = "Option::is_none", rename = "minRepeatCount")]
    pub min_repeat_count: Option<u64>,
    /// The minimum duration (in seconds) to replay the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "minDuration")]
    pub min_duration: Option<f64>,
    /// The clip rectangle to apply when replaying the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "clipRect")]
    pub clip_rect: Option<crate::dom::Rect>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSnapshotReturns {
    /// The array of paint profiles, one per run.
    pub timings: Vec<PaintProfile>,
}
/// Releases layer snapshot captured by the back-end.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.releaseSnapshot")]
pub struct ReleaseSnapshotParams<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId<'a>,
}
/// Replays the layer snapshot and returns the resulting bitmap.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.replaySnapshot", response = "ReplaySnapshotReturns<'a>")]
pub struct ReplaySnapshotParams<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId<'a>,
    /// The first step to replay from (replay from the very start if not specified).
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromStep")]
    pub from_step: Option<i64>,
    /// The last step to replay to (replay till the end if not specified).
    #[serde(skip_serializing_if = "Option::is_none", rename = "toStep")]
    pub to_step: Option<i64>,
    /// The scale to apply while replaying (defaults to 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}
/// Replays the layer snapshot and returns the resulting bitmap.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ReplaySnapshotReturns<'a> {
    /// A data: URL for resulting image.
    #[serde(rename = "dataURL")]
    pub data_url: Cow<'a, str>,
}
/// Replays the layer snapshot and returns canvas log.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.snapshotCommandLog", response = "SnapshotCommandLogReturns")]
pub struct SnapshotCommandLogParams<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    pub snapshot_id: SnapshotId<'a>,
}
/// Replays the layer snapshot and returns canvas log.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotCommandLogReturns {
    /// The array of canvas function calls.
    #[serde(rename = "commandLog")]
    pub command_log: Vec<serde_json::Map<String, JsonValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.layerPainted")]
pub struct LayerPainted<'a> {
    /// The id of the painted layer.
    #[serde(rename = "layerId")]
    pub layer_id: LayerId<'a>,
    /// Clip rectangle.
    pub clip: crate::dom::Rect,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "LayerTree.layerTreeDidChange")]
pub struct LayerTreeDidChange<'a> {
    /// Layer tree, absent if not in the compositing mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer<'a>>>,
}