use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Unique Layer identifier.

pub type LayerId<'a> = Cow<'a, str>;

/// Unique snapshot identifier.

pub type SnapshotId<'a> = Cow<'a, str>;

/// Rectangle where scrolling happens on the main thread.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollRect<'a> {
    /// Rectangle itself.
    rect: crate::dom::Rect,
    /// Reason for rectangle to force scrolling on the main thread
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
}

impl<'a> ScrollRect<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `rect`: Rectangle itself.
    /// * `type_`: Reason for rectangle to force scrolling on the main thread
    pub fn builder(rect: crate::dom::Rect, type_: impl Into<Cow<'a, str>>) -> ScrollRectBuilder<'a> {
        ScrollRectBuilder {
            rect: rect,
            type_: type_.into(),
        }
    }
    /// Rectangle itself.
    pub fn rect(&self) -> &crate::dom::Rect { &self.rect }
    /// Reason for rectangle to force scrolling on the main thread
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}


pub struct ScrollRectBuilder<'a> {
    rect: crate::dom::Rect,
    type_: Cow<'a, str>,
}

impl<'a> ScrollRectBuilder<'a> {
    pub fn build(self) -> ScrollRect<'a> {
        ScrollRect {
            rect: self.rect,
            type_: self.type_,
        }
    }
}

/// Sticky position constraints.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StickyPositionConstraint<'a> {
    /// Layout rectangle of the sticky element before being shifted
    #[serde(rename = "stickyBoxRect")]
    sticky_box_rect: crate::dom::Rect,
    /// Layout rectangle of the containing block of the sticky element
    #[serde(rename = "containingBlockRect")]
    containing_block_rect: crate::dom::Rect,
    /// The nearest sticky layer that shifts the sticky box
    #[serde(skip_serializing_if = "Option::is_none", rename = "nearestLayerShiftingStickyBox")]
    nearest_layer_shifting_sticky_box: Option<LayerId<'a>>,
    /// The nearest sticky layer that shifts the containing block
    #[serde(skip_serializing_if = "Option::is_none", rename = "nearestLayerShiftingContainingBlock")]
    nearest_layer_shifting_containing_block: Option<LayerId<'a>>,
}

impl<'a> StickyPositionConstraint<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `sticky_box_rect`: Layout rectangle of the sticky element before being shifted
    /// * `containing_block_rect`: Layout rectangle of the containing block of the sticky element
    pub fn builder(sticky_box_rect: crate::dom::Rect, containing_block_rect: crate::dom::Rect) -> StickyPositionConstraintBuilder<'a> {
        StickyPositionConstraintBuilder {
            sticky_box_rect: sticky_box_rect,
            containing_block_rect: containing_block_rect,
            nearest_layer_shifting_sticky_box: None,
            nearest_layer_shifting_containing_block: None,
        }
    }
    /// Layout rectangle of the sticky element before being shifted
    pub fn sticky_box_rect(&self) -> &crate::dom::Rect { &self.sticky_box_rect }
    /// Layout rectangle of the containing block of the sticky element
    pub fn containing_block_rect(&self) -> &crate::dom::Rect { &self.containing_block_rect }
    /// The nearest sticky layer that shifts the sticky box
    pub fn nearest_layer_shifting_sticky_box(&self) -> Option<&LayerId<'a>> { self.nearest_layer_shifting_sticky_box.as_ref() }
    /// The nearest sticky layer that shifts the containing block
    pub fn nearest_layer_shifting_containing_block(&self) -> Option<&LayerId<'a>> { self.nearest_layer_shifting_containing_block.as_ref() }
}


pub struct StickyPositionConstraintBuilder<'a> {
    sticky_box_rect: crate::dom::Rect,
    containing_block_rect: crate::dom::Rect,
    nearest_layer_shifting_sticky_box: Option<LayerId<'a>>,
    nearest_layer_shifting_containing_block: Option<LayerId<'a>>,
}

