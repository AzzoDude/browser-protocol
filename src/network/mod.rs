//! Network domain allows tracking network activities of the page. It exposes information about http,
//! file, data and other requests and responses, their headers, bodies, timing, etc.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Resource type as it was perceived by the rendering engine.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ResourceType {
    #[default]
    #[serde(rename = "Document")]
    Document,
    #[serde(rename = "Stylesheet")]
    Stylesheet,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Media")]
    Media,
    #[serde(rename = "Font")]
    Font,
    #[serde(rename = "Script")]
    Script,
    #[serde(rename = "TextTrack")]
    TextTrack,
    #[serde(rename = "XHR")]
    XHR,
    #[serde(rename = "Fetch")]
    Fetch,
    #[serde(rename = "Prefetch")]
    Prefetch,
    #[serde(rename = "EventSource")]
    EventSource,
    #[serde(rename = "WebSocket")]
    WebSocket,
    #[serde(rename = "Manifest")]
    Manifest,
    #[serde(rename = "SignedExchange")]
    SignedExchange,
    #[serde(rename = "Ping")]
    Ping,
    #[serde(rename = "CSPViolationReport")]
    CSPViolationReport,
    #[serde(rename = "Preflight")]
    Preflight,
    #[serde(rename = "FedCM")]
    FedCM,
    #[serde(rename = "Other")]
    Other,
}

/// Unique loader identifier.

pub type LoaderId<'a> = Cow<'a, str>;

/// Unique network request identifier.
/// Note that this does not identify individual HTTP requests that are part of
/// a network request.

pub type RequestId<'a> = Cow<'a, str>;

/// Network level fetch failure reason.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ErrorReason {
    #[default]
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Aborted")]
    Aborted,
    #[serde(rename = "TimedOut")]
    TimedOut,
    #[serde(rename = "AccessDenied")]
    AccessDenied,
    #[serde(rename = "ConnectionClosed")]
    ConnectionClosed,
    #[serde(rename = "ConnectionReset")]
    ConnectionReset,
    #[serde(rename = "ConnectionRefused")]
    ConnectionRefused,
    #[serde(rename = "ConnectionAborted")]
    ConnectionAborted,
    #[serde(rename = "ConnectionFailed")]
    ConnectionFailed,
    #[serde(rename = "NameNotResolved")]
    NameNotResolved,
    #[serde(rename = "InternetDisconnected")]
    InternetDisconnected,
    #[serde(rename = "AddressUnreachable")]
    AddressUnreachable,
    #[serde(rename = "BlockedByClient")]
    BlockedByClient,
    #[serde(rename = "BlockedByResponse")]
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
    #[serde(rename = "none")]
    None,
    #[serde(rename = "cellular2g")]
    Cellular2g,
    #[serde(rename = "cellular3g")]
    Cellular3g,
    #[serde(rename = "cellular4g")]
    Cellular4g,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "ethernet")]
    Ethernet,
    #[serde(rename = "wifi")]
    Wifi,
    #[serde(rename = "wimax")]
    Wimax,
    #[serde(rename = "other")]
    Other,
}

/// Represents the cookie's 'SameSite' status:
/// <https://tools.ietf.org/html/draft-west-first-party-cookies>

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieSameSite {
    #[default]
    #[serde(rename = "Strict")]
    Strict,
    #[serde(rename = "Lax")]
    Lax,
    #[serde(rename = "None")]
    None,
}

/// Represents the cookie's 'Priority' status:
/// <https://tools.ietf.org/html/draft-west-cookie-priority-00>

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookiePriority {
    #[default]
    #[serde(rename = "Low")]
    Low,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "High")]
    High,
}

/// Represents the source scheme of the origin that originally set the cookie.
/// A value of "Unset" allows protocol clients to emulate legacy cookie scope for the scheme.
/// This is a temporary ability and it will be removed in the future.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieSourceScheme {
    #[default]
    #[serde(rename = "Unset")]
    Unset,
    #[serde(rename = "NonSecure")]
    NonSecure,
    #[serde(rename = "Secure")]
    Secure,
}

/// Timing information for the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTiming {
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime.
    #[serde(rename = "requestTime")]
    pub request_time: f64,
    /// Started resolving proxy.
    #[serde(rename = "proxyStart")]
    pub proxy_start: f64,
    /// Finished resolving proxy.
    #[serde(rename = "proxyEnd")]
    pub proxy_end: f64,
    /// Started DNS address resolve.
    #[serde(rename = "dnsStart")]
    pub dns_start: f64,
    /// Finished DNS address resolve.
    #[serde(rename = "dnsEnd")]
    pub dns_end: f64,
    /// Started connecting to the remote host.
    #[serde(rename = "connectStart")]
    pub connect_start: f64,
    /// Connected to the remote host.
    #[serde(rename = "connectEnd")]
    pub connect_end: f64,
    /// Started SSL handshake.
    #[serde(rename = "sslStart")]
    pub ssl_start: f64,
    /// Finished SSL handshake.
    #[serde(rename = "sslEnd")]
    pub ssl_end: f64,
    /// Started running ServiceWorker.
    #[serde(rename = "workerStart")]
    pub worker_start: f64,
    /// Finished Starting ServiceWorker.
    #[serde(rename = "workerReady")]
    pub worker_ready: f64,
    /// Started fetch event.
    #[serde(rename = "workerFetchStart")]
    pub worker_fetch_start: f64,
    /// Settled fetch event respondWith promise.
    #[serde(rename = "workerRespondWithSettled")]
    pub worker_respond_with_settled: f64,
    /// Started ServiceWorker static routing source evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "workerRouterEvaluationStart")]
    pub worker_router_evaluation_start: Option<f64>,
    /// Started cache lookup when the source was evaluated to 'cache'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "workerCacheLookupStart")]
    pub worker_cache_lookup_start: Option<f64>,
    /// Started sending request.
    #[serde(rename = "sendStart")]
    pub send_start: f64,
    /// Finished sending request.
    #[serde(rename = "sendEnd")]
    pub send_end: f64,
    /// Time the server started pushing request.
    #[serde(rename = "pushStart")]
    pub push_start: f64,
    /// Time the server finished pushing request.
    #[serde(rename = "pushEnd")]
    pub push_end: f64,
    /// Started receiving response headers.
    #[serde(rename = "receiveHeadersStart")]
    pub receive_headers_start: f64,
    /// Finished receiving response headers.
    #[serde(rename = "receiveHeadersEnd")]
    pub receive_headers_end: f64,
}
/// Loading priority of a resource request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ResourcePriority {
    #[default]
    #[serde(rename = "VeryLow")]
    VeryLow,
    #[serde(rename = "Low")]
    Low,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "High")]
    High,
    #[serde(rename = "VeryHigh")]
    VeryHigh,
}

/// The render-blocking behavior of a resource request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum RenderBlockingBehavior {
    #[default]
    #[serde(rename = "Blocking")]
    Blocking,
    #[serde(rename = "InBodyParserBlocking")]
    InBodyParserBlocking,
    #[serde(rename = "NonBlocking")]
    NonBlocking,
    #[serde(rename = "NonBlockingDynamic")]
    NonBlockingDynamic,
    #[serde(rename = "PotentiallyBlocking")]
    PotentiallyBlocking,
}

/// Post data entry for HTTP request

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PostDataEntry<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Cow<'a, str>>,
}
/// HTTP request data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Request<'a> {
    /// Request URL (without fragment).
    pub url: Cow<'a, str>,
    /// Fragment of the requested URL starting with hash, if present.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlFragment")]
    pub url_fragment: Option<Cow<'a, str>>,
    /// HTTP request method.
    pub method: Cow<'a, str>,
    /// HTTP request headers.
    pub headers: Headers,
    /// HTTP POST request data.
    /// Use postDataEntries instead.
    #[serde(skip_serializing_if = "Option::is_none", rename = "postData")]
    pub post_data: Option<Cow<'a, str>>,
    /// True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasPostData")]
    pub has_post_data: Option<bool>,
    /// Request body elements (post data broken into individual entries).
    #[serde(skip_serializing_if = "Option::is_none", rename = "postDataEntries")]
    pub post_data_entries: Option<Vec<PostDataEntry<'a>>>,
    /// The mixed content type of the request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "mixedContentType")]
    pub mixed_content_type: Option<crate::security::MixedContentType>,
    /// Priority of the resource request at the time request is sent.
    #[serde(rename = "initialPriority")]
    pub initial_priority: ResourcePriority,
    /// The referrer policy of the request, as defined in <https://www.w3.org/TR/referrer-policy/>
    #[serde(rename = "referrerPolicy")]
    pub referrer_policy: Cow<'a, str>,
    /// Whether is loaded via link preload.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isLinkPreload")]
    pub is_link_preload: Option<bool>,
    /// Set for requests when the TrustToken API is used. Contains the parameters
    /// passed by the developer (e.g. via "fetch") as understood by the backend.
    #[serde(skip_serializing_if = "Option::is_none", rename = "trustTokenParams")]
    pub trust_token_params: Option<TrustTokenParams<'a>>,
    /// True if this resource request is considered to be the 'same site' as the
    /// request corresponding to the main frame.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isSameSite")]
    pub is_same_site: Option<bool>,
    /// True when the resource request is ad-related.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isAdRelated")]
    pub is_ad_related: Option<bool>,
}
/// Details of a signed certificate timestamp (SCT).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SignedCertificateTimestamp<'a> {
    /// Validation status.
    pub status: Cow<'a, str>,
    /// Origin.
    pub origin: Cow<'a, str>,
    /// Log name / description.
    #[serde(rename = "logDescription")]
    pub log_description: Cow<'a, str>,
    /// Log ID.
    #[serde(rename = "logId")]
    pub log_id: Cow<'a, str>,
    /// Issuance date. Unlike TimeSinceEpoch, this contains the number of
    /// milliseconds since January 1, 1970, UTC, not the number of seconds.
    pub timestamp: f64,
    /// Hash algorithm.
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: Cow<'a, str>,
    /// Signature algorithm.
    #[serde(rename = "signatureAlgorithm")]
    pub signature_algorithm: Cow<'a, str>,
    /// Signature data.
    #[serde(rename = "signatureData")]
    pub signature_data: Cow<'a, str>,
}
/// Security details about a request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDetails<'a> {
    /// Protocol name (e.g. "TLS 1.2" or "QUIC").
    pub protocol: Cow<'a, str>,
    /// Key Exchange used by the connection, or the empty string if not applicable.
    #[serde(rename = "keyExchange")]
    pub key_exchange: Cow<'a, str>,
    /// (EC)DH group used by the connection, if applicable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "keyExchangeGroup")]
    pub key_exchange_group: Option<Cow<'a, str>>,
    /// Cipher name.
    pub cipher: Cow<'a, str>,
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<Cow<'a, str>>,
    /// Certificate ID value.
    #[serde(rename = "certificateId")]
    pub certificate_id: crate::security::CertificateId,
    /// Certificate subject name.
    #[serde(rename = "subjectName")]
    pub subject_name: Cow<'a, str>,
    /// Subject Alternative Name (SAN) DNS names and IP addresses.
    #[serde(rename = "sanList")]
    pub san_list: Vec<Cow<'a, str>>,
    /// Name of the issuing CA.
    pub issuer: Cow<'a, str>,
    /// Certificate valid from date.
    #[serde(rename = "validFrom")]
    pub valid_from: TimeSinceEpoch,
    /// Certificate valid to (expiration) date
    #[serde(rename = "validTo")]
    pub valid_to: TimeSinceEpoch,
    /// List of signed certificate timestamps (SCTs).
    #[serde(rename = "signedCertificateTimestampList")]
    pub signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp<'a>>,
    /// Whether the request complied with Certificate Transparency policy
    #[serde(rename = "certificateTransparencyCompliance")]
    pub certificate_transparency_compliance: CertificateTransparencyCompliance,
    /// The signature algorithm used by the server in the TLS server signature,
    /// represented as a TLS SignatureScheme code point. Omitted if not
    /// applicable or not known.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serverSignatureAlgorithm")]
    pub server_signature_algorithm: Option<i64>,
    /// Whether the connection used Encrypted ClientHello
    #[serde(rename = "encryptedClientHello")]
    pub encrypted_client_hello: bool,
}
/// Whether the request complied with Certificate Transparency policy.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CertificateTransparencyCompliance {
    #[default]
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "not-compliant")]
    NotCompliant,
    #[serde(rename = "compliant")]
    Compliant,
}

