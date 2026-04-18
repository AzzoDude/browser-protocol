//! Network domain allows tracking network activities of the page. It exposes information about http,
//! file, data and other requests and responses, their headers, bodies, timing, etc.

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Resource type as it was perceived by the rendering engine.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ResourceType {
    #[default]
    Document,
    Stylesheet,
    Image,
    Media,
    Font,
    Script,
    TextTrack,
    XHR,
    Fetch,
    Prefetch,
    EventSource,
    WebSocket,
    Manifest,
    SignedExchange,
    Ping,
    CSPViolationReport,
    Preflight,
    FedCM,
    Other,
}

/// Unique loader identifier.

pub type LoaderId = String;

/// Unique network request identifier.
/// Note that this does not identify individual HTTP requests that are part of
/// a network request.

pub type RequestId = String;

/// Unique intercepted request identifier.

pub type InterceptionId = String;

/// Network level fetch failure reason.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ErrorReason {
    #[default]
    Failed,
    Aborted,
    TimedOut,
    AccessDenied,
    ConnectionClosed,
    ConnectionReset,
    ConnectionRefused,
    ConnectionAborted,
    ConnectionFailed,
    NameNotResolved,
    InternetDisconnected,
    AddressUnreachable,
    BlockedByClient,
    BlockedByResponse,
}

/// UTC time in seconds, counted from January 1, 1970.

pub type TimeSinceEpoch = f64;

/// Monotonically increasing time in seconds since an arbitrary point in the past.

pub type MonotonicTime = f64;

/// Request / response headers as keys / values of JSON object.

pub type Headers = serde_json::Map<String, JsonValue>;

/// The underlying connection technology that the browser is supposedly using.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ConnectionType {
    #[default]
    None,
    Cellular2g,
    Cellular3g,
    Cellular4g,
    Bluetooth,
    Ethernet,
    Wifi,
    Wimax,
    Other,
}

/// Represents the cookie's 'SameSite' status:
/// <https://tools.ietf.org/html/draft-west-first-party-cookies>

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieSameSite {
    #[default]
    Strict,
    Lax,
    None,
}

/// Represents the cookie's 'Priority' status:
/// <https://tools.ietf.org/html/draft-west-cookie-priority-00>

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookiePriority {
    #[default]
    Low,
    Medium,
    High,
}

/// Represents the source scheme of the origin that originally set the cookie.
/// A value of "Unset" allows protocol clients to emulate legacy cookie scope for the scheme.
/// This is a temporary ability and it will be removed in the future.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieSourceScheme {
    #[default]
    Unset,
    NonSecure,
    Secure,
}

/// Timing information for the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTiming {
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime.

    pub requestTime: f64,
    /// Started resolving proxy.

    pub proxyStart: f64,
    /// Finished resolving proxy.

    pub proxyEnd: f64,
    /// Started DNS address resolve.

    pub dnsStart: f64,
    /// Finished DNS address resolve.

    pub dnsEnd: f64,
    /// Started connecting to the remote host.

    pub connectStart: f64,
    /// Connected to the remote host.

    pub connectEnd: f64,
    /// Started SSL handshake.

    pub sslStart: f64,
    /// Finished SSL handshake.

    pub sslEnd: f64,
    /// Started running ServiceWorker.

    pub workerStart: f64,
    /// Finished Starting ServiceWorker.

    pub workerReady: f64,
    /// Started fetch event.

    pub workerFetchStart: f64,
    /// Settled fetch event respondWith promise.

    pub workerRespondWithSettled: f64,
    /// Started ServiceWorker static routing source evaluation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workerRouterEvaluationStart: Option<f64>,
    /// Started cache lookup when the source was evaluated to 'cache'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workerCacheLookupStart: Option<f64>,
    /// Started sending request.

    pub sendStart: f64,
    /// Finished sending request.

    pub sendEnd: f64,
    /// Time the server started pushing request.

    pub pushStart: f64,
    /// Time the server finished pushing request.

    pub pushEnd: f64,
    /// Started receiving response headers.

    pub receiveHeadersStart: f64,
    /// Finished receiving response headers.

    pub receiveHeadersEnd: f64,
}

/// Loading priority of a resource request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ResourcePriority {
    #[default]
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

/// The render-blocking behavior of a resource request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum RenderBlockingBehavior {
    #[default]
    Blocking,
    InBodyParserBlocking,
    NonBlocking,
    NonBlockingDynamic,
    PotentiallyBlocking,
}

/// Post data entry for HTTP request

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostDataEntry {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
}

/// HTTP request data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// Request URL (without fragment).

    pub url: String,
    /// Fragment of the requested URL starting with hash, if present.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlFragment: Option<String>,
    /// HTTP request method.

    pub method: String,
    /// HTTP request headers.

    pub headers: Headers,
    /// HTTP POST request data.
    /// Use postDataEntries instead.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postData: Option<String>,
    /// True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasPostData: Option<bool>,
    /// Request body elements (post data broken into individual entries).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postDataEntries: Option<Vec<PostDataEntry>>,
    /// The mixed content type of the request.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixedContentType: Option<crate::security::MixedContentType>,
    /// Priority of the resource request at the time request is sent.

    pub initialPriority: ResourcePriority,
    /// The referrer policy of the request, as defined in <https://www.w3.org/TR/referrer-policy/>

    pub referrerPolicy: String,
    /// Whether is loaded via link preload.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isLinkPreload: Option<bool>,
    /// Set for requests when the TrustToken API is used. Contains the parameters
    /// passed by the developer (e.g. via "fetch") as understood by the backend.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trustTokenParams: Option<TrustTokenParams>,
    /// True if this resource request is considered to be the 'same site' as the
    /// request corresponding to the main frame.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isSameSite: Option<bool>,
    /// True when the resource request is ad-related.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isAdRelated: Option<bool>,
}

/// Details of a signed certificate timestamp (SCT).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedCertificateTimestamp {
    /// Validation status.

    pub status: String,
    /// Origin.

    pub origin: String,
    /// Log name / description.

    pub logDescription: String,
    /// Log ID.

    pub logId: String,
    /// Issuance date. Unlike TimeSinceEpoch, this contains the number of
    /// milliseconds since January 1, 1970, UTC, not the number of seconds.

    pub timestamp: f64,
    /// Hash algorithm.

    pub hashAlgorithm: String,
    /// Signature algorithm.

    pub signatureAlgorithm: String,
    /// Signature data.

    pub signatureData: String,
}

