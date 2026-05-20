use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXValueSource<'a> {
    /// What type of source this is.
    #[serde(rename = "type")]
    type_: AXValueSourceType,
    /// The value of this property source.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<AXValue<'a>>,
    /// The name of the relevant attribute, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<Cow<'a, str>>,
    /// The value of the relevant attribute, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    attributeValue: Option<AXValue<'a>>,
    /// Whether this source is superseded by a higher priority source.
    #[serde(skip_serializing_if = "Option::is_none")]
    superseded: Option<bool>,
    /// The native markup source for this value, e.g. a '<label>' element.
    #[serde(skip_serializing_if = "Option::is_none")]
    nativeSource: Option<AXValueNativeSourceType>,
    /// The value, such as a node or node list, of the native source.
    #[serde(skip_serializing_if = "Option::is_none")]
    nativeSourceValue: Option<AXValue<'a>>,
    /// Whether the value for this property is invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    invalid: Option<bool>,
    /// Reason for the value being invalid, if it is.
    #[serde(skip_serializing_if = "Option::is_none")]
    invalidReason: Option<Cow<'a, str>>,
}

impl<'a> AXValueSource<'a> {
    pub fn builder() -> AXValueSourceBuilder<'a> { AXValueSourceBuilder::default() }
    pub fn type_(&self) -> &AXValueSourceType { &self.type_ }
    pub fn value(&self) -> Option<&AXValue<'a>> { self.value.as_ref() }
    pub fn attribute(&self) -> Option<&str> { self.attribute.as_deref() }
    pub fn attributeValue(&self) -> Option<&AXValue<'a>> { self.attributeValue.as_ref() }
    pub fn superseded(&self) -> Option<bool> { self.superseded }
    pub fn nativeSource(&self) -> Option<&AXValueNativeSourceType> { self.nativeSource.as_ref() }
    pub fn nativeSourceValue(&self) -> Option<&AXValue<'a>> { self.nativeSourceValue.as_ref() }
    pub fn invalid(&self) -> Option<bool> { self.invalid }
    pub fn invalidReason(&self) -> Option<&str> { self.invalidReason.as_deref() }
}

#[derive(Default)]
pub struct AXValueSourceBuilder<'a> {
    type_: Option<AXValueSourceType>,
    value: Option<AXValue<'a>>,
    attribute: Option<Cow<'a, str>>,
    attributeValue: Option<AXValue<'a>>,
    superseded: Option<bool>,
    nativeSource: Option<AXValueNativeSourceType>,
    nativeSourceValue: Option<AXValue<'a>>,
    invalid: Option<bool>,
    invalidReason: Option<Cow<'a, str>>,
}

