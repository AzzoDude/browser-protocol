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
    #[serde(skip_serializing_if = "Option::is_none", rename = "attributeValue")]
    attribute_value: Option<AXValue<'a>>,
    /// Whether this source is superseded by a higher priority source.
    #[serde(skip_serializing_if = "Option::is_none")]
    superseded: Option<bool>,
    /// The native markup source for this value, e.g. a '\<label\>' element.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nativeSource")]
    native_source: Option<AXValueNativeSourceType>,
    /// The value, such as a node or node list, of the native source.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nativeSourceValue")]
    native_source_value: Option<AXValue<'a>>,
    /// Whether the value for this property is invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    invalid: Option<bool>,
    /// Reason for the value being invalid, if it is.
    #[serde(skip_serializing_if = "Option::is_none", rename = "invalidReason")]
    invalid_reason: Option<Cow<'a, str>>,
}

impl<'a> AXValueSource<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: What type of source this is.
    pub fn builder(type_: impl Into<AXValueSourceType>) -> AXValueSourceBuilder<'a> {
        AXValueSourceBuilder {
            type_: type_.into(),
            value: None,
            attribute: None,
            attribute_value: None,
            superseded: None,
            native_source: None,
            native_source_value: None,
            invalid: None,
            invalid_reason: None,
        }
    }
    /// What type of source this is.
    pub fn type_(&self) -> &AXValueSourceType { &self.type_ }
    /// The value of this property source.
    pub fn value(&self) -> Option<&AXValue<'a>> { self.value.as_ref() }
    /// The name of the relevant attribute, if any.
    pub fn attribute(&self) -> Option<&str> { self.attribute.as_deref() }
    /// The value of the relevant attribute, if any.
    pub fn attribute_value(&self) -> Option<&AXValue<'a>> { self.attribute_value.as_ref() }
    /// Whether this source is superseded by a higher priority source.
    pub fn superseded(&self) -> Option<bool> { self.superseded }
    /// The native markup source for this value, e.g. a '\<label\>' element.
    pub fn native_source(&self) -> Option<&AXValueNativeSourceType> { self.native_source.as_ref() }
    /// The value, such as a node or node list, of the native source.
    pub fn native_source_value(&self) -> Option<&AXValue<'a>> { self.native_source_value.as_ref() }
    /// Whether the value for this property is invalid.
    pub fn invalid(&self) -> Option<bool> { self.invalid }
    /// Reason for the value being invalid, if it is.
    pub fn invalid_reason(&self) -> Option<&str> { self.invalid_reason.as_deref() }
}


pub struct AXValueSourceBuilder<'a> {
    type_: AXValueSourceType,
    value: Option<AXValue<'a>>,
    attribute: Option<Cow<'a, str>>,
    attribute_value: Option<AXValue<'a>>,
    superseded: Option<bool>,
    native_source: Option<AXValueNativeSourceType>,
    native_source_value: Option<AXValue<'a>>,
    invalid: Option<bool>,
    invalid_reason: Option<Cow<'a, str>>,
}

