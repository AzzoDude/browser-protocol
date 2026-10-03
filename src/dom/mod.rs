//! This domain exposes DOM read/write operations. Each DOM Node is represented with its mirror object
//! that has an 'id'. This 'id' can be used to get additional information on the Node, resolve it into
//! the JavaScript object wrapper, etc. It is important that client receives DOM events only for the
//! nodes that are known to the client. Backend keeps track of the nodes that were sent to the client
//! and never sends the same node twice. It is client's responsibility to collect information about
//! the nodes that were sent to the client. Note that 'iframe' owner elements will return
//! corresponding document elements as their child nodes.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Unique DOM node identifier.

pub type NodeId = i64;

/// Unique DOM node identifier used to reference a node that may not have been pushed to the
/// front-end.

pub type BackendNodeId = i64;

/// Unique identifier for a CSS stylesheet.

pub type StyleSheetId<'a> = Cow<'a, str>;

/// Backend node with a friendly name.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BackendNode<'a> {
    /// 'Node''s nodeType.
    #[serde(rename = "nodeType")]
    pub node_type: i64,
    /// 'Node''s nodeName.
    #[serde(rename = "nodeName")]
    pub node_name: Cow<'a, str>,
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
}
/// Pseudo element type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PseudoType {
    #[default]
    #[serde(rename = "first-line")]
    FirstLine,
    #[serde(rename = "first-letter")]
    FirstLetter,
    #[serde(rename = "checkmark")]
    Checkmark,
    #[serde(rename = "before")]
    Before,
    #[serde(rename = "after")]
    After,
    #[serde(rename = "expand-icon")]
    ExpandIcon,
    #[serde(rename = "picker-icon")]
    PickerIcon,
    #[serde(rename = "interest-button")]
    InterestButton,
    #[serde(rename = "marker")]
    Marker,
    #[serde(rename = "backdrop")]
    Backdrop,
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "selection")]
    Selection,
    #[serde(rename = "search-text")]
    SearchText,
    #[serde(rename = "target-text")]
    TargetText,
    #[serde(rename = "spelling-error")]
    SpellingError,
    #[serde(rename = "grammar-error")]
    GrammarError,
    #[serde(rename = "highlight")]
    Highlight,
    #[serde(rename = "first-line-inherited")]
    FirstLineInherited,
    #[serde(rename = "scroll-marker")]
    ScrollMarker,
    #[serde(rename = "scroll-marker-group")]
    ScrollMarkerGroup,
    #[serde(rename = "scroll-button")]
    ScrollButton,
    #[serde(rename = "scrollbar")]
    Scrollbar,
    #[serde(rename = "scrollbar-thumb")]
    ScrollbarThumb,
    #[serde(rename = "scrollbar-button")]
    ScrollbarButton,
    #[serde(rename = "scrollbar-track")]
    ScrollbarTrack,
    #[serde(rename = "scrollbar-track-piece")]
    ScrollbarTrackPiece,
    #[serde(rename = "scrollbar-corner")]
    ScrollbarCorner,
    #[serde(rename = "resizer")]
    Resizer,
    #[serde(rename = "input-list-button")]
    InputListButton,
    #[serde(rename = "view-transition")]
    ViewTransition,
    #[serde(rename = "view-transition-group")]
    ViewTransitionGroup,
    #[serde(rename = "view-transition-image-pair")]
    ViewTransitionImagePair,
    #[serde(rename = "view-transition-group-children")]
    ViewTransitionGroupChildren,
    #[serde(rename = "view-transition-old")]
    ViewTransitionOld,
    #[serde(rename = "view-transition-new")]
    ViewTransitionNew,
    #[serde(rename = "placeholder")]
    Placeholder,
    #[serde(rename = "file-selector-button")]
    FileSelectorButton,
    #[serde(rename = "details-content")]
    DetailsContent,
    #[serde(rename = "picker")]
    Picker,
    #[serde(rename = "select-listbox")]
    SelectListbox,
    #[serde(rename = "permission-icon")]
    PermissionIcon,
    #[serde(rename = "overscroll-area-parent")]
    OverscrollAreaParent,
    #[serde(rename = "overscroll-backdrop")]
    OverscrollBackdrop,
    #[serde(rename = "skeleton")]
    Skeleton,
}

/// Shadow root type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ShadowRootType {
    #[default]
    #[serde(rename = "user-agent")]
    UserAgent,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

/// Document compatibility mode.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CompatibilityMode {
    #[default]
    #[serde(rename = "QuirksMode")]
    QuirksMode,
    #[serde(rename = "LimitedQuirksMode")]
    LimitedQuirksMode,
    #[serde(rename = "NoQuirksMode")]
    NoQuirksMode,
}

/// ContainerSelector physical axes

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PhysicalAxes {
    #[default]
    #[serde(rename = "Horizontal")]
    Horizontal,
    #[serde(rename = "Vertical")]
    Vertical,
    #[serde(rename = "Both")]
    Both,
}

