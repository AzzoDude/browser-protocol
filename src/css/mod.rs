//! This domain exposes CSS read/write operations. All CSS objects (stylesheets, rules, and styles)
//! have an associated 'id' used in subsequent operations on the related object. Each object type has
//! a specific 'id' structure, and those are not interchangeable between objects of different kinds.
//! CSS objects can be loaded using the 'get*ForNode()' calls (which accept a DOM node id). A client
//! can also keep track of stylesheets via the 'styleSheetAdded'/'styleSheetRemoved' events and
//! subsequently load the required stylesheet contents using the 'getStyleSheet[Text]()' methods.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PseudoElementMatches<'a> {
    /// Pseudo element type.
    pseudoType: crate::dom::PseudoType,
    /// Pseudo element custom ident.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoIdentifier: Option<Cow<'a, str>>,
    /// Matches of CSS rules applicable to the pseudo style.
    matches: Vec<RuleMatch<'a>>,
}

impl<'a> PseudoElementMatches<'a> {
    pub fn builder() -> PseudoElementMatchesBuilder<'a> { PseudoElementMatchesBuilder::default() }
    pub fn pseudoType(&self) -> &crate::dom::PseudoType { &self.pseudoType }
    pub fn pseudoIdentifier(&self) -> Option<&str> { self.pseudoIdentifier.as_deref() }
    pub fn matches(&self) -> &[RuleMatch<'a>] { &self.matches }
}

#[derive(Default)]
pub struct PseudoElementMatchesBuilder<'a> {
    pseudoType: Option<crate::dom::PseudoType>,
    pseudoIdentifier: Option<Cow<'a, str>>,
    matches: Option<Vec<RuleMatch<'a>>>,
}

impl<'a> PseudoElementMatchesBuilder<'a> {
    /// Pseudo element type.
    pub fn pseudoType(mut self, pseudoType: crate::dom::PseudoType) -> Self { self.pseudoType = Some(pseudoType); self }
    /// Pseudo element custom ident.
    pub fn pseudoIdentifier(mut self, pseudoIdentifier: impl Into<Cow<'a, str>>) -> Self { self.pseudoIdentifier = Some(pseudoIdentifier.into()); self }
    /// Matches of CSS rules applicable to the pseudo style.
    pub fn matches(mut self, matches: Vec<RuleMatch<'a>>) -> Self { self.matches = Some(matches); self }
    pub fn build(self) -> PseudoElementMatches<'a> {
        PseudoElementMatches {
            pseudoType: self.pseudoType.unwrap_or_default(),
            pseudoIdentifier: self.pseudoIdentifier,
            matches: self.matches.unwrap_or_default(),
        }
    }
}

/// CSS style coming from animations with the name of the animation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSAnimationStyle<'a> {
    /// The name of the animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    /// The style coming from the animation.
    style: CSSStyle<'a>,
}

impl<'a> CSSAnimationStyle<'a> {
    pub fn builder() -> CSSAnimationStyleBuilder<'a> { CSSAnimationStyleBuilder::default() }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}

#[derive(Default)]
pub struct CSSAnimationStyleBuilder<'a> {
    name: Option<Cow<'a, str>>,
    style: Option<CSSStyle<'a>>,
}

impl<'a> CSSAnimationStyleBuilder<'a> {
    /// The name of the animation.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// The style coming from the animation.
    pub fn style(mut self, style: CSSStyle<'a>) -> Self { self.style = Some(style); self }
    pub fn build(self) -> CSSAnimationStyle<'a> {
        CSSAnimationStyle {
            name: self.name,
            style: self.style.unwrap_or_default(),
        }
    }
}

/// Inherited CSS rule collection from ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritedStyleEntry<'a> {
    /// The ancestor node's inline style, if any, in the style inheritance chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    inlineStyle: Option<CSSStyle<'a>>,
    /// Matches of CSS rules matching the ancestor node in the style inheritance chain.
    matchedCSSRules: Vec<RuleMatch<'a>>,
}

impl<'a> InheritedStyleEntry<'a> {
    pub fn builder() -> InheritedStyleEntryBuilder<'a> { InheritedStyleEntryBuilder::default() }
    pub fn inlineStyle(&self) -> Option<&CSSStyle<'a>> { self.inlineStyle.as_ref() }
    pub fn matchedCSSRules(&self) -> &[RuleMatch<'a>] { &self.matchedCSSRules }
}

#[derive(Default)]
pub struct InheritedStyleEntryBuilder<'a> {
    inlineStyle: Option<CSSStyle<'a>>,
    matchedCSSRules: Option<Vec<RuleMatch<'a>>>,
}

impl<'a> InheritedStyleEntryBuilder<'a> {
    /// The ancestor node's inline style, if any, in the style inheritance chain.
    pub fn inlineStyle(mut self, inlineStyle: CSSStyle<'a>) -> Self { self.inlineStyle = Some(inlineStyle); self }
    /// Matches of CSS rules matching the ancestor node in the style inheritance chain.
    pub fn matchedCSSRules(mut self, matchedCSSRules: Vec<RuleMatch<'a>>) -> Self { self.matchedCSSRules = Some(matchedCSSRules); self }
    pub fn build(self) -> InheritedStyleEntry<'a> {
        InheritedStyleEntry {
            inlineStyle: self.inlineStyle,
            matchedCSSRules: self.matchedCSSRules.unwrap_or_default(),
        }
    }
}

/// Inherited CSS style collection for animated styles from ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritedAnimatedStyleEntry<'a> {
    /// Styles coming from the animations of the ancestor, if any, in the style inheritance chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    animationStyles: Option<Vec<CSSAnimationStyle<'a>>>,
    /// The style coming from the transitions of the ancestor, if any, in the style inheritance chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    transitionsStyle: Option<CSSStyle<'a>>,
}

impl<'a> InheritedAnimatedStyleEntry<'a> {
    pub fn builder() -> InheritedAnimatedStyleEntryBuilder<'a> { InheritedAnimatedStyleEntryBuilder::default() }
    pub fn animationStyles(&self) -> Option<&[CSSAnimationStyle<'a>]> { self.animationStyles.as_deref() }
    pub fn transitionsStyle(&self) -> Option<&CSSStyle<'a>> { self.transitionsStyle.as_ref() }
}

#[derive(Default)]
pub struct InheritedAnimatedStyleEntryBuilder<'a> {
    animationStyles: Option<Vec<CSSAnimationStyle<'a>>>,
    transitionsStyle: Option<CSSStyle<'a>>,
}

impl<'a> InheritedAnimatedStyleEntryBuilder<'a> {
    /// Styles coming from the animations of the ancestor, if any, in the style inheritance chain.
    pub fn animationStyles(mut self, animationStyles: Vec<CSSAnimationStyle<'a>>) -> Self { self.animationStyles = Some(animationStyles); self }
    /// The style coming from the transitions of the ancestor, if any, in the style inheritance chain.
    pub fn transitionsStyle(mut self, transitionsStyle: CSSStyle<'a>) -> Self { self.transitionsStyle = Some(transitionsStyle); self }
    pub fn build(self) -> InheritedAnimatedStyleEntry<'a> {
        InheritedAnimatedStyleEntry {
            animationStyles: self.animationStyles,
            transitionsStyle: self.transitionsStyle,
        }
    }
}

/// Inherited pseudo element matches from pseudos of an ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritedPseudoElementMatches<'a> {
    /// Matches of pseudo styles from the pseudos of an ancestor node.
    pseudoElements: Vec<PseudoElementMatches<'a>>,
}

impl<'a> InheritedPseudoElementMatches<'a> {
    pub fn builder() -> InheritedPseudoElementMatchesBuilder<'a> { InheritedPseudoElementMatchesBuilder::default() }
    pub fn pseudoElements(&self) -> &[PseudoElementMatches<'a>] { &self.pseudoElements }
}

#[derive(Default)]
pub struct InheritedPseudoElementMatchesBuilder<'a> {
    pseudoElements: Option<Vec<PseudoElementMatches<'a>>>,
}

impl<'a> InheritedPseudoElementMatchesBuilder<'a> {
    /// Matches of pseudo styles from the pseudos of an ancestor node.
    pub fn pseudoElements(mut self, pseudoElements: Vec<PseudoElementMatches<'a>>) -> Self { self.pseudoElements = Some(pseudoElements); self }
    pub fn build(self) -> InheritedPseudoElementMatches<'a> {
        InheritedPseudoElementMatches {
            pseudoElements: self.pseudoElements.unwrap_or_default(),
        }
    }
}

/// Match data for a CSS rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuleMatch<'a> {
    /// CSS rule in the match.
    rule: CSSRule<'a>,
    /// Matching selector indices in the rule's selectorList selectors (0-based).
    matchingSelectors: Vec<i64>,
}

impl<'a> RuleMatch<'a> {
    pub fn builder() -> RuleMatchBuilder<'a> { RuleMatchBuilder::default() }
    pub fn rule(&self) -> &CSSRule<'a> { &self.rule }
    pub fn matchingSelectors(&self) -> &[i64] { &self.matchingSelectors }
}

#[derive(Default)]
pub struct RuleMatchBuilder<'a> {
    rule: Option<CSSRule<'a>>,
    matchingSelectors: Option<Vec<i64>>,
}

impl<'a> RuleMatchBuilder<'a> {
    /// CSS rule in the match.
    pub fn rule(mut self, rule: CSSRule<'a>) -> Self { self.rule = Some(rule); self }
    /// Matching selector indices in the rule's selectorList selectors (0-based).
    pub fn matchingSelectors(mut self, matchingSelectors: Vec<i64>) -> Self { self.matchingSelectors = Some(matchingSelectors); self }
    pub fn build(self) -> RuleMatch<'a> {
        RuleMatch {
            rule: self.rule.unwrap_or_default(),
            matchingSelectors: self.matchingSelectors.unwrap_or_default(),
        }
    }
}

/// Data for a simple selector (these are delimited by commas in a selector list).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolValue<'a> {
    /// Value text.
    text: Cow<'a, str>,
    /// Value range in the underlying resource (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Specificity of the selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    specificity: Option<Specificity>,
}

impl<'a> ProtocolValue<'a> {
    pub fn builder() -> ProtocolValueBuilder<'a> { ProtocolValueBuilder::default() }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    pub fn specificity(&self) -> Option<&Specificity> { self.specificity.as_ref() }
}

#[derive(Default)]
pub struct ProtocolValueBuilder<'a> {
    text: Option<Cow<'a, str>>,
    range: Option<SourceRange>,
    specificity: Option<Specificity>,
}

impl<'a> ProtocolValueBuilder<'a> {
    /// Value text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// Value range in the underlying resource (if available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Specificity of the selector.
    pub fn specificity(mut self, specificity: Specificity) -> Self { self.specificity = Some(specificity); self }
    pub fn build(self) -> ProtocolValue<'a> {
        ProtocolValue {
            text: self.text.unwrap_or_default(),
            range: self.range,
            specificity: self.specificity,
        }
    }
}

/// Specificity:
/// https://drafts.csswg.org/selectors/#specificity-rules

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Specificity {
    /// The a component, which represents the number of ID selectors.
    a: i64,
    /// The b component, which represents the number of class selectors, attributes selectors, and
    /// pseudo-classes.
    b: i64,
    /// The c component, which represents the number of type selectors and pseudo-elements.
    c: i64,
}

impl Specificity {
    pub fn builder() -> SpecificityBuilder { SpecificityBuilder::default() }
    pub fn a(&self) -> i64 { self.a }
    pub fn b(&self) -> i64 { self.b }
    pub fn c(&self) -> i64 { self.c }
}

#[derive(Default)]
pub struct SpecificityBuilder {
    a: Option<i64>,
    b: Option<i64>,
    c: Option<i64>,
}

impl SpecificityBuilder {
    /// The a component, which represents the number of ID selectors.
    pub fn a(mut self, a: i64) -> Self { self.a = Some(a); self }
    /// The b component, which represents the number of class selectors, attributes selectors, and
    /// pseudo-classes.
    pub fn b(mut self, b: i64) -> Self { self.b = Some(b); self }
    /// The c component, which represents the number of type selectors and pseudo-elements.
    pub fn c(mut self, c: i64) -> Self { self.c = Some(c); self }
    pub fn build(self) -> Specificity {
        Specificity {
            a: self.a.unwrap_or_default(),
            b: self.b.unwrap_or_default(),
            c: self.c.unwrap_or_default(),
        }
    }
}

/// Selector list data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectorList<'a> {
    /// Selectors in the list.
    selectors: Vec<ProtocolValue<'a>>,
    /// Rule selector text.
    text: Cow<'a, str>,
}

impl<'a> SelectorList<'a> {
    pub fn builder() -> SelectorListBuilder<'a> { SelectorListBuilder::default() }
    pub fn selectors(&self) -> &[ProtocolValue<'a>] { &self.selectors }
    pub fn text(&self) -> &str { self.text.as_ref() }
}

#[derive(Default)]
pub struct SelectorListBuilder<'a> {
    selectors: Option<Vec<ProtocolValue<'a>>>,
    text: Option<Cow<'a, str>>,
}

impl<'a> SelectorListBuilder<'a> {
    /// Selectors in the list.
    pub fn selectors(mut self, selectors: Vec<ProtocolValue<'a>>) -> Self { self.selectors = Some(selectors); self }
    /// Rule selector text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> SelectorList<'a> {
        SelectorList {
            selectors: self.selectors.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
        }
    }
}

/// CSS stylesheet metainformation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSStyleSheetHeader<'a> {
    /// The stylesheet identifier.
    styleSheetId: crate::dom::StyleSheetId<'a>,
    /// Owner frame identifier.
    frameId: crate::page::FrameId<'a>,
    /// Stylesheet resource URL. Empty if this is a constructed stylesheet created using
    /// new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported
    /// as a CSS module script).
    sourceURL: Cow<'a, str>,
    /// URL of source map associated with the stylesheet (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceMapURL: Option<Cow<'a, str>>,
    /// Stylesheet origin.
    origin: StyleSheetOrigin,
    /// Stylesheet title.
    title: Cow<'a, str>,
    /// The backend id for the owner node of the stylesheet.
    #[serde(skip_serializing_if = "Option::is_none")]
    ownerNode: Option<crate::dom::BackendNodeId>,
    /// Denotes whether the stylesheet is disabled.
    disabled: bool,
    /// Whether the sourceURL field value comes from the sourceURL comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasSourceURL: Option<bool>,
    /// Whether this stylesheet is created for STYLE tag by parser. This flag is not set for
    /// document.written STYLE tags.
    isInline: bool,
    /// Whether this stylesheet is mutable. Inline stylesheets become mutable
    /// after they have been modified via CSSOM API.
    /// '<link>' element's stylesheets become mutable only if DevTools modifies them.
    /// Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.
    isMutable: bool,
    /// True if this stylesheet is created through new CSSStyleSheet() or imported as a
    /// CSS module script.
    isConstructed: bool,
    /// Line offset of the stylesheet within the resource (zero based).
    startLine: f64,
    /// Column offset of the stylesheet within the resource (zero based).
    startColumn: f64,
    /// Size of the content (in characters).
    length: f64,
    /// Line offset of the end of the stylesheet within the resource (zero based).
    endLine: f64,
    /// Column offset of the end of the stylesheet within the resource (zero based).
    endColumn: f64,
    /// If the style sheet was loaded from a network resource, this indicates when the resource failed to load
    #[serde(skip_serializing_if = "Option::is_none")]
    loadingFailed: Option<bool>,
}

impl<'a> CSSStyleSheetHeader<'a> {
    pub fn builder() -> CSSStyleSheetHeaderBuilder<'a> { CSSStyleSheetHeaderBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
    pub fn sourceURL(&self) -> &str { self.sourceURL.as_ref() }
    pub fn sourceMapURL(&self) -> Option<&str> { self.sourceMapURL.as_deref() }
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    pub fn title(&self) -> &str { self.title.as_ref() }
    pub fn ownerNode(&self) -> Option<&crate::dom::BackendNodeId> { self.ownerNode.as_ref() }
    pub fn disabled(&self) -> bool { self.disabled }
    pub fn hasSourceURL(&self) -> Option<bool> { self.hasSourceURL }
    pub fn isInline(&self) -> bool { self.isInline }
    pub fn isMutable(&self) -> bool { self.isMutable }
    pub fn isConstructed(&self) -> bool { self.isConstructed }
    pub fn startLine(&self) -> f64 { self.startLine }
    pub fn startColumn(&self) -> f64 { self.startColumn }
    pub fn length(&self) -> f64 { self.length }
    pub fn endLine(&self) -> f64 { self.endLine }
    pub fn endColumn(&self) -> f64 { self.endColumn }
    pub fn loadingFailed(&self) -> Option<bool> { self.loadingFailed }
}

#[derive(Default)]
pub struct CSSStyleSheetHeaderBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    frameId: Option<crate::page::FrameId<'a>>,
    sourceURL: Option<Cow<'a, str>>,
    sourceMapURL: Option<Cow<'a, str>>,
    origin: Option<StyleSheetOrigin>,
    title: Option<Cow<'a, str>>,
    ownerNode: Option<crate::dom::BackendNodeId>,
    disabled: Option<bool>,
    hasSourceURL: Option<bool>,
    isInline: Option<bool>,
    isMutable: Option<bool>,
    isConstructed: Option<bool>,
    startLine: Option<f64>,
    startColumn: Option<f64>,
    length: Option<f64>,
    endLine: Option<f64>,
    endColumn: Option<f64>,
    loadingFailed: Option<bool>,
}

impl<'a> CSSStyleSheetHeaderBuilder<'a> {
    /// The stylesheet identifier.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Owner frame identifier.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// Stylesheet resource URL. Empty if this is a constructed stylesheet created using
    /// new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported
    /// as a CSS module script).
    pub fn sourceURL(mut self, sourceURL: impl Into<Cow<'a, str>>) -> Self { self.sourceURL = Some(sourceURL.into()); self }
    /// URL of source map associated with the stylesheet (if any).
    pub fn sourceMapURL(mut self, sourceMapURL: impl Into<Cow<'a, str>>) -> Self { self.sourceMapURL = Some(sourceMapURL.into()); self }
    /// Stylesheet origin.
    pub fn origin(mut self, origin: StyleSheetOrigin) -> Self { self.origin = Some(origin); self }
    /// Stylesheet title.
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self { self.title = Some(title.into()); self }
    /// The backend id for the owner node of the stylesheet.
    pub fn ownerNode(mut self, ownerNode: crate::dom::BackendNodeId) -> Self { self.ownerNode = Some(ownerNode); self }
    /// Denotes whether the stylesheet is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = Some(disabled); self }
    /// Whether the sourceURL field value comes from the sourceURL comment.
    pub fn hasSourceURL(mut self, hasSourceURL: bool) -> Self { self.hasSourceURL = Some(hasSourceURL); self }
    /// Whether this stylesheet is created for STYLE tag by parser. This flag is not set for
    /// document.written STYLE tags.
    pub fn isInline(mut self, isInline: bool) -> Self { self.isInline = Some(isInline); self }
    /// Whether this stylesheet is mutable. Inline stylesheets become mutable
    /// after they have been modified via CSSOM API.
    /// '<link>' element's stylesheets become mutable only if DevTools modifies them.
    /// Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.
    pub fn isMutable(mut self, isMutable: bool) -> Self { self.isMutable = Some(isMutable); self }
    /// True if this stylesheet is created through new CSSStyleSheet() or imported as a
    /// CSS module script.
    pub fn isConstructed(mut self, isConstructed: bool) -> Self { self.isConstructed = Some(isConstructed); self }
    /// Line offset of the stylesheet within the resource (zero based).
    pub fn startLine(mut self, startLine: f64) -> Self { self.startLine = Some(startLine); self }
    /// Column offset of the stylesheet within the resource (zero based).
    pub fn startColumn(mut self, startColumn: f64) -> Self { self.startColumn = Some(startColumn); self }
    /// Size of the content (in characters).
    pub fn length(mut self, length: f64) -> Self { self.length = Some(length); self }
    /// Line offset of the end of the stylesheet within the resource (zero based).
    pub fn endLine(mut self, endLine: f64) -> Self { self.endLine = Some(endLine); self }
    /// Column offset of the end of the stylesheet within the resource (zero based).
    pub fn endColumn(mut self, endColumn: f64) -> Self { self.endColumn = Some(endColumn); self }
    /// If the style sheet was loaded from a network resource, this indicates when the resource failed to load
    pub fn loadingFailed(mut self, loadingFailed: bool) -> Self { self.loadingFailed = Some(loadingFailed); self }
    pub fn build(self) -> CSSStyleSheetHeader<'a> {
        CSSStyleSheetHeader {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            frameId: self.frameId.unwrap_or_default(),
            sourceURL: self.sourceURL.unwrap_or_default(),
            sourceMapURL: self.sourceMapURL,
            origin: self.origin.unwrap_or_default(),
            title: self.title.unwrap_or_default(),
            ownerNode: self.ownerNode,
            disabled: self.disabled.unwrap_or_default(),
            hasSourceURL: self.hasSourceURL,
            isInline: self.isInline.unwrap_or_default(),
            isMutable: self.isMutable.unwrap_or_default(),
            isConstructed: self.isConstructed.unwrap_or_default(),
            startLine: self.startLine.unwrap_or_default(),
            startColumn: self.startColumn.unwrap_or_default(),
            length: self.length.unwrap_or_default(),
            endLine: self.endLine.unwrap_or_default(),
            endColumn: self.endColumn.unwrap_or_default(),
            loadingFailed: self.loadingFailed,
        }
    }
}

/// CSS rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// Rule selector data.
    selectorList: SelectorList<'a>,
    /// Array of selectors from ancestor style rules, sorted by distance from the current rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    nestingSelectors: Option<Vec<Cow<'a, str>>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated style declaration.
    style: CSSStyle<'a>,
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    originTreeScopeNodeId: Option<crate::dom::BackendNodeId>,
    /// Media list array (for rules involving media queries). The array enumerates media queries
    /// starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    media: Option<Vec<CSSMedia<'a>>>,
    /// Container query list array (for rules involving container queries).
    /// The array enumerates container queries starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    containerQueries: Option<Vec<CSSContainerQuery<'a>>>,
    /// @supports CSS at-rule array.
    /// The array enumerates @supports at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    supports: Option<Vec<CSSSupports<'a>>>,
    /// Cascade layer array. Contains the layer hierarchy that this rule belongs to starting
    /// with the innermost layer and going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    layers: Option<Vec<CSSLayer<'a>>>,
    /// @scope CSS at-rule array.
    /// The array enumerates @scope at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<Vec<CSSScope<'a>>>,
    /// The array keeps the types of ancestor CSSRules from the innermost going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    ruleTypes: Option<Vec<CSSRuleType>>,
    /// @starting-style CSS at-rule array.
    /// The array enumerates @starting-style at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    startingStyles: Option<Vec<CSSStartingStyle<'a>>>,
    /// @navigation CSS at-rule array.
    /// The array enumerates @navigation at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    navigations: Option<Vec<CSSNavigation<'a>>>,
}