/// Security details about a request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDetails {
    /// Protocol name (e.g. "TLS 1.2" or "QUIC").

    pub protocol: String,
    /// Key Exchange used by the connection, or the empty string if not applicable.

    pub keyExchange: String,
    /// (EC)DH group used by the connection, if applicable.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyExchangeGroup: Option<String>,
    /// Cipher name.

    pub cipher: String,
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    /// Certificate ID value.

    pub certificateId: crate::security::CertificateId,
    /// Certificate subject name.

    pub subjectName: String,
    /// Subject Alternative Name (SAN) DNS names and IP addresses.

    pub sanList: Vec<String>,
    /// Name of the issuing CA.

    pub issuer: String,
    /// Certificate valid from date.

    pub validFrom: TimeSinceEpoch,
    /// Certificate valid to (expiration) date

    pub validTo: TimeSinceEpoch,
    /// List of signed certificate timestamps (SCTs).

    pub signedCertificateTimestampList: Vec<SignedCertificateTimestamp>,
    /// Whether the request complied with Certificate Transparency policy

    pub certificateTransparencyCompliance: CertificateTransparencyCompliance,
    /// The signature algorithm used by the server in the TLS server signature,
    /// represented as a TLS SignatureScheme code point. Omitted if not
    /// applicable or not known.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverSignatureAlgorithm: Option<i64>,
    /// Whether the connection used Encrypted ClientHello

    pub encryptedClientHello: bool,
}

/// Whether the request complied with Certificate Transparency policy.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CertificateTransparencyCompliance {
    #[default]
    Unknown,
    NotCompliant,
    Compliant,
}

/// The reason why request was blocked.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BlockedReason {
    #[default]
    Other,
    Csp,
    MixedContent,
    Origin,
    Inspector,
    Integrity,
    SubresourceFilter,
    ContentType,
    CoepFrameResourceNeedsCoepHeader,
    CoopSandboxedIframeCannotNavigateToCoopPage,
    CorpNotSameOrigin,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    CorpNotSameSite,
    SriMessageSignatureMismatch,
}

/// The reason why request was blocked.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CorsError {
    #[default]
    DisallowedByMode,
    InvalidResponse,
    WildcardOriginNotAllowed,
    MissingAllowOriginHeader,
    MultipleAllowOriginValues,
    InvalidAllowOriginValue,
    AllowOriginMismatch,
    InvalidAllowCredentials,
    CorsDisabledScheme,
    PreflightInvalidStatus,
    PreflightDisallowedRedirect,
    PreflightWildcardOriginNotAllowed,
    PreflightMissingAllowOriginHeader,
    PreflightMultipleAllowOriginValues,
    PreflightInvalidAllowOriginValue,
    PreflightAllowOriginMismatch,
    PreflightInvalidAllowCredentials,
    PreflightMissingAllowExternal,
    PreflightInvalidAllowExternal,
    InvalidAllowMethodsPreflightResponse,
    InvalidAllowHeadersPreflightResponse,
    MethodDisallowedByPreflightResponse,
    HeaderDisallowedByPreflightResponse,
    RedirectContainsCredentials,
    InsecureLocalNetwork,
    InvalidLocalNetworkAccess,
    NoCorsRedirectModeNotFollow,
    LocalNetworkAccessPermissionDenied,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CorsErrorStatus {

    pub corsError: CorsError,

    pub failedParameter: String,
}

/// Source of serviceworker response.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceWorkerResponseSource {
    #[default]
    CacheStorage,
    HttpCache,
    FallbackCode,
    Network,
}

/// Determines what type of Trust Token operation is executed and
/// depending on the type, some additional parameters. The values
/// are specified in third_party/blink/renderer/core/fetch/trust_token.idl.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokenParams {

    pub operation: TrustTokenOperationType,
    /// Only set for "token-redemption" operation and determine whether
    /// to request a fresh SRR or use a still valid cached SRR.

    pub refreshPolicy: String,
    /// Origins of issuers from whom to request tokens or redemption
    /// records.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuers: Option<Vec<String>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TrustTokenOperationType {
    #[default]
    Issuance,
    Redemption,
    Signing,
}

/// The reason why Chrome uses a specific transport protocol for HTTP semantics.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AlternateProtocolUsage {
    #[default]
    AlternativeJobWonWithoutRace,
    AlternativeJobWonRace,
    MainJobWonRace,
    MappingMissing,
    Broken,
    DnsAlpnH3JobWonWithoutRace,
    DnsAlpnH3JobWonRace,
    UnspecifiedReason,
}

/// Source of service worker router.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceWorkerRouterSource {
    #[default]
    Network,
    Cache,
    FetchEvent,
    RaceNetworkAndFetchHandler,
    RaceNetworkAndCache,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRouterInfo {
    /// ID of the rule matched. If there is a matched rule, this field will
    /// be set, otherwiser no value will be set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleIdMatched: Option<u64>,
    /// The router source of the matched rule. If there is a matched rule, this
    /// field will be set, otherwise no value will be set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchedSourceType: Option<ServiceWorkerRouterSource>,
    /// The actual router source used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actualSourceType: Option<ServiceWorkerRouterSource>,
}

/// HTTP response data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Response URL. This URL can be different from CachedResource.url in case of redirect.

    pub url: String,
    /// HTTP response status code.

    pub status: i64,
    /// HTTP response status text.

    pub statusText: String,
    /// HTTP response headers.

    pub headers: Headers,
    /// HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headersText: Option<String>,
    /// Resource mimeType as determined by the browser.

    pub mimeType: String,
    /// Resource charset as determined by the browser (if applicable).

    pub charset: String,
    /// Refined HTTP request headers that were actually transmitted over the network.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestHeaders: Option<Headers>,
    /// HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestHeadersText: Option<String>,
    /// Specifies whether physical connection was actually reused for this request.

    pub connectionReused: bool,
    /// Physical connection id that was actually used for this request.

    pub connectionId: f64,
    /// Remote IP address.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remoteIPAddress: Option<String>,
    /// Remote port.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remotePort: Option<i64>,
    /// Specifies that the request was served from the disk cache.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromDiskCache: Option<bool>,
    /// Specifies that the request was served from the ServiceWorker.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromServiceWorker: Option<bool>,
    /// Specifies that the request was served from the prefetch cache.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromPrefetchCache: Option<bool>,
    /// Specifies that the request was served from the prefetch cache.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromEarlyHints: Option<bool>,
    /// Information about how ServiceWorker Static Router API was used. If this
    /// field is set with 'matchedSourceType' field, a matching rule is found.
    /// If this field is set without 'matchedSource', no matching rule is found.
    /// Otherwise, the API is not used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serviceWorkerRouterInfo: Option<ServiceWorkerRouterInfo>,
    /// Total number of bytes received for this request so far.

    pub encodedDataLength: f64,
    /// Timing information for the given request.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<ResourceTiming>,
    /// Response source of response from ServiceWorker.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serviceWorkerResponseSource: Option<ServiceWorkerResponseSource>,
    /// The time at which the returned response was generated.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseTime: Option<TimeSinceEpoch>,
    /// Cache Storage Cache Name.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cacheStorageCacheName: Option<String>,
    /// Protocol used to fetch this request.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// The reason why Chrome uses a specific transport protocol for HTTP semantics.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternateProtocolUsage: Option<AlternateProtocolUsage>,
    /// Security state of the request resource.

    pub securityState: crate::security::SecurityState,
    /// Security details for the request.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityDetails: Option<SecurityDetails>,
}