/// ContainerSelector logical axes

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum LogicalAxes {
    #[default]
    #[serde(rename = "Inline")]
    Inline,
    #[serde(rename = "Block")]
    Block,
    #[serde(rename = "Both")]
    Both,
}

/// Physical scroll orientation

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ScrollOrientation {
    #[default]
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}

/// DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
/// DOMNode is a base node mirror type.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Node<'a> {
    /// Node identifier that is passed into the rest of the DOM messages as the 'nodeId'. Backend
    /// will only push node with given 'id' once. It is aware of all requested nodes and will only
    /// fire DOM events for nodes known to the client.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// The id of the parent node if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    pub parent_id: Option<NodeId>,
    /// The BackendNodeId for this node.
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
    /// 'Node''s nodeType.
    #[serde(rename = "nodeType")]
    pub node_type: i64,
    /// 'Node''s nodeName.
    #[serde(rename = "nodeName")]
    pub node_name: Cow<'a, str>,
    /// 'Node''s localName.
    #[serde(rename = "localName")]
    pub local_name: Cow<'a, str>,
    /// 'Node''s nodeValue.
    #[serde(rename = "nodeValue")]
    pub node_value: Cow<'a, str>,
    /// Child count for 'Container' nodes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childNodeCount")]
    pub child_node_count: Option<u64>,
    /// Child nodes of this node when requested with children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Box<Node<'a>>>>,
    /// Attributes of the 'Element' node in the form of flat array '\[name1, value1, name2, value2\]'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Cow<'a, str>>>,
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    #[serde(skip_serializing_if = "Option::is_none", rename = "documentURL")]
    pub document_url: Option<Cow<'a, str>>,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseURL")]
    pub base_url: Option<Cow<'a, str>>,
    /// 'DocumentType''s publicId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "publicId")]
    pub public_id: Option<Cow<'a, str>>,
    /// 'DocumentType''s systemId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "systemId")]
    pub system_id: Option<Cow<'a, str>>,
    /// 'DocumentType''s internalSubset.
    #[serde(skip_serializing_if = "Option::is_none", rename = "internalSubset")]
    pub internal_subset: Option<Cow<'a, str>>,
    /// 'Document''s XML version in case of XML documents.
    #[serde(skip_serializing_if = "Option::is_none", rename = "xmlVersion")]
    pub xml_version: Option<Cow<'a, str>>,
    /// 'Attr''s name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    /// 'Attr''s value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Cow<'a, str>>,
    /// Pseudo element type for this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoType")]
    pub pseudo_type: Option<PseudoType>,
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoIdentifier")]
    pub pseudo_identifier: Option<Cow<'a, str>>,
    /// Shadow root type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "shadowRootType")]
    pub shadow_root_type: Option<ShadowRootType>,
    /// Frame ID for frame owner elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
    /// Content document for frame owner elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentDocument")]
    pub content_document: Option<Box<Node<'a>>>,
    /// Shadow root list for given element host.
    #[serde(skip_serializing_if = "Option::is_none", rename = "shadowRoots")]
    pub shadow_roots: Option<Vec<Box<Node<'a>>>>,
    /// Content document fragment for template elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "templateContent")]
    pub template_content: Option<Box<Node<'a>>>,
    /// Pseudo elements associated with this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoElements")]
    pub pseudo_elements: Option<Vec<Box<Node<'a>>>>,
    /// Deprecated, as the HTML Imports API has been removed (crbug.com/937746).
    /// This property used to return the imported document for the HTMLImport links.
    /// The property is always undefined now.
    #[serde(skip_serializing_if = "Option::is_none", rename = "importedDocument")]
    pub imported_document: Option<Box<Node<'a>>>,
    /// Distributed nodes for given insertion point.
    #[serde(skip_serializing_if = "Option::is_none", rename = "distributedNodes")]
    pub distributed_nodes: Option<Vec<BackendNode<'a>>>,
    /// Whether the node is SVG.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isSVG")]
    pub is_svg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "compatibilityMode")]
    pub compatibility_mode: Option<CompatibilityMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "assignedSlot")]
    pub assigned_slot: Option<BackendNode<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isScrollable")]
    pub is_scrollable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "affectedByStartingStyles")]
    pub affected_by_starting_styles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "adoptedStyleSheets")]
    pub adopted_style_sheets: Option<Vec<StyleSheetId<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "adProvenance")]
    pub ad_provenance: Option<crate::network::AdProvenance<'a>>,
}
/// A structure to hold the top-level node of a detached tree and an array of its retained descendants.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DetachedElementInfo<'a> {
    #[serde(rename = "treeNode")]
    pub tree_node: Node<'a>,
    #[serde(rename = "retainedNodeIds")]
    pub retained_node_ids: Vec<NodeId>,
}
/// A structure holding an RGBA color.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RGBA {
    /// The red component, in the \[0-255\] range.
    pub r: i64,
    /// The green component, in the \[0-255\] range.
    pub g: i64,
    /// The blue component, in the \[0-255\] range.
    pub b: i64,
    /// The alpha component, in the \[0-1\] range (default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,
}
/// An array of quad vertices, x immediately followed by y for each point, points clock-wise.