impl<'a> CSSRule<'a> {
    pub fn builder() -> CSSRuleBuilder<'a> { CSSRuleBuilder::default() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn selectorList(&self) -> &SelectorList<'a> { &self.selectorList }
    pub fn nestingSelectors(&self) -> Option<&[Cow<'a, str>]> { self.nestingSelectors.as_deref() }
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
    pub fn originTreeScopeNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.originTreeScopeNodeId.as_ref() }
    pub fn media(&self) -> Option<&[CSSMedia<'a>]> { self.media.as_deref() }
    pub fn containerQueries(&self) -> Option<&[CSSContainerQuery<'a>]> { self.containerQueries.as_deref() }
    pub fn supports(&self) -> Option<&[CSSSupports<'a>]> { self.supports.as_deref() }
    pub fn layers(&self) -> Option<&[CSSLayer<'a>]> { self.layers.as_deref() }
    pub fn scopes(&self) -> Option<&[CSSScope<'a>]> { self.scopes.as_deref() }
    pub fn ruleTypes(&self) -> Option<&[CSSRuleType]> { self.ruleTypes.as_deref() }
    pub fn startingStyles(&self) -> Option<&[CSSStartingStyle<'a>]> { self.startingStyles.as_deref() }
    pub fn navigations(&self) -> Option<&[CSSNavigation<'a>]> { self.navigations.as_deref() }
}

#[derive(Default)]
pub struct CSSRuleBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    selectorList: Option<SelectorList<'a>>,
    nestingSelectors: Option<Vec<Cow<'a, str>>>,
    origin: Option<StyleSheetOrigin>,
    style: Option<CSSStyle<'a>>,
    originTreeScopeNodeId: Option<crate::dom::BackendNodeId>,
    media: Option<Vec<CSSMedia<'a>>>,
    containerQueries: Option<Vec<CSSContainerQuery<'a>>>,
    supports: Option<Vec<CSSSupports<'a>>>,
    layers: Option<Vec<CSSLayer<'a>>>,
    scopes: Option<Vec<CSSScope<'a>>>,
    ruleTypes: Option<Vec<CSSRuleType>>,
    startingStyles: Option<Vec<CSSStartingStyle<'a>>>,
    navigations: Option<Vec<CSSNavigation<'a>>>,
}

impl<'a> CSSRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Rule selector data.
    pub fn selectorList(mut self, selectorList: SelectorList<'a>) -> Self { self.selectorList = Some(selectorList); self }
    /// Array of selectors from ancestor style rules, sorted by distance from the current rule.
    pub fn nestingSelectors(mut self, nestingSelectors: Vec<Cow<'a, str>>) -> Self { self.nestingSelectors = Some(nestingSelectors); self }
    /// Parent stylesheet's origin.
    pub fn origin(mut self, origin: StyleSheetOrigin) -> Self { self.origin = Some(origin); self }
    /// Associated style declaration.
    pub fn style(mut self, style: CSSStyle<'a>) -> Self { self.style = Some(style); self }
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    pub fn originTreeScopeNodeId(mut self, originTreeScopeNodeId: crate::dom::BackendNodeId) -> Self { self.originTreeScopeNodeId = Some(originTreeScopeNodeId); self }
    /// Media list array (for rules involving media queries). The array enumerates media queries
    /// starting with the innermost one, going outwards.
    pub fn media(mut self, media: Vec<CSSMedia<'a>>) -> Self { self.media = Some(media); self }
    /// Container query list array (for rules involving container queries).
    /// The array enumerates container queries starting with the innermost one, going outwards.
    pub fn containerQueries(mut self, containerQueries: Vec<CSSContainerQuery<'a>>) -> Self { self.containerQueries = Some(containerQueries); self }
    /// @supports CSS at-rule array.
    /// The array enumerates @supports at-rules starting with the innermost one, going outwards.
    pub fn supports(mut self, supports: Vec<CSSSupports<'a>>) -> Self { self.supports = Some(supports); self }
    /// Cascade layer array. Contains the layer hierarchy that this rule belongs to starting
    /// with the innermost layer and going outwards.
    pub fn layers(mut self, layers: Vec<CSSLayer<'a>>) -> Self { self.layers = Some(layers); self }
    /// @scope CSS at-rule array.
    /// The array enumerates @scope at-rules starting with the innermost one, going outwards.
    pub fn scopes(mut self, scopes: Vec<CSSScope<'a>>) -> Self { self.scopes = Some(scopes); self }
    /// The array keeps the types of ancestor CSSRules from the innermost going outwards.
    pub fn ruleTypes(mut self, ruleTypes: Vec<CSSRuleType>) -> Self { self.ruleTypes = Some(ruleTypes); self }
    /// @starting-style CSS at-rule array.
    /// The array enumerates @starting-style at-rules starting with the innermost one, going outwards.
    pub fn startingStyles(mut self, startingStyles: Vec<CSSStartingStyle<'a>>) -> Self { self.startingStyles = Some(startingStyles); self }
    /// @navigation CSS at-rule array.
    /// The array enumerates @navigation at-rules starting with the innermost one, going outwards.
    pub fn navigations(mut self, navigations: Vec<CSSNavigation<'a>>) -> Self { self.navigations = Some(navigations); self }
    pub fn build(self) -> CSSRule<'a> {
        CSSRule {
            styleSheetId: self.styleSheetId,
            selectorList: self.selectorList.unwrap_or_default(),
            nestingSelectors: self.nestingSelectors,
            origin: self.origin.unwrap_or_default(),
            style: self.style.unwrap_or_default(),
            originTreeScopeNodeId: self.originTreeScopeNodeId,
            media: self.media,
            containerQueries: self.containerQueries,
            supports: self.supports,
            layers: self.layers,
            scopes: self.scopes,
            ruleTypes: self.ruleTypes,
            startingStyles: self.startingStyles,
            navigations: self.navigations,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuleUsage<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    styleSheetId: crate::dom::StyleSheetId<'a>,
    /// Offset of the start of the rule (including selector) from the beginning of the stylesheet.
    startOffset: f64,
    /// Offset of the end of the rule body from the beginning of the stylesheet.
    endOffset: f64,
    /// Indicates whether the rule was actually used by some element in the page.
    used: bool,
}

impl<'a> RuleUsage<'a> {
    pub fn builder() -> RuleUsageBuilder<'a> { RuleUsageBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn startOffset(&self) -> f64 { self.startOffset }
    pub fn endOffset(&self) -> f64 { self.endOffset }
    pub fn used(&self) -> bool { self.used }
}

#[derive(Default)]
pub struct RuleUsageBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    startOffset: Option<f64>,
    endOffset: Option<f64>,
    used: Option<bool>,
}

impl<'a> RuleUsageBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Offset of the start of the rule (including selector) from the beginning of the stylesheet.
    pub fn startOffset(mut self, startOffset: f64) -> Self { self.startOffset = Some(startOffset); self }
    /// Offset of the end of the rule body from the beginning of the stylesheet.
    pub fn endOffset(mut self, endOffset: f64) -> Self { self.endOffset = Some(endOffset); self }
    /// Indicates whether the rule was actually used by some element in the page.
    pub fn used(mut self, used: bool) -> Self { self.used = Some(used); self }
    pub fn build(self) -> RuleUsage<'a> {
        RuleUsage {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            startOffset: self.startOffset.unwrap_or_default(),
            endOffset: self.endOffset.unwrap_or_default(),
            used: self.used.unwrap_or_default(),
        }
    }
}

/// Text range within a resource. All numbers are zero-based.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceRange {
    /// Start line of range.
    startLine: i64,
    /// Start column of range (inclusive).
    startColumn: i64,
    /// End line of range
    endLine: i64,
    /// End column of range (exclusive).
    endColumn: i64,
}

impl SourceRange {
    pub fn builder() -> SourceRangeBuilder { SourceRangeBuilder::default() }
    pub fn startLine(&self) -> i64 { self.startLine }
    pub fn startColumn(&self) -> i64 { self.startColumn }
    pub fn endLine(&self) -> i64 { self.endLine }
    pub fn endColumn(&self) -> i64 { self.endColumn }
}

#[derive(Default)]
pub struct SourceRangeBuilder {
    startLine: Option<i64>,
    startColumn: Option<i64>,
    endLine: Option<i64>,
    endColumn: Option<i64>,
}

impl SourceRangeBuilder {
    /// Start line of range.
    pub fn startLine(mut self, startLine: i64) -> Self { self.startLine = Some(startLine); self }
    /// Start column of range (inclusive).
    pub fn startColumn(mut self, startColumn: i64) -> Self { self.startColumn = Some(startColumn); self }
    /// End line of range
    pub fn endLine(mut self, endLine: i64) -> Self { self.endLine = Some(endLine); self }
    /// End column of range (exclusive).
    pub fn endColumn(mut self, endColumn: i64) -> Self { self.endColumn = Some(endColumn); self }
    pub fn build(self) -> SourceRange {
        SourceRange {
            startLine: self.startLine.unwrap_or_default(),
            startColumn: self.startColumn.unwrap_or_default(),
            endLine: self.endLine.unwrap_or_default(),
            endColumn: self.endColumn.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShorthandEntry<'a> {
    /// Shorthand name.
    name: Cow<'a, str>,
    /// Shorthand value.
    value: Cow<'a, str>,
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    important: Option<bool>,
}

impl<'a> ShorthandEntry<'a> {
    pub fn builder() -> ShorthandEntryBuilder<'a> { ShorthandEntryBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn important(&self) -> Option<bool> { self.important }
}

#[derive(Default)]
pub struct ShorthandEntryBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    important: Option<bool>,
}

impl<'a> ShorthandEntryBuilder<'a> {
    /// Shorthand name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Shorthand value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    pub fn important(mut self, important: bool) -> Self { self.important = Some(important); self }
    pub fn build(self) -> ShorthandEntry<'a> {
        ShorthandEntry {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            important: self.important,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSComputedStyleProperty<'a> {
    /// Computed style property name.
    name: Cow<'a, str>,
    /// Computed style property value.
    value: Cow<'a, str>,
}

impl<'a> CSSComputedStyleProperty<'a> {
    pub fn builder() -> CSSComputedStylePropertyBuilder<'a> { CSSComputedStylePropertyBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct CSSComputedStylePropertyBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> CSSComputedStylePropertyBuilder<'a> {
    /// Computed style property name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Computed style property value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> CSSComputedStyleProperty<'a> {
        CSSComputedStyleProperty {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComputedStyleExtraFields {
    /// Returns whether or not this node is being rendered with base appearance,
    /// which happens when it has its appearance property set to base/base-select
    /// or it is in the subtree of an element being rendered with base appearance.
    isAppearanceBase: bool,
}

impl ComputedStyleExtraFields {
    pub fn builder() -> ComputedStyleExtraFieldsBuilder { ComputedStyleExtraFieldsBuilder::default() }
    pub fn isAppearanceBase(&self) -> bool { self.isAppearanceBase }
}

#[derive(Default)]
pub struct ComputedStyleExtraFieldsBuilder {
    isAppearanceBase: Option<bool>,
}

impl ComputedStyleExtraFieldsBuilder {
    /// Returns whether or not this node is being rendered with base appearance,
    /// which happens when it has its appearance property set to base/base-select
    /// or it is in the subtree of an element being rendered with base appearance.
    pub fn isAppearanceBase(mut self, isAppearanceBase: bool) -> Self { self.isAppearanceBase = Some(isAppearanceBase); self }
    pub fn build(self) -> ComputedStyleExtraFields {
        ComputedStyleExtraFields {
            isAppearanceBase: self.isAppearanceBase.unwrap_or_default(),
        }
    }
}

/// CSS style representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSStyle<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// CSS properties in the style.
    cssProperties: Vec<CSSProperty<'a>>,
    /// Computed values for all shorthands found in the style.
    shorthandEntries: Vec<ShorthandEntry<'a>>,
    /// Style declaration text (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    cssText: Option<Cow<'a, str>>,
    /// Style declaration range in the enclosing stylesheet (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
}

impl<'a> CSSStyle<'a> {
    pub fn builder() -> CSSStyleBuilder<'a> { CSSStyleBuilder::default() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn cssProperties(&self) -> &[CSSProperty<'a>] { &self.cssProperties }
    pub fn shorthandEntries(&self) -> &[ShorthandEntry<'a>] { &self.shorthandEntries }
    pub fn cssText(&self) -> Option<&str> { self.cssText.as_deref() }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
}

#[derive(Default)]
pub struct CSSStyleBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    cssProperties: Option<Vec<CSSProperty<'a>>>,
    shorthandEntries: Option<Vec<ShorthandEntry<'a>>>,
    cssText: Option<Cow<'a, str>>,
    range: Option<SourceRange>,
}

impl<'a> CSSStyleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// CSS properties in the style.
    pub fn cssProperties(mut self, cssProperties: Vec<CSSProperty<'a>>) -> Self { self.cssProperties = Some(cssProperties); self }
    /// Computed values for all shorthands found in the style.
    pub fn shorthandEntries(mut self, shorthandEntries: Vec<ShorthandEntry<'a>>) -> Self { self.shorthandEntries = Some(shorthandEntries); self }
    /// Style declaration text (if available).
    pub fn cssText(mut self, cssText: impl Into<Cow<'a, str>>) -> Self { self.cssText = Some(cssText.into()); self }
    /// Style declaration range in the enclosing stylesheet (if available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn build(self) -> CSSStyle<'a> {
        CSSStyle {
            styleSheetId: self.styleSheetId,
            cssProperties: self.cssProperties.unwrap_or_default(),
            shorthandEntries: self.shorthandEntries.unwrap_or_default(),
            cssText: self.cssText,
            range: self.range,
        }
    }
}

/// CSS property declaration data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSProperty<'a> {
    /// The property name.
    name: Cow<'a, str>,
    /// The property value.
    value: Cow<'a, str>,
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    important: Option<bool>,
    /// Whether the property is implicit (implies 'false' if absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    implicit: Option<bool>,
    /// The full property text as specified in the style.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Cow<'a, str>>,
    /// Whether the property is understood by the browser (implies 'true' if absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    parsedOk: Option<bool>,
    /// Whether the property is disabled by the user (present for source-based properties only).
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// The entire property range in the enclosing style declaration (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Parsed longhand components of this property if it is a shorthand.
    /// This field will be empty if the given property is not a shorthand.
    #[serde(skip_serializing_if = "Option::is_none")]
    longhandProperties: Option<Vec<Box<CSSProperty<'a>>>>,
}

impl<'a> CSSProperty<'a> {
    pub fn builder() -> CSSPropertyBuilder<'a> { CSSPropertyBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn important(&self) -> Option<bool> { self.important }
    pub fn implicit(&self) -> Option<bool> { self.implicit }
    pub fn text(&self) -> Option<&str> { self.text.as_deref() }
    pub fn parsedOk(&self) -> Option<bool> { self.parsedOk }
    pub fn disabled(&self) -> Option<bool> { self.disabled }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    pub fn longhandProperties(&self) -> Option<&[Box<CSSProperty<'a>>]> { self.longhandProperties.as_deref() }
}

#[derive(Default)]
pub struct CSSPropertyBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    important: Option<bool>,
    implicit: Option<bool>,
    text: Option<Cow<'a, str>>,
    parsedOk: Option<bool>,
    disabled: Option<bool>,
    range: Option<SourceRange>,
    longhandProperties: Option<Vec<Box<CSSProperty<'a>>>>,
}

impl<'a> CSSPropertyBuilder<'a> {
    /// The property name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// The property value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    pub fn important(mut self, important: bool) -> Self { self.important = Some(important); self }
    /// Whether the property is implicit (implies 'false' if absent).
    pub fn implicit(mut self, implicit: bool) -> Self { self.implicit = Some(implicit); self }
    /// The full property text as specified in the style.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// Whether the property is understood by the browser (implies 'true' if absent).
    pub fn parsedOk(mut self, parsedOk: bool) -> Self { self.parsedOk = Some(parsedOk); self }
    /// Whether the property is disabled by the user (present for source-based properties only).
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = Some(disabled); self }
    /// The entire property range in the enclosing style declaration (if available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Parsed longhand components of this property if it is a shorthand.
    /// This field will be empty if the given property is not a shorthand.
    pub fn longhandProperties(mut self, longhandProperties: Vec<Box<CSSProperty<'a>>>) -> Self { self.longhandProperties = Some(longhandProperties); self }
    pub fn build(self) -> CSSProperty<'a> {
        CSSProperty {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            important: self.important,
            implicit: self.implicit,
            text: self.text,
            parsedOk: self.parsedOk,
            disabled: self.disabled,
            range: self.range,
            longhandProperties: self.longhandProperties,
        }
    }
}

/// CSS media rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSMedia<'a> {
    /// Media query text.
    text: Cow<'a, str>,
    /// Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if
    /// specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked
    /// stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline
    /// stylesheet's STYLE tag.
    source: Cow<'a, str>,
    /// URL of the document containing the media query description.
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceURL: Option<Cow<'a, str>>,
    /// The associated rule (@media or @import) header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// Array of media queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    mediaList: Option<Vec<MediaQuery<'a>>>,
}

impl<'a> CSSMedia<'a> {
    pub fn builder() -> CSSMediaBuilder<'a> { CSSMediaBuilder::default() }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn source(&self) -> &str { self.source.as_ref() }
    pub fn sourceURL(&self) -> Option<&str> { self.sourceURL.as_deref() }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn mediaList(&self) -> Option<&[MediaQuery<'a>]> { self.mediaList.as_deref() }
}

#[derive(Default)]
pub struct CSSMediaBuilder<'a> {
    text: Option<Cow<'a, str>>,
    source: Option<Cow<'a, str>>,
    sourceURL: Option<Cow<'a, str>>,
    range: Option<SourceRange>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    mediaList: Option<Vec<MediaQuery<'a>>>,
}

impl<'a> CSSMediaBuilder<'a> {
    /// Media query text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if
    /// specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked
    /// stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline
    /// stylesheet's STYLE tag.
    pub fn source(mut self, source: impl Into<Cow<'a, str>>) -> Self { self.source = Some(source.into()); self }
    /// URL of the document containing the media query description.
    pub fn sourceURL(mut self, sourceURL: impl Into<Cow<'a, str>>) -> Self { self.sourceURL = Some(sourceURL.into()); self }
    /// The associated rule (@media or @import) header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Array of media queries.
    pub fn mediaList(mut self, mediaList: Vec<MediaQuery<'a>>) -> Self { self.mediaList = Some(mediaList); self }
    pub fn build(self) -> CSSMedia<'a> {
        CSSMedia {
            text: self.text.unwrap_or_default(),
            source: self.source.unwrap_or_default(),
            sourceURL: self.sourceURL,
            range: self.range,
            styleSheetId: self.styleSheetId,
            mediaList: self.mediaList,
        }
    }
}

/// Media query descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaQuery<'a> {
    /// Array of media query expressions.
    expressions: Vec<MediaQueryExpression<'a>>,
    /// Whether the media query condition is satisfied.
    active: bool,
}

impl<'a> MediaQuery<'a> {
    pub fn builder() -> MediaQueryBuilder<'a> { MediaQueryBuilder::default() }
    pub fn expressions(&self) -> &[MediaQueryExpression<'a>] { &self.expressions }
    pub fn active(&self) -> bool { self.active }
}

#[derive(Default)]
pub struct MediaQueryBuilder<'a> {
    expressions: Option<Vec<MediaQueryExpression<'a>>>,
    active: Option<bool>,
}

impl<'a> MediaQueryBuilder<'a> {
    /// Array of media query expressions.
    pub fn expressions(mut self, expressions: Vec<MediaQueryExpression<'a>>) -> Self { self.expressions = Some(expressions); self }
    /// Whether the media query condition is satisfied.
    pub fn active(mut self, active: bool) -> Self { self.active = Some(active); self }
    pub fn build(self) -> MediaQuery<'a> {
        MediaQuery {
            expressions: self.expressions.unwrap_or_default(),
            active: self.active.unwrap_or_default(),
        }
    }
}

/// Media query expression descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaQueryExpression<'a> {
    /// Media query expression value.
    value: f64,
    /// Media query expression units.
    unit: Cow<'a, str>,
    /// Media query expression feature.
    feature: Cow<'a, str>,
    /// The associated range of the value text in the enclosing stylesheet (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    valueRange: Option<SourceRange>,
    /// Computed length of media query expression (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    computedLength: Option<f64>,
}

impl<'a> MediaQueryExpression<'a> {
    pub fn builder() -> MediaQueryExpressionBuilder<'a> { MediaQueryExpressionBuilder::default() }
    pub fn value(&self) -> f64 { self.value }
    pub fn unit(&self) -> &str { self.unit.as_ref() }
    pub fn feature(&self) -> &str { self.feature.as_ref() }
    pub fn valueRange(&self) -> Option<&SourceRange> { self.valueRange.as_ref() }
    pub fn computedLength(&self) -> Option<f64> { self.computedLength }
}

#[derive(Default)]
pub struct MediaQueryExpressionBuilder<'a> {
    value: Option<f64>,
    unit: Option<Cow<'a, str>>,
    feature: Option<Cow<'a, str>>,
    valueRange: Option<SourceRange>,
    computedLength: Option<f64>,
}

impl<'a> MediaQueryExpressionBuilder<'a> {
    /// Media query expression value.
    pub fn value(mut self, value: f64) -> Self { self.value = Some(value); self }
    /// Media query expression units.
    pub fn unit(mut self, unit: impl Into<Cow<'a, str>>) -> Self { self.unit = Some(unit.into()); self }
    /// Media query expression feature.
    pub fn feature(mut self, feature: impl Into<Cow<'a, str>>) -> Self { self.feature = Some(feature.into()); self }
    /// The associated range of the value text in the enclosing stylesheet (if available).
    pub fn valueRange(mut self, valueRange: SourceRange) -> Self { self.valueRange = Some(valueRange); self }
    /// Computed length of media query expression (if applicable).
    pub fn computedLength(mut self, computedLength: f64) -> Self { self.computedLength = Some(computedLength); self }
    pub fn build(self) -> MediaQueryExpression<'a> {
        MediaQueryExpression {
            value: self.value.unwrap_or_default(),
            unit: self.unit.unwrap_or_default(),
            feature: self.feature.unwrap_or_default(),
            valueRange: self.valueRange,
            computedLength: self.computedLength,
        }
    }
}

/// CSS container query rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSContainerQuery<'a> {
    /// Container query text.
    text: Cow<'a, str>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// Optional name for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    /// Optional physical axes queried for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    physicalAxes: Option<crate::dom::PhysicalAxes>,
    /// Optional logical axes queried for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    logicalAxes: Option<crate::dom::LogicalAxes>,
    /// true if the query contains scroll-state() queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    queriesScrollState: Option<bool>,
    /// true if the query contains anchored() queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    queriesAnchored: Option<bool>,
}

impl<'a> CSSContainerQuery<'a> {
    pub fn builder() -> CSSContainerQueryBuilder<'a> { CSSContainerQueryBuilder::default() }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn physicalAxes(&self) -> Option<&crate::dom::PhysicalAxes> { self.physicalAxes.as_ref() }
    pub fn logicalAxes(&self) -> Option<&crate::dom::LogicalAxes> { self.logicalAxes.as_ref() }
    pub fn queriesScrollState(&self) -> Option<bool> { self.queriesScrollState }
    pub fn queriesAnchored(&self) -> Option<bool> { self.queriesAnchored }
}

#[derive(Default)]
pub struct CSSContainerQueryBuilder<'a> {
    text: Option<Cow<'a, str>>,
    range: Option<SourceRange>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    name: Option<Cow<'a, str>>,
    physicalAxes: Option<crate::dom::PhysicalAxes>,
    logicalAxes: Option<crate::dom::LogicalAxes>,
    queriesScrollState: Option<bool>,
    queriesAnchored: Option<bool>,
}

impl<'a> CSSContainerQueryBuilder<'a> {
    /// Container query text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Optional name for the container.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Optional physical axes queried for the container.
    pub fn physicalAxes(mut self, physicalAxes: crate::dom::PhysicalAxes) -> Self { self.physicalAxes = Some(physicalAxes); self }
    /// Optional logical axes queried for the container.
    pub fn logicalAxes(mut self, logicalAxes: crate::dom::LogicalAxes) -> Self { self.logicalAxes = Some(logicalAxes); self }
    /// true if the query contains scroll-state() queries.
    pub fn queriesScrollState(mut self, queriesScrollState: bool) -> Self { self.queriesScrollState = Some(queriesScrollState); self }
    /// true if the query contains anchored() queries.
    pub fn queriesAnchored(mut self, queriesAnchored: bool) -> Self { self.queriesAnchored = Some(queriesAnchored); self }
    pub fn build(self) -> CSSContainerQuery<'a> {
        CSSContainerQuery {
            text: self.text.unwrap_or_default(),
            range: self.range,
            styleSheetId: self.styleSheetId,
            name: self.name,
            physicalAxes: self.physicalAxes,
            logicalAxes: self.logicalAxes,
            queriesScrollState: self.queriesScrollState,
            queriesAnchored: self.queriesAnchored,
        }
    }
}

/// CSS Supports at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSSupports<'a> {
    /// Supports rule text.
    text: Cow<'a, str>,
    /// Whether the supports condition is satisfied.
    active: bool,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSSupports<'a> {
    pub fn builder() -> CSSSupportsBuilder<'a> { CSSSupportsBuilder::default() }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn active(&self) -> bool { self.active }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
}

#[derive(Default)]
pub struct CSSSupportsBuilder<'a> {
    text: Option<Cow<'a, str>>,
    active: Option<bool>,
    range: Option<SourceRange>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSSupportsBuilder<'a> {
    /// Supports rule text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// Whether the supports condition is satisfied.
    pub fn active(mut self, active: bool) -> Self { self.active = Some(active); self }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn build(self) -> CSSSupports<'a> {
        CSSSupports {
            text: self.text.unwrap_or_default(),
            active: self.active.unwrap_or_default(),
            range: self.range,
            styleSheetId: self.styleSheetId,
        }
    }
}

/// CSS Navigation at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSNavigation<'a> {
    /// Navigation rule text.
    text: Cow<'a, str>,
    /// Whether the navigation condition is satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSNavigation<'a> {
    pub fn builder() -> CSSNavigationBuilder<'a> { CSSNavigationBuilder::default() }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn active(&self) -> Option<bool> { self.active }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
}

#[derive(Default)]
pub struct CSSNavigationBuilder<'a> {
    text: Option<Cow<'a, str>>,
    active: Option<bool>,
    range: Option<SourceRange>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSNavigationBuilder<'a> {
    /// Navigation rule text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// Whether the navigation condition is satisfied.
    pub fn active(mut self, active: bool) -> Self { self.active = Some(active); self }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn build(self) -> CSSNavigation<'a> {
        CSSNavigation {
            text: self.text.unwrap_or_default(),
            active: self.active,
            range: self.range,
            styleSheetId: self.styleSheetId,
        }
    }
}

/// CSS Scope at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSScope<'a> {
    /// Scope rule text.
    text: Cow<'a, str>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSScope<'a> {
    pub fn builder() -> CSSScopeBuilder<'a> { CSSScopeBuilder::default() }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
}

#[derive(Default)]
pub struct CSSScopeBuilder<'a> {
    text: Option<Cow<'a, str>>,
    range: Option<SourceRange>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSScopeBuilder<'a> {
    /// Scope rule text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn build(self) -> CSSScope<'a> {
        CSSScope {
            text: self.text.unwrap_or_default(),
            range: self.range,
            styleSheetId: self.styleSheetId,
        }
    }
}

/// CSS Layer at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSLayer<'a> {
    /// Layer name.
    text: Cow<'a, str>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSLayer<'a> {
    pub fn builder() -> CSSLayerBuilder<'a> { CSSLayerBuilder::default() }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
}

#[derive(Default)]
pub struct CSSLayerBuilder<'a> {
    text: Option<Cow<'a, str>>,
    range: Option<SourceRange>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSLayerBuilder<'a> {
    /// Layer name.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn build(self) -> CSSLayer<'a> {
        CSSLayer {
            text: self.text.unwrap_or_default(),
            range: self.range,
            styleSheetId: self.styleSheetId,
        }
    }
}

