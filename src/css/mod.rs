//! This domain exposes CSS read/write operations. All CSS objects (stylesheets, rules, and styles)
//! have an associated 'id' used in subsequent operations on the related object. Each object type has
//! a specific 'id' structure, and those are not interchangeable between objects of different kinds.
//! CSS objects can be loaded using the 'get*ForNode()' calls (which accept a DOM node id). A client
//! can also keep track of stylesheets via the 'styleSheetAdded'/'styleSheetRemoved' events and
//! subsequently load the required stylesheet contents using the 'getStyleSheet\[Text\]()' methods.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Stylesheet type: "injected" for stylesheets injected via extension, "user-agent" for user-agent
/// stylesheets, "inspector" for stylesheets created by the inspector (i.e. those holding the "via
/// inspector" rules), "regular" for regular stylesheets.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StyleSheetOrigin {
    #[default]
    #[serde(rename = "injected")]
    Injected,
    #[serde(rename = "user-agent")]
    UserAgent,
    #[serde(rename = "inspector")]
    Inspector,
    #[serde(rename = "regular")]
    Regular,
}

/// CSS rule collection for a single pseudo style.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PseudoElementMatches<'a> {
    /// Pseudo element type.
    #[serde(rename = "pseudoType")]
    pub pseudo_type: crate::dom::PseudoType,
    /// Pseudo element custom ident.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoIdentifier")]
    pub pseudo_identifier: Option<Cow<'a, str>>,
    /// Matches of CSS rules applicable to the pseudo style.
    pub matches: Vec<RuleMatch<'a>>,
}
/// CSS style coming from animations with the name of the animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSAnimationStyle<'a> {
    /// The name of the animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    /// The style coming from the animation.
    pub style: CSSStyle<'a>,
}
/// Inherited CSS rule collection from ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InheritedStyleEntry<'a> {
    /// The ancestor node's inline style, if any, in the style inheritance chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inlineStyle")]
    pub inline_style: Option<CSSStyle<'a>>,
    /// Matches of CSS rules matching the ancestor node in the style inheritance chain.
    #[serde(rename = "matchedCSSRules")]
    pub matched_css_rules: Vec<RuleMatch<'a>>,
}
/// Inherited CSS style collection for animated styles from ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InheritedAnimatedStyleEntry<'a> {
    /// Styles coming from the animations of the ancestor, if any, in the style inheritance chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "animationStyles")]
    pub animation_styles: Option<Vec<CSSAnimationStyle<'a>>>,
    /// The style coming from the transitions of the ancestor, if any, in the style inheritance chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitionsStyle")]
    pub transitions_style: Option<CSSStyle<'a>>,
}
/// Inherited pseudo element matches from pseudos of an ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InheritedPseudoElementMatches<'a> {
    /// Matches of pseudo styles from the pseudos of an ancestor node.
    #[serde(rename = "pseudoElements")]
    pub pseudo_elements: Vec<PseudoElementMatches<'a>>,
}
/// Match data for a CSS rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RuleMatch<'a> {
    /// CSS rule in the match.
    pub rule: CSSRule<'a>,
    /// Matching selector indices in the rule's selectorList selectors (0-based).
    #[serde(rename = "matchingSelectors")]
    pub matching_selectors: Vec<i64>,
}
/// Data for a simple selector (these are delimited by commas in a selector list).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolValue<'a> {
    /// Value text.
    pub text: Cow<'a, str>,
    /// Value range in the underlying resource (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Specificity of the selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificity: Option<Specificity<'a>>,
}
/// Contribution of an individual simple selector to specificity.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SpecificityComponent<'a> {
    /// The simple selector text that contributes to specificity.
    pub text: Cow<'a, str>,
    /// The a component contribution.
    pub a: i64,
    /// The b component contribution.
    pub b: i64,
    /// The c component contribution.
    pub c: i64,
}
/// Specificity:
/// <https://drafts.csswg.org/selectors/#specificity-rules>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Specificity<'a> {
    /// The a component, which represents the number of ID selectors.
    pub a: i64,
    /// The b component, which represents the number of class selectors, attributes selectors, and
    /// pseudo-classes.
    pub b: i64,
    /// The c component, which represents the number of type selectors and pseudo-elements.
    pub c: i64,
    /// Per-simple-selector contributions used to explain this specificity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<SpecificityComponent<'a>>>,
}
/// Selector list data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SelectorList<'a> {
    /// Selectors in the list.
    pub selectors: Vec<ProtocolValue<'a>>,
    /// Rule selector text.
    pub text: Cow<'a, str>,
}
/// CSS stylesheet metainformation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSStyleSheetHeader<'a> {
    /// The stylesheet identifier.
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    /// Owner frame identifier.
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
    /// Stylesheet resource URL. Empty if this is a constructed stylesheet created using
    /// new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported
    /// as a CSS module script).
    #[serde(rename = "sourceURL")]
    pub source_url: Cow<'a, str>,
    /// URL of source map associated with the stylesheet (if any).
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceMapURL")]
    pub source_map_url: Option<Cow<'a, str>>,
    /// Stylesheet origin.
    pub origin: StyleSheetOrigin,
    /// Stylesheet title.
    pub title: Cow<'a, str>,
    /// The backend id for the owner node of the stylesheet.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ownerNode")]
    pub owner_node: Option<crate::dom::BackendNodeId>,
    /// Denotes whether the stylesheet is disabled.
    pub disabled: bool,
    /// Whether the sourceURL field value comes from the sourceURL comment.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasSourceURL")]
    pub has_source_url: Option<bool>,
    /// Whether this stylesheet is created for STYLE tag by parser. This flag is not set for
    /// document.written STYLE tags.
    #[serde(rename = "isInline")]
    pub is_inline: bool,
    /// Whether this stylesheet is mutable. Inline stylesheets become mutable
    /// after they have been modified via CSSOM API.
    /// '\<link\>' element's stylesheets become mutable only if DevTools modifies them.
    /// Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.
    #[serde(rename = "isMutable")]
    pub is_mutable: bool,
    /// True if this stylesheet is created through new CSSStyleSheet() or imported as a
    /// CSS module script.
    #[serde(rename = "isConstructed")]
    pub is_constructed: bool,
    /// Line offset of the stylesheet within the resource (zero based).
    #[serde(rename = "startLine")]
    pub start_line: f64,
    /// Column offset of the stylesheet within the resource (zero based).
    #[serde(rename = "startColumn")]
    pub start_column: f64,
    /// Size of the content (in characters).
    pub length: f64,
    /// Line offset of the end of the stylesheet within the resource (zero based).
    #[serde(rename = "endLine")]
    pub end_line: f64,
    /// Column offset of the end of the stylesheet within the resource (zero based).
    #[serde(rename = "endColumn")]
    pub end_column: f64,
    /// If the style sheet was loaded from a network resource, this indicates when the resource failed to load
    #[serde(skip_serializing_if = "Option::is_none", rename = "loadingFailed")]
    pub loading_failed: Option<bool>,
}
/// CSS rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Rule selector data.
    #[serde(rename = "selectorList")]
    pub selector_list: SelectorList<'a>,
    /// Array of selectors from ancestor style rules, sorted by distance from the current rule.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nestingSelectors")]
    pub nesting_selectors: Option<Vec<Cow<'a, str>>>,
    /// Parent stylesheet's origin.
    pub origin: StyleSheetOrigin,
    /// Associated style declaration.
    pub style: CSSStyle<'a>,
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originTreeScopeNodeId")]
    pub origin_tree_scope_node_id: Option<crate::dom::BackendNodeId>,
    /// Media list array (for rules involving media queries). The array enumerates media queries
    /// starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<CSSMedia<'a>>>,
    /// Container query list array (for rules involving container queries).
    /// The array enumerates container queries starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerQueries")]
    pub container_queries: Option<Vec<CSSContainerQuery<'a>>>,
    /// @supports CSS at-rule array.
    /// The array enumerates @supports at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports: Option<Vec<CSSSupports<'a>>>,
    /// Cascade layer array. Contains the layer hierarchy that this rule belongs to starting
    /// with the innermost layer and going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<CSSLayer<'a>>>,
    /// @scope CSS at-rule array.
    /// The array enumerates @scope at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<CSSScope<'a>>>,
    /// The array keeps the types of ancestor CSSRules from the innermost going outwards.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ruleTypes")]
    pub rule_types: Option<Vec<CSSRuleType>>,
    /// @starting-style CSS at-rule array.
    /// The array enumerates @starting-style at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none", rename = "startingStyles")]
    pub starting_styles: Option<Vec<CSSStartingStyle<'a>>>,
    /// @navigation CSS at-rule array.
    /// The array enumerates @navigation at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigations: Option<Vec<CSSNavigation<'a>>>,
}
/// Enum indicating the type of a CSS rule, used to represent the order of a style rule's ancestors.
/// This list only contains rule types that are collected during the ancestor rule collection.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CSSRuleType {
    #[default]
    #[serde(rename = "MediaRule")]
    MediaRule,
    #[serde(rename = "SupportsRule")]
    SupportsRule,
    #[serde(rename = "ContainerRule")]
    ContainerRule,
    #[serde(rename = "LayerRule")]
    LayerRule,
    #[serde(rename = "ScopeRule")]
    ScopeRule,
    #[serde(rename = "StyleRule")]
    StyleRule,
    #[serde(rename = "StartingStyleRule")]
    StartingStyleRule,
    #[serde(rename = "NavigationRule")]
    NavigationRule,
}

