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

/// Unique DOM node identifier.

pub type NodeId = i64;

/// Unique DOM node identifier used to reference a node that may not have been pushed to the
/// front-end.

pub type BackendNodeId = i64;

/// Unique identifier for a CSS stylesheet.

pub type StyleSheetId<'a> = Cow<'a, str>;

/// Backend node with a friendly name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackendNode<'a> {
    /// 'Node''s nodeType.
    nodeType: i64,
    /// 'Node''s nodeName.
    nodeName: Cow<'a, str>,
    backendNodeId: BackendNodeId,
}

impl<'a> BackendNode<'a> {
    pub fn builder() -> BackendNodeBuilder<'a> { BackendNodeBuilder::default() }
    pub fn nodeType(&self) -> i64 { self.nodeType }
    pub fn nodeName(&self) -> &str { self.nodeName.as_ref() }
    pub fn backendNodeId(&self) -> &BackendNodeId { &self.backendNodeId }
}

#[derive(Default)]
pub struct BackendNodeBuilder<'a> {
    nodeType: Option<i64>,
    nodeName: Option<Cow<'a, str>>,
    backendNodeId: Option<BackendNodeId>,
}

impl<'a> BackendNodeBuilder<'a> {
    /// 'Node''s nodeType.
    pub fn nodeType(mut self, nodeType: i64) -> Self { self.nodeType = Some(nodeType); self }
    /// 'Node''s nodeName.
    pub fn nodeName(mut self, nodeName: impl Into<Cow<'a, str>>) -> Self { self.nodeName = Some(nodeName.into()); self }
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    pub fn build(self) -> BackendNode<'a> {
        BackendNode {
            nodeType: self.nodeType.unwrap_or_default(),
            nodeName: self.nodeName.unwrap_or_default(),
            backendNodeId: self.backendNodeId.unwrap_or_default(),
        }
    }
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
    #[serde(rename = "interest-hint")]
    InterestHint,
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
    #[serde(rename = "permission-icon")]
    PermissionIcon,
    #[serde(rename = "overscroll-area-parent")]
    OverscrollAreaParent,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Node<'a> {
    /// Node identifier that is passed into the rest of the DOM messages as the 'nodeId'. Backend
    /// will only push node with given 'id' once. It is aware of all requested nodes and will only
    /// fire DOM events for nodes known to the client.
    nodeId: NodeId,
    /// The id of the parent node if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    parentId: Option<NodeId>,
    /// The BackendNodeId for this node.
    backendNodeId: BackendNodeId,
    /// 'Node''s nodeType.
    nodeType: i64,
    /// 'Node''s nodeName.
    nodeName: Cow<'a, str>,
    /// 'Node''s localName.
    localName: Cow<'a, str>,
    /// 'Node''s nodeValue.
    nodeValue: Cow<'a, str>,
    /// Child count for 'Container' nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    childNodeCount: Option<u64>,
    /// Child nodes of this node when requested with children.
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Box<Node<'a>>>>,
    /// Attributes of the 'Element' node in the form of flat array '[name1, value1, name2, value2]'.
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<Cow<'a, str>>>,
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    #[serde(skip_serializing_if = "Option::is_none")]
    documentURL: Option<Cow<'a, str>>,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    baseURL: Option<Cow<'a, str>>,
    /// 'DocumentType''s publicId.
    #[serde(skip_serializing_if = "Option::is_none")]
    publicId: Option<Cow<'a, str>>,
    /// 'DocumentType''s systemId.
    #[serde(skip_serializing_if = "Option::is_none")]
    systemId: Option<Cow<'a, str>>,
    /// 'DocumentType''s internalSubset.
    #[serde(skip_serializing_if = "Option::is_none")]
    internalSubset: Option<Cow<'a, str>>,
    /// 'Document''s XML version in case of XML documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    xmlVersion: Option<Cow<'a, str>>,
    /// 'Attr''s name.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    /// 'Attr''s value.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Cow<'a, str>>,
    /// Pseudo element type for this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoType: Option<PseudoType>,
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoIdentifier: Option<Cow<'a, str>>,
    /// Shadow root type.
    #[serde(skip_serializing_if = "Option::is_none")]
    shadowRootType: Option<ShadowRootType>,
    /// Frame ID for frame owner elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
    /// Content document for frame owner elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    contentDocument: Option<Box<Node<'a>>>,
    /// Shadow root list for given element host.
    #[serde(skip_serializing_if = "Option::is_none")]
    shadowRoots: Option<Vec<Box<Node<'a>>>>,
    /// Content document fragment for template elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    templateContent: Option<Box<Node<'a>>>,
    /// Pseudo elements associated with this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoElements: Option<Vec<Box<Node<'a>>>>,
    /// Deprecated, as the HTML Imports API has been removed (crbug.com/937746).
    /// This property used to return the imported document for the HTMLImport links.
    /// The property is always undefined now.
    #[serde(skip_serializing_if = "Option::is_none")]
    importedDocument: Option<Box<Node<'a>>>,
    /// Distributed nodes for given insertion point.
    #[serde(skip_serializing_if = "Option::is_none")]
    distributedNodes: Option<Vec<BackendNode<'a>>>,
    /// Whether the node is SVG.
    #[serde(skip_serializing_if = "Option::is_none")]
    isSVG: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compatibilityMode: Option<CompatibilityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignedSlot: Option<BackendNode<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isScrollable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affectedByStartingStyles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adoptedStyleSheets: Option<Vec<StyleSheetId<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adProvenance: Option<crate::network::AdProvenance<'a>>,
}

