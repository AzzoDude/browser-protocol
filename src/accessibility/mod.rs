use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Unique accessibility node identifier.

pub type AXNodeId<'a> = Cow<'a, str>;

/// Enum of possible property types.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AXValueType {
    #[default]
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "tristate")]
    Tristate,
    #[serde(rename = "booleanOrUndefined")]
    BooleanOrUndefined,
    #[serde(rename = "idref")]
    Idref,
    #[serde(rename = "idrefList")]
    IdrefList,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "nodeList")]
    NodeList,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "computedString")]
    ComputedString,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "tokenList")]
    TokenList,
    #[serde(rename = "domRelation")]
    DomRelation,
    #[serde(rename = "role")]
    Role,
    #[serde(rename = "internalRole")]
    InternalRole,
    #[serde(rename = "valueUndefined")]
    ValueUndefined,
}

/// Enum of possible property sources.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AXValueSourceType {
    #[default]
    #[serde(rename = "attribute")]
    Attribute,
    #[serde(rename = "implicit")]
    Implicit,
    #[serde(rename = "style")]
    Style,
    #[serde(rename = "contents")]
    Contents,
    #[serde(rename = "placeholder")]
    Placeholder,
    #[serde(rename = "relatedElement")]
    RelatedElement,
}

/// Enum of possible native property sources (as a subtype of a particular AXValueSourceType).

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AXValueNativeSourceType {
    #[default]
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "figcaption")]
    Figcaption,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "labelfor")]
    Labelfor,
    #[serde(rename = "labelwrapped")]
    Labelwrapped,
    #[serde(rename = "legend")]
    Legend,
    #[serde(rename = "rubyannotation")]
    Rubyannotation,
    #[serde(rename = "tablecaption")]
    Tablecaption,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "other")]
    Other,
}

