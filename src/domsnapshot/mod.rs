//! This domain facilitates obtaining document snapshots with DOM, layout, and style information.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// A Node in the DOM tree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DOMNode<'a> {
    /// 'Node''s nodeType.
    #[serde(rename = "nodeType")]
    node_type: i64,
    /// 'Node''s nodeName.
    #[serde(rename = "nodeName")]
    node_name: Cow<'a, str>,
    /// 'Node''s nodeValue.
    #[serde(rename = "nodeValue")]
    node_value: Cow<'a, str>,
    /// Only set for textarea elements, contains the text value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "textValue")]
    text_value: Option<Cow<'a, str>>,
    /// Only set for input elements, contains the input's associated text value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputValue")]
    input_value: Option<Cow<'a, str>>,
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputChecked")]
    input_checked: Option<bool>,
    /// Only set for option elements, indicates if the element has been selected
    #[serde(skip_serializing_if = "Option::is_none", rename = "optionSelected")]
    option_selected: Option<bool>,
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    #[serde(rename = "backendNodeId")]
    backend_node_id: crate::dom::BackendNodeId,
    /// The indexes of the node's child nodes in the 'domNodes' array returned by 'getSnapshot', if
    /// any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childNodeIndexes")]
    child_node_indexes: Option<Vec<i64>>,
    /// Attributes of an 'Element' node.
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<NameValue<'a>>>,
    /// Indexes of pseudo elements associated with this node in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoElementIndexes")]
    pseudo_element_indexes: Option<Vec<i64>>,
    /// The index of the node's related layout tree node in the 'layoutTreeNodes' array returned by
    /// 'getSnapshot', if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "layoutNodeIndex")]
    layout_node_index: Option<u64>,
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    #[serde(skip_serializing_if = "Option::is_none", rename = "documentURL")]
    document_url: Option<Cow<'a, str>>,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseURL")]
    base_url: Option<Cow<'a, str>>,
    /// Only set for documents, contains the document's content language.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentLanguage")]
    content_language: Option<Cow<'a, str>>,
    /// Only set for documents, contains the document's character set encoding.
    #[serde(skip_serializing_if = "Option::is_none", rename = "documentEncoding")]
    document_encoding: Option<Cow<'a, str>>,
    /// 'DocumentType' node's publicId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "publicId")]
    public_id: Option<Cow<'a, str>>,
    /// 'DocumentType' node's systemId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "systemId")]
    system_id: Option<Cow<'a, str>>,
    /// Frame ID for frame owner elements and also for the document node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
    /// The index of a frame owner element's content document in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentDocumentIndex")]
    content_document_index: Option<u64>,
    /// Type of a pseudo element node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoType")]
    pseudo_type: Option<crate::dom::PseudoType>,
    /// Shadow root type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "shadowRootType")]
    shadow_root_type: Option<crate::dom::ShadowRootType>,
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isClickable")]
    is_clickable: Option<bool>,
    /// Details of the node's event listeners, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "eventListeners")]
    event_listeners: Option<Vec<crate::domdebugger::EventListener<'a>>>,
    /// The selected url for nodes with a srcset attribute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentSourceURL")]
    current_source_url: Option<Cow<'a, str>>,
    /// The url of the script (if any) that generates this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originURL")]
    origin_url: Option<Cow<'a, str>>,
    /// Scroll offsets, set when this node is a Document.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollOffsetX")]
    scroll_offset_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollOffsetY")]
    scroll_offset_y: Option<f64>,
}

impl<'a> DOMNode<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_type`: `Node`'s nodeType.
    /// * `node_name`: `Node`'s nodeName.
    /// * `node_value`: `Node`'s nodeValue.
    /// * `backend_node_id`: `Node`'s id, corresponds to DOM.Node.backendNodeId.
    pub fn builder(node_type: i64, node_name: impl Into<Cow<'a, str>>, node_value: impl Into<Cow<'a, str>>, backend_node_id: crate::dom::BackendNodeId) -> DOMNodeBuilder<'a> {
        DOMNodeBuilder {
            node_type: node_type,
            node_name: node_name.into(),
            node_value: node_value.into(),
            text_value: None,
            input_value: None,
            input_checked: None,
            option_selected: None,
            backend_node_id: backend_node_id,
            child_node_indexes: None,
            attributes: None,
            pseudo_element_indexes: None,
            layout_node_index: None,
            document_url: None,
            base_url: None,
            content_language: None,
            document_encoding: None,
            public_id: None,
            system_id: None,
            frame_id: None,
            content_document_index: None,
            pseudo_type: None,
            shadow_root_type: None,
            is_clickable: None,
            event_listeners: None,
            current_source_url: None,
            origin_url: None,
            scroll_offset_x: None,
            scroll_offset_y: None,
        }
    }
    /// 'Node''s nodeType.
    pub fn node_type(&self) -> i64 { self.node_type }
    /// 'Node''s nodeName.
    pub fn node_name(&self) -> &str { self.node_name.as_ref() }
    /// 'Node''s nodeValue.
    pub fn node_value(&self) -> &str { self.node_value.as_ref() }
    /// Only set for textarea elements, contains the text value.
    pub fn text_value(&self) -> Option<&str> { self.text_value.as_deref() }
    /// Only set for input elements, contains the input's associated text value.
    pub fn input_value(&self) -> Option<&str> { self.input_value.as_deref() }
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    pub fn input_checked(&self) -> Option<bool> { self.input_checked }
    /// Only set for option elements, indicates if the element has been selected
    pub fn option_selected(&self) -> Option<bool> { self.option_selected }
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    pub fn backend_node_id(&self) -> &crate::dom::BackendNodeId { &self.backend_node_id }
    /// The indexes of the node's child nodes in the 'domNodes' array returned by 'getSnapshot', if
    /// any.
    pub fn child_node_indexes(&self) -> Option<&[i64]> { self.child_node_indexes.as_deref() }
    /// Attributes of an 'Element' node.
    pub fn attributes(&self) -> Option<&[NameValue<'a>]> { self.attributes.as_deref() }
    /// Indexes of pseudo elements associated with this node in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    pub fn pseudo_element_indexes(&self) -> Option<&[i64]> { self.pseudo_element_indexes.as_deref() }
    /// The index of the node's related layout tree node in the 'layoutTreeNodes' array returned by
    /// 'getSnapshot', if any.
    pub fn layout_node_index(&self) -> Option<u64> { self.layout_node_index }
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    pub fn document_url(&self) -> Option<&str> { self.document_url.as_deref() }
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    pub fn base_url(&self) -> Option<&str> { self.base_url.as_deref() }
    /// Only set for documents, contains the document's content language.
    pub fn content_language(&self) -> Option<&str> { self.content_language.as_deref() }
    /// Only set for documents, contains the document's character set encoding.
    pub fn document_encoding(&self) -> Option<&str> { self.document_encoding.as_deref() }
    /// 'DocumentType' node's publicId.
    pub fn public_id(&self) -> Option<&str> { self.public_id.as_deref() }
    /// 'DocumentType' node's systemId.
    pub fn system_id(&self) -> Option<&str> { self.system_id.as_deref() }
    /// Frame ID for frame owner elements and also for the document node.
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
    /// The index of a frame owner element's content document in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    pub fn content_document_index(&self) -> Option<u64> { self.content_document_index }
    /// Type of a pseudo element node.
    pub fn pseudo_type(&self) -> Option<&crate::dom::PseudoType> { self.pseudo_type.as_ref() }
    /// Shadow root type.
    pub fn shadow_root_type(&self) -> Option<&crate::dom::ShadowRootType> { self.shadow_root_type.as_ref() }
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    pub fn is_clickable(&self) -> Option<bool> { self.is_clickable }
    /// Details of the node's event listeners, if any.
    pub fn event_listeners(&self) -> Option<&[crate::domdebugger::EventListener<'a>]> { self.event_listeners.as_deref() }
    /// The selected url for nodes with a srcset attribute.
    pub fn current_source_url(&self) -> Option<&str> { self.current_source_url.as_deref() }
    /// The url of the script (if any) that generates this node.
    pub fn origin_url(&self) -> Option<&str> { self.origin_url.as_deref() }
    /// Scroll offsets, set when this node is a Document.
    pub fn scroll_offset_x(&self) -> Option<f64> { self.scroll_offset_x }
    pub fn scroll_offset_y(&self) -> Option<f64> { self.scroll_offset_y }
}


pub struct DOMNodeBuilder<'a> {
    node_type: i64,
    node_name: Cow<'a, str>,
    node_value: Cow<'a, str>,
    text_value: Option<Cow<'a, str>>,
    input_value: Option<Cow<'a, str>>,
    input_checked: Option<bool>,
    option_selected: Option<bool>,
    backend_node_id: crate::dom::BackendNodeId,
    child_node_indexes: Option<Vec<i64>>,
    attributes: Option<Vec<NameValue<'a>>>,
    pseudo_element_indexes: Option<Vec<i64>>,
    layout_node_index: Option<u64>,
    document_url: Option<Cow<'a, str>>,
    base_url: Option<Cow<'a, str>>,
    content_language: Option<Cow<'a, str>>,
    document_encoding: Option<Cow<'a, str>>,
    public_id: Option<Cow<'a, str>>,
    system_id: Option<Cow<'a, str>>,
    frame_id: Option<crate::page::FrameId<'a>>,
    content_document_index: Option<u64>,
    pseudo_type: Option<crate::dom::PseudoType>,
    shadow_root_type: Option<crate::dom::ShadowRootType>,
    is_clickable: Option<bool>,
    event_listeners: Option<Vec<crate::domdebugger::EventListener<'a>>>,
    current_source_url: Option<Cow<'a, str>>,
    origin_url: Option<Cow<'a, str>>,
    scroll_offset_x: Option<f64>,
    scroll_offset_y: Option<f64>,
}