pub type Quad = Vec<f64>;

/// Box model.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BoxModel {
    /// Content box
    pub content: Quad,
    /// Padding box
    pub padding: Quad,
    /// Border box
    pub border: Quad,
    /// Margin box
    pub margin: Quad,
    /// Node width
    pub width: u64,
    /// Node height
    pub height: i64,
    /// Shape outside coordinates
    #[serde(skip_serializing_if = "Option::is_none", rename = "shapeOutside")]
    pub shape_outside: Option<ShapeOutsideInfo>,
}
/// CSS Shape Outside details.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ShapeOutsideInfo {
    /// Shape bounds
    pub bounds: Quad,
    /// Shape coordinate details
    pub shape: Vec<JsonValue>,
    /// Margin shape bounds
    #[serde(rename = "marginShape")]
    pub margin_shape: Vec<JsonValue>,
}
/// Rectangle.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Rect {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Rectangle width
    pub width: f64,
    /// Rectangle height
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSComputedStyleProperty<'a> {
    /// Computed style property name.
    pub name: Cow<'a, str>,
    /// Computed style property value.
    pub value: Cow<'a, str>,
}
/// Collects class names for the node with given id and all of it's child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.collectClassNamesFromSubtree", response = "CollectClassNamesFromSubtreeReturns<'a>")]
pub struct CollectClassNamesFromSubtreeParams {
    /// Id of the node to collect class names.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Collects class names for the node with given id and all of it's child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesFromSubtreeReturns<'a> {
    /// Class name list.
    #[serde(rename = "classNames")]
    pub class_names: Vec<Cow<'a, str>>,
}
/// Creates a deep copy of the specified node and places it into the target container before the
/// given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.copyTo", response = "CopyToReturns")]
pub struct CopyToParams {
    /// Id of the node to copy.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Id of the element to drop the copy into.
    #[serde(rename = "targetNodeId")]
    pub target_node_id: NodeId,
    /// Drop the copy before this node (if absent, the copy becomes the last child of
    /// 'targetNodeId').
    #[serde(skip_serializing_if = "Option::is_none", rename = "insertBeforeNodeId")]
    pub insert_before_node_id: Option<NodeId>,
}
/// Creates a deep copy of the specified node and places it into the target container before the
/// given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CopyToReturns {
    /// Id of the node clone.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Describes node given its id, does not require domain to be enabled. Does not start tracking any
/// objects, can be used for automation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.describeNode", response = "DescribeNodeReturns<'a>")]
pub struct DescribeNodeParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}
/// Describes node given its id, does not require domain to be enabled. Does not start tracking any
/// objects, can be used for automation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DescribeNodeReturns<'a> {
    /// Node description.
    pub node: Node<'a>,
}
/// Scrolls the specified rect of the given node into view if not already visible.
/// Note: exactly one between nodeId, backendNodeId and objectId should be passed
/// to identify the node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.scrollIntoViewIfNeeded")]
pub struct ScrollIntoViewIfNeededParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
    /// When omitted, center of the node will be used, similar to Element.scrollIntoView.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rect: Option<Rect>,
}
/// Disables DOM agent for the given page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.disable")]
pub struct DisableParams {

}
/// Discards search results from the session with the given id. 'getSearchResults' should no longer
/// be called for that search.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.discardSearchResults")]
pub struct DiscardSearchResultsParams<'a> {
    /// Unique search session identifier.
    #[serde(rename = "searchId")]
    pub search_id: Cow<'a, str>,
}
/// Enables DOM agent for the given page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.enable")]
pub struct EnableParams<'a> {
    /// Whether to include whitespaces in the children array of returned Nodes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeWhitespace")]
    pub include_whitespace: Option<Cow<'a, str>>,
}
/// Focuses the given element.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.focus")]
pub struct FocusParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}
/// Returns attributes for the specified node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getAttributes", response = "GetAttributesReturns<'a>")]
pub struct GetAttributesParams {
    /// Id of the node to retrieve attributes for.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Returns attributes for the specified node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAttributesReturns<'a> {
    /// An interleaved array of node attribute names and values.
    pub attributes: Vec<Cow<'a, str>>,
}
/// Returns boxes for the given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getBoxModel", response = "GetBoxModelReturns")]
pub struct GetBoxModelParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}
/// Returns boxes for the given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetBoxModelReturns {
    /// Box model for the node.
    pub model: BoxModel,
}
/// Returns quads that describe node position on the page. This method
/// might return multiple quads for inline nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getContentQuads", response = "GetContentQuadsReturns")]
pub struct GetContentQuadsParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}
/// Returns quads that describe node position on the page. This method
/// might return multiple quads for inline nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetContentQuadsReturns {
    /// Quads that describe node layout relative to viewport.
    pub quads: Vec<Quad>,
}
/// Returns the root DOM node (and optionally the subtree) to the caller.
/// Implicitly enables the DOM domain events for the current target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getDocument", response = "GetDocumentReturns<'a>")]
pub struct GetDocumentParams {
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}
/// Returns the root DOM node (and optionally the subtree) to the caller.
/// Implicitly enables the DOM domain events for the current target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetDocumentReturns<'a> {
    /// Resulting node.
    pub root: Node<'a>,
}
/// Returns the root DOM node (and optionally the subtree) to the caller.
/// Deprecated, as it is not designed to work well with the rest of the DOM agent.
/// Use DOMSnapshot.captureSnapshot instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getFlattenedDocument", response = "GetFlattenedDocumentReturns<'a>")]
pub struct GetFlattenedDocumentParams {
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}
/// Returns the root DOM node (and optionally the subtree) to the caller.
/// Deprecated, as it is not designed to work well with the rest of the DOM agent.
/// Use DOMSnapshot.captureSnapshot instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetFlattenedDocumentReturns<'a> {
    /// Resulting node.
    pub nodes: Vec<Node<'a>>,
}
/// Finds nodes with a given computed style in a subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getNodesForSubtreeByStyle", response = "GetNodesForSubtreeByStyleReturns")]
pub struct GetNodesForSubtreeByStyleParams<'a> {
    /// Node ID pointing to the root of a subtree.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// The style to filter nodes by (includes nodes if any of properties matches).
    #[serde(rename = "computedStyles")]
    pub computed_styles: Vec<CSSComputedStyleProperty<'a>>,
    /// Whether or not iframes and shadow roots in the same target should be traversed when returning the
    /// results (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}
