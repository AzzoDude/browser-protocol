//! This domain exposes CSS read/write operations. All CSS objects (stylesheets, rules, and styles)
//! have an associated 'id' used in subsequent operations on the related object. Each object type has
//! a specific 'id' structure, and those are not interchangeable between objects of different kinds.
//! CSS objects can be loaded using the 'get*ForNode()' calls (which accept a DOM node id). A client
//! can also keep track of stylesheets via the 'styleSheetAdded'/'styleSheetRemoved' events and
//! subsequently load the required stylesheet contents using the 'getStyleSheet\[Text\]()' methods.


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
    #[serde(rename = "pseudoType")]
    pseudo_type: crate::dom::PseudoType,
    /// Pseudo element custom ident.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoIdentifier")]
    pseudo_identifier: Option<Cow<'a, str>>,
    /// Matches of CSS rules applicable to the pseudo style.
    matches: Vec<RuleMatch<'a>>,
}

impl<'a> PseudoElementMatches<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `pseudo_type`: Pseudo element type.
    /// * `matches`: Matches of CSS rules applicable to the pseudo style.
    pub fn builder(pseudo_type: crate::dom::PseudoType, matches: Vec<RuleMatch<'a>>) -> PseudoElementMatchesBuilder<'a> {
        PseudoElementMatchesBuilder {
            pseudo_type: pseudo_type,
            pseudo_identifier: None,
            matches: matches,
        }
    }
    /// Pseudo element type.
    pub fn pseudo_type(&self) -> &crate::dom::PseudoType { &self.pseudo_type }
    /// Pseudo element custom ident.
    pub fn pseudo_identifier(&self) -> Option<&str> { self.pseudo_identifier.as_deref() }
    /// Matches of CSS rules applicable to the pseudo style.
    pub fn matches(&self) -> &[RuleMatch<'a>] { &self.matches }
}


pub struct PseudoElementMatchesBuilder<'a> {
    pseudo_type: crate::dom::PseudoType,
    pseudo_identifier: Option<Cow<'a, str>>,
    matches: Vec<RuleMatch<'a>>,
}

impl<'a> PseudoElementMatchesBuilder<'a> {
    /// Pseudo element custom ident.
    pub fn pseudo_identifier(mut self, pseudo_identifier: impl Into<Cow<'a, str>>) -> Self { self.pseudo_identifier = Some(pseudo_identifier.into()); self }
    pub fn build(self) -> PseudoElementMatches<'a> {
        PseudoElementMatches {
            pseudo_type: self.pseudo_type,
            pseudo_identifier: self.pseudo_identifier,
            matches: self.matches,
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
    /// Creates a builder for this type with the required parameters:
    /// * `style`: The style coming from the animation.
    pub fn builder(style: CSSStyle<'a>) -> CSSAnimationStyleBuilder<'a> {
        CSSAnimationStyleBuilder {
            name: None,
            style: style,
        }
    }
    /// The name of the animation.
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    /// The style coming from the animation.
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}


pub struct CSSAnimationStyleBuilder<'a> {
    name: Option<Cow<'a, str>>,
    style: CSSStyle<'a>,
}

impl<'a> CSSAnimationStyleBuilder<'a> {
    /// The name of the animation.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> CSSAnimationStyle<'a> {
        CSSAnimationStyle {
            name: self.name,
            style: self.style,
        }
    }
}

/// Inherited CSS rule collection from ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritedStyleEntry<'a> {
    /// The ancestor node's inline style, if any, in the style inheritance chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inlineStyle")]
    inline_style: Option<CSSStyle<'a>>,
    /// Matches of CSS rules matching the ancestor node in the style inheritance chain.
    #[serde(rename = "matchedCSSRules")]
    matched_css_rules: Vec<RuleMatch<'a>>,
}

impl<'a> InheritedStyleEntry<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `matched_css_rules`: Matches of CSS rules matching the ancestor node in the style inheritance chain.
    pub fn builder(matched_css_rules: Vec<RuleMatch<'a>>) -> InheritedStyleEntryBuilder<'a> {
        InheritedStyleEntryBuilder {
            inline_style: None,
            matched_css_rules: matched_css_rules,
        }
    }
    /// The ancestor node's inline style, if any, in the style inheritance chain.
    pub fn inline_style(&self) -> Option<&CSSStyle<'a>> { self.inline_style.as_ref() }
    /// Matches of CSS rules matching the ancestor node in the style inheritance chain.
    pub fn matched_css_rules(&self) -> &[RuleMatch<'a>] { &self.matched_css_rules }
}


pub struct InheritedStyleEntryBuilder<'a> {
    inline_style: Option<CSSStyle<'a>>,
    matched_css_rules: Vec<RuleMatch<'a>>,
}

impl<'a> InheritedStyleEntryBuilder<'a> {
    /// The ancestor node's inline style, if any, in the style inheritance chain.
    pub fn inline_style(mut self, inline_style: CSSStyle<'a>) -> Self { self.inline_style = Some(inline_style); self }
    pub fn build(self) -> InheritedStyleEntry<'a> {
        InheritedStyleEntry {
            inline_style: self.inline_style,
            matched_css_rules: self.matched_css_rules,
        }
    }
}

/// Inherited CSS style collection for animated styles from ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritedAnimatedStyleEntry<'a> {
    /// Styles coming from the animations of the ancestor, if any, in the style inheritance chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "animationStyles")]
    animation_styles: Option<Vec<CSSAnimationStyle<'a>>>,
    /// The style coming from the transitions of the ancestor, if any, in the style inheritance chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitionsStyle")]
    transitions_style: Option<CSSStyle<'a>>,
}

impl<'a> InheritedAnimatedStyleEntry<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> InheritedAnimatedStyleEntryBuilder<'a> {
        InheritedAnimatedStyleEntryBuilder {
            animation_styles: None,
            transitions_style: None,
        }
    }
    /// Styles coming from the animations of the ancestor, if any, in the style inheritance chain.
    pub fn animation_styles(&self) -> Option<&[CSSAnimationStyle<'a>]> { self.animation_styles.as_deref() }
    /// The style coming from the transitions of the ancestor, if any, in the style inheritance chain.
    pub fn transitions_style(&self) -> Option<&CSSStyle<'a>> { self.transitions_style.as_ref() }
}

#[derive(Default)]
pub struct InheritedAnimatedStyleEntryBuilder<'a> {
    animation_styles: Option<Vec<CSSAnimationStyle<'a>>>,
    transitions_style: Option<CSSStyle<'a>>,
}

impl<'a> InheritedAnimatedStyleEntryBuilder<'a> {
    /// Styles coming from the animations of the ancestor, if any, in the style inheritance chain.
    pub fn animation_styles(mut self, animation_styles: Vec<CSSAnimationStyle<'a>>) -> Self { self.animation_styles = Some(animation_styles); self }
    /// The style coming from the transitions of the ancestor, if any, in the style inheritance chain.
    pub fn transitions_style(mut self, transitions_style: CSSStyle<'a>) -> Self { self.transitions_style = Some(transitions_style); self }
    pub fn build(self) -> InheritedAnimatedStyleEntry<'a> {
        InheritedAnimatedStyleEntry {
            animation_styles: self.animation_styles,
            transitions_style: self.transitions_style,
        }
    }
}

/// Inherited pseudo element matches from pseudos of an ancestor node.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritedPseudoElementMatches<'a> {
    /// Matches of pseudo styles from the pseudos of an ancestor node.
    #[serde(rename = "pseudoElements")]
    pseudo_elements: Vec<PseudoElementMatches<'a>>,
}

impl<'a> InheritedPseudoElementMatches<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `pseudo_elements`: Matches of pseudo styles from the pseudos of an ancestor node.
    pub fn builder(pseudo_elements: Vec<PseudoElementMatches<'a>>) -> InheritedPseudoElementMatchesBuilder<'a> {
        InheritedPseudoElementMatchesBuilder {
            pseudo_elements: pseudo_elements,
        }
    }
    /// Matches of pseudo styles from the pseudos of an ancestor node.
    pub fn pseudo_elements(&self) -> &[PseudoElementMatches<'a>] { &self.pseudo_elements }
}


pub struct InheritedPseudoElementMatchesBuilder<'a> {
    pseudo_elements: Vec<PseudoElementMatches<'a>>,
}

impl<'a> InheritedPseudoElementMatchesBuilder<'a> {
    pub fn build(self) -> InheritedPseudoElementMatches<'a> {
        InheritedPseudoElementMatches {
            pseudo_elements: self.pseudo_elements,
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
    #[serde(rename = "matchingSelectors")]
    matching_selectors: Vec<i64>,
}

impl<'a> RuleMatch<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `rule`: CSS rule in the match.
    /// * `matching_selectors`: Matching selector indices in the rule's selectorList selectors (0-based).
    pub fn builder(rule: CSSRule<'a>, matching_selectors: Vec<i64>) -> RuleMatchBuilder<'a> {
        RuleMatchBuilder {
            rule: rule,
            matching_selectors: matching_selectors,
        }
    }
    /// CSS rule in the match.
    pub fn rule(&self) -> &CSSRule<'a> { &self.rule }
    /// Matching selector indices in the rule's selectorList selectors (0-based).
    pub fn matching_selectors(&self) -> &[i64] { &self.matching_selectors }
}


pub struct RuleMatchBuilder<'a> {
    rule: CSSRule<'a>,
    matching_selectors: Vec<i64>,
}

impl<'a> RuleMatchBuilder<'a> {
    pub fn build(self) -> RuleMatch<'a> {
        RuleMatch {
            rule: self.rule,
            matching_selectors: self.matching_selectors,
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
    /// Creates a builder for this type with the required parameters:
    /// * `text`: Value text.
    pub fn builder(text: impl Into<Cow<'a, str>>) -> ProtocolValueBuilder<'a> {
        ProtocolValueBuilder {
            text: text.into(),
            range: None,
            specificity: None,
        }
    }
    /// Value text.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// Value range in the underlying resource (if available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    /// Specificity of the selector.
    pub fn specificity(&self) -> Option<&Specificity> { self.specificity.as_ref() }
}


pub struct ProtocolValueBuilder<'a> {
    text: Cow<'a, str>,
    range: Option<SourceRange>,
    specificity: Option<Specificity>,
}

impl<'a> ProtocolValueBuilder<'a> {
    /// Value range in the underlying resource (if available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Specificity of the selector.
    pub fn specificity(mut self, specificity: Specificity) -> Self { self.specificity = Some(specificity); self }
    pub fn build(self) -> ProtocolValue<'a> {
        ProtocolValue {
            text: self.text,
            range: self.range,
            specificity: self.specificity,
        }
    }
}

/// Specificity:
/// <https://drafts.csswg.org/selectors/#specificity-rules>

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
    /// Creates a builder for this type with the required parameters:
    /// * `a`: The a component, which represents the number of ID selectors.
    /// * `b`: The b component, which represents the number of class selectors, attributes selectors, and pseudo-classes.
    /// * `c`: The c component, which represents the number of type selectors and pseudo-elements.
    pub fn builder(a: i64, b: i64, c: i64) -> SpecificityBuilder {
        SpecificityBuilder {
            a: a,
            b: b,
            c: c,
        }
    }
    /// The a component, which represents the number of ID selectors.
    pub fn a(&self) -> i64 { self.a }
    /// The b component, which represents the number of class selectors, attributes selectors, and
    /// pseudo-classes.
    pub fn b(&self) -> i64 { self.b }
    /// The c component, which represents the number of type selectors and pseudo-elements.
    pub fn c(&self) -> i64 { self.c }
}


pub struct SpecificityBuilder {
    a: i64,
    b: i64,
    c: i64,
}

impl SpecificityBuilder {
    pub fn build(self) -> Specificity {
        Specificity {
            a: self.a,
            b: self.b,
            c: self.c,
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
    /// Creates a builder for this type with the required parameters:
    /// * `selectors`: Selectors in the list.
    /// * `text`: Rule selector text.
    pub fn builder(selectors: Vec<ProtocolValue<'a>>, text: impl Into<Cow<'a, str>>) -> SelectorListBuilder<'a> {
        SelectorListBuilder {
            selectors: selectors,
            text: text.into(),
        }
    }
    /// Selectors in the list.
    pub fn selectors(&self) -> &[ProtocolValue<'a>] { &self.selectors }
    /// Rule selector text.
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct SelectorListBuilder<'a> {
    selectors: Vec<ProtocolValue<'a>>,
    text: Cow<'a, str>,
}

impl<'a> SelectorListBuilder<'a> {
    pub fn build(self) -> SelectorList<'a> {
        SelectorList {
            selectors: self.selectors,
            text: self.text,
        }
    }
}

/// CSS stylesheet metainformation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSStyleSheetHeader<'a> {
    /// The stylesheet identifier.
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    /// Owner frame identifier.
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
    /// Stylesheet resource URL. Empty if this is a constructed stylesheet created using
    /// new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported
    /// as a CSS module script).
    #[serde(rename = "sourceURL")]
    source_url: Cow<'a, str>,
    /// URL of source map associated with the stylesheet (if any).
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceMapURL")]
    source_map_url: Option<Cow<'a, str>>,
    /// Stylesheet origin.
    origin: StyleSheetOrigin,
    /// Stylesheet title.
    title: Cow<'a, str>,
    /// The backend id for the owner node of the stylesheet.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ownerNode")]
    owner_node: Option<crate::dom::BackendNodeId>,
    /// Denotes whether the stylesheet is disabled.
    disabled: bool,
    /// Whether the sourceURL field value comes from the sourceURL comment.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasSourceURL")]
    has_source_url: Option<bool>,
    /// Whether this stylesheet is created for STYLE tag by parser. This flag is not set for
    /// document.written STYLE tags.
    #[serde(rename = "isInline")]
    is_inline: bool,
    /// Whether this stylesheet is mutable. Inline stylesheets become mutable
    /// after they have been modified via CSSOM API.
    /// '\<link\>' element's stylesheets become mutable only if DevTools modifies them.
    /// Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.
    #[serde(rename = "isMutable")]
    is_mutable: bool,
    /// True if this stylesheet is created through new CSSStyleSheet() or imported as a
    /// CSS module script.
    #[serde(rename = "isConstructed")]
    is_constructed: bool,
    /// Line offset of the stylesheet within the resource (zero based).
    #[serde(rename = "startLine")]
    start_line: f64,
    /// Column offset of the stylesheet within the resource (zero based).
    #[serde(rename = "startColumn")]
    start_column: f64,
    /// Size of the content (in characters).
    length: f64,
    /// Line offset of the end of the stylesheet within the resource (zero based).
    #[serde(rename = "endLine")]
    end_line: f64,
    /// Column offset of the end of the stylesheet within the resource (zero based).
    #[serde(rename = "endColumn")]
    end_column: f64,
    /// If the style sheet was loaded from a network resource, this indicates when the resource failed to load
    #[serde(skip_serializing_if = "Option::is_none", rename = "loadingFailed")]
    loading_failed: Option<bool>,
}

impl<'a> CSSStyleSheetHeader<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: The stylesheet identifier.
    /// * `frame_id`: Owner frame identifier.
    /// * `source_url`: Stylesheet resource URL. Empty if this is a constructed stylesheet created using new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported as a CSS module script).
    /// * `origin`: Stylesheet origin.
    /// * `title`: Stylesheet title.
    /// * `disabled`: Denotes whether the stylesheet is disabled.
    /// * `is_inline`: Whether this stylesheet is created for STYLE tag by parser. This flag is not set for document.written STYLE tags.
    /// * `is_mutable`: Whether this stylesheet is mutable. Inline stylesheets become mutable after they have been modified via CSSOM API. `\<link\>` element's stylesheets become mutable only if DevTools modifies them. Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.
    /// * `is_constructed`: True if this stylesheet is created through new CSSStyleSheet() or imported as a CSS module script.
    /// * `start_line`: Line offset of the stylesheet within the resource (zero based).
    /// * `start_column`: Column offset of the stylesheet within the resource (zero based).
    /// * `length`: Size of the content (in characters).
    /// * `end_line`: Line offset of the end of the stylesheet within the resource (zero based).
    /// * `end_column`: Column offset of the end of the stylesheet within the resource (zero based).
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, frame_id: crate::page::FrameId<'a>, source_url: impl Into<Cow<'a, str>>, origin: impl Into<StyleSheetOrigin>, title: impl Into<Cow<'a, str>>, disabled: bool, is_inline: bool, is_mutable: bool, is_constructed: bool, start_line: f64, start_column: f64, length: f64, end_line: f64, end_column: f64) -> CSSStyleSheetHeaderBuilder<'a> {
        CSSStyleSheetHeaderBuilder {
            style_sheet_id: style_sheet_id,
            frame_id: frame_id,
            source_url: source_url.into(),
            source_map_url: None,
            origin: origin.into(),
            title: title.into(),
            owner_node: None,
            disabled: disabled,
            has_source_url: None,
            is_inline: is_inline,
            is_mutable: is_mutable,
            is_constructed: is_constructed,
            start_line: start_line,
            start_column: start_column,
            length: length,
            end_line: end_line,
            end_column: end_column,
            loading_failed: None,
        }
    }
    /// The stylesheet identifier.
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    /// Owner frame identifier.
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
    /// Stylesheet resource URL. Empty if this is a constructed stylesheet created using
    /// new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported
    /// as a CSS module script).
    pub fn source_url(&self) -> &str { self.source_url.as_ref() }
    /// URL of source map associated with the stylesheet (if any).
    pub fn source_map_url(&self) -> Option<&str> { self.source_map_url.as_deref() }
    /// Stylesheet origin.
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    /// Stylesheet title.
    pub fn title(&self) -> &str { self.title.as_ref() }
    /// The backend id for the owner node of the stylesheet.
    pub fn owner_node(&self) -> Option<&crate::dom::BackendNodeId> { self.owner_node.as_ref() }
    /// Denotes whether the stylesheet is disabled.
    pub fn disabled(&self) -> bool { self.disabled }
    /// Whether the sourceURL field value comes from the sourceURL comment.
    pub fn has_source_url(&self) -> Option<bool> { self.has_source_url }
    /// Whether this stylesheet is created for STYLE tag by parser. This flag is not set for
    /// document.written STYLE tags.
    pub fn is_inline(&self) -> bool { self.is_inline }
    /// Whether this stylesheet is mutable. Inline stylesheets become mutable
    /// after they have been modified via CSSOM API.
    /// '\<link\>' element's stylesheets become mutable only if DevTools modifies them.
    /// Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.
    pub fn is_mutable(&self) -> bool { self.is_mutable }
    /// True if this stylesheet is created through new CSSStyleSheet() or imported as a
    /// CSS module script.
    pub fn is_constructed(&self) -> bool { self.is_constructed }
    /// Line offset of the stylesheet within the resource (zero based).
    pub fn start_line(&self) -> f64 { self.start_line }
    /// Column offset of the stylesheet within the resource (zero based).
    pub fn start_column(&self) -> f64 { self.start_column }
    /// Size of the content (in characters).
    pub fn length(&self) -> f64 { self.length }
    /// Line offset of the end of the stylesheet within the resource (zero based).
    pub fn end_line(&self) -> f64 { self.end_line }
    /// Column offset of the end of the stylesheet within the resource (zero based).
    pub fn end_column(&self) -> f64 { self.end_column }
    /// If the style sheet was loaded from a network resource, this indicates when the resource failed to load
    pub fn loading_failed(&self) -> Option<bool> { self.loading_failed }
}


pub struct CSSStyleSheetHeaderBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    frame_id: crate::page::FrameId<'a>,
    source_url: Cow<'a, str>,
    source_map_url: Option<Cow<'a, str>>,
    origin: StyleSheetOrigin,
    title: Cow<'a, str>,
    owner_node: Option<crate::dom::BackendNodeId>,
    disabled: bool,
    has_source_url: Option<bool>,
    is_inline: bool,
    is_mutable: bool,
    is_constructed: bool,
    start_line: f64,
    start_column: f64,
    length: f64,
    end_line: f64,
    end_column: f64,
    loading_failed: Option<bool>,
}

impl<'a> CSSStyleSheetHeaderBuilder<'a> {
    /// URL of source map associated with the stylesheet (if any).
    pub fn source_map_url(mut self, source_map_url: impl Into<Cow<'a, str>>) -> Self { self.source_map_url = Some(source_map_url.into()); self }
    /// The backend id for the owner node of the stylesheet.
    pub fn owner_node(mut self, owner_node: crate::dom::BackendNodeId) -> Self { self.owner_node = Some(owner_node); self }
    /// Whether the sourceURL field value comes from the sourceURL comment.
    pub fn has_source_url(mut self, has_source_url: bool) -> Self { self.has_source_url = Some(has_source_url); self }
    /// If the style sheet was loaded from a network resource, this indicates when the resource failed to load
    pub fn loading_failed(mut self, loading_failed: bool) -> Self { self.loading_failed = Some(loading_failed); self }
    pub fn build(self) -> CSSStyleSheetHeader<'a> {
        CSSStyleSheetHeader {
            style_sheet_id: self.style_sheet_id,
            frame_id: self.frame_id,
            source_url: self.source_url,
            source_map_url: self.source_map_url,
            origin: self.origin,
            title: self.title,
            owner_node: self.owner_node,
            disabled: self.disabled,
            has_source_url: self.has_source_url,
            is_inline: self.is_inline,
            is_mutable: self.is_mutable,
            is_constructed: self.is_constructed,
            start_line: self.start_line,
            start_column: self.start_column,
            length: self.length,
            end_line: self.end_line,
            end_column: self.end_column,
            loading_failed: self.loading_failed,
        }
    }
}

/// CSS rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Rule selector data.
    #[serde(rename = "selectorList")]
    selector_list: SelectorList<'a>,
    /// Array of selectors from ancestor style rules, sorted by distance from the current rule.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nestingSelectors")]
    nesting_selectors: Option<Vec<Cow<'a, str>>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated style declaration.
    style: CSSStyle<'a>,
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originTreeScopeNodeId")]
    origin_tree_scope_node_id: Option<crate::dom::BackendNodeId>,
    /// Media list array (for rules involving media queries). The array enumerates media queries
    /// starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    media: Option<Vec<CSSMedia<'a>>>,
    /// Container query list array (for rules involving container queries).
    /// The array enumerates container queries starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerQueries")]
    container_queries: Option<Vec<CSSContainerQuery<'a>>>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "ruleTypes")]
    rule_types: Option<Vec<CSSRuleType>>,
    /// @starting-style CSS at-rule array.
    /// The array enumerates @starting-style at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none", rename = "startingStyles")]
    starting_styles: Option<Vec<CSSStartingStyle<'a>>>,
    /// @navigation CSS at-rule array.
    /// The array enumerates @navigation at-rules starting with the innermost one, going outwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    navigations: Option<Vec<CSSNavigation<'a>>>,
}

impl<'a> CSSRule<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `selector_list`: Rule selector data.
    /// * `origin`: Parent stylesheet's origin.
    /// * `style`: Associated style declaration.
    pub fn builder(selector_list: SelectorList<'a>, origin: impl Into<StyleSheetOrigin>, style: CSSStyle<'a>) -> CSSRuleBuilder<'a> {
        CSSRuleBuilder {
            style_sheet_id: None,
            selector_list: selector_list,
            nesting_selectors: None,
            origin: origin.into(),
            style: style,
            origin_tree_scope_node_id: None,
            media: None,
            container_queries: None,
            supports: None,
            layers: None,
            scopes: None,
            rule_types: None,
            starting_styles: None,
            navigations: None,
        }
    }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// Rule selector data.
    pub fn selector_list(&self) -> &SelectorList<'a> { &self.selector_list }
    /// Array of selectors from ancestor style rules, sorted by distance from the current rule.
    pub fn nesting_selectors(&self) -> Option<&[Cow<'a, str>]> { self.nesting_selectors.as_deref() }
    /// Parent stylesheet's origin.
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    /// Associated style declaration.
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    pub fn origin_tree_scope_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.origin_tree_scope_node_id.as_ref() }
    /// Media list array (for rules involving media queries). The array enumerates media queries
    /// starting with the innermost one, going outwards.
    pub fn media(&self) -> Option<&[CSSMedia<'a>]> { self.media.as_deref() }
    /// Container query list array (for rules involving container queries).
    /// The array enumerates container queries starting with the innermost one, going outwards.
    pub fn container_queries(&self) -> Option<&[CSSContainerQuery<'a>]> { self.container_queries.as_deref() }
    /// @supports CSS at-rule array.
    /// The array enumerates @supports at-rules starting with the innermost one, going outwards.
    pub fn supports(&self) -> Option<&[CSSSupports<'a>]> { self.supports.as_deref() }
    /// Cascade layer array. Contains the layer hierarchy that this rule belongs to starting
    /// with the innermost layer and going outwards.
    pub fn layers(&self) -> Option<&[CSSLayer<'a>]> { self.layers.as_deref() }
    /// @scope CSS at-rule array.
    /// The array enumerates @scope at-rules starting with the innermost one, going outwards.
    pub fn scopes(&self) -> Option<&[CSSScope<'a>]> { self.scopes.as_deref() }
    /// The array keeps the types of ancestor CSSRules from the innermost going outwards.
    pub fn rule_types(&self) -> Option<&[CSSRuleType]> { self.rule_types.as_deref() }
    /// @starting-style CSS at-rule array.
    /// The array enumerates @starting-style at-rules starting with the innermost one, going outwards.
    pub fn starting_styles(&self) -> Option<&[CSSStartingStyle<'a>]> { self.starting_styles.as_deref() }
    /// @navigation CSS at-rule array.
    /// The array enumerates @navigation at-rules starting with the innermost one, going outwards.
    pub fn navigations(&self) -> Option<&[CSSNavigation<'a>]> { self.navigations.as_deref() }
}


pub struct CSSRuleBuilder<'a> {
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    selector_list: SelectorList<'a>,
    nesting_selectors: Option<Vec<Cow<'a, str>>>,
    origin: StyleSheetOrigin,
    style: CSSStyle<'a>,
    origin_tree_scope_node_id: Option<crate::dom::BackendNodeId>,
    media: Option<Vec<CSSMedia<'a>>>,
    container_queries: Option<Vec<CSSContainerQuery<'a>>>,
    supports: Option<Vec<CSSSupports<'a>>>,
    layers: Option<Vec<CSSLayer<'a>>>,
    scopes: Option<Vec<CSSScope<'a>>>,
    rule_types: Option<Vec<CSSRuleType>>,
    starting_styles: Option<Vec<CSSStartingStyle<'a>>>,
    navigations: Option<Vec<CSSNavigation<'a>>>,
}

impl<'a> CSSRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    /// Array of selectors from ancestor style rules, sorted by distance from the current rule.
    pub fn nesting_selectors(mut self, nesting_selectors: Vec<Cow<'a, str>>) -> Self { self.nesting_selectors = Some(nesting_selectors); self }
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    pub fn origin_tree_scope_node_id(mut self, origin_tree_scope_node_id: crate::dom::BackendNodeId) -> Self { self.origin_tree_scope_node_id = Some(origin_tree_scope_node_id); self }
    /// Media list array (for rules involving media queries). The array enumerates media queries
    /// starting with the innermost one, going outwards.
    pub fn media(mut self, media: Vec<CSSMedia<'a>>) -> Self { self.media = Some(media); self }
    /// Container query list array (for rules involving container queries).
    /// The array enumerates container queries starting with the innermost one, going outwards.
    pub fn container_queries(mut self, container_queries: Vec<CSSContainerQuery<'a>>) -> Self { self.container_queries = Some(container_queries); self }
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
    pub fn rule_types(mut self, rule_types: Vec<CSSRuleType>) -> Self { self.rule_types = Some(rule_types); self }
    /// @starting-style CSS at-rule array.
    /// The array enumerates @starting-style at-rules starting with the innermost one, going outwards.
    pub fn starting_styles(mut self, starting_styles: Vec<CSSStartingStyle<'a>>) -> Self { self.starting_styles = Some(starting_styles); self }
    /// @navigation CSS at-rule array.
    /// The array enumerates @navigation at-rules starting with the innermost one, going outwards.
    pub fn navigations(mut self, navigations: Vec<CSSNavigation<'a>>) -> Self { self.navigations = Some(navigations); self }
    pub fn build(self) -> CSSRule<'a> {
        CSSRule {
            style_sheet_id: self.style_sheet_id,
            selector_list: self.selector_list,
            nesting_selectors: self.nesting_selectors,
            origin: self.origin,
            style: self.style,
            origin_tree_scope_node_id: self.origin_tree_scope_node_id,
            media: self.media,
            container_queries: self.container_queries,
            supports: self.supports,
            layers: self.layers,
            scopes: self.scopes,
            rule_types: self.rule_types,
            starting_styles: self.starting_styles,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    /// Offset of the start of the rule (including selector) from the beginning of the stylesheet.
    #[serde(rename = "startOffset")]
    start_offset: f64,
    /// Offset of the end of the rule body from the beginning of the stylesheet.
    #[serde(rename = "endOffset")]
    end_offset: f64,
    /// Indicates whether the rule was actually used by some element in the page.
    used: bool,
}

impl<'a> RuleUsage<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: The css style sheet identifier (absent for user agent stylesheet and user-specified stylesheet rules) this rule came from.
    /// * `start_offset`: Offset of the start of the rule (including selector) from the beginning of the stylesheet.
    /// * `end_offset`: Offset of the end of the rule body from the beginning of the stylesheet.
    /// * `used`: Indicates whether the rule was actually used by some element in the page.
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, start_offset: f64, end_offset: f64, used: bool) -> RuleUsageBuilder<'a> {
        RuleUsageBuilder {
            style_sheet_id: style_sheet_id,
            start_offset: start_offset,
            end_offset: end_offset,
            used: used,
        }
    }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    /// Offset of the start of the rule (including selector) from the beginning of the stylesheet.
    pub fn start_offset(&self) -> f64 { self.start_offset }
    /// Offset of the end of the rule body from the beginning of the stylesheet.
    pub fn end_offset(&self) -> f64 { self.end_offset }
    /// Indicates whether the rule was actually used by some element in the page.
    pub fn used(&self) -> bool { self.used }
}


pub struct RuleUsageBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    start_offset: f64,
    end_offset: f64,
    used: bool,
}

impl<'a> RuleUsageBuilder<'a> {
    pub fn build(self) -> RuleUsage<'a> {
        RuleUsage {
            style_sheet_id: self.style_sheet_id,
            start_offset: self.start_offset,
            end_offset: self.end_offset,
            used: self.used,
        }
    }
}

/// Text range within a resource. All numbers are zero-based.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceRange {
    /// Start line of range.
    #[serde(rename = "startLine")]
    start_line: i64,
    /// Start column of range (inclusive).
    #[serde(rename = "startColumn")]
    start_column: i64,
    /// End line of range
    #[serde(rename = "endLine")]
    end_line: i64,
    /// End column of range (exclusive).
    #[serde(rename = "endColumn")]
    end_column: i64,
}

impl SourceRange {
    /// Creates a builder for this type with the required parameters:
    /// * `start_line`: Start line of range.
    /// * `start_column`: Start column of range (inclusive).
    /// * `end_line`: End line of range
    /// * `end_column`: End column of range (exclusive).
    pub fn builder(start_line: i64, start_column: i64, end_line: i64, end_column: i64) -> SourceRangeBuilder {
        SourceRangeBuilder {
            start_line: start_line,
            start_column: start_column,
            end_line: end_line,
            end_column: end_column,
        }
    }
    /// Start line of range.
    pub fn start_line(&self) -> i64 { self.start_line }
    /// Start column of range (inclusive).
    pub fn start_column(&self) -> i64 { self.start_column }
    /// End line of range
    pub fn end_line(&self) -> i64 { self.end_line }
    /// End column of range (exclusive).
    pub fn end_column(&self) -> i64 { self.end_column }
}


pub struct SourceRangeBuilder {
    start_line: i64,
    start_column: i64,
    end_line: i64,
    end_column: i64,
}

impl SourceRangeBuilder {
    pub fn build(self) -> SourceRange {
        SourceRange {
            start_line: self.start_line,
            start_column: self.start_column,
            end_line: self.end_line,
            end_column: self.end_column,
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Shorthand name.
    /// * `value`: Shorthand value.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> ShorthandEntryBuilder<'a> {
        ShorthandEntryBuilder {
            name: name.into(),
            value: value.into(),
            important: None,
        }
    }
    /// Shorthand name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Shorthand value.
    pub fn value(&self) -> &str { self.value.as_ref() }
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    pub fn important(&self) -> Option<bool> { self.important }
}


pub struct ShorthandEntryBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    important: Option<bool>,
}

impl<'a> ShorthandEntryBuilder<'a> {
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    pub fn important(mut self, important: bool) -> Self { self.important = Some(important); self }
    pub fn build(self) -> ShorthandEntry<'a> {
        ShorthandEntry {
            name: self.name,
            value: self.value,
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Computed style property name.
    /// * `value`: Computed style property value.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> CSSComputedStylePropertyBuilder<'a> {
        CSSComputedStylePropertyBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    /// Computed style property name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Computed style property value.
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct CSSComputedStylePropertyBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> CSSComputedStylePropertyBuilder<'a> {
    pub fn build(self) -> CSSComputedStyleProperty<'a> {
        CSSComputedStyleProperty {
            name: self.name,
            value: self.value,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComputedStyleExtraFields {
    /// Returns whether or not this node is being rendered with base appearance,
    /// which happens when it has its appearance property set to base/base-select
    /// or it is in the subtree of an element being rendered with base appearance.
    #[serde(rename = "isAppearanceBase")]
    is_appearance_base: bool,
}

impl ComputedStyleExtraFields {
    /// Creates a builder for this type with the required parameters:
    /// * `is_appearance_base`: Returns whether or not this node is being rendered with base appearance, which happens when it has its appearance property set to base/base-select or it is in the subtree of an element being rendered with base appearance.
    pub fn builder(is_appearance_base: bool) -> ComputedStyleExtraFieldsBuilder {
        ComputedStyleExtraFieldsBuilder {
            is_appearance_base: is_appearance_base,
        }
    }
    /// Returns whether or not this node is being rendered with base appearance,
    /// which happens when it has its appearance property set to base/base-select
    /// or it is in the subtree of an element being rendered with base appearance.
    pub fn is_appearance_base(&self) -> bool { self.is_appearance_base }
}


pub struct ComputedStyleExtraFieldsBuilder {
    is_appearance_base: bool,
}

impl ComputedStyleExtraFieldsBuilder {
    pub fn build(self) -> ComputedStyleExtraFields {
        ComputedStyleExtraFields {
            is_appearance_base: self.is_appearance_base,
        }
    }
}

/// CSS style representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSStyle<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// CSS properties in the style.
    #[serde(rename = "cssProperties")]
    css_properties: Vec<CSSProperty<'a>>,
    /// Computed values for all shorthands found in the style.
    #[serde(rename = "shorthandEntries")]
    shorthand_entries: Vec<ShorthandEntry<'a>>,
    /// Style declaration text (if available).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssText")]
    css_text: Option<Cow<'a, str>>,
    /// Style declaration range in the enclosing stylesheet (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
}

impl<'a> CSSStyle<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `css_properties`: CSS properties in the style.
    /// * `shorthand_entries`: Computed values for all shorthands found in the style.
    pub fn builder(css_properties: Vec<CSSProperty<'a>>, shorthand_entries: Vec<ShorthandEntry<'a>>) -> CSSStyleBuilder<'a> {
        CSSStyleBuilder {
            style_sheet_id: None,
            css_properties: css_properties,
            shorthand_entries: shorthand_entries,
            css_text: None,
            range: None,
        }
    }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// CSS properties in the style.
    pub fn css_properties(&self) -> &[CSSProperty<'a>] { &self.css_properties }
    /// Computed values for all shorthands found in the style.
    pub fn shorthand_entries(&self) -> &[ShorthandEntry<'a>] { &self.shorthand_entries }
    /// Style declaration text (if available).
    pub fn css_text(&self) -> Option<&str> { self.css_text.as_deref() }
    /// Style declaration range in the enclosing stylesheet (if available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
}


pub struct CSSStyleBuilder<'a> {
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    css_properties: Vec<CSSProperty<'a>>,
    shorthand_entries: Vec<ShorthandEntry<'a>>,
    css_text: Option<Cow<'a, str>>,
    range: Option<SourceRange>,
}

impl<'a> CSSStyleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    /// Style declaration text (if available).
    pub fn css_text(mut self, css_text: impl Into<Cow<'a, str>>) -> Self { self.css_text = Some(css_text.into()); self }
    /// Style declaration range in the enclosing stylesheet (if available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    pub fn build(self) -> CSSStyle<'a> {
        CSSStyle {
            style_sheet_id: self.style_sheet_id,
            css_properties: self.css_properties,
            shorthand_entries: self.shorthand_entries,
            css_text: self.css_text,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "parsedOk")]
    parsed_ok: Option<bool>,
    /// Whether the property is disabled by the user (present for source-based properties only).
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// The entire property range in the enclosing style declaration (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Parsed longhand components of this property if it is a shorthand.
    /// This field will be empty if the given property is not a shorthand.
    #[serde(skip_serializing_if = "Option::is_none", rename = "longhandProperties")]
    longhand_properties: Option<Vec<Box<CSSProperty<'a>>>>,
}

impl<'a> CSSProperty<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: The property name.
    /// * `value`: The property value.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> CSSPropertyBuilder<'a> {
        CSSPropertyBuilder {
            name: name.into(),
            value: value.into(),
            important: None,
            implicit: None,
            text: None,
            parsed_ok: None,
            disabled: None,
            range: None,
            longhand_properties: None,
        }
    }
    /// The property name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// The property value.
    pub fn value(&self) -> &str { self.value.as_ref() }
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    pub fn important(&self) -> Option<bool> { self.important }
    /// Whether the property is implicit (implies 'false' if absent).
    pub fn implicit(&self) -> Option<bool> { self.implicit }
    /// The full property text as specified in the style.
    pub fn text(&self) -> Option<&str> { self.text.as_deref() }
    /// Whether the property is understood by the browser (implies 'true' if absent).
    pub fn parsed_ok(&self) -> Option<bool> { self.parsed_ok }
    /// Whether the property is disabled by the user (present for source-based properties only).
    pub fn disabled(&self) -> Option<bool> { self.disabled }
    /// The entire property range in the enclosing style declaration (if available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    /// Parsed longhand components of this property if it is a shorthand.
    /// This field will be empty if the given property is not a shorthand.
    pub fn longhand_properties(&self) -> Option<&[Box<CSSProperty<'a>>]> { self.longhand_properties.as_deref() }
}


pub struct CSSPropertyBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    important: Option<bool>,
    implicit: Option<bool>,
    text: Option<Cow<'a, str>>,
    parsed_ok: Option<bool>,
    disabled: Option<bool>,
    range: Option<SourceRange>,
    longhand_properties: Option<Vec<Box<CSSProperty<'a>>>>,
}

impl<'a> CSSPropertyBuilder<'a> {
    /// Whether the property has "!important" annotation (implies 'false' if absent).
    pub fn important(mut self, important: bool) -> Self { self.important = Some(important); self }
    /// Whether the property is implicit (implies 'false' if absent).
    pub fn implicit(mut self, implicit: bool) -> Self { self.implicit = Some(implicit); self }
    /// The full property text as specified in the style.
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// Whether the property is understood by the browser (implies 'true' if absent).
    pub fn parsed_ok(mut self, parsed_ok: bool) -> Self { self.parsed_ok = Some(parsed_ok); self }
    /// Whether the property is disabled by the user (present for source-based properties only).
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = Some(disabled); self }
    /// The entire property range in the enclosing style declaration (if available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Parsed longhand components of this property if it is a shorthand.
    /// This field will be empty if the given property is not a shorthand.
    pub fn longhand_properties(mut self, longhand_properties: Vec<Box<CSSProperty<'a>>>) -> Self { self.longhand_properties = Some(longhand_properties); self }
    pub fn build(self) -> CSSProperty<'a> {
        CSSProperty {
            name: self.name,
            value: self.value,
            important: self.important,
            implicit: self.implicit,
            text: self.text,
            parsed_ok: self.parsed_ok,
            disabled: self.disabled,
            range: self.range,
            longhand_properties: self.longhand_properties,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceURL")]
    source_url: Option<Cow<'a, str>>,
    /// The associated rule (@media or @import) header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Array of media queries.
    #[serde(skip_serializing_if = "Option::is_none", rename = "mediaList")]
    media_list: Option<Vec<MediaQuery<'a>>>,
}

impl<'a> CSSMedia<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `text`: Media query text.
    /// * `source`: Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline stylesheet's STYLE tag.
    pub fn builder(text: impl Into<Cow<'a, str>>, source: impl Into<Cow<'a, str>>) -> CSSMediaBuilder<'a> {
        CSSMediaBuilder {
            text: text.into(),
            source: source.into(),
            source_url: None,
            range: None,
            style_sheet_id: None,
            media_list: None,
        }
    }
    /// Media query text.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if
    /// specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked
    /// stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline
    /// stylesheet's STYLE tag.
    pub fn source(&self) -> &str { self.source.as_ref() }
    /// URL of the document containing the media query description.
    pub fn source_url(&self) -> Option<&str> { self.source_url.as_deref() }
    /// The associated rule (@media or @import) header range in the enclosing stylesheet (if
    /// available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// Array of media queries.
    pub fn media_list(&self) -> Option<&[MediaQuery<'a>]> { self.media_list.as_deref() }
}


pub struct CSSMediaBuilder<'a> {
    text: Cow<'a, str>,
    source: Cow<'a, str>,
    source_url: Option<Cow<'a, str>>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    media_list: Option<Vec<MediaQuery<'a>>>,
}

impl<'a> CSSMediaBuilder<'a> {
    /// URL of the document containing the media query description.
    pub fn source_url(mut self, source_url: impl Into<Cow<'a, str>>) -> Self { self.source_url = Some(source_url.into()); self }
    /// The associated rule (@media or @import) header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    /// Array of media queries.
    pub fn media_list(mut self, media_list: Vec<MediaQuery<'a>>) -> Self { self.media_list = Some(media_list); self }
    pub fn build(self) -> CSSMedia<'a> {
        CSSMedia {
            text: self.text,
            source: self.source,
            source_url: self.source_url,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
            media_list: self.media_list,
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
    /// Creates a builder for this type with the required parameters:
    /// * `expressions`: Array of media query expressions.
    /// * `active`: Whether the media query condition is satisfied.
    pub fn builder(expressions: Vec<MediaQueryExpression<'a>>, active: bool) -> MediaQueryBuilder<'a> {
        MediaQueryBuilder {
            expressions: expressions,
            active: active,
        }
    }
    /// Array of media query expressions.
    pub fn expressions(&self) -> &[MediaQueryExpression<'a>] { &self.expressions }
    /// Whether the media query condition is satisfied.
    pub fn active(&self) -> bool { self.active }
}


pub struct MediaQueryBuilder<'a> {
    expressions: Vec<MediaQueryExpression<'a>>,
    active: bool,
}

impl<'a> MediaQueryBuilder<'a> {
    pub fn build(self) -> MediaQuery<'a> {
        MediaQuery {
            expressions: self.expressions,
            active: self.active,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "valueRange")]
    value_range: Option<SourceRange>,
    /// Computed length of media query expression (if applicable).
    #[serde(skip_serializing_if = "Option::is_none", rename = "computedLength")]
    computed_length: Option<f64>,
}

impl<'a> MediaQueryExpression<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `value`: Media query expression value.
    /// * `unit`: Media query expression units.
    /// * `feature`: Media query expression feature.
    pub fn builder(value: f64, unit: impl Into<Cow<'a, str>>, feature: impl Into<Cow<'a, str>>) -> MediaQueryExpressionBuilder<'a> {
        MediaQueryExpressionBuilder {
            value: value,
            unit: unit.into(),
            feature: feature.into(),
            value_range: None,
            computed_length: None,
        }
    }
    /// Media query expression value.
    pub fn value(&self) -> f64 { self.value }
    /// Media query expression units.
    pub fn unit(&self) -> &str { self.unit.as_ref() }
    /// Media query expression feature.
    pub fn feature(&self) -> &str { self.feature.as_ref() }
    /// The associated range of the value text in the enclosing stylesheet (if available).
    pub fn value_range(&self) -> Option<&SourceRange> { self.value_range.as_ref() }
    /// Computed length of media query expression (if applicable).
    pub fn computed_length(&self) -> Option<f64> { self.computed_length }
}


pub struct MediaQueryExpressionBuilder<'a> {
    value: f64,
    unit: Cow<'a, str>,
    feature: Cow<'a, str>,
    value_range: Option<SourceRange>,
    computed_length: Option<f64>,
}

impl<'a> MediaQueryExpressionBuilder<'a> {
    /// The associated range of the value text in the enclosing stylesheet (if available).
    pub fn value_range(mut self, value_range: SourceRange) -> Self { self.value_range = Some(value_range); self }
    /// Computed length of media query expression (if applicable).
    pub fn computed_length(mut self, computed_length: f64) -> Self { self.computed_length = Some(computed_length); self }
    pub fn build(self) -> MediaQueryExpression<'a> {
        MediaQueryExpression {
            value: self.value,
            unit: self.unit,
            feature: self.feature,
            value_range: self.value_range,
            computed_length: self.computed_length,
        }
    }
}

