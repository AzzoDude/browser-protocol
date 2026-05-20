//! Audits domain allows investigation of page violations and possible improvements.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Information about a cookie that is affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AffectedCookie<'a> {
    /// The following three properties uniquely identify a cookie
    name: Cow<'a, str>,
    path: Cow<'a, str>,
    domain: Cow<'a, str>,
}

impl<'a> AffectedCookie<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: The following three properties uniquely identify a cookie
    /// * `path`: 
    /// * `domain`: 
    pub fn builder(name: impl Into<Cow<'a, str>>, path: impl Into<Cow<'a, str>>, domain: impl Into<Cow<'a, str>>) -> AffectedCookieBuilder<'a> {
        AffectedCookieBuilder {
            name: name.into(),
            path: path.into(),
            domain: domain.into(),
        }
    }
    /// The following three properties uniquely identify a cookie
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn path(&self) -> &str { self.path.as_ref() }
    pub fn domain(&self) -> &str { self.domain.as_ref() }
}


pub struct AffectedCookieBuilder<'a> {
    name: Cow<'a, str>,
    path: Cow<'a, str>,
    domain: Cow<'a, str>,
}

impl<'a> AffectedCookieBuilder<'a> {
    pub fn build(self) -> AffectedCookie<'a> {
        AffectedCookie {
            name: self.name,
            path: self.path,
            domain: self.domain,
        }
    }
}

/// Information about a request that is affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AffectedRequest<'a> {
    /// The unique request id.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestId")]
    request_id: Option<crate::network::RequestId<'a>>,
    url: Cow<'a, str>,
}

impl<'a> AffectedRequest<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: 
    pub fn builder(url: impl Into<Cow<'a, str>>) -> AffectedRequestBuilder<'a> {
        AffectedRequestBuilder {
            request_id: None,
            url: url.into(),
        }
    }
    /// The unique request id.
    pub fn request_id(&self) -> Option<&crate::network::RequestId<'a>> { self.request_id.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}


pub struct AffectedRequestBuilder<'a> {
    request_id: Option<crate::network::RequestId<'a>>,
    url: Cow<'a, str>,
}

impl<'a> AffectedRequestBuilder<'a> {
    /// The unique request id.
    pub fn request_id(mut self, request_id: crate::network::RequestId<'a>) -> Self { self.request_id = Some(request_id); self }
    pub fn build(self) -> AffectedRequest<'a> {
        AffectedRequest {
            request_id: self.request_id,
            url: self.url,
        }
    }
}

/// Information about the frame affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AffectedFrame<'a> {
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
}

impl<'a> AffectedFrame<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: 
    pub fn builder(frame_id: crate::page::FrameId<'a>) -> AffectedFrameBuilder<'a> {
        AffectedFrameBuilder {
            frame_id: frame_id,
        }
    }
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
}


pub struct AffectedFrameBuilder<'a> {
    frame_id: crate::page::FrameId<'a>,
}

impl<'a> AffectedFrameBuilder<'a> {
    pub fn build(self) -> AffectedFrame<'a> {
        AffectedFrame {
            frame_id: self.frame_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieExclusionReason {
    #[default]
    #[serde(rename = "ExcludeSameSiteUnspecifiedTreatedAsLax")]
    ExcludeSameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "ExcludeSameSiteNoneInsecure")]
    ExcludeSameSiteNoneInsecure,
    #[serde(rename = "ExcludeSameSiteLax")]
    ExcludeSameSiteLax,
    #[serde(rename = "ExcludeSameSiteStrict")]
    ExcludeSameSiteStrict,
    #[serde(rename = "ExcludeDomainNonASCII")]
    ExcludeDomainNonASCII,
    #[serde(rename = "ExcludeThirdPartyCookieBlockedInFirstPartySet")]
    ExcludeThirdPartyCookieBlockedInFirstPartySet,
    #[serde(rename = "ExcludeThirdPartyPhaseout")]
    ExcludeThirdPartyPhaseout,
    #[serde(rename = "ExcludePortMismatch")]
    ExcludePortMismatch,
    #[serde(rename = "ExcludeSchemeMismatch")]
    ExcludeSchemeMismatch,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieWarningReason {
    #[default]
    #[serde(rename = "WarnSameSiteUnspecifiedCrossSiteContext")]
    WarnSameSiteUnspecifiedCrossSiteContext,
    #[serde(rename = "WarnSameSiteNoneInsecure")]
    WarnSameSiteNoneInsecure,
    #[serde(rename = "WarnSameSiteUnspecifiedLaxAllowUnsafe")]
    WarnSameSiteUnspecifiedLaxAllowUnsafe,
    #[serde(rename = "WarnSameSiteStrictLaxDowngradeStrict")]
    WarnSameSiteStrictLaxDowngradeStrict,
    #[serde(rename = "WarnSameSiteStrictCrossDowngradeStrict")]
    WarnSameSiteStrictCrossDowngradeStrict,
    #[serde(rename = "WarnSameSiteStrictCrossDowngradeLax")]
    WarnSameSiteStrictCrossDowngradeLax,
    #[serde(rename = "WarnSameSiteLaxCrossDowngradeStrict")]
    WarnSameSiteLaxCrossDowngradeStrict,
    #[serde(rename = "WarnSameSiteLaxCrossDowngradeLax")]
    WarnSameSiteLaxCrossDowngradeLax,
    #[serde(rename = "WarnAttributeValueExceedsMaxSize")]
    WarnAttributeValueExceedsMaxSize,
    #[serde(rename = "WarnDomainNonASCII")]
    WarnDomainNonASCII,
    #[serde(rename = "WarnThirdPartyPhaseout")]
    WarnThirdPartyPhaseout,
    #[serde(rename = "WarnCrossSiteRedirectDowngradeChangesInclusion")]
    WarnCrossSiteRedirectDowngradeChangesInclusion,
    #[serde(rename = "WarnDeprecationTrialMetadata")]
    WarnDeprecationTrialMetadata,
    #[serde(rename = "WarnThirdPartyCookieHeuristic")]
    WarnThirdPartyCookieHeuristic,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieOperation {
    #[default]
    #[serde(rename = "SetCookie")]
    SetCookie,
    #[serde(rename = "ReadCookie")]
    ReadCookie,
}

/// Represents the category of insight that a cookie issue falls under.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InsightType {
    #[default]
    #[serde(rename = "GitHubResource")]
    GitHubResource,
    #[serde(rename = "GracePeriod")]
    GracePeriod,
    #[serde(rename = "Heuristics")]
    Heuristics,
}

/// Information about the suggested solution to a cookie issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookieIssueInsight<'a> {
    #[serde(rename = "type")]
    type_: InsightType,
    /// Link to table entry in third-party cookie migration readiness list.
    #[serde(skip_serializing_if = "Option::is_none", rename = "tableEntryUrl")]
    table_entry_url: Option<Cow<'a, str>>,
}

impl<'a> CookieIssueInsight<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: 
    pub fn builder(type_: impl Into<InsightType>) -> CookieIssueInsightBuilder<'a> {
        CookieIssueInsightBuilder {
            type_: type_.into(),
            table_entry_url: None,
        }
    }
    pub fn type_(&self) -> &InsightType { &self.type_ }
    /// Link to table entry in third-party cookie migration readiness list.
    pub fn table_entry_url(&self) -> Option<&str> { self.table_entry_url.as_deref() }
}


pub struct CookieIssueInsightBuilder<'a> {
    type_: InsightType,
    table_entry_url: Option<Cow<'a, str>>,
}

impl<'a> CookieIssueInsightBuilder<'a> {
    /// Link to table entry in third-party cookie migration readiness list.
    pub fn table_entry_url(mut self, table_entry_url: impl Into<Cow<'a, str>>) -> Self { self.table_entry_url = Some(table_entry_url.into()); self }
    pub fn build(self) -> CookieIssueInsight<'a> {
        CookieIssueInsight {
            type_: self.type_,
            table_entry_url: self.table_entry_url,
        }
    }
}

/// This information is currently necessary, as the front-end has a difficult
/// time finding a specific cookie. With this, we can convey specific error
/// information without the cookie.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookieIssueDetails<'a> {
    /// If AffectedCookie is not set then rawCookieLine contains the raw
    /// Set-Cookie header string. This hints at a problem where the
    /// cookie line is syntactically or semantically malformed in a way
    /// that no valid cookie could be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie: Option<AffectedCookie<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "rawCookieLine")]
    raw_cookie_line: Option<Cow<'a, str>>,
    #[serde(rename = "cookieWarningReasons")]
    cookie_warning_reasons: Vec<CookieWarningReason>,
    #[serde(rename = "cookieExclusionReasons")]
    cookie_exclusion_reasons: Vec<CookieExclusionReason>,
    /// Optionally identifies the site-for-cookies and the cookie url, which
    /// may be used by the front-end as additional context.
    operation: CookieOperation,
    #[serde(skip_serializing_if = "Option::is_none", rename = "siteForCookies")]
    site_for_cookies: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cookieUrl")]
    cookie_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AffectedRequest<'a>>,
    /// The recommended solution to the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    insight: Option<CookieIssueInsight<'a>>,
}

impl<'a> CookieIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cookie_warning_reasons`: 
    /// * `cookie_exclusion_reasons`: 
    /// * `operation`: Optionally identifies the site-for-cookies and the cookie url, which may be used by the front-end as additional context.
    pub fn builder(cookie_warning_reasons: Vec<CookieWarningReason>, cookie_exclusion_reasons: Vec<CookieExclusionReason>, operation: impl Into<CookieOperation>) -> CookieIssueDetailsBuilder<'a> {
        CookieIssueDetailsBuilder {
            cookie: None,
            raw_cookie_line: None,
            cookie_warning_reasons: cookie_warning_reasons,
            cookie_exclusion_reasons: cookie_exclusion_reasons,
            operation: operation.into(),
            site_for_cookies: None,
            cookie_url: None,
            request: None,
            insight: None,
        }
    }
    /// If AffectedCookie is not set then rawCookieLine contains the raw
    /// Set-Cookie header string. This hints at a problem where the
    /// cookie line is syntactically or semantically malformed in a way
    /// that no valid cookie could be created.
    pub fn cookie(&self) -> Option<&AffectedCookie<'a>> { self.cookie.as_ref() }
    pub fn raw_cookie_line(&self) -> Option<&str> { self.raw_cookie_line.as_deref() }
    pub fn cookie_warning_reasons(&self) -> &[CookieWarningReason] { &self.cookie_warning_reasons }
    pub fn cookie_exclusion_reasons(&self) -> &[CookieExclusionReason] { &self.cookie_exclusion_reasons }
    /// Optionally identifies the site-for-cookies and the cookie url, which
    /// may be used by the front-end as additional context.
    pub fn operation(&self) -> &CookieOperation { &self.operation }
    pub fn site_for_cookies(&self) -> Option<&str> { self.site_for_cookies.as_deref() }
    pub fn cookie_url(&self) -> Option<&str> { self.cookie_url.as_deref() }
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
    /// The recommended solution to the issue.
    pub fn insight(&self) -> Option<&CookieIssueInsight<'a>> { self.insight.as_ref() }
}


pub struct CookieIssueDetailsBuilder<'a> {
    cookie: Option<AffectedCookie<'a>>,
    raw_cookie_line: Option<Cow<'a, str>>,
    cookie_warning_reasons: Vec<CookieWarningReason>,
    cookie_exclusion_reasons: Vec<CookieExclusionReason>,
    operation: CookieOperation,
    site_for_cookies: Option<Cow<'a, str>>,
    cookie_url: Option<Cow<'a, str>>,
    request: Option<AffectedRequest<'a>>,
    insight: Option<CookieIssueInsight<'a>>,
}

