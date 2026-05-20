//! Network domain allows tracking network activities of the page. It exposes information about http,
//! file, data and other requests and responses, their headers, bodies, timing, etc.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

/// Unique intercepted request identifier.

pub type InterceptionId<'a> = Cow<'a, str>;

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
/// https://tools.ietf.org/html/draft-west-first-party-cookies

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
/// https://tools.ietf.org/html/draft-west-cookie-priority-00

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTiming {
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime.
    requestTime: f64,
    /// Started resolving proxy.
    proxyStart: f64,
    /// Finished resolving proxy.
    proxyEnd: f64,
    /// Started DNS address resolve.
    dnsStart: f64,
    /// Finished DNS address resolve.
    dnsEnd: f64,
    /// Started connecting to the remote host.
    connectStart: f64,
    /// Connected to the remote host.
    connectEnd: f64,
    /// Started SSL handshake.
    sslStart: f64,
    /// Finished SSL handshake.
    sslEnd: f64,
    /// Started running ServiceWorker.
    workerStart: f64,
    /// Finished Starting ServiceWorker.
    workerReady: f64,
    /// Started fetch event.
    workerFetchStart: f64,
    /// Settled fetch event respondWith promise.
    workerRespondWithSettled: f64,
    /// Started ServiceWorker static routing source evaluation.
    #[serde(skip_serializing_if = "Option::is_none")]
    workerRouterEvaluationStart: Option<f64>,
    /// Started cache lookup when the source was evaluated to 'cache'.
    #[serde(skip_serializing_if = "Option::is_none")]
    workerCacheLookupStart: Option<f64>,
    /// Started sending request.
    sendStart: f64,
    /// Finished sending request.
    sendEnd: f64,
    /// Time the server started pushing request.
    pushStart: f64,
    /// Time the server finished pushing request.
    pushEnd: f64,
    /// Started receiving response headers.
    receiveHeadersStart: f64,
    /// Finished receiving response headers.
    receiveHeadersEnd: f64,
}

impl ResourceTiming {
    pub fn builder() -> ResourceTimingBuilder { ResourceTimingBuilder::default() }
    pub fn requestTime(&self) -> f64 { self.requestTime }
    pub fn proxyStart(&self) -> f64 { self.proxyStart }
    pub fn proxyEnd(&self) -> f64 { self.proxyEnd }
    pub fn dnsStart(&self) -> f64 { self.dnsStart }
    pub fn dnsEnd(&self) -> f64 { self.dnsEnd }
    pub fn connectStart(&self) -> f64 { self.connectStart }
    pub fn connectEnd(&self) -> f64 { self.connectEnd }
    pub fn sslStart(&self) -> f64 { self.sslStart }
    pub fn sslEnd(&self) -> f64 { self.sslEnd }
    pub fn workerStart(&self) -> f64 { self.workerStart }
    pub fn workerReady(&self) -> f64 { self.workerReady }
    pub fn workerFetchStart(&self) -> f64 { self.workerFetchStart }
    pub fn workerRespondWithSettled(&self) -> f64 { self.workerRespondWithSettled }
    pub fn workerRouterEvaluationStart(&self) -> Option<f64> { self.workerRouterEvaluationStart }
    pub fn workerCacheLookupStart(&self) -> Option<f64> { self.workerCacheLookupStart }
    pub fn sendStart(&self) -> f64 { self.sendStart }
    pub fn sendEnd(&self) -> f64 { self.sendEnd }
    pub fn pushStart(&self) -> f64 { self.pushStart }
    pub fn pushEnd(&self) -> f64 { self.pushEnd }
    pub fn receiveHeadersStart(&self) -> f64 { self.receiveHeadersStart }
    pub fn receiveHeadersEnd(&self) -> f64 { self.receiveHeadersEnd }
}

#[derive(Default)]
pub struct ResourceTimingBuilder {
    requestTime: Option<f64>,
    proxyStart: Option<f64>,
    proxyEnd: Option<f64>,
    dnsStart: Option<f64>,
    dnsEnd: Option<f64>,
    connectStart: Option<f64>,
    connectEnd: Option<f64>,
    sslStart: Option<f64>,
    sslEnd: Option<f64>,
    workerStart: Option<f64>,
    workerReady: Option<f64>,
    workerFetchStart: Option<f64>,
    workerRespondWithSettled: Option<f64>,
    workerRouterEvaluationStart: Option<f64>,
    workerCacheLookupStart: Option<f64>,
    sendStart: Option<f64>,
    sendEnd: Option<f64>,
    pushStart: Option<f64>,
    pushEnd: Option<f64>,
    receiveHeadersStart: Option<f64>,
    receiveHeadersEnd: Option<f64>,
}

impl ResourceTimingBuilder {
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime.
    pub fn requestTime(mut self, requestTime: f64) -> Self { self.requestTime = Some(requestTime); self }
    /// Started resolving proxy.
    pub fn proxyStart(mut self, proxyStart: f64) -> Self { self.proxyStart = Some(proxyStart); self }
    /// Finished resolving proxy.
    pub fn proxyEnd(mut self, proxyEnd: f64) -> Self { self.proxyEnd = Some(proxyEnd); self }
    /// Started DNS address resolve.
    pub fn dnsStart(mut self, dnsStart: f64) -> Self { self.dnsStart = Some(dnsStart); self }
    /// Finished DNS address resolve.
    pub fn dnsEnd(mut self, dnsEnd: f64) -> Self { self.dnsEnd = Some(dnsEnd); self }
    /// Started connecting to the remote host.
    pub fn connectStart(mut self, connectStart: f64) -> Self { self.connectStart = Some(connectStart); self }
    /// Connected to the remote host.
    pub fn connectEnd(mut self, connectEnd: f64) -> Self { self.connectEnd = Some(connectEnd); self }
    /// Started SSL handshake.
    pub fn sslStart(mut self, sslStart: f64) -> Self { self.sslStart = Some(sslStart); self }
    /// Finished SSL handshake.
    pub fn sslEnd(mut self, sslEnd: f64) -> Self { self.sslEnd = Some(sslEnd); self }
    /// Started running ServiceWorker.
    pub fn workerStart(mut self, workerStart: f64) -> Self { self.workerStart = Some(workerStart); self }
    /// Finished Starting ServiceWorker.
    pub fn workerReady(mut self, workerReady: f64) -> Self { self.workerReady = Some(workerReady); self }
    /// Started fetch event.
    pub fn workerFetchStart(mut self, workerFetchStart: f64) -> Self { self.workerFetchStart = Some(workerFetchStart); self }
    /// Settled fetch event respondWith promise.
    pub fn workerRespondWithSettled(mut self, workerRespondWithSettled: f64) -> Self { self.workerRespondWithSettled = Some(workerRespondWithSettled); self }
    /// Started ServiceWorker static routing source evaluation.
    pub fn workerRouterEvaluationStart(mut self, workerRouterEvaluationStart: f64) -> Self { self.workerRouterEvaluationStart = Some(workerRouterEvaluationStart); self }
    /// Started cache lookup when the source was evaluated to 'cache'.
    pub fn workerCacheLookupStart(mut self, workerCacheLookupStart: f64) -> Self { self.workerCacheLookupStart = Some(workerCacheLookupStart); self }
    /// Started sending request.
    pub fn sendStart(mut self, sendStart: f64) -> Self { self.sendStart = Some(sendStart); self }
    /// Finished sending request.
    pub fn sendEnd(mut self, sendEnd: f64) -> Self { self.sendEnd = Some(sendEnd); self }
    /// Time the server started pushing request.
    pub fn pushStart(mut self, pushStart: f64) -> Self { self.pushStart = Some(pushStart); self }
    /// Time the server finished pushing request.
    pub fn pushEnd(mut self, pushEnd: f64) -> Self { self.pushEnd = Some(pushEnd); self }
    /// Started receiving response headers.
    pub fn receiveHeadersStart(mut self, receiveHeadersStart: f64) -> Self { self.receiveHeadersStart = Some(receiveHeadersStart); self }
    /// Finished receiving response headers.
    pub fn receiveHeadersEnd(mut self, receiveHeadersEnd: f64) -> Self { self.receiveHeadersEnd = Some(receiveHeadersEnd); self }
    pub fn build(self) -> ResourceTiming {
        ResourceTiming {
            requestTime: self.requestTime.unwrap_or_default(),
            proxyStart: self.proxyStart.unwrap_or_default(),
            proxyEnd: self.proxyEnd.unwrap_or_default(),
            dnsStart: self.dnsStart.unwrap_or_default(),
            dnsEnd: self.dnsEnd.unwrap_or_default(),
            connectStart: self.connectStart.unwrap_or_default(),
            connectEnd: self.connectEnd.unwrap_or_default(),
            sslStart: self.sslStart.unwrap_or_default(),
            sslEnd: self.sslEnd.unwrap_or_default(),
            workerStart: self.workerStart.unwrap_or_default(),
            workerReady: self.workerReady.unwrap_or_default(),
            workerFetchStart: self.workerFetchStart.unwrap_or_default(),
            workerRespondWithSettled: self.workerRespondWithSettled.unwrap_or_default(),
            workerRouterEvaluationStart: self.workerRouterEvaluationStart,
            workerCacheLookupStart: self.workerCacheLookupStart,
            sendStart: self.sendStart.unwrap_or_default(),
            sendEnd: self.sendEnd.unwrap_or_default(),
            pushStart: self.pushStart.unwrap_or_default(),
            pushEnd: self.pushEnd.unwrap_or_default(),
            receiveHeadersStart: self.receiveHeadersStart.unwrap_or_default(),
            receiveHeadersEnd: self.receiveHeadersEnd.unwrap_or_default(),
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostDataEntry<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes: Option<Cow<'a, str>>,
}

impl<'a> PostDataEntry<'a> {
    pub fn builder() -> PostDataEntryBuilder<'a> { PostDataEntryBuilder::default() }
    pub fn bytes(&self) -> Option<&str> { self.bytes.as_deref() }
}

#[derive(Default)]
pub struct PostDataEntryBuilder<'a> {
    bytes: Option<Cow<'a, str>>,
}

impl<'a> PostDataEntryBuilder<'a> {
    pub fn bytes(mut self, bytes: impl Into<Cow<'a, str>>) -> Self { self.bytes = Some(bytes.into()); self }
    pub fn build(self) -> PostDataEntry<'a> {
        PostDataEntry {
            bytes: self.bytes,
        }
    }
}

/// HTTP request data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Request<'a> {
    /// Request URL (without fragment).
    url: Cow<'a, str>,
    /// Fragment of the requested URL starting with hash, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    urlFragment: Option<Cow<'a, str>>,
    /// HTTP request method.
    method: Cow<'a, str>,
    /// HTTP request headers.
    headers: Headers,
    /// HTTP POST request data.
    /// Use postDataEntries instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    postData: Option<Cow<'a, str>>,
    /// True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasPostData: Option<bool>,
    /// Request body elements (post data broken into individual entries).
    #[serde(skip_serializing_if = "Option::is_none")]
    postDataEntries: Option<Vec<PostDataEntry<'a>>>,
    /// The mixed content type of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    mixedContentType: Option<crate::security::MixedContentType>,
    /// Priority of the resource request at the time request is sent.
    initialPriority: ResourcePriority,
    /// The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/
    referrerPolicy: Cow<'a, str>,
    /// Whether is loaded via link preload.
    #[serde(skip_serializing_if = "Option::is_none")]
    isLinkPreload: Option<bool>,
    /// Set for requests when the TrustToken API is used. Contains the parameters
    /// passed by the developer (e.g. via "fetch") as understood by the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    trustTokenParams: Option<TrustTokenParams<'a>>,
    /// True if this resource request is considered to be the 'same site' as the
    /// request corresponding to the main frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    isSameSite: Option<bool>,
    /// True when the resource request is ad-related.
    #[serde(skip_serializing_if = "Option::is_none")]
    isAdRelated: Option<bool>,
}

impl<'a> Request<'a> {
    pub fn builder() -> RequestBuilder<'a> { RequestBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn urlFragment(&self) -> Option<&str> { self.urlFragment.as_deref() }
    pub fn method(&self) -> &str { self.method.as_ref() }
    pub fn headers(&self) -> &Headers { &self.headers }
    pub fn postData(&self) -> Option<&str> { self.postData.as_deref() }
    pub fn hasPostData(&self) -> Option<bool> { self.hasPostData }
    pub fn postDataEntries(&self) -> Option<&[PostDataEntry<'a>]> { self.postDataEntries.as_deref() }
    pub fn mixedContentType(&self) -> Option<&crate::security::MixedContentType> { self.mixedContentType.as_ref() }
    pub fn initialPriority(&self) -> &ResourcePriority { &self.initialPriority }
    pub fn referrerPolicy(&self) -> &str { self.referrerPolicy.as_ref() }
    pub fn isLinkPreload(&self) -> Option<bool> { self.isLinkPreload }
    pub fn trustTokenParams(&self) -> Option<&TrustTokenParams<'a>> { self.trustTokenParams.as_ref() }
    pub fn isSameSite(&self) -> Option<bool> { self.isSameSite }
    pub fn isAdRelated(&self) -> Option<bool> { self.isAdRelated }
}

#[derive(Default)]
pub struct RequestBuilder<'a> {
    url: Option<Cow<'a, str>>,
    urlFragment: Option<Cow<'a, str>>,
    method: Option<Cow<'a, str>>,
    headers: Option<Headers>,
    postData: Option<Cow<'a, str>>,
    hasPostData: Option<bool>,
    postDataEntries: Option<Vec<PostDataEntry<'a>>>,
    mixedContentType: Option<crate::security::MixedContentType>,
    initialPriority: Option<ResourcePriority>,
    referrerPolicy: Option<Cow<'a, str>>,
    isLinkPreload: Option<bool>,
    trustTokenParams: Option<TrustTokenParams<'a>>,
    isSameSite: Option<bool>,
    isAdRelated: Option<bool>,
}

