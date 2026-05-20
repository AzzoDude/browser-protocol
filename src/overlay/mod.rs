//! This domain provides various functionality related to drawing atop the inspected page.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Configuration data for drawing the source order of an elements children.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceOrderConfig {
    /// the color to outline the given element in.
    parentOutlineColor: crate::dom::RGBA,
    /// the color to outline the child elements in.
    childOutlineColor: crate::dom::RGBA,
}

impl SourceOrderConfig {
    pub fn builder(parentOutlineColor: crate::dom::RGBA, childOutlineColor: crate::dom::RGBA) -> SourceOrderConfigBuilder {
        SourceOrderConfigBuilder {
            parentOutlineColor: parentOutlineColor,
            childOutlineColor: childOutlineColor,
        }
    }
    pub fn parentOutlineColor(&self) -> &crate::dom::RGBA { &self.parentOutlineColor }
    pub fn childOutlineColor(&self) -> &crate::dom::RGBA { &self.childOutlineColor }
}


pub struct SourceOrderConfigBuilder {
    parentOutlineColor: crate::dom::RGBA,
    childOutlineColor: crate::dom::RGBA,
}

impl SourceOrderConfigBuilder {
    pub fn build(self) -> SourceOrderConfig {
        SourceOrderConfig {
            parentOutlineColor: self.parentOutlineColor,
            childOutlineColor: self.childOutlineColor,
        }
    }
}

/// Configuration data for the highlighting of Grid elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GridHighlightConfig {
    /// Whether the extension lines from grid cells to the rulers should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    showGridExtensionLines: Option<bool>,
    /// Show Positive line number labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    showPositiveLineNumbers: Option<bool>,
    /// Show Negative line number labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    showNegativeLineNumbers: Option<bool>,
    /// Show area name labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    showAreaNames: Option<bool>,
    /// Show line name labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    showLineNames: Option<bool>,
    /// Show track size labels (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    showTrackSizes: Option<bool>,
    /// The grid container border highlight color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    gridBorderColor: Option<crate::dom::RGBA>,
    /// The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    cellBorderColor: Option<crate::dom::RGBA>,
    /// The row line color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    rowLineColor: Option<crate::dom::RGBA>,
    /// The column line color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    columnLineColor: Option<crate::dom::RGBA>,
    /// Whether the grid border is dashed (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    gridBorderDash: Option<bool>,
    /// Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    cellBorderDash: Option<bool>,
    /// Whether row lines are dashed (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    rowLineDash: Option<bool>,
    /// Whether column lines are dashed (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    columnLineDash: Option<bool>,
    /// The row gap highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    rowGapColor: Option<crate::dom::RGBA>,
    /// The row gap hatching fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    rowHatchColor: Option<crate::dom::RGBA>,
    /// The column gap highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    columnGapColor: Option<crate::dom::RGBA>,
    /// The column gap hatching fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    columnHatchColor: Option<crate::dom::RGBA>,
    /// The named grid areas border color (Default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    areaBorderColor: Option<crate::dom::RGBA>,
    /// The grid container background color (Default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    gridBackgroundColor: Option<crate::dom::RGBA>,
}

impl GridHighlightConfig {
    pub fn builder() -> GridHighlightConfigBuilder {
        GridHighlightConfigBuilder {
            showGridExtensionLines: None,
            showPositiveLineNumbers: None,
            showNegativeLineNumbers: None,
            showAreaNames: None,
            showLineNames: None,
            showTrackSizes: None,
            gridBorderColor: None,
            cellBorderColor: None,
            rowLineColor: None,
            columnLineColor: None,
            gridBorderDash: None,
            cellBorderDash: None,
            rowLineDash: None,
            columnLineDash: None,
            rowGapColor: None,
            rowHatchColor: None,
            columnGapColor: None,
            columnHatchColor: None,
            areaBorderColor: None,
            gridBackgroundColor: None,
        }
    }
    pub fn showGridExtensionLines(&self) -> Option<bool> { self.showGridExtensionLines }
    pub fn showPositiveLineNumbers(&self) -> Option<bool> { self.showPositiveLineNumbers }
    pub fn showNegativeLineNumbers(&self) -> Option<bool> { self.showNegativeLineNumbers }
    pub fn showAreaNames(&self) -> Option<bool> { self.showAreaNames }
    pub fn showLineNames(&self) -> Option<bool> { self.showLineNames }
    pub fn showTrackSizes(&self) -> Option<bool> { self.showTrackSizes }
    pub fn gridBorderColor(&self) -> Option<&crate::dom::RGBA> { self.gridBorderColor.as_ref() }
    pub fn cellBorderColor(&self) -> Option<&crate::dom::RGBA> { self.cellBorderColor.as_ref() }
    pub fn rowLineColor(&self) -> Option<&crate::dom::RGBA> { self.rowLineColor.as_ref() }
    pub fn columnLineColor(&self) -> Option<&crate::dom::RGBA> { self.columnLineColor.as_ref() }
    pub fn gridBorderDash(&self) -> Option<bool> { self.gridBorderDash }
    pub fn cellBorderDash(&self) -> Option<bool> { self.cellBorderDash }
    pub fn rowLineDash(&self) -> Option<bool> { self.rowLineDash }
    pub fn columnLineDash(&self) -> Option<bool> { self.columnLineDash }
    pub fn rowGapColor(&self) -> Option<&crate::dom::RGBA> { self.rowGapColor.as_ref() }
    pub fn rowHatchColor(&self) -> Option<&crate::dom::RGBA> { self.rowHatchColor.as_ref() }
    pub fn columnGapColor(&self) -> Option<&crate::dom::RGBA> { self.columnGapColor.as_ref() }
    pub fn columnHatchColor(&self) -> Option<&crate::dom::RGBA> { self.columnHatchColor.as_ref() }
    pub fn areaBorderColor(&self) -> Option<&crate::dom::RGBA> { self.areaBorderColor.as_ref() }
    pub fn gridBackgroundColor(&self) -> Option<&crate::dom::RGBA> { self.gridBackgroundColor.as_ref() }
}

#[derive(Default)]
pub struct GridHighlightConfigBuilder {
    showGridExtensionLines: Option<bool>,
    showPositiveLineNumbers: Option<bool>,
    showNegativeLineNumbers: Option<bool>,
    showAreaNames: Option<bool>,
    showLineNames: Option<bool>,
    showTrackSizes: Option<bool>,
    gridBorderColor: Option<crate::dom::RGBA>,
    cellBorderColor: Option<crate::dom::RGBA>,
    rowLineColor: Option<crate::dom::RGBA>,
    columnLineColor: Option<crate::dom::RGBA>,
    gridBorderDash: Option<bool>,
    cellBorderDash: Option<bool>,
    rowLineDash: Option<bool>,
    columnLineDash: Option<bool>,
    rowGapColor: Option<crate::dom::RGBA>,
    rowHatchColor: Option<crate::dom::RGBA>,
    columnGapColor: Option<crate::dom::RGBA>,
    columnHatchColor: Option<crate::dom::RGBA>,
    areaBorderColor: Option<crate::dom::RGBA>,
    gridBackgroundColor: Option<crate::dom::RGBA>,
}

