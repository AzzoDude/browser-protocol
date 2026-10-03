//! This domain provides various functionality related to drawing atop the inspected page.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Configuration data for drawing the source order of an elements children.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SourceOrderConfig {
    /// the color to outline the given element in.
    #[serde(rename = "parentOutlineColor")]
    pub parent_outline_color: crate::dom::RGBA,
    /// the color to outline the child elements in.
    #[serde(rename = "childOutlineColor")]
    pub child_outline_color: crate::dom::RGBA,
}
/// Configuration data for the highlighting of Grid elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GridHighlightConfig {
    /// Whether the extension lines from grid cells to the rulers should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showGridExtensionLines")]
    pub show_grid_extension_lines: Option<bool>,
    /// Show Positive line number labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showPositiveLineNumbers")]
    pub show_positive_line_numbers: Option<bool>,
    /// Show Negative line number labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showNegativeLineNumbers")]
    pub show_negative_line_numbers: Option<bool>,
    /// Show area name labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showAreaNames")]
    pub show_area_names: Option<bool>,
    /// Show line name labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showLineNames")]
    pub show_line_names: Option<bool>,
    /// Show track size labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showTrackSizes")]
    pub show_track_sizes: Option<bool>,
    /// The grid container border highlight color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridBorderColor")]
    pub grid_border_color: Option<crate::dom::RGBA>,
    /// The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cellBorderColor")]
    pub cell_border_color: Option<crate::dom::RGBA>,
    /// The row line color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowLineColor")]
    pub row_line_color: Option<crate::dom::RGBA>,
    /// The column line color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnLineColor")]
    pub column_line_color: Option<crate::dom::RGBA>,
    /// Whether the grid border is dashed (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridBorderDash")]
    pub grid_border_dash: Option<bool>,
    /// Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cellBorderDash")]
    pub cell_border_dash: Option<bool>,
    /// Whether row lines are dashed (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowLineDash")]
    pub row_line_dash: Option<bool>,
    /// Whether column lines are dashed (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnLineDash")]
    pub column_line_dash: Option<bool>,
    /// The row gap highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowGapColor")]
    pub row_gap_color: Option<crate::dom::RGBA>,
    /// The row gap hatching fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowHatchColor")]
    pub row_hatch_color: Option<crate::dom::RGBA>,
    /// The column gap highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnGapColor")]
    pub column_gap_color: Option<crate::dom::RGBA>,
    /// The column gap hatching fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnHatchColor")]
    pub column_hatch_color: Option<crate::dom::RGBA>,
    /// The named grid areas border color (Default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "areaBorderColor")]
    pub area_border_color: Option<crate::dom::RGBA>,
    /// The grid container background color (Default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridBackgroundColor")]
    pub grid_background_color: Option<crate::dom::RGBA>,
}
/// Configuration data for the highlighting of Flex container elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FlexContainerHighlightConfig<'a> {
    /// The style of the container border
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerBorder")]
    pub container_border: Option<LineStyle<'a>>,
    /// The style of the separator between lines
    #[serde(skip_serializing_if = "Option::is_none", rename = "lineSeparator")]
    pub line_separator: Option<LineStyle<'a>>,
    /// The style of the separator between items
    #[serde(skip_serializing_if = "Option::is_none", rename = "itemSeparator")]
    pub item_separator: Option<LineStyle<'a>>,
    /// Style of content-distribution space on the main axis (justify-content).
    #[serde(skip_serializing_if = "Option::is_none", rename = "mainDistributedSpace")]
    pub main_distributed_space: Option<BoxStyle>,
    /// Style of content-distribution space on the cross axis (align-content).
    #[serde(skip_serializing_if = "Option::is_none", rename = "crossDistributedSpace")]
    pub cross_distributed_space: Option<BoxStyle>,
    /// Style of empty space caused by row gaps (gap/row-gap).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowGapSpace")]
    pub row_gap_space: Option<BoxStyle>,
    /// Style of empty space caused by columns gaps (gap/column-gap).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnGapSpace")]
    pub column_gap_space: Option<BoxStyle>,
    /// Style of the self-alignment line (align-items).
    #[serde(skip_serializing_if = "Option::is_none", rename = "crossAlignment")]
    pub cross_alignment: Option<LineStyle<'a>>,
}
/// Configuration data for the highlighting of Flex item elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FlexItemHighlightConfig<'a> {
    /// Style of the box representing the item's base size
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseSizeBox")]
    pub base_size_box: Option<BoxStyle>,
    /// Style of the border around the box representing the item's base size
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseSizeBorder")]
    pub base_size_border: Option<LineStyle<'a>>,
    /// Style of the arrow representing if the item grew or shrank
    #[serde(skip_serializing_if = "Option::is_none", rename = "flexibilityArrow")]
    pub flexibility_arrow: Option<LineStyle<'a>>,
}
/// Style information for drawing a line.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle<'a> {
    /// The color of the line (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::dom::RGBA>,
    /// The line pattern (default: solid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Cow<'a, str>>,
}
/// Style information for drawing a box.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BoxStyle {
    /// The background color for the box (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillColor")]
    pub fill_color: Option<crate::dom::RGBA>,
    /// The hatching color for the box (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none", rename = "hatchColor")]
    pub hatch_color: Option<crate::dom::RGBA>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContrastAlgorithm {
    #[default]
    #[serde(rename = "aa")]
    Aa,
    #[serde(rename = "aaa")]
    Aaa,
    #[serde(rename = "apca")]
    Apca,
}