impl<'a> StickyPositionConstraintBuilder<'a> {
    /// The nearest sticky layer that shifts the sticky box
    pub fn nearest_layer_shifting_sticky_box(mut self, nearest_layer_shifting_sticky_box: impl Into<LayerId<'a>>) -> Self { self.nearest_layer_shifting_sticky_box = Some(nearest_layer_shifting_sticky_box.into()); self }
    /// The nearest sticky layer that shifts the containing block
    pub fn nearest_layer_shifting_containing_block(mut self, nearest_layer_shifting_containing_block: impl Into<LayerId<'a>>) -> Self { self.nearest_layer_shifting_containing_block = Some(nearest_layer_shifting_containing_block.into()); self }
    pub fn build(self) -> StickyPositionConstraint<'a> {
        StickyPositionConstraint {
            sticky_box_rect: self.sticky_box_rect,
            containing_block_rect: self.containing_block_rect,
            nearest_layer_shifting_sticky_box: self.nearest_layer_shifting_sticky_box,
            nearest_layer_shifting_containing_block: self.nearest_layer_shifting_containing_block,
        }
    }
}

/// Serialized fragment of layer picture along with its offset within the layer.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PictureTile<'a> {
    /// Offset from owning layer left boundary
    x: f64,
    /// Offset from owning layer top boundary
    y: f64,
    /// Base64-encoded snapshot data. (Encoded as a base64 string when passed over JSON)
    picture: Cow<'a, str>,
}

impl<'a> PictureTile<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `x`: Offset from owning layer left boundary
    /// * `y`: Offset from owning layer top boundary
    /// * `picture`: Base64-encoded snapshot data. (Encoded as a base64 string when passed over JSON)
    pub fn builder(x: f64, y: f64, picture: impl Into<Cow<'a, str>>) -> PictureTileBuilder<'a> {
        PictureTileBuilder {
            x: x,
            y: y,
            picture: picture.into(),
        }
    }
    /// Offset from owning layer left boundary
    pub fn x(&self) -> f64 { self.x }
    /// Offset from owning layer top boundary
    pub fn y(&self) -> f64 { self.y }
    /// Base64-encoded snapshot data. (Encoded as a base64 string when passed over JSON)
    pub fn picture(&self) -> &str { self.picture.as_ref() }
}


pub struct PictureTileBuilder<'a> {
    x: f64,
    y: f64,
    picture: Cow<'a, str>,
}

impl<'a> PictureTileBuilder<'a> {
    pub fn build(self) -> PictureTile<'a> {
        PictureTile {
            x: self.x,
            y: self.y,
            picture: self.picture,
        }
    }
}

/// Information about a compositing layer.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Layer<'a> {
    /// The unique id for this layer.
    #[serde(rename = "layerId")]
    layer_id: LayerId<'a>,
    /// The id of parent (not present for root).
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentLayerId")]
    parent_layer_id: Option<LayerId<'a>>,
    /// The backend id for the node associated with this layer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
    /// Offset from parent layer, X coordinate.
    #[serde(rename = "offsetX")]
    offset_x: f64,
    /// Offset from parent layer, Y coordinate.
    #[serde(rename = "offsetY")]
    offset_y: f64,
    /// Layer width.
    width: f64,
    /// Layer height.
    height: f64,
    /// Transformation matrix for layer, default is identity matrix
    #[serde(skip_serializing_if = "Option::is_none")]
    transform: Option<Vec<f64>>,
    /// Transform anchor point X, absent if no transform specified
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorX")]
    anchor_x: Option<f64>,
    /// Transform anchor point Y, absent if no transform specified
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorY")]
    anchor_y: Option<f64>,
    /// Transform anchor point Z, absent if no transform specified
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorZ")]
    anchor_z: Option<f64>,
    /// Indicates how many time this layer has painted.
    #[serde(rename = "paintCount")]
    paint_count: u64,
    /// Indicates whether this layer hosts any content, rather than being used for
    /// transform/scrolling purposes only.
    #[serde(rename = "drawsContent")]
    draws_content: bool,
    /// Set if layer is not visible.
    #[serde(skip_serializing_if = "Option::is_none")]
    invisible: Option<bool>,
    /// Rectangles scrolling on main thread only.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollRects")]
    scroll_rects: Option<Vec<ScrollRect<'a>>>,
    /// Sticky position constraint information
    #[serde(skip_serializing_if = "Option::is_none", rename = "stickyPositionConstraint")]
    sticky_position_constraint: Option<StickyPositionConstraint<'a>>,
}

