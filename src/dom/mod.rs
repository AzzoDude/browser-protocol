use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! This domain exposes DOM read/write operations. Each DOM Node is represented with its mirror object
//! that has an 'id'. This 'id' can be used to get additional information on the Node, resolve it into
//! the JavaScript object wrapper, etc. It is important that client receives DOM events only for the
//! nodes that are known to the client. Backend keeps track of the nodes that were sent to the client
//! and never sends the same node twice. It is client's responsibility to collect information about
//! the nodes that were sent to the client. Note that 'iframe' owner elements will return
//! corresponding document elements as their child nodes.

/// Unique DOM node identifier.

pub type NodeId = i64;

/// Unique DOM node identifier used to reference a node that may not have been pushed to the
/// front-end.

pub type BackendNodeId = i64;

/// Unique identifier for a CSS stylesheet.

pub type StyleSheetId = String;

/// Backend node with a friendly name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackendNode {
    /// 'Node''s nodeType.

    pub nodeType: i64,
    /// 'Node''s nodeName.

    pub nodeName: String,

    pub backendNodeId: BackendNodeId,
}

/// Pseudo element type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PseudoType {
    #[default]
    FirstLine,
    FirstLetter,
    Checkmark,
    Before,
    After,
    ExpandIcon,
    PickerIcon,
    InterestHint,
    Marker,
    Backdrop,
    Column,
    Selection,
    SearchText,
    TargetText,
    SpellingError,
    GrammarError,
    Highlight,
    FirstLineInherited,
    ScrollMarker,
    ScrollMarkerGroup,
    ScrollButton,
    Scrollbar,
    ScrollbarThumb,
    ScrollbarButton,
    ScrollbarTrack,
    ScrollbarTrackPiece,
    ScrollbarCorner,
    Resizer,
    InputListButton,
    ViewTransition,
    ViewTransitionGroup,
    ViewTransitionImagePair,
    ViewTransitionGroupChildren,
    ViewTransitionOld,
    ViewTransitionNew,
    Placeholder,
    FileSelectorButton,
    DetailsContent,
    Picker,
    PermissionIcon,
    OverscrollAreaParent,
}

/// Shadow root type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ShadowRootType {
    #[default]
    UserAgent,
    Open,
    Closed,
}

/// Document compatibility mode.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CompatibilityMode {
    #[default]
    QuirksMode,
    LimitedQuirksMode,
    NoQuirksMode,
}

/// ContainerSelector physical axes

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PhysicalAxes {
    #[default]
    Horizontal,
    Vertical,
    Both,
}

/// ContainerSelector logical axes

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum LogicalAxes {
    #[default]
    Inline,
    Block,
    Both,
}

/// Physical scroll orientation

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ScrollOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
/// DOMNode is a base node mirror type.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Node identifier that is passed into the rest of the DOM messages as the 'nodeId'. Backend
    /// will only push node with given 'id' once. It is aware of all requested nodes and will only
    /// fire DOM events for nodes known to the client.

    pub nodeId: NodeId,
    /// The id of the parent node if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentId: Option<NodeId>,
    /// The BackendNodeId for this node.

    pub backendNodeId: BackendNodeId,
    /// 'Node''s nodeType.

    pub nodeType: i64,
    /// 'Node''s nodeName.

    pub nodeName: String,
    /// 'Node''s localName.

    pub localName: String,
    /// 'Node''s nodeValue.

    pub nodeValue: String,
    /// Child count for 'Container' nodes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub childNodeCount: Option<u64>,
    /// Child nodes of this node when requested with children.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
    /// Attributes of the 'Element' node in the form of flat array '[name1, value1, name2, value2]'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    /// Document URL that 'Document' or 'FrameOwner' node points to.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentURL: Option<String>,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseURL: Option<String>,
    /// 'DocumentType''s publicId.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicId: Option<String>,
    /// 'DocumentType''s systemId.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub systemId: Option<String>,
    /// 'DocumentType''s internalSubset.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalSubset: Option<String>,
    /// 'Document''s XML version in case of XML documents.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xmlVersion: Option<String>,
    /// 'Attr''s name.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 'Attr''s value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Pseudo element type for this node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoType: Option<PseudoType>,
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoIdentifier: Option<String>,
    /// Shadow root type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadowRootType: Option<ShadowRootType>,
    /// Frame ID for frame owner elements.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
    /// Content document for frame owner elements.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentDocument: Option<Box<Node>>,
    /// Shadow root list for given element host.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadowRoots: Option<Vec<Node>>,
    /// Content document fragment for template elements.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub templateContent: Option<Box<Node>>,
    /// Pseudo elements associated with this node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoElements: Option<Vec<Node>>,
    /// Deprecated, as the HTML Imports API has been removed (crbug.com/937746).
    /// This property used to return the imported document for the HTMLImport links.
    /// The property is always undefined now.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub importedDocument: Option<Box<Node>>,
    /// Distributed nodes for given insertion point.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributedNodes: Option<Vec<BackendNode>>,
    /// Whether the node is SVG.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isSVG: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibilityMode: Option<CompatibilityMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignedSlot: Option<BackendNode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isScrollable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affectedByStartingStyles: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adoptedStyleSheets: Option<Vec<StyleSheetId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adProvenance: Option<crate::network::AdProvenance>,
}

