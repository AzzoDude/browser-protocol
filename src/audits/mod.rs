//! Audits domain allows investigation of page violations and possible improvements.

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Information about a cookie that is affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AffectedCookie {
    /// The following three properties uniquely identify a cookie

    pub name: String,

    pub path: String,

    pub domain: String,
}

/// Information about a request that is affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AffectedRequest {
    /// The unique request id.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestId: Option<crate::network::RequestId>,

    pub url: String,
}

/// Information about the frame affected by an inspector issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AffectedFrame {

    pub frameId: crate::page::FrameId,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieExclusionReason {
    #[default]
    ExcludeSameSiteUnspecifiedTreatedAsLax,
    ExcludeSameSiteNoneInsecure,
    ExcludeSameSiteLax,
    ExcludeSameSiteStrict,
    ExcludeDomainNonASCII,
    ExcludeThirdPartyCookieBlockedInFirstPartySet,
    ExcludeThirdPartyPhaseout,
    ExcludePortMismatch,
    ExcludeSchemeMismatch,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieWarningReason {
    #[default]
    WarnSameSiteUnspecifiedCrossSiteContext,
    WarnSameSiteNoneInsecure,
    WarnSameSiteUnspecifiedLaxAllowUnsafe,
    WarnSameSiteStrictLaxDowngradeStrict,
    WarnSameSiteStrictCrossDowngradeStrict,
    WarnSameSiteStrictCrossDowngradeLax,
    WarnSameSiteLaxCrossDowngradeStrict,
    WarnSameSiteLaxCrossDowngradeLax,
    WarnAttributeValueExceedsMaxSize,
    WarnDomainNonASCII,
    WarnThirdPartyPhaseout,
    WarnCrossSiteRedirectDowngradeChangesInclusion,
    WarnDeprecationTrialMetadata,
    WarnThirdPartyCookieHeuristic,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieOperation {
    #[default]
    SetCookie,
    ReadCookie,
}

/// Represents the category of insight that a cookie issue falls under.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InsightType {
    #[default]
    GitHubResource,
    GracePeriod,
    Heuristics,
}

/// Information about the suggested solution to a cookie issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookieIssueInsight {

    #[serde(rename = "type")]
    pub type_: InsightType,
    /// Link to table entry in third-party cookie migration readiness list.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tableEntryUrl: Option<String>,
}

/// This information is currently necessary, as the front-end has a difficult
/// time finding a specific cookie. With this, we can convey specific error
/// information without the cookie.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookieIssueDetails {
    /// If AffectedCookie is not set then rawCookieLine contains the raw
    /// Set-Cookie header string. This hints at a problem where the
    /// cookie line is syntactically or semantically malformed in a way
    /// that no valid cookie could be created.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<AffectedCookie>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rawCookieLine: Option<String>,

    pub cookieWarningReasons: Vec<CookieWarningReason>,

    pub cookieExclusionReasons: Vec<CookieExclusionReason>,
    /// Optionally identifies the site-for-cookies and the cookie url, which
    /// may be used by the front-end as additional context.

    pub operation: CookieOperation,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub siteForCookies: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookieUrl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest>,
    /// The recommended solution to the issue.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<CookieIssueInsight>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PerformanceIssueType {
    #[default]
    DocumentCookie,
}