/// CSS coverage information.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RuleUsage<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    /// Offset of the start of the rule (including selector) from the beginning of the stylesheet.
    #[serde(rename = "startOffset")]
    pub start_offset: f64,
    /// Offset of the end of the rule body from the beginning of the stylesheet.
    #[serde(rename = "endOffset")]
    pub end_offset: f64,
    /// Indicates whether the rule was actually used by some element in the page.
    pub used: bool,
}
/// Text range within a resource. All numbers are zero-based.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SourceRange {
    /// Start line of range.
    #[serde(rename = "startLine")]
    pub start_line: i64,
    /// Start column of range (inclusive).
    #[serde(rename = "startColumn")]
    pub start_column: i64,
    /// End line of range
    #[serde(rename = "endLine")]
    pub end_line: i64,
    /// End column of range (exclusive).
    #[serde(rename = "endColumn")]
    pub end_column: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ShorthandEntry<'a> {
    /// Shorthand name.
    pub name: Cow<'a, str>,
    /// Shorthand value.
    pub value: Cow<'a, str>,
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSComputedStyleProperty<'a> {
    /// Computed style property name.
    pub name: Cow<'a, str>,
    /// Computed style property value.
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ComputedStyleExtraFields {
    /// Returns whether or not this node is being rendered with base appearance,
    /// which happens when it has its appearance property set to base/base-select
    /// or it is in the subtree of an element being rendered with base appearance.
    #[serde(rename = "isAppearanceBase")]
    pub is_appearance_base: bool,
}
/// CSS style representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSStyle<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// CSS properties in the style.
    #[serde(rename = "cssProperties")]
    pub css_properties: Vec<CSSProperty<'a>>,
    /// Computed values for all shorthands found in the style.
    #[serde(rename = "shorthandEntries")]
    pub shorthand_entries: Vec<ShorthandEntry<'a>>,
    /// Style declaration text (if available).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssText")]
    pub css_text: Option<Cow<'a, str>>,
    /// Style declaration range in the enclosing stylesheet (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
}
/// CSS property declaration data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSProperty<'a> {
    /// The property name.
    pub name: Cow<'a, str>,
    /// The property value.
    pub value: Cow<'a, str>,
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
    /// Whether the property is implicit (implies 'false' if absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<bool>,
    /// The full property text as specified in the style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Cow<'a, str>>,
    /// Whether the property is understood by the browser (implies 'true' if absent).
    #[serde(skip_serializing_if = "Option::is_none", rename = "parsedOk")]
    pub parsed_ok: Option<bool>,
    /// Whether the property is disabled by the user (present for source-based properties only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The entire property range in the enclosing style declaration (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Parsed longhand components of this property if it is a shorthand.
    /// This field will be empty if the given property is not a shorthand.
    #[serde(skip_serializing_if = "Option::is_none", rename = "longhandProperties")]
    pub longhand_properties: Option<Vec<Box<CSSProperty<'a>>>>,
}
/// CSS media rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSMedia<'a> {
    /// Media query text.
    pub text: Cow<'a, str>,
    /// Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if
    /// specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked
    /// stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline
    /// stylesheet's STYLE tag.
    pub source: Cow<'a, str>,
    /// URL of the document containing the media query description.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceURL")]
    pub source_url: Option<Cow<'a, str>>,
    /// The associated rule (@media or @import) header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Array of media queries.
    #[serde(skip_serializing_if = "Option::is_none", rename = "mediaList")]
    pub media_list: Option<Vec<MediaQuery<'a>>>,
}
/// Media query descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MediaQuery<'a> {
    /// Array of media query expressions.
    pub expressions: Vec<MediaQueryExpression<'a>>,
    /// Whether the media query condition is satisfied.
    pub active: bool,
}
/// Media query expression descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MediaQueryExpression<'a> {
    /// Media query expression value.
    pub value: f64,
    /// Media query expression units.
    pub unit: Cow<'a, str>,
    /// Media query expression feature.
    pub feature: Cow<'a, str>,
    /// The associated range of the value text in the enclosing stylesheet (if available).
    #[serde(skip_serializing_if = "Option::is_none", rename = "valueRange")]
    pub value_range: Option<SourceRange>,
    /// Computed length of media query expression (if applicable).
    #[serde(skip_serializing_if = "Option::is_none", rename = "computedLength")]
    pub computed_length: Option<f64>,
}
/// CSS container query rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSContainerQuery<'a> {
    /// Container query text.
    /// Contains the query part without the container name for a single query.
    /// Deprecated in favor of conditionText which contains the full prelude
    /// after @container.
    pub text: Cow<'a, str>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Optional name for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    /// Optional physical axes queried for the container.
    #[serde(skip_serializing_if = "Option::is_none", rename = "physicalAxes")]
    pub physical_axes: Option<crate::dom::PhysicalAxes>,
    /// Optional logical axes queried for the container.
    #[serde(skip_serializing_if = "Option::is_none", rename = "logicalAxes")]
    pub logical_axes: Option<crate::dom::LogicalAxes>,
    /// true if the query contains scroll-state() queries.
    #[serde(skip_serializing_if = "Option::is_none", rename = "queriesScrollState")]
    pub queries_scroll_state: Option<bool>,
    /// true if the query contains anchored() queries.
    #[serde(skip_serializing_if = "Option::is_none", rename = "queriesAnchored")]
    pub queries_anchored: Option<bool>,
    /// CSSContainerRule.conditionText
    #[serde(rename = "conditionText")]
    pub condition_text: Cow<'a, str>,
}
/// CSS Supports at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSSupports<'a> {
    /// Supports rule text.
    pub text: Cow<'a, str>,
    /// Whether the supports condition is satisfied.
    pub active: bool,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}