/// CSS container query rule descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSContainerQuery<'a> {
    /// Container query text.
    /// Contains the query part without the container name for a single query.
    /// Deprecated in favor of conditionText which contains the full prelude
    /// after @container.
    text: Cow<'a, str>,
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<SourceRange>,
    /// Identifier of the stylesheet containing this object (if exists).
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Optional name for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    /// Optional physical axes queried for the container.
    #[serde(skip_serializing_if = "Option::is_none", rename = "physicalAxes")]
    physical_axes: Option<crate::dom::PhysicalAxes>,
    /// Optional logical axes queried for the container.
    #[serde(skip_serializing_if = "Option::is_none", rename = "logicalAxes")]
    logical_axes: Option<crate::dom::LogicalAxes>,
    /// true if the query contains scroll-state() queries.
    #[serde(skip_serializing_if = "Option::is_none", rename = "queriesScrollState")]
    queries_scroll_state: Option<bool>,
    /// true if the query contains anchored() queries.
    #[serde(skip_serializing_if = "Option::is_none", rename = "queriesAnchored")]
    queries_anchored: Option<bool>,
    /// CSSContainerRule.conditionText
    #[serde(rename = "conditionText")]
    condition_text: Cow<'a, str>,
}

impl<'a> CSSContainerQuery<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `text`: Container query text. Contains the query part without the container name for a single query. Deprecated in favor of conditionText which contains the full prelude after @container.
    /// * `condition_text`: CSSContainerRule.conditionText
    pub fn builder(text: impl Into<Cow<'a, str>>, condition_text: impl Into<Cow<'a, str>>) -> CSSContainerQueryBuilder<'a> {
        CSSContainerQueryBuilder {
            text: text.into(),
            range: None,
            style_sheet_id: None,
            name: None,
            physical_axes: None,
            logical_axes: None,
            queries_scroll_state: None,
            queries_anchored: None,
            condition_text: condition_text.into(),
        }
    }
    /// Container query text.
    /// Contains the query part without the container name for a single query.
    /// Deprecated in favor of conditionText which contains the full prelude
    /// after @container.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// Optional name for the container.
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    /// Optional physical axes queried for the container.
    pub fn physical_axes(&self) -> Option<&crate::dom::PhysicalAxes> { self.physical_axes.as_ref() }
    /// Optional logical axes queried for the container.
    pub fn logical_axes(&self) -> Option<&crate::dom::LogicalAxes> { self.logical_axes.as_ref() }
    /// true if the query contains scroll-state() queries.
    pub fn queries_scroll_state(&self) -> Option<bool> { self.queries_scroll_state }
    /// true if the query contains anchored() queries.
    pub fn queries_anchored(&self) -> Option<bool> { self.queries_anchored }
    /// CSSContainerRule.conditionText
    pub fn condition_text(&self) -> &str { self.condition_text.as_ref() }
}


pub struct CSSContainerQueryBuilder<'a> {
    text: Cow<'a, str>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    name: Option<Cow<'a, str>>,
    physical_axes: Option<crate::dom::PhysicalAxes>,
    logical_axes: Option<crate::dom::LogicalAxes>,
    queries_scroll_state: Option<bool>,
    queries_anchored: Option<bool>,
    condition_text: Cow<'a, str>,
}

impl<'a> CSSContainerQueryBuilder<'a> {
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    /// Optional name for the container.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Optional physical axes queried for the container.
    pub fn physical_axes(mut self, physical_axes: crate::dom::PhysicalAxes) -> Self { self.physical_axes = Some(physical_axes); self }
    /// Optional logical axes queried for the container.
    pub fn logical_axes(mut self, logical_axes: crate::dom::LogicalAxes) -> Self { self.logical_axes = Some(logical_axes); self }
    /// true if the query contains scroll-state() queries.
    pub fn queries_scroll_state(mut self, queries_scroll_state: bool) -> Self { self.queries_scroll_state = Some(queries_scroll_state); self }
    /// true if the query contains anchored() queries.
    pub fn queries_anchored(mut self, queries_anchored: bool) -> Self { self.queries_anchored = Some(queries_anchored); self }
    pub fn build(self) -> CSSContainerQuery<'a> {
        CSSContainerQuery {
            text: self.text,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
            name: self.name,
            physical_axes: self.physical_axes,
            logical_axes: self.logical_axes,
            queries_scroll_state: self.queries_scroll_state,
            queries_anchored: self.queries_anchored,
            condition_text: self.condition_text,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSSupports<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `text`: Supports rule text.
    /// * `active`: Whether the supports condition is satisfied.
    pub fn builder(text: impl Into<Cow<'a, str>>, active: bool) -> CSSSupportsBuilder<'a> {
        CSSSupportsBuilder {
            text: text.into(),
            active: active,
            range: None,
            style_sheet_id: None,
        }
    }
    /// Supports rule text.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// Whether the supports condition is satisfied.
    pub fn active(&self) -> bool { self.active }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
}


pub struct CSSSupportsBuilder<'a> {
    text: Cow<'a, str>,
    active: bool,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSSupportsBuilder<'a> {
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSSupports<'a> {
        CSSSupports {
            text: self.text,
            active: self.active,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSNavigation<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `text`: Navigation rule text.
    pub fn builder(text: impl Into<Cow<'a, str>>) -> CSSNavigationBuilder<'a> {
        CSSNavigationBuilder {
            text: text.into(),
            active: None,
            range: None,
            style_sheet_id: None,
        }
    }
    /// Navigation rule text.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// Whether the navigation condition is satisfied.
    pub fn active(&self) -> Option<bool> { self.active }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
}


pub struct CSSNavigationBuilder<'a> {
    text: Cow<'a, str>,
    active: Option<bool>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSNavigationBuilder<'a> {
    /// Whether the navigation condition is satisfied.
    pub fn active(mut self, active: bool) -> Self { self.active = Some(active); self }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSNavigation<'a> {
        CSSNavigation {
            text: self.text,
            active: self.active,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSScope<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `text`: Scope rule text.
    pub fn builder(text: impl Into<Cow<'a, str>>) -> CSSScopeBuilder<'a> {
        CSSScopeBuilder {
            text: text.into(),
            range: None,
            style_sheet_id: None,
        }
    }
    /// Scope rule text.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
}


pub struct CSSScopeBuilder<'a> {
    text: Cow<'a, str>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSScopeBuilder<'a> {
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSScope<'a> {
        CSSScope {
            text: self.text,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSLayer<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `text`: Layer name.
    pub fn builder(text: impl Into<Cow<'a, str>>) -> CSSLayerBuilder<'a> {
        CSSLayerBuilder {
            text: text.into(),
            range: None,
            style_sheet_id: None,
        }
    }
    /// Layer name.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
}


pub struct CSSLayerBuilder<'a> {
    text: Cow<'a, str>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSLayerBuilder<'a> {
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSLayer<'a> {
        CSSLayer {
            text: self.text,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSStartingStyle<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> CSSStartingStyleBuilder<'a> {
        CSSStartingStyleBuilder {
            range: None,
            style_sheet_id: None,
        }
    }
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(&self) -> Option<&SourceRange> { self.range.as_ref() }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
}

#[derive(Default)]
pub struct CSSStartingStyleBuilder<'a> {
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
}

impl<'a> CSSStartingStyleBuilder<'a> {
    /// The associated rule header range in the enclosing stylesheet (if
    /// available).
    pub fn range(mut self, range: SourceRange) -> Self { self.range = Some(range); self }
    /// Identifier of the stylesheet containing this object (if exists).
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSStartingStyle<'a> {
        CSSStartingStyle {
            range: self.range,
            style_sheet_id: self.style_sheet_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "subLayers")]
    sub_layers: Option<Vec<Box<CSSLayerData<'a>>>>,
    /// Layer order. The order determines the order of the layer in the cascade order.
    /// A higher number has higher priority in the cascade order.
    order: f64,
}

impl<'a> CSSLayerData<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Layer name.
    /// * `order`: Layer order. The order determines the order of the layer in the cascade order. A higher number has higher priority in the cascade order.
    pub fn builder(name: impl Into<Cow<'a, str>>, order: f64) -> CSSLayerDataBuilder<'a> {
        CSSLayerDataBuilder {
            name: name.into(),
            sub_layers: None,
            order: order,
        }
    }
    /// Layer name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Direct sub-layers
    pub fn sub_layers(&self) -> Option<&[Box<CSSLayerData<'a>>]> { self.sub_layers.as_deref() }
    /// Layer order. The order determines the order of the layer in the cascade order.
    /// A higher number has higher priority in the cascade order.
    pub fn order(&self) -> f64 { self.order }
}


pub struct CSSLayerDataBuilder<'a> {
    name: Cow<'a, str>,
    sub_layers: Option<Vec<Box<CSSLayerData<'a>>>>,
    order: f64,
}

impl<'a> CSSLayerDataBuilder<'a> {
    /// Direct sub-layers
    pub fn sub_layers(mut self, sub_layers: Vec<Box<CSSLayerData<'a>>>) -> Self { self.sub_layers = Some(sub_layers); self }
    pub fn build(self) -> CSSLayerData<'a> {
        CSSLayerData {
            name: self.name,
            sub_layers: self.sub_layers,
            order: self.order,
        }
    }
}

/// Information about amount of glyphs that were rendered with given font.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlatformFontUsage<'a> {
    /// Font's family name reported by platform.
    #[serde(rename = "familyName")]
    family_name: Cow<'a, str>,
    /// Font's PostScript name reported by platform.
    #[serde(rename = "postScriptName")]
    post_script_name: Cow<'a, str>,
    /// Indicates if the font was downloaded or resolved locally.
    #[serde(rename = "isCustomFont")]
    is_custom_font: bool,
    /// Amount of glyphs that were rendered with this font.
    #[serde(rename = "glyphCount")]
    glyph_count: f64,
}

impl<'a> PlatformFontUsage<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `family_name`: Font's family name reported by platform.
    /// * `post_script_name`: Font's PostScript name reported by platform.
    /// * `is_custom_font`: Indicates if the font was downloaded or resolved locally.
    /// * `glyph_count`: Amount of glyphs that were rendered with this font.
    pub fn builder(family_name: impl Into<Cow<'a, str>>, post_script_name: impl Into<Cow<'a, str>>, is_custom_font: bool, glyph_count: f64) -> PlatformFontUsageBuilder<'a> {
        PlatformFontUsageBuilder {
            family_name: family_name.into(),
            post_script_name: post_script_name.into(),
            is_custom_font: is_custom_font,
            glyph_count: glyph_count,
        }
    }
    /// Font's family name reported by platform.
    pub fn family_name(&self) -> &str { self.family_name.as_ref() }
    /// Font's PostScript name reported by platform.
    pub fn post_script_name(&self) -> &str { self.post_script_name.as_ref() }
    /// Indicates if the font was downloaded or resolved locally.
    pub fn is_custom_font(&self) -> bool { self.is_custom_font }
    /// Amount of glyphs that were rendered with this font.
    pub fn glyph_count(&self) -> f64 { self.glyph_count }
}


pub struct PlatformFontUsageBuilder<'a> {
    family_name: Cow<'a, str>,
    post_script_name: Cow<'a, str>,
    is_custom_font: bool,
    glyph_count: f64,
}