/// A structure to hold the top-level node of a detached tree and an array of its retained descendants.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DetachedElementInfo {

    pub treeNode: Node,

    pub retainedNodeIds: Vec<NodeId>,
}

/// A structure holding an RGBA color.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RGBA {
    /// The red component, in the [0-255] range.

    pub r: i64,
    /// The green component, in the [0-255] range.

    pub g: i64,
    /// The blue component, in the [0-255] range.

    pub b: i64,
    /// The alpha component, in the [0-1] range (default: 1).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,
}

/// An array of quad vertices, x immediately followed by y for each point, points clock-wise.

pub type Quad = Vec<f64>;

/// Box model.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shapeOutside: Option<ShapeOutsideInfo>,
}

/// CSS Shape Outside details.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShapeOutsideInfo {
    /// Shape bounds

    pub bounds: Quad,
    /// Shape coordinate details

    pub shape: Vec<JsonValue>,
    /// Margin shape bounds

    pub marginShape: Vec<JsonValue>,
}

/// Rectangle.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSComputedStyleProperty {
    /// Computed style property name.

    pub name: String,
    /// Computed style property value.

    pub value: String,
}

/// Collects class names for the node with given id and all of it's child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesFromSubtreeParams {
    /// Id of the node to collect class names.

    pub nodeId: NodeId,
}

/// Collects class names for the node with given id and all of it's child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesFromSubtreeReturns {
    /// Class name list.

    pub classNames: Vec<String>,
}

impl CollectClassNamesFromSubtreeParams { pub const METHOD: &'static str = "DOM.collectClassNamesFromSubtree"; }

impl crate::CdpCommand for CollectClassNamesFromSubtreeParams {
    const METHOD: &'static str = "DOM.collectClassNamesFromSubtree";
    type Response = CollectClassNamesFromSubtreeReturns;
}

/// Creates a deep copy of the specified node and places it into the target container before the
/// given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CopyToParams {
    /// Id of the node to copy.

    pub nodeId: NodeId,
    /// Id of the element to drop the copy into.

    pub targetNodeId: NodeId,
    /// Drop the copy before this node (if absent, the copy becomes the last child of
    /// 'targetNodeId').

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertBeforeNodeId: Option<NodeId>,
}

/// Creates a deep copy of the specified node and places it into the target container before the
/// given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CopyToReturns {
    /// Id of the node clone.

    pub nodeId: NodeId,
}

impl CopyToParams { pub const METHOD: &'static str = "DOM.copyTo"; }

impl crate::CdpCommand for CopyToParams {
    const METHOD: &'static str = "DOM.copyTo";
    type Response = CopyToReturns;
}

/// Describes node given its id, does not require domain to be enabled. Does not start tracking any
/// objects, can be used for automation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DescribeNodeParams {
    /// Identifier of the node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
    /// Identifier of the backend node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DescribeNodeReturns {
    /// Node description.

    pub node: Node,
}

impl DescribeNodeParams { pub const METHOD: &'static str = "DOM.describeNode"; }

impl crate::CdpCommand for DescribeNodeParams {
    const METHOD: &'static str = "DOM.describeNode";
    type Response = DescribeNodeReturns;
}

/// Scrolls the specified rect of the given node into view if not already visible.
/// Note: exactly one between nodeId, backendNodeId and objectId should be passed
/// to identify the node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollIntoViewIfNeededParams {
    /// Identifier of the node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
    /// Identifier of the backend node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
    /// The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
    /// When omitted, center of the node will be used, similar to Element.scrollIntoView.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rect: Option<Rect>,
}

impl ScrollIntoViewIfNeededParams { pub const METHOD: &'static str = "DOM.scrollIntoViewIfNeeded"; }

impl crate::CdpCommand for ScrollIntoViewIfNeededParams {
    const METHOD: &'static str = "DOM.scrollIntoViewIfNeeded";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "DOM.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "DOM.disable";
    type Response = crate::EmptyReturns;
}

/// Discards search results from the session with the given id. 'getSearchResults' should no longer
/// be called for that search.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiscardSearchResultsParams {
    /// Unique search session identifier.