/// WebSocket request data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketRequest {
    /// HTTP request headers.

    pub headers: Headers,
}

/// WebSocket response data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketResponse {
    /// HTTP response status code.

    pub status: i64,
    /// HTTP response status text.

    pub statusText: String,
    /// HTTP response headers.

    pub headers: Headers,
    /// HTTP response headers text.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headersText: Option<String>,
    /// HTTP request headers.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestHeaders: Option<Headers>,
    /// HTTP request headers text.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestHeadersText: Option<String>,
}

/// WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrame {
    /// WebSocket message opcode.

    pub opcode: f64,
    /// WebSocket message mask.

    pub mask: bool,
    /// WebSocket message payload data.
    /// If the opcode is 1, this is a text message and payloadData is a UTF-8 string.
    /// If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data.

    pub payloadData: String,
}

/// Information about the cached resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CachedResource {
    /// Resource URL. This is the url of the original network request.

    pub url: String,
    /// Type of this resource.

    #[serde(rename = "type")]
    pub type_: ResourceType,
    /// Cached response data.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Response>,
    /// Cached response body size.

    pub bodySize: f64,
}

/// Information about the request initiator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Initiator {
    /// Type of this initiator.

    #[serde(rename = "type")]
    pub type_: String,
    /// Initiator JavaScript stack trace, set for Script only.
    /// Requires the Debugger domain to be enabled.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<crate::runtime::StackTrace>,
    /// Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Initiator line number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineNumber: Option<f64>,
    /// Initiator column number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columnNumber: Option<f64>,
    /// Set if another request triggered this request (e.g. preflight).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestId: Option<RequestId>,
}

/// cookiePartitionKey object
/// The representation of the components of the key that are created by the cookiePartitionKey class contained in net/cookies/cookie_partition_key.h.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookiePartitionKey {
    /// The site of the top-level URL the browser was visiting at the start
    /// of the request to the endpoint that set the cookie.

    pub topLevelSite: String,
    /// Indicates if the cookie has any ancestors that are cross-site to the topLevelSite.

    pub hasCrossSiteAncestor: bool,
}

/// Cookie object

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    /// Cookie name.

    pub name: String,
    /// Cookie value.

    pub value: String,
    /// Cookie domain.

    pub domain: String,
    /// Cookie path.

    pub path: String,
    /// Cookie expiration date as the number of seconds since the UNIX epoch.
    /// The value is set to -1 if the expiry date is not set.
    /// The value can be null for values that cannot be represented in
    /// JSON (±Inf).

    pub expires: f64,
    /// Cookie size.

    pub size: u64,
    /// True if cookie is http-only.

    pub httpOnly: bool,
    /// True if cookie is secure.

    pub secure: bool,
    /// True in case of session cookie.

    pub session: bool,
    /// Cookie SameSite type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sameSite: Option<CookieSameSite>,
    /// Cookie Priority

    pub priority: CookiePriority,
    /// Cookie source scheme type.

    pub sourceScheme: CookieSourceScheme,
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.

    pub sourcePort: i64,
    /// Cookie partition key.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitionKey: Option<CookiePartitionKey>,
    /// True if cookie partition key is opaque.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitionKeyOpaque: Option<bool>,
}

/// Types of reasons why a cookie may not be stored from a response.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SetCookieBlockedReason {
    #[default]
    SecureOnly,
    SameSiteStrict,
    SameSiteLax,
    SameSiteUnspecifiedTreatedAsLax,
    SameSiteNoneInsecure,
    UserPreferences,
    ThirdPartyPhaseout,
    ThirdPartyBlockedInFirstPartySet,
    SyntaxError,
    SchemeNotSupported,
    OverwriteSecure,
    InvalidDomain,
    InvalidPrefix,
    UnknownError,
    SchemefulSameSiteStrict,
    SchemefulSameSiteLax,
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    NameValuePairExceedsMaxSize,
    DisallowedCharacter,
    NoCookieContent,
}

/// Types of reasons why a cookie may not be sent with a request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieBlockedReason {
    #[default]
    SecureOnly,
    NotOnPath,
    DomainMismatch,
    SameSiteStrict,
    SameSiteLax,
    SameSiteUnspecifiedTreatedAsLax,
    SameSiteNoneInsecure,
    UserPreferences,
    ThirdPartyPhaseout,
    ThirdPartyBlockedInFirstPartySet,
    UnknownError,
    SchemefulSameSiteStrict,
    SchemefulSameSiteLax,
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    NameValuePairExceedsMaxSize,
    PortMismatch,
    SchemeMismatch,
    AnonymousContext,
}

/// Types of reasons why a cookie should have been blocked by 3PCD but is exempted for the request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieExemptionReason {
    #[default]
    None,
    UserSetting,
    TPCDMetadata,
    TPCDDeprecationTrial,
    TopLevelTPCDDeprecationTrial,
    TPCDHeuristics,
    EnterprisePolicy,
    StorageAccess,
    TopLevelStorageAccess,
    Scheme,
    SameSiteNoneCookiesInSandbox,
}

/// A cookie which was not stored from a response with the corresponding reason.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockedSetCookieWithReason {
    /// The reason(s) this cookie was blocked.

    pub blockedReasons: Vec<SetCookieBlockedReason>,
    /// The string representing this individual cookie as it would appear in the header.
    /// This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.

    pub cookieLine: String,
    /// The cookie object which represents the cookie which was not stored. It is optional because
    /// sometimes complete cookie information is not available, such as in the case of parsing
    /// errors.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<Cookie>,
}

/// A cookie should have been blocked by 3PCD but is exempted and stored from a response with the
/// corresponding reason. A cookie could only have at most one exemption reason.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExemptedSetCookieWithReason {
    /// The reason the cookie was exempted.

    pub exemptionReason: CookieExemptionReason,
    /// The string representing this individual cookie as it would appear in the header.

    pub cookieLine: String,
    /// The cookie object representing the cookie.

    pub cookie: Cookie,
}