/// CSS Starting Style at-rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSStartingStyle<'a> {
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSStartingStyle<'a> {
    pub fn builder() -> CSSStartingStyleBuilder<'a> { CSSStartingStyleBuilder::default() }
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
}

#[derive(Default)]
pub struct CSSStartingStyleBuilder<'a> {
    range: Option<SourceRange>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSStartingStyleBuilder<'a> {
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn build(self) -> CSSStartingStyle<'a> {
        CSSStartingStyle {
            range: self.range,
            styleSheetId: self.styleSheetId,
        }
    }
}

/// CSS Layer data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSLayerData<'a> {
    /// Layer name.
    name: Cow<'a, str>,
    /// Direct sub-layers
    #[serde(skip_serializing_if = "Option::is_none")]
    subLayers: Option<Vec<Box<CSSLayerData<'a>>>>,
    /// Layer order. The order determines the order of the layer in the cascade order.
    /// A higher number has higher priority in the cascade order.
    order: f64,
}

impl<'a> CSSLayerData<'a> {
    pub fn builder() -> CSSLayerDataBuilder<'a> { CSSLayerDataBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn subLayers(&self) -> Option<&[Box<CSSLayerData<'a>>]> { self.subLayers.as_deref() }
    pub fn order(&self) -> f64 { self.order }
}

#[derive(Default)]
pub struct CSSLayerDataBuilder<'a> {
    name: Option<Cow<'a, str>>,
    subLayers: Option<Vec<Box<CSSLayerData<'a>>>>,
    order: Option<f64>,
}

impl<'a> CSSLayerDataBuilder<'a> {
    /// Layer name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Direct sub-layers
    pub fn subLayers(mut self, subLayers: Vec<Box<CSSLayerData<'a>>>) -> Self { self.subLayers = Some(subLayers); self }
    /// Layer order. The order determines the order of the layer in the cascade order.
    /// A higher number has higher priority in the cascade order.
    pub fn order(mut self, order: f64) -> Self { self.order = Some(order); self }
    pub fn build(self) -> CSSLayerData<'a> {
        CSSLayerData {
            name: self.name.unwrap_or_default(),
            subLayers: self.subLayers,
            order: self.order.unwrap_or_default(),
        }
    }
}

/// Information about amount of glyphs that were rendered with given font.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlatformFontUsage<'a> {
    /// Font's family name reported by platform.
    familyName: Cow<'a, str>,
    /// Font's PostScript name reported by platform.
    postScriptName: Cow<'a, str>,
    /// Indicates if the font was downloaded or resolved locally.
    isCustomFont: bool,
    /// Amount of glyphs that were rendered with this font.
    glyphCount: f64,
}

impl<'a> PlatformFontUsage<'a> {
    pub fn builder() -> PlatformFontUsageBuilder<'a> { PlatformFontUsageBuilder::default() }
    pub fn familyName(&self) -> &str { self.familyName.as_ref() }
    pub fn postScriptName(&self) -> &str { self.postScriptName.as_ref() }
    pub fn isCustomFont(&self) -> bool { self.isCustomFont }
    pub fn glyphCount(&self) -> f64 { self.glyphCount }
}

#[derive(Default)]
pub struct PlatformFontUsageBuilder<'a> {
    familyName: Option<Cow<'a, str>>,
    postScriptName: Option<Cow<'a, str>>,
    isCustomFont: Option<bool>,
    glyphCount: Option<f64>,
}

impl<'a> PlatformFontUsageBuilder<'a> {
    /// Font's family name reported by platform.
    pub fn familyName(mut self, familyName: impl Into<Cow<'a, str>>) -> Self { self.familyName = Some(familyName.into()); self }
    /// Font's PostScript name reported by platform.
    pub fn postScriptName(mut self, postScriptName: impl Into<Cow<'a, str>>) -> Self { self.postScriptName = Some(postScriptName.into()); self }
    /// Indicates if the font was downloaded or resolved locally.
    pub fn isCustomFont(mut self, isCustomFont: bool) -> Self { self.isCustomFont = Some(isCustomFont); self }
    /// Amount of glyphs that were rendered with this font.
    pub fn glyphCount(mut self, glyphCount: f64) -> Self { self.glyphCount = Some(glyphCount); self }
    pub fn build(self) -> PlatformFontUsage<'a> {
        PlatformFontUsage {
            familyName: self.familyName.unwrap_or_default(),
            postScriptName: self.postScriptName.unwrap_or_default(),
            isCustomFont: self.isCustomFont.unwrap_or_default(),
            glyphCount: self.glyphCount.unwrap_or_default(),
        }
    }
}

/// Information about font variation axes for variable fonts

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FontVariationAxis<'a> {
    /// The font-variation-setting tag (a.k.a. "axis tag").
    tag: Cow<'a, str>,
    /// Human-readable variation name in the default language (normally, "en").
    name: Cow<'a, str>,
    /// The minimum value (inclusive) the font supports for this tag.
    minValue: f64,
    /// The maximum value (inclusive) the font supports for this tag.
    maxValue: f64,
    /// The default value.
    defaultValue: f64,
}

impl<'a> FontVariationAxis<'a> {
    pub fn builder() -> FontVariationAxisBuilder<'a> { FontVariationAxisBuilder::default() }
    pub fn tag(&self) -> &str { self.tag.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn minValue(&self) -> f64 { self.minValue }
    pub fn maxValue(&self) -> f64 { self.maxValue }
    pub fn defaultValue(&self) -> f64 { self.defaultValue }
}

#[derive(Default)]
pub struct FontVariationAxisBuilder<'a> {
    tag: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    minValue: Option<f64>,
    maxValue: Option<f64>,
    defaultValue: Option<f64>,
}

impl<'a> FontVariationAxisBuilder<'a> {
    /// The font-variation-setting tag (a.k.a. "axis tag").
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self { self.tag = Some(tag.into()); self }
    /// Human-readable variation name in the default language (normally, "en").
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// The minimum value (inclusive) the font supports for this tag.
    pub fn minValue(mut self, minValue: f64) -> Self { self.minValue = Some(minValue); self }
    /// The maximum value (inclusive) the font supports for this tag.
    pub fn maxValue(mut self, maxValue: f64) -> Self { self.maxValue = Some(maxValue); self }
    /// The default value.
    pub fn defaultValue(mut self, defaultValue: f64) -> Self { self.defaultValue = Some(defaultValue); self }
    pub fn build(self) -> FontVariationAxis<'a> {
        FontVariationAxis {
            tag: self.tag.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            minValue: self.minValue.unwrap_or_default(),
            maxValue: self.maxValue.unwrap_or_default(),
            defaultValue: self.defaultValue.unwrap_or_default(),
        }
    }
}

/// Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
/// and additional information such as platformFontFamily and fontVariationAxes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FontFace<'a> {
    /// The font-family.
    fontFamily: Cow<'a, str>,
    /// The font-style.
    fontStyle: Cow<'a, str>,
    /// The font-variant.
    fontVariant: Cow<'a, str>,
    /// The font-weight.
    fontWeight: Cow<'a, str>,
    /// The font-stretch.
    fontStretch: Cow<'a, str>,
    /// The font-display.
    fontDisplay: Cow<'a, str>,
    /// The unicode-range.
    unicodeRange: Cow<'a, str>,
    /// The src.
    src: Cow<'a, str>,
    /// The resolved platform font family
    platformFontFamily: Cow<'a, str>,
    /// Available variation settings (a.k.a. "axes").
    #[serde(skip_serializing_if = "Option::is_none")]
    fontVariationAxes: Option<Vec<FontVariationAxis<'a>>>,
}

impl<'a> FontFace<'a> {
    pub fn builder() -> FontFaceBuilder<'a> { FontFaceBuilder::default() }
    pub fn fontFamily(&self) -> &str { self.fontFamily.as_ref() }
    pub fn fontStyle(&self) -> &str { self.fontStyle.as_ref() }
    pub fn fontVariant(&self) -> &str { self.fontVariant.as_ref() }
    pub fn fontWeight(&self) -> &str { self.fontWeight.as_ref() }
    pub fn fontStretch(&self) -> &str { self.fontStretch.as_ref() }
    pub fn fontDisplay(&self) -> &str { self.fontDisplay.as_ref() }
    pub fn unicodeRange(&self) -> &str { self.unicodeRange.as_ref() }
    pub fn src(&self) -> &str { self.src.as_ref() }
    pub fn platformFontFamily(&self) -> &str { self.platformFontFamily.as_ref() }
    pub fn fontVariationAxes(&self) -> Option<&[FontVariationAxis<'a>]> { self.fontVariationAxes.as_deref() }
}

#[derive(Default)]
pub struct FontFaceBuilder<'a> {
    fontFamily: Option<Cow<'a, str>>,
    fontStyle: Option<Cow<'a, str>>,
    fontVariant: Option<Cow<'a, str>>,
    fontWeight: Option<Cow<'a, str>>,
    fontStretch: Option<Cow<'a, str>>,
    fontDisplay: Option<Cow<'a, str>>,
    unicodeRange: Option<Cow<'a, str>>,
    src: Option<Cow<'a, str>>,
    platformFontFamily: Option<Cow<'a, str>>,
    fontVariationAxes: Option<Vec<FontVariationAxis<'a>>>,
}

