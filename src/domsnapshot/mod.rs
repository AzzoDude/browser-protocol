//! This domain facilitates obtaining document snapshots with DOM, layout, and style information.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// A Node in the DOM tree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DOMNode<'a> {
    /// 'Node''s nodeType.
    nodeType: i64,
    /// 'Node''s nodeName.
    nodeName: Cow<'a, str>,
    /// 'Node''s nodeValue.
    nodeValue: Cow<'a, str>,
    /// Only set for textarea elements, contains the text value.
    #[serde(skip_serializing_if = "Option::is_none")]
    textValue: Option<Cow<'a, str>>,
    /// Only set for input elements, contains the input's associated text value.
    #[serde(skip_serializing_if = "Option::is_none")]
    inputValue: Option<Cow<'a, str>>,
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    #[serde(skip_serializing_if = "Option::is_none")]
    inputChecked: Option<bool>,
    /// Only set for option elements, indicates if the element has been selected
    #[serde(skip_serializing_if = "Option::is_none")]
    optionSelected: Option<bool>,
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    backendNodeId: crate::dom::BackendNodeId,
    /// The indexes of the node's child nodes in the 'domNodes' array returned by 'getSnapshot', if
    /// any.
    #[serde(skip_serializing_if = "Option::is_none")]
    childNodeIndexes: Option<Vec<i64>>,
    /// Attributes of an 'Element' node.
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<NameValue<'a>>>,
    /// Indexes of pseudo elements associated with this node in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoElementIndexes: Option<Vec<i64>>,
    /// The index of the node's related layout tree node in the 'layoutTreeNodes' array returned by
    /// 'getSnapshot', if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    layoutNodeIndex: Option<u64>,
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    #[serde(skip_serializing_if = "Option::is_none")]
    documentURL: Option<Cow<'a, str>>,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    baseURL: Option<Cow<'a, str>>,
    /// Only set for documents, contains the document's content language.
    #[serde(skip_serializing_if = "Option::is_none")]
    contentLanguage: Option<Cow<'a, str>>,
    /// Only set for documents, contains the document's character set encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    documentEncoding: Option<Cow<'a, str>>,
    /// 'DocumentType' node's publicId.
    #[serde(skip_serializing_if = "Option::is_none")]
    publicId: Option<Cow<'a, str>>,
    /// 'DocumentType' node's systemId.
    #[serde(skip_serializing_if = "Option::is_none")]
    systemId: Option<Cow<'a, str>>,
    /// Frame ID for frame owner elements and also for the document node.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
    /// The index of a frame owner element's content document in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    contentDocumentIndex: Option<u64>,
    /// Type of a pseudo element node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoType: Option<crate::dom::PseudoType>,
    /// Shadow root type.
    #[serde(skip_serializing_if = "Option::is_none")]
    shadowRootType: Option<crate::dom::ShadowRootType>,
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    isClickable: Option<bool>,
    /// Details of the node's event listeners, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    eventListeners: Option<Vec<crate::domdebugger::EventListener<'a>>>,
    /// The selected url for nodes with a srcset attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    currentSourceURL: Option<Cow<'a, str>>,
    /// The url of the script (if any) that generates this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    originURL: Option<Cow<'a, str>>,
    /// Scroll offsets, set when this node is a Document.
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollOffsetX: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollOffsetY: Option<f64>,
}

impl<'a> DOMNode<'a> {
    pub fn builder(nodeType: i64, nodeName: impl Into<Cow<'a, str>>, nodeValue: impl Into<Cow<'a, str>>, backendNodeId: crate::dom::BackendNodeId) -> DOMNodeBuilder<'a> {
        DOMNodeBuilder {
            nodeType: nodeType,
            nodeName: nodeName.into(),
            nodeValue: nodeValue.into(),
            textValue: None,
            inputValue: None,
            inputChecked: None,
            optionSelected: None,
            backendNodeId: backendNodeId,
            childNodeIndexes: None,
            attributes: None,
            pseudoElementIndexes: None,
            layoutNodeIndex: None,
            documentURL: None,
            baseURL: None,
            contentLanguage: None,
            documentEncoding: None,
            publicId: None,
            systemId: None,
            frameId: None,
            contentDocumentIndex: None,
            pseudoType: None,
            shadowRootType: None,
            isClickable: None,
            eventListeners: None,
            currentSourceURL: None,
            originURL: None,
            scrollOffsetX: None,
            scrollOffsetY: None,
        }
    }
    pub fn nodeType(&self) -> i64 { self.nodeType }
    pub fn nodeName(&self) -> &str { self.nodeName.as_ref() }
    pub fn nodeValue(&self) -> &str { self.nodeValue.as_ref() }
    pub fn textValue(&self) -> Option<&str> { self.textValue.as_deref() }
    pub fn inputValue(&self) -> Option<&str> { self.inputValue.as_deref() }
    pub fn inputChecked(&self) -> Option<bool> { self.inputChecked }
    pub fn optionSelected(&self) -> Option<bool> { self.optionSelected }
    pub fn backendNodeId(&self) -> &crate::dom::BackendNodeId { &self.backendNodeId }
    pub fn childNodeIndexes(&self) -> Option<&[i64]> { self.childNodeIndexes.as_deref() }
    pub fn attributes(&self) -> Option<&[NameValue<'a>]> { self.attributes.as_deref() }
    pub fn pseudoElementIndexes(&self) -> Option<&[i64]> { self.pseudoElementIndexes.as_deref() }
    pub fn layoutNodeIndex(&self) -> Option<u64> { self.layoutNodeIndex }
    pub fn documentURL(&self) -> Option<&str> { self.documentURL.as_deref() }
    pub fn baseURL(&self) -> Option<&str> { self.baseURL.as_deref() }
    pub fn contentLanguage(&self) -> Option<&str> { self.contentLanguage.as_deref() }
    pub fn documentEncoding(&self) -> Option<&str> { self.documentEncoding.as_deref() }
    pub fn publicId(&self) -> Option<&str> { self.publicId.as_deref() }
    pub fn systemId(&self) -> Option<&str> { self.systemId.as_deref() }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
    pub fn contentDocumentIndex(&self) -> Option<u64> { self.contentDocumentIndex }
    pub fn pseudoType(&self) -> Option<&crate::dom::PseudoType> { self.pseudoType.as_ref() }
    pub fn shadowRootType(&self) -> Option<&crate::dom::ShadowRootType> { self.shadowRootType.as_ref() }
    pub fn isClickable(&self) -> Option<bool> { self.isClickable }
    pub fn eventListeners(&self) -> Option<&[crate::domdebugger::EventListener<'a>]> { self.eventListeners.as_deref() }
    pub fn currentSourceURL(&self) -> Option<&str> { self.currentSourceURL.as_deref() }
    pub fn originURL(&self) -> Option<&str> { self.originURL.as_deref() }
    pub fn scrollOffsetX(&self) -> Option<f64> { self.scrollOffsetX }
    pub fn scrollOffsetY(&self) -> Option<f64> { self.scrollOffsetY }
}


pub struct DOMNodeBuilder<'a> {
    nodeType: i64,
    nodeName: Cow<'a, str>,
    nodeValue: Cow<'a, str>,
    textValue: Option<Cow<'a, str>>,
    inputValue: Option<Cow<'a, str>>,
    inputChecked: Option<bool>,
    optionSelected: Option<bool>,
    backendNodeId: crate::dom::BackendNodeId,
    childNodeIndexes: Option<Vec<i64>>,
    attributes: Option<Vec<NameValue<'a>>>,
    pseudoElementIndexes: Option<Vec<i64>>,
    layoutNodeIndex: Option<u64>,
    documentURL: Option<Cow<'a, str>>,
    baseURL: Option<Cow<'a, str>>,
    contentLanguage: Option<Cow<'a, str>>,
    documentEncoding: Option<Cow<'a, str>>,
    publicId: Option<Cow<'a, str>>,
    systemId: Option<Cow<'a, str>>,
    frameId: Option<crate::page::FrameId<'a>>,
    contentDocumentIndex: Option<u64>,
    pseudoType: Option<crate::dom::PseudoType>,
    shadowRootType: Option<crate::dom::ShadowRootType>,
    isClickable: Option<bool>,
    eventListeners: Option<Vec<crate::domdebugger::EventListener<'a>>>,
    currentSourceURL: Option<Cow<'a, str>>,
    originURL: Option<Cow<'a, str>>,
    scrollOffsetX: Option<f64>,
    scrollOffsetY: Option<f64>,
}