/// Finds nodes with a given computed style in a subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetNodesForSubtreeByStyleReturns {
    /// Resulting nodes.
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
/// Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
/// either returned or not.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getNodeForLocation", response = "GetNodeForLocationReturns<'a>")]
pub struct GetNodeForLocationParams {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
    /// False to skip to the nearest non-UA shadow root ancestor (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeUserAgentShadowDOM")]
    pub include_user_agent_shadow_dom: Option<bool>,
    /// Whether to ignore pointer-events: none on elements and hit test them.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ignorePointerEventsNone")]
    pub ignore_pointer_events_none: Option<bool>,
}
/// Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
/// either returned or not.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeForLocationReturns<'a> {
    /// Resulting node.
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
    /// Frame this node belongs to.
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
    /// Id of the node at given coordinates, only when enabled and requested document.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
}
/// Returns node's HTML markup.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getOuterHTML", response = "GetOuterHTMLReturns<'a>")]
pub struct GetOuterHTMLParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Include all shadow roots. Equals to false if not specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeShadowDOM")]
    pub include_shadow_dom: Option<bool>,
}
/// Returns node's HTML markup.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetOuterHTMLReturns<'a> {
    /// Outer HTML markup.
    #[serde(rename = "outerHTML")]
    pub outer_html: Cow<'a, str>,
}
/// Returns the id of the nearest ancestor that is a relayout boundary.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getRelayoutBoundary", response = "GetRelayoutBoundaryReturns")]
pub struct GetRelayoutBoundaryParams {
    /// Id of the node.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Returns the id of the nearest ancestor that is a relayout boundary.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetRelayoutBoundaryReturns {
    /// Relayout boundary node id for the given node.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Returns search results from given 'fromIndex' to given 'toIndex' from the search with the given
/// identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getSearchResults", response = "GetSearchResultsReturns")]
pub struct GetSearchResultsParams<'a> {
    /// Unique search session identifier.
    #[serde(rename = "searchId")]
    pub search_id: Cow<'a, str>,
    /// Start index of the search result to be returned.
    #[serde(rename = "fromIndex")]
    pub from_index: u64,
    /// End index of the search result to be returned.
    #[serde(rename = "toIndex")]
    pub to_index: u64,
}
/// Returns search results from given 'fromIndex' to given 'toIndex' from the search with the given
/// identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetSearchResultsReturns {
    /// Ids of the search result nodes.
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
/// Hides any highlight.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.hideHighlight")]
pub struct HideHighlightParams {

}
/// Highlights DOM node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.highlightNode")]
pub struct HighlightNodeParams {

}
/// Highlights given rectangle.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.highlightRect")]
pub struct HighlightRectParams {

}
/// Marks last undoable state.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.markUndoableState")]
pub struct MarkUndoableStateParams {

}
/// Moves node into the new container, places it before the given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.moveTo", response = "MoveToReturns")]
pub struct MoveToParams {
    /// Id of the node to move.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Id of the element to drop the moved node into.
    #[serde(rename = "targetNodeId")]
    pub target_node_id: NodeId,
    /// Drop node before this one (if absent, the moved node becomes the last child of
    /// 'targetNodeId').
    #[serde(skip_serializing_if = "Option::is_none", rename = "insertBeforeNodeId")]
    pub insert_before_node_id: Option<NodeId>,
}
/// Moves node into the new container, places it before the given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MoveToReturns {
    /// New id of the moved node.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Searches for a given string in the DOM tree. Use 'getSearchResults' to access search results or
