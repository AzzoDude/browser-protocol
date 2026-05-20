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
    pub fn builder() -> ScrollRectBuilder<'a> { ScrollRectBuilder::default() }
    pub fn rect(&self) -> &crate::dom::Rect { &self.rect }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}

#[derive(Default)]
pub struct ScrollRectBuilder<'a> {
    rect: Option<crate::dom::Rect>,
    type_: Option<Cow<'a, str>>,
}

impl<'a> ScrollRectBuilder<'a> {
    /// Rectangle itself.
    pub fn rect(mut self, rect: crate::dom::Rect) -> Self { self.rect = Some(rect); self }
    /// Reason for rectangle to force scrolling on the main thread
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    pub fn build(self) -> ScrollRect<'a> {
        ScrollRect {
            rect: self.rect.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
        }
    }
}

/// Sticky position constraints.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StickyPositionConstraint<'a> {
    /// Layout rectangle of the sticky element before being shifted
    stickyBoxRect: crate::dom::Rect,
    /// Layout rectangle of the containing block of the sticky element
    containingBlockRect: crate::dom::Rect,
    /// The nearest sticky layer that shifts the sticky box
    #[serde(skip_serializing_if = "Option::is_none")]
    nearestLayerShiftingStickyBox: Option<LayerId<'a>>,
    /// The nearest sticky layer that shifts the containing block
    #[serde(skip_serializing_if = "Option::is_none")]
    nearestLayerShiftingContainingBlock: Option<LayerId<'a>>,
}

impl<'a> StickyPositionConstraint<'a> {
    pub fn builder() -> StickyPositionConstraintBuilder<'a> { StickyPositionConstraintBuilder::default() }
    pub fn stickyBoxRect(&self) -> &crate::dom::Rect { &self.stickyBoxRect }
    pub fn containingBlockRect(&self) -> &crate::dom::Rect { &self.containingBlockRect }
    pub fn nearestLayerShiftingStickyBox(&self) -> Option<&LayerId<'a>> { self.nearestLayerShiftingStickyBox.as_ref() }
    pub fn nearestLayerShiftingContainingBlock(&self) -> Option<&LayerId<'a>> { self.nearestLayerShiftingContainingBlock.as_ref() }
}

#[derive(Default)]
pub struct StickyPositionConstraintBuilder<'a> {
    stickyBoxRect: Option<crate::dom::Rect>,
    containingBlockRect: Option<crate::dom::Rect>,
    nearestLayerShiftingStickyBox: Option<LayerId<'a>>,
    nearestLayerShiftingContainingBlock: Option<LayerId<'a>>,
}

