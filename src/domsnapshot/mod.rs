//! This domain facilitates obtaining document snapshots with DOM, layout, and style information.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// A Node in the DOM tree.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DOMNode<'a> {
    /// 'Node''s nodeType.
    #[serde(rename = "nodeType")]
    pub node_type: i64,
    /// 'Node''s nodeName.
    #[serde(rename = "nodeName")]
    pub node_name: Cow<'a, str>,
    /// 'Node''s nodeValue.
    #[serde(rename = "nodeValue")]
    pub node_value: Cow<'a, str>,
    /// Only set for textarea elements, contains the text value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "textValue")]
    pub text_value: Option<Cow<'a, str>>,
    /// Only set for input elements, contains the input's associated text value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputValue")]
    pub input_value: Option<Cow<'a, str>>,
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputChecked")]
    pub input_checked: Option<bool>,
    /// Only set for option elements, indicates if the element has been selected
    #[serde(skip_serializing_if = "Option::is_none", rename = "optionSelected")]
    pub option_selected: Option<bool>,
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: crate::dom::BackendNodeId,
    /// The indexes of the node's child nodes in the 'domNodes' array returned by 'getSnapshot', if
    /// any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childNodeIndexes")]
    pub child_node_indexes: Option<Vec<i64>>,
    /// Attributes of an 'Element' node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<NameValue<'a>>>,
    /// Indexes of pseudo elements associated with this node in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoElementIndexes")]
    pub pseudo_element_indexes: Option<Vec<i64>>,
    /// The index of the node's related layout tree node in the 'layoutTreeNodes' array returned by
    /// 'getSnapshot', if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "layoutNodeIndex")]
    pub layout_node_index: Option<u64>,
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    #[serde(skip_serializing_if = "Option::is_none", rename = "documentURL")]
    pub document_url: Option<Cow<'a, str>>,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseURL")]
    pub base_url: Option<Cow<'a, str>>,
    /// Only set for documents, contains the document's content language.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentLanguage")]
    pub content_language: Option<Cow<'a, str>>,
    /// Only set for documents, contains the document's character set encoding.
    #[serde(skip_serializing_if = "Option::is_none", rename = "documentEncoding")]
    pub document_encoding: Option<Cow<'a, str>>,
    /// 'DocumentType' node's publicId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "publicId")]
    pub public_id: Option<Cow<'a, str>>,
    /// 'DocumentType' node's systemId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "systemId")]
    pub system_id: Option<Cow<'a, str>>,
    /// Frame ID for frame owner elements and also for the document node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
    /// The index of a frame owner element's content document in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentDocumentIndex")]
    pub content_document_index: Option<u64>,
    /// Type of a pseudo element node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoType")]
    pub pseudo_type: Option<crate::dom::PseudoType>,
    /// Shadow root type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "shadowRootType")]
    pub shadow_root_type: Option<crate::dom::ShadowRootType>,
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isClickable")]
    pub is_clickable: Option<bool>,
    /// Details of the node's event listeners, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "eventListeners")]
    pub event_listeners: Option<Vec<crate::domdebugger::EventListener<'a>>>,
    /// The selected url for nodes with a srcset attribute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentSourceURL")]
    pub current_source_url: Option<Cow<'a, str>>,
    /// The url of the script (if any) that generates this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originURL")]
    pub origin_url: Option<Cow<'a, str>>,
    /// Scroll offsets, set when this node is a Document.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollOffsetX")]
    pub scroll_offset_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollOffsetY")]
    pub scroll_offset_y: Option<f64>,
}
/// Details of post layout rendered text positions. The exact layout should not be regarded as
/// stable and may change between versions.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InlineTextBox {
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    #[serde(rename = "boundingBox")]
    pub bounding_box: crate::dom::Rect,
    /// The starting index in characters, for this post layout textbox substring. Characters that
    /// would be represented as a surrogate pair in UTF-16 have length 2.
    #[serde(rename = "startCharacterIndex")]
    pub start_character_index: u64,
    /// The number of characters in this post layout textbox substring. Characters that would be
    /// represented as a surrogate pair in UTF-16 have length 2.
    #[serde(rename = "numCharacters")]
    pub num_characters: i64,
}
/// Details of an element in the DOM tree with a LayoutObject.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LayoutTreeNode<'a> {
    /// The index of the related DOM node in the 'domNodes' array returned by 'getSnapshot'.
    #[serde(rename = "domNodeIndex")]
    pub dom_node_index: u64,
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    #[serde(rename = "boundingBox")]
    pub bounding_box: crate::dom::Rect,
    /// Contents of the LayoutText, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "layoutText")]
    pub layout_text: Option<Cow<'a, str>>,
    /// The post-layout inline text nodes, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inlineTextNodes")]
    pub inline_text_nodes: Option<Vec<InlineTextBox>>,
    /// Index into the 'computedStyles' array returned by 'getSnapshot'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleIndex")]
    pub style_index: Option<u64>,
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// getSnapshot was true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "paintOrder")]
    pub paint_order: Option<i64>,
    /// Set to true to indicate the element begins a new stacking context.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isStackingContext")]
    pub is_stacking_context: Option<bool>,
}
/// A subset of the full ComputedStyle as defined by the request whitelist.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ComputedStyle<'a> {
    /// Name/value pairs of computed style properties.
    pub properties: Vec<NameValue<'a>>,
}
/// A name/value pair.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NameValue<'a> {
    /// Attribute/property name.
    pub name: Cow<'a, str>,
    /// Attribute/property value.
    pub value: Cow<'a, str>,
}
/// Index of the string in the strings table.