    pub searchId: String,
}

impl DiscardSearchResultsParams { pub const METHOD: &'static str = "DOM.discardSearchResults"; }

impl crate::CdpCommand for DiscardSearchResultsParams {
    const METHOD: &'static str = "DOM.discardSearchResults";
    type Response = crate::EmptyReturns;
}

/// Enables DOM agent for the given page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Whether to include whitespaces in the children array of returned Nodes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeWhitespace: Option<String>,
}

impl EnableParams { pub const METHOD: &'static str = "DOM.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "DOM.enable";
    type Response = crate::EmptyReturns;
}

/// Focuses the given element.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FocusParams {
    /// Identifier of the node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
    /// Identifier of the backend node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
}

impl FocusParams { pub const METHOD: &'static str = "DOM.focus"; }

impl crate::CdpCommand for FocusParams {
    const METHOD: &'static str = "DOM.focus";
    type Response = crate::EmptyReturns;
}

/// Returns attributes for the specified node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAttributesParams {
    /// Id of the node to retrieve attributes for.

    pub nodeId: NodeId,
}

/// Returns attributes for the specified node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAttributesReturns {
    /// An interleaved array of node attribute names and values.

    pub attributes: Vec<String>,
}

impl GetAttributesParams { pub const METHOD: &'static str = "DOM.getAttributes"; }

impl crate::CdpCommand for GetAttributesParams {
    const METHOD: &'static str = "DOM.getAttributes";
    type Response = GetAttributesReturns;
}

/// Returns boxes for the given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBoxModelParams {
    /// Identifier of the node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
    /// Identifier of the backend node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
}

/// Returns boxes for the given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBoxModelReturns {
    /// Box model for the node.

    pub model: BoxModel,
}

impl GetBoxModelParams { pub const METHOD: &'static str = "DOM.getBoxModel"; }

impl crate::CdpCommand for GetBoxModelParams {
    const METHOD: &'static str = "DOM.getBoxModel";
    type Response = GetBoxModelReturns;
}

/// Returns quads that describe node position on the page. This method
/// might return multiple quads for inline nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetContentQuadsParams {
    /// Identifier of the node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
    /// Identifier of the backend node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
}

/// Returns quads that describe node position on the page. This method
/// might return multiple quads for inline nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetContentQuadsReturns {
    /// Quads that describe node layout relative to viewport.

    pub quads: Vec<Quad>,
}

impl GetContentQuadsParams { pub const METHOD: &'static str = "DOM.getContentQuads"; }

impl crate::CdpCommand for GetContentQuadsParams {
    const METHOD: &'static str = "DOM.getContentQuads";
    type Response = GetContentQuadsReturns;
}

/// Returns the root DOM node (and optionally the subtree) to the caller.
/// Implicitly enables the DOM domain events for the current target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDocumentReturns {
    /// Resulting node.

    pub root: Node,
}

impl GetDocumentParams { pub const METHOD: &'static str = "DOM.getDocument"; }

impl crate::CdpCommand for GetDocumentParams {
    const METHOD: &'static str = "DOM.getDocument";
    type Response = GetDocumentReturns;
}

/// Returns the root DOM node (and optionally the subtree) to the caller.
/// Deprecated, as it is not designed to work well with the rest of the DOM agent.
/// Use DOMSnapshot.captureSnapshot instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFlattenedDocumentReturns {
    /// Resulting node.

    pub nodes: Vec<Node>,
}

impl GetFlattenedDocumentParams { pub const METHOD: &'static str = "DOM.getFlattenedDocument"; }

impl crate::CdpCommand for GetFlattenedDocumentParams {
    const METHOD: &'static str = "DOM.getFlattenedDocument";
    type Response = GetFlattenedDocumentReturns;
}

/// Finds nodes with a given computed style in a subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodesForSubtreeByStyleParams {
    /// Node ID pointing to the root of a subtree.

    pub nodeId: NodeId,
    /// The style to filter nodes by (includes nodes if any of properties matches).

    pub computedStyles: Vec<CSSComputedStyleProperty>,
    /// Whether or not iframes and shadow roots in the same target should be traversed when returning the
    /// results (default is false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}

/// Finds nodes with a given computed style in a subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodesForSubtreeByStyleReturns {
    /// Resulting nodes.

    pub nodeIds: Vec<NodeId>,
}

impl GetNodesForSubtreeByStyleParams { pub const METHOD: &'static str = "DOM.getNodesForSubtreeByStyle"; }

impl crate::CdpCommand for GetNodesForSubtreeByStyleParams {
    const METHOD: &'static str = "DOM.getNodesForSubtreeByStyle";
    type Response = GetNodesForSubtreeByStyleReturns;
}

/// Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
/// either returned or not.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeForLocationParams {
    /// X coordinate.