/// A cookie associated with the request which may or may not be sent with it.
/// Includes the cookies itself and reasons for blocking or exemption.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedCookie {
    /// The cookie object representing the cookie which was not sent.

    pub cookie: Cookie,
    /// The reason(s) the cookie was blocked. If empty means the cookie is included.

    pub blockedReasons: Vec<CookieBlockedReason>,
    /// The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could
    /// only have at most one exemption reason.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemptionReason: Option<CookieExemptionReason>,
}

/// Cookie parameter object

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookieParam {
    /// Cookie name.

    pub name: String,
    /// Cookie value.

    pub value: String,
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Cookie domain.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Cookie path.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// True if cookie is secure.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// True if cookie is http-only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub httpOnly: Option<bool>,
    /// Cookie SameSite type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sameSite: Option<CookieSameSite>,
    /// Cookie expiration date, session cookie if not set

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<TimeSinceEpoch>,
    /// Cookie Priority.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<CookiePriority>,
    /// Cookie source scheme type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceScheme: Option<CookieSourceScheme>,
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourcePort: Option<i64>,
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitionKey: Option<CookiePartitionKey>,
}

/// Authorization challenge for HTTP status code 401 or 407.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallenge {
    /// Source of the authentication challenge.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Origin of the challenger.

    pub origin: String,
    /// The authentication scheme used, such as basic or digest

    pub scheme: String,
    /// The realm of the challenge. May be empty.

    pub realm: String,
}

/// Response to an AuthChallenge.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallengeResponse {
    /// The decision on what to do in response to the authorization challenge.  Default means
    /// deferring to the default behavior of the net stack, which will likely either the Cancel
    /// authentication or display a popup dialog box.

    pub response: String,
    /// The username to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The password to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Stages of the interception to begin intercepting. Request will intercept before the request is
/// sent. Response will intercept after the response is received.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InterceptionStage {
    #[default]
    Request,
    HeadersReceived,
}

/// Request pattern for interception.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestPattern {
    /// Wildcards (''*'' -\> zero or more, ''?'' -\> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlPattern: Option<String>,
    /// If set, only requests for matching resource types will be intercepted.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resourceType: Option<ResourceType>,
    /// Stage at which to begin intercepting requests. Default is Request.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interceptionStage: Option<InterceptionStage>,
}

/// Information about a signed exchange signature.
/// <https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeSignature {
    /// Signed exchange signature label.

    pub label: String,
    /// The hex string of signed exchange signature.

    pub signature: String,
    /// Signed exchange signature integrity.

    pub integrity: String,
    /// Signed exchange signature cert Url.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certUrl: Option<String>,
    /// The hex string of signed exchange signature cert sha256.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certSha256: Option<String>,
    /// Signed exchange signature validity Url.

    pub validityUrl: String,
    /// Signed exchange signature date.

    pub date: i64,
    /// Signed exchange signature expires.

    pub expires: i64,
    /// The encoded certificates.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<String>>,
}

/// Information about a signed exchange header.
/// <https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeHeader {
    /// Signed exchange request URL.

    pub requestUrl: String,
    /// Signed exchange response code.

    pub responseCode: i64,
    /// Signed exchange response headers.

    pub responseHeaders: Headers,
    /// Signed exchange response signature.

    pub signatures: Vec<SignedExchangeSignature>,
    /// Signed exchange header integrity hash in the form of 'sha256-\<base64-hash-value\>'.

    pub headerIntegrity: String,
}

/// Field type for a signed exchange related error.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SignedExchangeErrorField {
    #[default]
    SignatureSig,
    SignatureIntegrity,
    SignatureCertUrl,
    SignatureCertSha256,
    SignatureValidityUrl,
    SignatureTimestamps,
}

/// Information about a signed exchange response.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeError {
    /// Error message.

    pub message: String,
    /// The index of the signature which caused the error.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatureIndex: Option<u64>,
    /// The field which caused the error.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorField: Option<SignedExchangeErrorField>,
}

/// Information about a signed exchange response.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeInfo {
    /// The outer response of signed HTTP exchange which was received from network.

    pub outerResponse: Response,
    /// Whether network response for the signed exchange was accompanied by
    /// extra headers.

    pub hasExtraInfo: bool,
    /// Information about the signed exchange header.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<SignedExchangeHeader>,
    /// Security details for the signed exchange header.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityDetails: Option<SecurityDetails>,
    /// Errors occurred while handling the signed exchange.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<SignedExchangeError>>,
}