impl<'a> FontFaceBuilder<'a> {
    /// The font-family.
    pub fn fontFamily(mut self, fontFamily: impl Into<Cow<'a, str>>) -> Self { self.fontFamily = Some(fontFamily.into()); self }
    /// The font-style.
    pub fn fontStyle(mut self, fontStyle: impl Into<Cow<'a, str>>) -> Self { self.fontStyle = Some(fontStyle.into()); self }
    /// The font-variant.
    pub fn fontVariant(mut self, fontVariant: impl Into<Cow<'a, str>>) -> Self { self.fontVariant = Some(fontVariant.into()); self }
    /// The font-weight.
    pub fn fontWeight(mut self, fontWeight: impl Into<Cow<'a, str>>) -> Self { self.fontWeight = Some(fontWeight.into()); self }
    /// The font-stretch.
    pub fn fontStretch(mut self, fontStretch: impl Into<Cow<'a, str>>) -> Self { self.fontStretch = Some(fontStretch.into()); self }
    /// The font-display.
    pub fn fontDisplay(mut self, fontDisplay: impl Into<Cow<'a, str>>) -> Self { self.fontDisplay = Some(fontDisplay.into()); self }
    /// The unicode-range.
    pub fn unicodeRange(mut self, unicodeRange: impl Into<Cow<'a, str>>) -> Self { self.unicodeRange = Some(unicodeRange.into()); self }
    /// The src.
    pub fn src(mut self, src: impl Into<Cow<'a, str>>) -> Self { self.src = Some(src.into()); self }
    /// The resolved platform font family
    pub fn platformFontFamily(mut self, platformFontFamily: impl Into<Cow<'a, str>>) -> Self { self.platformFontFamily = Some(platformFontFamily.into()); self }
    /// Available variation settings (a.k.a. "axes").
    pub fn fontVariationAxes(mut self, fontVariationAxes: Vec<FontVariationAxis<'a>>) -> Self { self.fontVariationAxes = Some(fontVariationAxes); self }
    pub fn build(self) -> FontFace<'a> {
        FontFace {
            fontFamily: self.fontFamily.unwrap_or_default(),
            fontStyle: self.fontStyle.unwrap_or_default(),
            fontVariant: self.fontVariant.unwrap_or_default(),
            fontWeight: self.fontWeight.unwrap_or_default(),
            fontStretch: self.fontStretch.unwrap_or_default(),
            fontDisplay: self.fontDisplay.unwrap_or_default(),
            unicodeRange: self.unicodeRange.unwrap_or_default(),
            src: self.src.unwrap_or_default(),
            platformFontFamily: self.platformFontFamily.unwrap_or_default(),
            fontVariationAxes: self.fontVariationAxes,
        }
    }
}

/// CSS try rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSTryRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated style declaration.
    style: CSSStyle<'a>,
}

impl<'a> CSSTryRule<'a> {
    pub fn builder() -> CSSTryRuleBuilder<'a> { CSSTryRuleBuilder::default() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}

#[derive(Default)]
pub struct CSSTryRuleBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    origin: Option<StyleSheetOrigin>,
    style: Option<CSSStyle<'a>>,
}

impl<'a> CSSTryRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Parent stylesheet's origin.
    pub fn origin(mut self, origin: StyleSheetOrigin) -> Self { self.origin = Some(origin); self }
    /// Associated style declaration.
    pub fn style(mut self, style: CSSStyle<'a>) -> Self { self.style = Some(style); self }
    pub fn build(self) -> CSSTryRule<'a> {
        CSSTryRule {
            styleSheetId: self.styleSheetId,
            origin: self.origin.unwrap_or_default(),
            style: self.style.unwrap_or_default(),
        }
    }
}

/// CSS @position-try rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSPositionTryRule<'a> {
    /// The prelude dashed-ident name
    name: ProtocolValue<'a>,
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated style declaration.
    style: CSSStyle<'a>,
    active: bool,
}

impl<'a> CSSPositionTryRule<'a> {
    pub fn builder() -> CSSPositionTryRuleBuilder<'a> { CSSPositionTryRuleBuilder::default() }
    pub fn name(&self) -> &ProtocolValue<'a> { &self.name }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
    pub fn active(&self) -> bool { self.active }
}

#[derive(Default)]
pub struct CSSPositionTryRuleBuilder<'a> {
    name: Option<ProtocolValue<'a>>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    origin: Option<StyleSheetOrigin>,
    style: Option<CSSStyle<'a>>,
    active: Option<bool>,
}

impl<'a> CSSPositionTryRuleBuilder<'a> {
    /// The prelude dashed-ident name
    pub fn name(mut self, name: ProtocolValue<'a>) -> Self { self.name = Some(name); self }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Parent stylesheet's origin.
    pub fn origin(mut self, origin: StyleSheetOrigin) -> Self { self.origin = Some(origin); self }
    /// Associated style declaration.
    pub fn style(mut self, style: CSSStyle<'a>) -> Self { self.style = Some(style); self }
    pub fn active(mut self, active: bool) -> Self { self.active = Some(active); self }
    pub fn build(self) -> CSSPositionTryRule<'a> {
        CSSPositionTryRule {
            name: self.name.unwrap_or_default(),
            styleSheetId: self.styleSheetId,
            origin: self.origin.unwrap_or_default(),
            style: self.style.unwrap_or_default(),
            active: self.active.unwrap_or_default(),
        }
    }
}

/// CSS keyframes rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSKeyframesRule<'a> {
    /// Animation name.
    animationName: ProtocolValue<'a>,
    /// List of keyframes.
    keyframes: Vec<CSSKeyframeRule<'a>>,
}

impl<'a> CSSKeyframesRule<'a> {
    pub fn builder() -> CSSKeyframesRuleBuilder<'a> { CSSKeyframesRuleBuilder::default() }
    pub fn animationName(&self) -> &ProtocolValue<'a> { &self.animationName }
    pub fn keyframes(&self) -> &[CSSKeyframeRule<'a>] { &self.keyframes }
}

#[derive(Default)]
pub struct CSSKeyframesRuleBuilder<'a> {
    animationName: Option<ProtocolValue<'a>>,
    keyframes: Option<Vec<CSSKeyframeRule<'a>>>,
}

impl<'a> CSSKeyframesRuleBuilder<'a> {
    /// Animation name.
    pub fn animationName(mut self, animationName: ProtocolValue<'a>) -> Self { self.animationName = Some(animationName); self }
    /// List of keyframes.
    pub fn keyframes(mut self, keyframes: Vec<CSSKeyframeRule<'a>>) -> Self { self.keyframes = Some(keyframes); self }
    pub fn build(self) -> CSSKeyframesRule<'a> {
        CSSKeyframesRule {
            animationName: self.animationName.unwrap_or_default(),
            keyframes: self.keyframes.unwrap_or_default(),
        }
    }
}

/// Representation of a custom property registration through CSS.registerProperty

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSPropertyRegistration<'a> {
    propertyName: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initialValue: Option<ProtocolValue<'a>>,
    inherits: bool,
    syntax: Cow<'a, str>,
}

impl<'a> CSSPropertyRegistration<'a> {
    pub fn builder() -> CSSPropertyRegistrationBuilder<'a> { CSSPropertyRegistrationBuilder::default() }
    pub fn propertyName(&self) -> &str { self.propertyName.as_ref() }
    pub fn initialValue(&self) -> Option<&ProtocolValue<'a>> { self.initialValue.as_ref() }
    pub fn inherits(&self) -> bool { self.inherits }
    pub fn syntax(&self) -> &str { self.syntax.as_ref() }
}

#[derive(Default)]
pub struct CSSPropertyRegistrationBuilder<'a> {
    propertyName: Option<Cow<'a, str>>,
    initialValue: Option<ProtocolValue<'a>>,
    inherits: Option<bool>,
    syntax: Option<Cow<'a, str>>,
}

impl<'a> CSSPropertyRegistrationBuilder<'a> {
    pub fn propertyName(mut self, propertyName: impl Into<Cow<'a, str>>) -> Self { self.propertyName = Some(propertyName.into()); self }
    pub fn initialValue(mut self, initialValue: ProtocolValue<'a>) -> Self { self.initialValue = Some(initialValue); self }
    pub fn inherits(mut self, inherits: bool) -> Self { self.inherits = Some(inherits); self }
    pub fn syntax(mut self, syntax: impl Into<Cow<'a, str>>) -> Self { self.syntax = Some(syntax.into()); self }
    pub fn build(self) -> CSSPropertyRegistration<'a> {
        CSSPropertyRegistration {
            propertyName: self.propertyName.unwrap_or_default(),
            initialValue: self.initialValue,
            inherits: self.inherits.unwrap_or_default(),
            syntax: self.syntax.unwrap_or_default(),
        }
    }
}

/// CSS generic @rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSAtRule<'a> {
    /// Type of at-rule.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Subsection of font-feature-values, if this is a subsection.
    #[serde(skip_serializing_if = "Option::is_none")]
    subsection: Option<Cow<'a, str>>,
    /// LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)
    /// Associated name, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<ProtocolValue<'a>>,
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated style declaration.
    style: CSSStyle<'a>,
}

impl<'a> CSSAtRule<'a> {
    pub fn builder() -> CSSAtRuleBuilder<'a> { CSSAtRuleBuilder::default() }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn subsection(&self) -> Option<&str> { self.subsection.as_deref() }
    pub fn name(&self) -> Option<&ProtocolValue<'a>> { self.name.as_ref() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}

#[derive(Default)]
pub struct CSSAtRuleBuilder<'a> {
    type_: Option<Cow<'a, str>>,
    subsection: Option<Cow<'a, str>>,
    name: Option<ProtocolValue<'a>>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    origin: Option<StyleSheetOrigin>,
    style: Option<CSSStyle<'a>>,
}

impl<'a> CSSAtRuleBuilder<'a> {
    /// Type of at-rule.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    /// Subsection of font-feature-values, if this is a subsection.
    pub fn subsection(mut self, subsection: impl Into<Cow<'a, str>>) -> Self { self.subsection = Some(subsection.into()); self }
    /// LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)
    /// Associated name, if applicable.
    pub fn name(mut self, name: ProtocolValue<'a>) -> Self { self.name = Some(name); self }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Parent stylesheet's origin.
    pub fn origin(mut self, origin: StyleSheetOrigin) -> Self { self.origin = Some(origin); self }
    /// Associated style declaration.
    pub fn style(mut self, style: CSSStyle<'a>) -> Self { self.style = Some(style); self }
    pub fn build(self) -> CSSAtRule<'a> {
        CSSAtRule {
            type_: self.type_.unwrap_or_default(),
            subsection: self.subsection,
            name: self.name,
            styleSheetId: self.styleSheetId,
            origin: self.origin.unwrap_or_default(),
            style: self.style.unwrap_or_default(),
        }
    }
}

/// CSS property at-rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSPropertyRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated property name.
    propertyName: ProtocolValue<'a>,
    /// Associated style declaration.
    style: CSSStyle<'a>,
}

impl<'a> CSSPropertyRule<'a> {
    pub fn builder() -> CSSPropertyRuleBuilder<'a> { CSSPropertyRuleBuilder::default() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    pub fn propertyName(&self) -> &ProtocolValue<'a> { &self.propertyName }
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}

#[derive(Default)]
pub struct CSSPropertyRuleBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    origin: Option<StyleSheetOrigin>,
    propertyName: Option<ProtocolValue<'a>>,
    style: Option<CSSStyle<'a>>,
}

impl<'a> CSSPropertyRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Parent stylesheet's origin.
    pub fn origin(mut self, origin: StyleSheetOrigin) -> Self { self.origin = Some(origin); self }
    /// Associated property name.
    pub fn propertyName(mut self, propertyName: ProtocolValue<'a>) -> Self { self.propertyName = Some(propertyName); self }
    /// Associated style declaration.
    pub fn style(mut self, style: CSSStyle<'a>) -> Self { self.style = Some(style); self }
    pub fn build(self) -> CSSPropertyRule<'a> {
        CSSPropertyRule {
            styleSheetId: self.styleSheetId,
            origin: self.origin.unwrap_or_default(),
            propertyName: self.propertyName.unwrap_or_default(),
            style: self.style.unwrap_or_default(),
        }
    }
}

/// CSS function argument representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionParameter<'a> {
    /// The parameter name.
    name: Cow<'a, str>,
    /// The parameter type.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
}

impl<'a> CSSFunctionParameter<'a> {
    pub fn builder() -> CSSFunctionParameterBuilder<'a> { CSSFunctionParameterBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}

#[derive(Default)]
pub struct CSSFunctionParameterBuilder<'a> {
    name: Option<Cow<'a, str>>,
    type_: Option<Cow<'a, str>>,
}

impl<'a> CSSFunctionParameterBuilder<'a> {
    /// The parameter name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// The parameter type.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    pub fn build(self) -> CSSFunctionParameter<'a> {
        CSSFunctionParameter {
            name: self.name.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
        }
    }
}

/// CSS function conditional block representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionConditionNode<'a> {
    /// Media query for this conditional block. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    media: Option<CSSMedia<'a>>,
    /// Container query for this conditional block. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    containerQueries: Option<CSSContainerQuery<'a>>,
    /// @supports CSS at-rule condition. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    supports: Option<CSSSupports<'a>>,
    /// @navigation condition. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    navigation: Option<CSSNavigation<'a>>,
    /// Block body.
    children: Vec<CSSFunctionNode<'a>>,
    /// The condition text.
    conditionText: Cow<'a, str>,
}

impl<'a> CSSFunctionConditionNode<'a> {
    pub fn builder() -> CSSFunctionConditionNodeBuilder<'a> { CSSFunctionConditionNodeBuilder::default() }
    pub fn media(&self) -> Option<&CSSMedia<'a>> { self.media.as_ref() }
    pub fn containerQueries(&self) -> Option<&CSSContainerQuery<'a>> { self.containerQueries.as_ref() }
    pub fn supports(&self) -> Option<&CSSSupports<'a>> { self.supports.as_ref() }
    pub fn navigation(&self) -> Option<&CSSNavigation<'a>> { self.navigation.as_ref() }
    pub fn children(&self) -> &[CSSFunctionNode<'a>] { &self.children }
    pub fn conditionText(&self) -> &str { self.conditionText.as_ref() }
}

#[derive(Default)]
pub struct CSSFunctionConditionNodeBuilder<'a> {
    media: Option<CSSMedia<'a>>,
    containerQueries: Option<CSSContainerQuery<'a>>,
    supports: Option<CSSSupports<'a>>,
    navigation: Option<CSSNavigation<'a>>,
    children: Option<Vec<CSSFunctionNode<'a>>>,
    conditionText: Option<Cow<'a, str>>,
}

impl<'a> CSSFunctionConditionNodeBuilder<'a> {
    /// Media query for this conditional block. Only one type of condition should be set.
    pub fn media(mut self, media: CSSMedia<'a>) -> Self { self.media = Some(media); self }
    /// Container query for this conditional block. Only one type of condition should be set.
    pub fn containerQueries(mut self, containerQueries: CSSContainerQuery<'a>) -> Self { self.containerQueries = Some(containerQueries); self }
    /// @supports CSS at-rule condition. Only one type of condition should be set.
    pub fn supports(mut self, supports: CSSSupports<'a>) -> Self { self.supports = Some(supports); self }
    /// @navigation condition. Only one type of condition should be set.
    pub fn navigation(mut self, navigation: CSSNavigation<'a>) -> Self { self.navigation = Some(navigation); self }
    /// Block body.
    pub fn children(mut self, children: Vec<CSSFunctionNode<'a>>) -> Self { self.children = Some(children); self }
    /// The condition text.
    pub fn conditionText(mut self, conditionText: impl Into<Cow<'a, str>>) -> Self { self.conditionText = Some(conditionText.into()); self }
    pub fn build(self) -> CSSFunctionConditionNode<'a> {
        CSSFunctionConditionNode {
            media: self.media,
            containerQueries: self.containerQueries,
            supports: self.supports,
            navigation: self.navigation,
            children: self.children.unwrap_or_default(),
            conditionText: self.conditionText.unwrap_or_default(),
        }
    }
}

/// Section of the body of a CSS function rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionNode<'a> {
    /// A conditional block. If set, style should not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<CSSFunctionConditionNode<'a>>,
    /// Values set by this node. If set, condition should not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<CSSStyle<'a>>,
}

impl<'a> CSSFunctionNode<'a> {
    pub fn builder() -> CSSFunctionNodeBuilder<'a> { CSSFunctionNodeBuilder::default() }
    pub fn condition(&self) -> Option<&CSSFunctionConditionNode<'a>> { self.condition.as_ref() }
    pub fn style(&self) -> Option<&CSSStyle<'a>> { self.style.as_ref() }
}

#[derive(Default)]
pub struct CSSFunctionNodeBuilder<'a> {
    condition: Option<CSSFunctionConditionNode<'a>>,
    style: Option<CSSStyle<'a>>,
}

impl<'a> CSSFunctionNodeBuilder<'a> {
    /// A conditional block. If set, style should not be set.
    pub fn condition(mut self, condition: CSSFunctionConditionNode<'a>) -> Self { self.condition = Some(condition); self }
    /// Values set by this node. If set, condition should not be set.
    pub fn style(mut self, style: CSSStyle<'a>) -> Self { self.style = Some(style); self }
    pub fn build(self) -> CSSFunctionNode<'a> {
        CSSFunctionNode {
            condition: self.condition,
            style: self.style,
        }
    }
}

/// CSS function at-rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSFunctionRule<'a> {
    /// Name of the function.
    name: ProtocolValue<'a>,
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// List of parameters.
    parameters: Vec<CSSFunctionParameter<'a>>,
    /// Function body.
    children: Vec<CSSFunctionNode<'a>>,
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    originTreeScopeNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> CSSFunctionRule<'a> {
    pub fn builder() -> CSSFunctionRuleBuilder<'a> { CSSFunctionRuleBuilder::default() }
    pub fn name(&self) -> &ProtocolValue<'a> { &self.name }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    pub fn parameters(&self) -> &[CSSFunctionParameter<'a>] { &self.parameters }
    pub fn children(&self) -> &[CSSFunctionNode<'a>] { &self.children }
    pub fn originTreeScopeNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.originTreeScopeNodeId.as_ref() }
}

#[derive(Default)]
pub struct CSSFunctionRuleBuilder<'a> {
    name: Option<ProtocolValue<'a>>,
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    origin: Option<StyleSheetOrigin>,
    parameters: Option<Vec<CSSFunctionParameter<'a>>>,
    children: Option<Vec<CSSFunctionNode<'a>>>,
    originTreeScopeNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> CSSFunctionRuleBuilder<'a> {
    /// Name of the function.
    pub fn name(mut self, name: ProtocolValue<'a>) -> Self { self.name = Some(name); self }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Parent stylesheet's origin.
    pub fn origin(mut self, origin: StyleSheetOrigin) -> Self { self.origin = Some(origin); self }
    /// List of parameters.
    pub fn parameters(mut self, parameters: Vec<CSSFunctionParameter<'a>>) -> Self { self.parameters = Some(parameters); self }
    /// Function body.
    pub fn children(mut self, children: Vec<CSSFunctionNode<'a>>) -> Self { self.children = Some(children); self }
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    pub fn originTreeScopeNodeId(mut self, originTreeScopeNodeId: crate::dom::BackendNodeId) -> Self { self.originTreeScopeNodeId = Some(originTreeScopeNodeId); self }
    pub fn build(self) -> CSSFunctionRule<'a> {
        CSSFunctionRule {
            name: self.name.unwrap_or_default(),
            styleSheetId: self.styleSheetId,
            origin: self.origin.unwrap_or_default(),
            parameters: self.parameters.unwrap_or_default(),
            children: self.children.unwrap_or_default(),
            originTreeScopeNodeId: self.originTreeScopeNodeId,
        }
    }
}

/// CSS keyframe rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSKeyframeRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated key text.
    keyText: ProtocolValue<'a>,
    /// Associated style declaration.
    style: CSSStyle<'a>,
}

impl<'a> CSSKeyframeRule<'a> {
    pub fn builder() -> CSSKeyframeRuleBuilder<'a> { CSSKeyframeRuleBuilder::default() }
    pub fn styleSheetId(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.styleSheetId.as_ref() }
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    pub fn keyText(&self) -> &ProtocolValue<'a> { &self.keyText }
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}

#[derive(Default)]
pub struct CSSKeyframeRuleBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    origin: Option<StyleSheetOrigin>,
    keyText: Option<ProtocolValue<'a>>,
    style: Option<CSSStyle<'a>>,
}

impl<'a> CSSKeyframeRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// Parent stylesheet's origin.
    pub fn origin(mut self, origin: StyleSheetOrigin) -> Self { self.origin = Some(origin); self }
    /// Associated key text.
    pub fn keyText(mut self, keyText: ProtocolValue<'a>) -> Self { self.keyText = Some(keyText); self }
    /// Associated style declaration.
    pub fn style(mut self, style: CSSStyle<'a>) -> Self { self.style = Some(style); self }
    pub fn build(self) -> CSSKeyframeRule<'a> {
        CSSKeyframeRule {
            styleSheetId: self.styleSheetId,
            origin: self.origin.unwrap_or_default(),
            keyText: self.keyText.unwrap_or_default(),
            style: self.style.unwrap_or_default(),
        }
    }
}

/// A descriptor of operation to mutate style declaration text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StyleDeclarationEdit<'a> {
    /// The css style sheet identifier.
    styleSheetId: crate::dom::StyleSheetId<'a>,
    /// The range of the style text in the enclosing stylesheet.
    range: SourceRange,
    /// New style text.
    text: Cow<'a, str>,
}

impl<'a> StyleDeclarationEdit<'a> {
    pub fn builder() -> StyleDeclarationEditBuilder<'a> { StyleDeclarationEditBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}

#[derive(Default)]
pub struct StyleDeclarationEditBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    range: Option<SourceRange>,
    text: Option<Cow<'a, str>>,
}

impl<'a> StyleDeclarationEditBuilder<'a> {
    /// The css style sheet identifier.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// The range of the style text in the enclosing stylesheet.
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// New style text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> StyleDeclarationEdit<'a> {
        StyleDeclarationEdit {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
        }
    }
}

/// Inserts a new rule with the given 'ruleText' in a stylesheet with given 'styleSheetId', at the
/// position specified by 'location'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddRuleParams<'a> {
    /// The css style sheet identifier where a new rule should be inserted.
    styleSheetId: crate::dom::StyleSheetId<'a>,
    /// The text of a new rule.
    ruleText: Cow<'a, str>,
    /// Text position of a new rule in the target style sheet.
    location: SourceRange,
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeForPropertySyntaxValidation: Option<crate::dom::NodeId>,
}