impl<'a> CookieIssueDetailsBuilder<'a> {
    /// If AffectedCookie is not set then rawCookieLine contains the raw
    /// Set-Cookie header string. This hints at a problem where the
    /// cookie line is syntactically or semantically malformed in a way
    /// that no valid cookie could be created.
    pub fn cookie(mut self, cookie: AffectedCookie<'a>) -> Self { self.cookie = Some(cookie); self }
    pub fn raw_cookie_line(mut self, raw_cookie_line: impl Into<Cow<'a, str>>) -> Self { self.raw_cookie_line = Some(raw_cookie_line.into()); self }
    pub fn site_for_cookies(mut self, site_for_cookies: impl Into<Cow<'a, str>>) -> Self { self.site_for_cookies = Some(site_for_cookies.into()); self }
    pub fn cookie_url(mut self, cookie_url: impl Into<Cow<'a, str>>) -> Self { self.cookie_url = Some(cookie_url.into()); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    /// The recommended solution to the issue.
    pub fn insight(mut self, insight: CookieIssueInsight<'a>) -> Self { self.insight = Some(insight); self }
    pub fn build(self) -> CookieIssueDetails<'a> {
        CookieIssueDetails {
            cookie: self.cookie,
            raw_cookie_line: self.raw_cookie_line,
            cookie_warning_reasons: self.cookie_warning_reasons,
            cookie_exclusion_reasons: self.cookie_exclusion_reasons,
            operation: self.operation,
            site_for_cookies: self.site_for_cookies,
            cookie_url: self.cookie_url,
            request: self.request,
            insight: self.insight,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PerformanceIssueType {
    #[default]
    #[serde(rename = "DocumentCookie")]
    DocumentCookie,
}

/// Details for a performance issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceIssueDetails<'a> {
    #[serde(rename = "performanceIssueType")]
    performance_issue_type: PerformanceIssueType,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceCodeLocation")]
    source_code_location: Option<SourceCodeLocation<'a>>,
}

impl<'a> PerformanceIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `performance_issue_type`: 
    pub fn builder(performance_issue_type: impl Into<PerformanceIssueType>) -> PerformanceIssueDetailsBuilder<'a> {
        PerformanceIssueDetailsBuilder {
            performance_issue_type: performance_issue_type.into(),
            source_code_location: None,
        }
    }
    pub fn performance_issue_type(&self) -> &PerformanceIssueType { &self.performance_issue_type }
    pub fn source_code_location(&self) -> Option<&SourceCodeLocation<'a>> { self.source_code_location.as_ref() }
}


pub struct PerformanceIssueDetailsBuilder<'a> {
    performance_issue_type: PerformanceIssueType,
    source_code_location: Option<SourceCodeLocation<'a>>,
}

impl<'a> PerformanceIssueDetailsBuilder<'a> {
    pub fn source_code_location(mut self, source_code_location: SourceCodeLocation<'a>) -> Self { self.source_code_location = Some(source_code_location); self }
    pub fn build(self) -> PerformanceIssueDetails<'a> {
        PerformanceIssueDetails {
            performance_issue_type: self.performance_issue_type,
            source_code_location: self.source_code_location,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MixedContentResolutionStatus {
    #[default]
    #[serde(rename = "MixedContentBlocked")]
    MixedContentBlocked,
    #[serde(rename = "MixedContentAutomaticallyUpgraded")]
    MixedContentAutomaticallyUpgraded,
    #[serde(rename = "MixedContentWarning")]
    MixedContentWarning,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MixedContentResourceType {
    #[default]
    #[serde(rename = "AttributionSrc")]
    AttributionSrc,
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Beacon")]
    Beacon,
    #[serde(rename = "CSPReport")]
    CSPReport,
    #[serde(rename = "Download")]
    Download,
    #[serde(rename = "EventSource")]
    EventSource,
    #[serde(rename = "Favicon")]
    Favicon,
    #[serde(rename = "Font")]
    Font,
    #[serde(rename = "Form")]
    Form,
    #[serde(rename = "Frame")]
    Frame,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Import")]
    Import,
    #[serde(rename = "JSON")]
    JSON,
    #[serde(rename = "Manifest")]
    Manifest,
    #[serde(rename = "Ping")]
    Ping,
    #[serde(rename = "PluginData")]
    PluginData,
    #[serde(rename = "PluginResource")]
    PluginResource,
    #[serde(rename = "Prefetch")]
    Prefetch,
    #[serde(rename = "Resource")]
    Resource,
    #[serde(rename = "Script")]
    Script,
    #[serde(rename = "ServiceWorker")]
    ServiceWorker,
    #[serde(rename = "SharedWorker")]
    SharedWorker,
    #[serde(rename = "SpeculationRules")]
    SpeculationRules,
    #[serde(rename = "Stylesheet")]
    Stylesheet,
    #[serde(rename = "Track")]
    Track,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Worker")]
    Worker,
    #[serde(rename = "XMLHttpRequest")]
    XMLHttpRequest,
    #[serde(rename = "XSLT")]
    XSLT,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MixedContentIssueDetails<'a> {
    /// The type of resource causing the mixed content issue (css, js, iframe,
    /// form,...). Marked as optional because it is mapped to from
    /// blink::mojom::RequestContextType, which will be replaced
    /// by network::mojom::RequestDestination
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType")]
    resource_type: Option<MixedContentResourceType>,
    /// The way the mixed content issue is being resolved.
    #[serde(rename = "resolutionStatus")]
    resolution_status: MixedContentResolutionStatus,
    /// The unsafe http url causing the mixed content issue.
    #[serde(rename = "insecureURL")]
    insecure_url: Cow<'a, str>,
    /// The url responsible for the call to an unsafe url.
    #[serde(rename = "mainResourceURL")]
    main_resource_url: Cow<'a, str>,
    /// The mixed content request.
    /// Does not always exist (e.g. for unsafe form submission urls).
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AffectedRequest<'a>>,
    /// Optional because not every mixed content issue is necessarily linked to a frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    frame: Option<AffectedFrame<'a>>,
}

impl<'a> MixedContentIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `resolution_status`: The way the mixed content issue is being resolved.
    /// * `insecure_url`: The unsafe http url causing the mixed content issue.
    /// * `main_resource_url`: The url responsible for the call to an unsafe url.
    pub fn builder(resolution_status: impl Into<MixedContentResolutionStatus>, insecure_url: impl Into<Cow<'a, str>>, main_resource_url: impl Into<Cow<'a, str>>) -> MixedContentIssueDetailsBuilder<'a> {
        MixedContentIssueDetailsBuilder {
            resource_type: None,
            resolution_status: resolution_status.into(),
            insecure_url: insecure_url.into(),
            main_resource_url: main_resource_url.into(),
            request: None,
            frame: None,
        }
    }
    /// The type of resource causing the mixed content issue (css, js, iframe,
    /// form,...). Marked as optional because it is mapped to from
    /// blink::mojom::RequestContextType, which will be replaced
    /// by network::mojom::RequestDestination
    pub fn resource_type(&self) -> Option<&MixedContentResourceType> { self.resource_type.as_ref() }
    /// The way the mixed content issue is being resolved.
    pub fn resolution_status(&self) -> &MixedContentResolutionStatus { &self.resolution_status }
    /// The unsafe http url causing the mixed content issue.
    pub fn insecure_url(&self) -> &str { self.insecure_url.as_ref() }
    /// The url responsible for the call to an unsafe url.
    pub fn main_resource_url(&self) -> &str { self.main_resource_url.as_ref() }
    /// The mixed content request.
    /// Does not always exist (e.g. for unsafe form submission urls).
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
    /// Optional because not every mixed content issue is necessarily linked to a frame.
    pub fn frame(&self) -> Option<&AffectedFrame<'a>> { self.frame.as_ref() }
}


pub struct MixedContentIssueDetailsBuilder<'a> {
    resource_type: Option<MixedContentResourceType>,
    resolution_status: MixedContentResolutionStatus,
    insecure_url: Cow<'a, str>,
    main_resource_url: Cow<'a, str>,
    request: Option<AffectedRequest<'a>>,
    frame: Option<AffectedFrame<'a>>,
}

impl<'a> MixedContentIssueDetailsBuilder<'a> {
    /// The type of resource causing the mixed content issue (css, js, iframe,
    /// form,...). Marked as optional because it is mapped to from
    /// blink::mojom::RequestContextType, which will be replaced
    /// by network::mojom::RequestDestination
    pub fn resource_type(mut self, resource_type: impl Into<MixedContentResourceType>) -> Self { self.resource_type = Some(resource_type.into()); self }
    /// The mixed content request.
    /// Does not always exist (e.g. for unsafe form submission urls).
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    /// Optional because not every mixed content issue is necessarily linked to a frame.
    pub fn frame(mut self, frame: AffectedFrame<'a>) -> Self { self.frame = Some(frame); self }
    pub fn build(self) -> MixedContentIssueDetails<'a> {
        MixedContentIssueDetails {
            resource_type: self.resource_type,
            resolution_status: self.resolution_status,
            insecure_url: self.insecure_url,
            main_resource_url: self.main_resource_url,
            request: self.request,
            frame: self.frame,
        }
    }
}

/// Enum indicating the reason a response has been blocked. These reasons are
/// refinements of the net error BLOCKED_BY_RESPONSE.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BlockedByResponseReason {
    #[default]
    #[serde(rename = "CoepFrameResourceNeedsCoepHeader")]
    CoepFrameResourceNeedsCoepHeader,
    #[serde(rename = "CoopSandboxedIFrameCannotNavigateToCoopPage")]
    CoopSandboxedIFrameCannotNavigateToCoopPage,
    #[serde(rename = "CorpNotSameOrigin")]
    CorpNotSameOrigin,
    #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByCoep")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByDip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    #[serde(rename = "CorpNotSameSite")]
    CorpNotSameSite,
    #[serde(rename = "SRIMessageSignatureMismatch")]
    SRIMessageSignatureMismatch,
}

/// Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
/// code. Currently only used for COEP/COOP, but may be extended to include
/// some CSP errors in the future.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockedByResponseIssueDetails<'a> {
    request: AffectedRequest<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentFrame")]
    parent_frame: Option<AffectedFrame<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockedFrame")]
    blocked_frame: Option<AffectedFrame<'a>>,
    reason: BlockedByResponseReason,
}

impl<'a> BlockedByResponseIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request`: 
    /// * `reason`: 
    pub fn builder(request: AffectedRequest<'a>, reason: impl Into<BlockedByResponseReason>) -> BlockedByResponseIssueDetailsBuilder<'a> {
        BlockedByResponseIssueDetailsBuilder {
            request: request,
            parent_frame: None,
            blocked_frame: None,
            reason: reason.into(),
        }
    }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
    pub fn parent_frame(&self) -> Option<&AffectedFrame<'a>> { self.parent_frame.as_ref() }
    pub fn blocked_frame(&self) -> Option<&AffectedFrame<'a>> { self.blocked_frame.as_ref() }
    pub fn reason(&self) -> &BlockedByResponseReason { &self.reason }
}


pub struct BlockedByResponseIssueDetailsBuilder<'a> {
    request: AffectedRequest<'a>,
    parent_frame: Option<AffectedFrame<'a>>,
    blocked_frame: Option<AffectedFrame<'a>>,
    reason: BlockedByResponseReason,
}

impl<'a> BlockedByResponseIssueDetailsBuilder<'a> {
    pub fn parent_frame(mut self, parent_frame: AffectedFrame<'a>) -> Self { self.parent_frame = Some(parent_frame); self }
    pub fn blocked_frame(mut self, blocked_frame: AffectedFrame<'a>) -> Self { self.blocked_frame = Some(blocked_frame); self }
    pub fn build(self) -> BlockedByResponseIssueDetails<'a> {
        BlockedByResponseIssueDetails {
            request: self.request,
            parent_frame: self.parent_frame,
            blocked_frame: self.blocked_frame,
            reason: self.reason,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum HeavyAdResolutionStatus {
    #[default]
    #[serde(rename = "HeavyAdBlocked")]
    HeavyAdBlocked,
    #[serde(rename = "HeavyAdWarning")]
    HeavyAdWarning,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum HeavyAdReason {
    #[default]
    #[serde(rename = "NetworkTotalLimit")]
    NetworkTotalLimit,
    #[serde(rename = "CpuTotalLimit")]
    CpuTotalLimit,
    #[serde(rename = "CpuPeakLimit")]
    CpuPeakLimit,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeavyAdIssueDetails<'a> {
    /// The resolution status, either blocking the content or warning.
    resolution: HeavyAdResolutionStatus,
    /// The reason the ad was blocked, total network or cpu or peak cpu.
    reason: HeavyAdReason,
    /// The frame that was blocked.
    frame: AffectedFrame<'a>,
}

impl<'a> HeavyAdIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `resolution`: The resolution status, either blocking the content or warning.
    /// * `reason`: The reason the ad was blocked, total network or cpu or peak cpu.
    /// * `frame`: The frame that was blocked.
    pub fn builder(resolution: impl Into<HeavyAdResolutionStatus>, reason: impl Into<HeavyAdReason>, frame: AffectedFrame<'a>) -> HeavyAdIssueDetailsBuilder<'a> {
        HeavyAdIssueDetailsBuilder {
            resolution: resolution.into(),
            reason: reason.into(),
            frame: frame,
        }
    }
    /// The resolution status, either blocking the content or warning.
    pub fn resolution(&self) -> &HeavyAdResolutionStatus { &self.resolution }
    /// The reason the ad was blocked, total network or cpu or peak cpu.
    pub fn reason(&self) -> &HeavyAdReason { &self.reason }
    /// The frame that was blocked.
    pub fn frame(&self) -> &AffectedFrame<'a> { &self.frame }
}


pub struct HeavyAdIssueDetailsBuilder<'a> {
    resolution: HeavyAdResolutionStatus,
    reason: HeavyAdReason,
    frame: AffectedFrame<'a>,
}

impl<'a> HeavyAdIssueDetailsBuilder<'a> {
    pub fn build(self) -> HeavyAdIssueDetails<'a> {
        HeavyAdIssueDetails {
            resolution: self.resolution,
            reason: self.reason,
            frame: self.frame,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContentSecurityPolicyViolationType {
    #[default]
    #[serde(rename = "kInlineViolation")]
    KInlineViolation,
    #[serde(rename = "kEvalViolation")]
    KEvalViolation,
    #[serde(rename = "kURLViolation")]
    KURLViolation,
    #[serde(rename = "kSRIViolation")]
    KSRIViolation,
    #[serde(rename = "kTrustedTypesSinkViolation")]
    KTrustedTypesSinkViolation,
    #[serde(rename = "kTrustedTypesPolicyViolation")]
    KTrustedTypesPolicyViolation,
    #[serde(rename = "kWasmEvalViolation")]
    KWasmEvalViolation,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceCodeLocation<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptId")]
    script_id: Option<crate::runtime::ScriptId<'a>>,
    url: Cow<'a, str>,
    #[serde(rename = "lineNumber")]
    line_number: i64,
    #[serde(rename = "columnNumber")]
    column_number: i64,
}

impl<'a> SourceCodeLocation<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: 
    /// * `line_number`: 
    /// * `column_number`: 
    pub fn builder(url: impl Into<Cow<'a, str>>, line_number: i64, column_number: i64) -> SourceCodeLocationBuilder<'a> {
        SourceCodeLocationBuilder {
            script_id: None,
            url: url.into(),
            line_number: line_number,
            column_number: column_number,
        }
    }
    pub fn script_id(&self) -> Option<&crate::runtime::ScriptId<'a>> { self.script_id.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn line_number(&self) -> i64 { self.line_number }
    pub fn column_number(&self) -> i64 { self.column_number }
}


pub struct SourceCodeLocationBuilder<'a> {
    script_id: Option<crate::runtime::ScriptId<'a>>,
    url: Cow<'a, str>,
    line_number: i64,
    column_number: i64,
}

impl<'a> SourceCodeLocationBuilder<'a> {
    pub fn script_id(mut self, script_id: crate::runtime::ScriptId<'a>) -> Self { self.script_id = Some(script_id); self }
    pub fn build(self) -> SourceCodeLocation<'a> {
        SourceCodeLocation {
            script_id: self.script_id,
            url: self.url,
            line_number: self.line_number,
            column_number: self.column_number,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyIssueDetails<'a> {
    /// The url not included in allowed sources.
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockedURL")]
    blocked_url: Option<Cow<'a, str>>,
    /// Specific directive that is violated, causing the CSP issue.
    #[serde(rename = "violatedDirective")]
    violated_directive: Cow<'a, str>,
    #[serde(rename = "isReportOnly")]
    is_report_only: bool,
    #[serde(rename = "contentSecurityPolicyViolationType")]
    content_security_policy_violation_type: ContentSecurityPolicyViolationType,
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameAncestor")]
    frame_ancestor: Option<AffectedFrame<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceCodeLocation")]
    source_code_location: Option<SourceCodeLocation<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "violatingNodeId")]
    violating_node_id: Option<crate::dom::BackendNodeId>,
}

impl<'a> ContentSecurityPolicyIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `violated_directive`: Specific directive that is violated, causing the CSP issue.
    /// * `is_report_only`: 
    /// * `content_security_policy_violation_type`: 
    pub fn builder(violated_directive: impl Into<Cow<'a, str>>, is_report_only: bool, content_security_policy_violation_type: impl Into<ContentSecurityPolicyViolationType>) -> ContentSecurityPolicyIssueDetailsBuilder<'a> {
        ContentSecurityPolicyIssueDetailsBuilder {
            blocked_url: None,
            violated_directive: violated_directive.into(),
            is_report_only: is_report_only,
            content_security_policy_violation_type: content_security_policy_violation_type.into(),
            frame_ancestor: None,
            source_code_location: None,
            violating_node_id: None,
        }
    }
    /// The url not included in allowed sources.
    pub fn blocked_url(&self) -> Option<&str> { self.blocked_url.as_deref() }
    /// Specific directive that is violated, causing the CSP issue.
    pub fn violated_directive(&self) -> &str { self.violated_directive.as_ref() }
    pub fn is_report_only(&self) -> bool { self.is_report_only }
    pub fn content_security_policy_violation_type(&self) -> &ContentSecurityPolicyViolationType { &self.content_security_policy_violation_type }
    pub fn frame_ancestor(&self) -> Option<&AffectedFrame<'a>> { self.frame_ancestor.as_ref() }
    pub fn source_code_location(&self) -> Option<&SourceCodeLocation<'a>> { self.source_code_location.as_ref() }
    pub fn violating_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.violating_node_id.as_ref() }
}


pub struct ContentSecurityPolicyIssueDetailsBuilder<'a> {
    blocked_url: Option<Cow<'a, str>>,
    violated_directive: Cow<'a, str>,
    is_report_only: bool,
    content_security_policy_violation_type: ContentSecurityPolicyViolationType,
    frame_ancestor: Option<AffectedFrame<'a>>,
    source_code_location: Option<SourceCodeLocation<'a>>,
    violating_node_id: Option<crate::dom::BackendNodeId>,
}

impl<'a> ContentSecurityPolicyIssueDetailsBuilder<'a> {
    /// The url not included in allowed sources.
    pub fn blocked_url(mut self, blocked_url: impl Into<Cow<'a, str>>) -> Self { self.blocked_url = Some(blocked_url.into()); self }
    pub fn frame_ancestor(mut self, frame_ancestor: AffectedFrame<'a>) -> Self { self.frame_ancestor = Some(frame_ancestor); self }
    pub fn source_code_location(mut self, source_code_location: SourceCodeLocation<'a>) -> Self { self.source_code_location = Some(source_code_location); self }
    pub fn violating_node_id(mut self, violating_node_id: crate::dom::BackendNodeId) -> Self { self.violating_node_id = Some(violating_node_id); self }
    pub fn build(self) -> ContentSecurityPolicyIssueDetails<'a> {
        ContentSecurityPolicyIssueDetails {
            blocked_url: self.blocked_url,
            violated_directive: self.violated_directive,
            is_report_only: self.is_report_only,
            content_security_policy_violation_type: self.content_security_policy_violation_type,
            frame_ancestor: self.frame_ancestor,
            source_code_location: self.source_code_location,
            violating_node_id: self.violating_node_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SharedArrayBufferIssueType {
    #[default]
    #[serde(rename = "TransferIssue")]
    TransferIssue,
    #[serde(rename = "CreationIssue")]
    CreationIssue,
}

/// Details for a issue arising from an SAB being instantiated in, or
/// transferred to a context that is not cross-origin isolated.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedArrayBufferIssueDetails<'a> {
    #[serde(rename = "sourceCodeLocation")]
    source_code_location: SourceCodeLocation<'a>,
    #[serde(rename = "isWarning")]
    is_warning: bool,
    #[serde(rename = "type")]
    type_: SharedArrayBufferIssueType,
}

impl<'a> SharedArrayBufferIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `source_code_location`: 
    /// * `is_warning`: 
    /// * `type_`: 
    pub fn builder(source_code_location: SourceCodeLocation<'a>, is_warning: bool, type_: impl Into<SharedArrayBufferIssueType>) -> SharedArrayBufferIssueDetailsBuilder<'a> {
        SharedArrayBufferIssueDetailsBuilder {
            source_code_location: source_code_location,
            is_warning: is_warning,
            type_: type_.into(),
        }
    }
    pub fn source_code_location(&self) -> &SourceCodeLocation<'a> { &self.source_code_location }
    pub fn is_warning(&self) -> bool { self.is_warning }
    pub fn type_(&self) -> &SharedArrayBufferIssueType { &self.type_ }
}


pub struct SharedArrayBufferIssueDetailsBuilder<'a> {
    source_code_location: SourceCodeLocation<'a>,
    is_warning: bool,
    type_: SharedArrayBufferIssueType,
}

impl<'a> SharedArrayBufferIssueDetailsBuilder<'a> {
    pub fn build(self) -> SharedArrayBufferIssueDetails<'a> {
        SharedArrayBufferIssueDetails {
            source_code_location: self.source_code_location,
            is_warning: self.is_warning,
            type_: self.type_,
        }
    }
}

/// Details for a CORS related issue, e.g. a warning or error related to
/// CORS RFC1918 enforcement.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CorsIssueDetails<'a> {
    #[serde(rename = "corsErrorStatus")]
    cors_error_status: crate::network::CorsErrorStatus<'a>,
    #[serde(rename = "isWarning")]
    is_warning: bool,
    request: AffectedRequest<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<SourceCodeLocation<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "initiatorOrigin")]
    initiator_origin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceIPAddressSpace")]
    resource_ip_address_space: Option<crate::network::IPAddressSpace>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientSecurityState")]
    client_security_state: Option<crate::network::ClientSecurityState>,
}

impl<'a> CorsIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cors_error_status`: 
    /// * `is_warning`: 
    /// * `request`: 
    pub fn builder(cors_error_status: crate::network::CorsErrorStatus<'a>, is_warning: bool, request: AffectedRequest<'a>) -> CorsIssueDetailsBuilder<'a> {
        CorsIssueDetailsBuilder {
            cors_error_status: cors_error_status,
            is_warning: is_warning,
            request: request,
            location: None,
            initiator_origin: None,
            resource_ip_address_space: None,
            client_security_state: None,
        }
    }
    pub fn cors_error_status(&self) -> &crate::network::CorsErrorStatus<'a> { &self.cors_error_status }
    pub fn is_warning(&self) -> bool { self.is_warning }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
    pub fn location(&self) -> Option<&SourceCodeLocation<'a>> { self.location.as_ref() }
    pub fn initiator_origin(&self) -> Option<&str> { self.initiator_origin.as_deref() }
    pub fn resource_ip_address_space(&self) -> Option<&crate::network::IPAddressSpace> { self.resource_ip_address_space.as_ref() }
    pub fn client_security_state(&self) -> Option<&crate::network::ClientSecurityState> { self.client_security_state.as_ref() }
}


pub struct CorsIssueDetailsBuilder<'a> {
    cors_error_status: crate::network::CorsErrorStatus<'a>,
    is_warning: bool,
    request: AffectedRequest<'a>,
    location: Option<SourceCodeLocation<'a>>,
    initiator_origin: Option<Cow<'a, str>>,
    resource_ip_address_space: Option<crate::network::IPAddressSpace>,
    client_security_state: Option<crate::network::ClientSecurityState>,
}