/// The reason why request was blocked.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BlockedReason {
    #[default]
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "csp")]
    Csp,
    #[serde(rename = "mixed-content")]
    MixedContent,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "inspector")]
    Inspector,
    #[serde(rename = "integrity")]
    Integrity,
    #[serde(rename = "subresource-filter")]
    SubresourceFilter,
    #[serde(rename = "content-type")]
    ContentType,
    #[serde(rename = "coep-frame-resource-needs-coep-header")]
    CoepFrameResourceNeedsCoepHeader,
    #[serde(rename = "coop-sandboxed-iframe-cannot-navigate-to-coop-page")]
    CoopSandboxedIframeCannotNavigateToCoopPage,
    #[serde(rename = "corp-not-same-origin")]
    CorpNotSameOrigin,
    #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-coep")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-dip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-coep-and-dip")]
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    #[serde(rename = "corp-not-same-site")]
    CorpNotSameSite,
    #[serde(rename = "sri-message-signature-mismatch")]
    SriMessageSignatureMismatch,
}

/// The reason why request was blocked.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CorsError {
    #[default]
    #[serde(rename = "DisallowedByMode")]
    DisallowedByMode,
    #[serde(rename = "InvalidResponse")]
    InvalidResponse,
    #[serde(rename = "WildcardOriginNotAllowed")]
    WildcardOriginNotAllowed,
    #[serde(rename = "MissingAllowOriginHeader")]
    MissingAllowOriginHeader,
    #[serde(rename = "MultipleAllowOriginValues")]
    MultipleAllowOriginValues,
    #[serde(rename = "InvalidAllowOriginValue")]
    InvalidAllowOriginValue,
    #[serde(rename = "AllowOriginMismatch")]
    AllowOriginMismatch,
    #[serde(rename = "InvalidAllowCredentials")]
    InvalidAllowCredentials,
    #[serde(rename = "CorsDisabledScheme")]
    CorsDisabledScheme,
    #[serde(rename = "PreflightInvalidStatus")]
    PreflightInvalidStatus,
    #[serde(rename = "PreflightDisallowedRedirect")]
    PreflightDisallowedRedirect,
    #[serde(rename = "PreflightWildcardOriginNotAllowed")]
    PreflightWildcardOriginNotAllowed,
    #[serde(rename = "PreflightMissingAllowOriginHeader")]
    PreflightMissingAllowOriginHeader,
    #[serde(rename = "PreflightMultipleAllowOriginValues")]
    PreflightMultipleAllowOriginValues,
    #[serde(rename = "PreflightInvalidAllowOriginValue")]
    PreflightInvalidAllowOriginValue,
    #[serde(rename = "PreflightAllowOriginMismatch")]
    PreflightAllowOriginMismatch,
    #[serde(rename = "PreflightInvalidAllowCredentials")]
    PreflightInvalidAllowCredentials,
    #[serde(rename = "PreflightMissingAllowExternal")]
    PreflightMissingAllowExternal,
    #[serde(rename = "PreflightInvalidAllowExternal")]
    PreflightInvalidAllowExternal,
    #[serde(rename = "InvalidAllowMethodsPreflightResponse")]
    InvalidAllowMethodsPreflightResponse,
    #[serde(rename = "InvalidAllowHeadersPreflightResponse")]
    InvalidAllowHeadersPreflightResponse,
    #[serde(rename = "MethodDisallowedByPreflightResponse")]
    MethodDisallowedByPreflightResponse,
    #[serde(rename = "HeaderDisallowedByPreflightResponse")]
    HeaderDisallowedByPreflightResponse,
    #[serde(rename = "RedirectContainsCredentials")]
    RedirectContainsCredentials,
    #[serde(rename = "InsecureLocalNetwork")]
    InsecureLocalNetwork,
    #[serde(rename = "InvalidLocalNetworkAccess")]
    InvalidLocalNetworkAccess,
    #[serde(rename = "NoCorsRedirectModeNotFollow")]
    NoCorsRedirectModeNotFollow,
    #[serde(rename = "LocalNetworkAccessPermissionDenied")]
    LocalNetworkAccessPermissionDenied,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CorsErrorStatus<'a> {
    #[serde(rename = "corsError")]
    pub cors_error: CorsError,
    #[serde(rename = "failedParameter")]
    pub failed_parameter: Cow<'a, str>,
}
/// Source of serviceworker response.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceWorkerResponseSource {
    #[default]
    #[serde(rename = "cache-storage")]
    CacheStorage,
    #[serde(rename = "http-cache")]
    HttpCache,
    #[serde(rename = "fallback-code")]
    FallbackCode,
    #[serde(rename = "network")]
    Network,
}

/// Determines what type of Trust Token operation is executed and
/// depending on the type, some additional parameters. The values
/// are specified in third_party/blink/renderer/core/fetch/trust_token.idl.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokenParams<'a> {
    pub operation: TrustTokenOperationType,
    /// Only set for "token-redemption" operation and determine whether
    /// to request a fresh SRR or use a still valid cached SRR.
    #[serde(rename = "refreshPolicy")]
    pub refresh_policy: Cow<'a, str>,
    /// Origins of issuers from whom to request tokens or redemption
    /// records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuers: Option<Vec<Cow<'a, str>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TrustTokenOperationType {
    #[default]
    #[serde(rename = "Issuance")]
    Issuance,
    #[serde(rename = "Redemption")]
    Redemption,
    #[serde(rename = "Signing")]
    Signing,
}

/// The reason why Chrome uses a specific transport protocol for HTTP semantics.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AlternateProtocolUsage {
    #[default]
    #[serde(rename = "alternativeJobWonWithoutRace")]
    AlternativeJobWonWithoutRace,
    #[serde(rename = "alternativeJobWonRace")]
    AlternativeJobWonRace,
    #[serde(rename = "mainJobWonRace")]
    MainJobWonRace,
    #[serde(rename = "mappingMissing")]
    MappingMissing,
    #[serde(rename = "broken")]
    Broken,
    #[serde(rename = "dnsAlpnH3JobWonWithoutRace")]
    DnsAlpnH3JobWonWithoutRace,
    #[serde(rename = "dnsAlpnH3JobWonRace")]
    DnsAlpnH3JobWonRace,
    #[serde(rename = "unspecifiedReason")]
    UnspecifiedReason,
}

/// Source of service worker router.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceWorkerRouterSource {
    #[default]
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "fetch-event")]
    FetchEvent,
    #[serde(rename = "race-network-and-fetch-handler")]
    RaceNetworkAndFetchHandler,
    #[serde(rename = "race-network-and-cache")]
    RaceNetworkAndCache,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRouterInfo {
    /// ID of the rule matched. If there is a matched rule, this field will
    /// be set, otherwiser no value will be set.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ruleIdMatched")]
    pub rule_id_matched: Option<u64>,
    /// The router source of the matched rule. If there is a matched rule, this
    /// field will be set, otherwise no value will be set.
    #[serde(skip_serializing_if = "Option::is_none", rename = "matchedSourceType")]
    pub matched_source_type: Option<ServiceWorkerRouterSource>,
    /// The actual router source used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "actualSourceType")]
    pub actual_source_type: Option<ServiceWorkerRouterSource>,
}
/// HTTP response data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Response<'a> {
    /// Response URL. This URL can be different from CachedResource.url in case of redirect.
    pub url: Cow<'a, str>,
    /// HTTP response status code.
    pub status: i64,
    /// HTTP response status text.
    #[serde(rename = "statusText")]
    pub status_text: Cow<'a, str>,
    /// HTTP response headers.
    pub headers: Headers,
    /// HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo.
    #[serde(skip_serializing_if = "Option::is_none", rename = "headersText")]
    pub headers_text: Option<Cow<'a, str>>,
    /// Resource mimeType as determined by the browser.
    #[serde(rename = "mimeType")]
    pub mime_type: Cow<'a, str>,
    /// Resource charset as determined by the browser (if applicable).
    pub charset: Cow<'a, str>,
    /// Refined HTTP request headers that were actually transmitted over the network.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    pub request_headers: Option<Headers>,
    /// HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestHeadersText")]
    pub request_headers_text: Option<Cow<'a, str>>,
    /// Specifies whether physical connection was actually reused for this request.
    #[serde(rename = "connectionReused")]
    pub connection_reused: bool,
    /// Physical connection id that was actually used for this request.
    #[serde(rename = "connectionId")]
    pub connection_id: f64,
    /// Remote IP address.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remoteIPAddress")]
    pub remote_ip_address: Option<Cow<'a, str>>,
    /// Remote port.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remotePort")]
    pub remote_port: Option<i64>,
    /// Specifies that the request was served from the disk cache.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromDiskCache")]
    pub from_disk_cache: Option<bool>,
    /// Specifies that the request was served from the ServiceWorker.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromServiceWorker")]
    pub from_service_worker: Option<bool>,
    /// Specifies that the request was served from the prefetch cache.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromPrefetchCache")]
    pub from_prefetch_cache: Option<bool>,
    /// Specifies that the request was served from the prefetch cache.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromEarlyHints")]
    pub from_early_hints: Option<bool>,
    /// Information about how ServiceWorker Static Router API was used. If this
    /// field is set with 'matchedSourceType' field, a matching rule is found.
    /// If this field is set without 'matchedSource', no matching rule is found.
    /// Otherwise, the API is not used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serviceWorkerRouterInfo")]
    pub service_worker_router_info: Option<ServiceWorkerRouterInfo>,
    /// Total number of bytes received for this request so far.
    #[serde(rename = "encodedDataLength")]
    pub encoded_data_length: f64,
    /// Timing information for the given request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<ResourceTiming>,
    /// Response source of response from ServiceWorker.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serviceWorkerResponseSource")]
    pub service_worker_response_source: Option<ServiceWorkerResponseSource>,
    /// The time at which the returned response was generated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseTime")]
    pub response_time: Option<TimeSinceEpoch>,
    /// Cache Storage Cache Name.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cacheStorageCacheName")]
    pub cache_storage_cache_name: Option<Cow<'a, str>>,
    /// Protocol used to fetch this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Cow<'a, str>>,
    /// The reason why Chrome uses a specific transport protocol for HTTP semantics.
    #[serde(skip_serializing_if = "Option::is_none", rename = "alternateProtocolUsage")]
    pub alternate_protocol_usage: Option<AlternateProtocolUsage>,
    /// Security state of the request resource.
    #[serde(rename = "securityState")]
    pub security_state: crate::security::SecurityState,
    /// Security details for the request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityDetails")]
    pub security_details: Option<SecurityDetails<'a>>,
}
/// WebSocket request data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketRequest {
    /// HTTP request headers.
    pub headers: Headers,
}
/// WebSocket response data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketResponse<'a> {
    /// HTTP response status code.
    pub status: i64,
    /// HTTP response status text.
    #[serde(rename = "statusText")]
    pub status_text: Cow<'a, str>,
    /// HTTP response headers.
    pub headers: Headers,
    /// HTTP response headers text.
    #[serde(skip_serializing_if = "Option::is_none", rename = "headersText")]
    pub headers_text: Option<Cow<'a, str>>,
    /// HTTP request headers.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    pub request_headers: Option<Headers>,
    /// HTTP request headers text.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestHeadersText")]
    pub request_headers_text: Option<Cow<'a, str>>,
}
/// WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrame<'a> {
    /// WebSocket message opcode.
    pub opcode: f64,
    /// WebSocket message mask.
    pub mask: bool,
    /// WebSocket message payload data.
    /// If the opcode is 1, this is a text message and payloadData is a UTF-8 string.
    /// If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data.
    #[serde(rename = "payloadData")]
    pub payload_data: Cow<'a, str>,
}
/// Information about the cached resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CachedResource<'a> {
    /// Resource URL. This is the url of the original network request.
    pub url: Cow<'a, str>,
    /// Type of this resource.
    #[serde(rename = "type")]
    pub type_: ResourceType,
    /// Cached response data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Response<'a>>,
    /// Cached response body size.
    #[serde(rename = "bodySize")]
    pub body_size: f64,
}
/// Information about the request initiator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Initiator<'a> {
    /// Type of this initiator.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Initiator JavaScript stack trace, set for Script only.
    /// Requires the Debugger domain to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<crate::runtime::StackTrace>,
    /// Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// Initiator line number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    #[serde(skip_serializing_if = "Option::is_none", rename = "lineNumber")]
    pub line_number: Option<f64>,
    /// Initiator column number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnNumber")]
    pub column_number: Option<f64>,
    /// Set if another request triggered this request (e.g. preflight).
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestId")]
    pub request_id: Option<RequestId<'a>>,
}
/// cookiePartitionKey object
/// The representation of the components of the key that are created by the cookiePartitionKey class contained in net/cookies/cookie_partition_key.h.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CookiePartitionKey<'a> {
    /// The site of the top-level URL the browser was visiting at the start
    /// of the request to the endpoint that set the cookie.
    #[serde(rename = "topLevelSite")]
    pub top_level_site: Cow<'a, str>,
    /// Indicates if the cookie has any ancestors that are cross-site to the topLevelSite.
    #[serde(rename = "hasCrossSiteAncestor")]
    pub has_cross_site_ancestor: bool,
}
/// Cookie object

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Cookie<'a> {
    /// Cookie name.
    pub name: Cow<'a, str>,
    /// Cookie value.
    pub value: Cow<'a, str>,
    /// Cookie domain.
    pub domain: Cow<'a, str>,
    /// Cookie path.
    pub path: Cow<'a, str>,
    /// Cookie expiration date as the number of seconds since the UNIX epoch.
    /// The value is set to -1 if the expiry date is not set.
    /// The value can be null for values that cannot be represented in
    /// JSON (±Inf).
    pub expires: f64,
    /// Cookie size.
    pub size: u64,
    /// True if cookie is http-only.
    #[serde(rename = "httpOnly")]
    pub http_only: bool,
    /// True if cookie is secure.
    pub secure: bool,
    /// True in case of session cookie.
    pub session: bool,
    /// Cookie SameSite type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
    pub same_site: Option<CookieSameSite>,
    /// Cookie Priority
    pub priority: CookiePriority,
    /// Cookie source scheme type.
    #[serde(rename = "sourceScheme")]
    pub source_scheme: CookieSourceScheme,
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    #[serde(rename = "sourcePort")]
    pub source_port: i64,
    /// Cookie partition key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKey")]
    pub partition_key: Option<CookiePartitionKey<'a>>,
    /// True if cookie partition key is opaque.
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKeyOpaque")]
    pub partition_key_opaque: Option<bool>,
}
/// Types of reasons why a cookie may not be stored from a response.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SetCookieBlockedReason {
    #[default]
    #[serde(rename = "SecureOnly")]
    SecureOnly,
    #[serde(rename = "SameSiteStrict")]
    SameSiteStrict,
    #[serde(rename = "SameSiteLax")]
    SameSiteLax,
    #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
    SameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "SameSiteNoneInsecure")]
    SameSiteNoneInsecure,
    #[serde(rename = "UserPreferences")]
    UserPreferences,
    #[serde(rename = "ThirdPartyPhaseout")]
    ThirdPartyPhaseout,
    #[serde(rename = "SyntaxError")]
    SyntaxError,
    #[serde(rename = "SchemeNotSupported")]
    SchemeNotSupported,
    #[serde(rename = "OverwriteSecure")]
    OverwriteSecure,
    #[serde(rename = "InvalidDomain")]
    InvalidDomain,
    #[serde(rename = "InvalidPrefix")]
    InvalidPrefix,
    #[serde(rename = "UnknownError")]
    UnknownError,
    #[serde(rename = "SchemefulSameSiteStrict")]
    SchemefulSameSiteStrict,
    #[serde(rename = "SchemefulSameSiteLax")]
    SchemefulSameSiteLax,
    #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "NameValuePairExceedsMaxSize")]
    NameValuePairExceedsMaxSize,
    #[serde(rename = "DisallowedCharacter")]
    DisallowedCharacter,
    #[serde(rename = "NoCookieContent")]
    NoCookieContent,
}