impl<'a> DOMNodeBuilder<'a> {
    /// Only set for textarea elements, contains the text value.
    pub fn text_value(mut self, text_value: impl Into<Cow<'a, str>>) -> Self { self.text_value = Some(text_value.into()); self }
    /// Only set for input elements, contains the input's associated text value.
    pub fn input_value(mut self, input_value: impl Into<Cow<'a, str>>) -> Self { self.input_value = Some(input_value.into()); self }
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    pub fn input_checked(mut self, input_checked: bool) -> Self { self.input_checked = Some(input_checked); self }
    /// Only set for option elements, indicates if the element has been selected
    pub fn option_selected(mut self, option_selected: bool) -> Self { self.option_selected = Some(option_selected); self }
    /// The indexes of the node's child nodes in the 'domNodes' array returned by 'getSnapshot', if
    /// any.
    pub fn child_node_indexes(mut self, child_node_indexes: Vec<i64>) -> Self { self.child_node_indexes = Some(child_node_indexes); self }
    /// Attributes of an 'Element' node.
    pub fn attributes(mut self, attributes: Vec<NameValue<'a>>) -> Self { self.attributes = Some(attributes); self }
    /// Indexes of pseudo elements associated with this node in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    pub fn pseudo_element_indexes(mut self, pseudo_element_indexes: Vec<i64>) -> Self { self.pseudo_element_indexes = Some(pseudo_element_indexes); self }
    /// The index of the node's related layout tree node in the 'layoutTreeNodes' array returned by
    /// 'getSnapshot', if any.
    pub fn layout_node_index(mut self, layout_node_index: u64) -> Self { self.layout_node_index = Some(layout_node_index); self }
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    pub fn document_url(mut self, document_url: impl Into<Cow<'a, str>>) -> Self { self.document_url = Some(document_url.into()); self }
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    pub fn base_url(mut self, base_url: impl Into<Cow<'a, str>>) -> Self { self.base_url = Some(base_url.into()); self }
    /// Only set for documents, contains the document's content language.
    pub fn content_language(mut self, content_language: impl Into<Cow<'a, str>>) -> Self { self.content_language = Some(content_language.into()); self }
    /// Only set for documents, contains the document's character set encoding.
    pub fn document_encoding(mut self, document_encoding: impl Into<Cow<'a, str>>) -> Self { self.document_encoding = Some(document_encoding.into()); self }
    /// 'DocumentType' node's publicId.
    pub fn public_id(mut self, public_id: impl Into<Cow<'a, str>>) -> Self { self.public_id = Some(public_id.into()); self }
    /// 'DocumentType' node's systemId.
    pub fn system_id(mut self, system_id: impl Into<Cow<'a, str>>) -> Self { self.system_id = Some(system_id.into()); self }
    /// Frame ID for frame owner elements and also for the document node.
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    /// The index of a frame owner element's content document in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    pub fn content_document_index(mut self, content_document_index: u64) -> Self { self.content_document_index = Some(content_document_index); self }
    /// Type of a pseudo element node.
    pub fn pseudo_type(mut self, pseudo_type: crate::dom::PseudoType) -> Self { self.pseudo_type = Some(pseudo_type); self }
    /// Shadow root type.
    pub fn shadow_root_type(mut self, shadow_root_type: crate::dom::ShadowRootType) -> Self { self.shadow_root_type = Some(shadow_root_type); self }
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    pub fn is_clickable(mut self, is_clickable: bool) -> Self { self.is_clickable = Some(is_clickable); self }
    /// Details of the node's event listeners, if any.
    pub fn event_listeners(mut self, event_listeners: Vec<crate::domdebugger::EventListener<'a>>) -> Self { self.event_listeners = Some(event_listeners); self }
    /// The selected url for nodes with a srcset attribute.
    pub fn current_source_url(mut self, current_source_url: impl Into<Cow<'a, str>>) -> Self { self.current_source_url = Some(current_source_url.into()); self }
    /// The url of the script (if any) that generates this node.
    pub fn origin_url(mut self, origin_url: impl Into<Cow<'a, str>>) -> Self { self.origin_url = Some(origin_url.into()); self }
    /// Scroll offsets, set when this node is a Document.
    pub fn scroll_offset_x(mut self, scroll_offset_x: f64) -> Self { self.scroll_offset_x = Some(scroll_offset_x); self }
    pub fn scroll_offset_y(mut self, scroll_offset_y: f64) -> Self { self.scroll_offset_y = Some(scroll_offset_y); self }
    pub fn build(self) -> DOMNode<'a> {
        DOMNode {
            node_type: self.node_type,
            node_name: self.node_name,
            node_value: self.node_value,
            text_value: self.text_value,
            input_value: self.input_value,
            input_checked: self.input_checked,
            option_selected: self.option_selected,
            backend_node_id: self.backend_node_id,
            child_node_indexes: self.child_node_indexes,
            attributes: self.attributes,
            pseudo_element_indexes: self.pseudo_element_indexes,
            layout_node_index: self.layout_node_index,
            document_url: self.document_url,
            base_url: self.base_url,
            content_language: self.content_language,
            document_encoding: self.document_encoding,
            public_id: self.public_id,
            system_id: self.system_id,
            frame_id: self.frame_id,
            content_document_index: self.content_document_index,
            pseudo_type: self.pseudo_type,
            shadow_root_type: self.shadow_root_type,
            is_clickable: self.is_clickable,
            event_listeners: self.event_listeners,
            current_source_url: self.current_source_url,
            origin_url: self.origin_url,
            scroll_offset_x: self.scroll_offset_x,
            scroll_offset_y: self.scroll_offset_y,
        }
    }
}

/// Details of post layout rendered text positions. The exact layout should not be regarded as
/// stable and may change between versions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InlineTextBox {
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    #[serde(rename = "boundingBox")]
    bounding_box: crate::dom::Rect,
    /// The starting index in characters, for this post layout textbox substring. Characters that
    /// would be represented as a surrogate pair in UTF-16 have length 2.
    #[serde(rename = "startCharacterIndex")]
    start_character_index: u64,
    /// The number of characters in this post layout textbox substring. Characters that would be
    /// represented as a surrogate pair in UTF-16 have length 2.
    #[serde(rename = "numCharacters")]
    num_characters: i64,
}

impl InlineTextBox {
    /// Creates a builder for this type with the required parameters:
    /// * `bounding_box`: The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    /// * `start_character_index`: The starting index in characters, for this post layout textbox substring. Characters that would be represented as a surrogate pair in UTF-16 have length 2.
    /// * `num_characters`: The number of characters in this post layout textbox substring. Characters that would be represented as a surrogate pair in UTF-16 have length 2.
    pub fn builder(bounding_box: crate::dom::Rect, start_character_index: u64, num_characters: i64) -> InlineTextBoxBuilder {
        InlineTextBoxBuilder {
            bounding_box: bounding_box,
            start_character_index: start_character_index,
            num_characters: num_characters,
        }
    }
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    pub fn bounding_box(&self) -> &crate::dom::Rect { &self.bounding_box }
    /// The starting index in characters, for this post layout textbox substring. Characters that
    /// would be represented as a surrogate pair in UTF-16 have length 2.
    pub fn start_character_index(&self) -> u64 { self.start_character_index }
    /// The number of characters in this post layout textbox substring. Characters that would be
    /// represented as a surrogate pair in UTF-16 have length 2.
    pub fn num_characters(&self) -> i64 { self.num_characters }
}


pub struct InlineTextBoxBuilder {
    bounding_box: crate::dom::Rect,
    start_character_index: u64,
    num_characters: i64,
}

impl InlineTextBoxBuilder {
    pub fn build(self) -> InlineTextBox {
        InlineTextBox {
            bounding_box: self.bounding_box,
            start_character_index: self.start_character_index,
            num_characters: self.num_characters,
        }
    }
}

/// Details of an element in the DOM tree with a LayoutObject.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutTreeNode<'a> {
    /// The index of the related DOM node in the 'domNodes' array returned by 'getSnapshot'.
    #[serde(rename = "domNodeIndex")]
    dom_node_index: u64,
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    #[serde(rename = "boundingBox")]
    bounding_box: crate::dom::Rect,
    /// Contents of the LayoutText, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "layoutText")]
    layout_text: Option<Cow<'a, str>>,
    /// The post-layout inline text nodes, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inlineTextNodes")]
    inline_text_nodes: Option<Vec<InlineTextBox>>,
    /// Index into the 'computedStyles' array returned by 'getSnapshot'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleIndex")]
    style_index: Option<u64>,
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// getSnapshot was true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "paintOrder")]
    paint_order: Option<i64>,
    /// Set to true to indicate the element begins a new stacking context.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isStackingContext")]
    is_stacking_context: Option<bool>,
}

impl<'a> LayoutTreeNode<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `dom_node_index`: The index of the related DOM node in the `domNodes` array returned by `getSnapshot`.
    /// * `bounding_box`: The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    pub fn builder(dom_node_index: u64, bounding_box: crate::dom::Rect) -> LayoutTreeNodeBuilder<'a> {
        LayoutTreeNodeBuilder {
            dom_node_index: dom_node_index,
            bounding_box: bounding_box,
            layout_text: None,
            inline_text_nodes: None,
            style_index: None,
            paint_order: None,
            is_stacking_context: None,
        }
    }
    /// The index of the related DOM node in the 'domNodes' array returned by 'getSnapshot'.
    pub fn dom_node_index(&self) -> u64 { self.dom_node_index }
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    pub fn bounding_box(&self) -> &crate::dom::Rect { &self.bounding_box }
    /// Contents of the LayoutText, if any.
    pub fn layout_text(&self) -> Option<&str> { self.layout_text.as_deref() }
    /// The post-layout inline text nodes, if any.
    pub fn inline_text_nodes(&self) -> Option<&[InlineTextBox]> { self.inline_text_nodes.as_deref() }
    /// Index into the 'computedStyles' array returned by 'getSnapshot'.
    pub fn style_index(&self) -> Option<u64> { self.style_index }
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// getSnapshot was true.
    pub fn paint_order(&self) -> Option<i64> { self.paint_order }
    /// Set to true to indicate the element begins a new stacking context.
    pub fn is_stacking_context(&self) -> Option<bool> { self.is_stacking_context }
}


pub struct LayoutTreeNodeBuilder<'a> {
    dom_node_index: u64,
    bounding_box: crate::dom::Rect,
    layout_text: Option<Cow<'a, str>>,
    inline_text_nodes: Option<Vec<InlineTextBox>>,
    style_index: Option<u64>,
    paint_order: Option<i64>,
    is_stacking_context: Option<bool>,
}

impl<'a> LayoutTreeNodeBuilder<'a> {
    /// Contents of the LayoutText, if any.
    pub fn layout_text(mut self, layout_text: impl Into<Cow<'a, str>>) -> Self { self.layout_text = Some(layout_text.into()); self }
    /// The post-layout inline text nodes, if any.
    pub fn inline_text_nodes(mut self, inline_text_nodes: Vec<InlineTextBox>) -> Self { self.inline_text_nodes = Some(inline_text_nodes); self }
    /// Index into the 'computedStyles' array returned by 'getSnapshot'.
    pub fn style_index(mut self, style_index: u64) -> Self { self.style_index = Some(style_index); self }
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// getSnapshot was true.
    pub fn paint_order(mut self, paint_order: i64) -> Self { self.paint_order = Some(paint_order); self }
    /// Set to true to indicate the element begins a new stacking context.
    pub fn is_stacking_context(mut self, is_stacking_context: bool) -> Self { self.is_stacking_context = Some(is_stacking_context); self }
    pub fn build(self) -> LayoutTreeNode<'a> {
        LayoutTreeNode {
            dom_node_index: self.dom_node_index,
            bounding_box: self.bounding_box,
            layout_text: self.layout_text,
            inline_text_nodes: self.inline_text_nodes,
            style_index: self.style_index,
            paint_order: self.paint_order,
            is_stacking_context: self.is_stacking_context,
        }
    }
}

/// A subset of the full ComputedStyle as defined by the request whitelist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComputedStyle<'a> {
    /// Name/value pairs of computed style properties.
    properties: Vec<NameValue<'a>>,
}

impl<'a> ComputedStyle<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `properties`: Name/value pairs of computed style properties.
    pub fn builder(properties: Vec<NameValue<'a>>) -> ComputedStyleBuilder<'a> {
        ComputedStyleBuilder {
            properties: properties,
        }
    }
    /// Name/value pairs of computed style properties.
    pub fn properties(&self) -> &[NameValue<'a>] { &self.properties }
}


pub struct ComputedStyleBuilder<'a> {
    properties: Vec<NameValue<'a>>,
}

impl<'a> ComputedStyleBuilder<'a> {
    pub fn build(self) -> ComputedStyle<'a> {
        ComputedStyle {
            properties: self.properties,
        }
    }
}

/// A name/value pair.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NameValue<'a> {
    /// Attribute/property name.
    name: Cow<'a, str>,
    /// Attribute/property value.
    value: Cow<'a, str>,
}

impl<'a> NameValue<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Attribute/property name.
    /// * `value`: Attribute/property value.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> NameValueBuilder<'a> {
        NameValueBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    /// Attribute/property name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Attribute/property value.
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct NameValueBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> NameValueBuilder<'a> {
    pub fn build(self) -> NameValue<'a> {
        NameValue {
            name: self.name,
            value: self.value,
        }
    }
}

/// Index of the string in the strings table.

pub type StringIndex = i64;

/// Index of the string in the strings table.

pub type ArrayOfStrings = Vec<StringIndex>;

/// Data that is only present on rare nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RareStringData {
    index: Vec<i64>,
    value: Vec<StringIndex>,
}