impl GridHighlightConfigBuilder {
    /// Whether the extension lines from grid cells to the rulers should be shown (default: false).
    pub fn showGridExtensionLines(mut self, showGridExtensionLines: bool) -> Self { self.showGridExtensionLines = Some(showGridExtensionLines); self }
    /// Show Positive line number labels (default: false).
    pub fn showPositiveLineNumbers(mut self, showPositiveLineNumbers: bool) -> Self { self.showPositiveLineNumbers = Some(showPositiveLineNumbers); self }
    /// Show Negative line number labels (default: false).
    pub fn showNegativeLineNumbers(mut self, showNegativeLineNumbers: bool) -> Self { self.showNegativeLineNumbers = Some(showNegativeLineNumbers); self }
    /// Show area name labels (default: false).
    pub fn showAreaNames(mut self, showAreaNames: bool) -> Self { self.showAreaNames = Some(showAreaNames); self }
    /// Show line name labels (default: false).
    pub fn showLineNames(mut self, showLineNames: bool) -> Self { self.showLineNames = Some(showLineNames); self }
    /// Show track size labels (default: false).
    pub fn showTrackSizes(mut self, showTrackSizes: bool) -> Self { self.showTrackSizes = Some(showTrackSizes); self }
    /// The grid container border highlight color (default: transparent).
    pub fn gridBorderColor(mut self, gridBorderColor: crate::dom::RGBA) -> Self { self.gridBorderColor = Some(gridBorderColor); self }
    /// The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.
    pub fn cellBorderColor(mut self, cellBorderColor: crate::dom::RGBA) -> Self { self.cellBorderColor = Some(cellBorderColor); self }
    /// The row line color (default: transparent).
    pub fn rowLineColor(mut self, rowLineColor: crate::dom::RGBA) -> Self { self.rowLineColor = Some(rowLineColor); self }
    /// The column line color (default: transparent).
    pub fn columnLineColor(mut self, columnLineColor: crate::dom::RGBA) -> Self { self.columnLineColor = Some(columnLineColor); self }
    /// Whether the grid border is dashed (default: false).
    pub fn gridBorderDash(mut self, gridBorderDash: bool) -> Self { self.gridBorderDash = Some(gridBorderDash); self }
    /// Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.
    pub fn cellBorderDash(mut self, cellBorderDash: bool) -> Self { self.cellBorderDash = Some(cellBorderDash); self }
    /// Whether row lines are dashed (default: false).
    pub fn rowLineDash(mut self, rowLineDash: bool) -> Self { self.rowLineDash = Some(rowLineDash); self }
    /// Whether column lines are dashed (default: false).
    pub fn columnLineDash(mut self, columnLineDash: bool) -> Self { self.columnLineDash = Some(columnLineDash); self }
    /// The row gap highlight fill color (default: transparent).
    pub fn rowGapColor(mut self, rowGapColor: crate::dom::RGBA) -> Self { self.rowGapColor = Some(rowGapColor); self }
    /// The row gap hatching fill color (default: transparent).
    pub fn rowHatchColor(mut self, rowHatchColor: crate::dom::RGBA) -> Self { self.rowHatchColor = Some(rowHatchColor); self }
    /// The column gap highlight fill color (default: transparent).
    pub fn columnGapColor(mut self, columnGapColor: crate::dom::RGBA) -> Self { self.columnGapColor = Some(columnGapColor); self }
    /// The column gap hatching fill color (default: transparent).
    pub fn columnHatchColor(mut self, columnHatchColor: crate::dom::RGBA) -> Self { self.columnHatchColor = Some(columnHatchColor); self }
    /// The named grid areas border color (Default: transparent).
    pub fn areaBorderColor(mut self, areaBorderColor: crate::dom::RGBA) -> Self { self.areaBorderColor = Some(areaBorderColor); self }
    /// The grid container background color (Default: transparent).
    pub fn gridBackgroundColor(mut self, gridBackgroundColor: crate::dom::RGBA) -> Self { self.gridBackgroundColor = Some(gridBackgroundColor); self }
    pub fn build(self) -> GridHighlightConfig {
        GridHighlightConfig {
            showGridExtensionLines: self.showGridExtensionLines,
            showPositiveLineNumbers: self.showPositiveLineNumbers,
            showNegativeLineNumbers: self.showNegativeLineNumbers,
            showAreaNames: self.showAreaNames,
            showLineNames: self.showLineNames,
            showTrackSizes: self.showTrackSizes,
            gridBorderColor: self.gridBorderColor,
            cellBorderColor: self.cellBorderColor,
            rowLineColor: self.rowLineColor,
            columnLineColor: self.columnLineColor,
            gridBorderDash: self.gridBorderDash,
            cellBorderDash: self.cellBorderDash,
            rowLineDash: self.rowLineDash,
            columnLineDash: self.columnLineDash,
            rowGapColor: self.rowGapColor,
            rowHatchColor: self.rowHatchColor,
            columnGapColor: self.columnGapColor,
            columnHatchColor: self.columnHatchColor,
            areaBorderColor: self.areaBorderColor,
            gridBackgroundColor: self.gridBackgroundColor,
        }
    }
}

/// Configuration data for the highlighting of Flex container elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexContainerHighlightConfig<'a> {
    /// The style of the container border
    #[serde(skip_serializing_if = "Option::is_none")]
    containerBorder: Option<LineStyle<'a>>,
    /// The style of the separator between lines
    #[serde(skip_serializing_if = "Option::is_none")]
    lineSeparator: Option<LineStyle<'a>>,
    /// The style of the separator between items
    #[serde(skip_serializing_if = "Option::is_none")]
    itemSeparator: Option<LineStyle<'a>>,
    /// Style of content-distribution space on the main axis (justify-content).
    #[serde(skip_serializing_if = "Option::is_none")]
    mainDistributedSpace: Option<BoxStyle>,
    /// Style of content-distribution space on the cross axis (align-content).
    #[serde(skip_serializing_if = "Option::is_none")]
    crossDistributedSpace: Option<BoxStyle>,
    /// Style of empty space caused by row gaps (gap/row-gap).
    #[serde(skip_serializing_if = "Option::is_none")]
    rowGapSpace: Option<BoxStyle>,
    /// Style of empty space caused by columns gaps (gap/column-gap).
    #[serde(skip_serializing_if = "Option::is_none")]
    columnGapSpace: Option<BoxStyle>,
    /// Style of the self-alignment line (align-items).
    #[serde(skip_serializing_if = "Option::is_none")]
    crossAlignment: Option<LineStyle<'a>>,
}

impl<'a> FlexContainerHighlightConfig<'a> {
    pub fn builder() -> FlexContainerHighlightConfigBuilder<'a> {
        FlexContainerHighlightConfigBuilder {
            containerBorder: None,
            lineSeparator: None,
            itemSeparator: None,
            mainDistributedSpace: None,
            crossDistributedSpace: None,
            rowGapSpace: None,
            columnGapSpace: None,
            crossAlignment: None,
        }
    }
    pub fn containerBorder(&self) -> Option<&LineStyle<'a>> { self.containerBorder.as_ref() }
    pub fn lineSeparator(&self) -> Option<&LineStyle<'a>> { self.lineSeparator.as_ref() }
    pub fn itemSeparator(&self) -> Option<&LineStyle<'a>> { self.itemSeparator.as_ref() }
    pub fn mainDistributedSpace(&self) -> Option<&BoxStyle> { self.mainDistributedSpace.as_ref() }
    pub fn crossDistributedSpace(&self) -> Option<&BoxStyle> { self.crossDistributedSpace.as_ref() }
    pub fn rowGapSpace(&self) -> Option<&BoxStyle> { self.rowGapSpace.as_ref() }
    pub fn columnGapSpace(&self) -> Option<&BoxStyle> { self.columnGapSpace.as_ref() }
    pub fn crossAlignment(&self) -> Option<&LineStyle<'a>> { self.crossAlignment.as_ref() }
}

#[derive(Default)]
pub struct FlexContainerHighlightConfigBuilder<'a> {
    containerBorder: Option<LineStyle<'a>>,
    lineSeparator: Option<LineStyle<'a>>,
    itemSeparator: Option<LineStyle<'a>>,
    mainDistributedSpace: Option<BoxStyle>,
    crossDistributedSpace: Option<BoxStyle>,
    rowGapSpace: Option<BoxStyle>,
    columnGapSpace: Option<BoxStyle>,
    crossAlignment: Option<LineStyle<'a>>,
}

impl<'a> FlexContainerHighlightConfigBuilder<'a> {
    /// The style of the container border
    pub fn containerBorder(mut self, containerBorder: LineStyle<'a>) -> Self { self.containerBorder = Some(containerBorder); self }
    /// The style of the separator between lines
    pub fn lineSeparator(mut self, lineSeparator: LineStyle<'a>) -> Self { self.lineSeparator = Some(lineSeparator); self }
    /// The style of the separator between items
    pub fn itemSeparator(mut self, itemSeparator: LineStyle<'a>) -> Self { self.itemSeparator = Some(itemSeparator); self }
    /// Style of content-distribution space on the main axis (justify-content).
    pub fn mainDistributedSpace(mut self, mainDistributedSpace: BoxStyle) -> Self { self.mainDistributedSpace = Some(mainDistributedSpace); self }
    /// Style of content-distribution space on the cross axis (align-content).
    pub fn crossDistributedSpace(mut self, crossDistributedSpace: BoxStyle) -> Self { self.crossDistributedSpace = Some(crossDistributedSpace); self }
    /// Style of empty space caused by row gaps (gap/row-gap).
    pub fn rowGapSpace(mut self, rowGapSpace: BoxStyle) -> Self { self.rowGapSpace = Some(rowGapSpace); self }
    /// Style of empty space caused by columns gaps (gap/column-gap).
    pub fn columnGapSpace(mut self, columnGapSpace: BoxStyle) -> Self { self.columnGapSpace = Some(columnGapSpace); self }
    /// Style of the self-alignment line (align-items).
    pub fn crossAlignment(mut self, crossAlignment: LineStyle<'a>) -> Self { self.crossAlignment = Some(crossAlignment); self }
    pub fn build(self) -> FlexContainerHighlightConfig<'a> {
        FlexContainerHighlightConfig {
            containerBorder: self.containerBorder,
            lineSeparator: self.lineSeparator,
            itemSeparator: self.itemSeparator,
            mainDistributedSpace: self.mainDistributedSpace,
            crossDistributedSpace: self.crossDistributedSpace,
            rowGapSpace: self.rowGapSpace,
            columnGapSpace: self.columnGapSpace,
            crossAlignment: self.crossAlignment,
        }
    }
}

/// Configuration data for the highlighting of Flex item elements.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexItemHighlightConfig<'a> {
    /// Style of the box representing the item's base size
    #[serde(skip_serializing_if = "Option::is_none")]
    baseSizeBox: Option<BoxStyle>,
    /// Style of the border around the box representing the item's base size
    #[serde(skip_serializing_if = "Option::is_none")]
    baseSizeBorder: Option<LineStyle<'a>>,
    /// Style of the arrow representing if the item grew or shrank
    #[serde(skip_serializing_if = "Option::is_none")]
    flexibilityArrow: Option<LineStyle<'a>>,
}

impl<'a> FlexItemHighlightConfig<'a> {
    pub fn builder() -> FlexItemHighlightConfigBuilder<'a> {
        FlexItemHighlightConfigBuilder {
            baseSizeBox: None,
            baseSizeBorder: None,
            flexibilityArrow: None,
        }
    }
    pub fn baseSizeBox(&self) -> Option<&BoxStyle> { self.baseSizeBox.as_ref() }
    pub fn baseSizeBorder(&self) -> Option<&LineStyle<'a>> { self.baseSizeBorder.as_ref() }
    pub fn flexibilityArrow(&self) -> Option<&LineStyle<'a>> { self.flexibilityArrow.as_ref() }
}

#[derive(Default)]
pub struct FlexItemHighlightConfigBuilder<'a> {
    baseSizeBox: Option<BoxStyle>,
    baseSizeBorder: Option<LineStyle<'a>>,
    flexibilityArrow: Option<LineStyle<'a>>,
}

