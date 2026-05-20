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
    #[serde(rename = "nodeType")]
    node_type: i64,
    /// 'Node''s nodeName.
    #[serde(rename = "nodeName")]
    node_name: Cow<'a, str>,
    #[serde(rename = "backendNodeId")]
    backend_node_id: BackendNodeId,
}

impl<'a> BackendNode<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_type`: `Node`'s nodeType.
    /// * `node_name`: `Node`'s nodeName.
    /// * `backend_node_id`: 
    pub fn builder(node_type: i64, node_name: impl Into<Cow<'a, str>>, backend_node_id: BackendNodeId) -> BackendNodeBuilder<'a> {
        BackendNodeBuilder {
            node_type: node_type,
            node_name: node_name.into(),
            backend_node_id: backend_node_id,
        }
    }
    /// 'Node''s nodeType.
    pub fn node_type(&self) -> i64 { self.node_type }
    /// 'Node''s nodeName.
    pub fn node_name(&self) -> &str { self.node_name.as_ref() }
    pub fn backend_node_id(&self) -> &BackendNodeId { &self.backend_node_id }
}


pub struct BackendNodeBuilder<'a> {
    node_type: i64,
    node_name: Cow<'a, str>,
    backend_node_id: BackendNodeId,
}

impl<'a> BackendNodeBuilder<'a> {
    pub fn build(self) -> BackendNode<'a> {
        BackendNode {
            node_type: self.node_type,
            node_name: self.node_name,
            backend_node_id: self.backend_node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// The id of the parent node if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    parent_id: Option<NodeId>,
    /// The BackendNodeId for this node.
    #[serde(rename = "backendNodeId")]
    backend_node_id: BackendNodeId,
    /// 'Node''s nodeType.
    #[serde(rename = "nodeType")]
    node_type: i64,
    /// 'Node''s nodeName.
    #[serde(rename = "nodeName")]
    node_name: Cow<'a, str>,
    /// 'Node''s localName.
    #[serde(rename = "localName")]
    local_name: Cow<'a, str>,
    /// 'Node''s nodeValue.
    #[serde(rename = "nodeValue")]
    node_value: Cow<'a, str>,
    /// Child count for 'Container' nodes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childNodeCount")]
    child_node_count: Option<u64>,
    /// Child nodes of this node when requested with children.
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Box<Node<'a>>>>,
    /// Attributes of the 'Element' node in the form of flat array '\[name1, value1, name2, value2\]'.
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<Cow<'a, str>>>,
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    #[serde(skip_serializing_if = "Option::is_none", rename = "documentURL")]
    document_url: Option<Cow<'a, str>>,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseURL")]
    base_url: Option<Cow<'a, str>>,
    /// 'DocumentType''s publicId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "publicId")]
    public_id: Option<Cow<'a, str>>,
    /// 'DocumentType''s systemId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "systemId")]
    system_id: Option<Cow<'a, str>>,
    /// 'DocumentType''s internalSubset.
    #[serde(skip_serializing_if = "Option::is_none", rename = "internalSubset")]
    internal_subset: Option<Cow<'a, str>>,
    /// 'Document''s XML version in case of XML documents.
    #[serde(skip_serializing_if = "Option::is_none", rename = "xmlVersion")]
    xml_version: Option<Cow<'a, str>>,
    /// 'Attr''s name.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    /// 'Attr''s value.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Cow<'a, str>>,
    /// Pseudo element type for this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoType")]
    pseudo_type: Option<PseudoType>,
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoIdentifier")]
    pseudo_identifier: Option<Cow<'a, str>>,
    /// Shadow root type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "shadowRootType")]
    shadow_root_type: Option<ShadowRootType>,
    /// Frame ID for frame owner elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
    /// Content document for frame owner elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentDocument")]
    content_document: Option<Box<Node<'a>>>,
    /// Shadow root list for given element host.
    #[serde(skip_serializing_if = "Option::is_none", rename = "shadowRoots")]
    shadow_roots: Option<Vec<Box<Node<'a>>>>,
    /// Content document fragment for template elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "templateContent")]
    template_content: Option<Box<Node<'a>>>,
    /// Pseudo elements associated with this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoElements")]
    pseudo_elements: Option<Vec<Box<Node<'a>>>>,
    /// Deprecated, as the HTML Imports API has been removed (crbug.com/937746).
    /// This property used to return the imported document for the HTMLImport links.
    /// The property is always undefined now.
    #[serde(skip_serializing_if = "Option::is_none", rename = "importedDocument")]
    imported_document: Option<Box<Node<'a>>>,
    /// Distributed nodes for given insertion point.
    #[serde(skip_serializing_if = "Option::is_none", rename = "distributedNodes")]
    distributed_nodes: Option<Vec<BackendNode<'a>>>,
    /// Whether the node is SVG.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isSVG")]
    is_svg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "compatibilityMode")]
    compatibility_mode: Option<CompatibilityMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "assignedSlot")]
    assigned_slot: Option<BackendNode<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isScrollable")]
    is_scrollable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "affectedByStartingStyles")]
    affected_by_starting_styles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "adoptedStyleSheets")]
    adopted_style_sheets: Option<Vec<StyleSheetId<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "adProvenance")]
    ad_provenance: Option<crate::network::AdProvenance<'a>>,
}

