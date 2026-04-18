//! This domain facilitates obtaining document snapshots with DOM, layout, and style information.

use serde::{Serialize, Deserialize};

/// A Node in the DOM tree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DOMNode {
    /// 'Node''s nodeType.

    pub nodeType: i64,
    /// 'Node''s nodeName.

    pub nodeName: String,
    /// 'Node''s nodeValue.

    pub nodeValue: String,
    /// Only set for textarea elements, contains the text value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub textValue: Option<String>,
    /// Only set for input elements, contains the input's associated text value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputValue: Option<String>,
    /// Only set for radio and checkbox input elements, indicates if the element has been checked

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputChecked: Option<bool>,
    /// Only set for option elements, indicates if the element has been selected

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optionSelected: Option<bool>,
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.

    pub backendNodeId: crate::dom::BackendNodeId,
    /// The indexes of the node's child nodes in the 'domNodes' array returned by 'getSnapshot', if
    /// any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub childNodeIndexes: Option<Vec<i64>>,
    /// Attributes of an 'Element' node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<NameValue>>,
    /// Indexes of pseudo elements associated with this node in the 'domNodes' array returned by
    /// 'getSnapshot', if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoElementIndexes: Option<Vec<i64>>,
    /// The index of the node's related layout tree node in the 'layoutTreeNodes' array returned by
    /// 'getSnapshot', if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layoutNodeIndex: Option<u64>,
    /// Document URL that 'Document' or 'FrameOwner' node points to.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentURL: Option<String>,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseURL: Option<String>,
    /// Only set for documents, contains the document's content language.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentLanguage: Option<String>,
    /// Only set for documents, contains the document's character set encoding.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentEncoding: Option<String>,
    /// 'DocumentType' node's publicId.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicId: Option<String>,
    /// 'DocumentType' node's systemId.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub systemId: Option<String>,
    /// Frame ID for frame owner elements and also for the document node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
    /// The index of a frame owner element's content document in the 'domNodes' array returned by
    /// 'getSnapshot', if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentDocumentIndex: Option<u64>,
    /// Type of a pseudo element node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoType: Option<crate::dom::PseudoType>,
    /// Shadow root type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadowRootType: Option<crate::dom::ShadowRootType>,
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isClickable: Option<bool>,
    /// Details of the node's event listeners, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventListeners: Option<Vec<crate::domdebugger::EventListener>>,
    /// The selected url for nodes with a srcset attribute.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentSourceURL: Option<String>,
    /// The url of the script (if any) that generates this node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub originURL: Option<String>,
    /// Scroll offsets, set when this node is a Document.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollOffsetX: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollOffsetY: Option<f64>,
}

/// Details of post layout rendered text positions. The exact layout should not be regarded as
/// stable and may change between versions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InlineTextBox {
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.

    pub boundingBox: crate::dom::Rect,
    /// The starting index in characters, for this post layout textbox substring. Characters that
    /// would be represented as a surrogate pair in UTF-16 have length 2.

    pub startCharacterIndex: u64,
    /// The number of characters in this post layout textbox substring. Characters that would be
    /// represented as a surrogate pair in UTF-16 have length 2.

    pub numCharacters: i64,
}

/// Details of an element in the DOM tree with a LayoutObject.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutTreeNode {
    /// The index of the related DOM node in the 'domNodes' array returned by 'getSnapshot'.

    pub domNodeIndex: u64,
    /// The bounding box in document coordinates. Note that scroll offset of the document is ignored.

    pub boundingBox: crate::dom::Rect,
    /// Contents of the LayoutText, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layoutText: Option<String>,
    /// The post-layout inline text nodes, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlineTextNodes: Option<Vec<InlineTextBox>>,
    /// Index into the 'computedStyles' array returned by 'getSnapshot'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleIndex: Option<u64>,
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// getSnapshot was true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paintOrder: Option<i64>,
    /// Set to true to indicate the element begins a new stacking context.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isStackingContext: Option<bool>,
}

