use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! This domain exposes CSS read/write operations. All CSS objects (stylesheets, rules, and styles)
//! have an associated 'id' used in subsequent operations on the related object. Each object type has
//! a specific 'id' structure, and those are not interchangeable between objects of different kinds.
//! CSS objects can be loaded using the 'get*ForNode()' calls (which accept a DOM node id). A client
//! can also keep track of stylesheets via the 'styleSheetAdded'/'styleSheetRemoved' events and
//! subsequently load the required stylesheet contents using the 'getStyleSheet[Text]()' methods.

/// Stylesheet type: "injected" for stylesheets injected via extension, "user-agent" for user-agent
/// stylesheets, "inspector" for stylesheets created by the inspector (i.e. those holding the "via
/// inspector" rules), "regular" for regular stylesheets.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StyleSheetOrigin {
    #[default]
    Injected,
    UserAgent,
    Inspector,
    Regular,
}

/// CSS rule collection for a single pseudo style.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PseudoElementMatches {
    /// Pseudo element type.

    pub pseudoType: crate::dom::PseudoType,
    /// Pseudo element custom ident.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoIdentifier: Option<String>,
    /// Matches of CSS rules applicable to the pseudo style.

    pub matches: Vec<RuleMatch>,
}

/// CSS style coming from animations with the name of the animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSAnimationStyle {
    /// The name of the animation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The style coming from the animation.

    pub style: CSSStyle,
}

/// Inherited CSS rule collection from ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritedStyleEntry {
    /// The ancestor node's inline style, if any, in the style inheritance chain.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlineStyle: Option<CSSStyle>,
    /// Matches of CSS rules matching the ancestor node in the style inheritance chain.

    pub matchedCSSRules: Vec<RuleMatch>,
}

/// Inherited CSS style collection for animated styles from ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritedAnimatedStyleEntry {
    /// Styles coming from the animations of the ancestor, if any, in the style inheritance chain.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animationStyles: Option<Vec<CSSAnimationStyle>>,
    /// The style coming from the transitions of the ancestor, if any, in the style inheritance chain.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitionsStyle: Option<CSSStyle>,
}

/// Inherited pseudo element matches from pseudos of an ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritedPseudoElementMatches {
    /// Matches of pseudo styles from the pseudos of an ancestor node.

    pub pseudoElements: Vec<PseudoElementMatches>,
}

/// Match data for a CSS rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuleMatch {
    /// CSS rule in the match.

    pub rule: CSSRule,
    /// Matching selector indices in the rule's selectorList selectors (0-based).

    pub matchingSelectors: Vec<i64>,
}

/// Data for a simple selector (these are delimited by commas in a selector list).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolValue {
    /// Value text.

    pub text: String,
    /// Value range in the underlying resource (if available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Specificity of the selector.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificity: Option<Specificity>,
}

/// Specificity:
/// https://drafts.csswg.org/selectors/#specificity-rules

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Specificity {
    /// The a component, which represents the number of ID selectors.

    pub a: i64,
    /// The b component, which represents the number of class selectors, attributes selectors, and
    /// pseudo-classes.

    pub b: i64,
    /// The c component, which represents the number of type selectors and pseudo-elements.

    pub c: i64,
}

/// Selector list data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectorList {
    /// Selectors in the list.

    pub selectors: Vec<ProtocolValue>,
    /// Rule selector text.

    pub text: String,
}

/// CSS stylesheet metainformation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSStyleSheetHeader {
    /// The stylesheet identifier.

    pub styleSheetId: crate::dom::StyleSheetId,
    /// Owner frame identifier.

    pub frameId: crate::page::FrameId,
    /// Stylesheet resource URL. Empty if this is a constructed stylesheet created using
    /// new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported
    /// as a CSS module script).

    pub sourceURL: String,
    /// URL of source map associated with the stylesheet (if any).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceMapURL: Option<String>,
    /// Stylesheet origin.

    pub origin: StyleSheetOrigin,
    /// Stylesheet title.

    pub title: String,
    /// The backend id for the owner node of the stylesheet.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerNode: Option<crate::dom::BackendNodeId>,
    /// Denotes whether the stylesheet is disabled.

    pub disabled: bool,
    /// Whether the sourceURL field value comes from the sourceURL comment.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasSourceURL: Option<bool>,
    /// Whether this stylesheet is created for STYLE tag by parser. This flag is not set for
    /// document.written STYLE tags.

    pub isInline: bool,
    /// Whether this stylesheet is mutable. Inline stylesheets become mutable
    /// after they have been modified via CSSOM API.
    /// '<link>' element's stylesheets become mutable only if DevTools modifies them.
    /// Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.

    pub isMutable: bool,
    /// True if this stylesheet is created through new CSSStyleSheet() or imported as a
    /// CSS module script.

    pub isConstructed: bool,
    /// Line offset of the stylesheet within the resource (zero based).

    pub startLine: f64,
    /// Column offset of the stylesheet within the resource (zero based).

    pub startColumn: f64,
    /// Size of the content (in characters).

    pub length: f64,
    /// Line offset of the end of the stylesheet within the resource (zero based).

    pub endLine: f64,
    /// Column offset of the end of the stylesheet within the resource (zero based).

    pub endColumn: f64,
    /// If the style sheet was loaded from a network resource, this indicates when the resource failed to load

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loadingFailed: Option<bool>,
}