impl<'a> CorsIssueDetailsBuilder<'a> {
    pub fn location(mut self, location: SourceCodeLocation<'a>) -> Self { self.location = Some(location); self }
    pub fn initiator_origin(mut self, initiator_origin: impl Into<Cow<'a, str>>) -> Self { self.initiator_origin = Some(initiator_origin.into()); self }
    pub fn resource_ip_address_space(mut self, resource_ip_address_space: crate::network::IPAddressSpace) -> Self { self.resource_ip_address_space = Some(resource_ip_address_space); self }
    pub fn client_security_state(mut self, client_security_state: crate::network::ClientSecurityState) -> Self { self.client_security_state = Some(client_security_state); self }
    pub fn build(self) -> CorsIssueDetails<'a> {
        CorsIssueDetails {
            cors_error_status: self.cors_error_status,
            is_warning: self.is_warning,
            request: self.request,
            location: self.location,
            initiator_origin: self.initiator_origin,
            resource_ip_address_space: self.resource_ip_address_space,
            client_security_state: self.client_security_state,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AttributionReportingIssueType {
    #[default]
    #[serde(rename = "PermissionPolicyDisabled")]
    PermissionPolicyDisabled,
    #[serde(rename = "UntrustworthyReportingOrigin")]
    UntrustworthyReportingOrigin,
    #[serde(rename = "InsecureContext")]
    InsecureContext,
    #[serde(rename = "InvalidHeader")]
    InvalidHeader,
    #[serde(rename = "InvalidRegisterTriggerHeader")]
    InvalidRegisterTriggerHeader,
    #[serde(rename = "SourceAndTriggerHeaders")]
    SourceAndTriggerHeaders,
    #[serde(rename = "SourceIgnored")]
    SourceIgnored,
    #[serde(rename = "TriggerIgnored")]
    TriggerIgnored,
    #[serde(rename = "OsSourceIgnored")]
    OsSourceIgnored,
    #[serde(rename = "OsTriggerIgnored")]
    OsTriggerIgnored,
    #[serde(rename = "InvalidRegisterOsSourceHeader")]
    InvalidRegisterOsSourceHeader,
    #[serde(rename = "InvalidRegisterOsTriggerHeader")]
    InvalidRegisterOsTriggerHeader,
    #[serde(rename = "WebAndOsHeaders")]
    WebAndOsHeaders,
    #[serde(rename = "NoWebOrOsSupport")]
    NoWebOrOsSupport,
    #[serde(rename = "NavigationRegistrationWithoutTransientUserActivation")]
    NavigationRegistrationWithoutTransientUserActivation,
    #[serde(rename = "InvalidInfoHeader")]
    InvalidInfoHeader,
    #[serde(rename = "NoRegisterSourceHeader")]
    NoRegisterSourceHeader,
    #[serde(rename = "NoRegisterTriggerHeader")]
    NoRegisterTriggerHeader,
    #[serde(rename = "NoRegisterOsSourceHeader")]
    NoRegisterOsSourceHeader,
    #[serde(rename = "NoRegisterOsTriggerHeader")]
    NoRegisterOsTriggerHeader,
    #[serde(rename = "NavigationRegistrationUniqueScopeAlreadySet")]
    NavigationRegistrationUniqueScopeAlreadySet,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SharedDictionaryError {
    #[default]
    #[serde(rename = "UseErrorCrossOriginNoCorsRequest")]
    UseErrorCrossOriginNoCorsRequest,
    #[serde(rename = "UseErrorDictionaryLoadFailure")]
    UseErrorDictionaryLoadFailure,
    #[serde(rename = "UseErrorMatchingDictionaryNotUsed")]
    UseErrorMatchingDictionaryNotUsed,
    #[serde(rename = "UseErrorUnexpectedContentDictionaryHeader")]
    UseErrorUnexpectedContentDictionaryHeader,
    #[serde(rename = "WriteErrorCossOriginNoCorsRequest")]
    WriteErrorCossOriginNoCorsRequest,
    #[serde(rename = "WriteErrorDisallowedBySettings")]
    WriteErrorDisallowedBySettings,
    #[serde(rename = "WriteErrorExpiredResponse")]
    WriteErrorExpiredResponse,
    #[serde(rename = "WriteErrorFeatureDisabled")]
    WriteErrorFeatureDisabled,
    #[serde(rename = "WriteErrorInsufficientResources")]
    WriteErrorInsufficientResources,
    #[serde(rename = "WriteErrorInvalidMatchField")]
    WriteErrorInvalidMatchField,
    #[serde(rename = "WriteErrorInvalidStructuredHeader")]
    WriteErrorInvalidStructuredHeader,
    #[serde(rename = "WriteErrorInvalidTTLField")]
    WriteErrorInvalidTTLField,
    #[serde(rename = "WriteErrorNavigationRequest")]
    WriteErrorNavigationRequest,
    #[serde(rename = "WriteErrorNoMatchField")]
    WriteErrorNoMatchField,
    #[serde(rename = "WriteErrorNonIntegerTTLField")]
    WriteErrorNonIntegerTTLField,
    #[serde(rename = "WriteErrorNonListMatchDestField")]
    WriteErrorNonListMatchDestField,
    #[serde(rename = "WriteErrorNonSecureContext")]
    WriteErrorNonSecureContext,
    #[serde(rename = "WriteErrorNonStringIdField")]
    WriteErrorNonStringIdField,
    #[serde(rename = "WriteErrorNonStringInMatchDestList")]
    WriteErrorNonStringInMatchDestList,
    #[serde(rename = "WriteErrorNonStringMatchField")]
    WriteErrorNonStringMatchField,
    #[serde(rename = "WriteErrorNonTokenTypeField")]
    WriteErrorNonTokenTypeField,
    #[serde(rename = "WriteErrorRequestAborted")]
    WriteErrorRequestAborted,
    #[serde(rename = "WriteErrorShuttingDown")]
    WriteErrorShuttingDown,
    #[serde(rename = "WriteErrorTooLongIdField")]
    WriteErrorTooLongIdField,
    #[serde(rename = "WriteErrorUnsupportedType")]
    WriteErrorUnsupportedType,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SRIMessageSignatureError {
    #[default]
    #[serde(rename = "MissingSignatureHeader")]
    MissingSignatureHeader,
    #[serde(rename = "MissingSignatureInputHeader")]
    MissingSignatureInputHeader,
    #[serde(rename = "InvalidSignatureHeader")]
    InvalidSignatureHeader,
    #[serde(rename = "InvalidSignatureInputHeader")]
    InvalidSignatureInputHeader,
    #[serde(rename = "SignatureHeaderValueIsNotByteSequence")]
    SignatureHeaderValueIsNotByteSequence,
    #[serde(rename = "SignatureHeaderValueIsParameterized")]
    SignatureHeaderValueIsParameterized,
    #[serde(rename = "SignatureHeaderValueIsIncorrectLength")]
    SignatureHeaderValueIsIncorrectLength,
    #[serde(rename = "SignatureInputHeaderMissingLabel")]
    SignatureInputHeaderMissingLabel,
    #[serde(rename = "SignatureInputHeaderValueNotInnerList")]
    SignatureInputHeaderValueNotInnerList,
    #[serde(rename = "SignatureInputHeaderValueMissingComponents")]
    SignatureInputHeaderValueMissingComponents,
    #[serde(rename = "SignatureInputHeaderInvalidComponentType")]
    SignatureInputHeaderInvalidComponentType,
    #[serde(rename = "SignatureInputHeaderInvalidComponentName")]
    SignatureInputHeaderInvalidComponentName,
    #[serde(rename = "SignatureInputHeaderInvalidHeaderComponentParameter")]
    SignatureInputHeaderInvalidHeaderComponentParameter,
    #[serde(rename = "SignatureInputHeaderInvalidDerivedComponentParameter")]
    SignatureInputHeaderInvalidDerivedComponentParameter,
    #[serde(rename = "SignatureInputHeaderKeyIdLength")]
    SignatureInputHeaderKeyIdLength,
    #[serde(rename = "SignatureInputHeaderInvalidParameter")]
    SignatureInputHeaderInvalidParameter,
    #[serde(rename = "SignatureInputHeaderMissingRequiredParameters")]
    SignatureInputHeaderMissingRequiredParameters,
    #[serde(rename = "ValidationFailedSignatureExpired")]
    ValidationFailedSignatureExpired,
    #[serde(rename = "ValidationFailedInvalidLength")]
    ValidationFailedInvalidLength,
    #[serde(rename = "ValidationFailedSignatureMismatch")]
    ValidationFailedSignatureMismatch,
    #[serde(rename = "ValidationFailedIntegrityMismatch")]
    ValidationFailedIntegrityMismatch,
    #[serde(rename = "SignatureBaseUnknownDerivedComponent")]
    SignatureBaseUnknownDerivedComponent,
    #[serde(rename = "SignatureBaseMissingHeader")]
    SignatureBaseMissingHeader,
    #[serde(rename = "SignatureBaseInvalidUnencodedDigest")]
    SignatureBaseInvalidUnencodedDigest,
    #[serde(rename = "SignatureBaseUnsupportedComponent")]
    SignatureBaseUnsupportedComponent,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum UnencodedDigestError {
    #[default]
    #[serde(rename = "MalformedDictionary")]
    MalformedDictionary,
    #[serde(rename = "UnknownAlgorithm")]
    UnknownAlgorithm,
    #[serde(rename = "IncorrectDigestType")]
    IncorrectDigestType,
    #[serde(rename = "IncorrectDigestLength")]
    IncorrectDigestLength,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ConnectionAllowlistError {
    #[default]
    #[serde(rename = "InvalidHeader")]
    InvalidHeader,
    #[serde(rename = "MoreThanOneList")]
    MoreThanOneList,
    #[serde(rename = "ItemNotInnerList")]
    ItemNotInnerList,
    #[serde(rename = "InvalidAllowlistItemType")]
    InvalidAllowlistItemType,
    #[serde(rename = "ReportingEndpointNotToken")]
    ReportingEndpointNotToken,
    #[serde(rename = "InvalidUrlPattern")]
    InvalidUrlPattern,
}

/// Details for issues around "Attribution Reporting API" usage.
/// Explainer: <https://github.com/WICG/attribution-reporting-api>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingIssueDetails<'a> {
    #[serde(rename = "violationType")]
    violation_type: AttributionReportingIssueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AffectedRequest<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "violatingNodeId")]
    violating_node_id: Option<crate::dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "invalidParameter")]
    invalid_parameter: Option<Cow<'a, str>>,
}

impl<'a> AttributionReportingIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `violation_type`: 
    pub fn builder(violation_type: impl Into<AttributionReportingIssueType>) -> AttributionReportingIssueDetailsBuilder<'a> {
        AttributionReportingIssueDetailsBuilder {
            violation_type: violation_type.into(),
            request: None,
            violating_node_id: None,
            invalid_parameter: None,
        }
    }
    pub fn violation_type(&self) -> &AttributionReportingIssueType { &self.violation_type }
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
    pub fn violating_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.violating_node_id.as_ref() }
    pub fn invalid_parameter(&self) -> Option<&str> { self.invalid_parameter.as_deref() }
}


pub struct AttributionReportingIssueDetailsBuilder<'a> {
    violation_type: AttributionReportingIssueType,
    request: Option<AffectedRequest<'a>>,
    violating_node_id: Option<crate::dom::BackendNodeId>,
    invalid_parameter: Option<Cow<'a, str>>,
}

impl<'a> AttributionReportingIssueDetailsBuilder<'a> {
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn violating_node_id(mut self, violating_node_id: crate::dom::BackendNodeId) -> Self { self.violating_node_id = Some(violating_node_id); self }
    pub fn invalid_parameter(mut self, invalid_parameter: impl Into<Cow<'a, str>>) -> Self { self.invalid_parameter = Some(invalid_parameter.into()); self }
    pub fn build(self) -> AttributionReportingIssueDetails<'a> {
        AttributionReportingIssueDetails {
            violation_type: self.violation_type,
            request: self.request,
            violating_node_id: self.violating_node_id,
            invalid_parameter: self.invalid_parameter,
        }
    }
}

/// Details for issues about documents in Quirks Mode
/// or Limited Quirks Mode that affects page layouting.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuirksModeIssueDetails<'a> {
    /// If false, it means the document's mode is "quirks"
    /// instead of "limited-quirks".
    #[serde(rename = "isLimitedQuirksMode")]
    is_limited_quirks_mode: bool,
    #[serde(rename = "documentNodeId")]
    document_node_id: crate::dom::BackendNodeId,
    url: Cow<'a, str>,
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
    #[serde(rename = "loaderId")]
    loader_id: crate::network::LoaderId<'a>,
}