/// CSS Navigation at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSNavigation<'a> {
    /// Navigation rule text.
    pub text: Cow<'a, str>,
    /// Whether the navigation condition is satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}
/// CSS Scope at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSScope<'a> {
    /// Scope rule text.
    pub text: Cow<'a, str>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}
/// CSS Layer at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSLayer<'a> {
    /// Layer name.
    pub text: Cow<'a, str>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}
/// CSS Starting Style at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSStartingStyle<'a> {
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}
/// CSS Layer data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSLayerData<'a> {
    /// Layer name.
    pub name: Cow<'a, str>,
    /// Direct sub-layers
    #[serde(skip_serializing_if = "Option::is_none", rename = "subLayers")]
    pub sub_layers: Option<Vec<Box<CSSLayerData<'a>>>>,
    /// Layer order. The order determines the order of the layer in the cascade order.
    /// A higher number has higher priority in the cascade order.
    pub order: f64,
}
/// Information about amount of glyphs that were rendered with given font.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PlatformFontUsage<'a> {
    /// Font's family name reported by platform.
    #[serde(rename = "familyName")]
    pub family_name: Cow<'a, str>,
    /// Font's PostScript name reported by platform.
    #[serde(rename = "postScriptName")]
    pub post_script_name: Cow<'a, str>,
    /// Indicates if the font was downloaded or resolved locally.
    #[serde(rename = "isCustomFont")]
    pub is_custom_font: bool,
    /// Amount of glyphs that were rendered with this font.
    #[serde(rename = "glyphCount")]
    pub glyph_count: f64,
}
/// Information about font variation axes for variable fonts

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FontVariationAxis<'a> {
    /// The font-variation-setting tag (a.k.a. "axis tag").
    pub tag: Cow<'a, str>,
    /// Human-readable variation name in the default language (normally, "en").
    pub name: Cow<'a, str>,
    /// The minimum value (inclusive) the font supports for this tag.
    #[serde(rename = "minValue")]
    pub min_value: f64,
    /// The maximum value (inclusive) the font supports for this tag.
    #[serde(rename = "maxValue")]
    pub max_value: f64,
    /// The default value.
    #[serde(rename = "defaultValue")]
    pub default_value: f64,
}
/// Properties of a web font: <https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions>
/// and additional information such as platformFontFamily and fontVariationAxes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FontFace<'a> {
    /// The font-family.
    #[serde(rename = "fontFamily")]
    pub font_family: Cow<'a, str>,
    /// The font-style.
    #[serde(rename = "fontStyle")]
    pub font_style: Cow<'a, str>,
    /// The font-variant.
    #[serde(rename = "fontVariant")]
    pub font_variant: Cow<'a, str>,
    /// The font-weight.
    #[serde(rename = "fontWeight")]
    pub font_weight: Cow<'a, str>,
    /// The font-stretch.
    #[serde(rename = "fontStretch")]
    pub font_stretch: Cow<'a, str>,
    /// The font-display.
    #[serde(rename = "fontDisplay")]
    pub font_display: Cow<'a, str>,
    /// The unicode-range.
    #[serde(rename = "unicodeRange")]
    pub unicode_range: Cow<'a, str>,
    /// The src.
    pub src: Cow<'a, str>,
    /// The resolved platform font family
    #[serde(rename = "platformFontFamily")]
    pub platform_font_family: Cow<'a, str>,
    /// Available variation settings (a.k.a. "axes").
    #[serde(skip_serializing_if = "Option::is_none", rename = "fontVariationAxes")]
    pub font_variation_axes: Option<Vec<FontVariationAxis<'a>>>,
}
/// CSS try rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSTryRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    pub origin: StyleSheetOrigin,
    /// Associated style declaration.
    pub style: CSSStyle<'a>,
}
/// CSS @position-try rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSPositionTryRule<'a> {
    /// The prelude dashed-ident name
    pub name: ProtocolValue<'a>,
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    pub origin: StyleSheetOrigin,
    /// Associated style declaration.
    pub style: CSSStyle<'a>,
    pub active: bool,
}
/// CSS keyframes rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSKeyframesRule<'a> {
    /// Animation name.
    #[serde(rename = "animationName")]
    pub animation_name: ProtocolValue<'a>,
    /// List of keyframes.
    pub keyframes: Vec<CSSKeyframeRule<'a>>,
}
/// Representation of a custom property registration through CSS.registerProperty

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSPropertyRegistration<'a> {
    #[serde(rename = "propertyName")]
    pub property_name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "initialValue")]
    pub initial_value: Option<ProtocolValue<'a>>,
    pub inherits: bool,
    pub syntax: Cow<'a, str>,
}
/// CSS generic @rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSAtRule<'a> {
    /// Type of at-rule.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Subsection of font-feature-values, if this is a subsection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsection: Option<Cow<'a, str>>,
    /// LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)
    /// Associated name, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ProtocolValue<'a>>,
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    pub origin: StyleSheetOrigin,
    /// Associated style declaration.
    pub style: CSSStyle<'a>,
}
/// CSS property at-rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSPropertyRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    pub origin: StyleSheetOrigin,
    /// Associated property name.
    #[serde(rename = "propertyName")]
    pub property_name: ProtocolValue<'a>,
    /// Associated style declaration.
    pub style: CSSStyle<'a>,
}
/// CSS function argument representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionParameter<'a> {
    /// The parameter name.
    pub name: Cow<'a, str>,
    /// The parameter type.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
}
/// CSS function conditional block representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionConditionNode<'a> {
    /// Media query for this conditional block. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<CSSMedia<'a>>,
    /// Container query for this conditional block. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerQueries")]
    pub container_queries: Option<CSSContainerQuery<'a>>,
    /// @supports CSS at-rule condition. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports: Option<CSSSupports<'a>>,
    /// @navigation condition. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation: Option<CSSNavigation<'a>>,
    /// Block body.
    pub children: Vec<CSSFunctionNode<'a>>,
    /// The condition text.
    #[serde(rename = "conditionText")]
    pub condition_text: Cow<'a, str>,
}
/// Section of the body of a CSS function rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionNode<'a> {
    /// A conditional block. If set, style should not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<CSSFunctionConditionNode<'a>>,
    /// Values set by this node. If set, condition should not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CSSStyle<'a>>,
}
/// CSS function at-rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionRule<'a> {
    /// Name of the function.
    pub name: ProtocolValue<'a>,
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    pub origin: StyleSheetOrigin,
    /// List of parameters.
    pub parameters: Vec<CSSFunctionParameter<'a>>,
    /// Function body.
    pub children: Vec<CSSFunctionNode<'a>>,
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originTreeScopeNodeId")]
    pub origin_tree_scope_node_id: Option<crate::dom::BackendNodeId>,
}
/// CSS keyframe rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CSSKeyframeRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    pub style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    pub origin: StyleSheetOrigin,
    /// Associated key text.
    #[serde(rename = "keyText")]
    pub key_text: ProtocolValue<'a>,
    /// Associated style declaration.
    pub style: CSSStyle<'a>,
}
/// A descriptor of operation to mutate style declaration text.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StyleDeclarationEdit<'a> {
    /// The css style sheet identifier.
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    /// The range of the style text in the enclosing stylesheet.
    pub range: SourceRange,
    /// New style text.
    pub text: Cow<'a, str>,
}
/// Inserts a new rule with the given 'ruleText' in a stylesheet with given 'styleSheetId', at the
/// position specified by 'location'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.addRule", response = "AddRuleReturns<'a>")]
pub struct AddRuleParams<'a> {
    /// The css style sheet identifier where a new rule should be inserted.
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    /// The text of a new rule.
    #[serde(rename = "ruleText")]
    pub rule_text: Cow<'a, str>,
    /// Text position of a new rule in the target style sheet.
    pub location: SourceRange,
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeForPropertySyntaxValidation")]
    pub node_for_property_syntax_validation: Option<crate::dom::NodeId>,
}
/// Inserts a new rule with the given 'ruleText' in a stylesheet with given 'styleSheetId', at the
/// position specified by 'location'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddRuleReturns<'a> {
    /// The newly created rule.
    pub rule: CSSRule<'a>,
}
/// Returns all class names from specified stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.collectClassNames", response = "CollectClassNamesReturns<'a>")]
pub struct CollectClassNamesParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
}
/// Returns all class names from specified stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesReturns<'a> {
    /// Class name list.
    #[serde(rename = "classNames")]
    pub class_names: Vec<Cow<'a, str>>,
}
/// Creates a new special "via-inspector" stylesheet in the frame with given 'frameId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.createStyleSheet", response = "CreateStyleSheetReturns<'a>")]
pub struct CreateStyleSheetParams<'a> {
    /// Identifier of the frame where "via-inspector" stylesheet should be created.
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
    /// If true, creates a new stylesheet for every call. If false,
    /// returns a stylesheet previously created by a call with force=false
    /// for the frame's document if it exists or creates a new stylesheet
    /// (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}