/// Configuration for Inset-Modified Containing Block (IMCB) and CSS Anchor Positioning highlight.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ImcbHighlightConfig {
    /// Border color for the Inset-Modified Containing Block (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "imcbBorderColor")]
    pub imcb_border_color: Option<crate::dom::RGBA>,
    /// Background fill color for the Inset-Modified Containing Block (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "imcbBackgroundColor")]
    pub imcb_background_color: Option<crate::dom::RGBA>,
    /// Fill color for the inset modifiers area (difference between CB and IMCB).
    #[serde(skip_serializing_if = "Option::is_none", rename = "insetsBackgroundColor")]
    pub insets_background_color: Option<crate::dom::RGBA>,
    /// Hatch color for the inset modifiers area.
    #[serde(skip_serializing_if = "Option::is_none", rename = "insetsHatchColor")]
    pub insets_hatch_color: Option<crate::dom::RGBA>,
    /// Border color for the referenced target anchor element(s) (when element is anchor-positioned).
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorBorderColor")]
    pub anchor_border_color: Option<crate::dom::RGBA>,
    /// Background fill color for the referenced target anchor element(s) (when element is anchor-positioned).
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorBackgroundColor")]
    pub anchor_background_color: Option<crate::dom::RGBA>,
    /// Whether to render the 3x3 position-area grid lines when position-area is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "showPositionAreaGrid")]
    pub show_position_area_grid: Option<bool>,
    /// Line color for the 3x3 position-area grid lines.
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionAreaGridLineColor")]
    pub position_area_grid_line_color: Option<crate::dom::RGBA>,
    /// Fill color for the active region within the position-area grid.
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionAreaActiveRegionColor")]
    pub position_area_active_region_color: Option<crate::dom::RGBA>,
}
/// Configuration data for the highlighting of page elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct HighlightConfig<'a> {
    /// Whether the node info tooltip should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showInfo")]
    pub show_info: Option<bool>,
    /// Whether the node styles in the tooltip (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showStyles")]
    pub show_styles: Option<bool>,
    /// Whether the rulers should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showRulers")]
    pub show_rulers: Option<bool>,
    /// Whether the a11y info should be shown (default: true).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showAccessibilityInfo")]
    pub show_accessibility_info: Option<bool>,
    /// Whether the extension lines from node to the rulers should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showExtensionLines")]
    pub show_extension_lines: Option<bool>,
    /// The content box highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentColor")]
    pub content_color: Option<crate::dom::RGBA>,
    /// The padding highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "paddingColor")]
    pub padding_color: Option<crate::dom::RGBA>,
    /// The border highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderColor")]
    pub border_color: Option<crate::dom::RGBA>,
    /// The margin highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginColor")]
    pub margin_color: Option<crate::dom::RGBA>,
    /// The event target element highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "eventTargetColor")]
    pub event_target_color: Option<crate::dom::RGBA>,
    /// The shape outside fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "shapeColor")]
    pub shape_color: Option<crate::dom::RGBA>,
    /// The shape margin fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "shapeMarginColor")]
    pub shape_margin_color: Option<crate::dom::RGBA>,
    /// The grid layout color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssGridColor")]
    pub css_grid_color: Option<crate::dom::RGBA>,
    /// The color format used to format color styles (default: hex).
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorFormat")]
    pub color_format: Option<ColorFormat>,
    /// The grid layout highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridHighlightConfig")]
    pub grid_highlight_config: Option<GridHighlightConfig>,
    /// The flex container highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "flexContainerHighlightConfig")]
    pub flex_container_highlight_config: Option<FlexContainerHighlightConfig<'a>>,
    /// The flex item highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "flexItemHighlightConfig")]
    pub flex_item_highlight_config: Option<FlexItemHighlightConfig<'a>>,
    /// The contrast algorithm to use for the contrast ratio (default: aa).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contrastAlgorithm")]
    pub contrast_algorithm: Option<ContrastAlgorithm>,
    /// The container query container highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerQueryContainerHighlightConfig")]
    pub container_query_container_highlight_config: Option<ContainerQueryContainerHighlightConfig<'a>>,
    /// The IMCB highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "imcbHighlightConfig")]
    pub imcb_highlight_config: Option<ImcbHighlightConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ColorFormat {
    #[default]
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "hsl")]
    Hsl,
    #[serde(rename = "hwb")]
    Hwb,
    #[serde(rename = "hex")]
    Hex,
}