impl<'a> QuirksModeIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `is_limited_quirks_mode`: If false, it means the document's mode is "quirks" instead of "limited-quirks".
    /// * `document_node_id`: 
    /// * `url`: 
    /// * `frame_id`: 
    /// * `loader_id`: 
    pub fn builder(is_limited_quirks_mode: bool, document_node_id: crate::dom::BackendNodeId, url: impl Into<Cow<'a, str>>, frame_id: crate::page::FrameId<'a>, loader_id: crate::network::LoaderId<'a>) -> QuirksModeIssueDetailsBuilder<'a> {
        QuirksModeIssueDetailsBuilder {
            is_limited_quirks_mode: is_limited_quirks_mode,
            document_node_id: document_node_id,
            url: url.into(),
            frame_id: frame_id,
            loader_id: loader_id,
        }
    }
    /// If false, it means the document's mode is "quirks"
    /// instead of "limited-quirks".
    pub fn is_limited_quirks_mode(&self) -> bool { self.is_limited_quirks_mode }
    pub fn document_node_id(&self) -> &crate::dom::BackendNodeId { &self.document_node_id }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
    pub fn loader_id(&self) -> &crate::network::LoaderId<'a> { &self.loader_id }
}


pub struct QuirksModeIssueDetailsBuilder<'a> {
    is_limited_quirks_mode: bool,
    document_node_id: crate::dom::BackendNodeId,
    url: Cow<'a, str>,
    frame_id: crate::page::FrameId<'a>,
    loader_id: crate::network::LoaderId<'a>,
}

impl<'a> QuirksModeIssueDetailsBuilder<'a> {
    pub fn build(self) -> QuirksModeIssueDetails<'a> {
        QuirksModeIssueDetails {
            is_limited_quirks_mode: self.is_limited_quirks_mode,
            document_node_id: self.document_node_id,
            url: self.url,
            frame_id: self.frame_id,
            loader_id: self.loader_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigatorUserAgentIssueDetails<'a> {
    url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<SourceCodeLocation<'a>>,
}

impl<'a> NavigatorUserAgentIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: 
    pub fn builder(url: impl Into<Cow<'a, str>>) -> NavigatorUserAgentIssueDetailsBuilder<'a> {
        NavigatorUserAgentIssueDetailsBuilder {
            url: url.into(),
            location: None,
        }
    }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn location(&self) -> Option<&SourceCodeLocation<'a>> { self.location.as_ref() }
}


pub struct NavigatorUserAgentIssueDetailsBuilder<'a> {
    url: Cow<'a, str>,
    location: Option<SourceCodeLocation<'a>>,
}

impl<'a> NavigatorUserAgentIssueDetailsBuilder<'a> {
    pub fn location(mut self, location: SourceCodeLocation<'a>) -> Self { self.location = Some(location); self }
    pub fn build(self) -> NavigatorUserAgentIssueDetails<'a> {
        NavigatorUserAgentIssueDetails {
            url: self.url,
            location: self.location,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedDictionaryIssueDetails<'a> {
    #[serde(rename = "sharedDictionaryError")]
    shared_dictionary_error: SharedDictionaryError,
    request: AffectedRequest<'a>,
}

impl<'a> SharedDictionaryIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `shared_dictionary_error`: 
    /// * `request`: 
    pub fn builder(shared_dictionary_error: impl Into<SharedDictionaryError>, request: AffectedRequest<'a>) -> SharedDictionaryIssueDetailsBuilder<'a> {
        SharedDictionaryIssueDetailsBuilder {
            shared_dictionary_error: shared_dictionary_error.into(),
            request: request,
        }
    }
    pub fn shared_dictionary_error(&self) -> &SharedDictionaryError { &self.shared_dictionary_error }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
}


pub struct SharedDictionaryIssueDetailsBuilder<'a> {
    shared_dictionary_error: SharedDictionaryError,
    request: AffectedRequest<'a>,
}

impl<'a> SharedDictionaryIssueDetailsBuilder<'a> {
    pub fn build(self) -> SharedDictionaryIssueDetails<'a> {
        SharedDictionaryIssueDetails {
            shared_dictionary_error: self.shared_dictionary_error,
            request: self.request,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SRIMessageSignatureIssueDetails<'a> {
    error: SRIMessageSignatureError,
    #[serde(rename = "signatureBase")]
    signature_base: Cow<'a, str>,
    #[serde(rename = "integrityAssertions")]
    integrity_assertions: Vec<Cow<'a, str>>,
    request: AffectedRequest<'a>,
}

impl<'a> SRIMessageSignatureIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `error`: 
    /// * `signature_base`: 
    /// * `integrity_assertions`: 
    /// * `request`: 
    pub fn builder(error: impl Into<SRIMessageSignatureError>, signature_base: impl Into<Cow<'a, str>>, integrity_assertions: Vec<Cow<'a, str>>, request: AffectedRequest<'a>) -> SRIMessageSignatureIssueDetailsBuilder<'a> {
        SRIMessageSignatureIssueDetailsBuilder {
            error: error.into(),
            signature_base: signature_base.into(),
            integrity_assertions: integrity_assertions,
            request: request,
        }
    }
    pub fn error(&self) -> &SRIMessageSignatureError { &self.error }
    pub fn signature_base(&self) -> &str { self.signature_base.as_ref() }
    pub fn integrity_assertions(&self) -> &[Cow<'a, str>] { &self.integrity_assertions }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
}


pub struct SRIMessageSignatureIssueDetailsBuilder<'a> {
    error: SRIMessageSignatureError,
    signature_base: Cow<'a, str>,
    integrity_assertions: Vec<Cow<'a, str>>,
    request: AffectedRequest<'a>,
}

impl<'a> SRIMessageSignatureIssueDetailsBuilder<'a> {
    pub fn build(self) -> SRIMessageSignatureIssueDetails<'a> {
        SRIMessageSignatureIssueDetails {
            error: self.error,
            signature_base: self.signature_base,
            integrity_assertions: self.integrity_assertions,
            request: self.request,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnencodedDigestIssueDetails<'a> {
    error: UnencodedDigestError,
    request: AffectedRequest<'a>,
}

impl<'a> UnencodedDigestIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `error`: 
    /// * `request`: 
    pub fn builder(error: impl Into<UnencodedDigestError>, request: AffectedRequest<'a>) -> UnencodedDigestIssueDetailsBuilder<'a> {
        UnencodedDigestIssueDetailsBuilder {
            error: error.into(),
            request: request,
        }
    }
    pub fn error(&self) -> &UnencodedDigestError { &self.error }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
}


pub struct UnencodedDigestIssueDetailsBuilder<'a> {
    error: UnencodedDigestError,
    request: AffectedRequest<'a>,
}

impl<'a> UnencodedDigestIssueDetailsBuilder<'a> {
    pub fn build(self) -> UnencodedDigestIssueDetails<'a> {
        UnencodedDigestIssueDetails {
            error: self.error,
            request: self.request,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionAllowlistIssueDetails<'a> {
    error: ConnectionAllowlistError,
    request: AffectedRequest<'a>,
}

impl<'a> ConnectionAllowlistIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `error`: 
    /// * `request`: 
    pub fn builder(error: impl Into<ConnectionAllowlistError>, request: AffectedRequest<'a>) -> ConnectionAllowlistIssueDetailsBuilder<'a> {
        ConnectionAllowlistIssueDetailsBuilder {
            error: error.into(),
            request: request,
        }
    }
    pub fn error(&self) -> &ConnectionAllowlistError { &self.error }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
}


pub struct ConnectionAllowlistIssueDetailsBuilder<'a> {
    error: ConnectionAllowlistError,
    request: AffectedRequest<'a>,
}

impl<'a> ConnectionAllowlistIssueDetailsBuilder<'a> {
    pub fn build(self) -> ConnectionAllowlistIssueDetails<'a> {
        ConnectionAllowlistIssueDetails {
            error: self.error,
            request: self.request,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GenericIssueErrorType {
    #[default]
    #[serde(rename = "FormLabelForNameError")]
    FormLabelForNameError,
    #[serde(rename = "FormDuplicateIdForInputError")]
    FormDuplicateIdForInputError,
    #[serde(rename = "FormInputWithNoLabelError")]
    FormInputWithNoLabelError,
    #[serde(rename = "FormAutocompleteAttributeEmptyError")]
    FormAutocompleteAttributeEmptyError,
    #[serde(rename = "FormEmptyIdAndNameAttributesForInputError")]
    FormEmptyIdAndNameAttributesForInputError,
    #[serde(rename = "FormAriaLabelledByToNonExistingIdError")]
    FormAriaLabelledByToNonExistingIdError,
    #[serde(rename = "FormInputAssignedAutocompleteValueToIdOrNameAttributeError")]
    FormInputAssignedAutocompleteValueToIdOrNameAttributeError,
    #[serde(rename = "FormLabelHasNeitherForNorNestedInputError")]
    FormLabelHasNeitherForNorNestedInputError,
    #[serde(rename = "FormLabelForMatchesNonExistingIdError")]
    FormLabelForMatchesNonExistingIdError,
    #[serde(rename = "FormInputHasWrongButWellIntendedAutocompleteValueError")]
    FormInputHasWrongButWellIntendedAutocompleteValueError,
    #[serde(rename = "ResponseWasBlockedByORB")]
    ResponseWasBlockedByORB,
    #[serde(rename = "NavigationEntryMarkedSkippable")]
    NavigationEntryMarkedSkippable,
    #[serde(rename = "BackUINavigationWouldSkipAd")]
    BackUINavigationWouldSkipAd,
    #[serde(rename = "AutofillAndManualTextPolicyControlledFeaturesInfo")]
    AutofillAndManualTextPolicyControlledFeaturesInfo,
    #[serde(rename = "AutofillPolicyControlledFeatureInfo")]
    AutofillPolicyControlledFeatureInfo,
    #[serde(rename = "ManualTextPolicyControlledFeatureInfo")]
    ManualTextPolicyControlledFeatureInfo,
    #[serde(rename = "FormModelContextParameterMissingTitleAndDescription")]
    FormModelContextParameterMissingTitleAndDescription,
    #[serde(rename = "FormModelContextMissingToolName")]
    FormModelContextMissingToolName,
    #[serde(rename = "FormModelContextMissingToolDescription")]
    FormModelContextMissingToolDescription,
    #[serde(rename = "FormModelContextRequiredParameterMissingName")]
    FormModelContextRequiredParameterMissingName,
    #[serde(rename = "FormModelContextParameterMissingName")]
    FormModelContextParameterMissingName,
}

/// Depending on the concrete errorType, different properties are set.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenericIssueDetails<'a> {
    /// Issues with the same errorType are aggregated in the frontend.
    #[serde(rename = "errorType")]
    error_type: GenericIssueErrorType,
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "violatingNodeId")]
    violating_node_id: Option<crate::dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "violatingNodeAttribute")]
    violating_node_attribute: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AffectedRequest<'a>>,
}

impl<'a> GenericIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `error_type`: Issues with the same errorType are aggregated in the frontend.
    pub fn builder(error_type: impl Into<GenericIssueErrorType>) -> GenericIssueDetailsBuilder<'a> {
        GenericIssueDetailsBuilder {
            error_type: error_type.into(),
            frame_id: None,
            violating_node_id: None,
            violating_node_attribute: None,
            request: None,
        }
    }
    /// Issues with the same errorType are aggregated in the frontend.
    pub fn error_type(&self) -> &GenericIssueErrorType { &self.error_type }
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
    pub fn violating_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.violating_node_id.as_ref() }
    pub fn violating_node_attribute(&self) -> Option<&str> { self.violating_node_attribute.as_deref() }
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
}


pub struct GenericIssueDetailsBuilder<'a> {
    error_type: GenericIssueErrorType,
    frame_id: Option<crate::page::FrameId<'a>>,
    violating_node_id: Option<crate::dom::BackendNodeId>,
    violating_node_attribute: Option<Cow<'a, str>>,
    request: Option<AffectedRequest<'a>>,
}

impl<'a> GenericIssueDetailsBuilder<'a> {
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    pub fn violating_node_id(mut self, violating_node_id: crate::dom::BackendNodeId) -> Self { self.violating_node_id = Some(violating_node_id); self }
    pub fn violating_node_attribute(mut self, violating_node_attribute: impl Into<Cow<'a, str>>) -> Self { self.violating_node_attribute = Some(violating_node_attribute.into()); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn build(self) -> GenericIssueDetails<'a> {
        GenericIssueDetails {
            error_type: self.error_type,
            frame_id: self.frame_id,
            violating_node_id: self.violating_node_id,
            violating_node_attribute: self.violating_node_attribute,
            request: self.request,
        }
    }
}

/// This issue tracks information needed to print a deprecation message.
/// <https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeprecationIssueDetails<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "affectedFrame")]
    affected_frame: Option<AffectedFrame<'a>>,
    #[serde(rename = "sourceCodeLocation")]
    source_code_location: SourceCodeLocation<'a>,
    /// One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
}

impl<'a> DeprecationIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `source_code_location`: 
    /// * `type_`: One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5
    pub fn builder(source_code_location: SourceCodeLocation<'a>, type_: impl Into<Cow<'a, str>>) -> DeprecationIssueDetailsBuilder<'a> {
        DeprecationIssueDetailsBuilder {
            affected_frame: None,
            source_code_location: source_code_location,
            type_: type_.into(),
        }
    }
    pub fn affected_frame(&self) -> Option<&AffectedFrame<'a>> { self.affected_frame.as_ref() }
    pub fn source_code_location(&self) -> &SourceCodeLocation<'a> { &self.source_code_location }
    /// One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}


pub struct DeprecationIssueDetailsBuilder<'a> {
    affected_frame: Option<AffectedFrame<'a>>,
    source_code_location: SourceCodeLocation<'a>,
    type_: Cow<'a, str>,
}

impl<'a> DeprecationIssueDetailsBuilder<'a> {
    pub fn affected_frame(mut self, affected_frame: AffectedFrame<'a>) -> Self { self.affected_frame = Some(affected_frame); self }
    pub fn build(self) -> DeprecationIssueDetails<'a> {
        DeprecationIssueDetails {
            affected_frame: self.affected_frame,
            source_code_location: self.source_code_location,
            type_: self.type_,
        }
    }
}

/// This issue warns about sites in the redirect chain of a finished navigation
/// that may be flagged as trackers and have their state cleared if they don't
/// receive a user interaction. Note that in this context 'site' means eTLD+1.
/// For example, if the URL 'https://example.test:80/bounce' was in the
/// redirect chain, the site reported would be 'example.test'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BounceTrackingIssueDetails<'a> {
    #[serde(rename = "trackingSites")]
    tracking_sites: Vec<Cow<'a, str>>,
}

impl<'a> BounceTrackingIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `tracking_sites`: 
    pub fn builder(tracking_sites: Vec<Cow<'a, str>>) -> BounceTrackingIssueDetailsBuilder<'a> {
        BounceTrackingIssueDetailsBuilder {
            tracking_sites: tracking_sites,
        }
    }
    pub fn tracking_sites(&self) -> &[Cow<'a, str>] { &self.tracking_sites }
}


pub struct BounceTrackingIssueDetailsBuilder<'a> {
    tracking_sites: Vec<Cow<'a, str>>,
}

impl<'a> BounceTrackingIssueDetailsBuilder<'a> {
    pub fn build(self) -> BounceTrackingIssueDetails<'a> {
        BounceTrackingIssueDetails {
            tracking_sites: self.tracking_sites,
        }
    }
}

/// This issue warns about third-party sites that are accessing cookies on the
/// current page, and have been permitted due to having a global metadata grant.
/// Note that in this context 'site' means eTLD+1. For example, if the URL
/// 'https://example.test:80/web_page' was accessing cookies, the site reported
/// would be 'example.test'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookieDeprecationMetadataIssueDetails<'a> {
    #[serde(rename = "allowedSites")]
    allowed_sites: Vec<Cow<'a, str>>,
    #[serde(rename = "optOutPercentage")]
    opt_out_percentage: f64,
    #[serde(rename = "isOptOutTopLevel")]
    is_opt_out_top_level: bool,
    operation: CookieOperation,
}

impl<'a> CookieDeprecationMetadataIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `allowed_sites`: 
    /// * `opt_out_percentage`: 
    /// * `is_opt_out_top_level`: 
    /// * `operation`: 
    pub fn builder(allowed_sites: Vec<Cow<'a, str>>, opt_out_percentage: f64, is_opt_out_top_level: bool, operation: impl Into<CookieOperation>) -> CookieDeprecationMetadataIssueDetailsBuilder<'a> {
        CookieDeprecationMetadataIssueDetailsBuilder {
            allowed_sites: allowed_sites,
            opt_out_percentage: opt_out_percentage,
            is_opt_out_top_level: is_opt_out_top_level,
            operation: operation.into(),
        }
    }
    pub fn allowed_sites(&self) -> &[Cow<'a, str>] { &self.allowed_sites }
    pub fn opt_out_percentage(&self) -> f64 { self.opt_out_percentage }
    pub fn is_opt_out_top_level(&self) -> bool { self.is_opt_out_top_level }
    pub fn operation(&self) -> &CookieOperation { &self.operation }
}


pub struct CookieDeprecationMetadataIssueDetailsBuilder<'a> {
    allowed_sites: Vec<Cow<'a, str>>,
    opt_out_percentage: f64,
    is_opt_out_top_level: bool,
    operation: CookieOperation,
}

impl<'a> CookieDeprecationMetadataIssueDetailsBuilder<'a> {
    pub fn build(self) -> CookieDeprecationMetadataIssueDetails<'a> {
        CookieDeprecationMetadataIssueDetails {
            allowed_sites: self.allowed_sites,
            opt_out_percentage: self.opt_out_percentage,
            is_opt_out_top_level: self.is_opt_out_top_level,
            operation: self.operation,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ClientHintIssueReason {
    #[default]
    #[serde(rename = "MetaTagAllowListInvalidOrigin")]
    MetaTagAllowListInvalidOrigin,
    #[serde(rename = "MetaTagModifiedHTML")]
    MetaTagModifiedHTML,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FederatedAuthRequestIssueDetails {
    #[serde(rename = "federatedAuthRequestIssueReason")]
    federated_auth_request_issue_reason: FederatedAuthRequestIssueReason,
}

impl FederatedAuthRequestIssueDetails {
    /// Creates a builder for this type with the required parameters:
    /// * `federated_auth_request_issue_reason`: 
    pub fn builder(federated_auth_request_issue_reason: impl Into<FederatedAuthRequestIssueReason>) -> FederatedAuthRequestIssueDetailsBuilder {
        FederatedAuthRequestIssueDetailsBuilder {
            federated_auth_request_issue_reason: federated_auth_request_issue_reason.into(),
        }
    }
    pub fn federated_auth_request_issue_reason(&self) -> &FederatedAuthRequestIssueReason { &self.federated_auth_request_issue_reason }
}


pub struct FederatedAuthRequestIssueDetailsBuilder {
    federated_auth_request_issue_reason: FederatedAuthRequestIssueReason,
}

impl FederatedAuthRequestIssueDetailsBuilder {
    pub fn build(self) -> FederatedAuthRequestIssueDetails {
        FederatedAuthRequestIssueDetails {
            federated_auth_request_issue_reason: self.federated_auth_request_issue_reason,
        }
    }
}

/// Represents the failure reason when a federated authentication reason fails.
/// Should be updated alongside RequestIdTokenStatus in
/// third_party/blink/public/mojom/devtools/inspector_issue.mojom to include
/// all cases except for success.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum FederatedAuthRequestIssueReason {
    #[default]
    #[serde(rename = "ShouldEmbargo")]
    ShouldEmbargo,
    #[serde(rename = "TooManyRequests")]
    TooManyRequests,
    #[serde(rename = "WellKnownHttpNotFound")]
    WellKnownHttpNotFound,
    #[serde(rename = "WellKnownNoResponse")]
    WellKnownNoResponse,
    #[serde(rename = "WellKnownInvalidResponse")]
    WellKnownInvalidResponse,
    #[serde(rename = "WellKnownListEmpty")]
    WellKnownListEmpty,
    #[serde(rename = "WellKnownInvalidContentType")]
    WellKnownInvalidContentType,
    #[serde(rename = "ConfigNotInWellKnown")]
    ConfigNotInWellKnown,
    #[serde(rename = "WellKnownTooBig")]
    WellKnownTooBig,
    #[serde(rename = "ConfigHttpNotFound")]
    ConfigHttpNotFound,
    #[serde(rename = "ConfigNoResponse")]
    ConfigNoResponse,
    #[serde(rename = "ConfigInvalidResponse")]
    ConfigInvalidResponse,
    #[serde(rename = "ConfigInvalidContentType")]
    ConfigInvalidContentType,
    #[serde(rename = "IdpNotPotentiallyTrustworthy")]
    IdpNotPotentiallyTrustworthy,
    #[serde(rename = "DisabledInSettings")]
    DisabledInSettings,
    #[serde(rename = "DisabledInFlags")]
    DisabledInFlags,
    #[serde(rename = "ErrorFetchingSignin")]
    ErrorFetchingSignin,
    #[serde(rename = "InvalidSigninResponse")]
    InvalidSigninResponse,
    #[serde(rename = "AccountsHttpNotFound")]
    AccountsHttpNotFound,
    #[serde(rename = "AccountsNoResponse")]
    AccountsNoResponse,
    #[serde(rename = "AccountsInvalidResponse")]
    AccountsInvalidResponse,
    #[serde(rename = "AccountsListEmpty")]
    AccountsListEmpty,
    #[serde(rename = "AccountsInvalidContentType")]
    AccountsInvalidContentType,
    #[serde(rename = "IdTokenHttpNotFound")]
    IdTokenHttpNotFound,
    #[serde(rename = "IdTokenNoResponse")]
    IdTokenNoResponse,
    #[serde(rename = "IdTokenInvalidResponse")]
    IdTokenInvalidResponse,
    #[serde(rename = "IdTokenIdpErrorResponse")]
    IdTokenIdpErrorResponse,
    #[serde(rename = "IdTokenCrossSiteIdpErrorResponse")]
    IdTokenCrossSiteIdpErrorResponse,
    #[serde(rename = "IdTokenInvalidRequest")]
    IdTokenInvalidRequest,
    #[serde(rename = "IdTokenInvalidContentType")]
    IdTokenInvalidContentType,
    #[serde(rename = "ErrorIdToken")]
    ErrorIdToken,
    #[serde(rename = "Canceled")]
    Canceled,
    #[serde(rename = "RpPageNotVisible")]
    RpPageNotVisible,
    #[serde(rename = "SilentMediationFailure")]
    SilentMediationFailure,
    #[serde(rename = "NotSignedInWithIdp")]
    NotSignedInWithIdp,
    #[serde(rename = "MissingTransientUserActivation")]
    MissingTransientUserActivation,
    #[serde(rename = "ReplacedByActiveMode")]
    ReplacedByActiveMode,
    #[serde(rename = "RelyingPartyOriginIsOpaque")]
    RelyingPartyOriginIsOpaque,
    #[serde(rename = "TypeNotMatching")]
    TypeNotMatching,
    #[serde(rename = "UiDismissedNoEmbargo")]
    UiDismissedNoEmbargo,
    #[serde(rename = "CorsError")]
    CorsError,
    #[serde(rename = "SuppressedBySegmentationPlatform")]
    SuppressedBySegmentationPlatform,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FederatedAuthUserInfoRequestIssueDetails {
    #[serde(rename = "federatedAuthUserInfoRequestIssueReason")]
    federated_auth_user_info_request_issue_reason: FederatedAuthUserInfoRequestIssueReason,
}

impl FederatedAuthUserInfoRequestIssueDetails {
    /// Creates a builder for this type with the required parameters:
    /// * `federated_auth_user_info_request_issue_reason`: 
    pub fn builder(federated_auth_user_info_request_issue_reason: impl Into<FederatedAuthUserInfoRequestIssueReason>) -> FederatedAuthUserInfoRequestIssueDetailsBuilder {
        FederatedAuthUserInfoRequestIssueDetailsBuilder {
            federated_auth_user_info_request_issue_reason: federated_auth_user_info_request_issue_reason.into(),
        }
    }
    pub fn federated_auth_user_info_request_issue_reason(&self) -> &FederatedAuthUserInfoRequestIssueReason { &self.federated_auth_user_info_request_issue_reason }
}


pub struct FederatedAuthUserInfoRequestIssueDetailsBuilder {
    federated_auth_user_info_request_issue_reason: FederatedAuthUserInfoRequestIssueReason,
}

impl FederatedAuthUserInfoRequestIssueDetailsBuilder {
    pub fn build(self) -> FederatedAuthUserInfoRequestIssueDetails {
        FederatedAuthUserInfoRequestIssueDetails {
            federated_auth_user_info_request_issue_reason: self.federated_auth_user_info_request_issue_reason,
        }
    }
}

/// Represents the failure reason when a getUserInfo() call fails.
/// Should be updated alongside FederatedAuthUserInfoRequestResult in
/// third_party/blink/public/mojom/devtools/inspector_issue.mojom.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum FederatedAuthUserInfoRequestIssueReason {
    #[default]
    #[serde(rename = "NotSameOrigin")]
    NotSameOrigin,
    #[serde(rename = "NotIframe")]
    NotIframe,
    #[serde(rename = "NotPotentiallyTrustworthy")]
    NotPotentiallyTrustworthy,
    #[serde(rename = "NoApiPermission")]
    NoApiPermission,
    #[serde(rename = "NotSignedInWithIdp")]
    NotSignedInWithIdp,
    #[serde(rename = "NoAccountSharingPermission")]
    NoAccountSharingPermission,
    #[serde(rename = "InvalidConfigOrWellKnown")]
    InvalidConfigOrWellKnown,
    #[serde(rename = "InvalidAccountsResponse")]
    InvalidAccountsResponse,
    #[serde(rename = "NoReturningUserFromFetchedAccounts")]
    NoReturningUserFromFetchedAccounts,
}

/// This issue tracks client hints related issues. It's used to deprecate old
/// features, encourage the use of new ones, and provide general guidance.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientHintIssueDetails<'a> {
    #[serde(rename = "sourceCodeLocation")]
    source_code_location: SourceCodeLocation<'a>,
    #[serde(rename = "clientHintIssueReason")]
    client_hint_issue_reason: ClientHintIssueReason,
}

impl<'a> ClientHintIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `source_code_location`: 
    /// * `client_hint_issue_reason`: 
    pub fn builder(source_code_location: SourceCodeLocation<'a>, client_hint_issue_reason: impl Into<ClientHintIssueReason>) -> ClientHintIssueDetailsBuilder<'a> {
        ClientHintIssueDetailsBuilder {
            source_code_location: source_code_location,
            client_hint_issue_reason: client_hint_issue_reason.into(),
        }
    }
    pub fn source_code_location(&self) -> &SourceCodeLocation<'a> { &self.source_code_location }
    pub fn client_hint_issue_reason(&self) -> &ClientHintIssueReason { &self.client_hint_issue_reason }
}


pub struct ClientHintIssueDetailsBuilder<'a> {
    source_code_location: SourceCodeLocation<'a>,
    client_hint_issue_reason: ClientHintIssueReason,
}

impl<'a> ClientHintIssueDetailsBuilder<'a> {
    pub fn build(self) -> ClientHintIssueDetails<'a> {
        ClientHintIssueDetails {
            source_code_location: self.source_code_location,
            client_hint_issue_reason: self.client_hint_issue_reason,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FailedRequestInfo<'a> {
    /// The URL that failed to load.
    url: Cow<'a, str>,
    /// The failure message for the failed request.
    #[serde(rename = "failureMessage")]
    failure_message: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestId")]
    request_id: Option<crate::network::RequestId<'a>>,
}

impl<'a> FailedRequestInfo<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: The URL that failed to load.
    /// * `failure_message`: The failure message for the failed request.
    pub fn builder(url: impl Into<Cow<'a, str>>, failure_message: impl Into<Cow<'a, str>>) -> FailedRequestInfoBuilder<'a> {
        FailedRequestInfoBuilder {
            url: url.into(),
            failure_message: failure_message.into(),
            request_id: None,
        }
    }
    /// The URL that failed to load.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// The failure message for the failed request.
    pub fn failure_message(&self) -> &str { self.failure_message.as_ref() }
    pub fn request_id(&self) -> Option<&crate::network::RequestId<'a>> { self.request_id.as_ref() }
}


pub struct FailedRequestInfoBuilder<'a> {
    url: Cow<'a, str>,
    failure_message: Cow<'a, str>,
    request_id: Option<crate::network::RequestId<'a>>,
}