impl<'a> Node<'a> {
    pub fn builder() -> NodeBuilder<'a> { NodeBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn parentId(&self) -> Option<&NodeId> { self.parentId.as_ref() }
    pub fn backendNodeId(&self) -> &BackendNodeId { &self.backendNodeId }
    pub fn nodeType(&self) -> i64 { self.nodeType }
    pub fn nodeName(&self) -> &str { self.nodeName.as_ref() }
    pub fn localName(&self) -> &str { self.localName.as_ref() }
    pub fn nodeValue(&self) -> &str { self.nodeValue.as_ref() }
    pub fn childNodeCount(&self) -> Option<u64> { self.childNodeCount }
    pub fn children(&self) -> Option<&[Box<Node<'a>>]> { self.children.as_deref() }
    pub fn attributes(&self) -> Option<&[Cow<'a, str>]> { self.attributes.as_deref() }
    pub fn documentURL(&self) -> Option<&str> { self.documentURL.as_deref() }
    pub fn baseURL(&self) -> Option<&str> { self.baseURL.as_deref() }
    pub fn publicId(&self) -> Option<&str> { self.publicId.as_deref() }
    pub fn systemId(&self) -> Option<&str> { self.systemId.as_deref() }
    pub fn internalSubset(&self) -> Option<&str> { self.internalSubset.as_deref() }
    pub fn xmlVersion(&self) -> Option<&str> { self.xmlVersion.as_deref() }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn value(&self) -> Option<&str> { self.value.as_deref() }
    pub fn pseudoType(&self) -> Option<&PseudoType> { self.pseudoType.as_ref() }
    pub fn pseudoIdentifier(&self) -> Option<&str> { self.pseudoIdentifier.as_deref() }
    pub fn shadowRootType(&self) -> Option<&ShadowRootType> { self.shadowRootType.as_ref() }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
    pub fn contentDocument(&self) -> Option<&Node<'a>> { self.contentDocument.as_deref() }
    pub fn shadowRoots(&self) -> Option<&[Box<Node<'a>>]> { self.shadowRoots.as_deref() }
    pub fn templateContent(&self) -> Option<&Node<'a>> { self.templateContent.as_deref() }
    pub fn pseudoElements(&self) -> Option<&[Box<Node<'a>>]> { self.pseudoElements.as_deref() }
    pub fn importedDocument(&self) -> Option<&Node<'a>> { self.importedDocument.as_deref() }
    pub fn distributedNodes(&self) -> Option<&[BackendNode<'a>]> { self.distributedNodes.as_deref() }
    pub fn isSVG(&self) -> Option<bool> { self.isSVG }
    pub fn compatibilityMode(&self) -> Option<&CompatibilityMode> { self.compatibilityMode.as_ref() }
    pub fn assignedSlot(&self) -> Option<&BackendNode<'a>> { self.assignedSlot.as_ref() }
    pub fn isScrollable(&self) -> Option<bool> { self.isScrollable }
    pub fn affectedByStartingStyles(&self) -> Option<bool> { self.affectedByStartingStyles }
    pub fn adoptedStyleSheets(&self) -> Option<&[StyleSheetId<'a>]> { self.adoptedStyleSheets.as_deref() }
    pub fn adProvenance(&self) -> Option<&crate::network::AdProvenance<'a>> { self.adProvenance.as_ref() }
}

#[derive(Default)]
pub struct NodeBuilder<'a> {
    nodeId: Option<NodeId>,
    parentId: Option<NodeId>,
    backendNodeId: Option<BackendNodeId>,
    nodeType: Option<i64>,
    nodeName: Option<Cow<'a, str>>,
    localName: Option<Cow<'a, str>>,
    nodeValue: Option<Cow<'a, str>>,
    childNodeCount: Option<u64>,
    children: Option<Vec<Box<Node<'a>>>>,
    attributes: Option<Vec<Cow<'a, str>>>,
    documentURL: Option<Cow<'a, str>>,
    baseURL: Option<Cow<'a, str>>,
    publicId: Option<Cow<'a, str>>,
    systemId: Option<Cow<'a, str>>,
    internalSubset: Option<Cow<'a, str>>,
    xmlVersion: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    pseudoType: Option<PseudoType>,
    pseudoIdentifier: Option<Cow<'a, str>>,
    shadowRootType: Option<ShadowRootType>,
    frameId: Option<crate::page::FrameId<'a>>,
    contentDocument: Option<Box<Node<'a>>>,
    shadowRoots: Option<Vec<Box<Node<'a>>>>,
    templateContent: Option<Box<Node<'a>>>,
    pseudoElements: Option<Vec<Box<Node<'a>>>>,
    importedDocument: Option<Box<Node<'a>>>,
    distributedNodes: Option<Vec<BackendNode<'a>>>,
    isSVG: Option<bool>,
    compatibilityMode: Option<CompatibilityMode>,
    assignedSlot: Option<BackendNode<'a>>,
    isScrollable: Option<bool>,
    affectedByStartingStyles: Option<bool>,
    adoptedStyleSheets: Option<Vec<StyleSheetId<'a>>>,
    adProvenance: Option<crate::network::AdProvenance<'a>>,
}

impl<'a> NodeBuilder<'a> {
    /// Node identifier that is passed into the rest of the DOM messages as the 'nodeId'. Backend
    /// will only push node with given 'id' once. It is aware of all requested nodes and will only
    /// fire DOM events for nodes known to the client.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// The id of the parent node if any.
    pub fn parentId(mut self, parentId: NodeId) -> Self { self.parentId = Some(parentId); self }
    /// The BackendNodeId for this node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// 'Node''s nodeType.
    pub fn nodeType(mut self, nodeType: i64) -> Self { self.nodeType = Some(nodeType); self }
    /// 'Node''s nodeName.
    pub fn nodeName(mut self, nodeName: impl Into<Cow<'a, str>>) -> Self { self.nodeName = Some(nodeName.into()); self }
    /// 'Node''s localName.
    pub fn localName(mut self, localName: impl Into<Cow<'a, str>>) -> Self { self.localName = Some(localName.into()); self }
    /// 'Node''s nodeValue.
    pub fn nodeValue(mut self, nodeValue: impl Into<Cow<'a, str>>) -> Self { self.nodeValue = Some(nodeValue.into()); self }
    /// Child count for 'Container' nodes.
    pub fn childNodeCount(mut self, childNodeCount: u64) -> Self { self.childNodeCount = Some(childNodeCount); self }
    /// Child nodes of this node when requested with children.
    pub fn children(mut self, children: Vec<Box<Node<'a>>>) -> Self { self.children = Some(children); self }
    /// Attributes of the 'Element' node in the form of flat array '[name1, value1, name2, value2]'.
    pub fn attributes(mut self, attributes: Vec<Cow<'a, str>>) -> Self { self.attributes = Some(attributes); self }
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    pub fn documentURL(mut self, documentURL: impl Into<Cow<'a, str>>) -> Self { self.documentURL = Some(documentURL.into()); self }
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    pub fn baseURL(mut self, baseURL: impl Into<Cow<'a, str>>) -> Self { self.baseURL = Some(baseURL.into()); self }
    /// 'DocumentType''s publicId.
    pub fn publicId(mut self, publicId: impl Into<Cow<'a, str>>) -> Self { self.publicId = Some(publicId.into()); self }
    /// 'DocumentType''s systemId.
    pub fn systemId(mut self, systemId: impl Into<Cow<'a, str>>) -> Self { self.systemId = Some(systemId.into()); self }
    /// 'DocumentType''s internalSubset.
    pub fn internalSubset(mut self, internalSubset: impl Into<Cow<'a, str>>) -> Self { self.internalSubset = Some(internalSubset.into()); self }
    /// 'Document''s XML version in case of XML documents.
    pub fn xmlVersion(mut self, xmlVersion: impl Into<Cow<'a, str>>) -> Self { self.xmlVersion = Some(xmlVersion.into()); self }
    /// 'Attr''s name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// 'Attr''s value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// Pseudo element type for this node.
    pub fn pseudoType(mut self, pseudoType: PseudoType) -> Self { self.pseudoType = Some(pseudoType); self }
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    pub fn pseudoIdentifier(mut self, pseudoIdentifier: impl Into<Cow<'a, str>>) -> Self { self.pseudoIdentifier = Some(pseudoIdentifier.into()); self }
    /// Shadow root type.
    pub fn shadowRootType(mut self, shadowRootType: ShadowRootType) -> Self { self.shadowRootType = Some(shadowRootType); self }
    /// Frame ID for frame owner elements.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// Content document for frame owner elements.
    pub fn contentDocument(mut self, contentDocument: Box<Node<'a>>) -> Self { self.contentDocument = Some(contentDocument); self }
    /// Shadow root list for given element host.
    pub fn shadowRoots(mut self, shadowRoots: Vec<Box<Node<'a>>>) -> Self { self.shadowRoots = Some(shadowRoots); self }
    /// Content document fragment for template elements.
    pub fn templateContent(mut self, templateContent: Box<Node<'a>>) -> Self { self.templateContent = Some(templateContent); self }
    /// Pseudo elements associated with this node.
    pub fn pseudoElements(mut self, pseudoElements: Vec<Box<Node<'a>>>) -> Self { self.pseudoElements = Some(pseudoElements); self }
    /// Deprecated, as the HTML Imports API has been removed (crbug.com/937746).
    /// This property used to return the imported document for the HTMLImport links.
    /// The property is always undefined now.
    pub fn importedDocument(mut self, importedDocument: Box<Node<'a>>) -> Self { self.importedDocument = Some(importedDocument); self }
    /// Distributed nodes for given insertion point.
    pub fn distributedNodes(mut self, distributedNodes: Vec<BackendNode<'a>>) -> Self { self.distributedNodes = Some(distributedNodes); self }
    /// Whether the node is SVG.
    pub fn isSVG(mut self, isSVG: bool) -> Self { self.isSVG = Some(isSVG); self }
    pub fn compatibilityMode(mut self, compatibilityMode: CompatibilityMode) -> Self { self.compatibilityMode = Some(compatibilityMode); self }
    pub fn assignedSlot(mut self, assignedSlot: BackendNode<'a>) -> Self { self.assignedSlot = Some(assignedSlot); self }
    pub fn isScrollable(mut self, isScrollable: bool) -> Self { self.isScrollable = Some(isScrollable); self }
    pub fn affectedByStartingStyles(mut self, affectedByStartingStyles: bool) -> Self { self.affectedByStartingStyles = Some(affectedByStartingStyles); self }
    pub fn adoptedStyleSheets(mut self, adoptedStyleSheets: Vec<StyleSheetId<'a>>) -> Self { self.adoptedStyleSheets = Some(adoptedStyleSheets); self }
    pub fn adProvenance(mut self, adProvenance: crate::network::AdProvenance<'a>) -> Self { self.adProvenance = Some(adProvenance); self }
    pub fn build(self) -> Node<'a> {
        Node {
            nodeId: self.nodeId.unwrap_or_default(),
            parentId: self.parentId,
            backendNodeId: self.backendNodeId.unwrap_or_default(),
            nodeType: self.nodeType.unwrap_or_default(),
            nodeName: self.nodeName.unwrap_or_default(),
            localName: self.localName.unwrap_or_default(),
            nodeValue: self.nodeValue.unwrap_or_default(),
            childNodeCount: self.childNodeCount,
            children: self.children,
            attributes: self.attributes,
            documentURL: self.documentURL,
            baseURL: self.baseURL,
            publicId: self.publicId,
            systemId: self.systemId,
            internalSubset: self.internalSubset,
            xmlVersion: self.xmlVersion,
            name: self.name,
            value: self.value,
            pseudoType: self.pseudoType,
            pseudoIdentifier: self.pseudoIdentifier,
            shadowRootType: self.shadowRootType,
            frameId: self.frameId,
            contentDocument: self.contentDocument,
            shadowRoots: self.shadowRoots,
            templateContent: self.templateContent,
            pseudoElements: self.pseudoElements,
            importedDocument: self.importedDocument,
            distributedNodes: self.distributedNodes,
            isSVG: self.isSVG,
            compatibilityMode: self.compatibilityMode,
            assignedSlot: self.assignedSlot,
            isScrollable: self.isScrollable,
            affectedByStartingStyles: self.affectedByStartingStyles,
            adoptedStyleSheets: self.adoptedStyleSheets,
            adProvenance: self.adProvenance,
        }
    }
}

/// A structure to hold the top-level node of a detached tree and an array of its retained descendants.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DetachedElementInfo<'a> {
    treeNode: Node<'a>,
    retainedNodeIds: Vec<NodeId>,
}

impl<'a> DetachedElementInfo<'a> {
    pub fn builder() -> DetachedElementInfoBuilder<'a> { DetachedElementInfoBuilder::default() }
    pub fn treeNode(&self) -> &Node<'a> { &self.treeNode }
    pub fn retainedNodeIds(&self) -> &[NodeId] { &self.retainedNodeIds }
}

#[derive(Default)]
pub struct DetachedElementInfoBuilder<'a> {
    treeNode: Option<Node<'a>>,
    retainedNodeIds: Option<Vec<NodeId>>,
}

impl<'a> DetachedElementInfoBuilder<'a> {
    pub fn treeNode(mut self, treeNode: Node<'a>) -> Self { self.treeNode = Some(treeNode); self }
    pub fn retainedNodeIds(mut self, retainedNodeIds: Vec<NodeId>) -> Self { self.retainedNodeIds = Some(retainedNodeIds); self }
    pub fn build(self) -> DetachedElementInfo<'a> {
        DetachedElementInfo {
            treeNode: self.treeNode.unwrap_or_default(),
            retainedNodeIds: self.retainedNodeIds.unwrap_or_default(),
        }
    }
}

/// A structure holding an RGBA color.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RGBA {
    /// The red component, in the [0-255] range.
    r: i64,
    /// The green component, in the [0-255] range.
    g: i64,
    /// The blue component, in the [0-255] range.
    b: i64,
    /// The alpha component, in the [0-1] range (default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<f64>,
}

impl RGBA {
    pub fn builder() -> RGBABuilder { RGBABuilder::default() }
    pub fn r(&self) -> i64 { self.r }
    pub fn g(&self) -> i64 { self.g }
    pub fn b(&self) -> i64 { self.b }
    pub fn a(&self) -> Option<f64> { self.a }
}

#[derive(Default)]
pub struct RGBABuilder {
    r: Option<i64>,
    g: Option<i64>,
    b: Option<i64>,
    a: Option<f64>,
}

impl RGBABuilder {
    /// The red component, in the [0-255] range.
    pub fn r(mut self, r: i64) -> Self { self.r = Some(r); self }
    /// The green component, in the [0-255] range.
    pub fn g(mut self, g: i64) -> Self { self.g = Some(g); self }
    /// The blue component, in the [0-255] range.
    pub fn b(mut self, b: i64) -> Self { self.b = Some(b); self }
    /// The alpha component, in the [0-1] range (default: 1).
    pub fn a(mut self, a: f64) -> Self { self.a = Some(a); self }
    pub fn build(self) -> RGBA {
        RGBA {
            r: self.r.unwrap_or_default(),
            g: self.g.unwrap_or_default(),
            b: self.b.unwrap_or_default(),
            a: self.a,
        }
    }
}

/// An array of quad vertices, x immediately followed by y for each point, points clock-wise.

pub type Quad = Vec<f64>;

/// Box model.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoxModel {
    /// Content box
    content: Quad,
    /// Padding box
    padding: Quad,
    /// Border box
    border: Quad,
    /// Margin box
    margin: Quad,
    /// Node width
    width: u64,
    /// Node height
    height: i64,
    /// Shape outside coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    shapeOutside: Option<ShapeOutsideInfo>,
}

impl BoxModel {
    pub fn builder() -> BoxModelBuilder { BoxModelBuilder::default() }
    pub fn content(&self) -> &Quad { &self.content }
    pub fn padding(&self) -> &Quad { &self.padding }
    pub fn border(&self) -> &Quad { &self.border }
    pub fn margin(&self) -> &Quad { &self.margin }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> i64 { self.height }
    pub fn shapeOutside(&self) -> Option<&ShapeOutsideInfo> { self.shapeOutside.as_ref() }
}

#[derive(Default)]
pub struct BoxModelBuilder {
    content: Option<Quad>,
    padding: Option<Quad>,
    border: Option<Quad>,
    margin: Option<Quad>,
    width: Option<u64>,
    height: Option<i64>,
    shapeOutside: Option<ShapeOutsideInfo>,
}

impl BoxModelBuilder {
    /// Content box
    pub fn content(mut self, content: Quad) -> Self { self.content = Some(content); self }
    /// Padding box
    pub fn padding(mut self, padding: Quad) -> Self { self.padding = Some(padding); self }
    /// Border box
    pub fn border(mut self, border: Quad) -> Self { self.border = Some(border); self }
    /// Margin box
    pub fn margin(mut self, margin: Quad) -> Self { self.margin = Some(margin); self }
    /// Node width
    pub fn width(mut self, width: u64) -> Self { self.width = Some(width); self }
    /// Node height
    pub fn height(mut self, height: i64) -> Self { self.height = Some(height); self }
    /// Shape outside coordinates
    pub fn shapeOutside(mut self, shapeOutside: ShapeOutsideInfo) -> Self { self.shapeOutside = Some(shapeOutside); self }
    pub fn build(self) -> BoxModel {
        BoxModel {
            content: self.content.unwrap_or_default(),
            padding: self.padding.unwrap_or_default(),
            border: self.border.unwrap_or_default(),
            margin: self.margin.unwrap_or_default(),
            width: self.width.unwrap_or_default(),
            height: self.height.unwrap_or_default(),
            shapeOutside: self.shapeOutside,
        }
    }
}

/// CSS Shape Outside details.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShapeOutsideInfo {
    /// Shape bounds
    bounds: Quad,
    /// Shape coordinate details
    shape: Vec<JsonValue>,
    /// Margin shape bounds
    marginShape: Vec<JsonValue>,
}

impl ShapeOutsideInfo {
    pub fn builder() -> ShapeOutsideInfoBuilder { ShapeOutsideInfoBuilder::default() }
    pub fn bounds(&self) -> &Quad { &self.bounds }
    pub fn shape(&self) -> &[JsonValue] { &self.shape }
    pub fn marginShape(&self) -> &[JsonValue] { &self.marginShape }
}

#[derive(Default)]
pub struct ShapeOutsideInfoBuilder {
    bounds: Option<Quad>,
    shape: Option<Vec<JsonValue>>,
    marginShape: Option<Vec<JsonValue>>,
}

impl ShapeOutsideInfoBuilder {
    /// Shape bounds
    pub fn bounds(mut self, bounds: Quad) -> Self { self.bounds = Some(bounds); self }
    /// Shape coordinate details
    pub fn shape(mut self, shape: Vec<JsonValue>) -> Self { self.shape = Some(shape); self }
    /// Margin shape bounds
    pub fn marginShape(mut self, marginShape: Vec<JsonValue>) -> Self { self.marginShape = Some(marginShape); self }
    pub fn build(self) -> ShapeOutsideInfo {
        ShapeOutsideInfo {
            bounds: self.bounds.unwrap_or_default(),
            shape: self.shape.unwrap_or_default(),
            marginShape: self.marginShape.unwrap_or_default(),
        }
    }
}

/// Rectangle.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rect {
    /// X coordinate
    x: f64,
    /// Y coordinate
    y: f64,
    /// Rectangle width
    width: f64,
    /// Rectangle height
    height: f64,
}

impl Rect {
    pub fn builder() -> RectBuilder { RectBuilder::default() }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn width(&self) -> f64 { self.width }
    pub fn height(&self) -> f64 { self.height }
}

#[derive(Default)]
pub struct RectBuilder {
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
}

impl RectBuilder {
    /// X coordinate
    pub fn x(mut self, x: f64) -> Self { self.x = Some(x); self }
    /// Y coordinate
    pub fn y(mut self, y: f64) -> Self { self.y = Some(y); self }
    /// Rectangle width
    pub fn width(mut self, width: f64) -> Self { self.width = Some(width); self }
    /// Rectangle height
    pub fn height(mut self, height: f64) -> Self { self.height = Some(height); self }
    pub fn build(self) -> Rect {
        Rect {
            x: self.x.unwrap_or_default(),
            y: self.y.unwrap_or_default(),
            width: self.width.unwrap_or_default(),
            height: self.height.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSComputedStyleProperty<'a> {
    /// Computed style property name.
    name: Cow<'a, str>,
    /// Computed style property value.
    value: Cow<'a, str>,
}

impl<'a> CSSComputedStyleProperty<'a> {
    pub fn builder() -> CSSComputedStylePropertyBuilder<'a> { CSSComputedStylePropertyBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct CSSComputedStylePropertyBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> CSSComputedStylePropertyBuilder<'a> {
    /// Computed style property name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Computed style property value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> CSSComputedStyleProperty<'a> {
        CSSComputedStyleProperty {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}

/// Collects class names for the node with given id and all of it's child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesFromSubtreeParams {
    /// Id of the node to collect class names.
    nodeId: NodeId,
}

impl CollectClassNamesFromSubtreeParams {
    pub fn builder() -> CollectClassNamesFromSubtreeParamsBuilder { CollectClassNamesFromSubtreeParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct CollectClassNamesFromSubtreeParamsBuilder {
    nodeId: Option<NodeId>,
}

impl CollectClassNamesFromSubtreeParamsBuilder {
    /// Id of the node to collect class names.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> CollectClassNamesFromSubtreeParams {
        CollectClassNamesFromSubtreeParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Collects class names for the node with given id and all of it's child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesFromSubtreeReturns<'a> {
    /// Class name list.
    classNames: Vec<Cow<'a, str>>,
}

impl<'a> CollectClassNamesFromSubtreeReturns<'a> {
    pub fn builder() -> CollectClassNamesFromSubtreeReturnsBuilder<'a> { CollectClassNamesFromSubtreeReturnsBuilder::default() }
    pub fn classNames(&self) -> &[Cow<'a, str>] { &self.classNames }
}

#[derive(Default)]
pub struct CollectClassNamesFromSubtreeReturnsBuilder<'a> {
    classNames: Option<Vec<Cow<'a, str>>>,
}

impl<'a> CollectClassNamesFromSubtreeReturnsBuilder<'a> {
    /// Class name list.
    pub fn classNames(mut self, classNames: Vec<Cow<'a, str>>) -> Self { self.classNames = Some(classNames); self }
    pub fn build(self) -> CollectClassNamesFromSubtreeReturns<'a> {
        CollectClassNamesFromSubtreeReturns {
            classNames: self.classNames.unwrap_or_default(),
        }
    }
}

impl CollectClassNamesFromSubtreeParams { pub const METHOD: &'static str = "DOM.collectClassNamesFromSubtree"; }

impl<'a> crate::CdpCommand<'a> for CollectClassNamesFromSubtreeParams {
    const METHOD: &'static str = "DOM.collectClassNamesFromSubtree";
    type Response = CollectClassNamesFromSubtreeReturns<'a>;
}

/// Creates a deep copy of the specified node and places it into the target container before the
/// given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CopyToParams {
    /// Id of the node to copy.
    nodeId: NodeId,
    /// Id of the element to drop the copy into.
    targetNodeId: NodeId,
    /// Drop the copy before this node (if absent, the copy becomes the last child of
    /// 'targetNodeId').
    #[serde(skip_serializing_if = "Option::is_none")]
    insertBeforeNodeId: Option<NodeId>,
}

impl CopyToParams {
    pub fn builder() -> CopyToParamsBuilder { CopyToParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn targetNodeId(&self) -> &NodeId { &self.targetNodeId }
    pub fn insertBeforeNodeId(&self) -> Option<&NodeId> { self.insertBeforeNodeId.as_ref() }
}

#[derive(Default)]
pub struct CopyToParamsBuilder {
    nodeId: Option<NodeId>,
    targetNodeId: Option<NodeId>,
    insertBeforeNodeId: Option<NodeId>,
}

impl CopyToParamsBuilder {
    /// Id of the node to copy.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Id of the element to drop the copy into.
    pub fn targetNodeId(mut self, targetNodeId: NodeId) -> Self { self.targetNodeId = Some(targetNodeId); self }
    /// Drop the copy before this node (if absent, the copy becomes the last child of
    /// 'targetNodeId').
    pub fn insertBeforeNodeId(mut self, insertBeforeNodeId: NodeId) -> Self { self.insertBeforeNodeId = Some(insertBeforeNodeId); self }
    pub fn build(self) -> CopyToParams {
        CopyToParams {
            nodeId: self.nodeId.unwrap_or_default(),
            targetNodeId: self.targetNodeId.unwrap_or_default(),
            insertBeforeNodeId: self.insertBeforeNodeId,
        }
    }
}

/// Creates a deep copy of the specified node and places it into the target container before the
/// given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CopyToReturns {
    /// Id of the node clone.
    nodeId: NodeId,
}

impl CopyToReturns {
    pub fn builder() -> CopyToReturnsBuilder { CopyToReturnsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct CopyToReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl CopyToReturnsBuilder {
    /// Id of the node clone.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> CopyToReturns {
        CopyToReturns {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl CopyToParams { pub const METHOD: &'static str = "DOM.copyTo"; }

impl<'a> crate::CdpCommand<'a> for CopyToParams {
    const METHOD: &'static str = "DOM.copyTo";
    type Response = CopyToReturns;
}

/// Describes node given its id, does not require domain to be enabled. Does not start tracking any
/// objects, can be used for automation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DescribeNodeParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pierce: Option<bool>,
}

impl<'a> DescribeNodeParams<'a> {
    pub fn builder() -> DescribeNodeParamsBuilder<'a> { DescribeNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
    pub fn depth(&self) -> Option<i64> { self.depth }
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}

#[derive(Default)]
pub struct DescribeNodeParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    backendNodeId: Option<BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    depth: Option<i64>,
    pierce: Option<bool>,
}

impl<'a> DescribeNodeParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> DescribeNodeParams<'a> {
        DescribeNodeParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
            depth: self.depth,
            pierce: self.pierce,
        }
    }
}

/// Describes node given its id, does not require domain to be enabled. Does not start tracking any
/// objects, can be used for automation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DescribeNodeReturns<'a> {
    /// Node description.
    node: Node<'a>,
}

impl<'a> DescribeNodeReturns<'a> {
    pub fn builder() -> DescribeNodeReturnsBuilder<'a> { DescribeNodeReturnsBuilder::default() }
    pub fn node(&self) -> &Node<'a> { &self.node }
}

#[derive(Default)]
pub struct DescribeNodeReturnsBuilder<'a> {
    node: Option<Node<'a>>,
}

impl<'a> DescribeNodeReturnsBuilder<'a> {
    /// Node description.
    pub fn node(mut self, node: Node<'a>) -> Self { self.node = Some(node); self }
    pub fn build(self) -> DescribeNodeReturns<'a> {
        DescribeNodeReturns {
            node: self.node.unwrap_or_default(),
        }
    }
}

impl<'a> DescribeNodeParams<'a> { pub const METHOD: &'static str = "DOM.describeNode"; }

impl<'a> crate::CdpCommand<'a> for DescribeNodeParams<'a> {
    const METHOD: &'static str = "DOM.describeNode";
    type Response = DescribeNodeReturns<'a>;
}

/// Scrolls the specified rect of the given node into view if not already visible.
/// Note: exactly one between nodeId, backendNodeId and objectId should be passed
/// to identify the node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScrollIntoViewIfNeededParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    /// The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
    /// When omitted, center of the node will be used, similar to Element.scrollIntoView.
    #[serde(skip_serializing_if = "Option::is_none")]
    rect: Option<Rect>,
}

impl<'a> ScrollIntoViewIfNeededParams<'a> {
    pub fn builder() -> ScrollIntoViewIfNeededParamsBuilder<'a> { ScrollIntoViewIfNeededParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
    pub fn rect(&self) -> Option<&Rect> { self.rect.as_ref() }
}

#[derive(Default)]
pub struct ScrollIntoViewIfNeededParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    backendNodeId: Option<BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    rect: Option<Rect>,
}

impl<'a> ScrollIntoViewIfNeededParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    /// The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
    /// When omitted, center of the node will be used, similar to Element.scrollIntoView.
    pub fn rect(mut self, rect: Rect) -> Self { self.rect = Some(rect); self }
    pub fn build(self) -> ScrollIntoViewIfNeededParams<'a> {
        ScrollIntoViewIfNeededParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
            rect: self.rect,
        }
    }
}

impl<'a> ScrollIntoViewIfNeededParams<'a> { pub const METHOD: &'static str = "DOM.scrollIntoViewIfNeeded"; }

impl<'a> crate::CdpCommand<'a> for ScrollIntoViewIfNeededParams<'a> {
    const METHOD: &'static str = "DOM.scrollIntoViewIfNeeded";
    type Response = crate::EmptyReturns;
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

impl DisableParams { pub const METHOD: &'static str = "DOM.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "DOM.disable";
    type Response = crate::EmptyReturns;
}

/// Discards search results from the session with the given id. 'getSearchResults' should no longer
/// be called for that search.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiscardSearchResultsParams<'a> {
    /// Unique search session identifier.
    searchId: Cow<'a, str>,
}

impl<'a> DiscardSearchResultsParams<'a> {
    pub fn builder() -> DiscardSearchResultsParamsBuilder<'a> { DiscardSearchResultsParamsBuilder::default() }
    pub fn searchId(&self) -> &str { self.searchId.as_ref() }
}

#[derive(Default)]
pub struct DiscardSearchResultsParamsBuilder<'a> {
    searchId: Option<Cow<'a, str>>,
}

impl<'a> DiscardSearchResultsParamsBuilder<'a> {
    /// Unique search session identifier.
    pub fn searchId(mut self, searchId: impl Into<Cow<'a, str>>) -> Self { self.searchId = Some(searchId.into()); self }
    pub fn build(self) -> DiscardSearchResultsParams<'a> {
        DiscardSearchResultsParams {
            searchId: self.searchId.unwrap_or_default(),
        }
    }
}

impl<'a> DiscardSearchResultsParams<'a> { pub const METHOD: &'static str = "DOM.discardSearchResults"; }

impl<'a> crate::CdpCommand<'a> for DiscardSearchResultsParams<'a> {
    const METHOD: &'static str = "DOM.discardSearchResults";
    type Response = crate::EmptyReturns;
}

/// Enables DOM agent for the given page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams<'a> {
    /// Whether to include whitespaces in the children array of returned Nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    includeWhitespace: Option<Cow<'a, str>>,
}

impl<'a> EnableParams<'a> {
    pub fn builder() -> EnableParamsBuilder<'a> { EnableParamsBuilder::default() }
    pub fn includeWhitespace(&self) -> Option<&str> { self.includeWhitespace.as_deref() }
}

#[derive(Default)]
pub struct EnableParamsBuilder<'a> {
    includeWhitespace: Option<Cow<'a, str>>,
}

impl<'a> EnableParamsBuilder<'a> {
    /// Whether to include whitespaces in the children array of returned Nodes.
    pub fn includeWhitespace(mut self, includeWhitespace: impl Into<Cow<'a, str>>) -> Self { self.includeWhitespace = Some(includeWhitespace.into()); self }
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            includeWhitespace: self.includeWhitespace,
        }
    }
}

impl<'a> EnableParams<'a> { pub const METHOD: &'static str = "DOM.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams<'a> {
    const METHOD: &'static str = "DOM.enable";
    type Response = crate::EmptyReturns;
}

/// Focuses the given element.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FocusParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> FocusParams<'a> {
    pub fn builder() -> FocusParamsBuilder<'a> { FocusParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
}

#[derive(Default)]
pub struct FocusParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    backendNodeId: Option<BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> FocusParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    pub fn build(self) -> FocusParams<'a> {
        FocusParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
        }
    }
}

impl<'a> FocusParams<'a> { pub const METHOD: &'static str = "DOM.focus"; }

impl<'a> crate::CdpCommand<'a> for FocusParams<'a> {
    const METHOD: &'static str = "DOM.focus";
    type Response = crate::EmptyReturns;
}

/// Returns attributes for the specified node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAttributesParams {
    /// Id of the node to retrieve attributes for.
    nodeId: NodeId,
}

impl GetAttributesParams {
    pub fn builder() -> GetAttributesParamsBuilder { GetAttributesParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetAttributesParamsBuilder {
    nodeId: Option<NodeId>,
}

impl GetAttributesParamsBuilder {
    /// Id of the node to retrieve attributes for.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetAttributesParams {
        GetAttributesParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Returns attributes for the specified node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAttributesReturns<'a> {
    /// An interleaved array of node attribute names and values.
    attributes: Vec<Cow<'a, str>>,
}

impl<'a> GetAttributesReturns<'a> {
    pub fn builder() -> GetAttributesReturnsBuilder<'a> { GetAttributesReturnsBuilder::default() }
    pub fn attributes(&self) -> &[Cow<'a, str>] { &self.attributes }
}

#[derive(Default)]
pub struct GetAttributesReturnsBuilder<'a> {
    attributes: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GetAttributesReturnsBuilder<'a> {
    /// An interleaved array of node attribute names and values.
    pub fn attributes(mut self, attributes: Vec<Cow<'a, str>>) -> Self { self.attributes = Some(attributes); self }
    pub fn build(self) -> GetAttributesReturns<'a> {
        GetAttributesReturns {
            attributes: self.attributes.unwrap_or_default(),
        }
    }
}

impl GetAttributesParams { pub const METHOD: &'static str = "DOM.getAttributes"; }

impl<'a> crate::CdpCommand<'a> for GetAttributesParams {
    const METHOD: &'static str = "DOM.getAttributes";
    type Response = GetAttributesReturns<'a>;
}

/// Returns boxes for the given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBoxModelParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetBoxModelParams<'a> {
    pub fn builder() -> GetBoxModelParamsBuilder<'a> { GetBoxModelParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
}

#[derive(Default)]
pub struct GetBoxModelParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    backendNodeId: Option<BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetBoxModelParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    pub fn build(self) -> GetBoxModelParams<'a> {
        GetBoxModelParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
        }
    }
}

/// Returns boxes for the given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBoxModelReturns {
    /// Box model for the node.
    model: BoxModel,
}

impl GetBoxModelReturns {
    pub fn builder() -> GetBoxModelReturnsBuilder { GetBoxModelReturnsBuilder::default() }
    pub fn model(&self) -> &BoxModel { &self.model }
}

#[derive(Default)]
pub struct GetBoxModelReturnsBuilder {
    model: Option<BoxModel>,
}

impl GetBoxModelReturnsBuilder {
    /// Box model for the node.
    pub fn model(mut self, model: BoxModel) -> Self { self.model = Some(model); self }
    pub fn build(self) -> GetBoxModelReturns {
        GetBoxModelReturns {
            model: self.model.unwrap_or_default(),
        }
    }
}

impl<'a> GetBoxModelParams<'a> { pub const METHOD: &'static str = "DOM.getBoxModel"; }

impl<'a> crate::CdpCommand<'a> for GetBoxModelParams<'a> {
    const METHOD: &'static str = "DOM.getBoxModel";
    type Response = GetBoxModelReturns;
}

/// Returns quads that describe node position on the page. This method
/// might return multiple quads for inline nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetContentQuadsParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetContentQuadsParams<'a> {
    pub fn builder() -> GetContentQuadsParamsBuilder<'a> { GetContentQuadsParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
}

#[derive(Default)]
pub struct GetContentQuadsParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    backendNodeId: Option<BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetContentQuadsParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    pub fn build(self) -> GetContentQuadsParams<'a> {
        GetContentQuadsParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
        }
    }
}

/// Returns quads that describe node position on the page. This method
/// might return multiple quads for inline nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetContentQuadsReturns {
    /// Quads that describe node layout relative to viewport.
    quads: Vec<Quad>,
}

impl GetContentQuadsReturns {
    pub fn builder() -> GetContentQuadsReturnsBuilder { GetContentQuadsReturnsBuilder::default() }
    pub fn quads(&self) -> &[Quad] { &self.quads }
}

#[derive(Default)]
pub struct GetContentQuadsReturnsBuilder {
    quads: Option<Vec<Quad>>,
}

impl GetContentQuadsReturnsBuilder {
    /// Quads that describe node layout relative to viewport.
    pub fn quads(mut self, quads: Vec<Quad>) -> Self { self.quads = Some(quads); self }
    pub fn build(self) -> GetContentQuadsReturns {
        GetContentQuadsReturns {
            quads: self.quads.unwrap_or_default(),
        }
    }
}

impl<'a> GetContentQuadsParams<'a> { pub const METHOD: &'static str = "DOM.getContentQuads"; }

impl<'a> crate::CdpCommand<'a> for GetContentQuadsParams<'a> {
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
    depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pierce: Option<bool>,
}

impl GetDocumentParams {
    pub fn builder() -> GetDocumentParamsBuilder { GetDocumentParamsBuilder::default() }
    pub fn depth(&self) -> Option<i64> { self.depth }
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}

#[derive(Default)]
pub struct GetDocumentParamsBuilder {
    depth: Option<i64>,
    pierce: Option<bool>,
}

impl GetDocumentParamsBuilder {
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> GetDocumentParams {
        GetDocumentParams {
            depth: self.depth,
            pierce: self.pierce,
        }
    }
}

/// Returns the root DOM node (and optionally the subtree) to the caller.
/// Implicitly enables the DOM domain events for the current target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDocumentReturns<'a> {
    /// Resulting node.
    root: Node<'a>,
}

impl<'a> GetDocumentReturns<'a> {
    pub fn builder() -> GetDocumentReturnsBuilder<'a> { GetDocumentReturnsBuilder::default() }
    pub fn root(&self) -> &Node<'a> { &self.root }
}

#[derive(Default)]
pub struct GetDocumentReturnsBuilder<'a> {
    root: Option<Node<'a>>,
}

impl<'a> GetDocumentReturnsBuilder<'a> {
    /// Resulting node.
    pub fn root(mut self, root: Node<'a>) -> Self { self.root = Some(root); self }
    pub fn build(self) -> GetDocumentReturns<'a> {
        GetDocumentReturns {
            root: self.root.unwrap_or_default(),
        }
    }
}

impl GetDocumentParams { pub const METHOD: &'static str = "DOM.getDocument"; }

impl<'a> crate::CdpCommand<'a> for GetDocumentParams {
    const METHOD: &'static str = "DOM.getDocument";
    type Response = GetDocumentReturns<'a>;
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
    depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pierce: Option<bool>,
}

impl GetFlattenedDocumentParams {
    pub fn builder() -> GetFlattenedDocumentParamsBuilder { GetFlattenedDocumentParamsBuilder::default() }
    pub fn depth(&self) -> Option<i64> { self.depth }
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}

#[derive(Default)]
pub struct GetFlattenedDocumentParamsBuilder {
    depth: Option<i64>,
    pierce: Option<bool>,
}

impl GetFlattenedDocumentParamsBuilder {
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> GetFlattenedDocumentParams {
        GetFlattenedDocumentParams {
            depth: self.depth,
            pierce: self.pierce,
        }
    }
}

/// Returns the root DOM node (and optionally the subtree) to the caller.
/// Deprecated, as it is not designed to work well with the rest of the DOM agent.
/// Use DOMSnapshot.captureSnapshot instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFlattenedDocumentReturns<'a> {
    /// Resulting node.
    nodes: Vec<Node<'a>>,
}

impl<'a> GetFlattenedDocumentReturns<'a> {
    pub fn builder() -> GetFlattenedDocumentReturnsBuilder<'a> { GetFlattenedDocumentReturnsBuilder::default() }
    pub fn nodes(&self) -> &[Node<'a>] { &self.nodes }
}

#[derive(Default)]
pub struct GetFlattenedDocumentReturnsBuilder<'a> {
    nodes: Option<Vec<Node<'a>>>,
}

impl<'a> GetFlattenedDocumentReturnsBuilder<'a> {
    /// Resulting node.
    pub fn nodes(mut self, nodes: Vec<Node<'a>>) -> Self { self.nodes = Some(nodes); self }
    pub fn build(self) -> GetFlattenedDocumentReturns<'a> {
        GetFlattenedDocumentReturns {
            nodes: self.nodes.unwrap_or_default(),
        }
    }
}

impl GetFlattenedDocumentParams { pub const METHOD: &'static str = "DOM.getFlattenedDocument"; }

impl<'a> crate::CdpCommand<'a> for GetFlattenedDocumentParams {
    const METHOD: &'static str = "DOM.getFlattenedDocument";
    type Response = GetFlattenedDocumentReturns<'a>;
}

/// Finds nodes with a given computed style in a subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodesForSubtreeByStyleParams<'a> {
    /// Node ID pointing to the root of a subtree.
    nodeId: NodeId,
    /// The style to filter nodes by (includes nodes if any of properties matches).
    computedStyles: Vec<CSSComputedStyleProperty<'a>>,
    /// Whether or not iframes and shadow roots in the same target should be traversed when returning the
    /// results (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pierce: Option<bool>,
}

impl<'a> GetNodesForSubtreeByStyleParams<'a> {
    pub fn builder() -> GetNodesForSubtreeByStyleParamsBuilder<'a> { GetNodesForSubtreeByStyleParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn computedStyles(&self) -> &[CSSComputedStyleProperty<'a>] { &self.computedStyles }
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}

#[derive(Default)]
pub struct GetNodesForSubtreeByStyleParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    computedStyles: Option<Vec<CSSComputedStyleProperty<'a>>>,
    pierce: Option<bool>,
}