impl<'a> PlatformFontUsageBuilder<'a> {
    pub fn build(self) -> PlatformFontUsage<'a> {
        PlatformFontUsage {
            family_name: self.family_name,
            post_script_name: self.post_script_name,
            is_custom_font: self.is_custom_font,
            glyph_count: self.glyph_count,
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
    #[serde(rename = "minValue")]
    min_value: f64,
    /// The maximum value (inclusive) the font supports for this tag.
    #[serde(rename = "maxValue")]
    max_value: f64,
    /// The default value.
    #[serde(rename = "defaultValue")]
    default_value: f64,
}

impl<'a> FontVariationAxis<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `tag`: The font-variation-setting tag (a.k.a. "axis tag").
    /// * `name`: Human-readable variation name in the default language (normally, "en").
    /// * `min_value`: The minimum value (inclusive) the font supports for this tag.
    /// * `max_value`: The maximum value (inclusive) the font supports for this tag.
    /// * `default_value`: The default value.
    pub fn builder(tag: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, min_value: f64, max_value: f64, default_value: f64) -> FontVariationAxisBuilder<'a> {
        FontVariationAxisBuilder {
            tag: tag.into(),
            name: name.into(),
            min_value: min_value,
            max_value: max_value,
            default_value: default_value,
        }
    }
    /// The font-variation-setting tag (a.k.a. "axis tag").
    pub fn tag(&self) -> &str { self.tag.as_ref() }
    /// Human-readable variation name in the default language (normally, "en").
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// The minimum value (inclusive) the font supports for this tag.
    pub fn min_value(&self) -> f64 { self.min_value }
    /// The maximum value (inclusive) the font supports for this tag.
    pub fn max_value(&self) -> f64 { self.max_value }
    /// The default value.
    pub fn default_value(&self) -> f64 { self.default_value }
}


pub struct FontVariationAxisBuilder<'a> {
    tag: Cow<'a, str>,
    name: Cow<'a, str>,
    min_value: f64,
    max_value: f64,
    default_value: f64,
}

impl<'a> FontVariationAxisBuilder<'a> {
    pub fn build(self) -> FontVariationAxis<'a> {
        FontVariationAxis {
            tag: self.tag,
            name: self.name,
            min_value: self.min_value,
            max_value: self.max_value,
            default_value: self.default_value,
        }
    }
}

/// Properties of a web font: <https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions>
/// and additional information such as platformFontFamily and fontVariationAxes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FontFace<'a> {
    /// The font-family.
    #[serde(rename = "fontFamily")]
    font_family: Cow<'a, str>,
    /// The font-style.
    #[serde(rename = "fontStyle")]
    font_style: Cow<'a, str>,
    /// The font-variant.
    #[serde(rename = "fontVariant")]
    font_variant: Cow<'a, str>,
    /// The font-weight.
    #[serde(rename = "fontWeight")]
    font_weight: Cow<'a, str>,
    /// The font-stretch.
    #[serde(rename = "fontStretch")]
    font_stretch: Cow<'a, str>,
    /// The font-display.
    #[serde(rename = "fontDisplay")]
    font_display: Cow<'a, str>,
    /// The unicode-range.
    #[serde(rename = "unicodeRange")]
    unicode_range: Cow<'a, str>,
    /// The src.
    src: Cow<'a, str>,
    /// The resolved platform font family
    #[serde(rename = "platformFontFamily")]
    platform_font_family: Cow<'a, str>,
    /// Available variation settings (a.k.a. "axes").
    #[serde(skip_serializing_if = "Option::is_none", rename = "fontVariationAxes")]
    font_variation_axes: Option<Vec<FontVariationAxis<'a>>>,
}

impl<'a> FontFace<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `font_family`: The font-family.
    /// * `font_style`: The font-style.
    /// * `font_variant`: The font-variant.
    /// * `font_weight`: The font-weight.
    /// * `font_stretch`: The font-stretch.
    /// * `font_display`: The font-display.
    /// * `unicode_range`: The unicode-range.
    /// * `src`: The src.
    /// * `platform_font_family`: The resolved platform font family
    pub fn builder(font_family: impl Into<Cow<'a, str>>, font_style: impl Into<Cow<'a, str>>, font_variant: impl Into<Cow<'a, str>>, font_weight: impl Into<Cow<'a, str>>, font_stretch: impl Into<Cow<'a, str>>, font_display: impl Into<Cow<'a, str>>, unicode_range: impl Into<Cow<'a, str>>, src: impl Into<Cow<'a, str>>, platform_font_family: impl Into<Cow<'a, str>>) -> FontFaceBuilder<'a> {
        FontFaceBuilder {
            font_family: font_family.into(),
            font_style: font_style.into(),
            font_variant: font_variant.into(),
            font_weight: font_weight.into(),
            font_stretch: font_stretch.into(),
            font_display: font_display.into(),
            unicode_range: unicode_range.into(),
            src: src.into(),
            platform_font_family: platform_font_family.into(),
            font_variation_axes: None,
        }
    }
    /// The font-family.
    pub fn font_family(&self) -> &str { self.font_family.as_ref() }
    /// The font-style.
    pub fn font_style(&self) -> &str { self.font_style.as_ref() }
    /// The font-variant.
    pub fn font_variant(&self) -> &str { self.font_variant.as_ref() }
    /// The font-weight.
    pub fn font_weight(&self) -> &str { self.font_weight.as_ref() }
    /// The font-stretch.
    pub fn font_stretch(&self) -> &str { self.font_stretch.as_ref() }
    /// The font-display.
    pub fn font_display(&self) -> &str { self.font_display.as_ref() }
    /// The unicode-range.
    pub fn unicode_range(&self) -> &str { self.unicode_range.as_ref() }
    /// The src.
    pub fn src(&self) -> &str { self.src.as_ref() }
    /// The resolved platform font family
    pub fn platform_font_family(&self) -> &str { self.platform_font_family.as_ref() }
    /// Available variation settings (a.k.a. "axes").
    pub fn font_variation_axes(&self) -> Option<&[FontVariationAxis<'a>]> { self.font_variation_axes.as_deref() }
}


pub struct FontFaceBuilder<'a> {
    font_family: Cow<'a, str>,
    font_style: Cow<'a, str>,
    font_variant: Cow<'a, str>,
    font_weight: Cow<'a, str>,
    font_stretch: Cow<'a, str>,
    font_display: Cow<'a, str>,
    unicode_range: Cow<'a, str>,
    src: Cow<'a, str>,
    platform_font_family: Cow<'a, str>,
    font_variation_axes: Option<Vec<FontVariationAxis<'a>>>,
}

impl<'a> FontFaceBuilder<'a> {
    /// Available variation settings (a.k.a. "axes").
    pub fn font_variation_axes(mut self, font_variation_axes: Vec<FontVariationAxis<'a>>) -> Self { self.font_variation_axes = Some(font_variation_axes); self }
    pub fn build(self) -> FontFace<'a> {
        FontFace {
            font_family: self.font_family,
            font_style: self.font_style,
            font_variant: self.font_variant,
            font_weight: self.font_weight,
            font_stretch: self.font_stretch,
            font_display: self.font_display,
            unicode_range: self.unicode_range,
            src: self.src,
            platform_font_family: self.platform_font_family,
            font_variation_axes: self.font_variation_axes,
        }
    }
}

/// CSS try rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSTryRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated style declaration.
    style: CSSStyle<'a>,
}

impl<'a> CSSTryRule<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Parent stylesheet's origin.
    /// * `style`: Associated style declaration.
    pub fn builder(origin: impl Into<StyleSheetOrigin>, style: CSSStyle<'a>) -> CSSTryRuleBuilder<'a> {
        CSSTryRuleBuilder {
            style_sheet_id: None,
            origin: origin.into(),
            style: style,
        }
    }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// Parent stylesheet's origin.
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    /// Associated style declaration.
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}


pub struct CSSTryRuleBuilder<'a> {
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    origin: StyleSheetOrigin,
    style: CSSStyle<'a>,
}

impl<'a> CSSTryRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSTryRule<'a> {
        CSSTryRule {
            style_sheet_id: self.style_sheet_id,
            origin: self.origin,
            style: self.style,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated style declaration.
    style: CSSStyle<'a>,
    active: bool,
}

impl<'a> CSSPositionTryRule<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: The prelude dashed-ident name
    /// * `origin`: Parent stylesheet's origin.
    /// * `style`: Associated style declaration.
    /// * `active`: 
    pub fn builder(name: ProtocolValue<'a>, origin: impl Into<StyleSheetOrigin>, style: CSSStyle<'a>, active: bool) -> CSSPositionTryRuleBuilder<'a> {
        CSSPositionTryRuleBuilder {
            name: name,
            style_sheet_id: None,
            origin: origin.into(),
            style: style,
            active: active,
        }
    }
    /// The prelude dashed-ident name
    pub fn name(&self) -> &ProtocolValue<'a> { &self.name }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// Parent stylesheet's origin.
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    /// Associated style declaration.
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
    pub fn active(&self) -> bool { self.active }
}


pub struct CSSPositionTryRuleBuilder<'a> {
    name: ProtocolValue<'a>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    origin: StyleSheetOrigin,
    style: CSSStyle<'a>,
    active: bool,
}

impl<'a> CSSPositionTryRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSPositionTryRule<'a> {
        CSSPositionTryRule {
            name: self.name,
            style_sheet_id: self.style_sheet_id,
            origin: self.origin,
            style: self.style,
            active: self.active,
        }
    }
}

/// CSS keyframes rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSKeyframesRule<'a> {
    /// Animation name.
    #[serde(rename = "animationName")]
    animation_name: ProtocolValue<'a>,
    /// List of keyframes.
    keyframes: Vec<CSSKeyframeRule<'a>>,
}

impl<'a> CSSKeyframesRule<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `animation_name`: Animation name.
    /// * `keyframes`: List of keyframes.
    pub fn builder(animation_name: ProtocolValue<'a>, keyframes: Vec<CSSKeyframeRule<'a>>) -> CSSKeyframesRuleBuilder<'a> {
        CSSKeyframesRuleBuilder {
            animation_name: animation_name,
            keyframes: keyframes,
        }
    }
    /// Animation name.
    pub fn animation_name(&self) -> &ProtocolValue<'a> { &self.animation_name }
    /// List of keyframes.
    pub fn keyframes(&self) -> &[CSSKeyframeRule<'a>] { &self.keyframes }
}


pub struct CSSKeyframesRuleBuilder<'a> {
    animation_name: ProtocolValue<'a>,
    keyframes: Vec<CSSKeyframeRule<'a>>,
}

impl<'a> CSSKeyframesRuleBuilder<'a> {
    pub fn build(self) -> CSSKeyframesRule<'a> {
        CSSKeyframesRule {
            animation_name: self.animation_name,
            keyframes: self.keyframes,
        }
    }
}

/// Representation of a custom property registration through CSS.registerProperty

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSPropertyRegistration<'a> {
    #[serde(rename = "propertyName")]
    property_name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "initialValue")]
    initial_value: Option<ProtocolValue<'a>>,
    inherits: bool,
    syntax: Cow<'a, str>,
}

impl<'a> CSSPropertyRegistration<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `property_name`: 
    /// * `inherits`: 
    /// * `syntax`: 
    pub fn builder(property_name: impl Into<Cow<'a, str>>, inherits: bool, syntax: impl Into<Cow<'a, str>>) -> CSSPropertyRegistrationBuilder<'a> {
        CSSPropertyRegistrationBuilder {
            property_name: property_name.into(),
            initial_value: None,
            inherits: inherits,
            syntax: syntax.into(),
        }
    }
    pub fn property_name(&self) -> &str { self.property_name.as_ref() }
    pub fn initial_value(&self) -> Option<&ProtocolValue<'a>> { self.initial_value.as_ref() }
    pub fn inherits(&self) -> bool { self.inherits }
    pub fn syntax(&self) -> &str { self.syntax.as_ref() }
}


pub struct CSSPropertyRegistrationBuilder<'a> {
    property_name: Cow<'a, str>,
    initial_value: Option<ProtocolValue<'a>>,
    inherits: bool,
    syntax: Cow<'a, str>,
}

impl<'a> CSSPropertyRegistrationBuilder<'a> {
    pub fn initial_value(mut self, initial_value: ProtocolValue<'a>) -> Self { self.initial_value = Some(initial_value); self }
    pub fn build(self) -> CSSPropertyRegistration<'a> {
        CSSPropertyRegistration {
            property_name: self.property_name,
            initial_value: self.initial_value,
            inherits: self.inherits,
            syntax: self.syntax,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated style declaration.
    style: CSSStyle<'a>,
}

impl<'a> CSSAtRule<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Type of at-rule.
    /// * `origin`: Parent stylesheet's origin.
    /// * `style`: Associated style declaration.
    pub fn builder(type_: impl Into<Cow<'a, str>>, origin: impl Into<StyleSheetOrigin>, style: CSSStyle<'a>) -> CSSAtRuleBuilder<'a> {
        CSSAtRuleBuilder {
            type_: type_.into(),
            subsection: None,
            name: None,
            style_sheet_id: None,
            origin: origin.into(),
            style: style,
        }
    }
    /// Type of at-rule.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Subsection of font-feature-values, if this is a subsection.
    pub fn subsection(&self) -> Option<&str> { self.subsection.as_deref() }
    /// LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)
    /// Associated name, if applicable.
    pub fn name(&self) -> Option<&ProtocolValue<'a>> { self.name.as_ref() }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// Parent stylesheet's origin.
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    /// Associated style declaration.
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}


pub struct CSSAtRuleBuilder<'a> {
    type_: Cow<'a, str>,
    subsection: Option<Cow<'a, str>>,
    name: Option<ProtocolValue<'a>>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    origin: StyleSheetOrigin,
    style: CSSStyle<'a>,
}

impl<'a> CSSAtRuleBuilder<'a> {
    /// Subsection of font-feature-values, if this is a subsection.
    pub fn subsection(mut self, subsection: impl Into<Cow<'a, str>>) -> Self { self.subsection = Some(subsection.into()); self }
    /// LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)
    /// Associated name, if applicable.
    pub fn name(mut self, name: ProtocolValue<'a>) -> Self { self.name = Some(name); self }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSAtRule<'a> {
        CSSAtRule {
            type_: self.type_,
            subsection: self.subsection,
            name: self.name,
            style_sheet_id: self.style_sheet_id,
            origin: self.origin,
            style: self.style,
        }
    }
}

/// CSS property at-rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSPropertyRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated property name.
    #[serde(rename = "propertyName")]
    property_name: ProtocolValue<'a>,
    /// Associated style declaration.
    style: CSSStyle<'a>,
}

impl<'a> CSSPropertyRule<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Parent stylesheet's origin.
    /// * `property_name`: Associated property name.
    /// * `style`: Associated style declaration.
    pub fn builder(origin: impl Into<StyleSheetOrigin>, property_name: ProtocolValue<'a>, style: CSSStyle<'a>) -> CSSPropertyRuleBuilder<'a> {
        CSSPropertyRuleBuilder {
            style_sheet_id: None,
            origin: origin.into(),
            property_name: property_name,
            style: style,
        }
    }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// Parent stylesheet's origin.
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    /// Associated property name.
    pub fn property_name(&self) -> &ProtocolValue<'a> { &self.property_name }
    /// Associated style declaration.
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}


pub struct CSSPropertyRuleBuilder<'a> {
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    origin: StyleSheetOrigin,
    property_name: ProtocolValue<'a>,
    style: CSSStyle<'a>,
}

impl<'a> CSSPropertyRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSPropertyRule<'a> {
        CSSPropertyRule {
            style_sheet_id: self.style_sheet_id,
            origin: self.origin,
            property_name: self.property_name,
            style: self.style,
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: The parameter name.
    /// * `type_`: The parameter type.
    pub fn builder(name: impl Into<Cow<'a, str>>, type_: impl Into<Cow<'a, str>>) -> CSSFunctionParameterBuilder<'a> {
        CSSFunctionParameterBuilder {
            name: name.into(),
            type_: type_.into(),
        }
    }
    /// The parameter name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// The parameter type.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}


pub struct CSSFunctionParameterBuilder<'a> {
    name: Cow<'a, str>,
    type_: Cow<'a, str>,
}

impl<'a> CSSFunctionParameterBuilder<'a> {
    pub fn build(self) -> CSSFunctionParameter<'a> {
        CSSFunctionParameter {
            name: self.name,
            type_: self.type_,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "containerQueries")]
    container_queries: Option<CSSContainerQuery<'a>>,
    /// @supports CSS at-rule condition. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    supports: Option<CSSSupports<'a>>,
    /// @navigation condition. Only one type of condition should be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    navigation: Option<CSSNavigation<'a>>,
    /// Block body.
    children: Vec<CSSFunctionNode<'a>>,
    /// The condition text.
    #[serde(rename = "conditionText")]
    condition_text: Cow<'a, str>,
}

impl<'a> CSSFunctionConditionNode<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `children`: Block body.
    /// * `condition_text`: The condition text.
    pub fn builder(children: Vec<CSSFunctionNode<'a>>, condition_text: impl Into<Cow<'a, str>>) -> CSSFunctionConditionNodeBuilder<'a> {
        CSSFunctionConditionNodeBuilder {
            media: None,
            container_queries: None,
            supports: None,
            navigation: None,
            children: children,
            condition_text: condition_text.into(),
        }
    }
    /// Media query for this conditional block. Only one type of condition should be set.
    pub fn media(&self) -> Option<&CSSMedia<'a>> { self.media.as_ref() }
    /// Container query for this conditional block. Only one type of condition should be set.
    pub fn container_queries(&self) -> Option<&CSSContainerQuery<'a>> { self.container_queries.as_ref() }
    /// @supports CSS at-rule condition. Only one type of condition should be set.
    pub fn supports(&self) -> Option<&CSSSupports<'a>> { self.supports.as_ref() }
    /// @navigation condition. Only one type of condition should be set.
    pub fn navigation(&self) -> Option<&CSSNavigation<'a>> { self.navigation.as_ref() }
    /// Block body.
    pub fn children(&self) -> &[CSSFunctionNode<'a>] { &self.children }
    /// The condition text.
    pub fn condition_text(&self) -> &str { self.condition_text.as_ref() }
}


pub struct CSSFunctionConditionNodeBuilder<'a> {
    media: Option<CSSMedia<'a>>,
    container_queries: Option<CSSContainerQuery<'a>>,
    supports: Option<CSSSupports<'a>>,
    navigation: Option<CSSNavigation<'a>>,
    children: Vec<CSSFunctionNode<'a>>,
    condition_text: Cow<'a, str>,
}