impl<'a> Layer<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `layer_id`: The unique id for this layer.
    /// * `offset_x`: Offset from parent layer, X coordinate.
    /// * `offset_y`: Offset from parent layer, Y coordinate.
    /// * `width`: Layer width.
    /// * `height`: Layer height.
    /// * `paint_count`: Indicates how many time this layer has painted.
    /// * `draws_content`: Indicates whether this layer hosts any content, rather than being used for transform/scrolling purposes only.
    pub fn builder(layer_id: impl Into<LayerId<'a>>, offset_x: f64, offset_y: f64, width: f64, height: f64, paint_count: u64, draws_content: bool) -> LayerBuilder<'a> {
        LayerBuilder {
            layer_id: layer_id.into(),
            parent_layer_id: None,
            backend_node_id: None,
            offset_x: offset_x,
            offset_y: offset_y,
            width: width,
            height: height,
            transform: None,
            anchor_x: None,
            anchor_y: None,
            anchor_z: None,
            paint_count: paint_count,
            draws_content: draws_content,
            invisible: None,
            scroll_rects: None,
            sticky_position_constraint: None,
        }
    }
    /// The unique id for this layer.
    pub fn layer_id(&self) -> &LayerId<'a> { &self.layer_id }
    /// The id of parent (not present for root).
    pub fn parent_layer_id(&self) -> Option<&LayerId<'a>> { self.parent_layer_id.as_ref() }
    /// The backend id for the node associated with this layer.
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
    /// Offset from parent layer, X coordinate.
    pub fn offset_x(&self) -> f64 { self.offset_x }
    /// Offset from parent layer, Y coordinate.
    pub fn offset_y(&self) -> f64 { self.offset_y }
    /// Layer width.
    pub fn width(&self) -> f64 { self.width }
    /// Layer height.
    pub fn height(&self) -> f64 { self.height }
    /// Transformation matrix for layer, default is identity matrix
    pub fn transform(&self) -> Option<&[f64]> { self.transform.as_deref() }
    /// Transform anchor point X, absent if no transform specified
    pub fn anchor_x(&self) -> Option<f64> { self.anchor_x }
    /// Transform anchor point Y, absent if no transform specified
    pub fn anchor_y(&self) -> Option<f64> { self.anchor_y }
    /// Transform anchor point Z, absent if no transform specified
    pub fn anchor_z(&self) -> Option<f64> { self.anchor_z }
    /// Indicates how many time this layer has painted.
    pub fn paint_count(&self) -> u64 { self.paint_count }
    /// Indicates whether this layer hosts any content, rather than being used for
    /// transform/scrolling purposes only.
    pub fn draws_content(&self) -> bool { self.draws_content }
    /// Set if layer is not visible.
    pub fn invisible(&self) -> Option<bool> { self.invisible }
    /// Rectangles scrolling on main thread only.
    pub fn scroll_rects(&self) -> Option<&[ScrollRect<'a>]> { self.scroll_rects.as_deref() }
    /// Sticky position constraint information
    pub fn sticky_position_constraint(&self) -> Option<&StickyPositionConstraint<'a>> { self.sticky_position_constraint.as_ref() }
}


pub struct LayerBuilder<'a> {
    layer_id: LayerId<'a>,
    parent_layer_id: Option<LayerId<'a>>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
    offset_x: f64,
    offset_y: f64,
    width: f64,
    height: f64,
    transform: Option<Vec<f64>>,
    anchor_x: Option<f64>,
    anchor_y: Option<f64>,
    anchor_z: Option<f64>,
    paint_count: u64,
    draws_content: bool,
    invisible: Option<bool>,
    scroll_rects: Option<Vec<ScrollRect<'a>>>,
    sticky_position_constraint: Option<StickyPositionConstraint<'a>>,
}