impl<'a> StickyPositionConstraintBuilder<'a> {
    /// Layout rectangle of the sticky element before being shifted
    pub fn stickyBoxRect(mut self, stickyBoxRect: crate::dom::Rect) -> Self { self.stickyBoxRect = Some(stickyBoxRect); self }
    /// Layout rectangle of the containing block of the sticky element
    pub fn containingBlockRect(mut self, containingBlockRect: crate::dom::Rect) -> Self { self.containingBlockRect = Some(containingBlockRect); self }
    /// The nearest sticky layer that shifts the sticky box
    pub fn nearestLayerShiftingStickyBox(mut self, nearestLayerShiftingStickyBox: LayerId<'a>) -> Self { self.nearestLayerShiftingStickyBox = Some(nearestLayerShiftingStickyBox); self }
    /// The nearest sticky layer that shifts the containing block
    pub fn nearestLayerShiftingContainingBlock(mut self, nearestLayerShiftingContainingBlock: LayerId<'a>) -> Self { self.nearestLayerShiftingContainingBlock = Some(nearestLayerShiftingContainingBlock); self }
    pub fn build(self) -> StickyPositionConstraint<'a> {
        StickyPositionConstraint {
            stickyBoxRect: self.stickyBoxRect.unwrap_or_default(),
            containingBlockRect: self.containingBlockRect.unwrap_or_default(),
            nearestLayerShiftingStickyBox: self.nearestLayerShiftingStickyBox,
            nearestLayerShiftingContainingBlock: self.nearestLayerShiftingContainingBlock,
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
    pub fn builder() -> PictureTileBuilder<'a> { PictureTileBuilder::default() }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn picture(&self) -> &str { self.picture.as_ref() }
}

#[derive(Default)]
pub struct PictureTileBuilder<'a> {
    x: Option<f64>,
    y: Option<f64>,
    picture: Option<Cow<'a, str>>,
}

impl<'a> PictureTileBuilder<'a> {
    /// Offset from owning layer left boundary
    pub fn x(mut self, x: f64) -> Self { self.x = Some(x); self }
    /// Offset from owning layer top boundary
    pub fn y(mut self, y: f64) -> Self { self.y = Some(y); self }
    /// Base64-encoded snapshot data. (Encoded as a base64 string when passed over JSON)
    pub fn picture(mut self, picture: impl Into<Cow<'a, str>>) -> Self { self.picture = Some(picture.into()); self }
    pub fn build(self) -> PictureTile<'a> {
        PictureTile {
            x: self.x.unwrap_or_default(),
            y: self.y.unwrap_or_default(),
            picture: self.picture.unwrap_or_default(),
        }
    }
}

/// Information about a compositing layer.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Layer<'a> {
    /// The unique id for this layer.
    layerId: LayerId<'a>,
    /// The id of parent (not present for root).
    #[serde(skip_serializing_if = "Option::is_none")]
    parentLayerId: Option<LayerId<'a>>,
    /// The backend id for the node associated with this layer.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
    /// Offset from parent layer, X coordinate.
    offsetX: f64,
    /// Offset from parent layer, Y coordinate.
    offsetY: f64,
    /// Layer width.
    width: f64,
    /// Layer height.
    height: f64,
    /// Transformation matrix for layer, default is identity matrix
    #[serde(skip_serializing_if = "Option::is_none")]
    transform: Option<Vec<f64>>,
    /// Transform anchor point X, absent if no transform specified
    #[serde(skip_serializing_if = "Option::is_none")]
    anchorX: Option<f64>,
    /// Transform anchor point Y, absent if no transform specified
    #[serde(skip_serializing_if = "Option::is_none")]
    anchorY: Option<f64>,
    /// Transform anchor point Z, absent if no transform specified
    #[serde(skip_serializing_if = "Option::is_none")]
    anchorZ: Option<f64>,
    /// Indicates how many time this layer has painted.
    paintCount: u64,
    /// Indicates whether this layer hosts any content, rather than being used for
    /// transform/scrolling purposes only.
    drawsContent: bool,
    /// Set if layer is not visible.
    #[serde(skip_serializing_if = "Option::is_none")]
    invisible: Option<bool>,
    /// Rectangles scrolling on main thread only.
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollRects: Option<Vec<ScrollRect<'a>>>,
    /// Sticky position constraint information
    #[serde(skip_serializing_if = "Option::is_none")]
    stickyPositionConstraint: Option<StickyPositionConstraint<'a>>,
}

impl<'a> Layer<'a> {
    pub fn builder() -> LayerBuilder<'a> { LayerBuilder::default() }
    pub fn layerId(&self) -> &LayerId<'a> { &self.layerId }
    pub fn parentLayerId(&self) -> Option<&LayerId<'a>> { self.parentLayerId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn offsetX(&self) -> f64 { self.offsetX }
    pub fn offsetY(&self) -> f64 { self.offsetY }
    pub fn width(&self) -> f64 { self.width }
    pub fn height(&self) -> f64 { self.height }
    pub fn transform(&self) -> Option<&[f64]> { self.transform.as_deref() }
    pub fn anchorX(&self) -> Option<f64> { self.anchorX }
    pub fn anchorY(&self) -> Option<f64> { self.anchorY }
    pub fn anchorZ(&self) -> Option<f64> { self.anchorZ }
    pub fn paintCount(&self) -> u64 { self.paintCount }
    pub fn drawsContent(&self) -> bool { self.drawsContent }
    pub fn invisible(&self) -> Option<bool> { self.invisible }
    pub fn scrollRects(&self) -> Option<&[ScrollRect<'a>]> { self.scrollRects.as_deref() }
    pub fn stickyPositionConstraint(&self) -> Option<&StickyPositionConstraint<'a>> { self.stickyPositionConstraint.as_ref() }
}

#[derive(Default)]
pub struct LayerBuilder<'a> {
    layerId: Option<LayerId<'a>>,
    parentLayerId: Option<LayerId<'a>>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    offsetX: Option<f64>,
    offsetY: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    transform: Option<Vec<f64>>,
    anchorX: Option<f64>,
    anchorY: Option<f64>,
    anchorZ: Option<f64>,
    paintCount: Option<u64>,
    drawsContent: Option<bool>,
    invisible: Option<bool>,
    scrollRects: Option<Vec<ScrollRect<'a>>>,
    stickyPositionConstraint: Option<StickyPositionConstraint<'a>>,
}

impl<'a> LayerBuilder<'a> {
    /// The unique id for this layer.
    pub fn layerId(mut self, layerId: LayerId<'a>) -> Self { self.layerId = Some(layerId); self }
    /// The id of parent (not present for root).
    pub fn parentLayerId(mut self, parentLayerId: LayerId<'a>) -> Self { self.parentLayerId = Some(parentLayerId); self }
    /// The backend id for the node associated with this layer.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// Offset from parent layer, X coordinate.
    pub fn offsetX(mut self, offsetX: f64) -> Self { self.offsetX = Some(offsetX); self }
    /// Offset from parent layer, Y coordinate.
    pub fn offsetY(mut self, offsetY: f64) -> Self { self.offsetY = Some(offsetY); self }
    /// Layer width.
    pub fn width(mut self, width: f64) -> Self { self.width = Some(width); self }
    /// Layer height.
    pub fn height(mut self, height: f64) -> Self { self.height = Some(height); self }
    /// Transformation matrix for layer, default is identity matrix
    pub fn transform(mut self, transform: Vec<f64>) -> Self { self.transform = Some(transform); self }
    /// Transform anchor point X, absent if no transform specified
    pub fn anchorX(mut self, anchorX: f64) -> Self { self.anchorX = Some(anchorX); self }
    /// Transform anchor point Y, absent if no transform specified
    pub fn anchorY(mut self, anchorY: f64) -> Self { self.anchorY = Some(anchorY); self }
    /// Transform anchor point Z, absent if no transform specified
    pub fn anchorZ(mut self, anchorZ: f64) -> Self { self.anchorZ = Some(anchorZ); self }
    /// Indicates how many time this layer has painted.
    pub fn paintCount(mut self, paintCount: u64) -> Self { self.paintCount = Some(paintCount); self }
    /// Indicates whether this layer hosts any content, rather than being used for
    /// transform/scrolling purposes only.
    pub fn drawsContent(mut self, drawsContent: bool) -> Self { self.drawsContent = Some(drawsContent); self }
    /// Set if layer is not visible.
    pub fn invisible(mut self, invisible: bool) -> Self { self.invisible = Some(invisible); self }
    /// Rectangles scrolling on main thread only.
    pub fn scrollRects(mut self, scrollRects: Vec<ScrollRect<'a>>) -> Self { self.scrollRects = Some(scrollRects); self }
    /// Sticky position constraint information
    pub fn stickyPositionConstraint(mut self, stickyPositionConstraint: StickyPositionConstraint<'a>) -> Self { self.stickyPositionConstraint = Some(stickyPositionConstraint); self }
    pub fn build(self) -> Layer<'a> {
        Layer {
            layerId: self.layerId.unwrap_or_default(),
            parentLayerId: self.parentLayerId,
            backendNodeId: self.backendNodeId,
            offsetX: self.offsetX.unwrap_or_default(),
            offsetY: self.offsetY.unwrap_or_default(),
            width: self.width.unwrap_or_default(),
            height: self.height.unwrap_or_default(),
            transform: self.transform,
            anchorX: self.anchorX,
            anchorY: self.anchorY,
            anchorZ: self.anchorZ,
            paintCount: self.paintCount.unwrap_or_default(),
            drawsContent: self.drawsContent.unwrap_or_default(),
            invisible: self.invisible,
            scrollRects: self.scrollRects,
            stickyPositionConstraint: self.stickyPositionConstraint,
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
    layerId: LayerId<'a>,
}

impl<'a> CompositingReasonsParams<'a> {
    pub fn builder() -> CompositingReasonsParamsBuilder<'a> { CompositingReasonsParamsBuilder::default() }
    pub fn layerId(&self) -> &LayerId<'a> { &self.layerId }
}

#[derive(Default)]
pub struct CompositingReasonsParamsBuilder<'a> {
    layerId: Option<LayerId<'a>>,
}

impl<'a> CompositingReasonsParamsBuilder<'a> {
    /// The id of the layer for which we want to get the reasons it was composited.
    pub fn layerId(mut self, layerId: LayerId<'a>) -> Self { self.layerId = Some(layerId); self }
    pub fn build(self) -> CompositingReasonsParams<'a> {
        CompositingReasonsParams {
            layerId: self.layerId.unwrap_or_default(),
        }
    }
}

/// Provides the reasons why the given layer was composited.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompositingReasonsReturns<'a> {
    /// A list of strings specifying reasons for the given layer to become composited.
    compositingReasons: Vec<Cow<'a, str>>,
    /// A list of strings specifying reason IDs for the given layer to become composited.
    compositingReasonIds: Vec<Cow<'a, str>>,
}

impl<'a> CompositingReasonsReturns<'a> {
    pub fn builder() -> CompositingReasonsReturnsBuilder<'a> { CompositingReasonsReturnsBuilder::default() }
    pub fn compositingReasons(&self) -> &[Cow<'a, str>] { &self.compositingReasons }
    pub fn compositingReasonIds(&self) -> &[Cow<'a, str>] { &self.compositingReasonIds }
}

#[derive(Default)]
pub struct CompositingReasonsReturnsBuilder<'a> {
    compositingReasons: Option<Vec<Cow<'a, str>>>,
    compositingReasonIds: Option<Vec<Cow<'a, str>>>,
}

impl<'a> CompositingReasonsReturnsBuilder<'a> {
    /// A list of strings specifying reasons for the given layer to become composited.
    pub fn compositingReasons(mut self, compositingReasons: Vec<Cow<'a, str>>) -> Self { self.compositingReasons = Some(compositingReasons); self }
    /// A list of strings specifying reason IDs for the given layer to become composited.
    pub fn compositingReasonIds(mut self, compositingReasonIds: Vec<Cow<'a, str>>) -> Self { self.compositingReasonIds = Some(compositingReasonIds); self }
    pub fn build(self) -> CompositingReasonsReturns<'a> {
        CompositingReasonsReturns {
            compositingReasons: self.compositingReasons.unwrap_or_default(),
            compositingReasonIds: self.compositingReasonIds.unwrap_or_default(),
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

impl DisableParams { pub const METHOD: &'static str = "LayerTree.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "LayerTree.disable";
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
    pub fn builder() -> LoadSnapshotParamsBuilder<'a> { LoadSnapshotParamsBuilder::default() }
    pub fn tiles(&self) -> &[PictureTile<'a>] { &self.tiles }
}

#[derive(Default)]
pub struct LoadSnapshotParamsBuilder<'a> {
    tiles: Option<Vec<PictureTile<'a>>>,
}

impl<'a> LoadSnapshotParamsBuilder<'a> {
    /// An array of tiles composing the snapshot.
    pub fn tiles(mut self, tiles: Vec<PictureTile<'a>>) -> Self { self.tiles = Some(tiles); self }
    pub fn build(self) -> LoadSnapshotParams<'a> {
        LoadSnapshotParams {
            tiles: self.tiles.unwrap_or_default(),
        }
    }
}

/// Returns the snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadSnapshotReturns<'a> {
    /// The id of the snapshot.
    snapshotId: SnapshotId<'a>,
}

impl<'a> LoadSnapshotReturns<'a> {
    pub fn builder() -> LoadSnapshotReturnsBuilder<'a> { LoadSnapshotReturnsBuilder::default() }
    pub fn snapshotId(&self) -> &SnapshotId<'a> { &self.snapshotId }
}

#[derive(Default)]
pub struct LoadSnapshotReturnsBuilder<'a> {
    snapshotId: Option<SnapshotId<'a>>,
}

impl<'a> LoadSnapshotReturnsBuilder<'a> {
    /// The id of the snapshot.
    pub fn snapshotId(mut self, snapshotId: SnapshotId<'a>) -> Self { self.snapshotId = Some(snapshotId); self }
    pub fn build(self) -> LoadSnapshotReturns<'a> {
        LoadSnapshotReturns {
            snapshotId: self.snapshotId.unwrap_or_default(),
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
    layerId: LayerId<'a>,
}

impl<'a> MakeSnapshotParams<'a> {
    pub fn builder() -> MakeSnapshotParamsBuilder<'a> { MakeSnapshotParamsBuilder::default() }
    pub fn layerId(&self) -> &LayerId<'a> { &self.layerId }
}

#[derive(Default)]
pub struct MakeSnapshotParamsBuilder<'a> {
    layerId: Option<LayerId<'a>>,
}

impl<'a> MakeSnapshotParamsBuilder<'a> {
    /// The id of the layer.
    pub fn layerId(mut self, layerId: LayerId<'a>) -> Self { self.layerId = Some(layerId); self }
    pub fn build(self) -> MakeSnapshotParams<'a> {
        MakeSnapshotParams {
            layerId: self.layerId.unwrap_or_default(),
        }
    }
}

/// Returns the layer snapshot identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MakeSnapshotReturns<'a> {
    /// The id of the layer snapshot.
    snapshotId: SnapshotId<'a>,
}

impl<'a> MakeSnapshotReturns<'a> {
    pub fn builder() -> MakeSnapshotReturnsBuilder<'a> { MakeSnapshotReturnsBuilder::default() }
    pub fn snapshotId(&self) -> &SnapshotId<'a> { &self.snapshotId }
}

#[derive(Default)]
pub struct MakeSnapshotReturnsBuilder<'a> {
    snapshotId: Option<SnapshotId<'a>>,
}

impl<'a> MakeSnapshotReturnsBuilder<'a> {
    /// The id of the layer snapshot.
    pub fn snapshotId(mut self, snapshotId: SnapshotId<'a>) -> Self { self.snapshotId = Some(snapshotId); self }
    pub fn build(self) -> MakeSnapshotReturns<'a> {
        MakeSnapshotReturns {
            snapshotId: self.snapshotId.unwrap_or_default(),
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
    snapshotId: SnapshotId<'a>,
    /// The maximum number of times to replay the snapshot (1, if not specified).
    #[serde(skip_serializing_if = "Option::is_none")]
    minRepeatCount: Option<u64>,
    /// The minimum duration (in seconds) to replay the snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    minDuration: Option<f64>,
    /// The clip rectangle to apply when replaying the snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    clipRect: Option<crate::dom::Rect>,
}

impl<'a> ProfileSnapshotParams<'a> {
    pub fn builder() -> ProfileSnapshotParamsBuilder<'a> { ProfileSnapshotParamsBuilder::default() }
    pub fn snapshotId(&self) -> &SnapshotId<'a> { &self.snapshotId }
    pub fn minRepeatCount(&self) -> Option<u64> { self.minRepeatCount }
    pub fn minDuration(&self) -> Option<f64> { self.minDuration }
    pub fn clipRect(&self) -> Option<&crate::dom::Rect> { self.clipRect.as_ref() }
}

#[derive(Default)]
pub struct ProfileSnapshotParamsBuilder<'a> {
    snapshotId: Option<SnapshotId<'a>>,
    minRepeatCount: Option<u64>,
    minDuration: Option<f64>,
    clipRect: Option<crate::dom::Rect>,
}

impl<'a> ProfileSnapshotParamsBuilder<'a> {
    /// The id of the layer snapshot.
    pub fn snapshotId(mut self, snapshotId: SnapshotId<'a>) -> Self { self.snapshotId = Some(snapshotId); self }
    /// The maximum number of times to replay the snapshot (1, if not specified).
    pub fn minRepeatCount(mut self, minRepeatCount: u64) -> Self { self.minRepeatCount = Some(minRepeatCount); self }
    /// The minimum duration (in seconds) to replay the snapshot.
    pub fn minDuration(mut self, minDuration: f64) -> Self { self.minDuration = Some(minDuration); self }
    /// The clip rectangle to apply when replaying the snapshot.
    pub fn clipRect(mut self, clipRect: crate::dom::Rect) -> Self { self.clipRect = Some(clipRect); self }
    pub fn build(self) -> ProfileSnapshotParams<'a> {
        ProfileSnapshotParams {
            snapshotId: self.snapshotId.unwrap_or_default(),
            minRepeatCount: self.minRepeatCount,
            minDuration: self.minDuration,
            clipRect: self.clipRect,
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
    pub fn builder() -> ProfileSnapshotReturnsBuilder { ProfileSnapshotReturnsBuilder::default() }
    pub fn timings(&self) -> &[PaintProfile] { &self.timings }
}

#[derive(Default)]
pub struct ProfileSnapshotReturnsBuilder {
    timings: Option<Vec<PaintProfile>>,
}

impl ProfileSnapshotReturnsBuilder {
    /// The array of paint profiles, one per run.
    pub fn timings(mut self, timings: Vec<PaintProfile>) -> Self { self.timings = Some(timings); self }
    pub fn build(self) -> ProfileSnapshotReturns {
        ProfileSnapshotReturns {
            timings: self.timings.unwrap_or_default(),
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
    snapshotId: SnapshotId<'a>,
}

impl<'a> ReleaseSnapshotParams<'a> {
    pub fn builder() -> ReleaseSnapshotParamsBuilder<'a> { ReleaseSnapshotParamsBuilder::default() }
    pub fn snapshotId(&self) -> &SnapshotId<'a> { &self.snapshotId }
}

#[derive(Default)]
pub struct ReleaseSnapshotParamsBuilder<'a> {
    snapshotId: Option<SnapshotId<'a>>,
}

impl<'a> ReleaseSnapshotParamsBuilder<'a> {
    /// The id of the layer snapshot.
    pub fn snapshotId(mut self, snapshotId: SnapshotId<'a>) -> Self { self.snapshotId = Some(snapshotId); self }
    pub fn build(self) -> ReleaseSnapshotParams<'a> {
        ReleaseSnapshotParams {
            snapshotId: self.snapshotId.unwrap_or_default(),
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
    snapshotId: SnapshotId<'a>,
    /// The first step to replay from (replay from the very start if not specified).
    #[serde(skip_serializing_if = "Option::is_none")]
    fromStep: Option<i64>,
    /// The last step to replay to (replay till the end if not specified).
    #[serde(skip_serializing_if = "Option::is_none")]
    toStep: Option<i64>,
    /// The scale to apply while replaying (defaults to 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f64>,
}

impl<'a> ReplaySnapshotParams<'a> {
    pub fn builder() -> ReplaySnapshotParamsBuilder<'a> { ReplaySnapshotParamsBuilder::default() }
    pub fn snapshotId(&self) -> &SnapshotId<'a> { &self.snapshotId }
    pub fn fromStep(&self) -> Option<i64> { self.fromStep }
    pub fn toStep(&self) -> Option<i64> { self.toStep }
    pub fn scale(&self) -> Option<f64> { self.scale }
}

#[derive(Default)]
pub struct ReplaySnapshotParamsBuilder<'a> {
    snapshotId: Option<SnapshotId<'a>>,
    fromStep: Option<i64>,
    toStep: Option<i64>,
    scale: Option<f64>,
}

impl<'a> ReplaySnapshotParamsBuilder<'a> {
    /// The id of the layer snapshot.
    pub fn snapshotId(mut self, snapshotId: SnapshotId<'a>) -> Self { self.snapshotId = Some(snapshotId); self }
    /// The first step to replay from (replay from the very start if not specified).
    pub fn fromStep(mut self, fromStep: i64) -> Self { self.fromStep = Some(fromStep); self }
    /// The last step to replay to (replay till the end if not specified).
    pub fn toStep(mut self, toStep: i64) -> Self { self.toStep = Some(toStep); self }
    /// The scale to apply while replaying (defaults to 1).
    pub fn scale(mut self, scale: f64) -> Self { self.scale = Some(scale); self }
    pub fn build(self) -> ReplaySnapshotParams<'a> {
        ReplaySnapshotParams {
            snapshotId: self.snapshotId.unwrap_or_default(),
            fromStep: self.fromStep,
            toStep: self.toStep,
            scale: self.scale,
        }
    }
}

/// Replays the layer snapshot and returns the resulting bitmap.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplaySnapshotReturns<'a> {
    /// A data: URL for resulting image.
    dataURL: Cow<'a, str>,
}

impl<'a> ReplaySnapshotReturns<'a> {
    pub fn builder() -> ReplaySnapshotReturnsBuilder<'a> { ReplaySnapshotReturnsBuilder::default() }
    pub fn dataURL(&self) -> &str { self.dataURL.as_ref() }
}

#[derive(Default)]
pub struct ReplaySnapshotReturnsBuilder<'a> {
    dataURL: Option<Cow<'a, str>>,
}

impl<'a> ReplaySnapshotReturnsBuilder<'a> {
    /// A data: URL for resulting image.
    pub fn dataURL(mut self, dataURL: impl Into<Cow<'a, str>>) -> Self { self.dataURL = Some(dataURL.into()); self }
    pub fn build(self) -> ReplaySnapshotReturns<'a> {
        ReplaySnapshotReturns {
            dataURL: self.dataURL.unwrap_or_default(),
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
    snapshotId: SnapshotId<'a>,
}

impl<'a> SnapshotCommandLogParams<'a> {
    pub fn builder() -> SnapshotCommandLogParamsBuilder<'a> { SnapshotCommandLogParamsBuilder::default() }
    pub fn snapshotId(&self) -> &SnapshotId<'a> { &self.snapshotId }
}

#[derive(Default)]
pub struct SnapshotCommandLogParamsBuilder<'a> {
    snapshotId: Option<SnapshotId<'a>>,
}

impl<'a> SnapshotCommandLogParamsBuilder<'a> {
    /// The id of the layer snapshot.
    pub fn snapshotId(mut self, snapshotId: SnapshotId<'a>) -> Self { self.snapshotId = Some(snapshotId); self }
    pub fn build(self) -> SnapshotCommandLogParams<'a> {
        SnapshotCommandLogParams {
            snapshotId: self.snapshotId.unwrap_or_default(),
        }
    }
}

/// Replays the layer snapshot and returns canvas log.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotCommandLogReturns {
    /// The array of canvas function calls.
    commandLog: Vec<serde_json::Map<String, JsonValue>>,
}

impl SnapshotCommandLogReturns {
    pub fn builder() -> SnapshotCommandLogReturnsBuilder { SnapshotCommandLogReturnsBuilder::default() }
    pub fn commandLog(&self) -> &[serde_json::Map<String, JsonValue>] { &self.commandLog }
}

#[derive(Default)]
pub struct SnapshotCommandLogReturnsBuilder {
    commandLog: Option<Vec<serde_json::Map<String, JsonValue>>>,
}

impl SnapshotCommandLogReturnsBuilder {
    /// The array of canvas function calls.
    pub fn commandLog(mut self, commandLog: Vec<serde_json::Map<String, JsonValue>>) -> Self { self.commandLog = Some(commandLog); self }
    pub fn build(self) -> SnapshotCommandLogReturns {
        SnapshotCommandLogReturns {
            commandLog: self.commandLog.unwrap_or_default(),
        }
    }
}

impl<'a> SnapshotCommandLogParams<'a> { pub const METHOD: &'static str = "LayerTree.snapshotCommandLog"; }

impl<'a> crate::CdpCommand<'a> for SnapshotCommandLogParams<'a> {
    const METHOD: &'static str = "LayerTree.snapshotCommandLog";
    type Response = SnapshotCommandLogReturns;
}