/// CSS rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSRule {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// Rule selector data.

    pub selectorList: SelectorList,
    /// Array of selectors from ancestor style rules, sorted by distance from the current rule.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nestingSelectors: Option<Vec<String>>,
    /// Parent stylesheet's origin.

    pub origin: StyleSheetOrigin,
    /// Associated style declaration.

    pub style: CSSStyle,
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub originTreeScopeNodeId: Option<crate::dom::BackendNodeId>,
    /// Media list array (for rules involving media queries). The array enumerates media queries
    /// starting with the innermost one, going outwards.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<CSSMedia>>,
    /// Container query list array (for rules involving container queries).
    /// The array enumerates container queries starting with the innermost one, going outwards.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerQueries: Option<Vec<CSSContainerQuery>>,
    /// @supports CSS at-rule array.
    /// The array enumerates @supports at-rules starting with the innermost one, going outwards.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports: Option<Vec<CSSSupports>>,
    /// Cascade layer array. Contains the layer hierarchy that this rule belongs to starting
    /// with the innermost layer and going outwards.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<CSSLayer>>,
    /// @scope CSS at-rule array.
    /// The array enumerates @scope at-rules starting with the innermost one, going outwards.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<CSSScope>>,
    /// The array keeps the types of ancestor CSSRules from the innermost going outwards.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleTypes: Option<Vec<CSSRuleType>>,
    /// @starting-style CSS at-rule array.
    /// The array enumerates @starting-style at-rules starting with the innermost one, going outwards.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub startingStyles: Option<Vec<CSSStartingStyle>>,
    /// @navigation CSS at-rule array.
    /// The array enumerates @navigation at-rules starting with the innermost one, going outwards.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigations: Option<Vec<CSSNavigation>>,
}

/// Enum indicating the type of a CSS rule, used to represent the order of a style rule's ancestors.
/// This list only contains rule types that are collected during the ancestor rule collection.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CSSRuleType {
    #[default]
    MediaRule,
    SupportsRule,
    ContainerRule,
    LayerRule,
    ScopeRule,
    StyleRule,
    StartingStyleRule,
    NavigationRule,
}

/// CSS coverage information.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuleUsage {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.

    pub styleSheetId: crate::dom::StyleSheetId,
    /// Offset of the start of the rule (including selector) from the beginning of the stylesheet.

    pub startOffset: f64,
    /// Offset of the end of the rule body from the beginning of the stylesheet.

    pub endOffset: f64,
    /// Indicates whether the rule was actually used by some element in the page.

    pub used: bool,
}

/// Text range within a resource. All numbers are zero-based.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceRange {
    /// Start line of range.

    pub startLine: i64,
    /// Start column of range (inclusive).

    pub startColumn: i64,
    /// End line of range

    pub endLine: i64,
    /// End column of range (exclusive).

    pub endColumn: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShorthandEntry {
    /// Shorthand name.

    pub name: String,
    /// Shorthand value.

    pub value: String,
    /// Whether the property has "!important" annotation (implies 'false' if absent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSComputedStyleProperty {
    /// Computed style property name.

    pub name: String,
    /// Computed style property value.

    pub value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComputedStyleExtraFields {
    /// Returns whether or not this node is being rendered with base appearance,
    /// which happens when it has its appearance property set to base/base-select
    /// or it is in the subtree of an element being rendered with base appearance.

    pub isAppearanceBase: bool,
}

/// CSS style representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSStyle {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// CSS properties in the style.

    pub cssProperties: Vec<CSSProperty>,
    /// Computed values for all shorthands found in the style.

    pub shorthandEntries: Vec<ShorthandEntry>,
    /// Style declaration text (if available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssText: Option<String>,
    /// Style declaration range in the enclosing stylesheet (if available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
}

/// CSS property declaration data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSProperty {
    /// The property name.

    pub name: String,
    /// The property value.

    pub value: String,
    /// Whether the property has "!important" annotation (implies 'false' if absent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
    /// Whether the property is implicit (implies 'false' if absent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<bool>,
    /// The full property text as specified in the style.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Whether the property is understood by the browser (implies 'true' if absent).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsedOk: Option<bool>,
    /// Whether the property is disabled by the user (present for source-based properties only).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The entire property range in the enclosing style declaration (if available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Parsed longhand components of this property if it is a shorthand.
    /// This field will be empty if the given property is not a shorthand.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub longhandProperties: Option<Vec<CSSProperty>>,
}

/// CSS media rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSMedia {
    /// Media query text.

    pub text: String,
    /// Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if
    /// specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked
    /// stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline
    /// stylesheet's STYLE tag.

    pub source: String,
    /// URL of the document containing the media query description.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceURL: Option<String>,
    /// The associated rule (@media or @import) header range in the enclosing stylesheet (if
    /// available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// Array of media queries.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediaList: Option<Vec<MediaQuery>>,
}

/// Media query descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaQuery {
    /// Array of media query expressions.

    pub expressions: Vec<MediaQueryExpression>,
    /// Whether the media query condition is satisfied.

    pub active: bool,
}