/// List of content encodings supported by the backend.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContentEncoding {
    #[default]
    Deflate,
    Gzip,
    Br,
    Zstd,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConditions {
    /// Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string
    /// syntax (<https://urlpattern.spec.whatwg.org/>) and must be absolute. If the pattern is empty, all requests are
    /// matched (including p2p connections).

    pub urlPattern: String,
    /// Minimum latency from request sent to response headers received (ms).

    pub latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.

    pub downloadThroughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.

    pub uploadThroughput: f64,
    /// Connection type if known.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectionType: Option<ConnectionType>,
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packetLoss: Option<f64>,
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packetQueueLength: Option<u64>,
    /// WebRTC packetReordering feature.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packetReordering: Option<bool>,
    /// True to emulate internet disconnection.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockPattern {
    /// URL pattern to match. Patterns use the URLPattern constructor string syntax
    /// (<https://urlpattern.spec.whatwg.org/>) and must be absolute. Example: '*://*:*/*.css'.

    pub urlPattern: String,
    /// Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later
    /// 'BlockPattern'.

    pub block: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DirectSocketDnsQueryType {
    #[default]
    Ipv4,
    Ipv6,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectTCPSocketOptions {
    /// TCP_NODELAY option

    pub noDelay: bool,
    /// Expected to be unsigned integer.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepAliveDelay: Option<f64>,
    /// Expected to be unsigned integer.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sendBufferSize: Option<f64>,
    /// Expected to be unsigned integer.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiveBufferSize: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnsQueryType: Option<DirectSocketDnsQueryType>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectUDPSocketOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remoteAddr: Option<String>,
    /// Unsigned int 16.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remotePort: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub localAddr: Option<String>,
    /// Unsigned int 16.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub localPort: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnsQueryType: Option<DirectSocketDnsQueryType>,
    /// Expected to be unsigned integer.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sendBufferSize: Option<f64>,
    /// Expected to be unsigned integer.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiveBufferSize: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicastLoopback: Option<bool>,
    /// Unsigned int 8.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicastTimeToLive: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicastAllowAddressSharing: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectUDPMessage {

    pub data: String,
    /// Null for connected mode.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remoteAddr: Option<String>,
    /// Null for connected mode.
    /// Expected to be unsigned integer.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remotePort: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum LocalNetworkAccessRequestPolicy {
    #[default]
    Allow,
    BlockFromInsecureToMorePrivate,
    WarnFromInsecureToMorePrivate,
    PermissionBlock,
    PermissionWarn,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum IPAddressSpace {
    #[default]
    Loopback,
    Local,
    Public,
    Unknown,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectTiming {
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for
    /// the same request (but not for redirected requests).

    pub requestTime: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientSecurityState {

    pub initiatorIsSecureContext: bool,

    pub initiatorIPAddressSpace: IPAddressSpace,

    pub localNetworkAccessRequestPolicy: LocalNetworkAccessRequestPolicy,
}

/// Identifies the script on the stack that caused a resource or element to be
/// labeled as an ad. For resources, this indicates the context that triggered
/// the fetch. For elements, this indicates the context that caused the element
/// to be appended to the DOM.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdScriptIdentifier {
    /// The script's V8 identifier.

    pub scriptId: crate::runtime::ScriptId,
    /// V8's debugging ID for the v8::Context.

    pub debuggerId: crate::runtime::UniqueDebuggerId,
    /// The script's url (or generated name based on id if inline script).

    pub name: String,
}

/// Encapsulates the script ancestry and the root script filter list rule that
/// caused the resource or element to be labeled as an ad.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdAncestry {
    /// A chain of 'AdScriptIdentifier's representing the ancestry of an ad
    /// script that led to the creation of a resource or element. The chain is
    /// ordered from the script itself (lowest level) up to its root ancestor
    /// that was flagged by a filter list.

    pub ancestryChain: Vec<AdScriptIdentifier>,
    /// The filter list rule that caused the root (last) script in
    /// 'ancestryChain' to be tagged as an ad.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootScriptFilterlistRule: Option<String>,
}

/// Represents the provenance of an ad resource or element. Only one of
/// 'filterlistRule' or 'adScriptAncestry' can be set. If 'filterlistRule'
/// is provided, the resource URL directly matches a filter list rule. If
/// 'adScriptAncestry' is provided, an ad script initiated the resource fetch or
/// appended the element to the DOM. If neither is provided, the entity is
/// known to be an ad, but provenance tracking information is unavailable.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdProvenance {
    /// The filterlist rule that matched, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filterlistRule: Option<String>,
    /// The script ancestry that created the ad, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adScriptAncestry: Option<AdAncestry>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CrossOriginOpenerPolicyValue {
    #[default]
    SameOrigin,
    SameOriginAllowPopups,
    RestrictProperties,
    UnsafeNone,
    SameOriginPlusCoep,
    RestrictPropertiesPlusCoep,
    NoopenerAllowPopups,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginOpenerPolicyStatus {

    pub value: CrossOriginOpenerPolicyValue,

    pub reportOnlyValue: CrossOriginOpenerPolicyValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportingEndpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportOnlyReportingEndpoint: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CrossOriginEmbedderPolicyValue {
    #[default]
    None,
    Credentialless,
    RequireCorp,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginEmbedderPolicyStatus {

    pub value: CrossOriginEmbedderPolicyValue,

    pub reportOnlyValue: CrossOriginEmbedderPolicyValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportingEndpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportOnlyReportingEndpoint: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContentSecurityPolicySource {
    #[default]
    HTTP,
    Meta,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyStatus {

    pub effectiveDirectives: String,

    pub isEnforced: bool,

    pub source: ContentSecurityPolicySource,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityIsolationStatus {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coop: Option<CrossOriginOpenerPolicyStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coep: Option<CrossOriginEmbedderPolicyStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub csp: Option<Vec<ContentSecurityPolicyStatus>>,
}

/// The status of a Reporting API report.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ReportStatus {
    #[default]
    Queued,
    Pending,
    MarkedForRemoval,
    Success,
}


pub type ReportId = String;

/// An object representing a report generated by the Reporting API.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportingApiReport {

    pub id: ReportId,
    /// The URL of the document that triggered the report.

    pub initiatorUrl: String,
    /// The name of the endpoint group that should be used to deliver the report.

    pub destination: String,
    /// The type of the report (specifies the set of data that is contained in the report body).

    #[serde(rename = "type")]
    pub type_: String,
    /// When the report was generated.

    pub timestamp: crate::network::TimeSinceEpoch,
    /// How many uploads deep the related request was.

    pub depth: i64,
    /// The number of delivery attempts made so far, not including an active attempt.

    pub completedAttempts: i64,

    pub body: serde_json::Map<String, JsonValue>,

    pub status: ReportStatus,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportingApiEndpoint {
    /// The URL of the endpoint to which reports may be delivered.

    pub url: String,
    /// Name of the endpoint group.

    pub groupName: String,
}

/// Unique identifier for a device bound session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionKey {
    /// The site the session is set up for.

    pub site: String,
    /// The id of the session.

    pub id: String,
}

/// How a device bound session was used during a request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionWithUsage {
    /// The key for the session.

    pub sessionKey: DeviceBoundSessionKey,
    /// How the session was used (or not used).

    pub usage: String,
}

/// A device bound session's cookie craving.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionCookieCraving {
    /// The name of the craving.

    pub name: String,
    /// The domain of the craving.

    pub domain: String,
    /// The path of the craving.

    pub path: String,
    /// The 'Secure' attribute of the craving attributes.

    pub secure: bool,
    /// The 'HttpOnly' attribute of the craving attributes.

    pub httpOnly: bool,
    /// The 'SameSite' attribute of the craving attributes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sameSite: Option<CookieSameSite>,
}

/// A device bound session's inclusion URL rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionUrlRule {
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type'.

    pub ruleType: String,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern'.

    pub hostPattern: String,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix'.

    pub pathPrefix: String,
}

/// A device bound session's inclusion rules.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionInclusionRules {
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::origin_'.

    pub origin: String,
    /// Whether the whole site is included. See comments on
    /// 'net::device_bound_sessions::SessionInclusionRules::include_site_' for more
    /// details; this boolean is true if that value is populated.

    pub includeSite: bool,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::url_rules_'.

    pub urlRules: Vec<DeviceBoundSessionUrlRule>,
}

/// A device bound session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSession {
    /// The site and session ID of the session.

    pub key: DeviceBoundSessionKey,
    /// See comments on 'net::device_bound_sessions::Session::refresh_url_'.

    pub refreshUrl: String,
    /// See comments on 'net::device_bound_sessions::Session::inclusion_rules_'.

    pub inclusionRules: DeviceBoundSessionInclusionRules,
    /// See comments on 'net::device_bound_sessions::Session::cookie_cravings_'.

    pub cookieCravings: Vec<DeviceBoundSessionCookieCraving>,
    /// See comments on 'net::device_bound_sessions::Session::expiry_date_'.

    pub expiryDate: crate::network::TimeSinceEpoch,
    /// See comments on 'net::device_bound_sessions::Session::cached_challenge__'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cachedChallenge: Option<String>,
    /// See comments on 'net::device_bound_sessions::Session::allowed_refresh_initiators_'.

    pub allowedRefreshInitiators: Vec<String>,
}

/// A unique identifier for a device bound session event.

pub type DeviceBoundSessionEventId = String;

/// A fetch result for a device bound session creation or refresh.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DeviceBoundSessionFetchResult {
    #[default]
    Success,
    KeyError,
    SigningError,
    ServerRequestedTermination,
    InvalidSessionId,
    InvalidChallenge,
    TooManyChallenges,
    InvalidFetcherUrl,
    InvalidRefreshUrl,
    TransientHttpError,
    ScopeOriginSameSiteMismatch,
    RefreshUrlSameSiteMismatch,
    MismatchedSessionId,
    MissingScope,
    NoCredentials,
    SubdomainRegistrationWellKnownUnavailable,
    SubdomainRegistrationUnauthorized,
    SubdomainRegistrationWellKnownMalformed,
    SessionProviderWellKnownUnavailable,
    RelyingPartyWellKnownUnavailable,
    FederatedKeyThumbprintMismatch,
    InvalidFederatedSessionUrl,
    InvalidFederatedKey,
    TooManyRelyingOriginLabels,
    BoundCookieSetForbidden,
    NetError,
    ProxyError,
    EmptySessionConfig,
    InvalidCredentialsConfig,
    InvalidCredentialsType,
    InvalidCredentialsEmptyName,
    InvalidCredentialsCookie,
    PersistentHttpError,
    RegistrationAttemptedChallenge,
    InvalidScopeOrigin,
    ScopeOriginContainsPath,
    RefreshInitiatorNotString,
    RefreshInitiatorInvalidHostPattern,
    InvalidScopeSpecification,
    MissingScopeSpecificationType,
    EmptyScopeSpecificationDomain,
    EmptyScopeSpecificationPath,
    InvalidScopeSpecificationType,
    InvalidScopeIncludeSite,
    MissingScopeIncludeSite,
    FederatedNotAuthorizedByProvider,
    FederatedNotAuthorizedByRelyingParty,
    SessionProviderWellKnownMalformed,
    SessionProviderWellKnownHasProviderOrigin,
    RelyingPartyWellKnownMalformed,
    RelyingPartyWellKnownHasRelyingOrigins,
    InvalidFederatedSessionProviderSessionMissing,
    InvalidFederatedSessionWrongProviderOrigin,
    InvalidCredentialsCookieCreationTime,
    InvalidCredentialsCookieName,
    InvalidCredentialsCookieParsing,
    InvalidCredentialsCookieUnpermittedAttribute,
    InvalidCredentialsCookieInvalidDomain,
    InvalidCredentialsCookiePrefix,
    InvalidScopeRulePath,
    InvalidScopeRuleHostPattern,
    ScopeRuleOriginScopedHostPatternMismatch,
    ScopeRuleSiteScopedHostPatternMismatch,
    SigningQuotaExceeded,
    InvalidConfigJson,
    InvalidFederatedSessionProviderFailedToRestoreKey,
    FailedToUnwrapKey,
    SessionDeletedDuringRefresh,
}