    pub x: i32,
    /// Y coordinate.

    pub y: i32,
    /// False to skip to the nearest non-UA shadow root ancestor (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeUserAgentShadowDOM: Option<bool>,
    /// Whether to ignore pointer-events: none on elements and hit test them.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignorePointerEventsNone: Option<bool>,
}

/// Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
/// either returned or not.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeForLocationReturns {
    /// Resulting node.

    pub backendNodeId: BackendNodeId,
    /// Frame this node belongs to.

    pub frameId: crate::page::FrameId,
    /// Id of the node at given coordinates, only when enabled and requested document.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
}

impl GetNodeForLocationParams { pub const METHOD: &'static str = "DOM.getNodeForLocation"; }

impl crate::CdpCommand for GetNodeForLocationParams {
    const METHOD: &'static str = "DOM.getNodeForLocation";
    type Response = GetNodeForLocationReturns;
}

/// Returns node's HTML markup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOuterHTMLParams {
    /// Identifier of the node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
    /// Identifier of the backend node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
    /// Include all shadow roots. Equals to false if not specified.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeShadowDOM: Option<bool>,
}

/// Returns node's HTML markup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOuterHTMLReturns {
    /// Outer HTML markup.

    pub outerHTML: String,
}

impl GetOuterHTMLParams { pub const METHOD: &'static str = "DOM.getOuterHTML"; }

impl crate::CdpCommand for GetOuterHTMLParams {
    const METHOD: &'static str = "DOM.getOuterHTML";
    type Response = GetOuterHTMLReturns;
}

/// Returns the id of the nearest ancestor that is a relayout boundary.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRelayoutBoundaryParams {
    /// Id of the node.

    pub nodeId: NodeId,
}

/// Returns the id of the nearest ancestor that is a relayout boundary.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRelayoutBoundaryReturns {
    /// Relayout boundary node id for the given node.

    pub nodeId: NodeId,
}

impl GetRelayoutBoundaryParams { pub const METHOD: &'static str = "DOM.getRelayoutBoundary"; }

impl crate::CdpCommand for GetRelayoutBoundaryParams {
    const METHOD: &'static str = "DOM.getRelayoutBoundary";
    type Response = GetRelayoutBoundaryReturns;
}

/// Returns search results from given 'fromIndex' to given 'toIndex' from the search with the given
/// identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSearchResultsParams {
    /// Unique search session identifier.

    pub searchId: String,
    /// Start index of the search result to be returned.

    pub fromIndex: u64,
    /// End index of the search result to be returned.

    pub toIndex: u64,
}

/// Returns search results from given 'fromIndex' to given 'toIndex' from the search with the given
/// identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSearchResultsReturns {
    /// Ids of the search result nodes.

    pub nodeIds: Vec<NodeId>,
}

impl GetSearchResultsParams { pub const METHOD: &'static str = "DOM.getSearchResults"; }

impl crate::CdpCommand for GetSearchResultsParams {
    const METHOD: &'static str = "DOM.getSearchResults";
    type Response = GetSearchResultsReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HideHighlightParams {}

impl HideHighlightParams { pub const METHOD: &'static str = "DOM.hideHighlight"; }

impl crate::CdpCommand for HideHighlightParams {
    const METHOD: &'static str = "DOM.hideHighlight";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighlightNodeParams {}

impl HighlightNodeParams { pub const METHOD: &'static str = "DOM.highlightNode"; }

impl crate::CdpCommand for HighlightNodeParams {
    const METHOD: &'static str = "DOM.highlightNode";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighlightRectParams {}

impl HighlightRectParams { pub const METHOD: &'static str = "DOM.highlightRect"; }

impl crate::CdpCommand for HighlightRectParams {
    const METHOD: &'static str = "DOM.highlightRect";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkUndoableStateParams {}

impl MarkUndoableStateParams { pub const METHOD: &'static str = "DOM.markUndoableState"; }

impl crate::CdpCommand for MarkUndoableStateParams {
    const METHOD: &'static str = "DOM.markUndoableState";
    type Response = crate::EmptyReturns;
}

/// Moves node into the new container, places it before the given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MoveToParams {
    /// Id of the node to move.

    pub nodeId: NodeId,
    /// Id of the element to drop the moved node into.

    pub targetNodeId: NodeId,
    /// Drop node before this one (if absent, the moved node becomes the last child of
    /// 'targetNodeId').

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertBeforeNodeId: Option<NodeId>,
}

/// Moves node into the new container, places it before the given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MoveToReturns {
    /// New id of the moved node.