/// A subset of the full ComputedStyle as defined by the request whitelist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComputedStyle {
    /// Name/value pairs of computed style properties.

    pub properties: Vec<NameValue>,
}

/// A name/value pair.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NameValue {
    /// Attribute/property name.

    pub name: String,
    /// Attribute/property value.

    pub value: String,
}

/// Index of the string in the strings table.

pub type StringIndex = i64;

/// Index of the string in the strings table.

pub type ArrayOfStrings = Vec<StringIndex>;

/// Data that is only present on rare nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RareStringData {

    pub index: Vec<i64>,

    pub value: Vec<StringIndex>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RareBooleanData {

    pub index: Vec<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RareIntegerData {

    pub index: Vec<i64>,

    pub value: Vec<i64>,
}


pub type Rectangle = Vec<f64>;

/// Document snapshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSnapshot {
    /// Document URL that 'Document' or 'FrameOwner' node points to.

    pub documentURL: StringIndex,
    /// Document title.

    pub title: StringIndex,
    /// Base URL that 'Document' or 'FrameOwner' node uses for URL completion.

    pub baseURL: StringIndex,
    /// Contains the document's content language.

    pub contentLanguage: StringIndex,
    /// Contains the document's character set encoding.

    pub encodingName: StringIndex,
    /// 'DocumentType' node's publicId.

    pub publicId: StringIndex,
    /// 'DocumentType' node's systemId.

    pub systemId: StringIndex,
    /// Frame ID for frame owner elements and also for the document node.

    pub frameId: StringIndex,
    /// A table with dom nodes.

    pub nodes: NodeTreeSnapshot,
    /// The nodes in the layout tree.

    pub layout: LayoutTreeSnapshot,
    /// The post-layout inline text nodes.

    pub textBoxes: TextBoxSnapshot,
    /// Horizontal scroll offset.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollOffsetX: Option<f64>,
    /// Vertical scroll offset.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollOffsetY: Option<f64>,
    /// Document content width.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentWidth: Option<f64>,
    /// Document content height.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentHeight: Option<f64>,
}

/// Table containing nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeTreeSnapshot {
    /// Parent node index.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentIndex: Option<Vec<i64>>,
    /// 'Node''s nodeType.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeType: Option<Vec<i64>>,
    /// Type of the shadow root the 'Node' is in. String values are equal to the 'ShadowRootType' enum.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadowRootType: Option<RareStringData>,
    /// 'Node''s nodeName.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeName: Option<Vec<StringIndex>>,
    /// 'Node''s nodeValue.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeValue: Option<Vec<StringIndex>>,
    /// 'Node''s id, corresponds to DOM.Node.backendNodeId.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<Vec<crate::dom::BackendNodeId>>,
    /// Attributes of an 'Element' node. Flatten name, value pairs.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ArrayOfStrings>>,
    /// Only set for textarea elements, contains the text value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub textValue: Option<RareStringData>,
    /// Only set for input elements, contains the input's associated text value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputValue: Option<RareStringData>,
    /// Only set for radio and checkbox input elements, indicates if the element has been checked

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputChecked: Option<RareBooleanData>,
    /// Only set for option elements, indicates if the element has been selected

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optionSelected: Option<RareBooleanData>,
    /// The index of the document in the list of the snapshot documents.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentDocumentIndex: Option<RareIntegerData>,
    /// Type of a pseudo element node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoType: Option<RareStringData>,
    /// Pseudo element identifier for this node. Only present if there is a
    /// valid pseudoType.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoIdentifier: Option<RareStringData>,
    /// Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    /// event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    /// clicked.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isClickable: Option<RareBooleanData>,
    /// The selected url for nodes with a srcset attribute.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currentSourceURL: Option<RareStringData>,
    /// The url of the script (if any) that generates this node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub originURL: Option<RareStringData>,
}