impl<'a> Node<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Node identifier that is passed into the rest of the DOM messages as the `nodeId`. Backend will only push node with given `id` once. It is aware of all requested nodes and will only fire DOM events for nodes known to the client.
    /// * `backend_node_id`: The BackendNodeId for this node.
    /// * `node_type`: `Node`'s nodeType.
    /// * `node_name`: `Node`'s nodeName.
    /// * `local_name`: `Node`'s localName.
    /// * `node_value`: `Node`'s nodeValue.
    pub fn builder(node_id: NodeId, backend_node_id: BackendNodeId, node_type: i64, node_name: impl Into<Cow<'a, str>>, local_name: impl Into<Cow<'a, str>>, node_value: impl Into<Cow<'a, str>>) -> NodeBuilder<'a> {
        NodeBuilder {
            node_id: node_id,
            parent_id: None,
            backend_node_id: backend_node_id,
            node_type: node_type,
            node_name: node_name.into(),
            local_name: local_name.into(),
            node_value: node_value.into(),
            child_node_count: None,
            children: None,
            attributes: None,
            document_url: None,
            base_url: None,
            public_id: None,
            system_id: None,
            internal_subset: None,
            xml_version: None,
            name: None,
            value: None,
            pseudo_type: None,
            pseudo_identifier: None,
            shadow_root_type: None,
            frame_id: None,
            content_document: None,
            shadow_roots: None,
            template_content: None,
            pseudo_elements: None,
            imported_document: None,
            distributed_nodes: None,
            is_svg: None,
            compatibility_mode: None,
            assigned_slot: None,
            is_scrollable: None,
            affected_by_starting_styles: None,
            adopted_style_sheets: None,
            ad_provenance: None,
        }
    }
    /// Node identifier that is passed into the rest of the DOM messages as the 'nodeId'. Backend
    /// will only push node with given 'id' once. It is aware of all requested nodes and will only
    /// fire DOM events for nodes known to the client.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// The id of the parent node if any.
    pub fn parent_id(&self) -> Option<&NodeId> { self.parent_id.as_ref() }
    /// The BackendNodeId for this node.
    pub fn backend_node_id(&self) -> &BackendNodeId { &self.backend_node_id }
    /// 'Node''s nodeType.
    pub fn node_type(&self) -> i64 { self.node_type }
    /// 'Node''s nodeName.
    pub fn node_name(&self) -> &str { self.node_name.as_ref() }
    /// 'Node''s localName.
    pub fn local_name(&self) -> &str { self.local_name.as_ref() }
    /// 'Node''s nodeValue.
    pub fn node_value(&self) -> &str { self.node_value.as_ref() }
    /// Child count for 'Container' nodes.
    pub fn child_node_count(&self) -> Option<u64> { self.child_node_count }
    /// Child nodes of this node when requested with children.
    pub fn children(&self) -> Option<&[Box<Node<'a>>]> { self.children.as_deref() }
    /// Attributes of the 'Element' node in the form of flat array '\[name1, value1, name2, value2\]'.
    pub fn attributes(&self) -> Option<&[Cow<'a, str>]> { self.attributes.as_deref() }
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    pub fn document_url(&self) -> Option<&str> { self.document_url.as_deref() }
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    pub fn base_url(&self) -> Option<&str> { self.base_url.as_deref() }
    /// 'DocumentType''s publicId.
    pub fn public_id(&self) -> Option<&str> { self.public_id.as_deref() }
    /// 'DocumentType''s systemId.
    pub fn system_id(&self) -> Option<&str> { self.system_id.as_deref() }
    /// 'DocumentType''s internalSubset.
    pub fn internal_subset(&self) -> Option<&str> { self.internal_subset.as_deref() }
    /// 'Document''s XML version in case of XML documents.
    pub fn xml_version(&self) -> Option<&str> { self.xml_version.as_deref() }
    /// 'Attr''s name.
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    /// 'Attr''s value.
    pub fn value(&self) -> Option<&str> { self.value.as_deref() }
    /// Pseudo element type for this node.
    pub fn pseudo_type(&self) -> Option<&PseudoType> { self.pseudo_type.as_ref() }
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    pub fn pseudo_identifier(&self) -> Option<&str> { self.pseudo_identifier.as_deref() }
    /// Shadow root type.
    pub fn shadow_root_type(&self) -> Option<&ShadowRootType> { self.shadow_root_type.as_ref() }
    /// Frame ID for frame owner elements.
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
    /// Content document for frame owner elements.
    pub fn content_document(&self) -> Option<&Node<'a>> { self.content_document.as_deref() }
    /// Shadow root list for given element host.
    pub fn shadow_roots(&self) -> Option<&[Box<Node<'a>>]> { self.shadow_roots.as_deref() }
    /// Content document fragment for template elements.
    pub fn template_content(&self) -> Option<&Node<'a>> { self.template_content.as_deref() }
    /// Pseudo elements associated with this node.
    pub fn pseudo_elements(&self) -> Option<&[Box<Node<'a>>]> { self.pseudo_elements.as_deref() }
    /// Deprecated, as the HTML Imports API has been removed (crbug.com/937746).
    /// This property used to return the imported document for the HTMLImport links.
    /// The property is always undefined now.
    pub fn imported_document(&self) -> Option<&Node<'a>> { self.imported_document.as_deref() }
    /// Distributed nodes for given insertion point.
    pub fn distributed_nodes(&self) -> Option<&[BackendNode<'a>]> { self.distributed_nodes.as_deref() }
    /// Whether the node is SVG.
    pub fn is_svg(&self) -> Option<bool> { self.is_svg }
    pub fn compatibility_mode(&self) -> Option<&CompatibilityMode> { self.compatibility_mode.as_ref() }
    pub fn assigned_slot(&self) -> Option<&BackendNode<'a>> { self.assigned_slot.as_ref() }
    pub fn is_scrollable(&self) -> Option<bool> { self.is_scrollable }
    pub fn affected_by_starting_styles(&self) -> Option<bool> { self.affected_by_starting_styles }
    pub fn adopted_style_sheets(&self) -> Option<&[StyleSheetId<'a>]> { self.adopted_style_sheets.as_deref() }
    pub fn ad_provenance(&self) -> Option<&crate::network::AdProvenance<'a>> { self.ad_provenance.as_ref() }
}


pub struct NodeBuilder<'a> {
    node_id: NodeId,
    parent_id: Option<NodeId>,
    backend_node_id: BackendNodeId,
    node_type: i64,
    node_name: Cow<'a, str>,
    local_name: Cow<'a, str>,
    node_value: Cow<'a, str>,
    child_node_count: Option<u64>,
    children: Option<Vec<Box<Node<'a>>>>,
    attributes: Option<Vec<Cow<'a, str>>>,
    document_url: Option<Cow<'a, str>>,
    base_url: Option<Cow<'a, str>>,
    public_id: Option<Cow<'a, str>>,
    system_id: Option<Cow<'a, str>>,
    internal_subset: Option<Cow<'a, str>>,
    xml_version: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    pseudo_type: Option<PseudoType>,
    pseudo_identifier: Option<Cow<'a, str>>,
    shadow_root_type: Option<ShadowRootType>,
    frame_id: Option<crate::page::FrameId<'a>>,
    content_document: Option<Box<Node<'a>>>,
    shadow_roots: Option<Vec<Box<Node<'a>>>>,
    template_content: Option<Box<Node<'a>>>,
    pseudo_elements: Option<Vec<Box<Node<'a>>>>,
    imported_document: Option<Box<Node<'a>>>,
    distributed_nodes: Option<Vec<BackendNode<'a>>>,
    is_svg: Option<bool>,
    compatibility_mode: Option<CompatibilityMode>,
    assigned_slot: Option<BackendNode<'a>>,
    is_scrollable: Option<bool>,
    affected_by_starting_styles: Option<bool>,
    adopted_style_sheets: Option<Vec<StyleSheetId<'a>>>,
    ad_provenance: Option<crate::network::AdProvenance<'a>>,
}

impl<'a> NodeBuilder<'a> {
    /// The id of the parent node if any.
    pub fn parent_id(mut self, parent_id: NodeId) -> Self { self.parent_id = Some(parent_id); self }
    /// Child count for 'Container' nodes.
    pub fn child_node_count(mut self, child_node_count: u64) -> Self { self.child_node_count = Some(child_node_count); self }
    /// Child nodes of this node when requested with children.
    pub fn children(mut self, children: Vec<Box<Node<'a>>>) -> Self { self.children = Some(children); self }
    /// Attributes of the 'Element' node in the form of flat array '\[name1, value1, name2, value2\]'.
    pub fn attributes(mut self, attributes: Vec<Cow<'a, str>>) -> Self { self.attributes = Some(attributes); self }
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    pub fn document_url(mut self, document_url: impl Into<Cow<'a, str>>) -> Self { self.document_url = Some(document_url.into()); self }
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    pub fn base_url(mut self, base_url: impl Into<Cow<'a, str>>) -> Self { self.base_url = Some(base_url.into()); self }
    /// 'DocumentType''s publicId.
    pub fn public_id(mut self, public_id: impl Into<Cow<'a, str>>) -> Self { self.public_id = Some(public_id.into()); self }
    /// 'DocumentType''s systemId.
    pub fn system_id(mut self, system_id: impl Into<Cow<'a, str>>) -> Self { self.system_id = Some(system_id.into()); self }
    /// 'DocumentType''s internalSubset.
    pub fn internal_subset(mut self, internal_subset: impl Into<Cow<'a, str>>) -> Self { self.internal_subset = Some(internal_subset.into()); self }
    /// 'Document''s XML version in case of XML documents.
    pub fn xml_version(mut self, xml_version: impl Into<Cow<'a, str>>) -> Self { self.xml_version = Some(xml_version.into()); self }
    /// 'Attr''s name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// 'Attr''s value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// Pseudo element type for this node.
    pub fn pseudo_type(mut self, pseudo_type: impl Into<PseudoType>) -> Self { self.pseudo_type = Some(pseudo_type.into()); self }
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    pub fn pseudo_identifier(mut self, pseudo_identifier: impl Into<Cow<'a, str>>) -> Self { self.pseudo_identifier = Some(pseudo_identifier.into()); self }
    /// Shadow root type.
    pub fn shadow_root_type(mut self, shadow_root_type: impl Into<ShadowRootType>) -> Self { self.shadow_root_type = Some(shadow_root_type.into()); self }
    /// Frame ID for frame owner elements.
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    /// Content document for frame owner elements.
    pub fn content_document(mut self, content_document: Box<Node<'a>>) -> Self { self.content_document = Some(content_document); self }
    /// Shadow root list for given element host.
    pub fn shadow_roots(mut self, shadow_roots: Vec<Box<Node<'a>>>) -> Self { self.shadow_roots = Some(shadow_roots); self }
    /// Content document fragment for template elements.
    pub fn template_content(mut self, template_content: Box<Node<'a>>) -> Self { self.template_content = Some(template_content); self }
    /// Pseudo elements associated with this node.
    pub fn pseudo_elements(mut self, pseudo_elements: Vec<Box<Node<'a>>>) -> Self { self.pseudo_elements = Some(pseudo_elements); self }
    /// Deprecated, as the HTML Imports API has been removed (crbug.com/937746).
    /// This property used to return the imported document for the HTMLImport links.
    /// The property is always undefined now.
    pub fn imported_document(mut self, imported_document: Box<Node<'a>>) -> Self { self.imported_document = Some(imported_document); self }
    /// Distributed nodes for given insertion point.
    pub fn distributed_nodes(mut self, distributed_nodes: Vec<BackendNode<'a>>) -> Self { self.distributed_nodes = Some(distributed_nodes); self }
    /// Whether the node is SVG.
    pub fn is_svg(mut self, is_svg: bool) -> Self { self.is_svg = Some(is_svg); self }
    pub fn compatibility_mode(mut self, compatibility_mode: impl Into<CompatibilityMode>) -> Self { self.compatibility_mode = Some(compatibility_mode.into()); self }
    pub fn assigned_slot(mut self, assigned_slot: BackendNode<'a>) -> Self { self.assigned_slot = Some(assigned_slot); self }
    pub fn is_scrollable(mut self, is_scrollable: bool) -> Self { self.is_scrollable = Some(is_scrollable); self }
    pub fn affected_by_starting_styles(mut self, affected_by_starting_styles: bool) -> Self { self.affected_by_starting_styles = Some(affected_by_starting_styles); self }
    pub fn adopted_style_sheets(mut self, adopted_style_sheets: Vec<StyleSheetId<'a>>) -> Self { self.adopted_style_sheets = Some(adopted_style_sheets); self }
    pub fn ad_provenance(mut self, ad_provenance: crate::network::AdProvenance<'a>) -> Self { self.ad_provenance = Some(ad_provenance); self }
    pub fn build(self) -> Node<'a> {
        Node {
            node_id: self.node_id,
            parent_id: self.parent_id,
            backend_node_id: self.backend_node_id,
            node_type: self.node_type,
            node_name: self.node_name,
            local_name: self.local_name,
            node_value: self.node_value,
            child_node_count: self.child_node_count,
            children: self.children,
            attributes: self.attributes,
            document_url: self.document_url,
            base_url: self.base_url,
            public_id: self.public_id,
            system_id: self.system_id,
            internal_subset: self.internal_subset,
            xml_version: self.xml_version,
            name: self.name,
            value: self.value,
            pseudo_type: self.pseudo_type,
            pseudo_identifier: self.pseudo_identifier,
            shadow_root_type: self.shadow_root_type,
            frame_id: self.frame_id,
            content_document: self.content_document,
            shadow_roots: self.shadow_roots,
            template_content: self.template_content,
            pseudo_elements: self.pseudo_elements,
            imported_document: self.imported_document,
            distributed_nodes: self.distributed_nodes,
            is_svg: self.is_svg,
            compatibility_mode: self.compatibility_mode,
            assigned_slot: self.assigned_slot,
            is_scrollable: self.is_scrollable,
            affected_by_starting_styles: self.affected_by_starting_styles,
            adopted_style_sheets: self.adopted_style_sheets,
            ad_provenance: self.ad_provenance,
        }
    }
}

/// A structure to hold the top-level node of a detached tree and an array of its retained descendants.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DetachedElementInfo<'a> {
    #[serde(rename = "treeNode")]
    tree_node: Node<'a>,
    #[serde(rename = "retainedNodeIds")]
    retained_node_ids: Vec<NodeId>,
}

impl<'a> DetachedElementInfo<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `tree_node`: 
    /// * `retained_node_ids`: 
    pub fn builder(tree_node: Node<'a>, retained_node_ids: Vec<NodeId>) -> DetachedElementInfoBuilder<'a> {
        DetachedElementInfoBuilder {
            tree_node: tree_node,
            retained_node_ids: retained_node_ids,
        }
    }
    pub fn tree_node(&self) -> &Node<'a> { &self.tree_node }
    pub fn retained_node_ids(&self) -> &[NodeId] { &self.retained_node_ids }
}


pub struct DetachedElementInfoBuilder<'a> {
    tree_node: Node<'a>,
    retained_node_ids: Vec<NodeId>,
}

impl<'a> DetachedElementInfoBuilder<'a> {
    pub fn build(self) -> DetachedElementInfo<'a> {
        DetachedElementInfo {
            tree_node: self.tree_node,
            retained_node_ids: self.retained_node_ids,
        }
    }
}

/// A structure holding an RGBA color.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RGBA {
    /// The red component, in the \[0-255\] range.
    r: i64,
    /// The green component, in the \[0-255\] range.
    g: i64,
    /// The blue component, in the \[0-255\] range.
    b: i64,
    /// The alpha component, in the \[0-1\] range (default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<f64>,
}

impl RGBA {
    /// Creates a builder for this type with the required parameters:
    /// * `r`: The red component, in the \[0-255\] range.
    /// * `g`: The green component, in the \[0-255\] range.
    /// * `b`: The blue component, in the \[0-255\] range.
    pub fn builder(r: i64, g: i64, b: i64) -> RGBABuilder {
        RGBABuilder {
            r: r,
            g: g,
            b: b,
            a: None,
        }
    }
    /// The red component, in the \[0-255\] range.
    pub fn r(&self) -> i64 { self.r }
    /// The green component, in the \[0-255\] range.
    pub fn g(&self) -> i64 { self.g }
    /// The blue component, in the \[0-255\] range.
    pub fn b(&self) -> i64 { self.b }
    /// The alpha component, in the \[0-1\] range (default: 1).
    pub fn a(&self) -> Option<f64> { self.a }
}


pub struct RGBABuilder {
    r: i64,
    g: i64,
    b: i64,
    a: Option<f64>,
}

impl RGBABuilder {
    /// The alpha component, in the \[0-1\] range (default: 1).
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "shapeOutside")]
    shape_outside: Option<ShapeOutsideInfo>,
}

impl BoxModel {
    /// Creates a builder for this type with the required parameters:
    /// * `content`: Content box
    /// * `padding`: Padding box
    /// * `border`: Border box
    /// * `margin`: Margin box
    /// * `width`: Node width
    /// * `height`: Node height
    pub fn builder(content: Quad, padding: Quad, border: Quad, margin: Quad, width: u64, height: i64) -> BoxModelBuilder {
        BoxModelBuilder {
            content: content,
            padding: padding,
            border: border,
            margin: margin,
            width: width,
            height: height,
            shape_outside: None,
        }
    }
    /// Content box
    pub fn content(&self) -> &Quad { &self.content }
    /// Padding box
    pub fn padding(&self) -> &Quad { &self.padding }
    /// Border box
    pub fn border(&self) -> &Quad { &self.border }
    /// Margin box
    pub fn margin(&self) -> &Quad { &self.margin }
    /// Node width
    pub fn width(&self) -> u64 { self.width }
    /// Node height
    pub fn height(&self) -> i64 { self.height }
    /// Shape outside coordinates
    pub fn shape_outside(&self) -> Option<&ShapeOutsideInfo> { self.shape_outside.as_ref() }
}


pub struct BoxModelBuilder {
    content: Quad,
    padding: Quad,
    border: Quad,
    margin: Quad,
    width: u64,
    height: i64,
    shape_outside: Option<ShapeOutsideInfo>,
}

impl BoxModelBuilder {
    /// Shape outside coordinates
    pub fn shape_outside(mut self, shape_outside: ShapeOutsideInfo) -> Self { self.shape_outside = Some(shape_outside); self }
    pub fn build(self) -> BoxModel {
        BoxModel {
            content: self.content,
            padding: self.padding,
            border: self.border,
            margin: self.margin,
            width: self.width,
            height: self.height,
            shape_outside: self.shape_outside,
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
    #[serde(rename = "marginShape")]
    margin_shape: Vec<JsonValue>,
}

impl ShapeOutsideInfo {
    /// Creates a builder for this type with the required parameters:
    /// * `bounds`: Shape bounds
    /// * `shape`: Shape coordinate details
    /// * `margin_shape`: Margin shape bounds
    pub fn builder(bounds: Quad, shape: Vec<JsonValue>, margin_shape: Vec<JsonValue>) -> ShapeOutsideInfoBuilder {
        ShapeOutsideInfoBuilder {
            bounds: bounds,
            shape: shape,
            margin_shape: margin_shape,
        }
    }
    /// Shape bounds
    pub fn bounds(&self) -> &Quad { &self.bounds }
    /// Shape coordinate details
    pub fn shape(&self) -> &[JsonValue] { &self.shape }
    /// Margin shape bounds
    pub fn margin_shape(&self) -> &[JsonValue] { &self.margin_shape }
}


pub struct ShapeOutsideInfoBuilder {
    bounds: Quad,
    shape: Vec<JsonValue>,
    margin_shape: Vec<JsonValue>,
}

impl ShapeOutsideInfoBuilder {
    pub fn build(self) -> ShapeOutsideInfo {
        ShapeOutsideInfo {
            bounds: self.bounds,
            shape: self.shape,
            margin_shape: self.margin_shape,
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
    /// Creates a builder for this type with the required parameters:
    /// * `x`: X coordinate
    /// * `y`: Y coordinate
    /// * `width`: Rectangle width
    /// * `height`: Rectangle height
    pub fn builder(x: f64, y: f64, width: f64, height: f64) -> RectBuilder {
        RectBuilder {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
    /// X coordinate
    pub fn x(&self) -> f64 { self.x }
    /// Y coordinate
    pub fn y(&self) -> f64 { self.y }
    /// Rectangle width
    pub fn width(&self) -> f64 { self.width }
    /// Rectangle height
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Computed style property name.
    /// * `value`: Computed style property value.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> CSSComputedStylePropertyBuilder<'a> {
        CSSComputedStylePropertyBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    /// Computed style property name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Computed style property value.
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl CollectClassNamesFromSubtreeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to collect class names.
    pub fn builder(node_id: NodeId) -> CollectClassNamesFromSubtreeParamsBuilder {
        CollectClassNamesFromSubtreeParamsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the node to collect class names.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct CollectClassNamesFromSubtreeParamsBuilder {
    node_id: NodeId,
}

impl CollectClassNamesFromSubtreeParamsBuilder {
    pub fn build(self) -> CollectClassNamesFromSubtreeParams {
        CollectClassNamesFromSubtreeParams {
            node_id: self.node_id,
        }
    }
}

/// Collects class names for the node with given id and all of it's child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesFromSubtreeReturns<'a> {
    /// Class name list.
    #[serde(rename = "classNames")]
    class_names: Vec<Cow<'a, str>>,
}

impl<'a> CollectClassNamesFromSubtreeReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `class_names`: Class name list.
    pub fn builder(class_names: Vec<Cow<'a, str>>) -> CollectClassNamesFromSubtreeReturnsBuilder<'a> {
        CollectClassNamesFromSubtreeReturnsBuilder {
            class_names: class_names,
        }
    }
    /// Class name list.
    pub fn class_names(&self) -> &[Cow<'a, str>] { &self.class_names }
}


pub struct CollectClassNamesFromSubtreeReturnsBuilder<'a> {
    class_names: Vec<Cow<'a, str>>,
}

impl<'a> CollectClassNamesFromSubtreeReturnsBuilder<'a> {
    pub fn build(self) -> CollectClassNamesFromSubtreeReturns<'a> {
        CollectClassNamesFromSubtreeReturns {
            class_names: self.class_names,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// Id of the element to drop the copy into.
    #[serde(rename = "targetNodeId")]
    target_node_id: NodeId,
    /// Drop the copy before this node (if absent, the copy becomes the last child of
    /// 'targetNodeId').
    #[serde(skip_serializing_if = "Option::is_none", rename = "insertBeforeNodeId")]
    insert_before_node_id: Option<NodeId>,
}

impl CopyToParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to copy.
    /// * `target_node_id`: Id of the element to drop the copy into.
    pub fn builder(node_id: NodeId, target_node_id: NodeId) -> CopyToParamsBuilder {
        CopyToParamsBuilder {
            node_id: node_id,
            target_node_id: target_node_id,
            insert_before_node_id: None,
        }
    }
    /// Id of the node to copy.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// Id of the element to drop the copy into.
    pub fn target_node_id(&self) -> &NodeId { &self.target_node_id }
    /// Drop the copy before this node (if absent, the copy becomes the last child of
    /// 'targetNodeId').
    pub fn insert_before_node_id(&self) -> Option<&NodeId> { self.insert_before_node_id.as_ref() }
}


pub struct CopyToParamsBuilder {
    node_id: NodeId,
    target_node_id: NodeId,
    insert_before_node_id: Option<NodeId>,
}

impl CopyToParamsBuilder {
    /// Drop the copy before this node (if absent, the copy becomes the last child of
    /// 'targetNodeId').
    pub fn insert_before_node_id(mut self, insert_before_node_id: NodeId) -> Self { self.insert_before_node_id = Some(insert_before_node_id); self }
    pub fn build(self) -> CopyToParams {
        CopyToParams {
            node_id: self.node_id,
            target_node_id: self.target_node_id,
            insert_before_node_id: self.insert_before_node_id,
        }
    }
}

/// Creates a deep copy of the specified node and places it into the target container before the
/// given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CopyToReturns {
    /// Id of the node clone.
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl CopyToReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node clone.
    pub fn builder(node_id: NodeId) -> CopyToReturnsBuilder {
        CopyToReturnsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the node clone.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct CopyToReturnsBuilder {
    node_id: NodeId,
}

impl CopyToReturnsBuilder {
    pub fn build(self) -> CopyToReturns {
        CopyToReturns {
            node_id: self.node_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
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
    /// Creates a builder for this type.
    pub fn builder() -> DescribeNodeParamsBuilder<'a> {
        DescribeNodeParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_id: None,
            depth: None,
            pierce: None,
        }
    }
    /// Identifier of the node.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node.
    pub fn backend_node_id(&self) -> Option<&BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(&self) -> Option<i64> { self.depth }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}

#[derive(Default)]
pub struct DescribeNodeParamsBuilder<'a> {
    node_id: Option<NodeId>,
    backend_node_id: Option<BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    depth: Option<i64>,
    pierce: Option<bool>,
}

impl<'a> DescribeNodeParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node.
    pub fn backend_node_id(mut self, backend_node_id: BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> DescribeNodeParams<'a> {
        DescribeNodeParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `node`: Node description.
    pub fn builder(node: Node<'a>) -> DescribeNodeReturnsBuilder<'a> {
        DescribeNodeReturnsBuilder {
            node: node,
        }
    }
    /// Node description.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
    /// When omitted, center of the node will be used, similar to Element.scrollIntoView.
    #[serde(skip_serializing_if = "Option::is_none")]
    rect: Option<Rect>,
}

impl<'a> ScrollIntoViewIfNeededParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> ScrollIntoViewIfNeededParamsBuilder<'a> {
        ScrollIntoViewIfNeededParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_id: None,
            rect: None,
        }
    }
    /// Identifier of the node.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node.
    pub fn backend_node_id(&self) -> Option<&BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
    /// The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
    /// When omitted, center of the node will be used, similar to Element.scrollIntoView.
    pub fn rect(&self) -> Option<&Rect> { self.rect.as_ref() }
}

#[derive(Default)]
pub struct ScrollIntoViewIfNeededParamsBuilder<'a> {
    node_id: Option<NodeId>,
    backend_node_id: Option<BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    rect: Option<Rect>,
}

impl<'a> ScrollIntoViewIfNeededParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node.
    pub fn backend_node_id(mut self, backend_node_id: BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    /// The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
    /// When omitted, center of the node will be used, similar to Element.scrollIntoView.
    pub fn rect(mut self, rect: Rect) -> Self { self.rect = Some(rect); self }
    pub fn build(self) -> ScrollIntoViewIfNeededParams<'a> {
        ScrollIntoViewIfNeededParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
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
    #[serde(rename = "searchId")]
    search_id: Cow<'a, str>,
}

impl<'a> DiscardSearchResultsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `search_id`: Unique search session identifier.
    pub fn builder(search_id: impl Into<Cow<'a, str>>) -> DiscardSearchResultsParamsBuilder<'a> {
        DiscardSearchResultsParamsBuilder {
            search_id: search_id.into(),
        }
    }
    /// Unique search session identifier.
    pub fn search_id(&self) -> &str { self.search_id.as_ref() }
}


pub struct DiscardSearchResultsParamsBuilder<'a> {
    search_id: Cow<'a, str>,
}

impl<'a> DiscardSearchResultsParamsBuilder<'a> {
    pub fn build(self) -> DiscardSearchResultsParams<'a> {
        DiscardSearchResultsParams {
            search_id: self.search_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeWhitespace")]
    include_whitespace: Option<Cow<'a, str>>,
}

impl<'a> EnableParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> EnableParamsBuilder<'a> {
        EnableParamsBuilder {
            include_whitespace: None,
        }
    }
    /// Whether to include whitespaces in the children array of returned Nodes.
    pub fn include_whitespace(&self) -> Option<&str> { self.include_whitespace.as_deref() }
}

#[derive(Default)]
pub struct EnableParamsBuilder<'a> {
    include_whitespace: Option<Cow<'a, str>>,
}

impl<'a> EnableParamsBuilder<'a> {
    /// Whether to include whitespaces in the children array of returned Nodes.
    pub fn include_whitespace(mut self, include_whitespace: impl Into<Cow<'a, str>>) -> Self { self.include_whitespace = Some(include_whitespace.into()); self }
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            include_whitespace: self.include_whitespace,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> FocusParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> FocusParamsBuilder<'a> {
        FocusParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_id: None,
        }
    }
    /// Identifier of the node.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node.
    pub fn backend_node_id(&self) -> Option<&BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
}

#[derive(Default)]
pub struct FocusParamsBuilder<'a> {
    node_id: Option<NodeId>,
    backend_node_id: Option<BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> FocusParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node.
    pub fn backend_node_id(mut self, backend_node_id: BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    pub fn build(self) -> FocusParams<'a> {
        FocusParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl GetAttributesParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to retrieve attributes for.
    pub fn builder(node_id: NodeId) -> GetAttributesParamsBuilder {
        GetAttributesParamsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the node to retrieve attributes for.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct GetAttributesParamsBuilder {
    node_id: NodeId,
}

impl GetAttributesParamsBuilder {
    pub fn build(self) -> GetAttributesParams {
        GetAttributesParams {
            node_id: self.node_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `attributes`: An interleaved array of node attribute names and values.
    pub fn builder(attributes: Vec<Cow<'a, str>>) -> GetAttributesReturnsBuilder<'a> {
        GetAttributesReturnsBuilder {
            attributes: attributes,
        }
    }
    /// An interleaved array of node attribute names and values.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetBoxModelParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetBoxModelParamsBuilder<'a> {
        GetBoxModelParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_id: None,
        }
    }
    /// Identifier of the node.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node.
    pub fn backend_node_id(&self) -> Option<&BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
}

#[derive(Default)]
pub struct GetBoxModelParamsBuilder<'a> {
    node_id: Option<NodeId>,
    backend_node_id: Option<BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetBoxModelParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node.
    pub fn backend_node_id(mut self, backend_node_id: BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    pub fn build(self) -> GetBoxModelParams<'a> {
        GetBoxModelParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `model`: Box model for the node.
    pub fn builder(model: BoxModel) -> GetBoxModelReturnsBuilder {
        GetBoxModelReturnsBuilder {
            model: model,
        }
    }
    /// Box model for the node.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetContentQuadsParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetContentQuadsParamsBuilder<'a> {
        GetContentQuadsParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_id: None,
        }
    }
    /// Identifier of the node.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node.
    pub fn backend_node_id(&self) -> Option<&BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
}

#[derive(Default)]
pub struct GetContentQuadsParamsBuilder<'a> {
    node_id: Option<NodeId>,
    backend_node_id: Option<BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetContentQuadsParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node.
    pub fn backend_node_id(mut self, backend_node_id: BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    pub fn build(self) -> GetContentQuadsParams<'a> {
        GetContentQuadsParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `quads`: Quads that describe node layout relative to viewport.
    pub fn builder(quads: Vec<Quad>) -> GetContentQuadsReturnsBuilder {
        GetContentQuadsReturnsBuilder {
            quads: quads,
        }
    }
    /// Quads that describe node layout relative to viewport.
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
    /// Creates a builder for this type.
    pub fn builder() -> GetDocumentParamsBuilder {
        GetDocumentParamsBuilder {
            depth: None,
            pierce: None,
        }
    }
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(&self) -> Option<i64> { self.depth }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
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
    /// Creates a builder for this type with the required parameters:
    /// * `root`: Resulting node.
    pub fn builder(root: Node<'a>) -> GetDocumentReturnsBuilder<'a> {
        GetDocumentReturnsBuilder {
            root: root,
        }
    }
    /// Resulting node.
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
    /// Creates a builder for this type.
    pub fn builder() -> GetFlattenedDocumentParamsBuilder {
        GetFlattenedDocumentParamsBuilder {
            depth: None,
            pierce: None,
        }
    }
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(&self) -> Option<i64> { self.depth }
    /// Whether or not iframes and shadow roots should be traversed when returning the subtree
    /// (default is false).
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
    /// Creates a builder for this type with the required parameters:
    /// * `nodes`: Resulting node.
    pub fn builder(nodes: Vec<Node<'a>>) -> GetFlattenedDocumentReturnsBuilder<'a> {
        GetFlattenedDocumentReturnsBuilder {
            nodes: nodes,
        }
    }
    /// Resulting node.
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// The style to filter nodes by (includes nodes if any of properties matches).
    #[serde(rename = "computedStyles")]
    computed_styles: Vec<CSSComputedStyleProperty<'a>>,
    /// Whether or not iframes and shadow roots in the same target should be traversed when returning the
    /// results (default is false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pierce: Option<bool>,
}

impl<'a> GetNodesForSubtreeByStyleParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Node ID pointing to the root of a subtree.
    /// * `computed_styles`: The style to filter nodes by (includes nodes if any of properties matches).
    pub fn builder(node_id: NodeId, computed_styles: Vec<CSSComputedStyleProperty<'a>>) -> GetNodesForSubtreeByStyleParamsBuilder<'a> {
        GetNodesForSubtreeByStyleParamsBuilder {
            node_id: node_id,
            computed_styles: computed_styles,
            pierce: None,
        }
    }
    /// Node ID pointing to the root of a subtree.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// The style to filter nodes by (includes nodes if any of properties matches).
    pub fn computed_styles(&self) -> &[CSSComputedStyleProperty<'a>] { &self.computed_styles }
    /// Whether or not iframes and shadow roots in the same target should be traversed when returning the
    /// results (default is false).
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}


pub struct GetNodesForSubtreeByStyleParamsBuilder<'a> {
    node_id: NodeId,
    computed_styles: Vec<CSSComputedStyleProperty<'a>>,
    pierce: Option<bool>,
}

impl<'a> GetNodesForSubtreeByStyleParamsBuilder<'a> {
    /// Whether or not iframes and shadow roots in the same target should be traversed when returning the
    /// results (default is false).
    pub fn pierce(mut self, pierce: bool) -> Self { self.pierce = Some(pierce); self }
    pub fn build(self) -> GetNodesForSubtreeByStyleParams<'a> {
        GetNodesForSubtreeByStyleParams {
            node_id: self.node_id,
            computed_styles: self.computed_styles,
            pierce: self.pierce,
        }
    }
}

/// Finds nodes with a given computed style in a subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodesForSubtreeByStyleReturns {
    /// Resulting nodes.
    #[serde(rename = "nodeIds")]
    node_ids: Vec<NodeId>,
}

impl GetNodesForSubtreeByStyleReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_ids`: Resulting nodes.
    pub fn builder(node_ids: Vec<NodeId>) -> GetNodesForSubtreeByStyleReturnsBuilder {
        GetNodesForSubtreeByStyleReturnsBuilder {
            node_ids: node_ids,
        }
    }
    /// Resulting nodes.
    pub fn node_ids(&self) -> &[NodeId] { &self.node_ids }
}


pub struct GetNodesForSubtreeByStyleReturnsBuilder {
    node_ids: Vec<NodeId>,
}

impl GetNodesForSubtreeByStyleReturnsBuilder {
    pub fn build(self) -> GetNodesForSubtreeByStyleReturns {
        GetNodesForSubtreeByStyleReturns {
            node_ids: self.node_ids,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeUserAgentShadowDOM")]
    include_user_agent_shadow_dom: Option<bool>,
    /// Whether to ignore pointer-events: none on elements and hit test them.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ignorePointerEventsNone")]
    ignore_pointer_events_none: Option<bool>,
}

impl GetNodeForLocationParams {
    /// Creates a builder for this type with the required parameters:
    /// * `x`: X coordinate.
    /// * `y`: Y coordinate.
    pub fn builder(x: i32, y: i32) -> GetNodeForLocationParamsBuilder {
        GetNodeForLocationParamsBuilder {
            x: x,
            y: y,
            include_user_agent_shadow_dom: None,
            ignore_pointer_events_none: None,
        }
    }
    /// X coordinate.
    pub fn x(&self) -> i32 { self.x }
    /// Y coordinate.
    pub fn y(&self) -> i32 { self.y }
    /// False to skip to the nearest non-UA shadow root ancestor (default: false).
    pub fn include_user_agent_shadow_dom(&self) -> Option<bool> { self.include_user_agent_shadow_dom }
    /// Whether to ignore pointer-events: none on elements and hit test them.
    pub fn ignore_pointer_events_none(&self) -> Option<bool> { self.ignore_pointer_events_none }
}


pub struct GetNodeForLocationParamsBuilder {
    x: i32,
    y: i32,
    include_user_agent_shadow_dom: Option<bool>,
    ignore_pointer_events_none: Option<bool>,
}

impl GetNodeForLocationParamsBuilder {
    /// False to skip to the nearest non-UA shadow root ancestor (default: false).
    pub fn include_user_agent_shadow_dom(mut self, include_user_agent_shadow_dom: bool) -> Self { self.include_user_agent_shadow_dom = Some(include_user_agent_shadow_dom); self }
    /// Whether to ignore pointer-events: none on elements and hit test them.
    pub fn ignore_pointer_events_none(mut self, ignore_pointer_events_none: bool) -> Self { self.ignore_pointer_events_none = Some(ignore_pointer_events_none); self }
    pub fn build(self) -> GetNodeForLocationParams {
        GetNodeForLocationParams {
            x: self.x,
            y: self.y,
            include_user_agent_shadow_dom: self.include_user_agent_shadow_dom,
            ignore_pointer_events_none: self.ignore_pointer_events_none,
        }
    }
}

/// Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
/// either returned or not.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeForLocationReturns<'a> {
    /// Resulting node.
    #[serde(rename = "backendNodeId")]
    backend_node_id: BackendNodeId,
    /// Frame this node belongs to.
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
    /// Id of the node at given coordinates, only when enabled and requested document.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
}

impl<'a> GetNodeForLocationReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `backend_node_id`: Resulting node.
    /// * `frame_id`: Frame this node belongs to.
    pub fn builder(backend_node_id: BackendNodeId, frame_id: crate::page::FrameId<'a>) -> GetNodeForLocationReturnsBuilder<'a> {
        GetNodeForLocationReturnsBuilder {
            backend_node_id: backend_node_id,
            frame_id: frame_id,
            node_id: None,
        }
    }
    /// Resulting node.
    pub fn backend_node_id(&self) -> &BackendNodeId { &self.backend_node_id }
    /// Frame this node belongs to.
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
    /// Id of the node at given coordinates, only when enabled and requested document.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
}


pub struct GetNodeForLocationReturnsBuilder<'a> {
    backend_node_id: BackendNodeId,
    frame_id: crate::page::FrameId<'a>,
    node_id: Option<NodeId>,
}

impl<'a> GetNodeForLocationReturnsBuilder<'a> {
    /// Id of the node at given coordinates, only when enabled and requested document.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    pub fn build(self) -> GetNodeForLocationReturns<'a> {
        GetNodeForLocationReturns {
            backend_node_id: self.backend_node_id,
            frame_id: self.frame_id,
            node_id: self.node_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Include all shadow roots. Equals to false if not specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeShadowDOM")]
    include_shadow_dom: Option<bool>,
}

impl<'a> GetOuterHTMLParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetOuterHTMLParamsBuilder<'a> {
        GetOuterHTMLParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_id: None,
            include_shadow_dom: None,
        }
    }
    /// Identifier of the node.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node.
    pub fn backend_node_id(&self) -> Option<&BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
    /// Include all shadow roots. Equals to false if not specified.
    pub fn include_shadow_dom(&self) -> Option<bool> { self.include_shadow_dom }
}

#[derive(Default)]
pub struct GetOuterHTMLParamsBuilder<'a> {
    node_id: Option<NodeId>,
    backend_node_id: Option<BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    include_shadow_dom: Option<bool>,
}

impl<'a> GetOuterHTMLParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node.
    pub fn backend_node_id(mut self, backend_node_id: BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    /// Include all shadow roots. Equals to false if not specified.
    pub fn include_shadow_dom(mut self, include_shadow_dom: bool) -> Self { self.include_shadow_dom = Some(include_shadow_dom); self }
    pub fn build(self) -> GetOuterHTMLParams<'a> {
        GetOuterHTMLParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
            include_shadow_dom: self.include_shadow_dom,
        }
    }
}

/// Returns node's HTML markup.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOuterHTMLReturns<'a> {
    /// Outer HTML markup.
    #[serde(rename = "outerHTML")]
    outer_html: Cow<'a, str>,
}

impl<'a> GetOuterHTMLReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `outer_html`: Outer HTML markup.
    pub fn builder(outer_html: impl Into<Cow<'a, str>>) -> GetOuterHTMLReturnsBuilder<'a> {
        GetOuterHTMLReturnsBuilder {
            outer_html: outer_html.into(),
        }
    }
    /// Outer HTML markup.
    pub fn outer_html(&self) -> &str { self.outer_html.as_ref() }
}


pub struct GetOuterHTMLReturnsBuilder<'a> {
    outer_html: Cow<'a, str>,
}

impl<'a> GetOuterHTMLReturnsBuilder<'a> {
    pub fn build(self) -> GetOuterHTMLReturns<'a> {
        GetOuterHTMLReturns {
            outer_html: self.outer_html,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl GetRelayoutBoundaryParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node.
    pub fn builder(node_id: NodeId) -> GetRelayoutBoundaryParamsBuilder {
        GetRelayoutBoundaryParamsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the node.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct GetRelayoutBoundaryParamsBuilder {
    node_id: NodeId,
}

impl GetRelayoutBoundaryParamsBuilder {
    pub fn build(self) -> GetRelayoutBoundaryParams {
        GetRelayoutBoundaryParams {
            node_id: self.node_id,
        }
    }
}

/// Returns the id of the nearest ancestor that is a relayout boundary.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRelayoutBoundaryReturns {
    /// Relayout boundary node id for the given node.
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl GetRelayoutBoundaryReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Relayout boundary node id for the given node.
    pub fn builder(node_id: NodeId) -> GetRelayoutBoundaryReturnsBuilder {
        GetRelayoutBoundaryReturnsBuilder {
            node_id: node_id,
        }
    }
    /// Relayout boundary node id for the given node.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct GetRelayoutBoundaryReturnsBuilder {
    node_id: NodeId,
}

impl GetRelayoutBoundaryReturnsBuilder {
    pub fn build(self) -> GetRelayoutBoundaryReturns {
        GetRelayoutBoundaryReturns {
            node_id: self.node_id,
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
    #[serde(rename = "searchId")]
    search_id: Cow<'a, str>,
    /// Start index of the search result to be returned.
    #[serde(rename = "fromIndex")]
    from_index: u64,
    /// End index of the search result to be returned.
    #[serde(rename = "toIndex")]
    to_index: u64,
}

impl<'a> GetSearchResultsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `search_id`: Unique search session identifier.
    /// * `from_index`: Start index of the search result to be returned.
    /// * `to_index`: End index of the search result to be returned.
    pub fn builder(search_id: impl Into<Cow<'a, str>>, from_index: u64, to_index: u64) -> GetSearchResultsParamsBuilder<'a> {
        GetSearchResultsParamsBuilder {
            search_id: search_id.into(),
            from_index: from_index,
            to_index: to_index,
        }
    }
    /// Unique search session identifier.
    pub fn search_id(&self) -> &str { self.search_id.as_ref() }
    /// Start index of the search result to be returned.
    pub fn from_index(&self) -> u64 { self.from_index }
    /// End index of the search result to be returned.
    pub fn to_index(&self) -> u64 { self.to_index }
}


pub struct GetSearchResultsParamsBuilder<'a> {
    search_id: Cow<'a, str>,
    from_index: u64,
    to_index: u64,
}

impl<'a> GetSearchResultsParamsBuilder<'a> {
    pub fn build(self) -> GetSearchResultsParams<'a> {
        GetSearchResultsParams {
            search_id: self.search_id,
            from_index: self.from_index,
            to_index: self.to_index,
        }
    }
}

/// Returns search results from given 'fromIndex' to given 'toIndex' from the search with the given
/// identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSearchResultsReturns {
    /// Ids of the search result nodes.
    #[serde(rename = "nodeIds")]
    node_ids: Vec<NodeId>,
}

impl GetSearchResultsReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_ids`: Ids of the search result nodes.
    pub fn builder(node_ids: Vec<NodeId>) -> GetSearchResultsReturnsBuilder {
        GetSearchResultsReturnsBuilder {
            node_ids: node_ids,
        }
    }
    /// Ids of the search result nodes.
    pub fn node_ids(&self) -> &[NodeId] { &self.node_ids }
}


pub struct GetSearchResultsReturnsBuilder {
    node_ids: Vec<NodeId>,
}

impl GetSearchResultsReturnsBuilder {
    pub fn build(self) -> GetSearchResultsReturns {
        GetSearchResultsReturns {
            node_ids: self.node_ids,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// Id of the element to drop the moved node into.
    #[serde(rename = "targetNodeId")]
    target_node_id: NodeId,
    /// Drop node before this one (if absent, the moved node becomes the last child of
    /// 'targetNodeId').
    #[serde(skip_serializing_if = "Option::is_none", rename = "insertBeforeNodeId")]
    insert_before_node_id: Option<NodeId>,
}

impl MoveToParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to move.
    /// * `target_node_id`: Id of the element to drop the moved node into.
    pub fn builder(node_id: NodeId, target_node_id: NodeId) -> MoveToParamsBuilder {
        MoveToParamsBuilder {
            node_id: node_id,
            target_node_id: target_node_id,
            insert_before_node_id: None,
        }
    }
    /// Id of the node to move.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// Id of the element to drop the moved node into.
    pub fn target_node_id(&self) -> &NodeId { &self.target_node_id }
    /// Drop node before this one (if absent, the moved node becomes the last child of
    /// 'targetNodeId').
    pub fn insert_before_node_id(&self) -> Option<&NodeId> { self.insert_before_node_id.as_ref() }
}


pub struct MoveToParamsBuilder {
    node_id: NodeId,
    target_node_id: NodeId,
    insert_before_node_id: Option<NodeId>,
}

impl MoveToParamsBuilder {
    /// Drop node before this one (if absent, the moved node becomes the last child of
    /// 'targetNodeId').
    pub fn insert_before_node_id(mut self, insert_before_node_id: NodeId) -> Self { self.insert_before_node_id = Some(insert_before_node_id); self }
    pub fn build(self) -> MoveToParams {
        MoveToParams {
            node_id: self.node_id,
            target_node_id: self.target_node_id,
            insert_before_node_id: self.insert_before_node_id,
        }
    }
}

/// Moves node into the new container, places it before the given anchor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MoveToReturns {
    /// New id of the moved node.
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl MoveToReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: New id of the moved node.
    pub fn builder(node_id: NodeId) -> MoveToReturnsBuilder {
        MoveToReturnsBuilder {
            node_id: node_id,
        }
    }
    /// New id of the moved node.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct MoveToReturnsBuilder {
    node_id: NodeId,
}

impl MoveToReturnsBuilder {
    pub fn build(self) -> MoveToReturns {
        MoveToReturns {
            node_id: self.node_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeUserAgentShadowDOM")]
    include_user_agent_shadow_dom: Option<bool>,
}

impl<'a> PerformSearchParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `query`: Plain text or query selector or XPath search query.
    pub fn builder(query: impl Into<Cow<'a, str>>) -> PerformSearchParamsBuilder<'a> {
        PerformSearchParamsBuilder {
            query: query.into(),
            include_user_agent_shadow_dom: None,
        }
    }
    /// Plain text or query selector or XPath search query.
    pub fn query(&self) -> &str { self.query.as_ref() }
    /// True to search in user agent shadow DOM.
    pub fn include_user_agent_shadow_dom(&self) -> Option<bool> { self.include_user_agent_shadow_dom }
}


pub struct PerformSearchParamsBuilder<'a> {
    query: Cow<'a, str>,
    include_user_agent_shadow_dom: Option<bool>,
}

impl<'a> PerformSearchParamsBuilder<'a> {
    /// True to search in user agent shadow DOM.
    pub fn include_user_agent_shadow_dom(mut self, include_user_agent_shadow_dom: bool) -> Self { self.include_user_agent_shadow_dom = Some(include_user_agent_shadow_dom); self }
    pub fn build(self) -> PerformSearchParams<'a> {
        PerformSearchParams {
            query: self.query,
            include_user_agent_shadow_dom: self.include_user_agent_shadow_dom,
        }
    }
}

/// Searches for a given string in the DOM tree. Use 'getSearchResults' to access search results or
/// 'cancelSearch' to end this search session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerformSearchReturns<'a> {
    /// Unique search session identifier.
    #[serde(rename = "searchId")]
    search_id: Cow<'a, str>,
    /// Number of search results.
    #[serde(rename = "resultCount")]
    result_count: u64,
}

impl<'a> PerformSearchReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `search_id`: Unique search session identifier.
    /// * `result_count`: Number of search results.
    pub fn builder(search_id: impl Into<Cow<'a, str>>, result_count: u64) -> PerformSearchReturnsBuilder<'a> {
        PerformSearchReturnsBuilder {
            search_id: search_id.into(),
            result_count: result_count,
        }
    }
    /// Unique search session identifier.
    pub fn search_id(&self) -> &str { self.search_id.as_ref() }
    /// Number of search results.
    pub fn result_count(&self) -> u64 { self.result_count }
}


pub struct PerformSearchReturnsBuilder<'a> {
    search_id: Cow<'a, str>,
    result_count: u64,
}

impl<'a> PerformSearchReturnsBuilder<'a> {
    pub fn build(self) -> PerformSearchReturns<'a> {
        PerformSearchReturns {
            search_id: self.search_id,
            result_count: self.result_count,
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
    /// Creates a builder for this type with the required parameters:
    /// * `path`: Path to node in the proprietary format.
    pub fn builder(path: impl Into<Cow<'a, str>>) -> PushNodeByPathToFrontendParamsBuilder<'a> {
        PushNodeByPathToFrontendParamsBuilder {
            path: path.into(),
        }
    }
    /// Path to node in the proprietary format.
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl PushNodeByPathToFrontendReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node for given path.
    pub fn builder(node_id: NodeId) -> PushNodeByPathToFrontendReturnsBuilder {
        PushNodeByPathToFrontendReturnsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the node for given path.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct PushNodeByPathToFrontendReturnsBuilder {
    node_id: NodeId,
}

impl PushNodeByPathToFrontendReturnsBuilder {
    pub fn build(self) -> PushNodeByPathToFrontendReturns {
        PushNodeByPathToFrontendReturns {
            node_id: self.node_id,
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
    #[serde(rename = "backendNodeIds")]
    backend_node_ids: Vec<BackendNodeId>,
}

impl PushNodesByBackendIdsToFrontendParams {
    /// Creates a builder for this type with the required parameters:
    /// * `backend_node_ids`: The array of backend node ids.
    pub fn builder(backend_node_ids: Vec<BackendNodeId>) -> PushNodesByBackendIdsToFrontendParamsBuilder {
        PushNodesByBackendIdsToFrontendParamsBuilder {
            backend_node_ids: backend_node_ids,
        }
    }
    /// The array of backend node ids.
    pub fn backend_node_ids(&self) -> &[BackendNodeId] { &self.backend_node_ids }
}


pub struct PushNodesByBackendIdsToFrontendParamsBuilder {
    backend_node_ids: Vec<BackendNodeId>,
}

impl PushNodesByBackendIdsToFrontendParamsBuilder {
    pub fn build(self) -> PushNodesByBackendIdsToFrontendParams {
        PushNodesByBackendIdsToFrontendParams {
            backend_node_ids: self.backend_node_ids,
        }
    }
}

/// Requests that a batch of nodes is sent to the caller given their backend node ids.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PushNodesByBackendIdsToFrontendReturns {
    /// The array of ids of pushed nodes that correspond to the backend ids specified in
    /// backendNodeIds.
    #[serde(rename = "nodeIds")]
    node_ids: Vec<NodeId>,
}

impl PushNodesByBackendIdsToFrontendReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_ids`: The array of ids of pushed nodes that correspond to the backend ids specified in backendNodeIds.
    pub fn builder(node_ids: Vec<NodeId>) -> PushNodesByBackendIdsToFrontendReturnsBuilder {
        PushNodesByBackendIdsToFrontendReturnsBuilder {
            node_ids: node_ids,
        }
    }
    /// The array of ids of pushed nodes that correspond to the backend ids specified in
    /// backendNodeIds.
    pub fn node_ids(&self) -> &[NodeId] { &self.node_ids }
}


pub struct PushNodesByBackendIdsToFrontendReturnsBuilder {
    node_ids: Vec<NodeId>,
}

impl PushNodesByBackendIdsToFrontendReturnsBuilder {
    pub fn build(self) -> PushNodesByBackendIdsToFrontendReturns {
        PushNodesByBackendIdsToFrontendReturns {
            node_ids: self.node_ids,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// Selector string.
    selector: Cow<'a, str>,
}

impl<'a> QuerySelectorParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to query upon.
    /// * `selector`: Selector string.
    pub fn builder(node_id: NodeId, selector: impl Into<Cow<'a, str>>) -> QuerySelectorParamsBuilder<'a> {
        QuerySelectorParamsBuilder {
            node_id: node_id,
            selector: selector.into(),
        }
    }
    /// Id of the node to query upon.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// Selector string.
    pub fn selector(&self) -> &str { self.selector.as_ref() }
}


pub struct QuerySelectorParamsBuilder<'a> {
    node_id: NodeId,
    selector: Cow<'a, str>,
}

impl<'a> QuerySelectorParamsBuilder<'a> {
    pub fn build(self) -> QuerySelectorParams<'a> {
        QuerySelectorParams {
            node_id: self.node_id,
            selector: self.selector,
        }
    }
}

/// Executes 'querySelector' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorReturns {
    /// Query selector result.
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl QuerySelectorReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Query selector result.
    pub fn builder(node_id: NodeId) -> QuerySelectorReturnsBuilder {
        QuerySelectorReturnsBuilder {
            node_id: node_id,
        }
    }
    /// Query selector result.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct QuerySelectorReturnsBuilder {
    node_id: NodeId,
}

impl QuerySelectorReturnsBuilder {
    pub fn build(self) -> QuerySelectorReturns {
        QuerySelectorReturns {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// Selector string.
    selector: Cow<'a, str>,
}

impl<'a> QuerySelectorAllParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to query upon.
    /// * `selector`: Selector string.
    pub fn builder(node_id: NodeId, selector: impl Into<Cow<'a, str>>) -> QuerySelectorAllParamsBuilder<'a> {
        QuerySelectorAllParamsBuilder {
            node_id: node_id,
            selector: selector.into(),
        }
    }
    /// Id of the node to query upon.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// Selector string.
    pub fn selector(&self) -> &str { self.selector.as_ref() }
}


pub struct QuerySelectorAllParamsBuilder<'a> {
    node_id: NodeId,
    selector: Cow<'a, str>,
}

impl<'a> QuerySelectorAllParamsBuilder<'a> {
    pub fn build(self) -> QuerySelectorAllParams<'a> {
        QuerySelectorAllParams {
            node_id: self.node_id,
            selector: self.selector,
        }
    }
}

/// Executes 'querySelectorAll' on a given node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorAllReturns {
    /// Query selector result.
    #[serde(rename = "nodeIds")]
    node_ids: Vec<NodeId>,
}

impl QuerySelectorAllReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_ids`: Query selector result.
    pub fn builder(node_ids: Vec<NodeId>) -> QuerySelectorAllReturnsBuilder {
        QuerySelectorAllReturnsBuilder {
            node_ids: node_ids,
        }
    }
    /// Query selector result.
    pub fn node_ids(&self) -> &[NodeId] { &self.node_ids }
}


pub struct QuerySelectorAllReturnsBuilder {
    node_ids: Vec<NodeId>,
}

impl QuerySelectorAllReturnsBuilder {
    pub fn build(self) -> QuerySelectorAllReturns {
        QuerySelectorAllReturns {
            node_ids: self.node_ids,
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
    #[serde(rename = "nodeIds")]
    node_ids: Vec<NodeId>,
}

impl GetTopLayerElementsReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_ids`: NodeIds of top layer elements
    pub fn builder(node_ids: Vec<NodeId>) -> GetTopLayerElementsReturnsBuilder {
        GetTopLayerElementsReturnsBuilder {
            node_ids: node_ids,
        }
    }
    /// NodeIds of top layer elements
    pub fn node_ids(&self) -> &[NodeId] { &self.node_ids }
}


pub struct GetTopLayerElementsReturnsBuilder {
    node_ids: Vec<NodeId>,
}

impl GetTopLayerElementsReturnsBuilder {
    pub fn build(self) -> GetTopLayerElementsReturns {
        GetTopLayerElementsReturns {
            node_ids: self.node_ids,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// Type of relation to get.
    relation: Cow<'a, str>,
}

impl<'a> GetElementByRelationParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node from which to query the relation.
    /// * `relation`: Type of relation to get.
    pub fn builder(node_id: NodeId, relation: impl Into<Cow<'a, str>>) -> GetElementByRelationParamsBuilder<'a> {
        GetElementByRelationParamsBuilder {
            node_id: node_id,
            relation: relation.into(),
        }
    }
    /// Id of the node from which to query the relation.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// Type of relation to get.
    pub fn relation(&self) -> &str { self.relation.as_ref() }
}


pub struct GetElementByRelationParamsBuilder<'a> {
    node_id: NodeId,
    relation: Cow<'a, str>,
}

impl<'a> GetElementByRelationParamsBuilder<'a> {
    pub fn build(self) -> GetElementByRelationParams<'a> {
        GetElementByRelationParams {
            node_id: self.node_id,
            relation: self.relation,
        }
    }
}

/// Returns the NodeId of the matched element according to certain relations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetElementByRelationReturns {
    /// NodeId of the element matching the queried relation.
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl GetElementByRelationReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: NodeId of the element matching the queried relation.
    pub fn builder(node_id: NodeId) -> GetElementByRelationReturnsBuilder {
        GetElementByRelationReturnsBuilder {
            node_id: node_id,
        }
    }
    /// NodeId of the element matching the queried relation.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct GetElementByRelationReturnsBuilder {
    node_id: NodeId,
}

impl GetElementByRelationReturnsBuilder {
    pub fn build(self) -> GetElementByRelationReturns {
        GetElementByRelationReturns {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// Name of the attribute to remove.
    name: Cow<'a, str>,
}

impl<'a> RemoveAttributeParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the element to remove attribute from.
    /// * `name`: Name of the attribute to remove.
    pub fn builder(node_id: NodeId, name: impl Into<Cow<'a, str>>) -> RemoveAttributeParamsBuilder<'a> {
        RemoveAttributeParamsBuilder {
            node_id: node_id,
            name: name.into(),
        }
    }
    /// Id of the element to remove attribute from.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// Name of the attribute to remove.
    pub fn name(&self) -> &str { self.name.as_ref() }
}


pub struct RemoveAttributeParamsBuilder<'a> {
    node_id: NodeId,
    name: Cow<'a, str>,
}

impl<'a> RemoveAttributeParamsBuilder<'a> {
    pub fn build(self) -> RemoveAttributeParams<'a> {
        RemoveAttributeParams {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl RemoveNodeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to remove.
    pub fn builder(node_id: NodeId) -> RemoveNodeParamsBuilder {
        RemoveNodeParamsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the node to remove.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct RemoveNodeParamsBuilder {
    node_id: NodeId,
}

impl RemoveNodeParamsBuilder {
    pub fn build(self) -> RemoveNodeParams {
        RemoveNodeParams {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
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
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to get children for.
    pub fn builder(node_id: NodeId) -> RequestChildNodesParamsBuilder {
        RequestChildNodesParamsBuilder {
            node_id: node_id,
            depth: None,
            pierce: None,
        }
    }
    /// Id of the node to get children for.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    /// entire subtree or provide an integer larger than 0.
    pub fn depth(&self) -> Option<i64> { self.depth }
    /// Whether or not iframes and shadow roots should be traversed when returning the sub-tree
    /// (default is false).
    pub fn pierce(&self) -> Option<bool> { self.pierce }
}


pub struct RequestChildNodesParamsBuilder {
    node_id: NodeId,
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
            node_id: self.node_id,
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
    #[serde(rename = "objectId")]
    object_id: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> RequestNodeParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `object_id`: JavaScript object id to convert into node.
    pub fn builder(object_id: crate::runtime::RemoteObjectId<'a>) -> RequestNodeParamsBuilder<'a> {
        RequestNodeParamsBuilder {
            object_id: object_id,
        }
    }
    /// JavaScript object id to convert into node.
    pub fn object_id(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.object_id }
}


pub struct RequestNodeParamsBuilder<'a> {
    object_id: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> RequestNodeParamsBuilder<'a> {
    pub fn build(self) -> RequestNodeParams<'a> {
        RequestNodeParams {
            object_id: self.object_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl RequestNodeReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Node id for given object.
    pub fn builder(node_id: NodeId) -> RequestNodeReturnsBuilder {
        RequestNodeReturnsBuilder {
            node_id: node_id,
        }
    }
    /// Node id for given object.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct RequestNodeReturnsBuilder {
    node_id: NodeId,
}

impl RequestNodeReturnsBuilder {
    pub fn build(self) -> RequestNodeReturns {
        RequestNodeReturns {
            node_id: self.node_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
    /// Backend identifier of the node to resolve.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    object_group: Option<Cow<'a, str>>,
    /// Execution context in which to resolve the node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    execution_context_id: Option<crate::runtime::ExecutionContextId>,
}

impl<'a> ResolveNodeParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> ResolveNodeParamsBuilder<'a> {
        ResolveNodeParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_group: None,
            execution_context_id: None,
        }
    }
    /// Id of the node to resolve.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
    /// Backend identifier of the node to resolve.
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
    /// Symbolic group name that can be used to release multiple objects.
    pub fn object_group(&self) -> Option<&str> { self.object_group.as_deref() }
    /// Execution context in which to resolve the node.
    pub fn execution_context_id(&self) -> Option<&crate::runtime::ExecutionContextId> { self.execution_context_id.as_ref() }
}

#[derive(Default)]
pub struct ResolveNodeParamsBuilder<'a> {
    node_id: Option<NodeId>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
    object_group: Option<Cow<'a, str>>,
    execution_context_id: Option<crate::runtime::ExecutionContextId>,
}

impl<'a> ResolveNodeParamsBuilder<'a> {
    /// Id of the node to resolve.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Backend identifier of the node to resolve.
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// Symbolic group name that can be used to release multiple objects.
    pub fn object_group(mut self, object_group: impl Into<Cow<'a, str>>) -> Self { self.object_group = Some(object_group.into()); self }
    /// Execution context in which to resolve the node.
    pub fn execution_context_id(mut self, execution_context_id: crate::runtime::ExecutionContextId) -> Self { self.execution_context_id = Some(execution_context_id); self }
    pub fn build(self) -> ResolveNodeParams<'a> {
        ResolveNodeParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_group: self.object_group,
            execution_context_id: self.execution_context_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `object`: JavaScript object wrapper for given node.
    pub fn builder(object: crate::runtime::RemoteObject) -> ResolveNodeReturnsBuilder {
        ResolveNodeReturnsBuilder {
            object: object,
        }
    }
    /// JavaScript object wrapper for given node.
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// Attribute name.
    name: Cow<'a, str>,
    /// Attribute value.
    value: Cow<'a, str>,
}

impl<'a> SetAttributeValueParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the element to set attribute for.
    /// * `name`: Attribute name.
    /// * `value`: Attribute value.
    pub fn builder(node_id: NodeId, name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> SetAttributeValueParamsBuilder<'a> {
        SetAttributeValueParamsBuilder {
            node_id: node_id,
            name: name.into(),
            value: value.into(),
        }
    }
    /// Id of the element to set attribute for.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// Attribute name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Attribute value.
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct SetAttributeValueParamsBuilder<'a> {
    node_id: NodeId,
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SetAttributeValueParamsBuilder<'a> {
    pub fn build(self) -> SetAttributeValueParams<'a> {
        SetAttributeValueParams {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// Text with a number of attributes. Will parse this text using HTML parser.
    text: Cow<'a, str>,
    /// Attribute name to replace with new attributes derived from text in case text parsed
    /// successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
}

impl<'a> SetAttributesAsTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the element to set attributes for.
    /// * `text`: Text with a number of attributes. Will parse this text using HTML parser.
    pub fn builder(node_id: NodeId, text: impl Into<Cow<'a, str>>) -> SetAttributesAsTextParamsBuilder<'a> {
        SetAttributesAsTextParamsBuilder {
            node_id: node_id,
            text: text.into(),
            name: None,
        }
    }
    /// Id of the element to set attributes for.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// Text with a number of attributes. Will parse this text using HTML parser.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// Attribute name to replace with new attributes derived from text in case text parsed
    /// successfully.
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
}


pub struct SetAttributesAsTextParamsBuilder<'a> {
    node_id: NodeId,
    text: Cow<'a, str>,
    name: Option<Cow<'a, str>>,
}

impl<'a> SetAttributesAsTextParamsBuilder<'a> {
    /// Attribute name to replace with new attributes derived from text in case text parsed
    /// successfully.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> SetAttributesAsTextParams<'a> {
        SetAttributesAsTextParams {
            node_id: self.node_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> SetFileInputFilesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `files`: Array of file paths to set.
    pub fn builder(files: Vec<Cow<'a, str>>) -> SetFileInputFilesParamsBuilder<'a> {
        SetFileInputFilesParamsBuilder {
            files: files,
            node_id: None,
            backend_node_id: None,
            object_id: None,
        }
    }
    /// Array of file paths to set.
    pub fn files(&self) -> &[Cow<'a, str>] { &self.files }
    /// Identifier of the node.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node.
    pub fn backend_node_id(&self) -> Option<&BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
}


pub struct SetFileInputFilesParamsBuilder<'a> {
    files: Vec<Cow<'a, str>>,
    node_id: Option<NodeId>,
    backend_node_id: Option<BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> SetFileInputFilesParamsBuilder<'a> {
    /// Identifier of the node.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node.
    pub fn backend_node_id(mut self, backend_node_id: BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    pub fn build(self) -> SetFileInputFilesParams<'a> {
        SetFileInputFilesParams {
            files: self.files,
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `enable`: Enable or disable.
    pub fn builder(enable: bool) -> SetNodeStackTracesEnabledParamsBuilder {
        SetNodeStackTracesEnabledParamsBuilder {
            enable: enable,
        }
    }
    /// Enable or disable.
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl GetNodeStackTracesParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to get stack traces for.
    pub fn builder(node_id: NodeId) -> GetNodeStackTracesParamsBuilder {
        GetNodeStackTracesParamsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the node to get stack traces for.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct GetNodeStackTracesParamsBuilder {
    node_id: NodeId,
}

impl GetNodeStackTracesParamsBuilder {
    pub fn build(self) -> GetNodeStackTracesParams {
        GetNodeStackTracesParams {
            node_id: self.node_id,
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
    /// Creates a builder for this type.
    pub fn builder() -> GetNodeStackTracesReturnsBuilder {
        GetNodeStackTracesReturnsBuilder {
            creation: None,
        }
    }
    /// Creation stack trace, if available.
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
    #[serde(rename = "objectId")]
    object_id: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> GetFileInfoParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `object_id`: JavaScript object id of the node wrapper.
    pub fn builder(object_id: crate::runtime::RemoteObjectId<'a>) -> GetFileInfoParamsBuilder<'a> {
        GetFileInfoParamsBuilder {
            object_id: object_id,
        }
    }
    /// JavaScript object id of the node wrapper.
    pub fn object_id(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.object_id }
}


pub struct GetFileInfoParamsBuilder<'a> {
    object_id: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> GetFileInfoParamsBuilder<'a> {
    pub fn build(self) -> GetFileInfoParams<'a> {
        GetFileInfoParams {
            object_id: self.object_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `path`: 
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
    #[serde(rename = "detachedNodes")]
    detached_nodes: Vec<DetachedElementInfo<'a>>,
}

impl<'a> GetDetachedDomNodesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `detached_nodes`: The list of detached nodes
    pub fn builder(detached_nodes: Vec<DetachedElementInfo<'a>>) -> GetDetachedDomNodesReturnsBuilder<'a> {
        GetDetachedDomNodesReturnsBuilder {
            detached_nodes: detached_nodes,
        }
    }
    /// The list of detached nodes
    pub fn detached_nodes(&self) -> &[DetachedElementInfo<'a>] { &self.detached_nodes }
}


pub struct GetDetachedDomNodesReturnsBuilder<'a> {
    detached_nodes: Vec<DetachedElementInfo<'a>>,
}

impl<'a> GetDetachedDomNodesReturnsBuilder<'a> {
    pub fn build(self) -> GetDetachedDomNodesReturns<'a> {
        GetDetachedDomNodesReturns {
            detached_nodes: self.detached_nodes,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl SetInspectedNodeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: DOM node id to be accessible by means of $x command line API.
    pub fn builder(node_id: NodeId) -> SetInspectedNodeParamsBuilder {
        SetInspectedNodeParamsBuilder {
            node_id: node_id,
        }
    }
    /// DOM node id to be accessible by means of $x command line API.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct SetInspectedNodeParamsBuilder {
    node_id: NodeId,
}

impl SetInspectedNodeParamsBuilder {
    pub fn build(self) -> SetInspectedNodeParams {
        SetInspectedNodeParams {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// New node's name.
    name: Cow<'a, str>,
}

impl<'a> SetNodeNameParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to set name for.
    /// * `name`: New node's name.
    pub fn builder(node_id: NodeId, name: impl Into<Cow<'a, str>>) -> SetNodeNameParamsBuilder<'a> {
        SetNodeNameParamsBuilder {
            node_id: node_id,
            name: name.into(),
        }
    }
    /// Id of the node to set name for.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// New node's name.
    pub fn name(&self) -> &str { self.name.as_ref() }
}


pub struct SetNodeNameParamsBuilder<'a> {
    node_id: NodeId,
    name: Cow<'a, str>,
}

impl<'a> SetNodeNameParamsBuilder<'a> {
    pub fn build(self) -> SetNodeNameParams<'a> {
        SetNodeNameParams {
            node_id: self.node_id,
            name: self.name,
        }
    }
}

/// Sets node name for a node with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNodeNameReturns {
    /// New node's id.
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl SetNodeNameReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: New node's id.
    pub fn builder(node_id: NodeId) -> SetNodeNameReturnsBuilder {
        SetNodeNameReturnsBuilder {
            node_id: node_id,
        }
    }
    /// New node's id.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct SetNodeNameReturnsBuilder {
    node_id: NodeId,
}

impl SetNodeNameReturnsBuilder {
    pub fn build(self) -> SetNodeNameReturns {
        SetNodeNameReturns {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// New node's value.
    value: Cow<'a, str>,
}

impl<'a> SetNodeValueParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to set value for.
    /// * `value`: New node's value.
    pub fn builder(node_id: NodeId, value: impl Into<Cow<'a, str>>) -> SetNodeValueParamsBuilder<'a> {
        SetNodeValueParamsBuilder {
            node_id: node_id,
            value: value.into(),
        }
    }
    /// Id of the node to set value for.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// New node's value.
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct SetNodeValueParamsBuilder<'a> {
    node_id: NodeId,
    value: Cow<'a, str>,
}

impl<'a> SetNodeValueParamsBuilder<'a> {
    pub fn build(self) -> SetNodeValueParams<'a> {
        SetNodeValueParams {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// Outer HTML markup to set.
    #[serde(rename = "outerHTML")]
    outer_html: Cow<'a, str>,
}

impl<'a> SetOuterHTMLParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to set markup for.
    /// * `outer_html`: Outer HTML markup to set.
    pub fn builder(node_id: NodeId, outer_html: impl Into<Cow<'a, str>>) -> SetOuterHTMLParamsBuilder<'a> {
        SetOuterHTMLParamsBuilder {
            node_id: node_id,
            outer_html: outer_html.into(),
        }
    }
    /// Id of the node to set markup for.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// Outer HTML markup to set.
    pub fn outer_html(&self) -> &str { self.outer_html.as_ref() }
}


pub struct SetOuterHTMLParamsBuilder<'a> {
    node_id: NodeId,
    outer_html: Cow<'a, str>,
}

impl<'a> SetOuterHTMLParamsBuilder<'a> {
    pub fn build(self) -> SetOuterHTMLParams<'a> {
        SetOuterHTMLParams {
            node_id: self.node_id,
            outer_html: self.outer_html,
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
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
}

impl<'a> GetFrameOwnerParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: 
    pub fn builder(frame_id: crate::page::FrameId<'a>) -> GetFrameOwnerParamsBuilder<'a> {
        GetFrameOwnerParamsBuilder {
            frame_id: frame_id,
        }
    }
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
}


pub struct GetFrameOwnerParamsBuilder<'a> {
    frame_id: crate::page::FrameId<'a>,
}

impl<'a> GetFrameOwnerParamsBuilder<'a> {
    pub fn build(self) -> GetFrameOwnerParams<'a> {
        GetFrameOwnerParams {
            frame_id: self.frame_id,
        }
    }
}

/// Returns iframe node that owns iframe with the given domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameOwnerReturns {
    /// Resulting node.
    #[serde(rename = "backendNodeId")]
    backend_node_id: BackendNodeId,
    /// Id of the node at given coordinates, only when enabled and requested document.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
}

impl GetFrameOwnerReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `backend_node_id`: Resulting node.
    pub fn builder(backend_node_id: BackendNodeId) -> GetFrameOwnerReturnsBuilder {
        GetFrameOwnerReturnsBuilder {
            backend_node_id: backend_node_id,
            node_id: None,
        }
    }
    /// Resulting node.
    pub fn backend_node_id(&self) -> &BackendNodeId { &self.backend_node_id }
    /// Id of the node at given coordinates, only when enabled and requested document.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
}


pub struct GetFrameOwnerReturnsBuilder {
    backend_node_id: BackendNodeId,
    node_id: Option<NodeId>,
}

impl GetFrameOwnerReturnsBuilder {
    /// Id of the node at given coordinates, only when enabled and requested document.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    pub fn build(self) -> GetFrameOwnerReturns {
        GetFrameOwnerReturns {
            backend_node_id: self.backend_node_id,
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerName")]
    container_name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "physicalAxes")]
    physical_axes: Option<PhysicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "logicalAxes")]
    logical_axes: Option<LogicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "queriesScrollState")]
    queries_scroll_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "queriesAnchored")]
    queries_anchored: Option<bool>,
}

impl<'a> GetContainerForNodeParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: 
    pub fn builder(node_id: NodeId) -> GetContainerForNodeParamsBuilder<'a> {
        GetContainerForNodeParamsBuilder {
            node_id: node_id,
            container_name: None,
            physical_axes: None,
            logical_axes: None,
            queries_scroll_state: None,
            queries_anchored: None,
        }
    }
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    pub fn container_name(&self) -> Option<&str> { self.container_name.as_deref() }
    pub fn physical_axes(&self) -> Option<&PhysicalAxes> { self.physical_axes.as_ref() }
    pub fn logical_axes(&self) -> Option<&LogicalAxes> { self.logical_axes.as_ref() }
    pub fn queries_scroll_state(&self) -> Option<bool> { self.queries_scroll_state }
    pub fn queries_anchored(&self) -> Option<bool> { self.queries_anchored }
}


pub struct GetContainerForNodeParamsBuilder<'a> {
    node_id: NodeId,
    container_name: Option<Cow<'a, str>>,
    physical_axes: Option<PhysicalAxes>,
    logical_axes: Option<LogicalAxes>,
    queries_scroll_state: Option<bool>,
    queries_anchored: Option<bool>,
}

impl<'a> GetContainerForNodeParamsBuilder<'a> {
    pub fn container_name(mut self, container_name: impl Into<Cow<'a, str>>) -> Self { self.container_name = Some(container_name.into()); self }
    pub fn physical_axes(mut self, physical_axes: impl Into<PhysicalAxes>) -> Self { self.physical_axes = Some(physical_axes.into()); self }
    pub fn logical_axes(mut self, logical_axes: impl Into<LogicalAxes>) -> Self { self.logical_axes = Some(logical_axes.into()); self }
    pub fn queries_scroll_state(mut self, queries_scroll_state: bool) -> Self { self.queries_scroll_state = Some(queries_scroll_state); self }
    pub fn queries_anchored(mut self, queries_anchored: bool) -> Self { self.queries_anchored = Some(queries_anchored); self }
    pub fn build(self) -> GetContainerForNodeParams<'a> {
        GetContainerForNodeParams {
            node_id: self.node_id,
            container_name: self.container_name,
            physical_axes: self.physical_axes,
            logical_axes: self.logical_axes,
            queries_scroll_state: self.queries_scroll_state,
            queries_anchored: self.queries_anchored,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<NodeId>,
}

impl GetContainerForNodeReturns {
    /// Creates a builder for this type.
    pub fn builder() -> GetContainerForNodeReturnsBuilder {
        GetContainerForNodeReturnsBuilder {
            node_id: None,
        }
    }
    /// The container node for the given node, or null if not found.
    pub fn node_id(&self) -> Option<&NodeId> { self.node_id.as_ref() }
}

#[derive(Default)]
pub struct GetContainerForNodeReturnsBuilder {
    node_id: Option<NodeId>,
}

impl GetContainerForNodeReturnsBuilder {
    /// The container node for the given node, or null if not found.
    pub fn node_id(mut self, node_id: NodeId) -> Self { self.node_id = Some(node_id); self }
    pub fn build(self) -> GetContainerForNodeReturns {
        GetContainerForNodeReturns {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl GetQueryingDescendantsForContainerParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the container node to find querying descendants from.
    pub fn builder(node_id: NodeId) -> GetQueryingDescendantsForContainerParamsBuilder {
        GetQueryingDescendantsForContainerParamsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the container node to find querying descendants from.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct GetQueryingDescendantsForContainerParamsBuilder {
    node_id: NodeId,
}

impl GetQueryingDescendantsForContainerParamsBuilder {
    pub fn build(self) -> GetQueryingDescendantsForContainerParams {
        GetQueryingDescendantsForContainerParams {
            node_id: self.node_id,
        }
    }
}

/// Returns the descendants of a container query container that have
/// container queries against this container.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryingDescendantsForContainerReturns {
    /// Descendant nodes with container queries against the given container.
    #[serde(rename = "nodeIds")]
    node_ids: Vec<NodeId>,
}

impl GetQueryingDescendantsForContainerReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_ids`: Descendant nodes with container queries against the given container.
    pub fn builder(node_ids: Vec<NodeId>) -> GetQueryingDescendantsForContainerReturnsBuilder {
        GetQueryingDescendantsForContainerReturnsBuilder {
            node_ids: node_ids,
        }
    }
    /// Descendant nodes with container queries against the given container.
    pub fn node_ids(&self) -> &[NodeId] { &self.node_ids }
}


pub struct GetQueryingDescendantsForContainerReturnsBuilder {
    node_ids: Vec<NodeId>,
}

impl GetQueryingDescendantsForContainerReturnsBuilder {
    pub fn build(self) -> GetQueryingDescendantsForContainerReturns {
        GetQueryingDescendantsForContainerReturns {
            node_ids: self.node_ids,
        }
    }
}

impl GetQueryingDescendantsForContainerParams { pub const METHOD: &'static str = "DOM.getQueryingDescendantsForContainer"; }

impl<'a> crate::CdpCommand<'a> for GetQueryingDescendantsForContainerParams {
    const METHOD: &'static str = "DOM.getQueryingDescendantsForContainer";
    type Response = GetQueryingDescendantsForContainerReturns;
}

/// Returns the target anchor element of the given anchor query according to
/// <https://www.w3.org/TR/css-anchor-position-1/#target>.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnchorElementParams<'a> {
    /// Id of the positioned element from which to find the anchor.
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// An optional anchor specifier, as defined in
    /// <https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier>.
    /// If not provided, it will return the implicit anchor element for
    /// the given positioned element.
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorSpecifier")]
    anchor_specifier: Option<Cow<'a, str>>,
}

impl<'a> GetAnchorElementParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the positioned element from which to find the anchor.
    pub fn builder(node_id: NodeId) -> GetAnchorElementParamsBuilder<'a> {
        GetAnchorElementParamsBuilder {
            node_id: node_id,
            anchor_specifier: None,
        }
    }
    /// Id of the positioned element from which to find the anchor.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// An optional anchor specifier, as defined in
    /// <https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier>.
    /// If not provided, it will return the implicit anchor element for
    /// the given positioned element.
    pub fn anchor_specifier(&self) -> Option<&str> { self.anchor_specifier.as_deref() }
}


pub struct GetAnchorElementParamsBuilder<'a> {
    node_id: NodeId,
    anchor_specifier: Option<Cow<'a, str>>,
}

impl<'a> GetAnchorElementParamsBuilder<'a> {
    /// An optional anchor specifier, as defined in
    /// <https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier>.
    /// If not provided, it will return the implicit anchor element for
    /// the given positioned element.
    pub fn anchor_specifier(mut self, anchor_specifier: impl Into<Cow<'a, str>>) -> Self { self.anchor_specifier = Some(anchor_specifier.into()); self }
    pub fn build(self) -> GetAnchorElementParams<'a> {
        GetAnchorElementParams {
            node_id: self.node_id,
            anchor_specifier: self.anchor_specifier,
        }
    }
}

/// Returns the target anchor element of the given anchor query according to
/// <https://www.w3.org/TR/css-anchor-position-1/#target>.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnchorElementReturns {
    /// The anchor element of the given anchor query.
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

impl GetAnchorElementReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: The anchor element of the given anchor query.
    pub fn builder(node_id: NodeId) -> GetAnchorElementReturnsBuilder {
        GetAnchorElementReturnsBuilder {
            node_id: node_id,
        }
    }
    /// The anchor element of the given anchor query.
    pub fn node_id(&self) -> &NodeId { &self.node_id }
}


pub struct GetAnchorElementReturnsBuilder {
    node_id: NodeId,
}

impl GetAnchorElementReturnsBuilder {
    pub fn build(self) -> GetAnchorElementReturns {
        GetAnchorElementReturns {
            node_id: self.node_id,
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
    #[serde(rename = "nodeId")]
    node_id: NodeId,
    /// If true, opens the popover and keeps it open. If false, closes the
    /// popover if it was previously force-opened.
    enable: bool,
}

impl ForceShowPopoverParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the popover HTMLElement
    /// * `enable`: If true, opens the popover and keeps it open. If false, closes the popover if it was previously force-opened.
    pub fn builder(node_id: NodeId, enable: bool) -> ForceShowPopoverParamsBuilder {
        ForceShowPopoverParamsBuilder {
            node_id: node_id,
            enable: enable,
        }
    }
    /// Id of the popover HTMLElement
    pub fn node_id(&self) -> &NodeId { &self.node_id }
    /// If true, opens the popover and keeps it open. If false, closes the
    /// popover if it was previously force-opened.
    pub fn enable(&self) -> bool { self.enable }
}


pub struct ForceShowPopoverParamsBuilder {
    node_id: NodeId,
    enable: bool,
}

impl ForceShowPopoverParamsBuilder {
    pub fn build(self) -> ForceShowPopoverParams {
        ForceShowPopoverParams {
            node_id: self.node_id,
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
    #[serde(rename = "nodeIds")]
    node_ids: Vec<NodeId>,
}

impl ForceShowPopoverReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_ids`: List of popovers that were closed in order to respect popover stacking order.
    pub fn builder(node_ids: Vec<NodeId>) -> ForceShowPopoverReturnsBuilder {
        ForceShowPopoverReturnsBuilder {
            node_ids: node_ids,
        }
    }
    /// List of popovers that were closed in order to respect popover stacking order.
    pub fn node_ids(&self) -> &[NodeId] { &self.node_ids }
}


pub struct ForceShowPopoverReturnsBuilder {
    node_ids: Vec<NodeId>,
}

impl ForceShowPopoverReturnsBuilder {
    pub fn build(self) -> ForceShowPopoverReturns {
        ForceShowPopoverReturns {
            node_ids: self.node_ids,
        }
    }
}

impl ForceShowPopoverParams { pub const METHOD: &'static str = "DOM.forceShowPopover"; }

impl<'a> crate::CdpCommand<'a> for ForceShowPopoverParams {
    const METHOD: &'static str = "DOM.forceShowPopover";
    type Response = ForceShowPopoverReturns;
}