/// Types of reasons why a cookie may not be sent with a request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieBlockedReason {
    #[default]
    #[serde(rename = "SecureOnly")]
    SecureOnly,
    #[serde(rename = "NotOnPath")]
    NotOnPath,
    #[serde(rename = "DomainMismatch")]
    DomainMismatch,
    #[serde(rename = "SameSiteStrict")]
    SameSiteStrict,
    #[serde(rename = "SameSiteLax")]
    SameSiteLax,
    #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
    SameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "SameSiteNoneInsecure")]
    SameSiteNoneInsecure,
    #[serde(rename = "UserPreferences")]
    UserPreferences,
    #[serde(rename = "ThirdPartyPhaseout")]
    ThirdPartyPhaseout,
    #[serde(rename = "UnknownError")]
    UnknownError,
    #[serde(rename = "SchemefulSameSiteStrict")]
    SchemefulSameSiteStrict,
    #[serde(rename = "SchemefulSameSiteLax")]
    SchemefulSameSiteLax,
    #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    #[serde(rename = "NameValuePairExceedsMaxSize")]
    NameValuePairExceedsMaxSize,
    #[serde(rename = "PortMismatch")]
    PortMismatch,
    #[serde(rename = "SchemeMismatch")]
    SchemeMismatch,
    #[serde(rename = "AnonymousContext")]
    AnonymousContext,
}

/// Types of reasons why a cookie should have been blocked by 3PCD but is exempted for the request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CookieExemptionReason {
    #[default]
    #[serde(rename = "None")]
    None,
    #[serde(rename = "UserSetting")]
    UserSetting,
    #[serde(rename = "EnterprisePolicy")]
    EnterprisePolicy,
    #[serde(rename = "StorageAccess")]
    StorageAccess,
    #[serde(rename = "TopLevelStorageAccess")]
    TopLevelStorageAccess,
    #[serde(rename = "Scheme")]
    Scheme,
    #[serde(rename = "SameSiteNoneCookiesInSandbox")]
    SameSiteNoneCookiesInSandbox,
}

/// A cookie which was not stored from a response with the corresponding reason.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BlockedSetCookieWithReason<'a> {
    /// The reason(s) this cookie was blocked.
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<SetCookieBlockedReason>,
    /// The string representing this individual cookie as it would appear in the header.
    /// This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.
    #[serde(rename = "cookieLine")]
    pub cookie_line: Cow<'a, str>,
    /// The cookie object which represents the cookie which was not stored. It is optional because
    /// sometimes complete cookie information is not available, such as in the case of parsing
    /// errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<Cookie<'a>>,
}
/// A cookie should have been blocked by 3PCD but is exempted and stored from a response with the
/// corresponding reason. A cookie could only have at most one exemption reason.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ExemptedSetCookieWithReason<'a> {
    /// The reason the cookie was exempted.
    #[serde(rename = "exemptionReason")]
    pub exemption_reason: CookieExemptionReason,
    /// The string representing this individual cookie as it would appear in the header.
    #[serde(rename = "cookieLine")]
    pub cookie_line: Cow<'a, str>,
    /// The cookie object representing the cookie.
    pub cookie: Cookie<'a>,
}
/// A cookie associated with the request which may or may not be sent with it.
/// Includes the cookies itself and reasons for blocking or exemption.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedCookie<'a> {
    /// The cookie object representing the cookie which was not sent.
    pub cookie: Cookie<'a>,
    /// The reason(s) the cookie was blocked. If empty means the cookie is included.
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<CookieBlockedReason>,
    /// The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could
    /// only have at most one exemption reason.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exemptionReason")]
    pub exemption_reason: Option<CookieExemptionReason>,
}
/// Cookie parameter object

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CookieParam<'a> {
    /// Cookie name.
    pub name: Cow<'a, str>,
    /// Cookie value.
    pub value: Cow<'a, str>,
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// Cookie domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Cow<'a, str>>,
    /// Cookie path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Cow<'a, str>>,
    /// True if cookie is secure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// True if cookie is http-only.
    #[serde(skip_serializing_if = "Option::is_none", rename = "httpOnly")]
    pub http_only: Option<bool>,
    /// Cookie SameSite type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
    pub same_site: Option<CookieSameSite>,
    /// Cookie expiration date, session cookie if not set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<TimeSinceEpoch>,
    /// Cookie Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<CookiePriority>,
    /// Cookie source scheme type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceScheme")]
    pub source_scheme: Option<CookieSourceScheme>,
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourcePort")]
    pub source_port: Option<i64>,
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKey")]
    pub partition_key: Option<CookiePartitionKey<'a>>,
}
/// Authorization challenge for HTTP status code 401 or 407.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallenge<'a> {
    /// Source of the authentication challenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Cow<'a, str>>,
    /// Origin of the challenger.
    pub origin: Cow<'a, str>,
    /// The authentication scheme used, such as basic or digest
    pub scheme: Cow<'a, str>,
    /// The realm of the challenge. May be empty.
    pub realm: Cow<'a, str>,
}
/// Response to an AuthChallenge.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallengeResponse<'a> {
    /// The decision on what to do in response to the authorization challenge.  Default means
    /// deferring to the default behavior of the net stack, which will likely either the Cancel
    /// authentication or display a popup dialog box.
    pub response: Cow<'a, str>,
    /// The username to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Cow<'a, str>>,
    /// The password to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<Cow<'a, str>>,
}
/// Information about a signed exchange signature.
/// <https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeSignature<'a> {
    /// Signed exchange signature label.
    pub label: Cow<'a, str>,
    /// The hex string of signed exchange signature.
    pub signature: Cow<'a, str>,
    /// Signed exchange signature integrity.
    pub integrity: Cow<'a, str>,
    /// Signed exchange signature cert Url.
    #[serde(skip_serializing_if = "Option::is_none", rename = "certUrl")]
    pub cert_url: Option<Cow<'a, str>>,
    /// The hex string of signed exchange signature cert sha256.
    #[serde(skip_serializing_if = "Option::is_none", rename = "certSha256")]
    pub cert_sha256: Option<Cow<'a, str>>,
    /// Signed exchange signature validity Url.
    #[serde(rename = "validityUrl")]
    pub validity_url: Cow<'a, str>,
    /// Signed exchange signature date.
    pub date: i64,
    /// Signed exchange signature expires.
    pub expires: i64,
    /// The encoded certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Cow<'a, str>>>,
}
/// Information about a signed exchange header.
/// <https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeHeader<'a> {
    /// Signed exchange request URL.
    #[serde(rename = "requestUrl")]
    pub request_url: Cow<'a, str>,
    /// Signed exchange response code.
    #[serde(rename = "responseCode")]
    pub response_code: i64,
    /// Signed exchange response headers.
    #[serde(rename = "responseHeaders")]
    pub response_headers: Headers,
    /// Signed exchange response signature.
    pub signatures: Vec<SignedExchangeSignature<'a>>,
    /// Signed exchange header integrity hash in the form of 'sha256-\<base64-hash-value\>'.
    #[serde(rename = "headerIntegrity")]
    pub header_integrity: Cow<'a, str>,
}
/// Field type for a signed exchange related error.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SignedExchangeErrorField {
    #[default]
    #[serde(rename = "signatureSig")]
    SignatureSig,
    #[serde(rename = "signatureIntegrity")]
    SignatureIntegrity,
    #[serde(rename = "signatureCertUrl")]
    SignatureCertUrl,
    #[serde(rename = "signatureCertSha256")]
    SignatureCertSha256,
    #[serde(rename = "signatureValidityUrl")]
    SignatureValidityUrl,
    #[serde(rename = "signatureTimestamps")]
    SignatureTimestamps,
}