/// Details about a failed device bound session network request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionFailedRequest {
    /// The failed request URL.

    pub requestUrl: String,
    /// The net error of the response if it was not OK.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub netError: Option<String>,
    /// The response code if the net error was OK and the response code was not
    /// 200.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseError: Option<i64>,
    /// The body of the response if the net error was OK, the response code was
    /// not 200, and the response body was not empty.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseErrorBody: Option<String>,
}

/// Session event details specific to creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreationEventDetails {
    /// The result of the fetch attempt.

    pub fetchResult: DeviceBoundSessionFetchResult,
    /// The session if there was a newly created session. This is populated for
    /// all successful creation events.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub newSession: Option<DeviceBoundSession>,
    /// Details about a failed device bound session network request if there was
    /// one.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub failedRequest: Option<DeviceBoundSessionFailedRequest>,
}

/// Session event details specific to refresh.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RefreshEventDetails {
    /// The result of a refresh.

    pub refreshResult: String,
    /// If there was a fetch attempt, the result of that.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetchResult: Option<DeviceBoundSessionFetchResult>,
    /// The session display if there was a newly created session. This is populated
    /// for any refresh event that modifies the session config.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub newSession: Option<DeviceBoundSession>,
    /// See comments on 'net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh'.

    pub wasFullyProactiveRefresh: bool,
    /// Details about a failed device bound session network request if there was
    /// one.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub failedRequest: Option<DeviceBoundSessionFailedRequest>,
}

/// Session event details specific to termination.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TerminationEventDetails {
    /// The reason for a session being deleted.

    pub deletionReason: String,
}

/// Session event details specific to challenges.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeEventDetails {
    /// The result of a challenge.

    pub challengeResult: String,
    /// The challenge set.

    pub challenge: String,
}

/// An object providing the result of a network resource load.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourcePageResult {

    pub success: bool,
    /// Optional values used for error reporting.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub netError: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub netErrorName: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub httpStatusCode: Option<f64>,
    /// If successful, one of the following two fields holds the result.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<crate::io::StreamHandle>,
    /// Response headers.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<crate::network::Headers>,
}

/// An options object that may be extended later to better support CORS,
/// CORB and streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceOptions {

    pub disableCache: bool,

    pub includeCredentials: bool,
}

/// Sets a list of content encodings that will be accepted. Empty list means no encoding is accepted.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAcceptedEncodingsParams {
    /// List of accepted content encodings.

    pub encodings: Vec<ContentEncoding>,
}

/// Tells whether clearing browser cache is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCacheReturns {
    /// True if browser cache can be cleared.

    pub result: bool,
}

/// Tells whether clearing browser cookies is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCookiesReturns {
    /// True if browser cookies can be cleared.

    pub result: bool,
}

/// Tells whether emulation of network conditions is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulateNetworkConditionsReturns {
    /// True if emulation of network conditions is supported.

    pub result: bool,
}