impl<'a> AXValueSourceBuilder<'a> {
    /// The value of this property source.
    pub fn value(mut self, value: AXValue<'a>) -> Self { self.value = Some(value); self }
    /// The name of the relevant attribute, if any.
    pub fn attribute(mut self, attribute: impl Into<Cow<'a, str>>) -> Self { self.attribute = Some(attribute.into()); self }
    /// The value of the relevant attribute, if any.
    pub fn attribute_value(mut self, attribute_value: AXValue<'a>) -> Self { self.attribute_value = Some(attribute_value); self }
    /// Whether this source is superseded by a higher priority source.
    pub fn superseded(mut self, superseded: bool) -> Self { self.superseded = Some(superseded); self }
    /// The native markup source for this value, e.g. a '\<label\>' element.
    pub fn native_source(mut self, native_source: impl Into<AXValueNativeSourceType>) -> Self { self.native_source = Some(native_source.into()); self }
    /// The value, such as a node or node list, of the native source.
    pub fn native_source_value(mut self, native_source_value: AXValue<'a>) -> Self { self.native_source_value = Some(native_source_value); self }
    /// Whether the value for this property is invalid.
    pub fn invalid(mut self, invalid: bool) -> Self { self.invalid = Some(invalid); self }
    /// Reason for the value being invalid, if it is.
    pub fn invalid_reason(mut self, invalid_reason: impl Into<Cow<'a, str>>) -> Self { self.invalid_reason = Some(invalid_reason.into()); self }
    pub fn build(self) -> AXValueSource<'a> {
        AXValueSource {
            type_: self.type_,
            value: self.value,
            attribute: self.attribute,
            attribute_value: self.attribute_value,
            superseded: self.superseded,
            native_source: self.native_source,
            native_source_value: self.native_source_value,
            invalid: self.invalid,
            invalid_reason: self.invalid_reason,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AXRelatedNode<'a> {
    /// The BackendNodeId of the related DOM node.
    #[serde(rename = "backendDOMNodeId")]
    backend_dom_node_id: crate::dom::BackendNodeId,
    /// The IDRef value provided, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    idref: Option<Cow<'a, str>>,
    /// The text alternative of this node in the current context.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Cow<'a, str>>,
}

impl<'a> AXRelatedNode<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `backend_dom_node_id`: The BackendNodeId of the related DOM node.
    pub fn builder(backend_dom_node_id: crate::dom::BackendNodeId) -> AXRelatedNodeBuilder<'a> {
        AXRelatedNodeBuilder {
            backend_dom_node_id: backend_dom_node_id,
            idref: None,
            text: None,
        }
    }
    /// The BackendNodeId of the related DOM node.
    pub fn backend_dom_node_id(&self) -> &crate::dom::BackendNodeId { &self.backend_dom_node_id }
    /// The IDRef value provided, if any.
    pub fn idref(&self) -> Option<&str> { self.idref.as_deref() }
    /// The text alternative of this node in the current context.
    pub fn text(&self) -> Option<&str> { self.text.as_deref() }
}


pub struct AXRelatedNodeBuilder<'a> {
    backend_dom_node_id: crate::dom::BackendNodeId,
    idref: Option<Cow<'a, str>>,
    text: Option<Cow<'a, str>>,
}

impl<'a> AXRelatedNodeBuilder<'a> {
    /// The IDRef value provided, if any.
    pub fn idref(mut self, idref: impl Into<Cow<'a, str>>) -> Self { self.idref = Some(idref.into()); self }
    /// The text alternative of this node in the current context.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> AXRelatedNode<'a> {
        AXRelatedNode {
            backend_dom_node_id: self.backend_dom_node_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: The name of this property.
    /// * `value`: The value of this property.
    pub fn builder(name: impl Into<AXPropertyName>, value: AXValue<'a>) -> AXPropertyBuilder<'a> {
        AXPropertyBuilder {
            name: name.into(),
            value: value,
        }
    }
    /// The name of this property.
    pub fn name(&self) -> &AXPropertyName { &self.name }
    /// The value of this property.
    pub fn value(&self) -> &AXValue<'a> { &self.value }
}


pub struct AXPropertyBuilder<'a> {
    name: AXPropertyName,
    value: AXValue<'a>,
}

impl<'a> AXPropertyBuilder<'a> {
    pub fn build(self) -> AXProperty<'a> {
        AXProperty {
            name: self.name,
            value: self.value,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "relatedNodes")]
    related_nodes: Option<Vec<AXRelatedNode<'a>>>,
    /// The sources which contributed to the computation of this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<Vec<AXValueSource<'a>>>,
}

impl<'a> AXValue<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: The type of this value.
    pub fn builder(type_: impl Into<AXValueType>) -> AXValueBuilder<'a> {
        AXValueBuilder {
            type_: type_.into(),
            value: None,
            related_nodes: None,
            sources: None,
        }
    }
    /// The type of this value.
    pub fn type_(&self) -> &AXValueType { &self.type_ }
    /// The computed value of this property.
    pub fn value(&self) -> Option<&JsonValue> { self.value.as_ref() }
    /// One or more related nodes, if applicable.
    pub fn related_nodes(&self) -> Option<&[AXRelatedNode<'a>]> { self.related_nodes.as_deref() }
    /// The sources which contributed to the computation of this property.
    pub fn sources(&self) -> Option<&[AXValueSource<'a>]> { self.sources.as_deref() }
}


pub struct AXValueBuilder<'a> {
    type_: AXValueType,
    value: Option<JsonValue>,
    related_nodes: Option<Vec<AXRelatedNode<'a>>>,
    sources: Option<Vec<AXValueSource<'a>>>,
}

impl<'a> AXValueBuilder<'a> {
    /// The computed value of this property.
    pub fn value(mut self, value: JsonValue) -> Self { self.value = Some(value); self }
    /// One or more related nodes, if applicable.
    pub fn related_nodes(mut self, related_nodes: Vec<AXRelatedNode<'a>>) -> Self { self.related_nodes = Some(related_nodes); self }
    /// The sources which contributed to the computation of this property.
    pub fn sources(mut self, sources: Vec<AXValueSource<'a>>) -> Self { self.sources = Some(sources); self }
    pub fn build(self) -> AXValue<'a> {
        AXValue {
            type_: self.type_,
            value: self.value,
            related_nodes: self.related_nodes,
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
    #[serde(rename = "nodeId")]
    node_id: AXNodeId<'a>,
    /// Whether this node is ignored for accessibility
    ignored: bool,
    /// Collection of reasons why this node is hidden.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ignoredReasons")]
    ignored_reasons: Option<Vec<AXProperty<'a>>>,
    /// This 'Node''s role, whether explicit or implicit.
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<AXValue<'a>>,
    /// This 'Node''s Chrome raw role.
    #[serde(skip_serializing_if = "Option::is_none", rename = "chromeRole")]
    chrome_role: Option<AXValue<'a>>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    parent_id: Option<AXNodeId<'a>>,
    /// IDs for each of this node's child nodes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childIds")]
    child_ids: Option<Vec<AXNodeId<'a>>>,
    /// The backend ID for the associated DOM node, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendDOMNodeId")]
    backend_dom_node_id: Option<crate::dom::BackendNodeId>,
    /// The frame ID for the frame associated with this nodes document.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> AXNode<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Unique identifier for this node.
    /// * `ignored`: Whether this node is ignored for accessibility
    pub fn builder(node_id: impl Into<AXNodeId<'a>>, ignored: bool) -> AXNodeBuilder<'a> {
        AXNodeBuilder {
            node_id: node_id.into(),
            ignored: ignored,
            ignored_reasons: None,
            role: None,
            chrome_role: None,
            name: None,
            description: None,
            value: None,
            properties: None,
            parent_id: None,
            child_ids: None,
            backend_dom_node_id: None,
            frame_id: None,
        }
    }
    /// Unique identifier for this node.
    pub fn node_id(&self) -> &AXNodeId<'a> { &self.node_id }
    /// Whether this node is ignored for accessibility
    pub fn ignored(&self) -> bool { self.ignored }
    /// Collection of reasons why this node is hidden.
    pub fn ignored_reasons(&self) -> Option<&[AXProperty<'a>]> { self.ignored_reasons.as_deref() }
    /// This 'Node''s role, whether explicit or implicit.
    pub fn role(&self) -> Option<&AXValue<'a>> { self.role.as_ref() }
    /// This 'Node''s Chrome raw role.
    pub fn chrome_role(&self) -> Option<&AXValue<'a>> { self.chrome_role.as_ref() }
    /// The accessible name for this 'Node'.
    pub fn name(&self) -> Option<&AXValue<'a>> { self.name.as_ref() }
    /// The accessible description for this 'Node'.
    pub fn description(&self) -> Option<&AXValue<'a>> { self.description.as_ref() }
    /// The value for this 'Node'.
    pub fn value(&self) -> Option<&AXValue<'a>> { self.value.as_ref() }
    /// All other properties
    pub fn properties(&self) -> Option<&[AXProperty<'a>]> { self.properties.as_deref() }
    /// ID for this node's parent.
    pub fn parent_id(&self) -> Option<&AXNodeId<'a>> { self.parent_id.as_ref() }
    /// IDs for each of this node's child nodes.
    pub fn child_ids(&self) -> Option<&[AXNodeId<'a>]> { self.child_ids.as_deref() }
    /// The backend ID for the associated DOM node, if any.
    pub fn backend_dom_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_dom_node_id.as_ref() }
    /// The frame ID for the frame associated with this nodes document.
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
}


pub struct AXNodeBuilder<'a> {
    node_id: AXNodeId<'a>,
    ignored: bool,
    ignored_reasons: Option<Vec<AXProperty<'a>>>,
    role: Option<AXValue<'a>>,
    chrome_role: Option<AXValue<'a>>,
    name: Option<AXValue<'a>>,
    description: Option<AXValue<'a>>,
    value: Option<AXValue<'a>>,
    properties: Option<Vec<AXProperty<'a>>>,
    parent_id: Option<AXNodeId<'a>>,
    child_ids: Option<Vec<AXNodeId<'a>>>,
    backend_dom_node_id: Option<crate::dom::BackendNodeId>,
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> AXNodeBuilder<'a> {
    /// Collection of reasons why this node is hidden.
    pub fn ignored_reasons(mut self, ignored_reasons: Vec<AXProperty<'a>>) -> Self { self.ignored_reasons = Some(ignored_reasons); self }
    /// This 'Node''s role, whether explicit or implicit.
    pub fn role(mut self, role: AXValue<'a>) -> Self { self.role = Some(role); self }
    /// This 'Node''s Chrome raw role.
    pub fn chrome_role(mut self, chrome_role: AXValue<'a>) -> Self { self.chrome_role = Some(chrome_role); self }
    /// The accessible name for this 'Node'.
    pub fn name(mut self, name: AXValue<'a>) -> Self { self.name = Some(name); self }
    /// The accessible description for this 'Node'.
    pub fn description(mut self, description: AXValue<'a>) -> Self { self.description = Some(description); self }
    /// The value for this 'Node'.
    pub fn value(mut self, value: AXValue<'a>) -> Self { self.value = Some(value); self }
    /// All other properties
    pub fn properties(mut self, properties: Vec<AXProperty<'a>>) -> Self { self.properties = Some(properties); self }
    /// ID for this node's parent.
    pub fn parent_id(mut self, parent_id: impl Into<AXNodeId<'a>>) -> Self { self.parent_id = Some(parent_id.into()); self }
    /// IDs for each of this node's child nodes.
    pub fn child_ids(mut self, child_ids: Vec<AXNodeId<'a>>) -> Self { self.child_ids = Some(child_ids); self }
    /// The backend ID for the associated DOM node, if any.
    pub fn backend_dom_node_id(mut self, backend_dom_node_id: crate::dom::BackendNodeId) -> Self { self.backend_dom_node_id = Some(backend_dom_node_id); self }
    /// The frame ID for the frame associated with this nodes document.
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    pub fn build(self) -> AXNode<'a> {
        AXNode {
            node_id: self.node_id,
            ignored: self.ignored,
            ignored_reasons: self.ignored_reasons,
            role: self.role,
            chrome_role: self.chrome_role,
            name: self.name,
            description: self.description,
            value: self.value,
            properties: self.properties,
            parent_id: self.parent_id,
            child_ids: self.child_ids,
            backend_dom_node_id: self.backend_dom_node_id,
            frame_id: self.frame_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Accessibility.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Accessibility.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to get the partial accessibility tree for.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper to get the partial accessibility tree for.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Whether to fetch this node's ancestors, siblings and children. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fetchRelatives")]
    fetch_relatives: Option<bool>,
}

impl<'a> GetPartialAXTreeParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetPartialAXTreeParamsBuilder<'a> {
        GetPartialAXTreeParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_id: None,
            fetch_relatives: None,
        }
    }
    /// Identifier of the node to get the partial accessibility tree for.
    pub fn node_id(&self) -> Option<&crate::dom::NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node to get the partial accessibility tree for.
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper to get the partial accessibility tree for.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
    /// Whether to fetch this node's ancestors, siblings and children. Defaults to true.
    pub fn fetch_relatives(&self) -> Option<bool> { self.fetch_relatives }
}

#[derive(Default)]
pub struct GetPartialAXTreeParamsBuilder<'a> {
    node_id: Option<crate::dom::NodeId>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    fetch_relatives: Option<bool>,
}

impl<'a> GetPartialAXTreeParamsBuilder<'a> {
    /// Identifier of the node to get the partial accessibility tree for.
    pub fn node_id(mut self, node_id: crate::dom::NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node to get the partial accessibility tree for.
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper to get the partial accessibility tree for.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    /// Whether to fetch this node's ancestors, siblings and children. Defaults to true.
    pub fn fetch_relatives(mut self, fetch_relatives: bool) -> Self { self.fetch_relatives = Some(fetch_relatives); self }
    pub fn build(self) -> GetPartialAXTreeParams<'a> {
        GetPartialAXTreeParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
            fetch_relatives: self.fetch_relatives,
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
    /// Creates a builder for this type with the required parameters:
    /// * `nodes`: The `Accessibility.AXNode` for this DOM node, if it exists, plus its ancestors, siblings and children, if requested.
    pub fn builder(nodes: Vec<AXNode<'a>>) -> GetPartialAXTreeReturnsBuilder<'a> {
        GetPartialAXTreeReturnsBuilder {
            nodes: nodes,
        }
    }
    /// The 'Accessibility.AXNode' for this DOM node, if it exists, plus its ancestors, siblings and
    /// children, if requested.
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}


pub struct GetPartialAXTreeReturnsBuilder<'a> {
    nodes: Vec<AXNode<'a>>,
}

impl<'a> GetPartialAXTreeReturnsBuilder<'a> {
    pub fn build(self) -> GetPartialAXTreeReturns<'a> {
        GetPartialAXTreeReturns {
            nodes: self.nodes,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetFullAXTreeParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetFullAXTreeParamsBuilder<'a> {
        GetFullAXTreeParamsBuilder {
            depth: None,
            frame_id: None,
        }
    }
    /// The maximum depth at which descendants of the root node should be retrieved.
    /// If omitted, the full tree is returned.
    pub fn depth(&self) -> Option<i64> { self.depth }
    /// The frame for whose document the AX tree should be retrieved.
    /// If omitted, the root frame is used.
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
}

#[derive(Default)]
pub struct GetFullAXTreeParamsBuilder<'a> {
    depth: Option<i64>,
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetFullAXTreeParamsBuilder<'a> {
    /// The maximum depth at which descendants of the root node should be retrieved.
    /// If omitted, the full tree is returned.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// The frame for whose document the AX tree should be retrieved.
    /// If omitted, the root frame is used.
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    pub fn build(self) -> GetFullAXTreeParams<'a> {
        GetFullAXTreeParams {
            depth: self.depth,
            frame_id: self.frame_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `nodes`: 
    pub fn builder(nodes: Vec<AXNode<'a>>) -> GetFullAXTreeReturnsBuilder<'a> {
        GetFullAXTreeReturnsBuilder {
            nodes: nodes,
        }
    }
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}


pub struct GetFullAXTreeReturnsBuilder<'a> {
    nodes: Vec<AXNode<'a>>,
}

impl<'a> GetFullAXTreeReturnsBuilder<'a> {
    pub fn build(self) -> GetFullAXTreeReturns<'a> {
        GetFullAXTreeReturns {
            nodes: self.nodes,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetRootAXNodeParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetRootAXNodeParamsBuilder<'a> {
        GetRootAXNodeParamsBuilder {
            frame_id: None,
        }
    }
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
}

#[derive(Default)]
pub struct GetRootAXNodeParamsBuilder<'a> {
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetRootAXNodeParamsBuilder<'a> {
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    pub fn build(self) -> GetRootAXNodeParams<'a> {
        GetRootAXNodeParams {
            frame_id: self.frame_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `node`: 
    pub fn builder(node: AXNode<'a>) -> GetRootAXNodeReturnsBuilder<'a> {
        GetRootAXNodeReturnsBuilder {
            node: node,
        }
    }
    pub fn node(&self) -> &AXNode<'a> { &self.node }
}


pub struct GetRootAXNodeReturnsBuilder<'a> {
    node: AXNode<'a>,
}

impl<'a> GetRootAXNodeReturnsBuilder<'a> {
    pub fn build(self) -> GetRootAXNodeReturns<'a> {
        GetRootAXNodeReturns {
            node: self.node,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node to get.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper to get.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetAXNodeAndAncestorsParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetAXNodeAndAncestorsParamsBuilder<'a> {
        GetAXNodeAndAncestorsParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_id: None,
        }
    }
    /// Identifier of the node to get.
    pub fn node_id(&self) -> Option<&crate::dom::NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node to get.
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper to get.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
}

#[derive(Default)]
pub struct GetAXNodeAndAncestorsParamsBuilder<'a> {
    node_id: Option<crate::dom::NodeId>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
}

impl<'a> GetAXNodeAndAncestorsParamsBuilder<'a> {
    /// Identifier of the node to get.
    pub fn node_id(mut self, node_id: crate::dom::NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node to get.
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper to get.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    pub fn build(self) -> GetAXNodeAndAncestorsParams<'a> {
        GetAXNodeAndAncestorsParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `nodes`: 
    pub fn builder(nodes: Vec<AXNode<'a>>) -> GetAXNodeAndAncestorsReturnsBuilder<'a> {
        GetAXNodeAndAncestorsReturnsBuilder {
            nodes: nodes,
        }
    }
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}


pub struct GetAXNodeAndAncestorsReturnsBuilder<'a> {
    nodes: Vec<AXNode<'a>>,
}

impl<'a> GetAXNodeAndAncestorsReturnsBuilder<'a> {
    pub fn build(self) -> GetAXNodeAndAncestorsReturns<'a> {
        GetAXNodeAndAncestorsReturns {
            nodes: self.nodes,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetChildAXNodesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: 
    pub fn builder(id: impl Into<AXNodeId<'a>>) -> GetChildAXNodesParamsBuilder<'a> {
        GetChildAXNodesParamsBuilder {
            id: id.into(),
            frame_id: None,
        }
    }
    pub fn id(&self) -> &AXNodeId<'a> { &self.id }
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
}


pub struct GetChildAXNodesParamsBuilder<'a> {
    id: AXNodeId<'a>,
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetChildAXNodesParamsBuilder<'a> {
    /// The frame in whose document the node resides.
    /// If omitted, the root frame is used.
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    pub fn build(self) -> GetChildAXNodesParams<'a> {
        GetChildAXNodesParams {
            id: self.id,
            frame_id: self.frame_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `nodes`: 
    pub fn builder(nodes: Vec<AXNode<'a>>) -> GetChildAXNodesReturnsBuilder<'a> {
        GetChildAXNodesReturnsBuilder {
            nodes: nodes,
        }
    }
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}


pub struct GetChildAXNodesReturnsBuilder<'a> {
    nodes: Vec<AXNode<'a>>,
}

impl<'a> GetChildAXNodesReturnsBuilder<'a> {
    pub fn build(self) -> GetChildAXNodesReturns<'a> {
        GetChildAXNodesReturns {
            nodes: self.nodes,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::NodeId>,
    /// Identifier of the backend node for the root to query.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    backend_node_id: Option<crate::dom::BackendNodeId>,
    /// JavaScript object id of the node wrapper for the root to query.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    /// Find nodes with this computed name.
    #[serde(skip_serializing_if = "Option::is_none", rename = "accessibleName")]
    accessible_name: Option<Cow<'a, str>>,
    /// Find nodes with this computed role.
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<Cow<'a, str>>,
}

impl<'a> QueryAXTreeParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> QueryAXTreeParamsBuilder<'a> {
        QueryAXTreeParamsBuilder {
            node_id: None,
            backend_node_id: None,
            object_id: None,
            accessible_name: None,
            role: None,
        }
    }
    /// Identifier of the node for the root to query.
    pub fn node_id(&self) -> Option<&crate::dom::NodeId> { self.node_id.as_ref() }
    /// Identifier of the backend node for the root to query.
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
    /// JavaScript object id of the node wrapper for the root to query.
    pub fn object_id(&self) -> Option<&crate::runtime::RemoteObjectId<'a>> { self.object_id.as_ref() }
    /// Find nodes with this computed name.
    pub fn accessible_name(&self) -> Option<&str> { self.accessible_name.as_deref() }
    /// Find nodes with this computed role.
    pub fn role(&self) -> Option<&str> { self.role.as_deref() }
}

#[derive(Default)]
pub struct QueryAXTreeParamsBuilder<'a> {
    node_id: Option<crate::dom::NodeId>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
    object_id: Option<crate::runtime::RemoteObjectId<'a>>,
    accessible_name: Option<Cow<'a, str>>,
    role: Option<Cow<'a, str>>,
}

impl<'a> QueryAXTreeParamsBuilder<'a> {
    /// Identifier of the node for the root to query.
    pub fn node_id(mut self, node_id: crate::dom::NodeId) -> Self { self.node_id = Some(node_id); self }
    /// Identifier of the backend node for the root to query.
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    /// JavaScript object id of the node wrapper for the root to query.
    pub fn object_id(mut self, object_id: crate::runtime::RemoteObjectId<'a>) -> Self { self.object_id = Some(object_id); self }
    /// Find nodes with this computed name.
    pub fn accessible_name(mut self, accessible_name: impl Into<Cow<'a, str>>) -> Self { self.accessible_name = Some(accessible_name.into()); self }
    /// Find nodes with this computed role.
    pub fn role(mut self, role: impl Into<Cow<'a, str>>) -> Self { self.role = Some(role.into()); self }
    pub fn build(self) -> QueryAXTreeParams<'a> {
        QueryAXTreeParams {
            node_id: self.node_id,
            backend_node_id: self.backend_node_id,
            object_id: self.object_id,
            accessible_name: self.accessible_name,
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
    /// Creates a builder for this type with the required parameters:
    /// * `nodes`: A list of `Accessibility.AXNode` matching the specified attributes, including nodes that are ignored for accessibility.
    pub fn builder(nodes: Vec<AXNode<'a>>) -> QueryAXTreeReturnsBuilder<'a> {
        QueryAXTreeReturnsBuilder {
            nodes: nodes,
        }
    }
    /// A list of 'Accessibility.AXNode' matching the specified attributes,
    /// including nodes that are ignored for accessibility.
    pub fn nodes(&self) -> &[AXNode<'a>] { &self.nodes }
}


pub struct QueryAXTreeReturnsBuilder<'a> {
    nodes: Vec<AXNode<'a>>,
}

impl<'a> QueryAXTreeReturnsBuilder<'a> {
    pub fn build(self) -> QueryAXTreeReturns<'a> {
        QueryAXTreeReturns {
            nodes: self.nodes,
        }
    }
}

impl<'a> QueryAXTreeParams<'a> { pub const METHOD: &'static str = "Accessibility.queryAXTree"; }

impl<'a> crate::CdpCommand<'a> for QueryAXTreeParams<'a> {
    const METHOD: &'static str = "Accessibility.queryAXTree";
    type Response = QueryAXTreeReturns<'a>;
}