impl<'a> AddRuleParams<'a> {
    pub fn builder() -> AddRuleParamsBuilder<'a> { AddRuleParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn ruleText(&self) -> &str { self.ruleText.as_ref() }
    pub fn location(&self) -> &SourceRange { &self.location }
    pub fn nodeForPropertySyntaxValidation(&self) -> Option<&crate::dom::NodeId> { self.nodeForPropertySyntaxValidation.as_ref() }
}

#[derive(Default)]
pub struct AddRuleParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    ruleText: Option<Cow<'a, str>>,
    location: Option<SourceRange>,
    nodeForPropertySyntaxValidation: Option<crate::dom::NodeId>,
}

impl<'a> AddRuleParamsBuilder<'a> {
    /// The css style sheet identifier where a new rule should be inserted.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    /// The text of a new rule.
    pub fn ruleText(mut self, ruleText: impl Into<Cow<'a, str>>) -> Self { self.ruleText = Some(ruleText.into()); self }
    /// Text position of a new rule in the target style sheet.
    pub fn location(mut self, location: SourceRange) -> Self { self.location = Some(location); self }
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    pub fn nodeForPropertySyntaxValidation(mut self, nodeForPropertySyntaxValidation: crate::dom::NodeId) -> Self { self.nodeForPropertySyntaxValidation = Some(nodeForPropertySyntaxValidation); self }
    pub fn build(self) -> AddRuleParams<'a> {
        AddRuleParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            ruleText: self.ruleText.unwrap_or_default(),
            location: self.location.unwrap_or_default(),
            nodeForPropertySyntaxValidation: self.nodeForPropertySyntaxValidation,
        }
    }
}

/// Inserts a new rule with the given 'ruleText' in a stylesheet with given 'styleSheetId', at the
/// position specified by 'location'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddRuleReturns<'a> {
    /// The newly created rule.
    rule: CSSRule<'a>,
}

impl<'a> AddRuleReturns<'a> {
    pub fn builder() -> AddRuleReturnsBuilder<'a> { AddRuleReturnsBuilder::default() }
    pub fn rule(&self) -> &CSSRule<'a> { &self.rule }
}

#[derive(Default)]
pub struct AddRuleReturnsBuilder<'a> {
    rule: Option<CSSRule<'a>>,
}

impl<'a> AddRuleReturnsBuilder<'a> {
    /// The newly created rule.
    pub fn rule(mut self, rule: CSSRule<'a>) -> Self { self.rule = Some(rule); self }
    pub fn build(self) -> AddRuleReturns<'a> {
        AddRuleReturns {
            rule: self.rule.unwrap_or_default(),
        }
    }
}

impl<'a> AddRuleParams<'a> { pub const METHOD: &'static str = "CSS.addRule"; }

impl<'a> crate::CdpCommand<'a> for AddRuleParams<'a> {
    const METHOD: &'static str = "CSS.addRule";
    type Response = AddRuleReturns<'a>;
}

/// Returns all class names from specified stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
}

impl<'a> CollectClassNamesParams<'a> {
    pub fn builder() -> CollectClassNamesParamsBuilder<'a> { CollectClassNamesParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
}

#[derive(Default)]
pub struct CollectClassNamesParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CollectClassNamesParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn build(self) -> CollectClassNamesParams<'a> {
        CollectClassNamesParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
        }
    }
}

/// Returns all class names from specified stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesReturns<'a> {
    /// Class name list.
    classNames: Vec<Cow<'a, str>>,
}

impl<'a> CollectClassNamesReturns<'a> {
    pub fn builder() -> CollectClassNamesReturnsBuilder<'a> { CollectClassNamesReturnsBuilder::default() }
    pub fn classNames(&self) -> &[Cow<'a, str>] { &self.classNames }
}

#[derive(Default)]
pub struct CollectClassNamesReturnsBuilder<'a> {
    classNames: Option<Vec<Cow<'a, str>>>,
}

impl<'a> CollectClassNamesReturnsBuilder<'a> {
    /// Class name list.
    pub fn classNames(mut self, classNames: Vec<Cow<'a, str>>) -> Self { self.classNames = Some(classNames); self }
    pub fn build(self) -> CollectClassNamesReturns<'a> {
        CollectClassNamesReturns {
            classNames: self.classNames.unwrap_or_default(),
        }
    }
}

impl<'a> CollectClassNamesParams<'a> { pub const METHOD: &'static str = "CSS.collectClassNames"; }

impl<'a> crate::CdpCommand<'a> for CollectClassNamesParams<'a> {
    const METHOD: &'static str = "CSS.collectClassNames";
    type Response = CollectClassNamesReturns<'a>;
}

/// Creates a new special "via-inspector" stylesheet in the frame with given 'frameId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateStyleSheetParams<'a> {
    /// Identifier of the frame where "via-inspector" stylesheet should be created.
    frameId: crate::page::FrameId<'a>,
    /// If true, creates a new stylesheet for every call. If false,
    /// returns a stylesheet previously created by a call with force=false
    /// for the frame's document if it exists or creates a new stylesheet
    /// (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}

impl<'a> CreateStyleSheetParams<'a> {
    pub fn builder() -> CreateStyleSheetParamsBuilder<'a> { CreateStyleSheetParamsBuilder::default() }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
    pub fn force(&self) -> Option<bool> { self.force }
}

#[derive(Default)]
pub struct CreateStyleSheetParamsBuilder<'a> {
    frameId: Option<crate::page::FrameId<'a>>,
    force: Option<bool>,
}

impl<'a> CreateStyleSheetParamsBuilder<'a> {
    /// Identifier of the frame where "via-inspector" stylesheet should be created.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// If true, creates a new stylesheet for every call. If false,
    /// returns a stylesheet previously created by a call with force=false
    /// for the frame's document if it exists or creates a new stylesheet
    /// (default: false).
    pub fn force(mut self, force: bool) -> Self { self.force = Some(force); self }
    pub fn build(self) -> CreateStyleSheetParams<'a> {
        CreateStyleSheetParams {
            frameId: self.frameId.unwrap_or_default(),
            force: self.force,
        }
    }
}

/// Creates a new special "via-inspector" stylesheet in the frame with given 'frameId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateStyleSheetReturns<'a> {
    /// Identifier of the created "via-inspector" stylesheet.
    styleSheetId: crate::dom::StyleSheetId<'a>,
}

impl<'a> CreateStyleSheetReturns<'a> {
    pub fn builder() -> CreateStyleSheetReturnsBuilder<'a> { CreateStyleSheetReturnsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
}

#[derive(Default)]
pub struct CreateStyleSheetReturnsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CreateStyleSheetReturnsBuilder<'a> {
    /// Identifier of the created "via-inspector" stylesheet.
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn build(self) -> CreateStyleSheetReturns<'a> {
        CreateStyleSheetReturns {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
        }
    }
}

impl<'a> CreateStyleSheetParams<'a> { pub const METHOD: &'static str = "CSS.createStyleSheet"; }

impl<'a> crate::CdpCommand<'a> for CreateStyleSheetParams<'a> {
    const METHOD: &'static str = "CSS.createStyleSheet";
    type Response = CreateStyleSheetReturns<'a>;
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

impl DisableParams { pub const METHOD: &'static str = "CSS.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "CSS.disable";
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

impl EnableParams { pub const METHOD: &'static str = "CSS.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "CSS.enable";
    type Response = crate::EmptyReturns;
}

/// Ensures that the given node will have specified pseudo-classes whenever its style is computed by
/// the browser.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForcePseudoStateParams<'a> {
    /// The element id for which to force the pseudo state.
    nodeId: crate::dom::NodeId,
    /// Element pseudo classes to force when computing the element's style.
    forcedPseudoClasses: Vec<Cow<'a, str>>,
}

impl<'a> ForcePseudoStateParams<'a> {
    pub fn builder() -> ForcePseudoStateParamsBuilder<'a> { ForcePseudoStateParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
    pub fn forcedPseudoClasses(&self) -> &[Cow<'a, str>] { &self.forcedPseudoClasses }
}

#[derive(Default)]
pub struct ForcePseudoStateParamsBuilder<'a> {
    nodeId: Option<crate::dom::NodeId>,
    forcedPseudoClasses: Option<Vec<Cow<'a, str>>>,
}

impl<'a> ForcePseudoStateParamsBuilder<'a> {
    /// The element id for which to force the pseudo state.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Element pseudo classes to force when computing the element's style.
    pub fn forcedPseudoClasses(mut self, forcedPseudoClasses: Vec<Cow<'a, str>>) -> Self { self.forcedPseudoClasses = Some(forcedPseudoClasses); self }
    pub fn build(self) -> ForcePseudoStateParams<'a> {
        ForcePseudoStateParams {
            nodeId: self.nodeId.unwrap_or_default(),
            forcedPseudoClasses: self.forcedPseudoClasses.unwrap_or_default(),
        }
    }
}

impl<'a> ForcePseudoStateParams<'a> { pub const METHOD: &'static str = "CSS.forcePseudoState"; }

impl<'a> crate::CdpCommand<'a> for ForcePseudoStateParams<'a> {
    const METHOD: &'static str = "CSS.forcePseudoState";
    type Response = crate::EmptyReturns;
}

/// Ensures that the given node is in its starting-style state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForceStartingStyleParams {
    /// The element id for which to force the starting-style state.
    nodeId: crate::dom::NodeId,
    /// Boolean indicating if this is on or off.
    forced: bool,
}

impl ForceStartingStyleParams {
    pub fn builder() -> ForceStartingStyleParamsBuilder { ForceStartingStyleParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
    pub fn forced(&self) -> bool { self.forced }
}

#[derive(Default)]
pub struct ForceStartingStyleParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
    forced: Option<bool>,
}

impl ForceStartingStyleParamsBuilder {
    /// The element id for which to force the starting-style state.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Boolean indicating if this is on or off.
    pub fn forced(mut self, forced: bool) -> Self { self.forced = Some(forced); self }
    pub fn build(self) -> ForceStartingStyleParams {
        ForceStartingStyleParams {
            nodeId: self.nodeId.unwrap_or_default(),
            forced: self.forced.unwrap_or_default(),
        }
    }
}