impl<'a> AXValueSourceBuilder<'a> {
    /// What type of source this is.
    pub fn type_(mut self, type_: AXValueSourceType) -> Self { self.type_ = Some(type_); self }
    /// The value of this property source.
    pub fn value(mut self, value: AXValue<'a>) -> Self { self.value = Some(value); self }
    /// The name of the relevant attribute, if any.
    pub fn attribute(mut self, attribute: impl Into<Cow<'a, str>>) -> Self { self.attribute = Some(attribute.into()); self }
    /// The value of the relevant attribute, if any.
    pub fn attributeValue(mut self, attributeValue: AXValue<'a>) -> Self { self.attributeValue = Some(attributeValue); self }
    /// Whether this source is superseded by a higher priority source.
    pub fn superseded(mut self, superseded: bool) -> Self { self.superseded = Some(superseded); self }
    /// The native markup source for this value, e.g. a '<label>' element.
    pub fn nativeSource(mut self, nativeSource: AXValueNativeSourceType) -> Self { self.nativeSource = Some(nativeSource); self }
    /// The value, such as a node or node list, of the native source.
    pub fn nativeSourceValue(mut self, nativeSourceValue: AXValue<'a>) -> Self { self.nativeSourceValue = Some(nativeSourceValue); self }
    /// Whether the value for this property is invalid.
    pub fn invalid(mut self, invalid: bool) -> Self { self.invalid = Some(invalid); self }
    /// Reason for the value being invalid, if it is.
    pub fn invalidReason(mut self, invalidReason: impl Into<Cow<'a, str>>) -> Self { self.invalidReason = Some(invalidReason.into()); self }
    pub fn build(self) -> AXValueSource<'a> {
        AXValueSource {
            type_: self.type_.unwrap_or_default(),
            value: self.value,
            attribute: self.attribute,
            attributeValue: self.attributeValue,
            superseded: self.superseded,
            nativeSource: self.nativeSource,
            nativeSourceValue: self.nativeSourceValue,
            invalid: self.invalid,
            invalidReason: self.invalidReason,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXRelatedNode<'a> {
    /// The BackendNodeId of the related DOM node.
    backendDOMNodeId: crate::dom::BackendNodeId,
    /// The IDRef value provided, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    idref: Option<Cow<'a, str>>,
    /// The text alternative of this node in the current context.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Cow<'a, str>>,
}

impl<'a> AXRelatedNode<'a> {
    pub fn builder() -> AXRelatedNodeBuilder<'a> { AXRelatedNodeBuilder::default() }
    pub fn backendDOMNodeId(&self) -> &crate::dom::BackendNodeId { &self.backendDOMNodeId }
    pub fn idref(&self) -> Option<&str> { self.idref.as_deref() }
    pub fn text(&self) -> Option<&str> { self.text.as_deref() }
}

#[derive(Default)]
pub struct AXRelatedNodeBuilder<'a> {
    backendDOMNodeId: Option<crate::dom::BackendNodeId>,
    idref: Option<Cow<'a, str>>,
    text: Option<Cow<'a, str>>,
}

impl<'a> AXRelatedNodeBuilder<'a> {
    /// The BackendNodeId of the related DOM node.
    pub fn backendDOMNodeId(mut self, backendDOMNodeId: crate::dom::BackendNodeId) -> Self { self.backendDOMNodeId = Some(backendDOMNodeId); self }
    /// The IDRef value provided, if any.
    pub fn idref(mut self, idref: impl Into<Cow<'a, str>>) -> Self { self.idref = Some(idref.into()); self }
    /// The text alternative of this node in the current context.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> AXRelatedNode<'a> {
        AXRelatedNode {
            backendDOMNodeId: self.backendDOMNodeId.unwrap_or_default(),
            idref: self.idref,
            text: self.text,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXProperty<'a> {
    /// The name of this property.
    name: AXPropertyName,
    /// The value of this property.
    value: AXValue<'a>,
}

impl<'a> AXProperty<'a> {
    pub fn builder() -> AXPropertyBuilder<'a> { AXPropertyBuilder::default() }
    pub fn name(&self) -> &AXPropertyName { &self.name }
    pub fn value(&self) -> &AXValue<'a> { &self.value }
}

#[derive(Default)]
pub struct AXPropertyBuilder<'a> {
    name: Option<AXPropertyName>,
    value: Option<AXValue<'a>>,
}

impl<'a> AXPropertyBuilder<'a> {
    /// The name of this property.
    pub fn name(mut self, name: AXPropertyName) -> Self { self.name = Some(name); self }
    /// The value of this property.
    pub fn value(mut self, value: AXValue<'a>) -> Self { self.value = Some(value); self }
    pub fn build(self) -> AXProperty<'a> {
        AXProperty {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}

/// A single computed AX property.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXValue<'a> {
    /// The type of this value.
    #[serde(rename = "type")]
    type_: AXValueType,
    /// The computed value of this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<JsonValue>,
    /// One or more related nodes, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    relatedNodes: Option<Vec<AXRelatedNode<'a>>>,
    /// The sources which contributed to the computation of this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<Vec<AXValueSource<'a>>>,
}

impl<'a> AXValue<'a> {
    pub fn builder() -> AXValueBuilder<'a> { AXValueBuilder::default() }
    pub fn type_(&self) -> &AXValueType { &self.type_ }
    pub fn value(&self) -> Option<&JsonValue> { self.value.as_ref() }
    pub fn relatedNodes(&self) -> Option<&[AXRelatedNode<'a>]> { self.relatedNodes.as_deref() }
    pub fn sources(&self) -> Option<&[AXValueSource<'a>]> { self.sources.as_deref() }
}

#[derive(Default)]
pub struct AXValueBuilder<'a> {
    type_: Option<AXValueType>,
    value: Option<JsonValue>,
    relatedNodes: Option<Vec<AXRelatedNode<'a>>>,
    sources: Option<Vec<AXValueSource<'a>>>,
}

impl<'a> AXValueBuilder<'a> {
    /// The type of this value.
    pub fn type_(mut self, type_: AXValueType) -> Self { self.type_ = Some(type_); self }
    /// The computed value of this property.
    pub fn value(mut self, value: JsonValue) -> Self { self.value = Some(value); self }
    /// One or more related nodes, if applicable.
    pub fn relatedNodes(mut self, relatedNodes: Vec<AXRelatedNode<'a>>) -> Self { self.relatedNodes = Some(relatedNodes); self }
    /// The sources which contributed to the computation of this property.
    pub fn sources(mut self, sources: Vec<AXValueSource<'a>>) -> Self { self.sources = Some(sources); self }
    pub fn build(self) -> AXValue<'a> {
        AXValue {
            type_: self.type_.unwrap_or_default(),
            value: self.value,
            relatedNodes: self.relatedNodes,
            sources: self.sources,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXNode<'a> {
    /// Unique identifier for this node.
    nodeId: AXNodeId<'a>,
    /// Whether this node is ignored for accessibility
    ignored: bool,
    /// Collection of reasons why this node is hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    ignoredReasons: Option<Vec<AXProperty<'a>>>,
    /// This 'Node''s role, whether explicit or implicit.
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<AXValue<'a>>,
    /// This 'Node''s Chrome raw role.
    #[serde(skip_serializing_if = "Option::is_none")]
    chromeRole: Option<AXValue<'a>>,
    /// The accessible name for this 'Node'.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<AXValue<'a>>,
    /// The accessible description for this 'Node'.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<AXValue<'a>>,
    /// The value for this 'Node'.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<AXValue<'a>>,
    /// All other properties
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<AXProperty<'a>>>,
    /// ID for this node's parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    parentId: Option<AXNodeId<'a>>,
    /// IDs for each of this node's child nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    childIds: Option<Vec<AXNodeId<'a>>>,
    /// The backend ID for the associated DOM node, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendDOMNodeId: Option<crate::dom::BackendNodeId>,
    /// The frame ID for the frame associated with this nodes document.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> AXNode<'a> {
    pub fn builder() -> AXNodeBuilder<'a> { AXNodeBuilder::default() }
    pub fn nodeId(&self) -> &AXNodeId<'a> { &self.nodeId }
    pub fn ignored(&self) -> bool { self.ignored }
    pub fn ignoredReasons(&self) -> Option<&[AXProperty<'a>]> { self.ignoredReasons.as_deref() }
    pub fn role(&self) -> Option<&AXValue<'a>> { self.role.as_ref() }
    pub fn chromeRole(&self) -> Option<&AXValue<'a>> { self.chromeRole.as_ref() }
    pub fn name(&self) -> Option<&AXValue<'a>> { self.name.as_ref() }
    pub fn description(&self) -> Option<&AXValue<'a>> { self.description.as_ref() }
    pub fn value(&self) -> Option<&AXValue<'a>> { self.value.as_ref() }
    pub fn properties(&self) -> Option<&[AXProperty<'a>]> { self.properties.as_deref() }
    pub fn parentId(&self) -> Option<&AXNodeId<'a>> { self.parentId.as_ref() }
    pub fn childIds(&self) -> Option<&[AXNodeId<'a>]> { self.childIds.as_deref() }
    pub fn backendDOMNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendDOMNodeId.as_ref() }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
}

#[derive(Default)]
pub struct AXNodeBuilder<'a> {
    nodeId: Option<AXNodeId<'a>>,
    ignored: Option<bool>,
    ignoredReasons: Option<Vec<AXProperty<'a>>>,
    role: Option<AXValue<'a>>,
    chromeRole: Option<AXValue<'a>>,
    name: Option<AXValue<'a>>,
    description: Option<AXValue<'a>>,
    value: Option<AXValue<'a>>,
    properties: Option<Vec<AXProperty<'a>>>,
    parentId: Option<AXNodeId<'a>>,
    childIds: Option<Vec<AXNodeId<'a>>>,
    backendDOMNodeId: Option<crate::dom::BackendNodeId>,
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> AXNodeBuilder<'a> {
    /// Unique identifier for this node.
    pub fn nodeId(mut self, nodeId: AXNodeId<'a>) -> Self { self.nodeId = Some(nodeId); self }
    /// Whether this node is ignored for accessibility
    pub fn ignored(mut self, ignored: bool) -> Self { self.ignored = Some(ignored); self }
    /// Collection of reasons why this node is hidden.
    pub fn ignoredReasons(mut self, ignoredReasons: Vec<AXProperty<'a>>) -> Self { self.ignoredReasons = Some(ignoredReasons); self }
    /// This 'Node''s role, whether explicit or implicit.
    pub fn role(mut self, role: AXValue<'a>) -> Self { self.role = Some(role); self }
    /// This 'Node''s Chrome raw role.
    pub fn chromeRole(mut self, chromeRole: AXValue<'a>) -> Self { self.chromeRole = Some(chromeRole); self }
    /// The accessible name for this 'Node'.
    pub fn name(mut self, name: AXValue<'a>) -> Self { self.name = Some(name); self }
    /// The accessible description for this 'Node'.
    pub fn description(mut self, description: AXValue<'a>) -> Self { self.description = Some(description); self }
    /// The value for this 'Node'.
    pub fn value(mut self, value: AXValue<'a>) -> Self { self.value = Some(value); self }
    /// All other properties
    pub fn properties(mut self, properties: Vec<AXProperty<'a>>) -> Self { self.properties = Some(properties); self }
    /// ID for this node's parent.
    pub fn parentId(mut self, parentId: AXNodeId<'a>) -> Self { self.parentId = Some(parentId); self }
    /// IDs for each of this node's child nodes.
    pub fn childIds(mut self, childIds: Vec<AXNodeId<'a>>) -> Self { self.childIds = Some(childIds); self }
    /// The backend ID for the associated DOM node, if any.
    pub fn backendDOMNodeId(mut self, backendDOMNodeId: crate::dom::BackendNodeId) -> Self { self.backendDOMNodeId = Some(backendDOMNodeId); self }
    /// The frame ID for the frame associated with this nodes document.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> AXNode<'a> {
        AXNode {
            nodeId: self.nodeId.unwrap_or_default(),
            ignored: self.ignored.unwrap_or_default(),
            ignoredReasons: self.ignoredReasons,
            role: self.role,
            chromeRole: self.chromeRole,
            name: self.name,
            description: self.description,
            value: self.value,
            properties: self.properties,
            parentId: self.parentId,
            childIds: self.childIds,
            backendDOMNodeId: self.backendDOMNodeId,
            frameId: self.frameId,
        }
    }
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

impl DisableParams { pub const METHOD: &'static str = "Accessibility.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Accessibility.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct EnableParamsBuilder {}

impl EnableParamsBuilder {
    pub fn build(self) -> EnableParams {
        EnableParams {}
    }
}

impl EnableParams { pub const METHOD: &'static str = "Accessibility.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Accessibility.enable";
    type Response = crate::EmptyReturns;
}

/// Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPartialAXTreeParams<'a> {
    /// Identifier of the node to get the partial accessibility tree for.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to get the partial accessibility tree for.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper to get the partial accessibility tree for.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Whether to fetch this node's ancestors, siblings and children. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    fetchRelatives: Option<bool>,
}

impl<'a> GetPartialAXTreeParams<'a> {
    pub fn builder() -> GetPartialAXTreeParamsBuilder<'a> { GetPartialAXTreeParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&crate::dom::NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
    pub fn fetchRelatives(&self) -> Option<bool> { self.fetchRelatives }
}

#[derive(Default)]
pub struct GetPartialAXTreeParamsBuilder<'a> {
    nodeId: Option<crate::dom::NodeId>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    fetchRelatives: Option<bool>,
}

impl<'a> GetPartialAXTreeParamsBuilder<'a> {
    /// Identifier of the node to get the partial accessibility tree for.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node to get the partial accessibility tree for.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper to get the partial accessibility tree for.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    /// Whether to fetch this node's ancestors, siblings and children. Defaults to true.
    pub fn fetchRelatives(mut self, fetchRelatives: bool) -> Self { self.fetchRelatives = Some(fetchRelatives); self }
    pub fn build(self) -> GetPartialAXTreeParams<'a> {
        GetPartialAXTreeParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
            fetchRelatives: self.fetchRelatives,
        }
    }
}

/// Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPartialAXTreeReturns<'a> {
    /// The 'Accessibility.AXNode' for this DOM node, if it exists, plus its ancestors, siblings and
    /// children, if requested.
    nodes: Vec<AXNode<'a>>,
}

impl<'a> GetPartialAXTreeReturns<'a> {
    pub fn builder() -> GetPartialAXTreeReturnsBuilder<'a> { GetPartialAXTreeReturnsBuilder::default() }
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}

#[derive(Default)]
pub struct GetPartialAXTreeReturnsBuilder<'a> {
    nodes: Option<Vec<AXNode<'a>>>,
}

impl<'a> GetPartialAXTreeReturnsBuilder<'a> {
    /// The 'Accessibility.AXNode' for this DOM node, if it exists, plus its ancestors, siblings and
    /// children, if requested.
    pub fn nodes(mut self, nodes: Vec<AXNode<'a>>) -> Self { self.nodes = Some(nodes); self }
    pub fn build(self) -> GetPartialAXTreeReturns<'a> {
        GetPartialAXTreeReturns {
            nodes: self.nodes.unwrap_or_default(),
        }
    }
}

impl<'a> GetPartialAXTreeParams<'a> { pub const METHOD: &'static str = "Accessibility.getPartialAXTree"; }

impl<'a> crate::CdpCommand<'a> for GetPartialAXTreeParams<'a> {
    const METHOD: &'static str = "Accessibility.getPartialAXTree";
    type Response = GetPartialAXTreeReturns<'a>;
}

/// Fetches the entire accessibility tree for the root Document

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFullAXTreeParams<'a> {
    /// The maximum depth at which descendants of the root node should be retrieved.
    /// If omitted, the full tree is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<i64>,
    /// The frame for whose document the AX tree should be retrieved.
    /// If omitted, the root frame is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetFullAXTreeParams<'a> {
    pub fn builder() -> GetFullAXTreeParamsBuilder<'a> { GetFullAXTreeParamsBuilder::default() }
    pub fn depth(&self) -> Option<i64> { self.depth }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
}

#[derive(Default)]
pub struct GetFullAXTreeParamsBuilder<'a> {
    depth: Option<i64>,
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetFullAXTreeParamsBuilder<'a> {
    /// The maximum depth at which descendants of the root node should be retrieved.
    /// If omitted, the full tree is returned.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// The frame for whose document the AX tree should be retrieved.
    /// If omitted, the root frame is used.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetFullAXTreeParams<'a> {
        GetFullAXTreeParams {
            depth: self.depth,
            frameId: self.frameId,
        }
    }
}

/// Fetches the entire accessibility tree for the root Document

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFullAXTreeReturns<'a> {
    nodes: Vec<AXNode<'a>>,
}

impl<'a> GetFullAXTreeReturns<'a> {
    pub fn builder() -> GetFullAXTreeReturnsBuilder<'a> { GetFullAXTreeReturnsBuilder::default() }
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}

#[derive(Default)]
pub struct GetFullAXTreeReturnsBuilder<'a> {
    nodes: Option<Vec<AXNode<'a>>>,
}

impl<'a> GetFullAXTreeReturnsBuilder<'a> {
    pub fn nodes(mut self, nodes: Vec<AXNode<'a>>) -> Self { self.nodes = Some(nodes); self }
    pub fn build(self) -> GetFullAXTreeReturns<'a> {
        GetFullAXTreeReturns {
            nodes: self.nodes.unwrap_or_default(),
        }
    }
}

impl<'a> GetFullAXTreeParams<'a> { pub const METHOD: &'static str = "Accessibility.getFullAXTree"; }

impl<'a> crate::CdpCommand<'a> for GetFullAXTreeParams<'a> {
    const METHOD: &'static str = "Accessibility.getFullAXTree";
    type Response = GetFullAXTreeReturns<'a>;
}

/// Fetches the root node.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRootAXNodeParams<'a> {
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetRootAXNodeParams<'a> {
    pub fn builder() -> GetRootAXNodeParamsBuilder<'a> { GetRootAXNodeParamsBuilder::default() }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
}

#[derive(Default)]
pub struct GetRootAXNodeParamsBuilder<'a> {
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetRootAXNodeParamsBuilder<'a> {
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetRootAXNodeParams<'a> {
        GetRootAXNodeParams {
            frameId: self.frameId,
        }
    }
}

/// Fetches the root node.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRootAXNodeReturns<'a> {
    node: AXNode<'a>,
}

impl<'a> GetRootAXNodeReturns<'a> {
    pub fn builder() -> GetRootAXNodeReturnsBuilder<'a> { GetRootAXNodeReturnsBuilder::default() }
    pub fn node(&self) -> &AXNode<'a> { &self.node }
}

#[derive(Default)]
pub struct GetRootAXNodeReturnsBuilder<'a> {
    node: Option<AXNode<'a>>,
}

impl<'a> GetRootAXNodeReturnsBuilder<'a> {
    pub fn node(mut self, node: AXNode<'a>) -> Self { self.node = Some(node); self }
    pub fn build(self) -> GetRootAXNodeReturns<'a> {
        GetRootAXNodeReturns {
            node: self.node.unwrap_or_default(),
        }
    }
}

impl<'a> GetRootAXNodeParams<'a> { pub const METHOD: &'static str = "Accessibility.getRootAXNode"; }

impl<'a> crate::CdpCommand<'a> for GetRootAXNodeParams<'a> {
    const METHOD: &'static str = "Accessibility.getRootAXNode";
    type Response = GetRootAXNodeReturns<'a>;
}

/// Fetches a node and all ancestors up to and including the root.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAXNodeAndAncestorsParams<'a> {
    /// Identifier of the node to get.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to get.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper to get.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetAXNodeAndAncestorsParams<'a> {
    pub fn builder() -> GetAXNodeAndAncestorsParamsBuilder<'a> { GetAXNodeAndAncestorsParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&crate::dom::NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
}

#[derive(Default)]
pub struct GetAXNodeAndAncestorsParamsBuilder<'a> {
    nodeId: Option<crate::dom::NodeId>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetAXNodeAndAncestorsParamsBuilder<'a> {
    /// Identifier of the node to get.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node to get.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper to get.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    pub fn build(self) -> GetAXNodeAndAncestorsParams<'a> {
        GetAXNodeAndAncestorsParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
        }
    }
}

/// Fetches a node and all ancestors up to and including the root.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAXNodeAndAncestorsReturns<'a> {
    nodes: Vec<AXNode<'a>>,
}

impl<'a> GetAXNodeAndAncestorsReturns<'a> {
    pub fn builder() -> GetAXNodeAndAncestorsReturnsBuilder<'a> { GetAXNodeAndAncestorsReturnsBuilder::default() }
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}

#[derive(Default)]
pub struct GetAXNodeAndAncestorsReturnsBuilder<'a> {
    nodes: Option<Vec<AXNode<'a>>>,
}

impl<'a> GetAXNodeAndAncestorsReturnsBuilder<'a> {
    pub fn nodes(mut self, nodes: Vec<AXNode<'a>>) -> Self { self.nodes = Some(nodes); self }
    pub fn build(self) -> GetAXNodeAndAncestorsReturns<'a> {
        GetAXNodeAndAncestorsReturns {
            nodes: self.nodes.unwrap_or_default(),
        }
    }
}

impl<'a> GetAXNodeAndAncestorsParams<'a> { pub const METHOD: &'static str = "Accessibility.getAXNodeAndAncestors"; }

impl<'a> crate::CdpCommand<'a> for GetAXNodeAndAncestorsParams<'a> {
    const METHOD: &'static str = "Accessibility.getAXNodeAndAncestors";
    type Response = GetAXNodeAndAncestorsReturns<'a>;
}

/// Fetches a particular accessibility node by AXNodeId.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChildAXNodesParams<'a> {
    id: AXNodeId<'a>,
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetChildAXNodesParams<'a> {
    pub fn builder() -> GetChildAXNodesParamsBuilder<'a> { GetChildAXNodesParamsBuilder::default() }
    pub fn id(&self) -> &AXNodeId<'a> { &self.id }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
}

