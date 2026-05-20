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
    pub fn builder() -> AffectedCookieBuilder<'a> { AffectedCookieBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn path(&self) -> &str { self.path.as_ref() }
    pub fn domain(&self) -> &str { self.domain.as_ref() }
}

#[derive(Default)]
pub struct AffectedCookieBuilder<'a> {
    name: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    domain: Option<Cow<'a, str>>,
}

impl<'a> AffectedCookieBuilder<'a> {
    /// The following three properties uniquely identify a cookie
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    pub fn domain(mut self, domain: impl Into<Cow<'a, str>>) -> Self { self.domain = Some(domain.into()); self }
    pub fn build(self) -> AffectedCookie<'a> {
        AffectedCookie {
            name: self.name.unwrap_or_default(),
            path: self.path.unwrap_or_default(),
            domain: self.domain.unwrap_or_default(),
        }
    }
}

/// Information about a request that is affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AffectedRequest<'a> {
    /// The unique request id.
    #[serde(skip_serializing_if = "Option::is_none")]
    requestId: Option<crate::network::RequestId<'a>>,
    url: Cow<'a, str>,
}

impl<'a> AffectedRequest<'a> {
    pub fn builder() -> AffectedRequestBuilder<'a> { AffectedRequestBuilder::default() }
    pub fn requestId(&self) -> Option<&crate::network::RequestId<'a>> { self.requestId.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}

#[derive(Default)]
pub struct AffectedRequestBuilder<'a> {
    requestId: Option<crate::network::RequestId<'a>>,
    url: Option<Cow<'a, str>>,
}

impl<'a> AffectedRequestBuilder<'a> {
    /// The unique request id.
    pub fn requestId(mut self, requestId: crate::network::RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> AffectedRequest<'a> {
        AffectedRequest {
            requestId: self.requestId,
            url: self.url.unwrap_or_default(),
        }
    }
}

/// Information about the frame affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AffectedFrame<'a> {
    frameId: crate::page::FrameId<'a>,
}

impl<'a> AffectedFrame<'a> {
    pub fn builder() -> AffectedFrameBuilder<'a> { AffectedFrameBuilder::default() }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
}

#[derive(Default)]
pub struct AffectedFrameBuilder<'a> {
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> AffectedFrameBuilder<'a> {
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> AffectedFrame<'a> {
        AffectedFrame {
            frameId: self.frameId.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tableEntryUrl: Option<Cow<'a, str>>,
}

impl<'a> CookieIssueInsight<'a> {
    pub fn builder() -> CookieIssueInsightBuilder<'a> { CookieIssueInsightBuilder::default() }
    pub fn type_(&self) -> &InsightType { &self.type_ }
    pub fn tableEntryUrl(&self) -> Option<&str> { self.tableEntryUrl.as_deref() }
}

#[derive(Default)]
pub struct CookieIssueInsightBuilder<'a> {
    type_: Option<InsightType>,
    tableEntryUrl: Option<Cow<'a, str>>,
}