impl<'a> FlexItemHighlightConfigBuilder<'a> {
    /// Style of the box representing the item's base size
    pub fn baseSizeBox(mut self, baseSizeBox: BoxStyle) -> Self { self.baseSizeBox = Some(baseSizeBox); self }
    /// Style of the border around the box representing the item's base size
    pub fn baseSizeBorder(mut self, baseSizeBorder: LineStyle<'a>) -> Self { self.baseSizeBorder = Some(baseSizeBorder); self }
    /// Style of the arrow representing if the item grew or shrank
    pub fn flexibilityArrow(mut self, flexibilityArrow: LineStyle<'a>) -> Self { self.flexibilityArrow = Some(flexibilityArrow); self }
    pub fn build(self) -> FlexItemHighlightConfig<'a> {
        FlexItemHighlightConfig {
            baseSizeBox: self.baseSizeBox,
            baseSizeBorder: self.baseSizeBorder,
            flexibilityArrow: self.flexibilityArrow,
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
    pub fn builder() -> LineStyleBuilder<'a> {
        LineStyleBuilder {
            color: None,
            pattern: None,
        }
    }
    pub fn color(&self) -> Option<&crate::dom::RGBA> { self.color.as_ref() }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    fillColor: Option<crate::dom::RGBA>,
    /// The hatching color for the box (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    hatchColor: Option<crate::dom::RGBA>,
}

impl BoxStyle {
    pub fn builder() -> BoxStyleBuilder {
        BoxStyleBuilder {
            fillColor: None,
            hatchColor: None,
        }
    }
    pub fn fillColor(&self) -> Option<&crate::dom::RGBA> { self.fillColor.as_ref() }
    pub fn hatchColor(&self) -> Option<&crate::dom::RGBA> { self.hatchColor.as_ref() }
}

#[derive(Default)]
pub struct BoxStyleBuilder {
    fillColor: Option<crate::dom::RGBA>,
    hatchColor: Option<crate::dom::RGBA>,
}

impl BoxStyleBuilder {
    /// The background color for the box (default: transparent)
    pub fn fillColor(mut self, fillColor: crate::dom::RGBA) -> Self { self.fillColor = Some(fillColor); self }
    /// The hatching color for the box (default: transparent)
    pub fn hatchColor(mut self, hatchColor: crate::dom::RGBA) -> Self { self.hatchColor = Some(hatchColor); self }
    pub fn build(self) -> BoxStyle {
        BoxStyle {
            fillColor: self.fillColor,
            hatchColor: self.hatchColor,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    showInfo: Option<bool>,
    /// Whether the node styles in the tooltip (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    showStyles: Option<bool>,
    /// Whether the rulers should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    showRulers: Option<bool>,
    /// Whether the a11y info should be shown (default: true).
    #[serde(skip_serializing_if = "Option::is_none")]
    showAccessibilityInfo: Option<bool>,
    /// Whether the extension lines from node to the rulers should be shown (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    showExtensionLines: Option<bool>,
    /// The content box highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    contentColor: Option<crate::dom::RGBA>,
    /// The padding highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    paddingColor: Option<crate::dom::RGBA>,
    /// The border highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    borderColor: Option<crate::dom::RGBA>,
    /// The margin highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    marginColor: Option<crate::dom::RGBA>,
    /// The event target element highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    eventTargetColor: Option<crate::dom::RGBA>,
    /// The shape outside fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    shapeColor: Option<crate::dom::RGBA>,
    /// The shape margin fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    shapeMarginColor: Option<crate::dom::RGBA>,
    /// The grid layout color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    cssGridColor: Option<crate::dom::RGBA>,
    /// The color format used to format color styles (default: hex).
    #[serde(skip_serializing_if = "Option::is_none")]
    colorFormat: Option<ColorFormat>,
    /// The grid layout highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    gridHighlightConfig: Option<GridHighlightConfig>,
    /// The flex container highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    flexContainerHighlightConfig: Option<FlexContainerHighlightConfig<'a>>,
    /// The flex item highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    flexItemHighlightConfig: Option<FlexItemHighlightConfig<'a>>,
    /// The contrast algorithm to use for the contrast ratio (default: aa).
    #[serde(skip_serializing_if = "Option::is_none")]
    contrastAlgorithm: Option<ContrastAlgorithm>,
    /// The container query container highlight configuration (default: all transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    containerQueryContainerHighlightConfig: Option<ContainerQueryContainerHighlightConfig<'a>>,
}

impl<'a> HighlightConfig<'a> {
    pub fn builder() -> HighlightConfigBuilder<'a> {
        HighlightConfigBuilder {
            showInfo: None,
            showStyles: None,
            showRulers: None,
            showAccessibilityInfo: None,
            showExtensionLines: None,
            contentColor: None,
            paddingColor: None,
            borderColor: None,
            marginColor: None,
            eventTargetColor: None,
            shapeColor: None,
            shapeMarginColor: None,
            cssGridColor: None,
            colorFormat: None,
            gridHighlightConfig: None,
            flexContainerHighlightConfig: None,
            flexItemHighlightConfig: None,
            contrastAlgorithm: None,
            containerQueryContainerHighlightConfig: None,
        }
    }
    pub fn showInfo(&self) -> Option<bool> { self.showInfo }
    pub fn showStyles(&self) -> Option<bool> { self.showStyles }
    pub fn showRulers(&self) -> Option<bool> { self.showRulers }
    pub fn showAccessibilityInfo(&self) -> Option<bool> { self.showAccessibilityInfo }
    pub fn showExtensionLines(&self) -> Option<bool> { self.showExtensionLines }
    pub fn contentColor(&self) -> Option<&crate::dom::RGBA> { self.contentColor.as_ref() }
    pub fn paddingColor(&self) -> Option<&crate::dom::RGBA> { self.paddingColor.as_ref() }
    pub fn borderColor(&self) -> Option<&crate::dom::RGBA> { self.borderColor.as_ref() }
    pub fn marginColor(&self) -> Option<&crate::dom::RGBA> { self.marginColor.as_ref() }
    pub fn eventTargetColor(&self) -> Option<&crate::dom::RGBA> { self.eventTargetColor.as_ref() }
    pub fn shapeColor(&self) -> Option<&crate::dom::RGBA> { self.shapeColor.as_ref() }
    pub fn shapeMarginColor(&self) -> Option<&crate::dom::RGBA> { self.shapeMarginColor.as_ref() }
    pub fn cssGridColor(&self) -> Option<&crate::dom::RGBA> { self.cssGridColor.as_ref() }
    pub fn colorFormat(&self) -> Option<&ColorFormat> { self.colorFormat.as_ref() }
    pub fn gridHighlightConfig(&self) -> Option<&GridHighlightConfig> { self.gridHighlightConfig.as_ref() }
    pub fn flexContainerHighlightConfig(&self) -> Option<&FlexContainerHighlightConfig<'a>> { self.flexContainerHighlightConfig.as_ref() }
    pub fn flexItemHighlightConfig(&self) -> Option<&FlexItemHighlightConfig<'a>> { self.flexItemHighlightConfig.as_ref() }
    pub fn contrastAlgorithm(&self) -> Option<&ContrastAlgorithm> { self.contrastAlgorithm.as_ref() }
    pub fn containerQueryContainerHighlightConfig(&self) -> Option<&ContainerQueryContainerHighlightConfig<'a>> { self.containerQueryContainerHighlightConfig.as_ref() }
}

#[derive(Default)]
pub struct HighlightConfigBuilder<'a> {
    showInfo: Option<bool>,
    showStyles: Option<bool>,
    showRulers: Option<bool>,
    showAccessibilityInfo: Option<bool>,
    showExtensionLines: Option<bool>,
    contentColor: Option<crate::dom::RGBA>,
    paddingColor: Option<crate::dom::RGBA>,
    borderColor: Option<crate::dom::RGBA>,
    marginColor: Option<crate::dom::RGBA>,
    eventTargetColor: Option<crate::dom::RGBA>,
    shapeColor: Option<crate::dom::RGBA>,
    shapeMarginColor: Option<crate::dom::RGBA>,
    cssGridColor: Option<crate::dom::RGBA>,
    colorFormat: Option<ColorFormat>,
    gridHighlightConfig: Option<GridHighlightConfig>,
    flexContainerHighlightConfig: Option<FlexContainerHighlightConfig<'a>>,
    flexItemHighlightConfig: Option<FlexItemHighlightConfig<'a>>,
    contrastAlgorithm: Option<ContrastAlgorithm>,
    containerQueryContainerHighlightConfig: Option<ContainerQueryContainerHighlightConfig<'a>>,
}

impl<'a> HighlightConfigBuilder<'a> {
    /// Whether the node info tooltip should be shown (default: false).
    pub fn showInfo(mut self, showInfo: bool) -> Self { self.showInfo = Some(showInfo); self }
    /// Whether the node styles in the tooltip (default: false).
    pub fn showStyles(mut self, showStyles: bool) -> Self { self.showStyles = Some(showStyles); self }
    /// Whether the rulers should be shown (default: false).
    pub fn showRulers(mut self, showRulers: bool) -> Self { self.showRulers = Some(showRulers); self }
    /// Whether the a11y info should be shown (default: true).
    pub fn showAccessibilityInfo(mut self, showAccessibilityInfo: bool) -> Self { self.showAccessibilityInfo = Some(showAccessibilityInfo); self }
    /// Whether the extension lines from node to the rulers should be shown (default: false).
    pub fn showExtensionLines(mut self, showExtensionLines: bool) -> Self { self.showExtensionLines = Some(showExtensionLines); self }
    /// The content box highlight fill color (default: transparent).
    pub fn contentColor(mut self, contentColor: crate::dom::RGBA) -> Self { self.contentColor = Some(contentColor); self }
    /// The padding highlight fill color (default: transparent).
    pub fn paddingColor(mut self, paddingColor: crate::dom::RGBA) -> Self { self.paddingColor = Some(paddingColor); self }
    /// The border highlight fill color (default: transparent).
    pub fn borderColor(mut self, borderColor: crate::dom::RGBA) -> Self { self.borderColor = Some(borderColor); self }
    /// The margin highlight fill color (default: transparent).
    pub fn marginColor(mut self, marginColor: crate::dom::RGBA) -> Self { self.marginColor = Some(marginColor); self }
    /// The event target element highlight fill color (default: transparent).
    pub fn eventTargetColor(mut self, eventTargetColor: crate::dom::RGBA) -> Self { self.eventTargetColor = Some(eventTargetColor); self }
    /// The shape outside fill color (default: transparent).
    pub fn shapeColor(mut self, shapeColor: crate::dom::RGBA) -> Self { self.shapeColor = Some(shapeColor); self }
    /// The shape margin fill color (default: transparent).
    pub fn shapeMarginColor(mut self, shapeMarginColor: crate::dom::RGBA) -> Self { self.shapeMarginColor = Some(shapeMarginColor); self }
    /// The grid layout color (default: transparent).
    pub fn cssGridColor(mut self, cssGridColor: crate::dom::RGBA) -> Self { self.cssGridColor = Some(cssGridColor); self }
    /// The color format used to format color styles (default: hex).
    pub fn colorFormat(mut self, colorFormat: ColorFormat) -> Self { self.colorFormat = Some(colorFormat); self }
    /// The grid layout highlight configuration (default: all transparent).
    pub fn gridHighlightConfig(mut self, gridHighlightConfig: GridHighlightConfig) -> Self { self.gridHighlightConfig = Some(gridHighlightConfig); self }
    /// The flex container highlight configuration (default: all transparent).
    pub fn flexContainerHighlightConfig(mut self, flexContainerHighlightConfig: FlexContainerHighlightConfig<'a>) -> Self { self.flexContainerHighlightConfig = Some(flexContainerHighlightConfig); self }
    /// The flex item highlight configuration (default: all transparent).
    pub fn flexItemHighlightConfig(mut self, flexItemHighlightConfig: FlexItemHighlightConfig<'a>) -> Self { self.flexItemHighlightConfig = Some(flexItemHighlightConfig); self }
    /// The contrast algorithm to use for the contrast ratio (default: aa).
    pub fn contrastAlgorithm(mut self, contrastAlgorithm: ContrastAlgorithm) -> Self { self.contrastAlgorithm = Some(contrastAlgorithm); self }
    /// The container query container highlight configuration (default: all transparent).
    pub fn containerQueryContainerHighlightConfig(mut self, containerQueryContainerHighlightConfig: ContainerQueryContainerHighlightConfig<'a>) -> Self { self.containerQueryContainerHighlightConfig = Some(containerQueryContainerHighlightConfig); self }
    pub fn build(self) -> HighlightConfig<'a> {
        HighlightConfig {
            showInfo: self.showInfo,
            showStyles: self.showStyles,
            showRulers: self.showRulers,
            showAccessibilityInfo: self.showAccessibilityInfo,
            showExtensionLines: self.showExtensionLines,
            contentColor: self.contentColor,
            paddingColor: self.paddingColor,
            borderColor: self.borderColor,
            marginColor: self.marginColor,
            eventTargetColor: self.eventTargetColor,
            shapeColor: self.shapeColor,
            shapeMarginColor: self.shapeMarginColor,
            cssGridColor: self.cssGridColor,
            colorFormat: self.colorFormat,
            gridHighlightConfig: self.gridHighlightConfig,
            flexContainerHighlightConfig: self.flexContainerHighlightConfig,
            flexItemHighlightConfig: self.flexItemHighlightConfig,
            contrastAlgorithm: self.contrastAlgorithm,
            containerQueryContainerHighlightConfig: self.containerQueryContainerHighlightConfig,
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
    gridHighlightConfig: GridHighlightConfig,
    /// Identifier of the node to highlight.
    nodeId: crate::dom::NodeId,
}

impl GridNodeHighlightConfig {
    pub fn builder(gridHighlightConfig: GridHighlightConfig, nodeId: crate::dom::NodeId) -> GridNodeHighlightConfigBuilder {
        GridNodeHighlightConfigBuilder {
            gridHighlightConfig: gridHighlightConfig,
            nodeId: nodeId,
        }
    }
    pub fn gridHighlightConfig(&self) -> &GridHighlightConfig { &self.gridHighlightConfig }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}


pub struct GridNodeHighlightConfigBuilder {
    gridHighlightConfig: GridHighlightConfig,
    nodeId: crate::dom::NodeId,
}

impl GridNodeHighlightConfigBuilder {
    pub fn build(self) -> GridNodeHighlightConfig {
        GridNodeHighlightConfig {
            gridHighlightConfig: self.gridHighlightConfig,
            nodeId: self.nodeId,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexNodeHighlightConfig<'a> {
    /// A descriptor for the highlight appearance of flex containers.
    flexContainerHighlightConfig: FlexContainerHighlightConfig<'a>,
    /// Identifier of the node to highlight.
    nodeId: crate::dom::NodeId,
}

impl<'a> FlexNodeHighlightConfig<'a> {
    pub fn builder(flexContainerHighlightConfig: FlexContainerHighlightConfig<'a>, nodeId: crate::dom::NodeId) -> FlexNodeHighlightConfigBuilder<'a> {
        FlexNodeHighlightConfigBuilder {
            flexContainerHighlightConfig: flexContainerHighlightConfig,
            nodeId: nodeId,
        }
    }
    pub fn flexContainerHighlightConfig(&self) -> &FlexContainerHighlightConfig<'a> { &self.flexContainerHighlightConfig }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}


pub struct FlexNodeHighlightConfigBuilder<'a> {
    flexContainerHighlightConfig: FlexContainerHighlightConfig<'a>,
    nodeId: crate::dom::NodeId,
}

impl<'a> FlexNodeHighlightConfigBuilder<'a> {
    pub fn build(self) -> FlexNodeHighlightConfig<'a> {
        FlexNodeHighlightConfig {
            flexContainerHighlightConfig: self.flexContainerHighlightConfig,
            nodeId: self.nodeId,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapContainerHighlightConfig<'a> {
    /// The style of the snapport border (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    snapportBorder: Option<LineStyle<'a>>,
    /// The style of the snap area border (default: transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    snapAreaBorder: Option<LineStyle<'a>>,
    /// The margin highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollMarginColor: Option<crate::dom::RGBA>,
    /// The padding highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollPaddingColor: Option<crate::dom::RGBA>,
}

impl<'a> ScrollSnapContainerHighlightConfig<'a> {
    pub fn builder() -> ScrollSnapContainerHighlightConfigBuilder<'a> {
        ScrollSnapContainerHighlightConfigBuilder {
            snapportBorder: None,
            snapAreaBorder: None,
            scrollMarginColor: None,
            scrollPaddingColor: None,
        }
    }
    pub fn snapportBorder(&self) -> Option<&LineStyle<'a>> { self.snapportBorder.as_ref() }
    pub fn snapAreaBorder(&self) -> Option<&LineStyle<'a>> { self.snapAreaBorder.as_ref() }
    pub fn scrollMarginColor(&self) -> Option<&crate::dom::RGBA> { self.scrollMarginColor.as_ref() }
    pub fn scrollPaddingColor(&self) -> Option<&crate::dom::RGBA> { self.scrollPaddingColor.as_ref() }
}

#[derive(Default)]
pub struct ScrollSnapContainerHighlightConfigBuilder<'a> {
    snapportBorder: Option<LineStyle<'a>>,
    snapAreaBorder: Option<LineStyle<'a>>,
    scrollMarginColor: Option<crate::dom::RGBA>,
    scrollPaddingColor: Option<crate::dom::RGBA>,
}

impl<'a> ScrollSnapContainerHighlightConfigBuilder<'a> {
    /// The style of the snapport border (default: transparent)
    pub fn snapportBorder(mut self, snapportBorder: LineStyle<'a>) -> Self { self.snapportBorder = Some(snapportBorder); self }
    /// The style of the snap area border (default: transparent)
    pub fn snapAreaBorder(mut self, snapAreaBorder: LineStyle<'a>) -> Self { self.snapAreaBorder = Some(snapAreaBorder); self }
    /// The margin highlight fill color (default: transparent).
    pub fn scrollMarginColor(mut self, scrollMarginColor: crate::dom::RGBA) -> Self { self.scrollMarginColor = Some(scrollMarginColor); self }
    /// The padding highlight fill color (default: transparent).
    pub fn scrollPaddingColor(mut self, scrollPaddingColor: crate::dom::RGBA) -> Self { self.scrollPaddingColor = Some(scrollPaddingColor); self }
    pub fn build(self) -> ScrollSnapContainerHighlightConfig<'a> {
        ScrollSnapContainerHighlightConfig {
            snapportBorder: self.snapportBorder,
            snapAreaBorder: self.snapAreaBorder,
            scrollMarginColor: self.scrollMarginColor,
            scrollPaddingColor: self.scrollPaddingColor,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollSnapHighlightConfig<'a> {
    /// A descriptor for the highlight appearance of scroll snap containers.
    scrollSnapContainerHighlightConfig: ScrollSnapContainerHighlightConfig<'a>,
    /// Identifier of the node to highlight.
    nodeId: crate::dom::NodeId,
}

impl<'a> ScrollSnapHighlightConfig<'a> {
    pub fn builder(scrollSnapContainerHighlightConfig: ScrollSnapContainerHighlightConfig<'a>, nodeId: crate::dom::NodeId) -> ScrollSnapHighlightConfigBuilder<'a> {
        ScrollSnapHighlightConfigBuilder {
            scrollSnapContainerHighlightConfig: scrollSnapContainerHighlightConfig,
            nodeId: nodeId,
        }
    }
    pub fn scrollSnapContainerHighlightConfig(&self) -> &ScrollSnapContainerHighlightConfig<'a> { &self.scrollSnapContainerHighlightConfig }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}


pub struct ScrollSnapHighlightConfigBuilder<'a> {
    scrollSnapContainerHighlightConfig: ScrollSnapContainerHighlightConfig<'a>,
    nodeId: crate::dom::NodeId,
}

impl<'a> ScrollSnapHighlightConfigBuilder<'a> {
    pub fn build(self) -> ScrollSnapHighlightConfig<'a> {
        ScrollSnapHighlightConfig {
            scrollSnapContainerHighlightConfig: self.scrollSnapContainerHighlightConfig,
            nodeId: self.nodeId,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    contentColor: Option<crate::dom::RGBA>,
    /// The content box highlight outline color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    outlineColor: Option<crate::dom::RGBA>,
}

impl HingeConfig {
    pub fn builder(rect: crate::dom::Rect) -> HingeConfigBuilder {
        HingeConfigBuilder {
            rect: rect,
            contentColor: None,
            outlineColor: None,
        }
    }
    pub fn rect(&self) -> &crate::dom::Rect { &self.rect }
    pub fn contentColor(&self) -> Option<&crate::dom::RGBA> { self.contentColor.as_ref() }
    pub fn outlineColor(&self) -> Option<&crate::dom::RGBA> { self.outlineColor.as_ref() }
}


pub struct HingeConfigBuilder {
    rect: crate::dom::Rect,
    contentColor: Option<crate::dom::RGBA>,
    outlineColor: Option<crate::dom::RGBA>,
}

impl HingeConfigBuilder {
    /// The content box highlight fill color (default: a dark color).
    pub fn contentColor(mut self, contentColor: crate::dom::RGBA) -> Self { self.contentColor = Some(contentColor); self }
    /// The content box highlight outline color (default: transparent).
    pub fn outlineColor(mut self, outlineColor: crate::dom::RGBA) -> Self { self.outlineColor = Some(outlineColor); self }
    pub fn build(self) -> HingeConfig {
        HingeConfig {
            rect: self.rect,
            contentColor: self.contentColor,
            outlineColor: self.outlineColor,
        }
    }
}

/// Configuration for Window Controls Overlay

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowControlsOverlayConfig<'a> {
    /// Whether the title bar CSS should be shown when emulating the Window Controls Overlay.
    showCSS: bool,
    /// Selected platforms to show the overlay.
    selectedPlatform: Cow<'a, str>,
    /// The theme color defined in app manifest.
    themeColor: Cow<'a, str>,
}

impl<'a> WindowControlsOverlayConfig<'a> {
    pub fn builder(showCSS: bool, selectedPlatform: impl Into<Cow<'a, str>>, themeColor: impl Into<Cow<'a, str>>) -> WindowControlsOverlayConfigBuilder<'a> {
        WindowControlsOverlayConfigBuilder {
            showCSS: showCSS,
            selectedPlatform: selectedPlatform.into(),
            themeColor: themeColor.into(),
        }
    }
    pub fn showCSS(&self) -> bool { self.showCSS }
    pub fn selectedPlatform(&self) -> &str { self.selectedPlatform.as_ref() }
    pub fn themeColor(&self) -> &str { self.themeColor.as_ref() }
}


pub struct WindowControlsOverlayConfigBuilder<'a> {
    showCSS: bool,
    selectedPlatform: Cow<'a, str>,
    themeColor: Cow<'a, str>,
}

impl<'a> WindowControlsOverlayConfigBuilder<'a> {
    pub fn build(self) -> WindowControlsOverlayConfig<'a> {
        WindowControlsOverlayConfig {
            showCSS: self.showCSS,
            selectedPlatform: self.selectedPlatform,
            themeColor: self.themeColor,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryHighlightConfig<'a> {
    /// A descriptor for the highlight appearance of container query containers.
    containerQueryContainerHighlightConfig: ContainerQueryContainerHighlightConfig<'a>,
    /// Identifier of the container node to highlight.
    nodeId: crate::dom::NodeId,
}

impl<'a> ContainerQueryHighlightConfig<'a> {
    pub fn builder(containerQueryContainerHighlightConfig: ContainerQueryContainerHighlightConfig<'a>, nodeId: crate::dom::NodeId) -> ContainerQueryHighlightConfigBuilder<'a> {
        ContainerQueryHighlightConfigBuilder {
            containerQueryContainerHighlightConfig: containerQueryContainerHighlightConfig,
            nodeId: nodeId,
        }
    }
    pub fn containerQueryContainerHighlightConfig(&self) -> &ContainerQueryContainerHighlightConfig<'a> { &self.containerQueryContainerHighlightConfig }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}


pub struct ContainerQueryHighlightConfigBuilder<'a> {
    containerQueryContainerHighlightConfig: ContainerQueryContainerHighlightConfig<'a>,
    nodeId: crate::dom::NodeId,
}

impl<'a> ContainerQueryHighlightConfigBuilder<'a> {
    pub fn build(self) -> ContainerQueryHighlightConfig<'a> {
        ContainerQueryHighlightConfig {
            containerQueryContainerHighlightConfig: self.containerQueryContainerHighlightConfig,
            nodeId: self.nodeId,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerQueryContainerHighlightConfig<'a> {
    /// The style of the container border.
    #[serde(skip_serializing_if = "Option::is_none")]
    containerBorder: Option<LineStyle<'a>>,
    /// The style of the descendants' borders.
    #[serde(skip_serializing_if = "Option::is_none")]
    descendantBorder: Option<LineStyle<'a>>,
}

impl<'a> ContainerQueryContainerHighlightConfig<'a> {
    pub fn builder() -> ContainerQueryContainerHighlightConfigBuilder<'a> {
        ContainerQueryContainerHighlightConfigBuilder {
            containerBorder: None,
            descendantBorder: None,
        }
    }
    pub fn containerBorder(&self) -> Option<&LineStyle<'a>> { self.containerBorder.as_ref() }
    pub fn descendantBorder(&self) -> Option<&LineStyle<'a>> { self.descendantBorder.as_ref() }
}

#[derive(Default)]
pub struct ContainerQueryContainerHighlightConfigBuilder<'a> {
    containerBorder: Option<LineStyle<'a>>,
    descendantBorder: Option<LineStyle<'a>>,
}

impl<'a> ContainerQueryContainerHighlightConfigBuilder<'a> {
    /// The style of the container border.
    pub fn containerBorder(mut self, containerBorder: LineStyle<'a>) -> Self { self.containerBorder = Some(containerBorder); self }
    /// The style of the descendants' borders.
    pub fn descendantBorder(mut self, descendantBorder: LineStyle<'a>) -> Self { self.descendantBorder = Some(descendantBorder); self }
    pub fn build(self) -> ContainerQueryContainerHighlightConfig<'a> {
        ContainerQueryContainerHighlightConfig {
            containerBorder: self.containerBorder,
            descendantBorder: self.descendantBorder,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedElementHighlightConfig {
    /// A descriptor for the highlight appearance of an element in isolation mode.
    isolationModeHighlightConfig: IsolationModeHighlightConfig,
    /// Identifier of the isolated element to highlight.
    nodeId: crate::dom::NodeId,
}

impl IsolatedElementHighlightConfig {
    pub fn builder(isolationModeHighlightConfig: IsolationModeHighlightConfig, nodeId: crate::dom::NodeId) -> IsolatedElementHighlightConfigBuilder {
        IsolatedElementHighlightConfigBuilder {
            isolationModeHighlightConfig: isolationModeHighlightConfig,
            nodeId: nodeId,
        }
    }
    pub fn isolationModeHighlightConfig(&self) -> &IsolationModeHighlightConfig { &self.isolationModeHighlightConfig }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}


pub struct IsolatedElementHighlightConfigBuilder {
    isolationModeHighlightConfig: IsolationModeHighlightConfig,
    nodeId: crate::dom::NodeId,
}

impl IsolatedElementHighlightConfigBuilder {
    pub fn build(self) -> IsolatedElementHighlightConfig {
        IsolatedElementHighlightConfig {
            isolationModeHighlightConfig: self.isolationModeHighlightConfig,
            nodeId: self.nodeId,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IsolationModeHighlightConfig {
    /// The fill color of the resizers (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    resizerColor: Option<crate::dom::RGBA>,
    /// The fill color for resizer handles (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    resizerHandleColor: Option<crate::dom::RGBA>,
    /// The fill color for the mask covering non-isolated elements (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    maskColor: Option<crate::dom::RGBA>,
}

impl IsolationModeHighlightConfig {
    pub fn builder() -> IsolationModeHighlightConfigBuilder {
        IsolationModeHighlightConfigBuilder {
            resizerColor: None,
            resizerHandleColor: None,
            maskColor: None,
        }
    }
    pub fn resizerColor(&self) -> Option<&crate::dom::RGBA> { self.resizerColor.as_ref() }
    pub fn resizerHandleColor(&self) -> Option<&crate::dom::RGBA> { self.resizerHandleColor.as_ref() }
    pub fn maskColor(&self) -> Option<&crate::dom::RGBA> { self.maskColor.as_ref() }
}

#[derive(Default)]
pub struct IsolationModeHighlightConfigBuilder {
    resizerColor: Option<crate::dom::RGBA>,
    resizerHandleColor: Option<crate::dom::RGBA>,
    maskColor: Option<crate::dom::RGBA>,
}

impl IsolationModeHighlightConfigBuilder {
    /// The fill color of the resizers (default: transparent).
    pub fn resizerColor(mut self, resizerColor: crate::dom::RGBA) -> Self { self.resizerColor = Some(resizerColor); self }
    /// The fill color for resizer handles (default: transparent).
    pub fn resizerHandleColor(mut self, resizerHandleColor: crate::dom::RGBA) -> Self { self.resizerHandleColor = Some(resizerHandleColor); self }
    /// The fill color for the mask covering non-isolated elements (default: transparent).
    pub fn maskColor(mut self, maskColor: crate::dom::RGBA) -> Self { self.maskColor = Some(maskColor); self }
    pub fn build(self) -> IsolationModeHighlightConfig {
        IsolationModeHighlightConfig {
            resizerColor: self.resizerColor,
            resizerHandleColor: self.resizerHandleColor,
            maskColor: self.maskColor,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
}

impl InspectedElementAnchorConfig {
    pub fn builder() -> InspectedElementAnchorConfigBuilder {
        InspectedElementAnchorConfigBuilder {
            nodeId: None,
            backendNodeId: None,
        }
    }
    pub fn nodeId(&self) -> Option<&crate::dom::NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
}

#[derive(Default)]
pub struct InspectedElementAnchorConfigBuilder {
    nodeId: Option<crate::dom::NodeId>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
}

impl InspectedElementAnchorConfigBuilder {
    /// Identifier of the node to highlight.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node to highlight.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    pub fn build(self) -> InspectedElementAnchorConfig {
        InspectedElementAnchorConfig {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
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
    nodeId: crate::dom::NodeId,
    /// Whether to include distance info.
    #[serde(skip_serializing_if = "Option::is_none")]
    includeDistance: Option<bool>,
    /// Whether to include style info.
    #[serde(skip_serializing_if = "Option::is_none")]
    includeStyle: Option<bool>,
    /// The color format to get config with (default: hex).
    #[serde(skip_serializing_if = "Option::is_none")]
    colorFormat: Option<ColorFormat>,
    /// Whether to show accessibility info (default: true).
    #[serde(skip_serializing_if = "Option::is_none")]
    showAccessibilityInfo: Option<bool>,
}

impl GetHighlightObjectForTestParams {
    pub fn builder(nodeId: crate::dom::NodeId) -> GetHighlightObjectForTestParamsBuilder {
        GetHighlightObjectForTestParamsBuilder {
            nodeId: nodeId,
            includeDistance: None,
            includeStyle: None,
            colorFormat: None,
            showAccessibilityInfo: None,
        }
    }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
    pub fn includeDistance(&self) -> Option<bool> { self.includeDistance }
    pub fn includeStyle(&self) -> Option<bool> { self.includeStyle }
    pub fn colorFormat(&self) -> Option<&ColorFormat> { self.colorFormat.as_ref() }
    pub fn showAccessibilityInfo(&self) -> Option<bool> { self.showAccessibilityInfo }
}


pub struct GetHighlightObjectForTestParamsBuilder {
    nodeId: crate::dom::NodeId,
    includeDistance: Option<bool>,
    includeStyle: Option<bool>,
    colorFormat: Option<ColorFormat>,
    showAccessibilityInfo: Option<bool>,
}

impl GetHighlightObjectForTestParamsBuilder {
    /// Whether to include distance info.
    pub fn includeDistance(mut self, includeDistance: bool) -> Self { self.includeDistance = Some(includeDistance); self }
    /// Whether to include style info.
    pub fn includeStyle(mut self, includeStyle: bool) -> Self { self.includeStyle = Some(includeStyle); self }
    /// The color format to get config with (default: hex).
    pub fn colorFormat(mut self, colorFormat: ColorFormat) -> Self { self.colorFormat = Some(colorFormat); self }
    /// Whether to show accessibility info (default: true).
    pub fn showAccessibilityInfo(mut self, showAccessibilityInfo: bool) -> Self { self.showAccessibilityInfo = Some(showAccessibilityInfo); self }
    pub fn build(self) -> GetHighlightObjectForTestParams {
        GetHighlightObjectForTestParams {
            nodeId: self.nodeId,
            includeDistance: self.includeDistance,
            includeStyle: self.includeStyle,
            colorFormat: self.colorFormat,
            showAccessibilityInfo: self.showAccessibilityInfo,
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
    pub fn builder(highlight: serde_json::Map<String, JsonValue>) -> GetHighlightObjectForTestReturnsBuilder {
        GetHighlightObjectForTestReturnsBuilder {
            highlight: highlight,
        }
    }
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
    nodeIds: Vec<crate::dom::NodeId>,
}

impl GetGridHighlightObjectsForTestParams {
    pub fn builder(nodeIds: Vec<crate::dom::NodeId>) -> GetGridHighlightObjectsForTestParamsBuilder {
        GetGridHighlightObjectsForTestParamsBuilder {
            nodeIds: nodeIds,
        }
    }
    pub fn nodeIds(&self) -> &[crate::dom::NodeId] { &self.nodeIds }
}


pub struct GetGridHighlightObjectsForTestParamsBuilder {
    nodeIds: Vec<crate::dom::NodeId>,
}

impl GetGridHighlightObjectsForTestParamsBuilder {
    pub fn build(self) -> GetGridHighlightObjectsForTestParams {
        GetGridHighlightObjectsForTestParams {
            nodeIds: self.nodeIds,
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
    pub fn builder(highlights: serde_json::Map<String, JsonValue>) -> GetGridHighlightObjectsForTestReturnsBuilder {
        GetGridHighlightObjectsForTestReturnsBuilder {
            highlights: highlights,
        }
    }
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
    nodeId: crate::dom::NodeId,
}

impl GetSourceOrderHighlightObjectForTestParams {
    pub fn builder(nodeId: crate::dom::NodeId) -> GetSourceOrderHighlightObjectForTestParamsBuilder {
        GetSourceOrderHighlightObjectForTestParamsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}


pub struct GetSourceOrderHighlightObjectForTestParamsBuilder {
    nodeId: crate::dom::NodeId,
}

impl GetSourceOrderHighlightObjectForTestParamsBuilder {
    pub fn build(self) -> GetSourceOrderHighlightObjectForTestParams {
        GetSourceOrderHighlightObjectForTestParams {
            nodeId: self.nodeId,
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
    pub fn builder(highlight: serde_json::Map<String, JsonValue>) -> GetSourceOrderHighlightObjectForTestReturnsBuilder {
        GetSourceOrderHighlightObjectForTestReturnsBuilder {
            highlight: highlight,
        }
    }
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
    frameId: crate::page::FrameId<'a>,
    /// The content box highlight fill color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    contentColor: Option<crate::dom::RGBA>,
    /// The content box highlight outline color (default: transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    contentOutlineColor: Option<crate::dom::RGBA>,
}

impl<'a> HighlightFrameParams<'a> {
    pub fn builder(frameId: crate::page::FrameId<'a>) -> HighlightFrameParamsBuilder<'a> {
        HighlightFrameParamsBuilder {
            frameId: frameId,
            contentColor: None,
            contentOutlineColor: None,
        }
    }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
    pub fn contentColor(&self) -> Option<&crate::dom::RGBA> { self.contentColor.as_ref() }
    pub fn contentOutlineColor(&self) -> Option<&crate::dom::RGBA> { self.contentOutlineColor.as_ref() }
}


pub struct HighlightFrameParamsBuilder<'a> {
    frameId: crate::page::FrameId<'a>,
    contentColor: Option<crate::dom::RGBA>,
    contentOutlineColor: Option<crate::dom::RGBA>,
}

impl<'a> HighlightFrameParamsBuilder<'a> {
    /// The content box highlight fill color (default: transparent).
    pub fn contentColor(mut self, contentColor: crate::dom::RGBA) -> Self { self.contentColor = Some(contentColor); self }
    /// The content box highlight outline color (default: transparent).
    pub fn contentOutlineColor(mut self, contentOutlineColor: crate::dom::RGBA) -> Self { self.contentOutlineColor = Some(contentOutlineColor); self }
    pub fn build(self) -> HighlightFrameParams<'a> {
        HighlightFrameParams {
            frameId: self.frameId,
            contentColor: self.contentColor,
            contentOutlineColor: self.contentOutlineColor,
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
    highlightConfig: HighlightConfig<'a>,
    /// Identifier of the node to highlight.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node to be highlighted.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Selectors to highlight relevant nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    selector: Option<Cow<'a, str>>,
}

impl<'a> HighlightNodeParams<'a> {
    pub fn builder(highlightConfig: HighlightConfig<'a>) -> HighlightNodeParamsBuilder<'a> {
        HighlightNodeParamsBuilder {
            highlightConfig: highlightConfig,
            nodeId: None,
            backendNodeId: None,
            objectId: None,
            selector: None,
        }
    }
    pub fn highlightConfig(&self) -> &HighlightConfig<'a> { &self.highlightConfig }
    pub fn nodeId(&self) -> Option<&crate::dom::NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
    pub fn selector(&self) -> Option<&str> { self.selector.as_deref() }
}


pub struct HighlightNodeParamsBuilder<'a> {
    highlightConfig: HighlightConfig<'a>,
    nodeId: Option<crate::dom::NodeId>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    selector: Option<Cow<'a, str>>,
}

impl<'a> HighlightNodeParamsBuilder<'a> {
    /// Identifier of the node to highlight.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node to highlight.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node to be highlighted.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    /// Selectors to highlight relevant nodes.
    pub fn selector(mut self, selector: impl Into<Cow<'a, str>>) -> Self { self.selector = Some(selector.into()); self }
    pub fn build(self) -> HighlightNodeParams<'a> {
        HighlightNodeParams {
            highlightConfig: self.highlightConfig,
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    outlineColor: Option<crate::dom::RGBA>,
}

impl HighlightQuadParams {
    pub fn builder(quad: crate::dom::Quad) -> HighlightQuadParamsBuilder {
        HighlightQuadParamsBuilder {
            quad: quad,
            color: None,
            outlineColor: None,
        }
    }
    pub fn quad(&self) -> &crate::dom::Quad { &self.quad }
    pub fn color(&self) -> Option<&crate::dom::RGBA> { self.color.as_ref() }
    pub fn outlineColor(&self) -> Option<&crate::dom::RGBA> { self.outlineColor.as_ref() }
}


pub struct HighlightQuadParamsBuilder {
    quad: crate::dom::Quad,
    color: Option<crate::dom::RGBA>,
    outlineColor: Option<crate::dom::RGBA>,
}

impl HighlightQuadParamsBuilder {
    /// The highlight fill color (default: transparent).
    pub fn color(mut self, color: crate::dom::RGBA) -> Self { self.color = Some(color); self }
    /// The highlight outline color (default: transparent).
    pub fn outlineColor(mut self, outlineColor: crate::dom::RGBA) -> Self { self.outlineColor = Some(outlineColor); self }
    pub fn build(self) -> HighlightQuadParams {
        HighlightQuadParams {
            quad: self.quad,
            color: self.color,
            outlineColor: self.outlineColor,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    outlineColor: Option<crate::dom::RGBA>,
}

impl HighlightRectParams {
    pub fn builder(x: i32, y: i32, width: u64, height: i64) -> HighlightRectParamsBuilder {
        HighlightRectParamsBuilder {
            x: x,
            y: y,
            width: width,
            height: height,
            color: None,
            outlineColor: None,
        }
    }
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> i64 { self.height }
    pub fn color(&self) -> Option<&crate::dom::RGBA> { self.color.as_ref() }
    pub fn outlineColor(&self) -> Option<&crate::dom::RGBA> { self.outlineColor.as_ref() }
}


pub struct HighlightRectParamsBuilder {
    x: i32,
    y: i32,
    width: u64,
    height: i64,
    color: Option<crate::dom::RGBA>,
    outlineColor: Option<crate::dom::RGBA>,
}

impl HighlightRectParamsBuilder {
    /// The highlight fill color (default: transparent).
    pub fn color(mut self, color: crate::dom::RGBA) -> Self { self.color = Some(color); self }
    /// The highlight outline color (default: transparent).
    pub fn outlineColor(mut self, outlineColor: crate::dom::RGBA) -> Self { self.outlineColor = Some(outlineColor); self }
    pub fn build(self) -> HighlightRectParams {
        HighlightRectParams {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: self.color,
            outlineColor: self.outlineColor,
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
    sourceOrderConfig: SourceOrderConfig,
    /// Identifier of the node to highlight.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to highlight.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node to be highlighted.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> HighlightSourceOrderParams<'a> {
    pub fn builder(sourceOrderConfig: SourceOrderConfig) -> HighlightSourceOrderParamsBuilder<'a> {
        HighlightSourceOrderParamsBuilder {
            sourceOrderConfig: sourceOrderConfig,
            nodeId: None,
            backendNodeId: None,
            objectId: None,
        }
    }
    pub fn sourceOrderConfig(&self) -> &SourceOrderConfig { &self.sourceOrderConfig }
    pub fn nodeId(&self) -> Option<&crate::dom::NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
}


pub struct HighlightSourceOrderParamsBuilder<'a> {
    sourceOrderConfig: SourceOrderConfig,
    nodeId: Option<crate::dom::NodeId>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> HighlightSourceOrderParamsBuilder<'a> {
    /// Identifier of the node to highlight.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node to highlight.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node to be highlighted.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    pub fn build(self) -> HighlightSourceOrderParams<'a> {
        HighlightSourceOrderParams {
            sourceOrderConfig: self.sourceOrderConfig,
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    highlightConfig: Option<HighlightConfig<'a>>,
}

impl<'a> SetInspectModeParams<'a> {
    pub fn builder(mode: InspectMode) -> SetInspectModeParamsBuilder<'a> {
        SetInspectModeParamsBuilder {
            mode: mode,
            highlightConfig: None,
        }
    }
    pub fn mode(&self) -> &InspectMode { &self.mode }
    pub fn highlightConfig(&self) -> Option<&HighlightConfig<'a>> { self.highlightConfig.as_ref() }
}


pub struct SetInspectModeParamsBuilder<'a> {
    mode: InspectMode,
    highlightConfig: Option<HighlightConfig<'a>>,
}

impl<'a> SetInspectModeParamsBuilder<'a> {
    /// A descriptor for the highlight appearance of hovered-over nodes. May be omitted if 'enabled
    /// == false'.
    pub fn highlightConfig(mut self, highlightConfig: HighlightConfig<'a>) -> Self { self.highlightConfig = Some(highlightConfig); self }
    pub fn build(self) -> SetInspectModeParams<'a> {
        SetInspectModeParams {
            mode: self.mode,
            highlightConfig: self.highlightConfig,
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
    pub fn builder(show: bool) -> SetShowAdHighlightsParamsBuilder {
        SetShowAdHighlightsParamsBuilder {
            show: show,
        }
    }
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
    pub fn builder() -> SetPausedInDebuggerMessageParamsBuilder<'a> {
        SetPausedInDebuggerMessageParamsBuilder {
            message: None,
        }
    }
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
    pub fn builder(show: bool) -> SetShowDebugBordersParamsBuilder {
        SetShowDebugBordersParamsBuilder {
            show: show,
        }
    }
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
    pub fn builder(show: bool) -> SetShowFPSCounterParamsBuilder {
        SetShowFPSCounterParamsBuilder {
            show: show,
        }
    }
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
    gridNodeHighlightConfigs: Vec<GridNodeHighlightConfig>,
}

impl SetShowGridOverlaysParams {
    pub fn builder(gridNodeHighlightConfigs: Vec<GridNodeHighlightConfig>) -> SetShowGridOverlaysParamsBuilder {
        SetShowGridOverlaysParamsBuilder {
            gridNodeHighlightConfigs: gridNodeHighlightConfigs,
        }
    }
    pub fn gridNodeHighlightConfigs(&self) -> &[GridNodeHighlightConfig] { &self.gridNodeHighlightConfigs }
}


pub struct SetShowGridOverlaysParamsBuilder {
    gridNodeHighlightConfigs: Vec<GridNodeHighlightConfig>,
}

impl SetShowGridOverlaysParamsBuilder {
    pub fn build(self) -> SetShowGridOverlaysParams {
        SetShowGridOverlaysParams {
            gridNodeHighlightConfigs: self.gridNodeHighlightConfigs,
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
    flexNodeHighlightConfigs: Vec<FlexNodeHighlightConfig<'a>>,
}

impl<'a> SetShowFlexOverlaysParams<'a> {
    pub fn builder(flexNodeHighlightConfigs: Vec<FlexNodeHighlightConfig<'a>>) -> SetShowFlexOverlaysParamsBuilder<'a> {
        SetShowFlexOverlaysParamsBuilder {
            flexNodeHighlightConfigs: flexNodeHighlightConfigs,
        }
    }
    pub fn flexNodeHighlightConfigs(&self) -> &[FlexNodeHighlightConfig<'a>] { &self.flexNodeHighlightConfigs }
}


pub struct SetShowFlexOverlaysParamsBuilder<'a> {
    flexNodeHighlightConfigs: Vec<FlexNodeHighlightConfig<'a>>,
}

impl<'a> SetShowFlexOverlaysParamsBuilder<'a> {
    pub fn build(self) -> SetShowFlexOverlaysParams<'a> {
        SetShowFlexOverlaysParams {
            flexNodeHighlightConfigs: self.flexNodeHighlightConfigs,
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
    scrollSnapHighlightConfigs: Vec<ScrollSnapHighlightConfig<'a>>,
}

impl<'a> SetShowScrollSnapOverlaysParams<'a> {
    pub fn builder(scrollSnapHighlightConfigs: Vec<ScrollSnapHighlightConfig<'a>>) -> SetShowScrollSnapOverlaysParamsBuilder<'a> {
        SetShowScrollSnapOverlaysParamsBuilder {
            scrollSnapHighlightConfigs: scrollSnapHighlightConfigs,
        }
    }
    pub fn scrollSnapHighlightConfigs(&self) -> &[ScrollSnapHighlightConfig<'a>] { &self.scrollSnapHighlightConfigs }
}


pub struct SetShowScrollSnapOverlaysParamsBuilder<'a> {
    scrollSnapHighlightConfigs: Vec<ScrollSnapHighlightConfig<'a>>,
}

impl<'a> SetShowScrollSnapOverlaysParamsBuilder<'a> {
    pub fn build(self) -> SetShowScrollSnapOverlaysParams<'a> {
        SetShowScrollSnapOverlaysParams {
            scrollSnapHighlightConfigs: self.scrollSnapHighlightConfigs,
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
    containerQueryHighlightConfigs: Vec<ContainerQueryHighlightConfig<'a>>,
}

impl<'a> SetShowContainerQueryOverlaysParams<'a> {
    pub fn builder(containerQueryHighlightConfigs: Vec<ContainerQueryHighlightConfig<'a>>) -> SetShowContainerQueryOverlaysParamsBuilder<'a> {
        SetShowContainerQueryOverlaysParamsBuilder {
            containerQueryHighlightConfigs: containerQueryHighlightConfigs,
        }
    }
    pub fn containerQueryHighlightConfigs(&self) -> &[ContainerQueryHighlightConfig<'a>] { &self.containerQueryHighlightConfigs }
}


pub struct SetShowContainerQueryOverlaysParamsBuilder<'a> {
    containerQueryHighlightConfigs: Vec<ContainerQueryHighlightConfig<'a>>,
}

impl<'a> SetShowContainerQueryOverlaysParamsBuilder<'a> {
    pub fn build(self) -> SetShowContainerQueryOverlaysParams<'a> {
        SetShowContainerQueryOverlaysParams {
            containerQueryHighlightConfigs: self.containerQueryHighlightConfigs,
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
    inspectedElementAnchorConfig: InspectedElementAnchorConfig,
}

impl SetShowInspectedElementAnchorParams {
    pub fn builder(inspectedElementAnchorConfig: InspectedElementAnchorConfig) -> SetShowInspectedElementAnchorParamsBuilder {
        SetShowInspectedElementAnchorParamsBuilder {
            inspectedElementAnchorConfig: inspectedElementAnchorConfig,
        }
    }
    pub fn inspectedElementAnchorConfig(&self) -> &InspectedElementAnchorConfig { &self.inspectedElementAnchorConfig }
}


pub struct SetShowInspectedElementAnchorParamsBuilder {
    inspectedElementAnchorConfig: InspectedElementAnchorConfig,
}

impl SetShowInspectedElementAnchorParamsBuilder {
    pub fn build(self) -> SetShowInspectedElementAnchorParams {
        SetShowInspectedElementAnchorParams {
            inspectedElementAnchorConfig: self.inspectedElementAnchorConfig,
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
    pub fn builder(result: bool) -> SetShowPaintRectsParamsBuilder {
        SetShowPaintRectsParamsBuilder {
            result: result,
        }
    }
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
    pub fn builder(result: bool) -> SetShowLayoutShiftRegionsParamsBuilder {
        SetShowLayoutShiftRegionsParamsBuilder {
            result: result,
        }
    }
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
    pub fn builder(show: bool) -> SetShowScrollBottleneckRectsParamsBuilder {
        SetShowScrollBottleneckRectsParamsBuilder {
            show: show,
        }
    }
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
    pub fn builder(show: bool) -> SetShowHitTestBordersParamsBuilder {
        SetShowHitTestBordersParamsBuilder {
            show: show,
        }
    }
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
    pub fn builder(show: bool) -> SetShowViewportSizeOnResizeParamsBuilder {
        SetShowViewportSizeOnResizeParamsBuilder {
            show: show,
        }
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    hingeConfig: Option<HingeConfig>,
}

impl SetShowHingeParams {
    pub fn builder() -> SetShowHingeParamsBuilder {
        SetShowHingeParamsBuilder {
            hingeConfig: None,
        }
    }
    pub fn hingeConfig(&self) -> Option<&HingeConfig> { self.hingeConfig.as_ref() }
}

#[derive(Default)]
pub struct SetShowHingeParamsBuilder {
    hingeConfig: Option<HingeConfig>,
}

impl SetShowHingeParamsBuilder {
    /// hinge data, null means hideHinge
    pub fn hingeConfig(mut self, hingeConfig: HingeConfig) -> Self { self.hingeConfig = Some(hingeConfig); self }
    pub fn build(self) -> SetShowHingeParams {
        SetShowHingeParams {
            hingeConfig: self.hingeConfig,
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
    isolatedElementHighlightConfigs: Vec<IsolatedElementHighlightConfig>,
}

impl SetShowIsolatedElementsParams {
    pub fn builder(isolatedElementHighlightConfigs: Vec<IsolatedElementHighlightConfig>) -> SetShowIsolatedElementsParamsBuilder {
        SetShowIsolatedElementsParamsBuilder {
            isolatedElementHighlightConfigs: isolatedElementHighlightConfigs,
        }
    }
    pub fn isolatedElementHighlightConfigs(&self) -> &[IsolatedElementHighlightConfig] { &self.isolatedElementHighlightConfigs }
}


pub struct SetShowIsolatedElementsParamsBuilder {
    isolatedElementHighlightConfigs: Vec<IsolatedElementHighlightConfig>,
}

impl SetShowIsolatedElementsParamsBuilder {
    pub fn build(self) -> SetShowIsolatedElementsParams {
        SetShowIsolatedElementsParams {
            isolatedElementHighlightConfigs: self.isolatedElementHighlightConfigs,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    windowControlsOverlayConfig: Option<WindowControlsOverlayConfig<'a>>,
}

impl<'a> SetShowWindowControlsOverlayParams<'a> {
    pub fn builder() -> SetShowWindowControlsOverlayParamsBuilder<'a> {
        SetShowWindowControlsOverlayParamsBuilder {
            windowControlsOverlayConfig: None,
        }
    }
    pub fn windowControlsOverlayConfig(&self) -> Option<&WindowControlsOverlayConfig<'a>> { self.windowControlsOverlayConfig.as_ref() }
}

#[derive(Default)]
pub struct SetShowWindowControlsOverlayParamsBuilder<'a> {
    windowControlsOverlayConfig: Option<WindowControlsOverlayConfig<'a>>,
}

impl<'a> SetShowWindowControlsOverlayParamsBuilder<'a> {
    /// Window Controls Overlay data, null means hide Window Controls Overlay
    pub fn windowControlsOverlayConfig(mut self, windowControlsOverlayConfig: WindowControlsOverlayConfig<'a>) -> Self { self.windowControlsOverlayConfig = Some(windowControlsOverlayConfig); self }
    pub fn build(self) -> SetShowWindowControlsOverlayParams<'a> {
        SetShowWindowControlsOverlayParams {
            windowControlsOverlayConfig: self.windowControlsOverlayConfig,
        }
    }
}

impl<'a> SetShowWindowControlsOverlayParams<'a> { pub const METHOD: &'static str = "Overlay.setShowWindowControlsOverlay"; }

impl<'a> crate::CdpCommand<'a> for SetShowWindowControlsOverlayParams<'a> {
    const METHOD: &'static str = "Overlay.setShowWindowControlsOverlay";
    type Response = crate::EmptyReturns;
}