/// Information about a signed exchange response.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeError<'a> {
    /// Error message.
    pub message: Cow<'a, str>,
    /// The index of the signature which caused the error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "signatureIndex")]
    pub signature_index: Option<u64>,
    /// The field which caused the error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorField")]
    pub error_field: Option<SignedExchangeErrorField>,
}
/// Information about a signed exchange response.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeInfo<'a> {
    /// The outer response of signed HTTP exchange which was received from network.
    #[serde(rename = "outerResponse")]
    pub outer_response: Response<'a>,
    /// Whether network response for the signed exchange was accompanied by
    /// extra headers.
    #[serde(rename = "hasExtraInfo")]
    pub has_extra_info: bool,
    /// Information about the signed exchange header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<SignedExchangeHeader<'a>>,
    /// Security details for the signed exchange header.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityDetails")]
    pub security_details: Option<SecurityDetails<'a>>,
    /// Errors occurred while handling the signed exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<SignedExchangeError<'a>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConditions<'a> {
    /// Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string
    /// syntax (<https://urlpattern.spec.whatwg.org/>) and must be absolute. If the pattern is empty, all requests are
    /// matched (including p2p connections).
    #[serde(rename = "urlPattern")]
    pub url_pattern: Cow<'a, str>,
    /// Minimum latency from request sent to response headers received (ms).
    pub latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    #[serde(rename = "downloadThroughput")]
    pub download_throughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    #[serde(rename = "uploadThroughput")]
    pub upload_throughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectionType")]
    pub connection_type: Option<ConnectionType>,
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetLoss")]
    pub packet_loss: Option<f64>,
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetQueueLength")]
    pub packet_queue_length: Option<u64>,
    /// WebRTC packetReordering feature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetReordering")]
    pub packet_reordering: Option<bool>,
    /// True to emulate internet disconnection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BlockPattern<'a> {
    /// URL pattern to match. Patterns use the URLPattern constructor string syntax
    /// (<https://urlpattern.spec.whatwg.org/>) and must be absolute. Example: '*://*:*/*.css'.
    #[serde(rename = "urlPattern")]
    pub url_pattern: Cow<'a, str>,
    /// Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later
    /// 'BlockPattern'.
    pub block: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DirectSocketDnsQueryType {
    #[default]
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DirectTCPSocketOptions {
    /// TCP_NODELAY option
    #[serde(rename = "noDelay")]
    pub no_delay: bool,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAliveDelay")]
    pub keep_alive_delay: Option<f64>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sendBufferSize")]
    pub send_buffer_size: Option<f64>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "receiveBufferSize")]
    pub receive_buffer_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dnsQueryType")]
    pub dns_query_type: Option<DirectSocketDnsQueryType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DirectUDPSocketOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "remoteAddr")]
    pub remote_addr: Option<Cow<'a, str>>,
    /// Unsigned int 16.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remotePort")]
    pub remote_port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "localAddr")]
    pub local_addr: Option<Cow<'a, str>>,
    /// Unsigned int 16.
    #[serde(skip_serializing_if = "Option::is_none", rename = "localPort")]
    pub local_port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dnsQueryType")]
    pub dns_query_type: Option<DirectSocketDnsQueryType>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sendBufferSize")]
    pub send_buffer_size: Option<f64>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "receiveBufferSize")]
    pub receive_buffer_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multicastLoopback")]
    pub multicast_loopback: Option<bool>,
    /// Unsigned int 8.
    #[serde(skip_serializing_if = "Option::is_none", rename = "multicastTimeToLive")]
    pub multicast_time_to_live: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multicastAllowAddressSharing")]
    pub multicast_allow_address_sharing: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DirectUDPMessage<'a> {
    pub data: Cow<'a, str>,
    /// Null for connected mode.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remoteAddr")]
    pub remote_addr: Option<Cow<'a, str>>,
    /// Null for connected mode.
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remotePort")]
    pub remote_port: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum LocalNetworkAccessRequestPolicy {
    #[default]
    #[serde(rename = "Allow")]
    Allow,
    #[serde(rename = "BlockFromInsecureToMorePrivate")]
    BlockFromInsecureToMorePrivate,
    #[serde(rename = "WarnFromInsecureToMorePrivate")]
    WarnFromInsecureToMorePrivate,
    #[serde(rename = "PermissionBlock")]
    PermissionBlock,
    #[serde(rename = "PermissionWarn")]
    PermissionWarn,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum IPAddressSpace {
    #[default]
    #[serde(rename = "Loopback")]
    Loopback,
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "Unknown")]
    Unknown,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ConnectTiming {
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for
    /// the same request (but not for redirected requests).
    #[serde(rename = "requestTime")]
    pub request_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ClientSecurityState {
    #[serde(rename = "initiatorIsSecureContext")]
    pub initiator_is_secure_context: bool,
    #[serde(rename = "initiatorIPAddressSpace")]
    pub initiator_ip_address_space: IPAddressSpace,
    #[serde(rename = "localNetworkAccessRequestPolicy")]
    pub local_network_access_request_policy: LocalNetworkAccessRequestPolicy,
}
/// Identifies the script on the stack that caused a resource or element to be
/// labeled as an ad. For resources, this indicates the context that triggered
/// the fetch. For elements, this indicates the context that caused the element
/// to be appended to the DOM.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdScriptIdentifier<'a> {
    /// The script's V8 identifier.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// V8's debugging ID for the v8::Context.
    #[serde(rename = "debuggerId")]
    pub debugger_id: crate::runtime::UniqueDebuggerId<'a>,
    /// The script's url (or generated name based on id if inline script).
    pub name: Cow<'a, str>,
}
/// Encapsulates the script ancestry and the root script filter list rule that
/// caused the resource or element to be labeled as an ad.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdAncestry<'a> {
    /// A chain of 'AdScriptIdentifier's representing the ancestry of an ad
    /// script that led to the creation of a resource or element. The chain is
    /// ordered from the script itself (lowest level) up to its root ancestor
    /// that was flagged by a filter list.
    #[serde(rename = "ancestryChain")]
    pub ancestry_chain: Vec<AdScriptIdentifier<'a>>,
    /// The filter list rule that caused the root (last) script in
    /// 'ancestryChain' to be tagged as an ad.
    #[serde(skip_serializing_if = "Option::is_none", rename = "rootScriptFilterlistRule")]
    pub root_script_filterlist_rule: Option<Cow<'a, str>>,
}
/// Represents the provenance of an ad resource or element. Only one of
/// 'filterlistRule' or 'adScriptAncestry' can be set. If 'filterlistRule'
/// is provided, the resource URL directly matches a filter list rule. If
/// 'adScriptAncestry' is provided, an ad script initiated the resource fetch or
/// appended the element to the DOM. If neither is provided, the entity is
/// known to be an ad, but provenance tracking information is unavailable.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdProvenance<'a> {
    /// The filterlist rule that matched, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "filterlistRule")]
    pub filterlist_rule: Option<Cow<'a, str>>,
    /// The script ancestry that created the ad, if any.
    /// Note: depending on the context, this may represent the full ancestry up
    /// to the root script, or it may contain only one script representing the
    /// immediate ancestor.
    #[serde(skip_serializing_if = "Option::is_none", rename = "adScriptAncestry")]
    pub ad_script_ancestry: Option<AdAncestry<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CrossOriginOpenerPolicyValue {
    #[default]
    #[serde(rename = "SameOrigin")]
    SameOrigin,
    #[serde(rename = "SameOriginAllowPopups")]
    SameOriginAllowPopups,
    #[serde(rename = "RestrictProperties")]
    RestrictProperties,
    #[serde(rename = "UnsafeNone")]
    UnsafeNone,
    #[serde(rename = "SameOriginPlusCoep")]
    SameOriginPlusCoep,
    #[serde(rename = "RestrictPropertiesPlusCoep")]
    RestrictPropertiesPlusCoep,
    #[serde(rename = "NoopenerAllowPopups")]
    NoopenerAllowPopups,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginOpenerPolicyStatus<'a> {
    pub value: CrossOriginOpenerPolicyValue,
    #[serde(rename = "reportOnlyValue")]
    pub report_only_value: CrossOriginOpenerPolicyValue,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportingEndpoint")]
    pub reporting_endpoint: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportOnlyReportingEndpoint")]
    pub report_only_reporting_endpoint: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CrossOriginEmbedderPolicyValue {
    #[default]
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Credentialless")]
    Credentialless,
    #[serde(rename = "RequireCorp")]
    RequireCorp,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginEmbedderPolicyStatus<'a> {
    pub value: CrossOriginEmbedderPolicyValue,
    #[serde(rename = "reportOnlyValue")]
    pub report_only_value: CrossOriginEmbedderPolicyValue,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportingEndpoint")]
    pub reporting_endpoint: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportOnlyReportingEndpoint")]
    pub report_only_reporting_endpoint: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContentSecurityPolicySource {
    #[default]
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "Meta")]
    Meta,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyStatus<'a> {
    #[serde(rename = "effectiveDirectives")]
    pub effective_directives: Cow<'a, str>,
    #[serde(rename = "isEnforced")]
    pub is_enforced: bool,
    pub source: ContentSecurityPolicySource,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SecurityIsolationStatus<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coop: Option<CrossOriginOpenerPolicyStatus<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coep: Option<CrossOriginEmbedderPolicyStatus<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csp: Option<Vec<ContentSecurityPolicyStatus<'a>>>,
}
/// The status of a Reporting API report.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ReportStatus {
    #[default]
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "MarkedForRemoval")]
    MarkedForRemoval,
    #[serde(rename = "Success")]
    Success,
}


pub type ReportId<'a> = Cow<'a, str>;

/// An object representing a report generated by the Reporting API.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ReportingApiReport<'a> {
    pub id: ReportId<'a>,
    /// The URL of the document that triggered the report.
    #[serde(rename = "initiatorUrl")]
    pub initiator_url: Cow<'a, str>,
    /// The name of the endpoint group that should be used to deliver the report.
    pub destination: Cow<'a, str>,
    /// The type of the report (specifies the set of data that is contained in the report body).
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// When the report was generated.
    pub timestamp: crate::network::TimeSinceEpoch,
    /// How many uploads deep the related request was.
    pub depth: i64,
    /// The number of delivery attempts made so far, not including an active attempt.
    #[serde(rename = "completedAttempts")]
    pub completed_attempts: i64,
    pub body: serde_json::Map<String, JsonValue>,
    pub status: ReportStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ReportingApiEndpoint<'a> {
    /// The URL of the endpoint to which reports may be delivered.
    pub url: Cow<'a, str>,
    /// Name of the endpoint group.
    #[serde(rename = "groupName")]
    pub group_name: Cow<'a, str>,
}
/// Unique identifier for a device bound session.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionKey<'a> {
    /// The site the session is set up for.
    pub site: Cow<'a, str>,
    /// The id of the session.
    pub id: Cow<'a, str>,
}
/// How a device bound session was used during a request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionWithUsage<'a> {
    /// The key for the session.
    #[serde(rename = "sessionKey")]
    pub session_key: DeviceBoundSessionKey<'a>,
    /// How the session was used (or not used).
    pub usage: Cow<'a, str>,
}
/// A device bound session's cookie craving.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionCookieCraving<'a> {
    /// The name of the craving.
    pub name: Cow<'a, str>,
    /// The domain of the craving.
    pub domain: Cow<'a, str>,
    /// The path of the craving.
    pub path: Cow<'a, str>,
    /// The 'Secure' attribute of the craving attributes.
    pub secure: bool,
    /// The 'HttpOnly' attribute of the craving attributes.
    #[serde(rename = "httpOnly")]
    pub http_only: bool,
    /// The 'SameSite' attribute of the craving attributes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
    pub same_site: Option<CookieSameSite>,
}
/// A device bound session's inclusion URL rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionUrlRule<'a> {
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type'.
    #[serde(rename = "ruleType")]
    pub rule_type: Cow<'a, str>,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern'.
    #[serde(rename = "hostPattern")]
    pub host_pattern: Cow<'a, str>,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix'.
    #[serde(rename = "pathPrefix")]
    pub path_prefix: Cow<'a, str>,
}
/// A device bound session's inclusion rules.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionInclusionRules<'a> {
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::origin_'.
    pub origin: Cow<'a, str>,
    /// Whether the whole site is included. See comments on
    /// 'net::device_bound_sessions::SessionInclusionRules::include_site_' for more
    /// details; this boolean is true if that value is populated.
    #[serde(rename = "includeSite")]
    pub include_site: bool,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::url_rules_'.
    #[serde(rename = "urlRules")]
    pub url_rules: Vec<DeviceBoundSessionUrlRule<'a>>,
}
/// A device bound session.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSession<'a> {
    /// The site and session ID of the session.
    pub key: DeviceBoundSessionKey<'a>,
    /// See comments on 'net::device_bound_sessions::Session::refresh_url_'.
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Cow<'a, str>,
    /// See comments on 'net::device_bound_sessions::Session::inclusion_rules_'.
    #[serde(rename = "inclusionRules")]
    pub inclusion_rules: DeviceBoundSessionInclusionRules<'a>,
    /// See comments on 'net::device_bound_sessions::Session::cookie_cravings_'.
    #[serde(rename = "cookieCravings")]
    pub cookie_cravings: Vec<DeviceBoundSessionCookieCraving<'a>>,
    /// See comments on 'net::device_bound_sessions::Session::expiry_date_'.
    #[serde(rename = "expiryDate")]
    pub expiry_date: crate::network::TimeSinceEpoch,
    /// See comments on 'net::device_bound_sessions::Session::cached_challenge__'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cachedChallenge")]
    pub cached_challenge: Option<Cow<'a, str>>,
    /// See comments on 'net::device_bound_sessions::Session::allowed_refresh_initiators_'.
    #[serde(rename = "allowedRefreshInitiators")]
    pub allowed_refresh_initiators: Vec<Cow<'a, str>>,
}
/// A unique identifier for a device bound session event.