impl ForceStartingStyleParams { pub const METHOD: &'static str = "CSS.forceStartingStyle"; }

impl<'a> crate::CdpCommand<'a> for ForceStartingStyleParams {
    const METHOD: &'static str = "CSS.forceStartingStyle";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBackgroundColorsParams {
    /// Id of the node to get background colors for.
    nodeId: crate::dom::NodeId,
}

impl GetBackgroundColorsParams {
    pub fn builder() -> GetBackgroundColorsParamsBuilder { GetBackgroundColorsParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetBackgroundColorsParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
}

impl GetBackgroundColorsParamsBuilder {
    /// Id of the node to get background colors for.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetBackgroundColorsParams {
        GetBackgroundColorsParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBackgroundColorsReturns<'a> {
    /// The range of background colors behind this element, if it contains any visible text. If no
    /// visible text is present, this will be undefined. In the case of a flat background color,
    /// this will consist of simply that color. In the case of a gradient, this will consist of each
    /// of the color stops. For anything more complicated, this will be an empty array. Images will
    /// be ignored (as if the image had failed to load).
    #[serde(skip_serializing_if = "Option::is_none")]
    backgroundColors: Option<Vec<Cow<'a, str>>>,
    /// The computed font size for this node, as a CSS computed value string (e.g. '12px').
    #[serde(skip_serializing_if = "Option::is_none")]
    computedFontSize: Option<Cow<'a, str>>,
    /// The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
    /// '100').
    #[serde(skip_serializing_if = "Option::is_none")]
    computedFontWeight: Option<Cow<'a, str>>,
}

impl<'a> GetBackgroundColorsReturns<'a> {
    pub fn builder() -> GetBackgroundColorsReturnsBuilder<'a> { GetBackgroundColorsReturnsBuilder::default() }
    pub fn backgroundColors(&self) -> Option<&[Cow<'a, str>]> { self.backgroundColors.as_deref() }
    pub fn computedFontSize(&self) -> Option<&str> { self.computedFontSize.as_deref() }
    pub fn computedFontWeight(&self) -> Option<&str> { self.computedFontWeight.as_deref() }
}

#[derive(Default)]
pub struct GetBackgroundColorsReturnsBuilder<'a> {
    backgroundColors: Option<Vec<Cow<'a, str>>>,
    computedFontSize: Option<Cow<'a, str>>,
    computedFontWeight: Option<Cow<'a, str>>,
}

impl<'a> GetBackgroundColorsReturnsBuilder<'a> {
    /// The range of background colors behind this element, if it contains any visible text. If no
    /// visible text is present, this will be undefined. In the case of a flat background color,
    /// this will consist of simply that color. In the case of a gradient, this will consist of each
    /// of the color stops. For anything more complicated, this will be an empty array. Images will
    /// be ignored (as if the image had failed to load).
    pub fn backgroundColors(mut self, backgroundColors: Vec<Cow<'a, str>>) -> Self { self.backgroundColors = Some(backgroundColors); self }
    /// The computed font size for this node, as a CSS computed value string (e.g. '12px').
    pub fn computedFontSize(mut self, computedFontSize: impl Into<Cow<'a, str>>) -> Self { self.computedFontSize = Some(computedFontSize.into()); self }
    /// The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
    /// '100').
    pub fn computedFontWeight(mut self, computedFontWeight: impl Into<Cow<'a, str>>) -> Self { self.computedFontWeight = Some(computedFontWeight.into()); self }
    pub fn build(self) -> GetBackgroundColorsReturns<'a> {
        GetBackgroundColorsReturns {
            backgroundColors: self.backgroundColors,
            computedFontSize: self.computedFontSize,
            computedFontWeight: self.computedFontWeight,
        }
    }
}

impl GetBackgroundColorsParams { pub const METHOD: &'static str = "CSS.getBackgroundColors"; }

impl<'a> crate::CdpCommand<'a> for GetBackgroundColorsParams {
    const METHOD: &'static str = "CSS.getBackgroundColors";
    type Response = GetBackgroundColorsReturns<'a>;
}

/// Returns the computed style for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetComputedStyleForNodeParams {
    nodeId: crate::dom::NodeId,
}

impl GetComputedStyleForNodeParams {
    pub fn builder() -> GetComputedStyleForNodeParamsBuilder { GetComputedStyleForNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetComputedStyleForNodeParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
}

impl GetComputedStyleForNodeParamsBuilder {
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetComputedStyleForNodeParams {
        GetComputedStyleForNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Returns the computed style for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetComputedStyleForNodeReturns<'a> {
    /// Computed style for the specified DOM node.
    computedStyle: Vec<CSSComputedStyleProperty<'a>>,
    /// A list of non-standard "extra fields" which blink stores alongside each
    /// computed style.
    extraFields: ComputedStyleExtraFields,
}

impl<'a> GetComputedStyleForNodeReturns<'a> {
    pub fn builder() -> GetComputedStyleForNodeReturnsBuilder<'a> { GetComputedStyleForNodeReturnsBuilder::default() }
    pub fn computedStyle(&self) -> &[CSSComputedStyleProperty<'a>] { &self.computedStyle }
    pub fn extraFields(&self) -> &ComputedStyleExtraFields { &self.extraFields }
}

#[derive(Default)]
pub struct GetComputedStyleForNodeReturnsBuilder<'a> {
    computedStyle: Option<Vec<CSSComputedStyleProperty<'a>>>,
    extraFields: Option<ComputedStyleExtraFields>,
}

impl<'a> GetComputedStyleForNodeReturnsBuilder<'a> {
    /// Computed style for the specified DOM node.
    pub fn computedStyle(mut self, computedStyle: Vec<CSSComputedStyleProperty<'a>>) -> Self { self.computedStyle = Some(computedStyle); self }
    /// A list of non-standard "extra fields" which blink stores alongside each
    /// computed style.
    pub fn extraFields(mut self, extraFields: ComputedStyleExtraFields) -> Self { self.extraFields = Some(extraFields); self }
    pub fn build(self) -> GetComputedStyleForNodeReturns<'a> {
        GetComputedStyleForNodeReturns {
            computedStyle: self.computedStyle.unwrap_or_default(),
            extraFields: self.extraFields.unwrap_or_default(),
        }
    }
}

impl GetComputedStyleForNodeParams { pub const METHOD: &'static str = "CSS.getComputedStyleForNode"; }

impl<'a> crate::CdpCommand<'a> for GetComputedStyleForNodeParams {
    const METHOD: &'static str = "CSS.getComputedStyleForNode";
    type Response = GetComputedStyleForNodeReturns<'a>;
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
pub struct ResolveValuesParams<'a> {
    /// Cascade-dependent keywords (revert/revert-layer) do not work.
    values: Vec<Cow<'a, str>>,
    /// Id of the node in whose context the expression is evaluated
    nodeId: crate::dom::NodeId,
    /// Only longhands and custom property names are accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    propertyName: Option<Cow<'a, str>>,
    /// Pseudo element type, only works for pseudo elements that generate
    /// elements in the tree, such as ::before and ::after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoType: Option<crate::dom::PseudoType>,
    /// Pseudo element custom ident.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoIdentifier: Option<Cow<'a, str>>,
}

impl<'a> ResolveValuesParams<'a> {
    pub fn builder() -> ResolveValuesParamsBuilder<'a> { ResolveValuesParamsBuilder::default() }
    pub fn values(&self) -> &[Cow<'a, str>] { &self.values }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
    pub fn propertyName(&self) -> Option<&str> { self.propertyName.as_deref() }
    pub fn pseudoType(&self) -> Option<&crate::dom::PseudoType> { self.pseudoType.as_ref() }
    pub fn pseudoIdentifier(&self) -> Option<&str> { self.pseudoIdentifier.as_deref() }
}

#[derive(Default)]
pub struct ResolveValuesParamsBuilder<'a> {
    values: Option<Vec<Cow<'a, str>>>,
    nodeId: Option<crate::dom::NodeId>,
    propertyName: Option<Cow<'a, str>>,
    pseudoType: Option<crate::dom::PseudoType>,
    pseudoIdentifier: Option<Cow<'a, str>>,
}

impl<'a> ResolveValuesParamsBuilder<'a> {
    /// Cascade-dependent keywords (revert/revert-layer) do not work.
    pub fn values(mut self, values: Vec<Cow<'a, str>>) -> Self { self.values = Some(values); self }
    /// Id of the node in whose context the expression is evaluated
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// Only longhands and custom property names are accepted.
    pub fn propertyName(mut self, propertyName: impl Into<Cow<'a, str>>) -> Self { self.propertyName = Some(propertyName.into()); self }
    /// Pseudo element type, only works for pseudo elements that generate
    /// elements in the tree, such as ::before and ::after.
    pub fn pseudoType(mut self, pseudoType: crate::dom::PseudoType) -> Self { self.pseudoType = Some(pseudoType); self }
    /// Pseudo element custom ident.
    pub fn pseudoIdentifier(mut self, pseudoIdentifier: impl Into<Cow<'a, str>>) -> Self { self.pseudoIdentifier = Some(pseudoIdentifier.into()); self }
    pub fn build(self) -> ResolveValuesParams<'a> {
        ResolveValuesParams {
            values: self.values.unwrap_or_default(),
            nodeId: self.nodeId.unwrap_or_default(),
            propertyName: self.propertyName,
            pseudoType: self.pseudoType,
            pseudoIdentifier: self.pseudoIdentifier,
        }
    }
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
pub struct ResolveValuesReturns<'a> {
    results: Vec<Cow<'a, str>>,
}

impl<'a> ResolveValuesReturns<'a> {
    pub fn builder() -> ResolveValuesReturnsBuilder<'a> { ResolveValuesReturnsBuilder::default() }
    pub fn results(&self) -> &[Cow<'a, str>] { &self.results }
}

#[derive(Default)]
pub struct ResolveValuesReturnsBuilder<'a> {
    results: Option<Vec<Cow<'a, str>>>,
}

impl<'a> ResolveValuesReturnsBuilder<'a> {
    pub fn results(mut self, results: Vec<Cow<'a, str>>) -> Self { self.results = Some(results); self }
    pub fn build(self) -> ResolveValuesReturns<'a> {
        ResolveValuesReturns {
            results: self.results.unwrap_or_default(),
        }
    }
}

impl<'a> ResolveValuesParams<'a> { pub const METHOD: &'static str = "CSS.resolveValues"; }

impl<'a> crate::CdpCommand<'a> for ResolveValuesParams<'a> {
    const METHOD: &'static str = "CSS.resolveValues";
    type Response = ResolveValuesReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLonghandPropertiesParams<'a> {
    shorthandName: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> GetLonghandPropertiesParams<'a> {
    pub fn builder() -> GetLonghandPropertiesParamsBuilder<'a> { GetLonghandPropertiesParamsBuilder::default() }
    pub fn shorthandName(&self) -> &str { self.shorthandName.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct GetLonghandPropertiesParamsBuilder<'a> {
    shorthandName: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> GetLonghandPropertiesParamsBuilder<'a> {
    pub fn shorthandName(mut self, shorthandName: impl Into<Cow<'a, str>>) -> Self { self.shorthandName = Some(shorthandName.into()); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> GetLonghandPropertiesParams<'a> {
        GetLonghandPropertiesParams {
            shorthandName: self.shorthandName.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLonghandPropertiesReturns<'a> {
    longhandProperties: Vec<CSSProperty<'a>>,
}

impl<'a> GetLonghandPropertiesReturns<'a> {
    pub fn builder() -> GetLonghandPropertiesReturnsBuilder<'a> { GetLonghandPropertiesReturnsBuilder::default() }
    pub fn longhandProperties(&self) -> &[CSSProperty<'a>] { &self.longhandProperties }
}

#[derive(Default)]
pub struct GetLonghandPropertiesReturnsBuilder<'a> {
    longhandProperties: Option<Vec<CSSProperty<'a>>>,
}

impl<'a> GetLonghandPropertiesReturnsBuilder<'a> {
    pub fn longhandProperties(mut self, longhandProperties: Vec<CSSProperty<'a>>) -> Self { self.longhandProperties = Some(longhandProperties); self }
    pub fn build(self) -> GetLonghandPropertiesReturns<'a> {
        GetLonghandPropertiesReturns {
            longhandProperties: self.longhandProperties.unwrap_or_default(),
        }
    }
}

impl<'a> GetLonghandPropertiesParams<'a> { pub const METHOD: &'static str = "CSS.getLonghandProperties"; }

impl<'a> crate::CdpCommand<'a> for GetLonghandPropertiesParams<'a> {
    const METHOD: &'static str = "CSS.getLonghandProperties";
    type Response = GetLonghandPropertiesReturns<'a>;
}

/// Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
/// attributes) for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInlineStylesForNodeParams {
    nodeId: crate::dom::NodeId,
}

impl GetInlineStylesForNodeParams {
    pub fn builder() -> GetInlineStylesForNodeParamsBuilder { GetInlineStylesForNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetInlineStylesForNodeParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
}

impl GetInlineStylesForNodeParamsBuilder {
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetInlineStylesForNodeParams {
        GetInlineStylesForNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
/// attributes) for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInlineStylesForNodeReturns<'a> {
    /// Inline style for the specified DOM node.
    #[serde(skip_serializing_if = "Option::is_none")]
    inlineStyle: Option<CSSStyle<'a>>,
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    #[serde(skip_serializing_if = "Option::is_none")]
    attributesStyle: Option<CSSStyle<'a>>,
}

impl<'a> GetInlineStylesForNodeReturns<'a> {
    pub fn builder() -> GetInlineStylesForNodeReturnsBuilder<'a> { GetInlineStylesForNodeReturnsBuilder::default() }
    pub fn inlineStyle(&self) -> Option<&CSSStyle<'a>> { self.inlineStyle.as_ref() }
    pub fn attributesStyle(&self) -> Option<&CSSStyle<'a>> { self.attributesStyle.as_ref() }
}

#[derive(Default)]
pub struct GetInlineStylesForNodeReturnsBuilder<'a> {
    inlineStyle: Option<CSSStyle<'a>>,
    attributesStyle: Option<CSSStyle<'a>>,
}

impl<'a> GetInlineStylesForNodeReturnsBuilder<'a> {
    /// Inline style for the specified DOM node.
    pub fn inlineStyle(mut self, inlineStyle: CSSStyle<'a>) -> Self { self.inlineStyle = Some(inlineStyle); self }
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    pub fn attributesStyle(mut self, attributesStyle: CSSStyle<'a>) -> Self { self.attributesStyle = Some(attributesStyle); self }
    pub fn build(self) -> GetInlineStylesForNodeReturns<'a> {
        GetInlineStylesForNodeReturns {
            inlineStyle: self.inlineStyle,
            attributesStyle: self.attributesStyle,
        }
    }
}

impl GetInlineStylesForNodeParams { pub const METHOD: &'static str = "CSS.getInlineStylesForNode"; }

impl<'a> crate::CdpCommand<'a> for GetInlineStylesForNodeParams {
    const METHOD: &'static str = "CSS.getInlineStylesForNode";
    type Response = GetInlineStylesForNodeReturns<'a>;
}

/// Returns the styles coming from animations & transitions
/// including the animation & transition styles coming from inheritance chain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnimatedStylesForNodeParams {
    nodeId: crate::dom::NodeId,
}

impl GetAnimatedStylesForNodeParams {
    pub fn builder() -> GetAnimatedStylesForNodeParamsBuilder { GetAnimatedStylesForNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetAnimatedStylesForNodeParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
}

impl GetAnimatedStylesForNodeParamsBuilder {
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetAnimatedStylesForNodeParams {
        GetAnimatedStylesForNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Returns the styles coming from animations & transitions
/// including the animation & transition styles coming from inheritance chain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnimatedStylesForNodeReturns<'a> {
    /// Styles coming from animations.
    #[serde(skip_serializing_if = "Option::is_none")]
    animationStyles: Option<Vec<CSSAnimationStyle<'a>>>,
    /// Style coming from transitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    transitionsStyle: Option<CSSStyle<'a>>,
    /// Inherited style entries for animationsStyle and transitionsStyle from
    /// the inheritance chain of the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited: Option<Vec<InheritedAnimatedStyleEntry<'a>>>,
}

impl<'a> GetAnimatedStylesForNodeReturns<'a> {
    pub fn builder() -> GetAnimatedStylesForNodeReturnsBuilder<'a> { GetAnimatedStylesForNodeReturnsBuilder::default() }
    pub fn animationStyles(&self) -> Option<&[CSSAnimationStyle<'a>]> { self.animationStyles.as_deref() }
    pub fn transitionsStyle(&self) -> Option<&CSSStyle<'a>> { self.transitionsStyle.as_ref() }
    pub fn inherited(&self) -> Option<&[InheritedAnimatedStyleEntry<'a>]> { self.inherited.as_deref() }
}

#[derive(Default)]
pub struct GetAnimatedStylesForNodeReturnsBuilder<'a> {
    animationStyles: Option<Vec<CSSAnimationStyle<'a>>>,
    transitionsStyle: Option<CSSStyle<'a>>,
    inherited: Option<Vec<InheritedAnimatedStyleEntry<'a>>>,
}

impl<'a> GetAnimatedStylesForNodeReturnsBuilder<'a> {
    /// Styles coming from animations.
    pub fn animationStyles(mut self, animationStyles: Vec<CSSAnimationStyle<'a>>) -> Self { self.animationStyles = Some(animationStyles); self }
    /// Style coming from transitions.
    pub fn transitionsStyle(mut self, transitionsStyle: CSSStyle<'a>) -> Self { self.transitionsStyle = Some(transitionsStyle); self }
    /// Inherited style entries for animationsStyle and transitionsStyle from
    /// the inheritance chain of the element.
    pub fn inherited(mut self, inherited: Vec<InheritedAnimatedStyleEntry<'a>>) -> Self { self.inherited = Some(inherited); self }
    pub fn build(self) -> GetAnimatedStylesForNodeReturns<'a> {
        GetAnimatedStylesForNodeReturns {
            animationStyles: self.animationStyles,
            transitionsStyle: self.transitionsStyle,
            inherited: self.inherited,
        }
    }
}

impl GetAnimatedStylesForNodeParams { pub const METHOD: &'static str = "CSS.getAnimatedStylesForNode"; }

impl<'a> crate::CdpCommand<'a> for GetAnimatedStylesForNodeParams {
    const METHOD: &'static str = "CSS.getAnimatedStylesForNode";
    type Response = GetAnimatedStylesForNodeReturns<'a>;
}

/// Returns requested styles for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMatchedStylesForNodeParams {
    nodeId: crate::dom::NodeId,
}

impl GetMatchedStylesForNodeParams {
    pub fn builder() -> GetMatchedStylesForNodeParamsBuilder { GetMatchedStylesForNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetMatchedStylesForNodeParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
}

impl GetMatchedStylesForNodeParamsBuilder {
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetMatchedStylesForNodeParams {
        GetMatchedStylesForNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Returns requested styles for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMatchedStylesForNodeReturns<'a> {
    /// Inline style for the specified DOM node.
    #[serde(skip_serializing_if = "Option::is_none")]
    inlineStyle: Option<CSSStyle<'a>>,
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    #[serde(skip_serializing_if = "Option::is_none")]
    attributesStyle: Option<CSSStyle<'a>>,
    /// CSS rules matching this node, from all applicable stylesheets.
    #[serde(skip_serializing_if = "Option::is_none")]
    matchedCSSRules: Option<Vec<RuleMatch<'a>>>,
    /// Pseudo style matches for this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudoElements: Option<Vec<PseudoElementMatches<'a>>>,
    /// A chain of inherited styles (from the immediate node parent up to the DOM tree root).
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited: Option<Vec<InheritedStyleEntry<'a>>>,
    /// A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root).
    #[serde(skip_serializing_if = "Option::is_none")]
    inheritedPseudoElements: Option<Vec<InheritedPseudoElementMatches<'a>>>,
    /// A list of CSS keyframed animations matching this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    cssKeyframesRules: Option<Vec<CSSKeyframesRule<'a>>>,
    /// A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property.
    #[serde(skip_serializing_if = "Option::is_none")]
    cssPositionTryRules: Option<Vec<CSSPositionTryRule<'a>>>,
    /// Index of the active fallback in the applied position-try-fallback property,
    /// will not be set if there is no active position-try fallback.
    #[serde(skip_serializing_if = "Option::is_none")]
    activePositionFallbackIndex: Option<u64>,
    /// A list of CSS at-property rules matching this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    cssPropertyRules: Option<Vec<CSSPropertyRule<'a>>>,
    /// A list of CSS property registrations matching this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    cssPropertyRegistrations: Option<Vec<CSSPropertyRegistration<'a>>>,
    /// A list of simple @rules matching this node or its pseudo-elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    cssAtRules: Option<Vec<CSSAtRule<'a>>>,
    /// Id of the first parent element that does not have display: contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    parentLayoutNodeId: Option<crate::dom::NodeId>,
    /// A list of CSS at-function rules referenced by styles of this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    cssFunctionRules: Option<Vec<CSSFunctionRule<'a>>>,
}

impl<'a> GetMatchedStylesForNodeReturns<'a> {
    pub fn builder() -> GetMatchedStylesForNodeReturnsBuilder<'a> { GetMatchedStylesForNodeReturnsBuilder::default() }
    pub fn inlineStyle(&self) -> Option<&CSSStyle<'a>> { self.inlineStyle.as_ref() }
    pub fn attributesStyle(&self) -> Option<&CSSStyle<'a>> { self.attributesStyle.as_ref() }
    pub fn matchedCSSRules(&self) -> Option<&[RuleMatch<'a>]> { self.matchedCSSRules.as_deref() }
    pub fn pseudoElements(&self) -> Option<&[PseudoElementMatches<'a>]> { self.pseudoElements.as_deref() }
    pub fn inherited(&self) -> Option<&[InheritedStyleEntry<'a>]> { self.inherited.as_deref() }
    pub fn inheritedPseudoElements(&self) -> Option<&[InheritedPseudoElementMatches<'a>]> { self.inheritedPseudoElements.as_deref() }
    pub fn cssKeyframesRules(&self) -> Option<&[CSSKeyframesRule<'a>]> { self.cssKeyframesRules.as_deref() }
    pub fn cssPositionTryRules(&self) -> Option<&[CSSPositionTryRule<'a>]> { self.cssPositionTryRules.as_deref() }
    pub fn activePositionFallbackIndex(&self) -> Option<u64> { self.activePositionFallbackIndex }
    pub fn cssPropertyRules(&self) -> Option<&[CSSPropertyRule<'a>]> { self.cssPropertyRules.as_deref() }
    pub fn cssPropertyRegistrations(&self) -> Option<&[CSSPropertyRegistration<'a>]> { self.cssPropertyRegistrations.as_deref() }
    pub fn cssAtRules(&self) -> Option<&[CSSAtRule<'a>]> { self.cssAtRules.as_deref() }
    pub fn parentLayoutNodeId(&self) -> Option<&crate::dom::NodeId> { self.parentLayoutNodeId.as_ref() }
    pub fn cssFunctionRules(&self) -> Option<&[CSSFunctionRule<'a>]> { self.cssFunctionRules.as_deref() }
}

#[derive(Default)]
pub struct GetMatchedStylesForNodeReturnsBuilder<'a> {
    inlineStyle: Option<CSSStyle<'a>>,
    attributesStyle: Option<CSSStyle<'a>>,
    matchedCSSRules: Option<Vec<RuleMatch<'a>>>,
    pseudoElements: Option<Vec<PseudoElementMatches<'a>>>,
    inherited: Option<Vec<InheritedStyleEntry<'a>>>,
    inheritedPseudoElements: Option<Vec<InheritedPseudoElementMatches<'a>>>,
    cssKeyframesRules: Option<Vec<CSSKeyframesRule<'a>>>,
    cssPositionTryRules: Option<Vec<CSSPositionTryRule<'a>>>,
    activePositionFallbackIndex: Option<u64>,
    cssPropertyRules: Option<Vec<CSSPropertyRule<'a>>>,
    cssPropertyRegistrations: Option<Vec<CSSPropertyRegistration<'a>>>,
    cssAtRules: Option<Vec<CSSAtRule<'a>>>,
    parentLayoutNodeId: Option<crate::dom::NodeId>,
    cssFunctionRules: Option<Vec<CSSFunctionRule<'a>>>,
}

impl<'a> GetMatchedStylesForNodeReturnsBuilder<'a> {
    /// Inline style for the specified DOM node.
    pub fn inlineStyle(mut self, inlineStyle: CSSStyle<'a>) -> Self { self.inlineStyle = Some(inlineStyle); self }
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    pub fn attributesStyle(mut self, attributesStyle: CSSStyle<'a>) -> Self { self.attributesStyle = Some(attributesStyle); self }
    /// CSS rules matching this node, from all applicable stylesheets.
    pub fn matchedCSSRules(mut self, matchedCSSRules: Vec<RuleMatch<'a>>) -> Self { self.matchedCSSRules = Some(matchedCSSRules); self }
    /// Pseudo style matches for this node.
    pub fn pseudoElements(mut self, pseudoElements: Vec<PseudoElementMatches<'a>>) -> Self { self.pseudoElements = Some(pseudoElements); self }
    /// A chain of inherited styles (from the immediate node parent up to the DOM tree root).
    pub fn inherited(mut self, inherited: Vec<InheritedStyleEntry<'a>>) -> Self { self.inherited = Some(inherited); self }
    /// A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root).
    pub fn inheritedPseudoElements(mut self, inheritedPseudoElements: Vec<InheritedPseudoElementMatches<'a>>) -> Self { self.inheritedPseudoElements = Some(inheritedPseudoElements); self }
    /// A list of CSS keyframed animations matching this node.
    pub fn cssKeyframesRules(mut self, cssKeyframesRules: Vec<CSSKeyframesRule<'a>>) -> Self { self.cssKeyframesRules = Some(cssKeyframesRules); self }
    /// A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property.
    pub fn cssPositionTryRules(mut self, cssPositionTryRules: Vec<CSSPositionTryRule<'a>>) -> Self { self.cssPositionTryRules = Some(cssPositionTryRules); self }
    /// Index of the active fallback in the applied position-try-fallback property,
    /// will not be set if there is no active position-try fallback.
    pub fn activePositionFallbackIndex(mut self, activePositionFallbackIndex: u64) -> Self { self.activePositionFallbackIndex = Some(activePositionFallbackIndex); self }
    /// A list of CSS at-property rules matching this node.
    pub fn cssPropertyRules(mut self, cssPropertyRules: Vec<CSSPropertyRule<'a>>) -> Self { self.cssPropertyRules = Some(cssPropertyRules); self }
    /// A list of CSS property registrations matching this node.
    pub fn cssPropertyRegistrations(mut self, cssPropertyRegistrations: Vec<CSSPropertyRegistration<'a>>) -> Self { self.cssPropertyRegistrations = Some(cssPropertyRegistrations); self }
    /// A list of simple @rules matching this node or its pseudo-elements.
    pub fn cssAtRules(mut self, cssAtRules: Vec<CSSAtRule<'a>>) -> Self { self.cssAtRules = Some(cssAtRules); self }
    /// Id of the first parent element that does not have display: contents.
    pub fn parentLayoutNodeId(mut self, parentLayoutNodeId: crate::dom::NodeId) -> Self { self.parentLayoutNodeId = Some(parentLayoutNodeId); self }
    /// A list of CSS at-function rules referenced by styles of this node.
    pub fn cssFunctionRules(mut self, cssFunctionRules: Vec<CSSFunctionRule<'a>>) -> Self { self.cssFunctionRules = Some(cssFunctionRules); self }
    pub fn build(self) -> GetMatchedStylesForNodeReturns<'a> {
        GetMatchedStylesForNodeReturns {
            inlineStyle: self.inlineStyle,
            attributesStyle: self.attributesStyle,
            matchedCSSRules: self.matchedCSSRules,
            pseudoElements: self.pseudoElements,
            inherited: self.inherited,
            inheritedPseudoElements: self.inheritedPseudoElements,
            cssKeyframesRules: self.cssKeyframesRules,
            cssPositionTryRules: self.cssPositionTryRules,
            activePositionFallbackIndex: self.activePositionFallbackIndex,
            cssPropertyRules: self.cssPropertyRules,
            cssPropertyRegistrations: self.cssPropertyRegistrations,
            cssAtRules: self.cssAtRules,
            parentLayoutNodeId: self.parentLayoutNodeId,
            cssFunctionRules: self.cssFunctionRules,
        }
    }
}

impl GetMatchedStylesForNodeParams { pub const METHOD: &'static str = "CSS.getMatchedStylesForNode"; }

impl<'a> crate::CdpCommand<'a> for GetMatchedStylesForNodeParams {
    const METHOD: &'static str = "CSS.getMatchedStylesForNode";
    type Response = GetMatchedStylesForNodeReturns<'a>;
}

/// Returns the values of the default UA-defined environment variables used in env()

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEnvironmentVariablesReturns {
    environmentVariables: serde_json::Map<String, JsonValue>,
}

impl GetEnvironmentVariablesReturns {
    pub fn builder() -> GetEnvironmentVariablesReturnsBuilder { GetEnvironmentVariablesReturnsBuilder::default() }
    pub fn environmentVariables(&self) -> &serde_json::Map<String, JsonValue> { &self.environmentVariables }
}

#[derive(Default)]
pub struct GetEnvironmentVariablesReturnsBuilder {
    environmentVariables: Option<serde_json::Map<String, JsonValue>>,
}

impl GetEnvironmentVariablesReturnsBuilder {
    pub fn environmentVariables(mut self, environmentVariables: serde_json::Map<String, JsonValue>) -> Self { self.environmentVariables = Some(environmentVariables); self }
    pub fn build(self) -> GetEnvironmentVariablesReturns {
        GetEnvironmentVariablesReturns {
            environmentVariables: self.environmentVariables.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEnvironmentVariablesParams {}

impl GetEnvironmentVariablesParams {
    pub fn builder() -> GetEnvironmentVariablesParamsBuilder {
        GetEnvironmentVariablesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetEnvironmentVariablesParamsBuilder {}

impl GetEnvironmentVariablesParamsBuilder {
    pub fn build(self) -> GetEnvironmentVariablesParams {
        GetEnvironmentVariablesParams {}
    }
}

impl GetEnvironmentVariablesParams { pub const METHOD: &'static str = "CSS.getEnvironmentVariables"; }

impl<'a> crate::CdpCommand<'a> for GetEnvironmentVariablesParams {
    const METHOD: &'static str = "CSS.getEnvironmentVariables";
    type Response = GetEnvironmentVariablesReturns;
}

/// Returns all media queries parsed by the rendering engine.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaQueriesReturns<'a> {
    medias: Vec<CSSMedia<'a>>,
}

impl<'a> GetMediaQueriesReturns<'a> {
    pub fn builder() -> GetMediaQueriesReturnsBuilder<'a> { GetMediaQueriesReturnsBuilder::default() }
    pub fn medias(&self) -> &[CSSMedia<'a>] { &self.medias }
}

#[derive(Default)]
pub struct GetMediaQueriesReturnsBuilder<'a> {
    medias: Option<Vec<CSSMedia<'a>>>,
}

impl<'a> GetMediaQueriesReturnsBuilder<'a> {
    pub fn medias(mut self, medias: Vec<CSSMedia<'a>>) -> Self { self.medias = Some(medias); self }
    pub fn build(self) -> GetMediaQueriesReturns<'a> {
        GetMediaQueriesReturns {
            medias: self.medias.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMediaQueriesParams {}

impl GetMediaQueriesParams {
    pub fn builder() -> GetMediaQueriesParamsBuilder {
        GetMediaQueriesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetMediaQueriesParamsBuilder {}

impl GetMediaQueriesParamsBuilder {
    pub fn build(self) -> GetMediaQueriesParams {
        GetMediaQueriesParams {}
    }
}

impl GetMediaQueriesParams { pub const METHOD: &'static str = "CSS.getMediaQueries"; }

impl<'a> crate::CdpCommand<'a> for GetMediaQueriesParams {
    const METHOD: &'static str = "CSS.getMediaQueries";
    type Response = GetMediaQueriesReturns<'a>;
}

/// Requests information about platform fonts which we used to render child TextNodes in the given
/// node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPlatformFontsForNodeParams {
    nodeId: crate::dom::NodeId,
}

impl GetPlatformFontsForNodeParams {
    pub fn builder() -> GetPlatformFontsForNodeParamsBuilder { GetPlatformFontsForNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetPlatformFontsForNodeParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
}

impl GetPlatformFontsForNodeParamsBuilder {
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetPlatformFontsForNodeParams {
        GetPlatformFontsForNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Requests information about platform fonts which we used to render child TextNodes in the given
/// node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPlatformFontsForNodeReturns<'a> {
    /// Usage statistics for every employed platform font.
    fonts: Vec<PlatformFontUsage<'a>>,
}

impl<'a> GetPlatformFontsForNodeReturns<'a> {
    pub fn builder() -> GetPlatformFontsForNodeReturnsBuilder<'a> { GetPlatformFontsForNodeReturnsBuilder::default() }
    pub fn fonts(&self) -> &[PlatformFontUsage<'a>] { &self.fonts }
}

#[derive(Default)]
pub struct GetPlatformFontsForNodeReturnsBuilder<'a> {
    fonts: Option<Vec<PlatformFontUsage<'a>>>,
}

impl<'a> GetPlatformFontsForNodeReturnsBuilder<'a> {
    /// Usage statistics for every employed platform font.
    pub fn fonts(mut self, fonts: Vec<PlatformFontUsage<'a>>) -> Self { self.fonts = Some(fonts); self }
    pub fn build(self) -> GetPlatformFontsForNodeReturns<'a> {
        GetPlatformFontsForNodeReturns {
            fonts: self.fonts.unwrap_or_default(),
        }
    }
}

impl GetPlatformFontsForNodeParams { pub const METHOD: &'static str = "CSS.getPlatformFontsForNode"; }

impl<'a> crate::CdpCommand<'a> for GetPlatformFontsForNodeParams {
    const METHOD: &'static str = "CSS.getPlatformFontsForNode";
    type Response = GetPlatformFontsForNodeReturns<'a>;
}

/// Returns the current textual content for a stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStyleSheetTextParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
}

impl<'a> GetStyleSheetTextParams<'a> {
    pub fn builder() -> GetStyleSheetTextParamsBuilder<'a> { GetStyleSheetTextParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
}

#[derive(Default)]
pub struct GetStyleSheetTextParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> GetStyleSheetTextParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn build(self) -> GetStyleSheetTextParams<'a> {
        GetStyleSheetTextParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
        }
    }
}

/// Returns the current textual content for a stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStyleSheetTextReturns<'a> {
    /// The stylesheet text.
    text: Cow<'a, str>,
}

impl<'a> GetStyleSheetTextReturns<'a> {
    pub fn builder() -> GetStyleSheetTextReturnsBuilder<'a> { GetStyleSheetTextReturnsBuilder::default() }
    pub fn text(&self) -> &str { self.text.as_ref() }
}

#[derive(Default)]
pub struct GetStyleSheetTextReturnsBuilder<'a> {
    text: Option<Cow<'a, str>>,
}

impl<'a> GetStyleSheetTextReturnsBuilder<'a> {
    /// The stylesheet text.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> GetStyleSheetTextReturns<'a> {
        GetStyleSheetTextReturns {
            text: self.text.unwrap_or_default(),
        }
    }
}

impl<'a> GetStyleSheetTextParams<'a> { pub const METHOD: &'static str = "CSS.getStyleSheetText"; }

impl<'a> crate::CdpCommand<'a> for GetStyleSheetTextParams<'a> {
    const METHOD: &'static str = "CSS.getStyleSheetText";
    type Response = GetStyleSheetTextReturns<'a>;
}

/// Returns all layers parsed by the rendering engine for the tree scope of a node.
/// Given a DOM element identified by nodeId, getLayersForNode returns the root
/// layer for the nearest ancestor document or shadow root. The layer root contains
/// the full layer tree for the tree scope and their ordering.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLayersForNodeParams {
    nodeId: crate::dom::NodeId,
}

impl GetLayersForNodeParams {
    pub fn builder() -> GetLayersForNodeParamsBuilder { GetLayersForNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
}

#[derive(Default)]
pub struct GetLayersForNodeParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
}

impl GetLayersForNodeParamsBuilder {
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> GetLayersForNodeParams {
        GetLayersForNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
        }
    }
}

/// Returns all layers parsed by the rendering engine for the tree scope of a node.
/// Given a DOM element identified by nodeId, getLayersForNode returns the root
/// layer for the nearest ancestor document or shadow root. The layer root contains
/// the full layer tree for the tree scope and their ordering.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLayersForNodeReturns<'a> {
    rootLayer: CSSLayerData<'a>,
}

impl<'a> GetLayersForNodeReturns<'a> {
    pub fn builder() -> GetLayersForNodeReturnsBuilder<'a> { GetLayersForNodeReturnsBuilder::default() }
    pub fn rootLayer(&self) -> &CSSLayerData<'a> { &self.rootLayer }
}

#[derive(Default)]
pub struct GetLayersForNodeReturnsBuilder<'a> {
    rootLayer: Option<CSSLayerData<'a>>,
}

impl<'a> GetLayersForNodeReturnsBuilder<'a> {
    pub fn rootLayer(mut self, rootLayer: CSSLayerData<'a>) -> Self { self.rootLayer = Some(rootLayer); self }
    pub fn build(self) -> GetLayersForNodeReturns<'a> {
        GetLayersForNodeReturns {
            rootLayer: self.rootLayer.unwrap_or_default(),
        }
    }
}

impl GetLayersForNodeParams { pub const METHOD: &'static str = "CSS.getLayersForNode"; }

impl<'a> crate::CdpCommand<'a> for GetLayersForNodeParams {
    const METHOD: &'static str = "CSS.getLayersForNode";
    type Response = GetLayersForNodeReturns<'a>;
}

/// Given a CSS selector text and a style sheet ID, getLocationForSelector
/// returns an array of locations of the CSS selector in the style sheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationForSelectorParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    selectorText: Cow<'a, str>,
}

impl<'a> GetLocationForSelectorParams<'a> {
    pub fn builder() -> GetLocationForSelectorParamsBuilder<'a> { GetLocationForSelectorParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn selectorText(&self) -> &str { self.selectorText.as_ref() }
}

#[derive(Default)]
pub struct GetLocationForSelectorParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    selectorText: Option<Cow<'a, str>>,
}

impl<'a> GetLocationForSelectorParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn selectorText(mut self, selectorText: impl Into<Cow<'a, str>>) -> Self { self.selectorText = Some(selectorText.into()); self }
    pub fn build(self) -> GetLocationForSelectorParams<'a> {
        GetLocationForSelectorParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            selectorText: self.selectorText.unwrap_or_default(),
        }
    }
}

/// Given a CSS selector text and a style sheet ID, getLocationForSelector
/// returns an array of locations of the CSS selector in the style sheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationForSelectorReturns {
    ranges: Vec<SourceRange>,
}

impl GetLocationForSelectorReturns {
    pub fn builder() -> GetLocationForSelectorReturnsBuilder { GetLocationForSelectorReturnsBuilder::default() }
    pub fn ranges(&self) -> &[SourceRange] { &self.ranges }
}

#[derive(Default)]
pub struct GetLocationForSelectorReturnsBuilder {
    ranges: Option<Vec<SourceRange>>,
}

impl GetLocationForSelectorReturnsBuilder {
    pub fn ranges(mut self, ranges: Vec<SourceRange>) -> Self { self.ranges = Some(ranges); self }
    pub fn build(self) -> GetLocationForSelectorReturns {
        GetLocationForSelectorReturns {
            ranges: self.ranges.unwrap_or_default(),
        }
    }
}

impl<'a> GetLocationForSelectorParams<'a> { pub const METHOD: &'static str = "CSS.getLocationForSelector"; }

impl<'a> crate::CdpCommand<'a> for GetLocationForSelectorParams<'a> {
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
    nodeId: Option<crate::dom::NodeId>,
}

impl TrackComputedStyleUpdatesForNodeParams {
    pub fn builder() -> TrackComputedStyleUpdatesForNodeParamsBuilder { TrackComputedStyleUpdatesForNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> Option<&crate::dom::NodeId> { self.nodeId.as_ref() }
}

#[derive(Default)]
pub struct TrackComputedStyleUpdatesForNodeParamsBuilder {
    nodeId: Option<crate::dom::NodeId>,
}

impl TrackComputedStyleUpdatesForNodeParamsBuilder {
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn build(self) -> TrackComputedStyleUpdatesForNodeParams {
        TrackComputedStyleUpdatesForNodeParams {
            nodeId: self.nodeId,
        }
    }
}

impl TrackComputedStyleUpdatesForNodeParams { pub const METHOD: &'static str = "CSS.trackComputedStyleUpdatesForNode"; }

impl<'a> crate::CdpCommand<'a> for TrackComputedStyleUpdatesForNodeParams {
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
pub struct TrackComputedStyleUpdatesParams<'a> {
    propertiesToTrack: Vec<CSSComputedStyleProperty<'a>>,
}

impl<'a> TrackComputedStyleUpdatesParams<'a> {
    pub fn builder() -> TrackComputedStyleUpdatesParamsBuilder<'a> { TrackComputedStyleUpdatesParamsBuilder::default() }
    pub fn propertiesToTrack(&self) -> &[CSSComputedStyleProperty<'a>] { &self.propertiesToTrack }
}

#[derive(Default)]
pub struct TrackComputedStyleUpdatesParamsBuilder<'a> {
    propertiesToTrack: Option<Vec<CSSComputedStyleProperty<'a>>>,
}

impl<'a> TrackComputedStyleUpdatesParamsBuilder<'a> {
    pub fn propertiesToTrack(mut self, propertiesToTrack: Vec<CSSComputedStyleProperty<'a>>) -> Self { self.propertiesToTrack = Some(propertiesToTrack); self }
    pub fn build(self) -> TrackComputedStyleUpdatesParams<'a> {
        TrackComputedStyleUpdatesParams {
            propertiesToTrack: self.propertiesToTrack.unwrap_or_default(),
        }
    }
}

impl<'a> TrackComputedStyleUpdatesParams<'a> { pub const METHOD: &'static str = "CSS.trackComputedStyleUpdates"; }

impl<'a> crate::CdpCommand<'a> for TrackComputedStyleUpdatesParams<'a> {
    const METHOD: &'static str = "CSS.trackComputedStyleUpdates";
    type Response = crate::EmptyReturns;
}

/// Polls the next batch of computed style updates.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeComputedStyleUpdatesReturns {
    /// The list of node Ids that have their tracked computed styles updated.
    nodeIds: Vec<crate::dom::NodeId>,
}

impl TakeComputedStyleUpdatesReturns {
    pub fn builder() -> TakeComputedStyleUpdatesReturnsBuilder { TakeComputedStyleUpdatesReturnsBuilder::default() }
    pub fn nodeIds(&self) -> &[crate::dom::NodeId] { &self.nodeIds }
}

#[derive(Default)]
pub struct TakeComputedStyleUpdatesReturnsBuilder {
    nodeIds: Option<Vec<crate::dom::NodeId>>,
}

impl TakeComputedStyleUpdatesReturnsBuilder {
    /// The list of node Ids that have their tracked computed styles updated.
    pub fn nodeIds(mut self, nodeIds: Vec<crate::dom::NodeId>) -> Self { self.nodeIds = Some(nodeIds); self }
    pub fn build(self) -> TakeComputedStyleUpdatesReturns {
        TakeComputedStyleUpdatesReturns {
            nodeIds: self.nodeIds.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TakeComputedStyleUpdatesParams {}

impl TakeComputedStyleUpdatesParams {
    pub fn builder() -> TakeComputedStyleUpdatesParamsBuilder {
        TakeComputedStyleUpdatesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct TakeComputedStyleUpdatesParamsBuilder {}

impl TakeComputedStyleUpdatesParamsBuilder {
    pub fn build(self) -> TakeComputedStyleUpdatesParams {
        TakeComputedStyleUpdatesParams {}
    }
}

impl TakeComputedStyleUpdatesParams { pub const METHOD: &'static str = "CSS.takeComputedStyleUpdates"; }

impl<'a> crate::CdpCommand<'a> for TakeComputedStyleUpdatesParams {
    const METHOD: &'static str = "CSS.takeComputedStyleUpdates";
    type Response = TakeComputedStyleUpdatesReturns;
}

/// Find a rule with the given active property for the given node and set the new value for this
/// property

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEffectivePropertyValueForNodeParams<'a> {
    /// The element id for which to set property.
    nodeId: crate::dom::NodeId,
    propertyName: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SetEffectivePropertyValueForNodeParams<'a> {
    pub fn builder() -> SetEffectivePropertyValueForNodeParamsBuilder<'a> { SetEffectivePropertyValueForNodeParamsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::NodeId { &self.nodeId }
    pub fn propertyName(&self) -> &str { self.propertyName.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct SetEffectivePropertyValueForNodeParamsBuilder<'a> {
    nodeId: Option<crate::dom::NodeId>,
    propertyName: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> SetEffectivePropertyValueForNodeParamsBuilder<'a> {
    /// The element id for which to set property.
    pub fn nodeId(mut self, nodeId: crate::dom::NodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn propertyName(mut self, propertyName: impl Into<Cow<'a, str>>) -> Self { self.propertyName = Some(propertyName.into()); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> SetEffectivePropertyValueForNodeParams<'a> {
        SetEffectivePropertyValueForNodeParams {
            nodeId: self.nodeId.unwrap_or_default(),
            propertyName: self.propertyName.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}

impl<'a> SetEffectivePropertyValueForNodeParams<'a> { pub const METHOD: &'static str = "CSS.setEffectivePropertyValueForNode"; }

impl<'a> crate::CdpCommand<'a> for SetEffectivePropertyValueForNodeParams<'a> {
    const METHOD: &'static str = "CSS.setEffectivePropertyValueForNode";
    type Response = crate::EmptyReturns;
}

/// Modifies the property rule property name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPropertyRulePropertyNameParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    propertyName: Cow<'a, str>,
}

impl<'a> SetPropertyRulePropertyNameParams<'a> {
    pub fn builder() -> SetPropertyRulePropertyNameParamsBuilder<'a> { SetPropertyRulePropertyNameParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn propertyName(&self) -> &str { self.propertyName.as_ref() }
}

#[derive(Default)]
pub struct SetPropertyRulePropertyNameParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    range: Option<SourceRange>,
    propertyName: Option<Cow<'a, str>>,
}

impl<'a> SetPropertyRulePropertyNameParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn propertyName(mut self, propertyName: impl Into<Cow<'a, str>>) -> Self { self.propertyName = Some(propertyName.into()); self }
    pub fn build(self) -> SetPropertyRulePropertyNameParams<'a> {
        SetPropertyRulePropertyNameParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            propertyName: self.propertyName.unwrap_or_default(),
        }
    }
}

/// Modifies the property rule property name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPropertyRulePropertyNameReturns<'a> {
    /// The resulting key text after modification.
    propertyName: ProtocolValue<'a>,
}

impl<'a> SetPropertyRulePropertyNameReturns<'a> {
    pub fn builder() -> SetPropertyRulePropertyNameReturnsBuilder<'a> { SetPropertyRulePropertyNameReturnsBuilder::default() }
    pub fn propertyName(&self) -> &ProtocolValue<'a> { &self.propertyName }
}

#[derive(Default)]
pub struct SetPropertyRulePropertyNameReturnsBuilder<'a> {
    propertyName: Option<ProtocolValue<'a>>,
}

impl<'a> SetPropertyRulePropertyNameReturnsBuilder<'a> {
    /// The resulting key text after modification.
    pub fn propertyName(mut self, propertyName: ProtocolValue<'a>) -> Self { self.propertyName = Some(propertyName); self }
    pub fn build(self) -> SetPropertyRulePropertyNameReturns<'a> {
        SetPropertyRulePropertyNameReturns {
            propertyName: self.propertyName.unwrap_or_default(),
        }
    }
}

impl<'a> SetPropertyRulePropertyNameParams<'a> { pub const METHOD: &'static str = "CSS.setPropertyRulePropertyName"; }

impl<'a> crate::CdpCommand<'a> for SetPropertyRulePropertyNameParams<'a> {
    const METHOD: &'static str = "CSS.setPropertyRulePropertyName";
    type Response = SetPropertyRulePropertyNameReturns<'a>;
}

/// Modifies the keyframe rule key text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetKeyframeKeyParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    keyText: Cow<'a, str>,
}

impl<'a> SetKeyframeKeyParams<'a> {
    pub fn builder() -> SetKeyframeKeyParamsBuilder<'a> { SetKeyframeKeyParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn keyText(&self) -> &str { self.keyText.as_ref() }
}

#[derive(Default)]
pub struct SetKeyframeKeyParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    range: Option<SourceRange>,
    keyText: Option<Cow<'a, str>>,
}

impl<'a> SetKeyframeKeyParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn keyText(mut self, keyText: impl Into<Cow<'a, str>>) -> Self { self.keyText = Some(keyText.into()); self }
    pub fn build(self) -> SetKeyframeKeyParams<'a> {
        SetKeyframeKeyParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            keyText: self.keyText.unwrap_or_default(),
        }
    }
}

/// Modifies the keyframe rule key text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetKeyframeKeyReturns<'a> {
    /// The resulting key text after modification.
    keyText: ProtocolValue<'a>,
}

impl<'a> SetKeyframeKeyReturns<'a> {
    pub fn builder() -> SetKeyframeKeyReturnsBuilder<'a> { SetKeyframeKeyReturnsBuilder::default() }
    pub fn keyText(&self) -> &ProtocolValue<'a> { &self.keyText }
}

#[derive(Default)]
pub struct SetKeyframeKeyReturnsBuilder<'a> {
    keyText: Option<ProtocolValue<'a>>,
}

impl<'a> SetKeyframeKeyReturnsBuilder<'a> {
    /// The resulting key text after modification.
    pub fn keyText(mut self, keyText: ProtocolValue<'a>) -> Self { self.keyText = Some(keyText); self }
    pub fn build(self) -> SetKeyframeKeyReturns<'a> {
        SetKeyframeKeyReturns {
            keyText: self.keyText.unwrap_or_default(),
        }
    }
}

impl<'a> SetKeyframeKeyParams<'a> { pub const METHOD: &'static str = "CSS.setKeyframeKey"; }

impl<'a> crate::CdpCommand<'a> for SetKeyframeKeyParams<'a> {
    const METHOD: &'static str = "CSS.setKeyframeKey";
    type Response = SetKeyframeKeyReturns<'a>;
}

/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMediaTextParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetMediaTextParams<'a> {
    pub fn builder() -> SetMediaTextParamsBuilder<'a> { SetMediaTextParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}

#[derive(Default)]
pub struct SetMediaTextParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    range: Option<SourceRange>,
    text: Option<Cow<'a, str>>,
}

impl<'a> SetMediaTextParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> SetMediaTextParams<'a> {
        SetMediaTextParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
        }
    }
}

/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMediaTextReturns<'a> {
    /// The resulting CSS media rule after modification.
    media: CSSMedia<'a>,
}

impl<'a> SetMediaTextReturns<'a> {
    pub fn builder() -> SetMediaTextReturnsBuilder<'a> { SetMediaTextReturnsBuilder::default() }
    pub fn media(&self) -> &CSSMedia<'a> { &self.media }
}

#[derive(Default)]
pub struct SetMediaTextReturnsBuilder<'a> {
    media: Option<CSSMedia<'a>>,
}

impl<'a> SetMediaTextReturnsBuilder<'a> {
    /// The resulting CSS media rule after modification.
    pub fn media(mut self, media: CSSMedia<'a>) -> Self { self.media = Some(media); self }
    pub fn build(self) -> SetMediaTextReturns<'a> {
        SetMediaTextReturns {
            media: self.media.unwrap_or_default(),
        }
    }
}

impl<'a> SetMediaTextParams<'a> { pub const METHOD: &'static str = "CSS.setMediaText"; }

impl<'a> crate::CdpCommand<'a> for SetMediaTextParams<'a> {
    const METHOD: &'static str = "CSS.setMediaText";
    type Response = SetMediaTextReturns<'a>;
}

/// Modifies the expression of a container query.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryTextParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetContainerQueryTextParams<'a> {
    pub fn builder() -> SetContainerQueryTextParamsBuilder<'a> { SetContainerQueryTextParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}

#[derive(Default)]
pub struct SetContainerQueryTextParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    range: Option<SourceRange>,
    text: Option<Cow<'a, str>>,
}

impl<'a> SetContainerQueryTextParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> SetContainerQueryTextParams<'a> {
        SetContainerQueryTextParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
        }
    }
}

/// Modifies the expression of a container query.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryTextReturns<'a> {
    /// The resulting CSS container query rule after modification.
    containerQuery: CSSContainerQuery<'a>,
}

impl<'a> SetContainerQueryTextReturns<'a> {
    pub fn builder() -> SetContainerQueryTextReturnsBuilder<'a> { SetContainerQueryTextReturnsBuilder::default() }
    pub fn containerQuery(&self) -> &CSSContainerQuery<'a> { &self.containerQuery }
}

#[derive(Default)]
pub struct SetContainerQueryTextReturnsBuilder<'a> {
    containerQuery: Option<CSSContainerQuery<'a>>,
}

impl<'a> SetContainerQueryTextReturnsBuilder<'a> {
    /// The resulting CSS container query rule after modification.
    pub fn containerQuery(mut self, containerQuery: CSSContainerQuery<'a>) -> Self { self.containerQuery = Some(containerQuery); self }
    pub fn build(self) -> SetContainerQueryTextReturns<'a> {
        SetContainerQueryTextReturns {
            containerQuery: self.containerQuery.unwrap_or_default(),
        }
    }
}

impl<'a> SetContainerQueryTextParams<'a> { pub const METHOD: &'static str = "CSS.setContainerQueryText"; }

impl<'a> crate::CdpCommand<'a> for SetContainerQueryTextParams<'a> {
    const METHOD: &'static str = "CSS.setContainerQueryText";
    type Response = SetContainerQueryTextReturns<'a>;
}

/// Modifies the expression of a supports at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSupportsTextParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetSupportsTextParams<'a> {
    pub fn builder() -> SetSupportsTextParamsBuilder<'a> { SetSupportsTextParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}

#[derive(Default)]
pub struct SetSupportsTextParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    range: Option<SourceRange>,
    text: Option<Cow<'a, str>>,
}

impl<'a> SetSupportsTextParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> SetSupportsTextParams<'a> {
        SetSupportsTextParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
        }
    }
}

/// Modifies the expression of a supports at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSupportsTextReturns<'a> {
    /// The resulting CSS Supports rule after modification.
    supports: CSSSupports<'a>,
}

impl<'a> SetSupportsTextReturns<'a> {
    pub fn builder() -> SetSupportsTextReturnsBuilder<'a> { SetSupportsTextReturnsBuilder::default() }
    pub fn supports(&self) -> &CSSSupports<'a> { &self.supports }
}

#[derive(Default)]
pub struct SetSupportsTextReturnsBuilder<'a> {
    supports: Option<CSSSupports<'a>>,
}

impl<'a> SetSupportsTextReturnsBuilder<'a> {
    /// The resulting CSS Supports rule after modification.
    pub fn supports(mut self, supports: CSSSupports<'a>) -> Self { self.supports = Some(supports); self }
    pub fn build(self) -> SetSupportsTextReturns<'a> {
        SetSupportsTextReturns {
            supports: self.supports.unwrap_or_default(),
        }
    }
}

impl<'a> SetSupportsTextParams<'a> { pub const METHOD: &'static str = "CSS.setSupportsText"; }

impl<'a> crate::CdpCommand<'a> for SetSupportsTextParams<'a> {
    const METHOD: &'static str = "CSS.setSupportsText";
    type Response = SetSupportsTextReturns<'a>;
}

/// Modifies the expression of a navigation at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNavigationTextParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetNavigationTextParams<'a> {
    pub fn builder() -> SetNavigationTextParamsBuilder<'a> { SetNavigationTextParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}

#[derive(Default)]
pub struct SetNavigationTextParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    range: Option<SourceRange>,
    text: Option<Cow<'a, str>>,
}

impl<'a> SetNavigationTextParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> SetNavigationTextParams<'a> {
        SetNavigationTextParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
        }
    }
}

/// Modifies the expression of a navigation at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNavigationTextReturns<'a> {
    /// The resulting CSS Navigation rule after modification.
    navigation: CSSNavigation<'a>,
}

impl<'a> SetNavigationTextReturns<'a> {
    pub fn builder() -> SetNavigationTextReturnsBuilder<'a> { SetNavigationTextReturnsBuilder::default() }
    pub fn navigation(&self) -> &CSSNavigation<'a> { &self.navigation }
}

#[derive(Default)]
pub struct SetNavigationTextReturnsBuilder<'a> {
    navigation: Option<CSSNavigation<'a>>,
}

impl<'a> SetNavigationTextReturnsBuilder<'a> {
    /// The resulting CSS Navigation rule after modification.
    pub fn navigation(mut self, navigation: CSSNavigation<'a>) -> Self { self.navigation = Some(navigation); self }
    pub fn build(self) -> SetNavigationTextReturns<'a> {
        SetNavigationTextReturns {
            navigation: self.navigation.unwrap_or_default(),
        }
    }
}

impl<'a> SetNavigationTextParams<'a> { pub const METHOD: &'static str = "CSS.setNavigationText"; }

impl<'a> crate::CdpCommand<'a> for SetNavigationTextParams<'a> {
    const METHOD: &'static str = "CSS.setNavigationText";
    type Response = SetNavigationTextReturns<'a>;
}

/// Modifies the expression of a scope at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScopeTextParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetScopeTextParams<'a> {
    pub fn builder() -> SetScopeTextParamsBuilder<'a> { SetScopeTextParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}

#[derive(Default)]
pub struct SetScopeTextParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    range: Option<SourceRange>,
    text: Option<Cow<'a, str>>,
}

impl<'a> SetScopeTextParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> SetScopeTextParams<'a> {
        SetScopeTextParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
        }
    }
}

/// Modifies the expression of a scope at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScopeTextReturns<'a> {
    /// The resulting CSS Scope rule after modification.
    scope: CSSScope<'a>,
}

impl<'a> SetScopeTextReturns<'a> {
    pub fn builder() -> SetScopeTextReturnsBuilder<'a> { SetScopeTextReturnsBuilder::default() }
    pub fn scope(&self) -> &CSSScope<'a> { &self.scope }
}

#[derive(Default)]
pub struct SetScopeTextReturnsBuilder<'a> {
    scope: Option<CSSScope<'a>>,
}

impl<'a> SetScopeTextReturnsBuilder<'a> {
    /// The resulting CSS Scope rule after modification.
    pub fn scope(mut self, scope: CSSScope<'a>) -> Self { self.scope = Some(scope); self }
    pub fn build(self) -> SetScopeTextReturns<'a> {
        SetScopeTextReturns {
            scope: self.scope.unwrap_or_default(),
        }
    }
}

impl<'a> SetScopeTextParams<'a> { pub const METHOD: &'static str = "CSS.setScopeText"; }

impl<'a> crate::CdpCommand<'a> for SetScopeTextParams<'a> {
    const METHOD: &'static str = "CSS.setScopeText";
    type Response = SetScopeTextReturns<'a>;
}

/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRuleSelectorParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    selector: Cow<'a, str>,
}

impl<'a> SetRuleSelectorParams<'a> {
    pub fn builder() -> SetRuleSelectorParamsBuilder<'a> { SetRuleSelectorParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn selector(&self) -> &str { self.selector.as_ref() }
}

#[derive(Default)]
pub struct SetRuleSelectorParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    range: Option<SourceRange>,
    selector: Option<Cow<'a, str>>,
}

impl<'a> SetRuleSelectorParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn selector(mut self, selector: impl Into<Cow<'a, str>>) -> Self { self.selector = Some(selector.into()); self }
    pub fn build(self) -> SetRuleSelectorParams<'a> {
        SetRuleSelectorParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            selector: self.selector.unwrap_or_default(),
        }
    }
}

/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRuleSelectorReturns<'a> {
    /// The resulting selector list after modification.
    selectorList: SelectorList<'a>,
}

impl<'a> SetRuleSelectorReturns<'a> {
    pub fn builder() -> SetRuleSelectorReturnsBuilder<'a> { SetRuleSelectorReturnsBuilder::default() }
    pub fn selectorList(&self) -> &SelectorList<'a> { &self.selectorList }
}

#[derive(Default)]
pub struct SetRuleSelectorReturnsBuilder<'a> {
    selectorList: Option<SelectorList<'a>>,
}

