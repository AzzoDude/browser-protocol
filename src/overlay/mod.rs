use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! This domain provides various functionality related to drawing atop the inspected page.

/// Configuration data for drawing the source order of an elements children.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceOrderConfig {
    /// the color to outline the given element in.

    pub parentOutlineColor: crate::dom::RGBA,
    /// the color to outline the child elements in.

    pub childOutlineColor: crate::dom::RGBA,
}

/// Configuration data for the highlighting of Grid elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GridHighlightConfig {
    /// Whether the extension lines from grid cells to the rulers should be shown (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showGridExtensionLines: Option<bool>,
    /// Show Positive line number labels (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showPositiveLineNumbers: Option<bool>,
    /// Show Negative line number labels (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showNegativeLineNumbers: Option<bool>,
    /// Show area name labels (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showAreaNames: Option<bool>,
    /// Show line name labels (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showLineNames: Option<bool>,
    /// Show track size labels (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showTrackSizes: Option<bool>,
    /// The grid container border highlight color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gridBorderColor: Option<crate::dom::RGBA>,
    /// The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellBorderColor: Option<crate::dom::RGBA>,
    /// The row line color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowLineColor: Option<crate::dom::RGBA>,
    /// The column line color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columnLineColor: Option<crate::dom::RGBA>,
    /// Whether the grid border is dashed (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gridBorderDash: Option<bool>,
    /// Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellBorderDash: Option<bool>,
    /// Whether row lines are dashed (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowLineDash: Option<bool>,
    /// Whether column lines are dashed (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columnLineDash: Option<bool>,
    /// The row gap highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowGapColor: Option<crate::dom::RGBA>,
    /// The row gap hatching fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowHatchColor: Option<crate::dom::RGBA>,
    /// The column gap highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columnGapColor: Option<crate::dom::RGBA>,
    /// The column gap hatching fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columnHatchColor: Option<crate::dom::RGBA>,
    /// The named grid areas border color (Default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub areaBorderColor: Option<crate::dom::RGBA>,
    /// The grid container background color (Default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gridBackgroundColor: Option<crate::dom::RGBA>,
}

/// Configuration data for the highlighting of Flex container elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexContainerHighlightConfig {
    /// The style of the container border

    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerBorder: Option<LineStyle>,
    /// The style of the separator between lines

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineSeparator: Option<LineStyle>,
    /// The style of the separator between items

    #[serde(skip_serializing_if = "Option::is_none")]
    pub itemSeparator: Option<LineStyle>,
    /// Style of content-distribution space on the main axis (justify-content).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainDistributedSpace: Option<BoxStyle>,
    /// Style of content-distribution space on the cross axis (align-content).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossDistributedSpace: Option<BoxStyle>,
    /// Style of empty space caused by row gaps (gap/row-gap).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowGapSpace: Option<BoxStyle>,
    /// Style of empty space caused by columns gaps (gap/column-gap).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columnGapSpace: Option<BoxStyle>,
    /// Style of the self-alignment line (align-items).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossAlignment: Option<LineStyle>,
}

/// Configuration data for the highlighting of Flex item elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexItemHighlightConfig {
    /// Style of the box representing the item's base size

    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseSizeBox: Option<BoxStyle>,
    /// Style of the border around the box representing the item's base size

    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseSizeBorder: Option<LineStyle>,
    /// Style of the arrow representing if the item grew or shrank

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexibilityArrow: Option<LineStyle>,
}

/// Style information for drawing a line.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle {
    /// The color of the line (default: transparent)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::dom::RGBA>,
    /// The line pattern (default: solid)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// Style information for drawing a box.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoxStyle {
    /// The background color for the box (default: transparent)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fillColor: Option<crate::dom::RGBA>,
    /// The hatching color for the box (default: transparent)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatchColor: Option<crate::dom::RGBA>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContrastAlgorithm {
    #[default]
    Aa,
    Aaa,
    Apca,
}