/// 'cancelSearch' to end this search session.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.performSearch", response = "PerformSearchReturns<'a>")]
pub struct PerformSearchParams<'a> {
    /// Plain text or query selector or XPath search query.
    pub query: Cow<'a, str>,
    /// True to search in user agent shadow DOM.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeUserAgentShadowDOM")]
    pub include_user_agent_shadow_dom: Option<bool>,
}
/// Searches for a given string in the DOM tree. Use 'getSearchResults' to access search results or
/// 'cancelSearch' to end this search session.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PerformSearchReturns<'a> {
    /// Unique search session identifier.
    #[serde(rename = "searchId")]
    pub search_id: Cow<'a, str>,
    /// Number of search results.
    #[serde(rename = "resultCount")]
    pub result_count: u64,
}
/// Requests that the node is sent to the caller given its path. // FIXME, use XPath

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.pushNodeByPathToFrontend", response = "PushNodeByPathToFrontendReturns")]
pub struct PushNodeByPathToFrontendParams<'a> {
    /// Path to node in the proprietary format.
    pub path: Cow<'a, str>,
}
/// Requests that the node is sent to the caller given its path. // FIXME, use XPath

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PushNodeByPathToFrontendReturns {
    /// Id of the node for given path.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Requests that a batch of nodes is sent to the caller given their backend node ids.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.pushNodesByBackendIdsToFrontend", response = "PushNodesByBackendIdsToFrontendReturns")]
pub struct PushNodesByBackendIdsToFrontendParams {
    /// The array of backend node ids.
    #[serde(rename = "backendNodeIds")]
    pub backend_node_ids: Vec<BackendNodeId>,
}
/// Requests that a batch of nodes is sent to the caller given their backend node ids.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PushNodesByBackendIdsToFrontendReturns {
    /// The array of ids of pushed nodes that correspond to the backend ids specified in
    /// backendNodeIds.
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
/// Executes 'querySelector' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.querySelector", response = "QuerySelectorReturns")]
pub struct QuerySelectorParams<'a> {
    /// Id of the node to query upon.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Selector string.
    pub selector: Cow<'a, str>,
}
/// Executes 'querySelector' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorReturns {
    /// Query selector result.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Executes 'querySelectorAll' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.querySelectorAll", response = "QuerySelectorAllReturns")]
pub struct QuerySelectorAllParams<'a> {
    /// Id of the node to query upon.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Selector string.
    pub selector: Cow<'a, str>,
}
/// Executes 'querySelectorAll' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorAllReturns {
    /// Query selector result.
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
/// Returns NodeIds of current top layer elements.
/// Top layer is rendered closest to the user within a viewport, therefore its elements always
/// appear on top of all other content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getTopLayerElements", response = "GetTopLayerElementsReturns")]
pub struct GetTopLayerElementsParams {

}
/// Returns NodeIds of current top layer elements.
/// Top layer is rendered closest to the user within a viewport, therefore its elements always
/// appear on top of all other content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetTopLayerElementsReturns {
    /// NodeIds of top layer elements
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
/// Returns the NodeId of the matched element according to certain relations.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getElementByRelation", response = "GetElementByRelationReturns")]
pub struct GetElementByRelationParams<'a> {
    /// Id of the node from which to query the relation.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Type of relation to get.
    pub relation: Cow<'a, str>,
}
/// Returns the NodeId of the matched element according to certain relations.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetElementByRelationReturns {
    /// NodeId of the element matching the queried relation.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Re-does the last undone action.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.redo")]