/// Configurations for Persistent Grid Highlight

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GridNodeHighlightConfig {
    /// A descriptor for the highlight appearance.
    #[serde(rename = "gridHighlightConfig")]
    pub grid_highlight_config: GridHighlightConfig,
    /// Identifier of the node to highlight.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FlexNodeHighlightConfig<'a> {
    /// A descriptor for the highlight appearance of flex containers.
    #[serde(rename = "flexContainerHighlightConfig")]
    pub flex_container_highlight_config: FlexContainerHighlightConfig<'a>,
    /// Identifier of the node to highlight.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapContainerHighlightConfig<'a> {
    /// The style of the snapport border (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none", rename = "snapportBorder")]
    pub snapport_border: Option<LineStyle<'a>>,
    /// The style of the snap area border (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none", rename = "snapAreaBorder")]
    pub snap_area_border: Option<LineStyle<'a>>,
    /// The margin highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollMarginColor")]
    pub scroll_margin_color: Option<crate::dom::RGBA>,
    /// The padding highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollPaddingColor")]
    pub scroll_padding_color: Option<crate::dom::RGBA>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapHighlightConfig<'a> {
    /// A descriptor for the highlight appearance of scroll snap containers.
    #[serde(rename = "scrollSnapContainerHighlightConfig")]
    pub scroll_snap_container_highlight_config: ScrollSnapContainerHighlightConfig<'a>,
    /// Identifier of the node to highlight.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}
/// Configuration for dual screen hinge

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct HingeConfig {
    /// A rectangle represent hinge
    pub rect: crate::dom::Rect,
    /// The content box highlight fill color (default: a dark color).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentColor")]
    pub content_color: Option<crate::dom::RGBA>,
    /// The content box highlight outline color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "outlineColor")]
    pub outline_color: Option<crate::dom::RGBA>,
}
/// Supported display cutout shapes.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DisplayCutoutShape {
    #[default]
    #[serde(rename = "pill")]
    Pill,
    #[serde(rename = "notch")]
    Notch,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "rectangle")]
    Rectangle,
}