/// Configuration data for the highlighting of page elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightConfig {
    /// Whether the node info tooltip should be shown (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showInfo: Option<bool>,
    /// Whether the node styles in the tooltip (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showStyles: Option<bool>,
    /// Whether the rulers should be shown (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showRulers: Option<bool>,
    /// Whether the a11y info should be shown (default: true).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showAccessibilityInfo: Option<bool>,
    /// Whether the extension lines from node to the rulers should be shown (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showExtensionLines: Option<bool>,
    /// The content box highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentColor: Option<crate::dom::RGBA>,
    /// The padding highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddingColor: Option<crate::dom::RGBA>,
    /// The border highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderColor: Option<crate::dom::RGBA>,
    /// The margin highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub marginColor: Option<crate::dom::RGBA>,
    /// The event target element highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventTargetColor: Option<crate::dom::RGBA>,
    /// The shape outside fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shapeColor: Option<crate::dom::RGBA>,
    /// The shape margin fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shapeMarginColor: Option<crate::dom::RGBA>,
    /// The grid layout color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssGridColor: Option<crate::dom::RGBA>,
    /// The color format used to format color styles (default: hex).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorFormat: Option<ColorFormat>,
    /// The grid layout highlight configuration (default: all transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gridHighlightConfig: Option<GridHighlightConfig>,
    /// The flex container highlight configuration (default: all transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexContainerHighlightConfig: Option<FlexContainerHighlightConfig>,
    /// The flex item highlight configuration (default: all transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexItemHighlightConfig: Option<FlexItemHighlightConfig>,
    /// The contrast algorithm to use for the contrast ratio (default: aa).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrastAlgorithm: Option<ContrastAlgorithm>,
    /// The container query container highlight configuration (default: all transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerQueryContainerHighlightConfig: Option<ContainerQueryContainerHighlightConfig>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ColorFormat {
    #[default]
    Rgb,
    Hsl,
    Hwb,
    Hex,
}

