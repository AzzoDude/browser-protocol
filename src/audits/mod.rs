//! Audits domain allows investigation of page violations and possible improvements.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Information about a cookie that is affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AffectedCookie<'a> {
    /// The following three properties uniquely identify a cookie
    pub name: Cow<'a, str>,
    pub path: Cow<'a, str>,
    pub domain: Cow<'a, str>,
}
/// Information about a request that is affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AffectedRequest<'a> {
    /// The unique request id.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestId")]
    pub request_id: Option<crate::network::RequestId<'a>>,
    pub url: Cow<'a, str>,
}
/// Information about the frame affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AffectedFrame<'a> {
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CookieIssueInsight<'a> {
    #[serde(rename = "type")]
    pub type_: InsightType,
    /// Link to table entry in third-party cookie migration readiness list.
    #[serde(skip_serializing_if = "Option::is_none", rename = "tableEntryUrl")]
    pub table_entry_url: Option<Cow<'a, str>>,
}
/// This information is currently necessary, as the front-end has a difficult
/// time finding a specific cookie. With this, we can convey specific error
/// information without the cookie.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CookieIssueDetails<'a> {
    /// If AffectedCookie is not set then rawCookieLine contains the raw
    /// Set-Cookie header string. This hints at a problem where the
    /// cookie line is syntactically or semantically malformed in a way
    /// that no valid cookie could be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<AffectedCookie<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "rawCookieLine")]
    pub raw_cookie_line: Option<Cow<'a, str>>,
    #[serde(rename = "cookieWarningReasons")]
    pub cookie_warning_reasons: Vec<CookieWarningReason>,
    #[serde(rename = "cookieExclusionReasons")]
    pub cookie_exclusion_reasons: Vec<CookieExclusionReason>,
    /// Optionally identifies the site-for-cookies and the cookie url, which
    /// may be used by the front-end as additional context.
    pub operation: CookieOperation,
    #[serde(skip_serializing_if = "Option::is_none", rename = "siteForCookies")]
    pub site_for_cookies: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cookieUrl")]
    pub cookie_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest<'a>>,
    /// The recommended solution to the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<CookieIssueInsight<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PerformanceIssueType {
    #[default]
    #[serde(rename = "DocumentCookie")]
    DocumentCookie,
}

