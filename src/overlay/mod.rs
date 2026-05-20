//! This domain provides various functionality related to drawing atop the inspected page.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Configuration data for drawing the source order of an elements children.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceOrderConfig {
    /// the color to outline the given element in.
    #[serde(rename = "parentOutlineColor")]
    parent_outline_color: crate::dom::RGBA,
    /// the color to outline the child elements in.
    #[serde(rename = "childOutlineColor")]
    child_outline_color: crate::dom::RGBA,
}

impl SourceOrderConfig {
    /// Creates a builder for this type with the required parameters:
    /// * `parent_outline_color`: the color to outline the given element in.
    /// * `child_outline_color`: the color to outline the child elements in.
    pub fn builder(parent_outline_color: crate::dom::RGBA, child_outline_color: crate::dom::RGBA) -> SourceOrderConfigBuilder {
        SourceOrderConfigBuilder {
            parent_outline_color: parent_outline_color,
            child_outline_color: child_outline_color,
        }
    }
    /// the color to outline the given element in.
    pub fn parent_outline_color(&self) -> &crate::dom::RGBA { &self.parent_outline_color }
    /// the color to outline the child elements in.
    pub fn child_outline_color(&self) -> &crate::dom::RGBA { &self.child_outline_color }
}


pub struct SourceOrderConfigBuilder {
    parent_outline_color: crate::dom::RGBA,
    child_outline_color: crate::dom::RGBA,
}

impl SourceOrderConfigBuilder {
    pub fn build(self) -> SourceOrderConfig {
        SourceOrderConfig {
            parent_outline_color: self.parent_outline_color,
            child_outline_color: self.child_outline_color,
        }
    }
}

/// Configuration data for the highlighting of Grid elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GridHighlightConfig {
    /// Whether the extension lines from grid cells to the rulers should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showGridExtensionLines")]
    show_grid_extension_lines: Option<bool>,
    /// Show Positive line number labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showPositiveLineNumbers")]
    show_positive_line_numbers: Option<bool>,
    /// Show Negative line number labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showNegativeLineNumbers")]
    show_negative_line_numbers: Option<bool>,
    /// Show area name labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showAreaNames")]
    show_area_names: Option<bool>,
    /// Show line name labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showLineNames")]
    show_line_names: Option<bool>,
    /// Show track size labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showTrackSizes")]
    show_track_sizes: Option<bool>,
    /// The grid container border highlight color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridBorderColor")]
    grid_border_color: Option<crate::dom::RGBA>,
    /// The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cellBorderColor")]
    cell_border_color: Option<crate::dom::RGBA>,
    /// The row line color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowLineColor")]
    row_line_color: Option<crate::dom::RGBA>,
    /// The column line color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnLineColor")]
    column_line_color: Option<crate::dom::RGBA>,
    /// Whether the grid border is dashed (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridBorderDash")]
    grid_border_dash: Option<bool>,
    /// Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cellBorderDash")]
    cell_border_dash: Option<bool>,
    /// Whether row lines are dashed (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowLineDash")]
    row_line_dash: Option<bool>,
    /// Whether column lines are dashed (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnLineDash")]
    column_line_dash: Option<bool>,
    /// The row gap highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowGapColor")]
    row_gap_color: Option<crate::dom::RGBA>,
    /// The row gap hatching fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowHatchColor")]
    row_hatch_color: Option<crate::dom::RGBA>,
    /// The column gap highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnGapColor")]
    column_gap_color: Option<crate::dom::RGBA>,
    /// The column gap hatching fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnHatchColor")]
    column_hatch_color: Option<crate::dom::RGBA>,
    /// The named grid areas border color (Default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "areaBorderColor")]
    area_border_color: Option<crate::dom::RGBA>,
    /// The grid container background color (Default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridBackgroundColor")]
    grid_background_color: Option<crate::dom::RGBA>,
}

impl GridHighlightConfig {
    /// Creates a builder for this type.
    pub fn builder() -> GridHighlightConfigBuilder {
        GridHighlightConfigBuilder {
            show_grid_extension_lines: None,
            show_positive_line_numbers: None,
            show_negative_line_numbers: None,
            show_area_names: None,
            show_line_names: None,
            show_track_sizes: None,
            grid_border_color: None,
            cell_border_color: None,
            row_line_color: None,
            column_line_color: None,
            grid_border_dash: None,
            cell_border_dash: None,
            row_line_dash: None,
            column_line_dash: None,
            row_gap_color: None,
            row_hatch_color: None,
            column_gap_color: None,
            column_hatch_color: None,
            area_border_color: None,
            grid_background_color: None,
        }
    }
    /// Whether the extension lines from grid cells to the rulers should be shown (default: false).
    pub fn show_grid_extension_lines(&self) -> Option<bool> { self.show_grid_extension_lines }
    /// Show Positive line number labels (default: false).
    pub fn show_positive_line_numbers(&self) -> Option<bool> { self.show_positive_line_numbers }
    /// Show Negative line number labels (default: false).
    pub fn show_negative_line_numbers(&self) -> Option<bool> { self.show_negative_line_numbers }
    /// Show area name labels (default: false).
    pub fn show_area_names(&self) -> Option<bool> { self.show_area_names }
    /// Show line name labels (default: false).
    pub fn show_line_names(&self) -> Option<bool> { self.show_line_names }
    /// Show track size labels (default: false).
    pub fn show_track_sizes(&self) -> Option<bool> { self.show_track_sizes }
    /// The grid container border highlight color (default: transparent).
    pub fn grid_border_color(&self) -> Option<&crate::dom::RGBA> { self.grid_border_color.as_ref() }
    /// The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.
    pub fn cell_border_color(&self) -> Option<&crate::dom::RGBA> { self.cell_border_color.as_ref() }
    /// The row line color (default: transparent).
    pub fn row_line_color(&self) -> Option<&crate::dom::RGBA> { self.row_line_color.as_ref() }
    /// The column line color (default: transparent).
    pub fn column_line_color(&self) -> Option<&crate::dom::RGBA> { self.column_line_color.as_ref() }
    /// Whether the grid border is dashed (default: false).
    pub fn grid_border_dash(&self) -> Option<bool> { self.grid_border_dash }
    /// Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.
    pub fn cell_border_dash(&self) -> Option<bool> { self.cell_border_dash }
    /// Whether row lines are dashed (default: false).
    pub fn row_line_dash(&self) -> Option<bool> { self.row_line_dash }
    /// Whether column lines are dashed (default: false).
    pub fn column_line_dash(&self) -> Option<bool> { self.column_line_dash }
    /// The row gap highlight fill color (default: transparent).
    pub fn row_gap_color(&self) -> Option<&crate::dom::RGBA> { self.row_gap_color.as_ref() }
    /// The row gap hatching fill color (default: transparent).
    pub fn row_hatch_color(&self) -> Option<&crate::dom::RGBA> { self.row_hatch_color.as_ref() }
    /// The column gap highlight fill color (default: transparent).
    pub fn column_gap_color(&self) -> Option<&crate::dom::RGBA> { self.column_gap_color.as_ref() }
    /// The column gap hatching fill color (default: transparent).
    pub fn column_hatch_color(&self) -> Option<&crate::dom::RGBA> { self.column_hatch_color.as_ref() }
    /// The named grid areas border color (Default: transparent).
    pub fn area_border_color(&self) -> Option<&crate::dom::RGBA> { self.area_border_color.as_ref() }
    /// The grid container background color (Default: transparent).
    pub fn grid_background_color(&self) -> Option<&crate::dom::RGBA> { self.grid_background_color.as_ref() }
}

#[derive(Default)]
pub struct GridHighlightConfigBuilder {
    show_grid_extension_lines: Option<bool>,
    show_positive_line_numbers: Option<bool>,
    show_negative_line_numbers: Option<bool>,
    show_area_names: Option<bool>,
    show_line_names: Option<bool>,
    show_track_sizes: Option<bool>,
    grid_border_color: Option<crate::dom::RGBA>,
    cell_border_color: Option<crate::dom::RGBA>,
    row_line_color: Option<crate::dom::RGBA>,
    column_line_color: Option<crate::dom::RGBA>,
    grid_border_dash: Option<bool>,
    cell_border_dash: Option<bool>,
    row_line_dash: Option<bool>,
    column_line_dash: Option<bool>,
    row_gap_color: Option<crate::dom::RGBA>,
    row_hatch_color: Option<crate::dom::RGBA>,
    column_gap_color: Option<crate::dom::RGBA>,
    column_hatch_color: Option<crate::dom::RGBA>,
    area_border_color: Option<crate::dom::RGBA>,
    grid_background_color: Option<crate::dom::RGBA>,
}

impl GridHighlightConfigBuilder {
    /// Whether the extension lines from grid cells to the rulers should be shown (default: false).
    pub fn show_grid_extension_lines(mut self, show_grid_extension_lines: bool) -> Self { self.show_grid_extension_lines = Some(show_grid_extension_lines); self }
    /// Show Positive line number labels (default: false).
    pub fn show_positive_line_numbers(mut self, show_positive_line_numbers: bool) -> Self { self.show_positive_line_numbers = Some(show_positive_line_numbers); self }
    /// Show Negative line number labels (default: false).
    pub fn show_negative_line_numbers(mut self, show_negative_line_numbers: bool) -> Self { self.show_negative_line_numbers = Some(show_negative_line_numbers); self }
    /// Show area name labels (default: false).
    pub fn show_area_names(mut self, show_area_names: bool) -> Self { self.show_area_names = Some(show_area_names); self }
    /// Show line name labels (default: false).
    pub fn show_line_names(mut self, show_line_names: bool) -> Self { self.show_line_names = Some(show_line_names); self }
    /// Show track size labels (default: false).
    pub fn show_track_sizes(mut self, show_track_sizes: bool) -> Self { self.show_track_sizes = Some(show_track_sizes); self }
    /// The grid container border highlight color (default: transparent).
    pub fn grid_border_color(mut self, grid_border_color: crate::dom::RGBA) -> Self { self.grid_border_color = Some(grid_border_color); self }
    /// The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.
    pub fn cell_border_color(mut self, cell_border_color: crate::dom::RGBA) -> Self { self.cell_border_color = Some(cell_border_color); self }
    /// The row line color (default: transparent).
    pub fn row_line_color(mut self, row_line_color: crate::dom::RGBA) -> Self { self.row_line_color = Some(row_line_color); self }
    /// The column line color (default: transparent).
    pub fn column_line_color(mut self, column_line_color: crate::dom::RGBA) -> Self { self.column_line_color = Some(column_line_color); self }
    /// Whether the grid border is dashed (default: false).
    pub fn grid_border_dash(mut self, grid_border_dash: bool) -> Self { self.grid_border_dash = Some(grid_border_dash); self }
    /// Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.
    pub fn cell_border_dash(mut self, cell_border_dash: bool) -> Self { self.cell_border_dash = Some(cell_border_dash); self }
    /// Whether row lines are dashed (default: false).
    pub fn row_line_dash(mut self, row_line_dash: bool) -> Self { self.row_line_dash = Some(row_line_dash); self }
    /// Whether column lines are dashed (default: false).
    pub fn column_line_dash(mut self, column_line_dash: bool) -> Self { self.column_line_dash = Some(column_line_dash); self }
    /// The row gap highlight fill color (default: transparent).
    pub fn row_gap_color(mut self, row_gap_color: crate::dom::RGBA) -> Self { self.row_gap_color = Some(row_gap_color); self }
    /// The row gap hatching fill color (default: transparent).
    pub fn row_hatch_color(mut self, row_hatch_color: crate::dom::RGBA) -> Self { self.row_hatch_color = Some(row_hatch_color); self }
    /// The column gap highlight fill color (default: transparent).
    pub fn column_gap_color(mut self, column_gap_color: crate::dom::RGBA) -> Self { self.column_gap_color = Some(column_gap_color); self }
    /// The column gap hatching fill color (default: transparent).
    pub fn column_hatch_color(mut self, column_hatch_color: crate::dom::RGBA) -> Self { self.column_hatch_color = Some(column_hatch_color); self }
    /// The named grid areas border color (Default: transparent).
    pub fn area_border_color(mut self, area_border_color: crate::dom::RGBA) -> Self { self.area_border_color = Some(area_border_color); self }
    /// The grid container background color (Default: transparent).
    pub fn grid_background_color(mut self, grid_background_color: crate::dom::RGBA) -> Self { self.grid_background_color = Some(grid_background_color); self }
    pub fn build(self) -> GridHighlightConfig {
        GridHighlightConfig {
            show_grid_extension_lines: self.show_grid_extension_lines,
            show_positive_line_numbers: self.show_positive_line_numbers,
            show_negative_line_numbers: self.show_negative_line_numbers,
            show_area_names: self.show_area_names,
            show_line_names: self.show_line_names,
            show_track_sizes: self.show_track_sizes,
            grid_border_color: self.grid_border_color,
            cell_border_color: self.cell_border_color,
            row_line_color: self.row_line_color,
            column_line_color: self.column_line_color,
            grid_border_dash: self.grid_border_dash,
            cell_border_dash: self.cell_border_dash,
            row_line_dash: self.row_line_dash,
            column_line_dash: self.column_line_dash,
            row_gap_color: self.row_gap_color,
            row_hatch_color: self.row_hatch_color,
            column_gap_color: self.column_gap_color,
            column_hatch_color: self.column_hatch_color,
            area_border_color: self.area_border_color,
            grid_background_color: self.grid_background_color,
        }
    }
}