/// Response to Network.requestIntercepted which either modifies the request to continue with any
/// modifications, or blocks it, or completes it with the provided response bytes. If a network
/// fetch occurs as a result which encounters a redirect an additional Network.requestIntercepted
/// event will be sent with the same InterceptionId.
/// Deprecated, use Fetch.continueRequest, Fetch.fulfillRequest and Fetch.failRequest instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueInterceptedRequestParams {

    pub interceptionId: InterceptionId,
    /// If set this causes the request to fail with the given reason. Passing 'Aborted' for requests
    /// marked with 'isNavigationRequest' also cancels the navigation. Must not be set in response
    /// to an authChallenge.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorReason: Option<ErrorReason>,
    /// If set the requests completes using with the provided base64 encoded raw response, including
    /// HTTP status line and headers etc... Must not be set in response to an authChallenge. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rawResponse: Option<String>,
    /// If set the request url will be modified in a way that's not observable by page. Must not be
    /// set in response to an authChallenge.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// If set this allows the request method to be overridden. Must not be set in response to an
    /// authChallenge.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// If set this allows postData to be set. Must not be set in response to an authChallenge.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postData: Option<String>,
    /// If set this allows the request headers to be changed. Must not be set in response to an
    /// authChallenge.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
    /// Response to a requestIntercepted with an authChallenge. Must not be set otherwise.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authChallengeResponse: Option<AuthChallengeResponse>,
}

/// Deletes browser cookies with matching name and url or domain/path/partitionKey pair.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCookiesParams {
    /// Name of the cookies to remove.

    pub name: String,
    /// If specified, deletes all the cookies with the given name where domain and path match
    /// provided URL.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// If specified, deletes only cookies with the exact domain.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// If specified, deletes only cookies with the exact path.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// If specified, deletes only cookies with the the given name and partitionKey where
    /// all partition key attributes match the cookie partition key attribute.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitionKey: Option<CookiePartitionKey>,
}

/// Activates emulation of network conditions. This command is deprecated in favor of the emulateNetworkConditionsByRule
/// and overrideNetworkState commands, which can be used together to the same effect.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsParams {
    /// True to emulate internet disconnection.

    pub offline: bool,
    /// Minimum latency from request sent to response headers received (ms).

    pub latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.

    pub downloadThroughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.

    pub uploadThroughput: f64,
    /// Connection type if known.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectionType: Option<ConnectionType>,
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packetLoss: Option<f64>,
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packetQueueLength: Option<u64>,
    /// WebRTC packetReordering feature.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packetReordering: Option<bool>,
}

/// Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated
/// Network.emulateNetworkConditions this method does not affect 'navigator' state. Use Network.overrideNetworkState to
/// explicitly modify 'navigator' behavior.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsByRuleParams {
    /// True to emulate internet disconnection. Deprecated, use the offline property in matchedNetworkConditions
    /// or emulateOfflineServiceWorker instead.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    /// True to emulate offline service worker.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulateOfflineServiceWorker: Option<bool>,
    /// Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global
    /// conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are
    /// also applied for throttling of p2p connections.

    pub matchedNetworkConditions: Vec<NetworkConditions>,
}

/// Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated
/// Network.emulateNetworkConditions this method does not affect 'navigator' state. Use Network.overrideNetworkState to
/// explicitly modify 'navigator' behavior.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsByRuleReturns {
    /// An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for
    /// requests affected by a rule.

    pub ruleIds: Vec<String>,
}

/// Override the state of navigator.onLine and navigator.connection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OverrideNetworkStateParams {
    /// True to emulate internet disconnection.

    pub offline: bool,
    /// Minimum latency from request sent to response headers received (ms).

    pub latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.

    pub downloadThroughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.

    pub uploadThroughput: f64,
    /// Connection type if known.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectionType: Option<ConnectionType>,
}

/// Enables network tracking, network events will now be delivered to the client.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    /// This is the maximum number of bytes that will be collected by this
    /// DevTools session.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxTotalBufferSize: Option<u64>,
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxResourceBufferSize: Option<u64>,
    /// Longest post body size (in bytes) that would be included in requestWillBeSent notification

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxPostDataSize: Option<u64>,
    /// Whether DirectSocket chunk send/receive events should be reported.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportDirectSocketTraffic: Option<bool>,
    /// Enable storing response bodies outside of renderer, so that these survive
    /// a cross-process navigation. Requires maxTotalBufferSize to be set.
    /// Currently defaults to false. This field is being deprecated in favor of the dedicated
    /// configureDurableMessages command, due to the possibility of deadlocks when awaiting
    /// Network.enable before issuing Runtime.runIfWaitingForDebugger.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enableDurableMessages: Option<bool>,
}

/// Configures storing response bodies outside of renderer, so that these survive
/// a cross-process navigation.
/// If maxTotalBufferSize is not set, durable messages are disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureDurableMessagesParams {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxTotalBufferSize: Option<u64>,
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxResourceBufferSize: Option<u64>,
}

/// Returns all browser cookies. Depending on the backend support, will return detailed cookie
/// information in the 'cookies' field.
/// Deprecated. Use Storage.getCookies instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllCookiesReturns {
    /// Array of cookie objects.

    pub cookies: Vec<Cookie>,
}

/// Returns the DER-encoded certificate.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateParams {
    /// Origin to get certificate for.

    pub origin: String,
}

/// Returns the DER-encoded certificate.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateReturns {

    pub tableNames: Vec<String>,
}

/// Returns all browser cookies for the current URL. Depending on the backend support, will return
/// detailed cookie information in the 'cookies' field.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesParams {
    /// The list of URLs for which applicable cookies will be fetched.
    /// If not specified, it's assumed to be set to the list containing
    /// the URLs of the page and all of its subframes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// Returns all browser cookies for the current URL. Depending on the backend support, will return
/// detailed cookie information in the 'cookies' field.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesReturns {
    /// Array of cookie objects.

    pub cookies: Vec<Cookie>,
}

/// Returns content served for the given request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyParams {
    /// Identifier of the network request to get content for.

    pub requestId: RequestId,
}

/// Returns content served for the given request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyReturns {
    /// Response body.

    pub body: String,
    /// True, if content was sent as base64.

    pub base64Encoded: bool,
}

/// Returns post data sent with the request. Returns an error when no data was sent with the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRequestPostDataParams {
    /// Identifier of the network request to get content for.

    pub requestId: RequestId,
}

/// Returns post data sent with the request. Returns an error when no data was sent with the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRequestPostDataReturns {
    /// Request body string, omitting files from multipart requests

    pub postData: String,
    /// True, if content was sent as base64.

    pub base64Encoded: bool,
}