pub type StringIndex = i64;

/// Index of the string in the strings table.

pub type ArrayOfStrings = Vec<StringIndex>;

/// Data that is only present on rare nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RareStringData {
    pub index: Vec<i64>,
    pub value: Vec<StringIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RareBooleanData {
    pub index: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RareIntegerData {
    pub index: Vec<i64>,
    pub value: Vec<i64>,
}

pub type Rectangle = Vec<f64>;

/// Document snapshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSnapshot {
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    #[serde(rename = "documentURL")]
    pub document_url: StringIndex,
    /// Document title.
    pub title: StringIndex,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    #[serde(rename = "baseURL")]
    pub base_url: StringIndex,
    /// Contains the document's content language.
    #[serde(rename = "contentLanguage")]
    pub content_language: StringIndex,
    /// Contains the document's character set encoding.
    #[serde(rename = "encodingName")]
    pub encoding_name: StringIndex,
    /// 'DocumentType' node's publicId.
    #[serde(rename = "publicId")]
    pub public_id: StringIndex,
    /// 'DocumentType' node's systemId.
    #[serde(rename = "systemId")]
    pub system_id: StringIndex,
    /// Frame ID for frame owner elements and also for the document node.
    #[serde(rename = "frameId")]
    pub frame_id: StringIndex,
    /// A table with dom nodes.
    pub nodes: NodeTreeSnapshot,
    /// The nodes in the layout tree.
    pub layout: LayoutTreeSnapshot,
    /// The post-layout inline text nodes.
    #[serde(rename = "textBoxes")]
    pub text_boxes: TextBoxSnapshot,
    /// Horizontal scroll offset.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollOffsetX")]
    pub scroll_offset_x: Option<f64>,
    /// Vertical scroll offset.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollOffsetY")]
    pub scroll_offset_y: Option<f64>,
    /// Document content width.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentWidth")]
    pub content_width: Option<f64>,
    /// Document content height.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentHeight")]
    pub content_height: Option<f64>,
}
/// Table containing nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NodeTreeSnapshot {
    /// Parent node index.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentIndex")]
    pub parent_index: Option<Vec<i64>>,
    /// 'Node''s nodeType.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeType")]
    pub node_type: Option<Vec<i64>>,
    /// Type of the shadow root the 'Node' is in. String values are equal to the 'ShadowRootType' enum.
    #[serde(skip_serializing_if = "Option::is_none", rename = "shadowRootType")]
    pub shadow_root_type: Option<RareStringData>,
    /// 'Node''s nodeName.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeName")]
    pub node_name: Option<Vec<StringIndex>>,
    /// 'Node''s nodeValue.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeValue")]
    pub node_value: Option<Vec<StringIndex>>,
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<Vec<crate::dom::BackendNodeId>>,
    /// Attributes of an 'Element' node. Flatten name, value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ArrayOfStrings>>,
    /// Only set for textarea elements, contains the text value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "textValue")]
    pub text_value: Option<RareStringData>,
    /// Only set for input elements, contains the input's associated text value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputValue")]
    pub input_value: Option<RareStringData>,
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputChecked")]
    pub input_checked: Option<RareBooleanData>,
    /// Only set for option elements, indicates if the element has been selected
    #[serde(skip_serializing_if = "Option::is_none", rename = "optionSelected")]
    pub option_selected: Option<RareBooleanData>,
    /// The index of the document in the list of the snapshot documents.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentDocumentIndex")]
    pub content_document_index: Option<RareIntegerData>,
    /// Type of a pseudo element node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoType")]
    pub pseudo_type: Option<RareStringData>,
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoIdentifier")]
    pub pseudo_identifier: Option<RareStringData>,
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isClickable")]
    pub is_clickable: Option<RareBooleanData>,
    /// The selected url for nodes with a srcset attribute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentSourceURL")]
    pub current_source_url: Option<RareStringData>,
    /// The url of the script (if any) that generates this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originURL")]
    pub origin_url: Option<RareStringData>,
}
/// Table of details of an element in the DOM tree with a LayoutObject.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LayoutTreeSnapshot {
    /// Index of the corresponding node in the 'NodeTreeSnapshot' array returned by 'captureSnapshot'.
    #[serde(rename = "nodeIndex")]
    pub node_index: Vec<i64>,
    /// Array of indexes specifying computed style strings, filtered according to the 'computedStyles' parameter passed to 'captureSnapshot'.
    pub styles: Vec<ArrayOfStrings>,
    /// The absolute position bounding box.
    pub bounds: Vec<Rectangle>,
    /// Contents of the LayoutText, if any.
    pub text: Vec<StringIndex>,
    /// Stacking context information.
    #[serde(rename = "stackingContexts")]
    pub stacking_contexts: RareBooleanData,
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// captureSnapshot was true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "paintOrders")]
    pub paint_orders: Option<Vec<i64>>,
    /// The offset rect of nodes. Only available when includeDOMRects is set to true
    #[serde(skip_serializing_if = "Option::is_none", rename = "offsetRects")]
    pub offset_rects: Option<Vec<Rectangle>>,
    /// The scroll rect of nodes. Only available when includeDOMRects is set to true
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollRects")]
    pub scroll_rects: Option<Vec<Rectangle>>,
    /// The client rect of nodes. Only available when includeDOMRects is set to true
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientRects")]
    pub client_rects: Option<Vec<Rectangle>>,
    /// The list of background colors that are blended with colors of overlapping elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "blendedBackgroundColors")]
    pub blended_background_colors: Option<Vec<StringIndex>>,
    /// The list of computed text opacities.
    #[serde(skip_serializing_if = "Option::is_none", rename = "textColorOpacities")]
    pub text_color_opacities: Option<Vec<f64>>,
}
/// Table of details of the post layout rendered text positions. The exact layout should not be regarded as
/// stable and may change between versions.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TextBoxSnapshot {
    /// Index of the layout tree node that owns this box collection.
    #[serde(rename = "layoutIndex")]
    pub layout_index: Vec<i64>,
    /// The absolute position bounding box.
    pub bounds: Vec<Rectangle>,
    /// The starting index in characters, for this post layout textbox substring. Characters that
    /// would be represented as a surrogate pair in UTF-16 have length 2.
    pub start: Vec<i64>,
    /// The number of characters in this post layout textbox substring. Characters that would be
    /// represented as a surrogate pair in UTF-16 have length 2.
    pub length: Vec<i64>,
}
/// Disables DOM snapshot agent for the given page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMSnapshot.disable")]
pub struct DisableParams {

}
/// Enables DOM snapshot agent for the given page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMSnapshot.enable")]
pub struct EnableParams {

}
/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMSnapshot.getSnapshot", response = "GetSnapshotReturns<'a>")]
pub struct GetSnapshotParams<'a> {
    /// Whitelist of computed styles to return.
    #[serde(rename = "computedStyleWhitelist")]
    pub computed_style_whitelist: Vec<Cow<'a, str>>,
    /// Whether or not to retrieve details of DOM listeners (default false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeEventListeners")]
    pub include_event_listeners: Option<bool>,
    /// Whether to determine and include the paint order index of LayoutTreeNodes (default false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "includePaintOrder")]
    pub include_paint_order: Option<bool>,
    /// Whether to include UA shadow tree in the snapshot (default false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeUserAgentShadowTree")]
    pub include_user_agent_shadow_tree: Option<bool>,
}
/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetSnapshotReturns<'a> {
    /// The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.
    #[serde(rename = "domNodes")]
    pub dom_nodes: Vec<DOMNode<'a>>,
    /// The nodes in the layout tree.
    #[serde(rename = "layoutTreeNodes")]
    pub layout_tree_nodes: Vec<LayoutTreeNode<'a>>,
    /// Whitelisted ComputedStyle properties for each node in the layout tree.
    #[serde(rename = "computedStyles")]
    pub computed_styles: Vec<ComputedStyle<'a>>,
}
/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "DOMSnapshot.captureSnapshot", response = "CaptureSnapshotReturns<'a>")]
pub struct CaptureSnapshotParams<'a> {
    /// Whitelist of computed styles to return.
    #[serde(rename = "computedStyles")]
    pub computed_styles: Vec<Cow<'a, str>>,
    /// Whether to include layout object paint orders into the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includePaintOrder")]
    pub include_paint_order: Option<bool>,
    /// Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeDOMRects")]
    pub include_dom_rects: Option<bool>,
    /// Whether to include blended background colors in the snapshot (default: false).
    /// Blended background color is achieved by blending background colors of all elements
    /// that overlap with the current element.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeBlendedBackgroundColors")]
    pub include_blended_background_colors: Option<bool>,
    /// Whether to include text color opacity in the snapshot (default: false).
    /// An element might have the opacity property set that affects the text color of the element.
    /// The final text color opacity is computed based on the opacity of all overlapping elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeTextColorOpacities")]
    pub include_text_color_opacities: Option<bool>,
}
/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotReturns<'a> {
    /// The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.
    pub documents: Vec<DocumentSnapshot>,
    /// Shared string table that all string properties refer to with indexes.
    pub strings: Vec<Cow<'a, str>>,
}