impl<'a> FailedRequestInfoBuilder<'a> {
    pub fn request_id(mut self, request_id: crate::network::RequestId<'a>) -> Self { self.request_id = Some(request_id); self }
    pub fn build(self) -> FailedRequestInfo<'a> {
        FailedRequestInfo {
            url: self.url,
            failure_message: self.failure_message,
            request_id: self.request_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PartitioningBlobURLInfo {
    #[default]
    #[serde(rename = "BlockedCrossPartitionFetching")]
    BlockedCrossPartitionFetching,
    #[serde(rename = "EnforceNoopenerForNavigation")]
    EnforceNoopenerForNavigation,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PartitioningBlobURLIssueDetails<'a> {
    /// The BlobURL that failed to load.
    url: Cow<'a, str>,
    /// Additional information about the Partitioning Blob URL issue.
    #[serde(rename = "partitioningBlobURLInfo")]
    partitioning_blob_url_info: PartitioningBlobURLInfo,
}

impl<'a> PartitioningBlobURLIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: The BlobURL that failed to load.
    /// * `partitioning_blob_url_info`: Additional information about the Partitioning Blob URL issue.
    pub fn builder(url: impl Into<Cow<'a, str>>, partitioning_blob_url_info: impl Into<PartitioningBlobURLInfo>) -> PartitioningBlobURLIssueDetailsBuilder<'a> {
        PartitioningBlobURLIssueDetailsBuilder {
            url: url.into(),
            partitioning_blob_url_info: partitioning_blob_url_info.into(),
        }
    }
    /// The BlobURL that failed to load.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Additional information about the Partitioning Blob URL issue.
    pub fn partitioning_blob_url_info(&self) -> &PartitioningBlobURLInfo { &self.partitioning_blob_url_info }
}


pub struct PartitioningBlobURLIssueDetailsBuilder<'a> {
    url: Cow<'a, str>,
    partitioning_blob_url_info: PartitioningBlobURLInfo,
}

impl<'a> PartitioningBlobURLIssueDetailsBuilder<'a> {
    pub fn build(self) -> PartitioningBlobURLIssueDetails<'a> {
        PartitioningBlobURLIssueDetails {
            url: self.url,
            partitioning_blob_url_info: self.partitioning_blob_url_info,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ElementAccessibilityIssueReason {
    #[default]
    #[serde(rename = "DisallowedSelectChild")]
    DisallowedSelectChild,
    #[serde(rename = "DisallowedOptGroupChild")]
    DisallowedOptGroupChild,
    #[serde(rename = "NonPhrasingContentOptionChild")]
    NonPhrasingContentOptionChild,
    #[serde(rename = "InteractiveContentOptionChild")]
    InteractiveContentOptionChild,
    #[serde(rename = "InteractiveContentLegendChild")]
    InteractiveContentLegendChild,
    #[serde(rename = "InteractiveContentSummaryDescendant")]
    InteractiveContentSummaryDescendant,
}

/// This issue warns about errors in the select or summary element content model.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ElementAccessibilityIssueDetails {
    #[serde(rename = "nodeId")]
    node_id: crate::dom::BackendNodeId,
    #[serde(rename = "elementAccessibilityIssueReason")]
    element_accessibility_issue_reason: ElementAccessibilityIssueReason,
    #[serde(rename = "hasDisallowedAttributes")]
    has_disallowed_attributes: bool,
}

impl ElementAccessibilityIssueDetails {
    /// Creates a builder for this type with the required parameters:
    /// * `node_id`: 
    /// * `element_accessibility_issue_reason`: 
    /// * `has_disallowed_attributes`: 
    pub fn builder(node_id: crate::dom::BackendNodeId, element_accessibility_issue_reason: impl Into<ElementAccessibilityIssueReason>, has_disallowed_attributes: bool) -> ElementAccessibilityIssueDetailsBuilder {
        ElementAccessibilityIssueDetailsBuilder {
            node_id: node_id,
            element_accessibility_issue_reason: element_accessibility_issue_reason.into(),
            has_disallowed_attributes: has_disallowed_attributes,
        }
    }
    pub fn node_id(&self) -> &crate::dom::BackendNodeId { &self.node_id }
    pub fn element_accessibility_issue_reason(&self) -> &ElementAccessibilityIssueReason { &self.element_accessibility_issue_reason }
    pub fn has_disallowed_attributes(&self) -> bool { self.has_disallowed_attributes }
}


pub struct ElementAccessibilityIssueDetailsBuilder {
    node_id: crate::dom::BackendNodeId,
    element_accessibility_issue_reason: ElementAccessibilityIssueReason,
    has_disallowed_attributes: bool,
}

impl ElementAccessibilityIssueDetailsBuilder {
    pub fn build(self) -> ElementAccessibilityIssueDetails {
        ElementAccessibilityIssueDetails {
            node_id: self.node_id,
            element_accessibility_issue_reason: self.element_accessibility_issue_reason,
            has_disallowed_attributes: self.has_disallowed_attributes,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StyleSheetLoadingIssueReason {
    #[default]
    #[serde(rename = "LateImportRule")]
    LateImportRule,
    #[serde(rename = "RequestFailed")]
    RequestFailed,
}

/// This issue warns when a referenced stylesheet couldn't be loaded.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StylesheetLoadingIssueDetails<'a> {
    /// Source code position that referenced the failing stylesheet.
    #[serde(rename = "sourceCodeLocation")]
    source_code_location: SourceCodeLocation<'a>,
    /// Reason why the stylesheet couldn't be loaded.
    #[serde(rename = "styleSheetLoadingIssueReason")]
    style_sheet_loading_issue_reason: StyleSheetLoadingIssueReason,
    /// Contains additional info when the failure was due to a request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "failedRequestInfo")]
    failed_request_info: Option<FailedRequestInfo<'a>>,
}

impl<'a> StylesheetLoadingIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `source_code_location`: Source code position that referenced the failing stylesheet.
    /// * `style_sheet_loading_issue_reason`: Reason why the stylesheet couldn't be loaded.
    pub fn builder(source_code_location: SourceCodeLocation<'a>, style_sheet_loading_issue_reason: impl Into<StyleSheetLoadingIssueReason>) -> StylesheetLoadingIssueDetailsBuilder<'a> {
        StylesheetLoadingIssueDetailsBuilder {
            source_code_location: source_code_location,
            style_sheet_loading_issue_reason: style_sheet_loading_issue_reason.into(),
            failed_request_info: None,
        }
    }
    /// Source code position that referenced the failing stylesheet.
    pub fn source_code_location(&self) -> &SourceCodeLocation<'a> { &self.source_code_location }
    /// Reason why the stylesheet couldn't be loaded.
    pub fn style_sheet_loading_issue_reason(&self) -> &StyleSheetLoadingIssueReason { &self.style_sheet_loading_issue_reason }
    /// Contains additional info when the failure was due to a request.
    pub fn failed_request_info(&self) -> Option<&FailedRequestInfo<'a>> { self.failed_request_info.as_ref() }
}


pub struct StylesheetLoadingIssueDetailsBuilder<'a> {
    source_code_location: SourceCodeLocation<'a>,
    style_sheet_loading_issue_reason: StyleSheetLoadingIssueReason,
    failed_request_info: Option<FailedRequestInfo<'a>>,
}

impl<'a> StylesheetLoadingIssueDetailsBuilder<'a> {
    /// Contains additional info when the failure was due to a request.
    pub fn failed_request_info(mut self, failed_request_info: FailedRequestInfo<'a>) -> Self { self.failed_request_info = Some(failed_request_info); self }
    pub fn build(self) -> StylesheetLoadingIssueDetails<'a> {
        StylesheetLoadingIssueDetails {
            source_code_location: self.source_code_location,
            style_sheet_loading_issue_reason: self.style_sheet_loading_issue_reason,
            failed_request_info: self.failed_request_info,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PropertyRuleIssueReason {
    #[default]
    #[serde(rename = "InvalidSyntax")]
    InvalidSyntax,
    #[serde(rename = "InvalidInitialValue")]
    InvalidInitialValue,
    #[serde(rename = "InvalidInherits")]
    InvalidInherits,
    #[serde(rename = "InvalidName")]
    InvalidName,
}

/// This issue warns about errors in property rules that lead to property
/// registrations being ignored.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyRuleIssueDetails<'a> {
    /// Source code position of the property rule.
    #[serde(rename = "sourceCodeLocation")]
    source_code_location: SourceCodeLocation<'a>,
    /// Reason why the property rule was discarded.
    #[serde(rename = "propertyRuleIssueReason")]
    property_rule_issue_reason: PropertyRuleIssueReason,
    /// The value of the property rule property that failed to parse
    #[serde(skip_serializing_if = "Option::is_none", rename = "propertyValue")]
    property_value: Option<Cow<'a, str>>,
}

impl<'a> PropertyRuleIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `source_code_location`: Source code position of the property rule.
    /// * `property_rule_issue_reason`: Reason why the property rule was discarded.
    pub fn builder(source_code_location: SourceCodeLocation<'a>, property_rule_issue_reason: impl Into<PropertyRuleIssueReason>) -> PropertyRuleIssueDetailsBuilder<'a> {
        PropertyRuleIssueDetailsBuilder {
            source_code_location: source_code_location,
            property_rule_issue_reason: property_rule_issue_reason.into(),
            property_value: None,
        }
    }
    /// Source code position of the property rule.
    pub fn source_code_location(&self) -> &SourceCodeLocation<'a> { &self.source_code_location }
    /// Reason why the property rule was discarded.
    pub fn property_rule_issue_reason(&self) -> &PropertyRuleIssueReason { &self.property_rule_issue_reason }
    /// The value of the property rule property that failed to parse
    pub fn property_value(&self) -> Option<&str> { self.property_value.as_deref() }
}


pub struct PropertyRuleIssueDetailsBuilder<'a> {
    source_code_location: SourceCodeLocation<'a>,
    property_rule_issue_reason: PropertyRuleIssueReason,
    property_value: Option<Cow<'a, str>>,
}

impl<'a> PropertyRuleIssueDetailsBuilder<'a> {
    /// The value of the property rule property that failed to parse
    pub fn property_value(mut self, property_value: impl Into<Cow<'a, str>>) -> Self { self.property_value = Some(property_value.into()); self }
    pub fn build(self) -> PropertyRuleIssueDetails<'a> {
        PropertyRuleIssueDetails {
            source_code_location: self.source_code_location,
            property_rule_issue_reason: self.property_rule_issue_reason,
            property_value: self.property_value,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum UserReidentificationIssueType {
    #[default]
    #[serde(rename = "BlockedFrameNavigation")]
    BlockedFrameNavigation,
    #[serde(rename = "BlockedSubresource")]
    BlockedSubresource,
    #[serde(rename = "NoisedCanvasReadback")]
    NoisedCanvasReadback,
}

/// This issue warns about uses of APIs that may be considered misuse to
/// re-identify users.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserReidentificationIssueDetails<'a> {
    #[serde(rename = "type")]
    type_: UserReidentificationIssueType,
    /// Applies to BlockedFrameNavigation and BlockedSubresource issue types.
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AffectedRequest<'a>>,
    /// Applies to NoisedCanvasReadback issue type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceCodeLocation")]
    source_code_location: Option<SourceCodeLocation<'a>>,
}

impl<'a> UserReidentificationIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: 
    pub fn builder(type_: impl Into<UserReidentificationIssueType>) -> UserReidentificationIssueDetailsBuilder<'a> {
        UserReidentificationIssueDetailsBuilder {
            type_: type_.into(),
            request: None,
            source_code_location: None,
        }
    }
    pub fn type_(&self) -> &UserReidentificationIssueType { &self.type_ }
    /// Applies to BlockedFrameNavigation and BlockedSubresource issue types.
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
    /// Applies to NoisedCanvasReadback issue type.
    pub fn source_code_location(&self) -> Option<&SourceCodeLocation<'a>> { self.source_code_location.as_ref() }
}


pub struct UserReidentificationIssueDetailsBuilder<'a> {
    type_: UserReidentificationIssueType,
    request: Option<AffectedRequest<'a>>,
    source_code_location: Option<SourceCodeLocation<'a>>,
}

impl<'a> UserReidentificationIssueDetailsBuilder<'a> {
    /// Applies to BlockedFrameNavigation and BlockedSubresource issue types.
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    /// Applies to NoisedCanvasReadback issue type.
    pub fn source_code_location(mut self, source_code_location: SourceCodeLocation<'a>) -> Self { self.source_code_location = Some(source_code_location); self }
    pub fn build(self) -> UserReidentificationIssueDetails<'a> {
        UserReidentificationIssueDetails {
            type_: self.type_,
            request: self.request,
            source_code_location: self.source_code_location,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionElementIssueType {
    #[default]
    #[serde(rename = "InvalidType")]
    InvalidType,
    #[serde(rename = "FencedFrameDisallowed")]
    FencedFrameDisallowed,
    #[serde(rename = "CspFrameAncestorsMissing")]
    CspFrameAncestorsMissing,
    #[serde(rename = "PermissionsPolicyBlocked")]
    PermissionsPolicyBlocked,
    #[serde(rename = "PaddingRightUnsupported")]
    PaddingRightUnsupported,
    #[serde(rename = "PaddingBottomUnsupported")]
    PaddingBottomUnsupported,
    #[serde(rename = "InsetBoxShadowUnsupported")]
    InsetBoxShadowUnsupported,
    #[serde(rename = "RequestInProgress")]
    RequestInProgress,
    #[serde(rename = "UntrustedEvent")]
    UntrustedEvent,
    #[serde(rename = "RegistrationFailed")]
    RegistrationFailed,
    #[serde(rename = "TypeNotSupported")]
    TypeNotSupported,
    #[serde(rename = "InvalidTypeActivation")]
    InvalidTypeActivation,
    #[serde(rename = "SecurityChecksFailed")]
    SecurityChecksFailed,
    #[serde(rename = "ActivationDisabled")]
    ActivationDisabled,
    #[serde(rename = "GeolocationDeprecated")]
    GeolocationDeprecated,
    #[serde(rename = "InvalidDisplayStyle")]
    InvalidDisplayStyle,
    #[serde(rename = "NonOpaqueColor")]
    NonOpaqueColor,
    #[serde(rename = "LowContrast")]
    LowContrast,
    #[serde(rename = "FontSizeTooSmall")]
    FontSizeTooSmall,
    #[serde(rename = "FontSizeTooLarge")]
    FontSizeTooLarge,
    #[serde(rename = "InvalidSizeValue")]
    InvalidSizeValue,
}

/// This issue warns about improper usage of the \<permission\> element.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionElementIssueDetails<'a> {
    #[serde(rename = "issueType")]
    issue_type: PermissionElementIssueType,
    /// The value of the type attribute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    type_: Option<Cow<'a, str>>,
    /// The node ID of the \<permission\> element.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    node_id: Option<crate::dom::BackendNodeId>,
    /// True if the issue is a warning, false if it is an error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isWarning")]
    is_warning: Option<bool>,
    /// Fields for message construction:
    /// Used for messages that reference a specific permission name
    #[serde(skip_serializing_if = "Option::is_none", rename = "permissionName")]
    permission_name: Option<Cow<'a, str>>,
    /// Used for messages about occlusion
    #[serde(skip_serializing_if = "Option::is_none", rename = "occluderNodeInfo")]
    occluder_node_info: Option<Cow<'a, str>>,
    /// Used for messages about occluder's parent
    #[serde(skip_serializing_if = "Option::is_none", rename = "occluderParentNodeInfo")]
    occluder_parent_node_info: Option<Cow<'a, str>>,
    /// Used for messages about activation disabled reason
    #[serde(skip_serializing_if = "Option::is_none", rename = "disableReason")]
    disable_reason: Option<Cow<'a, str>>,
}

impl<'a> PermissionElementIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `issue_type`: 
    pub fn builder(issue_type: impl Into<PermissionElementIssueType>) -> PermissionElementIssueDetailsBuilder<'a> {
        PermissionElementIssueDetailsBuilder {
            issue_type: issue_type.into(),
            type_: None,
            node_id: None,
            is_warning: None,
            permission_name: None,
            occluder_node_info: None,
            occluder_parent_node_info: None,
            disable_reason: None,
        }
    }
    pub fn issue_type(&self) -> &PermissionElementIssueType { &self.issue_type }
    /// The value of the type attribute.
    pub fn type_(&self) -> Option<&str> { self.type_.as_deref() }
    /// The node ID of the \<permission\> element.
    pub fn node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.node_id.as_ref() }
    /// True if the issue is a warning, false if it is an error.
    pub fn is_warning(&self) -> Option<bool> { self.is_warning }
    /// Fields for message construction:
    /// Used for messages that reference a specific permission name
    pub fn permission_name(&self) -> Option<&str> { self.permission_name.as_deref() }
    /// Used for messages about occlusion
    pub fn occluder_node_info(&self) -> Option<&str> { self.occluder_node_info.as_deref() }
    /// Used for messages about occluder's parent
    pub fn occluder_parent_node_info(&self) -> Option<&str> { self.occluder_parent_node_info.as_deref() }
    /// Used for messages about activation disabled reason
    pub fn disable_reason(&self) -> Option<&str> { self.disable_reason.as_deref() }
}


pub struct PermissionElementIssueDetailsBuilder<'a> {
    issue_type: PermissionElementIssueType,
    type_: Option<Cow<'a, str>>,
    node_id: Option<crate::dom::BackendNodeId>,
    is_warning: Option<bool>,
    permission_name: Option<Cow<'a, str>>,
    occluder_node_info: Option<Cow<'a, str>>,
    occluder_parent_node_info: Option<Cow<'a, str>>,
    disable_reason: Option<Cow<'a, str>>,
}

impl<'a> PermissionElementIssueDetailsBuilder<'a> {
    /// The value of the type attribute.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    /// The node ID of the \<permission\> element.
    pub fn node_id(mut self, node_id: crate::dom::BackendNodeId) -> Self { self.node_id = Some(node_id); self }
    /// True if the issue is a warning, false if it is an error.
    pub fn is_warning(mut self, is_warning: bool) -> Self { self.is_warning = Some(is_warning); self }
    /// Fields for message construction:
    /// Used for messages that reference a specific permission name
    pub fn permission_name(mut self, permission_name: impl Into<Cow<'a, str>>) -> Self { self.permission_name = Some(permission_name.into()); self }
    /// Used for messages about occlusion
    pub fn occluder_node_info(mut self, occluder_node_info: impl Into<Cow<'a, str>>) -> Self { self.occluder_node_info = Some(occluder_node_info.into()); self }
    /// Used for messages about occluder's parent
    pub fn occluder_parent_node_info(mut self, occluder_parent_node_info: impl Into<Cow<'a, str>>) -> Self { self.occluder_parent_node_info = Some(occluder_parent_node_info.into()); self }
    /// Used for messages about activation disabled reason
    pub fn disable_reason(mut self, disable_reason: impl Into<Cow<'a, str>>) -> Self { self.disable_reason = Some(disable_reason.into()); self }
    pub fn build(self) -> PermissionElementIssueDetails<'a> {
        PermissionElementIssueDetails {
            issue_type: self.issue_type,
            type_: self.type_,
            node_id: self.node_id,
            is_warning: self.is_warning,
            permission_name: self.permission_name,
            occluder_node_info: self.occluder_node_info,
            occluder_parent_node_info: self.occluder_parent_node_info,
            disable_reason: self.disable_reason,
        }
    }
}

/// The issue warns about blocked calls to privacy sensitive APIs via the
/// Selective Permissions Intervention.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectivePermissionsInterventionIssueDetails<'a> {
    /// Which API was intervened on.
    #[serde(rename = "apiName")]
    api_name: Cow<'a, str>,
    /// Why the ad script using the API is considered an ad.
    #[serde(rename = "adAncestry")]
    ad_ancestry: crate::network::AdAncestry<'a>,
    /// The stack trace at the time of the intervention.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackTrace")]
    stack_trace: Option<crate::runtime::StackTrace>,
}

impl<'a> SelectivePermissionsInterventionIssueDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `api_name`: Which API was intervened on.
    /// * `ad_ancestry`: Why the ad script using the API is considered an ad.
    pub fn builder(api_name: impl Into<Cow<'a, str>>, ad_ancestry: crate::network::AdAncestry<'a>) -> SelectivePermissionsInterventionIssueDetailsBuilder<'a> {
        SelectivePermissionsInterventionIssueDetailsBuilder {
            api_name: api_name.into(),
            ad_ancestry: ad_ancestry,
            stack_trace: None,
        }
    }
    /// Which API was intervened on.
    pub fn api_name(&self) -> &str { self.api_name.as_ref() }
    /// Why the ad script using the API is considered an ad.
    pub fn ad_ancestry(&self) -> &crate::network::AdAncestry<'a> { &self.ad_ancestry }
    /// The stack trace at the time of the intervention.
    pub fn stack_trace(&self) -> Option<&crate::runtime::StackTrace> { self.stack_trace.as_ref() }
}


pub struct SelectivePermissionsInterventionIssueDetailsBuilder<'a> {
    api_name: Cow<'a, str>,
    ad_ancestry: crate::network::AdAncestry<'a>,
    stack_trace: Option<crate::runtime::StackTrace>,
}

impl<'a> SelectivePermissionsInterventionIssueDetailsBuilder<'a> {
    /// The stack trace at the time of the intervention.
    pub fn stack_trace(mut self, stack_trace: crate::runtime::StackTrace) -> Self { self.stack_trace = Some(stack_trace); self }
    pub fn build(self) -> SelectivePermissionsInterventionIssueDetails<'a> {
        SelectivePermissionsInterventionIssueDetails {
            api_name: self.api_name,
            ad_ancestry: self.ad_ancestry,
            stack_trace: self.stack_trace,
        }
    }
}