    pub nodeId: NodeId,
}

impl MoveToParams { pub const METHOD: &'static str = "DOM.moveTo"; }

impl crate::CdpCommand for MoveToParams {
    const METHOD: &'static str = "DOM.moveTo";
    type Response = MoveToReturns;
}

/// Searches for a given string in the DOM tree. Use 'getSearchResults' to access search results or
/// 'cancelSearch' to end this search session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerformSearchParams {
    /// Plain text or query selector or XPath search query.

    pub query: String,
    /// True to search in user agent shadow DOM.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeUserAgentShadowDOM: Option<bool>,
}

/// Searches for a given string in the DOM tree. Use 'getSearchResults' to access search results or
/// 'cancelSearch' to end this search session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerformSearchReturns {
    /// Unique search session identifier.

    pub searchId: String,
    /// Number of search results.

    pub resultCount: u64,
}

impl PerformSearchParams { pub const METHOD: &'static str = "DOM.performSearch"; }

impl crate::CdpCommand for PerformSearchParams {
    const METHOD: &'static str = "DOM.performSearch";
    type Response = PerformSearchReturns;
}

/// Requests that the node is sent to the caller given its path. // FIXME, use XPath

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PushNodeByPathToFrontendParams {
    /// Path to node in the proprietary format.

    pub path: String,
}

/// Requests that the node is sent to the caller given its path. // FIXME, use XPath

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PushNodeByPathToFrontendReturns {
    /// Id of the node for given path.

    pub nodeId: NodeId,
}

impl PushNodeByPathToFrontendParams { pub const METHOD: &'static str = "DOM.pushNodeByPathToFrontend"; }

impl crate::CdpCommand for PushNodeByPathToFrontendParams {
    const METHOD: &'static str = "DOM.pushNodeByPathToFrontend";
    type Response = PushNodeByPathToFrontendReturns;
}

/// Requests that a batch of nodes is sent to the caller given their backend node ids.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PushNodesByBackendIdsToFrontendParams {
    /// The array of backend node ids.

    pub backendNodeIds: Vec<BackendNodeId>,
}

/// Requests that a batch of nodes is sent to the caller given their backend node ids.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PushNodesByBackendIdsToFrontendReturns {
    /// The array of ids of pushed nodes that correspond to the backend ids specified in
    /// backendNodeIds.

    pub nodeIds: Vec<NodeId>,
}

impl PushNodesByBackendIdsToFrontendParams { pub const METHOD: &'static str = "DOM.pushNodesByBackendIdsToFrontend"; }

impl crate::CdpCommand for PushNodesByBackendIdsToFrontendParams {
    const METHOD: &'static str = "DOM.pushNodesByBackendIdsToFrontend";
    type Response = PushNodesByBackendIdsToFrontendReturns;
}

/// Executes 'querySelector' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorParams {
    /// Id of the node to query upon.

    pub nodeId: NodeId,
    /// Selector string.

    pub selector: String,
}

/// Executes 'querySelector' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorReturns {
    /// Query selector result.

    pub nodeId: NodeId,
}

impl QuerySelectorParams { pub const METHOD: &'static str = "DOM.querySelector"; }

impl crate::CdpCommand for QuerySelectorParams {
    const METHOD: &'static str = "DOM.querySelector";
    type Response = QuerySelectorReturns;
}

/// Executes 'querySelectorAll' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorAllParams {
    /// Id of the node to query upon.

    pub nodeId: NodeId,
    /// Selector string.

    pub selector: String,
}

/// Executes 'querySelectorAll' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorAllReturns {
    /// Query selector result.

    pub nodeIds: Vec<NodeId>,
}

impl QuerySelectorAllParams { pub const METHOD: &'static str = "DOM.querySelectorAll"; }

impl crate::CdpCommand for QuerySelectorAllParams {
    const METHOD: &'static str = "DOM.querySelectorAll";
    type Response = QuerySelectorAllReturns;
}

/// Returns NodeIds of current top layer elements.
/// Top layer is rendered closest to the user within a viewport, therefore its elements always
/// appear on top of all other content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTopLayerElementsReturns {
    /// NodeIds of top layer elements

    pub nodeIds: Vec<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTopLayerElementsParams {}

impl GetTopLayerElementsParams { pub const METHOD: &'static str = "DOM.getTopLayerElements"; }

impl crate::CdpCommand for GetTopLayerElementsParams {
    const METHOD: &'static str = "DOM.getTopLayerElements";
    type Response = GetTopLayerElementsReturns;
}

/// Returns the NodeId of the matched element according to certain relations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetElementByRelationParams {
    /// Id of the node from which to query the relation.

    pub nodeId: NodeId,
    /// Type of relation to get.