pub type DeviceBoundSessionEventId<'a> = Cow<'a, str>;

/// A fetch result for a device bound session creation or refresh.
/// LINT.IfChange(DeviceBoundSessionFetchResult)

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DeviceBoundSessionFetchResult {
    #[default]
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "SigningKeyGenerationError")]
    SigningKeyGenerationError,
    #[serde(rename = "AttestationKeyGenerationError")]
    AttestationKeyGenerationError,
    #[serde(rename = "SigningError")]
    SigningError,
    #[serde(rename = "TransientSigningError")]
    TransientSigningError,
    #[serde(rename = "ServerRequestedTermination")]
    ServerRequestedTermination,
    #[serde(rename = "InvalidSessionId")]
    InvalidSessionId,
    #[serde(rename = "InvalidChallenge")]
    InvalidChallenge,
    #[serde(rename = "TooManyChallenges")]
    TooManyChallenges,
    #[serde(rename = "InvalidFetcherUrl")]
    InvalidFetcherUrl,
    #[serde(rename = "InvalidRefreshUrl")]
    InvalidRefreshUrl,
    #[serde(rename = "TransientHttpError")]
    TransientHttpError,
    #[serde(rename = "ScopeOriginSameSiteMismatch")]
    ScopeOriginSameSiteMismatch,
    #[serde(rename = "RefreshUrlSameSiteMismatch")]
    RefreshUrlSameSiteMismatch,
    #[serde(rename = "MismatchedSessionId")]
    MismatchedSessionId,
    #[serde(rename = "MissingScope")]
    MissingScope,
    #[serde(rename = "NoCredentials")]
    NoCredentials,
    #[serde(rename = "SubdomainRegistrationWellKnownUnavailable")]
    SubdomainRegistrationWellKnownUnavailable,
    #[serde(rename = "SubdomainRegistrationUnauthorized")]
    SubdomainRegistrationUnauthorized,
    #[serde(rename = "SubdomainRegistrationWellKnownMalformed")]
    SubdomainRegistrationWellKnownMalformed,
    #[serde(rename = "SessionProviderWellKnownUnavailable")]
    SessionProviderWellKnownUnavailable,
    #[serde(rename = "RelyingPartyWellKnownUnavailable")]
    RelyingPartyWellKnownUnavailable,
    #[serde(rename = "FederatedKeyThumbprintMismatch")]
    FederatedKeyThumbprintMismatch,
    #[serde(rename = "InvalidFederatedSessionUrl")]
    InvalidFederatedSessionUrl,
    #[serde(rename = "InvalidFederatedKey")]
    InvalidFederatedKey,
    #[serde(rename = "TooManyRelyingOriginLabels")]
    TooManyRelyingOriginLabels,
    #[serde(rename = "BoundCookieSetForbidden")]
    BoundCookieSetForbidden,
    #[serde(rename = "NetError")]
    NetError,
    #[serde(rename = "ProxyError")]
    ProxyError,
    #[serde(rename = "EmptySessionConfig")]
    EmptySessionConfig,
    #[serde(rename = "InvalidCredentialsConfig")]
    InvalidCredentialsConfig,
    #[serde(rename = "InvalidCredentialsType")]
    InvalidCredentialsType,
    #[serde(rename = "InvalidCredentialsEmptyName")]
    InvalidCredentialsEmptyName,
    #[serde(rename = "InvalidCredentialsCookie")]
    InvalidCredentialsCookie,
    #[serde(rename = "PersistentHttpError")]
    PersistentHttpError,
    #[serde(rename = "RegistrationAttemptedChallenge")]
    RegistrationAttemptedChallenge,
    #[serde(rename = "InvalidScopeOrigin")]
    InvalidScopeOrigin,
    #[serde(rename = "ScopeOriginContainsPath")]
    ScopeOriginContainsPath,
    #[serde(rename = "RefreshInitiatorNotString")]
    RefreshInitiatorNotString,
    #[serde(rename = "RefreshInitiatorInvalidHostPattern")]
    RefreshInitiatorInvalidHostPattern,
    #[serde(rename = "InvalidScopeSpecification")]
    InvalidScopeSpecification,
    #[serde(rename = "MissingScopeSpecificationType")]
    MissingScopeSpecificationType,
    #[serde(rename = "EmptyScopeSpecificationDomain")]
    EmptyScopeSpecificationDomain,
    #[serde(rename = "EmptyScopeSpecificationPath")]
    EmptyScopeSpecificationPath,
    #[serde(rename = "InvalidScopeSpecificationType")]
    InvalidScopeSpecificationType,
    #[serde(rename = "InvalidScopeIncludeSite")]
    InvalidScopeIncludeSite,
    #[serde(rename = "MissingScopeIncludeSite")]
    MissingScopeIncludeSite,
    #[serde(rename = "FederatedNotAuthorizedByProvider")]
    FederatedNotAuthorizedByProvider,
    #[serde(rename = "FederatedNotAuthorizedByRelyingParty")]
    FederatedNotAuthorizedByRelyingParty,
    #[serde(rename = "SessionProviderWellKnownMalformed")]
    SessionProviderWellKnownMalformed,
    #[serde(rename = "SessionProviderWellKnownHasProviderOrigin")]
    SessionProviderWellKnownHasProviderOrigin,
    #[serde(rename = "RelyingPartyWellKnownMalformed")]
    RelyingPartyWellKnownMalformed,
    #[serde(rename = "RelyingPartyWellKnownHasRelyingOrigins")]
    RelyingPartyWellKnownHasRelyingOrigins,
    #[serde(rename = "InvalidFederatedSessionProviderSessionMissing")]
    InvalidFederatedSessionProviderSessionMissing,
    #[serde(rename = "InvalidFederatedSessionWrongProviderOrigin")]
    InvalidFederatedSessionWrongProviderOrigin,
    #[serde(rename = "InvalidCredentialsCookieCreationTime")]
    InvalidCredentialsCookieCreationTime,
    #[serde(rename = "InvalidCredentialsCookieName")]
    InvalidCredentialsCookieName,
    #[serde(rename = "InvalidCredentialsCookieParsing")]
    InvalidCredentialsCookieParsing,
    #[serde(rename = "InvalidCredentialsCookieUnpermittedAttribute")]
    InvalidCredentialsCookieUnpermittedAttribute,
    #[serde(rename = "InvalidCredentialsCookieInvalidDomain")]
    InvalidCredentialsCookieInvalidDomain,
    #[serde(rename = "InvalidCredentialsCookiePrefix")]
    InvalidCredentialsCookiePrefix,
    #[serde(rename = "InvalidScopeRulePath")]
    InvalidScopeRulePath,
    #[serde(rename = "InvalidScopeRuleHostPattern")]
    InvalidScopeRuleHostPattern,
    #[serde(rename = "ScopeRuleOriginScopedHostPatternMismatch")]
    ScopeRuleOriginScopedHostPatternMismatch,
    #[serde(rename = "ScopeRuleSiteScopedHostPatternMismatch")]
    ScopeRuleSiteScopedHostPatternMismatch,
    #[serde(rename = "SigningQuotaExceeded")]
    SigningQuotaExceeded,
    #[serde(rename = "InvalidConfigJson")]
    InvalidConfigJson,
    #[serde(rename = "InvalidFederatedSessionProviderFailedToRestoreKey")]
    InvalidFederatedSessionProviderFailedToRestoreKey,
    #[serde(rename = "FailedToUnwrapKey")]
    FailedToUnwrapKey,
    #[serde(rename = "SessionDeletedDuringRefresh")]
    SessionDeletedDuringRefresh,
    #[serde(rename = "CrossOriginRegistrationSiteNotIncluded")]
    CrossOriginRegistrationSiteNotIncluded,
    #[serde(rename = "InvalidPreProvisionedKeyInitiatorMissing")]
    InvalidPreProvisionedKeyInitiatorMissing,
    #[serde(rename = "PreProvisionedKeyAccessNotGranted")]
    PreProvisionedKeyAccessNotGranted,
    #[serde(rename = "PreProvisionedKeyNotFound")]
    PreProvisionedKeyNotFound,
    #[serde(rename = "AttestationCertificationError")]
    AttestationCertificationError,
    #[serde(rename = "AttestationSigningError")]
    AttestationSigningError,
}