impl<'a> CookieIssueInsightBuilder<'a> {
    pub fn type_(mut self, type_: InsightType) -> Self { self.type_ = Some(type_); self }
    /// Link to table entry in third-party cookie migration readiness list.
    pub fn tableEntryUrl(mut self, tableEntryUrl: impl Into<Cow<'a, str>>) -> Self { self.tableEntryUrl = Some(tableEntryUrl.into()); self }
    pub fn build(self) -> CookieIssueInsight<'a> {
        CookieIssueInsight {
            type_: self.type_.unwrap_or_default(),
            tableEntryUrl: self.tableEntryUrl,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    rawCookieLine: Option<Cow<'a, str>>,
    cookieWarningReasons: Vec<CookieWarningReason>,
    cookieExclusionReasons: Vec<CookieExclusionReason>,
    /// Optionally identifies the site-for-cookies and the cookie url, which
    /// may be used by the front-end as additional context.
    operation: CookieOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    siteForCookies: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookieUrl: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AffectedRequest<'a>>,
    /// The recommended solution to the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    insight: Option<CookieIssueInsight<'a>>,
}

impl<'a> CookieIssueDetails<'a> {
    pub fn builder() -> CookieIssueDetailsBuilder<'a> { CookieIssueDetailsBuilder::default() }
    pub fn cookie(&self) -> Option<&AffectedCookie<'a>> { self.cookie.as_ref() }
    pub fn rawCookieLine(&self) -> Option<&str> { self.rawCookieLine.as_deref() }
    pub fn cookieWarningReasons(&self) -> &[CookieWarningReason] { &self.cookieWarningReasons }
    pub fn cookieExclusionReasons(&self) -> &[CookieExclusionReason] { &self.cookieExclusionReasons }
    pub fn operation(&self) -> &CookieOperation { &self.operation }
    pub fn siteForCookies(&self) -> Option<&str> { self.siteForCookies.as_deref() }
    pub fn cookieUrl(&self) -> Option<&str> { self.cookieUrl.as_deref() }
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
    pub fn insight(&self) -> Option<&CookieIssueInsight<'a>> { self.insight.as_ref() }
}

#[derive(Default)]
pub struct CookieIssueDetailsBuilder<'a> {
    cookie: Option<AffectedCookie<'a>>,
    rawCookieLine: Option<Cow<'a, str>>,
    cookieWarningReasons: Option<Vec<CookieWarningReason>>,
    cookieExclusionReasons: Option<Vec<CookieExclusionReason>>,
    operation: Option<CookieOperation>,
    siteForCookies: Option<Cow<'a, str>>,
    cookieUrl: Option<Cow<'a, str>>,
    request: Option<AffectedRequest<'a>>,
    insight: Option<CookieIssueInsight<'a>>,
}

impl<'a> CookieIssueDetailsBuilder<'a> {
    /// If AffectedCookie is not set then rawCookieLine contains the raw
    /// Set-Cookie header string. This hints at a problem where the
    /// cookie line is syntactically or semantically malformed in a way
    /// that no valid cookie could be created.
    pub fn cookie(mut self, cookie: AffectedCookie<'a>) -> Self { self.cookie = Some(cookie); self }
    pub fn rawCookieLine(mut self, rawCookieLine: impl Into<Cow<'a, str>>) -> Self { self.rawCookieLine = Some(rawCookieLine.into()); self }
    pub fn cookieWarningReasons(mut self, cookieWarningReasons: Vec<CookieWarningReason>) -> Self { self.cookieWarningReasons = Some(cookieWarningReasons); self }
    pub fn cookieExclusionReasons(mut self, cookieExclusionReasons: Vec<CookieExclusionReason>) -> Self { self.cookieExclusionReasons = Some(cookieExclusionReasons); self }
    /// Optionally identifies the site-for-cookies and the cookie url, which
    /// may be used by the front-end as additional context.
    pub fn operation(mut self, operation: CookieOperation) -> Self { self.operation = Some(operation); self }
    pub fn siteForCookies(mut self, siteForCookies: impl Into<Cow<'a, str>>) -> Self { self.siteForCookies = Some(siteForCookies.into()); self }
    pub fn cookieUrl(mut self, cookieUrl: impl Into<Cow<'a, str>>) -> Self { self.cookieUrl = Some(cookieUrl.into()); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    /// The recommended solution to the issue.
    pub fn insight(mut self, insight: CookieIssueInsight<'a>) -> Self { self.insight = Some(insight); self }
    pub fn build(self) -> CookieIssueDetails<'a> {
        CookieIssueDetails {
            cookie: self.cookie,
            rawCookieLine: self.rawCookieLine,
            cookieWarningReasons: self.cookieWarningReasons.unwrap_or_default(),
            cookieExclusionReasons: self.cookieExclusionReasons.unwrap_or_default(),
            operation: self.operation.unwrap_or_default(),
            siteForCookies: self.siteForCookies,
            cookieUrl: self.cookieUrl,
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
    performanceIssueType: PerformanceIssueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
}

impl<'a> PerformanceIssueDetails<'a> {
    pub fn builder() -> PerformanceIssueDetailsBuilder<'a> { PerformanceIssueDetailsBuilder::default() }
    pub fn performanceIssueType(&self) -> &PerformanceIssueType { &self.performanceIssueType }
    pub fn sourceCodeLocation(&self) -> Option<&SourceCodeLocation<'a>> { self.sourceCodeLocation.as_ref() }
}

#[derive(Default)]
pub struct PerformanceIssueDetailsBuilder<'a> {
    performanceIssueType: Option<PerformanceIssueType>,
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
}

impl<'a> PerformanceIssueDetailsBuilder<'a> {
    pub fn performanceIssueType(mut self, performanceIssueType: PerformanceIssueType) -> Self { self.performanceIssueType = Some(performanceIssueType); self }
    pub fn sourceCodeLocation(mut self, sourceCodeLocation: SourceCodeLocation<'a>) -> Self { self.sourceCodeLocation = Some(sourceCodeLocation); self }
    pub fn build(self) -> PerformanceIssueDetails<'a> {
        PerformanceIssueDetails {
            performanceIssueType: self.performanceIssueType.unwrap_or_default(),
            sourceCodeLocation: self.sourceCodeLocation,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    resourceType: Option<MixedContentResourceType>,
    /// The way the mixed content issue is being resolved.
    resolutionStatus: MixedContentResolutionStatus,
    /// The unsafe http url causing the mixed content issue.
    insecureURL: Cow<'a, str>,
    /// The url responsible for the call to an unsafe url.
    mainResourceURL: Cow<'a, str>,
    /// The mixed content request.
    /// Does not always exist (e.g. for unsafe form submission urls).
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AffectedRequest<'a>>,
    /// Optional because not every mixed content issue is necessarily linked to a frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    frame: Option<AffectedFrame<'a>>,
}

impl<'a> MixedContentIssueDetails<'a> {
    pub fn builder() -> MixedContentIssueDetailsBuilder<'a> { MixedContentIssueDetailsBuilder::default() }
    pub fn resourceType(&self) -> Option<&MixedContentResourceType> { self.resourceType.as_ref() }
    pub fn resolutionStatus(&self) -> &MixedContentResolutionStatus { &self.resolutionStatus }
    pub fn insecureURL(&self) -> &str { self.insecureURL.as_ref() }
    pub fn mainResourceURL(&self) -> &str { self.mainResourceURL.as_ref() }
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
    pub fn frame(&self) -> Option<&AffectedFrame<'a>> { self.frame.as_ref() }
}

#[derive(Default)]
pub struct MixedContentIssueDetailsBuilder<'a> {
    resourceType: Option<MixedContentResourceType>,
    resolutionStatus: Option<MixedContentResolutionStatus>,
    insecureURL: Option<Cow<'a, str>>,
    mainResourceURL: Option<Cow<'a, str>>,
    request: Option<AffectedRequest<'a>>,
    frame: Option<AffectedFrame<'a>>,
}

impl<'a> MixedContentIssueDetailsBuilder<'a> {
    /// The type of resource causing the mixed content issue (css, js, iframe,
    /// form,...). Marked as optional because it is mapped to from
    /// blink::mojom::RequestContextType, which will be replaced
    /// by network::mojom::RequestDestination
    pub fn resourceType(mut self, resourceType: MixedContentResourceType) -> Self { self.resourceType = Some(resourceType); self }
    /// The way the mixed content issue is being resolved.
    pub fn resolutionStatus(mut self, resolutionStatus: MixedContentResolutionStatus) -> Self { self.resolutionStatus = Some(resolutionStatus); self }
    /// The unsafe http url causing the mixed content issue.
    pub fn insecureURL(mut self, insecureURL: impl Into<Cow<'a, str>>) -> Self { self.insecureURL = Some(insecureURL.into()); self }
    /// The url responsible for the call to an unsafe url.
    pub fn mainResourceURL(mut self, mainResourceURL: impl Into<Cow<'a, str>>) -> Self { self.mainResourceURL = Some(mainResourceURL.into()); self }
    /// The mixed content request.
    /// Does not always exist (e.g. for unsafe form submission urls).
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    /// Optional because not every mixed content issue is necessarily linked to a frame.
    pub fn frame(mut self, frame: AffectedFrame<'a>) -> Self { self.frame = Some(frame); self }
    pub fn build(self) -> MixedContentIssueDetails<'a> {
        MixedContentIssueDetails {
            resourceType: self.resourceType,
            resolutionStatus: self.resolutionStatus.unwrap_or_default(),
            insecureURL: self.insecureURL.unwrap_or_default(),
            mainResourceURL: self.mainResourceURL.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    parentFrame: Option<AffectedFrame<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blockedFrame: Option<AffectedFrame<'a>>,
    reason: BlockedByResponseReason,
}

impl<'a> BlockedByResponseIssueDetails<'a> {
    pub fn builder() -> BlockedByResponseIssueDetailsBuilder<'a> { BlockedByResponseIssueDetailsBuilder::default() }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
    pub fn parentFrame(&self) -> Option<&AffectedFrame<'a>> { self.parentFrame.as_ref() }
    pub fn blockedFrame(&self) -> Option<&AffectedFrame<'a>> { self.blockedFrame.as_ref() }
    pub fn reason(&self) -> &BlockedByResponseReason { &self.reason }
}

#[derive(Default)]
pub struct BlockedByResponseIssueDetailsBuilder<'a> {
    request: Option<AffectedRequest<'a>>,
    parentFrame: Option<AffectedFrame<'a>>,
    blockedFrame: Option<AffectedFrame<'a>>,
    reason: Option<BlockedByResponseReason>,
}

impl<'a> BlockedByResponseIssueDetailsBuilder<'a> {
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn parentFrame(mut self, parentFrame: AffectedFrame<'a>) -> Self { self.parentFrame = Some(parentFrame); self }
    pub fn blockedFrame(mut self, blockedFrame: AffectedFrame<'a>) -> Self { self.blockedFrame = Some(blockedFrame); self }
    pub fn reason(mut self, reason: BlockedByResponseReason) -> Self { self.reason = Some(reason); self }
    pub fn build(self) -> BlockedByResponseIssueDetails<'a> {
        BlockedByResponseIssueDetails {
            request: self.request.unwrap_or_default(),
            parentFrame: self.parentFrame,
            blockedFrame: self.blockedFrame,
            reason: self.reason.unwrap_or_default(),
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
    pub fn builder() -> HeavyAdIssueDetailsBuilder<'a> { HeavyAdIssueDetailsBuilder::default() }
    pub fn resolution(&self) -> &HeavyAdResolutionStatus { &self.resolution }
    pub fn reason(&self) -> &HeavyAdReason { &self.reason }
    pub fn frame(&self) -> &AffectedFrame<'a> { &self.frame }
}

#[derive(Default)]
pub struct HeavyAdIssueDetailsBuilder<'a> {
    resolution: Option<HeavyAdResolutionStatus>,
    reason: Option<HeavyAdReason>,
    frame: Option<AffectedFrame<'a>>,
}

impl<'a> HeavyAdIssueDetailsBuilder<'a> {
    /// The resolution status, either blocking the content or warning.
    pub fn resolution(mut self, resolution: HeavyAdResolutionStatus) -> Self { self.resolution = Some(resolution); self }
    /// The reason the ad was blocked, total network or cpu or peak cpu.
    pub fn reason(mut self, reason: HeavyAdReason) -> Self { self.reason = Some(reason); self }
    /// The frame that was blocked.
    pub fn frame(mut self, frame: AffectedFrame<'a>) -> Self { self.frame = Some(frame); self }
    pub fn build(self) -> HeavyAdIssueDetails<'a> {
        HeavyAdIssueDetails {
            resolution: self.resolution.unwrap_or_default(),
            reason: self.reason.unwrap_or_default(),
            frame: self.frame.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    scriptId: Option<crate::runtime::ScriptId<'a>>,
    url: Cow<'a, str>,
    lineNumber: i64,
    columnNumber: i64,
}

impl<'a> SourceCodeLocation<'a> {
    pub fn builder() -> SourceCodeLocationBuilder<'a> { SourceCodeLocationBuilder::default() }
    pub fn scriptId(&self) -> Option<&crate::runtime::ScriptId<'a>> { self.scriptId.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn lineNumber(&self) -> i64 { self.lineNumber }
    pub fn columnNumber(&self) -> i64 { self.columnNumber }
}

#[derive(Default)]
pub struct SourceCodeLocationBuilder<'a> {
    scriptId: Option<crate::runtime::ScriptId<'a>>,
    url: Option<Cow<'a, str>>,
    lineNumber: Option<i64>,
    columnNumber: Option<i64>,
}

impl<'a> SourceCodeLocationBuilder<'a> {
    pub fn scriptId(mut self, scriptId: crate::runtime::ScriptId<'a>) -> Self { self.scriptId = Some(scriptId); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn lineNumber(mut self, lineNumber: i64) -> Self { self.lineNumber = Some(lineNumber); self }
    pub fn columnNumber(mut self, columnNumber: i64) -> Self { self.columnNumber = Some(columnNumber); self }
    pub fn build(self) -> SourceCodeLocation<'a> {
        SourceCodeLocation {
            scriptId: self.scriptId,
            url: self.url.unwrap_or_default(),
            lineNumber: self.lineNumber.unwrap_or_default(),
            columnNumber: self.columnNumber.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyIssueDetails<'a> {
    /// The url not included in allowed sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    blockedURL: Option<Cow<'a, str>>,
    /// Specific directive that is violated, causing the CSP issue.
    violatedDirective: Cow<'a, str>,
    isReportOnly: bool,
    contentSecurityPolicyViolationType: ContentSecurityPolicyViolationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    frameAncestor: Option<AffectedFrame<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    violatingNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> ContentSecurityPolicyIssueDetails<'a> {
    pub fn builder() -> ContentSecurityPolicyIssueDetailsBuilder<'a> { ContentSecurityPolicyIssueDetailsBuilder::default() }
    pub fn blockedURL(&self) -> Option<&str> { self.blockedURL.as_deref() }
    pub fn violatedDirective(&self) -> &str { self.violatedDirective.as_ref() }
    pub fn isReportOnly(&self) -> bool { self.isReportOnly }
    pub fn contentSecurityPolicyViolationType(&self) -> &ContentSecurityPolicyViolationType { &self.contentSecurityPolicyViolationType }
    pub fn frameAncestor(&self) -> Option<&AffectedFrame<'a>> { self.frameAncestor.as_ref() }
    pub fn sourceCodeLocation(&self) -> Option<&SourceCodeLocation<'a>> { self.sourceCodeLocation.as_ref() }
    pub fn violatingNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.violatingNodeId.as_ref() }
}

#[derive(Default)]
pub struct ContentSecurityPolicyIssueDetailsBuilder<'a> {
    blockedURL: Option<Cow<'a, str>>,
    violatedDirective: Option<Cow<'a, str>>,
    isReportOnly: Option<bool>,
    contentSecurityPolicyViolationType: Option<ContentSecurityPolicyViolationType>,
    frameAncestor: Option<AffectedFrame<'a>>,
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
    violatingNodeId: Option<crate::dom::BackendNodeId>,
}

impl<'a> ContentSecurityPolicyIssueDetailsBuilder<'a> {
    /// The url not included in allowed sources.
    pub fn blockedURL(mut self, blockedURL: impl Into<Cow<'a, str>>) -> Self { self.blockedURL = Some(blockedURL.into()); self }
    /// Specific directive that is violated, causing the CSP issue.
    pub fn violatedDirective(mut self, violatedDirective: impl Into<Cow<'a, str>>) -> Self { self.violatedDirective = Some(violatedDirective.into()); self }
    pub fn isReportOnly(mut self, isReportOnly: bool) -> Self { self.isReportOnly = Some(isReportOnly); self }
    pub fn contentSecurityPolicyViolationType(mut self, contentSecurityPolicyViolationType: ContentSecurityPolicyViolationType) -> Self { self.contentSecurityPolicyViolationType = Some(contentSecurityPolicyViolationType); self }
    pub fn frameAncestor(mut self, frameAncestor: AffectedFrame<'a>) -> Self { self.frameAncestor = Some(frameAncestor); self }
    pub fn sourceCodeLocation(mut self, sourceCodeLocation: SourceCodeLocation<'a>) -> Self { self.sourceCodeLocation = Some(sourceCodeLocation); self }
    pub fn violatingNodeId(mut self, violatingNodeId: crate::dom::BackendNodeId) -> Self { self.violatingNodeId = Some(violatingNodeId); self }
    pub fn build(self) -> ContentSecurityPolicyIssueDetails<'a> {
        ContentSecurityPolicyIssueDetails {
            blockedURL: self.blockedURL,
            violatedDirective: self.violatedDirective.unwrap_or_default(),
            isReportOnly: self.isReportOnly.unwrap_or_default(),
            contentSecurityPolicyViolationType: self.contentSecurityPolicyViolationType.unwrap_or_default(),
            frameAncestor: self.frameAncestor,
            sourceCodeLocation: self.sourceCodeLocation,
            violatingNodeId: self.violatingNodeId,
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
    sourceCodeLocation: SourceCodeLocation<'a>,
    isWarning: bool,
    #[serde(rename = "type")]
    type_: SharedArrayBufferIssueType,
}

impl<'a> SharedArrayBufferIssueDetails<'a> {
    pub fn builder() -> SharedArrayBufferIssueDetailsBuilder<'a> { SharedArrayBufferIssueDetailsBuilder::default() }
    pub fn sourceCodeLocation(&self) -> &SourceCodeLocation<'a> { &self.sourceCodeLocation }
    pub fn isWarning(&self) -> bool { self.isWarning }
    pub fn type_(&self) -> &SharedArrayBufferIssueType { &self.type_ }
}

#[derive(Default)]
pub struct SharedArrayBufferIssueDetailsBuilder<'a> {
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
    isWarning: Option<bool>,
    type_: Option<SharedArrayBufferIssueType>,
}

impl<'a> SharedArrayBufferIssueDetailsBuilder<'a> {
    pub fn sourceCodeLocation(mut self, sourceCodeLocation: SourceCodeLocation<'a>) -> Self { self.sourceCodeLocation = Some(sourceCodeLocation); self }
    pub fn isWarning(mut self, isWarning: bool) -> Self { self.isWarning = Some(isWarning); self }
    pub fn type_(mut self, type_: SharedArrayBufferIssueType) -> Self { self.type_ = Some(type_); self }
    pub fn build(self) -> SharedArrayBufferIssueDetails<'a> {
        SharedArrayBufferIssueDetails {
            sourceCodeLocation: self.sourceCodeLocation.unwrap_or_default(),
            isWarning: self.isWarning.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
        }
    }
}

/// Details for a CORS related issue, e.g. a warning or error related to
/// CORS RFC1918 enforcement.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CorsIssueDetails<'a> {
    corsErrorStatus: crate::network::CorsErrorStatus<'a>,
    isWarning: bool,
    request: AffectedRequest<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<SourceCodeLocation<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiatorOrigin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resourceIPAddressSpace: Option<crate::network::IPAddressSpace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clientSecurityState: Option<crate::network::ClientSecurityState>,
}

impl<'a> CorsIssueDetails<'a> {
    pub fn builder() -> CorsIssueDetailsBuilder<'a> { CorsIssueDetailsBuilder::default() }
    pub fn corsErrorStatus(&self) -> &crate::network::CorsErrorStatus<'a> { &self.corsErrorStatus }
    pub fn isWarning(&self) -> bool { self.isWarning }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
    pub fn location(&self) -> Option<&SourceCodeLocation<'a>> { self.location.as_ref() }
    pub fn initiatorOrigin(&self) -> Option<&str> { self.initiatorOrigin.as_deref() }
    pub fn resourceIPAddressSpace(&self) -> Option<&crate::network::IPAddressSpace> { self.resourceIPAddressSpace.as_ref() }
    pub fn clientSecurityState(&self) -> Option<&crate::network::ClientSecurityState> { self.clientSecurityState.as_ref() }
}

#[derive(Default)]
pub struct CorsIssueDetailsBuilder<'a> {
    corsErrorStatus: Option<crate::network::CorsErrorStatus<'a>>,
    isWarning: Option<bool>,
    request: Option<AffectedRequest<'a>>,
    location: Option<SourceCodeLocation<'a>>,
    initiatorOrigin: Option<Cow<'a, str>>,
    resourceIPAddressSpace: Option<crate::network::IPAddressSpace>,
    clientSecurityState: Option<crate::network::ClientSecurityState>,
}

impl<'a> CorsIssueDetailsBuilder<'a> {
    pub fn corsErrorStatus(mut self, corsErrorStatus: crate::network::CorsErrorStatus<'a>) -> Self { self.corsErrorStatus = Some(corsErrorStatus); self }
    pub fn isWarning(mut self, isWarning: bool) -> Self { self.isWarning = Some(isWarning); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn location(mut self, location: SourceCodeLocation<'a>) -> Self { self.location = Some(location); self }
    pub fn initiatorOrigin(mut self, initiatorOrigin: impl Into<Cow<'a, str>>) -> Self { self.initiatorOrigin = Some(initiatorOrigin.into()); self }
    pub fn resourceIPAddressSpace(mut self, resourceIPAddressSpace: crate::network::IPAddressSpace) -> Self { self.resourceIPAddressSpace = Some(resourceIPAddressSpace); self }
    pub fn clientSecurityState(mut self, clientSecurityState: crate::network::ClientSecurityState) -> Self { self.clientSecurityState = Some(clientSecurityState); self }
    pub fn build(self) -> CorsIssueDetails<'a> {
        CorsIssueDetails {
            corsErrorStatus: self.corsErrorStatus.unwrap_or_default(),
            isWarning: self.isWarning.unwrap_or_default(),
            request: self.request.unwrap_or_default(),
            location: self.location,
            initiatorOrigin: self.initiatorOrigin,
            resourceIPAddressSpace: self.resourceIPAddressSpace,
            clientSecurityState: self.clientSecurityState,
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
/// Explainer: https://github.com/WICG/attribution-reporting-api

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingIssueDetails<'a> {
    violationType: AttributionReportingIssueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AffectedRequest<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    violatingNodeId: Option<crate::dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invalidParameter: Option<Cow<'a, str>>,
}

impl<'a> AttributionReportingIssueDetails<'a> {
    pub fn builder() -> AttributionReportingIssueDetailsBuilder<'a> { AttributionReportingIssueDetailsBuilder::default() }
    pub fn violationType(&self) -> &AttributionReportingIssueType { &self.violationType }
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
    pub fn violatingNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.violatingNodeId.as_ref() }
    pub fn invalidParameter(&self) -> Option<&str> { self.invalidParameter.as_deref() }
}

#[derive(Default)]
pub struct AttributionReportingIssueDetailsBuilder<'a> {
    violationType: Option<AttributionReportingIssueType>,
    request: Option<AffectedRequest<'a>>,
    violatingNodeId: Option<crate::dom::BackendNodeId>,
    invalidParameter: Option<Cow<'a, str>>,
}

impl<'a> AttributionReportingIssueDetailsBuilder<'a> {
    pub fn violationType(mut self, violationType: AttributionReportingIssueType) -> Self { self.violationType = Some(violationType); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn violatingNodeId(mut self, violatingNodeId: crate::dom::BackendNodeId) -> Self { self.violatingNodeId = Some(violatingNodeId); self }
    pub fn invalidParameter(mut self, invalidParameter: impl Into<Cow<'a, str>>) -> Self { self.invalidParameter = Some(invalidParameter.into()); self }
    pub fn build(self) -> AttributionReportingIssueDetails<'a> {
        AttributionReportingIssueDetails {
            violationType: self.violationType.unwrap_or_default(),
            request: self.request,
            violatingNodeId: self.violatingNodeId,
            invalidParameter: self.invalidParameter,
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
    isLimitedQuirksMode: bool,
    documentNodeId: crate::dom::BackendNodeId,
    url: Cow<'a, str>,
    frameId: crate::page::FrameId<'a>,
    loaderId: crate::network::LoaderId<'a>,
}

impl<'a> QuirksModeIssueDetails<'a> {
    pub fn builder() -> QuirksModeIssueDetailsBuilder<'a> { QuirksModeIssueDetailsBuilder::default() }
    pub fn isLimitedQuirksMode(&self) -> bool { self.isLimitedQuirksMode }
    pub fn documentNodeId(&self) -> &crate::dom::BackendNodeId { &self.documentNodeId }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
    pub fn loaderId(&self) -> &crate::network::LoaderId<'a> { &self.loaderId }
}

#[derive(Default)]
pub struct QuirksModeIssueDetailsBuilder<'a> {
    isLimitedQuirksMode: Option<bool>,
    documentNodeId: Option<crate::dom::BackendNodeId>,
    url: Option<Cow<'a, str>>,
    frameId: Option<crate::page::FrameId<'a>>,
    loaderId: Option<crate::network::LoaderId<'a>>,
}

impl<'a> QuirksModeIssueDetailsBuilder<'a> {
    /// If false, it means the document's mode is "quirks"
    /// instead of "limited-quirks".
    pub fn isLimitedQuirksMode(mut self, isLimitedQuirksMode: bool) -> Self { self.isLimitedQuirksMode = Some(isLimitedQuirksMode); self }
    pub fn documentNodeId(mut self, documentNodeId: crate::dom::BackendNodeId) -> Self { self.documentNodeId = Some(documentNodeId); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn loaderId(mut self, loaderId: crate::network::LoaderId<'a>) -> Self { self.loaderId = Some(loaderId); self }
    pub fn build(self) -> QuirksModeIssueDetails<'a> {
        QuirksModeIssueDetails {
            isLimitedQuirksMode: self.isLimitedQuirksMode.unwrap_or_default(),
            documentNodeId: self.documentNodeId.unwrap_or_default(),
            url: self.url.unwrap_or_default(),
            frameId: self.frameId.unwrap_or_default(),
            loaderId: self.loaderId.unwrap_or_default(),
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
    pub fn builder() -> NavigatorUserAgentIssueDetailsBuilder<'a> { NavigatorUserAgentIssueDetailsBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn location(&self) -> Option<&SourceCodeLocation<'a>> { self.location.as_ref() }
}

#[derive(Default)]
pub struct NavigatorUserAgentIssueDetailsBuilder<'a> {
    url: Option<Cow<'a, str>>,
    location: Option<SourceCodeLocation<'a>>,
}

impl<'a> NavigatorUserAgentIssueDetailsBuilder<'a> {
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn location(mut self, location: SourceCodeLocation<'a>) -> Self { self.location = Some(location); self }
    pub fn build(self) -> NavigatorUserAgentIssueDetails<'a> {
        NavigatorUserAgentIssueDetails {
            url: self.url.unwrap_or_default(),
            location: self.location,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedDictionaryIssueDetails<'a> {
    sharedDictionaryError: SharedDictionaryError,
    request: AffectedRequest<'a>,
}

impl<'a> SharedDictionaryIssueDetails<'a> {
    pub fn builder() -> SharedDictionaryIssueDetailsBuilder<'a> { SharedDictionaryIssueDetailsBuilder::default() }
    pub fn sharedDictionaryError(&self) -> &SharedDictionaryError { &self.sharedDictionaryError }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
}

#[derive(Default)]
pub struct SharedDictionaryIssueDetailsBuilder<'a> {
    sharedDictionaryError: Option<SharedDictionaryError>,
    request: Option<AffectedRequest<'a>>,
}

impl<'a> SharedDictionaryIssueDetailsBuilder<'a> {
    pub fn sharedDictionaryError(mut self, sharedDictionaryError: SharedDictionaryError) -> Self { self.sharedDictionaryError = Some(sharedDictionaryError); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn build(self) -> SharedDictionaryIssueDetails<'a> {
        SharedDictionaryIssueDetails {
            sharedDictionaryError: self.sharedDictionaryError.unwrap_or_default(),
            request: self.request.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SRIMessageSignatureIssueDetails<'a> {
    error: SRIMessageSignatureError,
    signatureBase: Cow<'a, str>,
    integrityAssertions: Vec<Cow<'a, str>>,
    request: AffectedRequest<'a>,
}

impl<'a> SRIMessageSignatureIssueDetails<'a> {
    pub fn builder() -> SRIMessageSignatureIssueDetailsBuilder<'a> { SRIMessageSignatureIssueDetailsBuilder::default() }
    pub fn error(&self) -> &SRIMessageSignatureError { &self.error }
    pub fn signatureBase(&self) -> &str { self.signatureBase.as_ref() }
    pub fn integrityAssertions(&self) -> &[Cow<'a, str>] { &self.integrityAssertions }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
}

#[derive(Default)]
pub struct SRIMessageSignatureIssueDetailsBuilder<'a> {
    error: Option<SRIMessageSignatureError>,
    signatureBase: Option<Cow<'a, str>>,
    integrityAssertions: Option<Vec<Cow<'a, str>>>,
    request: Option<AffectedRequest<'a>>,
}

impl<'a> SRIMessageSignatureIssueDetailsBuilder<'a> {
    pub fn error(mut self, error: SRIMessageSignatureError) -> Self { self.error = Some(error); self }
    pub fn signatureBase(mut self, signatureBase: impl Into<Cow<'a, str>>) -> Self { self.signatureBase = Some(signatureBase.into()); self }
    pub fn integrityAssertions(mut self, integrityAssertions: Vec<Cow<'a, str>>) -> Self { self.integrityAssertions = Some(integrityAssertions); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn build(self) -> SRIMessageSignatureIssueDetails<'a> {
        SRIMessageSignatureIssueDetails {
            error: self.error.unwrap_or_default(),
            signatureBase: self.signatureBase.unwrap_or_default(),
            integrityAssertions: self.integrityAssertions.unwrap_or_default(),
            request: self.request.unwrap_or_default(),
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
    pub fn builder() -> UnencodedDigestIssueDetailsBuilder<'a> { UnencodedDigestIssueDetailsBuilder::default() }
    pub fn error(&self) -> &UnencodedDigestError { &self.error }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
}

#[derive(Default)]
pub struct UnencodedDigestIssueDetailsBuilder<'a> {
    error: Option<UnencodedDigestError>,
    request: Option<AffectedRequest<'a>>,
}

impl<'a> UnencodedDigestIssueDetailsBuilder<'a> {
    pub fn error(mut self, error: UnencodedDigestError) -> Self { self.error = Some(error); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn build(self) -> UnencodedDigestIssueDetails<'a> {
        UnencodedDigestIssueDetails {
            error: self.error.unwrap_or_default(),
            request: self.request.unwrap_or_default(),
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
    pub fn builder() -> ConnectionAllowlistIssueDetailsBuilder<'a> { ConnectionAllowlistIssueDetailsBuilder::default() }
    pub fn error(&self) -> &ConnectionAllowlistError { &self.error }
    pub fn request(&self) -> &AffectedRequest<'a> { &self.request }
}

#[derive(Default)]
pub struct ConnectionAllowlistIssueDetailsBuilder<'a> {
    error: Option<ConnectionAllowlistError>,
    request: Option<AffectedRequest<'a>>,
}

impl<'a> ConnectionAllowlistIssueDetailsBuilder<'a> {
    pub fn error(mut self, error: ConnectionAllowlistError) -> Self { self.error = Some(error); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn build(self) -> ConnectionAllowlistIssueDetails<'a> {
        ConnectionAllowlistIssueDetails {
            error: self.error.unwrap_or_default(),
            request: self.request.unwrap_or_default(),
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
    errorType: GenericIssueErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    violatingNodeId: Option<crate::dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    violatingNodeAttribute: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AffectedRequest<'a>>,
}

impl<'a> GenericIssueDetails<'a> {
    pub fn builder() -> GenericIssueDetailsBuilder<'a> { GenericIssueDetailsBuilder::default() }
    pub fn errorType(&self) -> &GenericIssueErrorType { &self.errorType }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
    pub fn violatingNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.violatingNodeId.as_ref() }
    pub fn violatingNodeAttribute(&self) -> Option<&str> { self.violatingNodeAttribute.as_deref() }
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
}

#[derive(Default)]
pub struct GenericIssueDetailsBuilder<'a> {
    errorType: Option<GenericIssueErrorType>,
    frameId: Option<crate::page::FrameId<'a>>,
    violatingNodeId: Option<crate::dom::BackendNodeId>,
    violatingNodeAttribute: Option<Cow<'a, str>>,
    request: Option<AffectedRequest<'a>>,
}

impl<'a> GenericIssueDetailsBuilder<'a> {
    /// Issues with the same errorType are aggregated in the frontend.
    pub fn errorType(mut self, errorType: GenericIssueErrorType) -> Self { self.errorType = Some(errorType); self }
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn violatingNodeId(mut self, violatingNodeId: crate::dom::BackendNodeId) -> Self { self.violatingNodeId = Some(violatingNodeId); self }
    pub fn violatingNodeAttribute(mut self, violatingNodeAttribute: impl Into<Cow<'a, str>>) -> Self { self.violatingNodeAttribute = Some(violatingNodeAttribute.into()); self }
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    pub fn build(self) -> GenericIssueDetails<'a> {
        GenericIssueDetails {
            errorType: self.errorType.unwrap_or_default(),
            frameId: self.frameId,
            violatingNodeId: self.violatingNodeId,
            violatingNodeAttribute: self.violatingNodeAttribute,
            request: self.request,
        }
    }
}

/// This issue tracks information needed to print a deprecation message.
/// https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeprecationIssueDetails<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    affectedFrame: Option<AffectedFrame<'a>>,
    sourceCodeLocation: SourceCodeLocation<'a>,
    /// One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
}

impl<'a> DeprecationIssueDetails<'a> {
    pub fn builder() -> DeprecationIssueDetailsBuilder<'a> { DeprecationIssueDetailsBuilder::default() }
    pub fn affectedFrame(&self) -> Option<&AffectedFrame<'a>> { self.affectedFrame.as_ref() }
    pub fn sourceCodeLocation(&self) -> &SourceCodeLocation<'a> { &self.sourceCodeLocation }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}

#[derive(Default)]
pub struct DeprecationIssueDetailsBuilder<'a> {
    affectedFrame: Option<AffectedFrame<'a>>,
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
    type_: Option<Cow<'a, str>>,
}

impl<'a> DeprecationIssueDetailsBuilder<'a> {
    pub fn affectedFrame(mut self, affectedFrame: AffectedFrame<'a>) -> Self { self.affectedFrame = Some(affectedFrame); self }
    pub fn sourceCodeLocation(mut self, sourceCodeLocation: SourceCodeLocation<'a>) -> Self { self.sourceCodeLocation = Some(sourceCodeLocation); self }
    /// One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    pub fn build(self) -> DeprecationIssueDetails<'a> {
        DeprecationIssueDetails {
            affectedFrame: self.affectedFrame,
            sourceCodeLocation: self.sourceCodeLocation.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
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
    trackingSites: Vec<Cow<'a, str>>,
}

impl<'a> BounceTrackingIssueDetails<'a> {
    pub fn builder() -> BounceTrackingIssueDetailsBuilder<'a> { BounceTrackingIssueDetailsBuilder::default() }
    pub fn trackingSites(&self) -> &[Cow<'a, str>] { &self.trackingSites }
}

#[derive(Default)]
pub struct BounceTrackingIssueDetailsBuilder<'a> {
    trackingSites: Option<Vec<Cow<'a, str>>>,
}

impl<'a> BounceTrackingIssueDetailsBuilder<'a> {
    pub fn trackingSites(mut self, trackingSites: Vec<Cow<'a, str>>) -> Self { self.trackingSites = Some(trackingSites); self }
    pub fn build(self) -> BounceTrackingIssueDetails<'a> {
        BounceTrackingIssueDetails {
            trackingSites: self.trackingSites.unwrap_or_default(),
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
    allowedSites: Vec<Cow<'a, str>>,
    optOutPercentage: f64,
    isOptOutTopLevel: bool,
    operation: CookieOperation,
}

impl<'a> CookieDeprecationMetadataIssueDetails<'a> {
    pub fn builder() -> CookieDeprecationMetadataIssueDetailsBuilder<'a> { CookieDeprecationMetadataIssueDetailsBuilder::default() }
    pub fn allowedSites(&self) -> &[Cow<'a, str>] { &self.allowedSites }
    pub fn optOutPercentage(&self) -> f64 { self.optOutPercentage }
    pub fn isOptOutTopLevel(&self) -> bool { self.isOptOutTopLevel }
    pub fn operation(&self) -> &CookieOperation { &self.operation }
}

#[derive(Default)]
pub struct CookieDeprecationMetadataIssueDetailsBuilder<'a> {
    allowedSites: Option<Vec<Cow<'a, str>>>,
    optOutPercentage: Option<f64>,
    isOptOutTopLevel: Option<bool>,
    operation: Option<CookieOperation>,
}

impl<'a> CookieDeprecationMetadataIssueDetailsBuilder<'a> {
    pub fn allowedSites(mut self, allowedSites: Vec<Cow<'a, str>>) -> Self { self.allowedSites = Some(allowedSites); self }
    pub fn optOutPercentage(mut self, optOutPercentage: f64) -> Self { self.optOutPercentage = Some(optOutPercentage); self }
    pub fn isOptOutTopLevel(mut self, isOptOutTopLevel: bool) -> Self { self.isOptOutTopLevel = Some(isOptOutTopLevel); self }
    pub fn operation(mut self, operation: CookieOperation) -> Self { self.operation = Some(operation); self }
    pub fn build(self) -> CookieDeprecationMetadataIssueDetails<'a> {
        CookieDeprecationMetadataIssueDetails {
            allowedSites: self.allowedSites.unwrap_or_default(),
            optOutPercentage: self.optOutPercentage.unwrap_or_default(),
            isOptOutTopLevel: self.isOptOutTopLevel.unwrap_or_default(),
            operation: self.operation.unwrap_or_default(),
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
    federatedAuthRequestIssueReason: FederatedAuthRequestIssueReason,
}

impl FederatedAuthRequestIssueDetails {
    pub fn builder() -> FederatedAuthRequestIssueDetailsBuilder { FederatedAuthRequestIssueDetailsBuilder::default() }
    pub fn federatedAuthRequestIssueReason(&self) -> &FederatedAuthRequestIssueReason { &self.federatedAuthRequestIssueReason }
}

#[derive(Default)]
pub struct FederatedAuthRequestIssueDetailsBuilder {
    federatedAuthRequestIssueReason: Option<FederatedAuthRequestIssueReason>,
}

impl FederatedAuthRequestIssueDetailsBuilder {
    pub fn federatedAuthRequestIssueReason(mut self, federatedAuthRequestIssueReason: FederatedAuthRequestIssueReason) -> Self { self.federatedAuthRequestIssueReason = Some(federatedAuthRequestIssueReason); self }
    pub fn build(self) -> FederatedAuthRequestIssueDetails {
        FederatedAuthRequestIssueDetails {
            federatedAuthRequestIssueReason: self.federatedAuthRequestIssueReason.unwrap_or_default(),
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
    federatedAuthUserInfoRequestIssueReason: FederatedAuthUserInfoRequestIssueReason,
}

impl FederatedAuthUserInfoRequestIssueDetails {
    pub fn builder() -> FederatedAuthUserInfoRequestIssueDetailsBuilder { FederatedAuthUserInfoRequestIssueDetailsBuilder::default() }
    pub fn federatedAuthUserInfoRequestIssueReason(&self) -> &FederatedAuthUserInfoRequestIssueReason { &self.federatedAuthUserInfoRequestIssueReason }
}

#[derive(Default)]
pub struct FederatedAuthUserInfoRequestIssueDetailsBuilder {
    federatedAuthUserInfoRequestIssueReason: Option<FederatedAuthUserInfoRequestIssueReason>,
}

impl FederatedAuthUserInfoRequestIssueDetailsBuilder {
    pub fn federatedAuthUserInfoRequestIssueReason(mut self, federatedAuthUserInfoRequestIssueReason: FederatedAuthUserInfoRequestIssueReason) -> Self { self.federatedAuthUserInfoRequestIssueReason = Some(federatedAuthUserInfoRequestIssueReason); self }
    pub fn build(self) -> FederatedAuthUserInfoRequestIssueDetails {
        FederatedAuthUserInfoRequestIssueDetails {
            federatedAuthUserInfoRequestIssueReason: self.federatedAuthUserInfoRequestIssueReason.unwrap_or_default(),
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
    sourceCodeLocation: SourceCodeLocation<'a>,
    clientHintIssueReason: ClientHintIssueReason,
}

impl<'a> ClientHintIssueDetails<'a> {
    pub fn builder() -> ClientHintIssueDetailsBuilder<'a> { ClientHintIssueDetailsBuilder::default() }
    pub fn sourceCodeLocation(&self) -> &SourceCodeLocation<'a> { &self.sourceCodeLocation }
    pub fn clientHintIssueReason(&self) -> &ClientHintIssueReason { &self.clientHintIssueReason }
}

#[derive(Default)]
pub struct ClientHintIssueDetailsBuilder<'a> {
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
    clientHintIssueReason: Option<ClientHintIssueReason>,
}

impl<'a> ClientHintIssueDetailsBuilder<'a> {
    pub fn sourceCodeLocation(mut self, sourceCodeLocation: SourceCodeLocation<'a>) -> Self { self.sourceCodeLocation = Some(sourceCodeLocation); self }
    pub fn clientHintIssueReason(mut self, clientHintIssueReason: ClientHintIssueReason) -> Self { self.clientHintIssueReason = Some(clientHintIssueReason); self }
    pub fn build(self) -> ClientHintIssueDetails<'a> {
        ClientHintIssueDetails {
            sourceCodeLocation: self.sourceCodeLocation.unwrap_or_default(),
            clientHintIssueReason: self.clientHintIssueReason.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FailedRequestInfo<'a> {
    /// The URL that failed to load.
    url: Cow<'a, str>,
    /// The failure message for the failed request.
    failureMessage: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requestId: Option<crate::network::RequestId<'a>>,
}

impl<'a> FailedRequestInfo<'a> {
    pub fn builder() -> FailedRequestInfoBuilder<'a> { FailedRequestInfoBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn failureMessage(&self) -> &str { self.failureMessage.as_ref() }
    pub fn requestId(&self) -> Option<&crate::network::RequestId<'a>> { self.requestId.as_ref() }
}

#[derive(Default)]
pub struct FailedRequestInfoBuilder<'a> {
    url: Option<Cow<'a, str>>,
    failureMessage: Option<Cow<'a, str>>,
    requestId: Option<crate::network::RequestId<'a>>,
}

impl<'a> FailedRequestInfoBuilder<'a> {
    /// The URL that failed to load.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// The failure message for the failed request.
    pub fn failureMessage(mut self, failureMessage: impl Into<Cow<'a, str>>) -> Self { self.failureMessage = Some(failureMessage.into()); self }
    pub fn requestId(mut self, requestId: crate::network::RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    pub fn build(self) -> FailedRequestInfo<'a> {
        FailedRequestInfo {
            url: self.url.unwrap_or_default(),
            failureMessage: self.failureMessage.unwrap_or_default(),
            requestId: self.requestId,
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
    partitioningBlobURLInfo: PartitioningBlobURLInfo,
}

impl<'a> PartitioningBlobURLIssueDetails<'a> {
    pub fn builder() -> PartitioningBlobURLIssueDetailsBuilder<'a> { PartitioningBlobURLIssueDetailsBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn partitioningBlobURLInfo(&self) -> &PartitioningBlobURLInfo { &self.partitioningBlobURLInfo }
}

#[derive(Default)]
pub struct PartitioningBlobURLIssueDetailsBuilder<'a> {
    url: Option<Cow<'a, str>>,
    partitioningBlobURLInfo: Option<PartitioningBlobURLInfo>,
}

impl<'a> PartitioningBlobURLIssueDetailsBuilder<'a> {
    /// The BlobURL that failed to load.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Additional information about the Partitioning Blob URL issue.
    pub fn partitioningBlobURLInfo(mut self, partitioningBlobURLInfo: PartitioningBlobURLInfo) -> Self { self.partitioningBlobURLInfo = Some(partitioningBlobURLInfo); self }
    pub fn build(self) -> PartitioningBlobURLIssueDetails<'a> {
        PartitioningBlobURLIssueDetails {
            url: self.url.unwrap_or_default(),
            partitioningBlobURLInfo: self.partitioningBlobURLInfo.unwrap_or_default(),
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
    nodeId: crate::dom::BackendNodeId,
    elementAccessibilityIssueReason: ElementAccessibilityIssueReason,
    hasDisallowedAttributes: bool,
}

impl ElementAccessibilityIssueDetails {
    pub fn builder() -> ElementAccessibilityIssueDetailsBuilder { ElementAccessibilityIssueDetailsBuilder::default() }
    pub fn nodeId(&self) -> &crate::dom::BackendNodeId { &self.nodeId }
    pub fn elementAccessibilityIssueReason(&self) -> &ElementAccessibilityIssueReason { &self.elementAccessibilityIssueReason }
    pub fn hasDisallowedAttributes(&self) -> bool { self.hasDisallowedAttributes }
}

#[derive(Default)]
pub struct ElementAccessibilityIssueDetailsBuilder {
    nodeId: Option<crate::dom::BackendNodeId>,
    elementAccessibilityIssueReason: Option<ElementAccessibilityIssueReason>,
    hasDisallowedAttributes: Option<bool>,
}

impl ElementAccessibilityIssueDetailsBuilder {
    pub fn nodeId(mut self, nodeId: crate::dom::BackendNodeId) -> Self { self.nodeId = Some(nodeId); self }
    pub fn elementAccessibilityIssueReason(mut self, elementAccessibilityIssueReason: ElementAccessibilityIssueReason) -> Self { self.elementAccessibilityIssueReason = Some(elementAccessibilityIssueReason); self }
    pub fn hasDisallowedAttributes(mut self, hasDisallowedAttributes: bool) -> Self { self.hasDisallowedAttributes = Some(hasDisallowedAttributes); self }
    pub fn build(self) -> ElementAccessibilityIssueDetails {
        ElementAccessibilityIssueDetails {
            nodeId: self.nodeId.unwrap_or_default(),
            elementAccessibilityIssueReason: self.elementAccessibilityIssueReason.unwrap_or_default(),
            hasDisallowedAttributes: self.hasDisallowedAttributes.unwrap_or_default(),
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
    sourceCodeLocation: SourceCodeLocation<'a>,
    /// Reason why the stylesheet couldn't be loaded.
    styleSheetLoadingIssueReason: StyleSheetLoadingIssueReason,
    /// Contains additional info when the failure was due to a request.
    #[serde(skip_serializing_if = "Option::is_none")]
    failedRequestInfo: Option<FailedRequestInfo<'a>>,
}

impl<'a> StylesheetLoadingIssueDetails<'a> {
    pub fn builder() -> StylesheetLoadingIssueDetailsBuilder<'a> { StylesheetLoadingIssueDetailsBuilder::default() }
    pub fn sourceCodeLocation(&self) -> &SourceCodeLocation<'a> { &self.sourceCodeLocation }
    pub fn styleSheetLoadingIssueReason(&self) -> &StyleSheetLoadingIssueReason { &self.styleSheetLoadingIssueReason }
    pub fn failedRequestInfo(&self) -> Option<&FailedRequestInfo<'a>> { self.failedRequestInfo.as_ref() }
}

#[derive(Default)]
pub struct StylesheetLoadingIssueDetailsBuilder<'a> {
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
    styleSheetLoadingIssueReason: Option<StyleSheetLoadingIssueReason>,
    failedRequestInfo: Option<FailedRequestInfo<'a>>,
}

impl<'a> StylesheetLoadingIssueDetailsBuilder<'a> {
    /// Source code position that referenced the failing stylesheet.
    pub fn sourceCodeLocation(mut self, sourceCodeLocation: SourceCodeLocation<'a>) -> Self { self.sourceCodeLocation = Some(sourceCodeLocation); self }
    /// Reason why the stylesheet couldn't be loaded.
    pub fn styleSheetLoadingIssueReason(mut self, styleSheetLoadingIssueReason: StyleSheetLoadingIssueReason) -> Self { self.styleSheetLoadingIssueReason = Some(styleSheetLoadingIssueReason); self }
    /// Contains additional info when the failure was due to a request.
    pub fn failedRequestInfo(mut self, failedRequestInfo: FailedRequestInfo<'a>) -> Self { self.failedRequestInfo = Some(failedRequestInfo); self }
    pub fn build(self) -> StylesheetLoadingIssueDetails<'a> {
        StylesheetLoadingIssueDetails {
            sourceCodeLocation: self.sourceCodeLocation.unwrap_or_default(),
            styleSheetLoadingIssueReason: self.styleSheetLoadingIssueReason.unwrap_or_default(),
            failedRequestInfo: self.failedRequestInfo,
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
    sourceCodeLocation: SourceCodeLocation<'a>,
    /// Reason why the property rule was discarded.
    propertyRuleIssueReason: PropertyRuleIssueReason,
    /// The value of the property rule property that failed to parse
    #[serde(skip_serializing_if = "Option::is_none")]
    propertyValue: Option<Cow<'a, str>>,
}

impl<'a> PropertyRuleIssueDetails<'a> {
    pub fn builder() -> PropertyRuleIssueDetailsBuilder<'a> { PropertyRuleIssueDetailsBuilder::default() }
    pub fn sourceCodeLocation(&self) -> &SourceCodeLocation<'a> { &self.sourceCodeLocation }
    pub fn propertyRuleIssueReason(&self) -> &PropertyRuleIssueReason { &self.propertyRuleIssueReason }
    pub fn propertyValue(&self) -> Option<&str> { self.propertyValue.as_deref() }
}

#[derive(Default)]
pub struct PropertyRuleIssueDetailsBuilder<'a> {
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
    propertyRuleIssueReason: Option<PropertyRuleIssueReason>,
    propertyValue: Option<Cow<'a, str>>,
}

impl<'a> PropertyRuleIssueDetailsBuilder<'a> {
    /// Source code position of the property rule.
    pub fn sourceCodeLocation(mut self, sourceCodeLocation: SourceCodeLocation<'a>) -> Self { self.sourceCodeLocation = Some(sourceCodeLocation); self }
    /// Reason why the property rule was discarded.
    pub fn propertyRuleIssueReason(mut self, propertyRuleIssueReason: PropertyRuleIssueReason) -> Self { self.propertyRuleIssueReason = Some(propertyRuleIssueReason); self }
    /// The value of the property rule property that failed to parse
    pub fn propertyValue(mut self, propertyValue: impl Into<Cow<'a, str>>) -> Self { self.propertyValue = Some(propertyValue.into()); self }
    pub fn build(self) -> PropertyRuleIssueDetails<'a> {
        PropertyRuleIssueDetails {
            sourceCodeLocation: self.sourceCodeLocation.unwrap_or_default(),
            propertyRuleIssueReason: self.propertyRuleIssueReason.unwrap_or_default(),
            propertyValue: self.propertyValue,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
}

impl<'a> UserReidentificationIssueDetails<'a> {
    pub fn builder() -> UserReidentificationIssueDetailsBuilder<'a> { UserReidentificationIssueDetailsBuilder::default() }
    pub fn type_(&self) -> &UserReidentificationIssueType { &self.type_ }
    pub fn request(&self) -> Option<&AffectedRequest<'a>> { self.request.as_ref() }
    pub fn sourceCodeLocation(&self) -> Option<&SourceCodeLocation<'a>> { self.sourceCodeLocation.as_ref() }
}

#[derive(Default)]
pub struct UserReidentificationIssueDetailsBuilder<'a> {
    type_: Option<UserReidentificationIssueType>,
    request: Option<AffectedRequest<'a>>,
    sourceCodeLocation: Option<SourceCodeLocation<'a>>,
}

impl<'a> UserReidentificationIssueDetailsBuilder<'a> {
    pub fn type_(mut self, type_: UserReidentificationIssueType) -> Self { self.type_ = Some(type_); self }
    /// Applies to BlockedFrameNavigation and BlockedSubresource issue types.
    pub fn request(mut self, request: AffectedRequest<'a>) -> Self { self.request = Some(request); self }
    /// Applies to NoisedCanvasReadback issue type.
    pub fn sourceCodeLocation(mut self, sourceCodeLocation: SourceCodeLocation<'a>) -> Self { self.sourceCodeLocation = Some(sourceCodeLocation); self }
    pub fn build(self) -> UserReidentificationIssueDetails<'a> {
        UserReidentificationIssueDetails {
            type_: self.type_.unwrap_or_default(),
            request: self.request,
            sourceCodeLocation: self.sourceCodeLocation,
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

/// This issue warns about improper usage of the <permission> element.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionElementIssueDetails<'a> {
    issueType: PermissionElementIssueType,
    /// The value of the type attribute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    type_: Option<Cow<'a, str>>,
    /// The node ID of the <permission> element.
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeId: Option<crate::dom::BackendNodeId>,
    /// True if the issue is a warning, false if it is an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    isWarning: Option<bool>,
    /// Fields for message construction:
    /// Used for messages that reference a specific permission name
    #[serde(skip_serializing_if = "Option::is_none")]
    permissionName: Option<Cow<'a, str>>,
    /// Used for messages about occlusion
    #[serde(skip_serializing_if = "Option::is_none")]
    occluderNodeInfo: Option<Cow<'a, str>>,
    /// Used for messages about occluder's parent
    #[serde(skip_serializing_if = "Option::is_none")]
    occluderParentNodeInfo: Option<Cow<'a, str>>,
    /// Used for messages about activation disabled reason
    #[serde(skip_serializing_if = "Option::is_none")]
    disableReason: Option<Cow<'a, str>>,
}

impl<'a> PermissionElementIssueDetails<'a> {
    pub fn builder() -> PermissionElementIssueDetailsBuilder<'a> { PermissionElementIssueDetailsBuilder::default() }
    pub fn issueType(&self) -> &PermissionElementIssueType { &self.issueType }
    pub fn type_(&self) -> Option<&str> { self.type_.as_deref() }
    pub fn nodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.nodeId.as_ref() }
    pub fn isWarning(&self) -> Option<bool> { self.isWarning }
    pub fn permissionName(&self) -> Option<&str> { self.permissionName.as_deref() }
    pub fn occluderNodeInfo(&self) -> Option<&str> { self.occluderNodeInfo.as_deref() }
    pub fn occluderParentNodeInfo(&self) -> Option<&str> { self.occluderParentNodeInfo.as_deref() }
    pub fn disableReason(&self) -> Option<&str> { self.disableReason.as_deref() }
}

#[derive(Default)]
pub struct PermissionElementIssueDetailsBuilder<'a> {
    issueType: Option<PermissionElementIssueType>,
    type_: Option<Cow<'a, str>>,
    nodeId: Option<crate::dom::BackendNodeId>,
    isWarning: Option<bool>,
    permissionName: Option<Cow<'a, str>>,
    occluderNodeInfo: Option<Cow<'a, str>>,
    occluderParentNodeInfo: Option<Cow<'a, str>>,
    disableReason: Option<Cow<'a, str>>,
}

impl<'a> PermissionElementIssueDetailsBuilder<'a> {
    pub fn issueType(mut self, issueType: PermissionElementIssueType) -> Self { self.issueType = Some(issueType); self }
    /// The value of the type attribute.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    /// The node ID of the <permission> element.
    pub fn nodeId(mut self, nodeId: crate::dom::BackendNodeId) -> Self { self.nodeId = Some(nodeId); self }
    /// True if the issue is a warning, false if it is an error.
    pub fn isWarning(mut self, isWarning: bool) -> Self { self.isWarning = Some(isWarning); self }
    /// Fields for message construction:
    /// Used for messages that reference a specific permission name
    pub fn permissionName(mut self, permissionName: impl Into<Cow<'a, str>>) -> Self { self.permissionName = Some(permissionName.into()); self }
    /// Used for messages about occlusion
    pub fn occluderNodeInfo(mut self, occluderNodeInfo: impl Into<Cow<'a, str>>) -> Self { self.occluderNodeInfo = Some(occluderNodeInfo.into()); self }
    /// Used for messages about occluder's parent
    pub fn occluderParentNodeInfo(mut self, occluderParentNodeInfo: impl Into<Cow<'a, str>>) -> Self { self.occluderParentNodeInfo = Some(occluderParentNodeInfo.into()); self }
    /// Used for messages about activation disabled reason
    pub fn disableReason(mut self, disableReason: impl Into<Cow<'a, str>>) -> Self { self.disableReason = Some(disableReason.into()); self }
    pub fn build(self) -> PermissionElementIssueDetails<'a> {
        PermissionElementIssueDetails {
            issueType: self.issueType.unwrap_or_default(),
            type_: self.type_,
            nodeId: self.nodeId,
            isWarning: self.isWarning,
            permissionName: self.permissionName,
            occluderNodeInfo: self.occluderNodeInfo,
            occluderParentNodeInfo: self.occluderParentNodeInfo,
            disableReason: self.disableReason,
        }
    }
}

/// The issue warns about blocked calls to privacy sensitive APIs via the
/// Selective Permissions Intervention.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectivePermissionsInterventionIssueDetails<'a> {
    /// Which API was intervened on.
    apiName: Cow<'a, str>,
    /// Why the ad script using the API is considered an ad.
    adAncestry: crate::network::AdAncestry<'a>,
    /// The stack trace at the time of the intervention.
    #[serde(skip_serializing_if = "Option::is_none")]
    stackTrace: Option<crate::runtime::StackTrace>,
}

impl<'a> SelectivePermissionsInterventionIssueDetails<'a> {
    pub fn builder() -> SelectivePermissionsInterventionIssueDetailsBuilder<'a> { SelectivePermissionsInterventionIssueDetailsBuilder::default() }
    pub fn apiName(&self) -> &str { self.apiName.as_ref() }
    pub fn adAncestry(&self) -> &crate::network::AdAncestry<'a> { &self.adAncestry }
    pub fn stackTrace(&self) -> Option<&crate::runtime::StackTrace> { self.stackTrace.as_ref() }
}

#[derive(Default)]
pub struct SelectivePermissionsInterventionIssueDetailsBuilder<'a> {
    apiName: Option<Cow<'a, str>>,
    adAncestry: Option<crate::network::AdAncestry<'a>>,
    stackTrace: Option<crate::runtime::StackTrace>,
}

impl<'a> SelectivePermissionsInterventionIssueDetailsBuilder<'a> {
    /// Which API was intervened on.
    pub fn apiName(mut self, apiName: impl Into<Cow<'a, str>>) -> Self { self.apiName = Some(apiName.into()); self }
    /// Why the ad script using the API is considered an ad.
    pub fn adAncestry(mut self, adAncestry: crate::network::AdAncestry<'a>) -> Self { self.adAncestry = Some(adAncestry); self }
    /// The stack trace at the time of the intervention.
    pub fn stackTrace(mut self, stackTrace: crate::runtime::StackTrace) -> Self { self.stackTrace = Some(stackTrace); self }
    pub fn build(self) -> SelectivePermissionsInterventionIssueDetails<'a> {
        SelectivePermissionsInterventionIssueDetails {
            apiName: self.apiName.unwrap_or_default(),
            adAncestry: self.adAncestry.unwrap_or_default(),
            stackTrace: self.stackTrace,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    cookieIssueDetails: Option<CookieIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mixedContentIssueDetails: Option<MixedContentIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blockedByResponseIssueDetails: Option<BlockedByResponseIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heavyAdIssueDetails: Option<HeavyAdIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contentSecurityPolicyIssueDetails: Option<ContentSecurityPolicyIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharedArrayBufferIssueDetails: Option<SharedArrayBufferIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    corsIssueDetails: Option<CorsIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributionReportingIssueDetails: Option<AttributionReportingIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quirksModeIssueDetails: Option<QuirksModeIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partitioningBlobURLIssueDetails: Option<PartitioningBlobURLIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    navigatorUserAgentIssueDetails: Option<NavigatorUserAgentIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    genericIssueDetails: Option<GenericIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecationIssueDetails: Option<DeprecationIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clientHintIssueDetails: Option<ClientHintIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    federatedAuthRequestIssueDetails: Option<FederatedAuthRequestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bounceTrackingIssueDetails: Option<BounceTrackingIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookieDeprecationMetadataIssueDetails: Option<CookieDeprecationMetadataIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stylesheetLoadingIssueDetails: Option<StylesheetLoadingIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propertyRuleIssueDetails: Option<PropertyRuleIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    federatedAuthUserInfoRequestIssueDetails: Option<FederatedAuthUserInfoRequestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharedDictionaryIssueDetails: Option<SharedDictionaryIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elementAccessibilityIssueDetails: Option<ElementAccessibilityIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sriMessageSignatureIssueDetails: Option<SRIMessageSignatureIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unencodedDigestIssueDetails: Option<UnencodedDigestIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connectionAllowlistIssueDetails: Option<ConnectionAllowlistIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    userReidentificationIssueDetails: Option<UserReidentificationIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissionElementIssueDetails: Option<PermissionElementIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performanceIssueDetails: Option<PerformanceIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selectivePermissionsInterventionIssueDetails: Option<SelectivePermissionsInterventionIssueDetails<'a>>,
}

impl<'a> InspectorIssueDetails<'a> {
    pub fn builder() -> InspectorIssueDetailsBuilder<'a> { InspectorIssueDetailsBuilder::default() }
    pub fn cookieIssueDetails(&self) -> Option<&CookieIssueDetails<'a>> { self.cookieIssueDetails.as_ref() }
    pub fn mixedContentIssueDetails(&self) -> Option<&MixedContentIssueDetails<'a>> { self.mixedContentIssueDetails.as_ref() }
    pub fn blockedByResponseIssueDetails(&self) -> Option<&BlockedByResponseIssueDetails<'a>> { self.blockedByResponseIssueDetails.as_ref() }
    pub fn heavyAdIssueDetails(&self) -> Option<&HeavyAdIssueDetails<'a>> { self.heavyAdIssueDetails.as_ref() }
    pub fn contentSecurityPolicyIssueDetails(&self) -> Option<&ContentSecurityPolicyIssueDetails<'a>> { self.contentSecurityPolicyIssueDetails.as_ref() }
    pub fn sharedArrayBufferIssueDetails(&self) -> Option<&SharedArrayBufferIssueDetails<'a>> { self.sharedArrayBufferIssueDetails.as_ref() }
    pub fn corsIssueDetails(&self) -> Option<&CorsIssueDetails<'a>> { self.corsIssueDetails.as_ref() }
    pub fn attributionReportingIssueDetails(&self) -> Option<&AttributionReportingIssueDetails<'a>> { self.attributionReportingIssueDetails.as_ref() }
    pub fn quirksModeIssueDetails(&self) -> Option<&QuirksModeIssueDetails<'a>> { self.quirksModeIssueDetails.as_ref() }
    pub fn partitioningBlobURLIssueDetails(&self) -> Option<&PartitioningBlobURLIssueDetails<'a>> { self.partitioningBlobURLIssueDetails.as_ref() }
    pub fn navigatorUserAgentIssueDetails(&self) -> Option<&NavigatorUserAgentIssueDetails<'a>> { self.navigatorUserAgentIssueDetails.as_ref() }
    pub fn genericIssueDetails(&self) -> Option<&GenericIssueDetails<'a>> { self.genericIssueDetails.as_ref() }
    pub fn deprecationIssueDetails(&self) -> Option<&DeprecationIssueDetails<'a>> { self.deprecationIssueDetails.as_ref() }
    pub fn clientHintIssueDetails(&self) -> Option<&ClientHintIssueDetails<'a>> { self.clientHintIssueDetails.as_ref() }
    pub fn federatedAuthRequestIssueDetails(&self) -> Option<&FederatedAuthRequestIssueDetails> { self.federatedAuthRequestIssueDetails.as_ref() }
    pub fn bounceTrackingIssueDetails(&self) -> Option<&BounceTrackingIssueDetails<'a>> { self.bounceTrackingIssueDetails.as_ref() }
    pub fn cookieDeprecationMetadataIssueDetails(&self) -> Option<&CookieDeprecationMetadataIssueDetails<'a>> { self.cookieDeprecationMetadataIssueDetails.as_ref() }
    pub fn stylesheetLoadingIssueDetails(&self) -> Option<&StylesheetLoadingIssueDetails<'a>> { self.stylesheetLoadingIssueDetails.as_ref() }
    pub fn propertyRuleIssueDetails(&self) -> Option<&PropertyRuleIssueDetails<'a>> { self.propertyRuleIssueDetails.as_ref() }
    pub fn federatedAuthUserInfoRequestIssueDetails(&self) -> Option<&FederatedAuthUserInfoRequestIssueDetails> { self.federatedAuthUserInfoRequestIssueDetails.as_ref() }
    pub fn sharedDictionaryIssueDetails(&self) -> Option<&SharedDictionaryIssueDetails<'a>> { self.sharedDictionaryIssueDetails.as_ref() }
    pub fn elementAccessibilityIssueDetails(&self) -> Option<&ElementAccessibilityIssueDetails> { self.elementAccessibilityIssueDetails.as_ref() }
    pub fn sriMessageSignatureIssueDetails(&self) -> Option<&SRIMessageSignatureIssueDetails<'a>> { self.sriMessageSignatureIssueDetails.as_ref() }
    pub fn unencodedDigestIssueDetails(&self) -> Option<&UnencodedDigestIssueDetails<'a>> { self.unencodedDigestIssueDetails.as_ref() }
    pub fn connectionAllowlistIssueDetails(&self) -> Option<&ConnectionAllowlistIssueDetails<'a>> { self.connectionAllowlistIssueDetails.as_ref() }
    pub fn userReidentificationIssueDetails(&self) -> Option<&UserReidentificationIssueDetails<'a>> { self.userReidentificationIssueDetails.as_ref() }
    pub fn permissionElementIssueDetails(&self) -> Option<&PermissionElementIssueDetails<'a>> { self.permissionElementIssueDetails.as_ref() }
    pub fn performanceIssueDetails(&self) -> Option<&PerformanceIssueDetails<'a>> { self.performanceIssueDetails.as_ref() }
    pub fn selectivePermissionsInterventionIssueDetails(&self) -> Option<&SelectivePermissionsInterventionIssueDetails<'a>> { self.selectivePermissionsInterventionIssueDetails.as_ref() }
}

#[derive(Default)]
pub struct InspectorIssueDetailsBuilder<'a> {
    cookieIssueDetails: Option<CookieIssueDetails<'a>>,
    mixedContentIssueDetails: Option<MixedContentIssueDetails<'a>>,
    blockedByResponseIssueDetails: Option<BlockedByResponseIssueDetails<'a>>,
    heavyAdIssueDetails: Option<HeavyAdIssueDetails<'a>>,
    contentSecurityPolicyIssueDetails: Option<ContentSecurityPolicyIssueDetails<'a>>,
    sharedArrayBufferIssueDetails: Option<SharedArrayBufferIssueDetails<'a>>,
    corsIssueDetails: Option<CorsIssueDetails<'a>>,
    attributionReportingIssueDetails: Option<AttributionReportingIssueDetails<'a>>,
    quirksModeIssueDetails: Option<QuirksModeIssueDetails<'a>>,
    partitioningBlobURLIssueDetails: Option<PartitioningBlobURLIssueDetails<'a>>,
    navigatorUserAgentIssueDetails: Option<NavigatorUserAgentIssueDetails<'a>>,
    genericIssueDetails: Option<GenericIssueDetails<'a>>,
    deprecationIssueDetails: Option<DeprecationIssueDetails<'a>>,
    clientHintIssueDetails: Option<ClientHintIssueDetails<'a>>,
    federatedAuthRequestIssueDetails: Option<FederatedAuthRequestIssueDetails>,
    bounceTrackingIssueDetails: Option<BounceTrackingIssueDetails<'a>>,
    cookieDeprecationMetadataIssueDetails: Option<CookieDeprecationMetadataIssueDetails<'a>>,
    stylesheetLoadingIssueDetails: Option<StylesheetLoadingIssueDetails<'a>>,
    propertyRuleIssueDetails: Option<PropertyRuleIssueDetails<'a>>,
    federatedAuthUserInfoRequestIssueDetails: Option<FederatedAuthUserInfoRequestIssueDetails>,
    sharedDictionaryIssueDetails: Option<SharedDictionaryIssueDetails<'a>>,
    elementAccessibilityIssueDetails: Option<ElementAccessibilityIssueDetails>,
    sriMessageSignatureIssueDetails: Option<SRIMessageSignatureIssueDetails<'a>>,
    unencodedDigestIssueDetails: Option<UnencodedDigestIssueDetails<'a>>,
    connectionAllowlistIssueDetails: Option<ConnectionAllowlistIssueDetails<'a>>,
    userReidentificationIssueDetails: Option<UserReidentificationIssueDetails<'a>>,
    permissionElementIssueDetails: Option<PermissionElementIssueDetails<'a>>,
    performanceIssueDetails: Option<PerformanceIssueDetails<'a>>,
    selectivePermissionsInterventionIssueDetails: Option<SelectivePermissionsInterventionIssueDetails<'a>>,
}

impl<'a> InspectorIssueDetailsBuilder<'a> {
    pub fn cookieIssueDetails(mut self, cookieIssueDetails: CookieIssueDetails<'a>) -> Self { self.cookieIssueDetails = Some(cookieIssueDetails); self }
    pub fn mixedContentIssueDetails(mut self, mixedContentIssueDetails: MixedContentIssueDetails<'a>) -> Self { self.mixedContentIssueDetails = Some(mixedContentIssueDetails); self }
    pub fn blockedByResponseIssueDetails(mut self, blockedByResponseIssueDetails: BlockedByResponseIssueDetails<'a>) -> Self { self.blockedByResponseIssueDetails = Some(blockedByResponseIssueDetails); self }
    pub fn heavyAdIssueDetails(mut self, heavyAdIssueDetails: HeavyAdIssueDetails<'a>) -> Self { self.heavyAdIssueDetails = Some(heavyAdIssueDetails); self }
    pub fn contentSecurityPolicyIssueDetails(mut self, contentSecurityPolicyIssueDetails: ContentSecurityPolicyIssueDetails<'a>) -> Self { self.contentSecurityPolicyIssueDetails = Some(contentSecurityPolicyIssueDetails); self }
    pub fn sharedArrayBufferIssueDetails(mut self, sharedArrayBufferIssueDetails: SharedArrayBufferIssueDetails<'a>) -> Self { self.sharedArrayBufferIssueDetails = Some(sharedArrayBufferIssueDetails); self }
    pub fn corsIssueDetails(mut self, corsIssueDetails: CorsIssueDetails<'a>) -> Self { self.corsIssueDetails = Some(corsIssueDetails); self }
    pub fn attributionReportingIssueDetails(mut self, attributionReportingIssueDetails: AttributionReportingIssueDetails<'a>) -> Self { self.attributionReportingIssueDetails = Some(attributionReportingIssueDetails); self }
    pub fn quirksModeIssueDetails(mut self, quirksModeIssueDetails: QuirksModeIssueDetails<'a>) -> Self { self.quirksModeIssueDetails = Some(quirksModeIssueDetails); self }
    pub fn partitioningBlobURLIssueDetails(mut self, partitioningBlobURLIssueDetails: PartitioningBlobURLIssueDetails<'a>) -> Self { self.partitioningBlobURLIssueDetails = Some(partitioningBlobURLIssueDetails); self }
    pub fn navigatorUserAgentIssueDetails(mut self, navigatorUserAgentIssueDetails: NavigatorUserAgentIssueDetails<'a>) -> Self { self.navigatorUserAgentIssueDetails = Some(navigatorUserAgentIssueDetails); self }
    pub fn genericIssueDetails(mut self, genericIssueDetails: GenericIssueDetails<'a>) -> Self { self.genericIssueDetails = Some(genericIssueDetails); self }
    pub fn deprecationIssueDetails(mut self, deprecationIssueDetails: DeprecationIssueDetails<'a>) -> Self { self.deprecationIssueDetails = Some(deprecationIssueDetails); self }
    pub fn clientHintIssueDetails(mut self, clientHintIssueDetails: ClientHintIssueDetails<'a>) -> Self { self.clientHintIssueDetails = Some(clientHintIssueDetails); self }
    pub fn federatedAuthRequestIssueDetails(mut self, federatedAuthRequestIssueDetails: FederatedAuthRequestIssueDetails) -> Self { self.federatedAuthRequestIssueDetails = Some(federatedAuthRequestIssueDetails); self }
    pub fn bounceTrackingIssueDetails(mut self, bounceTrackingIssueDetails: BounceTrackingIssueDetails<'a>) -> Self { self.bounceTrackingIssueDetails = Some(bounceTrackingIssueDetails); self }
    pub fn cookieDeprecationMetadataIssueDetails(mut self, cookieDeprecationMetadataIssueDetails: CookieDeprecationMetadataIssueDetails<'a>) -> Self { self.cookieDeprecationMetadataIssueDetails = Some(cookieDeprecationMetadataIssueDetails); self }
    pub fn stylesheetLoadingIssueDetails(mut self, stylesheetLoadingIssueDetails: StylesheetLoadingIssueDetails<'a>) -> Self { self.stylesheetLoadingIssueDetails = Some(stylesheetLoadingIssueDetails); self }
    pub fn propertyRuleIssueDetails(mut self, propertyRuleIssueDetails: PropertyRuleIssueDetails<'a>) -> Self { self.propertyRuleIssueDetails = Some(propertyRuleIssueDetails); self }
    pub fn federatedAuthUserInfoRequestIssueDetails(mut self, federatedAuthUserInfoRequestIssueDetails: FederatedAuthUserInfoRequestIssueDetails) -> Self { self.federatedAuthUserInfoRequestIssueDetails = Some(federatedAuthUserInfoRequestIssueDetails); self }
    pub fn sharedDictionaryIssueDetails(mut self, sharedDictionaryIssueDetails: SharedDictionaryIssueDetails<'a>) -> Self { self.sharedDictionaryIssueDetails = Some(sharedDictionaryIssueDetails); self }
    pub fn elementAccessibilityIssueDetails(mut self, elementAccessibilityIssueDetails: ElementAccessibilityIssueDetails) -> Self { self.elementAccessibilityIssueDetails = Some(elementAccessibilityIssueDetails); self }
    pub fn sriMessageSignatureIssueDetails(mut self, sriMessageSignatureIssueDetails: SRIMessageSignatureIssueDetails<'a>) -> Self { self.sriMessageSignatureIssueDetails = Some(sriMessageSignatureIssueDetails); self }
    pub fn unencodedDigestIssueDetails(mut self, unencodedDigestIssueDetails: UnencodedDigestIssueDetails<'a>) -> Self { self.unencodedDigestIssueDetails = Some(unencodedDigestIssueDetails); self }
    pub fn connectionAllowlistIssueDetails(mut self, connectionAllowlistIssueDetails: ConnectionAllowlistIssueDetails<'a>) -> Self { self.connectionAllowlistIssueDetails = Some(connectionAllowlistIssueDetails); self }
    pub fn userReidentificationIssueDetails(mut self, userReidentificationIssueDetails: UserReidentificationIssueDetails<'a>) -> Self { self.userReidentificationIssueDetails = Some(userReidentificationIssueDetails); self }
    pub fn permissionElementIssueDetails(mut self, permissionElementIssueDetails: PermissionElementIssueDetails<'a>) -> Self { self.permissionElementIssueDetails = Some(permissionElementIssueDetails); self }
    pub fn performanceIssueDetails(mut self, performanceIssueDetails: PerformanceIssueDetails<'a>) -> Self { self.performanceIssueDetails = Some(performanceIssueDetails); self }
    pub fn selectivePermissionsInterventionIssueDetails(mut self, selectivePermissionsInterventionIssueDetails: SelectivePermissionsInterventionIssueDetails<'a>) -> Self { self.selectivePermissionsInterventionIssueDetails = Some(selectivePermissionsInterventionIssueDetails); self }
    pub fn build(self) -> InspectorIssueDetails<'a> {
        InspectorIssueDetails {
            cookieIssueDetails: self.cookieIssueDetails,
            mixedContentIssueDetails: self.mixedContentIssueDetails,
            blockedByResponseIssueDetails: self.blockedByResponseIssueDetails,
            heavyAdIssueDetails: self.heavyAdIssueDetails,
            contentSecurityPolicyIssueDetails: self.contentSecurityPolicyIssueDetails,
            sharedArrayBufferIssueDetails: self.sharedArrayBufferIssueDetails,
            corsIssueDetails: self.corsIssueDetails,
            attributionReportingIssueDetails: self.attributionReportingIssueDetails,
            quirksModeIssueDetails: self.quirksModeIssueDetails,
            partitioningBlobURLIssueDetails: self.partitioningBlobURLIssueDetails,
            navigatorUserAgentIssueDetails: self.navigatorUserAgentIssueDetails,
            genericIssueDetails: self.genericIssueDetails,
            deprecationIssueDetails: self.deprecationIssueDetails,
            clientHintIssueDetails: self.clientHintIssueDetails,
            federatedAuthRequestIssueDetails: self.federatedAuthRequestIssueDetails,
            bounceTrackingIssueDetails: self.bounceTrackingIssueDetails,
            cookieDeprecationMetadataIssueDetails: self.cookieDeprecationMetadataIssueDetails,
            stylesheetLoadingIssueDetails: self.stylesheetLoadingIssueDetails,
            propertyRuleIssueDetails: self.propertyRuleIssueDetails,
            federatedAuthUserInfoRequestIssueDetails: self.federatedAuthUserInfoRequestIssueDetails,
            sharedDictionaryIssueDetails: self.sharedDictionaryIssueDetails,
            elementAccessibilityIssueDetails: self.elementAccessibilityIssueDetails,
            sriMessageSignatureIssueDetails: self.sriMessageSignatureIssueDetails,
            unencodedDigestIssueDetails: self.unencodedDigestIssueDetails,
            connectionAllowlistIssueDetails: self.connectionAllowlistIssueDetails,
            userReidentificationIssueDetails: self.userReidentificationIssueDetails,
            permissionElementIssueDetails: self.permissionElementIssueDetails,
            performanceIssueDetails: self.performanceIssueDetails,
            selectivePermissionsInterventionIssueDetails: self.selectivePermissionsInterventionIssueDetails,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    issueId: Option<IssueId<'a>>,
}

impl<'a> InspectorIssue<'a> {
    pub fn builder() -> InspectorIssueBuilder<'a> { InspectorIssueBuilder::default() }
    pub fn code(&self) -> &InspectorIssueCode { &self.code }
    pub fn details(&self) -> &InspectorIssueDetails<'a> { &self.details }
    pub fn issueId(&self) -> Option<&IssueId<'a>> { self.issueId.as_ref() }
}

#[derive(Default)]
pub struct InspectorIssueBuilder<'a> {
    code: Option<InspectorIssueCode>,
    details: Option<InspectorIssueDetails<'a>>,
    issueId: Option<IssueId<'a>>,
}

impl<'a> InspectorIssueBuilder<'a> {
    pub fn code(mut self, code: InspectorIssueCode) -> Self { self.code = Some(code); self }
    pub fn details(mut self, details: InspectorIssueDetails<'a>) -> Self { self.details = Some(details); self }
    /// A unique id for this issue. May be omitted if no other entity (e.g.
    /// exception, CDP message, etc.) is referencing this issue.
    pub fn issueId(mut self, issueId: IssueId<'a>) -> Self { self.issueId = Some(issueId); self }
    pub fn build(self) -> InspectorIssue<'a> {
        InspectorIssue {
            code: self.code.unwrap_or_default(),
            details: self.details.unwrap_or_default(),
            issueId: self.issueId,
        }
    }
}

/// Returns the response body and size if it were re-encoded with the specified settings. Only
/// applies to images.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEncodedResponseParams<'a> {
    /// Identifier of the network request to get content for.
    requestId: crate::network::RequestId<'a>,
    /// The encoding to use.
    encoding: Cow<'a, str>,
    /// The quality of the encoding (0-1). (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<f64>,
    /// Whether to only return the size information (defaults to false).
    #[serde(skip_serializing_if = "Option::is_none")]
    sizeOnly: Option<bool>,
}

impl<'a> GetEncodedResponseParams<'a> {
    pub fn builder() -> GetEncodedResponseParamsBuilder<'a> { GetEncodedResponseParamsBuilder::default() }
    pub fn requestId(&self) -> &crate::network::RequestId<'a> { &self.requestId }
    pub fn encoding(&self) -> &str { self.encoding.as_ref() }
    pub fn quality(&self) -> Option<f64> { self.quality }
    pub fn sizeOnly(&self) -> Option<bool> { self.sizeOnly }
}

#[derive(Default)]
pub struct GetEncodedResponseParamsBuilder<'a> {
    requestId: Option<crate::network::RequestId<'a>>,
    encoding: Option<Cow<'a, str>>,
    quality: Option<f64>,
    sizeOnly: Option<bool>,
}

impl<'a> GetEncodedResponseParamsBuilder<'a> {
    /// Identifier of the network request to get content for.
    pub fn requestId(mut self, requestId: crate::network::RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    /// The encoding to use.
    pub fn encoding(mut self, encoding: impl Into<Cow<'a, str>>) -> Self { self.encoding = Some(encoding.into()); self }
    /// The quality of the encoding (0-1). (defaults to 1)
    pub fn quality(mut self, quality: f64) -> Self { self.quality = Some(quality); self }
    /// Whether to only return the size information (defaults to false).
    pub fn sizeOnly(mut self, sizeOnly: bool) -> Self { self.sizeOnly = Some(sizeOnly); self }
    pub fn build(self) -> GetEncodedResponseParams<'a> {
        GetEncodedResponseParams {
            requestId: self.requestId.unwrap_or_default(),
            encoding: self.encoding.unwrap_or_default(),
            quality: self.quality,
            sizeOnly: self.sizeOnly,
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
    originalSize: u64,
    /// Size after re-encoding.
    encodedSize: u64,
}

impl<'a> GetEncodedResponseReturns<'a> {
    pub fn builder() -> GetEncodedResponseReturnsBuilder<'a> { GetEncodedResponseReturnsBuilder::default() }
    pub fn body(&self) -> Option<&str> { self.body.as_deref() }
    pub fn originalSize(&self) -> u64 { self.originalSize }
    pub fn encodedSize(&self) -> u64 { self.encodedSize }
}

#[derive(Default)]
pub struct GetEncodedResponseReturnsBuilder<'a> {
    body: Option<Cow<'a, str>>,
    originalSize: Option<u64>,
    encodedSize: Option<u64>,
}

impl<'a> GetEncodedResponseReturnsBuilder<'a> {
    /// The encoded body as a base64 string. Omitted if sizeOnly is true. (Encoded as a base64 string when passed over JSON)
    pub fn body(mut self, body: impl Into<Cow<'a, str>>) -> Self { self.body = Some(body.into()); self }
    /// Size before re-encoding.
    pub fn originalSize(mut self, originalSize: u64) -> Self { self.originalSize = Some(originalSize); self }
    /// Size after re-encoding.
    pub fn encodedSize(mut self, encodedSize: u64) -> Self { self.encodedSize = Some(encodedSize); self }
    pub fn build(self) -> GetEncodedResponseReturns<'a> {
        GetEncodedResponseReturns {
            body: self.body,
            originalSize: self.originalSize.unwrap_or_default(),
            encodedSize: self.encodedSize.unwrap_or_default(),
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

impl DisableParams { pub const METHOD: &'static str = "Audits.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Audits.disable";
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
    formIssues: Vec<GenericIssueDetails<'a>>,
}

impl<'a> CheckFormsIssuesReturns<'a> {
    pub fn builder() -> CheckFormsIssuesReturnsBuilder<'a> { CheckFormsIssuesReturnsBuilder::default() }
    pub fn formIssues(&self) -> &[GenericIssueDetails<'a>] { &self.formIssues }
}

#[derive(Default)]
pub struct CheckFormsIssuesReturnsBuilder<'a> {
    formIssues: Option<Vec<GenericIssueDetails<'a>>>,
}

impl<'a> CheckFormsIssuesReturnsBuilder<'a> {
    pub fn formIssues(mut self, formIssues: Vec<GenericIssueDetails<'a>>) -> Self { self.formIssues = Some(formIssues); self }
    pub fn build(self) -> CheckFormsIssuesReturns<'a> {
        CheckFormsIssuesReturns {
            formIssues: self.formIssues.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckFormsIssuesParams {}

impl CheckFormsIssuesParams {
    pub fn builder() -> CheckFormsIssuesParamsBuilder {
        CheckFormsIssuesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct CheckFormsIssuesParamsBuilder {}

impl CheckFormsIssuesParamsBuilder {
    pub fn build(self) -> CheckFormsIssuesParams {
        CheckFormsIssuesParams {}
    }
}

impl CheckFormsIssuesParams { pub const METHOD: &'static str = "Audits.checkFormsIssues"; }

impl<'a> crate::CdpCommand<'a> for CheckFormsIssuesParams {
    const METHOD: &'static str = "Audits.checkFormsIssues";
    type Response = CheckFormsIssuesReturns<'a>;
}