/// Creates a new special "via-inspector" stylesheet in the frame with given 'frameId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CreateStyleSheetReturns<'a> {
    /// Identifier of the created "via-inspector" stylesheet.
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
}
/// Disables the CSS agent for the given page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.disable")]
pub struct DisableParams {

}
/// Enables the CSS agent for the given page. Clients should not assume that the CSS agent has been
/// enabled until the result of this command is received.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.enable")]
pub struct EnableParams {

}
/// Ensures that the given node will have specified pseudo-classes whenever its style is computed by
/// the browser.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.forcePseudoState")]
pub struct ForcePseudoStateParams<'a> {
    /// The element id for which to force the pseudo state.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    /// Element pseudo classes to force when computing the element's style.
    #[serde(rename = "forcedPseudoClasses")]
    pub forced_pseudo_classes: Vec<Cow<'a, str>>,
}
/// Ensures that the given node is in its starting-style state.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.forceStartingStyle")]
pub struct ForceStartingStyleParams {
    /// The element id for which to force the starting-style state.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    /// Boolean indicating if this is on or off.
    pub forced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getBackgroundColors", response = "GetBackgroundColorsReturns<'a>")]
pub struct GetBackgroundColorsParams {
    /// Id of the node to get background colors for.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetBackgroundColorsReturns<'a> {
    /// The range of background colors behind this element, if it contains any visible text. If no
    /// visible text is present, this will be undefined. In the case of a flat background color,
    /// this will consist of simply that color. In the case of a gradient, this will consist of each
    /// of the color stops. For anything more complicated, this will be an empty array. Images will
    /// be ignored (as if the image had failed to load).
    #[serde(skip_serializing_if = "Option::is_none", rename = "backgroundColors")]
    pub background_colors: Option<Vec<Cow<'a, str>>>,
    /// The computed font size for this node, as a CSS computed value string (e.g. '12px').
    #[serde(skip_serializing_if = "Option::is_none", rename = "computedFontSize")]
    pub computed_font_size: Option<Cow<'a, str>>,
    /// The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
    /// '100').
    #[serde(skip_serializing_if = "Option::is_none", rename = "computedFontWeight")]
    pub computed_font_weight: Option<Cow<'a, str>>,
}
/// Returns the computed style for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getComputedStyleForNode", response = "GetComputedStyleForNodeReturns<'a>")]
pub struct GetComputedStyleForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}
/// Returns the computed style for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetComputedStyleForNodeReturns<'a> {
    /// Computed style for the specified DOM node.
    #[serde(rename = "computedStyle")]
    pub computed_style: Vec<CSSComputedStyleProperty<'a>>,
    /// A list of non-standard "extra fields" which blink stores alongside each
    /// computed style.
    #[serde(rename = "extraFields")]
    pub extra_fields: ComputedStyleExtraFields,
}
/// Resolve the specified values in the context of the provided element.
/// For example, a value of '1em' is evaluated according to the computed
/// 'font-size' of the element and a value 'calc(1px + 2px)' will be
/// resolved to '3px'.
/// If the 'propertyName' was specified the 'values' are resolved as if
/// they were property's declaration. If a value cannot be parsed according
/// to the provided property syntax, the value is parsed using combined
/// syntax as if null 'propertyName' was provided. If the value cannot be
/// resolved even then, return the provided value without any changes.
/// Note: this function currently does not resolve CSS random() function,
/// it returns unmodified random() function parts.'

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.resolveValues", response = "ResolveValuesReturns<'a>")]
pub struct ResolveValuesParams<'a> {
    /// Cascade-dependent keywords (revert/revert-layer) do not work.
    pub values: Vec<Cow<'a, str>>,
    /// Id of the node in whose context the expression is evaluated
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    /// Only longhands and custom property names are accepted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "propertyName")]
    pub property_name: Option<Cow<'a, str>>,
    /// Pseudo element type, only works for pseudo elements that generate
    /// elements in the tree, such as ::before and ::after.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoType")]
    pub pseudo_type: Option<crate::dom::PseudoType>,
    /// Pseudo element custom ident.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoIdentifier")]
    pub pseudo_identifier: Option<Cow<'a, str>>,
}
/// Resolve the specified values in the context of the provided element.
/// For example, a value of '1em' is evaluated according to the computed
/// 'font-size' of the element and a value 'calc(1px + 2px)' will be
/// resolved to '3px'.
/// If the 'propertyName' was specified the 'values' are resolved as if
/// they were property's declaration. If a value cannot be parsed according
/// to the provided property syntax, the value is parsed using combined
/// syntax as if null 'propertyName' was provided. If the value cannot be
/// resolved even then, return the provided value without any changes.
/// Note: this function currently does not resolve CSS random() function,
/// it returns unmodified random() function parts.'

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ResolveValuesReturns<'a> {
    pub results: Vec<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getLonghandProperties", response = "GetLonghandPropertiesReturns<'a>")]