/// Details about a failed device bound session network request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionFailedRequest<'a> {
    /// The failed request URL.
    #[serde(rename = "requestUrl")]
    pub request_url: Cow<'a, str>,
    /// The net error of the response if it was not OK.
    #[serde(skip_serializing_if = "Option::is_none", rename = "netError")]
    pub net_error: Option<Cow<'a, str>>,
    /// The response code if the net error was OK and the response code was not
    /// 200.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseError")]
    pub response_error: Option<i64>,
    /// The body of the response if the net error was OK, the response code was
    /// not 200, and the response body was not empty.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseErrorBody")]
    pub response_error_body: Option<Cow<'a, str>>,
}
/// Session event details specific to creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CreationEventDetails<'a> {
    /// The result of the fetch attempt.
    #[serde(rename = "fetchResult")]
    pub fetch_result: DeviceBoundSessionFetchResult,
    /// The session if there was a newly created session. This is populated for
    /// all successful creation events.
    #[serde(skip_serializing_if = "Option::is_none", rename = "newSession")]
    pub new_session: Option<DeviceBoundSession<'a>>,
    /// Details about a failed device bound session network request if there was
    /// one.
    #[serde(skip_serializing_if = "Option::is_none", rename = "failedRequest")]
    pub failed_request: Option<DeviceBoundSessionFailedRequest<'a>>,
}
/// Session event details specific to refresh.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RefreshEventDetails<'a> {
    /// The result of a refresh.
    /// LINT.IfChange(DeviceBoundSessionRefreshResult)
    #[serde(rename = "refreshResult")]
    pub refresh_result: Cow<'a, str>,
    /// LINT.ThenChange(//net/device_bound_sessions/refresh_result.h:DeviceBoundSessionRefreshResult,//content/browser/devtools/protocol/network_handler.cc:DeviceBoundSessionRefreshResult)
    /// If there was a fetch attempt, the result of that.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fetchResult")]
    pub fetch_result: Option<DeviceBoundSessionFetchResult>,
    /// The session display if there was a newly created session. This is populated
    /// for any refresh event that modifies the session config.
    #[serde(skip_serializing_if = "Option::is_none", rename = "newSession")]
    pub new_session: Option<DeviceBoundSession<'a>>,
    /// See comments on 'net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh'.
    #[serde(rename = "wasFullyProactiveRefresh")]
    pub was_fully_proactive_refresh: bool,
    /// Details about a failed device bound session network request if there was
    /// one.
    #[serde(skip_serializing_if = "Option::is_none", rename = "failedRequest")]
    pub failed_request: Option<DeviceBoundSessionFailedRequest<'a>>,
}
/// Session event details specific to termination.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TerminationEventDetails<'a> {
    /// The reason for a session being deleted.
    #[serde(rename = "deletionReason")]
    pub deletion_reason: Cow<'a, str>,
}
/// Session event details specific to challenges.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeEventDetails<'a> {
    /// The result of a challenge.
    #[serde(rename = "challengeResult")]
    pub challenge_result: Cow<'a, str>,
    /// The challenge set.
    pub challenge: Cow<'a, str>,
}
/// An object providing the result of a network resource load.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourcePageResult<'a> {
    pub success: bool,
    /// Optional values used for error reporting.
    #[serde(skip_serializing_if = "Option::is_none", rename = "netError")]
    pub net_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "netErrorName")]
    pub net_error_name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "httpStatusCode")]
    pub http_status_code: Option<f64>,
    /// If successful, one of the following two fields holds the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<crate::io::StreamHandle<'a>>,
    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<crate::network::Headers>,
}
/// An options object that may be extended later to better support CORS,
/// CORB and streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceOptions {
    #[serde(rename = "disableCache")]
    pub disable_cache: bool,
    #[serde(rename = "includeCredentials")]
    pub include_credentials: bool,
}
/// Tells whether clearing browser cache is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.canClearBrowserCache", response = "CanClearBrowserCacheReturns")]
pub struct CanClearBrowserCacheParams {

}
/// Tells whether clearing browser cache is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCacheReturns {
    /// True if browser cache can be cleared.
    pub result: bool,
}
/// Tells whether clearing browser cookies is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.canClearBrowserCookies", response = "CanClearBrowserCookiesReturns")]
pub struct CanClearBrowserCookiesParams {

}
/// Tells whether clearing browser cookies is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCookiesReturns {
    /// True if browser cookies can be cleared.
    pub result: bool,
}
/// Tells whether emulation of network conditions is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.canEmulateNetworkConditions", response = "CanEmulateNetworkConditionsReturns")]
pub struct CanEmulateNetworkConditionsParams {

}
/// Tells whether emulation of network conditions is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulateNetworkConditionsReturns {
    /// True if emulation of network conditions is supported.
    pub result: bool,
}
/// Clears browser cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.clearBrowserCache")]
pub struct ClearBrowserCacheParams {

}
/// Clears browser cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.clearBrowserCookies")]
pub struct ClearBrowserCookiesParams {

}
/// Deletes browser cookies with matching name and url or domain/path/partitionKey pair.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.deleteCookies")]
pub struct DeleteCookiesParams<'a> {
    /// Name of the cookies to remove.
    pub name: Cow<'a, str>,
    /// If specified, deletes all the cookies with the given name where domain and path match
    /// provided URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// If specified, deletes only cookies with the exact domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Cow<'a, str>>,
    /// If specified, deletes only cookies with the exact path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Cow<'a, str>>,
    /// If specified, deletes only cookies with the the given name and partitionKey where
    /// all partition key attributes match the cookie partition key attribute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKey")]
    pub partition_key: Option<CookiePartitionKey<'a>>,
}
/// Disables network tracking, prevents network events from being sent to the client.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.disable")]
pub struct DisableParams {

}
/// Activates emulation of network conditions. This command is deprecated in favor of the emulateNetworkConditionsByRule
/// and overrideNetworkState commands, which can be used together to the same effect.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.emulateNetworkConditions")]
pub struct EmulateNetworkConditionsParams {
    /// True to emulate internet disconnection.
    pub offline: bool,
    /// Minimum latency from request sent to response headers received (ms).
    pub latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    #[serde(rename = "downloadThroughput")]
    pub download_throughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    #[serde(rename = "uploadThroughput")]
    pub upload_throughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectionType")]
    pub connection_type: Option<ConnectionType>,
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetLoss")]
    pub packet_loss: Option<f64>,
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetQueueLength")]
    pub packet_queue_length: Option<u64>,
    /// WebRTC packetReordering feature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetReordering")]
    pub packet_reordering: Option<bool>,
}
/// Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated
/// Network.emulateNetworkConditions this method does not affect 'navigator' state. Use Network.overrideNetworkState to
/// explicitly modify 'navigator' behavior.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.emulateNetworkConditionsByRule", response = "EmulateNetworkConditionsByRuleReturns<'a>")]
pub struct EmulateNetworkConditionsByRuleParams<'a> {
    /// True to emulate internet disconnection. Deprecated, use the offline property in matchedNetworkConditions
    /// or emulateOfflineServiceWorker instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    /// True to emulate offline service worker.
    #[serde(skip_serializing_if = "Option::is_none", rename = "emulateOfflineServiceWorker")]
    pub emulate_offline_service_worker: Option<bool>,
    /// Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global
    /// conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are
    /// also applied for throttling of p2p connections.
    #[serde(rename = "matchedNetworkConditions")]
    pub matched_network_conditions: Vec<NetworkConditions<'a>>,
}
/// Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated
/// Network.emulateNetworkConditions this method does not affect 'navigator' state. Use Network.overrideNetworkState to
/// explicitly modify 'navigator' behavior.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsByRuleReturns<'a> {
    /// An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for
    /// requests affected by a rule.
    #[serde(rename = "ruleIds")]
    pub rule_ids: Vec<Cow<'a, str>>,
}
/// Override the state of navigator.onLine and navigator.connection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.overrideNetworkState")]
pub struct OverrideNetworkStateParams {
    /// True to emulate internet disconnection.
    pub offline: bool,
    /// Minimum latency from request sent to response headers received (ms).
    pub latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    #[serde(rename = "downloadThroughput")]
    pub download_throughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    #[serde(rename = "uploadThroughput")]
    pub upload_throughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectionType")]
    pub connection_type: Option<ConnectionType>,
}
/// Enables network tracking, network events will now be delivered to the client.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.enable")]
pub struct EnableParams {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    /// This is the maximum number of bytes that will be collected by this
    /// DevTools session.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxTotalBufferSize")]
    pub max_total_buffer_size: Option<u64>,
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxResourceBufferSize")]
    pub max_resource_buffer_size: Option<u64>,
    /// Longest post body size (in bytes) that would be included in requestWillBeSent notification
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxPostDataSize")]
    pub max_post_data_size: Option<u64>,
    /// Whether DirectSocket chunk send/receive events should be reported.
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportDirectSocketTraffic")]
    pub report_direct_socket_traffic: Option<bool>,
    /// Enable storing response bodies outside of renderer, so that these survive
    /// a cross-process navigation. Requires maxTotalBufferSize to be set.
    /// Currently defaults to false. This field is being deprecated in favor of the dedicated
    /// configureDurableMessages command, due to the possibility of deadlocks when awaiting
    /// Network.enable before issuing Runtime.runIfWaitingForDebugger.
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableDurableMessages")]
    pub enable_durable_messages: Option<bool>,
}
/// Configures storing response bodies outside of renderer, so that these survive
/// a cross-process navigation.
/// If maxTotalBufferSize is not set, durable messages are disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.configureDurableMessages")]
pub struct ConfigureDurableMessagesParams {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxTotalBufferSize")]
    pub max_total_buffer_size: Option<u64>,
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxResourceBufferSize")]
    pub max_resource_buffer_size: Option<u64>,
}
/// Returns all browser cookies. Depending on the backend support, will return detailed cookie
/// information in the 'cookies' field.
/// Deprecated. Use Storage.getCookies instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.getAllCookies", response = "GetAllCookiesReturns<'a>")]
pub struct GetAllCookiesParams {

}
/// Returns all browser cookies. Depending on the backend support, will return detailed cookie
/// information in the 'cookies' field.
/// Deprecated. Use Storage.getCookies instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAllCookiesReturns<'a> {
    /// Array of cookie objects.
    pub cookies: Vec<Cookie<'a>>,
}
/// Returns the DER-encoded certificate.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.getCertificate", response = "GetCertificateReturns<'a>")]
pub struct GetCertificateParams<'a> {
    /// Origin to get certificate for.
    pub origin: Cow<'a, str>,
}
/// Returns the DER-encoded certificate.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateReturns<'a> {
    #[serde(rename = "tableNames")]
    pub table_names: Vec<Cow<'a, str>>,
}
/// Returns all browser cookies for the current URL. Depending on the backend support, will return
/// detailed cookie information in the 'cookies' field.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.getCookies", response = "GetCookiesReturns<'a>")]
pub struct GetCookiesParams<'a> {
    /// The list of URLs for which applicable cookies will be fetched.
    /// If not specified, it's assumed to be set to the list containing
    /// the URLs of the page and all of its subframes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<Cow<'a, str>>>,
}
/// Returns all browser cookies for the current URL. Depending on the backend support, will return
/// detailed cookie information in the 'cookies' field.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesReturns<'a> {
    /// Array of cookie objects.
    pub cookies: Vec<Cookie<'a>>,
}
/// Returns content served for the given request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.getResponseBody", response = "GetResponseBodyReturns<'a>")]
pub struct GetResponseBodyParams<'a> {
    /// Identifier of the network request to get content for.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
}
/// Returns content served for the given request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyReturns<'a> {
    /// Response body.
    pub body: Cow<'a, str>,
    /// True, if content was sent as base64.
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
/// Returns post data sent with the request. Returns an error when no data was sent with the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.getRequestPostData", response = "GetRequestPostDataReturns<'a>")]
pub struct GetRequestPostDataParams<'a> {
    /// Identifier of the network request to get content for.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
}
/// Returns post data sent with the request. Returns an error when no data was sent with the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetRequestPostDataReturns<'a> {
    /// Request body string, omitting files from multipart requests
    #[serde(rename = "postData")]
    pub post_data: Cow<'a, str>,
    /// True, if content was sent as base64.
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
/// This method sends a new XMLHttpRequest which is identical to the original one. The following
/// parameters should be identical: method, url, async, request body, extra headers, withCredentials
/// attribute, user, password.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.replayXHR")]
pub struct ReplayXHRParams<'a> {
    /// Identifier of XHR to replay.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
}
/// Searches for given string in response content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.searchInResponseBody", response = "SearchInResponseBodyReturns")]
pub struct SearchInResponseBodyParams<'a> {
    /// Identifier of the network response to search.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// String to search for.
    pub query: Cow<'a, str>,
    /// If true, search is case sensitive.
    #[serde(skip_serializing_if = "Option::is_none", rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    /// If true, treats string parameter as regex.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isRegex")]
    pub is_regex: Option<bool>,
}
/// Searches for given string in response content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResponseBodyReturns {
    /// List of search matches.
    pub result: Vec<crate::debugger::SearchMatch>,
}
/// Blocks URLs from loading.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.setBlockedURLs")]
pub struct SetBlockedURLsParams<'a> {
    /// Patterns to match in the order in which they are given. These patterns
    /// also take precedence over any wildcard patterns defined in 'urls'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlPatterns")]
    pub url_patterns: Option<Vec<BlockPattern<'a>>>,
    /// URL patterns to block. Wildcards ('*') are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<Cow<'a, str>>>,
}
/// Toggles ignoring of service worker for each request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.setBypassServiceWorker")]
pub struct SetBypassServiceWorkerParams {
    /// Bypass service worker and load from network.
    pub bypass: bool,
}
/// Toggles ignoring cache for each request. If 'true', cache will not be used.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.setCacheDisabled")]
pub struct SetCacheDisabledParams {
    /// Cache disabled state.
    #[serde(rename = "cacheDisabled")]
    pub cache_disabled: bool,
}
/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.setCookie", response = "SetCookieReturns")]
pub struct SetCookieParams<'a> {
    /// Cookie name.
    pub name: Cow<'a, str>,
    /// Cookie value.
    pub value: Cow<'a, str>,
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// Cookie domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Cow<'a, str>>,
    /// Cookie path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Cow<'a, str>>,
    /// True if cookie is secure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// True if cookie is http-only.
    #[serde(skip_serializing_if = "Option::is_none", rename = "httpOnly")]
    pub http_only: Option<bool>,
    /// Cookie SameSite type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
    pub same_site: Option<CookieSameSite>,
    /// Cookie expiration date, session cookie if not set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<TimeSinceEpoch>,
    /// Cookie Priority type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<CookiePriority>,
    /// Cookie source scheme type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceScheme")]
    pub source_scheme: Option<CookieSourceScheme>,
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourcePort")]
    pub source_port: Option<i64>,
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKey")]
    pub partition_key: Option<CookiePartitionKey<'a>>,
}
/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieReturns {
    /// Always set to true. If an error occurs, the response indicates protocol error.
    pub success: bool,
}
/// Sets given cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.setCookies")]
pub struct SetCookiesParams<'a> {
    /// Cookies to be set.
    pub cookies: Vec<CookieParam<'a>>,
}
/// Specifies whether to always send extra HTTP headers with the requests from this page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.setExtraHTTPHeaders")]
pub struct SetExtraHTTPHeadersParams {
    /// Map with extra HTTP headers.
    pub headers: Headers,
}
/// Specifies whether to attach a page script stack id in requests

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.setAttachDebugStack")]
pub struct SetAttachDebugStackParams {
    /// Whether to attach a page script stack for debugging purpose.
    pub enabled: bool,
}
/// Allows overriding user agent with the given string.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.setUserAgentOverride")]
pub struct SetUserAgentOverrideParams<'a> {
    /// User agent to use.
    #[serde(rename = "userAgent")]
    pub user_agent: Cow<'a, str>,
    /// Browser language to emulate.
    #[serde(skip_serializing_if = "Option::is_none", rename = "acceptLanguage")]
    pub accept_language: Option<Cow<'a, str>>,
    /// The platform navigator.platform should return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Cow<'a, str>>,
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    #[serde(skip_serializing_if = "Option::is_none", rename = "userAgentMetadata")]
    pub user_agent_metadata: Option<crate::emulation::UserAgentMetadata<'a>>,
}
/// Enables streaming of the response for the given requestId.
/// If enabled, the dataReceived event contains the data that was received during streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.streamResourceContent", response = "StreamResourceContentReturns<'a>")]
pub struct StreamResourceContentParams<'a> {
    /// Identifier of the request to stream.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
}
/// Enables streaming of the response for the given requestId.
/// If enabled, the dataReceived event contains the data that was received during streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StreamResourceContentReturns<'a> {
    /// Data that has been buffered until streaming is enabled. (Encoded as a base64 string when passed over JSON)
    #[serde(rename = "bufferedData")]
    pub buffered_data: Cow<'a, str>,
}
/// Returns information about the COEP/COOP isolation status.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.getSecurityIsolationStatus", response = "GetSecurityIsolationStatusReturns<'a>")]
pub struct GetSecurityIsolationStatusParams<'a> {
    /// If no frameId is provided, the status of the target is provided.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
}
/// Returns information about the COEP/COOP isolation status.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetSecurityIsolationStatusReturns<'a> {
    pub status: SecurityIsolationStatus<'a>,
}
/// Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.
/// Enabling triggers 'reportingApiReportAdded' for all existing reports.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.enableReportingApi")]
pub struct EnableReportingApiParams {
    /// Whether to enable or disable events for the Reporting API
    pub enable: bool,
}
/// Sets up tracking device bound sessions and fetching of initial set of sessions.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.enableDeviceBoundSessions")]
pub struct EnableDeviceBoundSessionsParams {
    /// Whether to enable or disable events.
    pub enable: bool,
}
/// Deletes a device bound session.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.deleteDeviceBoundSession")]
pub struct DeleteDeviceBoundSessionParams<'a> {
    pub key: DeviceBoundSessionKey<'a>,
}
/// Fetches the schemeful site for a specific origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.fetchSchemefulSite", response = "FetchSchemefulSiteReturns<'a>")]
pub struct FetchSchemefulSiteParams<'a> {
    /// The URL origin.
    pub origin: Cow<'a, str>,
}
/// Fetches the schemeful site for a specific origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FetchSchemefulSiteReturns<'a> {
    /// The corresponding schemeful site.
    #[serde(rename = "schemefulSite")]
    pub schemeful_site: Cow<'a, str>,
}
/// Fetches the resource and returns the content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.loadNetworkResource", response = "LoadNetworkResourceReturns<'a>")]
pub struct LoadNetworkResourceParams<'a> {
    /// Frame id to get the resource for. Mandatory for frame targets, and
    /// should be omitted for worker targets.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
    /// URL of the resource to get content for.
    pub url: Cow<'a, str>,
    /// Options for the request.
    pub options: LoadNetworkResourceOptions,
}
/// Fetches the resource and returns the content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceReturns<'a> {
    pub resource: LoadNetworkResourcePageResult<'a>,
}
/// Sets Controls for third-party cookie access
/// Page reload is required before the new cookie behavior will be observed

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.setCookieControls")]
pub struct SetCookieControlsParams {
    /// Whether 3pc restriction is enabled.
    #[serde(rename = "enableThirdPartyCookieRestriction")]
    pub enable_third_party_cookie_restriction: bool,
}
/// Fired when data chunk was received over the network.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.dataReceived")]
pub struct DataReceived<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// Data chunk length.
    #[serde(rename = "dataLength")]
    pub data_length: u64,
    /// Actual bytes received (might be less than dataLength for compressed encodings).
    #[serde(rename = "encodedDataLength")]
    pub encoded_data_length: u64,
    /// Data that was received. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Cow<'a, str>>,
}
/// Fired when EventSource message is received.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.eventSourceMessageReceived")]
pub struct EventSourceMessageReceived<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// Message type.
    #[serde(rename = "eventName")]
    pub event_name: Cow<'a, str>,
    /// Message identifier.
    #[serde(rename = "eventId")]
    pub event_id: Cow<'a, str>,
    /// Message content.
    pub data: Cow<'a, str>,
}
/// Fired when HTTP request has failed to load.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.loadingFailed")]
pub struct LoadingFailed<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// Resource type.
    #[serde(rename = "type")]
    pub type_: ResourceType,
    /// Error message. List of network errors: <https://cs.chromium.org/chromium/src/net/base/net_error_list.h>
    #[serde(rename = "errorText")]
    pub error_text: Cow<'a, str>,
    /// True if loading was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<bool>,
    /// The reason why loading was blocked, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockedReason")]
    pub blocked_reason: Option<BlockedReason>,
    /// The reason why loading was blocked by CORS, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "corsErrorStatus")]
    pub cors_error_status: Option<CorsErrorStatus<'a>>,
}
/// Fired when HTTP request has finished loading.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.loadingFinished")]
pub struct LoadingFinished<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// Total number of bytes received for this request.
    #[serde(rename = "encodedDataLength")]
    pub encoded_data_length: f64,
}
/// Fired if request ended up loading from cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.requestServedFromCache")]
pub struct RequestServedFromCache<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
}
/// Fired when page is about to send HTTP request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.requestWillBeSent")]
pub struct RequestWillBeSent<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Loader identifier. Empty string if the request is fetched from worker.
    #[serde(rename = "loaderId")]
    pub loader_id: LoaderId<'a>,
    /// URL of the document this request is loaded for.
    #[serde(rename = "documentURL")]
    pub document_url: Cow<'a, str>,
    /// Request data.
    pub request: Request<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// Timestamp.
    #[serde(rename = "wallTime")]
    pub wall_time: TimeSinceEpoch,
    /// Request initiator.
    pub initiator: Initiator<'a>,
    /// In the case that redirectResponse is populated, this flag indicates whether
    /// requestWillBeSentExtraInfo and responseReceivedExtraInfo events will be or were emitted
    /// for the request which was just redirected.
    #[serde(rename = "redirectHasExtraInfo")]
    pub redirect_has_extra_info: bool,
    /// Redirect response data.
    #[serde(skip_serializing_if = "Option::is_none", rename = "redirectResponse")]
    pub redirect_response: Option<Response<'a>>,
    /// Type of this resource.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<ResourceType>,
    /// Frame identifier.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
    /// Whether the request is initiated by a user gesture. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasUserGesture")]
    pub has_user_gesture: Option<bool>,
    /// The render-blocking behavior of the request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "renderBlockingBehavior")]
    pub render_blocking_behavior: Option<RenderBlockingBehavior>,
}
/// Fired when resource loading priority is changed

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.resourceChangedPriority")]
pub struct ResourceChangedPriority<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// New priority
    #[serde(rename = "newPriority")]
    pub new_priority: ResourcePriority,
    /// Timestamp.
    pub timestamp: MonotonicTime,
}
/// Fired when a signed exchange was received over the network

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.signedExchangeReceived")]
pub struct SignedExchangeReceived<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Information about the signed exchange response.
    pub info: SignedExchangeInfo<'a>,
}
/// Fired when HTTP response is available.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.responseReceived")]
pub struct ResponseReceived<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Loader identifier. Empty string if the request is fetched from worker.
    #[serde(rename = "loaderId")]
    pub loader_id: LoaderId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// Resource type.
    #[serde(rename = "type")]
    pub type_: ResourceType,
    /// Response data.
    pub response: Response<'a>,
    /// Indicates whether requestWillBeSentExtraInfo and responseReceivedExtraInfo events will be
    /// or were emitted for this request.
    #[serde(rename = "hasExtraInfo")]
    pub has_extra_info: bool,
    /// Frame identifier.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
}
/// Fired when WebSocket is closed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webSocketClosed")]
pub struct WebSocketClosed<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
}
/// Fired upon WebSocket creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webSocketCreated")]
pub struct WebSocketCreated<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// WebSocket request URL.
    pub url: Cow<'a, str>,
    /// Request initiator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator<'a>>,
}
/// Fired when WebSocket message error occurs.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webSocketFrameError")]
pub struct WebSocketFrameError<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// WebSocket error message.
    #[serde(rename = "errorMessage")]
    pub error_message: Cow<'a, str>,
}
/// Fired when WebSocket message is received.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webSocketFrameReceived")]
pub struct WebSocketFrameReceived<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// WebSocket response data.
    pub response: WebSocketFrame<'a>,
}
/// Fired when WebSocket message is sent.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webSocketFrameSent")]
pub struct WebSocketFrameSent<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// WebSocket response data.
    pub response: WebSocketFrame<'a>,
}
/// Fired when WebSocket handshake response becomes available.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webSocketHandshakeResponseReceived")]
pub struct WebSocketHandshakeResponseReceived<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// WebSocket response data.
    pub response: WebSocketResponse<'a>,
}
/// Fired when WebSocket is about to initiate handshake.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webSocketWillSendHandshakeRequest")]
pub struct WebSocketWillSendHandshakeRequest<'a> {
    /// Request identifier.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// UTC Timestamp.
    #[serde(rename = "wallTime")]
    pub wall_time: TimeSinceEpoch,
    /// WebSocket request data.
    pub request: WebSocketRequest,
}
/// Fired upon WebTransport creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webTransportCreated")]
pub struct WebTransportCreated<'a> {
    /// WebTransport identifier.
    #[serde(rename = "transportId")]
    pub transport_id: RequestId<'a>,
    /// WebTransport request URL.
    pub url: Cow<'a, str>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// Request initiator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator<'a>>,
}
/// Fired when WebTransport handshake is finished.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webTransportConnectionEstablished")]
pub struct WebTransportConnectionEstablished<'a> {
    /// WebTransport identifier.
    #[serde(rename = "transportId")]
    pub transport_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
}
/// Fired when WebTransport is disposed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.webTransportClosed")]
pub struct WebTransportClosed<'a> {
    /// WebTransport identifier.
    #[serde(rename = "transportId")]
    pub transport_id: RequestId<'a>,
    /// Timestamp.
    pub timestamp: MonotonicTime,
}
/// Fired upon direct_socket.TCPSocket creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directTCPSocketCreated")]
pub struct DirectTCPSocketCreated<'a> {
    pub identifier: RequestId<'a>,
    #[serde(rename = "remoteAddr")]
    pub remote_addr: Cow<'a, str>,
    /// Unsigned int 16.
    #[serde(rename = "remotePort")]
    pub remote_port: i64,
    pub options: DirectTCPSocketOptions,
    pub timestamp: MonotonicTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator<'a>>,
}
/// Fired when direct_socket.TCPSocket connection is opened.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directTCPSocketOpened")]
pub struct DirectTCPSocketOpened<'a> {
    pub identifier: RequestId<'a>,
    #[serde(rename = "remoteAddr")]
    pub remote_addr: Cow<'a, str>,
    /// Expected to be unsigned integer.
    #[serde(rename = "remotePort")]
    pub remote_port: i64,
    pub timestamp: MonotonicTime,
    #[serde(skip_serializing_if = "Option::is_none", rename = "localAddr")]
    pub local_addr: Option<Cow<'a, str>>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "localPort")]
    pub local_port: Option<i64>,
}
/// Fired when direct_socket.TCPSocket is aborted.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directTCPSocketAborted")]
pub struct DirectTCPSocketAborted<'a> {
    pub identifier: RequestId<'a>,
    #[serde(rename = "errorMessage")]
    pub error_message: ErrorReason,
    pub timestamp: MonotonicTime,
}
/// Fired when direct_socket.TCPSocket is closed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directTCPSocketClosed")]
pub struct DirectTCPSocketClosed<'a> {
    pub identifier: RequestId<'a>,
    pub timestamp: MonotonicTime,
}
/// Fired when data is sent to tcp direct socket stream.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directTCPSocketChunkSent")]
pub struct DirectTCPSocketChunkSent<'a> {
    pub identifier: RequestId<'a>,
    pub data: Cow<'a, str>,
    pub timestamp: MonotonicTime,
}
/// Fired when data is received from tcp direct socket stream.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directTCPSocketChunkReceived")]
pub struct DirectTCPSocketChunkReceived<'a> {
    pub identifier: RequestId<'a>,
    pub data: Cow<'a, str>,
    pub timestamp: MonotonicTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directUDPSocketJoinedMulticastGroup")]