/// Configuration for a display cutout.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DisplayCutoutConfig {
    /// A rectangle representing the cutout bounds.
    pub rect: crate::dom::Rect,
    /// Shape used to draw the cutout.
    pub shape: DisplayCutoutShape,
    /// Border radius for rounded cutout shapes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderRadius")]
    pub border_radius: Option<i64>,
    /// Upper shoulder radius for notch cutout shapes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "upperRadius")]
    pub upper_radius: Option<i64>,
    /// Lower transition radius for notch cutout shapes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "lowerRadius")]
    pub lower_radius: Option<i64>,
    /// Center x coordinate for circle cutout shapes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cx: Option<i64>,
    /// Center y coordinate for circle cutout shapes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cy: Option<i64>,
    /// Radius for circle cutout shapes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<i64>,
    /// The cutout fill color (default: black).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentColor")]
    pub content_color: Option<crate::dom::RGBA>,
}
/// Configuration for Window Controls Overlay

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct WindowControlsOverlayConfig<'a> {
    /// Whether the title bar CSS should be shown when emulating the Window Controls Overlay.
    #[serde(rename = "showCSS")]
    pub show_css: bool,
    /// Selected platforms to show the overlay.
    #[serde(rename = "selectedPlatform")]
    pub selected_platform: Cow<'a, str>,
    /// The theme color defined in app manifest.
    #[serde(rename = "themeColor")]
    pub theme_color: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryHighlightConfig<'a> {
    /// A descriptor for the highlight appearance of container query containers.
    #[serde(rename = "containerQueryContainerHighlightConfig")]
    pub container_query_container_highlight_config: ContainerQueryContainerHighlightConfig<'a>,
    /// Identifier of the container node to highlight.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryContainerHighlightConfig<'a> {
    /// The style of the container border.
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerBorder")]
    pub container_border: Option<LineStyle<'a>>,
    /// The style of the descendants' borders.
    #[serde(skip_serializing_if = "Option::is_none", rename = "descendantBorder")]
    pub descendant_border: Option<LineStyle<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedElementHighlightConfig {
    /// A descriptor for the highlight appearance of an element in isolation mode.
    #[serde(rename = "isolationModeHighlightConfig")]
    pub isolation_mode_highlight_config: IsolationModeHighlightConfig,
    /// Identifier of the isolated element to highlight.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct IsolationModeHighlightConfig {
    /// The fill color of the resizers (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resizerColor")]
    pub resizer_color: Option<crate::dom::RGBA>,
    /// The fill color for resizer handles (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resizerHandleColor")]
    pub resizer_handle_color: Option<crate::dom::RGBA>,
    /// The fill color for the mask covering non-isolated elements (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "maskColor")]
    pub mask_color: Option<crate::dom::RGBA>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InspectMode {
    #[default]
    #[serde(rename = "searchForNode")]
    SearchForNode,
    #[serde(rename = "searchForUAShadowDOM")]
    SearchForUAShadowDOM,
    #[serde(rename = "captureAreaScreenshot")]
    CaptureAreaScreenshot,
    #[serde(rename = "none")]
    None,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InspectedElementAnchorConfig {
    /// Identifier of the node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
}
/// Disables domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.disable")]
pub struct DisableParams {

}
/// Enables domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.enable")]
pub struct EnableParams {

}
/// For testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.getHighlightObjectForTest", response = "GetHighlightObjectForTestReturns")]
pub struct GetHighlightObjectForTestParams {
    /// Id of the node to get highlight object for.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    /// Whether to include distance info.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeDistance")]
    pub include_distance: Option<bool>,
    /// Whether to include style info.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeStyle")]
    pub include_style: Option<bool>,
    /// The color format to get config with (default: hex).
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorFormat")]
    pub color_format: Option<ColorFormat>,
    /// Whether to show accessibility info (default: true).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showAccessibilityInfo")]
    pub show_accessibility_info: Option<bool>,
}
/// For testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetHighlightObjectForTestReturns {
    /// Highlight data for the node.
    pub highlight: serde_json::Map<String, JsonValue>,
}
/// For Persistent Grid testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.getGridHighlightObjectsForTest", response = "GetGridHighlightObjectsForTestReturns")]
pub struct GetGridHighlightObjectsForTestParams {
    /// Ids of the node to get highlight object for.
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<crate::dom::NodeId>,
}
/// For Persistent Grid testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetGridHighlightObjectsForTestReturns {
    /// Grid Highlight data for the node ids provided.
    pub highlights: serde_json::Map<String, JsonValue>,
}
/// For Source Order Viewer testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.getSourceOrderHighlightObjectForTest", response = "GetSourceOrderHighlightObjectForTestReturns")]
pub struct GetSourceOrderHighlightObjectForTestParams {
    /// Id of the node to highlight.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}
/// For Source Order Viewer testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceOrderHighlightObjectForTestReturns {
    /// Source order highlight data for the node id provided.
    pub highlight: serde_json::Map<String, JsonValue>,
}
/// Hides any highlight.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.hideHighlight")]
pub struct HideHighlightParams {

}
/// Highlights owner element of the frame with given id.
/// Deprecated: Doesn't work reliably and cannot be fixed due to process
/// separation (the owner node might be in a different process). Determine
/// the owner node in the client and use highlightNode.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.highlightFrame")]
pub struct HighlightFrameParams<'a> {
    /// Identifier of the frame to highlight.
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
    /// The content box highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentColor")]
    pub content_color: Option<crate::dom::RGBA>,
    /// The content box highlight outline color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentOutlineColor")]
    pub content_outline_color: Option<crate::dom::RGBA>,
}
/// Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or
/// objectId must be specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.highlightNode")]
pub struct HighlightNodeParams<'a> {
    /// A descriptor for the highlight appearance.
    #[serde(rename = "highlightConfig")]
    pub highlight_config: HighlightConfig<'a>,
    /// Identifier of the node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node to be highlighted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Selectors to highlight relevant nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<Cow<'a, str>>,
}
/// Highlights given quad. Coordinates are absolute with respect to the main frame viewport.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.highlightQuad")]
pub struct HighlightQuadParams {
    /// Quad to highlight
    pub quad: crate::dom::Quad,
    /// The highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::dom::RGBA>,
    /// The highlight outline color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "outlineColor")]
    pub outline_color: Option<crate::dom::RGBA>,
}
/// Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.
/// Issue: the method does not handle device pixel ratio (DPR) correctly.
/// The coordinates currently have to be adjusted by the client
/// if DPR is not 1 (see crbug.com/437807128).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.highlightRect")]
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "outlineColor")]
    pub outline_color: Option<crate::dom::RGBA>,
}
/// Highlights the source order of the children of the DOM node with given id or with the given
/// JavaScript object wrapper. Either nodeId or objectId must be specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.highlightSourceOrder")]
pub struct HighlightSourceOrderParams<'a> {
    /// A descriptor for the appearance of the overlay drawing.
    #[serde(rename = "sourceOrderConfig")]
    pub source_order_config: SourceOrderConfig,
    /// Identifier of the node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node to be highlighted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}
/// Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.
/// Backend then generates 'inspectNodeRequested' event upon element selection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setInspectMode")]
pub struct SetInspectModeParams<'a> {
    /// Set an inspection mode.
    pub mode: InspectMode,
    /// A descriptor for the highlight appearance of hovered-over nodes. May be omitted if 'enabled
    /// == false'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "highlightConfig")]
    pub highlight_config: Option<HighlightConfig<'a>>,
}
/// Highlights owner element of all frames detected to be ads.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowAdHighlights")]
pub struct SetShowAdHighlightsParams {
    /// True for showing ad highlights
    pub show: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setPausedInDebuggerMessage")]
pub struct SetPausedInDebuggerMessageParams<'a> {
    /// The message to display, also triggers resume and step over controls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Cow<'a, str>>,
}
/// Requests that backend shows debug borders on layers

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowDebugBorders")]
pub struct SetShowDebugBordersParams {
    /// True for showing debug borders
    pub show: bool,
}
/// Requests that backend shows the FPS counter

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowFPSCounter")]
pub struct SetShowFPSCounterParams {
    /// True for showing the FPS counter
    pub show: bool,
}
/// Highlight multiple elements with the CSS Grid overlay.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowGridOverlays")]
pub struct SetShowGridOverlaysParams {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "gridNodeHighlightConfigs")]
    pub grid_node_highlight_configs: Vec<GridNodeHighlightConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowFlexOverlays")]
pub struct SetShowFlexOverlaysParams<'a> {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "flexNodeHighlightConfigs")]
    pub flex_node_highlight_configs: Vec<FlexNodeHighlightConfig<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowScrollSnapOverlays")]
pub struct SetShowScrollSnapOverlaysParams<'a> {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "scrollSnapHighlightConfigs")]
    pub scroll_snap_highlight_configs: Vec<ScrollSnapHighlightConfig<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowContainerQueryOverlays")]