pub struct RedoParams {

}
/// Removes attribute with given name from an element with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.removeAttribute")]
pub struct RemoveAttributeParams<'a> {
    /// Id of the element to remove attribute from.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Name of the attribute to remove.
    pub name: Cow<'a, str>,
}
/// Removes node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.removeNode")]
pub struct RemoveNodeParams {
    /// Id of the node to remove.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Requests that children of the node with given id are returned to the caller in form of
/// 'setChildNodes' events where not only immediate children are retrieved, but all children down to
/// the specified depth.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.requestChildNodes")]
pub struct RequestChildNodesParams {
    /// Id of the node to get children for.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the sub-tree
    /// (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}
/// Requests that the node is sent to the caller given the JavaScript node object reference. All
/// nodes that form the path from the node to the root are also sent to the client as a series of
/// 'setChildNodes' notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.requestNode", response = "RequestNodeReturns")]
pub struct RequestNodeParams<'a> {
    /// JavaScript object id to convert into node.
    #[serde(rename = "objectId")]
    pub object_id: crate::runtime::RemoteObjectId<'a>,
}
/// Requests that the node is sent to the caller given the JavaScript node object reference. All
/// nodes that form the path from the node to the root are also sent to the client as a series of
/// 'setChildNodes' notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RequestNodeReturns {
    /// Node id for given object.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.resolveNode", response = "ResolveNodeReturns")]
pub struct ResolveNodeParams<'a> {
    /// Id of the node to resolve.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
    /// Backend identifier of the node to resolve.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    pub object_group: Option<Cow<'a, str>>,
    /// Execution context in which to resolve the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    pub execution_context_id: Option<crate::runtime::ExecutionContextId>,
}
/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ResolveNodeReturns {
    /// JavaScript object wrapper for given node.
    pub object: crate::runtime::RemoteObject,
}
/// Sets attribute for an element with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setAttributeValue")]
pub struct SetAttributeValueParams<'a> {
    /// Id of the element to set attribute for.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Attribute name.
    pub name: Cow<'a, str>,
    /// Attribute value.
    pub value: Cow<'a, str>,
}
/// Sets attributes on element with given id. This method is useful when user edits some existing
/// attribute value and types in several attribute name/value pairs.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setAttributesAsText")]
pub struct SetAttributesAsTextParams<'a> {
    /// Id of the element to set attributes for.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Text with a number of attributes. Will parse this text using HTML parser.
    pub text: Cow<'a, str>,
    /// Attribute name to replace with new attributes derived from text in case text parsed
    /// successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
}
/// Sets files for the given file input element.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setFileInputFiles")]
pub struct SetFileInputFilesParams<'a> {
    /// Array of file paths to set.
    pub files: Vec<Cow<'a, str>>,
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}
/// Sets if stack traces should be captured for Nodes. See 'Node.getNodeStackTraces'. Default is disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setNodeStackTracesEnabled")]
pub struct SetNodeStackTracesEnabledParams {
    /// Enable or disable.
    pub enable: bool,
}
/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getNodeStackTraces", response = "GetNodeStackTracesReturns")]
pub struct GetNodeStackTracesParams {
    /// Id of the node to get stack traces for.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeStackTracesReturns {
    /// Creation stack trace, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation: Option<crate::runtime::StackTrace>,
}
/// Returns file information for the given
/// File wrapper.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getFileInfo", response = "GetFileInfoReturns<'a>")]
pub struct GetFileInfoParams<'a> {
    /// JavaScript object id of the node wrapper.
    #[serde(rename = "objectId")]
    pub object_id: crate::runtime::RemoteObjectId<'a>,
}
/// Returns file information for the given
/// File wrapper.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetFileInfoReturns<'a> {
    pub path: Cow<'a, str>,
}
/// Returns list of detached nodes

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getDetachedDomNodes", response = "GetDetachedDomNodesReturns<'a>")]
pub struct GetDetachedDomNodesParams {

}
/// Returns list of detached nodes

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetDetachedDomNodesReturns<'a> {
    /// The list of detached nodes
    #[serde(rename = "detachedNodes")]
    pub detached_nodes: Vec<DetachedElementInfo<'a>>,
}
/// Enables console to refer to the node with given id via $x (see Command Line API for more details
/// $x functions).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setInspectedNode")]
pub struct SetInspectedNodeParams {
    /// DOM node id to be accessible by means of $x command line API.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Sets node name for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setNodeName", response = "SetNodeNameReturns")]
pub struct SetNodeNameParams<'a> {
    /// Id of the node to set name for.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// New node's name.
    pub name: Cow<'a, str>,
}
/// Sets node name for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeNameReturns {
    /// New node's id.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Sets node value for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setNodeValue")]
pub struct SetNodeValueParams<'a> {
    /// Id of the node to set value for.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// New node's value.
    pub value: Cow<'a, str>,
}
/// Sets node HTML markup, returns new node id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setOuterHTML")]
pub struct SetOuterHTMLParams<'a> {
    /// Id of the node to set markup for.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Outer HTML markup to set.
    #[serde(rename = "outerHTML")]
    pub outer_html: Cow<'a, str>,
}
/// Undoes the last performed action.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.undo")]
pub struct UndoParams {

}
/// Returns iframe node that owns iframe with the given domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getFrameOwner", response = "GetFrameOwnerReturns")]
pub struct GetFrameOwnerParams<'a> {
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
}
/// Returns iframe node that owns iframe with the given domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameOwnerReturns {
    /// Resulting node.
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
    /// Id of the node at given coordinates, only when enabled and requested document.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
}
/// Returns the query container of the given node based on container query
/// conditions: containerName, physical and logical axes, and whether it queries
/// scroll-state or anchored elements. If no axes are provided and
/// queriesScrollState is false, the style container is returned, which is the
/// direct parent or the closest element with a matching container-name.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getContainerForNode", response = "GetContainerForNodeReturns")]
pub struct GetContainerForNodeParams<'a> {
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "physicalAxes")]
    pub physical_axes: Option<PhysicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "logicalAxes")]
    pub logical_axes: Option<LogicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "queriesScrollState")]
    pub queries_scroll_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "queriesAnchored")]
    pub queries_anchored: Option<bool>,
}
/// Returns the query container of the given node based on container query
/// conditions: containerName, physical and logical axes, and whether it queries
/// scroll-state or anchored elements. If no axes are provided and
/// queriesScrollState is false, the style container is returned, which is the
/// direct parent or the closest element with a matching container-name.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetContainerForNodeReturns {
    /// The container node for the given node, or null if not found.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
}
/// Returns the descendants of a container query container that have
/// container queries against this container.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getQueryingDescendantsForContainer", response = "GetQueryingDescendantsForContainerReturns")]
pub struct GetQueryingDescendantsForContainerParams {
    /// Id of the container node to find querying descendants from.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Returns the descendants of a container query container that have
/// container queries against this container.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryingDescendantsForContainerReturns {
    /// Descendant nodes with container queries against the given container.
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
/// Returns the target anchor element of the given anchor query according to
/// <https://www.w3.org/TR/css-anchor-position-1/#target>.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getAnchorElement", response = "GetAnchorElementReturns")]
pub struct GetAnchorElementParams<'a> {
    /// Id of the positioned element from which to find the anchor.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// An optional anchor specifier, as defined in
    /// <https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier>.
    /// If not provided, it will return the implicit anchor element for
    /// the given positioned element.
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorSpecifier")]
    pub anchor_specifier: Option<Cow<'a, str>>,
}
/// Returns the target anchor element of the given anchor query according to
/// <https://www.w3.org/TR/css-anchor-position-1/#target>.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAnchorElementReturns {
    /// The anchor element of the given anchor query.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// When enabling, this API force-opens the popover identified by nodeId