/// Details for a performance issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceIssueDetails<'a> {
    #[serde(rename = "performanceIssueType")]
    pub performance_issue_type: PerformanceIssueType,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceCodeLocation")]
    pub source_code_location: Option<SourceCodeLocation<'a>>,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MixedContentIssueDetails<'a> {
    /// The type of resource causing the mixed content issue (css, js, iframe,
    /// form,...). Marked as optional because it is mapped to from
    /// blink::mojom::RequestContextType, which will be replaced
    /// by network::mojom::RequestDestination
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType")]
    pub resource_type: Option<MixedContentResourceType>,
    /// The way the mixed content issue is being resolved.
    #[serde(rename = "resolutionStatus")]
    pub resolution_status: MixedContentResolutionStatus,
    /// The unsafe http url causing the mixed content issue.
    #[serde(rename = "insecureURL")]
    pub insecure_url: Cow<'a, str>,
    /// The url responsible for the call to an unsafe url.
    #[serde(rename = "mainResourceURL")]
    pub main_resource_url: Cow<'a, str>,
    /// The mixed content request.
    /// Does not always exist (e.g. for unsafe form submission urls).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest<'a>>,
    /// Optional because not every mixed content issue is necessarily linked to a frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<AffectedFrame<'a>>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BlockedByResponseIssueDetails<'a> {
    pub request: AffectedRequest<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentFrame")]
    pub parent_frame: Option<AffectedFrame<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockedFrame")]
    pub blocked_frame: Option<AffectedFrame<'a>>,
    pub reason: BlockedByResponseReason,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct HeavyAdIssueDetails<'a> {
    /// The resolution status, either blocking the content or warning.
    pub resolution: HeavyAdResolutionStatus,
    /// The reason the ad was blocked, total network or cpu or peak cpu.
    pub reason: HeavyAdReason,
    /// The frame that was blocked.
    pub frame: AffectedFrame<'a>,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SourceCodeLocation<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptId")]
    pub script_id: Option<crate::runtime::ScriptId<'a>>,
    pub url: Cow<'a, str>,
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyIssueDetails<'a> {
    /// The url not included in allowed sources.
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockedURL")]
    pub blocked_url: Option<Cow<'a, str>>,
    /// Specific directive that is violated, causing the CSP issue.
    #[serde(rename = "violatedDirective")]
    pub violated_directive: Cow<'a, str>,
    #[serde(rename = "isReportOnly")]
    pub is_report_only: bool,
    #[serde(rename = "contentSecurityPolicyViolationType")]
    pub content_security_policy_violation_type: ContentSecurityPolicyViolationType,
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameAncestor")]
    pub frame_ancestor: Option<AffectedFrame<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceCodeLocation")]
    pub source_code_location: Option<SourceCodeLocation<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "violatingNodeId")]
    pub violating_node_id: Option<crate::dom::BackendNodeId>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SharedArrayBufferIssueDetails<'a> {
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation<'a>,
    #[serde(rename = "isWarning")]
    pub is_warning: bool,
    #[serde(rename = "type")]
    pub type_: SharedArrayBufferIssueType,
}
/// Details for a CORS related issue, e.g. a warning or error related to
/// CORS RFC1918 enforcement.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CorsIssueDetails<'a> {
    #[serde(rename = "corsErrorStatus")]
    pub cors_error_status: crate::network::CorsErrorStatus<'a>,
    #[serde(rename = "isWarning")]
    pub is_warning: bool,
    pub request: AffectedRequest<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SourceCodeLocation<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "initiatorOrigin")]
    pub initiator_origin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceIPAddressSpace")]
    pub resource_ip_address_space: Option<crate::network::IPAddressSpace>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientSecurityState")]
    pub client_security_state: Option<crate::network::ClientSecurityState>,
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
    #[serde(rename = "WriteErrorInvalidMatchDestList")]
    WriteErrorInvalidMatchDestList,
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
    #[serde(rename = "IFrameAttributeLoosensEmbeddingRequirement")]
    IFrameAttributeLoosensEmbeddingRequirement,
    #[serde(rename = "InvalidAllowConnectionAllowlistFrom")]
    InvalidAllowConnectionAllowlistFrom,
    #[serde(rename = "EmbeddingRequirementNotSatisfied")]
    EmbeddingRequirementNotSatisfied,
}