#[derive(Default)]
pub struct GetChildAXNodesParamsBuilder<'a> {
    id: Option<AXNodeId<'a>>,
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetChildAXNodesParamsBuilder<'a> {
    pub fn id(mut self, id: AXNodeId<'a>) -> Self { self.id = Some(id); self }
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetChildAXNodesParams<'a> {
        GetChildAXNodesParams {
            id: self.id.unwrap_or_default(),
            frameId: self.frameId,
        }
    }
}

/// Fetches a particular accessibility node by AXNodeId.
/// Requires 'enable()' to have been called previously.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChildAXNodesReturns<'a> {
    nodes: Vec<AXNode<'a>>,
}

impl<'a> GetChildAXNodesReturns<'a> {
    pub fn builder() -> GetChildAXNodesReturnsBuilder<'a> { GetChildAXNodesReturnsBuilder::default() }
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}

#[derive(Default)]
pub struct GetChildAXNodesReturnsBuilder<'a> {
    nodes: Option<Vec<AXNode<'a>>>,
}

impl<'a> GetChildAXNodesReturnsBuilder<'a> {
    pub fn nodes(mut self, nodes: Vec<AXNode<'a>>) -> Self { self.nodes = Some(nodes); self }
    pub fn build(self) -> GetChildAXNodesReturns<'a> {
        GetChildAXNodesReturns {
            nodes: self.nodes.unwrap_or_default(),
        }
    }
}