/// Details for a performance issue.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceIssueDetails {

    pub performanceIssueType: PerformanceIssueType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceCodeLocation: Option<SourceCodeLocation>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MixedContentResolutionStatus {
    #[default]
    MixedContentBlocked,
    MixedContentAutomaticallyUpgraded,
    MixedContentWarning,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MixedContentResourceType {
    #[default]
    AttributionSrc,
    Audio,
    Beacon,
    CSPReport,
    Download,
    EventSource,
    Favicon,
    Font,
    Form,
    Frame,
    Image,
    Import,
    JSON,
    Manifest,
    Ping,
    PluginData,
    PluginResource,
    Prefetch,
    Resource,
    Script,
    ServiceWorker,
    SharedWorker,
    SpeculationRules,
    Stylesheet,
    Track,
    Video,
    Worker,
    XMLHttpRequest,
    XSLT,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MixedContentIssueDetails {
    /// The type of resource causing the mixed content issue (css, js, iframe,
    /// form,...). Marked as optional because it is mapped to from
    /// blink::mojom::RequestContextType, which will be replaced
    /// by network::mojom::RequestDestination

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resourceType: Option<MixedContentResourceType>,
    /// The way the mixed content issue is being resolved.

    pub resolutionStatus: MixedContentResolutionStatus,
    /// The unsafe http url causing the mixed content issue.

    pub insecureURL: String,
    /// The url responsible for the call to an unsafe url.

    pub mainResourceURL: String,
    /// The mixed content request.
    /// Does not always exist (e.g. for unsafe form submission urls).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest>,
    /// Optional because not every mixed content issue is necessarily linked to a frame.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<AffectedFrame>,
}

/// Enum indicating the reason a response has been blocked. These reasons are
/// refinements of the net error BLOCKED_BY_RESPONSE.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BlockedByResponseReason {
    #[default]
    CoepFrameResourceNeedsCoepHeader,
    CoopSandboxedIFrameCannotNavigateToCoopPage,
    CorpNotSameOrigin,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    CorpNotSameSite,
    SRIMessageSignatureMismatch,
}

/// Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
/// code. Currently only used for COEP/COOP, but may be extended to include
/// some CSP errors in the future.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockedByResponseIssueDetails {

    pub request: AffectedRequest,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentFrame: Option<AffectedFrame>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockedFrame: Option<AffectedFrame>,

    pub reason: BlockedByResponseReason,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum HeavyAdResolutionStatus {
    #[default]
    HeavyAdBlocked,
    HeavyAdWarning,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum HeavyAdReason {
    #[default]
    NetworkTotalLimit,
    CpuTotalLimit,
    CpuPeakLimit,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeavyAdIssueDetails {
    /// The resolution status, either blocking the content or warning.

    pub resolution: HeavyAdResolutionStatus,
    /// The reason the ad was blocked, total network or cpu or peak cpu.

    pub reason: HeavyAdReason,
    /// The frame that was blocked.

    pub frame: AffectedFrame,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContentSecurityPolicyViolationType {
    #[default]
    KInlineViolation,
    KEvalViolation,
    KURLViolation,
    KSRIViolation,
    KTrustedTypesSinkViolation,
    KTrustedTypesPolicyViolation,
    KWasmEvalViolation,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceCodeLocation {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriptId: Option<crate::runtime::ScriptId>,

    pub url: String,

    pub lineNumber: i64,

    pub columnNumber: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyIssueDetails {
    /// The url not included in allowed sources.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockedURL: Option<String>,
    /// Specific directive that is violated, causing the CSP issue.

    pub violatedDirective: String,

    pub isReportOnly: bool,

    pub contentSecurityPolicyViolationType: ContentSecurityPolicyViolationType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameAncestor: Option<AffectedFrame>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceCodeLocation: Option<SourceCodeLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub violatingNodeId: Option<crate::dom::BackendNodeId>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SharedArrayBufferIssueType {
    #[default]
    TransferIssue,
    CreationIssue,
}

/// Details for a issue arising from an SAB being instantiated in, or
/// transferred to a context that is not cross-origin isolated.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedArrayBufferIssueDetails {

    pub sourceCodeLocation: SourceCodeLocation,

    pub isWarning: bool,

    #[serde(rename = "type")]
    pub type_: SharedArrayBufferIssueType,
}

/// Details for a CORS related issue, e.g. a warning or error related to
/// CORS RFC1918 enforcement.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CorsIssueDetails {

    pub corsErrorStatus: crate::network::CorsErrorStatus,

    pub isWarning: bool,

    pub request: AffectedRequest,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SourceCodeLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiatorOrigin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resourceIPAddressSpace: Option<crate::network::IPAddressSpace>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientSecurityState: Option<crate::network::ClientSecurityState>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AttributionReportingIssueType {
    #[default]
    PermissionPolicyDisabled,
    UntrustworthyReportingOrigin,
    InsecureContext,
    InvalidHeader,
    InvalidRegisterTriggerHeader,
    SourceAndTriggerHeaders,
    SourceIgnored,
    TriggerIgnored,
    OsSourceIgnored,
    OsTriggerIgnored,
    InvalidRegisterOsSourceHeader,
    InvalidRegisterOsTriggerHeader,
    WebAndOsHeaders,
    NoWebOrOsSupport,
    NavigationRegistrationWithoutTransientUserActivation,
    InvalidInfoHeader,
    NoRegisterSourceHeader,
    NoRegisterTriggerHeader,
    NoRegisterOsSourceHeader,
    NoRegisterOsTriggerHeader,
    NavigationRegistrationUniqueScopeAlreadySet,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SharedDictionaryError {
    #[default]
    UseErrorCrossOriginNoCorsRequest,
    UseErrorDictionaryLoadFailure,
    UseErrorMatchingDictionaryNotUsed,
    UseErrorUnexpectedContentDictionaryHeader,
    WriteErrorCossOriginNoCorsRequest,
    WriteErrorDisallowedBySettings,
    WriteErrorExpiredResponse,
    WriteErrorFeatureDisabled,
    WriteErrorInsufficientResources,
    WriteErrorInvalidMatchField,
    WriteErrorInvalidStructuredHeader,
    WriteErrorInvalidTTLField,
    WriteErrorNavigationRequest,
    WriteErrorNoMatchField,
    WriteErrorNonIntegerTTLField,
    WriteErrorNonListMatchDestField,
    WriteErrorNonSecureContext,
    WriteErrorNonStringIdField,
    WriteErrorNonStringInMatchDestList,
    WriteErrorNonStringMatchField,
    WriteErrorNonTokenTypeField,
    WriteErrorRequestAborted,
    WriteErrorShuttingDown,
    WriteErrorTooLongIdField,
    WriteErrorUnsupportedType,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SRIMessageSignatureError {
    #[default]
    MissingSignatureHeader,
    MissingSignatureInputHeader,
    InvalidSignatureHeader,
    InvalidSignatureInputHeader,
    SignatureHeaderValueIsNotByteSequence,
    SignatureHeaderValueIsParameterized,
    SignatureHeaderValueIsIncorrectLength,
    SignatureInputHeaderMissingLabel,
    SignatureInputHeaderValueNotInnerList,
    SignatureInputHeaderValueMissingComponents,
    SignatureInputHeaderInvalidComponentType,
    SignatureInputHeaderInvalidComponentName,
    SignatureInputHeaderInvalidHeaderComponentParameter,
    SignatureInputHeaderInvalidDerivedComponentParameter,
    SignatureInputHeaderKeyIdLength,
    SignatureInputHeaderInvalidParameter,
    SignatureInputHeaderMissingRequiredParameters,
    ValidationFailedSignatureExpired,
    ValidationFailedInvalidLength,
    ValidationFailedSignatureMismatch,
    ValidationFailedIntegrityMismatch,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum UnencodedDigestError {
    #[default]
    MalformedDictionary,
    UnknownAlgorithm,
    IncorrectDigestType,
    IncorrectDigestLength,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ConnectionAllowlistError {
    #[default]
    InvalidHeader,
    MoreThanOneList,
    ItemNotInnerList,
    InvalidAllowlistItemType,
    ReportingEndpointNotToken,
    InvalidUrlPattern,
}

/// Details for issues around "Attribution Reporting API" usage.
/// Explainer: <https://github.com/WICG/attribution-reporting-api>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingIssueDetails {

    pub violationType: AttributionReportingIssueType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub violatingNodeId: Option<crate::dom::BackendNodeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidParameter: Option<String>,
}

/// Details for issues about documents in Quirks Mode
/// or Limited Quirks Mode that affects page layouting.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuirksModeIssueDetails {
    /// If false, it means the document's mode is "quirks"
    /// instead of "limited-quirks".

    pub isLimitedQuirksMode: bool,

    pub documentNodeId: crate::dom::BackendNodeId,

    pub url: String,

    pub frameId: crate::page::FrameId,

    pub loaderId: crate::network::LoaderId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigatorUserAgentIssueDetails {

    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SourceCodeLocation>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedDictionaryIssueDetails {

    pub sharedDictionaryError: SharedDictionaryError,

    pub request: AffectedRequest,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SRIMessageSignatureIssueDetails {

    pub error: SRIMessageSignatureError,

    pub signatureBase: String,

    pub integrityAssertions: Vec<String>,

    pub request: AffectedRequest,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnencodedDigestIssueDetails {

    pub error: UnencodedDigestError,

    pub request: AffectedRequest,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionAllowlistIssueDetails {

    pub error: ConnectionAllowlistError,

    pub request: AffectedRequest,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GenericIssueErrorType {
    #[default]
    FormLabelForNameError,
    FormDuplicateIdForInputError,
    FormInputWithNoLabelError,
    FormAutocompleteAttributeEmptyError,
    FormEmptyIdAndNameAttributesForInputError,
    FormAriaLabelledByToNonExistingIdError,
    FormInputAssignedAutocompleteValueToIdOrNameAttributeError,
    FormLabelHasNeitherForNorNestedInputError,
    FormLabelForMatchesNonExistingIdError,
    FormInputHasWrongButWellIntendedAutocompleteValueError,
    ResponseWasBlockedByORB,
    NavigationEntryMarkedSkippable,
    AutofillAndManualTextPolicyControlledFeaturesInfo,
    AutofillPolicyControlledFeatureInfo,
    ManualTextPolicyControlledFeatureInfo,
    FormModelContextParameterMissingTitleAndDescription,
    FormModelContextMissingToolName,
    FormModelContextMissingToolDescription,
    FormModelContextRequiredParameterMissingName,
    FormModelContextParameterMissingName,
}

/// Depending on the concrete errorType, different properties are set.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenericIssueDetails {
    /// Issues with the same errorType are aggregated in the frontend.

    pub errorType: GenericIssueErrorType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub violatingNodeId: Option<crate::dom::BackendNodeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub violatingNodeAttribute: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest>,
}

/// This issue tracks information needed to print a deprecation message.
/// <https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeprecationIssueDetails {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affectedFrame: Option<AffectedFrame>,

    pub sourceCodeLocation: SourceCodeLocation,
    /// One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5

    #[serde(rename = "type")]
    pub type_: String,
}

/// This issue warns about sites in the redirect chain of a finished navigation
/// that may be flagged as trackers and have their state cleared if they don't
/// receive a user interaction. Note that in this context 'site' means eTLD+1.
/// For example, if the URL '<https://example.test:80/bounce'> was in the
/// redirect chain, the site reported would be 'example.test'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BounceTrackingIssueDetails {

    pub trackingSites: Vec<String>,
}

/// This issue warns about third-party sites that are accessing cookies on the
/// current page, and have been permitted due to having a global metadata grant.
/// Note that in this context 'site' means eTLD+1. For example, if the URL
/// '<https://example.test:80/web_page'> was accessing cookies, the site reported
/// would be 'example.test'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookieDeprecationMetadataIssueDetails {

    pub allowedSites: Vec<String>,

    pub optOutPercentage: f64,

    pub isOptOutTopLevel: bool,

    pub operation: CookieOperation,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ClientHintIssueReason {
    #[default]
    MetaTagAllowListInvalidOrigin,
    MetaTagModifiedHTML,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FederatedAuthRequestIssueDetails {

    pub federatedAuthRequestIssueReason: FederatedAuthRequestIssueReason,
}

/// Represents the failure reason when a federated authentication reason fails.
/// Should be updated alongside RequestIdTokenStatus in
/// third_party/blink/public/mojom/devtools/inspector_issue.mojom to include
/// all cases except for success.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum FederatedAuthRequestIssueReason {
    #[default]
    ShouldEmbargo,
    TooManyRequests,
    WellKnownHttpNotFound,
    WellKnownNoResponse,
    WellKnownInvalidResponse,
    WellKnownListEmpty,
    WellKnownInvalidContentType,
    ConfigNotInWellKnown,
    WellKnownTooBig,
    ConfigHttpNotFound,
    ConfigNoResponse,
    ConfigInvalidResponse,
    ConfigInvalidContentType,
    IdpNotPotentiallyTrustworthy,
    DisabledInSettings,
    DisabledInFlags,
    ErrorFetchingSignin,
    InvalidSigninResponse,
    AccountsHttpNotFound,
    AccountsNoResponse,
    AccountsInvalidResponse,
    AccountsListEmpty,
    AccountsInvalidContentType,
    IdTokenHttpNotFound,
    IdTokenNoResponse,
    IdTokenInvalidResponse,
    IdTokenIdpErrorResponse,
    IdTokenCrossSiteIdpErrorResponse,
    IdTokenInvalidRequest,
    IdTokenInvalidContentType,
    ErrorIdToken,
    Canceled,
    RpPageNotVisible,
    SilentMediationFailure,
    NotSignedInWithIdp,
    MissingTransientUserActivation,
    ReplacedByActiveMode,
    RelyingPartyOriginIsOpaque,
    TypeNotMatching,
    UiDismissedNoEmbargo,
    CorsError,
    SuppressedBySegmentationPlatform,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FederatedAuthUserInfoRequestIssueDetails {

    pub federatedAuthUserInfoRequestIssueReason: FederatedAuthUserInfoRequestIssueReason,
}

/// Represents the failure reason when a getUserInfo() call fails.
/// Should be updated alongside FederatedAuthUserInfoRequestResult in
/// third_party/blink/public/mojom/devtools/inspector_issue.mojom.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum FederatedAuthUserInfoRequestIssueReason {
    #[default]
    NotSameOrigin,
    NotIframe,
    NotPotentiallyTrustworthy,
    NoApiPermission,
    NotSignedInWithIdp,
    NoAccountSharingPermission,
    InvalidConfigOrWellKnown,
    InvalidAccountsResponse,
    NoReturningUserFromFetchedAccounts,
}

/// This issue tracks client hints related issues. It's used to deprecate old
/// features, encourage the use of new ones, and provide general guidance.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientHintIssueDetails {

    pub sourceCodeLocation: SourceCodeLocation,

    pub clientHintIssueReason: ClientHintIssueReason,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FailedRequestInfo {
    /// The URL that failed to load.

    pub url: String,
    /// The failure message for the failed request.

    pub failureMessage: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestId: Option<crate::network::RequestId>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PartitioningBlobURLInfo {
    #[default]
    BlockedCrossPartitionFetching,
    EnforceNoopenerForNavigation,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PartitioningBlobURLIssueDetails {
    /// The BlobURL that failed to load.

    pub url: String,
    /// Additional information about the Partitioning Blob URL issue.

    pub partitioningBlobURLInfo: PartitioningBlobURLInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ElementAccessibilityIssueReason {
    #[default]
    DisallowedSelectChild,
    DisallowedOptGroupChild,
    NonPhrasingContentOptionChild,
    InteractiveContentOptionChild,
    InteractiveContentLegendChild,
    InteractiveContentSummaryDescendant,
}

/// This issue warns about errors in the select or summary element content model.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ElementAccessibilityIssueDetails {

    pub nodeId: crate::dom::BackendNodeId,

    pub elementAccessibilityIssueReason: ElementAccessibilityIssueReason,

    pub hasDisallowedAttributes: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StyleSheetLoadingIssueReason {
    #[default]
    LateImportRule,
    RequestFailed,
}

/// This issue warns when a referenced stylesheet couldn't be loaded.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StylesheetLoadingIssueDetails {
    /// Source code position that referenced the failing stylesheet.

    pub sourceCodeLocation: SourceCodeLocation,
    /// Reason why the stylesheet couldn't be loaded.

    pub styleSheetLoadingIssueReason: StyleSheetLoadingIssueReason,
    /// Contains additional info when the failure was due to a request.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub failedRequestInfo: Option<FailedRequestInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PropertyRuleIssueReason {
    #[default]
    InvalidSyntax,
    InvalidInitialValue,
    InvalidInherits,
    InvalidName,
}

/// This issue warns about errors in property rules that lead to property
/// registrations being ignored.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyRuleIssueDetails {
    /// Source code position of the property rule.

    pub sourceCodeLocation: SourceCodeLocation,
    /// Reason why the property rule was discarded.

    pub propertyRuleIssueReason: PropertyRuleIssueReason,
    /// The value of the property rule property that failed to parse

    #[serde(skip_serializing_if = "Option::is_none")]
    pub propertyValue: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum UserReidentificationIssueType {
    #[default]
    BlockedFrameNavigation,
    BlockedSubresource,
    NoisedCanvasReadback,
}

/// This issue warns about uses of APIs that may be considered misuse to
/// re-identify users.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserReidentificationIssueDetails {

    #[serde(rename = "type")]
    pub type_: UserReidentificationIssueType,
    /// Applies to BlockedFrameNavigation and BlockedSubresource issue types.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<AffectedRequest>,
    /// Applies to NoisedCanvasReadback issue type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceCodeLocation: Option<SourceCodeLocation>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionElementIssueType {
    #[default]
    InvalidType,
    FencedFrameDisallowed,
    CspFrameAncestorsMissing,
    PermissionsPolicyBlocked,
    PaddingRightUnsupported,
    PaddingBottomUnsupported,
    InsetBoxShadowUnsupported,
    RequestInProgress,
    UntrustedEvent,
    RegistrationFailed,
    TypeNotSupported,
    InvalidTypeActivation,
    SecurityChecksFailed,
    ActivationDisabled,
    GeolocationDeprecated,
    InvalidDisplayStyle,
    NonOpaqueColor,
    LowContrast,
    FontSizeTooSmall,
    FontSizeTooLarge,
    InvalidSizeValue,
}

/// This issue warns about improper usage of the \<permission\> element.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionElementIssueDetails {

    pub issueType: PermissionElementIssueType,
    /// The value of the type attribute.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// The node ID of the \<permission\> element.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeId: Option<crate::dom::BackendNodeId>,
    /// True if the issue is a warning, false if it is an error.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isWarning: Option<bool>,
    /// Fields for message construction:
    /// Used for messages that reference a specific permission name

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissionName: Option<String>,
    /// Used for messages about occlusion

    #[serde(skip_serializing_if = "Option::is_none")]
    pub occluderNodeInfo: Option<String>,
    /// Used for messages about occluder's parent

    #[serde(skip_serializing_if = "Option::is_none")]
    pub occluderParentNodeInfo: Option<String>,
    /// Used for messages about activation disabled reason

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disableReason: Option<String>,
}

/// The issue warns about blocked calls to privacy sensitive APIs via the
/// Selective Permissions Intervention.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectivePermissionsInterventionIssueDetails {
    /// Which API was intervened on.

    pub apiName: String,
    /// Why the ad script using the API is considered an ad.

    pub adAncestry: crate::network::AdAncestry,
    /// The stack trace at the time of the intervention.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackTrace: Option<crate::runtime::StackTrace>,
}

/// A unique identifier for the type of issue. Each type may use one of the
/// optional fields in InspectorIssueDetails to convey more specific
/// information about the kind of issue.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InspectorIssueCode {
    #[default]
    CookieIssue,
    MixedContentIssue,
    BlockedByResponseIssue,
    HeavyAdIssue,
    ContentSecurityPolicyIssue,
    SharedArrayBufferIssue,
    CorsIssue,
    AttributionReportingIssue,
    QuirksModeIssue,
    PartitioningBlobURLIssue,
    NavigatorUserAgentIssue,
    GenericIssue,
    DeprecationIssue,
    ClientHintIssue,
    FederatedAuthRequestIssue,
    BounceTrackingIssue,
    CookieDeprecationMetadataIssue,
    StylesheetLoadingIssue,
    FederatedAuthUserInfoRequestIssue,
    PropertyRuleIssue,
    SharedDictionaryIssue,
    ElementAccessibilityIssue,
    SRIMessageSignatureIssue,
    UnencodedDigestIssue,
    ConnectionAllowlistIssue,
    UserReidentificationIssue,
    PermissionElementIssue,
    PerformanceIssue,
    SelectivePermissionsInterventionIssue,
}

/// This struct holds a list of optional fields with additional information
/// specific to the kind of issue. When adding a new issue code, please also
/// add a new optional field to this type.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectorIssueDetails {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookieIssueDetails: Option<CookieIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixedContentIssueDetails: Option<MixedContentIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockedByResponseIssueDetails: Option<BlockedByResponseIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heavyAdIssueDetails: Option<HeavyAdIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentSecurityPolicyIssueDetails: Option<ContentSecurityPolicyIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharedArrayBufferIssueDetails: Option<SharedArrayBufferIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub corsIssueDetails: Option<CorsIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributionReportingIssueDetails: Option<AttributionReportingIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quirksModeIssueDetails: Option<QuirksModeIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitioningBlobURLIssueDetails: Option<PartitioningBlobURLIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigatorUserAgentIssueDetails: Option<NavigatorUserAgentIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub genericIssueDetails: Option<GenericIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecationIssueDetails: Option<DeprecationIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientHintIssueDetails: Option<ClientHintIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub federatedAuthRequestIssueDetails: Option<FederatedAuthRequestIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounceTrackingIssueDetails: Option<BounceTrackingIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookieDeprecationMetadataIssueDetails: Option<CookieDeprecationMetadataIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stylesheetLoadingIssueDetails: Option<StylesheetLoadingIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub propertyRuleIssueDetails: Option<PropertyRuleIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub federatedAuthUserInfoRequestIssueDetails: Option<FederatedAuthUserInfoRequestIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharedDictionaryIssueDetails: Option<SharedDictionaryIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub elementAccessibilityIssueDetails: Option<ElementAccessibilityIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sriMessageSignatureIssueDetails: Option<SRIMessageSignatureIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unencodedDigestIssueDetails: Option<UnencodedDigestIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectionAllowlistIssueDetails: Option<ConnectionAllowlistIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userReidentificationIssueDetails: Option<UserReidentificationIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissionElementIssueDetails: Option<PermissionElementIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performanceIssueDetails: Option<PerformanceIssueDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectivePermissionsInterventionIssueDetails: Option<SelectivePermissionsInterventionIssueDetails>,
}

/// A unique id for a DevTools inspector issue. Allows other entities (e.g.
/// exceptions, CDP message, console messages, etc.) to reference an issue.

pub type IssueId = String;

/// An inspector issue reported from the back-end.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectorIssue {

    pub code: InspectorIssueCode,

    pub details: InspectorIssueDetails,
    /// A unique id for this issue. May be omitted if no other entity (e.g.
    /// exception, CDP message, etc.) is referencing this issue.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issueId: Option<IssueId>,
}

/// Returns the response body and size if it were re-encoded with the specified settings. Only
/// applies to images.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEncodedResponseParams {
    /// Identifier of the network request to get content for.

    pub requestId: crate::network::RequestId,
    /// The encoding to use.

    pub encoding: String,
    /// The quality of the encoding (0-1). (defaults to 1)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<f64>,
    /// Whether to only return the size information (defaults to false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeOnly: Option<bool>,
}

/// Returns the response body and size if it were re-encoded with the specified settings. Only
/// applies to images.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEncodedResponseReturns {
    /// The encoded body as a base64 string. Omitted if sizeOnly is true. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Size before re-encoding.

    pub originalSize: u64,
    /// Size after re-encoding.

    pub encodedSize: u64,
}

/// Runs the form issues check for the target page. Found issues are reported
/// using Audits.issueAdded event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CheckFormsIssuesReturns {

    pub formIssues: Vec<GenericIssueDetails>,
}