/// Media query expression descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaQueryExpression {
    /// Media query expression value.

    pub value: f64,
    /// Media query expression units.

    pub unit: String,
    /// Media query expression feature.

    pub feature: String,
    /// The associated range of the value text in the enclosing stylesheet (if available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueRange: Option<SourceRange>,
    /// Computed length of media query expression (if applicable).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub computedLength: Option<f64>,
}

/// CSS container query rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSContainerQuery {
    /// Container query text.

    pub text: String,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// Optional name for the container.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional physical axes queried for the container.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub physicalAxes: Option<crate::dom::PhysicalAxes>,
    /// Optional logical axes queried for the container.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logicalAxes: Option<crate::dom::LogicalAxes>,
    /// true if the query contains scroll-state() queries.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queriesScrollState: Option<bool>,
    /// true if the query contains anchored() queries.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queriesAnchored: Option<bool>,
}

/// CSS Supports at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSSupports {
    /// Supports rule text.

    pub text: String,
    /// Whether the supports condition is satisfied.

    pub active: bool,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
}

/// CSS Navigation at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSNavigation {
    /// Navigation rule text.

    pub text: String,
    /// Whether the navigation condition is satisfied.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
}

/// CSS Scope at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSScope {
    /// Scope rule text.

    pub text: String,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
}

/// CSS Layer at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSLayer {
    /// Layer name.

    pub text: String,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
}

/// CSS Starting Style at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSStartingStyle {
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
}

/// CSS Layer data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSLayerData {
    /// Layer name.

    pub name: String,
    /// Direct sub-layers

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subLayers: Option<Vec<CSSLayerData>>,
    /// Layer order. The order determines the order of the layer in the cascade order.
    /// A higher number has higher priority in the cascade order.

    pub order: f64,
}

/// Information about amount of glyphs that were rendered with given font.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlatformFontUsage {
    /// Font's family name reported by platform.

    pub familyName: String,
    /// Font's PostScript name reported by platform.

    pub postScriptName: String,
    /// Indicates if the font was downloaded or resolved locally.

    pub isCustomFont: bool,
    /// Amount of glyphs that were rendered with this font.

    pub glyphCount: f64,
}

/// Information about font variation axes for variable fonts

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FontVariationAxis {
    /// The font-variation-setting tag (a.k.a. "axis tag").

    pub tag: String,
    /// Human-readable variation name in the default language (normally, "en").

    pub name: String,
    /// The minimum value (inclusive) the font supports for this tag.

    pub minValue: f64,
    /// The maximum value (inclusive) the font supports for this tag.

    pub maxValue: f64,
    /// The default value.

    pub defaultValue: f64,
}

/// Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
/// and additional information such as platformFontFamily and fontVariationAxes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FontFace {
    /// The font-family.

    pub fontFamily: String,
    /// The font-style.

    pub fontStyle: String,
    /// The font-variant.

    pub fontVariant: String,
    /// The font-weight.

    pub fontWeight: String,
    /// The font-stretch.

    pub fontStretch: String,
    /// The font-display.

    pub fontDisplay: String,
    /// The unicode-range.

    pub unicodeRange: String,
    /// The src.

    pub src: String,
    /// The resolved platform font family

    pub platformFontFamily: String,
    /// Available variation settings (a.k.a. "axes").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fontVariationAxes: Option<Vec<FontVariationAxis>>,
}

/// CSS try rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSTryRule {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// Parent stylesheet's origin.

    pub origin: StyleSheetOrigin,
    /// Associated style declaration.

    pub style: CSSStyle,
}

/// CSS @position-try rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSPositionTryRule {
    /// The prelude dashed-ident name

    pub name: ProtocolValue,
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// Parent stylesheet's origin.

    pub origin: StyleSheetOrigin,
    /// Associated style declaration.

    pub style: CSSStyle,

    pub active: bool,
}

/// CSS keyframes rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSKeyframesRule {
    /// Animation name.

    pub animationName: ProtocolValue,
    /// List of keyframes.

    pub keyframes: Vec<CSSKeyframeRule>,
}

/// Representation of a custom property registration through CSS.registerProperty

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSPropertyRegistration {

    pub propertyName: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialValue: Option<ProtocolValue>,

    pub inherits: bool,

    pub syntax: String,
}

/// CSS generic @rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSAtRule {
    /// Type of at-rule.

    #[serde(rename = "type")]
    pub type_: String,
    /// Subsection of font-feature-values, if this is a subsection.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsection: Option<String>,
    /// LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)
    /// Associated name, if applicable.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ProtocolValue>,
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// Parent stylesheet's origin.

    pub origin: StyleSheetOrigin,
    /// Associated style declaration.

    pub style: CSSStyle,
}

/// CSS property at-rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSPropertyRule {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// Parent stylesheet's origin.

    pub origin: StyleSheetOrigin,
    /// Associated property name.

    pub propertyName: ProtocolValue,
    /// Associated style declaration.

    pub style: CSSStyle,
}

/// CSS function argument representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionParameter {
    /// The parameter name.

    pub name: String,
    /// The parameter type.

    #[serde(rename = "type")]
    pub type_: String,
}

/// CSS function conditional block representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionConditionNode {
    /// Media query for this conditional block. Only one type of condition should be set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<CSSMedia>,
    /// Container query for this conditional block. Only one type of condition should be set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerQueries: Option<CSSContainerQuery>,
    /// @supports CSS at-rule condition. Only one type of condition should be set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports: Option<CSSSupports>,
    /// @navigation condition. Only one type of condition should be set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation: Option<CSSNavigation>,
    /// Block body.

    pub children: Vec<CSSFunctionNode>,
    /// The condition text.

    pub conditionText: String,
}