impl<'a> SetRuleSelectorReturnsBuilder<'a> {
    /// The resulting selector list after modification.
    pub fn selectorList(mut self, selectorList: SelectorList<'a>) -> Self { self.selectorList = Some(selectorList); self }
    pub fn build(self) -> SetRuleSelectorReturns<'a> {
        SetRuleSelectorReturns {
            selectorList: self.selectorList.unwrap_or_default(),
        }
    }
}

impl<'a> SetRuleSelectorParams<'a> { pub const METHOD: &'static str = "CSS.setRuleSelector"; }

impl<'a> crate::CdpCommand<'a> for SetRuleSelectorParams<'a> {
    const METHOD: &'static str = "CSS.setRuleSelector";
    type Response = SetRuleSelectorReturns<'a>;
}

/// Sets the new stylesheet text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleSheetTextParams<'a> {
    styleSheetId: crate::dom::StyleSheetId<'a>,
    text: Cow<'a, str>,
}

impl<'a> SetStyleSheetTextParams<'a> {
    pub fn builder() -> SetStyleSheetTextParamsBuilder<'a> { SetStyleSheetTextParamsBuilder::default() }
    pub fn styleSheetId(&self) -> &crate::dom::StyleSheetId<'a> { &self.styleSheetId }
    pub fn text(&self) -> &str { self.text.as_ref() }
}