impl<'a> DOMNodeBuilder<'a> {
    /// Only set for textarea elements, contains the text value.
    pub fn textValue(mut self, textValue: impl Into<Cow<'a, str>>) -> Self { self.textValue = Some(textValue.into()); self }
    /// Only set for input elements, contains the input's associated text value.
    pub fn inputValue(mut self, inputValue: impl Into<Cow<'a, str>>) -> Self { self.inputValue = Some(inputValue.into()); self }
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    pub fn inputChecked(mut self, inputChecked: bool) -> Self { self.inputChecked = Some(inputChecked); self }
    /// Only set for option elements, indicates if the element has been selected
    pub fn optionSelected(mut self, optionSelected: bool) -> Self { self.optionSelected = Some(optionSelected); self }
    /// The indexes of the node's child nodes in the 'domNodes' array returned by 'getSnapshot', if
    /// any.
    pub fn childNodeIndexes(mut self, childNodeIndexes: Vec<i64>) -> Self { self.childNodeIndexes = Some(childNodeIndexes); self }
    /// Attributes of an 'Element' node.
    pub fn attributes(mut self, attributes: Vec<NameValue<'a>>) -> Self { self.attributes = Some(attributes); self }
    /// Indexes of pseudo elements associated with this node in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    pub fn pseudoElementIndexes(mut self, pseudoElementIndexes: Vec<i64>) -> Self { self.pseudoElementIndexes = Some(pseudoElementIndexes); self }
    /// The index of the node's related layout tree node in the 'layoutTreeNodes' array returned by
    /// 'getSnapshot', if any.
    pub fn layoutNodeIndex(mut self, layoutNodeIndex: u64) -> Self { self.layoutNodeIndex = Some(layoutNodeIndex); self }
    /// Document URL that 'Document' or 'FrameOwner' node points to.
    pub fn documentURL(mut self, documentURL: impl Into<Cow<'a, str>>) -> Self { self.documentURL = Some(documentURL.into()); self }
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    pub fn baseURL(mut self, baseURL: impl Into<Cow<'a, str>>) -> Self { self.baseURL = Some(baseURL.into()); self }
    /// Only set for documents, contains the document's content language.
    pub fn contentLanguage(mut self, contentLanguage: impl Into<Cow<'a, str>>) -> Self { self.contentLanguage = Some(contentLanguage.into()); self }
    /// Only set for documents, contains the document's character set encoding.
    pub fn documentEncoding(mut self, documentEncoding: impl Into<Cow<'a, str>>) -> Self { self.documentEncoding = Some(documentEncoding.into()); self }
    /// 'DocumentType' node's publicId.
    pub fn publicId(mut self, publicId: impl Into<Cow<'a, str>>) -> Self { self.publicId = Some(publicId.into()); self }
    /// 'DocumentType' node's systemId.
    pub fn systemId(mut self, systemId: impl Into<Cow<'a, str>>) -> Self { self.systemId = Some(systemId.into()); self }
    /// Frame ID for frame owner elements and also for the document node.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// The index of a frame owner element's content document in the 'domNodes' array returned by
    /// 'getSnapshot', if any.
    pub fn contentDocumentIndex(mut self, contentDocumentIndex: u64) -> Self { self.contentDocumentIndex = Some(contentDocumentIndex); self }
    /// Type of a pseudo element node.
    pub fn pseudoType(mut self, pseudoType: crate::dom::PseudoType) -> Self { self.pseudoType = Some(pseudoType); self }
    /// Shadow root type.
    pub fn shadowRootType(mut self, shadowRootType: crate::dom::ShadowRootType) -> Self { self.shadowRootType = Some(shadowRootType); self }
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    pub fn isClickable(mut self, isClickable: bool) -> Self { self.isClickable = Some(isClickable); self }
    /// Details of the node's event listeners, if any.
    pub fn eventListeners(mut self, eventListeners: Vec<crate::domdebugger::EventListener<'a>>) -> Self { self.eventListeners = Some(eventListeners); self }
    /// The selected url for nodes with a srcset attribute.
    pub fn currentSourceURL(mut self, currentSourceURL: impl Into<Cow<'a, str>>) -> Self { self.currentSourceURL = Some(currentSourceURL.into()); self }
    /// The url of the script (if any) that generates this node.
    pub fn originURL(mut self, originURL: impl Into<Cow<'a, str>>) -> Self { self.originURL = Some(originURL.into()); self }
    /// Scroll offsets, set when this node is a Document.
    pub fn scrollOffsetX(mut self, scrollOffsetX: f64) -> Self { self.scrollOffsetX = Some(scrollOffsetX); self }
    pub fn scrollOffsetY(mut self, scrollOffsetY: f64) -> Self { self.scrollOffsetY = Some(scrollOffsetY); self }
    pub fn build(self) -> DOMNode<'a> {
        DOMNode {
            nodeType: self.nodeType,
            nodeName: self.nodeName,
            nodeValue: self.nodeValue,
            textValue: self.textValue,
            inputValue: self.inputValue,
            inputChecked: self.inputChecked,
            optionSelected: self.optionSelected,
            backendNodeId: self.backendNodeId,
            childNodeIndexes: self.childNodeIndexes,
            attributes: self.attributes,
            pseudoElementIndexes: self.pseudoElementIndexes,
            layoutNodeIndex: self.layoutNodeIndex,
            documentURL: self.documentURL,
            baseURL: self.baseURL,
            contentLanguage: self.contentLanguage,
            documentEncoding: self.documentEncoding,
            publicId: self.publicId,
            systemId: self.systemId,
            frameId: self.frameId,
            contentDocumentIndex: self.contentDocumentIndex,
            pseudoType: self.pseudoType,
            shadowRootType: self.shadowRootType,
            isClickable: self.isClickable,
            eventListeners: self.eventListeners,
            currentSourceURL: self.currentSourceURL,
            originURL: self.originURL,
            scrollOffsetX: self.scrollOffsetX,
            scrollOffsetY: self.scrollOffsetY,
        }
    }
}

/// Details of post layout rendered text positions. The exact layout should not be regarded as
/// stable and may change between versions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InlineTextBox {
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    boundingBox: crate::dom::Rect,
    /// The starting index in characters, for this post layout textbox substring. Characters that
    /// would be represented as a surrogate pair in UTF-16 have length 2.
    startCharacterIndex: u64,
    /// The number of characters in this post layout textbox substring. Characters that would be
    /// represented as a surrogate pair in UTF-16 have length 2.
    numCharacters: i64,
}

impl InlineTextBox {
    pub fn builder(boundingBox: crate::dom::Rect, startCharacterIndex: u64, numCharacters: i64) -> InlineTextBoxBuilder {
        InlineTextBoxBuilder {
            boundingBox: boundingBox,
            startCharacterIndex: startCharacterIndex,
            numCharacters: numCharacters,
        }
    }
    pub fn boundingBox(&self) -> &crate::dom::Rect { &self.boundingBox }
    pub fn startCharacterIndex(&self) -> u64 { self.startCharacterIndex }
    pub fn numCharacters(&self) -> i64 { self.numCharacters }
}


pub struct InlineTextBoxBuilder {
    boundingBox: crate::dom::Rect,
    startCharacterIndex: u64,
    numCharacters: i64,
}

impl InlineTextBoxBuilder {
    pub fn build(self) -> InlineTextBox {
        InlineTextBox {
            boundingBox: self.boundingBox,
            startCharacterIndex: self.startCharacterIndex,
            numCharacters: self.numCharacters,
        }
    }
}

/// Details of an element in the DOM tree with a LayoutObject.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutTreeNode<'a> {
    /// The index of the related DOM node in the 'domNodes' array returned by 'getSnapshot'.
    domNodeIndex: u64,
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.
    boundingBox: crate::dom::Rect,
    /// Contents of the LayoutText, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    layoutText: Option<Cow<'a, str>>,
    /// The post-layout inline text nodes, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    inlineTextNodes: Option<Vec<InlineTextBox>>,
    /// Index into the 'computedStyles' array returned by 'getSnapshot'.
    #[serde(skip_serializing_if = "Option::is_none")]
    styleIndex: Option<u64>,
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// getSnapshot was true.
    #[serde(skip_serializing_if = "Option::is_none")]
    paintOrder: Option<i64>,
    /// Set to true to indicate the element begins a new stacking context.
    #[serde(skip_serializing_if = "Option::is_none")]
    isStackingContext: Option<bool>,
}

impl<'a> LayoutTreeNode<'a> {
    pub fn builder(domNodeIndex: u64, boundingBox: crate::dom::Rect) -> LayoutTreeNodeBuilder<'a> {
        LayoutTreeNodeBuilder {
            domNodeIndex: domNodeIndex,
            boundingBox: boundingBox,
            layoutText: None,
            inlineTextNodes: None,
            styleIndex: None,
            paintOrder: None,
            isStackingContext: None,
        }
    }
    pub fn domNodeIndex(&self) -> u64 { self.domNodeIndex }
    pub fn boundingBox(&self) -> &crate::dom::Rect { &self.boundingBox }
    pub fn layoutText(&self) -> Option<&str> { self.layoutText.as_deref() }
    pub fn inlineTextNodes(&self) -> Option<&[InlineTextBox]> { self.inlineTextNodes.as_deref() }
    pub fn styleIndex(&self) -> Option<u64> { self.styleIndex }
    pub fn paintOrder(&self) -> Option<i64> { self.paintOrder }
    pub fn isStackingContext(&self) -> Option<bool> { self.isStackingContext }
}


pub struct LayoutTreeNodeBuilder<'a> {
    domNodeIndex: u64,
    boundingBox: crate::dom::Rect,
    layoutText: Option<Cow<'a, str>>,
    inlineTextNodes: Option<Vec<InlineTextBox>>,
    styleIndex: Option<u64>,
    paintOrder: Option<i64>,
    isStackingContext: Option<bool>,
}

impl<'a> LayoutTreeNodeBuilder<'a> {
    /// Contents of the LayoutText, if any.
    pub fn layoutText(mut self, layoutText: impl Into<Cow<'a, str>>) -> Self { self.layoutText = Some(layoutText.into()); self }
    /// The post-layout inline text nodes, if any.
    pub fn inlineTextNodes(mut self, inlineTextNodes: Vec<InlineTextBox>) -> Self { self.inlineTextNodes = Some(inlineTextNodes); self }
    /// Index into the 'computedStyles' array returned by 'getSnapshot'.
    pub fn styleIndex(mut self, styleIndex: u64) -> Self { self.styleIndex = Some(styleIndex); self }
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// getSnapshot was true.
    pub fn paintOrder(mut self, paintOrder: i64) -> Self { self.paintOrder = Some(paintOrder); self }
    /// Set to true to indicate the element begins a new stacking context.
    pub fn isStackingContext(mut self, isStackingContext: bool) -> Self { self.isStackingContext = Some(isStackingContext); self }
    pub fn build(self) -> LayoutTreeNode<'a> {
        LayoutTreeNode {
            domNodeIndex: self.domNodeIndex,
            boundingBox: self.boundingBox,
            layoutText: self.layoutText,
            inlineTextNodes: self.inlineTextNodes,
            styleIndex: self.styleIndex,
            paintOrder: self.paintOrder,
            isStackingContext: self.isStackingContext,
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
    pub fn builder(properties: Vec<NameValue<'a>>) -> ComputedStyleBuilder<'a> {
        ComputedStyleBuilder {
            properties: properties,
        }
    }
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
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> NameValueBuilder<'a> {
        NameValueBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
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
    documentURL: StringIndex,
    /// Document title.
    title: StringIndex,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.
    baseURL: StringIndex,
    /// Contains the document's content language.
    contentLanguage: StringIndex,
    /// Contains the document's character set encoding.
    encodingName: StringIndex,
    /// 'DocumentType' node's publicId.
    publicId: StringIndex,
    /// 'DocumentType' node's systemId.
    systemId: StringIndex,
    /// Frame ID for frame owner elements and also for the document node.
    frameId: StringIndex,
    /// A table with dom nodes.
    nodes: NodeTreeSnapshot,
    /// The nodes in the layout tree.
    layout: LayoutTreeSnapshot,
    /// The post-layout inline text nodes.
    textBoxes: TextBoxSnapshot,
    /// Horizontal scroll offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollOffsetX: Option<f64>,
    /// Vertical scroll offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollOffsetY: Option<f64>,
    /// Document content width.
    #[serde(skip_serializing_if = "Option::is_none")]
    contentWidth: Option<f64>,
    /// Document content height.
    #[serde(skip_serializing_if = "Option::is_none")]
    contentHeight: Option<f64>,
}

impl DocumentSnapshot {
    pub fn builder(documentURL: StringIndex, title: StringIndex, baseURL: StringIndex, contentLanguage: StringIndex, encodingName: StringIndex, publicId: StringIndex, systemId: StringIndex, frameId: StringIndex, nodes: NodeTreeSnapshot, layout: LayoutTreeSnapshot, textBoxes: TextBoxSnapshot) -> DocumentSnapshotBuilder {
        DocumentSnapshotBuilder {
            documentURL: documentURL,
            title: title,
            baseURL: baseURL,
            contentLanguage: contentLanguage,
            encodingName: encodingName,
            publicId: publicId,
            systemId: systemId,
            frameId: frameId,
            nodes: nodes,
            layout: layout,
            textBoxes: textBoxes,
            scrollOffsetX: None,
            scrollOffsetY: None,
            contentWidth: None,
            contentHeight: None,
        }
    }
    pub fn documentURL(&self) -> &StringIndex { &self.documentURL }
    pub fn title(&self) -> &StringIndex { &self.title }
    pub fn baseURL(&self) -> &StringIndex { &self.baseURL }
    pub fn contentLanguage(&self) -> &StringIndex { &self.contentLanguage }
    pub fn encodingName(&self) -> &StringIndex { &self.encodingName }
    pub fn publicId(&self) -> &StringIndex { &self.publicId }
    pub fn systemId(&self) -> &StringIndex { &self.systemId }
    pub fn frameId(&self) -> &StringIndex { &self.frameId }
    pub fn nodes(&self) -> &NodeTreeSnapshot { &self.nodes }
    pub fn layout(&self) -> &LayoutTreeSnapshot { &self.layout }
    pub fn textBoxes(&self) -> &TextBoxSnapshot { &self.textBoxes }
    pub fn scrollOffsetX(&self) -> Option<f64> { self.scrollOffsetX }
    pub fn scrollOffsetY(&self) -> Option<f64> { self.scrollOffsetY }
    pub fn contentWidth(&self) -> Option<f64> { self.contentWidth }
    pub fn contentHeight(&self) -> Option<f64> { self.contentHeight }
}


pub struct DocumentSnapshotBuilder {
    documentURL: StringIndex,
    title: StringIndex,
    baseURL: StringIndex,
    contentLanguage: StringIndex,
    encodingName: StringIndex,
    publicId: StringIndex,
    systemId: StringIndex,
    frameId: StringIndex,
    nodes: NodeTreeSnapshot,
    layout: LayoutTreeSnapshot,
    textBoxes: TextBoxSnapshot,
    scrollOffsetX: Option<f64>,
    scrollOffsetY: Option<f64>,
    contentWidth: Option<f64>,
    contentHeight: Option<f64>,
}

impl DocumentSnapshotBuilder {
    /// Horizontal scroll offset.
    pub fn scrollOffsetX(mut self, scrollOffsetX: f64) -> Self { self.scrollOffsetX = Some(scrollOffsetX); self }
    /// Vertical scroll offset.
    pub fn scrollOffsetY(mut self, scrollOffsetY: f64) -> Self { self.scrollOffsetY = Some(scrollOffsetY); self }
    /// Document content width.
    pub fn contentWidth(mut self, contentWidth: f64) -> Self { self.contentWidth = Some(contentWidth); self }
    /// Document content height.
    pub fn contentHeight(mut self, contentHeight: f64) -> Self { self.contentHeight = Some(contentHeight); self }
    pub fn build(self) -> DocumentSnapshot {
        DocumentSnapshot {
            documentURL: self.documentURL,
            title: self.title,
            baseURL: self.baseURL,
            contentLanguage: self.contentLanguage,
            encodingName: self.encodingName,
            publicId: self.publicId,
            systemId: self.systemId,
            frameId: self.frameId,
            nodes: self.nodes,
            layout: self.layout,
            textBoxes: self.textBoxes,
            scrollOffsetX: self.scrollOffsetX,
            scrollOffsetY: self.scrollOffsetY,
            contentWidth: self.contentWidth,
            contentHeight: self.contentHeight,
        }
    }
}

/// Table containing nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeTreeSnapshot {
    /// Parent node index.
    #[serde(skip_serializing_if = "Option::is_none")]
    parentIndex: Option<Vec<i64>>,
    /// 'Node''s nodeType.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeType: Option<Vec<i64>>,
    /// Type of the shadow root the 'Node' is in. String values are equal to the 'ShadowRootType' enum.
    #[serde(skip_serializing_if = "Option::is_none")]
    shadowRootType: Option<RareStringData>,
    /// 'Node''s nodeName.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeName: Option<Vec<StringIndex>>,
    /// 'Node''s nodeValue.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeValue: Option<Vec<StringIndex>>,
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<Vec<crate::dom::BackendNodeId>>,
    /// Attributes of an 'Element' node. Flatten name, value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<ArrayOfStrings>>,
    /// Only set for textarea elements, contains the text value.
    #[serde(skip_serializing_if = "Option::is_none")]
    textValue: Option<RareStringData>,
    /// Only set for input elements, contains the input's associated text value.
    #[serde(skip_serializing_if = "Option::is_none")]
    inputValue: Option<RareStringData>,
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    #[serde(skip_serializing_if = "Option::is_none")]
    inputChecked: Option<RareBooleanData>,
    /// Only set for option elements, indicates if the element has been selected
    #[serde(skip_serializing_if = "Option::is_none")]
    optionSelected: Option<RareBooleanData>,
    /// The index of the document in the list of the snapshot documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    contentDocumentIndex: Option<RareIntegerData>,
    /// Type of a pseudo element node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoType: Option<RareStringData>,
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoIdentifier: Option<RareStringData>,
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    isClickable: Option<RareBooleanData>,
    /// The selected url for nodes with a srcset attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    currentSourceURL: Option<RareStringData>,
    /// The url of the script (if any) that generates this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    originURL: Option<RareStringData>,
}

impl NodeTreeSnapshot {
    pub fn builder() -> NodeTreeSnapshotBuilder {
        NodeTreeSnapshotBuilder {
            parentIndex: None,
            nodeType: None,
            shadowRootType: None,
            nodeName: None,
            nodeValue: None,
            backendNodeId: None,
            attributes: None,
            textValue: None,
            inputValue: None,
            inputChecked: None,
            optionSelected: None,
            contentDocumentIndex: None,
            pseudoType: None,
            pseudoIdentifier: None,
            isClickable: None,
            currentSourceURL: None,
            originURL: None,
        }
    }
    pub fn parentIndex(&self) -> Option<&[i64]> { self.parentIndex.as_deref() }
    pub fn nodeType(&self) -> Option<&[i64]> { self.nodeType.as_deref() }
    pub fn shadowRootType(&self) -> Option<&RareStringData> { self.shadowRootType.as_ref() }
    pub fn nodeName(&self) -> Option<&[StringIndex]> { self.nodeName.as_deref() }
    pub fn nodeValue(&self) -> Option<&[StringIndex]> { self.nodeValue.as_deref() }
    pub fn backendNodeId(&self) -> Option<&[crate::dom::BackendNodeId]> { self.backendNodeId.as_deref() }
    pub fn attributes(&self) -> Option<&[ArrayOfStrings]> { self.attributes.as_deref() }
    pub fn textValue(&self) -> Option<&RareStringData> { self.textValue.as_ref() }
    pub fn inputValue(&self) -> Option<&RareStringData> { self.inputValue.as_ref() }
    pub fn inputChecked(&self) -> Option<&RareBooleanData> { self.inputChecked.as_ref() }
    pub fn optionSelected(&self) -> Option<&RareBooleanData> { self.optionSelected.as_ref() }
    pub fn contentDocumentIndex(&self) -> Option<&RareIntegerData> { self.contentDocumentIndex.as_ref() }
    pub fn pseudoType(&self) -> Option<&RareStringData> { self.pseudoType.as_ref() }
    pub fn pseudoIdentifier(&self) -> Option<&RareStringData> { self.pseudoIdentifier.as_ref() }
    pub fn isClickable(&self) -> Option<&RareBooleanData> { self.isClickable.as_ref() }
    pub fn currentSourceURL(&self) -> Option<&RareStringData> { self.currentSourceURL.as_ref() }
    pub fn originURL(&self) -> Option<&RareStringData> { self.originURL.as_ref() }
}

#[derive(Default)]
pub struct NodeTreeSnapshotBuilder {
    parentIndex: Option<Vec<i64>>,
    nodeType: Option<Vec<i64>>,
    shadowRootType: Option<RareStringData>,
    nodeName: Option<Vec<StringIndex>>,
    nodeValue: Option<Vec<StringIndex>>,
    backendNodeId: Option<Vec<crate::dom::BackendNodeId>>,
    attributes: Option<Vec<ArrayOfStrings>>,
    textValue: Option<RareStringData>,
    inputValue: Option<RareStringData>,
    inputChecked: Option<RareBooleanData>,
    optionSelected: Option<RareBooleanData>,
    contentDocumentIndex: Option<RareIntegerData>,
    pseudoType: Option<RareStringData>,
    pseudoIdentifier: Option<RareStringData>,
    isClickable: Option<RareBooleanData>,
    currentSourceURL: Option<RareStringData>,
    originURL: Option<RareStringData>,
}

impl NodeTreeSnapshotBuilder {
    /// Parent node index.
    pub fn parentIndex(mut self, parentIndex: Vec<i64>) -> Self { self.parentIndex = Some(parentIndex); self }
    /// 'Node''s nodeType.
    pub fn nodeType(mut self, nodeType: Vec<i64>) -> Self { self.nodeType = Some(nodeType); self }
    /// Type of the shadow root the 'Node' is in. String values are equal to the 'ShadowRootType' enum.
    pub fn shadowRootType(mut self, shadowRootType: RareStringData) -> Self { self.shadowRootType = Some(shadowRootType); self }
    /// 'Node''s nodeName.
    pub fn nodeName(mut self, nodeName: Vec<StringIndex>) -> Self { self.nodeName = Some(nodeName); self }
    /// 'Node''s nodeValue.
    pub fn nodeValue(mut self, nodeValue: Vec<StringIndex>) -> Self { self.nodeValue = Some(nodeValue); self }
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.
    pub fn backendNodeId(mut self, backendNodeId: Vec<crate::dom::BackendNodeId>) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// Attributes of an 'Element' node. Flatten name, value pairs.
    pub fn attributes(mut self, attributes: Vec<ArrayOfStrings>) -> Self { self.attributes = Some(attributes); self }
    /// Only set for textarea elements, contains the text value.
    pub fn textValue(mut self, textValue: RareStringData) -> Self { self.textValue = Some(textValue); self }
    /// Only set for input elements, contains the input's associated text value.
    pub fn inputValue(mut self, inputValue: RareStringData) -> Self { self.inputValue = Some(inputValue); self }
    /// Only set for radio and checkbox input elements, indicates if the element has been checked
    pub fn inputChecked(mut self, inputChecked: RareBooleanData) -> Self { self.inputChecked = Some(inputChecked); self }
    /// Only set for option elements, indicates if the element has been selected
    pub fn optionSelected(mut self, optionSelected: RareBooleanData) -> Self { self.optionSelected = Some(optionSelected); self }
    /// The index of the document in the list of the snapshot documents.
    pub fn contentDocumentIndex(mut self, contentDocumentIndex: RareIntegerData) -> Self { self.contentDocumentIndex = Some(contentDocumentIndex); self }
    /// Type of a pseudo element node.
    pub fn pseudoType(mut self, pseudoType: RareStringData) -> Self { self.pseudoType = Some(pseudoType); self }
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.
    pub fn pseudoIdentifier(mut self, pseudoIdentifier: RareStringData) -> Self { self.pseudoIdentifier = Some(pseudoIdentifier); self }
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.
    pub fn isClickable(mut self, isClickable: RareBooleanData) -> Self { self.isClickable = Some(isClickable); self }
    /// The selected url for nodes with a srcset attribute.
    pub fn currentSourceURL(mut self, currentSourceURL: RareStringData) -> Self { self.currentSourceURL = Some(currentSourceURL); self }
    /// The url of the script (if any) that generates this node.
    pub fn originURL(mut self, originURL: RareStringData) -> Self { self.originURL = Some(originURL); self }
    pub fn build(self) -> NodeTreeSnapshot {
        NodeTreeSnapshot {
            parentIndex: self.parentIndex,
            nodeType: self.nodeType,
            shadowRootType: self.shadowRootType,
            nodeName: self.nodeName,
            nodeValue: self.nodeValue,
            backendNodeId: self.backendNodeId,
            attributes: self.attributes,
            textValue: self.textValue,
            inputValue: self.inputValue,
            inputChecked: self.inputChecked,
            optionSelected: self.optionSelected,
            contentDocumentIndex: self.contentDocumentIndex,
            pseudoType: self.pseudoType,
            pseudoIdentifier: self.pseudoIdentifier,
            isClickable: self.isClickable,
            currentSourceURL: self.currentSourceURL,
            originURL: self.originURL,
        }
    }
}

/// Table of details of an element in the DOM tree with a LayoutObject.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutTreeSnapshot {
    /// Index of the corresponding node in the 'NodeTreeSnapshot' array returned by 'captureSnapshot'.
    nodeIndex: Vec<i64>,
    /// Array of indexes specifying computed style strings, filtered according to the 'computedStyles' parameter passed to 'captureSnapshot'.
    styles: Vec<ArrayOfStrings>,
    /// The absolute position bounding box.
    bounds: Vec<Rectangle>,
    /// Contents of the LayoutText, if any.
    text: Vec<StringIndex>,
    /// Stacking context information.
    stackingContexts: RareBooleanData,
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// captureSnapshot was true.
    #[serde(skip_serializing_if = "Option::is_none")]
    paintOrders: Option<Vec<i64>>,
    /// The offset rect of nodes. Only available when includeDOMRects is set to true
    #[serde(skip_serializing_if = "Option::is_none")]
    offsetRects: Option<Vec<Rectangle>>,
    /// The scroll rect of nodes. Only available when includeDOMRects is set to true
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollRects: Option<Vec<Rectangle>>,
    /// The client rect of nodes. Only available when includeDOMRects is set to true
    #[serde(skip_serializing_if = "Option::is_none")]
    clientRects: Option<Vec<Rectangle>>,
    /// The list of background colors that are blended with colors of overlapping elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    blendedBackgroundColors: Option<Vec<StringIndex>>,
    /// The list of computed text opacities.
    #[serde(skip_serializing_if = "Option::is_none")]
    textColorOpacities: Option<Vec<f64>>,
}

impl LayoutTreeSnapshot {
    pub fn builder(nodeIndex: Vec<i64>, styles: Vec<ArrayOfStrings>, bounds: Vec<Rectangle>, text: Vec<StringIndex>, stackingContexts: RareBooleanData) -> LayoutTreeSnapshotBuilder {
        LayoutTreeSnapshotBuilder {
            nodeIndex: nodeIndex,
            styles: styles,
            bounds: bounds,
            text: text,
            stackingContexts: stackingContexts,
            paintOrders: None,
            offsetRects: None,
            scrollRects: None,
            clientRects: None,
            blendedBackgroundColors: None,
            textColorOpacities: None,
        }
    }
    pub fn nodeIndex(&self) -> &[i64] { &self.nodeIndex }
    pub fn styles(&self) -> &[ArrayOfStrings] { &self.styles }
    pub fn bounds(&self) -> &[Rectangle] { &self.bounds }
    pub fn text(&self) -> &[StringIndex] { &self.text }
    pub fn stackingContexts(&self) -> &RareBooleanData { &self.stackingContexts }
    pub fn paintOrders(&self) -> Option<&[i64]> { self.paintOrders.as_deref() }
    pub fn offsetRects(&self) -> Option<&[Rectangle]> { self.offsetRects.as_deref() }
    pub fn scrollRects(&self) -> Option<&[Rectangle]> { self.scrollRects.as_deref() }
    pub fn clientRects(&self) -> Option<&[Rectangle]> { self.clientRects.as_deref() }
    pub fn blendedBackgroundColors(&self) -> Option<&[StringIndex]> { self.blendedBackgroundColors.as_deref() }
    pub fn textColorOpacities(&self) -> Option<&[f64]> { self.textColorOpacities.as_deref() }
}


pub struct LayoutTreeSnapshotBuilder {
    nodeIndex: Vec<i64>,
    styles: Vec<ArrayOfStrings>,
    bounds: Vec<Rectangle>,
    text: Vec<StringIndex>,
    stackingContexts: RareBooleanData,
    paintOrders: Option<Vec<i64>>,
    offsetRects: Option<Vec<Rectangle>>,
    scrollRects: Option<Vec<Rectangle>>,
    clientRects: Option<Vec<Rectangle>>,
    blendedBackgroundColors: Option<Vec<StringIndex>>,
    textColorOpacities: Option<Vec<f64>>,
}

impl LayoutTreeSnapshotBuilder {
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// captureSnapshot was true.
    pub fn paintOrders(mut self, paintOrders: Vec<i64>) -> Self { self.paintOrders = Some(paintOrders); self }
    /// The offset rect of nodes. Only available when includeDOMRects is set to true
    pub fn offsetRects(mut self, offsetRects: Vec<Rectangle>) -> Self { self.offsetRects = Some(offsetRects); self }
    /// The scroll rect of nodes. Only available when includeDOMRects is set to true
    pub fn scrollRects(mut self, scrollRects: Vec<Rectangle>) -> Self { self.scrollRects = Some(scrollRects); self }
    /// The client rect of nodes. Only available when includeDOMRects is set to true
    pub fn clientRects(mut self, clientRects: Vec<Rectangle>) -> Self { self.clientRects = Some(clientRects); self }
    /// The list of background colors that are blended with colors of overlapping elements.
    pub fn blendedBackgroundColors(mut self, blendedBackgroundColors: Vec<StringIndex>) -> Self { self.blendedBackgroundColors = Some(blendedBackgroundColors); self }
    /// The list of computed text opacities.
    pub fn textColorOpacities(mut self, textColorOpacities: Vec<f64>) -> Self { self.textColorOpacities = Some(textColorOpacities); self }
    pub fn build(self) -> LayoutTreeSnapshot {
        LayoutTreeSnapshot {
            nodeIndex: self.nodeIndex,
            styles: self.styles,
            bounds: self.bounds,
            text: self.text,
            stackingContexts: self.stackingContexts,
            paintOrders: self.paintOrders,
            offsetRects: self.offsetRects,
            scrollRects: self.scrollRects,
            clientRects: self.clientRects,
            blendedBackgroundColors: self.blendedBackgroundColors,
            textColorOpacities: self.textColorOpacities,
        }
    }
}

/// Table of details of the post layout rendered text positions. The exact layout should not be regarded as
/// stable and may change between versions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextBoxSnapshot {
    /// Index of the layout tree node that owns this box collection.
    layoutIndex: Vec<i64>,
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
    pub fn builder(layoutIndex: Vec<i64>, bounds: Vec<Rectangle>, start: Vec<i64>, length: Vec<i64>) -> TextBoxSnapshotBuilder {
        TextBoxSnapshotBuilder {
            layoutIndex: layoutIndex,
            bounds: bounds,
            start: start,
            length: length,
        }
    }
    pub fn layoutIndex(&self) -> &[i64] { &self.layoutIndex }
    pub fn bounds(&self) -> &[Rectangle] { &self.bounds }
    pub fn start(&self) -> &[i64] { &self.start }
    pub fn length(&self) -> &[i64] { &self.length }
}


pub struct TextBoxSnapshotBuilder {
    layoutIndex: Vec<i64>,
    bounds: Vec<Rectangle>,
    start: Vec<i64>,
    length: Vec<i64>,
}

impl TextBoxSnapshotBuilder {
    pub fn build(self) -> TextBoxSnapshot {
        TextBoxSnapshot {
            layoutIndex: self.layoutIndex,
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
    computedStyleWhitelist: Vec<Cow<'a, str>>,
    /// Whether or not to retrieve details of DOM listeners (default false).
    #[serde(skip_serializing_if = "Option::is_none")]
    includeEventListeners: Option<bool>,
    /// Whether to determine and include the paint order index of LayoutTreeNodes (default false).
    #[serde(skip_serializing_if = "Option::is_none")]
    includePaintOrder: Option<bool>,
    /// Whether to include UA shadow tree in the snapshot (default false).
    #[serde(skip_serializing_if = "Option::is_none")]
    includeUserAgentShadowTree: Option<bool>,
}

impl<'a> GetSnapshotParams<'a> {
    pub fn builder(computedStyleWhitelist: Vec<Cow<'a, str>>) -> GetSnapshotParamsBuilder<'a> {
        GetSnapshotParamsBuilder {
            computedStyleWhitelist: computedStyleWhitelist,
            includeEventListeners: None,
            includePaintOrder: None,
            includeUserAgentShadowTree: None,
        }
    }
    pub fn computedStyleWhitelist(&self) -> &[Cow<'a, str>] { &self.computedStyleWhitelist }
    pub fn includeEventListeners(&self) -> Option<bool> { self.includeEventListeners }
    pub fn includePaintOrder(&self) -> Option<bool> { self.includePaintOrder }
    pub fn includeUserAgentShadowTree(&self) -> Option<bool> { self.includeUserAgentShadowTree }
}


pub struct GetSnapshotParamsBuilder<'a> {
    computedStyleWhitelist: Vec<Cow<'a, str>>,
    includeEventListeners: Option<bool>,
    includePaintOrder: Option<bool>,
    includeUserAgentShadowTree: Option<bool>,
}

impl<'a> GetSnapshotParamsBuilder<'a> {
    /// Whether or not to retrieve details of DOM listeners (default false).
    pub fn includeEventListeners(mut self, includeEventListeners: bool) -> Self { self.includeEventListeners = Some(includeEventListeners); self }
    /// Whether to determine and include the paint order index of LayoutTreeNodes (default false).
    pub fn includePaintOrder(mut self, includePaintOrder: bool) -> Self { self.includePaintOrder = Some(includePaintOrder); self }
    /// Whether to include UA shadow tree in the snapshot (default false).
    pub fn includeUserAgentShadowTree(mut self, includeUserAgentShadowTree: bool) -> Self { self.includeUserAgentShadowTree = Some(includeUserAgentShadowTree); self }
    pub fn build(self) -> GetSnapshotParams<'a> {
        GetSnapshotParams {
            computedStyleWhitelist: self.computedStyleWhitelist,
            includeEventListeners: self.includeEventListeners,
            includePaintOrder: self.includePaintOrder,
            includeUserAgentShadowTree: self.includeUserAgentShadowTree,
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
    domNodes: Vec<DOMNode<'a>>,
    /// The nodes in the layout tree.
    layoutTreeNodes: Vec<LayoutTreeNode<'a>>,
    /// Whitelisted ComputedStyle properties for each node in the layout tree.
    computedStyles: Vec<ComputedStyle<'a>>,
}

impl<'a> GetSnapshotReturns<'a> {
    pub fn builder(domNodes: Vec<DOMNode<'a>>, layoutTreeNodes: Vec<LayoutTreeNode<'a>>, computedStyles: Vec<ComputedStyle<'a>>) -> GetSnapshotReturnsBuilder<'a> {
        GetSnapshotReturnsBuilder {
            domNodes: domNodes,
            layoutTreeNodes: layoutTreeNodes,
            computedStyles: computedStyles,
        }
    }
    pub fn domNodes(&self) -> &[DOMNode<'a>] { &self.domNodes }
    pub fn layoutTreeNodes(&self) -> &[LayoutTreeNode<'a>] { &self.layoutTreeNodes }
    pub fn computedStyles(&self) -> &[ComputedStyle<'a>] { &self.computedStyles }
}


pub struct GetSnapshotReturnsBuilder<'a> {
    domNodes: Vec<DOMNode<'a>>,
    layoutTreeNodes: Vec<LayoutTreeNode<'a>>,
    computedStyles: Vec<ComputedStyle<'a>>,
}

impl<'a> GetSnapshotReturnsBuilder<'a> {
    pub fn build(self) -> GetSnapshotReturns<'a> {
        GetSnapshotReturns {
            domNodes: self.domNodes,
            layoutTreeNodes: self.layoutTreeNodes,
            computedStyles: self.computedStyles,
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
    computedStyles: Vec<Cow<'a, str>>,
    /// Whether to include layout object paint orders into the snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    includePaintOrder: Option<bool>,
    /// Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot
    #[serde(skip_serializing_if = "Option::is_none")]
    includeDOMRects: Option<bool>,
    /// Whether to include blended background colors in the snapshot (default: false).
    /// Blended background color is achieved by blending background colors of all elements
    /// that overlap with the current element.
    #[serde(skip_serializing_if = "Option::is_none")]
    includeBlendedBackgroundColors: Option<bool>,
    /// Whether to include text color opacity in the snapshot (default: false).
    /// An element might have the opacity property set that affects the text color of the element.
    /// The final text color opacity is computed based on the opacity of all overlapping elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    includeTextColorOpacities: Option<bool>,
}

impl<'a> CaptureSnapshotParams<'a> {
    pub fn builder(computedStyles: Vec<Cow<'a, str>>) -> CaptureSnapshotParamsBuilder<'a> {
        CaptureSnapshotParamsBuilder {
            computedStyles: computedStyles,
            includePaintOrder: None,
            includeDOMRects: None,
            includeBlendedBackgroundColors: None,
            includeTextColorOpacities: None,
        }
    }
    pub fn computedStyles(&self) -> &[Cow<'a, str>] { &self.computedStyles }
    pub fn includePaintOrder(&self) -> Option<bool> { self.includePaintOrder }
    pub fn includeDOMRects(&self) -> Option<bool> { self.includeDOMRects }
    pub fn includeBlendedBackgroundColors(&self) -> Option<bool> { self.includeBlendedBackgroundColors }
    pub fn includeTextColorOpacities(&self) -> Option<bool> { self.includeTextColorOpacities }
}


pub struct CaptureSnapshotParamsBuilder<'a> {
    computedStyles: Vec<Cow<'a, str>>,
    includePaintOrder: Option<bool>,
    includeDOMRects: Option<bool>,
    includeBlendedBackgroundColors: Option<bool>,
    includeTextColorOpacities: Option<bool>,
}

impl<'a> CaptureSnapshotParamsBuilder<'a> {
    /// Whether to include layout object paint orders into the snapshot.
    pub fn includePaintOrder(mut self, includePaintOrder: bool) -> Self { self.includePaintOrder = Some(includePaintOrder); self }
    /// Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot
    pub fn includeDOMRects(mut self, includeDOMRects: bool) -> Self { self.includeDOMRects = Some(includeDOMRects); self }
    /// Whether to include blended background colors in the snapshot (default: false).
    /// Blended background color is achieved by blending background colors of all elements
    /// that overlap with the current element.
    pub fn includeBlendedBackgroundColors(mut self, includeBlendedBackgroundColors: bool) -> Self { self.includeBlendedBackgroundColors = Some(includeBlendedBackgroundColors); self }
    /// Whether to include text color opacity in the snapshot (default: false).
    /// An element might have the opacity property set that affects the text color of the element.
    /// The final text color opacity is computed based on the opacity of all overlapping elements.
    pub fn includeTextColorOpacities(mut self, includeTextColorOpacities: bool) -> Self { self.includeTextColorOpacities = Some(includeTextColorOpacities); self }
    pub fn build(self) -> CaptureSnapshotParams<'a> {
        CaptureSnapshotParams {
            computedStyles: self.computedStyles,
            includePaintOrder: self.includePaintOrder,
            includeDOMRects: self.includeDOMRects,
            includeBlendedBackgroundColors: self.includeBlendedBackgroundColors,
            includeTextColorOpacities: self.includeTextColorOpacities,
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
    pub fn builder(documents: Vec<DocumentSnapshot>, strings: Vec<Cow<'a, str>>) -> CaptureSnapshotReturnsBuilder<'a> {
        CaptureSnapshotReturnsBuilder {
            documents: documents,
            strings: strings,
        }
    }
    pub fn documents(&self) -> &[DocumentSnapshot] { &self.documents }
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