impl<'a> LayerBuilder<'a> {
    /// The id of parent (not present for root).
    pub fn parent_layer_id(mut self, parent_layer_id: impl Into<LayerId<'a>>) -> Self { self.parent_layer_id = Some(parent_layer_id.into()); self }
    /// The backend id for the node associated with this layer.
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// Transformation matrix for layer, default is identity matrix
    pub fn transform(mut self, transform: Vec<f64>) -> Self { self.transform = Some(transform); self }
    /// Transform anchor point X, absent if no transform specified
    pub fn anchor_x(mut self, anchor_x: f64) -> Self { self.anchor_x = Some(anchor_x); self }
    /// Transform anchor point Y, absent if no transform specified
    pub fn anchor_y(mut self, anchor_y: f64) -> Self { self.anchor_y = Some(anchor_y); self }
    /// Transform anchor point Z, absent if no transform specified
    pub fn anchor_z(mut self, anchor_z: f64) -> Self { self.anchor_z = Some(anchor_z); self }
    /// Set if layer is not visible.
    pub fn invisible(mut self, invisible: bool) -> Self { self.invisible = Some(invisible); self }
    /// Rectangles scrolling on main thread only.
    pub fn scroll_rects(mut self, scroll_rects: Vec<ScrollRect<'a>>) -> Self { self.scroll_rects = Some(scroll_rects); self }
    /// Sticky position constraint information
    pub fn sticky_position_constraint(mut self, sticky_position_constraint: StickyPositionConstraint<'a>) -> Self { self.sticky_position_constraint = Some(sticky_position_constraint); self }
    pub fn build(self) -> Layer<'a> {
        Layer {
            layer_id: self.layer_id,
            parent_layer_id: self.parent_layer_id,
            backend_node_id: self.backend_node_id,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            width: self.width,
            height: self.height,
            transform: self.transform,
            anchor_x: self.anchor_x,
            anchor_y: self.anchor_y,
            anchor_z: self.anchor_z,
            paint_count: self.paint_count,
            draws_content: self.draws_content,
            invisible: self.invisible,
            scroll_rects: self.scroll_rects,
            sticky_position_constraint: self.sticky_position_constraint,
        }
    }
}

/// Array of timings, one per paint step.

pub type PaintProfile = Vec<f64>;

/// Provides the reasons why the given layer was composited.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompositingReasonsParams<'a> {
    /// The id of the layer for which we want to get the reasons it was composited.
    #[serde(rename = "layerId")]
    layer_id: LayerId<'a>,
}

impl<'a> CompositingReasonsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `layer_id`: The id of the layer for which we want to get the reasons it was composited.
    pub fn builder(layer_id: impl Into<LayerId<'a>>) -> CompositingReasonsParamsBuilder<'a> {
        CompositingReasonsParamsBuilder {
            layer_id: layer_id.into(),
        }
    }
    /// The id of the layer for which we want to get the reasons it was composited.
    pub fn layer_id(&self) -> &LayerId<'a> { &self.layer_id }
}


pub struct CompositingReasonsParamsBuilder<'a> {
    layer_id: LayerId<'a>,
}

impl<'a> CompositingReasonsParamsBuilder<'a> {
    pub fn build(self) -> CompositingReasonsParams<'a> {
        CompositingReasonsParams {
            layer_id: self.layer_id,
        }
    }
}

/// Provides the reasons why the given layer was composited.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompositingReasonsReturns<'a> {
    /// A list of strings specifying reasons for the given layer to become composited.
    #[serde(rename = "compositingReasons")]
    compositing_reasons: Vec<Cow<'a, str>>,
    /// A list of strings specifying reason IDs for the given layer to become composited.
    #[serde(rename = "compositingReasonIds")]
    compositing_reason_ids: Vec<Cow<'a, str>>,
}

impl<'a> CompositingReasonsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `compositing_reasons`: A list of strings specifying reasons for the given layer to become composited.
    /// * `compositing_reason_ids`: A list of strings specifying reason IDs for the given layer to become composited.
    pub fn builder(compositing_reasons: Vec<Cow<'a, str>>, compositing_reason_ids: Vec<Cow<'a, str>>) -> CompositingReasonsReturnsBuilder<'a> {
        CompositingReasonsReturnsBuilder {
            compositing_reasons: compositing_reasons,
            compositing_reason_ids: compositing_reason_ids,
        }
    }
    /// A list of strings specifying reasons for the given layer to become composited.
    pub fn compositing_reasons(&self) -> &[Cow<'a, str>] { &self.compositing_reasons }
    /// A list of strings specifying reason IDs for the given layer to become composited.
    pub fn compositing_reason_ids(&self) -> &[Cow<'a, str>] { &self.compositing_reason_ids }
}


pub struct CompositingReasonsReturnsBuilder<'a> {
    compositing_reasons: Vec<Cow<'a, str>>,
    compositing_reason_ids: Vec<Cow<'a, str>>,
}