#[derive(Default)]
pub struct SetStyleSheetTextParamsBuilder<'a> {
    styleSheetId: Option<crate::dom::StyleSheetId<'a>>,
    text: Option<Cow<'a, str>>,
}

impl<'a> SetStyleSheetTextParamsBuilder<'a> {
    pub fn styleSheetId(mut self, styleSheetId: crate::dom::StyleSheetId<'a>) -> Self { self.styleSheetId = Some(styleSheetId); self }
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn build(self) -> SetStyleSheetTextParams<'a> {
        SetStyleSheetTextParams {
            styleSheetId: self.styleSheetId.unwrap_or_default(),
            text: self.text.unwrap_or_default(),
        }
    }
}

/// Sets the new stylesheet text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleSheetTextReturns<'a> {
    /// URL of source map associated with script (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceMapURL: Option<Cow<'a, str>>,
}

impl<'a> SetStyleSheetTextReturns<'a> {
    pub fn builder() -> SetStyleSheetTextReturnsBuilder<'a> { SetStyleSheetTextReturnsBuilder::default() }
    pub fn sourceMapURL(&self) -> Option<&str> { self.sourceMapURL.as_deref() }
}

#[derive(Default)]
pub struct SetStyleSheetTextReturnsBuilder<'a> {
    sourceMapURL: Option<Cow<'a, str>>,
}

impl<'a> SetStyleSheetTextReturnsBuilder<'a> {
    /// URL of source map associated with script (if any).
    pub fn sourceMapURL(mut self, sourceMapURL: impl Into<Cow<'a, str>>) -> Self { self.sourceMapURL = Some(sourceMapURL.into()); self }
    pub fn build(self) -> SetStyleSheetTextReturns<'a> {
        SetStyleSheetTextReturns {
            sourceMapURL: self.sourceMapURL,
        }
    }
}

impl<'a> SetStyleSheetTextParams<'a> { pub const METHOD: &'static str = "CSS.setStyleSheetText"; }

impl<'a> crate::CdpCommand<'a> for SetStyleSheetTextParams<'a> {
    const METHOD: &'static str = "CSS.setStyleSheetText";
    type Response = SetStyleSheetTextReturns<'a>;
}

/// Applies specified style edits one after another in the given order.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleTextsParams<'a> {
    edits: Vec<StyleDeclarationEdit<'a>>,
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeForPropertySyntaxValidation: Option<crate::dom::NodeId>,
}

impl<'a> SetStyleTextsParams<'a> {
    pub fn builder() -> SetStyleTextsParamsBuilder<'a> { SetStyleTextsParamsBuilder::default() }
    pub fn edits(&self) -> &[StyleDeclarationEdit<'a>] { &self.edits }
    pub fn nodeForPropertySyntaxValidation(&self) -> Option<&crate::dom::NodeId> { self.nodeForPropertySyntaxValidation.as_ref() }
}

#[derive(Default)]
pub struct SetStyleTextsParamsBuilder<'a> {
    edits: Option<Vec<StyleDeclarationEdit<'a>>>,
    nodeForPropertySyntaxValidation: Option<crate::dom::NodeId>,
}

impl<'a> SetStyleTextsParamsBuilder<'a> {
    pub fn edits(mut self, edits: Vec<StyleDeclarationEdit<'a>>) -> Self { self.edits = Some(edits); self }
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    pub fn nodeForPropertySyntaxValidation(mut self, nodeForPropertySyntaxValidation: crate::dom::NodeId) -> Self { self.nodeForPropertySyntaxValidation = Some(nodeForPropertySyntaxValidation); self }
    pub fn build(self) -> SetStyleTextsParams<'a> {
        SetStyleTextsParams {
            edits: self.edits.unwrap_or_default(),
            nodeForPropertySyntaxValidation: self.nodeForPropertySyntaxValidation,
        }
    }
}

/// Applies specified style edits one after another in the given order.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleTextsReturns<'a> {
    /// The resulting styles after modification.
    styles: Vec<CSSStyle<'a>>,
}

impl<'a> SetStyleTextsReturns<'a> {
    pub fn builder() -> SetStyleTextsReturnsBuilder<'a> { SetStyleTextsReturnsBuilder::default() }
    pub fn styles(&self) -> &[CSSStyle<'a>] { &self.styles }
}

#[derive(Default)]
pub struct SetStyleTextsReturnsBuilder<'a> {
    styles: Option<Vec<CSSStyle<'a>>>,
}

impl<'a> SetStyleTextsReturnsBuilder<'a> {
    /// The resulting styles after modification.
    pub fn styles(mut self, styles: Vec<CSSStyle<'a>>) -> Self { self.styles = Some(styles); self }
    pub fn build(self) -> SetStyleTextsReturns<'a> {
        SetStyleTextsReturns {
            styles: self.styles.unwrap_or_default(),
        }
    }
}

impl<'a> SetStyleTextsParams<'a> { pub const METHOD: &'static str = "CSS.setStyleTexts"; }

impl<'a> crate::CdpCommand<'a> for SetStyleTextsParams<'a> {
    const METHOD: &'static str = "CSS.setStyleTexts";
    type Response = SetStyleTextsReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartRuleUsageTrackingParams {}

impl StartRuleUsageTrackingParams {
    pub fn builder() -> StartRuleUsageTrackingParamsBuilder {
        StartRuleUsageTrackingParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct StartRuleUsageTrackingParamsBuilder {}

impl StartRuleUsageTrackingParamsBuilder {
    pub fn build(self) -> StartRuleUsageTrackingParams {
        StartRuleUsageTrackingParams {}
    }
}

impl StartRuleUsageTrackingParams { pub const METHOD: &'static str = "CSS.startRuleUsageTracking"; }

impl<'a> crate::CdpCommand<'a> for StartRuleUsageTrackingParams {
    const METHOD: &'static str = "CSS.startRuleUsageTracking";
    type Response = crate::EmptyReturns;
}

/// Stop tracking rule usage and return the list of rules that were used since last call to
/// 'takeCoverageDelta' (or since start of coverage instrumentation).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopRuleUsageTrackingReturns<'a> {
    ruleUsage: Vec<RuleUsage<'a>>,
}

impl<'a> StopRuleUsageTrackingReturns<'a> {
    pub fn builder() -> StopRuleUsageTrackingReturnsBuilder<'a> { StopRuleUsageTrackingReturnsBuilder::default() }
    pub fn ruleUsage(&self) -> &[RuleUsage<'a>] { &self.ruleUsage }
}

#[derive(Default)]
pub struct StopRuleUsageTrackingReturnsBuilder<'a> {
    ruleUsage: Option<Vec<RuleUsage<'a>>>,
}

impl<'a> StopRuleUsageTrackingReturnsBuilder<'a> {
    pub fn ruleUsage(mut self, ruleUsage: Vec<RuleUsage<'a>>) -> Self { self.ruleUsage = Some(ruleUsage); self }
    pub fn build(self) -> StopRuleUsageTrackingReturns<'a> {
        StopRuleUsageTrackingReturns {
            ruleUsage: self.ruleUsage.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopRuleUsageTrackingParams {}

impl StopRuleUsageTrackingParams {
    pub fn builder() -> StopRuleUsageTrackingParamsBuilder {
        StopRuleUsageTrackingParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct StopRuleUsageTrackingParamsBuilder {}

impl StopRuleUsageTrackingParamsBuilder {
    pub fn build(self) -> StopRuleUsageTrackingParams {
        StopRuleUsageTrackingParams {}
    }
}

impl StopRuleUsageTrackingParams { pub const METHOD: &'static str = "CSS.stopRuleUsageTracking"; }

impl<'a> crate::CdpCommand<'a> for StopRuleUsageTrackingParams {
    const METHOD: &'static str = "CSS.stopRuleUsageTracking";
    type Response = StopRuleUsageTrackingReturns<'a>;
}

/// Obtain list of rules that became used since last call to this method (or since start of coverage
/// instrumentation).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeCoverageDeltaReturns<'a> {
    coverage: Vec<RuleUsage<'a>>,
    /// Monotonically increasing time, in seconds.
    timestamp: f64,
}

impl<'a> TakeCoverageDeltaReturns<'a> {
    pub fn builder() -> TakeCoverageDeltaReturnsBuilder<'a> { TakeCoverageDeltaReturnsBuilder::default() }
    pub fn coverage(&self) -> &[RuleUsage<'a>] { &self.coverage }
    pub fn timestamp(&self) -> f64 { self.timestamp }
}

#[derive(Default)]
pub struct TakeCoverageDeltaReturnsBuilder<'a> {
    coverage: Option<Vec<RuleUsage<'a>>>,
    timestamp: Option<f64>,
}

impl<'a> TakeCoverageDeltaReturnsBuilder<'a> {
    pub fn coverage(mut self, coverage: Vec<RuleUsage<'a>>) -> Self { self.coverage = Some(coverage); self }
    /// Monotonically increasing time, in seconds.
    pub fn timestamp(mut self, timestamp: f64) -> Self { self.timestamp = Some(timestamp); self }
    pub fn build(self) -> TakeCoverageDeltaReturns<'a> {
        TakeCoverageDeltaReturns {
            coverage: self.coverage.unwrap_or_default(),
            timestamp: self.timestamp.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TakeCoverageDeltaParams {}

impl TakeCoverageDeltaParams {
    pub fn builder() -> TakeCoverageDeltaParamsBuilder {
        TakeCoverageDeltaParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct TakeCoverageDeltaParamsBuilder {}

impl TakeCoverageDeltaParamsBuilder {
    pub fn build(self) -> TakeCoverageDeltaParams {
        TakeCoverageDeltaParams {}
    }
}

impl TakeCoverageDeltaParams { pub const METHOD: &'static str = "CSS.takeCoverageDelta"; }

impl<'a> crate::CdpCommand<'a> for TakeCoverageDeltaParams {
    const METHOD: &'static str = "CSS.takeCoverageDelta";
    type Response = TakeCoverageDeltaReturns<'a>;
}

/// Enables/disables rendering of local CSS fonts (enabled by default).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetLocalFontsEnabledParams {
    /// Whether rendering of local fonts is enabled.
    enabled: bool,
}

impl SetLocalFontsEnabledParams {
    pub fn builder() -> SetLocalFontsEnabledParamsBuilder { SetLocalFontsEnabledParamsBuilder::default() }
    pub fn enabled(&self) -> bool { self.enabled }
}

#[derive(Default)]
pub struct SetLocalFontsEnabledParamsBuilder {
    enabled: Option<bool>,
}

impl SetLocalFontsEnabledParamsBuilder {
    /// Whether rendering of local fonts is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    pub fn build(self) -> SetLocalFontsEnabledParams {
        SetLocalFontsEnabledParams {
            enabled: self.enabled.unwrap_or_default(),
        }
    }
}

impl SetLocalFontsEnabledParams { pub const METHOD: &'static str = "CSS.setLocalFontsEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetLocalFontsEnabledParams {
    const METHOD: &'static str = "CSS.setLocalFontsEnabled";
    type Response = crate::EmptyReturns;
}