pub struct GetLonghandPropertiesParams<'a> {
    #[serde(rename = "shorthandName")]
    pub shorthand_name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetLonghandPropertiesReturns<'a> {
    #[serde(rename = "longhandProperties")]
    pub longhand_properties: Vec<CSSProperty<'a>>,
}
/// Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
/// attributes) for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getInlineStylesForNode", response = "GetInlineStylesForNodeReturns<'a>")]
pub struct GetInlineStylesForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}
/// Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
/// attributes) for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetInlineStylesForNodeReturns<'a> {
    /// Inline style for the specified DOM node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inlineStyle")]
    pub inline_style: Option<CSSStyle<'a>>,
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    #[serde(skip_serializing_if = "Option::is_none", rename = "attributesStyle")]
    pub attributes_style: Option<CSSStyle<'a>>,
}
/// Returns the styles coming from animations & transitions
/// including the animation & transition styles coming from inheritance chain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getAnimatedStylesForNode", response = "GetAnimatedStylesForNodeReturns<'a>")]
pub struct GetAnimatedStylesForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}
/// Returns the styles coming from animations & transitions
/// including the animation & transition styles coming from inheritance chain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAnimatedStylesForNodeReturns<'a> {
    /// Styles coming from animations.
    #[serde(skip_serializing_if = "Option::is_none", rename = "animationStyles")]
    pub animation_styles: Option<Vec<CSSAnimationStyle<'a>>>,
    /// Style coming from transitions.
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitionsStyle")]
    pub transitions_style: Option<CSSStyle<'a>>,
    /// Inherited style entries for animationsStyle and transitionsStyle from
    /// the inheritance chain of the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<Vec<InheritedAnimatedStyleEntry<'a>>>,
}
/// Returns requested styles for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getMatchedStylesForNode", response = "GetMatchedStylesForNodeReturns<'a>")]
pub struct GetMatchedStylesForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}
/// Returns requested styles for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetMatchedStylesForNodeReturns<'a> {
    /// Inline style for the specified DOM node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inlineStyle")]
    pub inline_style: Option<CSSStyle<'a>>,
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    #[serde(skip_serializing_if = "Option::is_none", rename = "attributesStyle")]
    pub attributes_style: Option<CSSStyle<'a>>,
    /// CSS rules matching this node, from all applicable stylesheets.
    #[serde(skip_serializing_if = "Option::is_none", rename = "matchedCSSRules")]
    pub matched_css_rules: Option<Vec<RuleMatch<'a>>>,
    /// Pseudo style matches for this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoElements")]
    pub pseudo_elements: Option<Vec<PseudoElementMatches<'a>>>,
    /// A chain of inherited styles (from the immediate node parent up to the DOM tree root).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<Vec<InheritedStyleEntry<'a>>>,
    /// A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root).
    #[serde(skip_serializing_if = "Option::is_none", rename = "inheritedPseudoElements")]
    pub inherited_pseudo_elements: Option<Vec<InheritedPseudoElementMatches<'a>>>,
    /// A list of CSS keyframed animations matching this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssKeyframesRules")]
    pub css_keyframes_rules: Option<Vec<CSSKeyframesRule<'a>>>,
    /// A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssPositionTryRules")]
    pub css_position_try_rules: Option<Vec<CSSPositionTryRule<'a>>>,
    /// Index of the active fallback in the applied position-try-fallback property,
    /// will not be set if there is no active position-try fallback.
    #[serde(skip_serializing_if = "Option::is_none", rename = "activePositionFallbackIndex")]
    pub active_position_fallback_index: Option<u64>,
    /// A list of CSS at-property rules matching this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssPropertyRules")]
    pub css_property_rules: Option<Vec<CSSPropertyRule<'a>>>,
    /// A list of CSS property registrations matching this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssPropertyRegistrations")]
    pub css_property_registrations: Option<Vec<CSSPropertyRegistration<'a>>>,
    /// A list of simple @rules matching this node or its pseudo-elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssAtRules")]
    pub css_at_rules: Option<Vec<CSSAtRule<'a>>>,
    /// Id of the first parent element that does not have display: contents.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentLayoutNodeId")]
    pub parent_layout_node_id: Option<crate::dom::NodeId>,
    /// A list of CSS at-function rules referenced by styles of this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssFunctionRules")]
    pub css_function_rules: Option<Vec<CSSFunctionRule<'a>>>,
}
/// Returns the values of the default UA-defined environment variables used in env()

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getEnvironmentVariables", response = "GetEnvironmentVariablesReturns")]
pub struct GetEnvironmentVariablesParams {

}
/// Returns the values of the default UA-defined environment variables used in env()

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetEnvironmentVariablesReturns {
    #[serde(rename = "environmentVariables")]
    pub environment_variables: serde_json::Map<String, JsonValue>,
}
/// Returns all media queries parsed by the rendering engine.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getMediaQueries", response = "GetMediaQueriesReturns<'a>")]
pub struct GetMediaQueriesParams {

}
/// Returns all media queries parsed by the rendering engine.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaQueriesReturns<'a> {
    pub medias: Vec<CSSMedia<'a>>,
}
/// Requests information about platform fonts which we used to render child TextNodes in the given
/// node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getPlatformFontsForNode", response = "GetPlatformFontsForNodeReturns<'a>")]
pub struct GetPlatformFontsForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}
/// Requests information about platform fonts which we used to render child TextNodes in the given
/// node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetPlatformFontsForNodeReturns<'a> {
    /// Usage statistics for every employed platform font.
    pub fonts: Vec<PlatformFontUsage<'a>>,
}
/// Returns the current textual content for a stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getStyleSheetText", response = "GetStyleSheetTextReturns<'a>")]
pub struct GetStyleSheetTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
}
/// Returns the current textual content for a stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetStyleSheetTextReturns<'a> {
    /// The stylesheet text.
    pub text: Cow<'a, str>,
}
/// Returns all layers parsed by the rendering engine for the tree scope of a node.
/// Given a DOM element identified by nodeId, getLayersForNode returns the root
/// layer for the nearest ancestor document or shadow root. The layer root contains
/// the full layer tree for the tree scope and their ordering.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getLayersForNode", response = "GetLayersForNodeReturns<'a>")]
pub struct GetLayersForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}
/// Returns all layers parsed by the rendering engine for the tree scope of a node.
/// Given a DOM element identified by nodeId, getLayersForNode returns the root
/// layer for the nearest ancestor document or shadow root. The layer root contains
/// the full layer tree for the tree scope and their ordering.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetLayersForNodeReturns<'a> {
    #[serde(rename = "rootLayer")]
    pub root_layer: CSSLayerData<'a>,
}
/// Given a CSS selector text and a style sheet ID, getLocationForSelector
/// returns an array of locations of the CSS selector in the style sheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.getLocationForSelector", response = "GetLocationForSelectorReturns")]
pub struct GetLocationForSelectorParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    #[serde(rename = "selectorText")]
    pub selector_text: Cow<'a, str>,
}
/// Given a CSS selector text and a style sheet ID, getLocationForSelector
/// returns an array of locations of the CSS selector in the style sheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationForSelectorReturns {
    pub ranges: Vec<SourceRange>,
}
/// Starts tracking the given node for the computed style updates
/// and whenever the computed style is updated for node, it queues
/// a 'computedStyleUpdated' event with throttling.
/// There can only be 1 node tracked for computed style updates
/// so passing a new node id removes tracking from the previous node.
/// Pass 'undefined' to disable tracking.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.trackComputedStyleUpdatesForNode")]
pub struct TrackComputedStyleUpdatesForNodeParams {
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::NodeId>,
}
/// Starts tracking the given computed styles for updates. The specified array of properties
/// replaces the one previously specified. Pass empty array to disable tracking.
/// Use takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.
/// The changes to computed style properties are only tracked for nodes pushed to the front-end
/// by the DOM agent. If no changes to the tracked properties occur after the node has been pushed
/// to the front-end, no updates will be issued for the node.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.trackComputedStyleUpdates")]
pub struct TrackComputedStyleUpdatesParams<'a> {
    #[serde(rename = "propertiesToTrack")]
    pub properties_to_track: Vec<CSSComputedStyleProperty<'a>>,
}
/// Polls the next batch of computed style updates.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.takeComputedStyleUpdates", response = "TakeComputedStyleUpdatesReturns")]
pub struct TakeComputedStyleUpdatesParams {

}
/// Polls the next batch of computed style updates.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TakeComputedStyleUpdatesReturns {
    /// The list of node Ids that have their tracked computed styles updated.
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<crate::dom::NodeId>,
}
/// Find a rule with the given active property for the given node and set the new value for this
/// property

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setEffectivePropertyValueForNode")]
pub struct SetEffectivePropertyValueForNodeParams<'a> {
    /// The element id for which to set property.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
    #[serde(rename = "propertyName")]
    pub property_name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}
