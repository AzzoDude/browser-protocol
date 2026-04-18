use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Unique accessibility node identifier.

pub type AXNodeId = String;

/// Enum of possible property types.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AXValueType {
    #[default]
    Boolean,
    Tristate,
    BooleanOrUndefined,
    Idref,
    IdrefList,
    Integer,
    Node,
    NodeList,
    Number,
    String,
    ComputedString,
    Token,
    TokenList,
    DomRelation,
    Role,
    InternalRole,
    ValueUndefined,
}

/// Enum of possible property sources.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AXValueSourceType {
    #[default]
    Attribute,
    Implicit,
    Style,
    Contents,
    Placeholder,
    RelatedElement,
}

/// Enum of possible native property sources (as a subtype of a particular AXValueSourceType).

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AXValueNativeSourceType {
    #[default]
    Description,
    Figcaption,
    Label,
    Labelfor,
    Labelwrapped,
    Legend,
    Rubyannotation,
    Tablecaption,
    Title,
    Other,
}

/// A single source for a computed AX property.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXValueSource {
    /// What type of source this is.

    #[serde(rename = "type")]
    pub type_: AXValueSourceType,
    /// The value of this property source.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AXValue>,
    /// The name of the relevant attribute, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// The value of the relevant attribute, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributeValue: Option<AXValue>,
    /// Whether this source is superseded by a higher priority source.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub superseded: Option<bool>,
    /// The native markup source for this value, e.g. a '<label>' element.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nativeSource: Option<AXValueNativeSourceType>,
    /// The value, such as a node or node list, of the native source.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nativeSourceValue: Option<AXValue>,
    /// Whether the value for this property is invalid.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<bool>,
    /// Reason for the value being invalid, if it is.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidReason: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXRelatedNode {
    /// The BackendNodeId of the related DOM node.

    pub backendDOMNodeId: crate::dom::BackendNodeId,
    /// The IDRef value provided, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idref: Option<String>,
    /// The text alternative of this node in the current context.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXProperty {
    /// The name of this property.

    pub name: AXPropertyName,
    /// The value of this property.

    pub value: AXValue,
}

/// A single computed AX property.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXValue {
    /// The type of this value.

    #[serde(rename = "type")]
    pub type_: AXValueType,
    /// The computed value of this property.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    /// One or more related nodes, if applicable.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relatedNodes: Option<Vec<AXRelatedNode>>,
    /// The sources which contributed to the computation of this property.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<AXValueSource>>,
}

/// Values of AXProperty name:
/// - from 'busy' to 'roledescription': states which apply to every AX node
/// - from 'live' to 'root': attributes which apply to nodes in live regions
/// - from 'autocomplete' to 'valuetext': attributes which apply to widgets
/// - from 'checked' to 'selected': states which apply to widgets
/// - from 'activedescendant' to 'owns': relationships between elements other than parent/child/sibling
/// - from 'activeFullscreenElement' to 'uninteresting': reasons why this noode is hidden

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AXPropertyName {
    #[default]
    Actions,
    Busy,
    Disabled,
    Editable,
    Focusable,
    Focused,
    Hidden,
    HiddenRoot,
    Invalid,
    Keyshortcuts,
    Settable,
    Roledescription,
    Live,
    Atomic,
    Relevant,
    Root,
    Autocomplete,
    HasPopup,
    Level,
    Multiselectable,
    Orientation,
    Multiline,
    Readonly,
    Required,
    Valuemin,
    Valuemax,
    Valuetext,
    Checked,
    Expanded,
    Modal,
    Pressed,
    Selected,
    Activedescendant,
    Controls,
    Describedby,
    Details,
    Errormessage,
    Flowto,
    Labelledby,
    Owns,
    Url,
    ActiveFullscreenElement,
    ActiveModalDialog,
    ActiveAriaModalDialog,
    AriaHiddenElement,
    AriaHiddenSubtree,
    EmptyAlt,
    EmptyText,
    InertElement,
    InertSubtree,
    LabelContainer,
    LabelFor,
    NotRendered,
    NotVisible,
    PresentationalRole,
    ProbablyPresentational,
    InactiveCarouselTabContent,
    Uninteresting,
}

/// A node in the accessibility tree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXNode {
    /// Unique identifier for this node.

    pub nodeId: AXNodeId,
    /// Whether this node is ignored for accessibility

    pub ignored: bool,
    /// Collection of reasons why this node is hidden.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignoredReasons: Option<Vec<AXProperty>>,
    /// This 'Node''s role, whether explicit or implicit.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AXValue>,
    /// This 'Node''s Chrome raw role.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chromeRole: Option<AXValue>,
    /// The accessible name for this 'Node'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<AXValue>,
    /// The accessible description for this 'Node'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AXValue>,
    /// The value for this 'Node'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AXValue>,
    /// All other properties

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<AXProperty>>,
    /// ID for this node's parent.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentId: Option<AXNodeId>,
    /// IDs for each of this node's child nodes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub childIds: Option<Vec<AXNodeId>>,
    /// The backend ID for the associated DOM node, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendDOMNodeId: Option<crate::dom::BackendNodeId>,
    /// The frame ID for the frame associated with this nodes document.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Accessibility.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "Accessibility.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Accessibility.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "Accessibility.enable";
    type Response = crate::EmptyReturns;
}

/// Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPartialAXTreeParams {
    /// Identifier of the node to get the partial accessibility tree for.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to get the partial accessibility tree for.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper to get the partial accessibility tree for.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
    /// Whether to fetch this node's ancestors, siblings and children. Defaults to true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetchRelatives: Option<bool>,
}

/// Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPartialAXTreeReturns {
    /// The 'Accessibility.AXNode' for this DOM node, if it exists, plus its ancestors, siblings and
    /// children, if requested.

    pub nodes: Vec<AXNode>,
}

impl GetPartialAXTreeParams { pub const METHOD: &'static str = "Accessibility.getPartialAXTree"; }

impl crate::CdpCommand for GetPartialAXTreeParams {
    const METHOD: &'static str = "Accessibility.getPartialAXTree";
    type Response = GetPartialAXTreeReturns;
}

/// Fetches the entire accessibility tree for the root Document

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFullAXTreeParams {
    /// The maximum depth at which descendants of the root node should be retrieved.
    /// If omitted, the full tree is returned.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    /// The frame for whose document the AX tree should be retrieved.
    /// If omitted, the root frame is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
}

/// Fetches the entire accessibility tree for the root Document

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFullAXTreeReturns {

    pub nodes: Vec<AXNode>,
}

impl GetFullAXTreeParams { pub const METHOD: &'static str = "Accessibility.getFullAXTree"; }

impl crate::CdpCommand for GetFullAXTreeParams {
    const METHOD: &'static str = "Accessibility.getFullAXTree";
    type Response = GetFullAXTreeReturns;
}

/// Fetches the root node.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRootAXNodeParams {
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
}

/// Fetches the root node.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRootAXNodeReturns {

    pub node: AXNode,
}

impl GetRootAXNodeParams { pub const METHOD: &'static str = "Accessibility.getRootAXNode"; }

impl crate::CdpCommand for GetRootAXNodeParams {
    const METHOD: &'static str = "Accessibility.getRootAXNode";
    type Response = GetRootAXNodeReturns;
}

/// Fetches a node and all ancestors up to and including the root.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAXNodeAndAncestorsParams {
    /// Identifier of the node to get.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to get.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper to get.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
}

/// Fetches a node and all ancestors up to and including the root.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAXNodeAndAncestorsReturns {

    pub nodes: Vec<AXNode>,
}

impl GetAXNodeAndAncestorsParams { pub const METHOD: &'static str = "Accessibility.getAXNodeAndAncestors"; }

impl crate::CdpCommand for GetAXNodeAndAncestorsParams {
    const METHOD: &'static str = "Accessibility.getAXNodeAndAncestors";
    type Response = GetAXNodeAndAncestorsReturns;
}

/// Fetches a particular accessibility node by AXNodeId.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChildAXNodesParams {

    pub id: AXNodeId,
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
}

/// Fetches a particular accessibility node by AXNodeId.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChildAXNodesReturns {

    pub nodes: Vec<AXNode>,
}

impl GetChildAXNodesParams { pub const METHOD: &'static str = "Accessibility.getChildAXNodes"; }

impl crate::CdpCommand for GetChildAXNodesParams {
    const METHOD: &'static str = "Accessibility.getChildAXNodes";
    type Response = GetChildAXNodesReturns;
}

/// Query a DOM node's accessibility subtree for accessible name and role.
/// This command computes the name and role for all nodes in the subtree, including those that are
/// ignored for accessibility, and returns those that match the specified name and role. If no DOM
/// node is specified, or the DOM node does not exist, the command returns an error. If neither
/// 'accessibleName' or 'role' is specified, it returns all the accessibility nodes in the subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryAXTreeParams {
    /// Identifier of the node for the root to query.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node for the root to query.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper for the root to query.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<crate::runtime::RemoteObjectId>,
    /// Find nodes with this computed name.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibleName: Option<String>,
    /// Find nodes with this computed role.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Query a DOM node's accessibility subtree for accessible name and role.
/// This command computes the name and role for all nodes in the subtree, including those that are
/// ignored for accessibility, and returns those that match the specified name and role. If no DOM
/// node is specified, or the DOM node does not exist, the command returns an error. If neither
/// 'accessibleName' or 'role' is specified, it returns all the accessibility nodes in the subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryAXTreeReturns {
    /// A list of 'Accessibility.AXNode' matching the specified attributes,
    /// including nodes that are ignored for accessibility.

    pub nodes: Vec<AXNode>,
}

impl QueryAXTreeParams { pub const METHOD: &'static str = "Accessibility.queryAXTree"; }

impl crate::CdpCommand for QueryAXTreeParams {
    const METHOD: &'static str = "Accessibility.queryAXTree";
    type Response = QueryAXTreeReturns;
}