    pub relation: String,
}

/// Returns the NodeId of the matched element according to certain relations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetElementByRelationReturns {
    /// NodeId of the element matching the queried relation.

    pub nodeId: NodeId,
}

impl GetElementByRelationParams { pub const METHOD: &'static str = "DOM.getElementByRelation"; }

impl crate::CdpCommand for GetElementByRelationParams {
    const METHOD: &'static str = "DOM.getElementByRelation";
    type Response = GetElementByRelationReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedoParams {}

impl RedoParams { pub const METHOD: &'static str = "DOM.redo"; }

impl crate::CdpCommand for RedoParams {
    const METHOD: &'static str = "DOM.redo";
    type Response = crate::EmptyReturns;
}

/// Removes attribute with given name from an element with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAttributeParams {
    /// Id of the element to remove attribute from.

    pub nodeId: NodeId,
    /// Name of the attribute to remove.

    pub name: String,
}

impl RemoveAttributeParams { pub const METHOD: &'static str = "DOM.removeAttribute"; }

impl crate::CdpCommand for RemoveAttributeParams {
    const METHOD: &'static str = "DOM.removeAttribute";
    type Response = crate::EmptyReturns;
}

/// Removes node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveNodeParams {
    /// Id of the node to remove.

    pub nodeId: NodeId,
}

impl RemoveNodeParams { pub const METHOD: &'static str = "DOM.removeNode"; }

impl crate::CdpCommand for RemoveNodeParams {
    const METHOD: &'static str = "DOM.removeNode";
    type Response = crate::EmptyReturns;
}

/// Requests that children of the node with given id are returned to the caller in form of
/// 'setChildNodes' events where not only immediate children are retrieved, but all children down to
/// the specified depth.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestChildNodesParams {
    /// Id of the node to get children for.

    pub nodeId: NodeId,
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the sub-tree
    /// (default is false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}

impl RequestChildNodesParams { pub const METHOD: &'static str = "DOM.requestChildNodes"; }

impl crate::CdpCommand for RequestChildNodesParams {
    const METHOD: &'static str = "DOM.requestChildNodes";
    type Response = crate::EmptyReturns;
}

/// Requests that the node is sent to the caller given the JavaScript node object reference. All
/// nodes that form the path from the node to the root are also sent to the client as a series of
/// 'setChildNodes' notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestNodeParams {
    /// JavaScript object id to convert into node.

    pub objectId: crate::runtime::RemoteObjectId,
}

/// Requests that the node is sent to the caller given the JavaScript node object reference. All
/// nodes that form the path from the node to the root are also sent to the client as a series of
/// 'setChildNodes' notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestNodeReturns {
    /// Node id for given object.

    pub nodeId: NodeId,
}

impl RequestNodeParams { pub const METHOD: &'static str = "DOM.requestNode"; }

impl crate::CdpCommand for RequestNodeParams {
    const METHOD: &'static str = "DOM.requestNode";
    type Response = RequestNodeReturns;
}

/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveNodeParams {
    /// Id of the node to resolve.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
    /// Backend identifier of the node to resolve.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
    /// Symbolic group name that can be used to release multiple objects.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectGroup: Option<String>,
    /// Execution context in which to resolve the node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executionContextId: Option<crate::runtime::ExecutionContextId>,
}

/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveNodeReturns {
    /// JavaScript object wrapper for given node.

    pub object: crate::runtime::RemoteObject,
}

impl ResolveNodeParams { pub const METHOD: &'static str = "DOM.resolveNode"; }

impl crate::CdpCommand for ResolveNodeParams {
    const METHOD: &'static str = "DOM.resolveNode";
    type Response = ResolveNodeReturns;
}

/// Sets attribute for an element with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAttributeValueParams {
    /// Id of the element to set attribute for.

    pub nodeId: NodeId,
    /// Attribute name.

    pub name: String,
    /// Attribute value.

    pub value: String,
}

impl SetAttributeValueParams { pub const METHOD: &'static str = "DOM.setAttributeValue"; }

impl crate::CdpCommand for SetAttributeValueParams {
    const METHOD: &'static str = "DOM.setAttributeValue";
    type Response = crate::EmptyReturns;
}

/// Sets attributes on element with given id. This method is useful when user edits some existing
/// attribute value and types in several attribute name/value pairs.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAttributesAsTextParams {
    /// Id of the element to set attributes for.

    pub nodeId: NodeId,
    /// Text with a number of attributes. Will parse this text using HTML parser.