impl<'a> RequestBuilder<'a> {
    /// Request URL (without fragment).
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Fragment of the requested URL starting with hash, if present.
    pub fn urlFragment(mut self, urlFragment: impl Into<Cow<'a, str>>) -> Self { self.urlFragment = Some(urlFragment.into()); self }
    /// HTTP request method.
    pub fn method(mut self, method: impl Into<Cow<'a, str>>) -> Self { self.method = Some(method.into()); self }
    /// HTTP request headers.
    pub fn headers(mut self, headers: Headers) -> Self { self.headers = Some(headers); self }
    /// HTTP POST request data.
    /// Use postDataEntries instead.
    pub fn postData(mut self, postData: impl Into<Cow<'a, str>>) -> Self { self.postData = Some(postData.into()); self }
    /// True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.
    pub fn hasPostData(mut self, hasPostData: bool) -> Self { self.hasPostData = Some(hasPostData); self }
    /// Request body elements (post data broken into individual entries).
    pub fn postDataEntries(mut self, postDataEntries: Vec<PostDataEntry<'a>>) -> Self { self.postDataEntries = Some(postDataEntries); self }
    /// The mixed content type of the request.
    pub fn mixedContentType(mut self, mixedContentType: crate::security::MixedContentType) -> Self { self.mixedContentType = Some(mixedContentType); self }
    /// Priority of the resource request at the time request is sent.
    pub fn initialPriority(mut self, initialPriority: ResourcePriority) -> Self { self.initialPriority = Some(initialPriority); self }
    /// The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/
    pub fn referrerPolicy(mut self, referrerPolicy: impl Into<Cow<'a, str>>) -> Self { self.referrerPolicy = Some(referrerPolicy.into()); self }
    /// Whether is loaded via link preload.
    pub fn isLinkPreload(mut self, isLinkPreload: bool) -> Self { self.isLinkPreload = Some(isLinkPreload); self }
    /// Set for requests when the TrustToken API is used. Contains the parameters
    /// passed by the developer (e.g. via "fetch") as understood by the backend.
    pub fn trustTokenParams(mut self, trustTokenParams: TrustTokenParams<'a>) -> Self { self.trustTokenParams = Some(trustTokenParams); self }
    /// True if this resource request is considered to be the 'same site' as the
    /// request corresponding to the main frame.
    pub fn isSameSite(mut self, isSameSite: bool) -> Self { self.isSameSite = Some(isSameSite); self }
    /// True when the resource request is ad-related.
    pub fn isAdRelated(mut self, isAdRelated: bool) -> Self { self.isAdRelated = Some(isAdRelated); self }
    pub fn build(self) -> Request<'a> {
        Request {
            url: self.url.unwrap_or_default(),
            urlFragment: self.urlFragment,
            method: self.method.unwrap_or_default(),
            headers: self.headers.unwrap_or_default(),
            postData: self.postData,
            hasPostData: self.hasPostData,
            postDataEntries: self.postDataEntries,
            mixedContentType: self.mixedContentType,
            initialPriority: self.initialPriority.unwrap_or_default(),
            referrerPolicy: self.referrerPolicy.unwrap_or_default(),
            isLinkPreload: self.isLinkPreload,
            trustTokenParams: self.trustTokenParams,
            isSameSite: self.isSameSite,
            isAdRelated: self.isAdRelated,
        }
    }
}

/// Details of a signed certificate timestamp (SCT).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedCertificateTimestamp<'a> {
    /// Validation status.
    status: Cow<'a, str>,
    /// Origin.
    origin: Cow<'a, str>,
    /// Log name / description.
    logDescription: Cow<'a, str>,
    /// Log ID.
    logId: Cow<'a, str>,
    /// Issuance date. Unlike TimeSinceEpoch, this contains the number of
    /// milliseconds since January 1, 1970, UTC, not the number of seconds.
    timestamp: f64,
    /// Hash algorithm.
    hashAlgorithm: Cow<'a, str>,
    /// Signature algorithm.
    signatureAlgorithm: Cow<'a, str>,
    /// Signature data.
    signatureData: Cow<'a, str>,
}

impl<'a> SignedCertificateTimestamp<'a> {
    pub fn builder() -> SignedCertificateTimestampBuilder<'a> { SignedCertificateTimestampBuilder::default() }
    pub fn status(&self) -> &str { self.status.as_ref() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn logDescription(&self) -> &str { self.logDescription.as_ref() }
    pub fn logId(&self) -> &str { self.logId.as_ref() }
    pub fn timestamp(&self) -> f64 { self.timestamp }
    pub fn hashAlgorithm(&self) -> &str { self.hashAlgorithm.as_ref() }
    pub fn signatureAlgorithm(&self) -> &str { self.signatureAlgorithm.as_ref() }
    pub fn signatureData(&self) -> &str { self.signatureData.as_ref() }
}

#[derive(Default)]
pub struct SignedCertificateTimestampBuilder<'a> {
    status: Option<Cow<'a, str>>,
    origin: Option<Cow<'a, str>>,
    logDescription: Option<Cow<'a, str>>,
    logId: Option<Cow<'a, str>>,
    timestamp: Option<f64>,
    hashAlgorithm: Option<Cow<'a, str>>,
    signatureAlgorithm: Option<Cow<'a, str>>,
    signatureData: Option<Cow<'a, str>>,
}

impl<'a> SignedCertificateTimestampBuilder<'a> {
    /// Validation status.
    pub fn status(mut self, status: impl Into<Cow<'a, str>>) -> Self { self.status = Some(status.into()); self }
    /// Origin.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// Log name / description.
    pub fn logDescription(mut self, logDescription: impl Into<Cow<'a, str>>) -> Self { self.logDescription = Some(logDescription.into()); self }
    /// Log ID.
    pub fn logId(mut self, logId: impl Into<Cow<'a, str>>) -> Self { self.logId = Some(logId.into()); self }
    /// Issuance date. Unlike TimeSinceEpoch, this contains the number of
    /// milliseconds since January 1, 1970, UTC, not the number of seconds.
    pub fn timestamp(mut self, timestamp: f64) -> Self { self.timestamp = Some(timestamp); self }
    /// Hash algorithm.
    pub fn hashAlgorithm(mut self, hashAlgorithm: impl Into<Cow<'a, str>>) -> Self { self.hashAlgorithm = Some(hashAlgorithm.into()); self }
    /// Signature algorithm.
    pub fn signatureAlgorithm(mut self, signatureAlgorithm: impl Into<Cow<'a, str>>) -> Self { self.signatureAlgorithm = Some(signatureAlgorithm.into()); self }
    /// Signature data.
    pub fn signatureData(mut self, signatureData: impl Into<Cow<'a, str>>) -> Self { self.signatureData = Some(signatureData.into()); self }
    pub fn build(self) -> SignedCertificateTimestamp<'a> {
        SignedCertificateTimestamp {
            status: self.status.unwrap_or_default(),
            origin: self.origin.unwrap_or_default(),
            logDescription: self.logDescription.unwrap_or_default(),
            logId: self.logId.unwrap_or_default(),
            timestamp: self.timestamp.unwrap_or_default(),
            hashAlgorithm: self.hashAlgorithm.unwrap_or_default(),
            signatureAlgorithm: self.signatureAlgorithm.unwrap_or_default(),
            signatureData: self.signatureData.unwrap_or_default(),
        }
    }
}

/// Security details about a request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDetails<'a> {
    /// Protocol name (e.g. "TLS 1.2" or "QUIC").
    protocol: Cow<'a, str>,
    /// Key Exchange used by the connection, or the empty string if not applicable.
    keyExchange: Cow<'a, str>,
    /// (EC)DH group used by the connection, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    keyExchangeGroup: Option<Cow<'a, str>>,
    /// Cipher name.
    cipher: Cow<'a, str>,
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.
    #[serde(skip_serializing_if = "Option::is_none")]
    mac: Option<Cow<'a, str>>,
    /// Certificate ID value.
    certificateId: crate::security::CertificateId,
    /// Certificate subject name.
    subjectName: Cow<'a, str>,
    /// Subject Alternative Name (SAN) DNS names and IP addresses.
    sanList: Vec<Cow<'a, str>>,
    /// Name of the issuing CA.
    issuer: Cow<'a, str>,
    /// Certificate valid from date.
    validFrom: TimeSinceEpoch,
    /// Certificate valid to (expiration) date
    validTo: TimeSinceEpoch,
    /// List of signed certificate timestamps (SCTs).
    signedCertificateTimestampList: Vec<SignedCertificateTimestamp<'a>>,
    /// Whether the request complied with Certificate Transparency policy
    certificateTransparencyCompliance: CertificateTransparencyCompliance,
    /// The signature algorithm used by the server in the TLS server signature,
    /// represented as a TLS SignatureScheme code point. Omitted if not
    /// applicable or not known.
    #[serde(skip_serializing_if = "Option::is_none")]
    serverSignatureAlgorithm: Option<i64>,
    /// Whether the connection used Encrypted ClientHello
    encryptedClientHello: bool,
}

impl<'a> SecurityDetails<'a> {
    pub fn builder() -> SecurityDetailsBuilder<'a> { SecurityDetailsBuilder::default() }
    pub fn protocol(&self) -> &str { self.protocol.as_ref() }
    pub fn keyExchange(&self) -> &str { self.keyExchange.as_ref() }
    pub fn keyExchangeGroup(&self) -> Option<&str> { self.keyExchangeGroup.as_deref() }
    pub fn cipher(&self) -> &str { self.cipher.as_ref() }
    pub fn mac(&self) -> Option<&str> { self.mac.as_deref() }
    pub fn certificateId(&self) -> &crate::security::CertificateId { &self.certificateId }
    pub fn subjectName(&self) -> &str { self.subjectName.as_ref() }
    pub fn sanList(&self) -> &[Cow<'a, str>] { &self.sanList }
    pub fn issuer(&self) -> &str { self.issuer.as_ref() }
    pub fn validFrom(&self) -> &TimeSinceEpoch { &self.validFrom }
    pub fn validTo(&self) -> &TimeSinceEpoch { &self.validTo }
    pub fn signedCertificateTimestampList(&self) -> &[SignedCertificateTimestamp<'a>] { &self.signedCertificateTimestampList }
    pub fn certificateTransparencyCompliance(&self) -> &CertificateTransparencyCompliance { &self.certificateTransparencyCompliance }
    pub fn serverSignatureAlgorithm(&self) -> Option<i64> { self.serverSignatureAlgorithm }
    pub fn encryptedClientHello(&self) -> bool { self.encryptedClientHello }
}

#[derive(Default)]
pub struct SecurityDetailsBuilder<'a> {
    protocol: Option<Cow<'a, str>>,
    keyExchange: Option<Cow<'a, str>>,
    keyExchangeGroup: Option<Cow<'a, str>>,
    cipher: Option<Cow<'a, str>>,
    mac: Option<Cow<'a, str>>,
    certificateId: Option<crate::security::CertificateId>,
    subjectName: Option<Cow<'a, str>>,
    sanList: Option<Vec<Cow<'a, str>>>,
    issuer: Option<Cow<'a, str>>,
    validFrom: Option<TimeSinceEpoch>,
    validTo: Option<TimeSinceEpoch>,
    signedCertificateTimestampList: Option<Vec<SignedCertificateTimestamp<'a>>>,
    certificateTransparencyCompliance: Option<CertificateTransparencyCompliance>,
    serverSignatureAlgorithm: Option<i64>,
    encryptedClientHello: Option<bool>,
}

impl<'a> SecurityDetailsBuilder<'a> {
    /// Protocol name (e.g. "TLS 1.2" or "QUIC").
    pub fn protocol(mut self, protocol: impl Into<Cow<'a, str>>) -> Self { self.protocol = Some(protocol.into()); self }
    /// Key Exchange used by the connection, or the empty string if not applicable.
    pub fn keyExchange(mut self, keyExchange: impl Into<Cow<'a, str>>) -> Self { self.keyExchange = Some(keyExchange.into()); self }
    /// (EC)DH group used by the connection, if applicable.
    pub fn keyExchangeGroup(mut self, keyExchangeGroup: impl Into<Cow<'a, str>>) -> Self { self.keyExchangeGroup = Some(keyExchangeGroup.into()); self }
    /// Cipher name.
    pub fn cipher(mut self, cipher: impl Into<Cow<'a, str>>) -> Self { self.cipher = Some(cipher.into()); self }
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.
    pub fn mac(mut self, mac: impl Into<Cow<'a, str>>) -> Self { self.mac = Some(mac.into()); self }
    /// Certificate ID value.
    pub fn certificateId(mut self, certificateId: crate::security::CertificateId) -> Self { self.certificateId = Some(certificateId); self }
    /// Certificate subject name.
    pub fn subjectName(mut self, subjectName: impl Into<Cow<'a, str>>) -> Self { self.subjectName = Some(subjectName.into()); self }
    /// Subject Alternative Name (SAN) DNS names and IP addresses.
    pub fn sanList(mut self, sanList: Vec<Cow<'a, str>>) -> Self { self.sanList = Some(sanList); self }
    /// Name of the issuing CA.
    pub fn issuer(mut self, issuer: impl Into<Cow<'a, str>>) -> Self { self.issuer = Some(issuer.into()); self }
    /// Certificate valid from date.
    pub fn validFrom(mut self, validFrom: TimeSinceEpoch) -> Self { self.validFrom = Some(validFrom); self }
    /// Certificate valid to (expiration) date
    pub fn validTo(mut self, validTo: TimeSinceEpoch) -> Self { self.validTo = Some(validTo); self }
    /// List of signed certificate timestamps (SCTs).
    pub fn signedCertificateTimestampList(mut self, signedCertificateTimestampList: Vec<SignedCertificateTimestamp<'a>>) -> Self { self.signedCertificateTimestampList = Some(signedCertificateTimestampList); self }
    /// Whether the request complied with Certificate Transparency policy
    pub fn certificateTransparencyCompliance(mut self, certificateTransparencyCompliance: CertificateTransparencyCompliance) -> Self { self.certificateTransparencyCompliance = Some(certificateTransparencyCompliance); self }
    /// The signature algorithm used by the server in the TLS server signature,
    /// represented as a TLS SignatureScheme code point. Omitted if not
    /// applicable or not known.
    pub fn serverSignatureAlgorithm(mut self, serverSignatureAlgorithm: i64) -> Self { self.serverSignatureAlgorithm = Some(serverSignatureAlgorithm); self }
    /// Whether the connection used Encrypted ClientHello
    pub fn encryptedClientHello(mut self, encryptedClientHello: bool) -> Self { self.encryptedClientHello = Some(encryptedClientHello); self }
    pub fn build(self) -> SecurityDetails<'a> {
        SecurityDetails {
            protocol: self.protocol.unwrap_or_default(),
            keyExchange: self.keyExchange.unwrap_or_default(),
            keyExchangeGroup: self.keyExchangeGroup,
            cipher: self.cipher.unwrap_or_default(),
            mac: self.mac,
            certificateId: self.certificateId.unwrap_or_default(),
            subjectName: self.subjectName.unwrap_or_default(),
            sanList: self.sanList.unwrap_or_default(),
            issuer: self.issuer.unwrap_or_default(),
            validFrom: self.validFrom.unwrap_or_default(),
            validTo: self.validTo.unwrap_or_default(),
            signedCertificateTimestampList: self.signedCertificateTimestampList.unwrap_or_default(),
            certificateTransparencyCompliance: self.certificateTransparencyCompliance.unwrap_or_default(),
            serverSignatureAlgorithm: self.serverSignatureAlgorithm,
            encryptedClientHello: self.encryptedClientHello.unwrap_or_default(),
        }
    }
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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CorsErrorStatus<'a> {
    corsError: CorsError,
    failedParameter: Cow<'a, str>,
}

impl<'a> CorsErrorStatus<'a> {
    pub fn builder() -> CorsErrorStatusBuilder<'a> { CorsErrorStatusBuilder::default() }
    pub fn corsError(&self) -> &CorsError { &self.corsError }
    pub fn failedParameter(&self) -> &str { self.failedParameter.as_ref() }
}

#[derive(Default)]
pub struct CorsErrorStatusBuilder<'a> {
    corsError: Option<CorsError>,
    failedParameter: Option<Cow<'a, str>>,
}

impl<'a> CorsErrorStatusBuilder<'a> {
    pub fn corsError(mut self, corsError: CorsError) -> Self { self.corsError = Some(corsError); self }
    pub fn failedParameter(mut self, failedParameter: impl Into<Cow<'a, str>>) -> Self { self.failedParameter = Some(failedParameter.into()); self }
    pub fn build(self) -> CorsErrorStatus<'a> {
        CorsErrorStatus {
            corsError: self.corsError.unwrap_or_default(),
            failedParameter: self.failedParameter.unwrap_or_default(),
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokenParams<'a> {
    operation: TrustTokenOperationType,
    /// Only set for "token-redemption" operation and determine whether
    /// to request a fresh SRR or use a still valid cached SRR.
    refreshPolicy: Cow<'a, str>,
    /// Origins of issuers from whom to request tokens or redemption
    /// records.
    #[serde(skip_serializing_if = "Option::is_none")]
    issuers: Option<Vec<Cow<'a, str>>>,
}

impl<'a> TrustTokenParams<'a> {
    pub fn builder() -> TrustTokenParamsBuilder<'a> { TrustTokenParamsBuilder::default() }
    pub fn operation(&self) -> &TrustTokenOperationType { &self.operation }
    pub fn refreshPolicy(&self) -> &str { self.refreshPolicy.as_ref() }
    pub fn issuers(&self) -> Option<&[Cow<'a, str>]> { self.issuers.as_deref() }
}

#[derive(Default)]
pub struct TrustTokenParamsBuilder<'a> {
    operation: Option<TrustTokenOperationType>,
    refreshPolicy: Option<Cow<'a, str>>,
    issuers: Option<Vec<Cow<'a, str>>>,
}

impl<'a> TrustTokenParamsBuilder<'a> {
    pub fn operation(mut self, operation: TrustTokenOperationType) -> Self { self.operation = Some(operation); self }
    /// Only set for "token-redemption" operation and determine whether
    /// to request a fresh SRR or use a still valid cached SRR.
    pub fn refreshPolicy(mut self, refreshPolicy: impl Into<Cow<'a, str>>) -> Self { self.refreshPolicy = Some(refreshPolicy.into()); self }
    /// Origins of issuers from whom to request tokens or redemption
    /// records.
    pub fn issuers(mut self, issuers: Vec<Cow<'a, str>>) -> Self { self.issuers = Some(issuers); self }
    pub fn build(self) -> TrustTokenParams<'a> {
        TrustTokenParams {
            operation: self.operation.unwrap_or_default(),
            refreshPolicy: self.refreshPolicy.unwrap_or_default(),
            issuers: self.issuers,
        }
    }
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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRouterInfo {
    /// ID of the rule matched. If there is a matched rule, this field will
    /// be set, otherwiser no value will be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    ruleIdMatched: Option<u64>,
    /// The router source of the matched rule. If there is a matched rule, this
    /// field will be set, otherwise no value will be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    matchedSourceType: Option<ServiceWorkerRouterSource>,
    /// The actual router source used.
    #[serde(skip_serializing_if = "Option::is_none")]
    actualSourceType: Option<ServiceWorkerRouterSource>,
}

impl ServiceWorkerRouterInfo {
    pub fn builder() -> ServiceWorkerRouterInfoBuilder { ServiceWorkerRouterInfoBuilder::default() }
    pub fn ruleIdMatched(&self) -> Option<u64> { self.ruleIdMatched }
    pub fn matchedSourceType(&self) -> Option<&ServiceWorkerRouterSource> { self.matchedSourceType.as_ref() }
    pub fn actualSourceType(&self) -> Option<&ServiceWorkerRouterSource> { self.actualSourceType.as_ref() }
}

#[derive(Default)]
pub struct ServiceWorkerRouterInfoBuilder {
    ruleIdMatched: Option<u64>,
    matchedSourceType: Option<ServiceWorkerRouterSource>,
    actualSourceType: Option<ServiceWorkerRouterSource>,
}

impl ServiceWorkerRouterInfoBuilder {
    /// ID of the rule matched. If there is a matched rule, this field will
    /// be set, otherwiser no value will be set.
    pub fn ruleIdMatched(mut self, ruleIdMatched: u64) -> Self { self.ruleIdMatched = Some(ruleIdMatched); self }
    /// The router source of the matched rule. If there is a matched rule, this
    /// field will be set, otherwise no value will be set.
    pub fn matchedSourceType(mut self, matchedSourceType: ServiceWorkerRouterSource) -> Self { self.matchedSourceType = Some(matchedSourceType); self }
    /// The actual router source used.
    pub fn actualSourceType(mut self, actualSourceType: ServiceWorkerRouterSource) -> Self { self.actualSourceType = Some(actualSourceType); self }
    pub fn build(self) -> ServiceWorkerRouterInfo {
        ServiceWorkerRouterInfo {
            ruleIdMatched: self.ruleIdMatched,
            matchedSourceType: self.matchedSourceType,
            actualSourceType: self.actualSourceType,
        }
    }
}

/// HTTP response data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Response<'a> {
    /// Response URL. This URL can be different from CachedResource.url in case of redirect.
    url: Cow<'a, str>,
    /// HTTP response status code.
    status: i64,
    /// HTTP response status text.
    statusText: Cow<'a, str>,
    /// HTTP response headers.
    headers: Headers,
    /// HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo.
    #[serde(skip_serializing_if = "Option::is_none")]
    headersText: Option<Cow<'a, str>>,
    /// Resource mimeType as determined by the browser.
    mimeType: Cow<'a, str>,
    /// Resource charset as determined by the browser (if applicable).
    charset: Cow<'a, str>,
    /// Refined HTTP request headers that were actually transmitted over the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    requestHeaders: Option<Headers>,
    /// HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo.
    #[serde(skip_serializing_if = "Option::is_none")]
    requestHeadersText: Option<Cow<'a, str>>,
    /// Specifies whether physical connection was actually reused for this request.
    connectionReused: bool,
    /// Physical connection id that was actually used for this request.
    connectionId: f64,
    /// Remote IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    remoteIPAddress: Option<Cow<'a, str>>,
    /// Remote port.
    #[serde(skip_serializing_if = "Option::is_none")]
    remotePort: Option<i64>,
    /// Specifies that the request was served from the disk cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    fromDiskCache: Option<bool>,
    /// Specifies that the request was served from the ServiceWorker.
    #[serde(skip_serializing_if = "Option::is_none")]
    fromServiceWorker: Option<bool>,
    /// Specifies that the request was served from the prefetch cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    fromPrefetchCache: Option<bool>,
    /// Specifies that the request was served from the prefetch cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    fromEarlyHints: Option<bool>,
    /// Information about how ServiceWorker Static Router API was used. If this
    /// field is set with 'matchedSourceType' field, a matching rule is found.
    /// If this field is set without 'matchedSource', no matching rule is found.
    /// Otherwise, the API is not used.
    #[serde(skip_serializing_if = "Option::is_none")]
    serviceWorkerRouterInfo: Option<ServiceWorkerRouterInfo>,
    /// Total number of bytes received for this request so far.
    encodedDataLength: f64,
    /// Timing information for the given request.
    #[serde(skip_serializing_if = "Option::is_none")]
    timing: Option<ResourceTiming>,
    /// Response source of response from ServiceWorker.
    #[serde(skip_serializing_if = "Option::is_none")]
    serviceWorkerResponseSource: Option<ServiceWorkerResponseSource>,
    /// The time at which the returned response was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    responseTime: Option<TimeSinceEpoch>,
    /// Cache Storage Cache Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    cacheStorageCacheName: Option<Cow<'a, str>>,
    /// Protocol used to fetch this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<Cow<'a, str>>,
    /// The reason why Chrome uses a specific transport protocol for HTTP semantics.
    #[serde(skip_serializing_if = "Option::is_none")]
    alternateProtocolUsage: Option<AlternateProtocolUsage>,
    /// Security state of the request resource.
    securityState: crate::security::SecurityState,
    /// Security details for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    securityDetails: Option<SecurityDetails<'a>>,
}

impl<'a> Response<'a> {
    pub fn builder() -> ResponseBuilder<'a> { ResponseBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn status(&self) -> i64 { self.status }
    pub fn statusText(&self) -> &str { self.statusText.as_ref() }
    pub fn headers(&self) -> &Headers { &self.headers }
    pub fn headersText(&self) -> Option<&str> { self.headersText.as_deref() }
    pub fn mimeType(&self) -> &str { self.mimeType.as_ref() }
    pub fn charset(&self) -> &str { self.charset.as_ref() }
    pub fn requestHeaders(&self) -> Option<&Headers> { self.requestHeaders.as_ref() }
    pub fn requestHeadersText(&self) -> Option<&str> { self.requestHeadersText.as_deref() }
    pub fn connectionReused(&self) -> bool { self.connectionReused }
    pub fn connectionId(&self) -> f64 { self.connectionId }
    pub fn remoteIPAddress(&self) -> Option<&str> { self.remoteIPAddress.as_deref() }
    pub fn remotePort(&self) -> Option<i64> { self.remotePort }
    pub fn fromDiskCache(&self) -> Option<bool> { self.fromDiskCache }
    pub fn fromServiceWorker(&self) -> Option<bool> { self.fromServiceWorker }
    pub fn fromPrefetchCache(&self) -> Option<bool> { self.fromPrefetchCache }
    pub fn fromEarlyHints(&self) -> Option<bool> { self.fromEarlyHints }
    pub fn serviceWorkerRouterInfo(&self) -> Option<&ServiceWorkerRouterInfo> { self.serviceWorkerRouterInfo.as_ref() }
    pub fn encodedDataLength(&self) -> f64 { self.encodedDataLength }
    pub fn timing(&self) -> Option<&ResourceTiming> { self.timing.as_ref() }
    pub fn serviceWorkerResponseSource(&self) -> Option<&ServiceWorkerResponseSource> { self.serviceWorkerResponseSource.as_ref() }
    pub fn responseTime(&self) -> Option<&TimeSinceEpoch> { self.responseTime.as_ref() }
    pub fn cacheStorageCacheName(&self) -> Option<&str> { self.cacheStorageCacheName.as_deref() }
    pub fn protocol(&self) -> Option<&str> { self.protocol.as_deref() }
    pub fn alternateProtocolUsage(&self) -> Option<&AlternateProtocolUsage> { self.alternateProtocolUsage.as_ref() }
    pub fn securityState(&self) -> &crate::security::SecurityState { &self.securityState }
    pub fn securityDetails(&self) -> Option<&SecurityDetails<'a>> { self.securityDetails.as_ref() }
}

#[derive(Default)]
pub struct ResponseBuilder<'a> {
    url: Option<Cow<'a, str>>,
    status: Option<i64>,
    statusText: Option<Cow<'a, str>>,
    headers: Option<Headers>,
    headersText: Option<Cow<'a, str>>,
    mimeType: Option<Cow<'a, str>>,
    charset: Option<Cow<'a, str>>,
    requestHeaders: Option<Headers>,
    requestHeadersText: Option<Cow<'a, str>>,
    connectionReused: Option<bool>,
    connectionId: Option<f64>,
    remoteIPAddress: Option<Cow<'a, str>>,
    remotePort: Option<i64>,
    fromDiskCache: Option<bool>,
    fromServiceWorker: Option<bool>,
    fromPrefetchCache: Option<bool>,
    fromEarlyHints: Option<bool>,
    serviceWorkerRouterInfo: Option<ServiceWorkerRouterInfo>,
    encodedDataLength: Option<f64>,
    timing: Option<ResourceTiming>,
    serviceWorkerResponseSource: Option<ServiceWorkerResponseSource>,
    responseTime: Option<TimeSinceEpoch>,
    cacheStorageCacheName: Option<Cow<'a, str>>,
    protocol: Option<Cow<'a, str>>,
    alternateProtocolUsage: Option<AlternateProtocolUsage>,
    securityState: Option<crate::security::SecurityState>,
    securityDetails: Option<SecurityDetails<'a>>,
}

impl<'a> ResponseBuilder<'a> {
    /// Response URL. This URL can be different from CachedResource.url in case of redirect.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// HTTP response status code.
    pub fn status(mut self, status: i64) -> Self { self.status = Some(status); self }
    /// HTTP response status text.
    pub fn statusText(mut self, statusText: impl Into<Cow<'a, str>>) -> Self { self.statusText = Some(statusText.into()); self }
    /// HTTP response headers.
    pub fn headers(mut self, headers: Headers) -> Self { self.headers = Some(headers); self }
    /// HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo.
    pub fn headersText(mut self, headersText: impl Into<Cow<'a, str>>) -> Self { self.headersText = Some(headersText.into()); self }
    /// Resource mimeType as determined by the browser.
    pub fn mimeType(mut self, mimeType: impl Into<Cow<'a, str>>) -> Self { self.mimeType = Some(mimeType.into()); self }
    /// Resource charset as determined by the browser (if applicable).
    pub fn charset(mut self, charset: impl Into<Cow<'a, str>>) -> Self { self.charset = Some(charset.into()); self }
    /// Refined HTTP request headers that were actually transmitted over the network.
    pub fn requestHeaders(mut self, requestHeaders: Headers) -> Self { self.requestHeaders = Some(requestHeaders); self }
    /// HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo.
    pub fn requestHeadersText(mut self, requestHeadersText: impl Into<Cow<'a, str>>) -> Self { self.requestHeadersText = Some(requestHeadersText.into()); self }
    /// Specifies whether physical connection was actually reused for this request.
    pub fn connectionReused(mut self, connectionReused: bool) -> Self { self.connectionReused = Some(connectionReused); self }
    /// Physical connection id that was actually used for this request.
    pub fn connectionId(mut self, connectionId: f64) -> Self { self.connectionId = Some(connectionId); self }
    /// Remote IP address.
    pub fn remoteIPAddress(mut self, remoteIPAddress: impl Into<Cow<'a, str>>) -> Self { self.remoteIPAddress = Some(remoteIPAddress.into()); self }
    /// Remote port.
    pub fn remotePort(mut self, remotePort: i64) -> Self { self.remotePort = Some(remotePort); self }
    /// Specifies that the request was served from the disk cache.
    pub fn fromDiskCache(mut self, fromDiskCache: bool) -> Self { self.fromDiskCache = Some(fromDiskCache); self }
    /// Specifies that the request was served from the ServiceWorker.
    pub fn fromServiceWorker(mut self, fromServiceWorker: bool) -> Self { self.fromServiceWorker = Some(fromServiceWorker); self }
    /// Specifies that the request was served from the prefetch cache.
    pub fn fromPrefetchCache(mut self, fromPrefetchCache: bool) -> Self { self.fromPrefetchCache = Some(fromPrefetchCache); self }
    /// Specifies that the request was served from the prefetch cache.
    pub fn fromEarlyHints(mut self, fromEarlyHints: bool) -> Self { self.fromEarlyHints = Some(fromEarlyHints); self }
    /// Information about how ServiceWorker Static Router API was used. If this
    /// field is set with 'matchedSourceType' field, a matching rule is found.
    /// If this field is set without 'matchedSource', no matching rule is found.
    /// Otherwise, the API is not used.
    pub fn serviceWorkerRouterInfo(mut self, serviceWorkerRouterInfo: ServiceWorkerRouterInfo) -> Self { self.serviceWorkerRouterInfo = Some(serviceWorkerRouterInfo); self }
    /// Total number of bytes received for this request so far.
    pub fn encodedDataLength(mut self, encodedDataLength: f64) -> Self { self.encodedDataLength = Some(encodedDataLength); self }
    /// Timing information for the given request.
    pub fn timing(mut self, timing: ResourceTiming) -> Self { self.timing = Some(timing); self }
    /// Response source of response from ServiceWorker.
    pub fn serviceWorkerResponseSource(mut self, serviceWorkerResponseSource: ServiceWorkerResponseSource) -> Self { self.serviceWorkerResponseSource = Some(serviceWorkerResponseSource); self }
    /// The time at which the returned response was generated.
    pub fn responseTime(mut self, responseTime: TimeSinceEpoch) -> Self { self.responseTime = Some(responseTime); self }
    /// Cache Storage Cache Name.
    pub fn cacheStorageCacheName(mut self, cacheStorageCacheName: impl Into<Cow<'a, str>>) -> Self { self.cacheStorageCacheName = Some(cacheStorageCacheName.into()); self }
    /// Protocol used to fetch this request.
    pub fn protocol(mut self, protocol: impl Into<Cow<'a, str>>) -> Self { self.protocol = Some(protocol.into()); self }
    /// The reason why Chrome uses a specific transport protocol for HTTP semantics.
    pub fn alternateProtocolUsage(mut self, alternateProtocolUsage: AlternateProtocolUsage) -> Self { self.alternateProtocolUsage = Some(alternateProtocolUsage); self }
    /// Security state of the request resource.
    pub fn securityState(mut self, securityState: crate::security::SecurityState) -> Self { self.securityState = Some(securityState); self }
    /// Security details for the request.
    pub fn securityDetails(mut self, securityDetails: SecurityDetails<'a>) -> Self { self.securityDetails = Some(securityDetails); self }
    pub fn build(self) -> Response<'a> {
        Response {
            url: self.url.unwrap_or_default(),
            status: self.status.unwrap_or_default(),
            statusText: self.statusText.unwrap_or_default(),
            headers: self.headers.unwrap_or_default(),
            headersText: self.headersText,
            mimeType: self.mimeType.unwrap_or_default(),
            charset: self.charset.unwrap_or_default(),
            requestHeaders: self.requestHeaders,
            requestHeadersText: self.requestHeadersText,
            connectionReused: self.connectionReused.unwrap_or_default(),
            connectionId: self.connectionId.unwrap_or_default(),
            remoteIPAddress: self.remoteIPAddress,
            remotePort: self.remotePort,
            fromDiskCache: self.fromDiskCache,
            fromServiceWorker: self.fromServiceWorker,
            fromPrefetchCache: self.fromPrefetchCache,
            fromEarlyHints: self.fromEarlyHints,
            serviceWorkerRouterInfo: self.serviceWorkerRouterInfo,
            encodedDataLength: self.encodedDataLength.unwrap_or_default(),
            timing: self.timing,
            serviceWorkerResponseSource: self.serviceWorkerResponseSource,
            responseTime: self.responseTime,
            cacheStorageCacheName: self.cacheStorageCacheName,
            protocol: self.protocol,
            alternateProtocolUsage: self.alternateProtocolUsage,
            securityState: self.securityState.unwrap_or_default(),
            securityDetails: self.securityDetails,
        }
    }
}

/// WebSocket request data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketRequest {
    /// HTTP request headers.
    headers: Headers,
}

impl WebSocketRequest {
    pub fn builder() -> WebSocketRequestBuilder { WebSocketRequestBuilder::default() }
    pub fn headers(&self) -> &Headers { &self.headers }
}

#[derive(Default)]
pub struct WebSocketRequestBuilder {
    headers: Option<Headers>,
}

impl WebSocketRequestBuilder {
    /// HTTP request headers.
    pub fn headers(mut self, headers: Headers) -> Self { self.headers = Some(headers); self }
    pub fn build(self) -> WebSocketRequest {
        WebSocketRequest {
            headers: self.headers.unwrap_or_default(),
        }
    }
}

/// WebSocket response data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketResponse<'a> {
    /// HTTP response status code.
    status: i64,
    /// HTTP response status text.
    statusText: Cow<'a, str>,
    /// HTTP response headers.
    headers: Headers,
    /// HTTP response headers text.
    #[serde(skip_serializing_if = "Option::is_none")]
    headersText: Option<Cow<'a, str>>,
    /// HTTP request headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    requestHeaders: Option<Headers>,
    /// HTTP request headers text.
    #[serde(skip_serializing_if = "Option::is_none")]
    requestHeadersText: Option<Cow<'a, str>>,
}

impl<'a> WebSocketResponse<'a> {
    pub fn builder() -> WebSocketResponseBuilder<'a> { WebSocketResponseBuilder::default() }
    pub fn status(&self) -> i64 { self.status }
    pub fn statusText(&self) -> &str { self.statusText.as_ref() }
    pub fn headers(&self) -> &Headers { &self.headers }
    pub fn headersText(&self) -> Option<&str> { self.headersText.as_deref() }
    pub fn requestHeaders(&self) -> Option<&Headers> { self.requestHeaders.as_ref() }
    pub fn requestHeadersText(&self) -> Option<&str> { self.requestHeadersText.as_deref() }
}

#[derive(Default)]
pub struct WebSocketResponseBuilder<'a> {
    status: Option<i64>,
    statusText: Option<Cow<'a, str>>,
    headers: Option<Headers>,
    headersText: Option<Cow<'a, str>>,
    requestHeaders: Option<Headers>,
    requestHeadersText: Option<Cow<'a, str>>,
}

impl<'a> WebSocketResponseBuilder<'a> {
    /// HTTP response status code.
    pub fn status(mut self, status: i64) -> Self { self.status = Some(status); self }
    /// HTTP response status text.
    pub fn statusText(mut self, statusText: impl Into<Cow<'a, str>>) -> Self { self.statusText = Some(statusText.into()); self }
    /// HTTP response headers.
    pub fn headers(mut self, headers: Headers) -> Self { self.headers = Some(headers); self }
    /// HTTP response headers text.
    pub fn headersText(mut self, headersText: impl Into<Cow<'a, str>>) -> Self { self.headersText = Some(headersText.into()); self }
    /// HTTP request headers.
    pub fn requestHeaders(mut self, requestHeaders: Headers) -> Self { self.requestHeaders = Some(requestHeaders); self }
    /// HTTP request headers text.
    pub fn requestHeadersText(mut self, requestHeadersText: impl Into<Cow<'a, str>>) -> Self { self.requestHeadersText = Some(requestHeadersText.into()); self }
    pub fn build(self) -> WebSocketResponse<'a> {
        WebSocketResponse {
            status: self.status.unwrap_or_default(),
            statusText: self.statusText.unwrap_or_default(),
            headers: self.headers.unwrap_or_default(),
            headersText: self.headersText,
            requestHeaders: self.requestHeaders,
            requestHeadersText: self.requestHeadersText,
        }
    }
}

/// WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrame<'a> {
    /// WebSocket message opcode.
    opcode: f64,
    /// WebSocket message mask.
    mask: bool,
    /// WebSocket message payload data.
    /// If the opcode is 1, this is a text message and payloadData is a UTF-8 string.
    /// If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data.
    payloadData: Cow<'a, str>,
}

impl<'a> WebSocketFrame<'a> {
    pub fn builder() -> WebSocketFrameBuilder<'a> { WebSocketFrameBuilder::default() }
    pub fn opcode(&self) -> f64 { self.opcode }
    pub fn mask(&self) -> bool { self.mask }
    pub fn payloadData(&self) -> &str { self.payloadData.as_ref() }
}

#[derive(Default)]
pub struct WebSocketFrameBuilder<'a> {
    opcode: Option<f64>,
    mask: Option<bool>,
    payloadData: Option<Cow<'a, str>>,
}

impl<'a> WebSocketFrameBuilder<'a> {
    /// WebSocket message opcode.
    pub fn opcode(mut self, opcode: f64) -> Self { self.opcode = Some(opcode); self }
    /// WebSocket message mask.
    pub fn mask(mut self, mask: bool) -> Self { self.mask = Some(mask); self }
    /// WebSocket message payload data.
    /// If the opcode is 1, this is a text message and payloadData is a UTF-8 string.
    /// If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data.
    pub fn payloadData(mut self, payloadData: impl Into<Cow<'a, str>>) -> Self { self.payloadData = Some(payloadData.into()); self }
    pub fn build(self) -> WebSocketFrame<'a> {
        WebSocketFrame {
            opcode: self.opcode.unwrap_or_default(),
            mask: self.mask.unwrap_or_default(),
            payloadData: self.payloadData.unwrap_or_default(),
        }
    }
}

/// Information about the cached resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CachedResource<'a> {
    /// Resource URL. This is the url of the original network request.
    url: Cow<'a, str>,
    /// Type of this resource.
    #[serde(rename = "type")]
    type_: ResourceType,
    /// Cached response data.
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<Response<'a>>,
    /// Cached response body size.
    bodySize: f64,
}

impl<'a> CachedResource<'a> {
    pub fn builder() -> CachedResourceBuilder<'a> { CachedResourceBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn type_(&self) -> &ResourceType { &self.type_ }
    pub fn response(&self) -> Option<&Response<'a>> { self.response.as_ref() }
    pub fn bodySize(&self) -> f64 { self.bodySize }
}

#[derive(Default)]
pub struct CachedResourceBuilder<'a> {
    url: Option<Cow<'a, str>>,
    type_: Option<ResourceType>,
    response: Option<Response<'a>>,
    bodySize: Option<f64>,
}

impl<'a> CachedResourceBuilder<'a> {
    /// Resource URL. This is the url of the original network request.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Type of this resource.
    pub fn type_(mut self, type_: ResourceType) -> Self { self.type_ = Some(type_); self }
    /// Cached response data.
    pub fn response(mut self, response: Response<'a>) -> Self { self.response = Some(response); self }
    /// Cached response body size.
    pub fn bodySize(mut self, bodySize: f64) -> Self { self.bodySize = Some(bodySize); self }
    pub fn build(self) -> CachedResource<'a> {
        CachedResource {
            url: self.url.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
            response: self.response,
            bodySize: self.bodySize.unwrap_or_default(),
        }
    }
}

/// Information about the request initiator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Initiator<'a> {
    /// Type of this initiator.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Initiator JavaScript stack trace, set for Script only.
    /// Requires the Debugger domain to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<crate::runtime::StackTrace>,
    /// Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// Initiator line number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    lineNumber: Option<f64>,
    /// Initiator column number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    columnNumber: Option<f64>,
    /// Set if another request triggered this request (e.g. preflight).
    #[serde(skip_serializing_if = "Option::is_none")]
    requestId: Option<RequestId<'a>>,
}

impl<'a> Initiator<'a> {
    pub fn builder() -> InitiatorBuilder<'a> { InitiatorBuilder::default() }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn stack(&self) -> Option<&crate::runtime::StackTrace> { self.stack.as_ref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn lineNumber(&self) -> Option<f64> { self.lineNumber }
    pub fn columnNumber(&self) -> Option<f64> { self.columnNumber }
    pub fn requestId(&self) -> Option<&RequestId<'a>> { self.requestId.as_ref() }
}

#[derive(Default)]
pub struct InitiatorBuilder<'a> {
    type_: Option<Cow<'a, str>>,
    stack: Option<crate::runtime::StackTrace>,
    url: Option<Cow<'a, str>>,
    lineNumber: Option<f64>,
    columnNumber: Option<f64>,
    requestId: Option<RequestId<'a>>,
}

impl<'a> InitiatorBuilder<'a> {
    /// Type of this initiator.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    /// Initiator JavaScript stack trace, set for Script only.
    /// Requires the Debugger domain to be enabled.
    pub fn stack(mut self, stack: crate::runtime::StackTrace) -> Self { self.stack = Some(stack); self }
    /// Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Initiator line number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    pub fn lineNumber(mut self, lineNumber: f64) -> Self { self.lineNumber = Some(lineNumber); self }
    /// Initiator column number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    pub fn columnNumber(mut self, columnNumber: f64) -> Self { self.columnNumber = Some(columnNumber); self }
    /// Set if another request triggered this request (e.g. preflight).
    pub fn requestId(mut self, requestId: RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    pub fn build(self) -> Initiator<'a> {
        Initiator {
            type_: self.type_.unwrap_or_default(),
            stack: self.stack,
            url: self.url,
            lineNumber: self.lineNumber,
            columnNumber: self.columnNumber,
            requestId: self.requestId,
        }
    }
}

/// cookiePartitionKey object
/// The representation of the components of the key that are created by the cookiePartitionKey class contained in net/cookies/cookie_partition_key.h.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookiePartitionKey<'a> {
    /// The site of the top-level URL the browser was visiting at the start
    /// of the request to the endpoint that set the cookie.
    topLevelSite: Cow<'a, str>,
    /// Indicates if the cookie has any ancestors that are cross-site to the topLevelSite.
    hasCrossSiteAncestor: bool,
}

impl<'a> CookiePartitionKey<'a> {
    pub fn builder() -> CookiePartitionKeyBuilder<'a> { CookiePartitionKeyBuilder::default() }
    pub fn topLevelSite(&self) -> &str { self.topLevelSite.as_ref() }
    pub fn hasCrossSiteAncestor(&self) -> bool { self.hasCrossSiteAncestor }
}

#[derive(Default)]
pub struct CookiePartitionKeyBuilder<'a> {
    topLevelSite: Option<Cow<'a, str>>,
    hasCrossSiteAncestor: Option<bool>,
}

impl<'a> CookiePartitionKeyBuilder<'a> {
    /// The site of the top-level URL the browser was visiting at the start
    /// of the request to the endpoint that set the cookie.
    pub fn topLevelSite(mut self, topLevelSite: impl Into<Cow<'a, str>>) -> Self { self.topLevelSite = Some(topLevelSite.into()); self }
    /// Indicates if the cookie has any ancestors that are cross-site to the topLevelSite.
    pub fn hasCrossSiteAncestor(mut self, hasCrossSiteAncestor: bool) -> Self { self.hasCrossSiteAncestor = Some(hasCrossSiteAncestor); self }
    pub fn build(self) -> CookiePartitionKey<'a> {
        CookiePartitionKey {
            topLevelSite: self.topLevelSite.unwrap_or_default(),
            hasCrossSiteAncestor: self.hasCrossSiteAncestor.unwrap_or_default(),
        }
    }
}

/// Cookie object

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cookie<'a> {
    /// Cookie name.
    name: Cow<'a, str>,
    /// Cookie value.
    value: Cow<'a, str>,
    /// Cookie domain.
    domain: Cow<'a, str>,
    /// Cookie path.
    path: Cow<'a, str>,
    /// Cookie expiration date as the number of seconds since the UNIX epoch.
    /// The value is set to -1 if the expiry date is not set.
    /// The value can be null for values that cannot be represented in
    /// JSON (±Inf).
    expires: f64,
    /// Cookie size.
    size: u64,
    /// True if cookie is http-only.
    httpOnly: bool,
    /// True if cookie is secure.
    secure: bool,
    /// True in case of session cookie.
    session: bool,
    /// Cookie SameSite type.
    #[serde(skip_serializing_if = "Option::is_none")]
    sameSite: Option<CookieSameSite>,
    /// Cookie Priority
    priority: CookiePriority,
    /// Cookie source scheme type.
    sourceScheme: CookieSourceScheme,
    /// Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    sourcePort: i64,
    /// Cookie partition key.
    #[serde(skip_serializing_if = "Option::is_none")]
    partitionKey: Option<CookiePartitionKey<'a>>,
    /// True if cookie partition key is opaque.
    #[serde(skip_serializing_if = "Option::is_none")]
    partitionKeyOpaque: Option<bool>,
}

impl<'a> Cookie<'a> {
    pub fn builder() -> CookieBuilder<'a> { CookieBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn domain(&self) -> &str { self.domain.as_ref() }
    pub fn path(&self) -> &str { self.path.as_ref() }
    pub fn expires(&self) -> f64 { self.expires }
    pub fn size(&self) -> u64 { self.size }
    pub fn httpOnly(&self) -> bool { self.httpOnly }
    pub fn secure(&self) -> bool { self.secure }
    pub fn session(&self) -> bool { self.session }
    pub fn sameSite(&self) -> Option<&CookieSameSite> { self.sameSite.as_ref() }
    pub fn priority(&self) -> &CookiePriority { &self.priority }
    pub fn sourceScheme(&self) -> &CookieSourceScheme { &self.sourceScheme }
    pub fn sourcePort(&self) -> i64 { self.sourcePort }
    pub fn partitionKey(&self) -> Option<&CookiePartitionKey<'a>> { self.partitionKey.as_ref() }
    pub fn partitionKeyOpaque(&self) -> Option<bool> { self.partitionKeyOpaque }
}

#[derive(Default)]
pub struct CookieBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    domain: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    expires: Option<f64>,
    size: Option<u64>,
    httpOnly: Option<bool>,
    secure: Option<bool>,
    session: Option<bool>,
    sameSite: Option<CookieSameSite>,
    priority: Option<CookiePriority>,
    sourceScheme: Option<CookieSourceScheme>,
    sourcePort: Option<i64>,
    partitionKey: Option<CookiePartitionKey<'a>>,
    partitionKeyOpaque: Option<bool>,
}

impl<'a> CookieBuilder<'a> {
    /// Cookie name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Cookie value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// Cookie domain.
    pub fn domain(mut self, domain: impl Into<Cow<'a, str>>) -> Self { self.domain = Some(domain.into()); self }
    /// Cookie path.
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    /// Cookie expiration date as the number of seconds since the UNIX epoch.
    /// The value is set to -1 if the expiry date is not set.
    /// The value can be null for values that cannot be represented in
    /// JSON (±Inf).
    pub fn expires(mut self, expires: f64) -> Self { self.expires = Some(expires); self }
    /// Cookie size.
    pub fn size(mut self, size: u64) -> Self { self.size = Some(size); self }
    /// True if cookie is http-only.
    pub fn httpOnly(mut self, httpOnly: bool) -> Self { self.httpOnly = Some(httpOnly); self }
    /// True if cookie is secure.
    pub fn secure(mut self, secure: bool) -> Self { self.secure = Some(secure); self }
    /// True in case of session cookie.
    pub fn session(mut self, session: bool) -> Self { self.session = Some(session); self }
    /// Cookie SameSite type.
    pub fn sameSite(mut self, sameSite: CookieSameSite) -> Self { self.sameSite = Some(sameSite); self }
    /// Cookie Priority
    pub fn priority(mut self, priority: CookiePriority) -> Self { self.priority = Some(priority); self }
    /// Cookie source scheme type.
    pub fn sourceScheme(mut self, sourceScheme: CookieSourceScheme) -> Self { self.sourceScheme = Some(sourceScheme); self }
    /// Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    pub fn sourcePort(mut self, sourcePort: i64) -> Self { self.sourcePort = Some(sourcePort); self }
    /// Cookie partition key.
    pub fn partitionKey(mut self, partitionKey: CookiePartitionKey<'a>) -> Self { self.partitionKey = Some(partitionKey); self }
    /// True if cookie partition key is opaque.
    pub fn partitionKeyOpaque(mut self, partitionKeyOpaque: bool) -> Self { self.partitionKeyOpaque = Some(partitionKeyOpaque); self }
    pub fn build(self) -> Cookie<'a> {
        Cookie {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            domain: self.domain.unwrap_or_default(),
            path: self.path.unwrap_or_default(),
            expires: self.expires.unwrap_or_default(),
            size: self.size.unwrap_or_default(),
            httpOnly: self.httpOnly.unwrap_or_default(),
            secure: self.secure.unwrap_or_default(),
            session: self.session.unwrap_or_default(),
            sameSite: self.sameSite,
            priority: self.priority.unwrap_or_default(),
            sourceScheme: self.sourceScheme.unwrap_or_default(),
            sourcePort: self.sourcePort.unwrap_or_default(),
            partitionKey: self.partitionKey,
            partitionKeyOpaque: self.partitionKeyOpaque,
        }
    }
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
    #[serde(rename = "ThirdPartyBlockedInFirstPartySet")]
    ThirdPartyBlockedInFirstPartySet,
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
    #[serde(rename = "ThirdPartyBlockedInFirstPartySet")]
    ThirdPartyBlockedInFirstPartySet,
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
    #[serde(rename = "TPCDMetadata")]
    TPCDMetadata,
    #[serde(rename = "TPCDDeprecationTrial")]
    TPCDDeprecationTrial,
    #[serde(rename = "TopLevelTPCDDeprecationTrial")]
    TopLevelTPCDDeprecationTrial,
    #[serde(rename = "TPCDHeuristics")]
    TPCDHeuristics,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockedSetCookieWithReason<'a> {
    /// The reason(s) this cookie was blocked.
    blockedReasons: Vec<SetCookieBlockedReason>,
    /// The string representing this individual cookie as it would appear in the header.
    /// This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.
    cookieLine: Cow<'a, str>,
    /// The cookie object which represents the cookie which was not stored. It is optional because
    /// sometimes complete cookie information is not available, such as in the case of parsing
    /// errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie: Option<Cookie<'a>>,
}

impl<'a> BlockedSetCookieWithReason<'a> {
    pub fn builder() -> BlockedSetCookieWithReasonBuilder<'a> { BlockedSetCookieWithReasonBuilder::default() }
    pub fn blockedReasons(&self) -> &[SetCookieBlockedReason] { &self.blockedReasons }
    pub fn cookieLine(&self) -> &str { self.cookieLine.as_ref() }
    pub fn cookie(&self) -> Option<&Cookie<'a>> { self.cookie.as_ref() }
}

#[derive(Default)]
pub struct BlockedSetCookieWithReasonBuilder<'a> {
    blockedReasons: Option<Vec<SetCookieBlockedReason>>,
    cookieLine: Option<Cow<'a, str>>,
    cookie: Option<Cookie<'a>>,
}

impl<'a> BlockedSetCookieWithReasonBuilder<'a> {
    /// The reason(s) this cookie was blocked.
    pub fn blockedReasons(mut self, blockedReasons: Vec<SetCookieBlockedReason>) -> Self { self.blockedReasons = Some(blockedReasons); self }
    /// The string representing this individual cookie as it would appear in the header.
    /// This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.
    pub fn cookieLine(mut self, cookieLine: impl Into<Cow<'a, str>>) -> Self { self.cookieLine = Some(cookieLine.into()); self }
    /// The cookie object which represents the cookie which was not stored. It is optional because
    /// sometimes complete cookie information is not available, such as in the case of parsing
    /// errors.
    pub fn cookie(mut self, cookie: Cookie<'a>) -> Self { self.cookie = Some(cookie); self }
    pub fn build(self) -> BlockedSetCookieWithReason<'a> {
        BlockedSetCookieWithReason {
            blockedReasons: self.blockedReasons.unwrap_or_default(),
            cookieLine: self.cookieLine.unwrap_or_default(),
            cookie: self.cookie,
        }
    }
}

/// A cookie should have been blocked by 3PCD but is exempted and stored from a response with the
/// corresponding reason. A cookie could only have at most one exemption reason.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExemptedSetCookieWithReason<'a> {
    /// The reason the cookie was exempted.
    exemptionReason: CookieExemptionReason,
    /// The string representing this individual cookie as it would appear in the header.
    cookieLine: Cow<'a, str>,
    /// The cookie object representing the cookie.
    cookie: Cookie<'a>,
}

impl<'a> ExemptedSetCookieWithReason<'a> {
    pub fn builder() -> ExemptedSetCookieWithReasonBuilder<'a> { ExemptedSetCookieWithReasonBuilder::default() }
    pub fn exemptionReason(&self) -> &CookieExemptionReason { &self.exemptionReason }
    pub fn cookieLine(&self) -> &str { self.cookieLine.as_ref() }
    pub fn cookie(&self) -> &Cookie<'a> { &self.cookie }
}

#[derive(Default)]
pub struct ExemptedSetCookieWithReasonBuilder<'a> {
    exemptionReason: Option<CookieExemptionReason>,
    cookieLine: Option<Cow<'a, str>>,
    cookie: Option<Cookie<'a>>,
}

impl<'a> ExemptedSetCookieWithReasonBuilder<'a> {
    /// The reason the cookie was exempted.
    pub fn exemptionReason(mut self, exemptionReason: CookieExemptionReason) -> Self { self.exemptionReason = Some(exemptionReason); self }
    /// The string representing this individual cookie as it would appear in the header.
    pub fn cookieLine(mut self, cookieLine: impl Into<Cow<'a, str>>) -> Self { self.cookieLine = Some(cookieLine.into()); self }
    /// The cookie object representing the cookie.
    pub fn cookie(mut self, cookie: Cookie<'a>) -> Self { self.cookie = Some(cookie); self }
    pub fn build(self) -> ExemptedSetCookieWithReason<'a> {
        ExemptedSetCookieWithReason {
            exemptionReason: self.exemptionReason.unwrap_or_default(),
            cookieLine: self.cookieLine.unwrap_or_default(),
            cookie: self.cookie.unwrap_or_default(),
        }
    }
}

/// A cookie associated with the request which may or may not be sent with it.
/// Includes the cookies itself and reasons for blocking or exemption.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedCookie<'a> {
    /// The cookie object representing the cookie which was not sent.
    cookie: Cookie<'a>,
    /// The reason(s) the cookie was blocked. If empty means the cookie is included.
    blockedReasons: Vec<CookieBlockedReason>,
    /// The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could
    /// only have at most one exemption reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    exemptionReason: Option<CookieExemptionReason>,
}

impl<'a> AssociatedCookie<'a> {
    pub fn builder() -> AssociatedCookieBuilder<'a> { AssociatedCookieBuilder::default() }
    pub fn cookie(&self) -> &Cookie<'a> { &self.cookie }
    pub fn blockedReasons(&self) -> &[CookieBlockedReason] { &self.blockedReasons }
    pub fn exemptionReason(&self) -> Option<&CookieExemptionReason> { self.exemptionReason.as_ref() }
}

#[derive(Default)]
pub struct AssociatedCookieBuilder<'a> {
    cookie: Option<Cookie<'a>>,
    blockedReasons: Option<Vec<CookieBlockedReason>>,
    exemptionReason: Option<CookieExemptionReason>,
}

impl<'a> AssociatedCookieBuilder<'a> {
    /// The cookie object representing the cookie which was not sent.
    pub fn cookie(mut self, cookie: Cookie<'a>) -> Self { self.cookie = Some(cookie); self }
    /// The reason(s) the cookie was blocked. If empty means the cookie is included.
    pub fn blockedReasons(mut self, blockedReasons: Vec<CookieBlockedReason>) -> Self { self.blockedReasons = Some(blockedReasons); self }
    /// The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could
    /// only have at most one exemption reason.
    pub fn exemptionReason(mut self, exemptionReason: CookieExemptionReason) -> Self { self.exemptionReason = Some(exemptionReason); self }
    pub fn build(self) -> AssociatedCookie<'a> {
        AssociatedCookie {
            cookie: self.cookie.unwrap_or_default(),
            blockedReasons: self.blockedReasons.unwrap_or_default(),
            exemptionReason: self.exemptionReason,
        }
    }
}

/// Cookie parameter object

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookieParam<'a> {
    /// Cookie name.
    name: Cow<'a, str>,
    /// Cookie value.
    value: Cow<'a, str>,
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// Cookie domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<Cow<'a, str>>,
    /// Cookie path.
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<Cow<'a, str>>,
    /// True if cookie is secure.
    #[serde(skip_serializing_if = "Option::is_none")]
    secure: Option<bool>,
    /// True if cookie is http-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    httpOnly: Option<bool>,
    /// Cookie SameSite type.
    #[serde(skip_serializing_if = "Option::is_none")]
    sameSite: Option<CookieSameSite>,
    /// Cookie expiration date, session cookie if not set
    #[serde(skip_serializing_if = "Option::is_none")]
    expires: Option<TimeSinceEpoch>,
    /// Cookie Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<CookiePriority>,
    /// Cookie source scheme type.
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceScheme: Option<CookieSourceScheme>,
    /// Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    sourcePort: Option<i64>,
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    partitionKey: Option<CookiePartitionKey<'a>>,
}

impl<'a> CookieParam<'a> {
    pub fn builder() -> CookieParamBuilder<'a> { CookieParamBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn domain(&self) -> Option<&str> { self.domain.as_deref() }
    pub fn path(&self) -> Option<&str> { self.path.as_deref() }
    pub fn secure(&self) -> Option<bool> { self.secure }
    pub fn httpOnly(&self) -> Option<bool> { self.httpOnly }
    pub fn sameSite(&self) -> Option<&CookieSameSite> { self.sameSite.as_ref() }
    pub fn expires(&self) -> Option<&TimeSinceEpoch> { self.expires.as_ref() }
    pub fn priority(&self) -> Option<&CookiePriority> { self.priority.as_ref() }
    pub fn sourceScheme(&self) -> Option<&CookieSourceScheme> { self.sourceScheme.as_ref() }
    pub fn sourcePort(&self) -> Option<i64> { self.sourcePort }
    pub fn partitionKey(&self) -> Option<&CookiePartitionKey<'a>> { self.partitionKey.as_ref() }
}

#[derive(Default)]
pub struct CookieParamBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    domain: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    secure: Option<bool>,
    httpOnly: Option<bool>,
    sameSite: Option<CookieSameSite>,
    expires: Option<TimeSinceEpoch>,
    priority: Option<CookiePriority>,
    sourceScheme: Option<CookieSourceScheme>,
    sourcePort: Option<i64>,
    partitionKey: Option<CookiePartitionKey<'a>>,
}

impl<'a> CookieParamBuilder<'a> {
    /// Cookie name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Cookie value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Cookie domain.
    pub fn domain(mut self, domain: impl Into<Cow<'a, str>>) -> Self { self.domain = Some(domain.into()); self }
    /// Cookie path.
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    /// True if cookie is secure.
    pub fn secure(mut self, secure: bool) -> Self { self.secure = Some(secure); self }
    /// True if cookie is http-only.
    pub fn httpOnly(mut self, httpOnly: bool) -> Self { self.httpOnly = Some(httpOnly); self }
    /// Cookie SameSite type.
    pub fn sameSite(mut self, sameSite: CookieSameSite) -> Self { self.sameSite = Some(sameSite); self }
    /// Cookie expiration date, session cookie if not set
    pub fn expires(mut self, expires: TimeSinceEpoch) -> Self { self.expires = Some(expires); self }
    /// Cookie Priority.
    pub fn priority(mut self, priority: CookiePriority) -> Self { self.priority = Some(priority); self }
    /// Cookie source scheme type.
    pub fn sourceScheme(mut self, sourceScheme: CookieSourceScheme) -> Self { self.sourceScheme = Some(sourceScheme); self }
    /// Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    pub fn sourcePort(mut self, sourcePort: i64) -> Self { self.sourcePort = Some(sourcePort); self }
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    pub fn partitionKey(mut self, partitionKey: CookiePartitionKey<'a>) -> Self { self.partitionKey = Some(partitionKey); self }
    pub fn build(self) -> CookieParam<'a> {
        CookieParam {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            url: self.url,
            domain: self.domain,
            path: self.path,
            secure: self.secure,
            httpOnly: self.httpOnly,
            sameSite: self.sameSite,
            expires: self.expires,
            priority: self.priority,
            sourceScheme: self.sourceScheme,
            sourcePort: self.sourcePort,
            partitionKey: self.partitionKey,
        }
    }
}

/// Authorization challenge for HTTP status code 401 or 407.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallenge<'a> {
    /// Source of the authentication challenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Cow<'a, str>>,
    /// Origin of the challenger.
    origin: Cow<'a, str>,
    /// The authentication scheme used, such as basic or digest
    scheme: Cow<'a, str>,
    /// The realm of the challenge. May be empty.
    realm: Cow<'a, str>,
}

impl<'a> AuthChallenge<'a> {
    pub fn builder() -> AuthChallengeBuilder<'a> { AuthChallengeBuilder::default() }
    pub fn source(&self) -> Option<&str> { self.source.as_deref() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn scheme(&self) -> &str { self.scheme.as_ref() }
    pub fn realm(&self) -> &str { self.realm.as_ref() }
}

#[derive(Default)]
pub struct AuthChallengeBuilder<'a> {
    source: Option<Cow<'a, str>>,
    origin: Option<Cow<'a, str>>,
    scheme: Option<Cow<'a, str>>,
    realm: Option<Cow<'a, str>>,
}

impl<'a> AuthChallengeBuilder<'a> {
    /// Source of the authentication challenge.
    pub fn source(mut self, source: impl Into<Cow<'a, str>>) -> Self { self.source = Some(source.into()); self }
    /// Origin of the challenger.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// The authentication scheme used, such as basic or digest
    pub fn scheme(mut self, scheme: impl Into<Cow<'a, str>>) -> Self { self.scheme = Some(scheme.into()); self }
    /// The realm of the challenge. May be empty.
    pub fn realm(mut self, realm: impl Into<Cow<'a, str>>) -> Self { self.realm = Some(realm.into()); self }
    pub fn build(self) -> AuthChallenge<'a> {
        AuthChallenge {
            source: self.source,
            origin: self.origin.unwrap_or_default(),
            scheme: self.scheme.unwrap_or_default(),
            realm: self.realm.unwrap_or_default(),
        }
    }
}

/// Response to an AuthChallenge.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallengeResponse<'a> {
    /// The decision on what to do in response to the authorization challenge.  Default means
    /// deferring to the default behavior of the net stack, which will likely either the Cancel
    /// authentication or display a popup dialog box.
    response: Cow<'a, str>,
    /// The username to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<Cow<'a, str>>,
    /// The password to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<Cow<'a, str>>,
}

impl<'a> AuthChallengeResponse<'a> {
    pub fn builder() -> AuthChallengeResponseBuilder<'a> { AuthChallengeResponseBuilder::default() }
    pub fn response(&self) -> &str { self.response.as_ref() }
    pub fn username(&self) -> Option<&str> { self.username.as_deref() }
    pub fn password(&self) -> Option<&str> { self.password.as_deref() }
}

#[derive(Default)]
pub struct AuthChallengeResponseBuilder<'a> {
    response: Option<Cow<'a, str>>,
    username: Option<Cow<'a, str>>,
    password: Option<Cow<'a, str>>,
}

impl<'a> AuthChallengeResponseBuilder<'a> {
    /// The decision on what to do in response to the authorization challenge.  Default means
    /// deferring to the default behavior of the net stack, which will likely either the Cancel
    /// authentication or display a popup dialog box.
    pub fn response(mut self, response: impl Into<Cow<'a, str>>) -> Self { self.response = Some(response.into()); self }
    /// The username to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    pub fn username(mut self, username: impl Into<Cow<'a, str>>) -> Self { self.username = Some(username.into()); self }
    /// The password to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    pub fn password(mut self, password: impl Into<Cow<'a, str>>) -> Self { self.password = Some(password.into()); self }
    pub fn build(self) -> AuthChallengeResponse<'a> {
        AuthChallengeResponse {
            response: self.response.unwrap_or_default(),
            username: self.username,
            password: self.password,
        }
    }
}

/// Stages of the interception to begin intercepting. Request will intercept before the request is
/// sent. Response will intercept after the response is received.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InterceptionStage {
    #[default]
    #[serde(rename = "Request")]
    Request,
    #[serde(rename = "HeadersReceived")]
    HeadersReceived,
}

/// Request pattern for interception.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestPattern<'a> {
    /// Wildcards (''*'' -> zero or more, ''?'' -> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    #[serde(skip_serializing_if = "Option::is_none")]
    urlPattern: Option<Cow<'a, str>>,
    /// If set, only requests for matching resource types will be intercepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    resourceType: Option<ResourceType>,
    /// Stage at which to begin intercepting requests. Default is Request.
    #[serde(skip_serializing_if = "Option::is_none")]
    interceptionStage: Option<InterceptionStage>,
}

impl<'a> RequestPattern<'a> {
    pub fn builder() -> RequestPatternBuilder<'a> { RequestPatternBuilder::default() }
    pub fn urlPattern(&self) -> Option<&str> { self.urlPattern.as_deref() }
    pub fn resourceType(&self) -> Option<&ResourceType> { self.resourceType.as_ref() }
    pub fn interceptionStage(&self) -> Option<&InterceptionStage> { self.interceptionStage.as_ref() }
}

#[derive(Default)]
pub struct RequestPatternBuilder<'a> {
    urlPattern: Option<Cow<'a, str>>,
    resourceType: Option<ResourceType>,
    interceptionStage: Option<InterceptionStage>,
}

impl<'a> RequestPatternBuilder<'a> {
    /// Wildcards (''*'' -> zero or more, ''?'' -> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    pub fn urlPattern(mut self, urlPattern: impl Into<Cow<'a, str>>) -> Self { self.urlPattern = Some(urlPattern.into()); self }
    /// If set, only requests for matching resource types will be intercepted.
    pub fn resourceType(mut self, resourceType: ResourceType) -> Self { self.resourceType = Some(resourceType); self }
    /// Stage at which to begin intercepting requests. Default is Request.
    pub fn interceptionStage(mut self, interceptionStage: InterceptionStage) -> Self { self.interceptionStage = Some(interceptionStage); self }
    pub fn build(self) -> RequestPattern<'a> {
        RequestPattern {
            urlPattern: self.urlPattern,
            resourceType: self.resourceType,
            interceptionStage: self.interceptionStage,
        }
    }
}

/// Information about a signed exchange signature.
/// https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeSignature<'a> {
    /// Signed exchange signature label.
    label: Cow<'a, str>,
    /// The hex string of signed exchange signature.
    signature: Cow<'a, str>,
    /// Signed exchange signature integrity.
    integrity: Cow<'a, str>,
    /// Signed exchange signature cert Url.
    #[serde(skip_serializing_if = "Option::is_none")]
    certUrl: Option<Cow<'a, str>>,
    /// The hex string of signed exchange signature cert sha256.
    #[serde(skip_serializing_if = "Option::is_none")]
    certSha256: Option<Cow<'a, str>>,
    /// Signed exchange signature validity Url.
    validityUrl: Cow<'a, str>,
    /// Signed exchange signature date.
    date: i64,
    /// Signed exchange signature expires.
    expires: i64,
    /// The encoded certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    certificates: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SignedExchangeSignature<'a> {
    pub fn builder() -> SignedExchangeSignatureBuilder<'a> { SignedExchangeSignatureBuilder::default() }
    pub fn label(&self) -> &str { self.label.as_ref() }
    pub fn signature(&self) -> &str { self.signature.as_ref() }
    pub fn integrity(&self) -> &str { self.integrity.as_ref() }
    pub fn certUrl(&self) -> Option<&str> { self.certUrl.as_deref() }
    pub fn certSha256(&self) -> Option<&str> { self.certSha256.as_deref() }
    pub fn validityUrl(&self) -> &str { self.validityUrl.as_ref() }
    pub fn date(&self) -> i64 { self.date }
    pub fn expires(&self) -> i64 { self.expires }
    pub fn certificates(&self) -> Option<&[Cow<'a, str>]> { self.certificates.as_deref() }
}

#[derive(Default)]
pub struct SignedExchangeSignatureBuilder<'a> {
    label: Option<Cow<'a, str>>,
    signature: Option<Cow<'a, str>>,
    integrity: Option<Cow<'a, str>>,
    certUrl: Option<Cow<'a, str>>,
    certSha256: Option<Cow<'a, str>>,
    validityUrl: Option<Cow<'a, str>>,
    date: Option<i64>,
    expires: Option<i64>,
    certificates: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SignedExchangeSignatureBuilder<'a> {
    /// Signed exchange signature label.
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self { self.label = Some(label.into()); self }
    /// The hex string of signed exchange signature.
    pub fn signature(mut self, signature: impl Into<Cow<'a, str>>) -> Self { self.signature = Some(signature.into()); self }
    /// Signed exchange signature integrity.
    pub fn integrity(mut self, integrity: impl Into<Cow<'a, str>>) -> Self { self.integrity = Some(integrity.into()); self }
    /// Signed exchange signature cert Url.
    pub fn certUrl(mut self, certUrl: impl Into<Cow<'a, str>>) -> Self { self.certUrl = Some(certUrl.into()); self }
    /// The hex string of signed exchange signature cert sha256.
    pub fn certSha256(mut self, certSha256: impl Into<Cow<'a, str>>) -> Self { self.certSha256 = Some(certSha256.into()); self }
    /// Signed exchange signature validity Url.
    pub fn validityUrl(mut self, validityUrl: impl Into<Cow<'a, str>>) -> Self { self.validityUrl = Some(validityUrl.into()); self }
    /// Signed exchange signature date.
    pub fn date(mut self, date: i64) -> Self { self.date = Some(date); self }
    /// Signed exchange signature expires.
    pub fn expires(mut self, expires: i64) -> Self { self.expires = Some(expires); self }
    /// The encoded certificates.
    pub fn certificates(mut self, certificates: Vec<Cow<'a, str>>) -> Self { self.certificates = Some(certificates); self }
    pub fn build(self) -> SignedExchangeSignature<'a> {
        SignedExchangeSignature {
            label: self.label.unwrap_or_default(),
            signature: self.signature.unwrap_or_default(),
            integrity: self.integrity.unwrap_or_default(),
            certUrl: self.certUrl,
            certSha256: self.certSha256,
            validityUrl: self.validityUrl.unwrap_or_default(),
            date: self.date.unwrap_or_default(),
            expires: self.expires.unwrap_or_default(),
            certificates: self.certificates,
        }
    }
}

/// Information about a signed exchange header.
/// https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeHeader<'a> {
    /// Signed exchange request URL.
    requestUrl: Cow<'a, str>,
    /// Signed exchange response code.
    responseCode: i64,
    /// Signed exchange response headers.
    responseHeaders: Headers,
    /// Signed exchange response signature.
    signatures: Vec<SignedExchangeSignature<'a>>,
    /// Signed exchange header integrity hash in the form of 'sha256-<base64-hash-value>'.
    headerIntegrity: Cow<'a, str>,
}

impl<'a> SignedExchangeHeader<'a> {
    pub fn builder() -> SignedExchangeHeaderBuilder<'a> { SignedExchangeHeaderBuilder::default() }
    pub fn requestUrl(&self) -> &str { self.requestUrl.as_ref() }
    pub fn responseCode(&self) -> i64 { self.responseCode }
    pub fn responseHeaders(&self) -> &Headers { &self.responseHeaders }
    pub fn signatures(&self) -> &[SignedExchangeSignature<'a>] { &self.signatures }
    pub fn headerIntegrity(&self) -> &str { self.headerIntegrity.as_ref() }
}

#[derive(Default)]
pub struct SignedExchangeHeaderBuilder<'a> {
    requestUrl: Option<Cow<'a, str>>,
    responseCode: Option<i64>,
    responseHeaders: Option<Headers>,
    signatures: Option<Vec<SignedExchangeSignature<'a>>>,
    headerIntegrity: Option<Cow<'a, str>>,
}

impl<'a> SignedExchangeHeaderBuilder<'a> {
    /// Signed exchange request URL.
    pub fn requestUrl(mut self, requestUrl: impl Into<Cow<'a, str>>) -> Self { self.requestUrl = Some(requestUrl.into()); self }
    /// Signed exchange response code.
    pub fn responseCode(mut self, responseCode: i64) -> Self { self.responseCode = Some(responseCode); self }
    /// Signed exchange response headers.
    pub fn responseHeaders(mut self, responseHeaders: Headers) -> Self { self.responseHeaders = Some(responseHeaders); self }
    /// Signed exchange response signature.
    pub fn signatures(mut self, signatures: Vec<SignedExchangeSignature<'a>>) -> Self { self.signatures = Some(signatures); self }
    /// Signed exchange header integrity hash in the form of 'sha256-<base64-hash-value>'.
    pub fn headerIntegrity(mut self, headerIntegrity: impl Into<Cow<'a, str>>) -> Self { self.headerIntegrity = Some(headerIntegrity.into()); self }
    pub fn build(self) -> SignedExchangeHeader<'a> {
        SignedExchangeHeader {
            requestUrl: self.requestUrl.unwrap_or_default(),
            responseCode: self.responseCode.unwrap_or_default(),
            responseHeaders: self.responseHeaders.unwrap_or_default(),
            signatures: self.signatures.unwrap_or_default(),
            headerIntegrity: self.headerIntegrity.unwrap_or_default(),
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeError<'a> {
    /// Error message.
    message: Cow<'a, str>,
    /// The index of the signature which caused the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    signatureIndex: Option<u64>,
    /// The field which caused the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    errorField: Option<SignedExchangeErrorField>,
}

impl<'a> SignedExchangeError<'a> {
    pub fn builder() -> SignedExchangeErrorBuilder<'a> { SignedExchangeErrorBuilder::default() }
    pub fn message(&self) -> &str { self.message.as_ref() }
    pub fn signatureIndex(&self) -> Option<u64> { self.signatureIndex }
    pub fn errorField(&self) -> Option<&SignedExchangeErrorField> { self.errorField.as_ref() }
}

#[derive(Default)]
pub struct SignedExchangeErrorBuilder<'a> {
    message: Option<Cow<'a, str>>,
    signatureIndex: Option<u64>,
    errorField: Option<SignedExchangeErrorField>,
}

impl<'a> SignedExchangeErrorBuilder<'a> {
    /// Error message.
    pub fn message(mut self, message: impl Into<Cow<'a, str>>) -> Self { self.message = Some(message.into()); self }
    /// The index of the signature which caused the error.
    pub fn signatureIndex(mut self, signatureIndex: u64) -> Self { self.signatureIndex = Some(signatureIndex); self }
    /// The field which caused the error.
    pub fn errorField(mut self, errorField: SignedExchangeErrorField) -> Self { self.errorField = Some(errorField); self }
    pub fn build(self) -> SignedExchangeError<'a> {
        SignedExchangeError {
            message: self.message.unwrap_or_default(),
            signatureIndex: self.signatureIndex,
            errorField: self.errorField,
        }
    }
}

/// Information about a signed exchange response.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeInfo<'a> {
    /// The outer response of signed HTTP exchange which was received from network.
    outerResponse: Response<'a>,
    /// Whether network response for the signed exchange was accompanied by
    /// extra headers.
    hasExtraInfo: bool,
    /// Information about the signed exchange header.
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<SignedExchangeHeader<'a>>,
    /// Security details for the signed exchange header.
    #[serde(skip_serializing_if = "Option::is_none")]
    securityDetails: Option<SecurityDetails<'a>>,
    /// Errors occurred while handling the signed exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<SignedExchangeError<'a>>>,
}

impl<'a> SignedExchangeInfo<'a> {
    pub fn builder() -> SignedExchangeInfoBuilder<'a> { SignedExchangeInfoBuilder::default() }
    pub fn outerResponse(&self) -> &Response<'a> { &self.outerResponse }
    pub fn hasExtraInfo(&self) -> bool { self.hasExtraInfo }
    pub fn header(&self) -> Option<&SignedExchangeHeader<'a>> { self.header.as_ref() }
    pub fn securityDetails(&self) -> Option<&SecurityDetails<'a>> { self.securityDetails.as_ref() }
    pub fn errors(&self) -> Option<&[SignedExchangeError<'a>]> { self.errors.as_deref() }
}

#[derive(Default)]
pub struct SignedExchangeInfoBuilder<'a> {
    outerResponse: Option<Response<'a>>,
    hasExtraInfo: Option<bool>,
    header: Option<SignedExchangeHeader<'a>>,
    securityDetails: Option<SecurityDetails<'a>>,
    errors: Option<Vec<SignedExchangeError<'a>>>,
}

impl<'a> SignedExchangeInfoBuilder<'a> {
    /// The outer response of signed HTTP exchange which was received from network.
    pub fn outerResponse(mut self, outerResponse: Response<'a>) -> Self { self.outerResponse = Some(outerResponse); self }
    /// Whether network response for the signed exchange was accompanied by
    /// extra headers.
    pub fn hasExtraInfo(mut self, hasExtraInfo: bool) -> Self { self.hasExtraInfo = Some(hasExtraInfo); self }
    /// Information about the signed exchange header.
    pub fn header(mut self, header: SignedExchangeHeader<'a>) -> Self { self.header = Some(header); self }
    /// Security details for the signed exchange header.
    pub fn securityDetails(mut self, securityDetails: SecurityDetails<'a>) -> Self { self.securityDetails = Some(securityDetails); self }
    /// Errors occurred while handling the signed exchange.
    pub fn errors(mut self, errors: Vec<SignedExchangeError<'a>>) -> Self { self.errors = Some(errors); self }
    pub fn build(self) -> SignedExchangeInfo<'a> {
        SignedExchangeInfo {
            outerResponse: self.outerResponse.unwrap_or_default(),
            hasExtraInfo: self.hasExtraInfo.unwrap_or_default(),
            header: self.header,
            securityDetails: self.securityDetails,
            errors: self.errors,
        }
    }
}

/// List of content encodings supported by the backend.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContentEncoding {
    #[default]
    #[serde(rename = "deflate")]
    Deflate,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "zstd")]
    Zstd,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConditions<'a> {
    /// Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string
    /// syntax (https://urlpattern.spec.whatwg.org/) and must be absolute. If the pattern is empty, all requests are
    /// matched (including p2p connections).
    urlPattern: Cow<'a, str>,
    /// Minimum latency from request sent to response headers received (ms).
    latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    downloadThroughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    uploadThroughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    connectionType: Option<ConnectionType>,
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    #[serde(skip_serializing_if = "Option::is_none")]
    packetLoss: Option<f64>,
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    #[serde(skip_serializing_if = "Option::is_none")]
    packetQueueLength: Option<u64>,
    /// WebRTC packetReordering feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    packetReordering: Option<bool>,
    /// True to emulate internet disconnection.
    #[serde(skip_serializing_if = "Option::is_none")]
    offline: Option<bool>,
}

impl<'a> NetworkConditions<'a> {
    pub fn builder() -> NetworkConditionsBuilder<'a> { NetworkConditionsBuilder::default() }
    pub fn urlPattern(&self) -> &str { self.urlPattern.as_ref() }
    pub fn latency(&self) -> f64 { self.latency }
    pub fn downloadThroughput(&self) -> f64 { self.downloadThroughput }
    pub fn uploadThroughput(&self) -> f64 { self.uploadThroughput }
    pub fn connectionType(&self) -> Option<&ConnectionType> { self.connectionType.as_ref() }
    pub fn packetLoss(&self) -> Option<f64> { self.packetLoss }
    pub fn packetQueueLength(&self) -> Option<u64> { self.packetQueueLength }
    pub fn packetReordering(&self) -> Option<bool> { self.packetReordering }
    pub fn offline(&self) -> Option<bool> { self.offline }
}

#[derive(Default)]
pub struct NetworkConditionsBuilder<'a> {
    urlPattern: Option<Cow<'a, str>>,
    latency: Option<f64>,
    downloadThroughput: Option<f64>,
    uploadThroughput: Option<f64>,
    connectionType: Option<ConnectionType>,
    packetLoss: Option<f64>,
    packetQueueLength: Option<u64>,
    packetReordering: Option<bool>,
    offline: Option<bool>,
}

impl<'a> NetworkConditionsBuilder<'a> {
    /// Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string
    /// syntax (https://urlpattern.spec.whatwg.org/) and must be absolute. If the pattern is empty, all requests are
    /// matched (including p2p connections).
    pub fn urlPattern(mut self, urlPattern: impl Into<Cow<'a, str>>) -> Self { self.urlPattern = Some(urlPattern.into()); self }
    /// Minimum latency from request sent to response headers received (ms).
    pub fn latency(mut self, latency: f64) -> Self { self.latency = Some(latency); self }
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    pub fn downloadThroughput(mut self, downloadThroughput: f64) -> Self { self.downloadThroughput = Some(downloadThroughput); self }
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub fn uploadThroughput(mut self, uploadThroughput: f64) -> Self { self.uploadThroughput = Some(uploadThroughput); self }
    /// Connection type if known.
    pub fn connectionType(mut self, connectionType: ConnectionType) -> Self { self.connectionType = Some(connectionType); self }
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    pub fn packetLoss(mut self, packetLoss: f64) -> Self { self.packetLoss = Some(packetLoss); self }
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    pub fn packetQueueLength(mut self, packetQueueLength: u64) -> Self { self.packetQueueLength = Some(packetQueueLength); self }
    /// WebRTC packetReordering feature.
    pub fn packetReordering(mut self, packetReordering: bool) -> Self { self.packetReordering = Some(packetReordering); self }
    /// True to emulate internet disconnection.
    pub fn offline(mut self, offline: bool) -> Self { self.offline = Some(offline); self }
    pub fn build(self) -> NetworkConditions<'a> {
        NetworkConditions {
            urlPattern: self.urlPattern.unwrap_or_default(),
            latency: self.latency.unwrap_or_default(),
            downloadThroughput: self.downloadThroughput.unwrap_or_default(),
            uploadThroughput: self.uploadThroughput.unwrap_or_default(),
            connectionType: self.connectionType,
            packetLoss: self.packetLoss,
            packetQueueLength: self.packetQueueLength,
            packetReordering: self.packetReordering,
            offline: self.offline,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockPattern<'a> {
    /// URL pattern to match. Patterns use the URLPattern constructor string syntax
    /// (https://urlpattern.spec.whatwg.org/) and must be absolute. Example: '*://*:*/*.css'.
    urlPattern: Cow<'a, str>,
    /// Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later
    /// 'BlockPattern'.
    block: bool,
}

impl<'a> BlockPattern<'a> {
    pub fn builder() -> BlockPatternBuilder<'a> { BlockPatternBuilder::default() }
    pub fn urlPattern(&self) -> &str { self.urlPattern.as_ref() }
    pub fn block(&self) -> bool { self.block }
}

#[derive(Default)]
pub struct BlockPatternBuilder<'a> {
    urlPattern: Option<Cow<'a, str>>,
    block: Option<bool>,
}

impl<'a> BlockPatternBuilder<'a> {
    /// URL pattern to match. Patterns use the URLPattern constructor string syntax
    /// (https://urlpattern.spec.whatwg.org/) and must be absolute. Example: '*://*:*/*.css'.
    pub fn urlPattern(mut self, urlPattern: impl Into<Cow<'a, str>>) -> Self { self.urlPattern = Some(urlPattern.into()); self }
    /// Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later
    /// 'BlockPattern'.
    pub fn block(mut self, block: bool) -> Self { self.block = Some(block); self }
    pub fn build(self) -> BlockPattern<'a> {
        BlockPattern {
            urlPattern: self.urlPattern.unwrap_or_default(),
            block: self.block.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DirectSocketDnsQueryType {
    #[default]
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectTCPSocketOptions {
    /// TCP_NODELAY option
    noDelay: bool,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    keepAliveDelay: Option<f64>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    sendBufferSize: Option<f64>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    receiveBufferSize: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dnsQueryType: Option<DirectSocketDnsQueryType>,
}

impl DirectTCPSocketOptions {
    pub fn builder() -> DirectTCPSocketOptionsBuilder { DirectTCPSocketOptionsBuilder::default() }
    pub fn noDelay(&self) -> bool { self.noDelay }
    pub fn keepAliveDelay(&self) -> Option<f64> { self.keepAliveDelay }
    pub fn sendBufferSize(&self) -> Option<f64> { self.sendBufferSize }
    pub fn receiveBufferSize(&self) -> Option<f64> { self.receiveBufferSize }
    pub fn dnsQueryType(&self) -> Option<&DirectSocketDnsQueryType> { self.dnsQueryType.as_ref() }
}

#[derive(Default)]
pub struct DirectTCPSocketOptionsBuilder {
    noDelay: Option<bool>,
    keepAliveDelay: Option<f64>,
    sendBufferSize: Option<f64>,
    receiveBufferSize: Option<f64>,
    dnsQueryType: Option<DirectSocketDnsQueryType>,
}

impl DirectTCPSocketOptionsBuilder {
    /// TCP_NODELAY option
    pub fn noDelay(mut self, noDelay: bool) -> Self { self.noDelay = Some(noDelay); self }
    /// Expected to be unsigned integer.
    pub fn keepAliveDelay(mut self, keepAliveDelay: f64) -> Self { self.keepAliveDelay = Some(keepAliveDelay); self }
    /// Expected to be unsigned integer.
    pub fn sendBufferSize(mut self, sendBufferSize: f64) -> Self { self.sendBufferSize = Some(sendBufferSize); self }
    /// Expected to be unsigned integer.
    pub fn receiveBufferSize(mut self, receiveBufferSize: f64) -> Self { self.receiveBufferSize = Some(receiveBufferSize); self }
    pub fn dnsQueryType(mut self, dnsQueryType: DirectSocketDnsQueryType) -> Self { self.dnsQueryType = Some(dnsQueryType); self }
    pub fn build(self) -> DirectTCPSocketOptions {
        DirectTCPSocketOptions {
            noDelay: self.noDelay.unwrap_or_default(),
            keepAliveDelay: self.keepAliveDelay,
            sendBufferSize: self.sendBufferSize,
            receiveBufferSize: self.receiveBufferSize,
            dnsQueryType: self.dnsQueryType,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectUDPSocketOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    remoteAddr: Option<Cow<'a, str>>,
    /// Unsigned int 16.
    #[serde(skip_serializing_if = "Option::is_none")]
    remotePort: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    localAddr: Option<Cow<'a, str>>,
    /// Unsigned int 16.
    #[serde(skip_serializing_if = "Option::is_none")]
    localPort: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dnsQueryType: Option<DirectSocketDnsQueryType>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    sendBufferSize: Option<f64>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    receiveBufferSize: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multicastLoopback: Option<bool>,
    /// Unsigned int 8.
    #[serde(skip_serializing_if = "Option::is_none")]
    multicastTimeToLive: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multicastAllowAddressSharing: Option<bool>,
}

impl<'a> DirectUDPSocketOptions<'a> {
    pub fn builder() -> DirectUDPSocketOptionsBuilder<'a> { DirectUDPSocketOptionsBuilder::default() }
    pub fn remoteAddr(&self) -> Option<&str> { self.remoteAddr.as_deref() }
    pub fn remotePort(&self) -> Option<i64> { self.remotePort }
    pub fn localAddr(&self) -> Option<&str> { self.localAddr.as_deref() }
    pub fn localPort(&self) -> Option<i64> { self.localPort }
    pub fn dnsQueryType(&self) -> Option<&DirectSocketDnsQueryType> { self.dnsQueryType.as_ref() }
    pub fn sendBufferSize(&self) -> Option<f64> { self.sendBufferSize }
    pub fn receiveBufferSize(&self) -> Option<f64> { self.receiveBufferSize }
    pub fn multicastLoopback(&self) -> Option<bool> { self.multicastLoopback }
    pub fn multicastTimeToLive(&self) -> Option<i64> { self.multicastTimeToLive }
    pub fn multicastAllowAddressSharing(&self) -> Option<bool> { self.multicastAllowAddressSharing }
}

#[derive(Default)]
pub struct DirectUDPSocketOptionsBuilder<'a> {
    remoteAddr: Option<Cow<'a, str>>,
    remotePort: Option<i64>,
    localAddr: Option<Cow<'a, str>>,
    localPort: Option<i64>,
    dnsQueryType: Option<DirectSocketDnsQueryType>,
    sendBufferSize: Option<f64>,
    receiveBufferSize: Option<f64>,
    multicastLoopback: Option<bool>,
    multicastTimeToLive: Option<i64>,
    multicastAllowAddressSharing: Option<bool>,
}

impl<'a> DirectUDPSocketOptionsBuilder<'a> {
    pub fn remoteAddr(mut self, remoteAddr: impl Into<Cow<'a, str>>) -> Self { self.remoteAddr = Some(remoteAddr.into()); self }
    /// Unsigned int 16.
    pub fn remotePort(mut self, remotePort: i64) -> Self { self.remotePort = Some(remotePort); self }
    pub fn localAddr(mut self, localAddr: impl Into<Cow<'a, str>>) -> Self { self.localAddr = Some(localAddr.into()); self }
    /// Unsigned int 16.
    pub fn localPort(mut self, localPort: i64) -> Self { self.localPort = Some(localPort); self }
    pub fn dnsQueryType(mut self, dnsQueryType: DirectSocketDnsQueryType) -> Self { self.dnsQueryType = Some(dnsQueryType); self }
    /// Expected to be unsigned integer.
    pub fn sendBufferSize(mut self, sendBufferSize: f64) -> Self { self.sendBufferSize = Some(sendBufferSize); self }
    /// Expected to be unsigned integer.
    pub fn receiveBufferSize(mut self, receiveBufferSize: f64) -> Self { self.receiveBufferSize = Some(receiveBufferSize); self }
    pub fn multicastLoopback(mut self, multicastLoopback: bool) -> Self { self.multicastLoopback = Some(multicastLoopback); self }
    /// Unsigned int 8.
    pub fn multicastTimeToLive(mut self, multicastTimeToLive: i64) -> Self { self.multicastTimeToLive = Some(multicastTimeToLive); self }
    pub fn multicastAllowAddressSharing(mut self, multicastAllowAddressSharing: bool) -> Self { self.multicastAllowAddressSharing = Some(multicastAllowAddressSharing); self }
    pub fn build(self) -> DirectUDPSocketOptions<'a> {
        DirectUDPSocketOptions {
            remoteAddr: self.remoteAddr,
            remotePort: self.remotePort,
            localAddr: self.localAddr,
            localPort: self.localPort,
            dnsQueryType: self.dnsQueryType,
            sendBufferSize: self.sendBufferSize,
            receiveBufferSize: self.receiveBufferSize,
            multicastLoopback: self.multicastLoopback,
            multicastTimeToLive: self.multicastTimeToLive,
            multicastAllowAddressSharing: self.multicastAllowAddressSharing,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectUDPMessage<'a> {
    data: Cow<'a, str>,
    /// Null for connected mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    remoteAddr: Option<Cow<'a, str>>,
    /// Null for connected mode.
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    remotePort: Option<i64>,
}

impl<'a> DirectUDPMessage<'a> {
    pub fn builder() -> DirectUDPMessageBuilder<'a> { DirectUDPMessageBuilder::default() }
    pub fn data(&self) -> &str { self.data.as_ref() }
    pub fn remoteAddr(&self) -> Option<&str> { self.remoteAddr.as_deref() }
    pub fn remotePort(&self) -> Option<i64> { self.remotePort }
}

#[derive(Default)]
pub struct DirectUDPMessageBuilder<'a> {
    data: Option<Cow<'a, str>>,
    remoteAddr: Option<Cow<'a, str>>,
    remotePort: Option<i64>,
}

impl<'a> DirectUDPMessageBuilder<'a> {
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    /// Null for connected mode.
    pub fn remoteAddr(mut self, remoteAddr: impl Into<Cow<'a, str>>) -> Self { self.remoteAddr = Some(remoteAddr.into()); self }
    /// Null for connected mode.
    /// Expected to be unsigned integer.
    pub fn remotePort(mut self, remotePort: i64) -> Self { self.remotePort = Some(remotePort); self }
    pub fn build(self) -> DirectUDPMessage<'a> {
        DirectUDPMessage {
            data: self.data.unwrap_or_default(),
            remoteAddr: self.remoteAddr,
            remotePort: self.remotePort,
        }
    }
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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectTiming {
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for
    /// the same request (but not for redirected requests).
    requestTime: f64,
}

impl ConnectTiming {
    pub fn builder() -> ConnectTimingBuilder { ConnectTimingBuilder::default() }
    pub fn requestTime(&self) -> f64 { self.requestTime }
}

#[derive(Default)]
pub struct ConnectTimingBuilder {
    requestTime: Option<f64>,
}

impl ConnectTimingBuilder {
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for
    /// the same request (but not for redirected requests).
    pub fn requestTime(mut self, requestTime: f64) -> Self { self.requestTime = Some(requestTime); self }
    pub fn build(self) -> ConnectTiming {
        ConnectTiming {
            requestTime: self.requestTime.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientSecurityState {
    initiatorIsSecureContext: bool,
    initiatorIPAddressSpace: IPAddressSpace,
    localNetworkAccessRequestPolicy: LocalNetworkAccessRequestPolicy,
}

impl ClientSecurityState {
    pub fn builder() -> ClientSecurityStateBuilder { ClientSecurityStateBuilder::default() }
    pub fn initiatorIsSecureContext(&self) -> bool { self.initiatorIsSecureContext }
    pub fn initiatorIPAddressSpace(&self) -> &IPAddressSpace { &self.initiatorIPAddressSpace }
    pub fn localNetworkAccessRequestPolicy(&self) -> &LocalNetworkAccessRequestPolicy { &self.localNetworkAccessRequestPolicy }
}

#[derive(Default)]
pub struct ClientSecurityStateBuilder {
    initiatorIsSecureContext: Option<bool>,
    initiatorIPAddressSpace: Option<IPAddressSpace>,
    localNetworkAccessRequestPolicy: Option<LocalNetworkAccessRequestPolicy>,
}

impl ClientSecurityStateBuilder {
    pub fn initiatorIsSecureContext(mut self, initiatorIsSecureContext: bool) -> Self { self.initiatorIsSecureContext = Some(initiatorIsSecureContext); self }
    pub fn initiatorIPAddressSpace(mut self, initiatorIPAddressSpace: IPAddressSpace) -> Self { self.initiatorIPAddressSpace = Some(initiatorIPAddressSpace); self }
    pub fn localNetworkAccessRequestPolicy(mut self, localNetworkAccessRequestPolicy: LocalNetworkAccessRequestPolicy) -> Self { self.localNetworkAccessRequestPolicy = Some(localNetworkAccessRequestPolicy); self }
    pub fn build(self) -> ClientSecurityState {
        ClientSecurityState {
            initiatorIsSecureContext: self.initiatorIsSecureContext.unwrap_or_default(),
            initiatorIPAddressSpace: self.initiatorIPAddressSpace.unwrap_or_default(),
            localNetworkAccessRequestPolicy: self.localNetworkAccessRequestPolicy.unwrap_or_default(),
        }
    }
}

/// Identifies the script on the stack that caused a resource or element to be
/// labeled as an ad. For resources, this indicates the context that triggered
/// the fetch. For elements, this indicates the context that caused the element
/// to be appended to the DOM.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdScriptIdentifier<'a> {
    /// The script's V8 identifier.
    scriptId: crate::runtime::ScriptId<'a>,
    /// V8's debugging ID for the v8::Context.
    debuggerId: crate::runtime::UniqueDebuggerId<'a>,
    /// The script's url (or generated name based on id if inline script).
    name: Cow<'a, str>,
}

impl<'a> AdScriptIdentifier<'a> {
    pub fn builder() -> AdScriptIdentifierBuilder<'a> { AdScriptIdentifierBuilder::default() }
    pub fn scriptId(&self) -> &crate::runtime::ScriptId<'a> { &self.scriptId }
    pub fn debuggerId(&self) -> &crate::runtime::UniqueDebuggerId<'a> { &self.debuggerId }
    pub fn name(&self) -> &str { self.name.as_ref() }
}

#[derive(Default)]
pub struct AdScriptIdentifierBuilder<'a> {
    scriptId: Option<crate::runtime::ScriptId<'a>>,
    debuggerId: Option<crate::runtime::UniqueDebuggerId<'a>>,
    name: Option<Cow<'a, str>>,
}

impl<'a> AdScriptIdentifierBuilder<'a> {
    /// The script's V8 identifier.
    pub fn scriptId(mut self, scriptId: crate::runtime::ScriptId<'a>) -> Self { self.scriptId = Some(scriptId); self }
    /// V8's debugging ID for the v8::Context.
    pub fn debuggerId(mut self, debuggerId: crate::runtime::UniqueDebuggerId<'a>) -> Self { self.debuggerId = Some(debuggerId); self }
    /// The script's url (or generated name based on id if inline script).
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> AdScriptIdentifier<'a> {
        AdScriptIdentifier {
            scriptId: self.scriptId.unwrap_or_default(),
            debuggerId: self.debuggerId.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
        }
    }
}

/// Encapsulates the script ancestry and the root script filter list rule that
/// caused the resource or element to be labeled as an ad.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdAncestry<'a> {
    /// A chain of 'AdScriptIdentifier's representing the ancestry of an ad
    /// script that led to the creation of a resource or element. The chain is
    /// ordered from the script itself (lowest level) up to its root ancestor
    /// that was flagged by a filter list.
    ancestryChain: Vec<AdScriptIdentifier<'a>>,
    /// The filter list rule that caused the root (last) script in
    /// 'ancestryChain' to be tagged as an ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    rootScriptFilterlistRule: Option<Cow<'a, str>>,
}

impl<'a> AdAncestry<'a> {
    pub fn builder() -> AdAncestryBuilder<'a> { AdAncestryBuilder::default() }
    pub fn ancestryChain(&self) -> &[AdScriptIdentifier<'a>] { &self.ancestryChain }
    pub fn rootScriptFilterlistRule(&self) -> Option<&str> { self.rootScriptFilterlistRule.as_deref() }
}

#[derive(Default)]
pub struct AdAncestryBuilder<'a> {
    ancestryChain: Option<Vec<AdScriptIdentifier<'a>>>,
    rootScriptFilterlistRule: Option<Cow<'a, str>>,
}

impl<'a> AdAncestryBuilder<'a> {
    /// A chain of 'AdScriptIdentifier's representing the ancestry of an ad
    /// script that led to the creation of a resource or element. The chain is
    /// ordered from the script itself (lowest level) up to its root ancestor
    /// that was flagged by a filter list.
    pub fn ancestryChain(mut self, ancestryChain: Vec<AdScriptIdentifier<'a>>) -> Self { self.ancestryChain = Some(ancestryChain); self }
    /// The filter list rule that caused the root (last) script in
    /// 'ancestryChain' to be tagged as an ad.
    pub fn rootScriptFilterlistRule(mut self, rootScriptFilterlistRule: impl Into<Cow<'a, str>>) -> Self { self.rootScriptFilterlistRule = Some(rootScriptFilterlistRule.into()); self }
    pub fn build(self) -> AdAncestry<'a> {
        AdAncestry {
            ancestryChain: self.ancestryChain.unwrap_or_default(),
            rootScriptFilterlistRule: self.rootScriptFilterlistRule,
        }
    }
}

/// Represents the provenance of an ad resource or element. Only one of
/// 'filterlistRule' or 'adScriptAncestry' can be set. If 'filterlistRule'
/// is provided, the resource URL directly matches a filter list rule. If
/// 'adScriptAncestry' is provided, an ad script initiated the resource fetch or
/// appended the element to the DOM. If neither is provided, the entity is
/// known to be an ad, but provenance tracking information is unavailable.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdProvenance<'a> {
    /// The filterlist rule that matched, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    filterlistRule: Option<Cow<'a, str>>,
    /// The script ancestry that created the ad, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    adScriptAncestry: Option<AdAncestry<'a>>,
}

impl<'a> AdProvenance<'a> {
    pub fn builder() -> AdProvenanceBuilder<'a> { AdProvenanceBuilder::default() }
    pub fn filterlistRule(&self) -> Option<&str> { self.filterlistRule.as_deref() }
    pub fn adScriptAncestry(&self) -> Option<&AdAncestry<'a>> { self.adScriptAncestry.as_ref() }
}

#[derive(Default)]
pub struct AdProvenanceBuilder<'a> {
    filterlistRule: Option<Cow<'a, str>>,
    adScriptAncestry: Option<AdAncestry<'a>>,
}

impl<'a> AdProvenanceBuilder<'a> {
    /// The filterlist rule that matched, if any.
    pub fn filterlistRule(mut self, filterlistRule: impl Into<Cow<'a, str>>) -> Self { self.filterlistRule = Some(filterlistRule.into()); self }
    /// The script ancestry that created the ad, if any.
    pub fn adScriptAncestry(mut self, adScriptAncestry: AdAncestry<'a>) -> Self { self.adScriptAncestry = Some(adScriptAncestry); self }
    pub fn build(self) -> AdProvenance<'a> {
        AdProvenance {
            filterlistRule: self.filterlistRule,
            adScriptAncestry: self.adScriptAncestry,
        }
    }
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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginOpenerPolicyStatus<'a> {
    value: CrossOriginOpenerPolicyValue,
    reportOnlyValue: CrossOriginOpenerPolicyValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    reportingEndpoint: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reportOnlyReportingEndpoint: Option<Cow<'a, str>>,
}

impl<'a> CrossOriginOpenerPolicyStatus<'a> {
    pub fn builder() -> CrossOriginOpenerPolicyStatusBuilder<'a> { CrossOriginOpenerPolicyStatusBuilder::default() }
    pub fn value(&self) -> &CrossOriginOpenerPolicyValue { &self.value }
    pub fn reportOnlyValue(&self) -> &CrossOriginOpenerPolicyValue { &self.reportOnlyValue }
    pub fn reportingEndpoint(&self) -> Option<&str> { self.reportingEndpoint.as_deref() }
    pub fn reportOnlyReportingEndpoint(&self) -> Option<&str> { self.reportOnlyReportingEndpoint.as_deref() }
}

#[derive(Default)]
pub struct CrossOriginOpenerPolicyStatusBuilder<'a> {
    value: Option<CrossOriginOpenerPolicyValue>,
    reportOnlyValue: Option<CrossOriginOpenerPolicyValue>,
    reportingEndpoint: Option<Cow<'a, str>>,
    reportOnlyReportingEndpoint: Option<Cow<'a, str>>,
}

impl<'a> CrossOriginOpenerPolicyStatusBuilder<'a> {
    pub fn value(mut self, value: CrossOriginOpenerPolicyValue) -> Self { self.value = Some(value); self }
    pub fn reportOnlyValue(mut self, reportOnlyValue: CrossOriginOpenerPolicyValue) -> Self { self.reportOnlyValue = Some(reportOnlyValue); self }
    pub fn reportingEndpoint(mut self, reportingEndpoint: impl Into<Cow<'a, str>>) -> Self { self.reportingEndpoint = Some(reportingEndpoint.into()); self }
    pub fn reportOnlyReportingEndpoint(mut self, reportOnlyReportingEndpoint: impl Into<Cow<'a, str>>) -> Self { self.reportOnlyReportingEndpoint = Some(reportOnlyReportingEndpoint.into()); self }
    pub fn build(self) -> CrossOriginOpenerPolicyStatus<'a> {
        CrossOriginOpenerPolicyStatus {
            value: self.value.unwrap_or_default(),
            reportOnlyValue: self.reportOnlyValue.unwrap_or_default(),
            reportingEndpoint: self.reportingEndpoint,
            reportOnlyReportingEndpoint: self.reportOnlyReportingEndpoint,
        }
    }
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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginEmbedderPolicyStatus<'a> {
    value: CrossOriginEmbedderPolicyValue,
    reportOnlyValue: CrossOriginEmbedderPolicyValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    reportingEndpoint: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reportOnlyReportingEndpoint: Option<Cow<'a, str>>,
}

impl<'a> CrossOriginEmbedderPolicyStatus<'a> {
    pub fn builder() -> CrossOriginEmbedderPolicyStatusBuilder<'a> { CrossOriginEmbedderPolicyStatusBuilder::default() }
    pub fn value(&self) -> &CrossOriginEmbedderPolicyValue { &self.value }
    pub fn reportOnlyValue(&self) -> &CrossOriginEmbedderPolicyValue { &self.reportOnlyValue }
    pub fn reportingEndpoint(&self) -> Option<&str> { self.reportingEndpoint.as_deref() }
    pub fn reportOnlyReportingEndpoint(&self) -> Option<&str> { self.reportOnlyReportingEndpoint.as_deref() }
}

#[derive(Default)]
pub struct CrossOriginEmbedderPolicyStatusBuilder<'a> {
    value: Option<CrossOriginEmbedderPolicyValue>,
    reportOnlyValue: Option<CrossOriginEmbedderPolicyValue>,
    reportingEndpoint: Option<Cow<'a, str>>,
    reportOnlyReportingEndpoint: Option<Cow<'a, str>>,
}

impl<'a> CrossOriginEmbedderPolicyStatusBuilder<'a> {
    pub fn value(mut self, value: CrossOriginEmbedderPolicyValue) -> Self { self.value = Some(value); self }
    pub fn reportOnlyValue(mut self, reportOnlyValue: CrossOriginEmbedderPolicyValue) -> Self { self.reportOnlyValue = Some(reportOnlyValue); self }
    pub fn reportingEndpoint(mut self, reportingEndpoint: impl Into<Cow<'a, str>>) -> Self { self.reportingEndpoint = Some(reportingEndpoint.into()); self }
    pub fn reportOnlyReportingEndpoint(mut self, reportOnlyReportingEndpoint: impl Into<Cow<'a, str>>) -> Self { self.reportOnlyReportingEndpoint = Some(reportOnlyReportingEndpoint.into()); self }
    pub fn build(self) -> CrossOriginEmbedderPolicyStatus<'a> {
        CrossOriginEmbedderPolicyStatus {
            value: self.value.unwrap_or_default(),
            reportOnlyValue: self.reportOnlyValue.unwrap_or_default(),
            reportingEndpoint: self.reportingEndpoint,
            reportOnlyReportingEndpoint: self.reportOnlyReportingEndpoint,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ContentSecurityPolicySource {
    #[default]
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "Meta")]
    Meta,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyStatus<'a> {
    effectiveDirectives: Cow<'a, str>,
    isEnforced: bool,
    source: ContentSecurityPolicySource,
}

impl<'a> ContentSecurityPolicyStatus<'a> {
    pub fn builder() -> ContentSecurityPolicyStatusBuilder<'a> { ContentSecurityPolicyStatusBuilder::default() }
    pub fn effectiveDirectives(&self) -> &str { self.effectiveDirectives.as_ref() }
    pub fn isEnforced(&self) -> bool { self.isEnforced }
    pub fn source(&self) -> &ContentSecurityPolicySource { &self.source }
}

#[derive(Default)]
pub struct ContentSecurityPolicyStatusBuilder<'a> {
    effectiveDirectives: Option<Cow<'a, str>>,
    isEnforced: Option<bool>,
    source: Option<ContentSecurityPolicySource>,
}

impl<'a> ContentSecurityPolicyStatusBuilder<'a> {
    pub fn effectiveDirectives(mut self, effectiveDirectives: impl Into<Cow<'a, str>>) -> Self { self.effectiveDirectives = Some(effectiveDirectives.into()); self }
    pub fn isEnforced(mut self, isEnforced: bool) -> Self { self.isEnforced = Some(isEnforced); self }
    pub fn source(mut self, source: ContentSecurityPolicySource) -> Self { self.source = Some(source); self }
    pub fn build(self) -> ContentSecurityPolicyStatus<'a> {
        ContentSecurityPolicyStatus {
            effectiveDirectives: self.effectiveDirectives.unwrap_or_default(),
            isEnforced: self.isEnforced.unwrap_or_default(),
            source: self.source.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityIsolationStatus<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    coop: Option<CrossOriginOpenerPolicyStatus<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coep: Option<CrossOriginEmbedderPolicyStatus<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csp: Option<Vec<ContentSecurityPolicyStatus<'a>>>,
}

impl<'a> SecurityIsolationStatus<'a> {
    pub fn builder() -> SecurityIsolationStatusBuilder<'a> { SecurityIsolationStatusBuilder::default() }
    pub fn coop(&self) -> Option<&CrossOriginOpenerPolicyStatus<'a>> { self.coop.as_ref() }
    pub fn coep(&self) -> Option<&CrossOriginEmbedderPolicyStatus<'a>> { self.coep.as_ref() }
    pub fn csp(&self) -> Option<&[ContentSecurityPolicyStatus<'a>]> { self.csp.as_deref() }
}

#[derive(Default)]
pub struct SecurityIsolationStatusBuilder<'a> {
    coop: Option<CrossOriginOpenerPolicyStatus<'a>>,
    coep: Option<CrossOriginEmbedderPolicyStatus<'a>>,
    csp: Option<Vec<ContentSecurityPolicyStatus<'a>>>,
}

impl<'a> SecurityIsolationStatusBuilder<'a> {
    pub fn coop(mut self, coop: CrossOriginOpenerPolicyStatus<'a>) -> Self { self.coop = Some(coop); self }
    pub fn coep(mut self, coep: CrossOriginEmbedderPolicyStatus<'a>) -> Self { self.coep = Some(coep); self }
    pub fn csp(mut self, csp: Vec<ContentSecurityPolicyStatus<'a>>) -> Self { self.csp = Some(csp); self }
    pub fn build(self) -> SecurityIsolationStatus<'a> {
        SecurityIsolationStatus {
            coop: self.coop,
            coep: self.coep,
            csp: self.csp,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportingApiReport<'a> {
    id: ReportId<'a>,
    /// The URL of the document that triggered the report.
    initiatorUrl: Cow<'a, str>,
    /// The name of the endpoint group that should be used to deliver the report.
    destination: Cow<'a, str>,
    /// The type of the report (specifies the set of data that is contained in the report body).
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// When the report was generated.
    timestamp: crate::network::TimeSinceEpoch,
    /// How many uploads deep the related request was.
    depth: i64,
    /// The number of delivery attempts made so far, not including an active attempt.
    completedAttempts: i64,
    body: serde_json::Map<String, JsonValue>,
    status: ReportStatus,
}

impl<'a> ReportingApiReport<'a> {
    pub fn builder() -> ReportingApiReportBuilder<'a> { ReportingApiReportBuilder::default() }
    pub fn id(&self) -> &ReportId<'a> { &self.id }
    pub fn initiatorUrl(&self) -> &str { self.initiatorUrl.as_ref() }
    pub fn destination(&self) -> &str { self.destination.as_ref() }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn timestamp(&self) -> &crate::network::TimeSinceEpoch { &self.timestamp }
    pub fn depth(&self) -> i64 { self.depth }
    pub fn completedAttempts(&self) -> i64 { self.completedAttempts }
    pub fn body(&self) -> &serde_json::Map<String, JsonValue> { &self.body }
    pub fn status(&self) -> &ReportStatus { &self.status }
}

#[derive(Default)]
pub struct ReportingApiReportBuilder<'a> {
    id: Option<ReportId<'a>>,
    initiatorUrl: Option<Cow<'a, str>>,
    destination: Option<Cow<'a, str>>,
    type_: Option<Cow<'a, str>>,
    timestamp: Option<crate::network::TimeSinceEpoch>,
    depth: Option<i64>,
    completedAttempts: Option<i64>,
    body: Option<serde_json::Map<String, JsonValue>>,
    status: Option<ReportStatus>,
}

impl<'a> ReportingApiReportBuilder<'a> {
    pub fn id(mut self, id: ReportId<'a>) -> Self { self.id = Some(id); self }
    /// The URL of the document that triggered the report.
    pub fn initiatorUrl(mut self, initiatorUrl: impl Into<Cow<'a, str>>) -> Self { self.initiatorUrl = Some(initiatorUrl.into()); self }
    /// The name of the endpoint group that should be used to deliver the report.
    pub fn destination(mut self, destination: impl Into<Cow<'a, str>>) -> Self { self.destination = Some(destination.into()); self }
    /// The type of the report (specifies the set of data that is contained in the report body).
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    /// When the report was generated.
    pub fn timestamp(mut self, timestamp: crate::network::TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    /// How many uploads deep the related request was.
    pub fn depth(mut self, depth: i64) -> Self { self.depth = Some(depth); self }
    /// The number of delivery attempts made so far, not including an active attempt.
    pub fn completedAttempts(mut self, completedAttempts: i64) -> Self { self.completedAttempts = Some(completedAttempts); self }
    pub fn body(mut self, body: serde_json::Map<String, JsonValue>) -> Self { self.body = Some(body); self }
    pub fn status(mut self, status: ReportStatus) -> Self { self.status = Some(status); self }
    pub fn build(self) -> ReportingApiReport<'a> {
        ReportingApiReport {
            id: self.id.unwrap_or_default(),
            initiatorUrl: self.initiatorUrl.unwrap_or_default(),
            destination: self.destination.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
            timestamp: self.timestamp.unwrap_or_default(),
            depth: self.depth.unwrap_or_default(),
            completedAttempts: self.completedAttempts.unwrap_or_default(),
            body: self.body.unwrap_or_default(),
            status: self.status.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportingApiEndpoint<'a> {
    /// The URL of the endpoint to which reports may be delivered.
    url: Cow<'a, str>,
    /// Name of the endpoint group.
    groupName: Cow<'a, str>,
}

impl<'a> ReportingApiEndpoint<'a> {
    pub fn builder() -> ReportingApiEndpointBuilder<'a> { ReportingApiEndpointBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn groupName(&self) -> &str { self.groupName.as_ref() }
}

#[derive(Default)]
pub struct ReportingApiEndpointBuilder<'a> {
    url: Option<Cow<'a, str>>,
    groupName: Option<Cow<'a, str>>,
}

impl<'a> ReportingApiEndpointBuilder<'a> {
    /// The URL of the endpoint to which reports may be delivered.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Name of the endpoint group.
    pub fn groupName(mut self, groupName: impl Into<Cow<'a, str>>) -> Self { self.groupName = Some(groupName.into()); self }
    pub fn build(self) -> ReportingApiEndpoint<'a> {
        ReportingApiEndpoint {
            url: self.url.unwrap_or_default(),
            groupName: self.groupName.unwrap_or_default(),
        }
    }
}

/// Unique identifier for a device bound session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionKey<'a> {
    /// The site the session is set up for.
    site: Cow<'a, str>,
    /// The id of the session.
    id: Cow<'a, str>,
}

impl<'a> DeviceBoundSessionKey<'a> {
    pub fn builder() -> DeviceBoundSessionKeyBuilder<'a> { DeviceBoundSessionKeyBuilder::default() }
    pub fn site(&self) -> &str { self.site.as_ref() }
    pub fn id(&self) -> &str { self.id.as_ref() }
}

#[derive(Default)]
pub struct DeviceBoundSessionKeyBuilder<'a> {
    site: Option<Cow<'a, str>>,
    id: Option<Cow<'a, str>>,
}

impl<'a> DeviceBoundSessionKeyBuilder<'a> {
    /// The site the session is set up for.
    pub fn site(mut self, site: impl Into<Cow<'a, str>>) -> Self { self.site = Some(site.into()); self }
    /// The id of the session.
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    pub fn build(self) -> DeviceBoundSessionKey<'a> {
        DeviceBoundSessionKey {
            site: self.site.unwrap_or_default(),
            id: self.id.unwrap_or_default(),
        }
    }
}

/// How a device bound session was used during a request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionWithUsage<'a> {
    /// The key for the session.
    sessionKey: DeviceBoundSessionKey<'a>,
    /// How the session was used (or not used).
    usage: Cow<'a, str>,
}

impl<'a> DeviceBoundSessionWithUsage<'a> {
    pub fn builder() -> DeviceBoundSessionWithUsageBuilder<'a> { DeviceBoundSessionWithUsageBuilder::default() }
    pub fn sessionKey(&self) -> &DeviceBoundSessionKey<'a> { &self.sessionKey }
    pub fn usage(&self) -> &str { self.usage.as_ref() }
}

#[derive(Default)]
pub struct DeviceBoundSessionWithUsageBuilder<'a> {
    sessionKey: Option<DeviceBoundSessionKey<'a>>,
    usage: Option<Cow<'a, str>>,
}

impl<'a> DeviceBoundSessionWithUsageBuilder<'a> {
    /// The key for the session.
    pub fn sessionKey(mut self, sessionKey: DeviceBoundSessionKey<'a>) -> Self { self.sessionKey = Some(sessionKey); self }
    /// How the session was used (or not used).
    pub fn usage(mut self, usage: impl Into<Cow<'a, str>>) -> Self { self.usage = Some(usage.into()); self }
    pub fn build(self) -> DeviceBoundSessionWithUsage<'a> {
        DeviceBoundSessionWithUsage {
            sessionKey: self.sessionKey.unwrap_or_default(),
            usage: self.usage.unwrap_or_default(),
        }
    }
}

/// A device bound session's cookie craving.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionCookieCraving<'a> {
    /// The name of the craving.
    name: Cow<'a, str>,
    /// The domain of the craving.
    domain: Cow<'a, str>,
    /// The path of the craving.
    path: Cow<'a, str>,
    /// The 'Secure' attribute of the craving attributes.
    secure: bool,
    /// The 'HttpOnly' attribute of the craving attributes.
    httpOnly: bool,
    /// The 'SameSite' attribute of the craving attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    sameSite: Option<CookieSameSite>,
}

impl<'a> DeviceBoundSessionCookieCraving<'a> {
    pub fn builder() -> DeviceBoundSessionCookieCravingBuilder<'a> { DeviceBoundSessionCookieCravingBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn domain(&self) -> &str { self.domain.as_ref() }
    pub fn path(&self) -> &str { self.path.as_ref() }
    pub fn secure(&self) -> bool { self.secure }
    pub fn httpOnly(&self) -> bool { self.httpOnly }
    pub fn sameSite(&self) -> Option<&CookieSameSite> { self.sameSite.as_ref() }
}

#[derive(Default)]
pub struct DeviceBoundSessionCookieCravingBuilder<'a> {
    name: Option<Cow<'a, str>>,
    domain: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    secure: Option<bool>,
    httpOnly: Option<bool>,
    sameSite: Option<CookieSameSite>,
}

impl<'a> DeviceBoundSessionCookieCravingBuilder<'a> {
    /// The name of the craving.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// The domain of the craving.
    pub fn domain(mut self, domain: impl Into<Cow<'a, str>>) -> Self { self.domain = Some(domain.into()); self }
    /// The path of the craving.
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    /// The 'Secure' attribute of the craving attributes.
    pub fn secure(mut self, secure: bool) -> Self { self.secure = Some(secure); self }
    /// The 'HttpOnly' attribute of the craving attributes.
    pub fn httpOnly(mut self, httpOnly: bool) -> Self { self.httpOnly = Some(httpOnly); self }
    /// The 'SameSite' attribute of the craving attributes.
    pub fn sameSite(mut self, sameSite: CookieSameSite) -> Self { self.sameSite = Some(sameSite); self }
    pub fn build(self) -> DeviceBoundSessionCookieCraving<'a> {
        DeviceBoundSessionCookieCraving {
            name: self.name.unwrap_or_default(),
            domain: self.domain.unwrap_or_default(),
            path: self.path.unwrap_or_default(),
            secure: self.secure.unwrap_or_default(),
            httpOnly: self.httpOnly.unwrap_or_default(),
            sameSite: self.sameSite,
        }
    }
}

/// A device bound session's inclusion URL rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionUrlRule<'a> {
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type'.
    ruleType: Cow<'a, str>,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern'.
    hostPattern: Cow<'a, str>,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix'.
    pathPrefix: Cow<'a, str>,
}

impl<'a> DeviceBoundSessionUrlRule<'a> {
    pub fn builder() -> DeviceBoundSessionUrlRuleBuilder<'a> { DeviceBoundSessionUrlRuleBuilder::default() }
    pub fn ruleType(&self) -> &str { self.ruleType.as_ref() }
    pub fn hostPattern(&self) -> &str { self.hostPattern.as_ref() }
    pub fn pathPrefix(&self) -> &str { self.pathPrefix.as_ref() }
}

#[derive(Default)]
pub struct DeviceBoundSessionUrlRuleBuilder<'a> {
    ruleType: Option<Cow<'a, str>>,
    hostPattern: Option<Cow<'a, str>>,
    pathPrefix: Option<Cow<'a, str>>,
}

impl<'a> DeviceBoundSessionUrlRuleBuilder<'a> {
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type'.
    pub fn ruleType(mut self, ruleType: impl Into<Cow<'a, str>>) -> Self { self.ruleType = Some(ruleType.into()); self }
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern'.
    pub fn hostPattern(mut self, hostPattern: impl Into<Cow<'a, str>>) -> Self { self.hostPattern = Some(hostPattern.into()); self }
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix'.
    pub fn pathPrefix(mut self, pathPrefix: impl Into<Cow<'a, str>>) -> Self { self.pathPrefix = Some(pathPrefix.into()); self }
    pub fn build(self) -> DeviceBoundSessionUrlRule<'a> {
        DeviceBoundSessionUrlRule {
            ruleType: self.ruleType.unwrap_or_default(),
            hostPattern: self.hostPattern.unwrap_or_default(),
            pathPrefix: self.pathPrefix.unwrap_or_default(),
        }
    }
}

/// A device bound session's inclusion rules.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionInclusionRules<'a> {
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::origin_'.
    origin: Cow<'a, str>,
    /// Whether the whole site is included. See comments on
    /// 'net::device_bound_sessions::SessionInclusionRules::include_site_' for more
    /// details; this boolean is true if that value is populated.
    includeSite: bool,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::url_rules_'.
    urlRules: Vec<DeviceBoundSessionUrlRule<'a>>,
}

impl<'a> DeviceBoundSessionInclusionRules<'a> {
    pub fn builder() -> DeviceBoundSessionInclusionRulesBuilder<'a> { DeviceBoundSessionInclusionRulesBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn includeSite(&self) -> bool { self.includeSite }
    pub fn urlRules(&self) -> &[DeviceBoundSessionUrlRule<'a>] { &self.urlRules }
}

#[derive(Default)]
pub struct DeviceBoundSessionInclusionRulesBuilder<'a> {
    origin: Option<Cow<'a, str>>,
    includeSite: Option<bool>,
    urlRules: Option<Vec<DeviceBoundSessionUrlRule<'a>>>,
}

impl<'a> DeviceBoundSessionInclusionRulesBuilder<'a> {
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::origin_'.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// Whether the whole site is included. See comments on
    /// 'net::device_bound_sessions::SessionInclusionRules::include_site_' for more
    /// details; this boolean is true if that value is populated.
    pub fn includeSite(mut self, includeSite: bool) -> Self { self.includeSite = Some(includeSite); self }
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::url_rules_'.
    pub fn urlRules(mut self, urlRules: Vec<DeviceBoundSessionUrlRule<'a>>) -> Self { self.urlRules = Some(urlRules); self }
    pub fn build(self) -> DeviceBoundSessionInclusionRules<'a> {
        DeviceBoundSessionInclusionRules {
            origin: self.origin.unwrap_or_default(),
            includeSite: self.includeSite.unwrap_or_default(),
            urlRules: self.urlRules.unwrap_or_default(),
        }
    }
}

/// A device bound session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSession<'a> {
    /// The site and session ID of the session.
    key: DeviceBoundSessionKey<'a>,
    /// See comments on 'net::device_bound_sessions::Session::refresh_url_'.
    refreshUrl: Cow<'a, str>,
    /// See comments on 'net::device_bound_sessions::Session::inclusion_rules_'.
    inclusionRules: DeviceBoundSessionInclusionRules<'a>,
    /// See comments on 'net::device_bound_sessions::Session::cookie_cravings_'.
    cookieCravings: Vec<DeviceBoundSessionCookieCraving<'a>>,
    /// See comments on 'net::device_bound_sessions::Session::expiry_date_'.
    expiryDate: crate::network::TimeSinceEpoch,
    /// See comments on 'net::device_bound_sessions::Session::cached_challenge__'.
    #[serde(skip_serializing_if = "Option::is_none")]
    cachedChallenge: Option<Cow<'a, str>>,
    /// See comments on 'net::device_bound_sessions::Session::allowed_refresh_initiators_'.
    allowedRefreshInitiators: Vec<Cow<'a, str>>,
}

impl<'a> DeviceBoundSession<'a> {
    pub fn builder() -> DeviceBoundSessionBuilder<'a> { DeviceBoundSessionBuilder::default() }
    pub fn key(&self) -> &DeviceBoundSessionKey<'a> { &self.key }
    pub fn refreshUrl(&self) -> &str { self.refreshUrl.as_ref() }
    pub fn inclusionRules(&self) -> &DeviceBoundSessionInclusionRules<'a> { &self.inclusionRules }
    pub fn cookieCravings(&self) -> &[DeviceBoundSessionCookieCraving<'a>] { &self.cookieCravings }
    pub fn expiryDate(&self) -> &crate::network::TimeSinceEpoch { &self.expiryDate }
    pub fn cachedChallenge(&self) -> Option<&str> { self.cachedChallenge.as_deref() }
    pub fn allowedRefreshInitiators(&self) -> &[Cow<'a, str>] { &self.allowedRefreshInitiators }
}

#[derive(Default)]
pub struct DeviceBoundSessionBuilder<'a> {
    key: Option<DeviceBoundSessionKey<'a>>,
    refreshUrl: Option<Cow<'a, str>>,
    inclusionRules: Option<DeviceBoundSessionInclusionRules<'a>>,
    cookieCravings: Option<Vec<DeviceBoundSessionCookieCraving<'a>>>,
    expiryDate: Option<crate::network::TimeSinceEpoch>,
    cachedChallenge: Option<Cow<'a, str>>,
    allowedRefreshInitiators: Option<Vec<Cow<'a, str>>>,
}

impl<'a> DeviceBoundSessionBuilder<'a> {
    /// The site and session ID of the session.
    pub fn key(mut self, key: DeviceBoundSessionKey<'a>) -> Self { self.key = Some(key); self }
    /// See comments on 'net::device_bound_sessions::Session::refresh_url_'.
    pub fn refreshUrl(mut self, refreshUrl: impl Into<Cow<'a, str>>) -> Self { self.refreshUrl = Some(refreshUrl.into()); self }
    /// See comments on 'net::device_bound_sessions::Session::inclusion_rules_'.
    pub fn inclusionRules(mut self, inclusionRules: DeviceBoundSessionInclusionRules<'a>) -> Self { self.inclusionRules = Some(inclusionRules); self }
    /// See comments on 'net::device_bound_sessions::Session::cookie_cravings_'.
    pub fn cookieCravings(mut self, cookieCravings: Vec<DeviceBoundSessionCookieCraving<'a>>) -> Self { self.cookieCravings = Some(cookieCravings); self }
    /// See comments on 'net::device_bound_sessions::Session::expiry_date_'.
    pub fn expiryDate(mut self, expiryDate: crate::network::TimeSinceEpoch) -> Self { self.expiryDate = Some(expiryDate); self }
    /// See comments on 'net::device_bound_sessions::Session::cached_challenge__'.
    pub fn cachedChallenge(mut self, cachedChallenge: impl Into<Cow<'a, str>>) -> Self { self.cachedChallenge = Some(cachedChallenge.into()); self }
    /// See comments on 'net::device_bound_sessions::Session::allowed_refresh_initiators_'.
    pub fn allowedRefreshInitiators(mut self, allowedRefreshInitiators: Vec<Cow<'a, str>>) -> Self { self.allowedRefreshInitiators = Some(allowedRefreshInitiators); self }
    pub fn build(self) -> DeviceBoundSession<'a> {
        DeviceBoundSession {
            key: self.key.unwrap_or_default(),
            refreshUrl: self.refreshUrl.unwrap_or_default(),
            inclusionRules: self.inclusionRules.unwrap_or_default(),
            cookieCravings: self.cookieCravings.unwrap_or_default(),
            expiryDate: self.expiryDate.unwrap_or_default(),
            cachedChallenge: self.cachedChallenge,
            allowedRefreshInitiators: self.allowedRefreshInitiators.unwrap_or_default(),
        }
    }
}

/// A unique identifier for a device bound session event.

pub type DeviceBoundSessionEventId<'a> = Cow<'a, str>;

/// A fetch result for a device bound session creation or refresh.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DeviceBoundSessionFetchResult {
    #[default]
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "KeyError")]
    KeyError,
    #[serde(rename = "SigningError")]
    SigningError,
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
}

/// Details about a failed device bound session network request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionFailedRequest<'a> {
    /// The failed request URL.
    requestUrl: Cow<'a, str>,
    /// The net error of the response if it was not OK.
    #[serde(skip_serializing_if = "Option::is_none")]
    netError: Option<Cow<'a, str>>,
    /// The response code if the net error was OK and the response code was not
    /// 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    responseError: Option<i64>,
    /// The body of the response if the net error was OK, the response code was
    /// not 200, and the response body was not empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    responseErrorBody: Option<Cow<'a, str>>,
}

impl<'a> DeviceBoundSessionFailedRequest<'a> {
    pub fn builder() -> DeviceBoundSessionFailedRequestBuilder<'a> { DeviceBoundSessionFailedRequestBuilder::default() }
    pub fn requestUrl(&self) -> &str { self.requestUrl.as_ref() }
    pub fn netError(&self) -> Option<&str> { self.netError.as_deref() }
    pub fn responseError(&self) -> Option<i64> { self.responseError }
    pub fn responseErrorBody(&self) -> Option<&str> { self.responseErrorBody.as_deref() }
}

#[derive(Default)]
pub struct DeviceBoundSessionFailedRequestBuilder<'a> {
    requestUrl: Option<Cow<'a, str>>,
    netError: Option<Cow<'a, str>>,
    responseError: Option<i64>,
    responseErrorBody: Option<Cow<'a, str>>,
}

impl<'a> DeviceBoundSessionFailedRequestBuilder<'a> {
    /// The failed request URL.
    pub fn requestUrl(mut self, requestUrl: impl Into<Cow<'a, str>>) -> Self { self.requestUrl = Some(requestUrl.into()); self }
    /// The net error of the response if it was not OK.
    pub fn netError(mut self, netError: impl Into<Cow<'a, str>>) -> Self { self.netError = Some(netError.into()); self }
    /// The response code if the net error was OK and the response code was not
    /// 200.
    pub fn responseError(mut self, responseError: i64) -> Self { self.responseError = Some(responseError); self }
    /// The body of the response if the net error was OK, the response code was
    /// not 200, and the response body was not empty.
    pub fn responseErrorBody(mut self, responseErrorBody: impl Into<Cow<'a, str>>) -> Self { self.responseErrorBody = Some(responseErrorBody.into()); self }
    pub fn build(self) -> DeviceBoundSessionFailedRequest<'a> {
        DeviceBoundSessionFailedRequest {
            requestUrl: self.requestUrl.unwrap_or_default(),
            netError: self.netError,
            responseError: self.responseError,
            responseErrorBody: self.responseErrorBody,
        }
    }
}

/// Session event details specific to creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreationEventDetails<'a> {
    /// The result of the fetch attempt.
    fetchResult: DeviceBoundSessionFetchResult,
    /// The session if there was a newly created session. This is populated for
    /// all successful creation events.
    #[serde(skip_serializing_if = "Option::is_none")]
    newSession: Option<DeviceBoundSession<'a>>,
    /// Details about a failed device bound session network request if there was
    /// one.
    #[serde(skip_serializing_if = "Option::is_none")]
    failedRequest: Option<DeviceBoundSessionFailedRequest<'a>>,
}

impl<'a> CreationEventDetails<'a> {
    pub fn builder() -> CreationEventDetailsBuilder<'a> { CreationEventDetailsBuilder::default() }
    pub fn fetchResult(&self) -> &DeviceBoundSessionFetchResult { &self.fetchResult }
    pub fn newSession(&self) -> Option<&DeviceBoundSession<'a>> { self.newSession.as_ref() }
    pub fn failedRequest(&self) -> Option<&DeviceBoundSessionFailedRequest<'a>> { self.failedRequest.as_ref() }
}

#[derive(Default)]
pub struct CreationEventDetailsBuilder<'a> {
    fetchResult: Option<DeviceBoundSessionFetchResult>,
    newSession: Option<DeviceBoundSession<'a>>,
    failedRequest: Option<DeviceBoundSessionFailedRequest<'a>>,
}

impl<'a> CreationEventDetailsBuilder<'a> {
    /// The result of the fetch attempt.
    pub fn fetchResult(mut self, fetchResult: DeviceBoundSessionFetchResult) -> Self { self.fetchResult = Some(fetchResult); self }
    /// The session if there was a newly created session. This is populated for
    /// all successful creation events.
    pub fn newSession(mut self, newSession: DeviceBoundSession<'a>) -> Self { self.newSession = Some(newSession); self }
    /// Details about a failed device bound session network request if there was
    /// one.
    pub fn failedRequest(mut self, failedRequest: DeviceBoundSessionFailedRequest<'a>) -> Self { self.failedRequest = Some(failedRequest); self }
    pub fn build(self) -> CreationEventDetails<'a> {
        CreationEventDetails {
            fetchResult: self.fetchResult.unwrap_or_default(),
            newSession: self.newSession,
            failedRequest: self.failedRequest,
        }
    }
}

/// Session event details specific to refresh.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RefreshEventDetails<'a> {
    /// The result of a refresh.
    refreshResult: Cow<'a, str>,
    /// If there was a fetch attempt, the result of that.
    #[serde(skip_serializing_if = "Option::is_none")]
    fetchResult: Option<DeviceBoundSessionFetchResult>,
    /// The session display if there was a newly created session. This is populated
    /// for any refresh event that modifies the session config.
    #[serde(skip_serializing_if = "Option::is_none")]
    newSession: Option<DeviceBoundSession<'a>>,
    /// See comments on 'net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh'.
    wasFullyProactiveRefresh: bool,
    /// Details about a failed device bound session network request if there was
    /// one.
    #[serde(skip_serializing_if = "Option::is_none")]
    failedRequest: Option<DeviceBoundSessionFailedRequest<'a>>,
}

impl<'a> RefreshEventDetails<'a> {
    pub fn builder() -> RefreshEventDetailsBuilder<'a> { RefreshEventDetailsBuilder::default() }
    pub fn refreshResult(&self) -> &str { self.refreshResult.as_ref() }
    pub fn fetchResult(&self) -> Option<&DeviceBoundSessionFetchResult> { self.fetchResult.as_ref() }
    pub fn newSession(&self) -> Option<&DeviceBoundSession<'a>> { self.newSession.as_ref() }
    pub fn wasFullyProactiveRefresh(&self) -> bool { self.wasFullyProactiveRefresh }
    pub fn failedRequest(&self) -> Option<&DeviceBoundSessionFailedRequest<'a>> { self.failedRequest.as_ref() }
}

#[derive(Default)]
pub struct RefreshEventDetailsBuilder<'a> {
    refreshResult: Option<Cow<'a, str>>,
    fetchResult: Option<DeviceBoundSessionFetchResult>,
    newSession: Option<DeviceBoundSession<'a>>,
    wasFullyProactiveRefresh: Option<bool>,
    failedRequest: Option<DeviceBoundSessionFailedRequest<'a>>,
}

impl<'a> RefreshEventDetailsBuilder<'a> {
    /// The result of a refresh.
    pub fn refreshResult(mut self, refreshResult: impl Into<Cow<'a, str>>) -> Self { self.refreshResult = Some(refreshResult.into()); self }
    /// If there was a fetch attempt, the result of that.
    pub fn fetchResult(mut self, fetchResult: DeviceBoundSessionFetchResult) -> Self { self.fetchResult = Some(fetchResult); self }
    /// The session display if there was a newly created session. This is populated
    /// for any refresh event that modifies the session config.
    pub fn newSession(mut self, newSession: DeviceBoundSession<'a>) -> Self { self.newSession = Some(newSession); self }
    /// See comments on 'net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh'.
    pub fn wasFullyProactiveRefresh(mut self, wasFullyProactiveRefresh: bool) -> Self { self.wasFullyProactiveRefresh = Some(wasFullyProactiveRefresh); self }
    /// Details about a failed device bound session network request if there was
    /// one.
    pub fn failedRequest(mut self, failedRequest: DeviceBoundSessionFailedRequest<'a>) -> Self { self.failedRequest = Some(failedRequest); self }
    pub fn build(self) -> RefreshEventDetails<'a> {
        RefreshEventDetails {
            refreshResult: self.refreshResult.unwrap_or_default(),
            fetchResult: self.fetchResult,
            newSession: self.newSession,
            wasFullyProactiveRefresh: self.wasFullyProactiveRefresh.unwrap_or_default(),
            failedRequest: self.failedRequest,
        }
    }
}

/// Session event details specific to termination.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TerminationEventDetails<'a> {
    /// The reason for a session being deleted.
    deletionReason: Cow<'a, str>,
}

impl<'a> TerminationEventDetails<'a> {
    pub fn builder() -> TerminationEventDetailsBuilder<'a> { TerminationEventDetailsBuilder::default() }
    pub fn deletionReason(&self) -> &str { self.deletionReason.as_ref() }
}

#[derive(Default)]
pub struct TerminationEventDetailsBuilder<'a> {
    deletionReason: Option<Cow<'a, str>>,
}

impl<'a> TerminationEventDetailsBuilder<'a> {
    /// The reason for a session being deleted.
    pub fn deletionReason(mut self, deletionReason: impl Into<Cow<'a, str>>) -> Self { self.deletionReason = Some(deletionReason.into()); self }
    pub fn build(self) -> TerminationEventDetails<'a> {
        TerminationEventDetails {
            deletionReason: self.deletionReason.unwrap_or_default(),
        }
    }
}

/// Session event details specific to challenges.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeEventDetails<'a> {
    /// The result of a challenge.
    challengeResult: Cow<'a, str>,
    /// The challenge set.
    challenge: Cow<'a, str>,
}

impl<'a> ChallengeEventDetails<'a> {
    pub fn builder() -> ChallengeEventDetailsBuilder<'a> { ChallengeEventDetailsBuilder::default() }
    pub fn challengeResult(&self) -> &str { self.challengeResult.as_ref() }
    pub fn challenge(&self) -> &str { self.challenge.as_ref() }
}

#[derive(Default)]
pub struct ChallengeEventDetailsBuilder<'a> {
    challengeResult: Option<Cow<'a, str>>,
    challenge: Option<Cow<'a, str>>,
}

impl<'a> ChallengeEventDetailsBuilder<'a> {
    /// The result of a challenge.
    pub fn challengeResult(mut self, challengeResult: impl Into<Cow<'a, str>>) -> Self { self.challengeResult = Some(challengeResult.into()); self }
    /// The challenge set.
    pub fn challenge(mut self, challenge: impl Into<Cow<'a, str>>) -> Self { self.challenge = Some(challenge.into()); self }
    pub fn build(self) -> ChallengeEventDetails<'a> {
        ChallengeEventDetails {
            challengeResult: self.challengeResult.unwrap_or_default(),
            challenge: self.challenge.unwrap_or_default(),
        }
    }
}

/// An object providing the result of a network resource load.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourcePageResult<'a> {
    success: bool,
    /// Optional values used for error reporting.
    #[serde(skip_serializing_if = "Option::is_none")]
    netError: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    netErrorName: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    httpStatusCode: Option<f64>,
    /// If successful, one of the following two fields holds the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<crate::io::StreamHandle<'a>>,
    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<crate::network::Headers>,
}

impl<'a> LoadNetworkResourcePageResult<'a> {
    pub fn builder() -> LoadNetworkResourcePageResultBuilder<'a> { LoadNetworkResourcePageResultBuilder::default() }
    pub fn success(&self) -> bool { self.success }
    pub fn netError(&self) -> Option<f64> { self.netError }
    pub fn netErrorName(&self) -> Option<&str> { self.netErrorName.as_deref() }
    pub fn httpStatusCode(&self) -> Option<f64> { self.httpStatusCode }
    pub fn stream(&self) -> Option<&crate::io::StreamHandle<'a>> { self.stream.as_ref() }
    pub fn headers(&self) -> Option<&crate::network::Headers> { self.headers.as_ref() }
}

#[derive(Default)]
pub struct LoadNetworkResourcePageResultBuilder<'a> {
    success: Option<bool>,
    netError: Option<f64>,
    netErrorName: Option<Cow<'a, str>>,
    httpStatusCode: Option<f64>,
    stream: Option<crate::io::StreamHandle<'a>>,
    headers: Option<crate::network::Headers>,
}

impl<'a> LoadNetworkResourcePageResultBuilder<'a> {
    pub fn success(mut self, success: bool) -> Self { self.success = Some(success); self }
    /// Optional values used for error reporting.
    pub fn netError(mut self, netError: f64) -> Self { self.netError = Some(netError); self }
    pub fn netErrorName(mut self, netErrorName: impl Into<Cow<'a, str>>) -> Self { self.netErrorName = Some(netErrorName.into()); self }
    pub fn httpStatusCode(mut self, httpStatusCode: f64) -> Self { self.httpStatusCode = Some(httpStatusCode); self }
    /// If successful, one of the following two fields holds the result.
    pub fn stream(mut self, stream: crate::io::StreamHandle<'a>) -> Self { self.stream = Some(stream); self }
    /// Response headers.
    pub fn headers(mut self, headers: crate::network::Headers) -> Self { self.headers = Some(headers); self }
    pub fn build(self) -> LoadNetworkResourcePageResult<'a> {
        LoadNetworkResourcePageResult {
            success: self.success.unwrap_or_default(),
            netError: self.netError,
            netErrorName: self.netErrorName,
            httpStatusCode: self.httpStatusCode,
            stream: self.stream,
            headers: self.headers,
        }
    }
}

/// An options object that may be extended later to better support CORS,
/// CORB and streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceOptions {
    disableCache: bool,
    includeCredentials: bool,
}

impl LoadNetworkResourceOptions {
    pub fn builder() -> LoadNetworkResourceOptionsBuilder { LoadNetworkResourceOptionsBuilder::default() }
    pub fn disableCache(&self) -> bool { self.disableCache }
    pub fn includeCredentials(&self) -> bool { self.includeCredentials }
}

#[derive(Default)]
pub struct LoadNetworkResourceOptionsBuilder {
    disableCache: Option<bool>,
    includeCredentials: Option<bool>,
}

impl LoadNetworkResourceOptionsBuilder {
    pub fn disableCache(mut self, disableCache: bool) -> Self { self.disableCache = Some(disableCache); self }
    pub fn includeCredentials(mut self, includeCredentials: bool) -> Self { self.includeCredentials = Some(includeCredentials); self }
    pub fn build(self) -> LoadNetworkResourceOptions {
        LoadNetworkResourceOptions {
            disableCache: self.disableCache.unwrap_or_default(),
            includeCredentials: self.includeCredentials.unwrap_or_default(),
        }
    }
}

/// Sets a list of content encodings that will be accepted. Empty list means no encoding is accepted.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAcceptedEncodingsParams {
    /// List of accepted content encodings.
    encodings: Vec<ContentEncoding>,
}

impl SetAcceptedEncodingsParams {
    pub fn builder() -> SetAcceptedEncodingsParamsBuilder { SetAcceptedEncodingsParamsBuilder::default() }
    pub fn encodings(&self) -> &[ContentEncoding] { &self.encodings }
}

#[derive(Default)]
pub struct SetAcceptedEncodingsParamsBuilder {
    encodings: Option<Vec<ContentEncoding>>,
}

impl SetAcceptedEncodingsParamsBuilder {
    /// List of accepted content encodings.
    pub fn encodings(mut self, encodings: Vec<ContentEncoding>) -> Self { self.encodings = Some(encodings); self }
    pub fn build(self) -> SetAcceptedEncodingsParams {
        SetAcceptedEncodingsParams {
            encodings: self.encodings.unwrap_or_default(),
        }
    }
}

impl SetAcceptedEncodingsParams { pub const METHOD: &'static str = "Network.setAcceptedEncodings"; }

impl<'a> crate::CdpCommand<'a> for SetAcceptedEncodingsParams {
    const METHOD: &'static str = "Network.setAcceptedEncodings";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearAcceptedEncodingsOverrideParams {}

impl ClearAcceptedEncodingsOverrideParams {
    pub fn builder() -> ClearAcceptedEncodingsOverrideParamsBuilder {
        ClearAcceptedEncodingsOverrideParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ClearAcceptedEncodingsOverrideParamsBuilder {}

impl ClearAcceptedEncodingsOverrideParamsBuilder {
    pub fn build(self) -> ClearAcceptedEncodingsOverrideParams {
        ClearAcceptedEncodingsOverrideParams {}
    }
}

impl ClearAcceptedEncodingsOverrideParams { pub const METHOD: &'static str = "Network.clearAcceptedEncodingsOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearAcceptedEncodingsOverrideParams {
    const METHOD: &'static str = "Network.clearAcceptedEncodingsOverride";
    type Response = crate::EmptyReturns;
}

/// Tells whether clearing browser cache is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCacheReturns {
    /// True if browser cache can be cleared.
    result: bool,
}

impl CanClearBrowserCacheReturns {
    pub fn builder() -> CanClearBrowserCacheReturnsBuilder { CanClearBrowserCacheReturnsBuilder::default() }
    pub fn result(&self) -> bool { self.result }
}

#[derive(Default)]
pub struct CanClearBrowserCacheReturnsBuilder {
    result: Option<bool>,
}

impl CanClearBrowserCacheReturnsBuilder {
    /// True if browser cache can be cleared.
    pub fn result(mut self, result: bool) -> Self { self.result = Some(result); self }
    pub fn build(self) -> CanClearBrowserCacheReturns {
        CanClearBrowserCacheReturns {
            result: self.result.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanClearBrowserCacheParams {}

impl CanClearBrowserCacheParams {
    pub fn builder() -> CanClearBrowserCacheParamsBuilder {
        CanClearBrowserCacheParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct CanClearBrowserCacheParamsBuilder {}

impl CanClearBrowserCacheParamsBuilder {
    pub fn build(self) -> CanClearBrowserCacheParams {
        CanClearBrowserCacheParams {}
    }
}

impl CanClearBrowserCacheParams { pub const METHOD: &'static str = "Network.canClearBrowserCache"; }

impl<'a> crate::CdpCommand<'a> for CanClearBrowserCacheParams {
    const METHOD: &'static str = "Network.canClearBrowserCache";
    type Response = CanClearBrowserCacheReturns;
}

/// Tells whether clearing browser cookies is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCookiesReturns {
    /// True if browser cookies can be cleared.
    result: bool,
}

impl CanClearBrowserCookiesReturns {
    pub fn builder() -> CanClearBrowserCookiesReturnsBuilder { CanClearBrowserCookiesReturnsBuilder::default() }
    pub fn result(&self) -> bool { self.result }
}

#[derive(Default)]
pub struct CanClearBrowserCookiesReturnsBuilder {
    result: Option<bool>,
}

impl CanClearBrowserCookiesReturnsBuilder {
    /// True if browser cookies can be cleared.
    pub fn result(mut self, result: bool) -> Self { self.result = Some(result); self }
    pub fn build(self) -> CanClearBrowserCookiesReturns {
        CanClearBrowserCookiesReturns {
            result: self.result.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanClearBrowserCookiesParams {}

impl CanClearBrowserCookiesParams {
    pub fn builder() -> CanClearBrowserCookiesParamsBuilder {
        CanClearBrowserCookiesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct CanClearBrowserCookiesParamsBuilder {}

impl CanClearBrowserCookiesParamsBuilder {
    pub fn build(self) -> CanClearBrowserCookiesParams {
        CanClearBrowserCookiesParams {}
    }
}

impl CanClearBrowserCookiesParams { pub const METHOD: &'static str = "Network.canClearBrowserCookies"; }

impl<'a> crate::CdpCommand<'a> for CanClearBrowserCookiesParams {
    const METHOD: &'static str = "Network.canClearBrowserCookies";
    type Response = CanClearBrowserCookiesReturns;
}

/// Tells whether emulation of network conditions is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulateNetworkConditionsReturns {
    /// True if emulation of network conditions is supported.
    result: bool,
}

impl CanEmulateNetworkConditionsReturns {
    pub fn builder() -> CanEmulateNetworkConditionsReturnsBuilder { CanEmulateNetworkConditionsReturnsBuilder::default() }
    pub fn result(&self) -> bool { self.result }
}

#[derive(Default)]
pub struct CanEmulateNetworkConditionsReturnsBuilder {
    result: Option<bool>,
}

impl CanEmulateNetworkConditionsReturnsBuilder {
    /// True if emulation of network conditions is supported.
    pub fn result(mut self, result: bool) -> Self { self.result = Some(result); self }
    pub fn build(self) -> CanEmulateNetworkConditionsReturns {
        CanEmulateNetworkConditionsReturns {
            result: self.result.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanEmulateNetworkConditionsParams {}

impl CanEmulateNetworkConditionsParams {
    pub fn builder() -> CanEmulateNetworkConditionsParamsBuilder {
        CanEmulateNetworkConditionsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct CanEmulateNetworkConditionsParamsBuilder {}

impl CanEmulateNetworkConditionsParamsBuilder {
    pub fn build(self) -> CanEmulateNetworkConditionsParams {
        CanEmulateNetworkConditionsParams {}
    }
}

impl CanEmulateNetworkConditionsParams { pub const METHOD: &'static str = "Network.canEmulateNetworkConditions"; }

impl<'a> crate::CdpCommand<'a> for CanEmulateNetworkConditionsParams {
    const METHOD: &'static str = "Network.canEmulateNetworkConditions";
    type Response = CanEmulateNetworkConditionsReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearBrowserCacheParams {}

impl ClearBrowserCacheParams {
    pub fn builder() -> ClearBrowserCacheParamsBuilder {
        ClearBrowserCacheParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ClearBrowserCacheParamsBuilder {}

impl ClearBrowserCacheParamsBuilder {
    pub fn build(self) -> ClearBrowserCacheParams {
        ClearBrowserCacheParams {}
    }
}

impl ClearBrowserCacheParams { pub const METHOD: &'static str = "Network.clearBrowserCache"; }

impl<'a> crate::CdpCommand<'a> for ClearBrowserCacheParams {
    const METHOD: &'static str = "Network.clearBrowserCache";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearBrowserCookiesParams {}

impl ClearBrowserCookiesParams {
    pub fn builder() -> ClearBrowserCookiesParamsBuilder {
        ClearBrowserCookiesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ClearBrowserCookiesParamsBuilder {}

impl ClearBrowserCookiesParamsBuilder {
    pub fn build(self) -> ClearBrowserCookiesParams {
        ClearBrowserCookiesParams {}
    }
}

impl ClearBrowserCookiesParams { pub const METHOD: &'static str = "Network.clearBrowserCookies"; }

impl<'a> crate::CdpCommand<'a> for ClearBrowserCookiesParams {
    const METHOD: &'static str = "Network.clearBrowserCookies";
    type Response = crate::EmptyReturns;
}

/// Response to Network.requestIntercepted which either modifies the request to continue with any
/// modifications, or blocks it, or completes it with the provided response bytes. If a network
/// fetch occurs as a result which encounters a redirect an additional Network.requestIntercepted
/// event will be sent with the same InterceptionId.
/// Deprecated, use Fetch.continueRequest, Fetch.fulfillRequest and Fetch.failRequest instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueInterceptedRequestParams<'a> {
    interceptionId: InterceptionId<'a>,
    /// If set this causes the request to fail with the given reason. Passing 'Aborted' for requests
    /// marked with 'isNavigationRequest' also cancels the navigation. Must not be set in response
    /// to an authChallenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    errorReason: Option<ErrorReason>,
    /// If set the requests completes using with the provided base64 encoded raw response, including
    /// HTTP status line and headers etc... Must not be set in response to an authChallenge. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    rawResponse: Option<Cow<'a, str>>,
    /// If set the request url will be modified in a way that's not observable by page. Must not be
    /// set in response to an authChallenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// If set this allows the request method to be overridden. Must not be set in response to an
    /// authChallenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<Cow<'a, str>>,
    /// If set this allows postData to be set. Must not be set in response to an authChallenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    postData: Option<Cow<'a, str>>,
    /// If set this allows the request headers to be changed. Must not be set in response to an
    /// authChallenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Headers>,
    /// Response to a requestIntercepted with an authChallenge. Must not be set otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    authChallengeResponse: Option<AuthChallengeResponse<'a>>,
}

impl<'a> ContinueInterceptedRequestParams<'a> {
    pub fn builder() -> ContinueInterceptedRequestParamsBuilder<'a> { ContinueInterceptedRequestParamsBuilder::default() }
    pub fn interceptionId(&self) -> &InterceptionId<'a> { &self.interceptionId }
    pub fn errorReason(&self) -> Option<&ErrorReason> { self.errorReason.as_ref() }
    pub fn rawResponse(&self) -> Option<&str> { self.rawResponse.as_deref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn method(&self) -> Option<&str> { self.method.as_deref() }
    pub fn postData(&self) -> Option<&str> { self.postData.as_deref() }
    pub fn headers(&self) -> Option<&Headers> { self.headers.as_ref() }
    pub fn authChallengeResponse(&self) -> Option<&AuthChallengeResponse<'a>> { self.authChallengeResponse.as_ref() }
}

#[derive(Default)]
pub struct ContinueInterceptedRequestParamsBuilder<'a> {
    interceptionId: Option<InterceptionId<'a>>,
    errorReason: Option<ErrorReason>,
    rawResponse: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    method: Option<Cow<'a, str>>,
    postData: Option<Cow<'a, str>>,
    headers: Option<Headers>,
    authChallengeResponse: Option<AuthChallengeResponse<'a>>,
}

impl<'a> ContinueInterceptedRequestParamsBuilder<'a> {
    pub fn interceptionId(mut self, interceptionId: InterceptionId<'a>) -> Self { self.interceptionId = Some(interceptionId); self }
    /// If set this causes the request to fail with the given reason. Passing 'Aborted' for requests
    /// marked with 'isNavigationRequest' also cancels the navigation. Must not be set in response
    /// to an authChallenge.
    pub fn errorReason(mut self, errorReason: ErrorReason) -> Self { self.errorReason = Some(errorReason); self }
    /// If set the requests completes using with the provided base64 encoded raw response, including
    /// HTTP status line and headers etc... Must not be set in response to an authChallenge. (Encoded as a base64 string when passed over JSON)
    pub fn rawResponse(mut self, rawResponse: impl Into<Cow<'a, str>>) -> Self { self.rawResponse = Some(rawResponse.into()); self }
    /// If set the request url will be modified in a way that's not observable by page. Must not be
    /// set in response to an authChallenge.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// If set this allows the request method to be overridden. Must not be set in response to an
    /// authChallenge.
    pub fn method(mut self, method: impl Into<Cow<'a, str>>) -> Self { self.method = Some(method.into()); self }
    /// If set this allows postData to be set. Must not be set in response to an authChallenge.
    pub fn postData(mut self, postData: impl Into<Cow<'a, str>>) -> Self { self.postData = Some(postData.into()); self }
    /// If set this allows the request headers to be changed. Must not be set in response to an
    /// authChallenge.
    pub fn headers(mut self, headers: Headers) -> Self { self.headers = Some(headers); self }
    /// Response to a requestIntercepted with an authChallenge. Must not be set otherwise.
    pub fn authChallengeResponse(mut self, authChallengeResponse: AuthChallengeResponse<'a>) -> Self { self.authChallengeResponse = Some(authChallengeResponse); self }
    pub fn build(self) -> ContinueInterceptedRequestParams<'a> {
        ContinueInterceptedRequestParams {
            interceptionId: self.interceptionId.unwrap_or_default(),
            errorReason: self.errorReason,
            rawResponse: self.rawResponse,
            url: self.url,
            method: self.method,
            postData: self.postData,
            headers: self.headers,
            authChallengeResponse: self.authChallengeResponse,
        }
    }
}

impl<'a> ContinueInterceptedRequestParams<'a> { pub const METHOD: &'static str = "Network.continueInterceptedRequest"; }

impl<'a> crate::CdpCommand<'a> for ContinueInterceptedRequestParams<'a> {
    const METHOD: &'static str = "Network.continueInterceptedRequest";
    type Response = crate::EmptyReturns;
}

/// Deletes browser cookies with matching name and url or domain/path/partitionKey pair.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCookiesParams<'a> {
    /// Name of the cookies to remove.
    name: Cow<'a, str>,
    /// If specified, deletes all the cookies with the given name where domain and path match
    /// provided URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// If specified, deletes only cookies with the exact domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<Cow<'a, str>>,
    /// If specified, deletes only cookies with the exact path.
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<Cow<'a, str>>,
    /// If specified, deletes only cookies with the the given name and partitionKey where
    /// all partition key attributes match the cookie partition key attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    partitionKey: Option<CookiePartitionKey<'a>>,
}

impl<'a> DeleteCookiesParams<'a> {
    pub fn builder() -> DeleteCookiesParamsBuilder<'a> { DeleteCookiesParamsBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn domain(&self) -> Option<&str> { self.domain.as_deref() }
    pub fn path(&self) -> Option<&str> { self.path.as_deref() }
    pub fn partitionKey(&self) -> Option<&CookiePartitionKey<'a>> { self.partitionKey.as_ref() }
}

#[derive(Default)]
pub struct DeleteCookiesParamsBuilder<'a> {
    name: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    domain: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    partitionKey: Option<CookiePartitionKey<'a>>,
}

impl<'a> DeleteCookiesParamsBuilder<'a> {
    /// Name of the cookies to remove.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// If specified, deletes all the cookies with the given name where domain and path match
    /// provided URL.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// If specified, deletes only cookies with the exact domain.
    pub fn domain(mut self, domain: impl Into<Cow<'a, str>>) -> Self { self.domain = Some(domain.into()); self }
    /// If specified, deletes only cookies with the exact path.
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    /// If specified, deletes only cookies with the the given name and partitionKey where
    /// all partition key attributes match the cookie partition key attribute.
    pub fn partitionKey(mut self, partitionKey: CookiePartitionKey<'a>) -> Self { self.partitionKey = Some(partitionKey); self }
    pub fn build(self) -> DeleteCookiesParams<'a> {
        DeleteCookiesParams {
            name: self.name.unwrap_or_default(),
            url: self.url,
            domain: self.domain,
            path: self.path,
            partitionKey: self.partitionKey,
        }
    }
}

impl<'a> DeleteCookiesParams<'a> { pub const METHOD: &'static str = "Network.deleteCookies"; }

impl<'a> crate::CdpCommand<'a> for DeleteCookiesParams<'a> {
    const METHOD: &'static str = "Network.deleteCookies";
    type Response = crate::EmptyReturns;
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

impl DisableParams { pub const METHOD: &'static str = "Network.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Network.disable";
    type Response = crate::EmptyReturns;
}

/// Activates emulation of network conditions. This command is deprecated in favor of the emulateNetworkConditionsByRule
/// and overrideNetworkState commands, which can be used together to the same effect.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsParams {
    /// True to emulate internet disconnection.
    offline: bool,
    /// Minimum latency from request sent to response headers received (ms).
    latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    downloadThroughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    uploadThroughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    connectionType: Option<ConnectionType>,
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    #[serde(skip_serializing_if = "Option::is_none")]
    packetLoss: Option<f64>,
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    #[serde(skip_serializing_if = "Option::is_none")]
    packetQueueLength: Option<u64>,
    /// WebRTC packetReordering feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    packetReordering: Option<bool>,
}

impl EmulateNetworkConditionsParams {
    pub fn builder() -> EmulateNetworkConditionsParamsBuilder { EmulateNetworkConditionsParamsBuilder::default() }
    pub fn offline(&self) -> bool { self.offline }
    pub fn latency(&self) -> f64 { self.latency }
    pub fn downloadThroughput(&self) -> f64 { self.downloadThroughput }
    pub fn uploadThroughput(&self) -> f64 { self.uploadThroughput }
    pub fn connectionType(&self) -> Option<&ConnectionType> { self.connectionType.as_ref() }
    pub fn packetLoss(&self) -> Option<f64> { self.packetLoss }
    pub fn packetQueueLength(&self) -> Option<u64> { self.packetQueueLength }
    pub fn packetReordering(&self) -> Option<bool> { self.packetReordering }
}

#[derive(Default)]
pub struct EmulateNetworkConditionsParamsBuilder {
    offline: Option<bool>,
    latency: Option<f64>,
    downloadThroughput: Option<f64>,
    uploadThroughput: Option<f64>,
    connectionType: Option<ConnectionType>,
    packetLoss: Option<f64>,
    packetQueueLength: Option<u64>,
    packetReordering: Option<bool>,
}

impl EmulateNetworkConditionsParamsBuilder {
    /// True to emulate internet disconnection.
    pub fn offline(mut self, offline: bool) -> Self { self.offline = Some(offline); self }
    /// Minimum latency from request sent to response headers received (ms).
    pub fn latency(mut self, latency: f64) -> Self { self.latency = Some(latency); self }
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    pub fn downloadThroughput(mut self, downloadThroughput: f64) -> Self { self.downloadThroughput = Some(downloadThroughput); self }
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub fn uploadThroughput(mut self, uploadThroughput: f64) -> Self { self.uploadThroughput = Some(uploadThroughput); self }
    /// Connection type if known.
    pub fn connectionType(mut self, connectionType: ConnectionType) -> Self { self.connectionType = Some(connectionType); self }
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    pub fn packetLoss(mut self, packetLoss: f64) -> Self { self.packetLoss = Some(packetLoss); self }
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    pub fn packetQueueLength(mut self, packetQueueLength: u64) -> Self { self.packetQueueLength = Some(packetQueueLength); self }
    /// WebRTC packetReordering feature.
    pub fn packetReordering(mut self, packetReordering: bool) -> Self { self.packetReordering = Some(packetReordering); self }
    pub fn build(self) -> EmulateNetworkConditionsParams {
        EmulateNetworkConditionsParams {
            offline: self.offline.unwrap_or_default(),
            latency: self.latency.unwrap_or_default(),
            downloadThroughput: self.downloadThroughput.unwrap_or_default(),
            uploadThroughput: self.uploadThroughput.unwrap_or_default(),
            connectionType: self.connectionType,
            packetLoss: self.packetLoss,
            packetQueueLength: self.packetQueueLength,
            packetReordering: self.packetReordering,
        }
    }
}

impl EmulateNetworkConditionsParams { pub const METHOD: &'static str = "Network.emulateNetworkConditions"; }

impl<'a> crate::CdpCommand<'a> for EmulateNetworkConditionsParams {
    const METHOD: &'static str = "Network.emulateNetworkConditions";
    type Response = crate::EmptyReturns;
}

/// Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated
/// Network.emulateNetworkConditions this method does not affect 'navigator' state. Use Network.overrideNetworkState to
/// explicitly modify 'navigator' behavior.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsByRuleParams<'a> {
    /// True to emulate internet disconnection. Deprecated, use the offline property in matchedNetworkConditions
    /// or emulateOfflineServiceWorker instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    offline: Option<bool>,
    /// True to emulate offline service worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    emulateOfflineServiceWorker: Option<bool>,
    /// Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global
    /// conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are
    /// also applied for throttling of p2p connections.
    matchedNetworkConditions: Vec<NetworkConditions<'a>>,
}

impl<'a> EmulateNetworkConditionsByRuleParams<'a> {
    pub fn builder() -> EmulateNetworkConditionsByRuleParamsBuilder<'a> { EmulateNetworkConditionsByRuleParamsBuilder::default() }
    pub fn offline(&self) -> Option<bool> { self.offline }
    pub fn emulateOfflineServiceWorker(&self) -> Option<bool> { self.emulateOfflineServiceWorker }
    pub fn matchedNetworkConditions(&self) -> &[NetworkConditions<'a>] { &self.matchedNetworkConditions }
}

#[derive(Default)]
pub struct EmulateNetworkConditionsByRuleParamsBuilder<'a> {
    offline: Option<bool>,
    emulateOfflineServiceWorker: Option<bool>,
    matchedNetworkConditions: Option<Vec<NetworkConditions<'a>>>,
}

impl<'a> EmulateNetworkConditionsByRuleParamsBuilder<'a> {
    /// True to emulate internet disconnection. Deprecated, use the offline property in matchedNetworkConditions
    /// or emulateOfflineServiceWorker instead.
    pub fn offline(mut self, offline: bool) -> Self { self.offline = Some(offline); self }
    /// True to emulate offline service worker.
    pub fn emulateOfflineServiceWorker(mut self, emulateOfflineServiceWorker: bool) -> Self { self.emulateOfflineServiceWorker = Some(emulateOfflineServiceWorker); self }
    /// Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global
    /// conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are
    /// also applied for throttling of p2p connections.
    pub fn matchedNetworkConditions(mut self, matchedNetworkConditions: Vec<NetworkConditions<'a>>) -> Self { self.matchedNetworkConditions = Some(matchedNetworkConditions); self }
    pub fn build(self) -> EmulateNetworkConditionsByRuleParams<'a> {
        EmulateNetworkConditionsByRuleParams {
            offline: self.offline,
            emulateOfflineServiceWorker: self.emulateOfflineServiceWorker,
            matchedNetworkConditions: self.matchedNetworkConditions.unwrap_or_default(),
        }
    }
}

/// Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated
/// Network.emulateNetworkConditions this method does not affect 'navigator' state. Use Network.overrideNetworkState to
/// explicitly modify 'navigator' behavior.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsByRuleReturns<'a> {
    /// An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for
    /// requests affected by a rule.
    ruleIds: Vec<Cow<'a, str>>,
}

impl<'a> EmulateNetworkConditionsByRuleReturns<'a> {
    pub fn builder() -> EmulateNetworkConditionsByRuleReturnsBuilder<'a> { EmulateNetworkConditionsByRuleReturnsBuilder::default() }
    pub fn ruleIds(&self) -> &[Cow<'a, str>] { &self.ruleIds }
}

#[derive(Default)]
pub struct EmulateNetworkConditionsByRuleReturnsBuilder<'a> {
    ruleIds: Option<Vec<Cow<'a, str>>>,
}

impl<'a> EmulateNetworkConditionsByRuleReturnsBuilder<'a> {
    /// An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for
    /// requests affected by a rule.
    pub fn ruleIds(mut self, ruleIds: Vec<Cow<'a, str>>) -> Self { self.ruleIds = Some(ruleIds); self }
    pub fn build(self) -> EmulateNetworkConditionsByRuleReturns<'a> {
        EmulateNetworkConditionsByRuleReturns {
            ruleIds: self.ruleIds.unwrap_or_default(),
        }
    }
}

impl<'a> EmulateNetworkConditionsByRuleParams<'a> { pub const METHOD: &'static str = "Network.emulateNetworkConditionsByRule"; }

impl<'a> crate::CdpCommand<'a> for EmulateNetworkConditionsByRuleParams<'a> {
    const METHOD: &'static str = "Network.emulateNetworkConditionsByRule";
    type Response = EmulateNetworkConditionsByRuleReturns<'a>;
}

/// Override the state of navigator.onLine and navigator.connection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OverrideNetworkStateParams {
    /// True to emulate internet disconnection.
    offline: bool,
    /// Minimum latency from request sent to response headers received (ms).
    latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    downloadThroughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    uploadThroughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    connectionType: Option<ConnectionType>,
}

impl OverrideNetworkStateParams {
    pub fn builder() -> OverrideNetworkStateParamsBuilder { OverrideNetworkStateParamsBuilder::default() }
    pub fn offline(&self) -> bool { self.offline }
    pub fn latency(&self) -> f64 { self.latency }
    pub fn downloadThroughput(&self) -> f64 { self.downloadThroughput }
    pub fn uploadThroughput(&self) -> f64 { self.uploadThroughput }
    pub fn connectionType(&self) -> Option<&ConnectionType> { self.connectionType.as_ref() }
}

#[derive(Default)]
pub struct OverrideNetworkStateParamsBuilder {
    offline: Option<bool>,
    latency: Option<f64>,
    downloadThroughput: Option<f64>,
    uploadThroughput: Option<f64>,
    connectionType: Option<ConnectionType>,
}

impl OverrideNetworkStateParamsBuilder {
    /// True to emulate internet disconnection.
    pub fn offline(mut self, offline: bool) -> Self { self.offline = Some(offline); self }
    /// Minimum latency from request sent to response headers received (ms).
    pub fn latency(mut self, latency: f64) -> Self { self.latency = Some(latency); self }
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    pub fn downloadThroughput(mut self, downloadThroughput: f64) -> Self { self.downloadThroughput = Some(downloadThroughput); self }
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub fn uploadThroughput(mut self, uploadThroughput: f64) -> Self { self.uploadThroughput = Some(uploadThroughput); self }
    /// Connection type if known.
    pub fn connectionType(mut self, connectionType: ConnectionType) -> Self { self.connectionType = Some(connectionType); self }
    pub fn build(self) -> OverrideNetworkStateParams {
        OverrideNetworkStateParams {
            offline: self.offline.unwrap_or_default(),
            latency: self.latency.unwrap_or_default(),
            downloadThroughput: self.downloadThroughput.unwrap_or_default(),
            uploadThroughput: self.uploadThroughput.unwrap_or_default(),
            connectionType: self.connectionType,
        }
    }
}

impl OverrideNetworkStateParams { pub const METHOD: &'static str = "Network.overrideNetworkState"; }

impl<'a> crate::CdpCommand<'a> for OverrideNetworkStateParams {
    const METHOD: &'static str = "Network.overrideNetworkState";
    type Response = crate::EmptyReturns;
}

/// Enables network tracking, network events will now be delivered to the client.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    /// This is the maximum number of bytes that will be collected by this
    /// DevTools session.
    #[serde(skip_serializing_if = "Option::is_none")]
    maxTotalBufferSize: Option<u64>,
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    #[serde(skip_serializing_if = "Option::is_none")]
    maxResourceBufferSize: Option<u64>,
    /// Longest post body size (in bytes) that would be included in requestWillBeSent notification
    #[serde(skip_serializing_if = "Option::is_none")]
    maxPostDataSize: Option<u64>,
    /// Whether DirectSocket chunk send/receive events should be reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    reportDirectSocketTraffic: Option<bool>,
    /// Enable storing response bodies outside of renderer, so that these survive
    /// a cross-process navigation. Requires maxTotalBufferSize to be set.
    /// Currently defaults to false. This field is being deprecated in favor of the dedicated
    /// configureDurableMessages command, due to the possibility of deadlocks when awaiting
    /// Network.enable before issuing Runtime.runIfWaitingForDebugger.
    #[serde(skip_serializing_if = "Option::is_none")]
    enableDurableMessages: Option<bool>,
}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder { EnableParamsBuilder::default() }
    pub fn maxTotalBufferSize(&self) -> Option<u64> { self.maxTotalBufferSize }
    pub fn maxResourceBufferSize(&self) -> Option<u64> { self.maxResourceBufferSize }
    pub fn maxPostDataSize(&self) -> Option<u64> { self.maxPostDataSize }
    pub fn reportDirectSocketTraffic(&self) -> Option<bool> { self.reportDirectSocketTraffic }
    pub fn enableDurableMessages(&self) -> Option<bool> { self.enableDurableMessages }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    maxTotalBufferSize: Option<u64>,
    maxResourceBufferSize: Option<u64>,
    maxPostDataSize: Option<u64>,
    reportDirectSocketTraffic: Option<bool>,
    enableDurableMessages: Option<bool>,
}

impl EnableParamsBuilder {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    /// This is the maximum number of bytes that will be collected by this
    /// DevTools session.
    pub fn maxTotalBufferSize(mut self, maxTotalBufferSize: u64) -> Self { self.maxTotalBufferSize = Some(maxTotalBufferSize); self }
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub fn maxResourceBufferSize(mut self, maxResourceBufferSize: u64) -> Self { self.maxResourceBufferSize = Some(maxResourceBufferSize); self }
    /// Longest post body size (in bytes) that would be included in requestWillBeSent notification
    pub fn maxPostDataSize(mut self, maxPostDataSize: u64) -> Self { self.maxPostDataSize = Some(maxPostDataSize); self }
    /// Whether DirectSocket chunk send/receive events should be reported.
    pub fn reportDirectSocketTraffic(mut self, reportDirectSocketTraffic: bool) -> Self { self.reportDirectSocketTraffic = Some(reportDirectSocketTraffic); self }
    /// Enable storing response bodies outside of renderer, so that these survive
    /// a cross-process navigation. Requires maxTotalBufferSize to be set.
    /// Currently defaults to false. This field is being deprecated in favor of the dedicated
    /// configureDurableMessages command, due to the possibility of deadlocks when awaiting
    /// Network.enable before issuing Runtime.runIfWaitingForDebugger.
    pub fn enableDurableMessages(mut self, enableDurableMessages: bool) -> Self { self.enableDurableMessages = Some(enableDurableMessages); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            maxTotalBufferSize: self.maxTotalBufferSize,
            maxResourceBufferSize: self.maxResourceBufferSize,
            maxPostDataSize: self.maxPostDataSize,
            reportDirectSocketTraffic: self.reportDirectSocketTraffic,
            enableDurableMessages: self.enableDurableMessages,
        }
    }
}

impl EnableParams { pub const METHOD: &'static str = "Network.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Network.enable";
    type Response = crate::EmptyReturns;
}

/// Configures storing response bodies outside of renderer, so that these survive
/// a cross-process navigation.
/// If maxTotalBufferSize is not set, durable messages are disabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureDurableMessagesParams {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    #[serde(skip_serializing_if = "Option::is_none")]
    maxTotalBufferSize: Option<u64>,
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    #[serde(skip_serializing_if = "Option::is_none")]
    maxResourceBufferSize: Option<u64>,
}

impl ConfigureDurableMessagesParams {
    pub fn builder() -> ConfigureDurableMessagesParamsBuilder { ConfigureDurableMessagesParamsBuilder::default() }
    pub fn maxTotalBufferSize(&self) -> Option<u64> { self.maxTotalBufferSize }
    pub fn maxResourceBufferSize(&self) -> Option<u64> { self.maxResourceBufferSize }
}

#[derive(Default)]
pub struct ConfigureDurableMessagesParamsBuilder {
    maxTotalBufferSize: Option<u64>,
    maxResourceBufferSize: Option<u64>,
}

impl ConfigureDurableMessagesParamsBuilder {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub fn maxTotalBufferSize(mut self, maxTotalBufferSize: u64) -> Self { self.maxTotalBufferSize = Some(maxTotalBufferSize); self }
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub fn maxResourceBufferSize(mut self, maxResourceBufferSize: u64) -> Self { self.maxResourceBufferSize = Some(maxResourceBufferSize); self }
    pub fn build(self) -> ConfigureDurableMessagesParams {
        ConfigureDurableMessagesParams {
            maxTotalBufferSize: self.maxTotalBufferSize,
            maxResourceBufferSize: self.maxResourceBufferSize,
        }
    }
}

impl ConfigureDurableMessagesParams { pub const METHOD: &'static str = "Network.configureDurableMessages"; }

impl<'a> crate::CdpCommand<'a> for ConfigureDurableMessagesParams {
    const METHOD: &'static str = "Network.configureDurableMessages";
    type Response = crate::EmptyReturns;
}

/// Returns all browser cookies. Depending on the backend support, will return detailed cookie
/// information in the 'cookies' field.
/// Deprecated. Use Storage.getCookies instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllCookiesReturns<'a> {
    /// Array of cookie objects.
    cookies: Vec<Cookie<'a>>,
}

impl<'a> GetAllCookiesReturns<'a> {
    pub fn builder() -> GetAllCookiesReturnsBuilder<'a> { GetAllCookiesReturnsBuilder::default() }
    pub fn cookies(&self) -> &[Cookie<'a>] { &self.cookies }
}

#[derive(Default)]
pub struct GetAllCookiesReturnsBuilder<'a> {
    cookies: Option<Vec<Cookie<'a>>>,
}

impl<'a> GetAllCookiesReturnsBuilder<'a> {
    /// Array of cookie objects.
    pub fn cookies(mut self, cookies: Vec<Cookie<'a>>) -> Self { self.cookies = Some(cookies); self }
    pub fn build(self) -> GetAllCookiesReturns<'a> {
        GetAllCookiesReturns {
            cookies: self.cookies.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAllCookiesParams {}

impl GetAllCookiesParams {
    pub fn builder() -> GetAllCookiesParamsBuilder {
        GetAllCookiesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetAllCookiesParamsBuilder {}

impl GetAllCookiesParamsBuilder {
    pub fn build(self) -> GetAllCookiesParams {
        GetAllCookiesParams {}
    }
}

impl GetAllCookiesParams { pub const METHOD: &'static str = "Network.getAllCookies"; }

impl<'a> crate::CdpCommand<'a> for GetAllCookiesParams {
    const METHOD: &'static str = "Network.getAllCookies";
    type Response = GetAllCookiesReturns<'a>;
}

/// Returns the DER-encoded certificate.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateParams<'a> {
    /// Origin to get certificate for.
    origin: Cow<'a, str>,
}

impl<'a> GetCertificateParams<'a> {
    pub fn builder() -> GetCertificateParamsBuilder<'a> { GetCertificateParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}

#[derive(Default)]
pub struct GetCertificateParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
}

impl<'a> GetCertificateParamsBuilder<'a> {
    /// Origin to get certificate for.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn build(self) -> GetCertificateParams<'a> {
        GetCertificateParams {
            origin: self.origin.unwrap_or_default(),
        }
    }
}

/// Returns the DER-encoded certificate.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateReturns<'a> {
    tableNames: Vec<Cow<'a, str>>,
}

impl<'a> GetCertificateReturns<'a> {
    pub fn builder() -> GetCertificateReturnsBuilder<'a> { GetCertificateReturnsBuilder::default() }
    pub fn tableNames(&self) -> &[Cow<'a, str>] { &self.tableNames }
}

#[derive(Default)]
pub struct GetCertificateReturnsBuilder<'a> {
    tableNames: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GetCertificateReturnsBuilder<'a> {
    pub fn tableNames(mut self, tableNames: Vec<Cow<'a, str>>) -> Self { self.tableNames = Some(tableNames); self }
    pub fn build(self) -> GetCertificateReturns<'a> {
        GetCertificateReturns {
            tableNames: self.tableNames.unwrap_or_default(),
        }
    }
}

impl<'a> GetCertificateParams<'a> { pub const METHOD: &'static str = "Network.getCertificate"; }

impl<'a> crate::CdpCommand<'a> for GetCertificateParams<'a> {
    const METHOD: &'static str = "Network.getCertificate";
    type Response = GetCertificateReturns<'a>;
}

/// Returns all browser cookies for the current URL. Depending on the backend support, will return
/// detailed cookie information in the 'cookies' field.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesParams<'a> {
    /// The list of URLs for which applicable cookies will be fetched.
    /// If not specified, it's assumed to be set to the list containing
    /// the URLs of the page and all of its subframes.
    #[serde(skip_serializing_if = "Option::is_none")]
    urls: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GetCookiesParams<'a> {
    pub fn builder() -> GetCookiesParamsBuilder<'a> { GetCookiesParamsBuilder::default() }
    pub fn urls(&self) -> Option<&[Cow<'a, str>]> { self.urls.as_deref() }
}

#[derive(Default)]
pub struct GetCookiesParamsBuilder<'a> {
    urls: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GetCookiesParamsBuilder<'a> {
    /// The list of URLs for which applicable cookies will be fetched.
    /// If not specified, it's assumed to be set to the list containing
    /// the URLs of the page and all of its subframes.
    pub fn urls(mut self, urls: Vec<Cow<'a, str>>) -> Self { self.urls = Some(urls); self }
    pub fn build(self) -> GetCookiesParams<'a> {
        GetCookiesParams {
            urls: self.urls,
        }
    }
}

/// Returns all browser cookies for the current URL. Depending on the backend support, will return
/// detailed cookie information in the 'cookies' field.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesReturns<'a> {
    /// Array of cookie objects.
    cookies: Vec<Cookie<'a>>,
}

impl<'a> GetCookiesReturns<'a> {
    pub fn builder() -> GetCookiesReturnsBuilder<'a> { GetCookiesReturnsBuilder::default() }
    pub fn cookies(&self) -> &[Cookie<'a>] { &self.cookies }
}

#[derive(Default)]
pub struct GetCookiesReturnsBuilder<'a> {
    cookies: Option<Vec<Cookie<'a>>>,
}

impl<'a> GetCookiesReturnsBuilder<'a> {
    /// Array of cookie objects.
    pub fn cookies(mut self, cookies: Vec<Cookie<'a>>) -> Self { self.cookies = Some(cookies); self }
    pub fn build(self) -> GetCookiesReturns<'a> {
        GetCookiesReturns {
            cookies: self.cookies.unwrap_or_default(),
        }
    }
}

impl<'a> GetCookiesParams<'a> { pub const METHOD: &'static str = "Network.getCookies"; }

impl<'a> crate::CdpCommand<'a> for GetCookiesParams<'a> {
    const METHOD: &'static str = "Network.getCookies";
    type Response = GetCookiesReturns<'a>;
}

/// Returns content served for the given request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyParams<'a> {
    /// Identifier of the network request to get content for.
    requestId: RequestId<'a>,
}

impl<'a> GetResponseBodyParams<'a> {
    pub fn builder() -> GetResponseBodyParamsBuilder<'a> { GetResponseBodyParamsBuilder::default() }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
}

#[derive(Default)]
pub struct GetResponseBodyParamsBuilder<'a> {
    requestId: Option<RequestId<'a>>,
}

impl<'a> GetResponseBodyParamsBuilder<'a> {
    /// Identifier of the network request to get content for.
    pub fn requestId(mut self, requestId: RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    pub fn build(self) -> GetResponseBodyParams<'a> {
        GetResponseBodyParams {
            requestId: self.requestId.unwrap_or_default(),
        }
    }
}

/// Returns content served for the given request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyReturns<'a> {
    /// Response body.
    body: Cow<'a, str>,
    /// True, if content was sent as base64.
    base64Encoded: bool,
}

impl<'a> GetResponseBodyReturns<'a> {
    pub fn builder() -> GetResponseBodyReturnsBuilder<'a> { GetResponseBodyReturnsBuilder::default() }
    pub fn body(&self) -> &str { self.body.as_ref() }
    pub fn base64Encoded(&self) -> bool { self.base64Encoded }
}

#[derive(Default)]
pub struct GetResponseBodyReturnsBuilder<'a> {
    body: Option<Cow<'a, str>>,
    base64Encoded: Option<bool>,
}

impl<'a> GetResponseBodyReturnsBuilder<'a> {
    /// Response body.
    pub fn body(mut self, body: impl Into<Cow<'a, str>>) -> Self { self.body = Some(body.into()); self }
    /// True, if content was sent as base64.
    pub fn base64Encoded(mut self, base64Encoded: bool) -> Self { self.base64Encoded = Some(base64Encoded); self }
    pub fn build(self) -> GetResponseBodyReturns<'a> {
        GetResponseBodyReturns {
            body: self.body.unwrap_or_default(),
            base64Encoded: self.base64Encoded.unwrap_or_default(),
        }
    }
}

impl<'a> GetResponseBodyParams<'a> { pub const METHOD: &'static str = "Network.getResponseBody"; }

impl<'a> crate::CdpCommand<'a> for GetResponseBodyParams<'a> {
    const METHOD: &'static str = "Network.getResponseBody";
    type Response = GetResponseBodyReturns<'a>;
}

/// Returns post data sent with the request. Returns an error when no data was sent with the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRequestPostDataParams<'a> {
    /// Identifier of the network request to get content for.
    requestId: RequestId<'a>,
}

impl<'a> GetRequestPostDataParams<'a> {
    pub fn builder() -> GetRequestPostDataParamsBuilder<'a> { GetRequestPostDataParamsBuilder::default() }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
}

#[derive(Default)]
pub struct GetRequestPostDataParamsBuilder<'a> {
    requestId: Option<RequestId<'a>>,
}

impl<'a> GetRequestPostDataParamsBuilder<'a> {
    /// Identifier of the network request to get content for.
    pub fn requestId(mut self, requestId: RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    pub fn build(self) -> GetRequestPostDataParams<'a> {
        GetRequestPostDataParams {
            requestId: self.requestId.unwrap_or_default(),
        }
    }
}

/// Returns post data sent with the request. Returns an error when no data was sent with the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRequestPostDataReturns<'a> {
    /// Request body string, omitting files from multipart requests
    postData: Cow<'a, str>,
    /// True, if content was sent as base64.
    base64Encoded: bool,
}

impl<'a> GetRequestPostDataReturns<'a> {
    pub fn builder() -> GetRequestPostDataReturnsBuilder<'a> { GetRequestPostDataReturnsBuilder::default() }
    pub fn postData(&self) -> &str { self.postData.as_ref() }
    pub fn base64Encoded(&self) -> bool { self.base64Encoded }
}

#[derive(Default)]
pub struct GetRequestPostDataReturnsBuilder<'a> {
    postData: Option<Cow<'a, str>>,
    base64Encoded: Option<bool>,
}

impl<'a> GetRequestPostDataReturnsBuilder<'a> {
    /// Request body string, omitting files from multipart requests
    pub fn postData(mut self, postData: impl Into<Cow<'a, str>>) -> Self { self.postData = Some(postData.into()); self }
    /// True, if content was sent as base64.
    pub fn base64Encoded(mut self, base64Encoded: bool) -> Self { self.base64Encoded = Some(base64Encoded); self }
    pub fn build(self) -> GetRequestPostDataReturns<'a> {
        GetRequestPostDataReturns {
            postData: self.postData.unwrap_or_default(),
            base64Encoded: self.base64Encoded.unwrap_or_default(),
        }
    }
}

impl<'a> GetRequestPostDataParams<'a> { pub const METHOD: &'static str = "Network.getRequestPostData"; }

impl<'a> crate::CdpCommand<'a> for GetRequestPostDataParams<'a> {
    const METHOD: &'static str = "Network.getRequestPostData";
    type Response = GetRequestPostDataReturns<'a>;
}

/// Returns content served for the given currently intercepted request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyForInterceptionParams<'a> {
    /// Identifier for the intercepted request to get body for.
    interceptionId: InterceptionId<'a>,
}

impl<'a> GetResponseBodyForInterceptionParams<'a> {
    pub fn builder() -> GetResponseBodyForInterceptionParamsBuilder<'a> { GetResponseBodyForInterceptionParamsBuilder::default() }
    pub fn interceptionId(&self) -> &InterceptionId<'a> { &self.interceptionId }
}

#[derive(Default)]
pub struct GetResponseBodyForInterceptionParamsBuilder<'a> {
    interceptionId: Option<InterceptionId<'a>>,
}

impl<'a> GetResponseBodyForInterceptionParamsBuilder<'a> {
    /// Identifier for the intercepted request to get body for.
    pub fn interceptionId(mut self, interceptionId: InterceptionId<'a>) -> Self { self.interceptionId = Some(interceptionId); self }
    pub fn build(self) -> GetResponseBodyForInterceptionParams<'a> {
        GetResponseBodyForInterceptionParams {
            interceptionId: self.interceptionId.unwrap_or_default(),
        }
    }
}

/// Returns content served for the given currently intercepted request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyForInterceptionReturns<'a> {
    /// Response body.
    body: Cow<'a, str>,
    /// True, if content was sent as base64.
    base64Encoded: bool,
}

impl<'a> GetResponseBodyForInterceptionReturns<'a> {
    pub fn builder() -> GetResponseBodyForInterceptionReturnsBuilder<'a> { GetResponseBodyForInterceptionReturnsBuilder::default() }
    pub fn body(&self) -> &str { self.body.as_ref() }
    pub fn base64Encoded(&self) -> bool { self.base64Encoded }
}

#[derive(Default)]
pub struct GetResponseBodyForInterceptionReturnsBuilder<'a> {
    body: Option<Cow<'a, str>>,
    base64Encoded: Option<bool>,
}

impl<'a> GetResponseBodyForInterceptionReturnsBuilder<'a> {
    /// Response body.
    pub fn body(mut self, body: impl Into<Cow<'a, str>>) -> Self { self.body = Some(body.into()); self }
    /// True, if content was sent as base64.
    pub fn base64Encoded(mut self, base64Encoded: bool) -> Self { self.base64Encoded = Some(base64Encoded); self }
    pub fn build(self) -> GetResponseBodyForInterceptionReturns<'a> {
        GetResponseBodyForInterceptionReturns {
            body: self.body.unwrap_or_default(),
            base64Encoded: self.base64Encoded.unwrap_or_default(),
        }
    }
}

impl<'a> GetResponseBodyForInterceptionParams<'a> { pub const METHOD: &'static str = "Network.getResponseBodyForInterception"; }

impl<'a> crate::CdpCommand<'a> for GetResponseBodyForInterceptionParams<'a> {
    const METHOD: &'static str = "Network.getResponseBodyForInterception";
    type Response = GetResponseBodyForInterceptionReturns<'a>;
}

/// Returns a handle to the stream representing the response body. Note that after this command,
/// the intercepted request can't be continued as is -- you either need to cancel it or to provide
/// the response body. The stream only supports sequential read, IO.read will fail if the position
/// is specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeResponseBodyForInterceptionAsStreamParams<'a> {
    interceptionId: InterceptionId<'a>,
}

impl<'a> TakeResponseBodyForInterceptionAsStreamParams<'a> {
    pub fn builder() -> TakeResponseBodyForInterceptionAsStreamParamsBuilder<'a> { TakeResponseBodyForInterceptionAsStreamParamsBuilder::default() }
    pub fn interceptionId(&self) -> &InterceptionId<'a> { &self.interceptionId }
}

#[derive(Default)]
pub struct TakeResponseBodyForInterceptionAsStreamParamsBuilder<'a> {
    interceptionId: Option<InterceptionId<'a>>,
}

impl<'a> TakeResponseBodyForInterceptionAsStreamParamsBuilder<'a> {
    pub fn interceptionId(mut self, interceptionId: InterceptionId<'a>) -> Self { self.interceptionId = Some(interceptionId); self }
    pub fn build(self) -> TakeResponseBodyForInterceptionAsStreamParams<'a> {
        TakeResponseBodyForInterceptionAsStreamParams {
            interceptionId: self.interceptionId.unwrap_or_default(),
        }
    }
}

/// Returns a handle to the stream representing the response body. Note that after this command,
/// the intercepted request can't be continued as is -- you either need to cancel it or to provide
/// the response body. The stream only supports sequential read, IO.read will fail if the position
/// is specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeResponseBodyForInterceptionAsStreamReturns<'a> {
    stream: crate::io::StreamHandle<'a>,
}

impl<'a> TakeResponseBodyForInterceptionAsStreamReturns<'a> {
    pub fn builder() -> TakeResponseBodyForInterceptionAsStreamReturnsBuilder<'a> { TakeResponseBodyForInterceptionAsStreamReturnsBuilder::default() }
    pub fn stream(&self) -> &crate::io::StreamHandle<'a> { &self.stream }
}

#[derive(Default)]
pub struct TakeResponseBodyForInterceptionAsStreamReturnsBuilder<'a> {
    stream: Option<crate::io::StreamHandle<'a>>,
}

impl<'a> TakeResponseBodyForInterceptionAsStreamReturnsBuilder<'a> {
    pub fn stream(mut self, stream: crate::io::StreamHandle<'a>) -> Self { self.stream = Some(stream); self }
    pub fn build(self) -> TakeResponseBodyForInterceptionAsStreamReturns<'a> {
        TakeResponseBodyForInterceptionAsStreamReturns {
            stream: self.stream.unwrap_or_default(),
        }
    }
}

impl<'a> TakeResponseBodyForInterceptionAsStreamParams<'a> { pub const METHOD: &'static str = "Network.takeResponseBodyForInterceptionAsStream"; }

impl<'a> crate::CdpCommand<'a> for TakeResponseBodyForInterceptionAsStreamParams<'a> {
    const METHOD: &'static str = "Network.takeResponseBodyForInterceptionAsStream";
    type Response = TakeResponseBodyForInterceptionAsStreamReturns<'a>;
}

/// This method sends a new XMLHttpRequest which is identical to the original one. The following
/// parameters should be identical: method, url, async, request body, extra headers, withCredentials
/// attribute, user, password.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplayXHRParams<'a> {
    /// Identifier of XHR to replay.
    requestId: RequestId<'a>,
}

impl<'a> ReplayXHRParams<'a> {
    pub fn builder() -> ReplayXHRParamsBuilder<'a> { ReplayXHRParamsBuilder::default() }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
}

#[derive(Default)]
pub struct ReplayXHRParamsBuilder<'a> {
    requestId: Option<RequestId<'a>>,
}

impl<'a> ReplayXHRParamsBuilder<'a> {
    /// Identifier of XHR to replay.
    pub fn requestId(mut self, requestId: RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    pub fn build(self) -> ReplayXHRParams<'a> {
        ReplayXHRParams {
            requestId: self.requestId.unwrap_or_default(),
        }
    }
}

impl<'a> ReplayXHRParams<'a> { pub const METHOD: &'static str = "Network.replayXHR"; }

impl<'a> crate::CdpCommand<'a> for ReplayXHRParams<'a> {
    const METHOD: &'static str = "Network.replayXHR";
    type Response = crate::EmptyReturns;
}

/// Searches for given string in response content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResponseBodyParams<'a> {
    /// Identifier of the network response to search.
    requestId: RequestId<'a>,
    /// String to search for.
    query: Cow<'a, str>,
    /// If true, search is case sensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    caseSensitive: Option<bool>,
    /// If true, treats string parameter as regex.
    #[serde(skip_serializing_if = "Option::is_none")]
    isRegex: Option<bool>,
}

impl<'a> SearchInResponseBodyParams<'a> {
    pub fn builder() -> SearchInResponseBodyParamsBuilder<'a> { SearchInResponseBodyParamsBuilder::default() }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
    pub fn query(&self) -> &str { self.query.as_ref() }
    pub fn caseSensitive(&self) -> Option<bool> { self.caseSensitive }
    pub fn isRegex(&self) -> Option<bool> { self.isRegex }
}

#[derive(Default)]
pub struct SearchInResponseBodyParamsBuilder<'a> {
    requestId: Option<RequestId<'a>>,
    query: Option<Cow<'a, str>>,
    caseSensitive: Option<bool>,
    isRegex: Option<bool>,
}

impl<'a> SearchInResponseBodyParamsBuilder<'a> {
    /// Identifier of the network response to search.
    pub fn requestId(mut self, requestId: RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    /// String to search for.
    pub fn query(mut self, query: impl Into<Cow<'a, str>>) -> Self { self.query = Some(query.into()); self }
    /// If true, search is case sensitive.
    pub fn caseSensitive(mut self, caseSensitive: bool) -> Self { self.caseSensitive = Some(caseSensitive); self }
    /// If true, treats string parameter as regex.
    pub fn isRegex(mut self, isRegex: bool) -> Self { self.isRegex = Some(isRegex); self }
    pub fn build(self) -> SearchInResponseBodyParams<'a> {
        SearchInResponseBodyParams {
            requestId: self.requestId.unwrap_or_default(),
            query: self.query.unwrap_or_default(),
            caseSensitive: self.caseSensitive,
            isRegex: self.isRegex,
        }
    }
}

/// Searches for given string in response content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResponseBodyReturns {
    /// List of search matches.
    result: Vec<crate::debugger::SearchMatch>,
}

impl SearchInResponseBodyReturns {
    pub fn builder() -> SearchInResponseBodyReturnsBuilder { SearchInResponseBodyReturnsBuilder::default() }
    pub fn result(&self) -> &[crate::debugger::SearchMatch] { &self.result }
}

#[derive(Default)]
pub struct SearchInResponseBodyReturnsBuilder {
    result: Option<Vec<crate::debugger::SearchMatch>>,
}

impl SearchInResponseBodyReturnsBuilder {
    /// List of search matches.
    pub fn result(mut self, result: Vec<crate::debugger::SearchMatch>) -> Self { self.result = Some(result); self }
    pub fn build(self) -> SearchInResponseBodyReturns {
        SearchInResponseBodyReturns {
            result: self.result.unwrap_or_default(),
        }
    }
}

impl<'a> SearchInResponseBodyParams<'a> { pub const METHOD: &'static str = "Network.searchInResponseBody"; }

impl<'a> crate::CdpCommand<'a> for SearchInResponseBodyParams<'a> {
    const METHOD: &'static str = "Network.searchInResponseBody";
    type Response = SearchInResponseBodyReturns;
}

/// Blocks URLs from loading.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBlockedURLsParams<'a> {
    /// Patterns to match in the order in which they are given. These patterns
    /// also take precedence over any wildcard patterns defined in 'urls'.
    #[serde(skip_serializing_if = "Option::is_none")]
    urlPatterns: Option<Vec<BlockPattern<'a>>>,
    /// URL patterns to block. Wildcards ('*') are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    urls: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SetBlockedURLsParams<'a> {
    pub fn builder() -> SetBlockedURLsParamsBuilder<'a> { SetBlockedURLsParamsBuilder::default() }
    pub fn urlPatterns(&self) -> Option<&[BlockPattern<'a>]> { self.urlPatterns.as_deref() }
    pub fn urls(&self) -> Option<&[Cow<'a, str>]> { self.urls.as_deref() }
}

#[derive(Default)]
pub struct SetBlockedURLsParamsBuilder<'a> {
    urlPatterns: Option<Vec<BlockPattern<'a>>>,
    urls: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SetBlockedURLsParamsBuilder<'a> {
    /// Patterns to match in the order in which they are given. These patterns
    /// also take precedence over any wildcard patterns defined in 'urls'.
    pub fn urlPatterns(mut self, urlPatterns: Vec<BlockPattern<'a>>) -> Self { self.urlPatterns = Some(urlPatterns); self }
    /// URL patterns to block. Wildcards ('*') are allowed.
    pub fn urls(mut self, urls: Vec<Cow<'a, str>>) -> Self { self.urls = Some(urls); self }
    pub fn build(self) -> SetBlockedURLsParams<'a> {
        SetBlockedURLsParams {
            urlPatterns: self.urlPatterns,
            urls: self.urls,
        }
    }
}

impl<'a> SetBlockedURLsParams<'a> { pub const METHOD: &'static str = "Network.setBlockedURLs"; }

impl<'a> crate::CdpCommand<'a> for SetBlockedURLsParams<'a> {
    const METHOD: &'static str = "Network.setBlockedURLs";
    type Response = crate::EmptyReturns;
}

/// Toggles ignoring of service worker for each request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBypassServiceWorkerParams {
    /// Bypass service worker and load from network.
    bypass: bool,
}

impl SetBypassServiceWorkerParams {
    pub fn builder() -> SetBypassServiceWorkerParamsBuilder { SetBypassServiceWorkerParamsBuilder::default() }
    pub fn bypass(&self) -> bool { self.bypass }
}

#[derive(Default)]
pub struct SetBypassServiceWorkerParamsBuilder {
    bypass: Option<bool>,
}

impl SetBypassServiceWorkerParamsBuilder {
    /// Bypass service worker and load from network.
    pub fn bypass(mut self, bypass: bool) -> Self { self.bypass = Some(bypass); self }
    pub fn build(self) -> SetBypassServiceWorkerParams {
        SetBypassServiceWorkerParams {
            bypass: self.bypass.unwrap_or_default(),
        }
    }
}

impl SetBypassServiceWorkerParams { pub const METHOD: &'static str = "Network.setBypassServiceWorker"; }

impl<'a> crate::CdpCommand<'a> for SetBypassServiceWorkerParams {
    const METHOD: &'static str = "Network.setBypassServiceWorker";
    type Response = crate::EmptyReturns;
}

/// Toggles ignoring cache for each request. If 'true', cache will not be used.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCacheDisabledParams {
    /// Cache disabled state.
    cacheDisabled: bool,
}

impl SetCacheDisabledParams {
    pub fn builder() -> SetCacheDisabledParamsBuilder { SetCacheDisabledParamsBuilder::default() }
    pub fn cacheDisabled(&self) -> bool { self.cacheDisabled }
}

#[derive(Default)]
pub struct SetCacheDisabledParamsBuilder {
    cacheDisabled: Option<bool>,
}

impl SetCacheDisabledParamsBuilder {
    /// Cache disabled state.
    pub fn cacheDisabled(mut self, cacheDisabled: bool) -> Self { self.cacheDisabled = Some(cacheDisabled); self }
    pub fn build(self) -> SetCacheDisabledParams {
        SetCacheDisabledParams {
            cacheDisabled: self.cacheDisabled.unwrap_or_default(),
        }
    }
}

impl SetCacheDisabledParams { pub const METHOD: &'static str = "Network.setCacheDisabled"; }

impl<'a> crate::CdpCommand<'a> for SetCacheDisabledParams {
    const METHOD: &'static str = "Network.setCacheDisabled";
    type Response = crate::EmptyReturns;
}

/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieParams<'a> {
    /// Cookie name.
    name: Cow<'a, str>,
    /// Cookie value.
    value: Cow<'a, str>,
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// Cookie domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<Cow<'a, str>>,
    /// Cookie path.
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<Cow<'a, str>>,
    /// True if cookie is secure.
    #[serde(skip_serializing_if = "Option::is_none")]
    secure: Option<bool>,
    /// True if cookie is http-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    httpOnly: Option<bool>,
    /// Cookie SameSite type.
    #[serde(skip_serializing_if = "Option::is_none")]
    sameSite: Option<CookieSameSite>,
    /// Cookie expiration date, session cookie if not set
    #[serde(skip_serializing_if = "Option::is_none")]
    expires: Option<TimeSinceEpoch>,
    /// Cookie Priority type.
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<CookiePriority>,
    /// Cookie source scheme type.
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceScheme: Option<CookieSourceScheme>,
    /// Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    sourcePort: Option<i64>,
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    partitionKey: Option<CookiePartitionKey<'a>>,
}

impl<'a> SetCookieParams<'a> {
    pub fn builder() -> SetCookieParamsBuilder<'a> { SetCookieParamsBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn domain(&self) -> Option<&str> { self.domain.as_deref() }
    pub fn path(&self) -> Option<&str> { self.path.as_deref() }
    pub fn secure(&self) -> Option<bool> { self.secure }
    pub fn httpOnly(&self) -> Option<bool> { self.httpOnly }
    pub fn sameSite(&self) -> Option<&CookieSameSite> { self.sameSite.as_ref() }
    pub fn expires(&self) -> Option<&TimeSinceEpoch> { self.expires.as_ref() }
    pub fn priority(&self) -> Option<&CookiePriority> { self.priority.as_ref() }
    pub fn sourceScheme(&self) -> Option<&CookieSourceScheme> { self.sourceScheme.as_ref() }
    pub fn sourcePort(&self) -> Option<i64> { self.sourcePort }
    pub fn partitionKey(&self) -> Option<&CookiePartitionKey<'a>> { self.partitionKey.as_ref() }
}

#[derive(Default)]
pub struct SetCookieParamsBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    domain: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    secure: Option<bool>,
    httpOnly: Option<bool>,
    sameSite: Option<CookieSameSite>,
    expires: Option<TimeSinceEpoch>,
    priority: Option<CookiePriority>,
    sourceScheme: Option<CookieSourceScheme>,
    sourcePort: Option<i64>,
    partitionKey: Option<CookiePartitionKey<'a>>,
}

impl<'a> SetCookieParamsBuilder<'a> {
    /// Cookie name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Cookie value.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Cookie domain.
    pub fn domain(mut self, domain: impl Into<Cow<'a, str>>) -> Self { self.domain = Some(domain.into()); self }
    /// Cookie path.
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    /// True if cookie is secure.
    pub fn secure(mut self, secure: bool) -> Self { self.secure = Some(secure); self }
    /// True if cookie is http-only.
    pub fn httpOnly(mut self, httpOnly: bool) -> Self { self.httpOnly = Some(httpOnly); self }
    /// Cookie SameSite type.
    pub fn sameSite(mut self, sameSite: CookieSameSite) -> Self { self.sameSite = Some(sameSite); self }
    /// Cookie expiration date, session cookie if not set
    pub fn expires(mut self, expires: TimeSinceEpoch) -> Self { self.expires = Some(expires); self }
    /// Cookie Priority type.
    pub fn priority(mut self, priority: CookiePriority) -> Self { self.priority = Some(priority); self }
    /// Cookie source scheme type.
    pub fn sourceScheme(mut self, sourceScheme: CookieSourceScheme) -> Self { self.sourceScheme = Some(sourceScheme); self }
    /// Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    pub fn sourcePort(mut self, sourcePort: i64) -> Self { self.sourcePort = Some(sourcePort); self }
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    pub fn partitionKey(mut self, partitionKey: CookiePartitionKey<'a>) -> Self { self.partitionKey = Some(partitionKey); self }
    pub fn build(self) -> SetCookieParams<'a> {
        SetCookieParams {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            url: self.url,
            domain: self.domain,
            path: self.path,
            secure: self.secure,
            httpOnly: self.httpOnly,
            sameSite: self.sameSite,
            expires: self.expires,
            priority: self.priority,
            sourceScheme: self.sourceScheme,
            sourcePort: self.sourcePort,
            partitionKey: self.partitionKey,
        }
    }
}

/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieReturns {
    /// Always set to true. If an error occurs, the response indicates protocol error.
    success: bool,
}

impl SetCookieReturns {
    pub fn builder() -> SetCookieReturnsBuilder { SetCookieReturnsBuilder::default() }
    pub fn success(&self) -> bool { self.success }
}

#[derive(Default)]
pub struct SetCookieReturnsBuilder {
    success: Option<bool>,
}

impl SetCookieReturnsBuilder {
    /// Always set to true. If an error occurs, the response indicates protocol error.
    pub fn success(mut self, success: bool) -> Self { self.success = Some(success); self }
    pub fn build(self) -> SetCookieReturns {
        SetCookieReturns {
            success: self.success.unwrap_or_default(),
        }
    }
}

impl<'a> SetCookieParams<'a> { pub const METHOD: &'static str = "Network.setCookie"; }

impl<'a> crate::CdpCommand<'a> for SetCookieParams<'a> {
    const METHOD: &'static str = "Network.setCookie";
    type Response = SetCookieReturns;
}

/// Sets given cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookiesParams<'a> {
    /// Cookies to be set.
    cookies: Vec<CookieParam<'a>>,
}

impl<'a> SetCookiesParams<'a> {
    pub fn builder() -> SetCookiesParamsBuilder<'a> { SetCookiesParamsBuilder::default() }
    pub fn cookies(&self) -> &[CookieParam<'a>] { &self.cookies }
}

#[derive(Default)]
pub struct SetCookiesParamsBuilder<'a> {
    cookies: Option<Vec<CookieParam<'a>>>,
}

impl<'a> SetCookiesParamsBuilder<'a> {
    /// Cookies to be set.
    pub fn cookies(mut self, cookies: Vec<CookieParam<'a>>) -> Self { self.cookies = Some(cookies); self }
    pub fn build(self) -> SetCookiesParams<'a> {
        SetCookiesParams {
            cookies: self.cookies.unwrap_or_default(),
        }
    }
}

impl<'a> SetCookiesParams<'a> { pub const METHOD: &'static str = "Network.setCookies"; }

impl<'a> crate::CdpCommand<'a> for SetCookiesParams<'a> {
    const METHOD: &'static str = "Network.setCookies";
    type Response = crate::EmptyReturns;
}

/// Specifies whether to always send extra HTTP headers with the requests from this page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetExtraHTTPHeadersParams {
    /// Map with extra HTTP headers.
    headers: Headers,
}

impl SetExtraHTTPHeadersParams {
    pub fn builder() -> SetExtraHTTPHeadersParamsBuilder { SetExtraHTTPHeadersParamsBuilder::default() }
    pub fn headers(&self) -> &Headers { &self.headers }
}

#[derive(Default)]
pub struct SetExtraHTTPHeadersParamsBuilder {
    headers: Option<Headers>,
}

impl SetExtraHTTPHeadersParamsBuilder {
    /// Map with extra HTTP headers.
    pub fn headers(mut self, headers: Headers) -> Self { self.headers = Some(headers); self }
    pub fn build(self) -> SetExtraHTTPHeadersParams {
        SetExtraHTTPHeadersParams {
            headers: self.headers.unwrap_or_default(),
        }
    }
}

impl SetExtraHTTPHeadersParams { pub const METHOD: &'static str = "Network.setExtraHTTPHeaders"; }

impl<'a> crate::CdpCommand<'a> for SetExtraHTTPHeadersParams {
    const METHOD: &'static str = "Network.setExtraHTTPHeaders";
    type Response = crate::EmptyReturns;
}

/// Specifies whether to attach a page script stack id in requests

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAttachDebugStackParams {
    /// Whether to attach a page script stack for debugging purpose.
    enabled: bool,
}

impl SetAttachDebugStackParams {
    pub fn builder() -> SetAttachDebugStackParamsBuilder { SetAttachDebugStackParamsBuilder::default() }
    pub fn enabled(&self) -> bool { self.enabled }
}

#[derive(Default)]
pub struct SetAttachDebugStackParamsBuilder {
    enabled: Option<bool>,
}

impl SetAttachDebugStackParamsBuilder {
    /// Whether to attach a page script stack for debugging purpose.
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    pub fn build(self) -> SetAttachDebugStackParams {
        SetAttachDebugStackParams {
            enabled: self.enabled.unwrap_or_default(),
        }
    }
}

impl SetAttachDebugStackParams { pub const METHOD: &'static str = "Network.setAttachDebugStack"; }

impl<'a> crate::CdpCommand<'a> for SetAttachDebugStackParams {
    const METHOD: &'static str = "Network.setAttachDebugStack";
    type Response = crate::EmptyReturns;
}

/// Sets the requests to intercept that match the provided patterns and optionally resource types.
/// Deprecated, please use Fetch.enable instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRequestInterceptionParams<'a> {
    /// Requests matching any of these patterns will be forwarded and wait for the corresponding
    /// continueInterceptedRequest call.
    patterns: Vec<RequestPattern<'a>>,
}

impl<'a> SetRequestInterceptionParams<'a> {
    pub fn builder() -> SetRequestInterceptionParamsBuilder<'a> { SetRequestInterceptionParamsBuilder::default() }
    pub fn patterns(&self) -> &[RequestPattern<'a>] { &self.patterns }
}

#[derive(Default)]
pub struct SetRequestInterceptionParamsBuilder<'a> {
    patterns: Option<Vec<RequestPattern<'a>>>,
}

impl<'a> SetRequestInterceptionParamsBuilder<'a> {
    /// Requests matching any of these patterns will be forwarded and wait for the corresponding
    /// continueInterceptedRequest call.
    pub fn patterns(mut self, patterns: Vec<RequestPattern<'a>>) -> Self { self.patterns = Some(patterns); self }
    pub fn build(self) -> SetRequestInterceptionParams<'a> {
        SetRequestInterceptionParams {
            patterns: self.patterns.unwrap_or_default(),
        }
    }
}

impl<'a> SetRequestInterceptionParams<'a> { pub const METHOD: &'static str = "Network.setRequestInterception"; }

impl<'a> crate::CdpCommand<'a> for SetRequestInterceptionParams<'a> {
    const METHOD: &'static str = "Network.setRequestInterception";
    type Response = crate::EmptyReturns;
}

/// Allows overriding user agent with the given string.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAgentOverrideParams<'a> {
    /// User agent to use.
    userAgent: Cow<'a, str>,
    /// Browser language to emulate.
    #[serde(skip_serializing_if = "Option::is_none")]
    acceptLanguage: Option<Cow<'a, str>>,
    /// The platform navigator.platform should return.
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<Cow<'a, str>>,
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    #[serde(skip_serializing_if = "Option::is_none")]
    userAgentMetadata: Option<crate::emulation::UserAgentMetadata<'a>>,
}

impl<'a> SetUserAgentOverrideParams<'a> {
    pub fn builder() -> SetUserAgentOverrideParamsBuilder<'a> { SetUserAgentOverrideParamsBuilder::default() }
    pub fn userAgent(&self) -> &str { self.userAgent.as_ref() }
    pub fn acceptLanguage(&self) -> Option<&str> { self.acceptLanguage.as_deref() }
    pub fn platform(&self) -> Option<&str> { self.platform.as_deref() }
    pub fn userAgentMetadata(&self) -> Option<&crate::emulation::UserAgentMetadata<'a>> { self.userAgentMetadata.as_ref() }
}

#[derive(Default)]
pub struct SetUserAgentOverrideParamsBuilder<'a> {
    userAgent: Option<Cow<'a, str>>,
    acceptLanguage: Option<Cow<'a, str>>,
    platform: Option<Cow<'a, str>>,
    userAgentMetadata: Option<crate::emulation::UserAgentMetadata<'a>>,
}

impl<'a> SetUserAgentOverrideParamsBuilder<'a> {
    /// User agent to use.
    pub fn userAgent(mut self, userAgent: impl Into<Cow<'a, str>>) -> Self { self.userAgent = Some(userAgent.into()); self }
    /// Browser language to emulate.
    pub fn acceptLanguage(mut self, acceptLanguage: impl Into<Cow<'a, str>>) -> Self { self.acceptLanguage = Some(acceptLanguage.into()); self }
    /// The platform navigator.platform should return.
    pub fn platform(mut self, platform: impl Into<Cow<'a, str>>) -> Self { self.platform = Some(platform.into()); self }
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    pub fn userAgentMetadata(mut self, userAgentMetadata: crate::emulation::UserAgentMetadata<'a>) -> Self { self.userAgentMetadata = Some(userAgentMetadata); self }
    pub fn build(self) -> SetUserAgentOverrideParams<'a> {
        SetUserAgentOverrideParams {
            userAgent: self.userAgent.unwrap_or_default(),
            acceptLanguage: self.acceptLanguage,
            platform: self.platform,
            userAgentMetadata: self.userAgentMetadata,
        }
    }
}

impl<'a> SetUserAgentOverrideParams<'a> { pub const METHOD: &'static str = "Network.setUserAgentOverride"; }

impl<'a> crate::CdpCommand<'a> for SetUserAgentOverrideParams<'a> {
    const METHOD: &'static str = "Network.setUserAgentOverride";
    type Response = crate::EmptyReturns;
}

/// Enables streaming of the response for the given requestId.
/// If enabled, the dataReceived event contains the data that was received during streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StreamResourceContentParams<'a> {
    /// Identifier of the request to stream.
    requestId: RequestId<'a>,
}

impl<'a> StreamResourceContentParams<'a> {
    pub fn builder() -> StreamResourceContentParamsBuilder<'a> { StreamResourceContentParamsBuilder::default() }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
}

#[derive(Default)]
pub struct StreamResourceContentParamsBuilder<'a> {
    requestId: Option<RequestId<'a>>,
}

impl<'a> StreamResourceContentParamsBuilder<'a> {
    /// Identifier of the request to stream.
    pub fn requestId(mut self, requestId: RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    pub fn build(self) -> StreamResourceContentParams<'a> {
        StreamResourceContentParams {
            requestId: self.requestId.unwrap_or_default(),
        }
    }
}

/// Enables streaming of the response for the given requestId.
/// If enabled, the dataReceived event contains the data that was received during streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StreamResourceContentReturns<'a> {
    /// Data that has been buffered until streaming is enabled. (Encoded as a base64 string when passed over JSON)
    bufferedData: Cow<'a, str>,
}

impl<'a> StreamResourceContentReturns<'a> {
    pub fn builder() -> StreamResourceContentReturnsBuilder<'a> { StreamResourceContentReturnsBuilder::default() }
    pub fn bufferedData(&self) -> &str { self.bufferedData.as_ref() }
}

#[derive(Default)]
pub struct StreamResourceContentReturnsBuilder<'a> {
    bufferedData: Option<Cow<'a, str>>,
}

impl<'a> StreamResourceContentReturnsBuilder<'a> {
    /// Data that has been buffered until streaming is enabled. (Encoded as a base64 string when passed over JSON)
    pub fn bufferedData(mut self, bufferedData: impl Into<Cow<'a, str>>) -> Self { self.bufferedData = Some(bufferedData.into()); self }
    pub fn build(self) -> StreamResourceContentReturns<'a> {
        StreamResourceContentReturns {
            bufferedData: self.bufferedData.unwrap_or_default(),
        }
    }
}

impl<'a> StreamResourceContentParams<'a> { pub const METHOD: &'static str = "Network.streamResourceContent"; }

impl<'a> crate::CdpCommand<'a> for StreamResourceContentParams<'a> {
    const METHOD: &'static str = "Network.streamResourceContent";
    type Response = StreamResourceContentReturns<'a>;
}

/// Returns information about the COEP/COOP isolation status.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSecurityIsolationStatusParams<'a> {
    /// If no frameId is provided, the status of the target is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetSecurityIsolationStatusParams<'a> {
    pub fn builder() -> GetSecurityIsolationStatusParamsBuilder<'a> { GetSecurityIsolationStatusParamsBuilder::default() }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
}

#[derive(Default)]
pub struct GetSecurityIsolationStatusParamsBuilder<'a> {
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetSecurityIsolationStatusParamsBuilder<'a> {
    /// If no frameId is provided, the status of the target is provided.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetSecurityIsolationStatusParams<'a> {
        GetSecurityIsolationStatusParams {
            frameId: self.frameId,
        }
    }
}

/// Returns information about the COEP/COOP isolation status.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSecurityIsolationStatusReturns<'a> {
    status: SecurityIsolationStatus<'a>,
}

impl<'a> GetSecurityIsolationStatusReturns<'a> {
    pub fn builder() -> GetSecurityIsolationStatusReturnsBuilder<'a> { GetSecurityIsolationStatusReturnsBuilder::default() }
    pub fn status(&self) -> &SecurityIsolationStatus<'a> { &self.status }
}

#[derive(Default)]
pub struct GetSecurityIsolationStatusReturnsBuilder<'a> {
    status: Option<SecurityIsolationStatus<'a>>,
}

impl<'a> GetSecurityIsolationStatusReturnsBuilder<'a> {
    pub fn status(mut self, status: SecurityIsolationStatus<'a>) -> Self { self.status = Some(status); self }
    pub fn build(self) -> GetSecurityIsolationStatusReturns<'a> {
        GetSecurityIsolationStatusReturns {
            status: self.status.unwrap_or_default(),
        }
    }
}

impl<'a> GetSecurityIsolationStatusParams<'a> { pub const METHOD: &'static str = "Network.getSecurityIsolationStatus"; }

impl<'a> crate::CdpCommand<'a> for GetSecurityIsolationStatusParams<'a> {
    const METHOD: &'static str = "Network.getSecurityIsolationStatus";
    type Response = GetSecurityIsolationStatusReturns<'a>;
}

/// Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.
/// Enabling triggers 'reportingApiReportAdded' for all existing reports.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableReportingApiParams {
    /// Whether to enable or disable events for the Reporting API
    enable: bool,
}

impl EnableReportingApiParams {
    pub fn builder() -> EnableReportingApiParamsBuilder { EnableReportingApiParamsBuilder::default() }
    pub fn enable(&self) -> bool { self.enable }
}

#[derive(Default)]
pub struct EnableReportingApiParamsBuilder {
    enable: Option<bool>,
}

impl EnableReportingApiParamsBuilder {
    /// Whether to enable or disable events for the Reporting API
    pub fn enable(mut self, enable: bool) -> Self { self.enable = Some(enable); self }
    pub fn build(self) -> EnableReportingApiParams {
        EnableReportingApiParams {
            enable: self.enable.unwrap_or_default(),
        }
    }
}

impl EnableReportingApiParams { pub const METHOD: &'static str = "Network.enableReportingApi"; }

impl<'a> crate::CdpCommand<'a> for EnableReportingApiParams {
    const METHOD: &'static str = "Network.enableReportingApi";
    type Response = crate::EmptyReturns;
}

/// Sets up tracking device bound sessions and fetching of initial set of sessions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableDeviceBoundSessionsParams {
    /// Whether to enable or disable events.
    enable: bool,
}

impl EnableDeviceBoundSessionsParams {
    pub fn builder() -> EnableDeviceBoundSessionsParamsBuilder { EnableDeviceBoundSessionsParamsBuilder::default() }
    pub fn enable(&self) -> bool { self.enable }
}

#[derive(Default)]
pub struct EnableDeviceBoundSessionsParamsBuilder {
    enable: Option<bool>,
}

impl EnableDeviceBoundSessionsParamsBuilder {
    /// Whether to enable or disable events.
    pub fn enable(mut self, enable: bool) -> Self { self.enable = Some(enable); self }
    pub fn build(self) -> EnableDeviceBoundSessionsParams {
        EnableDeviceBoundSessionsParams {
            enable: self.enable.unwrap_or_default(),
        }
    }
}

impl EnableDeviceBoundSessionsParams { pub const METHOD: &'static str = "Network.enableDeviceBoundSessions"; }

impl<'a> crate::CdpCommand<'a> for EnableDeviceBoundSessionsParams {
    const METHOD: &'static str = "Network.enableDeviceBoundSessions";
    type Response = crate::EmptyReturns;
}

/// Deletes a device bound session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDeviceBoundSessionParams<'a> {
    key: DeviceBoundSessionKey<'a>,
}

impl<'a> DeleteDeviceBoundSessionParams<'a> {
    pub fn builder() -> DeleteDeviceBoundSessionParamsBuilder<'a> { DeleteDeviceBoundSessionParamsBuilder::default() }
    pub fn key(&self) -> &DeviceBoundSessionKey<'a> { &self.key }
}

#[derive(Default)]
pub struct DeleteDeviceBoundSessionParamsBuilder<'a> {
    key: Option<DeviceBoundSessionKey<'a>>,
}

impl<'a> DeleteDeviceBoundSessionParamsBuilder<'a> {
    pub fn key(mut self, key: DeviceBoundSessionKey<'a>) -> Self { self.key = Some(key); self }
    pub fn build(self) -> DeleteDeviceBoundSessionParams<'a> {
        DeleteDeviceBoundSessionParams {
            key: self.key.unwrap_or_default(),
        }
    }
}

impl<'a> DeleteDeviceBoundSessionParams<'a> { pub const METHOD: &'static str = "Network.deleteDeviceBoundSession"; }

impl<'a> crate::CdpCommand<'a> for DeleteDeviceBoundSessionParams<'a> {
    const METHOD: &'static str = "Network.deleteDeviceBoundSession";
    type Response = crate::EmptyReturns;
}

/// Fetches the schemeful site for a specific origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchSchemefulSiteParams<'a> {
    /// The URL origin.
    origin: Cow<'a, str>,
}

impl<'a> FetchSchemefulSiteParams<'a> {
    pub fn builder() -> FetchSchemefulSiteParamsBuilder<'a> { FetchSchemefulSiteParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}

#[derive(Default)]
pub struct FetchSchemefulSiteParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
}

impl<'a> FetchSchemefulSiteParamsBuilder<'a> {
    /// The URL origin.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn build(self) -> FetchSchemefulSiteParams<'a> {
        FetchSchemefulSiteParams {
            origin: self.origin.unwrap_or_default(),
        }
    }
}

/// Fetches the schemeful site for a specific origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchSchemefulSiteReturns<'a> {
    /// The corresponding schemeful site.
    schemefulSite: Cow<'a, str>,
}

impl<'a> FetchSchemefulSiteReturns<'a> {
    pub fn builder() -> FetchSchemefulSiteReturnsBuilder<'a> { FetchSchemefulSiteReturnsBuilder::default() }
    pub fn schemefulSite(&self) -> &str { self.schemefulSite.as_ref() }
}

#[derive(Default)]
pub struct FetchSchemefulSiteReturnsBuilder<'a> {
    schemefulSite: Option<Cow<'a, str>>,
}

impl<'a> FetchSchemefulSiteReturnsBuilder<'a> {
    /// The corresponding schemeful site.
    pub fn schemefulSite(mut self, schemefulSite: impl Into<Cow<'a, str>>) -> Self { self.schemefulSite = Some(schemefulSite.into()); self }
    pub fn build(self) -> FetchSchemefulSiteReturns<'a> {
        FetchSchemefulSiteReturns {
            schemefulSite: self.schemefulSite.unwrap_or_default(),
        }
    }
}

impl<'a> FetchSchemefulSiteParams<'a> { pub const METHOD: &'static str = "Network.fetchSchemefulSite"; }

impl<'a> crate::CdpCommand<'a> for FetchSchemefulSiteParams<'a> {
    const METHOD: &'static str = "Network.fetchSchemefulSite";
    type Response = FetchSchemefulSiteReturns<'a>;
}

/// Fetches the resource and returns the content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceParams<'a> {
    /// Frame id to get the resource for. Mandatory for frame targets, and
    /// should be omitted for worker targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
    /// URL of the resource to get content for.
    url: Cow<'a, str>,
    /// Options for the request.
    options: LoadNetworkResourceOptions,
}

impl<'a> LoadNetworkResourceParams<'a> {
    pub fn builder() -> LoadNetworkResourceParamsBuilder<'a> { LoadNetworkResourceParamsBuilder::default() }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn options(&self) -> &LoadNetworkResourceOptions { &self.options }
}

#[derive(Default)]
pub struct LoadNetworkResourceParamsBuilder<'a> {
    frameId: Option<crate::page::FrameId<'a>>,
    url: Option<Cow<'a, str>>,
    options: Option<LoadNetworkResourceOptions>,
}

impl<'a> LoadNetworkResourceParamsBuilder<'a> {
    /// Frame id to get the resource for. Mandatory for frame targets, and
    /// should be omitted for worker targets.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// URL of the resource to get content for.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Options for the request.
    pub fn options(mut self, options: LoadNetworkResourceOptions) -> Self { self.options = Some(options); self }
    pub fn build(self) -> LoadNetworkResourceParams<'a> {
        LoadNetworkResourceParams {
            frameId: self.frameId,
            url: self.url.unwrap_or_default(),
            options: self.options.unwrap_or_default(),
        }
    }
}

/// Fetches the resource and returns the content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceReturns<'a> {
    resource: LoadNetworkResourcePageResult<'a>,
}

impl<'a> LoadNetworkResourceReturns<'a> {
    pub fn builder() -> LoadNetworkResourceReturnsBuilder<'a> { LoadNetworkResourceReturnsBuilder::default() }
    pub fn resource(&self) -> &LoadNetworkResourcePageResult<'a> { &self.resource }
}

#[derive(Default)]
pub struct LoadNetworkResourceReturnsBuilder<'a> {
    resource: Option<LoadNetworkResourcePageResult<'a>>,
}

impl<'a> LoadNetworkResourceReturnsBuilder<'a> {
    pub fn resource(mut self, resource: LoadNetworkResourcePageResult<'a>) -> Self { self.resource = Some(resource); self }
    pub fn build(self) -> LoadNetworkResourceReturns<'a> {
        LoadNetworkResourceReturns {
            resource: self.resource.unwrap_or_default(),
        }
    }
}

impl<'a> LoadNetworkResourceParams<'a> { pub const METHOD: &'static str = "Network.loadNetworkResource"; }

impl<'a> crate::CdpCommand<'a> for LoadNetworkResourceParams<'a> {
    const METHOD: &'static str = "Network.loadNetworkResource";
    type Response = LoadNetworkResourceReturns<'a>;
}

/// Sets Controls for third-party cookie access
/// Page reload is required before the new cookie behavior will be observed

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieControlsParams {
    /// Whether 3pc restriction is enabled.
    enableThirdPartyCookieRestriction: bool,
}

impl SetCookieControlsParams {
    pub fn builder() -> SetCookieControlsParamsBuilder { SetCookieControlsParamsBuilder::default() }
    pub fn enableThirdPartyCookieRestriction(&self) -> bool { self.enableThirdPartyCookieRestriction }
}

#[derive(Default)]
pub struct SetCookieControlsParamsBuilder {
    enableThirdPartyCookieRestriction: Option<bool>,
}

impl SetCookieControlsParamsBuilder {
    /// Whether 3pc restriction is enabled.
    pub fn enableThirdPartyCookieRestriction(mut self, enableThirdPartyCookieRestriction: bool) -> Self { self.enableThirdPartyCookieRestriction = Some(enableThirdPartyCookieRestriction); self }
    pub fn build(self) -> SetCookieControlsParams {
        SetCookieControlsParams {
            enableThirdPartyCookieRestriction: self.enableThirdPartyCookieRestriction.unwrap_or_default(),
        }
    }
}

impl SetCookieControlsParams { pub const METHOD: &'static str = "Network.setCookieControls"; }

impl<'a> crate::CdpCommand<'a> for SetCookieControlsParams {
    const METHOD: &'static str = "Network.setCookieControls";
    type Response = crate::EmptyReturns;
}