/// and keeps it open until disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.forceShowPopover", response = "ForceShowPopoverReturns")]
pub struct ForceShowPopoverParams {
    /// Id of the popover HTMLElement
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// If true, opens the popover and keeps it open. If false, closes the
    /// popover if it was previously force-opened.
    pub enable: bool,
    /// Optional ID of the element invoking this popover, used to establish the implicit anchor.
    /// If not provided, it will fall back to the first invoker in the document, preferring
    /// elements with a popovertarget attribute over those with a commandfor attribute. Note that
    /// if there are multiple invokers, this is just an estimate.
    #[serde(skip_serializing_if = "Option::is_none", rename = "invokerNodeId")]
    pub invoker_node_id: Option<BackendNodeId>,
}
/// When enabling, this API force-opens the popover identified by nodeId
/// and keeps it open until disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ForceShowPopoverReturns {
    /// List of popovers that were closed in order to respect popover stacking order.
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
/// Returns candidate nodes that are configured as triggers for the given popover.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.getImplicitAnchorCandidates", response = "GetImplicitAnchorCandidatesReturns")]
pub struct GetImplicitAnchorCandidatesParams {
    /// Id of the popover HTMLElement.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Returns candidate nodes that are configured as triggers for the given popover.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetImplicitAnchorCandidatesReturns {
    /// Candidate elements that can invoke this popover.
    #[serde(rename = "backendNodeIds")]
    pub backend_node_ids: Vec<BackendNodeId>,
}
/// When enabling, this API forces an element to gain interest in its target,
/// keeping interest active until disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.forceShowInterest")]
pub struct ForceShowInterestParams {
    /// Id of the interest invoker HTMLElement.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// If true, opens and holds interest. If false, releases forced interest.
    pub enable: bool,
}
/// Sets a spelling or grammar error marker on the given range of text.
/// See <https://github.com/Igalia/explainers/blob/main/force-spelling-grammar-markers/README.md>
/// Note: exactly one between nodeId, backendNodeId and objectId should be passed
/// to identify the node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setTextMarker")]
pub struct SetTextMarkerParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// The type of marker to set on the given range of text.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Start offset into the element's rendered text in UTF-16 code units.
    /// For a text control, an offset into the control's value.
    /// Offsets count text in DOM order and do not enter shadow trees.
    /// To mark text inside a shadow tree, pass the element inside the shadow tree.
    pub start: i64,
    /// End offset (exclusive) in the same units and space as start.
    pub end: i64,
}
/// Clears the spelling and grammar error text markers overlapping the ranges
/// set by setTextMarker in this session. These markers are also removed when
/// the DOM domain is disabled or the session ends.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.clearTextMarkers")]
pub struct ClearTextMarkersParams {

}
/// Fired when 'Element''s attribute is modified.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.attributeModified")]
pub struct AttributeModified<'a> {
    /// Id of the node that has changed.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// Attribute name.
    pub name: Cow<'a, str>,
    /// Attribute value.
    pub value: Cow<'a, str>,
}
/// Fired when 'Element''s adoptedStyleSheets are modified.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.adoptedStyleSheetsModified")]
pub struct AdoptedStyleSheetsModified<'a> {
    /// Id of the node that has changed.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// New adoptedStyleSheets array.
    #[serde(rename = "adoptedStyleSheets")]
    pub adopted_style_sheets: Vec<StyleSheetId<'a>>,
}
/// Fired when 'Element''s attribute is removed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.attributeRemoved")]
pub struct AttributeRemoved<'a> {
    /// Id of the node that has changed.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// A ttribute name.
    pub name: Cow<'a, str>,
}
/// Mirrors 'DOMCharacterDataModified' event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.characterDataModified")]
pub struct CharacterDataModified<'a> {
    /// Id of the node that has changed.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// New text value.
    #[serde(rename = "characterData")]
    pub character_data: Cow<'a, str>,
}
/// Fired when 'Container''s child node count has changed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.childNodeCountUpdated")]
pub struct ChildNodeCountUpdated {
    /// Id of the node that has changed.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    /// New node count.
    #[serde(rename = "childNodeCount")]
    pub child_node_count: u64,
}
/// Mirrors 'DOMNodeInserted' event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.childNodeInserted")]
pub struct ChildNodeInserted<'a> {
    /// Id of the node that has changed.
    #[serde(rename = "parentNodeId")]
    pub parent_node_id: NodeId,
    /// Id of the previous sibling.
    #[serde(rename = "previousNodeId")]
    pub previous_node_id: NodeId,
    /// Inserted node data.
    pub node: Node<'a>,
}
/// Mirrors 'DOMNodeRemoved' event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.childNodeRemoved")]
pub struct ChildNodeRemoved {
    /// Parent id.
    #[serde(rename = "parentNodeId")]
    pub parent_node_id: NodeId,
    /// Id of the node that has been removed.
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
}
/// Called when distribution is changed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.distributedNodesUpdated")]
pub struct DistributedNodesUpdated<'a> {
    /// Insertion point where distributed nodes were updated.
    #[serde(rename = "insertionPointId")]
    pub insertion_point_id: NodeId,
    /// Distributed nodes for given insertion point.
    #[serde(rename = "distributedNodes")]
    pub distributed_nodes: Vec<BackendNode<'a>>,
}
/// Fired when 'Document' has been totally updated. Node ids are no longer valid.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.documentUpdated")]
pub struct DocumentUpdated {

}
/// Fired when 'Element''s inline style is modified via a CSS property modification.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.inlineStyleInvalidated")]
pub struct InlineStyleInvalidated {
    /// Ids of the nodes for which the inline styles have been invalidated.
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<NodeId>,
}
/// Called when a pseudo element is added to an element.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.pseudoElementAdded")]
pub struct PseudoElementAdded<'a> {
    /// Pseudo element's parent element id.
    #[serde(rename = "parentId")]
    pub parent_id: NodeId,
    /// The added pseudo element.
    #[serde(rename = "pseudoElement")]
    pub pseudo_element: Node<'a>,
}
/// Called when top layer elements are changed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.topLayerElementsUpdated")]
pub struct TopLayerElementsUpdated {

}
/// Fired when a node's scrollability state changes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.scrollableFlagUpdated")]
pub struct ScrollableFlagUpdated {
    /// The id of the node.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    /// If the node is scrollable.
    #[serde(rename = "isScrollable")]
    pub is_scrollable: bool,
}
/// Fired when a node's ad related state changes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.adRelatedStateUpdated")]
pub struct AdRelatedStateUpdated<'a> {
    /// The id of the node.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    /// The provenance of the ad related node, if it is ad related.
    #[serde(skip_serializing_if = "Option::is_none", rename = "adProvenance")]
    pub ad_provenance: Option<crate::network::AdProvenance<'a>>,
}
/// Fired when a node's starting styles changes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.affectedByStartingStylesFlagUpdated")]
pub struct AffectedByStartingStylesFlagUpdated {
    /// The id of the node.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    /// If the node has starting styles.
    #[serde(rename = "affectedByStartingStyles")]
    pub affected_by_starting_styles: bool,
}
/// Called when a pseudo element is removed from an element.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.pseudoElementRemoved")]
pub struct PseudoElementRemoved {
    /// Pseudo element's parent element id.
    #[serde(rename = "parentId")]
    pub parent_id: NodeId,
    /// The removed pseudo element id.
    #[serde(rename = "pseudoElementId")]
    pub pseudo_element_id: NodeId,
}
/// Fired when backend wants to provide client with the missing DOM structure. This happens upon
/// most of the calls requesting node ids.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.setChildNodes")]
pub struct SetChildNodes<'a> {
    /// Parent node id to populate with children.
    #[serde(rename = "parentId")]
    pub parent_id: NodeId,
    /// Child nodes array.
    pub nodes: Vec<Node<'a>>,
}
/// Called when shadow root is popped from the element.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.shadowRootPopped")]
pub struct ShadowRootPopped {
    /// Host element id.
    #[serde(rename = "hostId")]
    pub host_id: NodeId,
    /// Shadow root id.
    #[serde(rename = "rootId")]
    pub root_id: NodeId,
}
/// Called when shadow root is pushed into the element.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOM.shadowRootPushed")]
pub struct ShadowRootPushed<'a> {
    /// Host element id.
    #[serde(rename = "hostId")]
    pub host_id: NodeId,
    /// Shadow root.
    pub root: Node<'a>,
}