    pub text: String,
    /// Attribute name to replace with new attributes derived from text in case text parsed
    /// successfully.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetAttributesAsTextParams { pub const METHOD: &'static str = "DOM.setAttributesAsText"; }

impl crate::CdpCommand for SetAttributesAsTextParams {
    const METHOD: &'static str = "DOM.setAttributesAsText";
    type Response = crate::EmptyReturns;
}

/// Sets files for the given file input element.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFileInputFilesParams {
    /// Array of file paths to set.

    pub files: Vec<String>,
    /// Identifier of the node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
    /// Identifier of the backend node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
}

impl SetFileInputFilesParams { pub const METHOD: &'static str = "DOM.setFileInputFiles"; }

impl crate::CdpCommand for SetFileInputFilesParams {
    const METHOD: &'static str = "DOM.setFileInputFiles";
    type Response = crate::EmptyReturns;
}

/// Sets if stack traces should be captured for Nodes. See 'Node.getNodeStackTraces'. Default is disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeStackTracesEnabledParams {
    /// Enable or disable.

    pub enable: bool,
}

impl SetNodeStackTracesEnabledParams { pub const METHOD: &'static str = "DOM.setNodeStackTracesEnabled"; }

impl crate::CdpCommand for SetNodeStackTracesEnabledParams {
    const METHOD: &'static str = "DOM.setNodeStackTracesEnabled";
    type Response = crate::EmptyReturns;
}

/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeStackTracesParams {
    /// Id of the node to get stack traces for.

    pub nodeId: NodeId,
}

/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeStackTracesReturns {
    /// Creation stack trace, if available.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation: Option<crate::runtime::StackTrace>,
}

impl GetNodeStackTracesParams { pub const METHOD: &'static str = "DOM.getNodeStackTraces"; }

impl crate::CdpCommand for GetNodeStackTracesParams {
    const METHOD: &'static str = "DOM.getNodeStackTraces";
    type Response = GetNodeStackTracesReturns;
}

/// Returns file information for the given
/// File wrapper.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFileInfoParams {
    /// JavaScript object id of the node wrapper.

    pub objectId: crate::runtime::RemoteObjectId,
}

/// Returns file information for the given
/// File wrapper.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFileInfoReturns {

    pub path: String,
}

impl GetFileInfoParams { pub const METHOD: &'static str = "DOM.getFileInfo"; }

impl crate::CdpCommand for GetFileInfoParams {
    const METHOD: &'static str = "DOM.getFileInfo";
    type Response = GetFileInfoReturns;
}

/// Returns list of detached nodes

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDetachedDomNodesReturns {
    /// The list of detached nodes

    pub detachedNodes: Vec<DetachedElementInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDetachedDomNodesParams {}

impl GetDetachedDomNodesParams { pub const METHOD: &'static str = "DOM.getDetachedDomNodes"; }

impl crate::CdpCommand for GetDetachedDomNodesParams {
    const METHOD: &'static str = "DOM.getDetachedDomNodes";
    type Response = GetDetachedDomNodesReturns;
}

/// Enables console to refer to the node with given id via $x (see Command Line API for more details
/// $x functions).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInspectedNodeParams {
    /// DOM node id to be accessible by means of $x command line API.

    pub nodeId: NodeId,
}

impl SetInspectedNodeParams { pub const METHOD: &'static str = "DOM.setInspectedNode"; }

impl crate::CdpCommand for SetInspectedNodeParams {
    const METHOD: &'static str = "DOM.setInspectedNode";
    type Response = crate::EmptyReturns;
}

/// Sets node name for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeNameParams {
    /// Id of the node to set name for.

    pub nodeId: NodeId,
    /// New node's name.

    pub name: String,
}

/// Sets node name for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeNameReturns {
    /// New node's id.

    pub nodeId: NodeId,
}

impl SetNodeNameParams { pub const METHOD: &'static str = "DOM.setNodeName"; }

impl crate::CdpCommand for SetNodeNameParams {
    const METHOD: &'static str = "DOM.setNodeName";
    type Response = SetNodeNameReturns;
}

/// Sets node value for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeValueParams {
    /// Id of the node to set value for.

    pub nodeId: NodeId,
    /// New node's value.

    pub value: String,
}

impl SetNodeValueParams { pub const METHOD: &'static str = "DOM.setNodeValue"; }

impl crate::CdpCommand for SetNodeValueParams {
    const METHOD: &'static str = "DOM.setNodeValue";
    type Response = crate::EmptyReturns;
}

/// Sets node HTML markup, returns new node id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetOuterHTMLParams {
    /// Id of the node to set markup for.

    pub nodeId: NodeId,
    /// Outer HTML markup to set.