/// Configurations for Persistent Grid Highlight

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GridNodeHighlightConfig {
    /// A descriptor for the highlight appearance.

    pub gridHighlightConfig: GridHighlightConfig,
    /// Identifier of the node to highlight.

    pub nodeId: crate::dom::NodeId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexNodeHighlightConfig {
    /// A descriptor for the highlight appearance of flex containers.

    pub flexContainerHighlightConfig: FlexContainerHighlightConfig,
    /// Identifier of the node to highlight.

    pub nodeId: crate::dom::NodeId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapContainerHighlightConfig {
    /// The style of the snapport border (default: transparent)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapportBorder: Option<LineStyle>,
    /// The style of the snap area border (default: transparent)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapAreaBorder: Option<LineStyle>,
    /// The margin highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollMarginColor: Option<crate::dom::RGBA>,
    /// The padding highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollPaddingColor: Option<crate::dom::RGBA>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapHighlightConfig {
    /// A descriptor for the highlight appearance of scroll snap containers.

    pub scrollSnapContainerHighlightConfig: ScrollSnapContainerHighlightConfig,
    /// Identifier of the node to highlight.

    pub nodeId: crate::dom::NodeId,
}

/// Configuration for dual screen hinge

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HingeConfig {
    /// A rectangle represent hinge

    pub rect: crate::dom::Rect,
    /// The content box highlight fill color (default: a dark color).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentColor: Option<crate::dom::RGBA>,
    /// The content box highlight outline color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlineColor: Option<crate::dom::RGBA>,
}

/// Configuration for Window Controls Overlay

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowControlsOverlayConfig {
    /// Whether the title bar CSS should be shown when emulating the Window Controls Overlay.

    pub showCSS: bool,
    /// Selected platforms to show the overlay.

    pub selectedPlatform: String,
    /// The theme color defined in app manifest.

    pub themeColor: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryHighlightConfig {
    /// A descriptor for the highlight appearance of container query containers.

    pub containerQueryContainerHighlightConfig: ContainerQueryContainerHighlightConfig,
    /// Identifier of the container node to highlight.

    pub nodeId: crate::dom::NodeId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryContainerHighlightConfig {
    /// The style of the container border.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerBorder: Option<LineStyle>,
    /// The style of the descendants' borders.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub descendantBorder: Option<LineStyle>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedElementHighlightConfig {
    /// A descriptor for the highlight appearance of an element in isolation mode.

    pub isolationModeHighlightConfig: IsolationModeHighlightConfig,
    /// Identifier of the isolated element to highlight.

    pub nodeId: crate::dom::NodeId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IsolationModeHighlightConfig {
    /// The fill color of the resizers (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resizerColor: Option<crate::dom::RGBA>,
    /// The fill color for resizer handles (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resizerHandleColor: Option<crate::dom::RGBA>,
    /// The fill color for the mask covering non-isolated elements (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maskColor: Option<crate::dom::RGBA>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InspectMode {
    #[default]
    SearchForNode,
    SearchForUAShadowDOM,
    CaptureAreaScreenshot,
    None,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectedElementAnchorConfig {
    /// Identifier of the node to highlight.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Overlay.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "Overlay.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Overlay.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "Overlay.enable";
    type Response = crate::EmptyReturns;
}

/// For testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHighlightObjectForTestParams {
    /// Id of the node to get highlight object for.

    pub nodeId: crate::dom::NodeId,
    /// Whether to include distance info.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeDistance: Option<bool>,
    /// Whether to include style info.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeStyle: Option<bool>,
    /// The color format to get config with (default: hex).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorFormat: Option<ColorFormat>,
    /// Whether to show accessibility info (default: true).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub showAccessibilityInfo: Option<bool>,
}

/// For testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHighlightObjectForTestReturns {
    /// Highlight data for the node.

    pub highlight: serde_json::Map<String, JsonValue>,
}

impl GetHighlightObjectForTestParams { pub const METHOD: &'static str = "Overlay.getHighlightObjectForTest"; }

impl crate::CdpCommand for GetHighlightObjectForTestParams {
    const METHOD: &'static str = "Overlay.getHighlightObjectForTest";
    type Response = GetHighlightObjectForTestReturns;
}

/// For Persistent Grid testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGridHighlightObjectsForTestParams {
    /// Ids of the node to get highlight object for.

    pub nodeIds: Vec<crate::dom::NodeId>,
}

/// For Persistent Grid testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGridHighlightObjectsForTestReturns {
    /// Grid Highlight data for the node ids provided.

    pub highlights: serde_json::Map<String, JsonValue>,
}

impl GetGridHighlightObjectsForTestParams { pub const METHOD: &'static str = "Overlay.getGridHighlightObjectsForTest"; }

impl crate::CdpCommand for GetGridHighlightObjectsForTestParams {
    const METHOD: &'static str = "Overlay.getGridHighlightObjectsForTest";
    type Response = GetGridHighlightObjectsForTestReturns;
}

/// For Source Order Viewer testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceOrderHighlightObjectForTestParams {
    /// Id of the node to highlight.

    pub nodeId: crate::dom::NodeId,
}

/// For Source Order Viewer testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceOrderHighlightObjectForTestReturns {
    /// Source order highlight data for the node id provided.

    pub highlight: serde_json::Map<String, JsonValue>,
}

impl GetSourceOrderHighlightObjectForTestParams { pub const METHOD: &'static str = "Overlay.getSourceOrderHighlightObjectForTest"; }

impl crate::CdpCommand for GetSourceOrderHighlightObjectForTestParams {
    const METHOD: &'static str = "Overlay.getSourceOrderHighlightObjectForTest";
    type Response = GetSourceOrderHighlightObjectForTestReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HideHighlightParams {}

impl HideHighlightParams { pub const METHOD: &'static str = "Overlay.hideHighlight"; }

impl crate::CdpCommand for HideHighlightParams {
    const METHOD: &'static str = "Overlay.hideHighlight";
    type Response = crate::EmptyReturns;
}

/// Highlights owner element of the frame with given id.
/// Deprecated: Doesn't work reliably and cannot be fixed due to process
/// separation (the owner node might be in a different process). Determine
/// the owner node in the client and use highlightNode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightFrameParams {
    /// Identifier of the frame to highlight.

    pub frameId: crate::page::FrameId,
    /// The content box highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentColor: Option<crate::dom::RGBA>,
    /// The content box highlight outline color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentOutlineColor: Option<crate::dom::RGBA>,
}

impl HighlightFrameParams { pub const METHOD: &'static str = "Overlay.highlightFrame"; }

impl crate::CdpCommand for HighlightFrameParams {
    const METHOD: &'static str = "Overlay.highlightFrame";
    type Response = crate::EmptyReturns;
}

/// Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or
/// objectId must be specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightNodeParams {
    /// A descriptor for the highlight appearance.

    pub highlightConfig: HighlightConfig,
    /// Identifier of the node to highlight.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node to be highlighted.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
    /// Selectors to highlight relevant nodes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

impl HighlightNodeParams { pub const METHOD: &'static str = "Overlay.highlightNode"; }

impl crate::CdpCommand for HighlightNodeParams {
    const METHOD: &'static str = "Overlay.highlightNode";
    type Response = crate::EmptyReturns;
}

/// Highlights given quad. Coordinates are absolute with respect to the main frame viewport.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightQuadParams {
    /// Quad to highlight

    pub quad: crate::dom::Quad,
    /// The highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::dom::RGBA>,
    /// The highlight outline color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlineColor: Option<crate::dom::RGBA>,
}

impl HighlightQuadParams { pub const METHOD: &'static str = "Overlay.highlightQuad"; }

impl crate::CdpCommand for HighlightQuadParams {
    const METHOD: &'static str = "Overlay.highlightQuad";
    type Response = crate::EmptyReturns;
}

/// Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.
/// Issue: the method does not handle device pixel ratio (DPR) correctly.
/// The coordinates currently have to be adjusted by the client
/// if DPR is not 1 (see crbug.com/437807128).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightRectParams {
    /// X coordinate

    pub x: i32,
    /// Y coordinate

    pub y: i32,
    /// Rectangle width

    pub width: u64,
    /// Rectangle height

    pub height: i64,
    /// The highlight fill color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::dom::RGBA>,
    /// The highlight outline color (default: transparent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlineColor: Option<crate::dom::RGBA>,
}

impl HighlightRectParams { pub const METHOD: &'static str = "Overlay.highlightRect"; }

impl crate::CdpCommand for HighlightRectParams {
    const METHOD: &'static str = "Overlay.highlightRect";
    type Response = crate::EmptyReturns;
}

/// Highlights the source order of the children of the DOM node with given id or with the given
/// JavaScript object wrapper. Either nodeId or objectId must be specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightSourceOrderParams {
    /// A descriptor for the appearance of the overlay drawing.

    pub sourceOrderConfig: SourceOrderConfig,
    /// Identifier of the node to highlight.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node to be highlighted.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
}

impl HighlightSourceOrderParams { pub const METHOD: &'static str = "Overlay.highlightSourceOrder"; }

impl crate::CdpCommand for HighlightSourceOrderParams {
    const METHOD: &'static str = "Overlay.highlightSourceOrder";
    type Response = crate::EmptyReturns;
}

/// Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.
/// Backend then generates 'inspectNodeRequested' event upon element selection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInspectModeParams {
    /// Set an inspection mode.

    pub mode: InspectMode,
    /// A descriptor for the highlight appearance of hovered-over nodes. May be omitted if 'enabled
    /// == false'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlightConfig: Option<HighlightConfig>,
}

impl SetInspectModeParams { pub const METHOD: &'static str = "Overlay.setInspectMode"; }

impl crate::CdpCommand for SetInspectModeParams {
    const METHOD: &'static str = "Overlay.setInspectMode";
    type Response = crate::EmptyReturns;
}

/// Highlights owner element of all frames detected to be ads.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowAdHighlightsParams {
    /// True for showing ad highlights

    pub show: bool,
}

impl SetShowAdHighlightsParams { pub const METHOD: &'static str = "Overlay.setShowAdHighlights"; }

impl crate::CdpCommand for SetShowAdHighlightsParams {
    const METHOD: &'static str = "Overlay.setShowAdHighlights";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPausedInDebuggerMessageParams {
    /// The message to display, also triggers resume and step over controls.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl SetPausedInDebuggerMessageParams { pub const METHOD: &'static str = "Overlay.setPausedInDebuggerMessage"; }

impl crate::CdpCommand for SetPausedInDebuggerMessageParams {
    const METHOD: &'static str = "Overlay.setPausedInDebuggerMessage";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows debug borders on layers

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowDebugBordersParams {
    /// True for showing debug borders

    pub show: bool,
}

impl SetShowDebugBordersParams { pub const METHOD: &'static str = "Overlay.setShowDebugBorders"; }

impl crate::CdpCommand for SetShowDebugBordersParams {
    const METHOD: &'static str = "Overlay.setShowDebugBorders";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows the FPS counter

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowFPSCounterParams {
    /// True for showing the FPS counter

    pub show: bool,
}

impl SetShowFPSCounterParams { pub const METHOD: &'static str = "Overlay.setShowFPSCounter"; }

impl crate::CdpCommand for SetShowFPSCounterParams {
    const METHOD: &'static str = "Overlay.setShowFPSCounter";
    type Response = crate::EmptyReturns;
}

/// Highlight multiple elements with the CSS Grid overlay.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowGridOverlaysParams {
    /// An array of node identifiers and descriptors for the highlight appearance.

    pub gridNodeHighlightConfigs: Vec<GridNodeHighlightConfig>,
}

impl SetShowGridOverlaysParams { pub const METHOD: &'static str = "Overlay.setShowGridOverlays"; }

impl crate::CdpCommand for SetShowGridOverlaysParams {
    const METHOD: &'static str = "Overlay.setShowGridOverlays";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowFlexOverlaysParams {
    /// An array of node identifiers and descriptors for the highlight appearance.

    pub flexNodeHighlightConfigs: Vec<FlexNodeHighlightConfig>,
}

impl SetShowFlexOverlaysParams { pub const METHOD: &'static str = "Overlay.setShowFlexOverlays"; }

impl crate::CdpCommand for SetShowFlexOverlaysParams {
    const METHOD: &'static str = "Overlay.setShowFlexOverlays";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowScrollSnapOverlaysParams {
    /// An array of node identifiers and descriptors for the highlight appearance.

    pub scrollSnapHighlightConfigs: Vec<ScrollSnapHighlightConfig>,
}

impl SetShowScrollSnapOverlaysParams { pub const METHOD: &'static str = "Overlay.setShowScrollSnapOverlays"; }

impl crate::CdpCommand for SetShowScrollSnapOverlaysParams {
    const METHOD: &'static str = "Overlay.setShowScrollSnapOverlays";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowContainerQueryOverlaysParams {
    /// An array of node identifiers and descriptors for the highlight appearance.

    pub containerQueryHighlightConfigs: Vec<ContainerQueryHighlightConfig>,
}

impl SetShowContainerQueryOverlaysParams { pub const METHOD: &'static str = "Overlay.setShowContainerQueryOverlays"; }

impl crate::CdpCommand for SetShowContainerQueryOverlaysParams {
    const METHOD: &'static str = "Overlay.setShowContainerQueryOverlays";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowInspectedElementAnchorParams {
    /// Node identifier for which to show an anchor for.

    pub inspectedElementAnchorConfig: InspectedElementAnchorConfig,
}

impl SetShowInspectedElementAnchorParams { pub const METHOD: &'static str = "Overlay.setShowInspectedElementAnchor"; }

impl crate::CdpCommand for SetShowInspectedElementAnchorParams {
    const METHOD: &'static str = "Overlay.setShowInspectedElementAnchor";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows paint rectangles

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowPaintRectsParams {
    /// True for showing paint rectangles

    pub result: bool,
}

impl SetShowPaintRectsParams { pub const METHOD: &'static str = "Overlay.setShowPaintRects"; }

impl crate::CdpCommand for SetShowPaintRectsParams {
    const METHOD: &'static str = "Overlay.setShowPaintRects";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows layout shift regions

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowLayoutShiftRegionsParams {
    /// True for showing layout shift regions

    pub result: bool,
}

impl SetShowLayoutShiftRegionsParams { pub const METHOD: &'static str = "Overlay.setShowLayoutShiftRegions"; }

impl crate::CdpCommand for SetShowLayoutShiftRegionsParams {
    const METHOD: &'static str = "Overlay.setShowLayoutShiftRegions";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows scroll bottleneck rects

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowScrollBottleneckRectsParams {
    /// True for showing scroll bottleneck rects

    pub show: bool,
}

impl SetShowScrollBottleneckRectsParams { pub const METHOD: &'static str = "Overlay.setShowScrollBottleneckRects"; }

impl crate::CdpCommand for SetShowScrollBottleneckRectsParams {
    const METHOD: &'static str = "Overlay.setShowScrollBottleneckRects";
    type Response = crate::EmptyReturns;
}

/// Deprecated, no longer has any effect.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowHitTestBordersParams {
    /// True for showing hit-test borders

    pub show: bool,
}

impl SetShowHitTestBordersParams { pub const METHOD: &'static str = "Overlay.setShowHitTestBorders"; }

impl crate::CdpCommand for SetShowHitTestBordersParams {
    const METHOD: &'static str = "Overlay.setShowHitTestBorders";
    type Response = crate::EmptyReturns;
}

/// Deprecated, no longer has any effect.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowWebVitalsParams {

    pub show: bool,
}

impl SetShowWebVitalsParams { pub const METHOD: &'static str = "Overlay.setShowWebVitals"; }

impl crate::CdpCommand for SetShowWebVitalsParams {
    const METHOD: &'static str = "Overlay.setShowWebVitals";
    type Response = crate::EmptyReturns;
}

/// Paints viewport size upon main frame resize.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowViewportSizeOnResizeParams {
    /// Whether to paint size or not.

    pub show: bool,
}

impl SetShowViewportSizeOnResizeParams { pub const METHOD: &'static str = "Overlay.setShowViewportSizeOnResize"; }

impl crate::CdpCommand for SetShowViewportSizeOnResizeParams {
    const METHOD: &'static str = "Overlay.setShowViewportSizeOnResize";
    type Response = crate::EmptyReturns;
}

/// Add a dual screen device hinge

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowHingeParams {
    /// hinge data, null means hideHinge

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hingeConfig: Option<HingeConfig>,
}

impl SetShowHingeParams { pub const METHOD: &'static str = "Overlay.setShowHinge"; }

impl crate::CdpCommand for SetShowHingeParams {
    const METHOD: &'static str = "Overlay.setShowHinge";
    type Response = crate::EmptyReturns;
}

/// Show elements in isolation mode with overlays.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowIsolatedElementsParams {
    /// An array of node identifiers and descriptors for the highlight appearance.

    pub isolatedElementHighlightConfigs: Vec<IsolatedElementHighlightConfig>,
}

impl SetShowIsolatedElementsParams { pub const METHOD: &'static str = "Overlay.setShowIsolatedElements"; }

impl crate::CdpCommand for SetShowIsolatedElementsParams {
    const METHOD: &'static str = "Overlay.setShowIsolatedElements";
    type Response = crate::EmptyReturns;
}

/// Show Window Controls Overlay for PWA

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowWindowControlsOverlayParams {
    /// Window Controls Overlay data, null means hide Window Controls Overlay

    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowControlsOverlayConfig: Option<WindowControlsOverlayConfig>,
}

impl SetShowWindowControlsOverlayParams { pub const METHOD: &'static str = "Overlay.setShowWindowControlsOverlay"; }

impl crate::CdpCommand for SetShowWindowControlsOverlayParams {
    const METHOD: &'static str = "Overlay.setShowWindowControlsOverlay";
    type Response = crate::EmptyReturns;
}