/// Section of the body of a CSS function rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionNode {
    /// A conditional block. If set, style should not be set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<CSSFunctionConditionNode>,
    /// Values set by this node. If set, condition should not be set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CSSStyle>,
}

/// CSS function at-rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionRule {
    /// Name of the function.

    pub name: ProtocolValue,
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// Parent stylesheet's origin.

    pub origin: StyleSheetOrigin,
    /// List of parameters.

    pub parameters: Vec<CSSFunctionParameter>,
    /// Function body.

    pub children: Vec<CSSFunctionNode>,
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub originTreeScopeNodeId: Option<crate::dom::BackendNodeId>,
}

/// CSS keyframe rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSKeyframeRule {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styleSheetId: Option<crate::dom::StyleSheetId>,
    /// Parent stylesheet's origin.

    pub origin: StyleSheetOrigin,
    /// Associated key text.

    pub keyText: ProtocolValue,
    /// Associated style declaration.

    pub style: CSSStyle,
}

/// A descriptor of operation to mutate style declaration text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StyleDeclarationEdit {
    /// The css style sheet identifier.

    pub styleSheetId: crate::dom::StyleSheetId,
    /// The range of the style text in the enclosing stylesheet.

    pub range: SourceRange,
    /// New style text.

    pub text: String,
}

/// Inserts a new rule with the given 'ruleText' in a stylesheet with given 'styleSheetId', at the
/// position specified by 'location'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddRuleParams {
    /// The css style sheet identifier where a new rule should be inserted.

    pub styleSheetId: crate::dom::StyleSheetId,
    /// The text of a new rule.

    pub ruleText: String,
    /// Text position of a new rule in the target style sheet.

    pub location: SourceRange,
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeForPropertySyntaxValidation: Option<crate::dom::NodeId>,
}

/// Inserts a new rule with the given 'ruleText' in a stylesheet with given 'styleSheetId', at the
/// position specified by 'location'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddRuleReturns {
    /// The newly created rule.

    pub rule: CSSRule,
}

impl AddRuleParams { pub const METHOD: &'static str = "CSS.addRule"; }

impl crate::CdpCommand for AddRuleParams {
    const METHOD: &'static str = "CSS.addRule";
    type Response = AddRuleReturns;
}

/// Returns all class names from specified stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesParams {

    pub styleSheetId: crate::dom::StyleSheetId,
}

/// Returns all class names from specified stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesReturns {
    /// Class name list.

    pub classNames: Vec<String>,
}

impl CollectClassNamesParams { pub const METHOD: &'static str = "CSS.collectClassNames"; }

impl crate::CdpCommand for CollectClassNamesParams {
    const METHOD: &'static str = "CSS.collectClassNames";
    type Response = CollectClassNamesReturns;
}

/// Creates a new special "via-inspector" stylesheet in the frame with given 'frameId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateStyleSheetParams {
    /// Identifier of the frame where "via-inspector" stylesheet should be created.

    pub frameId: crate::page::FrameId,
    /// If true, creates a new stylesheet for every call. If false,
    /// returns a stylesheet previously created by a call with force=false
    /// for the frame's document if it exists or creates a new stylesheet
    /// (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

/// Creates a new special "via-inspector" stylesheet in the frame with given 'frameId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateStyleSheetReturns {
    /// Identifier of the created "via-inspector" stylesheet.

    pub styleSheetId: crate::dom::StyleSheetId,
}

impl CreateStyleSheetParams { pub const METHOD: &'static str = "CSS.createStyleSheet"; }

impl crate::CdpCommand for CreateStyleSheetParams {
    const METHOD: &'static str = "CSS.createStyleSheet";
    type Response = CreateStyleSheetReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "CSS.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "CSS.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "CSS.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "CSS.enable";
    type Response = crate::EmptyReturns;
}

/// Ensures that the given node will have specified pseudo-classes whenever its style is computed by
/// the browser.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForcePseudoStateParams {
    /// The element id for which to force the pseudo state.

    pub nodeId: crate::dom::NodeId,
    /// Element pseudo classes to force when computing the element's style.

    pub forcedPseudoClasses: Vec<String>,
}

impl ForcePseudoStateParams { pub const METHOD: &'static str = "CSS.forcePseudoState"; }

impl crate::CdpCommand for ForcePseudoStateParams {
    const METHOD: &'static str = "CSS.forcePseudoState";
    type Response = crate::EmptyReturns;
}

/// Ensures that the given node is in its starting-style state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForceStartingStyleParams {
    /// The element id for which to force the starting-style state.

    pub nodeId: crate::dom::NodeId,
    /// Boolean indicating if this is on or off.

    pub forced: bool,
}

impl ForceStartingStyleParams { pub const METHOD: &'static str = "CSS.forceStartingStyle"; }

impl crate::CdpCommand for ForceStartingStyleParams {
    const METHOD: &'static str = "CSS.forceStartingStyle";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBackgroundColorsParams {
    /// Id of the node to get background colors for.