impl RareStringData {
    /// Creates a builder for this type with the required parameters:
    /// * `index`: 
    /// * `value`: 
    pub fn builder(index: Vec<i64>, value: Vec<StringIndex>) -> RareStringDataBuilder {
        RareStringDataBuilder {
            index: index,
            value: value,
        }
    }
    pub fn index(&self) -> &[i64] { &self.index }
    pub fn value(&self) -> &[StringIndex] { &self.value }
}


pub struct RareStringDataBuilder {
    index: Vec<i64>,
    value: Vec<StringIndex>,
}

impl RareStringDataBuilder {
    pub fn build(self) -> RareStringData {
        RareStringData {
            index: self.index,
            value: self.value,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RareBooleanData {
    index: Vec<i64>,
}

impl RareBooleanData {
    /// Creates a builder for this type with the required parameters:
    /// * `index`: 
    pub fn builder(index: Vec<i64>) -> RareBooleanDataBuilder {
        RareBooleanDataBuilder {
            index: index,
        }
    }
    pub fn index(&self) -> &[i64] { &self.index }
}


pub struct RareBooleanDataBuilder {
    index: Vec<i64>,
}

impl RareBooleanDataBuilder {
    pub fn build(self) -> RareBooleanData {
        RareBooleanData {
            index: self.index,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RareIntegerData {
    index: Vec<i64>,
    value: Vec<i64>,
}

impl RareIntegerData {
    /// Creates a builder for this type with the required parameters:
    /// * `index`: 
    /// * `value`: 
    pub fn builder(index: Vec<i64>, value: Vec<i64>) -> RareIntegerDataBuilder {
        RareIntegerDataBuilder {
            index: index,
            value: value,
        }
    }
    pub fn index(&self) -> &[i64] { &self.index }
    pub fn value(&self) -> &[i64] { &self.value }
}


pub struct RareIntegerDataBuilder {
    index: Vec<i64>,
    value: Vec<i64>,
}

impl RareIntegerDataBuilder {
    pub fn build(self) -> RareIntegerData {
        RareIntegerData {
            index: self.index,
            value: self.value,
        }
    }
}


pub type Rectangle = Vec<f64>;

/// Document snapshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSnapshot {
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    #[serde(rename = "documentURL")]
    document_url: StringIndex,
    /// Document title.
    title: StringIndex,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    #[serde(rename = "baseURL")]
    base_url: StringIndex,
    /// Contains the document's content language.
    #[serde(rename = "contentLanguage")]
    content_language: StringIndex,
    /// Contains the document's character set encoding.
    #[serde(rename = "encodingName")]
    encoding_name: StringIndex,
    /// 'DocumentType' node's publicId.
    #[serde(rename = "publicId")]
    public_id: StringIndex,
    /// 'DocumentType' node's systemId.
    #[serde(rename = "systemId")]
    system_id: StringIndex,
    /// Frame ID for frame owner elements and also for the document node.
    #[serde(rename = "frameId")]
    frame_id: StringIndex,
    /// A table with dom nodes.
    nodes: NodeTreeSnapshot,
    /// The nodes in the layout tree.
    layout: LayoutTreeSnapshot,
    /// The post-layout inline text nodes.
    #[serde(rename = "textBoxes")]
    text_boxes: TextBoxSnapshot,
    /// Horizontal scroll offset.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollOffsetX")]
    scroll_offset_x: Option<f64>,
    /// Vertical scroll offset.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollOffsetY")]
    scroll_offset_y: Option<f64>,
    /// Document content width.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentWidth")]
    content_width: Option<f64>,
    /// Document content height.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentHeight")]
    content_height: Option<f64>,
}

impl DocumentSnapshot {
    /// Creates a builder for this type with the required parameters:
    /// * `document_url`: Document URL that `Document` or `FrameOwner` node points to.
    /// * `title`: Document title.
    /// * `base_url`: Base URL that `Document` or `FrameOwner` node uses for URL completion.
    /// * `content_language`: Contains the document's content language.
    /// * `encoding_name`: Contains the document's character set encoding.
    /// * `public_id`: `DocumentType` node's publicId.
    /// * `system_id`: `DocumentType` node's systemId.
    /// * `frame_id`: Frame ID for frame owner elements and also for the document node.
    /// * `nodes`: A table with dom nodes.
    /// * `layout`: The nodes in the layout tree.
    /// * `text_boxes`: The post-layout inline text nodes.
    pub fn builder(document_url: StringIndex, title: StringIndex, base_url: StringIndex, content_language: StringIndex, encoding_name: StringIndex, public_id: StringIndex, system_id: StringIndex, frame_id: StringIndex, nodes: NodeTreeSnapshot, layout: LayoutTreeSnapshot, text_boxes: TextBoxSnapshot) -> DocumentSnapshotBuilder {
        DocumentSnapshotBuilder {
            document_url: document_url,
            title: title,
            base_url: base_url,
            content_language: content_language,
            encoding_name: encoding_name,
            public_id: public_id,
            system_id: system_id,
            frame_id: frame_id,
            nodes: nodes,
            layout: layout,
            text_boxes: text_boxes,
            scroll_offset_x: None,
            scroll_offset_y: None,
            content_width: None,
            content_height: None,
        }
    }
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    pub fn document_url(&self) -> &StringIndex { &self.document_url }
    /// Document title.
    pub fn title(&self) -> &StringIndex { &self.title }
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    pub fn base_url(&self) -> &StringIndex { &self.base_url }
    /// Contains the document's content language.
    pub fn content_language(&self) -> &StringIndex { &self.content_language }
    /// Contains the document's character set encoding.
    pub fn encoding_name(&self) -> &StringIndex { &self.encoding_name }
    /// 'DocumentType' node's publicId.
    pub fn public_id(&self) -> &StringIndex { &self.public_id }
    /// 'DocumentType' node's systemId.
    pub fn system_id(&self) -> &StringIndex { &self.system_id }
    /// Frame ID for frame owner elements and also for the document node.
    pub fn frame_id(&self) -> &StringIndex { &self.frame_id }
    /// A table with dom nodes.
    pub fn nodes(&self) -> &NodeTreeSnapshot { &self.nodes }
    /// The nodes in the layout tree.
    pub fn layout(&self) -> &LayoutTreeSnapshot { &self.layout }
    /// The post-layout inline text nodes.
    pub fn text_boxes(&self) -> &TextBoxSnapshot { &self.text_boxes }
    /// Horizontal scroll offset.
    pub fn scroll_offset_x(&self) -> Option<f64> { self.scroll_offset_x }
    /// Vertical scroll offset.
    pub fn scroll_offset_y(&self) -> Option<f64> { self.scroll_offset_y }
    /// Document content width.
    pub fn content_width(&self) -> Option<f64> { self.content_width }
    /// Document content height.
    pub fn content_height(&self) -> Option<f64> { self.content_height }
}


pub struct DocumentSnapshotBuilder {
    document_url: StringIndex,
    title: StringIndex,
    base_url: StringIndex,
    content_language: StringIndex,
    encoding_name: StringIndex,
    public_id: StringIndex,
    system_id: StringIndex,
    frame_id: StringIndex,
    nodes: NodeTreeSnapshot,
    layout: LayoutTreeSnapshot,
    text_boxes: TextBoxSnapshot,
    scroll_offset_x: Option<f64>,
    scroll_offset_y: Option<f64>,
    content_width: Option<f64>,
    content_height: Option<f64>,
}

impl DocumentSnapshotBuilder {
    /// Horizontal scroll offset.
    pub fn scroll_offset_x(mut self, scroll_offset_x: f64) -> Self { self.scroll_offset_x = Some(scroll_offset_x); self }
    /// Vertical scroll offset.
    pub fn scroll_offset_y(mut self, scroll_offset_y: f64) -> Self { self.scroll_offset_y = Some(scroll_offset_y); self }
    /// Document content width.
    pub fn content_width(mut self, content_width: f64) -> Self { self.content_width = Some(content_width); self }
    /// Document content height.
    pub fn content_height(mut self, content_height: f64) -> Self { self.content_height = Some(content_height); self }
    pub fn build(self) -> DocumentSnapshot {
        DocumentSnapshot {
            document_url: self.document_url,
            title: self.title,
            base_url: self.base_url,
            content_language: self.content_language,
            encoding_name: self.encoding_name,
            public_id: self.public_id,
            system_id: self.system_id,
            frame_id: self.frame_id,
            nodes: self.nodes,
            layout: self.layout,
            text_boxes: self.text_boxes,
            scroll_offset_x: self.scroll_offset_x,
            scroll_offset_y: self.scroll_offset_y,
            content_width: self.content_width,
            content_height: self.content_height,
        }
    }
}

/// Table containing nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeTreeSnapshot {
    /// Parent node index.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentIndex")]
    parent_index: Option<Vec<i64>>,
    /// 'Node''s nodeType.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeType")]
    node_type: Option<Vec<i64>>,
    /// Type of the shadow root the 'Node' is in. String values are equal to the 'ShadowRootType' enum.
    #[serde(skip_serializing_if = "Option::is_none", rename = "shadowRootType")]
    shadow_root_type: Option<RareStringData>,
    /// 'Node''s nodeName.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeName")]
    node_name: Option<Vec<StringIndex>>,
    /// 'Node''s nodeValue.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeValue")]
    node_value: Option<Vec<StringIndex>>,
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<Vec<crate::dom::BackendNodeId>>,
    /// Attributes of an 'Element' node. Flatten name, value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<ArrayOfStrings>>,
    /// Only set for textarea elements, contains the text value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "textValue")]
    text_value: Option<RareStringData>,
    /// Only set for input elements, contains the input's associated text value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputValue")]
    input_value: Option<RareStringData>,
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputChecked")]
    input_checked: Option<RareBooleanData>,
    /// Only set for option elements, indicates if the element has been selected
    #[serde(skip_serializing_if = "Option::is_none", rename = "optionSelected")]
    option_selected: Option<RareBooleanData>,
    /// The index of the document in the list of the snapshot documents.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentDocumentIndex")]
    content_document_index: Option<RareIntegerData>,
    /// Type of a pseudo element node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoType")]
    pseudo_type: Option<RareStringData>,
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoIdentifier")]
    pseudo_identifier: Option<RareStringData>,
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isClickable")]
    is_clickable: Option<RareBooleanData>,
    /// The selected url for nodes with a srcset attribute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentSourceURL")]
    current_source_url: Option<RareStringData>,
    /// The url of the script (if any) that generates this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originURL")]
    origin_url: Option<RareStringData>,
}

impl NodeTreeSnapshot {
    /// Creates a builder for this type.
    pub fn builder() -> NodeTreeSnapshotBuilder {
        NodeTreeSnapshotBuilder {
            parent_index: None,
            node_type: None,
            shadow_root_type: None,
            node_name: None,
            node_value: None,
            backend_node_id: None,
            attributes: None,
            text_value: None,
            input_value: None,
            input_checked: None,
            option_selected: None,
            content_document_index: None,
            pseudo_type: None,
            pseudo_identifier: None,
            is_clickable: None,
            current_source_url: None,
            origin_url: None,
        }
    }
    /// Parent node index.
    pub fn parent_index(&self) -> Option<&[i64]> { self.parent_index.as_deref() }
    /// 'Node''s nodeType.
    pub fn node_type(&self) -> Option<&[i64]> { self.node_type.as_deref() }
    /// Type of the shadow root the 'Node' is in. String values are equal to the 'ShadowRootType' enum.
    pub fn shadow_root_type(&self) -> Option<&RareStringData> { self.shadow_root_type.as_ref() }
    /// 'Node''s nodeName.
    pub fn node_name(&self) -> Option<&[StringIndex]> { self.node_name.as_deref() }
    /// 'Node''s nodeValue.
    pub fn node_value(&self) -> Option<&[StringIndex]> { self.node_value.as_deref() }
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    pub fn backend_node_id(&self) -> Option<&[crate::dom::BackendNodeId]> { self.backend_node_id.as_deref() }
    /// Attributes of an 'Element' node. Flatten name, value pairs.
    pub fn attributes(&self) -> Option<&[ArrayOfStrings]> { self.attributes.as_deref() }
    /// Only set for textarea elements, contains the text value.
    pub fn text_value(&self) -> Option<&RareStringData> { self.text_value.as_ref() }
    /// Only set for input elements, contains the input's associated text value.
    pub fn input_value(&self) -> Option<&RareStringData> { self.input_value.as_ref() }
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    pub fn input_checked(&self) -> Option<&RareBooleanData> { self.input_checked.as_ref() }
    /// Only set for option elements, indicates if the element has been selected
    pub fn option_selected(&self) -> Option<&RareBooleanData> { self.option_selected.as_ref() }
    /// The index of the document in the list of the snapshot documents.
    pub fn content_document_index(&self) -> Option<&RareIntegerData> { self.content_document_index.as_ref() }
    /// Type of a pseudo element node.
    pub fn pseudo_type(&self) -> Option<&RareStringData> { self.pseudo_type.as_ref() }
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    pub fn pseudo_identifier(&self) -> Option<&RareStringData> { self.pseudo_identifier.as_ref() }
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    pub fn is_clickable(&self) -> Option<&RareBooleanData> { self.is_clickable.as_ref() }
    /// The selected url for nodes with a srcset attribute.
    pub fn current_source_url(&self) -> Option<&RareStringData> { self.current_source_url.as_ref() }
    /// The url of the script (if any) that generates this node.
    pub fn origin_url(&self) -> Option<&RareStringData> { self.origin_url.as_ref() }
}

#[derive(Default)]
pub struct NodeTreeSnapshotBuilder {
    parent_index: Option<Vec<i64>>,
    node_type: Option<Vec<i64>>,
    shadow_root_type: Option<RareStringData>,
    node_name: Option<Vec<StringIndex>>,
    node_value: Option<Vec<StringIndex>>,
    backend_node_id: Option<Vec<crate::dom::BackendNodeId>>,
    attributes: Option<Vec<ArrayOfStrings>>,
    text_value: Option<RareStringData>,
    input_value: Option<RareStringData>,
    input_checked: Option<RareBooleanData>,
    option_selected: Option<RareBooleanData>,
    content_document_index: Option<RareIntegerData>,
    pseudo_type: Option<RareStringData>,
    pseudo_identifier: Option<RareStringData>,
    is_clickable: Option<RareBooleanData>,
    current_source_url: Option<RareStringData>,
    origin_url: Option<RareStringData>,
}

impl NodeTreeSnapshotBuilder {
    /// Parent node index.
    pub fn parent_index(mut self, parent_index: Vec<i64>) -> Self { self.parent_index = Some(parent_index); self }
    /// 'Node''s nodeType.
    pub fn node_type(mut self, node_type: Vec<i64>) -> Self { self.node_type = Some(node_type); self }
    /// Type of the shadow root the 'Node' is in. String values are equal to the 'ShadowRootType' enum.
    pub fn shadow_root_type(mut self, shadow_root_type: RareStringData) -> Self { self.shadow_root_type = Some(shadow_root_type); self }
    /// 'Node''s nodeName.
    pub fn node_name(mut self, node_name: Vec<StringIndex>) -> Self { self.node_name = Some(node_name); self }
    /// 'Node''s nodeValue.
    pub fn node_value(mut self, node_value: Vec<StringIndex>) -> Self { self.node_value = Some(node_value); self }
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    pub fn backend_node_id(mut self, backend_node_id: Vec<crate::dom::BackendNodeId>) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// Attributes of an 'Element' node. Flatten name, value pairs.
    pub fn attributes(mut self, attributes: Vec<ArrayOfStrings>) -> Self { self.attributes = Some(attributes); self }
    /// Only set for textarea elements, contains the text value.
    pub fn text_value(mut self, text_value: RareStringData) -> Self { self.text_value = Some(text_value); self }
    /// Only set for input elements, contains the input's associated text value.
    pub fn input_value(mut self, input_value: RareStringData) -> Self { self.input_value = Some(input_value); self }
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    pub fn input_checked(mut self, input_checked: RareBooleanData) -> Self { self.input_checked = Some(input_checked); self }
    /// Only set for option elements, indicates if the element has been selected
    pub fn option_selected(mut self, option_selected: RareBooleanData) -> Self { self.option_selected = Some(option_selected); self }
    /// The index of the document in the list of the snapshot documents.
    pub fn content_document_index(mut self, content_document_index: RareIntegerData) -> Self { self.content_document_index = Some(content_document_index); self }
    /// Type of a pseudo element node.
    pub fn pseudo_type(mut self, pseudo_type: RareStringData) -> Self { self.pseudo_type = Some(pseudo_type); self }
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    pub fn pseudo_identifier(mut self, pseudo_identifier: RareStringData) -> Self { self.pseudo_identifier = Some(pseudo_identifier); self }
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    pub fn is_clickable(mut self, is_clickable: RareBooleanData) -> Self { self.is_clickable = Some(is_clickable); self }
    /// The selected url for nodes with a srcset attribute.
    pub fn current_source_url(mut self, current_source_url: RareStringData) -> Self { self.current_source_url = Some(current_source_url); self }
    /// The url of the script (if any) that generates this node.
    pub fn origin_url(mut self, origin_url: RareStringData) -> Self { self.origin_url = Some(origin_url); self }
    pub fn build(self) -> NodeTreeSnapshot {
        NodeTreeSnapshot {
            parent_index: self.parent_index,
            node_type: self.node_type,
            shadow_root_type: self.shadow_root_type,
            node_name: self.node_name,
            node_value: self.node_value,
            backend_node_id: self.backend_node_id,
            attributes: self.attributes,
            text_value: self.text_value,
            input_value: self.input_value,
            input_checked: self.input_checked,
            option_selected: self.option_selected,
            content_document_index: self.content_document_index,
            pseudo_type: self.pseudo_type,
            pseudo_identifier: self.pseudo_identifier,
            is_clickable: self.is_clickable,
            current_source_url: self.current_source_url,
            origin_url: self.origin_url,
        }
    }
}

/// Table of details of an element in the DOM tree with a LayoutObject.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutTreeSnapshot {
    /// Index of the corresponding node in the 'NodeTreeSnapshot' array returned by 'captureSnapshot'.
    #[serde(rename = "nodeIndex")]
    node_index: Vec<i64>,
    /// Array of indexes specifying computed style strings, filtered according to the 'computedStyles' parameter passed to 'captureSnapshot'.
    styles: Vec<ArrayOfStrings>,
    /// The absolute position bounding box.
    bounds: Vec<Rectangle>,
    /// Contents of the LayoutText, if any.
    text: Vec<StringIndex>,
    /// Stacking context information.
    #[serde(rename = "stackingContexts")]
    stacking_contexts: RareBooleanData,
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// captureSnapshot was true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "paintOrders")]
    paint_orders: Option<Vec<i64>>,
    /// The offset rect of nodes. Only available when includeDOMRects is set to true
    #[serde(skip_serializing_if = "Option::is_none", rename = "offsetRects")]
    offset_rects: Option<Vec<Rectangle>>,
    /// The scroll rect of nodes. Only available when includeDOMRects is set to true
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollRects")]
    scroll_rects: Option<Vec<Rectangle>>,
    /// The client rect of nodes. Only available when includeDOMRects is set to true
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientRects")]
    client_rects: Option<Vec<Rectangle>>,
    /// The list of background colors that are blended with colors of overlapping elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "blendedBackgroundColors")]
    blended_background_colors: Option<Vec<StringIndex>>,
    /// The list of computed text opacities.
    #[serde(skip_serializing_if = "Option::is_none", rename = "textColorOpacities")]
    text_color_opacities: Option<Vec<f64>>,
}

impl LayoutTreeSnapshot {
    /// Creates a builder for this type with the required parameters:
    /// * `node_index`: Index of the corresponding node in the `NodeTreeSnapshot` array returned by `captureSnapshot`.
    /// * `styles`: Array of indexes specifying computed style strings, filtered according to the `computedStyles` parameter passed to `captureSnapshot`.
    /// * `bounds`: The absolute position bounding box.
    /// * `text`: Contents of the LayoutText, if any.
    /// * `stacking_contexts`: Stacking context information.
    pub fn builder(node_index: Vec<i64>, styles: Vec<ArrayOfStrings>, bounds: Vec<Rectangle>, text: Vec<StringIndex>, stacking_contexts: RareBooleanData) -> LayoutTreeSnapshotBuilder {
        LayoutTreeSnapshotBuilder {
            node_index: node_index,
            styles: styles,
            bounds: bounds,
            text: text,
            stacking_contexts: stacking_contexts,
            paint_orders: None,
            offset_rects: None,
            scroll_rects: None,
            client_rects: None,
            blended_background_colors: None,
            text_color_opacities: None,
        }
    }
    /// Index of the corresponding node in the 'NodeTreeSnapshot' array returned by 'captureSnapshot'.
    pub fn node_index(&self) -> &[i64] { &self.node_index }
    /// Array of indexes specifying computed style strings, filtered according to the 'computedStyles' parameter passed to 'captureSnapshot'.
    pub fn styles(&self) -> &[ArrayOfStrings] { &self.styles }
    /// The absolute position bounding box.
    pub fn bounds(&self) -> &[Rectangle] { &self.bounds }
    /// Contents of the LayoutText, if any.
    pub fn text(&self) -> &[StringIndex] { &self.text }
    /// Stacking context information.
    pub fn stacking_contexts(&self) -> &RareBooleanData { &self.stacking_contexts }
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// captureSnapshot was true.
    pub fn paint_orders(&self) -> Option<&[i64]> { self.paint_orders.as_deref() }
    /// The offset rect of nodes. Only available when includeDOMRects is set to true
    pub fn offset_rects(&self) -> Option<&[Rectangle]> { self.offset_rects.as_deref() }
    /// The scroll rect of nodes. Only available when includeDOMRects is set to true
    pub fn scroll_rects(&self) -> Option<&[Rectangle]> { self.scroll_rects.as_deref() }
    /// The client rect of nodes. Only available when includeDOMRects is set to true
    pub fn client_rects(&self) -> Option<&[Rectangle]> { self.client_rects.as_deref() }
    /// The list of background colors that are blended with colors of overlapping elements.
    pub fn blended_background_colors(&self) -> Option<&[StringIndex]> { self.blended_background_colors.as_deref() }
    /// The list of computed text opacities.
    pub fn text_color_opacities(&self) -> Option<&[f64]> { self.text_color_opacities.as_deref() }
}


pub struct LayoutTreeSnapshotBuilder {
    node_index: Vec<i64>,
    styles: Vec<ArrayOfStrings>,
    bounds: Vec<Rectangle>,
    text: Vec<StringIndex>,
    stacking_contexts: RareBooleanData,
    paint_orders: Option<Vec<i64>>,
    offset_rects: Option<Vec<Rectangle>>,
    scroll_rects: Option<Vec<Rectangle>>,
    client_rects: Option<Vec<Rectangle>>,
    blended_background_colors: Option<Vec<StringIndex>>,
    text_color_opacities: Option<Vec<f64>>,
}

impl LayoutTreeSnapshotBuilder {
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// captureSnapshot was true.
    pub fn paint_orders(mut self, paint_orders: Vec<i64>) -> Self { self.paint_orders = Some(paint_orders); self }
    /// The offset rect of nodes. Only available when includeDOMRects is set to true
    pub fn offset_rects(mut self, offset_rects: Vec<Rectangle>) -> Self { self.offset_rects = Some(offset_rects); self }
    /// The scroll rect of nodes. Only available when includeDOMRects is set to true
    pub fn scroll_rects(mut self, scroll_rects: Vec<Rectangle>) -> Self { self.scroll_rects = Some(scroll_rects); self }
    /// The client rect of nodes. Only available when includeDOMRects is set to true
    pub fn client_rects(mut self, client_rects: Vec<Rectangle>) -> Self { self.client_rects = Some(client_rects); self }
    /// The list of background colors that are blended with colors of overlapping elements.
    pub fn blended_background_colors(mut self, blended_background_colors: Vec<StringIndex>) -> Self { self.blended_background_colors = Some(blended_background_colors); self }
    /// The list of computed text opacities.
    pub fn text_color_opacities(mut self, text_color_opacities: Vec<f64>) -> Self { self.text_color_opacities = Some(text_color_opacities); self }
    pub fn build(self) -> LayoutTreeSnapshot {
        LayoutTreeSnapshot {
            node_index: self.node_index,
            styles: self.styles,
            bounds: self.bounds,
            text: self.text,
            stacking_contexts: self.stacking_contexts,
            paint_orders: self.paint_orders,
            offset_rects: self.offset_rects,
            scroll_rects: self.scroll_rects,
            client_rects: self.client_rects,
            blended_background_colors: self.blended_background_colors,
            text_color_opacities: self.text_color_opacities,
        }
    }
}

/// Table of details of the post layout rendered text positions. The exact layout should not be regarded as
/// stable and may change between versions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextBoxSnapshot {
    /// Index of the layout tree node that owns this box collection.
    #[serde(rename = "layoutIndex")]
    layout_index: Vec<i64>,
    /// The absolute position bounding box.
    bounds: Vec<Rectangle>,
    /// The starting index in characters, for this post layout textbox substring. Characters that
    /// would be represented as a surrogate pair in UTF-16 have length 2.
    start: Vec<i64>,
    /// The number of characters in this post layout textbox substring. Characters that would be
    /// represented as a surrogate pair in UTF-16 have length 2.
    length: Vec<i64>,
}

impl TextBoxSnapshot {
    /// Creates a builder for this type with the required parameters:
    /// * `layout_index`: Index of the layout tree node that owns this box collection.
    /// * `bounds`: The absolute position bounding box.
    /// * `start`: The starting index in characters, for this post layout textbox substring. Characters that would be represented as a surrogate pair in UTF-16 have length 2.
    /// * `length`: The number of characters in this post layout textbox substring. Characters that would be represented as a surrogate pair in UTF-16 have length 2.
    pub fn builder(layout_index: Vec<i64>, bounds: Vec<Rectangle>, start: Vec<i64>, length: Vec<i64>) -> TextBoxSnapshotBuilder {
        TextBoxSnapshotBuilder {
            layout_index: layout_index,
            bounds: bounds,
            start: start,
            length: length,
        }
    }
    /// Index of the layout tree node that owns this box collection.
    pub fn layout_index(&self) -> &[i64] { &self.layout_index }
    /// The absolute position bounding box.
    pub fn bounds(&self) -> &[Rectangle] { &self.bounds }
    /// The starting index in characters, for this post layout textbox substring. Characters that
    /// would be represented as a surrogate pair in UTF-16 have length 2.
    pub fn start(&self) -> &[i64] { &self.start }
    /// The number of characters in this post layout textbox substring. Characters that would be
    /// represented as a surrogate pair in UTF-16 have length 2.
    pub fn length(&self) -> &[i64] { &self.length }
}


pub struct TextBoxSnapshotBuilder {
    layout_index: Vec<i64>,
    bounds: Vec<Rectangle>,
    start: Vec<i64>,
    length: Vec<i64>,
}

impl TextBoxSnapshotBuilder {
    pub fn build(self) -> TextBoxSnapshot {
        TextBoxSnapshot {
            layout_index: self.layout_index,
            bounds: self.bounds,
            start: self.start,
            length: self.length,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "DOMSnapshot.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "DOMSnapshot.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "DOMSnapshot.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "DOMSnapshot.enable";
    type Response = crate::EmptyReturns;
}

/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSnapshotParams<'a> {
    /// Whitelist of computed styles to return.
    #[serde(rename = "computedStyleWhitelist")]
    computed_style_whitelist: Vec<Cow<'a, str>>,
    /// Whether or not to retrieve details of DOM listeners (default false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeEventListeners")]
    include_event_listeners: Option<bool>,
    /// Whether to determine and include the paint order index of LayoutTreeNodes (default false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "includePaintOrder")]
    include_paint_order: Option<bool>,
    /// Whether to include UA shadow tree in the snapshot (default false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeUserAgentShadowTree")]
    include_user_agent_shadow_tree: Option<bool>,
}

impl<'a> GetSnapshotParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `computed_style_whitelist`: Whitelist of computed styles to return.
    pub fn builder(computed_style_whitelist: Vec<Cow<'a, str>>) -> GetSnapshotParamsBuilder<'a> {
        GetSnapshotParamsBuilder {
            computed_style_whitelist: computed_style_whitelist,
            include_event_listeners: None,
            include_paint_order: None,
            include_user_agent_shadow_tree: None,
        }
    }
    /// Whitelist of computed styles to return.
    pub fn computed_style_whitelist(&self) -> &[Cow<'a, str>] { &self.computed_style_whitelist }
    /// Whether or not to retrieve details of DOM listeners (default false).
    pub fn include_event_listeners(&self) -> Option<bool> { self.include_event_listeners }
    /// Whether to determine and include the paint order index of LayoutTreeNodes (default false).
    pub fn include_paint_order(&self) -> Option<bool> { self.include_paint_order }
    /// Whether to include UA shadow tree in the snapshot (default false).
    pub fn include_user_agent_shadow_tree(&self) -> Option<bool> { self.include_user_agent_shadow_tree }
}


pub struct GetSnapshotParamsBuilder<'a> {
    computed_style_whitelist: Vec<Cow<'a, str>>,
    include_event_listeners: Option<bool>,
    include_paint_order: Option<bool>,
    include_user_agent_shadow_tree: Option<bool>,
}

impl<'a> GetSnapshotParamsBuilder<'a> {
    /// Whether or not to retrieve details of DOM listeners (default false).
    pub fn include_event_listeners(mut self, include_event_listeners: bool) -> Self { self.include_event_listeners = Some(include_event_listeners); self }
    /// Whether to determine and include the paint order index of LayoutTreeNodes (default false).
    pub fn include_paint_order(mut self, include_paint_order: bool) -> Self { self.include_paint_order = Some(include_paint_order); self }
    /// Whether to include UA shadow tree in the snapshot (default false).
    pub fn include_user_agent_shadow_tree(mut self, include_user_agent_shadow_tree: bool) -> Self { self.include_user_agent_shadow_tree = Some(include_user_agent_shadow_tree); self }
    pub fn build(self) -> GetSnapshotParams<'a> {
        GetSnapshotParams {
            computed_style_whitelist: self.computed_style_whitelist,
            include_event_listeners: self.include_event_listeners,
            include_paint_order: self.include_paint_order,
            include_user_agent_shadow_tree: self.include_user_agent_shadow_tree,
        }
    }
}

/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSnapshotReturns<'a> {
    /// The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.
    #[serde(rename = "domNodes")]
    dom_nodes: Vec<DOMNode<'a>>,
    /// The nodes in the layout tree.
    #[serde(rename = "layoutTreeNodes")]
    layout_tree_nodes: Vec<LayoutTreeNode<'a>>,
    /// Whitelisted ComputedStyle properties for each node in the layout tree.
    #[serde(rename = "computedStyles")]
    computed_styles: Vec<ComputedStyle<'a>>,
}

impl<'a> GetSnapshotReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `dom_nodes`: The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.
    /// * `layout_tree_nodes`: The nodes in the layout tree.
    /// * `computed_styles`: Whitelisted ComputedStyle properties for each node in the layout tree.
    pub fn builder(dom_nodes: Vec<DOMNode<'a>>, layout_tree_nodes: Vec<LayoutTreeNode<'a>>, computed_styles: Vec<ComputedStyle<'a>>) -> GetSnapshotReturnsBuilder<'a> {
        GetSnapshotReturnsBuilder {
            dom_nodes: dom_nodes,
            layout_tree_nodes: layout_tree_nodes,
            computed_styles: computed_styles,
        }
    }
    /// The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.
    pub fn dom_nodes(&self) -> &[DOMNode<'a>] { &self.dom_nodes }
    /// The nodes in the layout tree.
    pub fn layout_tree_nodes(&self) -> &[LayoutTreeNode<'a>] { &self.layout_tree_nodes }
    /// Whitelisted ComputedStyle properties for each node in the layout tree.
    pub fn computed_styles(&self) -> &[ComputedStyle<'a>] { &self.computed_styles }
}


pub struct GetSnapshotReturnsBuilder<'a> {
    dom_nodes: Vec<DOMNode<'a>>,
    layout_tree_nodes: Vec<LayoutTreeNode<'a>>,
    computed_styles: Vec<ComputedStyle<'a>>,
}

impl<'a> GetSnapshotReturnsBuilder<'a> {
    pub fn build(self) -> GetSnapshotReturns<'a> {
        GetSnapshotReturns {
            dom_nodes: self.dom_nodes,
            layout_tree_nodes: self.layout_tree_nodes,
            computed_styles: self.computed_styles,
        }
    }
}

impl<'a> GetSnapshotParams<'a> { pub const METHOD: &'static str = "DOMSnapshot.getSnapshot"; }

impl<'a> crate::CdpCommand<'a> for GetSnapshotParams<'a> {
    const METHOD: &'static str = "DOMSnapshot.getSnapshot";
    type Response = GetSnapshotReturns<'a>;
}

/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotParams<'a> {
    /// Whitelist of computed styles to return.
    #[serde(rename = "computedStyles")]
    computed_styles: Vec<Cow<'a, str>>,
    /// Whether to include layout object paint orders into the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includePaintOrder")]
    include_paint_order: Option<bool>,
    /// Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeDOMRects")]
    include_dom_rects: Option<bool>,
    /// Whether to include blended background colors in the snapshot (default: false).
    /// Blended background color is achieved by blending background colors of all elements
    /// that overlap with the current element.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeBlendedBackgroundColors")]
    include_blended_background_colors: Option<bool>,
    /// Whether to include text color opacity in the snapshot (default: false).
    /// An element might have the opacity property set that affects the text color of the element.
    /// The final text color opacity is computed based on the opacity of all overlapping elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeTextColorOpacities")]
    include_text_color_opacities: Option<bool>,
}

impl<'a> CaptureSnapshotParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `computed_styles`: Whitelist of computed styles to return.
    pub fn builder(computed_styles: Vec<Cow<'a, str>>) -> CaptureSnapshotParamsBuilder<'a> {
        CaptureSnapshotParamsBuilder {
            computed_styles: computed_styles,
            include_paint_order: None,
            include_dom_rects: None,
            include_blended_background_colors: None,
            include_text_color_opacities: None,
        }
    }
    /// Whitelist of computed styles to return.
    pub fn computed_styles(&self) -> &[Cow<'a, str>] { &self.computed_styles }
    /// Whether to include layout object paint orders into the snapshot.
    pub fn include_paint_order(&self) -> Option<bool> { self.include_paint_order }
    /// Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot
    pub fn include_dom_rects(&self) -> Option<bool> { self.include_dom_rects }
    /// Whether to include blended background colors in the snapshot (default: false).
    /// Blended background color is achieved by blending background colors of all elements
    /// that overlap with the current element.
    pub fn include_blended_background_colors(&self) -> Option<bool> { self.include_blended_background_colors }
    /// Whether to include text color opacity in the snapshot (default: false).
    /// An element might have the opacity property set that affects the text color of the element.
    /// The final text color opacity is computed based on the opacity of all overlapping elements.
    pub fn include_text_color_opacities(&self) -> Option<bool> { self.include_text_color_opacities }
}


pub struct CaptureSnapshotParamsBuilder<'a> {
    computed_styles: Vec<Cow<'a, str>>,
    include_paint_order: Option<bool>,
    include_dom_rects: Option<bool>,
    include_blended_background_colors: Option<bool>,
    include_text_color_opacities: Option<bool>,
}

impl<'a> CaptureSnapshotParamsBuilder<'a> {
    /// Whether to include layout object paint orders into the snapshot.
    pub fn include_paint_order(mut self, include_paint_order: bool) -> Self { self.include_paint_order = Some(include_paint_order); self }
    /// Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot
    pub fn include_dom_rects(mut self, include_dom_rects: bool) -> Self { self.include_dom_rects = Some(include_dom_rects); self }
    /// Whether to include blended background colors in the snapshot (default: false).
    /// Blended background color is achieved by blending background colors of all elements
    /// that overlap with the current element.
    pub fn include_blended_background_colors(mut self, include_blended_background_colors: bool) -> Self { self.include_blended_background_colors = Some(include_blended_background_colors); self }
    /// Whether to include text color opacity in the snapshot (default: false).
    /// An element might have the opacity property set that affects the text color of the element.
    /// The final text color opacity is computed based on the opacity of all overlapping elements.
    pub fn include_text_color_opacities(mut self, include_text_color_opacities: bool) -> Self { self.include_text_color_opacities = Some(include_text_color_opacities); self }
    pub fn build(self) -> CaptureSnapshotParams<'a> {
        CaptureSnapshotParams {
            computed_styles: self.computed_styles,
            include_paint_order: self.include_paint_order,
            include_dom_rects: self.include_dom_rects,
            include_blended_background_colors: self.include_blended_background_colors,
            include_text_color_opacities: self.include_text_color_opacities,
        }
    }
}