pub struct SetShowContainerQueryOverlaysParams<'a> {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "containerQueryHighlightConfigs")]
    pub container_query_highlight_configs: Vec<ContainerQueryHighlightConfig<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowInspectedElementAnchor")]
pub struct SetShowInspectedElementAnchorParams {
    /// Node identifier for which to show an anchor for.
    #[serde(rename = "inspectedElementAnchorConfig")]
    pub inspected_element_anchor_config: InspectedElementAnchorConfig,
}
/// Requests that backend shows paint rectangles

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowPaintRects")]
pub struct SetShowPaintRectsParams {
    /// True for showing paint rectangles
    pub result: bool,
}
/// Requests that backend shows layout shift regions

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowLayoutShiftRegions")]
pub struct SetShowLayoutShiftRegionsParams {
    /// True for showing layout shift regions
    pub result: bool,
}
/// Requests that backend shows scroll bottleneck rects

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowScrollBottleneckRects")]
pub struct SetShowScrollBottleneckRectsParams {
    /// True for showing scroll bottleneck rects
    pub show: bool,
}
/// Deprecated, no longer has any effect.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowHitTestBorders")]
pub struct SetShowHitTestBordersParams {
    /// True for showing hit-test borders
    pub show: bool,
}
/// Deprecated, no longer has any effect.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowWebVitals")]
pub struct SetShowWebVitalsParams {
    pub show: bool,
}
/// Paints viewport size upon main frame resize.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowViewportSizeOnResize")]
pub struct SetShowViewportSizeOnResizeParams {
    /// Whether to paint size or not.
    pub show: bool,
}
/// Add a dual screen device hinge

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowHinge")]
pub struct SetShowHingeParams {
    /// hinge data, null means hideHinge
    #[serde(skip_serializing_if = "Option::is_none", rename = "hingeConfig")]
    pub hinge_config: Option<HingeConfig>,
}
/// Add a display cutout overlay.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowDisplayCutout")]
pub struct SetShowDisplayCutoutParams {
    /// display cutout data, null means hide display cutout
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayCutoutConfig")]
    pub display_cutout_config: Option<DisplayCutoutConfig>,
}
/// Show elements in isolation mode with overlays.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowIsolatedElements")]
pub struct SetShowIsolatedElementsParams {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "isolatedElementHighlightConfigs")]
    pub isolated_element_highlight_configs: Vec<IsolatedElementHighlightConfig>,
}
/// Show Window Controls Overlay for PWA

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.setShowWindowControlsOverlay")]
pub struct SetShowWindowControlsOverlayParams<'a> {
    /// Window Controls Overlay data, null means hide Window Controls Overlay
    #[serde(skip_serializing_if = "Option::is_none", rename = "windowControlsOverlayConfig")]
    pub window_controls_overlay_config: Option<WindowControlsOverlayConfig<'a>>,
}
/// Fired when the node should be inspected. This happens after call to 'setInspectMode' or when
/// user manually inspects an element.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.inspectNodeRequested")]
pub struct InspectNodeRequested {
    /// Id of the node to inspect.
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: crate::dom::BackendNodeId,
}
/// Fired when the node should be highlighted. This happens after call to 'setInspectMode'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.nodeHighlightRequested")]
pub struct NodeHighlightRequested {
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}
/// Fired when user asks to capture screenshot of some area on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.screenshotRequested")]
pub struct ScreenshotRequested {
    /// Viewport to capture, in device independent pixels (dip).
    pub viewport: crate::page::Viewport,
}
/// Fired when user asks to show the Inspect panel.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.inspectPanelShowRequested")]
pub struct InspectPanelShowRequested {
    /// Id of the node to show in the panel.
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: crate::dom::BackendNodeId,
}
/// Fired when user asks to restore the Inspected Element floating window.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.inspectedElementWindowRestored")]
pub struct InspectedElementWindowRestored {
    /// Id of the node to restore the floating window for.
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: crate::dom::BackendNodeId,
}
/// Fired when user cancels the inspect mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Overlay.inspectModeCanceled")]
pub struct InspectModeCanceled {

}