impl<'a> CompositingReasonsReturnsBuilder<'a> {
    pub fn build(self) -> CompositingReasonsReturns<'a> {
        CompositingReasonsReturns {
            compositing_reasons: self.compositing_reasons,
            compositing_reason_ids: self.compositing_reason_ids,
        }
    }
}

impl<'a> CompositingReasonsParams<'a> { pub const METHOD: &'static str = "LayerTree.compositingReasons"; }

impl<'a> crate::CdpCommand<'a> for CompositingReasonsParams<'a> {
    const METHOD: &'static str = "LayerTree.compositingReasons";
    type Response = CompositingReasonsReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "LayerTree.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "LayerTree.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "LayerTree.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "LayerTree.enable";
    type Response = crate::EmptyReturns;
}

/// Returns the snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadSnapshotParams<'a> {
    /// An array of tiles composing the snapshot.
    tiles: Vec<PictureTile<'a>>,
}

impl<'a> LoadSnapshotParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `tiles`: An array of tiles composing the snapshot.
    pub fn builder(tiles: Vec<PictureTile<'a>>) -> LoadSnapshotParamsBuilder<'a> {
        LoadSnapshotParamsBuilder {
            tiles: tiles,
        }
    }
    /// An array of tiles composing the snapshot.
    pub fn tiles(&self) -> &[PictureTile<'a>] { &self.tiles }
}


pub struct LoadSnapshotParamsBuilder<'a> {
    tiles: Vec<PictureTile<'a>>,
}

impl<'a> LoadSnapshotParamsBuilder<'a> {
    pub fn build(self) -> LoadSnapshotParams<'a> {
        LoadSnapshotParams {
            tiles: self.tiles,
        }
    }
}

/// Returns the snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadSnapshotReturns<'a> {
    /// The id of the snapshot.
    #[serde(rename = "snapshotId")]
    snapshot_id: SnapshotId<'a>,
}

impl<'a> LoadSnapshotReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `snapshot_id`: The id of the snapshot.
    pub fn builder(snapshot_id: impl Into<SnapshotId<'a>>) -> LoadSnapshotReturnsBuilder<'a> {
        LoadSnapshotReturnsBuilder {
            snapshot_id: snapshot_id.into(),
        }
    }
    /// The id of the snapshot.
    pub fn snapshot_id(&self) -> &SnapshotId<'a> { &self.snapshot_id }
}


pub struct LoadSnapshotReturnsBuilder<'a> {
    snapshot_id: SnapshotId<'a>,
}

impl<'a> LoadSnapshotReturnsBuilder<'a> {
    pub fn build(self) -> LoadSnapshotReturns<'a> {
        LoadSnapshotReturns {
            snapshot_id: self.snapshot_id,
        }
    }
}

impl<'a> LoadSnapshotParams<'a> { pub const METHOD: &'static str = "LayerTree.loadSnapshot"; }

impl<'a> crate::CdpCommand<'a> for LoadSnapshotParams<'a> {
    const METHOD: &'static str = "LayerTree.loadSnapshot";
    type Response = LoadSnapshotReturns<'a>;
}

/// Returns the layer snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MakeSnapshotParams<'a> {
    /// The id of the layer.
    #[serde(rename = "layerId")]
    layer_id: LayerId<'a>,
}

impl<'a> MakeSnapshotParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `layer_id`: The id of the layer.
    pub fn builder(layer_id: impl Into<LayerId<'a>>) -> MakeSnapshotParamsBuilder<'a> {
        MakeSnapshotParamsBuilder {
            layer_id: layer_id.into(),
        }
    }
    /// The id of the layer.
    pub fn layer_id(&self) -> &LayerId<'a> { &self.layer_id }
}


pub struct MakeSnapshotParamsBuilder<'a> {
    layer_id: LayerId<'a>,
}

impl<'a> MakeSnapshotParamsBuilder<'a> {
    pub fn build(self) -> MakeSnapshotParams<'a> {
        MakeSnapshotParams {
            layer_id: self.layer_id,
        }
    }
}

/// Returns the layer snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MakeSnapshotReturns<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    snapshot_id: SnapshotId<'a>,
}