/// Configuration data for the highlighting of Flex container elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexContainerHighlightConfig<'a> {
    /// The style of the container border
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerBorder")]
    container_border: Option<LineStyle<'a>>,
    /// The style of the separator between lines
    #[serde(skip_serializing_if = "Option::is_none", rename = "lineSeparator")]
    line_separator: Option<LineStyle<'a>>,
    /// The style of the separator between items
    #[serde(skip_serializing_if = "Option::is_none", rename = "itemSeparator")]
    item_separator: Option<LineStyle<'a>>,
    /// Style of content-distribution space on the main axis (justify-content).
    #[serde(skip_serializing_if = "Option::is_none", rename = "mainDistributedSpace")]
    main_distributed_space: Option<BoxStyle>,
    /// Style of content-distribution space on the cross axis (align-content).
    #[serde(skip_serializing_if = "Option::is_none", rename = "crossDistributedSpace")]
    cross_distributed_space: Option<BoxStyle>,
    /// Style of empty space caused by row gaps (gap/row-gap).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rowGapSpace")]
    row_gap_space: Option<BoxStyle>,
    /// Style of empty space caused by columns gaps (gap/column-gap).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnGapSpace")]
    column_gap_space: Option<BoxStyle>,
    /// Style of the self-alignment line (align-items).
    #[serde(skip_serializing_if = "Option::is_none", rename = "crossAlignment")]
    cross_alignment: Option<LineStyle<'a>>,
}

impl<'a> FlexContainerHighlightConfig<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> FlexContainerHighlightConfigBuilder<'a> {
        FlexContainerHighlightConfigBuilder {
            container_border: None,
            line_separator: None,
            item_separator: None,
            main_distributed_space: None,
            cross_distributed_space: None,
            row_gap_space: None,
            column_gap_space: None,
            cross_alignment: None,
        }
    }
    /// The style of the container border
    pub fn container_border(&self) -> Option<&LineStyle<'a>> { self.container_border.as_ref() }
    /// The style of the separator between lines
    pub fn line_separator(&self) -> Option<&LineStyle<'a>> { self.line_separator.as_ref() }
    /// The style of the separator between items
    pub fn item_separator(&self) -> Option<&LineStyle<'a>> { self.item_separator.as_ref() }
    /// Style of content-distribution space on the main axis (justify-content).
    pub fn main_distributed_space(&self) -> Option<&BoxStyle> { self.main_distributed_space.as_ref() }
    /// Style of content-distribution space on the cross axis (align-content).
    pub fn cross_distributed_space(&self) -> Option<&BoxStyle> { self.cross_distributed_space.as_ref() }
    /// Style of empty space caused by row gaps (gap/row-gap).
    pub fn row_gap_space(&self) -> Option<&BoxStyle> { self.row_gap_space.as_ref() }
    /// Style of empty space caused by columns gaps (gap/column-gap).
    pub fn column_gap_space(&self) -> Option<&BoxStyle> { self.column_gap_space.as_ref() }
    /// Style of the self-alignment line (align-items).
    pub fn cross_alignment(&self) -> Option<&LineStyle<'a>> { self.cross_alignment.as_ref() }
}

#[derive(Default)]
pub struct FlexContainerHighlightConfigBuilder<'a> {
    container_border: Option<LineStyle<'a>>,
    line_separator: Option<LineStyle<'a>>,
    item_separator: Option<LineStyle<'a>>,
    main_distributed_space: Option<BoxStyle>,
    cross_distributed_space: Option<BoxStyle>,
    row_gap_space: Option<BoxStyle>,
    column_gap_space: Option<BoxStyle>,
    cross_alignment: Option<LineStyle<'a>>,
}

impl<'a> FlexContainerHighlightConfigBuilder<'a> {
    /// The style of the container border
    pub fn container_border(mut self, container_border: LineStyle<'a>) -> Self { self.container_border = Some(container_border); self }
    /// The style of the separator between lines
    pub fn line_separator(mut self, line_separator: LineStyle<'a>) -> Self { self.line_separator = Some(line_separator); self }
    /// The style of the separator between items
    pub fn item_separator(mut self, item_separator: LineStyle<'a>) -> Self { self.item_separator = Some(item_separator); self }
    /// Style of content-distribution space on the main axis (justify-content).
    pub fn main_distributed_space(mut self, main_distributed_space: BoxStyle) -> Self { self.main_distributed_space = Some(main_distributed_space); self }
    /// Style of content-distribution space on the cross axis (align-content).
    pub fn cross_distributed_space(mut self, cross_distributed_space: BoxStyle) -> Self { self.cross_distributed_space = Some(cross_distributed_space); self }
    /// Style of empty space caused by row gaps (gap/row-gap).
    pub fn row_gap_space(mut self, row_gap_space: BoxStyle) -> Self { self.row_gap_space = Some(row_gap_space); self }
    /// Style of empty space caused by columns gaps (gap/column-gap).
    pub fn column_gap_space(mut self, column_gap_space: BoxStyle) -> Self { self.column_gap_space = Some(column_gap_space); self }
    /// Style of the self-alignment line (align-items).
    pub fn cross_alignment(mut self, cross_alignment: LineStyle<'a>) -> Self { self.cross_alignment = Some(cross_alignment); self }
    pub fn build(self) -> FlexContainerHighlightConfig<'a> {
        FlexContainerHighlightConfig {
            container_border: self.container_border,
            line_separator: self.line_separator,
            item_separator: self.item_separator,
            main_distributed_space: self.main_distributed_space,
            cross_distributed_space: self.cross_distributed_space,
            row_gap_space: self.row_gap_space,
            column_gap_space: self.column_gap_space,
            cross_alignment: self.cross_alignment,
        }
    }
}

/// Configuration data for the highlighting of Flex item elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexItemHighlightConfig<'a> {
    /// Style of the box representing the item's base size
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseSizeBox")]
    base_size_box: Option<BoxStyle>,
    /// Style of the border around the box representing the item's base size
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseSizeBorder")]
    base_size_border: Option<LineStyle<'a>>,
    /// Style of the arrow representing if the item grew or shrank
    #[serde(skip_serializing_if = "Option::is_none", rename = "flexibilityArrow")]
    flexibility_arrow: Option<LineStyle<'a>>,
}

impl<'a> FlexItemHighlightConfig<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> FlexItemHighlightConfigBuilder<'a> {
        FlexItemHighlightConfigBuilder {
            base_size_box: None,
            base_size_border: None,
            flexibility_arrow: None,
        }
    }
    /// Style of the box representing the item's base size
    pub fn base_size_box(&self) -> Option<&BoxStyle> { self.base_size_box.as_ref() }
    /// Style of the border around the box representing the item's base size
    pub fn base_size_border(&self) -> Option<&LineStyle<'a>> { self.base_size_border.as_ref() }
    /// Style of the arrow representing if the item grew or shrank
    pub fn flexibility_arrow(&self) -> Option<&LineStyle<'a>> { self.flexibility_arrow.as_ref() }
}

#[derive(Default)]
pub struct FlexItemHighlightConfigBuilder<'a> {
    base_size_box: Option<BoxStyle>,
    base_size_border: Option<LineStyle<'a>>,
    flexibility_arrow: Option<LineStyle<'a>>,
}

impl<'a> FlexItemHighlightConfigBuilder<'a> {
    /// Style of the box representing the item's base size
    pub fn base_size_box(mut self, base_size_box: BoxStyle) -> Self { self.base_size_box = Some(base_size_box); self }
    /// Style of the border around the box representing the item's base size
    pub fn base_size_border(mut self, base_size_border: LineStyle<'a>) -> Self { self.base_size_border = Some(base_size_border); self }
    /// Style of the arrow representing if the item grew or shrank
    pub fn flexibility_arrow(mut self, flexibility_arrow: LineStyle<'a>) -> Self { self.flexibility_arrow = Some(flexibility_arrow); self }
    pub fn build(self) -> FlexItemHighlightConfig<'a> {
        FlexItemHighlightConfig {
            base_size_box: self.base_size_box,
            base_size_border: self.base_size_border,
            flexibility_arrow: self.flexibility_arrow,
        }
    }
}

/// Style information for drawing a line.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle<'a> {
    /// The color of the line (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<crate::dom::RGBA>,
    /// The line pattern (default: solid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<Cow<'a, str>>,
}

impl<'a> LineStyle<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> LineStyleBuilder<'a> {
        LineStyleBuilder {
            color: None,
            pattern: None,
        }
    }
    /// The color of the line (default: transparent)
    pub fn color(&self) -> Option<&crate::dom::RGBA> { self.color.as_ref() }
    /// The line pattern (default: solid)
    pub fn pattern(&self) -> Option<&str> { self.pattern.as_deref() }
}

#[derive(Default)]
pub struct LineStyleBuilder<'a> {
    color: Option<crate::dom::RGBA>,
    pattern: Option<Cow<'a, str>>,
}

impl<'a> LineStyleBuilder<'a> {
    /// The color of the line (default: transparent)
    pub fn color(mut self, color: crate::dom::RGBA) -> Self { self.color = Some(color); self }
    /// The line pattern (default: solid)
    pub fn pattern(mut self, pattern: impl Into<Cow<'a, str>>) -> Self { self.pattern = Some(pattern.into()); self }
    pub fn build(self) -> LineStyle<'a> {
        LineStyle {
            color: self.color,
            pattern: self.pattern,
        }
    }
}

/// Style information for drawing a box.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoxStyle {
    /// The background color for the box (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillColor")]
    fill_color: Option<crate::dom::RGBA>,
    /// The hatching color for the box (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none", rename = "hatchColor")]
    hatch_color: Option<crate::dom::RGBA>,
}

impl BoxStyle {
    /// Creates a builder for this type.
    pub fn builder() -> BoxStyleBuilder {
        BoxStyleBuilder {
            fill_color: None,
            hatch_color: None,
        }
    }
    /// The background color for the box (default: transparent)
    pub fn fill_color(&self) -> Option<&crate::dom::RGBA> { self.fill_color.as_ref() }
    /// The hatching color for the box (default: transparent)
    pub fn hatch_color(&self) -> Option<&crate::dom::RGBA> { self.hatch_color.as_ref() }
}

#[derive(Default)]
pub struct BoxStyleBuilder {
    fill_color: Option<crate::dom::RGBA>,
    hatch_color: Option<crate::dom::RGBA>,
}

impl BoxStyleBuilder {
    /// The background color for the box (default: transparent)
    pub fn fill_color(mut self, fill_color: crate::dom::RGBA) -> Self { self.fill_color = Some(fill_color); self }
    /// The hatching color for the box (default: transparent)
    pub fn hatch_color(mut self, hatch_color: crate::dom::RGBA) -> Self { self.hatch_color = Some(hatch_color); self }
    pub fn build(self) -> BoxStyle {
        BoxStyle {
            fill_color: self.fill_color,
            hatch_color: self.hatch_color,
        }
    }
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

/// Configuration data for the highlighting of page elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightConfig<'a> {
    /// Whether the node info tooltip should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showInfo")]
    show_info: Option<bool>,
    /// Whether the node styles in the tooltip (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showStyles")]
    show_styles: Option<bool>,
    /// Whether the rulers should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showRulers")]
    show_rulers: Option<bool>,
    /// Whether the a11y info should be shown (default: true).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showAccessibilityInfo")]
    show_accessibility_info: Option<bool>,
    /// Whether the extension lines from node to the rulers should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showExtensionLines")]
    show_extension_lines: Option<bool>,
    /// The content box highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentColor")]
    content_color: Option<crate::dom::RGBA>,
    /// The padding highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "paddingColor")]
    padding_color: Option<crate::dom::RGBA>,
    /// The border highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderColor")]
    border_color: Option<crate::dom::RGBA>,
    /// The margin highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginColor")]
    margin_color: Option<crate::dom::RGBA>,
    /// The event target element highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "eventTargetColor")]
    event_target_color: Option<crate::dom::RGBA>,
    /// The shape outside fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "shapeColor")]
    shape_color: Option<crate::dom::RGBA>,
    /// The shape margin fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "shapeMarginColor")]
    shape_margin_color: Option<crate::dom::RGBA>,
    /// The grid layout color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssGridColor")]
    css_grid_color: Option<crate::dom::RGBA>,
    /// The color format used to format color styles (default: hex).
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorFormat")]
    color_format: Option<ColorFormat>,
    /// The grid layout highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridHighlightConfig")]
    grid_highlight_config: Option<GridHighlightConfig>,
    /// The flex container highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "flexContainerHighlightConfig")]
    flex_container_highlight_config: Option<FlexContainerHighlightConfig<'a>>,
    /// The flex item highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "flexItemHighlightConfig")]
    flex_item_highlight_config: Option<FlexItemHighlightConfig<'a>>,
    /// The contrast algorithm to use for the contrast ratio (default: aa).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contrastAlgorithm")]
    contrast_algorithm: Option<ContrastAlgorithm>,
    /// The container query container highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerQueryContainerHighlightConfig")]
    container_query_container_highlight_config: Option<ContainerQueryContainerHighlightConfig<'a>>,
}