impl<'a> GetNodesForSubtreeByStyleParamsBuilder<'a> {
    /// Node ID pointing to the root of a subtree.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// The style to filter nodes by (includes nodes if any of properties matches).
    pub fn computedStyles(mut self, computedStyles: Vec<CSSComputedStyleProperty<'a>>) -> Self { self.computedStyles = Some(computedStyles); self }
    /// Whether or not iframes and shadow roots in the same target should be traversed when returning the
    /// results (default is false).
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> GetNodesForSubtreeByStyleParams<'a> {
        GetNodesForSubtreeByStyleParams {
            nodeId: self.nodeId.unwrap_or_default(),
            computedStyles: self.computedStyles.unwrap_or_default(),
            pierce: self.pierce,
        }
    }
}

/// Finds nodes with a given computed style in a subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodesForSubtreeByStyleReturns {
    /// Resulting nodes.
    nodeIds: Vec<NodeId>,
}

impl GetNodesForSubtreeByStyleReturns {
    pub fn builder() -> GetNodesForSubtreeByStyleReturnsBuilder { GetNodesForSubtreeByStyleReturnsBuilder::default() }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}

#[derive(Default)]
pub struct GetNodesForSubtreeByStyleReturnsBuilder {
    nodeIds: Option<Vec<NodeId>>,
}