impl<'a> MakeSnapshotReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `snapshot_id`: The id of the layer snapshot.
    pub fn builder(snapshot_id: impl Into<SnapshotId<'a>>) -> MakeSnapshotReturnsBuilder<'a> {
        MakeSnapshotReturnsBuilder {
            snapshot_id: snapshot_id.into(),
        }
    }
    /// The id of the layer snapshot.
    pub fn snapshot_id(&self) -> &SnapshotId<'a> { &self.snapshot_id }
}


pub struct MakeSnapshotReturnsBuilder<'a> {
    snapshot_id: SnapshotId<'a>,
}

impl<'a> MakeSnapshotReturnsBuilder<'a> {
    pub fn build(self) -> MakeSnapshotReturns<'a> {
        MakeSnapshotReturns {
            snapshot_id: self.snapshot_id,
        }
    }
}

impl<'a> MakeSnapshotParams<'a> { pub const METHOD: &'static str = "LayerTree.makeSnapshot"; }

impl<'a> crate::CdpCommand<'a> for MakeSnapshotParams<'a> {
    const METHOD: &'static str = "LayerTree.makeSnapshot";
    type Response = MakeSnapshotReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSnapshotParams<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    snapshot_id: SnapshotId<'a>,
    /// The maximum number of times to replay the snapshot (1, if not specified).
    #[serde(skip_serializing_if = "Option::is_none", rename = "minRepeatCount")]
    min_repeat_count: Option<u64>,
    /// The minimum duration (in seconds) to replay the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "minDuration")]
    min_duration: Option<f64>,
    /// The clip rectangle to apply when replaying the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "clipRect")]
    clip_rect: Option<crate::dom::Rect>,
}

impl<'a> ProfileSnapshotParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `snapshot_id`: The id of the layer snapshot.
    pub fn builder(snapshot_id: impl Into<SnapshotId<'a>>) -> ProfileSnapshotParamsBuilder<'a> {
        ProfileSnapshotParamsBuilder {
            snapshot_id: snapshot_id.into(),
            min_repeat_count: None,
            min_duration: None,
            clip_rect: None,
        }
    }
    /// The id of the layer snapshot.
    pub fn snapshot_id(&self) -> &SnapshotId<'a> { &self.snapshot_id }
    /// The maximum number of times to replay the snapshot (1, if not specified).
    pub fn min_repeat_count(&self) -> Option<u64> { self.min_repeat_count }
    /// The minimum duration (in seconds) to replay the snapshot.
    pub fn min_duration(&self) -> Option<f64> { self.min_duration }
    /// The clip rectangle to apply when replaying the snapshot.
    pub fn clip_rect(&self) -> Option<&crate::dom::Rect> { self.clip_rect.as_ref() }
}


pub struct ProfileSnapshotParamsBuilder<'a> {
    snapshot_id: SnapshotId<'a>,
    min_repeat_count: Option<u64>,
    min_duration: Option<f64>,
    clip_rect: Option<crate::dom::Rect>,
}

impl<'a> ProfileSnapshotParamsBuilder<'a> {
    /// The maximum number of times to replay the snapshot (1, if not specified).
    pub fn min_repeat_count(mut self, min_repeat_count: u64) -> Self { self.min_repeat_count = Some(min_repeat_count); self }
    /// The minimum duration (in seconds) to replay the snapshot.
    pub fn min_duration(mut self, min_duration: f64) -> Self { self.min_duration = Some(min_duration); self }
    /// The clip rectangle to apply when replaying the snapshot.
    pub fn clip_rect(mut self, clip_rect: crate::dom::Rect) -> Self { self.clip_rect = Some(clip_rect); self }
    pub fn build(self) -> ProfileSnapshotParams<'a> {
        ProfileSnapshotParams {
            snapshot_id: self.snapshot_id,
            min_repeat_count: self.min_repeat_count,
            min_duration: self.min_duration,
            clip_rect: self.clip_rect,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSnapshotReturns {
    /// The array of paint profiles, one per run.
    timings: Vec<PaintProfile>,
}

impl ProfileSnapshotReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `timings`: The array of paint profiles, one per run.
    pub fn builder(timings: Vec<PaintProfile>) -> ProfileSnapshotReturnsBuilder {
        ProfileSnapshotReturnsBuilder {
            timings: timings,
        }
    }
    /// The array of paint profiles, one per run.
    pub fn timings(&self) -> &[PaintProfile] { &self.timings }
}


pub struct ProfileSnapshotReturnsBuilder {
    timings: Vec<PaintProfile>,
}

impl ProfileSnapshotReturnsBuilder {
    pub fn build(self) -> ProfileSnapshotReturns {
        ProfileSnapshotReturns {
            timings: self.timings,
        }
    }
}

impl<'a> ProfileSnapshotParams<'a> { pub const METHOD: &'static str = "LayerTree.profileSnapshot"; }

impl<'a> crate::CdpCommand<'a> for ProfileSnapshotParams<'a> {
    const METHOD: &'static str = "LayerTree.profileSnapshot";
    type Response = ProfileSnapshotReturns;
}

/// Releases layer snapshot captured by the back-end.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseSnapshotParams<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    snapshot_id: SnapshotId<'a>,
}