pub struct DirectUDPSocketJoinedMulticastGroup<'a> {
    pub identifier: RequestId<'a>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directUDPSocketLeftMulticastGroup")]
pub struct DirectUDPSocketLeftMulticastGroup<'a> {
    pub identifier: RequestId<'a>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Cow<'a, str>,
}
/// Fired upon direct_socket.UDPSocket creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directUDPSocketCreated")]
pub struct DirectUDPSocketCreated<'a> {
    pub identifier: RequestId<'a>,
    pub options: DirectUDPSocketOptions<'a>,
    pub timestamp: MonotonicTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator<'a>>,
}
/// Fired when direct_socket.UDPSocket connection is opened.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directUDPSocketOpened")]
pub struct DirectUDPSocketOpened<'a> {
    pub identifier: RequestId<'a>,
    #[serde(rename = "localAddr")]
    pub local_addr: Cow<'a, str>,
    /// Expected to be unsigned integer.
    #[serde(rename = "localPort")]
    pub local_port: i64,
    pub timestamp: MonotonicTime,
    #[serde(skip_serializing_if = "Option::is_none", rename = "remoteAddr")]
    pub remote_addr: Option<Cow<'a, str>>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remotePort")]
    pub remote_port: Option<i64>,
}
/// Fired when direct_socket.UDPSocket is aborted.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directUDPSocketAborted")]
pub struct DirectUDPSocketAborted<'a> {
    pub identifier: RequestId<'a>,
    #[serde(rename = "errorMessage")]
    pub error_message: ErrorReason,
    pub timestamp: MonotonicTime,
}
/// Fired when direct_socket.UDPSocket is closed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directUDPSocketClosed")]
pub struct DirectUDPSocketClosed<'a> {
    pub identifier: RequestId<'a>,
    pub timestamp: MonotonicTime,
}
/// Fired when message is sent to udp direct socket stream.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directUDPSocketChunkSent")]
pub struct DirectUDPSocketChunkSent<'a> {
    pub identifier: RequestId<'a>,
    pub message: DirectUDPMessage<'a>,
    pub timestamp: MonotonicTime,
}
/// Fired when message is received from udp direct socket stream.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.directUDPSocketChunkReceived")]
pub struct DirectUDPSocketChunkReceived<'a> {
    pub identifier: RequestId<'a>,
    pub message: DirectUDPMessage<'a>,
    pub timestamp: MonotonicTime,
}
/// Fired when additional information about a requestWillBeSent event is available from the
/// network stack. Not every requestWillBeSent event will have an additional
/// requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
/// or requestWillBeSentExtraInfo will be fired first for the same request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.requestWillBeSentExtraInfo")]
pub struct RequestWillBeSentExtraInfo<'a> {
    /// Request identifier. Used to match this information to an existing requestWillBeSent event.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// A list of cookies potentially associated to the requested URL. This includes both cookies sent with
    /// the request and the ones not sent; the latter are distinguished by having blockedReasons field set.
    #[serde(rename = "associatedCookies")]
    pub associated_cookies: Vec<AssociatedCookie<'a>>,
    /// Raw request headers as they will be sent over the wire.
    pub headers: Headers,
    /// Connection timing information for the request.
    #[serde(rename = "connectTiming")]
    pub connect_timing: ConnectTiming,
    /// How the request site's device bound sessions were used during this request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "deviceBoundSessionUsages")]
    pub device_bound_session_usages: Option<Vec<DeviceBoundSessionWithUsage<'a>>>,
    /// The client security state set for the request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientSecurityState")]
    pub client_security_state: Option<ClientSecurityState>,
    /// Whether the site has partitioned cookies stored in a partition different than the current one.
    #[serde(skip_serializing_if = "Option::is_none", rename = "siteHasCookieInOtherPartition")]
    pub site_has_cookie_in_other_partition: Option<bool>,
    /// The network conditions id if this request was affected by network conditions configured via
    /// emulateNetworkConditionsByRule.
    #[serde(skip_serializing_if = "Option::is_none", rename = "appliedNetworkConditionsId")]
    pub applied_network_conditions_id: Option<Cow<'a, str>>,
}
/// Fired when additional information about a responseReceived event is available from the network
/// stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
/// it, and responseReceivedExtraInfo may be fired before or after responseReceived.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.responseReceivedExtraInfo")]
pub struct ResponseReceivedExtraInfo<'a> {
    /// Request identifier. Used to match this information to another responseReceived event.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// A list of cookies which were not stored from the response along with the corresponding
    /// reasons for blocking. The cookies here may not be valid due to syntax errors, which
    /// are represented by the invalid cookie line string instead of a proper cookie.
    #[serde(rename = "blockedCookies")]
    pub blocked_cookies: Vec<BlockedSetCookieWithReason<'a>>,
    /// Raw response headers as they were received over the wire.
    /// Duplicate headers in the response are represented as a single key with their values
    /// concatentated using '
    /// ' as the separator.
    /// See also 'headersText' that contains verbatim text for HTTP/1.*.
    pub headers: Headers,
    /// The IP address space of the resource. The address space can only be determined once the transport
    /// established the connection, so we can't send it in 'requestWillBeSentExtraInfo'.
    #[serde(rename = "resourceIPAddressSpace")]
    pub resource_ip_address_space: IPAddressSpace,
    /// The status code of the response. This is useful in cases the request failed and no responseReceived
    /// event is triggered, which is the case for, e.g., CORS errors. This is also the correct status code
    /// for cached requests, where the status in responseReceived is a 200 and this will be 304.
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    /// Raw response header text as it was received over the wire. The raw text may not always be
    /// available, such as in the case of HTTP/2 or QUIC.
    #[serde(skip_serializing_if = "Option::is_none", rename = "headersText")]
    pub headers_text: Option<Cow<'a, str>>,
    /// The cookie partition key that will be used to store partitioned cookies set in this response.
    /// Only sent when partitioned cookies are enabled.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cookiePartitionKey")]
    pub cookie_partition_key: Option<CookiePartitionKey<'a>>,
    /// True if partitioned cookies are enabled, but the partition key is not serializable to string.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cookiePartitionKeyOpaque")]
    pub cookie_partition_key_opaque: Option<bool>,
    /// A list of cookies which should have been blocked by 3PCD but are exempted and stored from
    /// the response with the corresponding reason.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exemptedCookies")]
    pub exempted_cookies: Option<Vec<ExemptedSetCookieWithReason<'a>>>,
}
/// Fired when 103 Early Hints headers is received in addition to the common response.
/// Not every responseReceived event will have an responseReceivedEarlyHints fired.
/// Only one responseReceivedEarlyHints may be fired for eached responseReceived event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.responseReceivedEarlyHints")]
pub struct ResponseReceivedEarlyHints<'a> {
    /// Request identifier. Used to match this information to another responseReceived event.
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Raw response headers as they were received over the wire.
    /// Duplicate headers in the response are represented as a single key with their values
    /// concatentated using '
    /// ' as the separator.
    /// See also 'headersText' that contains verbatim text for HTTP/1.*.
    pub headers: Headers,
}
/// Fired exactly once for each Trust Token operation. Depending on
/// the type of the operation and whether the operation succeeded or
/// failed, the event is fired before the corresponding request was sent
/// or after the response was received.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.trustTokenOperationDone")]
pub struct TrustTokenOperationDone<'a> {
    /// Detailed success or error status of the operation.
    /// 'AlreadyExists' also signifies a successful operation, as the result
    /// of the operation already exists und thus, the operation was abort
    /// preemptively (e.g. a cache hit).
    pub status: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_: TrustTokenOperationType,
    #[serde(rename = "requestId")]
    pub request_id: RequestId<'a>,
    /// Top level origin. The context in which the operation was attempted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "topLevelOrigin")]
    pub top_level_origin: Option<Cow<'a, str>>,
    /// Origin of the issuer in case of a "Issuance" or "Redemption" operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "issuerOrigin")]
    pub issuer_origin: Option<Cow<'a, str>>,
    /// The number of obtained Trust Tokens on a successful "Issuance" operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "issuedTokenCount")]
    pub issued_token_count: Option<u64>,
}
/// Fired once security policy has been updated.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.policyUpdated")]
pub struct PolicyUpdated {

}
/// Is sent whenever a new report is added.
/// And after 'enableReportingApi' for all existing reports.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.reportingApiReportAdded")]
pub struct ReportingApiReportAdded<'a> {
    pub report: ReportingApiReport<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.reportingApiReportUpdated")]