/// Table of details of an element in the DOM tree with a LayoutObject.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutTreeSnapshot {
    /// Index of the corresponding node in the 'NodeTreeSnapshot' array returned by 'captureSnapshot'.

    pub nodeIndex: Vec<i64>,
    /// Array of indexes specifying computed style strings, filtered according to the 'computedStyles' parameter passed to 'captureSnapshot'.

    pub styles: Vec<ArrayOfStrings>,
    /// The absolute position bounding box.

    pub bounds: Vec<Rectangle>,
    /// Contents of the LayoutText, if any.

    pub text: Vec<StringIndex>,
    /// Stacking context information.

    pub stackingContexts: RareBooleanData,
    /// Global paint order index, which is determined by the stacking order of the nodes. Nodes
    /// that are painted together will have the same index. Only provided if includePaintOrder in
    /// captureSnapshot was true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paintOrders: Option<Vec<i64>>,
    /// The offset rect of nodes. Only available when includeDOMRects is set to true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsetRects: Option<Vec<Rectangle>>,
    /// The scroll rect of nodes. Only available when includeDOMRects is set to true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollRects: Option<Vec<Rectangle>>,
    /// The client rect of nodes. Only available when includeDOMRects is set to true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientRects: Option<Vec<Rectangle>>,
    /// The list of background colors that are blended with colors of overlapping elements.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blendedBackgroundColors: Option<Vec<StringIndex>>,
    /// The list of computed text opacities.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub textColorOpacities: Option<Vec<f64>>,
}

/// Table of details of the post layout rendered text positions. The exact layout should not be regarded as
/// stable and may change between versions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextBoxSnapshot {
    /// Index of the layout tree node that owns this box collection.

    pub layoutIndex: Vec<i64>,
    /// The absolute position bounding box.

    pub bounds: Vec<Rectangle>,
    /// The starting index in characters, for this post layout textbox substring. Characters that
    /// would be represented as a surrogate pair in UTF-16 have length 2.

    pub start: Vec<i64>,
    /// The number of characters in this post layout textbox substring. Characters that would be
    /// represented as a surrogate pair in UTF-16 have length 2.

    pub length: Vec<i64>,
}

/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSnapshotParams {
    /// Whitelist of computed styles to return.

    pub computedStyleWhitelist: Vec<String>,
    /// Whether or not to retrieve details of DOM listeners (default false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeEventListeners: Option<bool>,
    /// Whether to determine and include the paint order index of LayoutTreeNodes (default false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includePaintOrder: Option<bool>,
    /// Whether to include UA shadow tree in the snapshot (default false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeUserAgentShadowTree: Option<bool>,
}

/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSnapshotReturns {
    /// The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.

    pub domNodes: Vec<DOMNode>,
    /// The nodes in the layout tree.

    pub layoutTreeNodes: Vec<LayoutTreeNode>,
    /// Whitelisted ComputedStyle properties for each node in the layout tree.

    pub computedStyles: Vec<ComputedStyle>,
}

/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotParams {
    /// Whitelist of computed styles to return.

    pub computedStyles: Vec<String>,
    /// Whether to include layout object paint orders into the snapshot.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includePaintOrder: Option<bool>,
    /// Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeDOMRects: Option<bool>,
    /// Whether to include blended background colors in the snapshot (default: false).
    /// Blended background color is achieved by blending background colors of all elements
    /// that overlap with the current element.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeBlendedBackgroundColors: Option<bool>,
    /// Whether to include text color opacity in the snapshot (default: false).
    /// An element might have the opacity property set that affects the text color of the element.
    /// The final text color opacity is computed based on the opacity of all overlapping elements.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeTextColorOpacities: Option<bool>,
}

/// Returns a document snapshot, including the full DOM tree of the root node (including iframes,
/// template contents, and imported documents) in a flattened array, as well as layout and
/// white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
/// flattened.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotReturns {
    /// The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.

    pub documents: Vec<DocumentSnapshot>,
    /// Shared string table that all string properties refer to with indexes.

    pub strings: Vec<String>,
}