/// A single source for a computed AX property.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AXValueSource<'a> {
    /// What type of source this is.
    #[serde(rename = "type")]
    pub type_: AXValueSourceType,
    /// The value of this property source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AXValue<'a>>,
    /// The name of the relevant attribute, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<Cow<'a, str>>,
    /// The value of the relevant attribute, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "attributeValue")]
    pub attribute_value: Option<AXValue<'a>>,
    /// Whether this source is superseded by a higher priority source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superseded: Option<bool>,
    /// The native markup source for this value, e.g. a '\<label\>' element.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nativeSource")]
    pub native_source: Option<AXValueNativeSourceType>,
    /// The value, such as a node or node list, of the native source.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nativeSourceValue")]
    pub native_source_value: Option<AXValue<'a>>,
    /// Whether the value for this property is invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<bool>,
    /// Reason for the value being invalid, if it is.
    #[serde(skip_serializing_if = "Option::is_none", rename = "invalidReason")]
    pub invalid_reason: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AXRelatedNode<'a> {
    /// The BackendNodeId of the related DOM node.
    #[serde(rename = "backendDOMNodeId")]
    pub backend_dom_node_id: crate::dom::BackendNodeId,
    /// The IDRef value provided, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idref: Option<Cow<'a, str>>,
    /// The text alternative of this node in the current context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AXProperty<'a> {
    /// The name of this property.
    pub name: AXPropertyName,
    /// The value of this property.
    pub value: AXValue<'a>,
}
/// A single computed AX property.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AXValue<'a> {
    /// The type of this value.
    #[serde(rename = "type")]
    pub type_: AXValueType,
    /// The computed value of this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    /// One or more related nodes, if applicable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "relatedNodes")]
    pub related_nodes: Option<Vec<AXRelatedNode<'a>>>,
    /// The sources which contributed to the computation of this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<AXValueSource<'a>>>,
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
    #[serde(rename = "actions")]
    Actions,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "editable")]
    Editable,
    #[serde(rename = "focusable")]
    Focusable,
    #[serde(rename = "focused")]
    Focused,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "hiddenRoot")]
    HiddenRoot,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "keyshortcuts")]
    Keyshortcuts,
    #[serde(rename = "settable")]
    Settable,
    #[serde(rename = "roledescription")]
    Roledescription,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "atomic")]
    Atomic,
    #[serde(rename = "relevant")]
    Relevant,
    #[serde(rename = "root")]
    Root,
    #[serde(rename = "autocomplete")]
    Autocomplete,
    #[serde(rename = "hasPopup")]
    HasPopup,
    #[serde(rename = "level")]
    Level,
    #[serde(rename = "multiselectable")]
    Multiselectable,
    #[serde(rename = "orientation")]
    Orientation,
    #[serde(rename = "multiline")]
    Multiline,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "valuemin")]
    Valuemin,
    #[serde(rename = "valuemax")]
    Valuemax,
    #[serde(rename = "valuetext")]
    Valuetext,
    #[serde(rename = "checked")]
    Checked,
    #[serde(rename = "expanded")]
    Expanded,
    #[serde(rename = "modal")]
    Modal,
    #[serde(rename = "pressed")]
    Pressed,
    #[serde(rename = "selected")]
    Selected,
    #[serde(rename = "activedescendant")]
    Activedescendant,
    #[serde(rename = "controls")]
    Controls,
    #[serde(rename = "describedby")]
    Describedby,
    #[serde(rename = "details")]
    Details,
    #[serde(rename = "errormessage")]
    Errormessage,
    #[serde(rename = "flowto")]
    Flowto,
    #[serde(rename = "labelledby")]
    Labelledby,
    #[serde(rename = "owns")]
    Owns,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "activeFullscreenElement")]
    ActiveFullscreenElement,
    #[serde(rename = "activeModalDialog")]
    ActiveModalDialog,
    #[serde(rename = "activeAriaModalDialog")]
    ActiveAriaModalDialog,
    #[serde(rename = "ariaHiddenElement")]
    AriaHiddenElement,
    #[serde(rename = "ariaHiddenSubtree")]
    AriaHiddenSubtree,
    #[serde(rename = "emptyAlt")]
    EmptyAlt,
    #[serde(rename = "emptyText")]
    EmptyText,
    #[serde(rename = "inertElement")]
    InertElement,
    #[serde(rename = "inertSubtree")]
    InertSubtree,
    #[serde(rename = "labelContainer")]
    LabelContainer,
    #[serde(rename = "labelFor")]
    LabelFor,
    #[serde(rename = "notRendered")]
    NotRendered,
    #[serde(rename = "notVisible")]
    NotVisible,
    #[serde(rename = "presentationalRole")]
    PresentationalRole,
    #[serde(rename = "probablyPresentational")]
    ProbablyPresentational,
    #[serde(rename = "inactiveCarouselTabContent")]
    InactiveCarouselTabContent,
    #[serde(rename = "uninteresting")]
    Uninteresting,
}