/// A unique identifier for the type of issue. Each type may use one of the
/// optional fields in InspectorIssueDetails to convey more specific
/// information about the kind of issue.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InspectorIssueCode {
    #[default]
    #[serde(rename = "CookieIssue")]
    CookieIssue,
    #[serde(rename = "MixedContentIssue")]
    MixedContentIssue,
    #[serde(rename = "BlockedByResponseIssue")]
    BlockedByResponseIssue,
    #[serde(rename = "HeavyAdIssue")]
    HeavyAdIssue,
    #[serde(rename = "ContentSecurityPolicyIssue")]
    ContentSecurityPolicyIssue,
    #[serde(rename = "SharedArrayBufferIssue")]
    SharedArrayBufferIssue,
    #[serde(rename = "CorsIssue")]
    CorsIssue,
    #[serde(rename = "AttributionReportingIssue")]
    AttributionReportingIssue,
    #[serde(rename = "QuirksModeIssue")]
    QuirksModeIssue,
    #[serde(rename = "PartitioningBlobURLIssue")]
    PartitioningBlobURLIssue,
    #[serde(rename = "NavigatorUserAgentIssue")]
    NavigatorUserAgentIssue,
    #[serde(rename = "GenericIssue")]
    GenericIssue,
    #[serde(rename = "DeprecationIssue")]
    DeprecationIssue,
    #[serde(rename = "ClientHintIssue")]
    ClientHintIssue,
    #[serde(rename = "FederatedAuthRequestIssue")]
    FederatedAuthRequestIssue,
    #[serde(rename = "BounceTrackingIssue")]
    BounceTrackingIssue,
    #[serde(rename = "CookieDeprecationMetadataIssue")]
    CookieDeprecationMetadataIssue,
    #[serde(rename = "StylesheetLoadingIssue")]
    StylesheetLoadingIssue,
    #[serde(rename = "FederatedAuthUserInfoRequestIssue")]
    FederatedAuthUserInfoRequestIssue,
    #[serde(rename = "PropertyRuleIssue")]
    PropertyRuleIssue,
    #[serde(rename = "SharedDictionaryIssue")]
    SharedDictionaryIssue,
    #[serde(rename = "ElementAccessibilityIssue")]
    ElementAccessibilityIssue,
    #[serde(rename = "SRIMessageSignatureIssue")]
    SRIMessageSignatureIssue,
    #[serde(rename = "UnencodedDigestIssue")]
    UnencodedDigestIssue,
    #[serde(rename = "ConnectionAllowlistIssue")]
    ConnectionAllowlistIssue,
    #[serde(rename = "UserReidentificationIssue")]
    UserReidentificationIssue,
    #[serde(rename = "PermissionElementIssue")]
    PermissionElementIssue,
    #[serde(rename = "PerformanceIssue")]
    PerformanceIssue,
    #[serde(rename = "SelectivePermissionsInterventionIssue")]
    SelectivePermissionsInterventionIssue,
}

/// This struct holds a list of optional fields with additional information
/// specific to the kind of issue. When adding a new issue code, please also
/// add a new optional field to this type.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectorIssueDetails<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "cookieIssueDetails")]
    cookie_issue_details: Option<CookieIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mixedContentIssueDetails")]
    mixed_content_issue_details: Option<MixedContentIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockedByResponseIssueDetails")]
    blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "heavyAdIssueDetails")]
    heavy_ad_issue_details: Option<HeavyAdIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentSecurityPolicyIssueDetails")]
    content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sharedArrayBufferIssueDetails")]
    shared_array_buffer_issue_details: Option<SharedArrayBufferIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "corsIssueDetails")]
    cors_issue_details: Option<CorsIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "attributionReportingIssueDetails")]
    attribution_reporting_issue_details: Option<AttributionReportingIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quirksModeIssueDetails")]
    quirks_mode_issue_details: Option<QuirksModeIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitioningBlobURLIssueDetails")]
    partitioning_blob_url_issue_details: Option<PartitioningBlobURLIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "navigatorUserAgentIssueDetails")]
    navigator_user_agent_issue_details: Option<NavigatorUserAgentIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "genericIssueDetails")]
    generic_issue_details: Option<GenericIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "deprecationIssueDetails")]
    deprecation_issue_details: Option<DeprecationIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientHintIssueDetails")]
    client_hint_issue_details: Option<ClientHintIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "federatedAuthRequestIssueDetails")]
    federated_auth_request_issue_details: Option<FederatedAuthRequestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bounceTrackingIssueDetails")]
    bounce_tracking_issue_details: Option<BounceTrackingIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cookieDeprecationMetadataIssueDetails")]
    cookie_deprecation_metadata_issue_details: Option<CookieDeprecationMetadataIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "stylesheetLoadingIssueDetails")]
    stylesheet_loading_issue_details: Option<StylesheetLoadingIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "propertyRuleIssueDetails")]
    property_rule_issue_details: Option<PropertyRuleIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "federatedAuthUserInfoRequestIssueDetails")]
    federated_auth_user_info_request_issue_details: Option<FederatedAuthUserInfoRequestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sharedDictionaryIssueDetails")]
    shared_dictionary_issue_details: Option<SharedDictionaryIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "elementAccessibilityIssueDetails")]
    element_accessibility_issue_details: Option<ElementAccessibilityIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sriMessageSignatureIssueDetails")]
    sri_message_signature_issue_details: Option<SRIMessageSignatureIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "unencodedDigestIssueDetails")]
    unencoded_digest_issue_details: Option<UnencodedDigestIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectionAllowlistIssueDetails")]
    connection_allowlist_issue_details: Option<ConnectionAllowlistIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "userReidentificationIssueDetails")]
    user_reidentification_issue_details: Option<UserReidentificationIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "permissionElementIssueDetails")]
    permission_element_issue_details: Option<PermissionElementIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "performanceIssueDetails")]
    performance_issue_details: Option<PerformanceIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "selectivePermissionsInterventionIssueDetails")]
    selective_permissions_intervention_issue_details: Option<SelectivePermissionsInterventionIssueDetails<'a>>,
}

impl<'a> InspectorIssueDetails<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> InspectorIssueDetailsBuilder<'a> {
        InspectorIssueDetailsBuilder {
            cookie_issue_details: None,
            mixed_content_issue_details: None,
            blocked_by_response_issue_details: None,
            heavy_ad_issue_details: None,
            content_security_policy_issue_details: None,
            shared_array_buffer_issue_details: None,
            cors_issue_details: None,
            attribution_reporting_issue_details: None,
            quirks_mode_issue_details: None,
            partitioning_blob_url_issue_details: None,
            navigator_user_agent_issue_details: None,
            generic_issue_details: None,
            deprecation_issue_details: None,
            client_hint_issue_details: None,
            federated_auth_request_issue_details: None,
            bounce_tracking_issue_details: None,
            cookie_deprecation_metadata_issue_details: None,
            stylesheet_loading_issue_details: None,
            property_rule_issue_details: None,
            federated_auth_user_info_request_issue_details: None,
            shared_dictionary_issue_details: None,
            element_accessibility_issue_details: None,
            sri_message_signature_issue_details: None,
            unencoded_digest_issue_details: None,
            connection_allowlist_issue_details: None,
            user_reidentification_issue_details: None,
            permission_element_issue_details: None,
            performance_issue_details: None,
            selective_permissions_intervention_issue_details: None,
        }
    }
    pub fn cookie_issue_details(&self) -> Option<&CookieIssueDetails<'a>> { self.cookie_issue_details.as_ref() }
    pub fn mixed_content_issue_details(&self) -> Option<&MixedContentIssueDetails<'a>> { self.mixed_content_issue_details.as_ref() }
    pub fn blocked_by_response_issue_details(&self) -> Option<&BlockedByResponseIssueDetails<'a>> { self.blocked_by_response_issue_details.as_ref() }
    pub fn heavy_ad_issue_details(&self) -> Option<&HeavyAdIssueDetails<'a>> { self.heavy_ad_issue_details.as_ref() }
    pub fn content_security_policy_issue_details(&self) -> Option<&ContentSecurityPolicyIssueDetails<'a>> { self.content_security_policy_issue_details.as_ref() }
    pub fn shared_array_buffer_issue_details(&self) -> Option<&SharedArrayBufferIssueDetails<'a>> { self.shared_array_buffer_issue_details.as_ref() }
    pub fn cors_issue_details(&self) -> Option<&CorsIssueDetails<'a>> { self.cors_issue_details.as_ref() }
    pub fn attribution_reporting_issue_details(&self) -> Option<&AttributionReportingIssueDetails<'a>> { self.attribution_reporting_issue_details.as_ref() }
    pub fn quirks_mode_issue_details(&self) -> Option<&QuirksModeIssueDetails<'a>> { self.quirks_mode_issue_details.as_ref() }
    pub fn partitioning_blob_url_issue_details(&self) -> Option<&PartitioningBlobURLIssueDetails<'a>> { self.partitioning_blob_url_issue_details.as_ref() }
    pub fn navigator_user_agent_issue_details(&self) -> Option<&NavigatorUserAgentIssueDetails<'a>> { self.navigator_user_agent_issue_details.as_ref() }
    pub fn generic_issue_details(&self) -> Option<&GenericIssueDetails<'a>> { self.generic_issue_details.as_ref() }
    pub fn deprecation_issue_details(&self) -> Option<&DeprecationIssueDetails<'a>> { self.deprecation_issue_details.as_ref() }
    pub fn client_hint_issue_details(&self) -> Option<&ClientHintIssueDetails<'a>> { self.client_hint_issue_details.as_ref() }
    pub fn federated_auth_request_issue_details(&self) -> Option<&FederatedAuthRequestIssueDetails> { self.federated_auth_request_issue_details.as_ref() }
    pub fn bounce_tracking_issue_details(&self) -> Option<&BounceTrackingIssueDetails<'a>> { self.bounce_tracking_issue_details.as_ref() }
    pub fn cookie_deprecation_metadata_issue_details(&self) -> Option<&CookieDeprecationMetadataIssueDetails<'a>> { self.cookie_deprecation_metadata_issue_details.as_ref() }
    pub fn stylesheet_loading_issue_details(&self) -> Option<&StylesheetLoadingIssueDetails<'a>> { self.stylesheet_loading_issue_details.as_ref() }
    pub fn property_rule_issue_details(&self) -> Option<&PropertyRuleIssueDetails<'a>> { self.property_rule_issue_details.as_ref() }
    pub fn federated_auth_user_info_request_issue_details(&self) -> Option<&FederatedAuthUserInfoRequestIssueDetails> { self.federated_auth_user_info_request_issue_details.as_ref() }
    pub fn shared_dictionary_issue_details(&self) -> Option<&SharedDictionaryIssueDetails<'a>> { self.shared_dictionary_issue_details.as_ref() }
    pub fn element_accessibility_issue_details(&self) -> Option<&ElementAccessibilityIssueDetails> { self.element_accessibility_issue_details.as_ref() }
    pub fn sri_message_signature_issue_details(&self) -> Option<&SRIMessageSignatureIssueDetails<'a>> { self.sri_message_signature_issue_details.as_ref() }
    pub fn unencoded_digest_issue_details(&self) -> Option<&UnencodedDigestIssueDetails<'a>> { self.unencoded_digest_issue_details.as_ref() }
    pub fn connection_allowlist_issue_details(&self) -> Option<&ConnectionAllowlistIssueDetails<'a>> { self.connection_allowlist_issue_details.as_ref() }
    pub fn user_reidentification_issue_details(&self) -> Option<&UserReidentificationIssueDetails<'a>> { self.user_reidentification_issue_details.as_ref() }
    pub fn permission_element_issue_details(&self) -> Option<&PermissionElementIssueDetails<'a>> { self.permission_element_issue_details.as_ref() }
    pub fn performance_issue_details(&self) -> Option<&PerformanceIssueDetails<'a>> { self.performance_issue_details.as_ref() }
    pub fn selective_permissions_intervention_issue_details(&self) -> Option<&SelectivePermissionsInterventionIssueDetails<'a>> { self.selective_permissions_intervention_issue_details.as_ref() }
}

#[derive(Default)]
pub struct InspectorIssueDetailsBuilder<'a> {
    cookie_issue_details: Option<CookieIssueDetails<'a>>,
    mixed_content_issue_details: Option<MixedContentIssueDetails<'a>>,
    blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails<'a>>,
    heavy_ad_issue_details: Option<HeavyAdIssueDetails<'a>>,
    content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails<'a>>,
    shared_array_buffer_issue_details: Option<SharedArrayBufferIssueDetails<'a>>,
    cors_issue_details: Option<CorsIssueDetails<'a>>,
    attribution_reporting_issue_details: Option<AttributionReportingIssueDetails<'a>>,
    quirks_mode_issue_details: Option<QuirksModeIssueDetails<'a>>,
    partitioning_blob_url_issue_details: Option<PartitioningBlobURLIssueDetails<'a>>,
    navigator_user_agent_issue_details: Option<NavigatorUserAgentIssueDetails<'a>>,
    generic_issue_details: Option<GenericIssueDetails<'a>>,
    deprecation_issue_details: Option<DeprecationIssueDetails<'a>>,
    client_hint_issue_details: Option<ClientHintIssueDetails<'a>>,
    federated_auth_request_issue_details: Option<FederatedAuthRequestIssueDetails>,
    bounce_tracking_issue_details: Option<BounceTrackingIssueDetails<'a>>,
    cookie_deprecation_metadata_issue_details: Option<CookieDeprecationMetadataIssueDetails<'a>>,
    stylesheet_loading_issue_details: Option<StylesheetLoadingIssueDetails<'a>>,
    property_rule_issue_details: Option<PropertyRuleIssueDetails<'a>>,
    federated_auth_user_info_request_issue_details: Option<FederatedAuthUserInfoRequestIssueDetails>,
    shared_dictionary_issue_details: Option<SharedDictionaryIssueDetails<'a>>,
    element_accessibility_issue_details: Option<ElementAccessibilityIssueDetails>,
    sri_message_signature_issue_details: Option<SRIMessageSignatureIssueDetails<'a>>,
    unencoded_digest_issue_details: Option<UnencodedDigestIssueDetails<'a>>,
    connection_allowlist_issue_details: Option<ConnectionAllowlistIssueDetails<'a>>,
    user_reidentification_issue_details: Option<UserReidentificationIssueDetails<'a>>,
    permission_element_issue_details: Option<PermissionElementIssueDetails<'a>>,
    performance_issue_details: Option<PerformanceIssueDetails<'a>>,
    selective_permissions_intervention_issue_details: Option<SelectivePermissionsInterventionIssueDetails<'a>>,
}