impl<'a> CSSFunctionConditionNodeBuilder<'a> {
    /// Media query for this conditional block. Only one type of condition should be set.
    pub fn media(mut self, media: CSSMedia<'a>) -> Self { self.media = Some(media); self }
    /// Container query for this conditional block. Only one type of condition should be set.
    pub fn container_queries(mut self, container_queries: CSSContainerQuery<'a>) -> Self { self.container_queries = Some(container_queries); self }
    /// @supports CSS at-rule condition. Only one type of condition should be set.
    pub fn supports(mut self, supports: CSSSupports<'a>) -> Self { self.supports = Some(supports); self }
    /// @navigation condition. Only one type of condition should be set.
    pub fn navigation(mut self, navigation: CSSNavigation<'a>) -> Self { self.navigation = Some(navigation); self }
    pub fn build(self) -> CSSFunctionConditionNode<'a> {
        CSSFunctionConditionNode {
            media: self.media,
            container_queries: self.container_queries,
            supports: self.supports,
            navigation: self.navigation,
            children: self.children,
            condition_text: self.condition_text,
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
    /// Creates a builder for this type.
    pub fn builder() -> CSSFunctionNodeBuilder<'a> {
        CSSFunctionNodeBuilder {
            condition: None,
            style: None,
        }
    }
    /// A conditional block. If set, style should not be set.
    pub fn condition(&self) -> Option<&CSSFunctionConditionNode<'a>> { self.condition.as_ref() }
    /// Values set by this node. If set, condition should not be set.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// List of parameters.
    parameters: Vec<CSSFunctionParameter<'a>>,
    /// Function body.
    children: Vec<CSSFunctionNode<'a>>,
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originTreeScopeNodeId")]
    origin_tree_scope_node_id: Option<crate::dom::BackendNodeId>,
}

impl<'a> CSSFunctionRule<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Name of the function.
    /// * `origin`: Parent stylesheet's origin.
    /// * `parameters`: List of parameters.
    /// * `children`: Function body.
    pub fn builder(name: ProtocolValue<'a>, origin: impl Into<StyleSheetOrigin>, parameters: Vec<CSSFunctionParameter<'a>>, children: Vec<CSSFunctionNode<'a>>) -> CSSFunctionRuleBuilder<'a> {
        CSSFunctionRuleBuilder {
            name: name,
            style_sheet_id: None,
            origin: origin.into(),
            parameters: parameters,
            children: children,
            origin_tree_scope_node_id: None,
        }
    }
    /// Name of the function.
    pub fn name(&self) -> &ProtocolValue<'a> { &self.name }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// Parent stylesheet's origin.
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    /// List of parameters.
    pub fn parameters(&self) -> &[CSSFunctionParameter<'a>] { &self.parameters }
    /// Function body.
    pub fn children(&self) -> &[CSSFunctionNode<'a>] { &self.children }
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    pub fn origin_tree_scope_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.origin_tree_scope_node_id.as_ref() }
}


pub struct CSSFunctionRuleBuilder<'a> {
    name: ProtocolValue<'a>,
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    origin: StyleSheetOrigin,
    parameters: Vec<CSSFunctionParameter<'a>>,
    children: Vec<CSSFunctionNode<'a>>,
    origin_tree_scope_node_id: Option<crate::dom::BackendNodeId>,
}

impl<'a> CSSFunctionRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    /// The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.
    pub fn origin_tree_scope_node_id(mut self, origin_tree_scope_node_id: crate::dom::BackendNodeId) -> Self { self.origin_tree_scope_node_id = Some(origin_tree_scope_node_id); self }
    pub fn build(self) -> CSSFunctionRule<'a> {
        CSSFunctionRule {
            name: self.name,
            style_sheet_id: self.style_sheet_id,
            origin: self.origin,
            parameters: self.parameters,
            children: self.children,
            origin_tree_scope_node_id: self.origin_tree_scope_node_id,
        }
    }
}

/// CSS keyframe rule representation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSSKeyframeRule<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleSheetId")]
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    /// Parent stylesheet's origin.
    origin: StyleSheetOrigin,
    /// Associated key text.
    #[serde(rename = "keyText")]
    key_text: ProtocolValue<'a>,
    /// Associated style declaration.
    style: CSSStyle<'a>,
}

impl<'a> CSSKeyframeRule<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Parent stylesheet's origin.
    /// * `key_text`: Associated key text.
    /// * `style`: Associated style declaration.
    pub fn builder(origin: impl Into<StyleSheetOrigin>, key_text: ProtocolValue<'a>, style: CSSStyle<'a>) -> CSSKeyframeRuleBuilder<'a> {
        CSSKeyframeRuleBuilder {
            style_sheet_id: None,
            origin: origin.into(),
            key_text: key_text,
            style: style,
        }
    }
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(&self) -> Option<&crate::dom::StyleSheetId<'a>> { self.style_sheet_id.as_ref() }
    /// Parent stylesheet's origin.
    pub fn origin(&self) -> &StyleSheetOrigin { &self.origin }
    /// Associated key text.
    pub fn key_text(&self) -> &ProtocolValue<'a> { &self.key_text }
    /// Associated style declaration.
    pub fn style(&self) -> &CSSStyle<'a> { &self.style }
}


pub struct CSSKeyframeRuleBuilder<'a> {
    style_sheet_id: Option<crate::dom::StyleSheetId<'a>>,
    origin: StyleSheetOrigin,
    key_text: ProtocolValue<'a>,
    style: CSSStyle<'a>,
}

impl<'a> CSSKeyframeRuleBuilder<'a> {
    /// The css style sheet identifier (absent for user agent stylesheet and user-specified
    /// stylesheet rules) this rule came from.
    pub fn style_sheet_id(mut self, style_sheet_id: crate::dom::StyleSheetId<'a>) -> Self { self.style_sheet_id = Some(style_sheet_id); self }
    pub fn build(self) -> CSSKeyframeRule<'a> {
        CSSKeyframeRule {
            style_sheet_id: self.style_sheet_id,
            origin: self.origin,
            key_text: self.key_text,
            style: self.style,
        }
    }
}

/// A descriptor of operation to mutate style declaration text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StyleDeclarationEdit<'a> {
    /// The css style sheet identifier.
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    /// The range of the style text in the enclosing stylesheet.
    range: SourceRange,
    /// New style text.
    text: Cow<'a, str>,
}

impl<'a> StyleDeclarationEdit<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: The css style sheet identifier.
    /// * `range`: The range of the style text in the enclosing stylesheet.
    /// * `text`: New style text.
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, text: impl Into<Cow<'a, str>>) -> StyleDeclarationEditBuilder<'a> {
        StyleDeclarationEditBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            text: text.into(),
        }
    }
    /// The css style sheet identifier.
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    /// The range of the style text in the enclosing stylesheet.
    pub fn range(&self) -> &SourceRange { &self.range }
    /// New style text.
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct StyleDeclarationEditBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> StyleDeclarationEditBuilder<'a> {
    pub fn build(self) -> StyleDeclarationEdit<'a> {
        StyleDeclarationEdit {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            text: self.text,
        }
    }
}

/// Inserts a new rule with the given 'ruleText' in a stylesheet with given 'styleSheetId', at the
/// position specified by 'location'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddRuleParams<'a> {
    /// The css style sheet identifier where a new rule should be inserted.
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    /// The text of a new rule.
    #[serde(rename = "ruleText")]
    rule_text: Cow<'a, str>,
    /// Text position of a new rule in the target style sheet.
    location: SourceRange,
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeForPropertySyntaxValidation")]
    node_for_property_syntax_validation: Option<crate::dom::NodeId>,
}

impl<'a> AddRuleParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: The css style sheet identifier where a new rule should be inserted.
    /// * `rule_text`: The text of a new rule.
    /// * `location`: Text position of a new rule in the target style sheet.
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, rule_text: impl Into<Cow<'a, str>>, location: SourceRange) -> AddRuleParamsBuilder<'a> {
        AddRuleParamsBuilder {
            style_sheet_id: style_sheet_id,
            rule_text: rule_text.into(),
            location: location,
            node_for_property_syntax_validation: None,
        }
    }
    /// The css style sheet identifier where a new rule should be inserted.
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    /// The text of a new rule.
    pub fn rule_text(&self) -> &str { self.rule_text.as_ref() }
    /// Text position of a new rule in the target style sheet.
    pub fn location(&self) -> &SourceRange { &self.location }
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    pub fn node_for_property_syntax_validation(&self) -> Option<&crate::dom::NodeId> { self.node_for_property_syntax_validation.as_ref() }
}


pub struct AddRuleParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    rule_text: Cow<'a, str>,
    location: SourceRange,
    node_for_property_syntax_validation: Option<crate::dom::NodeId>,
}

impl<'a> AddRuleParamsBuilder<'a> {
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    pub fn node_for_property_syntax_validation(mut self, node_for_property_syntax_validation: crate::dom::NodeId) -> Self { self.node_for_property_syntax_validation = Some(node_for_property_syntax_validation); self }
    pub fn build(self) -> AddRuleParams<'a> {
        AddRuleParams {
            style_sheet_id: self.style_sheet_id,
            rule_text: self.rule_text,
            location: self.location,
            node_for_property_syntax_validation: self.node_for_property_syntax_validation,
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
    /// Creates a builder for this type with the required parameters:
    /// * `rule`: The newly created rule.
    pub fn builder(rule: CSSRule<'a>) -> AddRuleReturnsBuilder<'a> {
        AddRuleReturnsBuilder {
            rule: rule,
        }
    }
    /// The newly created rule.
    pub fn rule(&self) -> &CSSRule<'a> { &self.rule }
}


pub struct AddRuleReturnsBuilder<'a> {
    rule: CSSRule<'a>,
}

impl<'a> AddRuleReturnsBuilder<'a> {
    pub fn build(self) -> AddRuleReturns<'a> {
        AddRuleReturns {
            rule: self.rule,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
}

impl<'a> CollectClassNamesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>) -> CollectClassNamesParamsBuilder<'a> {
        CollectClassNamesParamsBuilder {
            style_sheet_id: style_sheet_id,
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
}


pub struct CollectClassNamesParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
}

impl<'a> CollectClassNamesParamsBuilder<'a> {
    pub fn build(self) -> CollectClassNamesParams<'a> {
        CollectClassNamesParams {
            style_sheet_id: self.style_sheet_id,
        }
    }
}

/// Returns all class names from specified stylesheet.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectClassNamesReturns<'a> {
    /// Class name list.
    #[serde(rename = "classNames")]
    class_names: Vec<Cow<'a, str>>,
}

impl<'a> CollectClassNamesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `class_names`: Class name list.
    pub fn builder(class_names: Vec<Cow<'a, str>>) -> CollectClassNamesReturnsBuilder<'a> {
        CollectClassNamesReturnsBuilder {
            class_names: class_names,
        }
    }
    /// Class name list.
    pub fn class_names(&self) -> &[Cow<'a, str>] { &self.class_names }
}


pub struct CollectClassNamesReturnsBuilder<'a> {
    class_names: Vec<Cow<'a, str>>,
}

impl<'a> CollectClassNamesReturnsBuilder<'a> {
    pub fn build(self) -> CollectClassNamesReturns<'a> {
        CollectClassNamesReturns {
            class_names: self.class_names,
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
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
    /// If true, creates a new stylesheet for every call. If false,
    /// returns a stylesheet previously created by a call with force=false
    /// for the frame's document if it exists or creates a new stylesheet
    /// (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}

impl<'a> CreateStyleSheetParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: Identifier of the frame where "via-inspector" stylesheet should be created.
    pub fn builder(frame_id: crate::page::FrameId<'a>) -> CreateStyleSheetParamsBuilder<'a> {
        CreateStyleSheetParamsBuilder {
            frame_id: frame_id,
            force: None,
        }
    }
    /// Identifier of the frame where "via-inspector" stylesheet should be created.
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
    /// If true, creates a new stylesheet for every call. If false,
    /// returns a stylesheet previously created by a call with force=false
    /// for the frame's document if it exists or creates a new stylesheet
    /// (default: false).
    pub fn force(&self) -> Option<bool> { self.force }
}


pub struct CreateStyleSheetParamsBuilder<'a> {
    frame_id: crate::page::FrameId<'a>,
    force: Option<bool>,
}

impl<'a> CreateStyleSheetParamsBuilder<'a> {
    /// If true, creates a new stylesheet for every call. If false,
    /// returns a stylesheet previously created by a call with force=false
    /// for the frame's document if it exists or creates a new stylesheet
    /// (default: false).
    pub fn force(mut self, force: bool) -> Self { self.force = Some(force); self }
    pub fn build(self) -> CreateStyleSheetParams<'a> {
        CreateStyleSheetParams {
            frame_id: self.frame_id,
            force: self.force,
        }
    }
}

/// Creates a new special "via-inspector" stylesheet in the frame with given 'frameId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateStyleSheetReturns<'a> {
    /// Identifier of the created "via-inspector" stylesheet.
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
}

impl<'a> CreateStyleSheetReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: Identifier of the created "via-inspector" stylesheet.
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>) -> CreateStyleSheetReturnsBuilder<'a> {
        CreateStyleSheetReturnsBuilder {
            style_sheet_id: style_sheet_id,
        }
    }
    /// Identifier of the created "via-inspector" stylesheet.
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
}


pub struct CreateStyleSheetReturnsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
}