/// Details for issues about documents in Quirks Mode
/// or Limited Quirks Mode that affects page layouting.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct QuirksModeIssueDetails<'a> {
    /// If false, it means the document's mode is "quirks"
    /// instead of "limited-quirks".
    #[serde(rename = "isLimitedQuirksMode")]
    pub is_limited_quirks_mode: bool,
    #[serde(rename = "documentNodeId")]
    pub document_node_id: crate::dom::BackendNodeId,
    pub url: Cow<'a, str>,
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
    #[serde(rename = "loaderId")]
    pub loader_id: crate::network::LoaderId<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NavigatorUserAgentIssueDetails<'a> {
    pub url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SourceCodeLocation<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SharedDictionaryIssueDetails<'a> {
    #[serde(rename = "sharedDictionaryError")]
    pub shared_dictionary_error: SharedDictionaryError,
    pub request: AffectedRequest<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SRIMessageSignatureIssueDetails<'a> {
    pub error: SRIMessageSignatureError,
    #[serde(rename = "signatureBase")]
    pub signature_base: Cow<'a, str>,
    #[serde(rename = "integrityAssertions")]
    pub integrity_assertions: Vec<Cow<'a, str>>,
    pub request: AffectedRequest<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct UnencodedDigestIssueDetails<'a> {
    pub error: UnencodedDigestError,
    pub request: AffectedRequest<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionAllowlistIssueDetails<'a> {
    pub error: ConnectionAllowlistError,
    pub request: AffectedRequest<'a>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GenericIssueDetails<'a> {
    /// Issues with the same errorType are aggregated in the frontend.
    #[serde(rename = "errorType")]
    pub error_type: GenericIssueErrorType,
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "violatingNodeId")]
    pub violating_node_id: Option<crate::dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "violatingNodeAttribute")]
    pub violating_node_attribute: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest<'a>>,
}
/// This issue tracks information needed to print a deprecation message.
/// <https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeprecationIssueDetails<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "affectedFrame")]
    pub affected_frame: Option<AffectedFrame<'a>>,
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation<'a>,
    /// One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
}
/// This issue warns about sites in the redirect chain of a finished navigation
/// that may be flagged as trackers and have their state cleared if they don't
/// receive a user interaction. Note that in this context 'site' means eTLD+1.
/// For example, if the URL 'https://example.test:80/bounce' was in the
/// redirect chain, the site reported would be 'example.test'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BounceTrackingIssueDetails<'a> {
    #[serde(rename = "trackingSites")]
    pub tracking_sites: Vec<Cow<'a, str>>,
}
/// This issue warns about third-party sites that are accessing cookies on the
/// current page, and have been permitted due to having a global metadata grant.
/// Note that in this context 'site' means eTLD+1. For example, if the URL
/// 'https://example.test:80/web_page' was accessing cookies, the site reported
/// would be 'example.test'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CookieDeprecationMetadataIssueDetails<'a> {
    #[serde(rename = "allowedSites")]
    pub allowed_sites: Vec<Cow<'a, str>>,
    #[serde(rename = "optOutPercentage")]
    pub opt_out_percentage: f64,
    #[serde(rename = "isOptOutTopLevel")]
    pub is_opt_out_top_level: bool,
    pub operation: CookieOperation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ClientHintIssueReason {
    #[default]
    #[serde(rename = "MetaTagAllowListInvalidOrigin")]
    MetaTagAllowListInvalidOrigin,
    #[serde(rename = "MetaTagModifiedHTML")]
    MetaTagModifiedHTML,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FederatedAuthRequestIssueDetails {
    #[serde(rename = "federatedAuthRequestIssueReason")]
    pub federated_auth_request_issue_reason: FederatedAuthRequestIssueReason,
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
    #[serde(rename = "WellKnownBlockedByConnectionAllowlist")]
    WellKnownBlockedByConnectionAllowlist,
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
    #[serde(rename = "ConfigBlockedByConnectionAllowlist")]
    ConfigBlockedByConnectionAllowlist,
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
    #[serde(rename = "AccountsBlockedByConnectionAllowlist")]
    AccountsBlockedByConnectionAllowlist,
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
    #[serde(rename = "IdTokenBlockedByConnectionAllowlist")]
    IdTokenBlockedByConnectionAllowlist,
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
    #[serde(rename = "PopupBlockedByConnectionAllowlist")]
    PopupBlockedByConnectionAllowlist,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FederatedAuthUserInfoRequestIssueDetails {
    #[serde(rename = "federatedAuthUserInfoRequestIssueReason")]
    pub federated_auth_user_info_request_issue_reason: FederatedAuthUserInfoRequestIssueReason,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct EmailVerificationRequestIssueDetails {
    #[serde(rename = "emailVerificationRequestIssueReason")]
    pub email_verification_request_issue_reason: EmailVerificationRequestIssueReason,
}
/// Represents the failure reason when an email verification request fails.
/// Should be updated alongside EmailVerificationRequestResult in
/// third_party/blink/public/mojom/devtools/inspector_issue.mojom.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum EmailVerificationRequestIssueReason {
    #[default]
    #[serde(rename = "InvalidEmail")]
    InvalidEmail,
    #[serde(rename = "DnsFetchFailed")]
    DnsFetchFailed,
    #[serde(rename = "DnsInvalidRecord")]
    DnsInvalidRecord,
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
    #[serde(rename = "WellKnownMissingIssuanceEndpoint")]
    WellKnownMissingIssuanceEndpoint,
    #[serde(rename = "WellKnownIssuanceEndpointCrossOrigin")]
    WellKnownIssuanceEndpointCrossOrigin,
    #[serde(rename = "WellKnownUnsupportedSigningAlgorithm")]
    WellKnownUnsupportedSigningAlgorithm,
    #[serde(rename = "TokenHttpNotFound")]
    TokenHttpNotFound,
    #[serde(rename = "TokenNoResponse")]
    TokenNoResponse,
    #[serde(rename = "TokenInvalidResponse")]
    TokenInvalidResponse,
    #[serde(rename = "TokenInvalidContentType")]
    TokenInvalidContentType,
    #[serde(rename = "TokenMalformedSdJwt")]
    TokenMalformedSdJwt,
    #[serde(rename = "TokenInvalidSdJwt")]
    TokenInvalidSdJwt,
    #[serde(rename = "KeyBindingSigningFailed")]
    KeyBindingSigningFailed,
    #[serde(rename = "RpOriginIsOpaque")]
    RpOriginIsOpaque,
    #[serde(rename = "WellKnownMissingAccountsEndpoint")]
    WellKnownMissingAccountsEndpoint,
    #[serde(rename = "UserLoggedOut")]
    UserLoggedOut,
    #[serde(rename = "WellKnownAccountsEndpointCrossOrigin")]
    WellKnownAccountsEndpointCrossOrigin,
    #[serde(rename = "AccountsHttpNotFound")]
    AccountsHttpNotFound,
    #[serde(rename = "AccountsNoResponse")]
    AccountsNoResponse,
    #[serde(rename = "AccountsInvalidResponse")]
    AccountsInvalidResponse,
    #[serde(rename = "AccountsInvalidContentType")]
    AccountsInvalidContentType,
    #[serde(rename = "AccountsEmptyList")]
    AccountsEmptyList,
    #[serde(rename = "EmailVerificationWellKnownHttpNotFound")]
    EmailVerificationWellKnownHttpNotFound,
    #[serde(rename = "EmailVerificationWellKnownNoResponse")]
    EmailVerificationWellKnownNoResponse,
    #[serde(rename = "EmailVerificationWellKnownInvalidResponse")]
    EmailVerificationWellKnownInvalidResponse,
    #[serde(rename = "EmailVerificationWellKnownInvalidContentType")]
    EmailVerificationWellKnownInvalidContentType,
    #[serde(rename = "JwksHttpNotFound")]
    JwksHttpNotFound,
    #[serde(rename = "JwksInvalidResponse")]
    JwksInvalidResponse,
    #[serde(rename = "TokenVerificationSdJwtUnsupportedHeaderAlg")]
    TokenVerificationSdJwtUnsupportedHeaderAlg,
    #[serde(rename = "TokenVerificationSdJwtInvalidTyp")]
    TokenVerificationSdJwtInvalidTyp,
    #[serde(rename = "TokenVerificationSdJwtMissingIss")]
    TokenVerificationSdJwtMissingIss,
    #[serde(rename = "TokenVerificationSdJwtMissingIat")]
    TokenVerificationSdJwtMissingIat,
    #[serde(rename = "TokenVerificationSdJwtMissingCnf")]
    TokenVerificationSdJwtMissingCnf,
    #[serde(rename = "TokenVerificationSdJwtMissingEmail")]
    TokenVerificationSdJwtMissingEmail,
    #[serde(rename = "TokenVerificationSdJwtInvalidIssuedAt")]
    TokenVerificationSdJwtInvalidIssuedAt,
    #[serde(rename = "TokenVerificationSdJwtInvalidIssuer")]
    TokenVerificationSdJwtInvalidIssuer,
    #[serde(rename = "TokenVerificationSdJwtJwksMissingKeys")]
    TokenVerificationSdJwtJwksMissingKeys,
    #[serde(rename = "TokenVerificationSdJwtSignatureFailed")]
    TokenVerificationSdJwtSignatureFailed,
    #[serde(rename = "TokenVerificationSdJwtInvalidEmailVerified")]
    TokenVerificationSdJwtInvalidEmailVerified,
    #[serde(rename = "TokenVerificationSdJwtInvalidEmail")]
    TokenVerificationSdJwtInvalidEmail,
    #[serde(rename = "TokenVerificationSdJwtInvalidHolderKey")]
    TokenVerificationSdJwtInvalidHolderKey,
    #[serde(rename = "TokenVerificationKbInvalidTyp")]
    TokenVerificationKbInvalidTyp,
    #[serde(rename = "TokenVerificationKbMissingAud")]
    TokenVerificationKbMissingAud,
    #[serde(rename = "TokenVerificationKbMissingNonce")]
    TokenVerificationKbMissingNonce,
    #[serde(rename = "TokenVerificationKbMissingIat")]
    TokenVerificationKbMissingIat,
    #[serde(rename = "TokenVerificationKbMissingSdHash")]
    TokenVerificationKbMissingSdHash,
    #[serde(rename = "TokenVerificationKbInvalidIssuedAt")]
    TokenVerificationKbInvalidIssuedAt,
    #[serde(rename = "TokenVerificationKbInvalidAudience")]
    TokenVerificationKbInvalidAudience,
    #[serde(rename = "TokenVerificationKbInvalidNonce")]
    TokenVerificationKbInvalidNonce,
    #[serde(rename = "TokenVerificationKbInvalidSdHash")]
    TokenVerificationKbInvalidSdHash,
    #[serde(rename = "TokenVerificationKbMissingCnf")]
    TokenVerificationKbMissingCnf,
    #[serde(rename = "TokenVerificationKbSignatureFailed")]
    TokenVerificationKbSignatureFailed,
    #[serde(rename = "CrossOriginIframeNotSupported")]
    CrossOriginIframeNotSupported,
}

/// This issue tracks client hints related issues. It's used to deprecate old
/// features, encourage the use of new ones, and provide general guidance.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ClientHintIssueDetails<'a> {
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation<'a>,
    #[serde(rename = "clientHintIssueReason")]
    pub client_hint_issue_reason: ClientHintIssueReason,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FailedRequestInfo<'a> {
    /// The URL that failed to load.
    pub url: Cow<'a, str>,
    /// The failure message for the failed request.
    #[serde(rename = "failureMessage")]
    pub failure_message: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestId")]
    pub request_id: Option<crate::network::RequestId<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PartitioningBlobURLInfo {
    #[default]
    #[serde(rename = "BlockedCrossPartitionFetching")]
    BlockedCrossPartitionFetching,
    #[serde(rename = "EnforceNoopenerForNavigation")]
    EnforceNoopenerForNavigation,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PartitioningBlobURLIssueDetails<'a> {
    /// The BlobURL that failed to load.
    pub url: Cow<'a, str>,
    /// Additional information about the Partitioning Blob URL issue.
    #[serde(rename = "partitioningBlobURLInfo")]
    pub partitioning_blob_url_info: PartitioningBlobURLInfo,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ElementAccessibilityIssueDetails {
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::BackendNodeId,
    #[serde(rename = "elementAccessibilityIssueReason")]
    pub element_accessibility_issue_reason: ElementAccessibilityIssueReason,
    #[serde(rename = "hasDisallowedAttributes")]
    pub has_disallowed_attributes: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StylesheetLoadingIssueDetails<'a> {
    /// Source code position that referenced the failing stylesheet.
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation<'a>,
    /// Reason why the stylesheet couldn't be loaded.
    #[serde(rename = "styleSheetLoadingIssueReason")]
    pub style_sheet_loading_issue_reason: StyleSheetLoadingIssueReason,
    /// Contains additional info when the failure was due to a request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "failedRequestInfo")]
    pub failed_request_info: Option<FailedRequestInfo<'a>>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PropertyRuleIssueDetails<'a> {
    /// Source code position of the property rule.
    #[serde(rename = "sourceCodeLocation")]
    pub source_code_location: SourceCodeLocation<'a>,
    /// Reason why the property rule was discarded.
    #[serde(rename = "propertyRuleIssueReason")]
    pub property_rule_issue_reason: PropertyRuleIssueReason,
    /// The value of the property rule property that failed to parse
    #[serde(skip_serializing_if = "Option::is_none", rename = "propertyValue")]
    pub property_value: Option<Cow<'a, str>>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct UserReidentificationIssueDetails<'a> {
    #[serde(rename = "type")]
    pub type_: UserReidentificationIssueType,
    /// Applies to BlockedFrameNavigation and BlockedSubresource issue types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest<'a>>,
    /// Applies to NoisedCanvasReadback issue type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceCodeLocation")]
    pub source_code_location: Option<SourceCodeLocation<'a>>,
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
    #[serde(rename = "NonSecureContext")]
    NonSecureContext,
    #[serde(rename = "MissingTransientUserActivation")]
    MissingTransientUserActivation,
}

/// This issue warns about improper usage of the \<permission\> element.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PermissionElementIssueDetails<'a> {
    #[serde(rename = "issueType")]
    pub issue_type: PermissionElementIssueType,
    /// The value of the type attribute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Cow<'a, str>>,
    /// The node ID of the \<permission\> element.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeId")]
    pub node_id: Option<crate::dom::BackendNodeId>,
    /// True if the issue is a warning, false if it is an error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isWarning")]
    pub is_warning: Option<bool>,
    /// Fields for message construction:
    /// Used for messages that reference a specific permission name
    #[serde(skip_serializing_if = "Option::is_none", rename = "permissionName")]
    pub permission_name: Option<Cow<'a, str>>,
    /// Used for messages about occlusion
    #[serde(skip_serializing_if = "Option::is_none", rename = "occluderNodeInfo")]
    pub occluder_node_info: Option<Cow<'a, str>>,
    /// Used for messages about occluder's parent
    #[serde(skip_serializing_if = "Option::is_none", rename = "occluderParentNodeInfo")]
    pub occluder_parent_node_info: Option<Cow<'a, str>>,
    /// Used for messages about activation disabled reason
    #[serde(skip_serializing_if = "Option::is_none", rename = "disableReason")]
    pub disable_reason: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum WebInstallIssueReason {
    #[default]
    #[serde(rename = "ManifestParsingOrNetworkError")]
    ManifestParsingOrNetworkError,
    #[serde(rename = "StartUrlInvalid")]
    StartUrlInvalid,
    #[serde(rename = "ManifestMissingNameOrShortName")]
    ManifestMissingNameOrShortName,
    #[serde(rename = "ManifestMissingId")]
    ManifestMissingId,
    #[serde(rename = "NoManifest")]
    NoManifest,
}

/// This issue reports a failure involving a web app manifest used by a Web
/// Install operation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct WebInstallIssueDetails<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "manifestUrl")]
    pub manifest_url: Option<Cow<'a, str>>,
    pub reason: WebInstallIssueReason,
}
/// The issue warns about blocked calls to privacy sensitive APIs via the
/// Selective Permissions Intervention.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SelectivePermissionsInterventionIssueDetails<'a> {
    /// Which API was intervened on.
    #[serde(rename = "apiName")]
    pub api_name: Cow<'a, str>,
    /// Why the ad script using the API is considered an ad.
    #[serde(rename = "adAncestry")]
    pub ad_ancestry: crate::network::AdAncestry<'a>,
    /// The stack trace at the time of the intervention.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackTrace")]
    pub stack_trace: Option<crate::runtime::StackTrace>,
}
/// Details for issues about lazy-loaded images without explicit dimensions.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LazyLoadImageIssueDetails<'a> {
    /// DOM node of the problematic HTMLImageElement.
    #[serde(rename = "nodeId")]
    pub node_id: crate::dom::BackendNodeId,
    /// URL or src attribute of the image.
    pub url: Cow<'a, str>,
    /// Frame containing the image.
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
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
    #[serde(rename = "EmailVerificationRequestIssue")]
    EmailVerificationRequestIssue,
    #[serde(rename = "LazyLoadImageIssue")]
    LazyLoadImageIssue,
    #[serde(rename = "WebInstallIssue")]
    WebInstallIssue,
}

/// This struct holds a list of optional fields with additional information
/// specific to the kind of issue. When adding a new issue code, please also
/// add a new optional field to this type.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InspectorIssueDetails<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "cookieIssueDetails")]
    pub cookie_issue_details: Option<CookieIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mixedContentIssueDetails")]
    pub mixed_content_issue_details: Option<MixedContentIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockedByResponseIssueDetails")]
    pub blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "heavyAdIssueDetails")]
    pub heavy_ad_issue_details: Option<HeavyAdIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentSecurityPolicyIssueDetails")]
    pub content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sharedArrayBufferIssueDetails")]
    pub shared_array_buffer_issue_details: Option<SharedArrayBufferIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "corsIssueDetails")]
    pub cors_issue_details: Option<CorsIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quirksModeIssueDetails")]
    pub quirks_mode_issue_details: Option<QuirksModeIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitioningBlobURLIssueDetails")]
    pub partitioning_blob_url_issue_details: Option<PartitioningBlobURLIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "navigatorUserAgentIssueDetails")]
    pub navigator_user_agent_issue_details: Option<NavigatorUserAgentIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "genericIssueDetails")]
    pub generic_issue_details: Option<GenericIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "deprecationIssueDetails")]
    pub deprecation_issue_details: Option<DeprecationIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientHintIssueDetails")]
    pub client_hint_issue_details: Option<ClientHintIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "federatedAuthRequestIssueDetails")]
    pub federated_auth_request_issue_details: Option<FederatedAuthRequestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bounceTrackingIssueDetails")]
    pub bounce_tracking_issue_details: Option<BounceTrackingIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cookieDeprecationMetadataIssueDetails")]
    pub cookie_deprecation_metadata_issue_details: Option<CookieDeprecationMetadataIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "stylesheetLoadingIssueDetails")]
    pub stylesheet_loading_issue_details: Option<StylesheetLoadingIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "propertyRuleIssueDetails")]
    pub property_rule_issue_details: Option<PropertyRuleIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "federatedAuthUserInfoRequestIssueDetails")]
    pub federated_auth_user_info_request_issue_details: Option<FederatedAuthUserInfoRequestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sharedDictionaryIssueDetails")]
    pub shared_dictionary_issue_details: Option<SharedDictionaryIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "elementAccessibilityIssueDetails")]
    pub element_accessibility_issue_details: Option<ElementAccessibilityIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sriMessageSignatureIssueDetails")]
    pub sri_message_signature_issue_details: Option<SRIMessageSignatureIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "unencodedDigestIssueDetails")]
    pub unencoded_digest_issue_details: Option<UnencodedDigestIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectionAllowlistIssueDetails")]
    pub connection_allowlist_issue_details: Option<ConnectionAllowlistIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "userReidentificationIssueDetails")]
    pub user_reidentification_issue_details: Option<UserReidentificationIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "permissionElementIssueDetails")]
    pub permission_element_issue_details: Option<PermissionElementIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "performanceIssueDetails")]
    pub performance_issue_details: Option<PerformanceIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "selectivePermissionsInterventionIssueDetails")]
    pub selective_permissions_intervention_issue_details: Option<SelectivePermissionsInterventionIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "emailVerificationRequestIssueDetails")]
    pub email_verification_request_issue_details: Option<EmailVerificationRequestIssueDetails>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lazyLoadImageIssueDetails")]
    pub lazy_load_image_issue_details: Option<LazyLoadImageIssueDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "webInstallIssueDetails")]
    pub web_install_issue_details: Option<WebInstallIssueDetails<'a>>,
}
/// A unique id for a DevTools inspector issue. Allows other entities (e.g.
/// exceptions, CDP message, console messages, etc.) to reference an issue.