/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotReturns<'a> {
    /// The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.
    documents: Vec<DocumentSnapshot>,
    /// Shared string table that all string properties refer to with indexes.
    strings: Vec<Cow<'a, str>>,
}

impl<'a> CaptureSnapshotReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `documents`: The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.
    /// * `strings`: Shared string table that all string properties refer to with indexes.
    pub fn builder(documents: Vec<DocumentSnapshot>, strings: Vec<Cow<'a, str>>) -> CaptureSnapshotReturnsBuilder<'a> {
        CaptureSnapshotReturnsBuilder {
            documents: documents,
            strings: strings,
        }
    }
    /// The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.
    pub fn documents(&self) -> &[DocumentSnapshot] { &self.documents }
    /// Shared string table that all string properties refer to with indexes.
    pub fn strings(&self) -> &[Cow<'a, str>] { &self.strings }
}


pub struct CaptureSnapshotReturnsBuilder<'a> {
    documents: Vec<DocumentSnapshot>,
    strings: Vec<Cow<'a, str>>,
}

impl<'a> CaptureSnapshotReturnsBuilder<'a> {
    pub fn build(self) -> CaptureSnapshotReturns<'a> {
        CaptureSnapshotReturns {
            documents: self.documents,
            strings: self.strings,
        }
    }
}

impl<'a> CaptureSnapshotParams<'a> { pub const METHOD: &'static str = "DOMSnapshot.captureSnapshot"; }

impl<'a> crate::CdpCommand<'a> for CaptureSnapshotParams<'a> {
    const METHOD: &'static str = "DOMSnapshot.captureSnapshot";
    type Response = CaptureSnapshotReturns<'a>;
}