pub struct ReportingApiReportUpdated<'a> {
    pub report: ReportingApiReport<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.reportingApiEndpointsChangedForOrigin")]
pub struct ReportingApiEndpointsChangedForOrigin<'a> {
    /// Origin of the document(s) which configured the endpoints.
    pub origin: Cow<'a, str>,
    pub endpoints: Vec<ReportingApiEndpoint<'a>>,
}
/// Triggered when the initial set of device bound sessions is added.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.deviceBoundSessionsAdded")]
pub struct DeviceBoundSessionsAdded<'a> {
    /// The device bound sessions.
    pub sessions: Vec<DeviceBoundSession<'a>>,
}
/// Triggered when a device bound session event occurs.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Network.deviceBoundSessionEventOccurred")]
pub struct DeviceBoundSessionEventOccurred<'a> {
    /// A unique identifier for this session event.
    #[serde(rename = "eventId")]
    pub event_id: DeviceBoundSessionEventId<'a>,
    /// The site this session event is associated with.
    pub site: Cow<'a, str>,
    /// Whether this event was considered successful.
    pub succeeded: bool,
    /// The session ID this event is associated with. May not be populated for
    /// failed events.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sessionId")]
    pub session_id: Option<Cow<'a, str>>,
    /// The below are the different session event type details. Exactly one is populated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "creationEventDetails")]
    pub creation_event_details: Option<CreationEventDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "refreshEventDetails")]
    pub refresh_event_details: Option<RefreshEventDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "terminationEventDetails")]
    pub termination_event_details: Option<TerminationEventDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "challengeEventDetails")]
    pub challenge_event_details: Option<ChallengeEventDetails<'a>>,
}