    pub outerHTML: String,
}

impl SetOuterHTMLParams { pub const METHOD: &'static str = "DOM.setOuterHTML"; }

impl crate::CdpCommand for SetOuterHTMLParams {
    const METHOD: &'static str = "DOM.setOuterHTML";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UndoParams {}

impl UndoParams { pub const METHOD: &'static str = "DOM.undo"; }

impl crate::CdpCommand for UndoParams {
    const METHOD: &'static str = "DOM.undo";
    type Response = crate::EmptyReturns;
}

/// Returns iframe node that owns iframe with the given domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameOwnerParams {

    pub frameId: crate::page::FrameId,
}

/// Returns iframe node that owns iframe with the given domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameOwnerReturns {
    /// Resulting node.

    pub backendNodeId: BackendNodeId,
    /// Id of the node at given coordinates, only when enabled and requested document.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
}

impl GetFrameOwnerParams { pub const METHOD: &'static str = "DOM.getFrameOwner"; }

impl crate::CdpCommand for GetFrameOwnerParams {
    const METHOD: &'static str = "DOM.getFrameOwner";
    type Response = GetFrameOwnerReturns;
}

/// Returns the query container of the given node based on container query
/// conditions: containerName, physical and logical axes, and whether it queries
/// scroll-state or anchored elements. If no axes are provided and
/// queriesScrollState is false, the style container is returned, which is the
/// direct parent or the closest element with a matching container-name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetContainerForNodeParams {

    pub nodeId: NodeId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerName: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub physicalAxes: Option<PhysicalAxes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logicalAxes: Option<LogicalAxes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queriesScrollState: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queriesAnchored: Option<bool>,
}

/// Returns the query container of the given node based on container query
/// conditions: containerName, physical and logical axes, and whether it queries
/// scroll-state or anchored elements. If no axes are provided and
/// queriesScrollState is false, the style container is returned, which is the
/// direct parent or the closest element with a matching container-name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetContainerForNodeReturns {
    /// The container node for the given node, or null if not found.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<NodeId>,
}

impl GetContainerForNodeParams { pub const METHOD: &'static str = "DOM.getContainerForNode"; }

impl crate::CdpCommand for GetContainerForNodeParams {
    const METHOD: &'static str = "DOM.getContainerForNode";
    type Response = GetContainerForNodeReturns;
}

/// Returns the descendants of a container query container that have
/// container queries against this container.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryingDescendantsForContainerParams {
    /// Id of the container node to find querying descendants from.

    pub nodeId: NodeId,
}

/// Returns the descendants of a container query container that have
/// container queries against this container.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryingDescendantsForContainerReturns {
    /// Descendant nodes with container queries against the given container.

    pub nodeIds: Vec<NodeId>,
}

impl GetQueryingDescendantsForContainerParams { pub const METHOD: &'static str = "DOM.getQueryingDescendantsForContainer"; }

impl crate::CdpCommand for GetQueryingDescendantsForContainerParams {
    const METHOD: &'static str = "DOM.getQueryingDescendantsForContainer";
    type Response = GetQueryingDescendantsForContainerReturns;
}

/// Returns the target anchor element of the given anchor query according to
/// https://www.w3.org/TR/css-anchor-position-1/#target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnchorElementParams {
    /// Id of the positioned element from which to find the anchor.

    pub nodeId: NodeId,
    /// An optional anchor specifier, as defined in
    /// https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier.
    /// If not provided, it will return the implicit anchor element for
    /// the given positioned element.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchorSpecifier: Option<String>,
}

/// Returns the target anchor element of the given anchor query according to
/// https://www.w3.org/TR/css-anchor-position-1/#target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnchorElementReturns {
    /// The anchor element of the given anchor query.

    pub nodeId: NodeId,
}

impl GetAnchorElementParams { pub const METHOD: &'static str = "DOM.getAnchorElement"; }

impl crate::CdpCommand for GetAnchorElementParams {
    const METHOD: &'static str = "DOM.getAnchorElement";
    type Response = GetAnchorElementReturns;
}

/// When enabling, this API force-opens the popover identified by nodeId
/// and keeps it open until disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForceShowPopoverParams {
    /// Id of the popover HTMLElement

    pub nodeId: NodeId,
    /// If true, opens the popover and keeps it open. If false, closes the
    /// popover if it was previously force-opened.

    pub enable: bool,
}

/// When enabling, this API force-opens the popover identified by nodeId
/// and keeps it open until disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForceShowPopoverReturns {
    /// List of popovers that were closed in order to respect popover stacking order.

    pub nodeIds: Vec<NodeId>,
}

impl ForceShowPopoverParams { pub const METHOD: &'static str = "DOM.forceShowPopover"; }

impl crate::CdpCommand for ForceShowPopoverParams {
    const METHOD: &'static str = "DOM.forceShowPopover";
    type Response = ForceShowPopoverReturns;
}