impl<'a> ReleaseSnapshotParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `snapshot_id`: The id of the layer snapshot.
    pub fn builder(snapshot_id: impl Into<SnapshotId<'a>>) -> ReleaseSnapshotParamsBuilder<'a> {
        ReleaseSnapshotParamsBuilder {
            snapshot_id: snapshot_id.into(),
        }
    }
    /// The id of the layer snapshot.
    pub fn snapshot_id(&self) -> &SnapshotId<'a> { &self.snapshot_id }
}


pub struct ReleaseSnapshotParamsBuilder<'a> {
    snapshot_id: SnapshotId<'a>,
}

impl<'a> ReleaseSnapshotParamsBuilder<'a> {
    pub fn build(self) -> ReleaseSnapshotParams<'a> {
        ReleaseSnapshotParams {
            snapshot_id: self.snapshot_id,
        }
    }
}

impl<'a> ReleaseSnapshotParams<'a> { pub const METHOD: &'static str = "LayerTree.releaseSnapshot"; }

impl<'a> crate::CdpCommand<'a> for ReleaseSnapshotParams<'a> {
    const METHOD: &'static str = "LayerTree.releaseSnapshot";
    type Response = crate::EmptyReturns;
}

/// Replays the layer snapshot and returns the resulting bitmap.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplaySnapshotParams<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    snapshot_id: SnapshotId<'a>,
    /// The first step to replay from (replay from the very start if not specified).
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromStep")]
    from_step: Option<i64>,
    /// The last step to replay to (replay till the end if not specified).
    #[serde(skip_serializing_if = "Option::is_none", rename = "toStep")]
    to_step: Option<i64>,
    /// The scale to apply while replaying (defaults to 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f64>,
}

impl<'a> ReplaySnapshotParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `snapshot_id`: The id of the layer snapshot.
    pub fn builder(snapshot_id: impl Into<SnapshotId<'a>>) -> ReplaySnapshotParamsBuilder<'a> {
        ReplaySnapshotParamsBuilder {
            snapshot_id: snapshot_id.into(),
            from_step: None,
            to_step: None,
            scale: None,
        }
    }
    /// The id of the layer snapshot.
    pub fn snapshot_id(&self) -> &SnapshotId<'a> { &self.snapshot_id }
    /// The first step to replay from (replay from the very start if not specified).
    pub fn from_step(&self) -> Option<i64> { self.from_step }
    /// The last step to replay to (replay till the end if not specified).
    pub fn to_step(&self) -> Option<i64> { self.to_step }
    /// The scale to apply while replaying (defaults to 1).
    pub fn scale(&self) -> Option<f64> { self.scale }
}


pub struct ReplaySnapshotParamsBuilder<'a> {
    snapshot_id: SnapshotId<'a>,
    from_step: Option<i64>,
    to_step: Option<i64>,
    scale: Option<f64>,
}

impl<'a> ReplaySnapshotParamsBuilder<'a> {
    /// The first step to replay from (replay from the very start if not specified).
    pub fn from_step(mut self, from_step: i64) -> Self { self.from_step = Some(from_step); self }
    /// The last step to replay to (replay till the end if not specified).
    pub fn to_step(mut self, to_step: i64) -> Self { self.to_step = Some(to_step); self }
    /// The scale to apply while replaying (defaults to 1).
    pub fn scale(mut self, scale: f64) -> Self { self.scale = Some(scale); self }
    pub fn build(self) -> ReplaySnapshotParams<'a> {
        ReplaySnapshotParams {
            snapshot_id: self.snapshot_id,
            from_step: self.from_step,
            to_step: self.to_step,
            scale: self.scale,
        }
    }
}