impl<'a> CreateStyleSheetReturnsBuilder<'a> {
    pub fn build(self) -> CreateStyleSheetReturns<'a> {
        CreateStyleSheetReturns {
            style_sheet_id: self.style_sheet_id,
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

impl DisableParams { pub const METHOD: &'static str = "CSS.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "CSS.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
    /// Element pseudo classes to force when computing the element's style.
    #[serde(rename = "forcedPseudoClasses")]
    forced_pseudo_classes: Vec<Cow<'a, str>>,
}

impl<'a> ForcePseudoStateParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: The element id for which to force the pseudo state.
    /// * `forced_pseudo_classes`: Element pseudo classes to force when computing the element's style.
    pub fn builder(node_id: crate::dom::NodeId, forced_pseudo_classes: Vec<Cow<'a, str>>) -> ForcePseudoStateParamsBuilder<'a> {
        ForcePseudoStateParamsBuilder {
            node_id: node_id,
            forced_pseudo_classes: forced_pseudo_classes,
        }
    }
    /// The element id for which to force the pseudo state.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
    /// Element pseudo classes to force when computing the element's style.
    pub fn forced_pseudo_classes(&self) -> &[Cow<'a, str>] { &self.forced_pseudo_classes }
}


pub struct ForcePseudoStateParamsBuilder<'a> {
    node_id: crate::dom::NodeId,
    forced_pseudo_classes: Vec<Cow<'a, str>>,
}

impl<'a> ForcePseudoStateParamsBuilder<'a> {
    pub fn build(self) -> ForcePseudoStateParams<'a> {
        ForcePseudoStateParams {
            node_id: self.node_id,
            forced_pseudo_classes: self.forced_pseudo_classes,
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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
    /// Boolean indicating if this is on or off.
    forced: bool,
}

impl ForceStartingStyleParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: The element id for which to force the starting-style state.
    /// * `forced`: Boolean indicating if this is on or off.
    pub fn builder(node_id: crate::dom::NodeId, forced: bool) -> ForceStartingStyleParamsBuilder {
        ForceStartingStyleParamsBuilder {
            node_id: node_id,
            forced: forced,
        }
    }
    /// The element id for which to force the starting-style state.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
    /// Boolean indicating if this is on or off.
    pub fn forced(&self) -> bool { self.forced }
}


pub struct ForceStartingStyleParamsBuilder {
    node_id: crate::dom::NodeId,
    forced: bool,
}

impl ForceStartingStyleParamsBuilder {
    pub fn build(self) -> ForceStartingStyleParams {
        ForceStartingStyleParams {
            node_id: self.node_id,
            forced: self.forced,
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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl GetBackgroundColorsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: Id of the node to get background colors for.
    pub fn builder(node_id: crate::dom::NodeId) -> GetBackgroundColorsParamsBuilder {
        GetBackgroundColorsParamsBuilder {
            node_id: node_id,
        }
    }
    /// Id of the node to get background colors for.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct GetBackgroundColorsParamsBuilder {
    node_id: crate::dom::NodeId,
}

impl GetBackgroundColorsParamsBuilder {
    pub fn build(self) -> GetBackgroundColorsParams {
        GetBackgroundColorsParams {
            node_id: self.node_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "backgroundColors")]
    background_colors: Option<Vec<Cow<'a, str>>>,
    /// The computed font size for this node, as a CSS computed value string (e.g. '12px').
    #[serde(skip_serializing_if = "Option::is_none", rename = "computedFontSize")]
    computed_font_size: Option<Cow<'a, str>>,
    /// The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
    /// '100').
    #[serde(skip_serializing_if = "Option::is_none", rename = "computedFontWeight")]
    computed_font_weight: Option<Cow<'a, str>>,
}

impl<'a> GetBackgroundColorsReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetBackgroundColorsReturnsBuilder<'a> {
        GetBackgroundColorsReturnsBuilder {
            background_colors: None,
            computed_font_size: None,
            computed_font_weight: None,
        }
    }
    /// The range of background colors behind this element, if it contains any visible text. If no
    /// visible text is present, this will be undefined. In the case of a flat background color,
    /// this will consist of simply that color. In the case of a gradient, this will consist of each
    /// of the color stops. For anything more complicated, this will be an empty array. Images will
    /// be ignored (as if the image had failed to load).
    pub fn background_colors(&self) -> Option<&[Cow<'a, str>]> { self.background_colors.as_deref() }
    /// The computed font size for this node, as a CSS computed value string (e.g. '12px').
    pub fn computed_font_size(&self) -> Option<&str> { self.computed_font_size.as_deref() }
    /// The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
    /// '100').
    pub fn computed_font_weight(&self) -> Option<&str> { self.computed_font_weight.as_deref() }
}

#[derive(Default)]
pub struct GetBackgroundColorsReturnsBuilder<'a> {
    background_colors: Option<Vec<Cow<'a, str>>>,
    computed_font_size: Option<Cow<'a, str>>,
    computed_font_weight: Option<Cow<'a, str>>,
}

impl<'a> GetBackgroundColorsReturnsBuilder<'a> {
    /// The range of background colors behind this element, if it contains any visible text. If no
    /// visible text is present, this will be undefined. In the case of a flat background color,
    /// this will consist of simply that color. In the case of a gradient, this will consist of each
    /// of the color stops. For anything more complicated, this will be an empty array. Images will
    /// be ignored (as if the image had failed to load).
    pub fn background_colors(mut self, background_colors: Vec<Cow<'a, str>>) -> Self { self.background_colors = Some(background_colors); self }
    /// The computed font size for this node, as a CSS computed value string (e.g. '12px').
    pub fn computed_font_size(mut self, computed_font_size: impl Into<Cow<'a, str>>) -> Self { self.computed_font_size = Some(computed_font_size.into()); self }
    /// The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
    /// '100').
    pub fn computed_font_weight(mut self, computed_font_weight: impl Into<Cow<'a, str>>) -> Self { self.computed_font_weight = Some(computed_font_weight.into()); self }
    pub fn build(self) -> GetBackgroundColorsReturns<'a> {
        GetBackgroundColorsReturns {
            background_colors: self.background_colors,
            computed_font_size: self.computed_font_size,
            computed_font_weight: self.computed_font_weight,
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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl GetComputedStyleForNodeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: 
    pub fn builder(node_id: crate::dom::NodeId) -> GetComputedStyleForNodeParamsBuilder {
        GetComputedStyleForNodeParamsBuilder {
            node_id: node_id,
        }
    }
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct GetComputedStyleForNodeParamsBuilder {
    node_id: crate::dom::NodeId,
}

impl GetComputedStyleForNodeParamsBuilder {
    pub fn build(self) -> GetComputedStyleForNodeParams {
        GetComputedStyleForNodeParams {
            node_id: self.node_id,
        }
    }
}

/// Returns the computed style for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetComputedStyleForNodeReturns<'a> {
    /// Computed style for the specified DOM node.
    #[serde(rename = "computedStyle")]
    computed_style: Vec<CSSComputedStyleProperty<'a>>,
    /// A list of non-standard "extra fields" which blink stores alongside each
    /// computed style.
    #[serde(rename = "extraFields")]
    extra_fields: ComputedStyleExtraFields,
}

impl<'a> GetComputedStyleForNodeReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `computed_style`: Computed style for the specified DOM node.
    /// * `extra_fields`: A list of non-standard "extra fields" which blink stores alongside each computed style.
    pub fn builder(computed_style: Vec<CSSComputedStyleProperty<'a>>, extra_fields: ComputedStyleExtraFields) -> GetComputedStyleForNodeReturnsBuilder<'a> {
        GetComputedStyleForNodeReturnsBuilder {
            computed_style: computed_style,
            extra_fields: extra_fields,
        }
    }
    /// Computed style for the specified DOM node.
    pub fn computed_style(&self) -> &[CSSComputedStyleProperty<'a>] { &self.computed_style }
    /// A list of non-standard "extra fields" which blink stores alongside each
    /// computed style.
    pub fn extra_fields(&self) -> &ComputedStyleExtraFields { &self.extra_fields }
}


pub struct GetComputedStyleForNodeReturnsBuilder<'a> {
    computed_style: Vec<CSSComputedStyleProperty<'a>>,
    extra_fields: ComputedStyleExtraFields,
}

impl<'a> GetComputedStyleForNodeReturnsBuilder<'a> {
    pub fn build(self) -> GetComputedStyleForNodeReturns<'a> {
        GetComputedStyleForNodeReturns {
            computed_style: self.computed_style,
            extra_fields: self.extra_fields,
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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
    /// Only longhands and custom property names are accepted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "propertyName")]
    property_name: Option<Cow<'a, str>>,
    /// Pseudo element type, only works for pseudo elements that generate
    /// elements in the tree, such as ::before and ::after.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoType")]
    pseudo_type: Option<crate::dom::PseudoType>,
    /// Pseudo element custom ident.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoIdentifier")]
    pseudo_identifier: Option<Cow<'a, str>>,
}

impl<'a> ResolveValuesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `values`: Cascade-dependent keywords (revert/revert-layer) do not work.
    /// * `node_id`: Id of the node in whose context the expression is evaluated
    pub fn builder(values: Vec<Cow<'a, str>>, node_id: crate::dom::NodeId) -> ResolveValuesParamsBuilder<'a> {
        ResolveValuesParamsBuilder {
            values: values,
            node_id: node_id,
            property_name: None,
            pseudo_type: None,
            pseudo_identifier: None,
        }
    }
    /// Cascade-dependent keywords (revert/revert-layer) do not work.
    pub fn values(&self) -> &[Cow<'a, str>] { &self.values }
    /// Id of the node in whose context the expression is evaluated
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
    /// Only longhands and custom property names are accepted.
    pub fn property_name(&self) -> Option<&str> { self.property_name.as_deref() }
    /// Pseudo element type, only works for pseudo elements that generate
    /// elements in the tree, such as ::before and ::after.
    pub fn pseudo_type(&self) -> Option<&crate::dom::PseudoType> { self.pseudo_type.as_ref() }
    /// Pseudo element custom ident.
    pub fn pseudo_identifier(&self) -> Option<&str> { self.pseudo_identifier.as_deref() }
}


pub struct ResolveValuesParamsBuilder<'a> {
    values: Vec<Cow<'a, str>>,
    node_id: crate::dom::NodeId,
    property_name: Option<Cow<'a, str>>,
    pseudo_type: Option<crate::dom::PseudoType>,
    pseudo_identifier: Option<Cow<'a, str>>,
}

impl<'a> ResolveValuesParamsBuilder<'a> {
    /// Only longhands and custom property names are accepted.
    pub fn property_name(mut self, property_name: impl Into<Cow<'a, str>>) -> Self { self.property_name = Some(property_name.into()); self }
    /// Pseudo element type, only works for pseudo elements that generate
    /// elements in the tree, such as ::before and ::after.
    pub fn pseudo_type(mut self, pseudo_type: crate::dom::PseudoType) -> Self { self.pseudo_type = Some(pseudo_type); self }
    /// Pseudo element custom ident.
    pub fn pseudo_identifier(mut self, pseudo_identifier: impl Into<Cow<'a, str>>) -> Self { self.pseudo_identifier = Some(pseudo_identifier.into()); self }
    pub fn build(self) -> ResolveValuesParams<'a> {
        ResolveValuesParams {
            values: self.values,
            node_id: self.node_id,
            property_name: self.property_name,
            pseudo_type: self.pseudo_type,
            pseudo_identifier: self.pseudo_identifier,
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
    /// Creates a builder for this type with the required parameters:
    /// * `results`: 
    pub fn builder(results: Vec<Cow<'a, str>>) -> ResolveValuesReturnsBuilder<'a> {
        ResolveValuesReturnsBuilder {
            results: results,
        }
    }
    pub fn results(&self) -> &[Cow<'a, str>] { &self.results }
}


pub struct ResolveValuesReturnsBuilder<'a> {
    results: Vec<Cow<'a, str>>,
}

impl<'a> ResolveValuesReturnsBuilder<'a> {
    pub fn build(self) -> ResolveValuesReturns<'a> {
        ResolveValuesReturns {
            results: self.results,
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
    #[serde(rename = "shorthandName")]
    shorthand_name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> GetLonghandPropertiesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `shorthand_name`: 
    /// * `value`: 
    pub fn builder(shorthand_name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> GetLonghandPropertiesParamsBuilder<'a> {
        GetLonghandPropertiesParamsBuilder {
            shorthand_name: shorthand_name.into(),
            value: value.into(),
        }
    }
    pub fn shorthand_name(&self) -> &str { self.shorthand_name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct GetLonghandPropertiesParamsBuilder<'a> {
    shorthand_name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> GetLonghandPropertiesParamsBuilder<'a> {
    pub fn build(self) -> GetLonghandPropertiesParams<'a> {
        GetLonghandPropertiesParams {
            shorthand_name: self.shorthand_name,
            value: self.value,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLonghandPropertiesReturns<'a> {
    #[serde(rename = "longhandProperties")]
    longhand_properties: Vec<CSSProperty<'a>>,
}

impl<'a> GetLonghandPropertiesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `longhand_properties`: 
    pub fn builder(longhand_properties: Vec<CSSProperty<'a>>) -> GetLonghandPropertiesReturnsBuilder<'a> {
        GetLonghandPropertiesReturnsBuilder {
            longhand_properties: longhand_properties,
        }
    }
    pub fn longhand_properties(&self) -> &[CSSProperty<'a>] { &self.longhand_properties }
}


pub struct GetLonghandPropertiesReturnsBuilder<'a> {
    longhand_properties: Vec<CSSProperty<'a>>,
}

impl<'a> GetLonghandPropertiesReturnsBuilder<'a> {
    pub fn build(self) -> GetLonghandPropertiesReturns<'a> {
        GetLonghandPropertiesReturns {
            longhand_properties: self.longhand_properties,
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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl GetInlineStylesForNodeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: 
    pub fn builder(node_id: crate::dom::NodeId) -> GetInlineStylesForNodeParamsBuilder {
        GetInlineStylesForNodeParamsBuilder {
            node_id: node_id,
        }
    }
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct GetInlineStylesForNodeParamsBuilder {
    node_id: crate::dom::NodeId,
}

impl GetInlineStylesForNodeParamsBuilder {
    pub fn build(self) -> GetInlineStylesForNodeParams {
        GetInlineStylesForNodeParams {
            node_id: self.node_id,
        }
    }
}

/// Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
/// attributes) for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInlineStylesForNodeReturns<'a> {
    /// Inline style for the specified DOM node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inlineStyle")]
    inline_style: Option<CSSStyle<'a>>,
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    #[serde(skip_serializing_if = "Option::is_none", rename = "attributesStyle")]
    attributes_style: Option<CSSStyle<'a>>,
}

impl<'a> GetInlineStylesForNodeReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetInlineStylesForNodeReturnsBuilder<'a> {
        GetInlineStylesForNodeReturnsBuilder {
            inline_style: None,
            attributes_style: None,
        }
    }
    /// Inline style for the specified DOM node.
    pub fn inline_style(&self) -> Option<&CSSStyle<'a>> { self.inline_style.as_ref() }
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    pub fn attributes_style(&self) -> Option<&CSSStyle<'a>> { self.attributes_style.as_ref() }
}

#[derive(Default)]
pub struct GetInlineStylesForNodeReturnsBuilder<'a> {
    inline_style: Option<CSSStyle<'a>>,
    attributes_style: Option<CSSStyle<'a>>,
}

impl<'a> GetInlineStylesForNodeReturnsBuilder<'a> {
    /// Inline style for the specified DOM node.
    pub fn inline_style(mut self, inline_style: CSSStyle<'a>) -> Self { self.inline_style = Some(inline_style); self }
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    pub fn attributes_style(mut self, attributes_style: CSSStyle<'a>) -> Self { self.attributes_style = Some(attributes_style); self }
    pub fn build(self) -> GetInlineStylesForNodeReturns<'a> {
        GetInlineStylesForNodeReturns {
            inline_style: self.inline_style,
            attributes_style: self.attributes_style,
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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl GetAnimatedStylesForNodeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: 
    pub fn builder(node_id: crate::dom::NodeId) -> GetAnimatedStylesForNodeParamsBuilder {
        GetAnimatedStylesForNodeParamsBuilder {
            node_id: node_id,
        }
    }
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct GetAnimatedStylesForNodeParamsBuilder {
    node_id: crate::dom::NodeId,
}

impl GetAnimatedStylesForNodeParamsBuilder {
    pub fn build(self) -> GetAnimatedStylesForNodeParams {
        GetAnimatedStylesForNodeParams {
            node_id: self.node_id,
        }
    }
}

/// Returns the styles coming from animations & transitions
/// including the animation & transition styles coming from inheritance chain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnimatedStylesForNodeReturns<'a> {
    /// Styles coming from animations.
    #[serde(skip_serializing_if = "Option::is_none", rename = "animationStyles")]
    animation_styles: Option<Vec<CSSAnimationStyle<'a>>>,
    /// Style coming from transitions.
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitionsStyle")]
    transitions_style: Option<CSSStyle<'a>>,
    /// Inherited style entries for animationsStyle and transitionsStyle from
    /// the inheritance chain of the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited: Option<Vec<InheritedAnimatedStyleEntry<'a>>>,
}

impl<'a> GetAnimatedStylesForNodeReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetAnimatedStylesForNodeReturnsBuilder<'a> {
        GetAnimatedStylesForNodeReturnsBuilder {
            animation_styles: None,
            transitions_style: None,
            inherited: None,
        }
    }
    /// Styles coming from animations.
    pub fn animation_styles(&self) -> Option<&[CSSAnimationStyle<'a>]> { self.animation_styles.as_deref() }
    /// Style coming from transitions.
    pub fn transitions_style(&self) -> Option<&CSSStyle<'a>> { self.transitions_style.as_ref() }
    /// Inherited style entries for animationsStyle and transitionsStyle from
    /// the inheritance chain of the element.
    pub fn inherited(&self) -> Option<&[InheritedAnimatedStyleEntry<'a>]> { self.inherited.as_deref() }
}

#[derive(Default)]
pub struct GetAnimatedStylesForNodeReturnsBuilder<'a> {
    animation_styles: Option<Vec<CSSAnimationStyle<'a>>>,
    transitions_style: Option<CSSStyle<'a>>,
    inherited: Option<Vec<InheritedAnimatedStyleEntry<'a>>>,
}

impl<'a> GetAnimatedStylesForNodeReturnsBuilder<'a> {
    /// Styles coming from animations.
    pub fn animation_styles(mut self, animation_styles: Vec<CSSAnimationStyle<'a>>) -> Self { self.animation_styles = Some(animation_styles); self }
    /// Style coming from transitions.
    pub fn transitions_style(mut self, transitions_style: CSSStyle<'a>) -> Self { self.transitions_style = Some(transitions_style); self }
    /// Inherited style entries for animationsStyle and transitionsStyle from
    /// the inheritance chain of the element.
    pub fn inherited(mut self, inherited: Vec<InheritedAnimatedStyleEntry<'a>>) -> Self { self.inherited = Some(inherited); self }
    pub fn build(self) -> GetAnimatedStylesForNodeReturns<'a> {
        GetAnimatedStylesForNodeReturns {
            animation_styles: self.animation_styles,
            transitions_style: self.transitions_style,
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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl GetMatchedStylesForNodeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: 
    pub fn builder(node_id: crate::dom::NodeId) -> GetMatchedStylesForNodeParamsBuilder {
        GetMatchedStylesForNodeParamsBuilder {
            node_id: node_id,
        }
    }
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct GetMatchedStylesForNodeParamsBuilder {
    node_id: crate::dom::NodeId,
}

impl GetMatchedStylesForNodeParamsBuilder {
    pub fn build(self) -> GetMatchedStylesForNodeParams {
        GetMatchedStylesForNodeParams {
            node_id: self.node_id,
        }
    }
}

/// Returns requested styles for a DOM node identified by 'nodeId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMatchedStylesForNodeReturns<'a> {
    /// Inline style for the specified DOM node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "inlineStyle")]
    inline_style: Option<CSSStyle<'a>>,
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    #[serde(skip_serializing_if = "Option::is_none", rename = "attributesStyle")]
    attributes_style: Option<CSSStyle<'a>>,
    /// CSS rules matching this node, from all applicable stylesheets.
    #[serde(skip_serializing_if = "Option::is_none", rename = "matchedCSSRules")]
    matched_css_rules: Option<Vec<RuleMatch<'a>>>,
    /// Pseudo style matches for this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pseudoElements")]
    pseudo_elements: Option<Vec<PseudoElementMatches<'a>>>,
    /// A chain of inherited styles (from the immediate node parent up to the DOM tree root).
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited: Option<Vec<InheritedStyleEntry<'a>>>,
    /// A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root).
    #[serde(skip_serializing_if = "Option::is_none", rename = "inheritedPseudoElements")]
    inherited_pseudo_elements: Option<Vec<InheritedPseudoElementMatches<'a>>>,
    /// A list of CSS keyframed animations matching this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssKeyframesRules")]
    css_keyframes_rules: Option<Vec<CSSKeyframesRule<'a>>>,
    /// A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssPositionTryRules")]
    css_position_try_rules: Option<Vec<CSSPositionTryRule<'a>>>,
    /// Index of the active fallback in the applied position-try-fallback property,
    /// will not be set if there is no active position-try fallback.
    #[serde(skip_serializing_if = "Option::is_none", rename = "activePositionFallbackIndex")]
    active_position_fallback_index: Option<u64>,
    /// A list of CSS at-property rules matching this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssPropertyRules")]
    css_property_rules: Option<Vec<CSSPropertyRule<'a>>>,
    /// A list of CSS property registrations matching this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssPropertyRegistrations")]
    css_property_registrations: Option<Vec<CSSPropertyRegistration<'a>>>,
    /// A list of simple @rules matching this node or its pseudo-elements.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssAtRules")]
    css_at_rules: Option<Vec<CSSAtRule<'a>>>,
    /// Id of the first parent element that does not have display: contents.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentLayoutNodeId")]
    parent_layout_node_id: Option<crate::dom::NodeId>,
    /// A list of CSS at-function rules referenced by styles of this node.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cssFunctionRules")]
    css_function_rules: Option<Vec<CSSFunctionRule<'a>>>,
}

impl<'a> GetMatchedStylesForNodeReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetMatchedStylesForNodeReturnsBuilder<'a> {
        GetMatchedStylesForNodeReturnsBuilder {
            inline_style: None,
            attributes_style: None,
            matched_css_rules: None,
            pseudo_elements: None,
            inherited: None,
            inherited_pseudo_elements: None,
            css_keyframes_rules: None,
            css_position_try_rules: None,
            active_position_fallback_index: None,
            css_property_rules: None,
            css_property_registrations: None,
            css_at_rules: None,
            parent_layout_node_id: None,
            css_function_rules: None,
        }
    }
    /// Inline style for the specified DOM node.
    pub fn inline_style(&self) -> Option<&CSSStyle<'a>> { self.inline_style.as_ref() }
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    pub fn attributes_style(&self) -> Option<&CSSStyle<'a>> { self.attributes_style.as_ref() }
    /// CSS rules matching this node, from all applicable stylesheets.
    pub fn matched_css_rules(&self) -> Option<&[RuleMatch<'a>]> { self.matched_css_rules.as_deref() }
    /// Pseudo style matches for this node.
    pub fn pseudo_elements(&self) -> Option<&[PseudoElementMatches<'a>]> { self.pseudo_elements.as_deref() }
    /// A chain of inherited styles (from the immediate node parent up to the DOM tree root).
    pub fn inherited(&self) -> Option<&[InheritedStyleEntry<'a>]> { self.inherited.as_deref() }
    /// A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root).
    pub fn inherited_pseudo_elements(&self) -> Option<&[InheritedPseudoElementMatches<'a>]> { self.inherited_pseudo_elements.as_deref() }
    /// A list of CSS keyframed animations matching this node.
    pub fn css_keyframes_rules(&self) -> Option<&[CSSKeyframesRule<'a>]> { self.css_keyframes_rules.as_deref() }
    /// A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property.
    pub fn css_position_try_rules(&self) -> Option<&[CSSPositionTryRule<'a>]> { self.css_position_try_rules.as_deref() }
    /// Index of the active fallback in the applied position-try-fallback property,
    /// will not be set if there is no active position-try fallback.
    pub fn active_position_fallback_index(&self) -> Option<u64> { self.active_position_fallback_index }
    /// A list of CSS at-property rules matching this node.
    pub fn css_property_rules(&self) -> Option<&[CSSPropertyRule<'a>]> { self.css_property_rules.as_deref() }
    /// A list of CSS property registrations matching this node.
    pub fn css_property_registrations(&self) -> Option<&[CSSPropertyRegistration<'a>]> { self.css_property_registrations.as_deref() }
    /// A list of simple @rules matching this node or its pseudo-elements.
    pub fn css_at_rules(&self) -> Option<&[CSSAtRule<'a>]> { self.css_at_rules.as_deref() }
    /// Id of the first parent element that does not have display: contents.
    pub fn parent_layout_node_id(&self) -> Option<&crate::dom::NodeId> { self.parent_layout_node_id.as_ref() }
    /// A list of CSS at-function rules referenced by styles of this node.
    pub fn css_function_rules(&self) -> Option<&[CSSFunctionRule<'a>]> { self.css_function_rules.as_deref() }
}

#[derive(Default)]
pub struct GetMatchedStylesForNodeReturnsBuilder<'a> {
    inline_style: Option<CSSStyle<'a>>,
    attributes_style: Option<CSSStyle<'a>>,
    matched_css_rules: Option<Vec<RuleMatch<'a>>>,
    pseudo_elements: Option<Vec<PseudoElementMatches<'a>>>,
    inherited: Option<Vec<InheritedStyleEntry<'a>>>,
    inherited_pseudo_elements: Option<Vec<InheritedPseudoElementMatches<'a>>>,
    css_keyframes_rules: Option<Vec<CSSKeyframesRule<'a>>>,
    css_position_try_rules: Option<Vec<CSSPositionTryRule<'a>>>,
    active_position_fallback_index: Option<u64>,
    css_property_rules: Option<Vec<CSSPropertyRule<'a>>>,
    css_property_registrations: Option<Vec<CSSPropertyRegistration<'a>>>,
    css_at_rules: Option<Vec<CSSAtRule<'a>>>,
    parent_layout_node_id: Option<crate::dom::NodeId>,
    css_function_rules: Option<Vec<CSSFunctionRule<'a>>>,
}

impl<'a> GetMatchedStylesForNodeReturnsBuilder<'a> {
    /// Inline style for the specified DOM node.
    pub fn inline_style(mut self, inline_style: CSSStyle<'a>) -> Self { self.inline_style = Some(inline_style); self }
    /// Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    pub fn attributes_style(mut self, attributes_style: CSSStyle<'a>) -> Self { self.attributes_style = Some(attributes_style); self }
    /// CSS rules matching this node, from all applicable stylesheets.
    pub fn matched_css_rules(mut self, matched_css_rules: Vec<RuleMatch<'a>>) -> Self { self.matched_css_rules = Some(matched_css_rules); self }
    /// Pseudo style matches for this node.
    pub fn pseudo_elements(mut self, pseudo_elements: Vec<PseudoElementMatches<'a>>) -> Self { self.pseudo_elements = Some(pseudo_elements); self }
    /// A chain of inherited styles (from the immediate node parent up to the DOM tree root).
    pub fn inherited(mut self, inherited: Vec<InheritedStyleEntry<'a>>) -> Self { self.inherited = Some(inherited); self }
    /// A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root).
    pub fn inherited_pseudo_elements(mut self, inherited_pseudo_elements: Vec<InheritedPseudoElementMatches<'a>>) -> Self { self.inherited_pseudo_elements = Some(inherited_pseudo_elements); self }
    /// A list of CSS keyframed animations matching this node.
    pub fn css_keyframes_rules(mut self, css_keyframes_rules: Vec<CSSKeyframesRule<'a>>) -> Self { self.css_keyframes_rules = Some(css_keyframes_rules); self }
    /// A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property.
    pub fn css_position_try_rules(mut self, css_position_try_rules: Vec<CSSPositionTryRule<'a>>) -> Self { self.css_position_try_rules = Some(css_position_try_rules); self }
    /// Index of the active fallback in the applied position-try-fallback property,
    /// will not be set if there is no active position-try fallback.
    pub fn active_position_fallback_index(mut self, active_position_fallback_index: u64) -> Self { self.active_position_fallback_index = Some(active_position_fallback_index); self }
    /// A list of CSS at-property rules matching this node.
    pub fn css_property_rules(mut self, css_property_rules: Vec<CSSPropertyRule<'a>>) -> Self { self.css_property_rules = Some(css_property_rules); self }
    /// A list of CSS property registrations matching this node.
    pub fn css_property_registrations(mut self, css_property_registrations: Vec<CSSPropertyRegistration<'a>>) -> Self { self.css_property_registrations = Some(css_property_registrations); self }
    /// A list of simple @rules matching this node or its pseudo-elements.
    pub fn css_at_rules(mut self, css_at_rules: Vec<CSSAtRule<'a>>) -> Self { self.css_at_rules = Some(css_at_rules); self }
    /// Id of the first parent element that does not have display: contents.
    pub fn parent_layout_node_id(mut self, parent_layout_node_id: crate::dom::NodeId) -> Self { self.parent_layout_node_id = Some(parent_layout_node_id); self }
    /// A list of CSS at-function rules referenced by styles of this node.
    pub fn css_function_rules(mut self, css_function_rules: Vec<CSSFunctionRule<'a>>) -> Self { self.css_function_rules = Some(css_function_rules); self }
    pub fn build(self) -> GetMatchedStylesForNodeReturns<'a> {
        GetMatchedStylesForNodeReturns {
            inline_style: self.inline_style,
            attributes_style: self.attributes_style,
            matched_css_rules: self.matched_css_rules,
            pseudo_elements: self.pseudo_elements,
            inherited: self.inherited,
            inherited_pseudo_elements: self.inherited_pseudo_elements,
            css_keyframes_rules: self.css_keyframes_rules,
            css_position_try_rules: self.css_position_try_rules,
            active_position_fallback_index: self.active_position_fallback_index,
            css_property_rules: self.css_property_rules,
            css_property_registrations: self.css_property_registrations,
            css_at_rules: self.css_at_rules,
            parent_layout_node_id: self.parent_layout_node_id,
            css_function_rules: self.css_function_rules,
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
    #[serde(rename = "environmentVariables")]
    environment_variables: serde_json::Map<String, JsonValue>,
}

impl GetEnvironmentVariablesReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `environment_variables`: 
    pub fn builder(environment_variables: serde_json::Map<String, JsonValue>) -> GetEnvironmentVariablesReturnsBuilder {
        GetEnvironmentVariablesReturnsBuilder {
            environment_variables: environment_variables,
        }
    }
    pub fn environment_variables(&self) -> &serde_json::Map<String, JsonValue> { &self.environment_variables }
}


pub struct GetEnvironmentVariablesReturnsBuilder {
    environment_variables: serde_json::Map<String, JsonValue>,
}

impl GetEnvironmentVariablesReturnsBuilder {
    pub fn build(self) -> GetEnvironmentVariablesReturns {
        GetEnvironmentVariablesReturns {
            environment_variables: self.environment_variables,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEnvironmentVariablesParams {}

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
    /// Creates a builder for this type with the required parameters:
    /// * `medias`: 
    pub fn builder(medias: Vec<CSSMedia<'a>>) -> GetMediaQueriesReturnsBuilder<'a> {
        GetMediaQueriesReturnsBuilder {
            medias: medias,
        }
    }
    pub fn medias(&self) -> &[CSSMedia<'a>] { &self.medias }
}


pub struct GetMediaQueriesReturnsBuilder<'a> {
    medias: Vec<CSSMedia<'a>>,
}

impl<'a> GetMediaQueriesReturnsBuilder<'a> {
    pub fn build(self) -> GetMediaQueriesReturns<'a> {
        GetMediaQueriesReturns {
            medias: self.medias,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMediaQueriesParams {}

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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl GetPlatformFontsForNodeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: 
    pub fn builder(node_id: crate::dom::NodeId) -> GetPlatformFontsForNodeParamsBuilder {
        GetPlatformFontsForNodeParamsBuilder {
            node_id: node_id,
        }
    }
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct GetPlatformFontsForNodeParamsBuilder {
    node_id: crate::dom::NodeId,
}

impl GetPlatformFontsForNodeParamsBuilder {
    pub fn build(self) -> GetPlatformFontsForNodeParams {
        GetPlatformFontsForNodeParams {
            node_id: self.node_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `fonts`: Usage statistics for every employed platform font.
    pub fn builder(fonts: Vec<PlatformFontUsage<'a>>) -> GetPlatformFontsForNodeReturnsBuilder<'a> {
        GetPlatformFontsForNodeReturnsBuilder {
            fonts: fonts,
        }
    }
    /// Usage statistics for every employed platform font.
    pub fn fonts(&self) -> &[PlatformFontUsage<'a>] { &self.fonts }
}


pub struct GetPlatformFontsForNodeReturnsBuilder<'a> {
    fonts: Vec<PlatformFontUsage<'a>>,
}

impl<'a> GetPlatformFontsForNodeReturnsBuilder<'a> {
    pub fn build(self) -> GetPlatformFontsForNodeReturns<'a> {
        GetPlatformFontsForNodeReturns {
            fonts: self.fonts,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
}

impl<'a> GetStyleSheetTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>) -> GetStyleSheetTextParamsBuilder<'a> {
        GetStyleSheetTextParamsBuilder {
            style_sheet_id: style_sheet_id,
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
}


pub struct GetStyleSheetTextParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
}

impl<'a> GetStyleSheetTextParamsBuilder<'a> {
    pub fn build(self) -> GetStyleSheetTextParams<'a> {
        GetStyleSheetTextParams {
            style_sheet_id: self.style_sheet_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `text`: The stylesheet text.
    pub fn builder(text: impl Into<Cow<'a, str>>) -> GetStyleSheetTextReturnsBuilder<'a> {
        GetStyleSheetTextReturnsBuilder {
            text: text.into(),
        }
    }
    /// The stylesheet text.
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct GetStyleSheetTextReturnsBuilder<'a> {
    text: Cow<'a, str>,
}

impl<'a> GetStyleSheetTextReturnsBuilder<'a> {
    pub fn build(self) -> GetStyleSheetTextReturns<'a> {
        GetStyleSheetTextReturns {
            text: self.text,
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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
}

impl GetLayersForNodeParams {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: 
    pub fn builder(node_id: crate::dom::NodeId) -> GetLayersForNodeParamsBuilder {
        GetLayersForNodeParamsBuilder {
            node_id: node_id,
        }
    }
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
}


pub struct GetLayersForNodeParamsBuilder {
    node_id: crate::dom::NodeId,
}

impl GetLayersForNodeParamsBuilder {
    pub fn build(self) -> GetLayersForNodeParams {
        GetLayersForNodeParams {
            node_id: self.node_id,
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
    #[serde(rename = "rootLayer")]
    root_layer: CSSLayerData<'a>,
}

impl<'a> GetLayersForNodeReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `root_layer`: 
    pub fn builder(root_layer: CSSLayerData<'a>) -> GetLayersForNodeReturnsBuilder<'a> {
        GetLayersForNodeReturnsBuilder {
            root_layer: root_layer,
        }
    }
    pub fn root_layer(&self) -> &CSSLayerData<'a> { &self.root_layer }
}


pub struct GetLayersForNodeReturnsBuilder<'a> {
    root_layer: CSSLayerData<'a>,
}

impl<'a> GetLayersForNodeReturnsBuilder<'a> {
    pub fn build(self) -> GetLayersForNodeReturns<'a> {
        GetLayersForNodeReturns {
            root_layer: self.root_layer,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    #[serde(rename = "selectorText")]
    selector_text: Cow<'a, str>,
}

impl<'a> GetLocationForSelectorParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `selector_text`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, selector_text: impl Into<Cow<'a, str>>) -> GetLocationForSelectorParamsBuilder<'a> {
        GetLocationForSelectorParamsBuilder {
            style_sheet_id: style_sheet_id,
            selector_text: selector_text.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn selector_text(&self) -> &str { self.selector_text.as_ref() }
}


pub struct GetLocationForSelectorParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    selector_text: Cow<'a, str>,
}

impl<'a> GetLocationForSelectorParamsBuilder<'a> {
    pub fn build(self) -> GetLocationForSelectorParams<'a> {
        GetLocationForSelectorParams {
            style_sheet_id: self.style_sheet_id,
            selector_text: self.selector_text,
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
    /// Creates a builder for this type with the required parameters:
    /// * `ranges`: 
    pub fn builder(ranges: Vec<SourceRange>) -> GetLocationForSelectorReturnsBuilder {
        GetLocationForSelectorReturnsBuilder {
            ranges: ranges,
        }
    }
    pub fn ranges(&self) -> &[SourceRange] { &self.ranges }
}


pub struct GetLocationForSelectorReturnsBuilder {
    ranges: Vec<SourceRange>,
}

impl GetLocationForSelectorReturnsBuilder {
    pub fn build(self) -> GetLocationForSelectorReturns {
        GetLocationForSelectorReturns {
            ranges: self.ranges,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::NodeId>,
}

impl TrackComputedStyleUpdatesForNodeParams {
    /// Creates a builder for this type.
    pub fn builder() -> TrackComputedStyleUpdatesForNodeParamsBuilder {
        TrackComputedStyleUpdatesForNodeParamsBuilder {
            node_id: None,
        }
    }
    pub fn node_id(&self) -> Option<&crate::dom::NodeId> { self.node_id.as_ref() }
}

#[derive(Default)]
pub struct TrackComputedStyleUpdatesForNodeParamsBuilder {
    node_id: Option<crate::dom::NodeId>,
}

impl TrackComputedStyleUpdatesForNodeParamsBuilder {
    pub fn node_id(mut self, node_id: crate::dom::NodeId) -> Self { self.node_id = Some(node_id); self }
    pub fn build(self) -> TrackComputedStyleUpdatesForNodeParams {
        TrackComputedStyleUpdatesForNodeParams {
            node_id: self.node_id,
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
    #[serde(rename = "propertiesToTrack")]
    properties_to_track: Vec<CSSComputedStyleProperty<'a>>,
}

impl<'a> TrackComputedStyleUpdatesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `properties_to_track`: 
    pub fn builder(properties_to_track: Vec<CSSComputedStyleProperty<'a>>) -> TrackComputedStyleUpdatesParamsBuilder<'a> {
        TrackComputedStyleUpdatesParamsBuilder {
            properties_to_track: properties_to_track,
        }
    }
    pub fn properties_to_track(&self) -> &[CSSComputedStyleProperty<'a>] { &self.properties_to_track }
}


pub struct TrackComputedStyleUpdatesParamsBuilder<'a> {
    properties_to_track: Vec<CSSComputedStyleProperty<'a>>,
}

impl<'a> TrackComputedStyleUpdatesParamsBuilder<'a> {
    pub fn build(self) -> TrackComputedStyleUpdatesParams<'a> {
        TrackComputedStyleUpdatesParams {
            properties_to_track: self.properties_to_track,
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
    #[serde(rename = "nodeIds")]
    node_ids: Vec<crate::dom::NodeId>,
}

impl TakeComputedStyleUpdatesReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `node_ids`: The list of node Ids that have their tracked computed styles updated.
    pub fn builder(node_ids: Vec<crate::dom::NodeId>) -> TakeComputedStyleUpdatesReturnsBuilder {
        TakeComputedStyleUpdatesReturnsBuilder {
            node_ids: node_ids,
        }
    }
    /// The list of node Ids that have their tracked computed styles updated.
    pub fn node_ids(&self) -> &[crate::dom::NodeId] { &self.node_ids }
}


pub struct TakeComputedStyleUpdatesReturnsBuilder {
    node_ids: Vec<crate::dom::NodeId>,
}

impl TakeComputedStyleUpdatesReturnsBuilder {
    pub fn build(self) -> TakeComputedStyleUpdatesReturns {
        TakeComputedStyleUpdatesReturns {
            node_ids: self.node_ids,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TakeComputedStyleUpdatesParams {}

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
    #[serde(rename = "nodeId")]
    node_id: crate::dom::NodeId,
    #[serde(rename = "propertyName")]
    property_name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SetEffectivePropertyValueForNodeParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: The element id for which to set property.
    /// * `property_name`: 
    /// * `value`: 
    pub fn builder(node_id: crate::dom::NodeId, property_name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> SetEffectivePropertyValueForNodeParamsBuilder<'a> {
        SetEffectivePropertyValueForNodeParamsBuilder {
            node_id: node_id,
            property_name: property_name.into(),
            value: value.into(),
        }
    }
    /// The element id for which to set property.
    pub fn node_id(&self) -> &crate::dom::NodeId { &self.node_id }
    pub fn property_name(&self) -> &str { self.property_name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct SetEffectivePropertyValueForNodeParamsBuilder<'a> {
    node_id: crate::dom::NodeId,
    property_name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SetEffectivePropertyValueForNodeParamsBuilder<'a> {
    pub fn build(self) -> SetEffectivePropertyValueForNodeParams<'a> {
        SetEffectivePropertyValueForNodeParams {
            node_id: self.node_id,
            property_name: self.property_name,
            value: self.value,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    #[serde(rename = "propertyName")]
    property_name: Cow<'a, str>,
}

impl<'a> SetPropertyRulePropertyNameParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `range`: 
    /// * `property_name`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, property_name: impl Into<Cow<'a, str>>) -> SetPropertyRulePropertyNameParamsBuilder<'a> {
        SetPropertyRulePropertyNameParamsBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            property_name: property_name.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn property_name(&self) -> &str { self.property_name.as_ref() }
}


pub struct SetPropertyRulePropertyNameParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    property_name: Cow<'a, str>,
}

impl<'a> SetPropertyRulePropertyNameParamsBuilder<'a> {
    pub fn build(self) -> SetPropertyRulePropertyNameParams<'a> {
        SetPropertyRulePropertyNameParams {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            property_name: self.property_name,
        }
    }
}

/// Modifies the property rule property name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPropertyRulePropertyNameReturns<'a> {
    /// The resulting key text after modification.
    #[serde(rename = "propertyName")]
    property_name: ProtocolValue<'a>,
}

impl<'a> SetPropertyRulePropertyNameReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `property_name`: The resulting key text after modification.
    pub fn builder(property_name: ProtocolValue<'a>) -> SetPropertyRulePropertyNameReturnsBuilder<'a> {
        SetPropertyRulePropertyNameReturnsBuilder {
            property_name: property_name,
        }
    }
    /// The resulting key text after modification.
    pub fn property_name(&self) -> &ProtocolValue<'a> { &self.property_name }
}


pub struct SetPropertyRulePropertyNameReturnsBuilder<'a> {
    property_name: ProtocolValue<'a>,
}

impl<'a> SetPropertyRulePropertyNameReturnsBuilder<'a> {
    pub fn build(self) -> SetPropertyRulePropertyNameReturns<'a> {
        SetPropertyRulePropertyNameReturns {
            property_name: self.property_name,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    #[serde(rename = "keyText")]
    key_text: Cow<'a, str>,
}

impl<'a> SetKeyframeKeyParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `range`: 
    /// * `key_text`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, key_text: impl Into<Cow<'a, str>>) -> SetKeyframeKeyParamsBuilder<'a> {
        SetKeyframeKeyParamsBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            key_text: key_text.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn key_text(&self) -> &str { self.key_text.as_ref() }
}


pub struct SetKeyframeKeyParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    key_text: Cow<'a, str>,
}

impl<'a> SetKeyframeKeyParamsBuilder<'a> {
    pub fn build(self) -> SetKeyframeKeyParams<'a> {
        SetKeyframeKeyParams {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            key_text: self.key_text,
        }
    }
}

/// Modifies the keyframe rule key text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetKeyframeKeyReturns<'a> {
    /// The resulting key text after modification.
    #[serde(rename = "keyText")]
    key_text: ProtocolValue<'a>,
}

impl<'a> SetKeyframeKeyReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `key_text`: The resulting key text after modification.
    pub fn builder(key_text: ProtocolValue<'a>) -> SetKeyframeKeyReturnsBuilder<'a> {
        SetKeyframeKeyReturnsBuilder {
            key_text: key_text,
        }
    }
    /// The resulting key text after modification.
    pub fn key_text(&self) -> &ProtocolValue<'a> { &self.key_text }
}


pub struct SetKeyframeKeyReturnsBuilder<'a> {
    key_text: ProtocolValue<'a>,
}

impl<'a> SetKeyframeKeyReturnsBuilder<'a> {
    pub fn build(self) -> SetKeyframeKeyReturns<'a> {
        SetKeyframeKeyReturns {
            key_text: self.key_text,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetMediaTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `range`: 
    /// * `text`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, text: impl Into<Cow<'a, str>>) -> SetMediaTextParamsBuilder<'a> {
        SetMediaTextParamsBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            text: text.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct SetMediaTextParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetMediaTextParamsBuilder<'a> {
    pub fn build(self) -> SetMediaTextParams<'a> {
        SetMediaTextParams {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            text: self.text,
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
    /// Creates a builder for this type with the required parameters:
    /// * `media`: The resulting CSS media rule after modification.
    pub fn builder(media: CSSMedia<'a>) -> SetMediaTextReturnsBuilder<'a> {
        SetMediaTextReturnsBuilder {
            media: media,
        }
    }
    /// The resulting CSS media rule after modification.
    pub fn media(&self) -> &CSSMedia<'a> { &self.media }
}


pub struct SetMediaTextReturnsBuilder<'a> {
    media: CSSMedia<'a>,
}

impl<'a> SetMediaTextReturnsBuilder<'a> {
    pub fn build(self) -> SetMediaTextReturns<'a> {
        SetMediaTextReturns {
            media: self.media,
        }
    }
}

impl<'a> SetMediaTextParams<'a> { pub const METHOD: &'static str = "CSS.setMediaText"; }

impl<'a> crate::CdpCommand<'a> for SetMediaTextParams<'a> {
    const METHOD: &'static str = "CSS.setMediaText";
    type Response = SetMediaTextReturns<'a>;
}

/// Modifies the expression of a container query.
/// Deprecated. Use setContainerQueryConditionText instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetContainerQueryTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `range`: 
    /// * `text`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, text: impl Into<Cow<'a, str>>) -> SetContainerQueryTextParamsBuilder<'a> {
        SetContainerQueryTextParamsBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            text: text.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct SetContainerQueryTextParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetContainerQueryTextParamsBuilder<'a> {
    pub fn build(self) -> SetContainerQueryTextParams<'a> {
        SetContainerQueryTextParams {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            text: self.text,
        }
    }
}

/// Modifies the expression of a container query.
/// Deprecated. Use setContainerQueryConditionText instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryTextReturns<'a> {
    /// The resulting CSS container query rule after modification.
    #[serde(rename = "containerQuery")]
    container_query: CSSContainerQuery<'a>,
}

impl<'a> SetContainerQueryTextReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `container_query`: The resulting CSS container query rule after modification.
    pub fn builder(container_query: CSSContainerQuery<'a>) -> SetContainerQueryTextReturnsBuilder<'a> {
        SetContainerQueryTextReturnsBuilder {
            container_query: container_query,
        }
    }
    /// The resulting CSS container query rule after modification.
    pub fn container_query(&self) -> &CSSContainerQuery<'a> { &self.container_query }
}


pub struct SetContainerQueryTextReturnsBuilder<'a> {
    container_query: CSSContainerQuery<'a>,
}

impl<'a> SetContainerQueryTextReturnsBuilder<'a> {
    pub fn build(self) -> SetContainerQueryTextReturns<'a> {
        SetContainerQueryTextReturns {
            container_query: self.container_query,
        }
    }
}

impl<'a> SetContainerQueryTextParams<'a> { pub const METHOD: &'static str = "CSS.setContainerQueryText"; }

impl<'a> crate::CdpCommand<'a> for SetContainerQueryTextParams<'a> {
    const METHOD: &'static str = "CSS.setContainerQueryText";
    type Response = SetContainerQueryTextReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryConditionTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetContainerQueryConditionTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `range`: 
    /// * `text`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, text: impl Into<Cow<'a, str>>) -> SetContainerQueryConditionTextParamsBuilder<'a> {
        SetContainerQueryConditionTextParamsBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            text: text.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct SetContainerQueryConditionTextParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetContainerQueryConditionTextParamsBuilder<'a> {
    pub fn build(self) -> SetContainerQueryConditionTextParams<'a> {
        SetContainerQueryConditionTextParams {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            text: self.text,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetContainerQueryConditionTextReturns<'a> {
    /// The resulting CSS container query rule after modification.
    #[serde(rename = "containerQuery")]
    container_query: CSSContainerQuery<'a>,
}

impl<'a> SetContainerQueryConditionTextReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `container_query`: The resulting CSS container query rule after modification.
    pub fn builder(container_query: CSSContainerQuery<'a>) -> SetContainerQueryConditionTextReturnsBuilder<'a> {
        SetContainerQueryConditionTextReturnsBuilder {
            container_query: container_query,
        }
    }
    /// The resulting CSS container query rule after modification.
    pub fn container_query(&self) -> &CSSContainerQuery<'a> { &self.container_query }
}


pub struct SetContainerQueryConditionTextReturnsBuilder<'a> {
    container_query: CSSContainerQuery<'a>,
}

impl<'a> SetContainerQueryConditionTextReturnsBuilder<'a> {
    pub fn build(self) -> SetContainerQueryConditionTextReturns<'a> {
        SetContainerQueryConditionTextReturns {
            container_query: self.container_query,
        }
    }
}

impl<'a> SetContainerQueryConditionTextParams<'a> { pub const METHOD: &'static str = "CSS.setContainerQueryConditionText"; }

impl<'a> crate::CdpCommand<'a> for SetContainerQueryConditionTextParams<'a> {
    const METHOD: &'static str = "CSS.setContainerQueryConditionText";
    type Response = SetContainerQueryConditionTextReturns<'a>;
}

/// Modifies the expression of a supports at-rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSupportsTextParams<'a> {
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetSupportsTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `range`: 
    /// * `text`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, text: impl Into<Cow<'a, str>>) -> SetSupportsTextParamsBuilder<'a> {
        SetSupportsTextParamsBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            text: text.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct SetSupportsTextParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetSupportsTextParamsBuilder<'a> {
    pub fn build(self) -> SetSupportsTextParams<'a> {
        SetSupportsTextParams {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            text: self.text,
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
    /// Creates a builder for this type with the required parameters:
    /// * `supports`: The resulting CSS Supports rule after modification.
    pub fn builder(supports: CSSSupports<'a>) -> SetSupportsTextReturnsBuilder<'a> {
        SetSupportsTextReturnsBuilder {
            supports: supports,
        }
    }
    /// The resulting CSS Supports rule after modification.
    pub fn supports(&self) -> &CSSSupports<'a> { &self.supports }
}


pub struct SetSupportsTextReturnsBuilder<'a> {
    supports: CSSSupports<'a>,
}

impl<'a> SetSupportsTextReturnsBuilder<'a> {
    pub fn build(self) -> SetSupportsTextReturns<'a> {
        SetSupportsTextReturns {
            supports: self.supports,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetNavigationTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `range`: 
    /// * `text`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, text: impl Into<Cow<'a, str>>) -> SetNavigationTextParamsBuilder<'a> {
        SetNavigationTextParamsBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            text: text.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct SetNavigationTextParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetNavigationTextParamsBuilder<'a> {
    pub fn build(self) -> SetNavigationTextParams<'a> {
        SetNavigationTextParams {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            text: self.text,
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
    /// Creates a builder for this type with the required parameters:
    /// * `navigation`: The resulting CSS Navigation rule after modification.
    pub fn builder(navigation: CSSNavigation<'a>) -> SetNavigationTextReturnsBuilder<'a> {
        SetNavigationTextReturnsBuilder {
            navigation: navigation,
        }
    }
    /// The resulting CSS Navigation rule after modification.
    pub fn navigation(&self) -> &CSSNavigation<'a> { &self.navigation }
}


pub struct SetNavigationTextReturnsBuilder<'a> {
    navigation: CSSNavigation<'a>,
}

impl<'a> SetNavigationTextReturnsBuilder<'a> {
    pub fn build(self) -> SetNavigationTextReturns<'a> {
        SetNavigationTextReturns {
            navigation: self.navigation,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetScopeTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `range`: 
    /// * `text`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, text: impl Into<Cow<'a, str>>) -> SetScopeTextParamsBuilder<'a> {
        SetScopeTextParamsBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            text: text.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct SetScopeTextParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    text: Cow<'a, str>,
}

impl<'a> SetScopeTextParamsBuilder<'a> {
    pub fn build(self) -> SetScopeTextParams<'a> {
        SetScopeTextParams {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            text: self.text,
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
    /// Creates a builder for this type with the required parameters:
    /// * `scope`: The resulting CSS Scope rule after modification.
    pub fn builder(scope: CSSScope<'a>) -> SetScopeTextReturnsBuilder<'a> {
        SetScopeTextReturnsBuilder {
            scope: scope,
        }
    }
    /// The resulting CSS Scope rule after modification.
    pub fn scope(&self) -> &CSSScope<'a> { &self.scope }
}


pub struct SetScopeTextReturnsBuilder<'a> {
    scope: CSSScope<'a>,
}

impl<'a> SetScopeTextReturnsBuilder<'a> {
    pub fn build(self) -> SetScopeTextReturns<'a> {
        SetScopeTextReturns {
            scope: self.scope,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    selector: Cow<'a, str>,
}

impl<'a> SetRuleSelectorParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `range`: 
    /// * `selector`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, range: SourceRange, selector: impl Into<Cow<'a, str>>) -> SetRuleSelectorParamsBuilder<'a> {
        SetRuleSelectorParamsBuilder {
            style_sheet_id: style_sheet_id,
            range: range,
            selector: selector.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn range(&self) -> &SourceRange { &self.range }
    pub fn selector(&self) -> &str { self.selector.as_ref() }
}


pub struct SetRuleSelectorParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    range: SourceRange,
    selector: Cow<'a, str>,
}

impl<'a> SetRuleSelectorParamsBuilder<'a> {
    pub fn build(self) -> SetRuleSelectorParams<'a> {
        SetRuleSelectorParams {
            style_sheet_id: self.style_sheet_id,
            range: self.range,
            selector: self.selector,
        }
    }
}

/// Modifies the rule selector.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRuleSelectorReturns<'a> {
    /// The resulting selector list after modification.
    #[serde(rename = "selectorList")]
    selector_list: SelectorList<'a>,
}

impl<'a> SetRuleSelectorReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `selector_list`: The resulting selector list after modification.
    pub fn builder(selector_list: SelectorList<'a>) -> SetRuleSelectorReturnsBuilder<'a> {
        SetRuleSelectorReturnsBuilder {
            selector_list: selector_list,
        }
    }
    /// The resulting selector list after modification.
    pub fn selector_list(&self) -> &SelectorList<'a> { &self.selector_list }
}


pub struct SetRuleSelectorReturnsBuilder<'a> {
    selector_list: SelectorList<'a>,
}

impl<'a> SetRuleSelectorReturnsBuilder<'a> {
    pub fn build(self) -> SetRuleSelectorReturns<'a> {
        SetRuleSelectorReturns {
            selector_list: self.selector_list,
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
    #[serde(rename = "styleSheetId")]
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    text: Cow<'a, str>,
}

impl<'a> SetStyleSheetTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `style_sheet_id`: 
    /// * `text`: 
    pub fn builder(style_sheet_id: crate::dom::StyleSheetId<'a>, text: impl Into<Cow<'a, str>>) -> SetStyleSheetTextParamsBuilder<'a> {
        SetStyleSheetTextParamsBuilder {
            style_sheet_id: style_sheet_id,
            text: text.into(),
        }
    }
    pub fn style_sheet_id(&self) -> &crate::dom::StyleSheetId<'a> { &self.style_sheet_id }
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct SetStyleSheetTextParamsBuilder<'a> {
    style_sheet_id: crate::dom::StyleSheetId<'a>,
    text: Cow<'a, str>,
}

impl<'a> SetStyleSheetTextParamsBuilder<'a> {
    pub fn build(self) -> SetStyleSheetTextParams<'a> {
        SetStyleSheetTextParams {
            style_sheet_id: self.style_sheet_id,
            text: self.text,
        }
    }
}

/// Sets the new stylesheet text.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStyleSheetTextReturns<'a> {
    /// URL of source map associated with script (if any).
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceMapURL")]
    source_map_url: Option<Cow<'a, str>>,
}

impl<'a> SetStyleSheetTextReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> SetStyleSheetTextReturnsBuilder<'a> {
        SetStyleSheetTextReturnsBuilder {
            source_map_url: None,
        }
    }
    /// URL of source map associated with script (if any).
    pub fn source_map_url(&self) -> Option<&str> { self.source_map_url.as_deref() }
}

#[derive(Default)]
pub struct SetStyleSheetTextReturnsBuilder<'a> {
    source_map_url: Option<Cow<'a, str>>,
}

impl<'a> SetStyleSheetTextReturnsBuilder<'a> {
    /// URL of source map associated with script (if any).
    pub fn source_map_url(mut self, source_map_url: impl Into<Cow<'a, str>>) -> Self { self.source_map_url = Some(source_map_url.into()); self }
    pub fn build(self) -> SetStyleSheetTextReturns<'a> {
        SetStyleSheetTextReturns {
            source_map_url: self.source_map_url,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeForPropertySyntaxValidation")]
    node_for_property_syntax_validation: Option<crate::dom::NodeId>,
}

impl<'a> SetStyleTextsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `edits`: 
    pub fn builder(edits: Vec<StyleDeclarationEdit<'a>>) -> SetStyleTextsParamsBuilder<'a> {
        SetStyleTextsParamsBuilder {
            edits: edits,
            node_for_property_syntax_validation: None,
        }
    }
    pub fn edits(&self) -> &[StyleDeclarationEdit<'a>] { &self.edits }
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    pub fn node_for_property_syntax_validation(&self) -> Option<&crate::dom::NodeId> { self.node_for_property_syntax_validation.as_ref() }
}


pub struct SetStyleTextsParamsBuilder<'a> {
    edits: Vec<StyleDeclarationEdit<'a>>,
    node_for_property_syntax_validation: Option<crate::dom::NodeId>,
}

impl<'a> SetStyleTextsParamsBuilder<'a> {
    /// NodeId for the DOM node in whose context custom property declarations for registered properties should be
    /// validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    /// incorrect results if the declaration contains a var() for example.
    pub fn node_for_property_syntax_validation(mut self, node_for_property_syntax_validation: crate::dom::NodeId) -> Self { self.node_for_property_syntax_validation = Some(node_for_property_syntax_validation); self }
    pub fn build(self) -> SetStyleTextsParams<'a> {
        SetStyleTextsParams {
            edits: self.edits,
            node_for_property_syntax_validation: self.node_for_property_syntax_validation,
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
    /// Creates a builder for this type with the required parameters:
    /// * `styles`: The resulting styles after modification.
    pub fn builder(styles: Vec<CSSStyle<'a>>) -> SetStyleTextsReturnsBuilder<'a> {
        SetStyleTextsReturnsBuilder {
            styles: styles,
        }
    }
    /// The resulting styles after modification.
    pub fn styles(&self) -> &[CSSStyle<'a>] { &self.styles }
}


pub struct SetStyleTextsReturnsBuilder<'a> {
    styles: Vec<CSSStyle<'a>>,
}

impl<'a> SetStyleTextsReturnsBuilder<'a> {
    pub fn build(self) -> SetStyleTextsReturns<'a> {
        SetStyleTextsReturns {
            styles: self.styles,
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
    #[serde(rename = "ruleUsage")]
    rule_usage: Vec<RuleUsage<'a>>,
}

impl<'a> StopRuleUsageTrackingReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `rule_usage`: 
    pub fn builder(rule_usage: Vec<RuleUsage<'a>>) -> StopRuleUsageTrackingReturnsBuilder<'a> {
        StopRuleUsageTrackingReturnsBuilder {
            rule_usage: rule_usage,
        }
    }
    pub fn rule_usage(&self) -> &[RuleUsage<'a>] { &self.rule_usage }
}


pub struct StopRuleUsageTrackingReturnsBuilder<'a> {
    rule_usage: Vec<RuleUsage<'a>>,
}

impl<'a> StopRuleUsageTrackingReturnsBuilder<'a> {
    pub fn build(self) -> StopRuleUsageTrackingReturns<'a> {
        StopRuleUsageTrackingReturns {
            rule_usage: self.rule_usage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopRuleUsageTrackingParams {}

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
    /// Creates a builder for this type with the required parameters:
    /// * `coverage`: 
    /// * `timestamp`: Monotonically increasing time, in seconds.
    pub fn builder(coverage: Vec<RuleUsage<'a>>, timestamp: f64) -> TakeCoverageDeltaReturnsBuilder<'a> {
        TakeCoverageDeltaReturnsBuilder {
            coverage: coverage,
            timestamp: timestamp,
        }
    }
    pub fn coverage(&self) -> &[RuleUsage<'a>] { &self.coverage }
    /// Monotonically increasing time, in seconds.
    pub fn timestamp(&self) -> f64 { self.timestamp }
}


pub struct TakeCoverageDeltaReturnsBuilder<'a> {
    coverage: Vec<RuleUsage<'a>>,
    timestamp: f64,
}

impl<'a> TakeCoverageDeltaReturnsBuilder<'a> {
    pub fn build(self) -> TakeCoverageDeltaReturns<'a> {
        TakeCoverageDeltaReturns {
            coverage: self.coverage,
            timestamp: self.timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TakeCoverageDeltaParams {}

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
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: Whether rendering of local fonts is enabled.
    pub fn builder(enabled: bool) -> SetLocalFontsEnabledParamsBuilder {
        SetLocalFontsEnabledParamsBuilder {
            enabled: enabled,
        }
    }
    /// Whether rendering of local fonts is enabled.
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetLocalFontsEnabledParamsBuilder {
    enabled: bool,
}

impl SetLocalFontsEnabledParamsBuilder {
    pub fn build(self) -> SetLocalFontsEnabledParams {
        SetLocalFontsEnabledParams {
            enabled: self.enabled,
        }
    }
}

impl SetLocalFontsEnabledParams { pub const METHOD: &'static str = "CSS.setLocalFontsEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetLocalFontsEnabledParams {
    const METHOD: &'static str = "CSS.setLocalFontsEnabled";
    type Response = crate::EmptyReturns;
}