pub type IssueId<'a> = Cow<'a, str>;

/// An inspector issue reported from the back-end.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InspectorIssue<'a> {
    pub code: InspectorIssueCode,
    pub details: InspectorIssueDetails<'a>,
    /// A unique id for this issue. May be omitted if no other entity (e.g.
    /// exception, CDP message, etc.) is referencing this issue.
    #[serde(skip_serializing_if = "Option::is_none", rename = "issueId")]
    pub issue_id: Option<IssueId<'a>>,
}
/// Returns the response body and size if it were re-encoded with the specified settings. Only
/// applies to images.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Audits.getEncodedResponse", response = "GetEncodedResponseReturns<'a>")]
pub struct GetEncodedResponseParams<'a> {
    /// Identifier of the network request to get content for.
    #[serde(rename = "requestId")]
    pub request_id: crate::network::RequestId<'a>,
    /// The encoding to use.
    pub encoding: Cow<'a, str>,
    /// The quality of the encoding (0-1). (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<f64>,
    /// Whether to only return the size information (defaults to false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "sizeOnly")]
    pub size_only: Option<bool>,
}
/// Returns the response body and size if it were re-encoded with the specified settings. Only
/// applies to images.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetEncodedResponseReturns<'a> {
    /// The encoded body as a base64 string. Omitted if sizeOnly is true. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Cow<'a, str>>,
    /// Size before re-encoding.
    #[serde(rename = "originalSize")]
    pub original_size: u64,
    /// Size after re-encoding.
    #[serde(rename = "encodedSize")]
    pub encoded_size: u64,
}
/// Disables issues domain, prevents further issues from being reported to the client.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Audits.disable")]
pub struct DisableParams {

}
/// Enables issues domain, sends the issues collected so far to the client by means of the
/// 'issueAdded' event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Audits.enable")]
pub struct EnableParams {

}
/// Runs the form issues check for the target page. Found issues are reported
/// using Audits.issueAdded event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Audits.checkFormsIssues", response = "CheckFormsIssuesReturns<'a>")]
pub struct CheckFormsIssuesParams {

}
/// Runs the form issues check for the target page. Found issues are reported
/// using Audits.issueAdded event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CheckFormsIssuesReturns<'a> {
    #[serde(rename = "formIssues")]
    pub form_issues: Vec<GenericIssueDetails<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Audits.issueAdded")]
pub struct IssueAdded<'a> {
    pub issue: InspectorIssue<'a>,
}