/// Replays the layer snapshot and returns the resulting bitmap.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplaySnapshotReturns<'a> {
    /// A data: URL for resulting image.
    #[serde(rename = "dataURL")]
    data_url: Cow<'a, str>,
}

impl<'a> ReplaySnapshotReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `data_url`: A data: URL for resulting image.
    pub fn builder(data_url: impl Into<Cow<'a, str>>) -> ReplaySnapshotReturnsBuilder<'a> {
        ReplaySnapshotReturnsBuilder {
            data_url: data_url.into(),
        }
    }
    /// A data: URL for resulting image.
    pub fn data_url(&self) -> &str { self.data_url.as_ref() }
}


pub struct ReplaySnapshotReturnsBuilder<'a> {
    data_url: Cow<'a, str>,
}

impl<'a> ReplaySnapshotReturnsBuilder<'a> {
    pub fn build(self) -> ReplaySnapshotReturns<'a> {
        ReplaySnapshotReturns {
            data_url: self.data_url,
        }
    }
}

impl<'a> ReplaySnapshotParams<'a> { pub const METHOD: &'static str = "LayerTree.replaySnapshot"; }

impl<'a> crate::CdpCommand<'a> for ReplaySnapshotParams<'a> {
    const METHOD: &'static str = "LayerTree.replaySnapshot";
    type Response = ReplaySnapshotReturns<'a>;
}

/// Replays the layer snapshot and returns canvas log.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotCommandLogParams<'a> {
    /// The id of the layer snapshot.
    #[serde(rename = "snapshotId")]
    snapshot_id: SnapshotId<'a>,
}

impl<'a> SnapshotCommandLogParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `snapshot_id`: The id of the layer snapshot.
    pub fn builder(snapshot_id: impl Into<SnapshotId<'a>>) -> SnapshotCommandLogParamsBuilder<'a> {
        SnapshotCommandLogParamsBuilder {
            snapshot_id: snapshot_id.into(),
        }
    }
    /// The id of the layer snapshot.
    pub fn snapshot_id(&self) -> &SnapshotId<'a> { &self.snapshot_id }
}


pub struct SnapshotCommandLogParamsBuilder<'a> {
    snapshot_id: SnapshotId<'a>,
}

impl<'a> SnapshotCommandLogParamsBuilder<'a> {
    pub fn build(self) -> SnapshotCommandLogParams<'a> {
        SnapshotCommandLogParams {
            snapshot_id: self.snapshot_id,
        }
    }
}

/// Replays the layer snapshot and returns canvas log.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotCommandLogReturns {
    /// The array of canvas function calls.
    #[serde(rename = "commandLog")]
    command_log: Vec<serde_json::Map<String, JsonValue>>,
}

impl SnapshotCommandLogReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `command_log`: The array of canvas function calls.
    pub fn builder(command_log: Vec<serde_json::Map<String, JsonValue>>) -> SnapshotCommandLogReturnsBuilder {
        SnapshotCommandLogReturnsBuilder {
            command_log: command_log,
        }
    }
    /// The array of canvas function calls.
    pub fn command_log(&self) -> &[serde_json::Map<String, JsonValue>] { &self.command_log }
}


pub struct SnapshotCommandLogReturnsBuilder {
    command_log: Vec<serde_json::Map<String, JsonValue>>,
}

impl SnapshotCommandLogReturnsBuilder {
    pub fn build(self) -> SnapshotCommandLogReturns {
        SnapshotCommandLogReturns {
            command_log: self.command_log,
        }
    }
}

impl<'a> SnapshotCommandLogParams<'a> { pub const METHOD: &'static str = "LayerTree.snapshotCommandLog"; }

impl<'a> crate::CdpCommand<'a> for SnapshotCommandLogParams<'a> {
    const METHOD: &'static str = "LayerTree.snapshotCommandLog";
    type Response = SnapshotCommandLogReturns;
}