impl<'a> HighlightConfig<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> HighlightConfigBuilder<'a> {
        HighlightConfigBuilder {
            show_info: None,
            show_styles: None,
            show_rulers: None,
            show_accessibility_info: None,
            show_extension_lines: None,
            content_color: None,
            padding_color: None,
            border_color: None,
            margin_color: None,
            event_target_color: None,
            shape_color: None,
            shape_margin_color: None,
            css_grid_color: None,
            color_format: None,
            grid_highlight_config: None,
            flex_container_highlight_config: None,
            flex_item_highlight_config: None,
            contrast_algorithm: None,
            container_query_container_highlight_config: None,
        }
    }
    /// Whether the node info tooltip should be shown (default: false).
    pub fn show_info(&self) -> Option<bool> { self.show_info }
    /// Whether the node styles in the tooltip (default: false).
    pub fn show_styles(&self) -> Option<bool> { self.show_styles }
    /// Whether the rulers should be shown (default: false).
    pub fn show_rulers(&self) -> Option<bool> { self.show_rulers }
    /// Whether the a11y info should be shown (default: true).
    pub fn show_accessibility_info(&self) -> Option<bool> { self.show_accessibility_info }
    /// Whether the extension lines from node to the rulers should be shown (default: false).
    pub fn show_extension_lines(&self) -> Option<bool> { self.show_extension_lines }
    /// The content box highlight fill color (default: transparent).
    pub fn content_color(&self) -> Option<&crate::dom::RGBA> { self.content_color.as_ref() }
    /// The padding highlight fill color (default: transparent).
    pub fn padding_color(&self) -> Option<&crate::dom::RGBA> { self.padding_color.as_ref() }
    /// The border highlight fill color (default: transparent).
    pub fn border_color(&self) -> Option<&crate::dom::RGBA> { self.border_color.as_ref() }
    /// The margin highlight fill color (default: transparent).
    pub fn margin_color(&self) -> Option<&crate::dom::RGBA> { self.margin_color.as_ref() }
    /// The event target element highlight fill color (default: transparent).
    pub fn event_target_color(&self) -> Option<&crate::dom::RGBA> { self.event_target_color.as_ref() }
    /// The shape outside fill color (default: transparent).
    pub fn shape_color(&self) -> Option<&crate::dom::RGBA> { self.shape_color.as_ref() }
    /// The shape margin fill color (default: transparent).
    pub fn shape_margin_color(&self) -> Option<&crate::dom::RGBA> { self.shape_margin_color.as_ref() }
    /// The grid layout color (default: transparent).
    pub fn css_grid_color(&self) -> Option<&crate::dom::RGBA> { self.css_grid_color.as_ref() }
    /// The color format used to format color styles (default: hex).
    pub fn color_format(&self) -> Option<&ColorFormat> { self.color_format.as_ref() }
    /// The grid layout highlight configuration (default: all transparent).
    pub fn grid_highlight_config(&self) -> Option<&GridHighlightConfig> { self.grid_highlight_config.as_ref() }
    /// The flex container highlight configuration (default: all transparent).
    pub fn flex_container_highlight_config(&self) -> Option<&FlexContainerHighlightConfig<'a>> { self.flex_container_highlight_config.as_ref() }
    /// The flex item highlight configuration (default: all transparent).
    pub fn flex_item_highlight_config(&self) -> Option<&FlexItemHighlightConfig<'a>> { self.flex_item_highlight_config.as_ref() }
    /// The contrast algorithm to use for the contrast ratio (default: aa).
    pub fn contrast_algorithm(&self) -> Option<&ContrastAlgorithm> { self.contrast_algorithm.as_ref() }
    /// The container query container highlight configuration (default: all transparent).
    pub fn container_query_container_highlight_config(&self) -> Option<&ContainerQueryContainerHighlightConfig<'a>> { self.container_query_container_highlight_config.as_ref() }
}

#[derive(Default)]
pub struct HighlightConfigBuilder<'a> {
    show_info: Option<bool>,
    show_styles: Option<bool>,
    show_rulers: Option<bool>,
    show_accessibility_info: Option<bool>,
    show_extension_lines: Option<bool>,
    content_color: Option<crate::dom::RGBA>,
    padding_color: Option<crate::dom::RGBA>,
    border_color: Option<crate::dom::RGBA>,
    margin_color: Option<crate::dom::RGBA>,
    event_target_color: Option<crate::dom::RGBA>,
    shape_color: Option<crate::dom::RGBA>,
    shape_margin_color: Option<crate::dom::RGBA>,
    css_grid_color: Option<crate::dom::RGBA>,
    color_format: Option<ColorFormat>,
    grid_highlight_config: Option<GridHighlightConfig>,
    flex_container_highlight_config: Option<FlexContainerHighlightConfig<'a>>,
    flex_item_highlight_config: Option<FlexItemHighlightConfig<'a>>,
    contrast_algorithm: Option<ContrastAlgorithm>,
    container_query_container_highlight_config: Option<ContainerQueryContainerHighlightConfig<'a>>,
}