impl<'a> InspectorIssueDetailsBuilder<'a> {
    pub fn cookie_issue_details(mut self, cookie_issue_details: CookieIssueDetails<'a>) -> Self { self.cookie_issue_details = Some(cookie_issue_details); self }
    pub fn mixed_content_issue_details(mut self, mixed_content_issue_details: MixedContentIssueDetails<'a>) -> Self { self.mixed_content_issue_details = Some(mixed_content_issue_details); self }
    pub fn blocked_by_response_issue_details(mut self, blocked_by_response_issue_details: BlockedByResponseIssueDetails<'a>) -> Self { self.blocked_by_response_issue_details = Some(blocked_by_response_issue_details); self }
    pub fn heavy_ad_issue_details(mut self, heavy_ad_issue_details: HeavyAdIssueDetails<'a>) -> Self { self.heavy_ad_issue_details = Some(heavy_ad_issue_details); self }
    pub fn content_security_policy_issue_details(mut self, content_security_policy_issue_details: ContentSecurityPolicyIssueDetails<'a>) -> Self { self.content_security_policy_issue_details = Some(content_security_policy_issue_details); self }
    pub fn shared_array_buffer_issue_details(mut self, shared_array_buffer_issue_details: SharedArrayBufferIssueDetails<'a>) -> Self { self.shared_array_buffer_issue_details = Some(shared_array_buffer_issue_details); self }
    pub fn cors_issue_details(mut self, cors_issue_details: CorsIssueDetails<'a>) -> Self { self.cors_issue_details = Some(cors_issue_details); self }
    pub fn attribution_reporting_issue_details(mut self, attribution_reporting_issue_details: AttributionReportingIssueDetails<'a>) -> Self { self.attribution_reporting_issue_details = Some(attribution_reporting_issue_details); self }
    pub fn quirks_mode_issue_details(mut self, quirks_mode_issue_details: QuirksModeIssueDetails<'a>) -> Self { self.quirks_mode_issue_details = Some(quirks_mode_issue_details); self }
    pub fn partitioning_blob_url_issue_details(mut self, partitioning_blob_url_issue_details: PartitioningBlobURLIssueDetails<'a>) -> Self { self.partitioning_blob_url_issue_details = Some(partitioning_blob_url_issue_details); self }
    pub fn navigator_user_agent_issue_details(mut self, navigator_user_agent_issue_details: NavigatorUserAgentIssueDetails<'a>) -> Self { self.navigator_user_agent_issue_details = Some(navigator_user_agent_issue_details); self }
    pub fn generic_issue_details(mut self, generic_issue_details: GenericIssueDetails<'a>) -> Self { self.generic_issue_details = Some(generic_issue_details); self }
    pub fn deprecation_issue_details(mut self, deprecation_issue_details: DeprecationIssueDetails<'a>) -> Self { self.deprecation_issue_details = Some(deprecation_issue_details); self }
    pub fn client_hint_issue_details(mut self, client_hint_issue_details: ClientHintIssueDetails<'a>) -> Self { self.client_hint_issue_details = Some(client_hint_issue_details); self }
    pub fn federated_auth_request_issue_details(mut self, federated_auth_request_issue_details: FederatedAuthRequestIssueDetails) -> Self { self.federated_auth_request_issue_details = Some(federated_auth_request_issue_details); self }
    pub fn bounce_tracking_issue_details(mut self, bounce_tracking_issue_details: BounceTrackingIssueDetails<'a>) -> Self { self.bounce_tracking_issue_details = Some(bounce_tracking_issue_details); self }
    pub fn cookie_deprecation_metadata_issue_details(mut self, cookie_deprecation_metadata_issue_details: CookieDeprecationMetadataIssueDetails<'a>) -> Self { self.cookie_deprecation_metadata_issue_details = Some(cookie_deprecation_metadata_issue_details); self }
    pub fn stylesheet_loading_issue_details(mut self, stylesheet_loading_issue_details: StylesheetLoadingIssueDetails<'a>) -> Self { self.stylesheet_loading_issue_details = Some(stylesheet_loading_issue_details); self }
    pub fn property_rule_issue_details(mut self, property_rule_issue_details: PropertyRuleIssueDetails<'a>) -> Self { self.property_rule_issue_details = Some(property_rule_issue_details); self }
    pub fn federated_auth_user_info_request_issue_details(mut self, federated_auth_user_info_request_issue_details: FederatedAuthUserInfoRequestIssueDetails) -> Self { self.federated_auth_user_info_request_issue_details = Some(federated_auth_user_info_request_issue_details); self }
    pub fn shared_dictionary_issue_details(mut self, shared_dictionary_issue_details: SharedDictionaryIssueDetails<'a>) -> Self { self.shared_dictionary_issue_details = Some(shared_dictionary_issue_details); self }
    pub fn element_accessibility_issue_details(mut self, element_accessibility_issue_details: ElementAccessibilityIssueDetails) -> Self { self.element_accessibility_issue_details = Some(element_accessibility_issue_details); self }
    pub fn sri_message_signature_issue_details(mut self, sri_message_signature_issue_details: SRIMessageSignatureIssueDetails<'a>) -> Self { self.sri_message_signature_issue_details = Some(sri_message_signature_issue_details); self }
    pub fn unencoded_digest_issue_details(mut self, unencoded_digest_issue_details: UnencodedDigestIssueDetails<'a>) -> Self { self.unencoded_digest_issue_details = Some(unencoded_digest_issue_details); self }
    pub fn connection_allowlist_issue_details(mut self, connection_allowlist_issue_details: ConnectionAllowlistIssueDetails<'a>) -> Self { self.connection_allowlist_issue_details = Some(connection_allowlist_issue_details); self }
    pub fn user_reidentification_issue_details(mut self, user_reidentification_issue_details: UserReidentificationIssueDetails<'a>) -> Self { self.user_reidentification_issue_details = Some(user_reidentification_issue_details); self }
    pub fn permission_element_issue_details(mut self, permission_element_issue_details: PermissionElementIssueDetails<'a>) -> Self { self.permission_element_issue_details = Some(permission_element_issue_details); self }
    pub fn performance_issue_details(mut self, performance_issue_details: PerformanceIssueDetails<'a>) -> Self { self.performance_issue_details = Some(performance_issue_details); self }
    pub fn selective_permissions_intervention_issue_details(mut self, selective_permissions_intervention_issue_details: SelectivePermissionsInterventionIssueDetails<'a>) -> Self { self.selective_permissions_intervention_issue_details = Some(selective_permissions_intervention_issue_details); self }
    pub fn build(self) -> InspectorIssueDetails<'a> {
        InspectorIssueDetails {
            cookie_issue_details: self.cookie_issue_details,
            mixed_content_issue_details: self.mixed_content_issue_details,
            blocked_by_response_issue_details: self.blocked_by_response_issue_details,
            heavy_ad_issue_details: self.heavy_ad_issue_details,
            content_security_policy_issue_details: self.content_security_policy_issue_details,
            shared_array_buffer_issue_details: self.shared_array_buffer_issue_details,
            cors_issue_details: self.cors_issue_details,
            attribution_reporting_issue_details: self.attribution_reporting_issue_details,
            quirks_mode_issue_details: self.quirks_mode_issue_details,
            partitioning_blob_url_issue_details: self.partitioning_blob_url_issue_details,
            navigator_user_agent_issue_details: self.navigator_user_agent_issue_details,
            generic_issue_details: self.generic_issue_details,
            deprecation_issue_details: self.deprecation_issue_details,
            client_hint_issue_details: self.client_hint_issue_details,
            federated_auth_request_issue_details: self.federated_auth_request_issue_details,
            bounce_tracking_issue_details: self.bounce_tracking_issue_details,
            cookie_deprecation_metadata_issue_details: self.cookie_deprecation_metadata_issue_details,
            stylesheet_loading_issue_details: self.stylesheet_loading_issue_details,
            property_rule_issue_details: self.property_rule_issue_details,
            federated_auth_user_info_request_issue_details: self.federated_auth_user_info_request_issue_details,
            shared_dictionary_issue_details: self.shared_dictionary_issue_details,
            element_accessibility_issue_details: self.element_accessibility_issue_details,
            sri_message_signature_issue_details: self.sri_message_signature_issue_details,
            unencoded_digest_issue_details: self.unencoded_digest_issue_details,
            connection_allowlist_issue_details: self.connection_allowlist_issue_details,
            user_reidentification_issue_details: self.user_reidentification_issue_details,
            permission_element_issue_details: self.permission_element_issue_details,
            performance_issue_details: self.performance_issue_details,
            selective_permissions_intervention_issue_details: self.selective_permissions_intervention_issue_details,
        }
    }
}

/// A unique id for a DevTools inspector issue. Allows other entities (e.g.
/// exceptions, CDP message, console messages, etc.) to reference an issue.

pub type IssueId<'a> = Cow<'a, str>;

/// An inspector issue reported from the back-end.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectorIssue<'a> {
    code: InspectorIssueCode,
    details: InspectorIssueDetails<'a>,
    /// A unique id for this issue. May be omitted if no other entity (e.g.
    /// exception, CDP message, etc.) is referencing this issue.
    #[serde(skip_serializing_if = "Option::is_none", rename = "issueId")]
    issue_id: Option<IssueId<'a>>,
}

impl<'a> InspectorIssue<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `code`: 
    /// * `details`: 
    pub fn builder(code: impl Into<InspectorIssueCode>, details: InspectorIssueDetails<'a>) -> InspectorIssueBuilder<'a> {
        InspectorIssueBuilder {
            code: code.into(),
            details: details,
            issue_id: None,
        }
    }
    pub fn code(&self) -> &InspectorIssueCode { &self.code }
    pub fn details(&self) -> &InspectorIssueDetails<'a> { &self.details }
    /// A unique id for this issue. May be omitted if no other entity (e.g.
    /// exception, CDP message, etc.) is referencing this issue.
    pub fn issue_id(&self) -> Option<&IssueId<'a>> { self.issue_id.as_ref() }
}


pub struct InspectorIssueBuilder<'a> {
    code: InspectorIssueCode,
    details: InspectorIssueDetails<'a>,
    issue_id: Option<IssueId<'a>>,
}

impl<'a> InspectorIssueBuilder<'a> {
    /// A unique id for this issue. May be omitted if no other entity (e.g.
    /// exception, CDP message, etc.) is referencing this issue.
    pub fn issue_id(mut self, issue_id: impl Into<IssueId<'a>>) -> Self { self.issue_id = Some(issue_id.into()); self }
    pub fn build(self) -> InspectorIssue<'a> {
        InspectorIssue {
            code: self.code,
            details: self.details,
            issue_id: self.issue_id,
        }
    }
}

/// Returns the response body and size if it were re-encoded with the specified settings. Only
/// applies to images.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEncodedResponseParams<'a> {
    /// Identifier of the network request to get content for.
    #[serde(rename = "requestId")]
    request_id: crate::network::RequestId<'a>,
    /// The encoding to use.
    encoding: Cow<'a, str>,
    /// The quality of the encoding (0-1). (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<f64>,
    /// Whether to only return the size information (defaults to false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "sizeOnly")]
    size_only: Option<bool>,
}

impl<'a> GetEncodedResponseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: Identifier of the network request to get content for.
    /// * `encoding`: The encoding to use.
    pub fn builder(request_id: crate::network::RequestId<'a>, encoding: impl Into<Cow<'a, str>>) -> GetEncodedResponseParamsBuilder<'a> {
        GetEncodedResponseParamsBuilder {
            request_id: request_id,
            encoding: encoding.into(),
            quality: None,
            size_only: None,
        }
    }
    /// Identifier of the network request to get content for.
    pub fn request_id(&self) -> &crate::network::RequestId<'a> { &self.request_id }
    /// The encoding to use.
    pub fn encoding(&self) -> &str { self.encoding.as_ref() }
    /// The quality of the encoding (0-1). (defaults to 1)
    pub fn quality(&self) -> Option<f64> { self.quality }
    /// Whether to only return the size information (defaults to false).
    pub fn size_only(&self) -> Option<bool> { self.size_only }
}


pub struct GetEncodedResponseParamsBuilder<'a> {
    request_id: crate::network::RequestId<'a>,
    encoding: Cow<'a, str>,
    quality: Option<f64>,
    size_only: Option<bool>,
}

impl<'a> GetEncodedResponseParamsBuilder<'a> {
    /// The quality of the encoding (0-1). (defaults to 1)
    pub fn quality(mut self, quality: f64) -> Self { self.quality = Some(quality); self }
    /// Whether to only return the size information (defaults to false).
    pub fn size_only(mut self, size_only: bool) -> Self { self.size_only = Some(size_only); self }
    pub fn build(self) -> GetEncodedResponseParams<'a> {
        GetEncodedResponseParams {
            request_id: self.request_id,
            encoding: self.encoding,
            quality: self.quality,
            size_only: self.size_only,
        }
    }
}

/// Returns the response body and size if it were re-encoded with the specified settings. Only
/// applies to images.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEncodedResponseReturns<'a> {
    /// The encoded body as a base64 string. Omitted if sizeOnly is true. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Cow<'a, str>>,
    /// Size before re-encoding.
    #[serde(rename = "originalSize")]
    original_size: u64,
    /// Size after re-encoding.
    #[serde(rename = "encodedSize")]
    encoded_size: u64,
}

impl<'a> GetEncodedResponseReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `original_size`: Size before re-encoding.
    /// * `encoded_size`: Size after re-encoding.
    pub fn builder(original_size: u64, encoded_size: u64) -> GetEncodedResponseReturnsBuilder<'a> {
        GetEncodedResponseReturnsBuilder {
            body: None,
            original_size: original_size,
            encoded_size: encoded_size,
        }
    }
    /// The encoded body as a base64 string. Omitted if sizeOnly is true. (Encoded as a base64 string when passed over JSON)
    pub fn body(&self) -> Option<&str> { self.body.as_deref() }
    /// Size before re-encoding.
    pub fn original_size(&self) -> u64 { self.original_size }
    /// Size after re-encoding.
    pub fn encoded_size(&self) -> u64 { self.encoded_size }
}


pub struct GetEncodedResponseReturnsBuilder<'a> {
    body: Option<Cow<'a, str>>,
    original_size: u64,
    encoded_size: u64,
}

impl<'a> GetEncodedResponseReturnsBuilder<'a> {
    /// The encoded body as a base64 string. Omitted if sizeOnly is true. (Encoded as a base64 string when passed over JSON)
    pub fn body(mut self, body: impl Into<Cow<'a, str>>) -> Self { self.body = Some(body.into()); self }
    pub fn build(self) -> GetEncodedResponseReturns<'a> {
        GetEncodedResponseReturns {
            body: self.body,
            original_size: self.original_size,
            encoded_size: self.encoded_size,
        }
    }
}

impl<'a> GetEncodedResponseParams<'a> { pub const METHOD: &'static str = "Audits.getEncodedResponse"; }

impl<'a> crate::CdpCommand<'a> for GetEncodedResponseParams<'a> {
    const METHOD: &'static str = "Audits.getEncodedResponse";
    type Response = GetEncodedResponseReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Audits.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Audits.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Audits.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Audits.enable";
    type Response = crate::EmptyReturns;
}

/// Runs the form issues check for the target page. Found issues are reported
/// using Audits.issueAdded event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CheckFormsIssuesReturns<'a> {
    #[serde(rename = "formIssues")]
    form_issues: Vec<GenericIssueDetails<'a>>,
}

impl<'a> CheckFormsIssuesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `form_issues`: 
    pub fn builder(form_issues: Vec<GenericIssueDetails<'a>>) -> CheckFormsIssuesReturnsBuilder<'a> {
        CheckFormsIssuesReturnsBuilder {
            form_issues: form_issues,
        }
    }
    pub fn form_issues(&self) -> &[GenericIssueDetails<'a>] { &self.form_issues }
}


pub struct CheckFormsIssuesReturnsBuilder<'a> {
    form_issues: Vec<GenericIssueDetails<'a>>,
}

impl<'a> CheckFormsIssuesReturnsBuilder<'a> {
    pub fn build(self) -> CheckFormsIssuesReturns<'a> {
        CheckFormsIssuesReturns {
            form_issues: self.form_issues,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckFormsIssuesParams {}

impl CheckFormsIssuesParams { pub const METHOD: &'static str = "Audits.checkFormsIssues"; }

impl<'a> crate::CdpCommand<'a> for CheckFormsIssuesParams {
    const METHOD: &'static str = "Audits.checkFormsIssues";
    type Response = CheckFormsIssuesReturns<'a>;
}