impl<'a> GetChildAXNodesParams<'a> { pub const METHOD: &'static str = "Accessibility.getChildAXNodes"; }

impl<'a> crate::CdpCommand<'a> for GetChildAXNodesParams<'a> {
    const METHOD: &'static str = "Accessibility.getChildAXNodes";
    type Response = GetChildAXNodesReturns<'a>;
}

/// Query a DOM node's accessibility subtree for accessible name and role.
/// This command computes the name and role for all nodes in the subtree, including those that are
/// ignored for accessibility, and returns those that match the specified name and role. If no DOM
/// node is specified, or the DOM node does not exist, the command returns an error. If neither
/// 'accessibleName' or 'role' is specified, it returns all the accessibility nodes in the subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryAXTreeParams<'a> {
    /// Identifier of the node for the root to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<crate::dom::NodeId>,
    /// Identifier of the backend node for the root to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper for the root to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Find nodes with this computed name.
    #[serde(skip_serializing_if = "Option::is_none")]
    accessibleName: Option<Cow<'a, str>>,
    /// Find nodes with this computed role.
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<Cow<'a, str>>,
}

impl<'a> QueryAXTreeParams<'a> {
    pub fn builder() -> QueryAXTreeParamsBuilder<'a> { QueryAXTreeParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&crate::dom::NodeId> { self.nodeId.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn objectId(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.objectId.as_ref() }
    pub fn accessibleName(&self) -> Option<&str> { self.accessibleName.as_deref() }
    pub fn role(&self) -> Option<&str> { self.role.as_deref() }
}