/// A node in the accessibility tree.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AXNode<'a> {
    /// Unique identifier for this node.
    #[serde(rename = "nodeId")]
    pub node_id: AXNodeId<'a>,
    /// Whether this node is ignored for accessibility
    pub ignored: bool,
    /// Collection of reasons why this node is hidden.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ignoredReasons")]
    pub ignored_reasons: Option<Vec<AXProperty<'a>>>,
    /// This 'Node''s role, whether explicit or implicit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AXValue<'a>>,
    /// This 'Node''s Chrome raw role.
    #[serde(skip_serializing_if = "Option::is_none", rename = "chromeRole")]
    pub chrome_role: Option<AXValue<'a>>,
    /// The accessible name for this 'Node'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<AXValue<'a>>,
    /// The accessible description for this 'Node'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AXValue<'a>>,
    /// The value for this 'Node'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AXValue<'a>>,
    /// All other properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<AXProperty<'a>>>,
    /// ID for this node's parent.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    pub parent_id: Option<AXNodeId<'a>>,
    /// IDs for each of this node's child nodes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childIds")]
    pub child_ids: Option<Vec<AXNodeId<'a>>>,
    /// The backend ID for the associated DOM node, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendDOMNodeId")]
    pub backend_dom_node_id: Option<crate::dom::BackendNodeId>,
    /// The frame ID for the frame associated with this nodes document.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
}
/// Disables the accessibility domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.disable")]
pub struct DisableParams {

}
/// Enables the accessibility domain which causes 'AXNodeId's to remain consistent between method calls.
/// This turns on accessibility for the page, which can impact performance until accessibility is disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.enable")]
pub struct EnableParams {

}
/// Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.getPartialAXTree", response = "GetPartialAXTreeReturns<'a>")]
pub struct GetPartialAXTreeParams<'a> {
    /// Identifier of the node to get the partial accessibility tree for.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to get the partial accessibility tree for.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper to get the partial accessibility tree for.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Whether to fetch this node's ancestors, siblings and children. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fetchRelatives")]
    pub fetch_relatives: Option<bool>,
}
/// Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetPartialAXTreeReturns<'a> {
    /// The 'Accessibility.AXNode' for this DOM node, if it exists, plus its ancestors, siblings and
    /// children, if requested.
    pub nodes: Vec<AXNode<'a>>,
}
/// Fetches the entire accessibility tree for the root Document

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.getFullAXTree", response = "GetFullAXTreeReturns<'a>")]
pub struct GetFullAXTreeParams<'a> {
    /// The maximum depth at which descendants of the root node should be retrieved.
    /// If omitted, the full tree is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    /// The frame for whose document the AX tree should be retrieved.
    /// If omitted, the root frame is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
}
/// Fetches the entire accessibility tree for the root Document

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetFullAXTreeReturns<'a> {
    pub nodes: Vec<AXNode<'a>>,
}
/// Fetches the root node.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.getRootAXNode", response = "GetRootAXNodeReturns<'a>")]
pub struct GetRootAXNodeParams<'a> {
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
}
/// Fetches the root node.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetRootAXNodeReturns<'a> {
    pub node: AXNode<'a>,
}
/// Fetches a node and all ancestors up to and including the root.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.getAXNodeAndAncestors", response = "GetAXNodeAndAncestorsReturns<'a>")]
pub struct GetAXNodeAndAncestorsParams<'a> {
    /// Identifier of the node to get.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to get.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper to get.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}
/// Fetches a node and all ancestors up to and including the root.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAXNodeAndAncestorsReturns<'a> {
    pub nodes: Vec<AXNode<'a>>,
}
/// Fetches a particular accessibility node by AXNodeId.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.getChildAXNodes", response = "GetChildAXNodesReturns<'a>")]
pub struct GetChildAXNodesParams<'a> {
    pub id: AXNodeId<'a>,
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
}
/// Fetches a particular accessibility node by AXNodeId.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetChildAXNodesReturns<'a> {
    pub nodes: Vec<AXNode<'a>>,
}
/// Query a DOM node's accessibility subtree for accessible name and role.
/// This command computes the name and role for all nodes in the subtree, including those that are
/// ignored for accessibility, and returns those that match the specified name and role. If no DOM
/// node is specified, or the DOM node does not exist, the command returns an error. If neither
/// 'accessibleName' or 'role' is specified, it returns all the accessibility nodes in the subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.queryAXTree", response = "QueryAXTreeReturns<'a>")]
pub struct QueryAXTreeParams<'a> {
    /// Identifier of the node for the root to query.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node for the root to query.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper for the root to query.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Find nodes with this computed name.
    #[serde(skip_serializing_if = "Option::is_none", rename = "accessibleName")]
    pub accessible_name: Option<Cow<'a, str>>,
    /// Find nodes with this computed role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Cow<'a, str>>,
}
/// Query a DOM node's accessibility subtree for accessible name and role.
/// This command computes the name and role for all nodes in the subtree, including those that are
/// ignored for accessibility, and returns those that match the specified name and role. If no DOM
/// node is specified, or the DOM node does not exist, the command returns an error. If neither
/// 'accessibleName' or 'role' is specified, it returns all the accessibility nodes in the subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct QueryAXTreeReturns<'a> {
    /// A list of 'Accessibility.AXNode' matching the specified attributes,
    /// including nodes that are ignored for accessibility.
    pub nodes: Vec<AXNode<'a>>,
}
/// The loadComplete event mirrors the load complete event sent by the browser to assistive
/// technology when the web page has finished loading.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.loadComplete")]
pub struct LoadComplete<'a> {
    /// New document root node.
    pub root: AXNode<'a>,
}
/// The nodesUpdated event is sent every time a previously requested node has changed the in tree.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Accessibility.nodesUpdated")]
pub struct NodesUpdated<'a> {
    /// Updated node data.
    pub nodes: Vec<AXNode<'a>>,
}