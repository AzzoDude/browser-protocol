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
    pub fn builder(nodeType: i64, nodeName: impl Into<Cow<'a, str>>, backendNodeId: BackendNodeId) -> BackendNodeBuilder<'a> {
        BackendNodeBuilder {
            nodeType: nodeType,
            nodeName: nodeName.into(),
            backendNodeId: backendNodeId,
        }
    }
    pub fn nodeType(&self) -> i64 { self.nodeType }
    pub fn nodeName(&self) -> &str { self.nodeName.as_ref() }
    pub fn backendNodeId(&self) -> &BackendNodeId { &self.backendNodeId }
}


pub struct BackendNodeBuilder<'a> {
    nodeType: i64,
    nodeName: Cow<'a, str>,
    backendNodeId: BackendNodeId,
}

impl<'a> BackendNodeBuilder<'a> {
    pub fn build(self) -> BackendNode<'a> {
        BackendNode {
            nodeType: self.nodeType,
            nodeName: self.nodeName,
            backendNodeId: self.backendNodeId,
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
    pub fn builder(nodeId: NodeId, backendNodeId: BackendNodeId, nodeType: i64, nodeName: impl Into<Cow<'a, str>>, localName: impl Into<Cow<'a, str>>, nodeValue: impl Into<Cow<'a, str>>) -> NodeBuilder<'a> {
        NodeBuilder {
            nodeId: nodeId,
            parentId: None,
            backendNodeId: backendNodeId,
            nodeType: nodeType,
            nodeName: nodeName.into(),
            localName: localName.into(),
            nodeValue: nodeValue.into(),
            childNodeCount: None,
            children: None,
            attributes: None,
            documentURL: None,
            baseURL: None,
            publicId: None,
            systemId: None,
            internalSubset: None,
            xmlVersion: None,
            name: None,
            value: None,
            pseudoType: None,
            pseudoIdentifier: None,
            shadowRootType: None,
            frameId: None,
            contentDocument: None,
            shadowRoots: None,
            templateContent: None,
            pseudoElements: None,
            importedDocument: None,
            distributedNodes: None,
            isSVG: None,
            compatibilityMode: None,
            assignedSlot: None,
            isScrollable: None,
            affectedByStartingStyles: None,
            adoptedStyleSheets: None,
            adProvenance: None,
        }
    }
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


pub struct NodeBuilder<'a> {
    nodeId: NodeId,
    parentId: Option<NodeId>,
    backendNodeId: BackendNodeId,
    nodeType: i64,
    nodeName: Cow<'a, str>,
    localName: Cow<'a, str>,
    nodeValue: Cow<'a, str>,
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
    /// The id of the parent node if any.
    pub fn parentId(mut self, parentId: NodeId) -> Self { self.parentId = Some(parentId); self }
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
            nodeId: self.nodeId,
            parentId: self.parentId,
            backendNodeId: self.backendNodeId,
            nodeType: self.nodeType,
            nodeName: self.nodeName,
            localName: self.localName,
            nodeValue: self.nodeValue,
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
    pub fn builder(treeNode: Node<'a>, retainedNodeIds: Vec<NodeId>) -> DetachedElementInfoBuilder<'a> {
        DetachedElementInfoBuilder {
            treeNode: treeNode,
            retainedNodeIds: retainedNodeIds,
        }
    }
    pub fn treeNode(&self) -> &Node<'a> { &self.treeNode }
    pub fn retainedNodeIds(&self) -> &[NodeId] { &self.retainedNodeIds }
}


pub struct DetachedElementInfoBuilder<'a> {
    treeNode: Node<'a>,
    retainedNodeIds: Vec<NodeId>,
}

impl<'a> DetachedElementInfoBuilder<'a> {
    pub fn build(self) -> DetachedElementInfo<'a> {
        DetachedElementInfo {
            treeNode: self.treeNode,
            retainedNodeIds: self.retainedNodeIds,
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
    pub fn builder(r: i64, g: i64, b: i64) -> RGBABuilder {
        RGBABuilder {
            r: r,
            g: g,
            b: b,
            a: None,
        }
    }
    pub fn r(&self) -> i64 { self.r }
    pub fn g(&self) -> i64 { self.g }
    pub fn b(&self) -> i64 { self.b }
    pub fn a(&self) -> Option<f64> { self.a }
}


pub struct RGBABuilder {
    r: i64,
    g: i64,
    b: i64,
    a: Option<f64>,
}

impl RGBABuilder {
    /// The alpha component, in the [0-1] range (default: 1).
    pub fn a(mut self, a: f64) -> Self { self.a = Some(a); self }
    pub fn build(self) -> RGBA {
        RGBA {
            r: self.r,
            g: self.g,
            b: self.b,
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
    pub fn builder(content: Quad, padding: Quad, border: Quad, margin: Quad, width: u64, height: i64) -> BoxModelBuilder {
        BoxModelBuilder {
            content: content,
            padding: padding,
            border: border,
            margin: margin,
            width: width,
            height: height,
            shapeOutside: None,
        }
    }
    pub fn content(&self) -> &Quad { &self.content }
    pub fn padding(&self) -> &Quad { &self.padding }
    pub fn border(&self) -> &Quad { &self.border }
    pub fn margin(&self) -> &Quad { &self.margin }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> i64 { self.height }
    pub fn shapeOutside(&self) -> Option<&ShapeOutsideInfo> { self.shapeOutside.as_ref() }
}


pub struct BoxModelBuilder {
    content: Quad,
    padding: Quad,
    border: Quad,
    margin: Quad,
    width: u64,
    height: i64,
    shapeOutside: Option<ShapeOutsideInfo>,
}

impl BoxModelBuilder {
    /// Shape outside coordinates
    pub fn shapeOutside(mut self, shapeOutside: ShapeOutsideInfo) -> Self { self.shapeOutside = Some(shapeOutside); self }
    pub fn build(self) -> BoxModel {
        BoxModel {
            content: self.content,
            padding: self.padding,
            border: self.border,
            margin: self.margin,
            width: self.width,
            height: self.height,
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
    pub fn builder(bounds: Quad, shape: Vec<JsonValue>, marginShape: Vec<JsonValue>) -> ShapeOutsideInfoBuilder {
        ShapeOutsideInfoBuilder {
            bounds: bounds,
            shape: shape,
            marginShape: marginShape,
        }
    }
    pub fn bounds(&self) -> &Quad { &self.bounds }
    pub fn shape(&self) -> &[JsonValue] { &self.shape }
    pub fn marginShape(&self) -> &[JsonValue] { &self.marginShape }
}


pub struct ShapeOutsideInfoBuilder {
    bounds: Quad,
    shape: Vec<JsonValue>,
    marginShape: Vec<JsonValue>,
}

impl ShapeOutsideInfoBuilder {
    pub fn build(self) -> ShapeOutsideInfo {
        ShapeOutsideInfo {
            bounds: self.bounds,
            shape: self.shape,
            marginShape: self.marginShape,
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
    pub fn builder(x: f64, y: f64, width: f64, height: f64) -> RectBuilder {
        RectBuilder {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn width(&self) -> f64 { self.width }
    pub fn height(&self) -> f64 { self.height }
}


pub struct RectBuilder {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl RectBuilder {
    pub fn build(self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
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
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> CSSComputedStylePropertyBuilder<'a> {
        CSSComputedStylePropertyBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct CSSComputedStylePropertyBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> CSSComputedStylePropertyBuilder<'a> {
    pub fn build(self) -> CSSComputedStyleProperty<'a> {
        CSSComputedStyleProperty {
            name: self.name,
            value: self.value,
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
    pub fn builder(nodeId: NodeId) -> CollectClassNamesFromSubtreeParamsBuilder {
        CollectClassNamesFromSubtreeParamsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct CollectClassNamesFromSubtreeParamsBuilder {
    nodeId: NodeId,
}

impl CollectClassNamesFromSubtreeParamsBuilder {
    pub fn build(self) -> CollectClassNamesFromSubtreeParams {
        CollectClassNamesFromSubtreeParams {
            nodeId: self.nodeId,
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
    pub fn builder(classNames: Vec<Cow<'a, str>>) -> CollectClassNamesFromSubtreeReturnsBuilder<'a> {
        CollectClassNamesFromSubtreeReturnsBuilder {
            classNames: classNames,
        }
    }
    pub fn classNames(&self) -> &[Cow<'a, str>] { &self.classNames }
}


pub struct CollectClassNamesFromSubtreeReturnsBuilder<'a> {
    classNames: Vec<Cow<'a, str>>,
}

impl<'a> CollectClassNamesFromSubtreeReturnsBuilder<'a> {
    pub fn build(self) -> CollectClassNamesFromSubtreeReturns<'a> {
        CollectClassNamesFromSubtreeReturns {
            classNames: self.classNames,
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
    pub fn builder(nodeId: NodeId, targetNodeId: NodeId) -> CopyToParamsBuilder {
        CopyToParamsBuilder {
            nodeId: nodeId,
            targetNodeId: targetNodeId,
            insertBeforeNodeId: None,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn targetNodeId(&self) -> &NodeId { &self.targetNodeId }
    pub fn insertBeforeNodeId(&self) -> Option<&NodeId> { self.insertBeforeNodeId.as_ref() }
}


pub struct CopyToParamsBuilder {
    nodeId: NodeId,
    targetNodeId: NodeId,
    insertBeforeNodeId: Option<NodeId>,
}

impl CopyToParamsBuilder {
    /// Drop the copy before this node (if absent, the copy becomes the last child of
    /// 'targetNodeId').
    pub fn insertBeforeNodeId(mut self, insertBeforeNodeId: NodeId) -> Self { self.insertBeforeNodeId = Some(insertBeforeNodeId); self }
    pub fn build(self) -> CopyToParams {
        CopyToParams {
            nodeId: self.nodeId,
            targetNodeId: self.targetNodeId,
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
    pub fn builder(nodeId: NodeId) -> CopyToReturnsBuilder {
        CopyToReturnsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct CopyToReturnsBuilder {
    nodeId: NodeId,
}

impl CopyToReturnsBuilder {
    pub fn build(self) -> CopyToReturns {
        CopyToReturns {
            nodeId: self.nodeId,
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
    pub fn builder() -> DescribeNodeParamsBuilder<'a> {
        DescribeNodeParamsBuilder {
            nodeId: None,
            backendNodeId: None,
            objectId: None,
            depth: None,
            pierce: None,
        }
    }
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
    pub fn builder(node: Node<'a>) -> DescribeNodeReturnsBuilder<'a> {
        DescribeNodeReturnsBuilder {
            node: node,
        }
    }
    pub fn node(&self) -> &Node<'a> { &self.node }
}


pub struct DescribeNodeReturnsBuilder<'a> {
    node: Node<'a>,
}

impl<'a> DescribeNodeReturnsBuilder<'a> {
    pub fn build(self) -> DescribeNodeReturns<'a> {
        DescribeNodeReturns {
            node: self.node,
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
    pub fn builder() -> ScrollIntoViewIfNeededParamsBuilder<'a> {
        ScrollIntoViewIfNeededParamsBuilder {
            nodeId: None,
            backendNodeId: None,
            objectId: None,
            rect: None,
        }
    }
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
    pub fn builder(searchId: impl Into<Cow<'a, str>>) -> DiscardSearchResultsParamsBuilder<'a> {
        DiscardSearchResultsParamsBuilder {
            searchId: searchId.into(),
        }
    }
    pub fn searchId(&self) -> &str { self.searchId.as_ref() }
}


pub struct DiscardSearchResultsParamsBuilder<'a> {
    searchId: Cow<'a, str>,
}

impl<'a> DiscardSearchResultsParamsBuilder<'a> {
    pub fn build(self) -> DiscardSearchResultsParams<'a> {
        DiscardSearchResultsParams {
            searchId: self.searchId,
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
    pub fn builder() -> EnableParamsBuilder<'a> {
        EnableParamsBuilder {
            includeWhitespace: None,
        }
    }
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
    pub fn builder() -> FocusParamsBuilder<'a> {
        FocusParamsBuilder {
            nodeId: None,
            backendNodeId: None,
            objectId: None,
        }
    }
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
    pub fn builder(nodeId: NodeId) -> GetAttributesParamsBuilder {
        GetAttributesParamsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct GetAttributesParamsBuilder {
    nodeId: NodeId,
}

impl GetAttributesParamsBuilder {
    pub fn build(self) -> GetAttributesParams {
        GetAttributesParams {
            nodeId: self.nodeId,
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
    pub fn builder(attributes: Vec<Cow<'a, str>>) -> GetAttributesReturnsBuilder<'a> {
        GetAttributesReturnsBuilder {
            attributes: attributes,
        }
    }
    pub fn attributes(&self) -> &[Cow<'a, str>] { &self.attributes }
}


pub struct GetAttributesReturnsBuilder<'a> {
    attributes: Vec<Cow<'a, str>>,
}

impl<'a> GetAttributesReturnsBuilder<'a> {
    pub fn build(self) -> GetAttributesReturns<'a> {
        GetAttributesReturns {
            attributes: self.attributes,
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
    pub fn builder() -> GetBoxModelParamsBuilder<'a> {
        GetBoxModelParamsBuilder {
            nodeId: None,
            backendNodeId: None,
            objectId: None,
        }
    }
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
    pub fn builder(model: BoxModel) -> GetBoxModelReturnsBuilder {
        GetBoxModelReturnsBuilder {
            model: model,
        }
    }
    pub fn model(&self) -> &BoxModel { &self.model }
}


pub struct GetBoxModelReturnsBuilder {
    model: BoxModel,
}

impl GetBoxModelReturnsBuilder {
    pub fn build(self) -> GetBoxModelReturns {
        GetBoxModelReturns {
            model: self.model,
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
    pub fn builder() -> GetContentQuadsParamsBuilder<'a> {
        GetContentQuadsParamsBuilder {
            nodeId: None,
            backendNodeId: None,
            objectId: None,
        }
    }
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
    pub fn builder(quads: Vec<Quad>) -> GetContentQuadsReturnsBuilder {
        GetContentQuadsReturnsBuilder {
            quads: quads,
        }
    }
    pub fn quads(&self) -> &[Quad] { &self.quads }
}


pub struct GetContentQuadsReturnsBuilder {
    quads: Vec<Quad>,
}

impl GetContentQuadsReturnsBuilder {
    pub fn build(self) -> GetContentQuadsReturns {
        GetContentQuadsReturns {
            quads: self.quads,
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
    pub fn builder() -> GetDocumentParamsBuilder {
        GetDocumentParamsBuilder {
            depth: None,
            pierce: None,
        }
    }
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
    pub fn builder(root: Node<'a>) -> GetDocumentReturnsBuilder<'a> {
        GetDocumentReturnsBuilder {
            root: root,
        }
    }
    pub fn root(&self) -> &Node<'a> { &self.root }
}


pub struct GetDocumentReturnsBuilder<'a> {
    root: Node<'a>,
}

impl<'a> GetDocumentReturnsBuilder<'a> {
    pub fn build(self) -> GetDocumentReturns<'a> {
        GetDocumentReturns {
            root: self.root,
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
    pub fn builder() -> GetFlattenedDocumentParamsBuilder {
        GetFlattenedDocumentParamsBuilder {
            depth: None,
            pierce: None,
        }
    }
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
    pub fn builder(nodes: Vec<Node<'a>>) -> GetFlattenedDocumentReturnsBuilder<'a> {
        GetFlattenedDocumentReturnsBuilder {
            nodes: nodes,
        }
    }
    pub fn nodes(&self) -> &[Node<'a>] { &self.nodes }
}


pub struct GetFlattenedDocumentReturnsBuilder<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> GetFlattenedDocumentReturnsBuilder<'a> {
    pub fn build(self) -> GetFlattenedDocumentReturns<'a> {
        GetFlattenedDocumentReturns {
            nodes: self.nodes,
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
    pub fn builder(nodeId: NodeId, computedStyles: Vec<CSSComputedStyleProperty<'a>>) -> GetNodesForSubtreeByStyleParamsBuilder<'a> {
        GetNodesForSubtreeByStyleParamsBuilder {
            nodeId: nodeId,
            computedStyles: computedStyles,
            pierce: None,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn computedStyles(&self) -> &[CSSComputedStyleProperty<'a>] { &self.computedStyles }
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}


pub struct GetNodesForSubtreeByStyleParamsBuilder<'a> {
    nodeId: NodeId,
    computedStyles: Vec<CSSComputedStyleProperty<'a>>,
    pierce: Option<bool>,
}

impl<'a> GetNodesForSubtreeByStyleParamsBuilder<'a> {
    /// Whether or not iframes and shadow roots in the same target should be traversed when returning the
    /// results (default is false).
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> GetNodesForSubtreeByStyleParams<'a> {
        GetNodesForSubtreeByStyleParams {
            nodeId: self.nodeId,
            computedStyles: self.computedStyles,
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
    pub fn builder(nodeIds: Vec<NodeId>) -> GetNodesForSubtreeByStyleReturnsBuilder {
        GetNodesForSubtreeByStyleReturnsBuilder {
            nodeIds: nodeIds,
        }
    }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}


pub struct GetNodesForSubtreeByStyleReturnsBuilder {
    nodeIds: Vec<NodeId>,
}

impl GetNodesForSubtreeByStyleReturnsBuilder {
    pub fn build(self) -> GetNodesForSubtreeByStyleReturns {
        GetNodesForSubtreeByStyleReturns {
            nodeIds: self.nodeIds,
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
    pub fn builder(x: i32, y: i32) -> GetNodeForLocationParamsBuilder {
        GetNodeForLocationParamsBuilder {
            x: x,
            y: y,
            includeUserAgentShadowDOM: None,
            ignorePointerEventsNone: None,
        }
    }
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn includeUserAgentShadowDOM(&self) -> Option<bool> { self.includeUserAgentShadowDOM }
    pub fn ignorePointerEventsNone(&self) -> Option<bool> { self.ignorePointerEventsNone }
}


pub struct GetNodeForLocationParamsBuilder {
    x: i32,
    y: i32,
    includeUserAgentShadowDOM: Option<bool>,
    ignorePointerEventsNone: Option<bool>,
}

impl GetNodeForLocationParamsBuilder {
    /// False to skip to the nearest non-UA shadow root ancestor (default: false).
    pub fn includeUserAgentShadowDOM(mut self, includeUserAgentShadowDOM: bool) -> Self { self.includeUserAgentShadowDOM = Some(includeUserAgentShadowDOM); self }
    /// Whether to ignore pointer-events: none on elements and hit test them.
    pub fn ignorePointerEventsNone(mut self, ignorePointerEventsNone: bool) -> Self { self.ignorePointerEventsNone = Some(ignorePointerEventsNone); self }
    pub fn build(self) -> GetNodeForLocationParams {
        GetNodeForLocationParams {
            x: self.x,
            y: self.y,
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
    pub fn builder(backendNodeId: BackendNodeId, frameId: crate::page::FrameId<'a>) -> GetNodeForLocationReturnsBuilder<'a> {
        GetNodeForLocationReturnsBuilder {
            backendNodeId: backendNodeId,
            frameId: frameId,
            nodeId: None,
        }
    }
    pub fn backendNodeId(&self) -> &BackendNodeId { &self.backendNodeId }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
}


pub struct GetNodeForLocationReturnsBuilder<'a> {
    backendNodeId: BackendNodeId,
    frameId: crate::page::FrameId<'a>,
    nodeId: Option<NodeId>,
}

impl<'a> GetNodeForLocationReturnsBuilder<'a> {
    /// Id of the node at given coordinates, only when enabled and requested document.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetNodeForLocationReturns<'a> {
        GetNodeForLocationReturns {
            backendNodeId: self.backendNodeId,
            frameId: self.frameId,
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
    pub fn builder() -> GetOuterHTMLParamsBuilder<'a> {
        GetOuterHTMLParamsBuilder {
            nodeId: None,
            backendNodeId: None,
            objectId: None,
            includeShadowDOM: None,
        }
    }
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
    pub fn builder(outerHTML: impl Into<Cow<'a, str>>) -> GetOuterHTMLReturnsBuilder<'a> {
        GetOuterHTMLReturnsBuilder {
            outerHTML: outerHTML.into(),
        }
    }
    pub fn outerHTML(&self) -> &str { self.outerHTML.as_ref() }
}


pub struct GetOuterHTMLReturnsBuilder<'a> {
    outerHTML: Cow<'a, str>,
}

impl<'a> GetOuterHTMLReturnsBuilder<'a> {
    pub fn build(self) -> GetOuterHTMLReturns<'a> {
        GetOuterHTMLReturns {
            outerHTML: self.outerHTML,
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
    pub fn builder(nodeId: NodeId) -> GetRelayoutBoundaryParamsBuilder {
        GetRelayoutBoundaryParamsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct GetRelayoutBoundaryParamsBuilder {
    nodeId: NodeId,
}

impl GetRelayoutBoundaryParamsBuilder {
    pub fn build(self) -> GetRelayoutBoundaryParams {
        GetRelayoutBoundaryParams {
            nodeId: self.nodeId,
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
    pub fn builder(nodeId: NodeId) -> GetRelayoutBoundaryReturnsBuilder {
        GetRelayoutBoundaryReturnsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct GetRelayoutBoundaryReturnsBuilder {
    nodeId: NodeId,
}

impl GetRelayoutBoundaryReturnsBuilder {
    pub fn build(self) -> GetRelayoutBoundaryReturns {
        GetRelayoutBoundaryReturns {
            nodeId: self.nodeId,
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
    pub fn builder(searchId: impl Into<Cow<'a, str>>, fromIndex: u64, toIndex: u64) -> GetSearchResultsParamsBuilder<'a> {
        GetSearchResultsParamsBuilder {
            searchId: searchId.into(),
            fromIndex: fromIndex,
            toIndex: toIndex,
        }
    }
    pub fn searchId(&self) -> &str { self.searchId.as_ref() }
    pub fn fromIndex(&self) -> u64 { self.fromIndex }
    pub fn toIndex(&self) -> u64 { self.toIndex }
}


pub struct GetSearchResultsParamsBuilder<'a> {
    searchId: Cow<'a, str>,
    fromIndex: u64,
    toIndex: u64,
}

impl<'a> GetSearchResultsParamsBuilder<'a> {
    pub fn build(self) -> GetSearchResultsParams<'a> {
        GetSearchResultsParams {
            searchId: self.searchId,
            fromIndex: self.fromIndex,
            toIndex: self.toIndex,
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
    pub fn builder(nodeIds: Vec<NodeId>) -> GetSearchResultsReturnsBuilder {
        GetSearchResultsReturnsBuilder {
            nodeIds: nodeIds,
        }
    }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}


pub struct GetSearchResultsReturnsBuilder {
    nodeIds: Vec<NodeId>,
}

impl GetSearchResultsReturnsBuilder {
    pub fn build(self) -> GetSearchResultsReturns {
        GetSearchResultsReturns {
            nodeIds: self.nodeIds,
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

impl HideHighlightParams { pub const METHOD: &'static str = "DOM.hideHighlight"; }

impl<'a> crate::CdpCommand<'a> for HideHighlightParams {
    const METHOD: &'static str = "DOM.hideHighlight";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighlightNodeParams {}

impl HighlightNodeParams { pub const METHOD: &'static str = "DOM.highlightNode"; }

impl<'a> crate::CdpCommand<'a> for HighlightNodeParams {
    const METHOD: &'static str = "DOM.highlightNode";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighlightRectParams {}

impl HighlightRectParams { pub const METHOD: &'static str = "DOM.highlightRect"; }

impl<'a> crate::CdpCommand<'a> for HighlightRectParams {
    const METHOD: &'static str = "DOM.highlightRect";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkUndoableStateParams {}

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
    pub fn builder(nodeId: NodeId, targetNodeId: NodeId) -> MoveToParamsBuilder {
        MoveToParamsBuilder {
            nodeId: nodeId,
            targetNodeId: targetNodeId,
            insertBeforeNodeId: None,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn targetNodeId(&self) -> &NodeId { &self.targetNodeId }
    pub fn insertBeforeNodeId(&self) -> Option<&NodeId> { self.insertBeforeNodeId.as_ref() }
}


pub struct MoveToParamsBuilder {
    nodeId: NodeId,
    targetNodeId: NodeId,
    insertBeforeNodeId: Option<NodeId>,
}

impl MoveToParamsBuilder {
    /// Drop node before this one (if absent, the moved node becomes the last child of
    /// 'targetNodeId').
    pub fn insertBeforeNodeId(mut self, insertBeforeNodeId: NodeId) -> Self { self.insertBeforeNodeId = Some(insertBeforeNodeId); self }
    pub fn build(self) -> MoveToParams {
        MoveToParams {
            nodeId: self.nodeId,
            targetNodeId: self.targetNodeId,
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
    pub fn builder(nodeId: NodeId) -> MoveToReturnsBuilder {
        MoveToReturnsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct MoveToReturnsBuilder {
    nodeId: NodeId,
}

impl MoveToReturnsBuilder {
    pub fn build(self) -> MoveToReturns {
        MoveToReturns {
            nodeId: self.nodeId,
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
    pub fn builder(query: impl Into<Cow<'a, str>>) -> PerformSearchParamsBuilder<'a> {
        PerformSearchParamsBuilder {
            query: query.into(),
            includeUserAgentShadowDOM: None,
        }
    }
    pub fn query(&self) -> &str { self.query.as_ref() }
    pub fn includeUserAgentShadowDOM(&self) -> Option<bool> { self.includeUserAgentShadowDOM }
}


pub struct PerformSearchParamsBuilder<'a> {
    query: Cow<'a, str>,
    includeUserAgentShadowDOM: Option<bool>,
}

impl<'a> PerformSearchParamsBuilder<'a> {
    /// True to search in user agent shadow DOM.
    pub fn includeUserAgentShadowDOM(mut self, includeUserAgentShadowDOM: bool) -> Self { self.includeUserAgentShadowDOM = Some(includeUserAgentShadowDOM); self }
    pub fn build(self) -> PerformSearchParams<'a> {
        PerformSearchParams {
            query: self.query,
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
    pub fn builder(searchId: impl Into<Cow<'a, str>>, resultCount: u64) -> PerformSearchReturnsBuilder<'a> {
        PerformSearchReturnsBuilder {
            searchId: searchId.into(),
            resultCount: resultCount,
        }
    }
    pub fn searchId(&self) -> &str { self.searchId.as_ref() }
    pub fn resultCount(&self) -> u64 { self.resultCount }
}


pub struct PerformSearchReturnsBuilder<'a> {
    searchId: Cow<'a, str>,
    resultCount: u64,
}

impl<'a> PerformSearchReturnsBuilder<'a> {
    pub fn build(self) -> PerformSearchReturns<'a> {
        PerformSearchReturns {
            searchId: self.searchId,
            resultCount: self.resultCount,
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
    pub fn builder(path: impl Into<Cow<'a, str>>) -> PushNodeByPathToFrontendParamsBuilder<'a> {
        PushNodeByPathToFrontendParamsBuilder {
            path: path.into(),
        }
    }
    pub fn path(&self) -> &str { self.path.as_ref() }
}


pub struct PushNodeByPathToFrontendParamsBuilder<'a> {
    path: Cow<'a, str>,
}

impl<'a> PushNodeByPathToFrontendParamsBuilder<'a> {
    pub fn build(self) -> PushNodeByPathToFrontendParams<'a> {
        PushNodeByPathToFrontendParams {
            path: self.path,
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
    pub fn builder(nodeId: NodeId) -> PushNodeByPathToFrontendReturnsBuilder {
        PushNodeByPathToFrontendReturnsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct PushNodeByPathToFrontendReturnsBuilder {
    nodeId: NodeId,
}

impl PushNodeByPathToFrontendReturnsBuilder {
    pub fn build(self) -> PushNodeByPathToFrontendReturns {
        PushNodeByPathToFrontendReturns {
            nodeId: self.nodeId,
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
    pub fn builder(backendNodeIds: Vec<BackendNodeId>) -> PushNodesByBackendIdsToFrontendParamsBuilder {
        PushNodesByBackendIdsToFrontendParamsBuilder {
            backendNodeIds: backendNodeIds,
        }
    }
    pub fn backendNodeIds(&self) -> &[BackendNodeId] { &self.backendNodeIds }
}


pub struct PushNodesByBackendIdsToFrontendParamsBuilder {
    backendNodeIds: Vec<BackendNodeId>,
}

impl PushNodesByBackendIdsToFrontendParamsBuilder {
    pub fn build(self) -> PushNodesByBackendIdsToFrontendParams {
        PushNodesByBackendIdsToFrontendParams {
            backendNodeIds: self.backendNodeIds,
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
    pub fn builder(nodeIds: Vec<NodeId>) -> PushNodesByBackendIdsToFrontendReturnsBuilder {
        PushNodesByBackendIdsToFrontendReturnsBuilder {
            nodeIds: nodeIds,
        }
    }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}


pub struct PushNodesByBackendIdsToFrontendReturnsBuilder {
    nodeIds: Vec<NodeId>,
}

impl PushNodesByBackendIdsToFrontendReturnsBuilder {
    pub fn build(self) -> PushNodesByBackendIdsToFrontendReturns {
        PushNodesByBackendIdsToFrontendReturns {
            nodeIds: self.nodeIds,
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
    pub fn builder(nodeId: NodeId, selector: impl Into<Cow<'a, str>>) -> QuerySelectorParamsBuilder<'a> {
        QuerySelectorParamsBuilder {
            nodeId: nodeId,
            selector: selector.into(),
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn selector(&self) -> &str { self.selector.as_ref() }
}


pub struct QuerySelectorParamsBuilder<'a> {
    nodeId: NodeId,
    selector: Cow<'a, str>,
}

impl<'a> QuerySelectorParamsBuilder<'a> {
    pub fn build(self) -> QuerySelectorParams<'a> {
        QuerySelectorParams {
            nodeId: self.nodeId,
            selector: self.selector,
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
    pub fn builder(nodeId: NodeId) -> QuerySelectorReturnsBuilder {
        QuerySelectorReturnsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct QuerySelectorReturnsBuilder {
    nodeId: NodeId,
}

impl QuerySelectorReturnsBuilder {
    pub fn build(self) -> QuerySelectorReturns {
        QuerySelectorReturns {
            nodeId: self.nodeId,
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
    pub fn builder(nodeId: NodeId, selector: impl Into<Cow<'a, str>>) -> QuerySelectorAllParamsBuilder<'a> {
        QuerySelectorAllParamsBuilder {
            nodeId: nodeId,
            selector: selector.into(),
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn selector(&self) -> &str { self.selector.as_ref() }
}


pub struct QuerySelectorAllParamsBuilder<'a> {
    nodeId: NodeId,
    selector: Cow<'a, str>,
}

impl<'a> QuerySelectorAllParamsBuilder<'a> {
    pub fn build(self) -> QuerySelectorAllParams<'a> {
        QuerySelectorAllParams {
            nodeId: self.nodeId,
            selector: self.selector,
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
    pub fn builder(nodeIds: Vec<NodeId>) -> QuerySelectorAllReturnsBuilder {
        QuerySelectorAllReturnsBuilder {
            nodeIds: nodeIds,
        }
    }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}


pub struct QuerySelectorAllReturnsBuilder {
    nodeIds: Vec<NodeId>,
}

impl QuerySelectorAllReturnsBuilder {
    pub fn build(self) -> QuerySelectorAllReturns {
        QuerySelectorAllReturns {
            nodeIds: self.nodeIds,
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
    pub fn builder(nodeIds: Vec<NodeId>) -> GetTopLayerElementsReturnsBuilder {
        GetTopLayerElementsReturnsBuilder {
            nodeIds: nodeIds,
        }
    }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}


pub struct GetTopLayerElementsReturnsBuilder {
    nodeIds: Vec<NodeId>,
}

impl GetTopLayerElementsReturnsBuilder {
    pub fn build(self) -> GetTopLayerElementsReturns {
        GetTopLayerElementsReturns {
            nodeIds: self.nodeIds,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTopLayerElementsParams {}

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
    pub fn builder(nodeId: NodeId, relation: impl Into<Cow<'a, str>>) -> GetElementByRelationParamsBuilder<'a> {
        GetElementByRelationParamsBuilder {
            nodeId: nodeId,
            relation: relation.into(),
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn relation(&self) -> &str { self.relation.as_ref() }
}


pub struct GetElementByRelationParamsBuilder<'a> {
    nodeId: NodeId,
    relation: Cow<'a, str>,
}

impl<'a> GetElementByRelationParamsBuilder<'a> {
    pub fn build(self) -> GetElementByRelationParams<'a> {
        GetElementByRelationParams {
            nodeId: self.nodeId,
            relation: self.relation,
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
    pub fn builder(nodeId: NodeId) -> GetElementByRelationReturnsBuilder {
        GetElementByRelationReturnsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct GetElementByRelationReturnsBuilder {
    nodeId: NodeId,
}

impl GetElementByRelationReturnsBuilder {
    pub fn build(self) -> GetElementByRelationReturns {
        GetElementByRelationReturns {
            nodeId: self.nodeId,
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
    pub fn builder(nodeId: NodeId, name: impl Into<Cow<'a, str>>) -> RemoveAttributeParamsBuilder<'a> {
        RemoveAttributeParamsBuilder {
            nodeId: nodeId,
            name: name.into(),
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn name(&self) -> &str { self.name.as_ref() }
}


pub struct RemoveAttributeParamsBuilder<'a> {
    nodeId: NodeId,
    name: Cow<'a, str>,
}

impl<'a> RemoveAttributeParamsBuilder<'a> {
    pub fn build(self) -> RemoveAttributeParams<'a> {
        RemoveAttributeParams {
            nodeId: self.nodeId,
            name: self.name,
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
    pub fn builder(nodeId: NodeId) -> RemoveNodeParamsBuilder {
        RemoveNodeParamsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct RemoveNodeParamsBuilder {
    nodeId: NodeId,
}

impl RemoveNodeParamsBuilder {
    pub fn build(self) -> RemoveNodeParams {
        RemoveNodeParams {
            nodeId: self.nodeId,
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
    pub fn builder(nodeId: NodeId) -> RequestChildNodesParamsBuilder {
        RequestChildNodesParamsBuilder {
            nodeId: nodeId,
            depth: None,
            pierce: None,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn depth(&self) -> Option<i64> { self.depth }
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}


pub struct RequestChildNodesParamsBuilder {
    nodeId: NodeId,
    depth: Option<i64>,
    pierce: Option<bool>,
}

impl RequestChildNodesParamsBuilder {
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// Whether or not iframes and shadow roots should be traversed when returning the sub-tree
    /// (default is false).
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> RequestChildNodesParams {
        RequestChildNodesParams {
            nodeId: self.nodeId,
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
    pub fn builder(objectId: crate::runtime::RemoteObjectId<'a>) -> RequestNodeParamsBuilder<'a> {
        RequestNodeParamsBuilder {
            objectId: objectId,
        }
    }
    pub fn objectId(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.objectId }
}


pub struct RequestNodeParamsBuilder<'a> {
    objectId: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> RequestNodeParamsBuilder<'a> {
    pub fn build(self) -> RequestNodeParams<'a> {
        RequestNodeParams {
            objectId: self.objectId,
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
    pub fn builder(nodeId: NodeId) -> RequestNodeReturnsBuilder {
        RequestNodeReturnsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct RequestNodeReturnsBuilder {
    nodeId: NodeId,
}

impl RequestNodeReturnsBuilder {
    pub fn build(self) -> RequestNodeReturns {
        RequestNodeReturns {
            nodeId: self.nodeId,
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
    pub fn builder() -> ResolveNodeParamsBuilder<'a> {
        ResolveNodeParamsBuilder {
            nodeId: None,
            backendNodeId: None,
            objectGroup: None,
            executionContextId: None,
        }
    }
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
    pub fn builder(object: crate::runtime::RemoteObject) -> ResolveNodeReturnsBuilder {
        ResolveNodeReturnsBuilder {
            object: object,
        }
    }
    pub fn object(&self) -> &crate::runtime::RemoteObject { &self.object }
}


pub struct ResolveNodeReturnsBuilder {
    object: crate::runtime::RemoteObject,
}

impl ResolveNodeReturnsBuilder {
    pub fn build(self) -> ResolveNodeReturns {
        ResolveNodeReturns {
            object: self.object,
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
    pub fn builder(nodeId: NodeId, name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> SetAttributeValueParamsBuilder<'a> {
        SetAttributeValueParamsBuilder {
            nodeId: nodeId,
            name: name.into(),
            value: value.into(),
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct SetAttributeValueParamsBuilder<'a> {
    nodeId: NodeId,
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SetAttributeValueParamsBuilder<'a> {
    pub fn build(self) -> SetAttributeValueParams<'a> {
        SetAttributeValueParams {
            nodeId: self.nodeId,
            name: self.name,
            value: self.value,
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
    pub fn builder(nodeId: NodeId, text: impl Into<Cow<'a, str>>) -> SetAttributesAsTextParamsBuilder<'a> {
        SetAttributesAsTextParamsBuilder {
            nodeId: nodeId,
            text: text.into(),
            name: None,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
}


pub struct SetAttributesAsTextParamsBuilder<'a> {
    nodeId: NodeId,
    text: Cow<'a, str>,
    name: Option<Cow<'a, str>>,
}

impl<'a> SetAttributesAsTextParamsBuilder<'a> {
    /// Attribute name to replace with new attributes derived from text in case text parsed
    /// successfully.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> SetAttributesAsTextParams<'a> {
        SetAttributesAsTextParams {
            nodeId: self.nodeId,
            text: self.text,
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
    pub fn builder(files: Vec<Cow<'a, str>>) -> SetFileInputFilesParamsBuilder<'a> {
        SetFileInputFilesParamsBuilder {
            files: files,
            nodeId: None,
            backendNodeId: None,
            objectId: None,
        }
    }
    pub fn files(&self) -> &[Cow<'a, str>] { &self.files }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
}


pub struct SetFileInputFilesParamsBuilder<'a> {
    files: Vec<Cow<'a, str>>,
    nodeId: Option<NodeId>,
    backendNodeId: Option<BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> SetFileInputFilesParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node.
    pub fn backendNodeId(mut self, backendNodeId: BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    pub fn build(self) -> SetFileInputFilesParams<'a> {
        SetFileInputFilesParams {
            files: self.files,
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
    pub fn builder(enable: bool) -> SetNodeStackTracesEnabledParamsBuilder {
        SetNodeStackTracesEnabledParamsBuilder {
            enable: enable,
        }
    }
    pub fn enable(&self) -> bool { self.enable }
}


pub struct SetNodeStackTracesEnabledParamsBuilder {
    enable: bool,
}

impl SetNodeStackTracesEnabledParamsBuilder {
    pub fn build(self) -> SetNodeStackTracesEnabledParams {
        SetNodeStackTracesEnabledParams {
            enable: self.enable,
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
    pub fn builder(nodeId: NodeId) -> GetNodeStackTracesParamsBuilder {
        GetNodeStackTracesParamsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct GetNodeStackTracesParamsBuilder {
    nodeId: NodeId,
}

impl GetNodeStackTracesParamsBuilder {
    pub fn build(self) -> GetNodeStackTracesParams {
        GetNodeStackTracesParams {
            nodeId: self.nodeId,
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
    pub fn builder() -> GetNodeStackTracesReturnsBuilder {
        GetNodeStackTracesReturnsBuilder {
            creation: None,
        }
    }
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
    pub fn builder(objectId: crate::runtime::RemoteObjectId<'a>) -> GetFileInfoParamsBuilder<'a> {
        GetFileInfoParamsBuilder {
            objectId: objectId,
        }
    }
    pub fn objectId(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.objectId }
}


pub struct GetFileInfoParamsBuilder<'a> {
    objectId: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> GetFileInfoParamsBuilder<'a> {
    pub fn build(self) -> GetFileInfoParams<'a> {
        GetFileInfoParams {
            objectId: self.objectId,
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
    pub fn builder(path: impl Into<Cow<'a, str>>) -> GetFileInfoReturnsBuilder<'a> {
        GetFileInfoReturnsBuilder {
            path: path.into(),
        }
    }
    pub fn path(&self) -> &str { self.path.as_ref() }
}


pub struct GetFileInfoReturnsBuilder<'a> {
    path: Cow<'a, str>,
}

impl<'a> GetFileInfoReturnsBuilder<'a> {
    pub fn build(self) -> GetFileInfoReturns<'a> {
        GetFileInfoReturns {
            path: self.path,
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
    pub fn builder(detachedNodes: Vec<DetachedElementInfo<'a>>) -> GetDetachedDomNodesReturnsBuilder<'a> {
        GetDetachedDomNodesReturnsBuilder {
            detachedNodes: detachedNodes,
        }
    }
    pub fn detachedNodes(&self) -> &[DetachedElementInfo<'a>] { &self.detachedNodes }
}


pub struct GetDetachedDomNodesReturnsBuilder<'a> {
    detachedNodes: Vec<DetachedElementInfo<'a>>,
}

impl<'a> GetDetachedDomNodesReturnsBuilder<'a> {
    pub fn build(self) -> GetDetachedDomNodesReturns<'a> {
        GetDetachedDomNodesReturns {
            detachedNodes: self.detachedNodes,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDetachedDomNodesParams {}

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
    pub fn builder(nodeId: NodeId) -> SetInspectedNodeParamsBuilder {
        SetInspectedNodeParamsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct SetInspectedNodeParamsBuilder {
    nodeId: NodeId,
}

impl SetInspectedNodeParamsBuilder {
    pub fn build(self) -> SetInspectedNodeParams {
        SetInspectedNodeParams {
            nodeId: self.nodeId,
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
    pub fn builder(nodeId: NodeId, name: impl Into<Cow<'a, str>>) -> SetNodeNameParamsBuilder<'a> {
        SetNodeNameParamsBuilder {
            nodeId: nodeId,
            name: name.into(),
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn name(&self) -> &str { self.name.as_ref() }
}


pub struct SetNodeNameParamsBuilder<'a> {
    nodeId: NodeId,
    name: Cow<'a, str>,
}

impl<'a> SetNodeNameParamsBuilder<'a> {
    pub fn build(self) -> SetNodeNameParams<'a> {
        SetNodeNameParams {
            nodeId: self.nodeId,
            name: self.name,
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
    pub fn builder(nodeId: NodeId) -> SetNodeNameReturnsBuilder {
        SetNodeNameReturnsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct SetNodeNameReturnsBuilder {
    nodeId: NodeId,
}

impl SetNodeNameReturnsBuilder {
    pub fn build(self) -> SetNodeNameReturns {
        SetNodeNameReturns {
            nodeId: self.nodeId,
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
    pub fn builder(nodeId: NodeId, value: impl Into<Cow<'a, str>>) -> SetNodeValueParamsBuilder<'a> {
        SetNodeValueParamsBuilder {
            nodeId: nodeId,
            value: value.into(),
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct SetNodeValueParamsBuilder<'a> {
    nodeId: NodeId,
    value: Cow<'a, str>,
}

impl<'a> SetNodeValueParamsBuilder<'a> {
    pub fn build(self) -> SetNodeValueParams<'a> {
        SetNodeValueParams {
            nodeId: self.nodeId,
            value: self.value,
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
    pub fn builder(nodeId: NodeId, outerHTML: impl Into<Cow<'a, str>>) -> SetOuterHTMLParamsBuilder<'a> {
        SetOuterHTMLParamsBuilder {
            nodeId: nodeId,
            outerHTML: outerHTML.into(),
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn outerHTML(&self) -> &str { self.outerHTML.as_ref() }
}


pub struct SetOuterHTMLParamsBuilder<'a> {
    nodeId: NodeId,
    outerHTML: Cow<'a, str>,
}

impl<'a> SetOuterHTMLParamsBuilder<'a> {
    pub fn build(self) -> SetOuterHTMLParams<'a> {
        SetOuterHTMLParams {
            nodeId: self.nodeId,
            outerHTML: self.outerHTML,
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
    pub fn builder(frameId: crate::page::FrameId<'a>) -> GetFrameOwnerParamsBuilder<'a> {
        GetFrameOwnerParamsBuilder {
            frameId: frameId,
        }
    }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
}


pub struct GetFrameOwnerParamsBuilder<'a> {
    frameId: crate::page::FrameId<'a>,
}

impl<'a> GetFrameOwnerParamsBuilder<'a> {
    pub fn build(self) -> GetFrameOwnerParams<'a> {
        GetFrameOwnerParams {
            frameId: self.frameId,
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
    pub fn builder(backendNodeId: BackendNodeId) -> GetFrameOwnerReturnsBuilder {
        GetFrameOwnerReturnsBuilder {
            backendNodeId: backendNodeId,
            nodeId: None,
        }
    }
    pub fn backendNodeId(&self) -> &BackendNodeId { &self.backendNodeId }
    pub fn nodeId(&self) -> Option<&NodeId> { self.nodeId.as_ref() }
}


pub struct GetFrameOwnerReturnsBuilder {
    backendNodeId: BackendNodeId,
    nodeId: Option<NodeId>,
}

impl GetFrameOwnerReturnsBuilder {
    /// Id of the node at given coordinates, only when enabled and requested document.
    pub fn nodeId(mut self, nodeId: NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetFrameOwnerReturns {
        GetFrameOwnerReturns {
            backendNodeId: self.backendNodeId,
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
    pub fn builder(nodeId: NodeId) -> GetContainerForNodeParamsBuilder<'a> {
        GetContainerForNodeParamsBuilder {
            nodeId: nodeId,
            containerName: None,
            physicalAxes: None,
            logicalAxes: None,
            queriesScrollState: None,
            queriesAnchored: None,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn containerName(&self) -> Option<&str> { self.containerName.as_deref() }
    pub fn physicalAxes(&self) -> Option<&PhysicalAxes> { self.physicalAxes.as_ref() }
    pub fn logicalAxes(&self) -> Option<&LogicalAxes> { self.logicalAxes.as_ref() }
    pub fn queriesScrollState(&self) -> Option<bool> { self.queriesScrollState }
    pub fn queriesAnchored(&self) -> Option<bool> { self.queriesAnchored }
}


pub struct GetContainerForNodeParamsBuilder<'a> {
    nodeId: NodeId,
    containerName: Option<Cow<'a, str>>,
    physicalAxes: Option<PhysicalAxes>,
    logicalAxes: Option<LogicalAxes>,
    queriesScrollState: Option<bool>,
    queriesAnchored: Option<bool>,
}

impl<'a> GetContainerForNodeParamsBuilder<'a> {
    pub fn containerName(mut self, containerName: impl Into<Cow<'a, str>>) -> Self { self.containerName = Some(containerName.into()); self }
    pub fn physicalAxes(mut self, physicalAxes: PhysicalAxes) -> Self { self.physicalAxes = Some(physicalAxes); self }
    pub fn logicalAxes(mut self, logicalAxes: LogicalAxes) -> Self { self.logicalAxes = Some(logicalAxes); self }
    pub fn queriesScrollState(mut self, queriesScrollState: bool) -> Self { self.queriesScrollState = Some(queriesScrollState); self }
    pub fn queriesAnchored(mut self, queriesAnchored: bool) -> Self { self.queriesAnchored = Some(queriesAnchored); self }
    pub fn build(self) -> GetContainerForNodeParams<'a> {
        GetContainerForNodeParams {
            nodeId: self.nodeId,
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
    pub fn builder() -> GetContainerForNodeReturnsBuilder {
        GetContainerForNodeReturnsBuilder {
            nodeId: None,
        }
    }
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
    pub fn builder(nodeId: NodeId) -> GetQueryingDescendantsForContainerParamsBuilder {
        GetQueryingDescendantsForContainerParamsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct GetQueryingDescendantsForContainerParamsBuilder {
    nodeId: NodeId,
}

impl GetQueryingDescendantsForContainerParamsBuilder {
    pub fn build(self) -> GetQueryingDescendantsForContainerParams {
        GetQueryingDescendantsForContainerParams {
            nodeId: self.nodeId,
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
    pub fn builder(nodeIds: Vec<NodeId>) -> GetQueryingDescendantsForContainerReturnsBuilder {
        GetQueryingDescendantsForContainerReturnsBuilder {
            nodeIds: nodeIds,
        }
    }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}


pub struct GetQueryingDescendantsForContainerReturnsBuilder {
    nodeIds: Vec<NodeId>,
}

impl GetQueryingDescendantsForContainerReturnsBuilder {
    pub fn build(self) -> GetQueryingDescendantsForContainerReturns {
        GetQueryingDescendantsForContainerReturns {
            nodeIds: self.nodeIds,
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
    pub fn builder(nodeId: NodeId) -> GetAnchorElementParamsBuilder<'a> {
        GetAnchorElementParamsBuilder {
            nodeId: nodeId,
            anchorSpecifier: None,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn anchorSpecifier(&self) -> Option<&str> { self.anchorSpecifier.as_deref() }
}


pub struct GetAnchorElementParamsBuilder<'a> {
    nodeId: NodeId,
    anchorSpecifier: Option<Cow<'a, str>>,
}

impl<'a> GetAnchorElementParamsBuilder<'a> {
    /// An optional anchor specifier, as defined in
    /// https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier.
    /// If not provided, it will return the implicit anchor element for
    /// the given positioned element.
    pub fn anchorSpecifier(mut self, anchorSpecifier: impl Into<Cow<'a, str>>) -> Self { self.anchorSpecifier = Some(anchorSpecifier.into()); self }
    pub fn build(self) -> GetAnchorElementParams<'a> {
        GetAnchorElementParams {
            nodeId: self.nodeId,
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
    pub fn builder(nodeId: NodeId) -> GetAnchorElementReturnsBuilder {
        GetAnchorElementReturnsBuilder {
            nodeId: nodeId,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
}


pub struct GetAnchorElementReturnsBuilder {
    nodeId: NodeId,
}

impl GetAnchorElementReturnsBuilder {
    pub fn build(self) -> GetAnchorElementReturns {
        GetAnchorElementReturns {
            nodeId: self.nodeId,
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
    pub fn builder(nodeId: NodeId, enable: bool) -> ForceShowPopoverParamsBuilder {
        ForceShowPopoverParamsBuilder {
            nodeId: nodeId,
            enable: enable,
        }
    }
    pub fn nodeId(&self) -> &NodeId { &self.nodeId }
    pub fn enable(&self) -> bool { self.enable }
}


pub struct ForceShowPopoverParamsBuilder {
    nodeId: NodeId,
    enable: bool,
}

impl ForceShowPopoverParamsBuilder {
    pub fn build(self) -> ForceShowPopoverParams {
        ForceShowPopoverParams {
            nodeId: self.nodeId,
            enable: self.enable,
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
    pub fn builder(nodeIds: Vec<NodeId>) -> ForceShowPopoverReturnsBuilder {
        ForceShowPopoverReturnsBuilder {
            nodeIds: nodeIds,
        }
    }
    pub fn nodeIds(&self) -> &[NodeId] { &self.nodeIds }
}


pub struct ForceShowPopoverReturnsBuilder {
    nodeIds: Vec<NodeId>,
}

impl ForceShowPopoverReturnsBuilder {
    pub fn build(self) -> ForceShowPopoverReturns {
        ForceShowPopoverReturns {
            nodeIds: self.nodeIds,
        }
    }
}

impl ForceShowPopoverParams { pub const METHOD: &'static str = "DOM.forceShowPopover"; }

impl<'a> crate::CdpCommand<'a> for ForceShowPopoverParams {
    const METHOD: &'static str = "DOM.forceShowPopover";
    type Response = ForceShowPopoverReturns;
}