#[derive(Default)]
pub struct QueryAXTreeParamsBuilder<'a> {
    nodeId: Option<crate::dom::NodeId>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    objectId: Option<crate::runtime::RemoteObjectId<'a>>,
    accessibleName: Option<Cow<'a, str>>,
    role: Option<Cow<'a, str>>,
}

impl<'a> QueryAXTreeParamsBuilder<'a> {
    /// Identifier of the node for the root to query.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Identifier of the backend node for the root to query.
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    /// JavaScript object id of the node wrapper for the root to query.
    pub fn objectId(mut self, objectId: crate::runtime::RemoteObjectId<'a>) -> Self { self.objectId = Some(objectId); self }
    /// Find nodes with this computed name.
    pub fn accessibleName(mut self, accessibleName: impl Into<Cow<'a, str>>) -> Self { self.accessibleName = Some(accessibleName.into()); self }
    /// Find nodes with this computed role.
    pub fn role(mut self, role: impl Into<Cow<'a, str>>) -> Self { self.role = Some(role.into()); self }
    pub fn build(self) -> QueryAXTreeParams<'a> {
        QueryAXTreeParams {
            nodeId: self.nodeId,
            backendNodeId: self.backendNodeId,
            objectId: self.objectId,
            accessibleName: self.accessibleName,
            role: self.role,
        }
    }
}

/// Query a DOM node's accessibility subtree for accessible name and role.
/// This command computes the name and role for all nodes in the subtree, including those that are
/// ignored for accessibility, and returns those that match the specified name and role. If no DOM
/// node is specified, or the DOM node does not exist, the command returns an error. If neither
/// 'accessibleName' or 'role' is specified, it returns all the accessibility nodes in the subtree.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryAXTreeReturns<'a> {
    /// A list of 'Accessibility.AXNode' matching the specified attributes,
    /// including nodes that are ignored for accessibility.
    nodes: Vec<AXNode<'a>>,
}

impl<'a> QueryAXTreeReturns<'a> {
    pub fn builder() -> QueryAXTreeReturnsBuilder<'a> { QueryAXTreeReturnsBuilder::default() }
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}

#[derive(Default)]
pub struct QueryAXTreeReturnsBuilder<'a> {
    nodes: Option<Vec<AXNode<'a>>>,
}

impl<'a> QueryAXTreeReturnsBuilder<'a> {
    /// A list of 'Accessibility.AXNode' matching the specified attributes,
    /// including nodes that are ignored for accessibility.
    pub fn nodes(mut self, nodes: Vec<AXNode<'a>>) -> Self { self.nodes = Some(nodes); self }
    pub fn build(self) -> QueryAXTreeReturns<'a> {
        QueryAXTreeReturns {
            nodes: self.nodes.unwrap_or_default(),
        }
    }
}

impl<'a> QueryAXTreeParams<'a> { pub const METHOD: &'static str = "Accessibility.queryAXTree"; }

impl<'a> crate::CdpCommand<'a> for QueryAXTreeParams<'a> {
    const METHOD: &'static str = "Accessibility.queryAXTree";
    type Response = QueryAXTreeReturns<'a>;
}