impl<'a> HighlightConfigBuilder<'a> {
    /// Whether the node info tooltip should be shown (default: false).
    pub fn show_info(mut self, show_info: bool) -> Self { self.show_info = Some(show_info); self }
    /// Whether the node styles in the tooltip (default: false).
    pub fn show_styles(mut self, show_styles: bool) -> Self { self.show_styles = Some(show_styles); self }
    /// Whether the rulers should be shown (default: false).
    pub fn show_rulers(mut self, show_rulers: bool) -> Self { self.show_rulers = Some(show_rulers); self }
    /// Whether the a11y info should be shown (default: true).
    pub fn show_accessibility_info(mut self, show_accessibility_info: bool) -> Self { self.show_accessibility_info = Some(show_accessibility_info); self }
    /// Whether the extension lines from node to the rulers should be shown (default: false).
    pub fn show_extension_lines(mut self, show_extension_lines: bool) -> Self { self.show_extension_lines = Some(show_extension_lines); self }
    /// The content box highlight fill color (default: transparent).
    pub fn content_color(mut self, content_color: crate::dom::RGBA) -> Self { self.content_color = Some(content_color); self }
    /// The padding highlight fill color (default: transparent).
    pub fn padding_color(mut self, padding_color: crate::dom::RGBA) -> Self { self.padding_color = Some(padding_color); self }
    /// The border highlight fill color (default: transparent).
    pub fn border_color(mut self, border_color: crate::dom::RGBA) -> Self { self.border_color = Some(border_color); self }
    /// The margin highlight fill color (default: transparent).
    pub fn margin_color(mut self, margin_color: crate::dom::RGBA) -> Self { self.margin_color = Some(margin_color); self }
    /// The event target element highlight fill color (default: transparent).
    pub fn event_target_color(mut self, event_target_color: crate::dom::RGBA) -> Self { self.event_target_color = Some(event_target_color); self }
    /// The shape outside fill color (default: transparent).
    pub fn shape_color(mut self, shape_color: crate::dom::RGBA) -> Self { self.shape_color = Some(shape_color); self }
    /// The shape margin fill color (default: transparent).
    pub fn shape_margin_color(mut self, shape_margin_color: crate::dom::RGBA) -> Self { self.shape_margin_color = Some(shape_margin_color); self }
    /// The grid layout color (default: transparent).
    pub fn css_grid_color(mut self, css_grid_color: crate::dom::RGBA) -> Self { self.css_grid_color = Some(css_grid_color); self }
    /// The color format used to format color styles (default: hex).
    pub fn color_format(mut self, color_format: impl Into<ColorFormat>) -> Self { self.color_format = Some(color_format.into()); self }
    /// The grid layout highlight configuration (default: all transparent).
    pub fn grid_highlight_config(mut self, grid_highlight_config: GridHighlightConfig) -> Self { self.grid_highlight_config = Some(grid_highlight_config); self }
    /// The flex container highlight configuration (default: all transparent).
    pub fn flex_container_highlight_config(mut self, flex_container_highlight_config: FlexContainerHighlightConfig<'a>) -> Self { self.flex_container_highlight_config = Some(flex_container_highlight_config); self }
    /// The flex item highlight configuration (default: all transparent).
    pub fn flex_item_highlight_config(mut self, flex_item_highlight_config: FlexItemHighlightConfig<'a>) -> Self { self.flex_item_highlight_config = Some(flex_item_highlight_config); self }
    /// The contrast algorithm to use for the contrast ratio (default: aa).
    pub fn contrast_algorithm(mut self, contrast_algorithm: impl Into<ContrastAlgorithm>) -> Self { self.contrast_algorithm = Some(contrast_algorithm.into()); self }
    /// The container query container highlight configuration (default: all transparent).
    pub fn container_query_container_highlight_config(mut self, container_query_container_highlight_config: ContainerQueryContainerHighlightConfig<'a>) -> Self { self.container_query_container_highlight_config = Some(container_query_container_highlight_config); self }
    pub fn build(self) -> HighlightConfig<'a> {
        HighlightConfig {
            show_info: self.show_info,
            show_styles: self.show_styles,
            show_rulers: self.show_rulers,
            show_accessibility_info: self.show_accessibility_info,
            show_extension_lines: self.show_extension_lines,
            content_color: self.content_color,
            padding_color: self.padding_color,
            border_color: self.border_color,
            margin_color: self.margin_color,
            event_target_color: self.event_target_color,
            shape_color: self.shape_color,
            shape_margin_color: self.shape_margin_color,
            css_grid_color: self.css_grid_color,
            color_format: self.color_format,
            grid_highlight_config: self.grid_highlight_config,
            flex_container_highlight_config: self.flex_container_highlight_config,
            flex_item_highlight_config: self.flex_item_highlight_config,
            contrast_algorithm: self.contrast_algorithm,
            container_query_container_highlight_config: self.container_query_container_highlight_config,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GridNodeHighlightConfig {
    /// A descriptor for the highlight appearance.
    #[serde(rename = "gridHighlightConfig")]
    grid_highlight_config: GridHighlightConfig,
    /// Identifier of the node to highlight.
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl GridNodeHighlightConfig {
    /// Creates a builder for this type with the required parameters:
    /// * `grid_highlight_config`: A descriptor for the highlight appearance.
    /// * `node_id`: Identifier of the node to highlight.
    pub fn builder(grid_highlight_config: GridHighlightConfig, node_id: crate::dom::NodeId) -> GridNodeHighlightConfigBuilder {
        GridNodeHighlightConfigBuilder {
            grid_highlight_config: grid_highlight_config,
            node_id: node_id,
        }
    }
    /// A descriptor for the highlight appearance.
    pub fn grid_highlight_config(&self) -> &GridHighlightConfig { &self.grid_highlight_config }
    /// Identifier of the node to highlight.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct GridNodeHighlightConfigBuilder {
    grid_highlight_config: GridHighlightConfig,
    node_id: crate::dom::NodeId,
}

impl GridNodeHighlightConfigBuilder {
    pub fn build(self) -> GridNodeHighlightConfig {
        GridNodeHighlightConfig {
            grid_highlight_config: self.grid_highlight_config,
            node_id: self.node_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexNodeHighlightConfig<'a> {
    /// A descriptor for the highlight appearance of flex containers.
    #[serde(rename = "flexContainerHighlightConfig")]
    flex_container_highlight_config: FlexContainerHighlightConfig<'a>,
    /// Identifier of the node to highlight.
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl<'a> FlexNodeHighlightConfig<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `flex_container_highlight_config`: A descriptor for the highlight appearance of flex containers.
    /// * `node_id`: Identifier of the node to highlight.
    pub fn builder(flex_container_highlight_config: FlexContainerHighlightConfig<'a>, node_id: crate::dom::NodeId) -> FlexNodeHighlightConfigBuilder<'a> {
        FlexNodeHighlightConfigBuilder {
            flex_container_highlight_config: flex_container_highlight_config,
            node_id: node_id,
        }
    }
    /// A descriptor for the highlight appearance of flex containers.
    pub fn flex_container_highlight_config(&self) -> &FlexContainerHighlightConfig<'a> { &self.flex_container_highlight_config }
    /// Identifier of the node to highlight.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct FlexNodeHighlightConfigBuilder<'a> {
    flex_container_highlight_config: FlexContainerHighlightConfig<'a>,
    node_id: crate::dom::NodeId,
}

impl<'a> FlexNodeHighlightConfigBuilder<'a> {
    pub fn build(self) -> FlexNodeHighlightConfig<'a> {
        FlexNodeHighlightConfig {
            flex_container_highlight_config: self.flex_container_highlight_config,
            node_id: self.node_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapContainerHighlightConfig<'a> {
    /// The style of the snapport border (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none", rename = "snapportBorder")]
    snapport_border: Option<LineStyle<'a>>,
    /// The style of the snap area border (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none", rename = "snapAreaBorder")]
    snap_area_border: Option<LineStyle<'a>>,
    /// The margin highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollMarginColor")]
    scroll_margin_color: Option<crate::dom::RGBA>,
    /// The padding highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollPaddingColor")]
    scroll_padding_color: Option<crate::dom::RGBA>,
}

impl<'a> ScrollSnapContainerHighlightConfig<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> ScrollSnapContainerHighlightConfigBuilder<'a> {
        ScrollSnapContainerHighlightConfigBuilder {
            snapport_border: None,
            snap_area_border: None,
            scroll_margin_color: None,
            scroll_padding_color: None,
        }
    }
    /// The style of the snapport border (default: transparent)
    pub fn snapport_border(&self) -> Option<&LineStyle<'a>> { self.snapport_border.as_ref() }
    /// The style of the snap area border (default: transparent)
    pub fn snap_area_border(&self) -> Option<&LineStyle<'a>> { self.snap_area_border.as_ref() }
    /// The margin highlight fill color (default: transparent).
    pub fn scroll_margin_color(&self) -> Option<&crate::dom::RGBA> { self.scroll_margin_color.as_ref() }
    /// The padding highlight fill color (default: transparent).
    pub fn scroll_padding_color(&self) -> Option<&crate::dom::RGBA> { self.scroll_padding_color.as_ref() }
}

#[derive(Default)]
pub struct ScrollSnapContainerHighlightConfigBuilder<'a> {
    snapport_border: Option<LineStyle<'a>>,
    snap_area_border: Option<LineStyle<'a>>,
    scroll_margin_color: Option<crate::dom::RGBA>,
    scroll_padding_color: Option<crate::dom::RGBA>,
}

impl<'a> ScrollSnapContainerHighlightConfigBuilder<'a> {
    /// The style of the snapport border (default: transparent)
    pub fn snapport_border(mut self, snapport_border: LineStyle<'a>) -> Self { self.snapport_border = Some(snapport_border); self }
    /// The style of the snap area border (default: transparent)
    pub fn snap_area_border(mut self, snap_area_border: LineStyle<'a>) -> Self { self.snap_area_border = Some(snap_area_border); self }
    /// The margin highlight fill color (default: transparent).
    pub fn scroll_margin_color(mut self, scroll_margin_color: crate::dom::RGBA) -> Self { self.scroll_margin_color = Some(scroll_margin_color); self }
    /// The padding highlight fill color (default: transparent).
    pub fn scroll_padding_color(mut self, scroll_padding_color: crate::dom::RGBA) -> Self { self.scroll_padding_color = Some(scroll_padding_color); self }
    pub fn build(self) -> ScrollSnapContainerHighlightConfig<'a> {
        ScrollSnapContainerHighlightConfig {
            snapport_border: self.snapport_border,
            snap_area_border: self.snap_area_border,
            scroll_margin_color: self.scroll_margin_color,
            scroll_padding_color: self.scroll_padding_color,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapHighlightConfig<'a> {
    /// A descriptor for the highlight appearance of scroll snap containers.
    #[serde(rename = "scrollSnapContainerHighlightConfig")]
    scroll_snap_container_highlight_config: ScrollSnapContainerHighlightConfig<'a>,
    /// Identifier of the node to highlight.
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl<'a> ScrollSnapHighlightConfig<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `scroll_snap_container_highlight_config`: A descriptor for the highlight appearance of scroll snap containers.
    /// * `node_id`: Identifier of the node to highlight.
    pub fn builder(scroll_snap_container_highlight_config: ScrollSnapContainerHighlightConfig<'a>, node_id: crate::dom::NodeId) -> ScrollSnapHighlightConfigBuilder<'a> {
        ScrollSnapHighlightConfigBuilder {
            scroll_snap_container_highlight_config: scroll_snap_container_highlight_config,
            node_id: node_id,
        }
    }
    /// A descriptor for the highlight appearance of scroll snap containers.
    pub fn scroll_snap_container_highlight_config(&self) -> &ScrollSnapContainerHighlightConfig<'a> { &self.scroll_snap_container_highlight_config }
    /// Identifier of the node to highlight.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct ScrollSnapHighlightConfigBuilder<'a> {
    scroll_snap_container_highlight_config: ScrollSnapContainerHighlightConfig<'a>,
    node_id: crate::dom::NodeId,
}

impl<'a> ScrollSnapHighlightConfigBuilder<'a> {
    pub fn build(self) -> ScrollSnapHighlightConfig<'a> {
        ScrollSnapHighlightConfig {
            scroll_snap_container_highlight_config: self.scroll_snap_container_highlight_config,
            node_id: self.node_id,
        }
    }
}

/// Configuration for dual screen hinge

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HingeConfig {
    /// A rectangle represent hinge
    rect: crate::dom::Rect,
    /// The content box highlight fill color (default: a dark color).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentColor")]
    content_color: Option<crate::dom::RGBA>,
    /// The content box highlight outline color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "outlineColor")]
    outline_color: Option<crate::dom::RGBA>,
}

impl HingeConfig {
    /// Creates a builder for this type with the required parameters:
    /// * `rect`: A rectangle represent hinge
    pub fn builder(rect: crate::dom::Rect) -> HingeConfigBuilder {
        HingeConfigBuilder {
            rect: rect,
            content_color: None,
            outline_color: None,
        }
    }
    /// A rectangle represent hinge
    pub fn rect(&self) -> &crate::dom::Rect { &self.rect }
    /// The content box highlight fill color (default: a dark color).
    pub fn content_color(&self) -> Option<&crate::dom::RGBA> { self.content_color.as_ref() }
    /// The content box highlight outline color (default: transparent).
    pub fn outline_color(&self) -> Option<&crate::dom::RGBA> { self.outline_color.as_ref() }
}


pub struct HingeConfigBuilder {
    rect: crate::dom::Rect,
    content_color: Option<crate::dom::RGBA>,
    outline_color: Option<crate::dom::RGBA>,
}

impl HingeConfigBuilder {
    /// The content box highlight fill color (default: a dark color).
    pub fn content_color(mut self, content_color: crate::dom::RGBA) -> Self { self.content_color = Some(content_color); self }
    /// The content box highlight outline color (default: transparent).
    pub fn outline_color(mut self, outline_color: crate::dom::RGBA) -> Self { self.outline_color = Some(outline_color); self }
    pub fn build(self) -> HingeConfig {
        HingeConfig {
            rect: self.rect,
            content_color: self.content_color,
            outline_color: self.outline_color,
        }
    }
}

/// Configuration for Window Controls Overlay

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowControlsOverlayConfig<'a> {
    /// Whether the title bar CSS should be shown when emulating the Window Controls Overlay.
    #[serde(rename = "showCSS")]
    show_css: bool,
    /// Selected platforms to show the overlay.
    #[serde(rename = "selectedPlatform")]
    selected_platform: Cow<'a, str>,
    /// The theme color defined in app manifest.
    #[serde(rename = "themeColor")]
    theme_color: Cow<'a, str>,
}

impl<'a> WindowControlsOverlayConfig<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `show_css`: Whether the title bar CSS should be shown when emulating the Window Controls Overlay.
    /// * `selected_platform`: Selected platforms to show the overlay.
    /// * `theme_color`: The theme color defined in app manifest.
    pub fn builder(show_css: bool, selected_platform: impl Into<Cow<'a, str>>, theme_color: impl Into<Cow<'a, str>>) -> WindowControlsOverlayConfigBuilder<'a> {
        WindowControlsOverlayConfigBuilder {
            show_css: show_css,
            selected_platform: selected_platform.into(),
            theme_color: theme_color.into(),
        }
    }
    /// Whether the title bar CSS should be shown when emulating the Window Controls Overlay.
    pub fn show_css(&self) -> bool { self.show_css }
    /// Selected platforms to show the overlay.
    pub fn selected_platform(&self) -> &str { self.selected_platform.as_ref() }
    /// The theme color defined in app manifest.
    pub fn theme_color(&self) -> &str { self.theme_color.as_ref() }
}


pub struct WindowControlsOverlayConfigBuilder<'a> {
    show_css: bool,
    selected_platform: Cow<'a, str>,
    theme_color: Cow<'a, str>,
}

impl<'a> WindowControlsOverlayConfigBuilder<'a> {
    pub fn build(self) -> WindowControlsOverlayConfig<'a> {
        WindowControlsOverlayConfig {
            show_css: self.show_css,
            selected_platform: self.selected_platform,
            theme_color: self.theme_color,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryHighlightConfig<'a> {
    /// A descriptor for the highlight appearance of container query containers.
    #[serde(rename = "containerQueryContainerHighlightConfig")]
    container_query_container_highlight_config: ContainerQueryContainerHighlightConfig<'a>,
    /// Identifier of the container node to highlight.
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl<'a> ContainerQueryHighlightConfig<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `container_query_container_highlight_config`: A descriptor for the highlight appearance of container query containers.
    /// * `node_id`: Identifier of the container node to highlight.
    pub fn builder(container_query_container_highlight_config: ContainerQueryContainerHighlightConfig<'a>, node_id: crate::dom::NodeId) -> ContainerQueryHighlightConfigBuilder<'a> {
        ContainerQueryHighlightConfigBuilder {
            container_query_container_highlight_config: container_query_container_highlight_config,
            node_id: node_id,
        }
    }
    /// A descriptor for the highlight appearance of container query containers.
    pub fn container_query_container_highlight_config(&self) -> &ContainerQueryContainerHighlightConfig<'a> { &self.container_query_container_highlight_config }
    /// Identifier of the container node to highlight.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct ContainerQueryHighlightConfigBuilder<'a> {
    container_query_container_highlight_config: ContainerQueryContainerHighlightConfig<'a>,
    node_id: crate::dom::NodeId,
}

impl<'a> ContainerQueryHighlightConfigBuilder<'a> {
    pub fn build(self) -> ContainerQueryHighlightConfig<'a> {
        ContainerQueryHighlightConfig {
            container_query_container_highlight_config: self.container_query_container_highlight_config,
            node_id: self.node_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryContainerHighlightConfig<'a> {
    /// The style of the container border.
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerBorder")]
    container_border: Option<LineStyle<'a>>,
    /// The style of the descendants' borders.
    #[serde(skip_serializing_if = "Option::is_none", rename = "descendantBorder")]
    descendant_border: Option<LineStyle<'a>>,
}

impl<'a> ContainerQueryContainerHighlightConfig<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> ContainerQueryContainerHighlightConfigBuilder<'a> {
        ContainerQueryContainerHighlightConfigBuilder {
            container_border: None,
            descendant_border: None,
        }
    }
    /// The style of the container border.
    pub fn container_border(&self) -> Option<&LineStyle<'a>> { self.container_border.as_ref() }
    /// The style of the descendants' borders.
    pub fn descendant_border(&self) -> Option<&LineStyle<'a>> { self.descendant_border.as_ref() }
}

#[derive(Default)]
pub struct ContainerQueryContainerHighlightConfigBuilder<'a> {
    container_border: Option<LineStyle<'a>>,
    descendant_border: Option<LineStyle<'a>>,
}

impl<'a> ContainerQueryContainerHighlightConfigBuilder<'a> {
    /// The style of the container border.
    pub fn container_border(mut self, container_border: LineStyle<'a>) -> Self { self.container_border = Some(container_border); self }
    /// The style of the descendants' borders.
    pub fn descendant_border(mut self, descendant_border: LineStyle<'a>) -> Self { self.descendant_border = Some(descendant_border); self }
    pub fn build(self) -> ContainerQueryContainerHighlightConfig<'a> {
        ContainerQueryContainerHighlightConfig {
            container_border: self.container_border,
            descendant_border: self.descendant_border,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedElementHighlightConfig {
    /// A descriptor for the highlight appearance of an element in isolation mode.
    #[serde(rename = "isolationModeHighlightConfig")]
    isolation_mode_highlight_config: IsolationModeHighlightConfig,
    /// Identifier of the isolated element to highlight.
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl IsolatedElementHighlightConfig {
    /// Creates a builder for this type with the required parameters:
    /// * `isolation_mode_highlight_config`: A descriptor for the highlight appearance of an element in isolation mode.
    /// * `node_id`: Identifier of the isolated element to highlight.
    pub fn builder(isolation_mode_highlight_config: IsolationModeHighlightConfig, node_id: crate::dom::NodeId) -> IsolatedElementHighlightConfigBuilder {
        IsolatedElementHighlightConfigBuilder {
            isolation_mode_highlight_config: isolation_mode_highlight_config,
            node_id: node_id,
        }
    }
    /// A descriptor for the highlight appearance of an element in isolation mode.
    pub fn isolation_mode_highlight_config(&self) -> &IsolationModeHighlightConfig { &self.isolation_mode_highlight_config }
    /// Identifier of the isolated element to highlight.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct IsolatedElementHighlightConfigBuilder {
    isolation_mode_highlight_config: IsolationModeHighlightConfig,
    node_id: crate::dom::NodeId,
}

impl IsolatedElementHighlightConfigBuilder {
    pub fn build(self) -> IsolatedElementHighlightConfig {
        IsolatedElementHighlightConfig {
            isolation_mode_highlight_config: self.isolation_mode_highlight_config,
            node_id: self.node_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IsolationModeHighlightConfig {
    /// The fill color of the resizers (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resizerColor")]
    resizer_color: Option<crate::dom::RGBA>,
    /// The fill color for resizer handles (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resizerHandleColor")]
    resizer_handle_color: Option<crate::dom::RGBA>,
    /// The fill color for the mask covering non-isolated elements (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "maskColor")]
    mask_color: Option<crate::dom::RGBA>,
}

impl IsolationModeHighlightConfig {
    /// Creates a builder for this type.
    pub fn builder() -> IsolationModeHighlightConfigBuilder {
        IsolationModeHighlightConfigBuilder {
            resizer_color: None,
            resizer_handle_color: None,
            mask_color: None,
        }
    }
    /// The fill color of the resizers (default: transparent).
    pub fn resizer_color(&self) -> Option<&crate::dom::RGBA> { self.resizer_color.as_ref() }
    /// The fill color for resizer handles (default: transparent).
    pub fn resizer_handle_color(&self) -> Option<&crate::dom::RGBA> { self.resizer_handle_color.as_ref() }
    /// The fill color for the mask covering non-isolated elements (default: transparent).
    pub fn mask_color(&self) -> Option<&crate::dom::RGBA> { self.mask_color.as_ref() }
}

#[derive(Default)]
pub struct IsolationModeHighlightConfigBuilder {
    resizer_color: Option<crate::dom::RGBA>,
    resizer_handle_color: Option<crate::dom::RGBA>,
    mask_color: Option<crate::dom::RGBA>,
}

impl IsolationModeHighlightConfigBuilder {
    /// The fill color of the resizers (default: transparent).
    pub fn resizer_color(mut self, resizer_color: crate::dom::RGBA) -> Self { self.resizer_color = Some(resizer_color); self }
    /// The fill color for resizer handles (default: transparent).
    pub fn resizer_handle_color(mut self, resizer_handle_color: crate::dom::RGBA) -> Self { self.resizer_handle_color = Some(resizer_handle_color); self }
    /// The fill color for the mask covering non-isolated elements (default: transparent).
    pub fn mask_color(mut self, mask_color: crate::dom::RGBA) -> Self { self.mask_color = Some(mask_color); self }
    pub fn build(self) -> IsolationModeHighlightConfig {
        IsolationModeHighlightConfig {
            resizer_color: self.resizer_color,
            resizer_handle_color: self.resizer_handle_color,
            mask_color: self.mask_color,
        }
    }
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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectedElementAnchorConfig {
    /// Identifier of the node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
}

impl InspectedElementAnchorConfig {
    /// Creates a builder for this type.
    pub fn builder() -> InspectedElementAnchorConfigBuilder {
        InspectedElementAnchorConfigBuilder {
            node_id: None,
            backend_node_id: None,
        }
    }
    /// Identifier of the node to highlight.
    pub fn node_id(&self) -> Option<&crate::dom::NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node to highlight.
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
}

#[derive(Default)]
pub struct InspectedElementAnchorConfigBuilder {
    node_id: Option<crate::dom::NodeId>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
}

impl InspectedElementAnchorConfigBuilder {
    /// Identifier of the node to highlight.
    pub fn node_id(mut self, node_id: crate::dom::NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node to highlight.
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    pub fn build(self) -> InspectedElementAnchorConfig {
        InspectedElementAnchorConfig {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Overlay.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Overlay.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Overlay.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Overlay.enable";
    type Response = crate::EmptyReturns;
}

/// For testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHighlightObjectForTestParams {
    /// Id of the node to get highlight object for.
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
    /// Whether to include distance info.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeDistance")]
    include_distance: Option<bool>,
    /// Whether to include style info.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeStyle")]
    include_style: Option<bool>,
    /// The color format to get config with (default: hex).
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorFormat")]
    color_format: Option<ColorFormat>,
    /// Whether to show accessibility info (default: true).
    #[serde(skip_serializing_if = "Option::is_none", rename = "showAccessibilityInfo")]
    show_accessibility_info: Option<bool>,
}

impl GetHighlightObjectForTestParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to get highlight object for.
    pub fn builder(node_id: crate::dom::NodeId) -> GetHighlightObjectForTestParamsBuilder {
        GetHighlightObjectForTestParamsBuilder {
            node_id: node_id,
            include_distance: None,
            include_style: None,
            color_format: None,
            show_accessibility_info: None,
        }
    }
    /// Id of the node to get highlight object for.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
    /// Whether to include distance info.
    pub fn include_distance(&self) -> Option<bool> { self.include_distance }
    /// Whether to include style info.
    pub fn include_style(&self) -> Option<bool> { self.include_style }
    /// The color format to get config with (default: hex).
    pub fn color_format(&self) -> Option<&ColorFormat> { self.color_format.as_ref() }
    /// Whether to show accessibility info (default: true).
    pub fn show_accessibility_info(&self) -> Option<bool> { self.show_accessibility_info }
}


pub struct GetHighlightObjectForTestParamsBuilder {
    node_id: crate::dom::NodeId,
    include_distance: Option<bool>,
    include_style: Option<bool>,
    color_format: Option<ColorFormat>,
    show_accessibility_info: Option<bool>,
}

impl GetHighlightObjectForTestParamsBuilder {
    /// Whether to include distance info.
    pub fn include_distance(mut self, include_distance: bool) -> Self { self.include_distance = Some(include_distance); self }
    /// Whether to include style info.
    pub fn include_style(mut self, include_style: bool) -> Self { self.include_style = Some(include_style); self }
    /// The color format to get config with (default: hex).
    pub fn color_format(mut self, color_format: impl Into<ColorFormat>) -> Self { self.color_format = Some(color_format.into()); self }
    /// Whether to show accessibility info (default: true).
    pub fn show_accessibility_info(mut self, show_accessibility_info: bool) -> Self { self.show_accessibility_info = Some(show_accessibility_info); self }
    pub fn build(self) -> GetHighlightObjectForTestParams {
        GetHighlightObjectForTestParams {
            node_id: self.node_id,
            include_distance: self.include_distance,
            include_style: self.include_style,
            color_format: self.color_format,
            show_accessibility_info: self.show_accessibility_info,
        }
    }
}

/// For testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHighlightObjectForTestReturns {
    /// Highlight data for the node.
    highlight: serde_json::Map<String, JsonValue>,
}

impl GetHighlightObjectForTestReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `highlight`: Highlight data for the node.
    pub fn builder(highlight: serde_json::Map<String, JsonValue>) -> GetHighlightObjectForTestReturnsBuilder {
        GetHighlightObjectForTestReturnsBuilder {
            highlight: highlight,
        }
    }
    /// Highlight data for the node.
    pub fn highlight(&self) -> &serde_json::Map<String, JsonValue> { &self.highlight }
}


pub struct GetHighlightObjectForTestReturnsBuilder {
    highlight: serde_json::Map<String, JsonValue>,
}

impl GetHighlightObjectForTestReturnsBuilder {
    pub fn build(self) -> GetHighlightObjectForTestReturns {
        GetHighlightObjectForTestReturns {
            highlight: self.highlight,
        }
    }
}

impl GetHighlightObjectForTestParams { pub const METHOD: &'static str = "Overlay.getHighlightObjectForTest"; }

impl<'a> crate::CdpCommand<'a> for GetHighlightObjectForTestParams {
    const METHOD: &'static str = "Overlay.getHighlightObjectForTest";
    type Response = GetHighlightObjectForTestReturns;
}

/// For Persistent Grid testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGridHighlightObjectsForTestParams {
    /// Ids of the node to get highlight object for.
    #[serde(rename = "nodeIds")]
    node_ids: Vec<crate::dom::NodeId>,
}

impl GetGridHighlightObjectsForTestParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_ids`: Ids of the node to get highlight object for.
    pub fn builder(node_ids: Vec<crate::dom::NodeId>) -> GetGridHighlightObjectsForTestParamsBuilder {
        GetGridHighlightObjectsForTestParamsBuilder {
            node_ids: node_ids,
        }
    }
    /// Ids of the node to get highlight object for.
    pub fn node_ids(&self) -> &[crate::dom::NodeId] { &self.node_ids }
}


pub struct GetGridHighlightObjectsForTestParamsBuilder {
    node_ids: Vec<crate::dom::NodeId>,
}

impl GetGridHighlightObjectsForTestParamsBuilder {
    pub fn build(self) -> GetGridHighlightObjectsForTestParams {
        GetGridHighlightObjectsForTestParams {
            node_ids: self.node_ids,
        }
    }
}

/// For Persistent Grid testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetGridHighlightObjectsForTestReturns {
    /// Grid Highlight data for the node ids provided.
    highlights: serde_json::Map<String, JsonValue>,
}

impl GetGridHighlightObjectsForTestReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `highlights`: Grid Highlight data for the node ids provided.
    pub fn builder(highlights: serde_json::Map<String, JsonValue>) -> GetGridHighlightObjectsForTestReturnsBuilder {
        GetGridHighlightObjectsForTestReturnsBuilder {
            highlights: highlights,
        }
    }
    /// Grid Highlight data for the node ids provided.
    pub fn highlights(&self) -> &serde_json::Map<String, JsonValue> { &self.highlights }
}


pub struct GetGridHighlightObjectsForTestReturnsBuilder {
    highlights: serde_json::Map<String, JsonValue>,
}

impl GetGridHighlightObjectsForTestReturnsBuilder {
    pub fn build(self) -> GetGridHighlightObjectsForTestReturns {
        GetGridHighlightObjectsForTestReturns {
            highlights: self.highlights,
        }
    }
}

impl GetGridHighlightObjectsForTestParams { pub const METHOD: &'static str = "Overlay.getGridHighlightObjectsForTest"; }

impl<'a> crate::CdpCommand<'a> for GetGridHighlightObjectsForTestParams {
    const METHOD: &'static str = "Overlay.getGridHighlightObjectsForTest";
    type Response = GetGridHighlightObjectsForTestReturns;
}

/// For Source Order Viewer testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceOrderHighlightObjectForTestParams {
    /// Id of the node to highlight.
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl GetSourceOrderHighlightObjectForTestParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to highlight.
    pub fn builder(node_id: crate::dom::NodeId) -> GetSourceOrderHighlightObjectForTestParamsBuilder {
        GetSourceOrderHighlightObjectForTestParamsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the node to highlight.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct GetSourceOrderHighlightObjectForTestParamsBuilder {
    node_id: crate::dom::NodeId,
}

impl GetSourceOrderHighlightObjectForTestParamsBuilder {
    pub fn build(self) -> GetSourceOrderHighlightObjectForTestParams {
        GetSourceOrderHighlightObjectForTestParams {
            node_id: self.node_id,
        }
    }
}

/// For Source Order Viewer testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceOrderHighlightObjectForTestReturns {
    /// Source order highlight data for the node id provided.
    highlight: serde_json::Map<String, JsonValue>,
}

impl GetSourceOrderHighlightObjectForTestReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `highlight`: Source order highlight data for the node id provided.
    pub fn builder(highlight: serde_json::Map<String, JsonValue>) -> GetSourceOrderHighlightObjectForTestReturnsBuilder {
        GetSourceOrderHighlightObjectForTestReturnsBuilder {
            highlight: highlight,
        }
    }
    /// Source order highlight data for the node id provided.
    pub fn highlight(&self) -> &serde_json::Map<String, JsonValue> { &self.highlight }
}


pub struct GetSourceOrderHighlightObjectForTestReturnsBuilder {
    highlight: serde_json::Map<String, JsonValue>,
}

impl GetSourceOrderHighlightObjectForTestReturnsBuilder {
    pub fn build(self) -> GetSourceOrderHighlightObjectForTestReturns {
        GetSourceOrderHighlightObjectForTestReturns {
            highlight: self.highlight,
        }
    }
}

impl GetSourceOrderHighlightObjectForTestParams { pub const METHOD: &'static str = "Overlay.getSourceOrderHighlightObjectForTest"; }

impl<'a> crate::CdpCommand<'a> for GetSourceOrderHighlightObjectForTestParams {
    const METHOD: &'static str = "Overlay.getSourceOrderHighlightObjectForTest";
    type Response = GetSourceOrderHighlightObjectForTestReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HideHighlightParams {}

impl HideHighlightParams { pub const METHOD: &'static str = "Overlay.hideHighlight"; }

impl<'a> crate::CdpCommand<'a> for HideHighlightParams {
    const METHOD: &'static str = "Overlay.hideHighlight";
    type Response = crate::EmptyReturns;
}

/// Highlights owner element of the frame with given id.
/// Deprecated: Doesn't work reliably and cannot be fixed due to process
/// separation (the owner node might be in a different process). Determine
/// the owner node in the client and use highlightNode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightFrameParams<'a> {
    /// Identifier of the frame to highlight.
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
    /// The content box highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentColor")]
    content_color: Option<crate::dom::RGBA>,
    /// The content box highlight outline color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentOutlineColor")]
    content_outline_color: Option<crate::dom::RGBA>,
}

impl<'a> HighlightFrameParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: Identifier of the frame to highlight.
    pub fn builder(frame_id: crate::page::FrameId<'a>) -> HighlightFrameParamsBuilder<'a> {
        HighlightFrameParamsBuilder {
            frame_id: frame_id,
            content_color: None,
            content_outline_color: None,
        }
    }
    /// Identifier of the frame to highlight.
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
    /// The content box highlight fill color (default: transparent).
    pub fn content_color(&self) -> Option<&crate::dom::RGBA> { self.content_color.as_ref() }
    /// The content box highlight outline color (default: transparent).
    pub fn content_outline_color(&self) -> Option<&crate::dom::RGBA> { self.content_outline_color.as_ref() }
}


pub struct HighlightFrameParamsBuilder<'a> {
    frame_id: crate::page::FrameId<'a>,
    content_color: Option<crate::dom::RGBA>,
    content_outline_color: Option<crate::dom::RGBA>,
}

impl<'a> HighlightFrameParamsBuilder<'a> {
    /// The content box highlight fill color (default: transparent).
    pub fn content_color(mut self, content_color: crate::dom::RGBA) -> Self { self.content_color = Some(content_color); self }
    /// The content box highlight outline color (default: transparent).
    pub fn content_outline_color(mut self, content_outline_color: crate::dom::RGBA) -> Self { self.content_outline_color = Some(content_outline_color); self }
    pub fn build(self) -> HighlightFrameParams<'a> {
        HighlightFrameParams {
            frame_id: self.frame_id,
            content_color: self.content_color,
            content_outline_color: self.content_outline_color,
        }
    }
}

impl<'a> HighlightFrameParams<'a> { pub const METHOD: &'static str = "Overlay.highlightFrame"; }

impl<'a> crate::CdpCommand<'a> for HighlightFrameParams<'a> {
    const METHOD: &'static str = "Overlay.highlightFrame";
    type Response = crate::EmptyReturns;
}

/// Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or
/// objectId must be specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightNodeParams<'a> {
    /// A descriptor for the highlight appearance.
    #[serde(rename = "highlightConfig")]
    highlight_config: HighlightConfig<'a>,
    /// Identifier of the node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node to be highlighted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Selectors to highlight relevant nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    selector: Option<Cow<'a, str>>,
}

impl<'a> HighlightNodeParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `highlight_config`: A descriptor for the highlight appearance.
    pub fn builder(highlight_config: HighlightConfig<'a>) -> HighlightNodeParamsBuilder<'a> {
        HighlightNodeParamsBuilder {
            highlight_config: highlight_config,
            node_id: None,
            backend_node_id: None,
            object_id: None,
            selector: None,
        }
    }
    /// A descriptor for the highlight appearance.
    pub fn highlight_config(&self) -> &HighlightConfig<'a> { &self.highlight_config }
    /// Identifier of the node to highlight.
    pub fn node_id(&self) -> Option<&crate::dom::NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node to highlight.
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node to be highlighted.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
    /// Selectors to highlight relevant nodes.
    pub fn selector(&self) -> Option<&str> { self.selector.as_deref() }
}


pub struct HighlightNodeParamsBuilder<'a> {
    highlight_config: HighlightConfig<'a>,
    node_id: Option<crate::dom::NodeId>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    selector: Option<Cow<'a, str>>,
}

impl<'a> HighlightNodeParamsBuilder<'a> {
    /// Identifier of the node to highlight.
    pub fn node_id(mut self, node_id: crate::dom::NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node to highlight.
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node to be highlighted.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    /// Selectors to highlight relevant nodes.
    pub fn selector(mut self, selector: impl Into<Cow<'a, str>>) -> Self { self.selector = Some(selector.into()); self }
    pub fn build(self) -> HighlightNodeParams<'a> {
        HighlightNodeParams {
            highlight_config: self.highlight_config,
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
            selector: self.selector,
        }
    }
}

impl<'a> HighlightNodeParams<'a> { pub const METHOD: &'static str = "Overlay.highlightNode"; }

impl<'a> crate::CdpCommand<'a> for HighlightNodeParams<'a> {
    const METHOD: &'static str = "Overlay.highlightNode";
    type Response = crate::EmptyReturns;
}

/// Highlights given quad. Coordinates are absolute with respect to the main frame viewport.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightQuadParams {
    /// Quad to highlight
    quad: crate::dom::Quad,
    /// The highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<crate::dom::RGBA>,
    /// The highlight outline color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "outlineColor")]
    outline_color: Option<crate::dom::RGBA>,
}

impl HighlightQuadParams {
    /// Creates a builder for this type with the required parameters:
    /// * `quad`: Quad to highlight
    pub fn builder(quad: crate::dom::Quad) -> HighlightQuadParamsBuilder {
        HighlightQuadParamsBuilder {
            quad: quad,
            color: None,
            outline_color: None,
        }
    }
    /// Quad to highlight
    pub fn quad(&self) -> &crate::dom::Quad { &self.quad }
    /// The highlight fill color (default: transparent).
    pub fn color(&self) -> Option<&crate::dom::RGBA> { self.color.as_ref() }
    /// The highlight outline color (default: transparent).
    pub fn outline_color(&self) -> Option<&crate::dom::RGBA> { self.outline_color.as_ref() }
}


pub struct HighlightQuadParamsBuilder {
    quad: crate::dom::Quad,
    color: Option<crate::dom::RGBA>,
    outline_color: Option<crate::dom::RGBA>,
}

impl HighlightQuadParamsBuilder {
    /// The highlight fill color (default: transparent).
    pub fn color(mut self, color: crate::dom::RGBA) -> Self { self.color = Some(color); self }
    /// The highlight outline color (default: transparent).
    pub fn outline_color(mut self, outline_color: crate::dom::RGBA) -> Self { self.outline_color = Some(outline_color); self }
    pub fn build(self) -> HighlightQuadParams {
        HighlightQuadParams {
            quad: self.quad,
            color: self.color,
            outline_color: self.outline_color,
        }
    }
}

impl HighlightQuadParams { pub const METHOD: &'static str = "Overlay.highlightQuad"; }

impl<'a> crate::CdpCommand<'a> for HighlightQuadParams {
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
    x: i32,
    /// Y coordinate
    y: i32,
    /// Rectangle width
    width: u64,
    /// Rectangle height
    height: i64,
    /// The highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<crate::dom::RGBA>,
    /// The highlight outline color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "outlineColor")]
    outline_color: Option<crate::dom::RGBA>,
}

impl HighlightRectParams {
    /// Creates a builder for this type with the required parameters:
    /// * `x`: X coordinate
    /// * `y`: Y coordinate
    /// * `width`: Rectangle width
    /// * `height`: Rectangle height
    pub fn builder(x: i32, y: i32, width: u64, height: i64) -> HighlightRectParamsBuilder {
        HighlightRectParamsBuilder {
            x: x,
            y: y,
            width: width,
            height: height,
            color: None,
            outline_color: None,
        }
    }
    /// X coordinate
    pub fn x(&self) -> i32 { self.x }
    /// Y coordinate
    pub fn y(&self) -> i32 { self.y }
    /// Rectangle width
    pub fn width(&self) -> u64 { self.width }
    /// Rectangle height
    pub fn height(&self) -> i64 { self.height }
    /// The highlight fill color (default: transparent).
    pub fn color(&self) -> Option<&crate::dom::RGBA> { self.color.as_ref() }
    /// The highlight outline color (default: transparent).
    pub fn outline_color(&self) -> Option<&crate::dom::RGBA> { self.outline_color.as_ref() }
}


pub struct HighlightRectParamsBuilder {
    x: i32,
    y: i32,
    width: u64,
    height: i64,
    color: Option<crate::dom::RGBA>,
    outline_color: Option<crate::dom::RGBA>,
}

impl HighlightRectParamsBuilder {
    /// The highlight fill color (default: transparent).
    pub fn color(mut self, color: crate::dom::RGBA) -> Self { self.color = Some(color); self }
    /// The highlight outline color (default: transparent).
    pub fn outline_color(mut self, outline_color: crate::dom::RGBA) -> Self { self.outline_color = Some(outline_color); self }
    pub fn build(self) -> HighlightRectParams {
        HighlightRectParams {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: self.color,
            outline_color: self.outline_color,
        }
    }
}

impl HighlightRectParams { pub const METHOD: &'static str = "Overlay.highlightRect"; }

impl<'a> crate::CdpCommand<'a> for HighlightRectParams {
    const METHOD: &'static str = "Overlay.highlightRect";
    type Response = crate::EmptyReturns;
}

/// Highlights the source order of the children of the DOM node with given id or with the given
/// JavaScript object wrapper. Either nodeId or objectId must be specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HighlightSourceOrderParams<'a> {
    /// A descriptor for the appearance of the overlay drawing.
    #[serde(rename = "sourceOrderConfig")]
    source_order_config: SourceOrderConfig,
    /// Identifier of the node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node to be highlighted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> HighlightSourceOrderParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `source_order_config`: A descriptor for the appearance of the overlay drawing.
    pub fn builder(source_order_config: SourceOrderConfig) -> HighlightSourceOrderParamsBuilder<'a> {
        HighlightSourceOrderParamsBuilder {
            source_order_config: source_order_config,
            node_id: None,
            backend_node_id: None,
            object_id: None,
        }
    }
    /// A descriptor for the appearance of the overlay drawing.
    pub fn source_order_config(&self) -> &SourceOrderConfig { &self.source_order_config }
    /// Identifier of the node to highlight.
    pub fn node_id(&self) -> Option<&crate::dom::NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node to highlight.
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node to be highlighted.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
}


pub struct HighlightSourceOrderParamsBuilder<'a> {
    source_order_config: SourceOrderConfig,
    node_id: Option<crate::dom::NodeId>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> HighlightSourceOrderParamsBuilder<'a> {
    /// Identifier of the node to highlight.
    pub fn node_id(mut self, node_id: crate::dom::NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node to highlight.
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node to be highlighted.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    pub fn build(self) -> HighlightSourceOrderParams<'a> {
        HighlightSourceOrderParams {
            source_order_config: self.source_order_config,
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
        }
    }
}

impl<'a> HighlightSourceOrderParams<'a> { pub const METHOD: &'static str = "Overlay.highlightSourceOrder"; }

impl<'a> crate::CdpCommand<'a> for HighlightSourceOrderParams<'a> {
    const METHOD: &'static str = "Overlay.highlightSourceOrder";
    type Response = crate::EmptyReturns;
}

/// Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.
/// Backend then generates 'inspectNodeRequested' event upon element selection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInspectModeParams<'a> {
    /// Set an inspection mode.
    mode: InspectMode,
    /// A descriptor for the highlight appearance of hovered-over nodes. May be omitted if 'enabled
    /// == false'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "highlightConfig")]
    highlight_config: Option<HighlightConfig<'a>>,
}

impl<'a> SetInspectModeParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `mode`: Set an inspection mode.
    pub fn builder(mode: impl Into<InspectMode>) -> SetInspectModeParamsBuilder<'a> {
        SetInspectModeParamsBuilder {
            mode: mode.into(),
            highlight_config: None,
        }
    }
    /// Set an inspection mode.
    pub fn mode(&self) -> &InspectMode { &self.mode }
    /// A descriptor for the highlight appearance of hovered-over nodes. May be omitted if 'enabled
    /// == false'.
    pub fn highlight_config(&self) -> Option<&HighlightConfig<'a>> { self.highlight_config.as_ref() }
}


pub struct SetInspectModeParamsBuilder<'a> {
    mode: InspectMode,
    highlight_config: Option<HighlightConfig<'a>>,
}

impl<'a> SetInspectModeParamsBuilder<'a> {
    /// A descriptor for the highlight appearance of hovered-over nodes. May be omitted if 'enabled
    /// == false'.
    pub fn highlight_config(mut self, highlight_config: HighlightConfig<'a>) -> Self { self.highlight_config = Some(highlight_config); self }
    pub fn build(self) -> SetInspectModeParams<'a> {
        SetInspectModeParams {
            mode: self.mode,
            highlight_config: self.highlight_config,
        }
    }
}

impl<'a> SetInspectModeParams<'a> { pub const METHOD: &'static str = "Overlay.setInspectMode"; }

impl<'a> crate::CdpCommand<'a> for SetInspectModeParams<'a> {
    const METHOD: &'static str = "Overlay.setInspectMode";
    type Response = crate::EmptyReturns;
}

/// Highlights owner element of all frames detected to be ads.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowAdHighlightsParams {
    /// True for showing ad highlights
    show: bool,
}

impl SetShowAdHighlightsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `show`: True for showing ad highlights
    pub fn builder(show: bool) -> SetShowAdHighlightsParamsBuilder {
        SetShowAdHighlightsParamsBuilder {
            show: show,
        }
    }
    /// True for showing ad highlights
    pub fn show(&self) -> bool { self.show }
}


pub struct SetShowAdHighlightsParamsBuilder {
    show: bool,
}

impl SetShowAdHighlightsParamsBuilder {
    pub fn build(self) -> SetShowAdHighlightsParams {
        SetShowAdHighlightsParams {
            show: self.show,
        }
    }
}

impl SetShowAdHighlightsParams { pub const METHOD: &'static str = "Overlay.setShowAdHighlights"; }

impl<'a> crate::CdpCommand<'a> for SetShowAdHighlightsParams {
    const METHOD: &'static str = "Overlay.setShowAdHighlights";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPausedInDebuggerMessageParams<'a> {
    /// The message to display, also triggers resume and step over controls.
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Cow<'a, str>>,
}

impl<'a> SetPausedInDebuggerMessageParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> SetPausedInDebuggerMessageParamsBuilder<'a> {
        SetPausedInDebuggerMessageParamsBuilder {
            message: None,
        }
    }
    /// The message to display, also triggers resume and step over controls.
    pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}

#[derive(Default)]
pub struct SetPausedInDebuggerMessageParamsBuilder<'a> {
    message: Option<Cow<'a, str>>,
}

impl<'a> SetPausedInDebuggerMessageParamsBuilder<'a> {
    /// The message to display, also triggers resume and step over controls.
    pub fn message(mut self, message: impl Into<Cow<'a, str>>) -> Self { self.message = Some(message.into()); self }
    pub fn build(self) -> SetPausedInDebuggerMessageParams<'a> {
        SetPausedInDebuggerMessageParams {
            message: self.message,
        }
    }
}

impl<'a> SetPausedInDebuggerMessageParams<'a> { pub const METHOD: &'static str = "Overlay.setPausedInDebuggerMessage"; }

impl<'a> crate::CdpCommand<'a> for SetPausedInDebuggerMessageParams<'a> {
    const METHOD: &'static str = "Overlay.setPausedInDebuggerMessage";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows debug borders on layers

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowDebugBordersParams {
    /// True for showing debug borders
    show: bool,
}

impl SetShowDebugBordersParams {
    /// Creates a builder for this type with the required parameters:
    /// * `show`: True for showing debug borders
    pub fn builder(show: bool) -> SetShowDebugBordersParamsBuilder {
        SetShowDebugBordersParamsBuilder {
            show: show,
        }
    }
    /// True for showing debug borders
    pub fn show(&self) -> bool { self.show }
}


pub struct SetShowDebugBordersParamsBuilder {
    show: bool,
}

impl SetShowDebugBordersParamsBuilder {
    pub fn build(self) -> SetShowDebugBordersParams {
        SetShowDebugBordersParams {
            show: self.show,
        }
    }
}

impl SetShowDebugBordersParams { pub const METHOD: &'static str = "Overlay.setShowDebugBorders"; }

impl<'a> crate::CdpCommand<'a> for SetShowDebugBordersParams {
    const METHOD: &'static str = "Overlay.setShowDebugBorders";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows the FPS counter

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowFPSCounterParams {
    /// True for showing the FPS counter
    show: bool,
}

impl SetShowFPSCounterParams {
    /// Creates a builder for this type with the required parameters:
    /// * `show`: True for showing the FPS counter
    pub fn builder(show: bool) -> SetShowFPSCounterParamsBuilder {
        SetShowFPSCounterParamsBuilder {
            show: show,
        }
    }
    /// True for showing the FPS counter
    pub fn show(&self) -> bool { self.show }
}


pub struct SetShowFPSCounterParamsBuilder {
    show: bool,
}

impl SetShowFPSCounterParamsBuilder {
    pub fn build(self) -> SetShowFPSCounterParams {
        SetShowFPSCounterParams {
            show: self.show,
        }
    }
}

impl SetShowFPSCounterParams { pub const METHOD: &'static str = "Overlay.setShowFPSCounter"; }

impl<'a> crate::CdpCommand<'a> for SetShowFPSCounterParams {
    const METHOD: &'static str = "Overlay.setShowFPSCounter";
    type Response = crate::EmptyReturns;
}

/// Highlight multiple elements with the CSS Grid overlay.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowGridOverlaysParams {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "gridNodeHighlightConfigs")]
    grid_node_highlight_configs: Vec<GridNodeHighlightConfig>,
}

impl SetShowGridOverlaysParams {
    /// Creates a builder for this type with the required parameters:
    /// * `grid_node_highlight_configs`: An array of node identifiers and descriptors for the highlight appearance.
    pub fn builder(grid_node_highlight_configs: Vec<GridNodeHighlightConfig>) -> SetShowGridOverlaysParamsBuilder {
        SetShowGridOverlaysParamsBuilder {
            grid_node_highlight_configs: grid_node_highlight_configs,
        }
    }
    /// An array of node identifiers and descriptors for the highlight appearance.
    pub fn grid_node_highlight_configs(&self) -> &[GridNodeHighlightConfig] { &self.grid_node_highlight_configs }
}


pub struct SetShowGridOverlaysParamsBuilder {
    grid_node_highlight_configs: Vec<GridNodeHighlightConfig>,
}

impl SetShowGridOverlaysParamsBuilder {
    pub fn build(self) -> SetShowGridOverlaysParams {
        SetShowGridOverlaysParams {
            grid_node_highlight_configs: self.grid_node_highlight_configs,
        }
    }
}

impl SetShowGridOverlaysParams { pub const METHOD: &'static str = "Overlay.setShowGridOverlays"; }

impl<'a> crate::CdpCommand<'a> for SetShowGridOverlaysParams {
    const METHOD: &'static str = "Overlay.setShowGridOverlays";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowFlexOverlaysParams<'a> {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "flexNodeHighlightConfigs")]
    flex_node_highlight_configs: Vec<FlexNodeHighlightConfig<'a>>,
}

impl<'a> SetShowFlexOverlaysParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `flex_node_highlight_configs`: An array of node identifiers and descriptors for the highlight appearance.
    pub fn builder(flex_node_highlight_configs: Vec<FlexNodeHighlightConfig<'a>>) -> SetShowFlexOverlaysParamsBuilder<'a> {
        SetShowFlexOverlaysParamsBuilder {
            flex_node_highlight_configs: flex_node_highlight_configs,
        }
    }
    /// An array of node identifiers and descriptors for the highlight appearance.
    pub fn flex_node_highlight_configs(&self) -> &[FlexNodeHighlightConfig<'a>] { &self.flex_node_highlight_configs }
}


pub struct SetShowFlexOverlaysParamsBuilder<'a> {
    flex_node_highlight_configs: Vec<FlexNodeHighlightConfig<'a>>,
}

impl<'a> SetShowFlexOverlaysParamsBuilder<'a> {
    pub fn build(self) -> SetShowFlexOverlaysParams<'a> {
        SetShowFlexOverlaysParams {
            flex_node_highlight_configs: self.flex_node_highlight_configs,
        }
    }
}

impl<'a> SetShowFlexOverlaysParams<'a> { pub const METHOD: &'static str = "Overlay.setShowFlexOverlays"; }

impl<'a> crate::CdpCommand<'a> for SetShowFlexOverlaysParams<'a> {
    const METHOD: &'static str = "Overlay.setShowFlexOverlays";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowScrollSnapOverlaysParams<'a> {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "scrollSnapHighlightConfigs")]
    scroll_snap_highlight_configs: Vec<ScrollSnapHighlightConfig<'a>>,
}

impl<'a> SetShowScrollSnapOverlaysParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `scroll_snap_highlight_configs`: An array of node identifiers and descriptors for the highlight appearance.
    pub fn builder(scroll_snap_highlight_configs: Vec<ScrollSnapHighlightConfig<'a>>) -> SetShowScrollSnapOverlaysParamsBuilder<'a> {
        SetShowScrollSnapOverlaysParamsBuilder {
            scroll_snap_highlight_configs: scroll_snap_highlight_configs,
        }
    }
    /// An array of node identifiers and descriptors for the highlight appearance.
    pub fn scroll_snap_highlight_configs(&self) -> &[ScrollSnapHighlightConfig<'a>] { &self.scroll_snap_highlight_configs }
}


pub struct SetShowScrollSnapOverlaysParamsBuilder<'a> {
    scroll_snap_highlight_configs: Vec<ScrollSnapHighlightConfig<'a>>,
}

impl<'a> SetShowScrollSnapOverlaysParamsBuilder<'a> {
    pub fn build(self) -> SetShowScrollSnapOverlaysParams<'a> {
        SetShowScrollSnapOverlaysParams {
            scroll_snap_highlight_configs: self.scroll_snap_highlight_configs,
        }
    }
}

impl<'a> SetShowScrollSnapOverlaysParams<'a> { pub const METHOD: &'static str = "Overlay.setShowScrollSnapOverlays"; }

impl<'a> crate::CdpCommand<'a> for SetShowScrollSnapOverlaysParams<'a> {
    const METHOD: &'static str = "Overlay.setShowScrollSnapOverlays";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowContainerQueryOverlaysParams<'a> {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "containerQueryHighlightConfigs")]
    container_query_highlight_configs: Vec<ContainerQueryHighlightConfig<'a>>,
}

impl<'a> SetShowContainerQueryOverlaysParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `container_query_highlight_configs`: An array of node identifiers and descriptors for the highlight appearance.
    pub fn builder(container_query_highlight_configs: Vec<ContainerQueryHighlightConfig<'a>>) -> SetShowContainerQueryOverlaysParamsBuilder<'a> {
        SetShowContainerQueryOverlaysParamsBuilder {
            container_query_highlight_configs: container_query_highlight_configs,
        }
    }
    /// An array of node identifiers and descriptors for the highlight appearance.
    pub fn container_query_highlight_configs(&self) -> &[ContainerQueryHighlightConfig<'a>] { &self.container_query_highlight_configs }
}


pub struct SetShowContainerQueryOverlaysParamsBuilder<'a> {
    container_query_highlight_configs: Vec<ContainerQueryHighlightConfig<'a>>,
}

impl<'a> SetShowContainerQueryOverlaysParamsBuilder<'a> {
    pub fn build(self) -> SetShowContainerQueryOverlaysParams<'a> {
        SetShowContainerQueryOverlaysParams {
            container_query_highlight_configs: self.container_query_highlight_configs,
        }
    }
}

impl<'a> SetShowContainerQueryOverlaysParams<'a> { pub const METHOD: &'static str = "Overlay.setShowContainerQueryOverlays"; }

impl<'a> crate::CdpCommand<'a> for SetShowContainerQueryOverlaysParams<'a> {
    const METHOD: &'static str = "Overlay.setShowContainerQueryOverlays";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowInspectedElementAnchorParams {
    /// Node identifier for which to show an anchor for.
    #[serde(rename = "inspectedElementAnchorConfig")]
    inspected_element_anchor_config: InspectedElementAnchorConfig,
}

impl SetShowInspectedElementAnchorParams {
    /// Creates a builder for this type with the required parameters:
    /// * `inspected_element_anchor_config`: Node identifier for which to show an anchor for.
    pub fn builder(inspected_element_anchor_config: InspectedElementAnchorConfig) -> SetShowInspectedElementAnchorParamsBuilder {
        SetShowInspectedElementAnchorParamsBuilder {
            inspected_element_anchor_config: inspected_element_anchor_config,
        }
    }
    /// Node identifier for which to show an anchor for.
    pub fn inspected_element_anchor_config(&self) -> &InspectedElementAnchorConfig { &self.inspected_element_anchor_config }
}


pub struct SetShowInspectedElementAnchorParamsBuilder {
    inspected_element_anchor_config: InspectedElementAnchorConfig,
}

impl SetShowInspectedElementAnchorParamsBuilder {
    pub fn build(self) -> SetShowInspectedElementAnchorParams {
        SetShowInspectedElementAnchorParams {
            inspected_element_anchor_config: self.inspected_element_anchor_config,
        }
    }
}

impl SetShowInspectedElementAnchorParams { pub const METHOD: &'static str = "Overlay.setShowInspectedElementAnchor"; }

impl<'a> crate::CdpCommand<'a> for SetShowInspectedElementAnchorParams {
    const METHOD: &'static str = "Overlay.setShowInspectedElementAnchor";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows paint rectangles

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowPaintRectsParams {
    /// True for showing paint rectangles
    result: bool,
}

impl SetShowPaintRectsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: True for showing paint rectangles
    pub fn builder(result: bool) -> SetShowPaintRectsParamsBuilder {
        SetShowPaintRectsParamsBuilder {
            result: result,
        }
    }
    /// True for showing paint rectangles
    pub fn result(&self) -> bool { self.result }
}


pub struct SetShowPaintRectsParamsBuilder {
    result: bool,
}

impl SetShowPaintRectsParamsBuilder {
    pub fn build(self) -> SetShowPaintRectsParams {
        SetShowPaintRectsParams {
            result: self.result,
        }
    }
}

impl SetShowPaintRectsParams { pub const METHOD: &'static str = "Overlay.setShowPaintRects"; }

impl<'a> crate::CdpCommand<'a> for SetShowPaintRectsParams {
    const METHOD: &'static str = "Overlay.setShowPaintRects";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows layout shift regions

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowLayoutShiftRegionsParams {
    /// True for showing layout shift regions
    result: bool,
}

impl SetShowLayoutShiftRegionsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: True for showing layout shift regions
    pub fn builder(result: bool) -> SetShowLayoutShiftRegionsParamsBuilder {
        SetShowLayoutShiftRegionsParamsBuilder {
            result: result,
        }
    }
    /// True for showing layout shift regions
    pub fn result(&self) -> bool { self.result }
}


pub struct SetShowLayoutShiftRegionsParamsBuilder {
    result: bool,
}

impl SetShowLayoutShiftRegionsParamsBuilder {
    pub fn build(self) -> SetShowLayoutShiftRegionsParams {
        SetShowLayoutShiftRegionsParams {
            result: self.result,
        }
    }
}

impl SetShowLayoutShiftRegionsParams { pub const METHOD: &'static str = "Overlay.setShowLayoutShiftRegions"; }

impl<'a> crate::CdpCommand<'a> for SetShowLayoutShiftRegionsParams {
    const METHOD: &'static str = "Overlay.setShowLayoutShiftRegions";
    type Response = crate::EmptyReturns;
}

/// Requests that backend shows scroll bottleneck rects

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowScrollBottleneckRectsParams {
    /// True for showing scroll bottleneck rects
    show: bool,
}

impl SetShowScrollBottleneckRectsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `show`: True for showing scroll bottleneck rects
    pub fn builder(show: bool) -> SetShowScrollBottleneckRectsParamsBuilder {
        SetShowScrollBottleneckRectsParamsBuilder {
            show: show,
        }
    }
    /// True for showing scroll bottleneck rects
    pub fn show(&self) -> bool { self.show }
}


pub struct SetShowScrollBottleneckRectsParamsBuilder {
    show: bool,
}

impl SetShowScrollBottleneckRectsParamsBuilder {
    pub fn build(self) -> SetShowScrollBottleneckRectsParams {
        SetShowScrollBottleneckRectsParams {
            show: self.show,
        }
    }
}

impl SetShowScrollBottleneckRectsParams { pub const METHOD: &'static str = "Overlay.setShowScrollBottleneckRects"; }

impl<'a> crate::CdpCommand<'a> for SetShowScrollBottleneckRectsParams {
    const METHOD: &'static str = "Overlay.setShowScrollBottleneckRects";
    type Response = crate::EmptyReturns;
}

/// Deprecated, no longer has any effect.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowHitTestBordersParams {
    /// True for showing hit-test borders
    show: bool,
}

impl SetShowHitTestBordersParams {
    /// Creates a builder for this type with the required parameters:
    /// * `show`: True for showing hit-test borders
    pub fn builder(show: bool) -> SetShowHitTestBordersParamsBuilder {
        SetShowHitTestBordersParamsBuilder {
            show: show,
        }
    }
    /// True for showing hit-test borders
    pub fn show(&self) -> bool { self.show }
}


pub struct SetShowHitTestBordersParamsBuilder {
    show: bool,
}

impl SetShowHitTestBordersParamsBuilder {
    pub fn build(self) -> SetShowHitTestBordersParams {
        SetShowHitTestBordersParams {
            show: self.show,
        }
    }
}

impl SetShowHitTestBordersParams { pub const METHOD: &'static str = "Overlay.setShowHitTestBorders"; }

impl<'a> crate::CdpCommand<'a> for SetShowHitTestBordersParams {
    const METHOD: &'static str = "Overlay.setShowHitTestBorders";
    type Response = crate::EmptyReturns;
}

/// Deprecated, no longer has any effect.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowWebVitalsParams {
    show: bool,
}

impl SetShowWebVitalsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `show`: 
    pub fn builder(show: bool) -> SetShowWebVitalsParamsBuilder {
        SetShowWebVitalsParamsBuilder {
            show: show,
        }
    }
    pub fn show(&self) -> bool { self.show }
}


pub struct SetShowWebVitalsParamsBuilder {
    show: bool,
}

impl SetShowWebVitalsParamsBuilder {
    pub fn build(self) -> SetShowWebVitalsParams {
        SetShowWebVitalsParams {
            show: self.show,
        }
    }
}

impl SetShowWebVitalsParams { pub const METHOD: &'static str = "Overlay.setShowWebVitals"; }

impl<'a> crate::CdpCommand<'a> for SetShowWebVitalsParams {
    const METHOD: &'static str = "Overlay.setShowWebVitals";
    type Response = crate::EmptyReturns;
}

/// Paints viewport size upon main frame resize.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowViewportSizeOnResizeParams {
    /// Whether to paint size or not.
    show: bool,
}

impl SetShowViewportSizeOnResizeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `show`: Whether to paint size or not.
    pub fn builder(show: bool) -> SetShowViewportSizeOnResizeParamsBuilder {
        SetShowViewportSizeOnResizeParamsBuilder {
            show: show,
        }
    }
    /// Whether to paint size or not.
    pub fn show(&self) -> bool { self.show }
}


pub struct SetShowViewportSizeOnResizeParamsBuilder {
    show: bool,
}

impl SetShowViewportSizeOnResizeParamsBuilder {
    pub fn build(self) -> SetShowViewportSizeOnResizeParams {
        SetShowViewportSizeOnResizeParams {
            show: self.show,
        }
    }
}

impl SetShowViewportSizeOnResizeParams { pub const METHOD: &'static str = "Overlay.setShowViewportSizeOnResize"; }

impl<'a> crate::CdpCommand<'a> for SetShowViewportSizeOnResizeParams {
    const METHOD: &'static str = "Overlay.setShowViewportSizeOnResize";
    type Response = crate::EmptyReturns;
}

/// Add a dual screen device hinge

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowHingeParams {
    /// hinge data, null means hideHinge
    #[serde(skip_serializing_if = "Option::is_none", rename = "hingeConfig")]
    hinge_config: Option<HingeConfig>,
}

impl SetShowHingeParams {
    /// Creates a builder for this type.
    pub fn builder() -> SetShowHingeParamsBuilder {
        SetShowHingeParamsBuilder {
            hinge_config: None,
        }
    }
    /// hinge data, null means hideHinge
    pub fn hinge_config(&self) -> Option<&HingeConfig> { self.hinge_config.as_ref() }
}

#[derive(Default)]
pub struct SetShowHingeParamsBuilder {
    hinge_config: Option<HingeConfig>,
}

impl SetShowHingeParamsBuilder {
    /// hinge data, null means hideHinge
    pub fn hinge_config(mut self, hinge_config: HingeConfig) -> Self { self.hinge_config = Some(hinge_config); self }
    pub fn build(self) -> SetShowHingeParams {
        SetShowHingeParams {
            hinge_config: self.hinge_config,
        }
    }
}

impl SetShowHingeParams { pub const METHOD: &'static str = "Overlay.setShowHinge"; }

impl<'a> crate::CdpCommand<'a> for SetShowHingeParams {
    const METHOD: &'static str = "Overlay.setShowHinge";
    type Response = crate::EmptyReturns;
}

/// Show elements in isolation mode with overlays.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowIsolatedElementsParams {
    /// An array of node identifiers and descriptors for the highlight appearance.
    #[serde(rename = "isolatedElementHighlightConfigs")]
    isolated_element_highlight_configs: Vec<IsolatedElementHighlightConfig>,
}

impl SetShowIsolatedElementsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `isolated_element_highlight_configs`: An array of node identifiers and descriptors for the highlight appearance.
    pub fn builder(isolated_element_highlight_configs: Vec<IsolatedElementHighlightConfig>) -> SetShowIsolatedElementsParamsBuilder {
        SetShowIsolatedElementsParamsBuilder {
            isolated_element_highlight_configs: isolated_element_highlight_configs,
        }
    }
    /// An array of node identifiers and descriptors for the highlight appearance.
    pub fn isolated_element_highlight_configs(&self) -> &[IsolatedElementHighlightConfig] { &self.isolated_element_highlight_configs }
}


pub struct SetShowIsolatedElementsParamsBuilder {
    isolated_element_highlight_configs: Vec<IsolatedElementHighlightConfig>,
}

impl SetShowIsolatedElementsParamsBuilder {
    pub fn build(self) -> SetShowIsolatedElementsParams {
        SetShowIsolatedElementsParams {
            isolated_element_highlight_configs: self.isolated_element_highlight_configs,
        }
    }
}

impl SetShowIsolatedElementsParams { pub const METHOD: &'static str = "Overlay.setShowIsolatedElements"; }

impl<'a> crate::CdpCommand<'a> for SetShowIsolatedElementsParams {
    const METHOD: &'static str = "Overlay.setShowIsolatedElements";
    type Response = crate::EmptyReturns;
}

/// Show Window Controls Overlay for PWA

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetShowWindowControlsOverlayParams<'a> {
    /// Window Controls Overlay data, null means hide Window Controls Overlay
    #[serde(skip_serializing_if = "Option::is_none", rename = "windowControlsOverlayConfig")]
    window_controls_overlay_config: Option<WindowControlsOverlayConfig<'a>>,
}

impl<'a> SetShowWindowControlsOverlayParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> SetShowWindowControlsOverlayParamsBuilder<'a> {
        SetShowWindowControlsOverlayParamsBuilder {
            window_controls_overlay_config: None,
        }
    }
    /// Window Controls Overlay data, null means hide Window Controls Overlay
    pub fn window_controls_overlay_config(&self) -> Option<&WindowControlsOverlayConfig<'a>> { self.window_controls_overlay_config.as_ref() }
}

#[derive(Default)]
pub struct SetShowWindowControlsOverlayParamsBuilder<'a> {
    window_controls_overlay_config: Option<WindowControlsOverlayConfig<'a>>,
}

impl<'a> SetShowWindowControlsOverlayParamsBuilder<'a> {
    /// Window Controls Overlay data, null means hide Window Controls Overlay
    pub fn window_controls_overlay_config(mut self, window_controls_overlay_config: WindowControlsOverlayConfig<'a>) -> Self { self.window_controls_overlay_config = Some(window_controls_overlay_config); self }
    pub fn build(self) -> SetShowWindowControlsOverlayParams<'a> {
        SetShowWindowControlsOverlayParams {
            window_controls_overlay_config: self.window_controls_overlay_config,
        }
    }
}

impl<'a> SetShowWindowControlsOverlayParams<'a> { pub const METHOD: &'static str = "Overlay.setShowWindowControlsOverlay"; }

impl<'a> crate::CdpCommand<'a> for SetShowWindowControlsOverlayParams<'a> {
    const METHOD: &'static str = "Overlay.setShowWindowControlsOverlay";
    type Response = crate::EmptyReturns;
}