/// Returns content served for the given currently intercepted request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyForInterceptionParams {
    /// Identifier for the intercepted request to get body for.

    pub interceptionId: InterceptionId,
}

/// Returns content served for the given currently intercepted request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyForInterceptionReturns {
    /// Response body.

    pub body: String,
    /// True, if content was sent as base64.

    pub base64Encoded: bool,
}

/// Returns a handle to the stream representing the response body. Note that after this command,
/// the intercepted request can't be continued as is -- you either need to cancel it or to provide
/// the response body. The stream only supports sequential read, IO.read will fail if the position
/// is specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeResponseBodyForInterceptionAsStreamParams {

    pub interceptionId: InterceptionId,
}

/// Returns a handle to the stream representing the response body. Note that after this command,
/// the intercepted request can't be continued as is -- you either need to cancel it or to provide
/// the response body. The stream only supports sequential read, IO.read will fail if the position
/// is specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeResponseBodyForInterceptionAsStreamReturns {

    pub stream: crate::io::StreamHandle,
}

/// This method sends a new XMLHttpRequest which is identical to the original one. The following
/// parameters should be identical: method, url, async, request body, extra headers, withCredentials
/// attribute, user, password.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplayXHRParams {
    /// Identifier of XHR to replay.

    pub requestId: RequestId,
}

/// Searches for given string in response content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResponseBodyParams {
    /// Identifier of the network response to search.

    pub requestId: RequestId,
    /// String to search for.

    pub query: String,
    /// If true, search is case sensitive.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caseSensitive: Option<bool>,
    /// If true, treats string parameter as regex.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isRegex: Option<bool>,
}

/// Searches for given string in response content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResponseBodyReturns {
    /// List of search matches.

    pub result: Vec<crate::debugger::SearchMatch>,
}

/// Blocks URLs from loading.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBlockedURLsParams {
    /// Patterns to match in the order in which they are given. These patterns
    /// also take precedence over any wildcard patterns defined in 'urls'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlPatterns: Option<Vec<BlockPattern>>,
    /// URL patterns to block. Wildcards ('*') are allowed.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// Toggles ignoring of service worker for each request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBypassServiceWorkerParams {
    /// Bypass service worker and load from network.

    pub bypass: bool,
}

/// Toggles ignoring cache for each request. If 'true', cache will not be used.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCacheDisabledParams {
    /// Cache disabled state.

    pub cacheDisabled: bool,
}

/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieParams {
    /// Cookie name.

    pub name: String,
    /// Cookie value.

    pub value: String,
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Cookie domain.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Cookie path.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// True if cookie is secure.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// True if cookie is http-only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub httpOnly: Option<bool>,
    /// Cookie SameSite type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sameSite: Option<CookieSameSite>,
    /// Cookie expiration date, session cookie if not set

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<TimeSinceEpoch>,
    /// Cookie Priority type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<CookiePriority>,
    /// Cookie source scheme type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceScheme: Option<CookieSourceScheme>,
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourcePort: Option<i64>,
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitionKey: Option<CookiePartitionKey>,
}

/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieReturns {
    /// Always set to true. If an error occurs, the response indicates protocol error.

    pub success: bool,
}

/// Sets given cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookiesParams {
    /// Cookies to be set.

    pub cookies: Vec<CookieParam>,
}

/// Specifies whether to always send extra HTTP headers with the requests from this page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetExtraHTTPHeadersParams {
    /// Map with extra HTTP headers.

    pub headers: Headers,
}

/// Specifies whether to attach a page script stack id in requests

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAttachDebugStackParams {
    /// Whether to attach a page script stack for debugging purpose.

    pub enabled: bool,
}

/// Sets the requests to intercept that match the provided patterns and optionally resource types.
/// Deprecated, please use Fetch.enable instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRequestInterceptionParams {
    /// Requests matching any of these patterns will be forwarded and wait for the corresponding
    /// continueInterceptedRequest call.

    pub patterns: Vec<RequestPattern>,
}

/// Allows overriding user agent with the given string.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAgentOverrideParams {
    /// User agent to use.

    pub userAgent: String,
    /// Browser language to emulate.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptLanguage: Option<String>,
    /// The platform navigator.platform should return.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userAgentMetadata: Option<crate::emulation::UserAgentMetadata>,
}

/// Enables streaming of the response for the given requestId.
/// If enabled, the dataReceived event contains the data that was received during streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StreamResourceContentParams {
    /// Identifier of the request to stream.

    pub requestId: RequestId,
}

/// Enables streaming of the response for the given requestId.
/// If enabled, the dataReceived event contains the data that was received during streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StreamResourceContentReturns {
    /// Data that has been buffered until streaming is enabled. (Encoded as a base64 string when passed over JSON)

    pub bufferedData: String,
}

/// Returns information about the COEP/COOP isolation status.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSecurityIsolationStatusParams {
    /// If no frameId is provided, the status of the target is provided.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
}

/// Returns information about the COEP/COOP isolation status.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSecurityIsolationStatusReturns {

    pub status: SecurityIsolationStatus,
}

/// Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.
/// Enabling triggers 'reportingApiReportAdded' for all existing reports.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableReportingApiParams {
    /// Whether to enable or disable events for the Reporting API

    pub enable: bool,
}

/// Sets up tracking device bound sessions and fetching of initial set of sessions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableDeviceBoundSessionsParams {
    /// Whether to enable or disable events.

    pub enable: bool,
}

/// Deletes a device bound session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDeviceBoundSessionParams {

    pub key: DeviceBoundSessionKey,
}

/// Fetches the schemeful site for a specific origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchSchemefulSiteParams {
    /// The URL origin.

    pub origin: String,
}

/// Fetches the schemeful site for a specific origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchSchemefulSiteReturns {
    /// The corresponding schemeful site.

    pub schemefulSite: String,
}

/// Fetches the resource and returns the content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceParams {
    /// Frame id to get the resource for. Mandatory for frame targets, and
    /// should be omitted for worker targets.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
    /// URL of the resource to get content for.

    pub url: String,
    /// Options for the request.

    pub options: LoadNetworkResourceOptions,
}

/// Fetches the resource and returns the content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceReturns {

    pub resource: LoadNetworkResourcePageResult,
}

/// Sets Controls for third-party cookie access
/// Page reload is required before the new cookie behavior will be observed

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieControlsParams {
    /// Whether 3pc restriction is enabled.

    pub enableThirdPartyCookieRestriction: bool,
}