    pub nodeId: crate::dom::NodeId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBackgroundColorsReturns {
    /// The range of background colors behind this element, if it contains any visible text. If no
    /// visible text is present, this will be undefined. In the case of a flat background color,
    /// this will consist of simply that color. In the case of a gradient, this will consist of each
    /// of the color stops. For anything more complicated, this will be an empty array. Images will
    /// be ignored (as if the image had failed to load).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backgroundColors: Option<Vec<String>>,
    /// The computed font size for this node, as a CSS computed value string (e.g. '12px').

    #[serde(skip_serializing_if = "Option::is_none")]
    pub computedFontSize: Option<String>,
    /// The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
    /// '100').

    #[serde(skip_serializing_if = "Option::is_none")]
    pub computedFontWeight: Option<String>,
}

impl GetBackgroundColorsParams { pub const METHOD: &'static str = "CSS.getBackgroundColors"; }

impl crate::CdpCommand for GetBackgroundColorsParams {
    const METHOD: &'static str = "CSS.getBackgroundColors";
    type Response = GetBackgroundColorsReturns;
}

/// Returns the computed style for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetComputedStyleForNodeParams {

    pub nodeId: crate::dom::NodeId,
}

/// Returns the computed style for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetComputedStyleForNodeReturns {
    /// Computed style for the specified DOM node.

    pub computedStyle: Vec<CSSComputedStyleProperty>,
    /// A list of non-standard "extra fields" which blink stores alongside each
    /// computed style.

    pub extraFields: ComputedStyleExtraFields,
}

impl GetComputedStyleForNodeParams { pub const METHOD: &'static str = "CSS.getComputedStyleForNode"; }

impl crate::CdpCommand for GetComputedStyleForNodeParams {
    const METHOD: &'static str = "CSS.getComputedStyleForNode";
    type Response = GetComputedStyleForNodeReturns;
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveValuesParams {
    /// Cascade-dependent keywords (revert/revert-layer) do not work.

    pub values: Vec<String>,
    /// Id of the node in whose context the expression is evaluated

    pub nodeId: crate::dom::NodeId,
    /// Only longhands and custom property names are accepted.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub propertyName: Option<String>,
    /// Pseudo element type, only works for pseudo elements that generate
    /// elements in the tree, such as ::before and ::after.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoType: Option<crate::dom::PseudoType>,
    /// Pseudo element custom ident.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoIdentifier: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveValuesReturns {

    pub results: Vec<String>,
}

impl ResolveValuesParams { pub const METHOD: &'static str = "CSS.resolveValues"; }

impl crate::CdpCommand for ResolveValuesParams {
    const METHOD: &'static str = "CSS.resolveValues";
    type Response = ResolveValuesReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLonghandPropertiesParams {

    pub shorthandName: String,

    pub value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLonghandPropertiesReturns {

    pub longhandProperties: Vec<CSSProperty>,
}

impl GetLonghandPropertiesParams { pub const METHOD: &'static str = "CSS.getLonghandProperties"; }

impl crate::CdpCommand for GetLonghandPropertiesParams {
    const METHOD: &'static str = "CSS.getLonghandProperties";
    type Response = GetLonghandPropertiesReturns;
}

/// Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
/// attributes) for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInlineStylesForNodeParams {

    pub nodeId: crate::dom::NodeId,
}

/// Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
/// attributes) for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInlineStylesForNodeReturns {
    /// Inline style for the specified DOM node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlineStyle: Option<CSSStyle>,
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributesStyle: Option<CSSStyle>,
}

impl GetInlineStylesForNodeParams { pub const METHOD: &'static str = "CSS.getInlineStylesForNode"; }

impl crate::CdpCommand for GetInlineStylesForNodeParams {
    const METHOD: &'static str = "CSS.getInlineStylesForNode";
    type Response = GetInlineStylesForNodeReturns;
}

/// Returns the styles coming from animations & transitions
/// including the animation & transition styles coming from inheritance chain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnimatedStylesForNodeParams {

    pub nodeId: crate::dom::NodeId,
}

/// Returns the styles coming from animations & transitions
/// including the animation & transition styles coming from inheritance chain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnimatedStylesForNodeReturns {
    /// Styles coming from animations.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animationStyles: Option<Vec<CSSAnimationStyle>>,
    /// Style coming from transitions.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitionsStyle: Option<CSSStyle>,
    /// Inherited style entries for animationsStyle and transitionsStyle from
    /// the inheritance chain of the element.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<Vec<InheritedAnimatedStyleEntry>>,
}

impl GetAnimatedStylesForNodeParams { pub const METHOD: &'static str = "CSS.getAnimatedStylesForNode"; }

impl crate::CdpCommand for GetAnimatedStylesForNodeParams {
    const METHOD: &'static str = "CSS.getAnimatedStylesForNode";
    type Response = GetAnimatedStylesForNodeReturns;
}

/// Returns requested styles for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMatchedStylesForNodeParams {

    pub nodeId: crate::dom::NodeId,
}

/// Returns requested styles for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMatchedStylesForNodeReturns {
    /// Inline style for the specified DOM node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlineStyle: Option<CSSStyle>,
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributesStyle: Option<CSSStyle>,
    /// CSS rules matching this node, from all applicable stylesheets.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchedCSSRules: Option<Vec<RuleMatch>>,
    /// Pseudo style matches for this node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudoElements: Option<Vec<PseudoElementMatches>>,
    /// A chain of inherited styles (from the immediate node parent up to the DOM tree root).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<Vec<InheritedStyleEntry>>,
    /// A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inheritedPseudoElements: Option<Vec<InheritedPseudoElementMatches>>,
    /// A list of CSS keyframed animations matching this node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssKeyframesRules: Option<Vec<CSSKeyframesRule>>,
    /// A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssPositionTryRules: Option<Vec<CSSPositionTryRule>>,
    /// Index of the active fallback in the applied position-try-fallback property,
    /// will not be set if there is no active position-try fallback.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub activePositionFallbackIndex: Option<u64>,
    /// A list of CSS at-property rules matching this node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssPropertyRules: Option<Vec<CSSPropertyRule>>,
    /// A list of CSS property registrations matching this node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssPropertyRegistrations: Option<Vec<CSSPropertyRegistration>>,
    /// A list of simple @rules matching this node or its pseudo-elements.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssAtRules: Option<Vec<CSSAtRule>>,
    /// Id of the first parent element that does not have display: contents.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentLayoutNodeId: Option<crate::dom::NodeId>,
    /// A list of CSS at-function rules referenced by styles of this node.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssFunctionRules: Option<Vec<CSSFunctionRule>>,
}

impl GetMatchedStylesForNodeParams { pub const METHOD: &'static str = "CSS.getMatchedStylesForNode"; }

impl crate::CdpCommand for GetMatchedStylesForNodeParams {
    const METHOD: &'static str = "CSS.getMatchedStylesForNode";
    type Response = GetMatchedStylesForNodeReturns;
}

/// Returns the values of the default UA-defined environment variables used in env()

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEnvironmentVariablesReturns {

    pub environmentVariables: serde_json::Map<String, JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEnvironmentVariablesParams {}

impl GetEnvironmentVariablesParams { pub const METHOD: &'static str = "CSS.getEnvironmentVariables"; }

impl crate::CdpCommand for GetEnvironmentVariablesParams {
    const METHOD: &'static str = "CSS.getEnvironmentVariables";
    type Response = GetEnvironmentVariablesReturns;
}

/// Returns all media queries parsed by the rendering engine.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaQueriesReturns {

    pub medias: Vec<CSSMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMediaQueriesParams {}

impl GetMediaQueriesParams { pub const METHOD: &'static str = "CSS.getMediaQueries"; }

impl crate::CdpCommand for GetMediaQueriesParams {
    const METHOD: &'static str = "CSS.getMediaQueries";
    type Response = GetMediaQueriesReturns;
}

/// Requests information about platform fonts which we used to render child TextNodes in the given
/// node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPlatformFontsForNodeParams {

    pub nodeId: crate::dom::NodeId,
}

/// Requests information about platform fonts which we used to render child TextNodes in the given
/// node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPlatformFontsForNodeReturns {
    /// Usage statistics for every employed platform font.

    pub fonts: Vec<PlatformFontUsage>,
}

impl GetPlatformFontsForNodeParams { pub const METHOD: &'static str = "CSS.getPlatformFontsForNode"; }

impl crate::CdpCommand for GetPlatformFontsForNodeParams {
    const METHOD: &'static str = "CSS.getPlatformFontsForNode";
    type Response = GetPlatformFontsForNodeReturns;
}

/// Returns the current textual content for a stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStyleSheetTextParams {

    pub styleSheetId: crate::dom::StyleSheetId,
}

/// Returns the current textual content for a stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStyleSheetTextReturns {
    /// The stylesheet text.

    pub text: String,
}

impl GetStyleSheetTextParams { pub const METHOD: &'static str = "CSS.getStyleSheetText"; }

impl crate::CdpCommand for GetStyleSheetTextParams {
    const METHOD: &'static str = "CSS.getStyleSheetText";
    type Response = GetStyleSheetTextReturns;
}

/// Returns all layers parsed by the rendering engine for the tree scope of a node.
/// Given a DOM element identified by nodeId, getLayersForNode returns the root
/// layer for the nearest ancestor document or shadow root. The layer root contains
/// the full layer tree for the tree scope and their ordering.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLayersForNodeParams {

    pub nodeId: crate::dom::NodeId,
}

/// Returns all layers parsed by the rendering engine for the tree scope of a node.
/// Given a DOM element identified by nodeId, getLayersForNode returns the root
/// layer for the nearest ancestor document or shadow root. The layer root contains
/// the full layer tree for the tree scope and their ordering.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLayersForNodeReturns {

    pub rootLayer: CSSLayerData,
}

impl GetLayersForNodeParams { pub const METHOD: &'static str = "CSS.getLayersForNode"; }

impl crate::CdpCommand for GetLayersForNodeParams {
    const METHOD: &'static str = "CSS.getLayersForNode";
    type Response = GetLayersForNodeReturns;
}

/// Given a CSS selector text and a style sheet ID, getLocationForSelector
/// returns an array of locations of the CSS selector in the style sheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationForSelectorParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub selectorText: String,
}

/// Given a CSS selector text and a style sheet ID, getLocationForSelector
/// returns an array of locations of the CSS selector in the style sheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationForSelectorReturns {

    pub ranges: Vec<SourceRange>,
}

impl GetLocationForSelectorParams { pub const METHOD: &'static str = "CSS.getLocationForSelector"; }

impl crate::CdpCommand for GetLocationForSelectorParams {
    const METHOD: &'static str = "CSS.getLocationForSelector";
    type Response = GetLocationForSelectorReturns;
}

/// Starts tracking the given node for the computed style updates
/// and whenever the computed style is updated for node, it queues
/// a 'computedStyleUpdated' event with throttling.
/// There can only be 1 node tracked for computed style updates
/// so passing a new node id removes tracking from the previous node.
/// Pass 'undefined' to disable tracking.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackComputedStyleUpdatesForNodeParams {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::NodeId>,
}

impl TrackComputedStyleUpdatesForNodeParams { pub const METHOD: &'static str = "CSS.trackComputedStyleUpdatesForNode"; }

impl crate::CdpCommand for TrackComputedStyleUpdatesForNodeParams {
    const METHOD: &'static str = "CSS.trackComputedStyleUpdatesForNode";
    type Response = crate::EmptyReturns;
}

/// Starts tracking the given computed styles for updates. The specified array of properties
/// replaces the one previously specified. Pass empty array to disable tracking.
/// Use takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.
/// The changes to computed style properties are only tracked for nodes pushed to the front-end
/// by the DOM agent. If no changes to the tracked properties occur after the node has been pushed
/// to the front-end, no updates will be issued for the node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackComputedStyleUpdatesParams {

    pub propertiesToTrack: Vec<CSSComputedStyleProperty>,
}

impl TrackComputedStyleUpdatesParams { pub const METHOD: &'static str = "CSS.trackComputedStyleUpdates"; }

impl crate::CdpCommand for TrackComputedStyleUpdatesParams {
    const METHOD: &'static str = "CSS.trackComputedStyleUpdates";
    type Response = crate::EmptyReturns;
}

/// Polls the next batch of computed style updates.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeComputedStyleUpdatesReturns {
    /// The list of node Ids that have their tracked computed styles updated.

    pub nodeIds: Vec<crate::dom::NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TakeComputedStyleUpdatesParams {}

impl TakeComputedStyleUpdatesParams { pub const METHOD: &'static str = "CSS.takeComputedStyleUpdates"; }

impl crate::CdpCommand for TakeComputedStyleUpdatesParams {
    const METHOD: &'static str = "CSS.takeComputedStyleUpdates";
    type Response = TakeComputedStyleUpdatesReturns;
}

/// Find a rule with the given active property for the given node and set the new value for this
/// property

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEffectivePropertyValueForNodeParams {
    /// The element id for which to set property.

    pub nodeId: crate::dom::NodeId,

    pub propertyName: String,

    pub value: String,
}

impl SetEffectivePropertyValueForNodeParams { pub const METHOD: &'static str = "CSS.setEffectivePropertyValueForNode"; }

impl crate::CdpCommand for SetEffectivePropertyValueForNodeParams {
    const METHOD: &'static str = "CSS.setEffectivePropertyValueForNode";
    type Response = crate::EmptyReturns;
}

/// Modifies the property rule property name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPropertyRulePropertyNameParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub range: SourceRange,

    pub propertyName: String,
}

/// Modifies the property rule property name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPropertyRulePropertyNameReturns {
    /// The resulting key text after modification.

    pub propertyName: ProtocolValue,
}

impl SetPropertyRulePropertyNameParams { pub const METHOD: &'static str = "CSS.setPropertyRulePropertyName"; }

impl crate::CdpCommand for SetPropertyRulePropertyNameParams {
    const METHOD: &'static str = "CSS.setPropertyRulePropertyName";
    type Response = SetPropertyRulePropertyNameReturns;
}

/// Modifies the keyframe rule key text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetKeyframeKeyParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub range: SourceRange,

    pub keyText: String,
}

/// Modifies the keyframe rule key text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetKeyframeKeyReturns {
    /// The resulting key text after modification.

    pub keyText: ProtocolValue,
}

impl SetKeyframeKeyParams { pub const METHOD: &'static str = "CSS.setKeyframeKey"; }

impl crate::CdpCommand for SetKeyframeKeyParams {
    const METHOD: &'static str = "CSS.setKeyframeKey";
    type Response = SetKeyframeKeyReturns;
}

/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMediaTextParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub range: SourceRange,

    pub text: String,
}

/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMediaTextReturns {
    /// The resulting CSS media rule after modification.

    pub media: CSSMedia,
}

impl SetMediaTextParams { pub const METHOD: &'static str = "CSS.setMediaText"; }

impl crate::CdpCommand for SetMediaTextParams {
    const METHOD: &'static str = "CSS.setMediaText";
    type Response = SetMediaTextReturns;
}

/// Modifies the expression of a container query.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryTextParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub range: SourceRange,

    pub text: String,
}

/// Modifies the expression of a container query.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryTextReturns {
    /// The resulting CSS container query rule after modification.

    pub containerQuery: CSSContainerQuery,
}

impl SetContainerQueryTextParams { pub const METHOD: &'static str = "CSS.setContainerQueryText"; }

impl crate::CdpCommand for SetContainerQueryTextParams {
    const METHOD: &'static str = "CSS.setContainerQueryText";
    type Response = SetContainerQueryTextReturns;
}

/// Modifies the expression of a supports at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSupportsTextParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub range: SourceRange,