impl GetNodesForSubtreeByStyleReturnsBuilder {
    /// Resulting nodes.
    pub fn nodeIds(mut self, nodeIds: Vec<NodeId>) -> Self { self.nodeIds = Some(nodeIds); self }
    pub fn build(self) -> GetNodesForSubtreeByStyleReturns {
        GetNodesForSubtreeByStyleReturns {
            nodeIds: self.nodeIds.unwrap_or_default(),
        }
    }
}

impl<'a> GetNodesForSubtreeByStyleParams<'a> { pub const METHOD: &'static str = "DOM.getNodesForSubtreeByStyle"; }

impl<'a> crate::CdpCommand<'a> for GetNodesForSubtreeByStyleParams<'a> {
    const METHOD: &'static str = "DOM.getNodesForSubtreeByStyle";
    type Response = GetNodesForSubtreeByStyleReturns;
}

/// Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
/// either returned or not.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeForLocationParams {
    /// X coordinate.
    x: i32,
    /// Y coordinate.
    y: i32,
    /// False to skip to the nearest non-UA shadow root ancestor (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    includeUserAgentShadowDOM: Option<bool>,
    /// Whether to ignore pointer-events: none on elements and hit test them.
    #[serde(skip_serializing_if = "Option::is_none")]
    ignorePointerEventsNone: Option<bool>,
}

impl GetNodeForLocationParams {
    pub fn builder() -> GetNodeForLocationParamsBuilder { GetNodeForLocationParamsBuilder::default() }
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn includeUserAgentShadowDOM(&self) -> Option<bool> { self.includeUserAgentShadowDOM }
    pub fn ignorePointerEventsNone(&self) -> Option<bool> { self.ignorePointerEventsNone }
}

#[derive(Default)]
pub struct GetNodeForLocationParamsBuilder {
    x: Option<i32>,
    y: Option<i32>,
    includeUserAgentShadowDOM: Option<bool>,
    ignorePointerEventsNone: Option<bool>,
}

impl GetNodeForLocationParamsBuilder {
    /// X coordinate.
    pub fn x(mut self, x: i32) -> Self { self.x = Some(x); self }
    /// Y coordinate.
    pub fn y(mut self, y: i32) -> Self { self.y = Some(y); self }
    /// False to skip to the nearest non-UA shadow root ancestor (default: false).
    pub fn includeUserAgentShadowDOM(mut self, includeUserAgentShadowDOM: bool) -> Self { self.includeUserAgentShadowDOM = Some(includeUserAgentShadowDOM); self }
    /// Whether to ignore pointer-events: none on elements and hit test them.
    pub fn ignorePointerEventsNone(mut self, ignorePointerEventsNone: bool) -> Self { self.ignorePointerEventsNone = Some(ignorePointerEventsNone); self }
    pub fn build(self) -> GetNodeForLocationParams {
        GetNodeForLocationParams {
            x: self.x.unwrap_or_default(),
            y: self.y.unwrap_or_default(),
            includeUserAgentShadowDOM: self.includeUserAgentShadowDOM,
            ignorePointerEventsNone: self.ignorePointerEventsNone,
        }
    }
}

/// Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
/// either returned or not.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeForLocationReturns<'a> {
    /// Resulting node.
    backendNodeId: BackendNodeId,
    /// Frame this node belongs to.
    frameId: crate::page::FrameId<'a>,
    /// Id of the node at given coordinates, only when enabled and requested document.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
}

impl<'a> GetNodeForLocationReturns<'a> {
    pub fn builder() -> GetNodeForLocationReturnsBuilder<'a> { GetNodeForLocationReturnsBuilder::default() }
    pub fn backendNodeId(&self) -> &BackendNodeId { &self.backendNodeId }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
}

#[derive(Default)]
pub struct GetNodeForLocationReturnsBuilder<'a> {
    backendNodeId: Option<BackendNodeId>,
    frameId: Option<crate::page::FrameId<'a>>,
    nodeId: Option<NodeId>,
}

impl<'a> GetNodeForLocationReturnsBuilder<'a> {
    /// Resulting node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// Frame this node belongs to.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// Id of the node at given coordinates, only when enabled and requested document.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetNodeForLocationReturns<'a> {
        GetNodeForLocationReturns {
            backendNodeId: self.backendNodeId.unwrap_or_default(),
            frameId: self.frameId.unwrap_or_default(),
            nodeId: self.nodeId,
        }
    }
}

impl GetNodeForLocationParams { pub const METHOD: &'static str = "DOM.getNodeForLocation"; }

impl<'a> crate::CdpCommand<'a> for GetNodeForLocationParams {
    const METHOD: &'static str = "DOM.getNodeForLocation";
    type Response = GetNodeForLocationReturns<'a>;
}

/// Returns node's HTML markup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOuterHTMLParams<'a> {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Include all shadow roots. Equals to false if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    includeShadowDOM: Option<bool>,
}

impl<'a> GetOuterHTMLParams<'a> {
    pub fn builder() -> GetOuterHTMLParamsBuilder<'a> { GetOuterHTMLParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
    pub fn includeShadowDOM(&self) -> Option<bool> { self.includeShadowDOM }
}

#[derive(Default)]
pub struct GetOuterHTMLParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    backendNodeId: Option<BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    includeShadowDOM: Option<bool>,
}

impl<'a> GetOuterHTMLParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    /// Include all shadow roots. Equals to false if not specified.
    pub fn includeShadowDOM(mut self, includeShadowDOM: bool) -> Self { self.includeShadowDOM = Some(includeShadowDOM); self }
    pub fn build(self) -> GetOuterHTMLParams<'a> {
        GetOuterHTMLParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
            includeShadowDOM: self.includeShadowDOM,
        }
    }
}

/// Returns node's HTML markup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOuterHTMLReturns<'a> {
    /// Outer HTML markup.
    outerHTML: Cow<'a, str>,
}

impl<'a> GetOuterHTMLReturns<'a> {
    pub fn builder() -> GetOuterHTMLReturnsBuilder<'a> { GetOuterHTMLReturnsBuilder::default() }
    pub fn outerHTML(&self) -> &str { self.outerHTML.as_ref() }
}

#[derive(Default)]
pub struct GetOuterHTMLReturnsBuilder<'a> {
    outerHTML: Option<Cow<'a, str>>,
}

impl<'a> GetOuterHTMLReturnsBuilder<'a> {
    /// Outer HTML markup.
    pub fn outerHTML(mut self, outerHTML: impl Into<Cow<'a, str>>) -> Self { self.outerHTML = Some(outerHTML.into()); self }
    pub fn build(self) -> GetOuterHTMLReturns<'a> {
        GetOuterHTMLReturns {
            outerHTML: self.outerHTML.unwrap_or_default(),
        }
    }
}

impl<'a> GetOuterHTMLParams<'a> { pub const METHOD: &'static str = "DOM.getOuterHTML"; }

impl<'a> crate::CdpCommand<'a> for GetOuterHTMLParams<'a> {
    const METHOD: &'static str = "DOM.getOuterHTML";
    type Response = GetOuterHTMLReturns<'a>;
}

/// Returns the id of the nearest ancestor that is a relayout boundary.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRelayoutBoundaryParams {
    /// Id of the node.
    nodeId: NodeId,
}