/// Modifies the property rule property name.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setPropertyRulePropertyName", response = "SetPropertyRulePropertyNameReturns<'a>")]
pub struct SetPropertyRulePropertyNameParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub range: SourceRange,
    #[serde(rename = "propertyName")]
    pub property_name: Cow<'a, str>,
}
/// Modifies the property rule property name.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetPropertyRulePropertyNameReturns<'a> {
    /// The resulting key text after modification.
    #[serde(rename = "propertyName")]
    pub property_name: ProtocolValue<'a>,
}
/// Modifies the keyframe rule key text.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setKeyframeKey", response = "SetKeyframeKeyReturns<'a>")]
pub struct SetKeyframeKeyParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub range: SourceRange,
    #[serde(rename = "keyText")]
    pub key_text: Cow<'a, str>,
}
/// Modifies the keyframe rule key text.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetKeyframeKeyReturns<'a> {
    /// The resulting key text after modification.
    #[serde(rename = "keyText")]
    pub key_text: ProtocolValue<'a>,
}
/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setMediaText", response = "SetMediaTextReturns<'a>")]
pub struct SetMediaTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub range: SourceRange,
    pub text: Cow<'a, str>,
}
/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetMediaTextReturns<'a> {
    /// The resulting CSS media rule after modification.
    pub media: CSSMedia<'a>,
}
/// Modifies the expression of a container query.
/// Deprecated. Use setContainerQueryConditionText instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setContainerQueryText", response = "SetContainerQueryTextReturns<'a>")]
pub struct SetContainerQueryTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub range: SourceRange,
    pub text: Cow<'a, str>,
}
/// Modifies the expression of a container query.
/// Deprecated. Use setContainerQueryConditionText instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryTextReturns<'a> {
    /// The resulting CSS container query rule after modification.
    #[serde(rename = "containerQuery")]
    pub container_query: CSSContainerQuery<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setContainerQueryConditionText", response = "SetContainerQueryConditionTextReturns<'a>")]