    pub text: String,
}

/// Modifies the expression of a supports at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSupportsTextReturns {
    /// The resulting CSS Supports rule after modification.

    pub supports: CSSSupports,
}

impl SetSupportsTextParams { pub const METHOD: &'static str = "CSS.setSupportsText"; }

impl crate::CdpCommand for SetSupportsTextParams {
    const METHOD: &'static str = "CSS.setSupportsText";
    type Response = SetSupportsTextReturns;
}

/// Modifies the expression of a navigation at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNavigationTextParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub range: SourceRange,

    pub text: String,
}

/// Modifies the expression of a navigation at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNavigationTextReturns {
    /// The resulting CSS Navigation rule after modification.

    pub navigation: CSSNavigation,
}

impl SetNavigationTextParams { pub const METHOD: &'static str = "CSS.setNavigationText"; }

impl crate::CdpCommand for SetNavigationTextParams {
    const METHOD: &'static str = "CSS.setNavigationText";
    type Response = SetNavigationTextReturns;
}

/// Modifies the expression of a scope at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScopeTextParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub range: SourceRange,

    pub text: String,
}

/// Modifies the expression of a scope at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScopeTextReturns {
    /// The resulting CSS Scope rule after modification.

    pub scope: CSSScope,
}

impl SetScopeTextParams { pub const METHOD: &'static str = "CSS.setScopeText"; }

impl crate::CdpCommand for SetScopeTextParams {
    const METHOD: &'static str = "CSS.setScopeText";
    type Response = SetScopeTextReturns;
}

/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRuleSelectorParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub range: SourceRange,

    pub selector: String,
}

/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRuleSelectorReturns {
    /// The resulting selector list after modification.

    pub selectorList: SelectorList,
}

impl SetRuleSelectorParams { pub const METHOD: &'static str = "CSS.setRuleSelector"; }

impl crate::CdpCommand for SetRuleSelectorParams {
    const METHOD: &'static str = "CSS.setRuleSelector";
    type Response = SetRuleSelectorReturns;
}

/// Sets the new stylesheet text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleSheetTextParams {

    pub styleSheetId: crate::dom::StyleSheetId,

    pub text: String,
}

/// Sets the new stylesheet text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleSheetTextReturns {
    /// URL of source map associated with script (if any).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceMapURL: Option<String>,
}

impl SetStyleSheetTextParams { pub const METHOD: &'static str = "CSS.setStyleSheetText"; }

impl crate::CdpCommand for SetStyleSheetTextParams {
    const METHOD: &'static str = "CSS.setStyleSheetText";
    type Response = SetStyleSheetTextReturns;
}

/// Applies specified style edits one after another in the given order.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleTextsParams {

    pub edits: Vec<StyleDeclarationEdit>,
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeForPropertySyntaxValidation: Option<crate::dom::NodeId>,
}

/// Applies specified style edits one after another in the given order.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleTextsReturns {
    /// The resulting styles after modification.

    pub styles: Vec<CSSStyle>,
}

impl SetStyleTextsParams { pub const METHOD: &'static str = "CSS.setStyleTexts"; }

impl crate::CdpCommand for SetStyleTextsParams {
    const METHOD: &'static str = "CSS.setStyleTexts";
    type Response = SetStyleTextsReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartRuleUsageTrackingParams {}

impl StartRuleUsageTrackingParams { pub const METHOD: &'static str = "CSS.startRuleUsageTracking"; }

impl crate::CdpCommand for StartRuleUsageTrackingParams {
    const METHOD: &'static str = "CSS.startRuleUsageTracking";
    type Response = crate::EmptyReturns;
}

/// Stop tracking rule usage and return the list of rules that were used since last call to
/// 'takeCoverageDelta' (or since start of coverage instrumentation).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopRuleUsageTrackingReturns {

    pub ruleUsage: Vec<RuleUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopRuleUsageTrackingParams {}

impl StopRuleUsageTrackingParams { pub const METHOD: &'static str = "CSS.stopRuleUsageTracking"; }

impl crate::CdpCommand for StopRuleUsageTrackingParams {
    const METHOD: &'static str = "CSS.stopRuleUsageTracking";
    type Response = StopRuleUsageTrackingReturns;
}

/// Obtain list of rules that became used since last call to this method (or since start of coverage
/// instrumentation).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeCoverageDeltaReturns {

    pub coverage: Vec<RuleUsage>,
    /// Monotonically increasing time, in seconds.

    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TakeCoverageDeltaParams {}

impl TakeCoverageDeltaParams { pub const METHOD: &'static str = "CSS.takeCoverageDelta"; }

impl crate::CdpCommand for TakeCoverageDeltaParams {
    const METHOD: &'static str = "CSS.takeCoverageDelta";
    type Response = TakeCoverageDeltaReturns;
}

/// Enables/disables rendering of local CSS fonts (enabled by default).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetLocalFontsEnabledParams {
    /// Whether rendering of local fonts is enabled.

    pub enabled: bool,
}

impl SetLocalFontsEnabledParams { pub const METHOD: &'static str = "CSS.setLocalFontsEnabled"; }

impl crate::CdpCommand for SetLocalFontsEnabledParams {
    const METHOD: &'static str = "CSS.setLocalFontsEnabled";
    type Response = crate::EmptyReturns;
}