impl GetRelayoutBoundaryParams {
    pub fn builder() -> GetRelayoutBoundaryParamsBuilder { GetRelayoutBoundaryParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetRelayoutBoundaryParamsBuilder {
    nodeId: Option<NodeId>,
}

impl GetRelayoutBoundaryParamsBuilder {
    /// Id of the node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetRelayoutBoundaryParams {
        GetRelayoutBoundaryParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Returns the id of the nearest ancestor that is a relayout boundary.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRelayoutBoundaryReturns {
    /// Relayout boundary node id for the given node.
    nodeId: NodeId,
}

impl GetRelayoutBoundaryReturns {
    pub fn builder() -> GetRelayoutBoundaryReturnsBuilder { GetRelayoutBoundaryReturnsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetRelayoutBoundaryReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl GetRelayoutBoundaryReturnsBuilder {
    /// Relayout boundary node id for the given node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetRelayoutBoundaryReturns {
        GetRelayoutBoundaryReturns {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl GetRelayoutBoundaryParams { pub const METHOD: &'static str = "DOM.getRelayoutBoundary"; }

impl<'a> crate::CdpCommand<'a> for GetRelayoutBoundaryParams {
    const METHOD: &'static str = "DOM.getRelayoutBoundary";
    type Response = GetRelayoutBoundaryReturns;
}

/// Returns search results from given 'fromIndex' to given 'toIndex' from the search with the given
/// identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSearchResultsParams<'a> {
    /// Unique search session identifier.
    searchId: Cow<'a, str>,
    /// Start index of the search result to be returned.
    fromIndex: u64,
    /// End index of the search result to be returned.
    toIndex: u64,
}

impl<'a> GetSearchResultsParams<'a> {
    pub fn builder() -> GetSearchResultsParamsBuilder<'a> { GetSearchResultsParamsBuilder::default() }
    pub fn searchId(&self) -> &str { self.searchId.as_ref() }
    pub fn fromIndex(&self) -> u64 { self.fromIndex }
    pub fn toIndex(&self) -> u64 { self.toIndex }
}

#[derive(Default)]
pub struct GetSearchResultsParamsBuilder<'a> {
    searchId: Option<Cow<'a, str>>,
    fromIndex: Option<u64>,
    toIndex: Option<u64>,
}

impl<'a> GetSearchResultsParamsBuilder<'a> {
    /// Unique search session identifier.
    pub fn searchId(mut self, searchId: impl Into<Cow<'a, str>>) -> Self { self.searchId = Some(searchId.into()); self }
    /// Start index of the search result to be returned.
    pub fn fromIndex(mut self, fromIndex: u64) -> Self { self.fromIndex = Some(fromIndex); self }
    /// End index of the search result to be returned.
    pub fn toIndex(mut self, toIndex: u64) -> Self { self.toIndex = Some(toIndex); self }
    pub fn build(self) -> GetSearchResultsParams<'a> {
        GetSearchResultsParams {
            searchId: self.searchId.unwrap_or_default(),
            fromIndex: self.fromIndex.unwrap_or_default(),
            toIndex: self.toIndex.unwrap_or_default(),
        }
    }
}

/// Returns search results from given 'fromIndex' to given 'toIndex' from the search with the given
/// identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSearchResultsReturns {
    /// Ids of the search result nodes.
    nodeIds: Vec<NodeId>,
}

impl GetSearchResultsReturns {
    pub fn builder() -> GetSearchResultsReturnsBuilder { GetSearchResultsReturnsBuilder::default() }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}

#[derive(Default)]
pub struct GetSearchResultsReturnsBuilder {
    nodeIds: Option<Vec<NodeId>>,
}

impl GetSearchResultsReturnsBuilder {
    /// Ids of the search result nodes.
    pub fn nodeIds(mut self, nodeIds: Vec<NodeId>) -> Self { self.nodeIds = Some(nodeIds); self }
    pub fn build(self) -> GetSearchResultsReturns {
        GetSearchResultsReturns {
            nodeIds: self.nodeIds.unwrap_or_default(),
        }
    }
}

impl<'a> GetSearchResultsParams<'a> { pub const METHOD: &'static str = "DOM.getSearchResults"; }

impl<'a> crate::CdpCommand<'a> for GetSearchResultsParams<'a> {
    const METHOD: &'static str = "DOM.getSearchResults";
    type Response = GetSearchResultsReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HideHighlightParams {}

impl HideHighlightParams {
    pub fn builder() -> HideHighlightParamsBuilder {
        HideHighlightParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct HideHighlightParamsBuilder {}

impl HideHighlightParamsBuilder {
    pub fn build(self) -> HideHighlightParams {
        HideHighlightParams {}
    }
}

impl HideHighlightParams { pub const METHOD: &'static str = "DOM.hideHighlight"; }

impl<'a> crate::CdpCommand<'a> for HideHighlightParams {
    const METHOD: &'static str = "DOM.hideHighlight";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighlightNodeParams {}

impl HighlightNodeParams {
    pub fn builder() -> HighlightNodeParamsBuilder {
        HighlightNodeParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct HighlightNodeParamsBuilder {}

impl HighlightNodeParamsBuilder {
    pub fn build(self) -> HighlightNodeParams {
        HighlightNodeParams {}
    }
}

impl HighlightNodeParams { pub const METHOD: &'static str = "DOM.highlightNode"; }

impl<'a> crate::CdpCommand<'a> for HighlightNodeParams {
    const METHOD: &'static str = "DOM.highlightNode";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighlightRectParams {}

impl HighlightRectParams {
    pub fn builder() -> HighlightRectParamsBuilder {
        HighlightRectParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct HighlightRectParamsBuilder {}

impl HighlightRectParamsBuilder {
    pub fn build(self) -> HighlightRectParams {
        HighlightRectParams {}
    }
}

impl HighlightRectParams { pub const METHOD: &'static str = "DOM.highlightRect"; }

impl<'a> crate::CdpCommand<'a> for HighlightRectParams {
    const METHOD: &'static str = "DOM.highlightRect";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkUndoableStateParams {}

impl MarkUndoableStateParams {
    pub fn builder() -> MarkUndoableStateParamsBuilder {
        MarkUndoableStateParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct MarkUndoableStateParamsBuilder {}

impl MarkUndoableStateParamsBuilder {
    pub fn build(self) -> MarkUndoableStateParams {
        MarkUndoableStateParams {}
    }
}

impl MarkUndoableStateParams { pub const METHOD: &'static str = "DOM.markUndoableState"; }

impl<'a> crate::CdpCommand<'a> for MarkUndoableStateParams {
    const METHOD: &'static str = "DOM.markUndoableState";
    type Response = crate::EmptyReturns;
}

/// Moves node into the new container, places it before the given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MoveToParams {
    /// Id of the node to move.
    nodeId: NodeId,
    /// Id of the element to drop the moved node into.
    targetNodeId: NodeId,
    /// Drop node before this one (if absent, the moved node becomes the last child of
    /// 'targetNodeId').
    #[serde(skip_serializing_if = "Option::is_none")]
    insertBeforeNodeId: Option<NodeId>,
}

impl MoveToParams {
    pub fn builder() -> MoveToParamsBuilder { MoveToParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn targetNodeId(&self) -> &NodeId { &self.targetNodeId }
    pub fn insertBeforeNodeId(&self) -> Option<&NodeId> { self.insertBeforeNodeId.as_ref() }
}

#[derive(Default)]
pub struct MoveToParamsBuilder {
    nodeId: Option<NodeId>,
    targetNodeId: Option<NodeId>,
    insertBeforeNodeId: Option<NodeId>,
}

impl MoveToParamsBuilder {
    /// Id of the node to move.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Id of the element to drop the moved node into.
    pub fn targetNodeId(mut self, targetNodeId: NodeId) -> Self { self.targetNodeId = Some(targetNodeId); self }
    /// Drop node before this one (if absent, the moved node becomes the last child of
    /// 'targetNodeId').
    pub fn insertBeforeNodeId(mut self, insertBeforeNodeId: NodeId) -> Self { self.insertBeforeNodeId = Some(insertBeforeNodeId); self }
    pub fn build(self) -> MoveToParams {
        MoveToParams {
            nodeId: self.nodeId.unwrap_or_default(),
            targetNodeId: self.targetNodeId.unwrap_or_default(),
            insertBeforeNodeId: self.insertBeforeNodeId,
        }
    }
}

/// Moves node into the new container, places it before the given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MoveToReturns {
    /// New id of the moved node.
    nodeId: NodeId,
}

impl MoveToReturns {
    pub fn builder() -> MoveToReturnsBuilder { MoveToReturnsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct MoveToReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl MoveToReturnsBuilder {
    /// New id of the moved node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> MoveToReturns {
        MoveToReturns {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl MoveToParams { pub const METHOD: &'static str = "DOM.moveTo"; }

impl<'a> crate::CdpCommand<'a> for MoveToParams {
    const METHOD: &'static str = "DOM.moveTo";
    type Response = MoveToReturns;
}

/// Searches for a given string in the DOM tree. Use 'getSearchResults' to access search results or
/// 'cancelSearch' to end this search session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerformSearchParams<'a> {
    /// Plain text or query selector or XPath search query.
    query: Cow<'a, str>,
    /// True to search in user agent shadow DOM.
    #[serde(skip_serializing_if = "Option::is_none")]
    includeUserAgentShadowDOM: Option<bool>,
}

impl<'a> PerformSearchParams<'a> {
    pub fn builder() -> PerformSearchParamsBuilder<'a> { PerformSearchParamsBuilder::default() }
    pub fn query(&self) -> &str { self.query.as_ref() }
    pub fn includeUserAgentShadowDOM(&self) -> Option<bool> { self.includeUserAgentShadowDOM }
}

#[derive(Default)]
pub struct PerformSearchParamsBuilder<'a> {
    query: Option<Cow<'a, str>>,
    includeUserAgentShadowDOM: Option<bool>,
}

impl<'a> PerformSearchParamsBuilder<'a> {
    /// Plain text or query selector or XPath search query.
    pub fn query(mut self, query: impl Into<Cow<'a, str>>) -> Self { self.query = Some(query.into()); self }
    /// True to search in user agent shadow DOM.
    pub fn includeUserAgentShadowDOM(mut self, includeUserAgentShadowDOM: bool) -> Self { self.includeUserAgentShadowDOM = Some(includeUserAgentShadowDOM); self }
    pub fn build(self) -> PerformSearchParams<'a> {
        PerformSearchParams {
            query: self.query.unwrap_or_default(),
            includeUserAgentShadowDOM: self.includeUserAgentShadowDOM,
        }
    }
}

/// Searches for a given string in the DOM tree. Use 'getSearchResults' to access search results or
/// 'cancelSearch' to end this search session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerformSearchReturns<'a> {
    /// Unique search session identifier.
    searchId: Cow<'a, str>,
    /// Number of search results.
    resultCount: u64,
}

impl<'a> PerformSearchReturns<'a> {
    pub fn builder() -> PerformSearchReturnsBuilder<'a> { PerformSearchReturnsBuilder::default() }
    pub fn searchId(&self) -> &str { self.searchId.as_ref() }
    pub fn resultCount(&self) -> u64 { self.resultCount }
}

#[derive(Default)]
pub struct PerformSearchReturnsBuilder<'a> {
    searchId: Option<Cow<'a, str>>,
    resultCount: Option<u64>,
}

impl<'a> PerformSearchReturnsBuilder<'a> {
    /// Unique search session identifier.
    pub fn searchId(mut self, searchId: impl Into<Cow<'a, str>>) -> Self { self.searchId = Some(searchId.into()); self }
    /// Number of search results.
    pub fn resultCount(mut self, resultCount: u64) -> Self { self.resultCount = Some(resultCount); self }
    pub fn build(self) -> PerformSearchReturns<'a> {
        PerformSearchReturns {
            searchId: self.searchId.unwrap_or_default(),
            resultCount: self.resultCount.unwrap_or_default(),
        }
    }
}

impl<'a> PerformSearchParams<'a> { pub const METHOD: &'static str = "DOM.performSearch"; }

impl<'a> crate::CdpCommand<'a> for PerformSearchParams<'a> {
    const METHOD: &'static str = "DOM.performSearch";
    type Response = PerformSearchReturns<'a>;
}

/// Requests that the node is sent to the caller given its path. // FIXME, use XPath

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PushNodeByPathToFrontendParams<'a> {
    /// Path to node in the proprietary format.
    path: Cow<'a, str>,
}

impl<'a> PushNodeByPathToFrontendParams<'a> {
    pub fn builder() -> PushNodeByPathToFrontendParamsBuilder<'a> { PushNodeByPathToFrontendParamsBuilder::default() }
    pub fn path(&self) -> &str { self.path.as_ref() }
}

#[derive(Default)]
pub struct PushNodeByPathToFrontendParamsBuilder<'a> {
    path: Option<Cow<'a, str>>,
}

impl<'a> PushNodeByPathToFrontendParamsBuilder<'a> {
    /// Path to node in the proprietary format.
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    pub fn build(self) -> PushNodeByPathToFrontendParams<'a> {
        PushNodeByPathToFrontendParams {
            path: self.path.unwrap_or_default(),
        }
    }
}

/// Requests that the node is sent to the caller given its path. // FIXME, use XPath

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PushNodeByPathToFrontendReturns {
    /// Id of the node for given path.
    nodeId: NodeId,
}

impl PushNodeByPathToFrontendReturns {
    pub fn builder() -> PushNodeByPathToFrontendReturnsBuilder { PushNodeByPathToFrontendReturnsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct PushNodeByPathToFrontendReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl PushNodeByPathToFrontendReturnsBuilder {
    /// Id of the node for given path.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> PushNodeByPathToFrontendReturns {
        PushNodeByPathToFrontendReturns {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl<'a> PushNodeByPathToFrontendParams<'a> { pub const METHOD: &'static str = "DOM.pushNodeByPathToFrontend"; }

impl<'a> crate::CdpCommand<'a> for PushNodeByPathToFrontendParams<'a> {
    const METHOD: &'static str = "DOM.pushNodeByPathToFrontend";
    type Response = PushNodeByPathToFrontendReturns;
}

/// Requests that a batch of nodes is sent to the caller given their backend node ids.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PushNodesByBackendIdsToFrontendParams {
    /// The array of backend node ids.
    backendNodeIds: Vec<BackendNodeId>,
}

impl PushNodesByBackendIdsToFrontendParams {
    pub fn builder() -> PushNodesByBackendIdsToFrontendParamsBuilder { PushNodesByBackendIdsToFrontendParamsBuilder::default() }
    pub fn backendNodeIds(&self) -> &[BackendNodeId] { &self.backendNodeIds }
}

#[derive(Default)]
pub struct PushNodesByBackendIdsToFrontendParamsBuilder {
    backendNodeIds: Option<Vec<BackendNodeId>>,
}

impl PushNodesByBackendIdsToFrontendParamsBuilder {
    /// The array of backend node ids.
    pub fn backendNodeIds(mut self, backendNodeIds: Vec<BackendNodeId>) -> Self { self.backendNodeIds = Some(backendNodeIds); self }
    pub fn build(self) -> PushNodesByBackendIdsToFrontendParams {
        PushNodesByBackendIdsToFrontendParams {
            backendNodeIds: self.backendNodeIds.unwrap_or_default(),
        }
    }
}

/// Requests that a batch of nodes is sent to the caller given their backend node ids.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PushNodesByBackendIdsToFrontendReturns {
    /// The array of ids of pushed nodes that correspond to the backend ids specified in
    /// backendNodeIds.
    nodeIds: Vec<NodeId>,
}

impl PushNodesByBackendIdsToFrontendReturns {
    pub fn builder() -> PushNodesByBackendIdsToFrontendReturnsBuilder { PushNodesByBackendIdsToFrontendReturnsBuilder::default() }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}

#[derive(Default)]
pub struct PushNodesByBackendIdsToFrontendReturnsBuilder {
    nodeIds: Option<Vec<NodeId>>,
}

impl PushNodesByBackendIdsToFrontendReturnsBuilder {
    /// The array of ids of pushed nodes that correspond to the backend ids specified in
    /// backendNodeIds.
    pub fn nodeIds(mut self, nodeIds: Vec<NodeId>) -> Self { self.nodeIds = Some(nodeIds); self }
    pub fn build(self) -> PushNodesByBackendIdsToFrontendReturns {
        PushNodesByBackendIdsToFrontendReturns {
            nodeIds: self.nodeIds.unwrap_or_default(),
        }
    }
}

impl PushNodesByBackendIdsToFrontendParams { pub const METHOD: &'static str = "DOM.pushNodesByBackendIdsToFrontend"; }

impl<'a> crate::CdpCommand<'a> for PushNodesByBackendIdsToFrontendParams {
    const METHOD: &'static str = "DOM.pushNodesByBackendIdsToFrontend";
    type Response = PushNodesByBackendIdsToFrontendReturns;
}

/// Executes 'querySelector' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorParams<'a> {
    /// Id of the node to query upon.
    nodeId: NodeId,
    /// Selector string.
    selector: Cow<'a, str>,
}

impl<'a> QuerySelectorParams<'a> {
    pub fn builder() -> QuerySelectorParamsBuilder<'a> { QuerySelectorParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn selector(&self) -> &str { self.selector.as_ref() }
}

#[derive(Default)]
pub struct QuerySelectorParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    selector: Option<Cow<'a, str>>,
}

impl<'a> QuerySelectorParamsBuilder<'a> {
    /// Id of the node to query upon.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Selector string.
    pub fn selector(mut self, selector: impl Into<Cow<'a, str>>) -> Self { self.selector = Some(selector.into()); self }
    pub fn build(self) -> QuerySelectorParams<'a> {
        QuerySelectorParams {
            nodeId: self.nodeId.unwrap_or_default(),
            selector: self.selector.unwrap_or_default(),
        }
    }
}

/// Executes 'querySelector' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorReturns {
    /// Query selector result.
    nodeId: NodeId,
}

impl QuerySelectorReturns {
    pub fn builder() -> QuerySelectorReturnsBuilder { QuerySelectorReturnsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct QuerySelectorReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl QuerySelectorReturnsBuilder {
    /// Query selector result.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> QuerySelectorReturns {
        QuerySelectorReturns {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl<'a> QuerySelectorParams<'a> { pub const METHOD: &'static str = "DOM.querySelector"; }

impl<'a> crate::CdpCommand<'a> for QuerySelectorParams<'a> {
    const METHOD: &'static str = "DOM.querySelector";
    type Response = QuerySelectorReturns;
}

/// Executes 'querySelectorAll' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorAllParams<'a> {
    /// Id of the node to query upon.
    nodeId: NodeId,
    /// Selector string.
    selector: Cow<'a, str>,
}

impl<'a> QuerySelectorAllParams<'a> {
    pub fn builder() -> QuerySelectorAllParamsBuilder<'a> { QuerySelectorAllParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn selector(&self) -> &str { self.selector.as_ref() }
}

#[derive(Default)]
pub struct QuerySelectorAllParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    selector: Option<Cow<'a, str>>,
}

impl<'a> QuerySelectorAllParamsBuilder<'a> {
    /// Id of the node to query upon.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Selector string.
    pub fn selector(mut self, selector: impl Into<Cow<'a, str>>) -> Self { self.selector = Some(selector.into()); self }
    pub fn build(self) -> QuerySelectorAllParams<'a> {
        QuerySelectorAllParams {
            nodeId: self.nodeId.unwrap_or_default(),
            selector: self.selector.unwrap_or_default(),
        }
    }
}

/// Executes 'querySelectorAll' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorAllReturns {
    /// Query selector result.
    nodeIds: Vec<NodeId>,
}

impl QuerySelectorAllReturns {
    pub fn builder() -> QuerySelectorAllReturnsBuilder { QuerySelectorAllReturnsBuilder::default() }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}

#[derive(Default)]
pub struct QuerySelectorAllReturnsBuilder {
    nodeIds: Option<Vec<NodeId>>,
}

impl QuerySelectorAllReturnsBuilder {
    /// Query selector result.
    pub fn nodeIds(mut self, nodeIds: Vec<NodeId>) -> Self { self.nodeIds = Some(nodeIds); self }
    pub fn build(self) -> QuerySelectorAllReturns {
        QuerySelectorAllReturns {
            nodeIds: self.nodeIds.unwrap_or_default(),
        }
    }
}

impl<'a> QuerySelectorAllParams<'a> { pub const METHOD: &'static str = "DOM.querySelectorAll"; }

impl<'a> crate::CdpCommand<'a> for QuerySelectorAllParams<'a> {
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
    nodeIds: Vec<NodeId>,
}

impl GetTopLayerElementsReturns {
    pub fn builder() -> GetTopLayerElementsReturnsBuilder { GetTopLayerElementsReturnsBuilder::default() }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}

#[derive(Default)]
pub struct GetTopLayerElementsReturnsBuilder {
    nodeIds: Option<Vec<NodeId>>,
}

impl GetTopLayerElementsReturnsBuilder {
    /// NodeIds of top layer elements
    pub fn nodeIds(mut self, nodeIds: Vec<NodeId>) -> Self { self.nodeIds = Some(nodeIds); self }
    pub fn build(self) -> GetTopLayerElementsReturns {
        GetTopLayerElementsReturns {
            nodeIds: self.nodeIds.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTopLayerElementsParams {}

impl GetTopLayerElementsParams {
    pub fn builder() -> GetTopLayerElementsParamsBuilder {
        GetTopLayerElementsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetTopLayerElementsParamsBuilder {}

impl GetTopLayerElementsParamsBuilder {
    pub fn build(self) -> GetTopLayerElementsParams {
        GetTopLayerElementsParams {}
    }
}

impl GetTopLayerElementsParams { pub const METHOD: &'static str = "DOM.getTopLayerElements"; }

impl<'a> crate::CdpCommand<'a> for GetTopLayerElementsParams {
    const METHOD: &'static str = "DOM.getTopLayerElements";
    type Response = GetTopLayerElementsReturns;
}

/// Returns the NodeId of the matched element according to certain relations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetElementByRelationParams<'a> {
    /// Id of the node from which to query the relation.
    nodeId: NodeId,
    /// Type of relation to get.
    relation: Cow<'a, str>,
}

impl<'a> GetElementByRelationParams<'a> {
    pub fn builder() -> GetElementByRelationParamsBuilder<'a> { GetElementByRelationParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn relation(&self) -> &str { self.relation.as_ref() }
}

#[derive(Default)]
pub struct GetElementByRelationParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    relation: Option<Cow<'a, str>>,
}

impl<'a> GetElementByRelationParamsBuilder<'a> {
    /// Id of the node from which to query the relation.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Type of relation to get.
    pub fn relation(mut self, relation: impl Into<Cow<'a, str>>) -> Self { self.relation = Some(relation.into()); self }
    pub fn build(self) -> GetElementByRelationParams<'a> {
        GetElementByRelationParams {
            nodeId: self.nodeId.unwrap_or_default(),
            relation: self.relation.unwrap_or_default(),
        }
    }
}

/// Returns the NodeId of the matched element according to certain relations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetElementByRelationReturns {
    /// NodeId of the element matching the queried relation.
    nodeId: NodeId,
}

impl GetElementByRelationReturns {
    pub fn builder() -> GetElementByRelationReturnsBuilder { GetElementByRelationReturnsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetElementByRelationReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl GetElementByRelationReturnsBuilder {
    /// NodeId of the element matching the queried relation.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetElementByRelationReturns {
        GetElementByRelationReturns {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl<'a> GetElementByRelationParams<'a> { pub const METHOD: &'static str = "DOM.getElementByRelation"; }

impl<'a> crate::CdpCommand<'a> for GetElementByRelationParams<'a> {
    const METHOD: &'static str = "DOM.getElementByRelation";
    type Response = GetElementByRelationReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedoParams {}

impl RedoParams {
    pub fn builder() -> RedoParamsBuilder {
        RedoParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct RedoParamsBuilder {}

impl RedoParamsBuilder {
    pub fn build(self) -> RedoParams {
        RedoParams {}
    }
}

impl RedoParams { pub const METHOD: &'static str = "DOM.redo"; }

impl<'a> crate::CdpCommand<'a> for RedoParams {
    const METHOD: &'static str = "DOM.redo";
    type Response = crate::EmptyReturns;
}

/// Removes attribute with given name from an element with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAttributeParams<'a> {
    /// Id of the element to remove attribute from.
    nodeId: NodeId,
    /// Name of the attribute to remove.
    name: Cow<'a, str>,
}

impl<'a> RemoveAttributeParams<'a> {
    pub fn builder() -> RemoveAttributeParamsBuilder<'a> { RemoveAttributeParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn name(&self) -> &str { self.name.as_ref() }
}

#[derive(Default)]
pub struct RemoveAttributeParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    name: Option<Cow<'a, str>>,
}

impl<'a> RemoveAttributeParamsBuilder<'a> {
    /// Id of the element to remove attribute from.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Name of the attribute to remove.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> RemoveAttributeParams<'a> {
        RemoveAttributeParams {
            nodeId: self.nodeId.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
        }
    }
}

impl<'a> RemoveAttributeParams<'a> { pub const METHOD: &'static str = "DOM.removeAttribute"; }

impl<'a> crate::CdpCommand<'a> for RemoveAttributeParams<'a> {
    const METHOD: &'static str = "DOM.removeAttribute";
    type Response = crate::EmptyReturns;
}

/// Removes node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveNodeParams {
    /// Id of the node to remove.
    nodeId: NodeId,
}

impl RemoveNodeParams {
    pub fn builder() -> RemoveNodeParamsBuilder { RemoveNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct RemoveNodeParamsBuilder {
    nodeId: Option<NodeId>,
}

impl RemoveNodeParamsBuilder {
    /// Id of the node to remove.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> RemoveNodeParams {
        RemoveNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl RemoveNodeParams { pub const METHOD: &'static str = "DOM.removeNode"; }

impl<'a> crate::CdpCommand<'a> for RemoveNodeParams {
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
    nodeId: NodeId,
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<i64>,
    /// Whether or not iframes and shadow roots should be traversed when returning the sub-tree
    /// (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pierce: Option<bool>,
}

impl RequestChildNodesParams {
    pub fn builder() -> RequestChildNodesParamsBuilder { RequestChildNodesParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn depth(&self) -> Option<i64> { self.depth }
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}

#[derive(Default)]
pub struct RequestChildNodesParamsBuilder {
    nodeId: Option<NodeId>,
    depth: Option<i64>,
    pierce: Option<bool>,
}

impl RequestChildNodesParamsBuilder {
    /// Id of the node to get children for.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// Whether or not iframes and shadow roots should be traversed when returning the sub-tree
    /// (default is false).
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> RequestChildNodesParams {
        RequestChildNodesParams {
            nodeId: self.nodeId.unwrap_or_default(),
            depth: self.depth,
            pierce: self.pierce,
        }
    }
}

impl RequestChildNodesParams { pub const METHOD: &'static str = "DOM.requestChildNodes"; }

impl<'a> crate::CdpCommand<'a> for RequestChildNodesParams {
    const METHOD: &'static str = "DOM.requestChildNodes";
    type Response = crate::EmptyReturns;
}

/// Requests that the node is sent to the caller given the JavaScript node object reference. All
/// nodes that form the path from the node to the root are also sent to the client as a series of
/// 'setChildNodes' notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestNodeParams<'a> {
    /// JavaScript object id to convert into node.
    objectId: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> RequestNodeParams<'a> {
    pub fn builder() -> RequestNodeParamsBuilder<'a> { RequestNodeParamsBuilder::default() }
    pub fn objectId(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.objectId }
}

#[derive(Default)]
pub struct RequestNodeParamsBuilder<'a> {
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> RequestNodeParamsBuilder<'a> {
    /// JavaScript object id to convert into node.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    pub fn build(self) -> RequestNodeParams<'a> {
        RequestNodeParams {
            objectId: self.objectId.unwrap_or_default(),
        }
    }
}

/// Requests that the node is sent to the caller given the JavaScript node object reference. All
/// nodes that form the path from the node to the root are also sent to the client as a series of
/// 'setChildNodes' notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestNodeReturns {
    /// Node id for given object.
    nodeId: NodeId,
}

impl RequestNodeReturns {
    pub fn builder() -> RequestNodeReturnsBuilder { RequestNodeReturnsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct RequestNodeReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl RequestNodeReturnsBuilder {
    /// Node id for given object.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> RequestNodeReturns {
        RequestNodeReturns {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl<'a> RequestNodeParams<'a> { pub const METHOD: &'static str = "DOM.requestNode"; }

impl<'a> crate::CdpCommand<'a> for RequestNodeParams<'a> {
    const METHOD: &'static str = "DOM.requestNode";
    type Response = RequestNodeReturns;
}

/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveNodeParams<'a> {
    /// Id of the node to resolve.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
    /// Backend identifier of the node to resolve.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectGroup: Option<Cow<'a, str>>,
    /// Execution context in which to resolve the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    executionContextId: Option<crate::runtime::ExecutionContextId>,
}

impl<'a> ResolveNodeParams<'a> {
    pub fn builder() -> ResolveNodeParamsBuilder<'a> { ResolveNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectGroup(&self) -> Option<&str> { self.objectGroup.as_deref() }
    pub fn executionContextId(&self) -> Option<&crate::runtime::ExecutionContextId> { self.executionContextId.as_ref() }
}

#[derive(Default)]
pub struct ResolveNodeParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    objectGroup: Option<Cow<'a, str>>,
    executionContextId: Option<crate::runtime::ExecutionContextId>,
}

impl<'a> ResolveNodeParamsBuilder<'a> {
    /// Id of the node to resolve.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Backend identifier of the node to resolve.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// Symbolic group name that can be used to release multiple objects.
    pub fn objectGroup(mut self, objectGroup: impl Into<Cow<'a, str>>) -> Self { self.objectGroup = Some(objectGroup.into()); self }
    /// Execution context in which to resolve the node.
    pub fn executionContextId(mut self, executionContextId: crate::runtime::ExecutionContextId) -> Self { self.executionContextId = Some(executionContextId); self }
    pub fn build(self) -> ResolveNodeParams<'a> {
        ResolveNodeParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectGroup: self.objectGroup,
            executionContextId: self.executionContextId,
        }
    }
}

/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveNodeReturns {
    /// JavaScript object wrapper for given node.
    object: crate::runtime::RemoteObject,
}

impl ResolveNodeReturns {
    pub fn builder() -> ResolveNodeReturnsBuilder { ResolveNodeReturnsBuilder::default() }
    pub fn object(&self) -> &crate::runtime::RemoteObject { &self.object }
}

#[derive(Default)]
pub struct ResolveNodeReturnsBuilder {
    object: Option<crate::runtime::RemoteObject>,
}

impl ResolveNodeReturnsBuilder {
    /// JavaScript object wrapper for given node.
    pub fn object(mut self, object: crate::runtime::RemoteObject) -> Self { self.object = Some(object); self }
    pub fn build(self) -> ResolveNodeReturns {
        ResolveNodeReturns {
            object: self.object.unwrap_or_default(),
        }
    }
}

impl<'a> ResolveNodeParams<'a> { pub const METHOD: &'static str = "DOM.resolveNode"; }

impl<'a> crate::CdpCommand<'a> for ResolveNodeParams<'a> {
    const METHOD: &'static str = "DOM.resolveNode";
    type Response = ResolveNodeReturns;
}

/// Sets attribute for an element with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAttributeValueParams<'a> {
    /// Id of the element to set attribute for.
    nodeId: NodeId,
    /// Attribute name.
    name: Cow<'a, str>,
    /// Attribute value.
    value: Cow<'a, str>,
}

impl<'a> SetAttributeValueParams<'a> {
    pub fn builder() -> SetAttributeValueParamsBuilder<'a> { SetAttributeValueParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct SetAttributeValueParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> SetAttributeValueParamsBuilder<'a> {
    /// Id of the element to set attribute for.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Attribute name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Attribute value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> SetAttributeValueParams<'a> {
        SetAttributeValueParams {
            nodeId: self.nodeId.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}

impl<'a> SetAttributeValueParams<'a> { pub const METHOD: &'static str = "DOM.setAttributeValue"; }

impl<'a> crate::CdpCommand<'a> for SetAttributeValueParams<'a> {
    const METHOD: &'static str = "DOM.setAttributeValue";
    type Response = crate::EmptyReturns;
}

/// Sets attributes on element with given id. This method is useful when user edits some existing
/// attribute value and types in several attribute name/value pairs.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAttributesAsTextParams<'a> {
    /// Id of the element to set attributes for.
    nodeId: NodeId,
    /// Text with a number of attributes. Will parse this text using HTML parser.
    text: Cow<'a, str>,
    /// Attribute name to replace with new attributes derived from text in case text parsed
    /// successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
}

impl<'a> SetAttributesAsTextParams<'a> {
    pub fn builder() -> SetAttributesAsTextParamsBuilder<'a> { SetAttributesAsTextParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
}

#[derive(Default)]
pub struct SetAttributesAsTextParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    text: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
}

impl<'a> SetAttributesAsTextParamsBuilder<'a> {
    /// Id of the element to set attributes for.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Text with a number of attributes. Will parse this text using HTML parser.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// Attribute name to replace with new attributes derived from text in case text parsed
    /// successfully.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> SetAttributesAsTextParams<'a> {
        SetAttributesAsTextParams {
            nodeId: self.nodeId.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
            name: self.name,
        }
    }
}

impl<'a> SetAttributesAsTextParams<'a> { pub const METHOD: &'static str = "DOM.setAttributesAsText"; }

impl<'a> crate::CdpCommand<'a> for SetAttributesAsTextParams<'a> {
    const METHOD: &'static str = "DOM.setAttributesAsText";
    type Response = crate::EmptyReturns;
}

/// Sets files for the given file input element.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFileInputFilesParams<'a> {
    /// Array of file paths to set.
    files: Vec<Cow<'a, str>>,
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> SetFileInputFilesParams<'a> {
    pub fn builder() -> SetFileInputFilesParamsBuilder<'a> { SetFileInputFilesParamsBuilder::default() }
    pub fn files(&self) -> &[Cow<'a, str>] { &self.files }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
}

#[derive(Default)]
pub struct SetFileInputFilesParamsBuilder<'a> {
    files: Option<Vec<Cow<'a, str>>>,
    nodeId: Option<NodeId>,
    backendNodeId: Option<BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> SetFileInputFilesParamsBuilder<'a> {
    /// Array of file paths to set.
    pub fn files(mut self, files: Vec<Cow<'a, str>>) -> Self { self.files = Some(files); self }
    /// Identifier of the node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    pub fn build(self) -> SetFileInputFilesParams<'a> {
        SetFileInputFilesParams {
            files: self.files.unwrap_or_default(),
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
        }
    }
}

impl<'a> SetFileInputFilesParams<'a> { pub const METHOD: &'static str = "DOM.setFileInputFiles"; }

impl<'a> crate::CdpCommand<'a> for SetFileInputFilesParams<'a> {
    const METHOD: &'static str = "DOM.setFileInputFiles";
    type Response = crate::EmptyReturns;
}

/// Sets if stack traces should be captured for Nodes. See 'Node.getNodeStackTraces'. Default is disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeStackTracesEnabledParams {
    /// Enable or disable.
    enable: bool,
}

impl SetNodeStackTracesEnabledParams {
    pub fn builder() -> SetNodeStackTracesEnabledParamsBuilder { SetNodeStackTracesEnabledParamsBuilder::default() }
    pub fn enable(&self) -> bool { self.enable }
}

#[derive(Default)]
pub struct SetNodeStackTracesEnabledParamsBuilder {
    enable: Option<bool>,
}

impl SetNodeStackTracesEnabledParamsBuilder {
    /// Enable or disable.
    pub fn enable(mut self, enable: bool) -> Self { self.enable = Some(enable); self }
    pub fn build(self) -> SetNodeStackTracesEnabledParams {
        SetNodeStackTracesEnabledParams {
            enable: self.enable.unwrap_or_default(),
        }
    }
}

impl SetNodeStackTracesEnabledParams { pub const METHOD: &'static str = "DOM.setNodeStackTracesEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetNodeStackTracesEnabledParams {
    const METHOD: &'static str = "DOM.setNodeStackTracesEnabled";
    type Response = crate::EmptyReturns;
}

/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeStackTracesParams {
    /// Id of the node to get stack traces for.
    nodeId: NodeId,
}

impl GetNodeStackTracesParams {
    pub fn builder() -> GetNodeStackTracesParamsBuilder { GetNodeStackTracesParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetNodeStackTracesParamsBuilder {
    nodeId: Option<NodeId>,
}

impl GetNodeStackTracesParamsBuilder {
    /// Id of the node to get stack traces for.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetNodeStackTracesParams {
        GetNodeStackTracesParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeStackTracesReturns {
    /// Creation stack trace, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    creation: Option<crate::runtime::StackTrace>,
}

impl GetNodeStackTracesReturns {
    pub fn builder() -> GetNodeStackTracesReturnsBuilder { GetNodeStackTracesReturnsBuilder::default() }
    pub fn creation(&self) -> Option<&crate::runtime::StackTrace> { self.creation.as_ref() }
}

#[derive(Default)]
pub struct GetNodeStackTracesReturnsBuilder {
    creation: Option<crate::runtime::StackTrace>,
}

impl GetNodeStackTracesReturnsBuilder {
    /// Creation stack trace, if available.
    pub fn creation(mut self, creation: crate::runtime::StackTrace) -> Self { self.creation = Some(creation); self }
    pub fn build(self) -> GetNodeStackTracesReturns {
        GetNodeStackTracesReturns {
            creation: self.creation,
        }
    }
}

impl GetNodeStackTracesParams { pub const METHOD: &'static str = "DOM.getNodeStackTraces"; }

impl<'a> crate::CdpCommand<'a> for GetNodeStackTracesParams {
    const METHOD: &'static str = "DOM.getNodeStackTraces";
    type Response = GetNodeStackTracesReturns;
}

/// Returns file information for the given
/// File wrapper.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFileInfoParams<'a> {
    /// JavaScript object id of the node wrapper.
    objectId: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> GetFileInfoParams<'a> {
    pub fn builder() -> GetFileInfoParamsBuilder<'a> { GetFileInfoParamsBuilder::default() }
    pub fn objectId(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.objectId }
}

#[derive(Default)]
pub struct GetFileInfoParamsBuilder<'a> {
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetFileInfoParamsBuilder<'a> {
    /// JavaScript object id of the node wrapper.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    pub fn build(self) -> GetFileInfoParams<'a> {
        GetFileInfoParams {
            objectId: self.objectId.unwrap_or_default(),
        }
    }
}

/// Returns file information for the given
/// File wrapper.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFileInfoReturns<'a> {
    path: Cow<'a, str>,
}

impl<'a> GetFileInfoReturns<'a> {
    pub fn builder() -> GetFileInfoReturnsBuilder<'a> { GetFileInfoReturnsBuilder::default() }
    pub fn path(&self) -> &str { self.path.as_ref() }
}

#[derive(Default)]
pub struct GetFileInfoReturnsBuilder<'a> {
    path: Option<Cow<'a, str>>,
}

impl<'a> GetFileInfoReturnsBuilder<'a> {
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    pub fn build(self) -> GetFileInfoReturns<'a> {
        GetFileInfoReturns {
            path: self.path.unwrap_or_default(),
        }
    }
}

impl<'a> GetFileInfoParams<'a> { pub const METHOD: &'static str = "DOM.getFileInfo"; }

impl<'a> crate::CdpCommand<'a> for GetFileInfoParams<'a> {
    const METHOD: &'static str = "DOM.getFileInfo";
    type Response = GetFileInfoReturns<'a>;
}

/// Returns list of detached nodes

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDetachedDomNodesReturns<'a> {
    /// The list of detached nodes
    detachedNodes: Vec<DetachedElementInfo<'a>>,
}

impl<'a> GetDetachedDomNodesReturns<'a> {
    pub fn builder() -> GetDetachedDomNodesReturnsBuilder<'a> { GetDetachedDomNodesReturnsBuilder::default() }
    pub fn detachedNodes(&self) -> &[DetachedElementInfo<'a>] { &self.detachedNodes }
}

#[derive(Default)]
pub struct GetDetachedDomNodesReturnsBuilder<'a> {
    detachedNodes: Option<Vec<DetachedElementInfo<'a>>>,
}

impl<'a> GetDetachedDomNodesReturnsBuilder<'a> {
    /// The list of detached nodes
    pub fn detachedNodes(mut self, detachedNodes: Vec<DetachedElementInfo<'a>>) -> Self { self.detachedNodes = Some(detachedNodes); self }
    pub fn build(self) -> GetDetachedDomNodesReturns<'a> {
        GetDetachedDomNodesReturns {
            detachedNodes: self.detachedNodes.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDetachedDomNodesParams {}

impl GetDetachedDomNodesParams {
    pub fn builder() -> GetDetachedDomNodesParamsBuilder {
        GetDetachedDomNodesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetDetachedDomNodesParamsBuilder {}

impl GetDetachedDomNodesParamsBuilder {
    pub fn build(self) -> GetDetachedDomNodesParams {
        GetDetachedDomNodesParams {}
    }
}

impl GetDetachedDomNodesParams { pub const METHOD: &'static str = "DOM.getDetachedDomNodes"; }

impl<'a> crate::CdpCommand<'a> for GetDetachedDomNodesParams {
    const METHOD: &'static str = "DOM.getDetachedDomNodes";
    type Response = GetDetachedDomNodesReturns<'a>;
}

/// Enables console to refer to the node with given id via $x (see Command Line API for more details
/// $x functions).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInspectedNodeParams {
    /// DOM node id to be accessible by means of $x command line API.
    nodeId: NodeId,
}

impl SetInspectedNodeParams {
    pub fn builder() -> SetInspectedNodeParamsBuilder { SetInspectedNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct SetInspectedNodeParamsBuilder {
    nodeId: Option<NodeId>,
}

impl SetInspectedNodeParamsBuilder {
    /// DOM node id to be accessible by means of $x command line API.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> SetInspectedNodeParams {
        SetInspectedNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl SetInspectedNodeParams { pub const METHOD: &'static str = "DOM.setInspectedNode"; }

impl<'a> crate::CdpCommand<'a> for SetInspectedNodeParams {
    const METHOD: &'static str = "DOM.setInspectedNode";
    type Response = crate::EmptyReturns;
}

/// Sets node name for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeNameParams<'a> {
    /// Id of the node to set name for.
    nodeId: NodeId,
    /// New node's name.
    name: Cow<'a, str>,
}

impl<'a> SetNodeNameParams<'a> {
    pub fn builder() -> SetNodeNameParamsBuilder<'a> { SetNodeNameParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn name(&self) -> &str { self.name.as_ref() }
}

#[derive(Default)]
pub struct SetNodeNameParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    name: Option<Cow<'a, str>>,
}

impl<'a> SetNodeNameParamsBuilder<'a> {
    /// Id of the node to set name for.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// New node's name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> SetNodeNameParams<'a> {
        SetNodeNameParams {
            nodeId: self.nodeId.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
        }
    }
}

/// Sets node name for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeNameReturns {
    /// New node's id.
    nodeId: NodeId,
}

impl SetNodeNameReturns {
    pub fn builder() -> SetNodeNameReturnsBuilder { SetNodeNameReturnsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct SetNodeNameReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl SetNodeNameReturnsBuilder {
    /// New node's id.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> SetNodeNameReturns {
        SetNodeNameReturns {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl<'a> SetNodeNameParams<'a> { pub const METHOD: &'static str = "DOM.setNodeName"; }

impl<'a> crate::CdpCommand<'a> for SetNodeNameParams<'a> {
    const METHOD: &'static str = "DOM.setNodeName";
    type Response = SetNodeNameReturns;
}

/// Sets node value for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeValueParams<'a> {
    /// Id of the node to set value for.
    nodeId: NodeId,
    /// New node's value.
    value: Cow<'a, str>,
}

impl<'a> SetNodeValueParams<'a> {
    pub fn builder() -> SetNodeValueParamsBuilder<'a> { SetNodeValueParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct SetNodeValueParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    value: Option<Cow<'a, str>>,
}

impl<'a> SetNodeValueParamsBuilder<'a> {
    /// Id of the node to set value for.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// New node's value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> SetNodeValueParams<'a> {
        SetNodeValueParams {
            nodeId: self.nodeId.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}

impl<'a> SetNodeValueParams<'a> { pub const METHOD: &'static str = "DOM.setNodeValue"; }

impl<'a> crate::CdpCommand<'a> for SetNodeValueParams<'a> {
    const METHOD: &'static str = "DOM.setNodeValue";
    type Response = crate::EmptyReturns;
}

/// Sets node HTML markup, returns new node id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetOuterHTMLParams<'a> {
    /// Id of the node to set markup for.
    nodeId: NodeId,
    /// Outer HTML markup to set.
    outerHTML: Cow<'a, str>,
}

impl<'a> SetOuterHTMLParams<'a> {
    pub fn builder() -> SetOuterHTMLParamsBuilder<'a> { SetOuterHTMLParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn outerHTML(&self) -> &str { self.outerHTML.as_ref() }
}

#[derive(Default)]
pub struct SetOuterHTMLParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    outerHTML: Option<Cow<'a, str>>,
}

impl<'a> SetOuterHTMLParamsBuilder<'a> {
    /// Id of the node to set markup for.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Outer HTML markup to set.
    pub fn outerHTML(mut self, outerHTML: impl Into<Cow<'a, str>>) -> Self { self.outerHTML = Some(outerHTML.into()); self }
    pub fn build(self) -> SetOuterHTMLParams<'a> {
        SetOuterHTMLParams {
            nodeId: self.nodeId.unwrap_or_default(),
            outerHTML: self.outerHTML.unwrap_or_default(),
        }
    }
}

impl<'a> SetOuterHTMLParams<'a> { pub const METHOD: &'static str = "DOM.setOuterHTML"; }

impl<'a> crate::CdpCommand<'a> for SetOuterHTMLParams<'a> {
    const METHOD: &'static str = "DOM.setOuterHTML";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UndoParams {}

impl UndoParams {
    pub fn builder() -> UndoParamsBuilder {
        UndoParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct UndoParamsBuilder {}

impl UndoParamsBuilder {
    pub fn build(self) -> UndoParams {
        UndoParams {}
    }
}

impl UndoParams { pub const METHOD: &'static str = "DOM.undo"; }

impl<'a> crate::CdpCommand<'a> for UndoParams {
    const METHOD: &'static str = "DOM.undo";
    type Response = crate::EmptyReturns;
}

/// Returns iframe node that owns iframe with the given domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameOwnerParams<'a> {
    frameId: crate::page::FrameId<'a>,
}

impl<'a> GetFrameOwnerParams<'a> {
    pub fn builder() -> GetFrameOwnerParamsBuilder<'a> { GetFrameOwnerParamsBuilder::default() }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
}

#[derive(Default)]
pub struct GetFrameOwnerParamsBuilder<'a> {
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetFrameOwnerParamsBuilder<'a> {
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetFrameOwnerParams<'a> {
        GetFrameOwnerParams {
            frameId: self.frameId.unwrap_or_default(),
        }
    }
}

/// Returns iframe node that owns iframe with the given domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameOwnerReturns {
    /// Resulting node.
    backendNodeId: BackendNodeId,
    /// Id of the node at given coordinates, only when enabled and requested document.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<NodeId>,
}

impl GetFrameOwnerReturns {
    pub fn builder() -> GetFrameOwnerReturnsBuilder { GetFrameOwnerReturnsBuilder::default() }
    pub fn backendNodeId(&self) -> &BackendNodeId { &self.backendNodeId }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
}

#[derive(Default)]
pub struct GetFrameOwnerReturnsBuilder {
    backendNodeId: Option<BackendNodeId>,
    nodeId: Option<NodeId>,
}

impl GetFrameOwnerReturnsBuilder {
    /// Resulting node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// Id of the node at given coordinates, only when enabled and requested document.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetFrameOwnerReturns {
        GetFrameOwnerReturns {
            backendNodeId: self.backendNodeId.unwrap_or_default(),
            nodeId: self.nodeId,
        }
    }
}

impl<'a> GetFrameOwnerParams<'a> { pub const METHOD: &'static str = "DOM.getFrameOwner"; }

impl<'a> crate::CdpCommand<'a> for GetFrameOwnerParams<'a> {
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
pub struct GetContainerForNodeParams<'a> {
    nodeId: NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    containerName: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physicalAxes: Option<PhysicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logicalAxes: Option<LogicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queriesScrollState: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queriesAnchored: Option<bool>,
}

impl<'a> GetContainerForNodeParams<'a> {
    pub fn builder() -> GetContainerForNodeParamsBuilder<'a> { GetContainerForNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn containerName(&self) -> Option<&str> { self.containerName.as_deref() }
    pub fn physicalAxes(&self) -> Option<&PhysicalAxes> { self.physicalAxes.as_ref() }
    pub fn logicalAxes(&self) -> Option<&LogicalAxes> { self.logicalAxes.as_ref() }
    pub fn queriesScrollState(&self) -> Option<bool> { self.queriesScrollState }
    pub fn queriesAnchored(&self) -> Option<bool> { self.queriesAnchored }
}

#[derive(Default)]
pub struct GetContainerForNodeParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    containerName: Option<Cow<'a, str>>,
    physicalAxes: Option<PhysicalAxes>,
    logicalAxes: Option<LogicalAxes>,
    queriesScrollState: Option<bool>,
    queriesAnchored: Option<bool>,
}

impl<'a> GetContainerForNodeParamsBuilder<'a> {
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn containerName(mut self, containerName: impl Into<Cow<'a, str>>) -> Self { self.containerName = Some(containerName.into()); self }
    pub fn physicalAxes(mut self, physicalAxes: PhysicalAxes) -> Self { self.physicalAxes = Some(physicalAxes); self }
    pub fn logicalAxes(mut self, logicalAxes: LogicalAxes) -> Self { self.logicalAxes = Some(logicalAxes); self }
    pub fn queriesScrollState(mut self, queriesScrollState: bool) -> Self { self.queriesScrollState = Some(queriesScrollState); self }
    pub fn queriesAnchored(mut self, queriesAnchored: bool) -> Self { self.queriesAnchored = Some(queriesAnchored); self }
    pub fn build(self) -> GetContainerForNodeParams<'a> {
        GetContainerForNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
            containerName: self.containerName,
            physicalAxes: self.physicalAxes,
            logicalAxes: self.logicalAxes,
            queriesScrollState: self.queriesScrollState,
            queriesAnchored: self.queriesAnchored,
        }
    }
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
    nodeId: Option<NodeId>,
}

impl GetContainerForNodeReturns {
    pub fn builder() -> GetContainerForNodeReturnsBuilder { GetContainerForNodeReturnsBuilder::default() }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
}

#[derive(Default)]
pub struct GetContainerForNodeReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl GetContainerForNodeReturnsBuilder {
    /// The container node for the given node, or null if not found.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetContainerForNodeReturns {
        GetContainerForNodeReturns {
            nodeId: self.nodeId,
        }
    }
}

impl<'a> GetContainerForNodeParams<'a> { pub const METHOD: &'static str = "DOM.getContainerForNode"; }

impl<'a> crate::CdpCommand<'a> for GetContainerForNodeParams<'a> {
    const METHOD: &'static str = "DOM.getContainerForNode";
    type Response = GetContainerForNodeReturns;
}

/// Returns the descendants of a container query container that have
/// container queries against this container.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryingDescendantsForContainerParams {
    /// Id of the container node to find querying descendants from.
    nodeId: NodeId,
}

impl GetQueryingDescendantsForContainerParams {
    pub fn builder() -> GetQueryingDescendantsForContainerParamsBuilder { GetQueryingDescendantsForContainerParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetQueryingDescendantsForContainerParamsBuilder {
    nodeId: Option<NodeId>,
}

impl GetQueryingDescendantsForContainerParamsBuilder {
    /// Id of the container node to find querying descendants from.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetQueryingDescendantsForContainerParams {
        GetQueryingDescendantsForContainerParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Returns the descendants of a container query container that have
/// container queries against this container.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryingDescendantsForContainerReturns {
    /// Descendant nodes with container queries against the given container.
    nodeIds: Vec<NodeId>,
}

impl GetQueryingDescendantsForContainerReturns {
    pub fn builder() -> GetQueryingDescendantsForContainerReturnsBuilder { GetQueryingDescendantsForContainerReturnsBuilder::default() }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}

#[derive(Default)]
pub struct GetQueryingDescendantsForContainerReturnsBuilder {
    nodeIds: Option<Vec<NodeId>>,
}

impl GetQueryingDescendantsForContainerReturnsBuilder {
    /// Descendant nodes with container queries against the given container.
    pub fn nodeIds(mut self, nodeIds: Vec<NodeId>) -> Self { self.nodeIds = Some(nodeIds); self }
    pub fn build(self) -> GetQueryingDescendantsForContainerReturns {
        GetQueryingDescendantsForContainerReturns {
            nodeIds: self.nodeIds.unwrap_or_default(),
        }
    }
}

impl GetQueryingDescendantsForContainerParams { pub const METHOD: &'static str = "DOM.getQueryingDescendantsForContainer"; }

impl<'a> crate::CdpCommand<'a> for GetQueryingDescendantsForContainerParams {
    const METHOD: &'static str = "DOM.getQueryingDescendantsForContainer";
    type Response = GetQueryingDescendantsForContainerReturns;
}

/// Returns the target anchor element of the given anchor query according to
/// https://www.w3.org/TR/css-anchor-position-1/#target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnchorElementParams<'a> {
    /// Id of the positioned element from which to find the anchor.
    nodeId: NodeId,
    /// An optional anchor specifier, as defined in
    /// https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier.
    /// If not provided, it will return the implicit anchor element for
    /// the given positioned element.
    #[serde(skip_serializing_if = "Option::is_none")]
    anchorSpecifier: Option<Cow<'a, str>>,
}

impl<'a> GetAnchorElementParams<'a> {
    pub fn builder() -> GetAnchorElementParamsBuilder<'a> { GetAnchorElementParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn anchorSpecifier(&self) -> Option<&str> { self.anchorSpecifier.as_deref() }
}

#[derive(Default)]
pub struct GetAnchorElementParamsBuilder<'a> {
    nodeId: Option<NodeId>,
    anchorSpecifier: Option<Cow<'a, str>>,
}

impl<'a> GetAnchorElementParamsBuilder<'a> {
    /// Id of the positioned element from which to find the anchor.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// An optional anchor specifier, as defined in
    /// https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier.
    /// If not provided, it will return the implicit anchor element for
    /// the given positioned element.
    pub fn anchorSpecifier(mut self, anchorSpecifier: impl Into<Cow<'a, str>>) -> Self { self.anchorSpecifier = Some(anchorSpecifier.into()); self }
    pub fn build(self) -> GetAnchorElementParams<'a> {
        GetAnchorElementParams {
            nodeId: self.nodeId.unwrap_or_default(),
            anchorSpecifier: self.anchorSpecifier,
        }
    }
}

/// Returns the target anchor element of the given anchor query according to
/// https://www.w3.org/TR/css-anchor-position-1/#target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnchorElementReturns {
    /// The anchor element of the given anchor query.
    nodeId: NodeId,
}

impl GetAnchorElementReturns {
    pub fn builder() -> GetAnchorElementReturnsBuilder { GetAnchorElementReturnsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetAnchorElementReturnsBuilder {
    nodeId: Option<NodeId>,
}

impl GetAnchorElementReturnsBuilder {
    /// The anchor element of the given anchor query.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetAnchorElementReturns {
        GetAnchorElementReturns {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

impl<'a> GetAnchorElementParams<'a> { pub const METHOD: &'static str = "DOM.getAnchorElement"; }

impl<'a> crate::CdpCommand<'a> for GetAnchorElementParams<'a> {
    const METHOD: &'static str = "DOM.getAnchorElement";
    type Response = GetAnchorElementReturns;
}

/// When enabling, this API force-opens the popover identified by nodeId
/// and keeps it open until disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForceShowPopoverParams {
    /// Id of the popover HTMLElement
    nodeId: NodeId,
    /// If true, opens the popover and keeps it open. If false, closes the
    /// popover if it was previously force-opened.
    enable: bool,
}

impl ForceShowPopoverParams {
    pub fn builder() -> ForceShowPopoverParamsBuilder { ForceShowPopoverParamsBuilder::default() }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn enable(&self) -> bool { self.enable }
}

#[derive(Default)]
pub struct ForceShowPopoverParamsBuilder {
    nodeId: Option<NodeId>,
    enable: Option<bool>,
}

impl ForceShowPopoverParamsBuilder {
    /// Id of the popover HTMLElement
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// If true, opens the popover and keeps it open. If false, closes the
    /// popover if it was previously force-opened.
    pub fn enable(mut self, enable: bool) -> Self { self.enable = Some(enable); self }
    pub fn build(self) -> ForceShowPopoverParams {
        ForceShowPopoverParams {
            nodeId: self.nodeId.unwrap_or_default(),
            enable: self.enable.unwrap_or_default(),
        }
    }
}

/// When enabling, this API force-opens the popover identified by nodeId
/// and keeps it open until disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForceShowPopoverReturns {
    /// List of popovers that were closed in order to respect popover stacking order.
    nodeIds: Vec<NodeId>,
}

impl ForceShowPopoverReturns {
    pub fn builder() -> ForceShowPopoverReturnsBuilder { ForceShowPopoverReturnsBuilder::default() }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}

#[derive(Default)]
pub struct ForceShowPopoverReturnsBuilder {
    nodeIds: Option<Vec<NodeId>>,
}

impl ForceShowPopoverReturnsBuilder {
    /// List of popovers that were closed in order to respect popover stacking order.
    pub fn nodeIds(mut self, nodeIds: Vec<NodeId>) -> Self { self.nodeIds = Some(nodeIds); self }
    pub fn build(self) -> ForceShowPopoverReturns {
        ForceShowPopoverReturns {
            nodeIds: self.nodeIds.unwrap_or_default(),
        }
    }
}

impl ForceShowPopoverParams { pub const METHOD: &'static str = "DOM.forceShowPopover"; }

impl<'a> crate::CdpCommand<'a> for ForceShowPopoverParams {
    const METHOD: &'static str = "DOM.forceShowPopover";
    type Response = ForceShowPopoverReturns;
}