pub struct SetContainerQueryConditionTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub range: SourceRange,
    pub text: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryConditionTextReturns<'a> {
    /// The resulting CSS container query rule after modification.
    #[serde(rename = "containerQuery")]
    pub container_query: CSSContainerQuery<'a>,
}
/// Modifies the expression of a supports at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setSupportsText", response = "SetSupportsTextReturns<'a>")]
pub struct SetSupportsTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub range: SourceRange,
    pub text: Cow<'a, str>,
}
/// Modifies the expression of a supports at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetSupportsTextReturns<'a> {
    /// The resulting CSS Supports rule after modification.
    pub supports: CSSSupports<'a>,
}
/// Modifies the expression of a navigation at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setNavigationText", response = "SetNavigationTextReturns<'a>")]
pub struct SetNavigationTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub range: SourceRange,
    pub text: Cow<'a, str>,
}
/// Modifies the expression of a navigation at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetNavigationTextReturns<'a> {
    /// The resulting CSS Navigation rule after modification.
    pub navigation: CSSNavigation<'a>,
}
/// Modifies the expression of a scope at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setScopeText", response = "SetScopeTextReturns<'a>")]
pub struct SetScopeTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub range: SourceRange,
    pub text: Cow<'a, str>,
}
/// Modifies the expression of a scope at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetScopeTextReturns<'a> {
    /// The resulting CSS Scope rule after modification.
    pub scope: CSSScope<'a>,
}
/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setRuleSelector", response = "SetRuleSelectorReturns<'a>")]
pub struct SetRuleSelectorParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub range: SourceRange,
    pub selector: Cow<'a, str>,
}
/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetRuleSelectorReturns<'a> {
    /// The resulting selector list after modification.
    #[serde(rename = "selectorList")]
    pub selector_list: SelectorList<'a>,
}
/// Sets the new stylesheet text.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setStyleSheetText", response = "SetStyleSheetTextReturns<'a>")]
pub struct SetStyleSheetTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
    pub text: Cow<'a, str>,
}
/// Sets the new stylesheet text.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleSheetTextReturns<'a> {
    /// URL of source map associated with script (if any).
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceMapURL")]
    pub source_map_url: Option<Cow<'a, str>>,
}
/// Applies specified style edits one after another in the given order.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setStyleTexts", response = "SetStyleTextsReturns<'a>")]
pub struct SetStyleTextsParams<'a> {
    pub edits: Vec<StyleDeclarationEdit<'a>>,
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeForPropertySyntaxValidation")]
    pub node_for_property_syntax_validation: Option<crate::dom::NodeId>,
}
/// Applies specified style edits one after another in the given order.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleTextsReturns<'a> {
    /// The resulting styles after modification.
    pub styles: Vec<CSSStyle<'a>>,
}
/// Enables the selector recording.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.startRuleUsageTracking")]
pub struct StartRuleUsageTrackingParams {

}
/// Stop tracking rule usage and return the list of rules that were used since last call to
/// 'takeCoverageDelta' (or since start of coverage instrumentation).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.stopRuleUsageTracking", response = "StopRuleUsageTrackingReturns<'a>")]
pub struct StopRuleUsageTrackingParams {

}
/// Stop tracking rule usage and return the list of rules that were used since last call to
/// 'takeCoverageDelta' (or since start of coverage instrumentation).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StopRuleUsageTrackingReturns<'a> {
    #[serde(rename = "ruleUsage")]
    pub rule_usage: Vec<RuleUsage<'a>>,
}
/// Obtain list of rules that became used since last call to this method (or since start of coverage
/// instrumentation).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.takeCoverageDelta", response = "TakeCoverageDeltaReturns<'a>")]
pub struct TakeCoverageDeltaParams {

}
/// Obtain list of rules that became used since last call to this method (or since start of coverage
/// instrumentation).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TakeCoverageDeltaReturns<'a> {
    pub coverage: Vec<RuleUsage<'a>>,
    /// Monotonically increasing time, in seconds.
    pub timestamp: f64,
}
/// Enables/disables rendering of local CSS fonts (enabled by default).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.setLocalFontsEnabled")]
pub struct SetLocalFontsEnabledParams {
    /// Whether rendering of local fonts is enabled.
    pub enabled: bool,
}
/// Fires whenever a web font is updated.  A non-empty font parameter indicates a successfully loaded
/// web font.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.fontsUpdated")]
pub struct FontsUpdated<'a> {
    /// The web font that has loaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<FontFace<'a>>,
}
/// Fires whenever a MediaQuery result changes (for example, after a browser window has been
/// resized.) The current implementation considers only viewport-dependent media features.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.mediaQueryResultChanged")]
pub struct MediaQueryResultChanged {

}
/// Fired whenever an active document stylesheet is added.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.styleSheetAdded")]
pub struct StyleSheetAdded<'a> {
    /// Added stylesheet metainfo.
    pub header: CSSStyleSheetHeader<'a>,
}
/// Fired whenever a stylesheet is changed as a result of the client operation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.styleSheetChanged")]
pub struct StyleSheetChanged<'a> {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
}
/// Fired whenever an active document stylesheet is removed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.styleSheetRemoved")]
pub struct StyleSheetRemoved<'a> {
    /// Identifier of the removed stylesheet.
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::dom::StyleSheetId<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CSS.computedStyleUpdated")]
pub struct ComputedStyleUpdated {
    /// The node id that has updated computed styles.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::NodeId,
}