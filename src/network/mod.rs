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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTiming {
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime.
    #[serde(rename = "requestTime")]
    request_time: f64,
    /// Started resolving proxy.
    #[serde(rename = "proxyStart")]
    proxy_start: f64,
    /// Finished resolving proxy.
    #[serde(rename = "proxyEnd")]
    proxy_end: f64,
    /// Started DNS address resolve.
    #[serde(rename = "dnsStart")]
    dns_start: f64,
    /// Finished DNS address resolve.
    #[serde(rename = "dnsEnd")]
    dns_end: f64,
    /// Started connecting to the remote host.
    #[serde(rename = "connectStart")]
    connect_start: f64,
    /// Connected to the remote host.
    #[serde(rename = "connectEnd")]
    connect_end: f64,
    /// Started SSL handshake.
    #[serde(rename = "sslStart")]
    ssl_start: f64,
    /// Finished SSL handshake.
    #[serde(rename = "sslEnd")]
    ssl_end: f64,
    /// Started running ServiceWorker.
    #[serde(rename = "workerStart")]
    worker_start: f64,
    /// Finished Starting ServiceWorker.
    #[serde(rename = "workerReady")]
    worker_ready: f64,
    /// Started fetch event.
    #[serde(rename = "workerFetchStart")]
    worker_fetch_start: f64,
    /// Settled fetch event respondWith promise.
    #[serde(rename = "workerRespondWithSettled")]
    worker_respond_with_settled: f64,
    /// Started ServiceWorker static routing source evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "workerRouterEvaluationStart")]
    worker_router_evaluation_start: Option<f64>,
    /// Started cache lookup when the source was evaluated to 'cache'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "workerCacheLookupStart")]
    worker_cache_lookup_start: Option<f64>,
    /// Started sending request.
    #[serde(rename = "sendStart")]
    send_start: f64,
    /// Finished sending request.
    #[serde(rename = "sendEnd")]
    send_end: f64,
    /// Time the server started pushing request.
    #[serde(rename = "pushStart")]
    push_start: f64,
    /// Time the server finished pushing request.
    #[serde(rename = "pushEnd")]
    push_end: f64,
    /// Started receiving response headers.
    #[serde(rename = "receiveHeadersStart")]
    receive_headers_start: f64,
    /// Finished receiving response headers.
    #[serde(rename = "receiveHeadersEnd")]
    receive_headers_end: f64,
}

impl ResourceTiming {
    /// Creates a builder for this type with the required parameters:
    /// * `request_time`: Timing's requestTime is a baseline in seconds, while the other numbers are ticks in milliseconds relatively to this requestTime.
    /// * `proxy_start`: Started resolving proxy.
    /// * `proxy_end`: Finished resolving proxy.
    /// * `dns_start`: Started DNS address resolve.
    /// * `dns_end`: Finished DNS address resolve.
    /// * `connect_start`: Started connecting to the remote host.
    /// * `connect_end`: Connected to the remote host.
    /// * `ssl_start`: Started SSL handshake.
    /// * `ssl_end`: Finished SSL handshake.
    /// * `worker_start`: Started running ServiceWorker.
    /// * `worker_ready`: Finished Starting ServiceWorker.
    /// * `worker_fetch_start`: Started fetch event.
    /// * `worker_respond_with_settled`: Settled fetch event respondWith promise.
    /// * `send_start`: Started sending request.
    /// * `send_end`: Finished sending request.
    /// * `push_start`: Time the server started pushing request.
    /// * `push_end`: Time the server finished pushing request.
    /// * `receive_headers_start`: Started receiving response headers.
    /// * `receive_headers_end`: Finished receiving response headers.
    pub fn builder(request_time: f64, proxy_start: f64, proxy_end: f64, dns_start: f64, dns_end: f64, connect_start: f64, connect_end: f64, ssl_start: f64, ssl_end: f64, worker_start: f64, worker_ready: f64, worker_fetch_start: f64, worker_respond_with_settled: f64, send_start: f64, send_end: f64, push_start: f64, push_end: f64, receive_headers_start: f64, receive_headers_end: f64) -> ResourceTimingBuilder {
        ResourceTimingBuilder {
            request_time: request_time,
            proxy_start: proxy_start,
            proxy_end: proxy_end,
            dns_start: dns_start,
            dns_end: dns_end,
            connect_start: connect_start,
            connect_end: connect_end,
            ssl_start: ssl_start,
            ssl_end: ssl_end,
            worker_start: worker_start,
            worker_ready: worker_ready,
            worker_fetch_start: worker_fetch_start,
            worker_respond_with_settled: worker_respond_with_settled,
            worker_router_evaluation_start: None,
            worker_cache_lookup_start: None,
            send_start: send_start,
            send_end: send_end,
            push_start: push_start,
            push_end: push_end,
            receive_headers_start: receive_headers_start,
            receive_headers_end: receive_headers_end,
        }
    }
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime.
    pub fn request_time(&self) -> f64 { self.request_time }
    /// Started resolving proxy.
    pub fn proxy_start(&self) -> f64 { self.proxy_start }
    /// Finished resolving proxy.
    pub fn proxy_end(&self) -> f64 { self.proxy_end }
    /// Started DNS address resolve.
    pub fn dns_start(&self) -> f64 { self.dns_start }
    /// Finished DNS address resolve.
    pub fn dns_end(&self) -> f64 { self.dns_end }
    /// Started connecting to the remote host.
    pub fn connect_start(&self) -> f64 { self.connect_start }
    /// Connected to the remote host.
    pub fn connect_end(&self) -> f64 { self.connect_end }
    /// Started SSL handshake.
    pub fn ssl_start(&self) -> f64 { self.ssl_start }
    /// Finished SSL handshake.
    pub fn ssl_end(&self) -> f64 { self.ssl_end }
    /// Started running ServiceWorker.
    pub fn worker_start(&self) -> f64 { self.worker_start }
    /// Finished Starting ServiceWorker.
    pub fn worker_ready(&self) -> f64 { self.worker_ready }
    /// Started fetch event.
    pub fn worker_fetch_start(&self) -> f64 { self.worker_fetch_start }
    /// Settled fetch event respondWith promise.
    pub fn worker_respond_with_settled(&self) -> f64 { self.worker_respond_with_settled }
    /// Started ServiceWorker static routing source evaluation.
    pub fn worker_router_evaluation_start(&self) -> Option<f64> { self.worker_router_evaluation_start }
    /// Started cache lookup when the source was evaluated to 'cache'.
    pub fn worker_cache_lookup_start(&self) -> Option<f64> { self.worker_cache_lookup_start }
    /// Started sending request.
    pub fn send_start(&self) -> f64 { self.send_start }
    /// Finished sending request.
    pub fn send_end(&self) -> f64 { self.send_end }
    /// Time the server started pushing request.
    pub fn push_start(&self) -> f64 { self.push_start }
    /// Time the server finished pushing request.
    pub fn push_end(&self) -> f64 { self.push_end }
    /// Started receiving response headers.
    pub fn receive_headers_start(&self) -> f64 { self.receive_headers_start }
    /// Finished receiving response headers.
    pub fn receive_headers_end(&self) -> f64 { self.receive_headers_end }
}


pub struct ResourceTimingBuilder {
    request_time: f64,
    proxy_start: f64,
    proxy_end: f64,
    dns_start: f64,
    dns_end: f64,
    connect_start: f64,
    connect_end: f64,
    ssl_start: f64,
    ssl_end: f64,
    worker_start: f64,
    worker_ready: f64,
    worker_fetch_start: f64,
    worker_respond_with_settled: f64,
    worker_router_evaluation_start: Option<f64>,
    worker_cache_lookup_start: Option<f64>,
    send_start: f64,
    send_end: f64,
    push_start: f64,
    push_end: f64,
    receive_headers_start: f64,
    receive_headers_end: f64,
}

impl ResourceTimingBuilder {
    /// Started ServiceWorker static routing source evaluation.
    pub fn worker_router_evaluation_start(mut self, worker_router_evaluation_start: f64) -> Self { self.worker_router_evaluation_start = Some(worker_router_evaluation_start); self }
    /// Started cache lookup when the source was evaluated to 'cache'.
    pub fn worker_cache_lookup_start(mut self, worker_cache_lookup_start: f64) -> Self { self.worker_cache_lookup_start = Some(worker_cache_lookup_start); self }
    pub fn build(self) -> ResourceTiming {
        ResourceTiming {
            request_time: self.request_time,
            proxy_start: self.proxy_start,
            proxy_end: self.proxy_end,
            dns_start: self.dns_start,
            dns_end: self.dns_end,
            connect_start: self.connect_start,
            connect_end: self.connect_end,
            ssl_start: self.ssl_start,
            ssl_end: self.ssl_end,
            worker_start: self.worker_start,
            worker_ready: self.worker_ready,
            worker_fetch_start: self.worker_fetch_start,
            worker_respond_with_settled: self.worker_respond_with_settled,
            worker_router_evaluation_start: self.worker_router_evaluation_start,
            worker_cache_lookup_start: self.worker_cache_lookup_start,
            send_start: self.send_start,
            send_end: self.send_end,
            push_start: self.push_start,
            push_end: self.push_end,
            receive_headers_start: self.receive_headers_start,
            receive_headers_end: self.receive_headers_end,
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
    /// Creates a builder for this type.
    pub fn builder() -> PostDataEntryBuilder<'a> {
        PostDataEntryBuilder {
            bytes: None,
        }
    }
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlFragment")]
    url_fragment: Option<Cow<'a, str>>,
    /// HTTP request method.
    method: Cow<'a, str>,
    /// HTTP request headers.
    headers: Headers,
    /// HTTP POST request data.
    /// Use postDataEntries instead.
    #[serde(skip_serializing_if = "Option::is_none", rename = "postData")]
    post_data: Option<Cow<'a, str>>,
    /// True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasPostData")]
    has_post_data: Option<bool>,
    /// Request body elements (post data broken into individual entries).
    #[serde(skip_serializing_if = "Option::is_none", rename = "postDataEntries")]
    post_data_entries: Option<Vec<PostDataEntry<'a>>>,
    /// The mixed content type of the request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "mixedContentType")]
    mixed_content_type: Option<crate::security::MixedContentType>,
    /// Priority of the resource request at the time request is sent.
    #[serde(rename = "initialPriority")]
    initial_priority: ResourcePriority,
    /// The referrer policy of the request, as defined in <https://www.w3.org/TR/referrer-policy/>
    #[serde(rename = "referrerPolicy")]
    referrer_policy: Cow<'a, str>,
    /// Whether is loaded via link preload.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isLinkPreload")]
    is_link_preload: Option<bool>,
    /// Set for requests when the TrustToken API is used. Contains the parameters
    /// passed by the developer (e.g. via "fetch") as understood by the backend.
    #[serde(skip_serializing_if = "Option::is_none", rename = "trustTokenParams")]
    trust_token_params: Option<TrustTokenParams<'a>>,
    /// True if this resource request is considered to be the 'same site' as the
    /// request corresponding to the main frame.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isSameSite")]
    is_same_site: Option<bool>,
    /// True when the resource request is ad-related.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isAdRelated")]
    is_ad_related: Option<bool>,
}

impl<'a> Request<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: Request URL (without fragment).
    /// * `method`: HTTP request method.
    /// * `headers`: HTTP request headers.
    /// * `initial_priority`: Priority of the resource request at the time request is sent.
    /// * `referrer_policy`: The referrer policy of the request, as defined in <https://www.w3.org/TR/referrer-policy/>
    pub fn builder(url: impl Into<Cow<'a, str>>, method: impl Into<Cow<'a, str>>, headers: Headers, initial_priority: impl Into<ResourcePriority>, referrer_policy: impl Into<Cow<'a, str>>) -> RequestBuilder<'a> {
        RequestBuilder {
            url: url.into(),
            url_fragment: None,
            method: method.into(),
            headers: headers,
            post_data: None,
            has_post_data: None,
            post_data_entries: None,
            mixed_content_type: None,
            initial_priority: initial_priority.into(),
            referrer_policy: referrer_policy.into(),
            is_link_preload: None,
            trust_token_params: None,
            is_same_site: None,
            is_ad_related: None,
        }
    }
    /// Request URL (without fragment).
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Fragment of the requested URL starting with hash, if present.
    pub fn url_fragment(&self) -> Option<&str> { self.url_fragment.as_deref() }
    /// HTTP request method.
    pub fn method(&self) -> &str { self.method.as_ref() }
    /// HTTP request headers.
    pub fn headers(&self) -> &Headers { &self.headers }
    /// HTTP POST request data.
    /// Use postDataEntries instead.
    pub fn post_data(&self) -> Option<&str> { self.post_data.as_deref() }
    /// True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.
    pub fn has_post_data(&self) -> Option<bool> { self.has_post_data }
    /// Request body elements (post data broken into individual entries).
    pub fn post_data_entries(&self) -> Option<&[PostDataEntry<'a>]> { self.post_data_entries.as_deref() }
    /// The mixed content type of the request.
    pub fn mixed_content_type(&self) -> Option<&crate::security::MixedContentType> { self.mixed_content_type.as_ref() }
    /// Priority of the resource request at the time request is sent.
    pub fn initial_priority(&self) -> &ResourcePriority { &self.initial_priority }
    /// The referrer policy of the request, as defined in <https://www.w3.org/TR/referrer-policy/>
    pub fn referrer_policy(&self) -> &str { self.referrer_policy.as_ref() }
    /// Whether is loaded via link preload.
    pub fn is_link_preload(&self) -> Option<bool> { self.is_link_preload }
    /// Set for requests when the TrustToken API is used. Contains the parameters
    /// passed by the developer (e.g. via "fetch") as understood by the backend.
    pub fn trust_token_params(&self) -> Option<&TrustTokenParams<'a>> { self.trust_token_params.as_ref() }
    /// True if this resource request is considered to be the 'same site' as the
    /// request corresponding to the main frame.
    pub fn is_same_site(&self) -> Option<bool> { self.is_same_site }
    /// True when the resource request is ad-related.
    pub fn is_ad_related(&self) -> Option<bool> { self.is_ad_related }
}


pub struct RequestBuilder<'a> {
    url: Cow<'a, str>,
    url_fragment: Option<Cow<'a, str>>,
    method: Cow<'a, str>,
    headers: Headers,
    post_data: Option<Cow<'a, str>>,
    has_post_data: Option<bool>,
    post_data_entries: Option<Vec<PostDataEntry<'a>>>,
    mixed_content_type: Option<crate::security::MixedContentType>,
    initial_priority: ResourcePriority,
    referrer_policy: Cow<'a, str>,
    is_link_preload: Option<bool>,
    trust_token_params: Option<TrustTokenParams<'a>>,
    is_same_site: Option<bool>,
    is_ad_related: Option<bool>,
}

impl<'a> RequestBuilder<'a> {
    /// Fragment of the requested URL starting with hash, if present.
    pub fn url_fragment(mut self, url_fragment: impl Into<Cow<'a, str>>) -> Self { self.url_fragment = Some(url_fragment.into()); self }
    /// HTTP POST request data.
    /// Use postDataEntries instead.
    pub fn post_data(mut self, post_data: impl Into<Cow<'a, str>>) -> Self { self.post_data = Some(post_data.into()); self }
    /// True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.
    pub fn has_post_data(mut self, has_post_data: bool) -> Self { self.has_post_data = Some(has_post_data); self }
    /// Request body elements (post data broken into individual entries).
    pub fn post_data_entries(mut self, post_data_entries: Vec<PostDataEntry<'a>>) -> Self { self.post_data_entries = Some(post_data_entries); self }
    /// The mixed content type of the request.
    pub fn mixed_content_type(mut self, mixed_content_type: crate::security::MixedContentType) -> Self { self.mixed_content_type = Some(mixed_content_type); self }
    /// Whether is loaded via link preload.
    pub fn is_link_preload(mut self, is_link_preload: bool) -> Self { self.is_link_preload = Some(is_link_preload); self }
    /// Set for requests when the TrustToken API is used. Contains the parameters
    /// passed by the developer (e.g. via "fetch") as understood by the backend.
    pub fn trust_token_params(mut self, trust_token_params: TrustTokenParams<'a>) -> Self { self.trust_token_params = Some(trust_token_params); self }
    /// True if this resource request is considered to be the 'same site' as the
    /// request corresponding to the main frame.
    pub fn is_same_site(mut self, is_same_site: bool) -> Self { self.is_same_site = Some(is_same_site); self }
    /// True when the resource request is ad-related.
    pub fn is_ad_related(mut self, is_ad_related: bool) -> Self { self.is_ad_related = Some(is_ad_related); self }
    pub fn build(self) -> Request<'a> {
        Request {
            url: self.url,
            url_fragment: self.url_fragment,
            method: self.method,
            headers: self.headers,
            post_data: self.post_data,
            has_post_data: self.has_post_data,
            post_data_entries: self.post_data_entries,
            mixed_content_type: self.mixed_content_type,
            initial_priority: self.initial_priority,
            referrer_policy: self.referrer_policy,
            is_link_preload: self.is_link_preload,
            trust_token_params: self.trust_token_params,
            is_same_site: self.is_same_site,
            is_ad_related: self.is_ad_related,
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
    #[serde(rename = "logDescription")]
    log_description: Cow<'a, str>,
    /// Log ID.
    #[serde(rename = "logId")]
    log_id: Cow<'a, str>,
    /// Issuance date. Unlike TimeSinceEpoch, this contains the number of
    /// milliseconds since January 1, 1970, UTC, not the number of seconds.
    timestamp: f64,
    /// Hash algorithm.
    #[serde(rename = "hashAlgorithm")]
    hash_algorithm: Cow<'a, str>,
    /// Signature algorithm.
    #[serde(rename = "signatureAlgorithm")]
    signature_algorithm: Cow<'a, str>,
    /// Signature data.
    #[serde(rename = "signatureData")]
    signature_data: Cow<'a, str>,
}

impl<'a> SignedCertificateTimestamp<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `status`: Validation status.
    /// * `origin`: Origin.
    /// * `log_description`: Log name / description.
    /// * `log_id`: Log ID.
    /// * `timestamp`: Issuance date. Unlike TimeSinceEpoch, this contains the number of milliseconds since January 1, 1970, UTC, not the number of seconds.
    /// * `hash_algorithm`: Hash algorithm.
    /// * `signature_algorithm`: Signature algorithm.
    /// * `signature_data`: Signature data.
    pub fn builder(status: impl Into<Cow<'a, str>>, origin: impl Into<Cow<'a, str>>, log_description: impl Into<Cow<'a, str>>, log_id: impl Into<Cow<'a, str>>, timestamp: f64, hash_algorithm: impl Into<Cow<'a, str>>, signature_algorithm: impl Into<Cow<'a, str>>, signature_data: impl Into<Cow<'a, str>>) -> SignedCertificateTimestampBuilder<'a> {
        SignedCertificateTimestampBuilder {
            status: status.into(),
            origin: origin.into(),
            log_description: log_description.into(),
            log_id: log_id.into(),
            timestamp: timestamp,
            hash_algorithm: hash_algorithm.into(),
            signature_algorithm: signature_algorithm.into(),
            signature_data: signature_data.into(),
        }
    }
    /// Validation status.
    pub fn status(&self) -> &str { self.status.as_ref() }
    /// Origin.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    /// Log name / description.
    pub fn log_description(&self) -> &str { self.log_description.as_ref() }
    /// Log ID.
    pub fn log_id(&self) -> &str { self.log_id.as_ref() }
    /// Issuance date. Unlike TimeSinceEpoch, this contains the number of
    /// milliseconds since January 1, 1970, UTC, not the number of seconds.
    pub fn timestamp(&self) -> f64 { self.timestamp }
    /// Hash algorithm.
    pub fn hash_algorithm(&self) -> &str { self.hash_algorithm.as_ref() }
    /// Signature algorithm.
    pub fn signature_algorithm(&self) -> &str { self.signature_algorithm.as_ref() }
    /// Signature data.
    pub fn signature_data(&self) -> &str { self.signature_data.as_ref() }
}


pub struct SignedCertificateTimestampBuilder<'a> {
    status: Cow<'a, str>,
    origin: Cow<'a, str>,
    log_description: Cow<'a, str>,
    log_id: Cow<'a, str>,
    timestamp: f64,
    hash_algorithm: Cow<'a, str>,
    signature_algorithm: Cow<'a, str>,
    signature_data: Cow<'a, str>,
}

impl<'a> SignedCertificateTimestampBuilder<'a> {
    pub fn build(self) -> SignedCertificateTimestamp<'a> {
        SignedCertificateTimestamp {
            status: self.status,
            origin: self.origin,
            log_description: self.log_description,
            log_id: self.log_id,
            timestamp: self.timestamp,
            hash_algorithm: self.hash_algorithm,
            signature_algorithm: self.signature_algorithm,
            signature_data: self.signature_data,
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
    #[serde(rename = "keyExchange")]
    key_exchange: Cow<'a, str>,
    /// (EC)DH group used by the connection, if applicable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "keyExchangeGroup")]
    key_exchange_group: Option<Cow<'a, str>>,
    /// Cipher name.
    cipher: Cow<'a, str>,
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.
    #[serde(skip_serializing_if = "Option::is_none")]
    mac: Option<Cow<'a, str>>,
    /// Certificate ID value.
    #[serde(rename = "certificateId")]
    certificate_id: crate::security::CertificateId,
    /// Certificate subject name.
    #[serde(rename = "subjectName")]
    subject_name: Cow<'a, str>,
    /// Subject Alternative Name (SAN) DNS names and IP addresses.
    #[serde(rename = "sanList")]
    san_list: Vec<Cow<'a, str>>,
    /// Name of the issuing CA.
    issuer: Cow<'a, str>,
    /// Certificate valid from date.
    #[serde(rename = "validFrom")]
    valid_from: TimeSinceEpoch,
    /// Certificate valid to (expiration) date
    #[serde(rename = "validTo")]
    valid_to: TimeSinceEpoch,
    /// List of signed certificate timestamps (SCTs).
    #[serde(rename = "signedCertificateTimestampList")]
    signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp<'a>>,
    /// Whether the request complied with Certificate Transparency policy
    #[serde(rename = "certificateTransparencyCompliance")]
    certificate_transparency_compliance: CertificateTransparencyCompliance,
    /// The signature algorithm used by the server in the TLS server signature,
    /// represented as a TLS SignatureScheme code point. Omitted if not
    /// applicable or not known.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serverSignatureAlgorithm")]
    server_signature_algorithm: Option<i64>,
    /// Whether the connection used Encrypted ClientHello
    #[serde(rename = "encryptedClientHello")]
    encrypted_client_hello: bool,
}

impl<'a> SecurityDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `protocol`: Protocol name (e.g. "TLS 1.2" or "QUIC").
    /// * `key_exchange`: Key Exchange used by the connection, or the empty string if not applicable.
    /// * `cipher`: Cipher name.
    /// * `certificate_id`: Certificate ID value.
    /// * `subject_name`: Certificate subject name.
    /// * `san_list`: Subject Alternative Name (SAN) DNS names and IP addresses.
    /// * `issuer`: Name of the issuing CA.
    /// * `valid_from`: Certificate valid from date.
    /// * `valid_to`: Certificate valid to (expiration) date
    /// * `signed_certificate_timestamp_list`: List of signed certificate timestamps (SCTs).
    /// * `certificate_transparency_compliance`: Whether the request complied with Certificate Transparency policy
    /// * `encrypted_client_hello`: Whether the connection used Encrypted ClientHello
    pub fn builder(protocol: impl Into<Cow<'a, str>>, key_exchange: impl Into<Cow<'a, str>>, cipher: impl Into<Cow<'a, str>>, certificate_id: crate::security::CertificateId, subject_name: impl Into<Cow<'a, str>>, san_list: Vec<Cow<'a, str>>, issuer: impl Into<Cow<'a, str>>, valid_from: TimeSinceEpoch, valid_to: TimeSinceEpoch, signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp<'a>>, certificate_transparency_compliance: impl Into<CertificateTransparencyCompliance>, encrypted_client_hello: bool) -> SecurityDetailsBuilder<'a> {
        SecurityDetailsBuilder {
            protocol: protocol.into(),
            key_exchange: key_exchange.into(),
            key_exchange_group: None,
            cipher: cipher.into(),
            mac: None,
            certificate_id: certificate_id,
            subject_name: subject_name.into(),
            san_list: san_list,
            issuer: issuer.into(),
            valid_from: valid_from,
            valid_to: valid_to,
            signed_certificate_timestamp_list: signed_certificate_timestamp_list,
            certificate_transparency_compliance: certificate_transparency_compliance.into(),
            server_signature_algorithm: None,
            encrypted_client_hello: encrypted_client_hello,
        }
    }
    /// Protocol name (e.g. "TLS 1.2" or "QUIC").
    pub fn protocol(&self) -> &str { self.protocol.as_ref() }
    /// Key Exchange used by the connection, or the empty string if not applicable.
    pub fn key_exchange(&self) -> &str { self.key_exchange.as_ref() }
    /// (EC)DH group used by the connection, if applicable.
    pub fn key_exchange_group(&self) -> Option<&str> { self.key_exchange_group.as_deref() }
    /// Cipher name.
    pub fn cipher(&self) -> &str { self.cipher.as_ref() }
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.
    pub fn mac(&self) -> Option<&str> { self.mac.as_deref() }
    /// Certificate ID value.
    pub fn certificate_id(&self) -> &crate::security::CertificateId { &self.certificate_id }
    /// Certificate subject name.
    pub fn subject_name(&self) -> &str { self.subject_name.as_ref() }
    /// Subject Alternative Name (SAN) DNS names and IP addresses.
    pub fn san_list(&self) -> &[Cow<'a, str>] { &self.san_list }
    /// Name of the issuing CA.
    pub fn issuer(&self) -> &str { self.issuer.as_ref() }
    /// Certificate valid from date.
    pub fn valid_from(&self) -> &TimeSinceEpoch { &self.valid_from }
    /// Certificate valid to (expiration) date
    pub fn valid_to(&self) -> &TimeSinceEpoch { &self.valid_to }
    /// List of signed certificate timestamps (SCTs).
    pub fn signed_certificate_timestamp_list(&self) -> &[SignedCertificateTimestamp<'a>] { &self.signed_certificate_timestamp_list }
    /// Whether the request complied with Certificate Transparency policy
    pub fn certificate_transparency_compliance(&self) -> &CertificateTransparencyCompliance { &self.certificate_transparency_compliance }
    /// The signature algorithm used by the server in the TLS server signature,
    /// represented as a TLS SignatureScheme code point. Omitted if not
    /// applicable or not known.
    pub fn server_signature_algorithm(&self) -> Option<i64> { self.server_signature_algorithm }
    /// Whether the connection used Encrypted ClientHello
    pub fn encrypted_client_hello(&self) -> bool { self.encrypted_client_hello }
}


pub struct SecurityDetailsBuilder<'a> {
    protocol: Cow<'a, str>,
    key_exchange: Cow<'a, str>,
    key_exchange_group: Option<Cow<'a, str>>,
    cipher: Cow<'a, str>,
    mac: Option<Cow<'a, str>>,
    certificate_id: crate::security::CertificateId,
    subject_name: Cow<'a, str>,
    san_list: Vec<Cow<'a, str>>,
    issuer: Cow<'a, str>,
    valid_from: TimeSinceEpoch,
    valid_to: TimeSinceEpoch,
    signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp<'a>>,
    certificate_transparency_compliance: CertificateTransparencyCompliance,
    server_signature_algorithm: Option<i64>,
    encrypted_client_hello: bool,
}

impl<'a> SecurityDetailsBuilder<'a> {
    /// (EC)DH group used by the connection, if applicable.
    pub fn key_exchange_group(mut self, key_exchange_group: impl Into<Cow<'a, str>>) -> Self { self.key_exchange_group = Some(key_exchange_group.into()); self }
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.
    pub fn mac(mut self, mac: impl Into<Cow<'a, str>>) -> Self { self.mac = Some(mac.into()); self }
    /// The signature algorithm used by the server in the TLS server signature,
    /// represented as a TLS SignatureScheme code point. Omitted if not
    /// applicable or not known.
    pub fn server_signature_algorithm(mut self, server_signature_algorithm: i64) -> Self { self.server_signature_algorithm = Some(server_signature_algorithm); self }
    pub fn build(self) -> SecurityDetails<'a> {
        SecurityDetails {
            protocol: self.protocol,
            key_exchange: self.key_exchange,
            key_exchange_group: self.key_exchange_group,
            cipher: self.cipher,
            mac: self.mac,
            certificate_id: self.certificate_id,
            subject_name: self.subject_name,
            san_list: self.san_list,
            issuer: self.issuer,
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            signed_certificate_timestamp_list: self.signed_certificate_timestamp_list,
            certificate_transparency_compliance: self.certificate_transparency_compliance,
            server_signature_algorithm: self.server_signature_algorithm,
            encrypted_client_hello: self.encrypted_client_hello,
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
    #[serde(rename = "corsError")]
    cors_error: CorsError,
    #[serde(rename = "failedParameter")]
    failed_parameter: Cow<'a, str>,
}

impl<'a> CorsErrorStatus<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cors_error`: 
    /// * `failed_parameter`: 
    pub fn builder(cors_error: impl Into<CorsError>, failed_parameter: impl Into<Cow<'a, str>>) -> CorsErrorStatusBuilder<'a> {
        CorsErrorStatusBuilder {
            cors_error: cors_error.into(),
            failed_parameter: failed_parameter.into(),
        }
    }
    pub fn cors_error(&self) -> &CorsError { &self.cors_error }
    pub fn failed_parameter(&self) -> &str { self.failed_parameter.as_ref() }
}


pub struct CorsErrorStatusBuilder<'a> {
    cors_error: CorsError,
    failed_parameter: Cow<'a, str>,
}

impl<'a> CorsErrorStatusBuilder<'a> {
    pub fn build(self) -> CorsErrorStatus<'a> {
        CorsErrorStatus {
            cors_error: self.cors_error,
            failed_parameter: self.failed_parameter,
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
    #[serde(rename = "refreshPolicy")]
    refresh_policy: Cow<'a, str>,
    /// Origins of issuers from whom to request tokens or redemption
    /// records.
    #[serde(skip_serializing_if = "Option::is_none")]
    issuers: Option<Vec<Cow<'a, str>>>,
}

impl<'a> TrustTokenParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `operation`: 
    /// * `refresh_policy`: Only set for "token-redemption" operation and determine whether to request a fresh SRR or use a still valid cached SRR.
    pub fn builder(operation: impl Into<TrustTokenOperationType>, refresh_policy: impl Into<Cow<'a, str>>) -> TrustTokenParamsBuilder<'a> {
        TrustTokenParamsBuilder {
            operation: operation.into(),
            refresh_policy: refresh_policy.into(),
            issuers: None,
        }
    }
    pub fn operation(&self) -> &TrustTokenOperationType { &self.operation }
    /// Only set for "token-redemption" operation and determine whether
    /// to request a fresh SRR or use a still valid cached SRR.
    pub fn refresh_policy(&self) -> &str { self.refresh_policy.as_ref() }
    /// Origins of issuers from whom to request tokens or redemption
    /// records.
    pub fn issuers(&self) -> Option<&[Cow<'a, str>]> { self.issuers.as_deref() }
}


pub struct TrustTokenParamsBuilder<'a> {
    operation: TrustTokenOperationType,
    refresh_policy: Cow<'a, str>,
    issuers: Option<Vec<Cow<'a, str>>>,
}

impl<'a> TrustTokenParamsBuilder<'a> {
    /// Origins of issuers from whom to request tokens or redemption
    /// records.
    pub fn issuers(mut self, issuers: Vec<Cow<'a, str>>) -> Self { self.issuers = Some(issuers); self }
    pub fn build(self) -> TrustTokenParams<'a> {
        TrustTokenParams {
            operation: self.operation,
            refresh_policy: self.refresh_policy,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "ruleIdMatched")]
    rule_id_matched: Option<u64>,
    /// The router source of the matched rule. If there is a matched rule, this
    /// field will be set, otherwise no value will be set.
    #[serde(skip_serializing_if = "Option::is_none", rename = "matchedSourceType")]
    matched_source_type: Option<ServiceWorkerRouterSource>,
    /// The actual router source used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "actualSourceType")]
    actual_source_type: Option<ServiceWorkerRouterSource>,
}

impl ServiceWorkerRouterInfo {
    /// Creates a builder for this type.
    pub fn builder() -> ServiceWorkerRouterInfoBuilder {
        ServiceWorkerRouterInfoBuilder {
            rule_id_matched: None,
            matched_source_type: None,
            actual_source_type: None,
        }
    }
    /// ID of the rule matched. If there is a matched rule, this field will
    /// be set, otherwiser no value will be set.
    pub fn rule_id_matched(&self) -> Option<u64> { self.rule_id_matched }
    /// The router source of the matched rule. If there is a matched rule, this
    /// field will be set, otherwise no value will be set.
    pub fn matched_source_type(&self) -> Option<&ServiceWorkerRouterSource> { self.matched_source_type.as_ref() }
    /// The actual router source used.
    pub fn actual_source_type(&self) -> Option<&ServiceWorkerRouterSource> { self.actual_source_type.as_ref() }
}

#[derive(Default)]
pub struct ServiceWorkerRouterInfoBuilder {
    rule_id_matched: Option<u64>,
    matched_source_type: Option<ServiceWorkerRouterSource>,
    actual_source_type: Option<ServiceWorkerRouterSource>,
}

impl ServiceWorkerRouterInfoBuilder {
    /// ID of the rule matched. If there is a matched rule, this field will
    /// be set, otherwiser no value will be set.
    pub fn rule_id_matched(mut self, rule_id_matched: u64) -> Self { self.rule_id_matched = Some(rule_id_matched); self }
    /// The router source of the matched rule. If there is a matched rule, this
    /// field will be set, otherwise no value will be set.
    pub fn matched_source_type(mut self, matched_source_type: impl Into<ServiceWorkerRouterSource>) -> Self { self.matched_source_type = Some(matched_source_type.into()); self }
    /// The actual router source used.
    pub fn actual_source_type(mut self, actual_source_type: impl Into<ServiceWorkerRouterSource>) -> Self { self.actual_source_type = Some(actual_source_type.into()); self }
    pub fn build(self) -> ServiceWorkerRouterInfo {
        ServiceWorkerRouterInfo {
            rule_id_matched: self.rule_id_matched,
            matched_source_type: self.matched_source_type,
            actual_source_type: self.actual_source_type,
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
    #[serde(rename = "statusText")]
    status_text: Cow<'a, str>,
    /// HTTP response headers.
    headers: Headers,
    /// HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo.
    #[serde(skip_serializing_if = "Option::is_none", rename = "headersText")]
    headers_text: Option<Cow<'a, str>>,
    /// Resource mimeType as determined by the browser.
    #[serde(rename = "mimeType")]
    mime_type: Cow<'a, str>,
    /// Resource charset as determined by the browser (if applicable).
    charset: Cow<'a, str>,
    /// Refined HTTP request headers that were actually transmitted over the network.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    request_headers: Option<Headers>,
    /// HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestHeadersText")]
    request_headers_text: Option<Cow<'a, str>>,
    /// Specifies whether physical connection was actually reused for this request.
    #[serde(rename = "connectionReused")]
    connection_reused: bool,
    /// Physical connection id that was actually used for this request.
    #[serde(rename = "connectionId")]
    connection_id: f64,
    /// Remote IP address.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remoteIPAddress")]
    remote_ip_address: Option<Cow<'a, str>>,
    /// Remote port.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remotePort")]
    remote_port: Option<i64>,
    /// Specifies that the request was served from the disk cache.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromDiskCache")]
    from_disk_cache: Option<bool>,
    /// Specifies that the request was served from the ServiceWorker.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromServiceWorker")]
    from_service_worker: Option<bool>,
    /// Specifies that the request was served from the prefetch cache.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromPrefetchCache")]
    from_prefetch_cache: Option<bool>,
    /// Specifies that the request was served from the prefetch cache.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromEarlyHints")]
    from_early_hints: Option<bool>,
    /// Information about how ServiceWorker Static Router API was used. If this
    /// field is set with 'matchedSourceType' field, a matching rule is found.
    /// If this field is set without 'matchedSource', no matching rule is found.
    /// Otherwise, the API is not used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serviceWorkerRouterInfo")]
    service_worker_router_info: Option<ServiceWorkerRouterInfo>,
    /// Total number of bytes received for this request so far.
    #[serde(rename = "encodedDataLength")]
    encoded_data_length: f64,
    /// Timing information for the given request.
    #[serde(skip_serializing_if = "Option::is_none")]
    timing: Option<ResourceTiming>,
    /// Response source of response from ServiceWorker.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serviceWorkerResponseSource")]
    service_worker_response_source: Option<ServiceWorkerResponseSource>,
    /// The time at which the returned response was generated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseTime")]
    response_time: Option<TimeSinceEpoch>,
    /// Cache Storage Cache Name.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cacheStorageCacheName")]
    cache_storage_cache_name: Option<Cow<'a, str>>,
    /// Protocol used to fetch this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<Cow<'a, str>>,
    /// The reason why Chrome uses a specific transport protocol for HTTP semantics.
    #[serde(skip_serializing_if = "Option::is_none", rename = "alternateProtocolUsage")]
    alternate_protocol_usage: Option<AlternateProtocolUsage>,
    /// Security state of the request resource.
    #[serde(rename = "securityState")]
    security_state: crate::security::SecurityState,
    /// Security details for the request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityDetails")]
    security_details: Option<SecurityDetails<'a>>,
}

impl<'a> Response<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: Response URL. This URL can be different from CachedResource.url in case of redirect.
    /// * `status`: HTTP response status code.
    /// * `status_text`: HTTP response status text.
    /// * `headers`: HTTP response headers.
    /// * `mime_type`: Resource mimeType as determined by the browser.
    /// * `charset`: Resource charset as determined by the browser (if applicable).
    /// * `connection_reused`: Specifies whether physical connection was actually reused for this request.
    /// * `connection_id`: Physical connection id that was actually used for this request.
    /// * `encoded_data_length`: Total number of bytes received for this request so far.
    /// * `security_state`: Security state of the request resource.
    pub fn builder(url: impl Into<Cow<'a, str>>, status: i64, status_text: impl Into<Cow<'a, str>>, headers: Headers, mime_type: impl Into<Cow<'a, str>>, charset: impl Into<Cow<'a, str>>, connection_reused: bool, connection_id: f64, encoded_data_length: f64, security_state: crate::security::SecurityState) -> ResponseBuilder<'a> {
        ResponseBuilder {
            url: url.into(),
            status: status,
            status_text: status_text.into(),
            headers: headers,
            headers_text: None,
            mime_type: mime_type.into(),
            charset: charset.into(),
            request_headers: None,
            request_headers_text: None,
            connection_reused: connection_reused,
            connection_id: connection_id,
            remote_ip_address: None,
            remote_port: None,
            from_disk_cache: None,
            from_service_worker: None,
            from_prefetch_cache: None,
            from_early_hints: None,
            service_worker_router_info: None,
            encoded_data_length: encoded_data_length,
            timing: None,
            service_worker_response_source: None,
            response_time: None,
            cache_storage_cache_name: None,
            protocol: None,
            alternate_protocol_usage: None,
            security_state: security_state,
            security_details: None,
        }
    }
    /// Response URL. This URL can be different from CachedResource.url in case of redirect.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// HTTP response status code.
    pub fn status(&self) -> i64 { self.status }
    /// HTTP response status text.
    pub fn status_text(&self) -> &str { self.status_text.as_ref() }
    /// HTTP response headers.
    pub fn headers(&self) -> &Headers { &self.headers }
    /// HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo.
    pub fn headers_text(&self) -> Option<&str> { self.headers_text.as_deref() }
    /// Resource mimeType as determined by the browser.
    pub fn mime_type(&self) -> &str { self.mime_type.as_ref() }
    /// Resource charset as determined by the browser (if applicable).
    pub fn charset(&self) -> &str { self.charset.as_ref() }
    /// Refined HTTP request headers that were actually transmitted over the network.
    pub fn request_headers(&self) -> Option<&Headers> { self.request_headers.as_ref() }
    /// HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo.
    pub fn request_headers_text(&self) -> Option<&str> { self.request_headers_text.as_deref() }
    /// Specifies whether physical connection was actually reused for this request.
    pub fn connection_reused(&self) -> bool { self.connection_reused }
    /// Physical connection id that was actually used for this request.
    pub fn connection_id(&self) -> f64 { self.connection_id }
    /// Remote IP address.
    pub fn remote_ip_address(&self) -> Option<&str> { self.remote_ip_address.as_deref() }
    /// Remote port.
    pub fn remote_port(&self) -> Option<i64> { self.remote_port }
    /// Specifies that the request was served from the disk cache.
    pub fn from_disk_cache(&self) -> Option<bool> { self.from_disk_cache }
    /// Specifies that the request was served from the ServiceWorker.
    pub fn from_service_worker(&self) -> Option<bool> { self.from_service_worker }
    /// Specifies that the request was served from the prefetch cache.
    pub fn from_prefetch_cache(&self) -> Option<bool> { self.from_prefetch_cache }
    /// Specifies that the request was served from the prefetch cache.
    pub fn from_early_hints(&self) -> Option<bool> { self.from_early_hints }
    /// Information about how ServiceWorker Static Router API was used. If this
    /// field is set with 'matchedSourceType' field, a matching rule is found.
    /// If this field is set without 'matchedSource', no matching rule is found.
    /// Otherwise, the API is not used.
    pub fn service_worker_router_info(&self) -> Option<&ServiceWorkerRouterInfo> { self.service_worker_router_info.as_ref() }
    /// Total number of bytes received for this request so far.
    pub fn encoded_data_length(&self) -> f64 { self.encoded_data_length }
    /// Timing information for the given request.
    pub fn timing(&self) -> Option<&ResourceTiming> { self.timing.as_ref() }
    /// Response source of response from ServiceWorker.
    pub fn service_worker_response_source(&self) -> Option<&ServiceWorkerResponseSource> { self.service_worker_response_source.as_ref() }
    /// The time at which the returned response was generated.
    pub fn response_time(&self) -> Option<&TimeSinceEpoch> { self.response_time.as_ref() }
    /// Cache Storage Cache Name.
    pub fn cache_storage_cache_name(&self) -> Option<&str> { self.cache_storage_cache_name.as_deref() }
    /// Protocol used to fetch this request.
    pub fn protocol(&self) -> Option<&str> { self.protocol.as_deref() }
    /// The reason why Chrome uses a specific transport protocol for HTTP semantics.
    pub fn alternate_protocol_usage(&self) -> Option<&AlternateProtocolUsage> { self.alternate_protocol_usage.as_ref() }
    /// Security state of the request resource.
    pub fn security_state(&self) -> &crate::security::SecurityState { &self.security_state }
    /// Security details for the request.
    pub fn security_details(&self) -> Option<&SecurityDetails<'a>> { self.security_details.as_ref() }
}


pub struct ResponseBuilder<'a> {
    url: Cow<'a, str>,
    status: i64,
    status_text: Cow<'a, str>,
    headers: Headers,
    headers_text: Option<Cow<'a, str>>,
    mime_type: Cow<'a, str>,
    charset: Cow<'a, str>,
    request_headers: Option<Headers>,
    request_headers_text: Option<Cow<'a, str>>,
    connection_reused: bool,
    connection_id: f64,
    remote_ip_address: Option<Cow<'a, str>>,
    remote_port: Option<i64>,
    from_disk_cache: Option<bool>,
    from_service_worker: Option<bool>,
    from_prefetch_cache: Option<bool>,
    from_early_hints: Option<bool>,
    service_worker_router_info: Option<ServiceWorkerRouterInfo>,
    encoded_data_length: f64,
    timing: Option<ResourceTiming>,
    service_worker_response_source: Option<ServiceWorkerResponseSource>,
    response_time: Option<TimeSinceEpoch>,
    cache_storage_cache_name: Option<Cow<'a, str>>,
    protocol: Option<Cow<'a, str>>,
    alternate_protocol_usage: Option<AlternateProtocolUsage>,
    security_state: crate::security::SecurityState,
    security_details: Option<SecurityDetails<'a>>,
}

impl<'a> ResponseBuilder<'a> {
    /// HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo.
    pub fn headers_text(mut self, headers_text: impl Into<Cow<'a, str>>) -> Self { self.headers_text = Some(headers_text.into()); self }
    /// Refined HTTP request headers that were actually transmitted over the network.
    pub fn request_headers(mut self, request_headers: Headers) -> Self { self.request_headers = Some(request_headers); self }
    /// HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo.
    pub fn request_headers_text(mut self, request_headers_text: impl Into<Cow<'a, str>>) -> Self { self.request_headers_text = Some(request_headers_text.into()); self }
    /// Remote IP address.
    pub fn remote_ip_address(mut self, remote_ip_address: impl Into<Cow<'a, str>>) -> Self { self.remote_ip_address = Some(remote_ip_address.into()); self }
    /// Remote port.
    pub fn remote_port(mut self, remote_port: i64) -> Self { self.remote_port = Some(remote_port); self }
    /// Specifies that the request was served from the disk cache.
    pub fn from_disk_cache(mut self, from_disk_cache: bool) -> Self { self.from_disk_cache = Some(from_disk_cache); self }
    /// Specifies that the request was served from the ServiceWorker.
    pub fn from_service_worker(mut self, from_service_worker: bool) -> Self { self.from_service_worker = Some(from_service_worker); self }
    /// Specifies that the request was served from the prefetch cache.
    pub fn from_prefetch_cache(mut self, from_prefetch_cache: bool) -> Self { self.from_prefetch_cache = Some(from_prefetch_cache); self }
    /// Specifies that the request was served from the prefetch cache.
    pub fn from_early_hints(mut self, from_early_hints: bool) -> Self { self.from_early_hints = Some(from_early_hints); self }
    /// Information about how ServiceWorker Static Router API was used. If this
    /// field is set with 'matchedSourceType' field, a matching rule is found.
    /// If this field is set without 'matchedSource', no matching rule is found.
    /// Otherwise, the API is not used.
    pub fn service_worker_router_info(mut self, service_worker_router_info: ServiceWorkerRouterInfo) -> Self { self.service_worker_router_info = Some(service_worker_router_info); self }
    /// Timing information for the given request.
    pub fn timing(mut self, timing: ResourceTiming) -> Self { self.timing = Some(timing); self }
    /// Response source of response from ServiceWorker.
    pub fn service_worker_response_source(mut self, service_worker_response_source: impl Into<ServiceWorkerResponseSource>) -> Self { self.service_worker_response_source = Some(service_worker_response_source.into()); self }
    /// The time at which the returned response was generated.
    pub fn response_time(mut self, response_time: TimeSinceEpoch) -> Self { self.response_time = Some(response_time); self }
    /// Cache Storage Cache Name.
    pub fn cache_storage_cache_name(mut self, cache_storage_cache_name: impl Into<Cow<'a, str>>) -> Self { self.cache_storage_cache_name = Some(cache_storage_cache_name.into()); self }
    /// Protocol used to fetch this request.
    pub fn protocol(mut self, protocol: impl Into<Cow<'a, str>>) -> Self { self.protocol = Some(protocol.into()); self }
    /// The reason why Chrome uses a specific transport protocol for HTTP semantics.
    pub fn alternate_protocol_usage(mut self, alternate_protocol_usage: impl Into<AlternateProtocolUsage>) -> Self { self.alternate_protocol_usage = Some(alternate_protocol_usage.into()); self }
    /// Security details for the request.
    pub fn security_details(mut self, security_details: SecurityDetails<'a>) -> Self { self.security_details = Some(security_details); self }
    pub fn build(self) -> Response<'a> {
        Response {
            url: self.url,
            status: self.status,
            status_text: self.status_text,
            headers: self.headers,
            headers_text: self.headers_text,
            mime_type: self.mime_type,
            charset: self.charset,
            request_headers: self.request_headers,
            request_headers_text: self.request_headers_text,
            connection_reused: self.connection_reused,
            connection_id: self.connection_id,
            remote_ip_address: self.remote_ip_address,
            remote_port: self.remote_port,
            from_disk_cache: self.from_disk_cache,
            from_service_worker: self.from_service_worker,
            from_prefetch_cache: self.from_prefetch_cache,
            from_early_hints: self.from_early_hints,
            service_worker_router_info: self.service_worker_router_info,
            encoded_data_length: self.encoded_data_length,
            timing: self.timing,
            service_worker_response_source: self.service_worker_response_source,
            response_time: self.response_time,
            cache_storage_cache_name: self.cache_storage_cache_name,
            protocol: self.protocol,
            alternate_protocol_usage: self.alternate_protocol_usage,
            security_state: self.security_state,
            security_details: self.security_details,
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
    /// Creates a builder for this type with the required parameters:
    /// * `headers`: HTTP request headers.
    pub fn builder(headers: Headers) -> WebSocketRequestBuilder {
        WebSocketRequestBuilder {
            headers: headers,
        }
    }
    /// HTTP request headers.
    pub fn headers(&self) -> &Headers { &self.headers }
}


pub struct WebSocketRequestBuilder {
    headers: Headers,
}

impl WebSocketRequestBuilder {
    pub fn build(self) -> WebSocketRequest {
        WebSocketRequest {
            headers: self.headers,
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
    #[serde(rename = "statusText")]
    status_text: Cow<'a, str>,
    /// HTTP response headers.
    headers: Headers,
    /// HTTP response headers text.
    #[serde(skip_serializing_if = "Option::is_none", rename = "headersText")]
    headers_text: Option<Cow<'a, str>>,
    /// HTTP request headers.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    request_headers: Option<Headers>,
    /// HTTP request headers text.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestHeadersText")]
    request_headers_text: Option<Cow<'a, str>>,
}

impl<'a> WebSocketResponse<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `status`: HTTP response status code.
    /// * `status_text`: HTTP response status text.
    /// * `headers`: HTTP response headers.
    pub fn builder(status: i64, status_text: impl Into<Cow<'a, str>>, headers: Headers) -> WebSocketResponseBuilder<'a> {
        WebSocketResponseBuilder {
            status: status,
            status_text: status_text.into(),
            headers: headers,
            headers_text: None,
            request_headers: None,
            request_headers_text: None,
        }
    }
    /// HTTP response status code.
    pub fn status(&self) -> i64 { self.status }
    /// HTTP response status text.
    pub fn status_text(&self) -> &str { self.status_text.as_ref() }
    /// HTTP response headers.
    pub fn headers(&self) -> &Headers { &self.headers }
    /// HTTP response headers text.
    pub fn headers_text(&self) -> Option<&str> { self.headers_text.as_deref() }
    /// HTTP request headers.
    pub fn request_headers(&self) -> Option<&Headers> { self.request_headers.as_ref() }
    /// HTTP request headers text.
    pub fn request_headers_text(&self) -> Option<&str> { self.request_headers_text.as_deref() }
}


pub struct WebSocketResponseBuilder<'a> {
    status: i64,
    status_text: Cow<'a, str>,
    headers: Headers,
    headers_text: Option<Cow<'a, str>>,
    request_headers: Option<Headers>,
    request_headers_text: Option<Cow<'a, str>>,
}

impl<'a> WebSocketResponseBuilder<'a> {
    /// HTTP response headers text.
    pub fn headers_text(mut self, headers_text: impl Into<Cow<'a, str>>) -> Self { self.headers_text = Some(headers_text.into()); self }
    /// HTTP request headers.
    pub fn request_headers(mut self, request_headers: Headers) -> Self { self.request_headers = Some(request_headers); self }
    /// HTTP request headers text.
    pub fn request_headers_text(mut self, request_headers_text: impl Into<Cow<'a, str>>) -> Self { self.request_headers_text = Some(request_headers_text.into()); self }
    pub fn build(self) -> WebSocketResponse<'a> {
        WebSocketResponse {
            status: self.status,
            status_text: self.status_text,
            headers: self.headers,
            headers_text: self.headers_text,
            request_headers: self.request_headers,
            request_headers_text: self.request_headers_text,
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
    #[serde(rename = "payloadData")]
    payload_data: Cow<'a, str>,
}

impl<'a> WebSocketFrame<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `opcode`: WebSocket message opcode.
    /// * `mask`: WebSocket message mask.
    /// * `payload_data`: WebSocket message payload data. If the opcode is 1, this is a text message and payloadData is a UTF-8 string. If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data.
    pub fn builder(opcode: f64, mask: bool, payload_data: impl Into<Cow<'a, str>>) -> WebSocketFrameBuilder<'a> {
        WebSocketFrameBuilder {
            opcode: opcode,
            mask: mask,
            payload_data: payload_data.into(),
        }
    }
    /// WebSocket message opcode.
    pub fn opcode(&self) -> f64 { self.opcode }
    /// WebSocket message mask.
    pub fn mask(&self) -> bool { self.mask }
    /// WebSocket message payload data.
    /// If the opcode is 1, this is a text message and payloadData is a UTF-8 string.
    /// If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data.
    pub fn payload_data(&self) -> &str { self.payload_data.as_ref() }
}


pub struct WebSocketFrameBuilder<'a> {
    opcode: f64,
    mask: bool,
    payload_data: Cow<'a, str>,
}

impl<'a> WebSocketFrameBuilder<'a> {
    pub fn build(self) -> WebSocketFrame<'a> {
        WebSocketFrame {
            opcode: self.opcode,
            mask: self.mask,
            payload_data: self.payload_data,
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
    #[serde(rename = "bodySize")]
    body_size: f64,
}

impl<'a> CachedResource<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: Resource URL. This is the url of the original network request.
    /// * `type_`: Type of this resource.
    /// * `body_size`: Cached response body size.
    pub fn builder(url: impl Into<Cow<'a, str>>, type_: impl Into<ResourceType>, body_size: f64) -> CachedResourceBuilder<'a> {
        CachedResourceBuilder {
            url: url.into(),
            type_: type_.into(),
            response: None,
            body_size: body_size,
        }
    }
    /// Resource URL. This is the url of the original network request.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Type of this resource.
    pub fn type_(&self) -> &ResourceType { &self.type_ }
    /// Cached response data.
    pub fn response(&self) -> Option<&Response<'a>> { self.response.as_ref() }
    /// Cached response body size.
    pub fn body_size(&self) -> f64 { self.body_size }
}


pub struct CachedResourceBuilder<'a> {
    url: Cow<'a, str>,
    type_: ResourceType,
    response: Option<Response<'a>>,
    body_size: f64,
}

impl<'a> CachedResourceBuilder<'a> {
    /// Cached response data.
    pub fn response(mut self, response: Response<'a>) -> Self { self.response = Some(response); self }
    pub fn build(self) -> CachedResource<'a> {
        CachedResource {
            url: self.url,
            type_: self.type_,
            response: self.response,
            body_size: self.body_size,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "lineNumber")]
    line_number: Option<f64>,
    /// Initiator column number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnNumber")]
    column_number: Option<f64>,
    /// Set if another request triggered this request (e.g. preflight).
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestId")]
    request_id: Option<RequestId<'a>>,
}

impl<'a> Initiator<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Type of this initiator.
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> InitiatorBuilder<'a> {
        InitiatorBuilder {
            type_: type_.into(),
            stack: None,
            url: None,
            line_number: None,
            column_number: None,
            request_id: None,
        }
    }
    /// Type of this initiator.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Initiator JavaScript stack trace, set for Script only.
    /// Requires the Debugger domain to be enabled.
    pub fn stack(&self) -> Option<&crate::runtime::StackTrace> { self.stack.as_ref() }
    /// Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// Initiator line number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    pub fn line_number(&self) -> Option<f64> { self.line_number }
    /// Initiator column number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    pub fn column_number(&self) -> Option<f64> { self.column_number }
    /// Set if another request triggered this request (e.g. preflight).
    pub fn request_id(&self) -> Option<&RequestId<'a>> { self.request_id.as_ref() }
}


pub struct InitiatorBuilder<'a> {
    type_: Cow<'a, str>,
    stack: Option<crate::runtime::StackTrace>,
    url: Option<Cow<'a, str>>,
    line_number: Option<f64>,
    column_number: Option<f64>,
    request_id: Option<RequestId<'a>>,
}

impl<'a> InitiatorBuilder<'a> {
    /// Initiator JavaScript stack trace, set for Script only.
    /// Requires the Debugger domain to be enabled.
    pub fn stack(mut self, stack: crate::runtime::StackTrace) -> Self { self.stack = Some(stack); self }
    /// Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Initiator line number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    pub fn line_number(mut self, line_number: f64) -> Self { self.line_number = Some(line_number); self }
    /// Initiator column number, set for Parser type or for Script type (when script is importing
    /// module) (0-based).
    pub fn column_number(mut self, column_number: f64) -> Self { self.column_number = Some(column_number); self }
    /// Set if another request triggered this request (e.g. preflight).
    pub fn request_id(mut self, request_id: impl Into<RequestId<'a>>) -> Self { self.request_id = Some(request_id.into()); self }
    pub fn build(self) -> Initiator<'a> {
        Initiator {
            type_: self.type_,
            stack: self.stack,
            url: self.url,
            line_number: self.line_number,
            column_number: self.column_number,
            request_id: self.request_id,
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
    #[serde(rename = "topLevelSite")]
    top_level_site: Cow<'a, str>,
    /// Indicates if the cookie has any ancestors that are cross-site to the topLevelSite.
    #[serde(rename = "hasCrossSiteAncestor")]
    has_cross_site_ancestor: bool,
}

impl<'a> CookiePartitionKey<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `top_level_site`: The site of the top-level URL the browser was visiting at the start of the request to the endpoint that set the cookie.
    /// * `has_cross_site_ancestor`: Indicates if the cookie has any ancestors that are cross-site to the topLevelSite.
    pub fn builder(top_level_site: impl Into<Cow<'a, str>>, has_cross_site_ancestor: bool) -> CookiePartitionKeyBuilder<'a> {
        CookiePartitionKeyBuilder {
            top_level_site: top_level_site.into(),
            has_cross_site_ancestor: has_cross_site_ancestor,
        }
    }
    /// The site of the top-level URL the browser was visiting at the start
    /// of the request to the endpoint that set the cookie.
    pub fn top_level_site(&self) -> &str { self.top_level_site.as_ref() }
    /// Indicates if the cookie has any ancestors that are cross-site to the topLevelSite.
    pub fn has_cross_site_ancestor(&self) -> bool { self.has_cross_site_ancestor }
}


pub struct CookiePartitionKeyBuilder<'a> {
    top_level_site: Cow<'a, str>,
    has_cross_site_ancestor: bool,
}

impl<'a> CookiePartitionKeyBuilder<'a> {
    pub fn build(self) -> CookiePartitionKey<'a> {
        CookiePartitionKey {
            top_level_site: self.top_level_site,
            has_cross_site_ancestor: self.has_cross_site_ancestor,
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
    #[serde(rename = "httpOnly")]
    http_only: bool,
    /// True if cookie is secure.
    secure: bool,
    /// True in case of session cookie.
    session: bool,
    /// Cookie SameSite type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
    same_site: Option<CookieSameSite>,
    /// Cookie Priority
    priority: CookiePriority,
    /// Cookie source scheme type.
    #[serde(rename = "sourceScheme")]
    source_scheme: CookieSourceScheme,
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    #[serde(rename = "sourcePort")]
    source_port: i64,
    /// Cookie partition key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKey")]
    partition_key: Option<CookiePartitionKey<'a>>,
    /// True if cookie partition key is opaque.
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKeyOpaque")]
    partition_key_opaque: Option<bool>,
}

impl<'a> Cookie<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Cookie name.
    /// * `value`: Cookie value.
    /// * `domain`: Cookie domain.
    /// * `path`: Cookie path.
    /// * `expires`: Cookie expiration date as the number of seconds since the UNIX epoch. The value is set to -1 if the expiry date is not set. The value can be null for values that cannot be represented in JSON (±Inf).
    /// * `size`: Cookie size.
    /// * `http_only`: True if cookie is http-only.
    /// * `secure`: True if cookie is secure.
    /// * `session`: True in case of session cookie.
    /// * `priority`: Cookie Priority
    /// * `source_scheme`: Cookie source scheme type.
    /// * `source_port`: Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port. An unspecified port value allows protocol clients to emulate legacy cookie scope for the port. This is a temporary ability and it will be removed in the future.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>, domain: impl Into<Cow<'a, str>>, path: impl Into<Cow<'a, str>>, expires: f64, size: u64, http_only: bool, secure: bool, session: bool, priority: impl Into<CookiePriority>, source_scheme: impl Into<CookieSourceScheme>, source_port: i64) -> CookieBuilder<'a> {
        CookieBuilder {
            name: name.into(),
            value: value.into(),
            domain: domain.into(),
            path: path.into(),
            expires: expires,
            size: size,
            http_only: http_only,
            secure: secure,
            session: session,
            same_site: None,
            priority: priority.into(),
            source_scheme: source_scheme.into(),
            source_port: source_port,
            partition_key: None,
            partition_key_opaque: None,
        }
    }
    /// Cookie name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Cookie value.
    pub fn value(&self) -> &str { self.value.as_ref() }
    /// Cookie domain.
    pub fn domain(&self) -> &str { self.domain.as_ref() }
    /// Cookie path.
    pub fn path(&self) -> &str { self.path.as_ref() }
    /// Cookie expiration date as the number of seconds since the UNIX epoch.
    /// The value is set to -1 if the expiry date is not set.
    /// The value can be null for values that cannot be represented in
    /// JSON (±Inf).
    pub fn expires(&self) -> f64 { self.expires }
    /// Cookie size.
    pub fn size(&self) -> u64 { self.size }
    /// True if cookie is http-only.
    pub fn http_only(&self) -> bool { self.http_only }
    /// True if cookie is secure.
    pub fn secure(&self) -> bool { self.secure }
    /// True in case of session cookie.
    pub fn session(&self) -> bool { self.session }
    /// Cookie SameSite type.
    pub fn same_site(&self) -> Option<&CookieSameSite> { self.same_site.as_ref() }
    /// Cookie Priority
    pub fn priority(&self) -> &CookiePriority { &self.priority }
    /// Cookie source scheme type.
    pub fn source_scheme(&self) -> &CookieSourceScheme { &self.source_scheme }
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    pub fn source_port(&self) -> i64 { self.source_port }
    /// Cookie partition key.
    pub fn partition_key(&self) -> Option<&CookiePartitionKey<'a>> { self.partition_key.as_ref() }
    /// True if cookie partition key is opaque.
    pub fn partition_key_opaque(&self) -> Option<bool> { self.partition_key_opaque }
}


pub struct CookieBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    domain: Cow<'a, str>,
    path: Cow<'a, str>,
    expires: f64,
    size: u64,
    http_only: bool,
    secure: bool,
    session: bool,
    same_site: Option<CookieSameSite>,
    priority: CookiePriority,
    source_scheme: CookieSourceScheme,
    source_port: i64,
    partition_key: Option<CookiePartitionKey<'a>>,
    partition_key_opaque: Option<bool>,
}

impl<'a> CookieBuilder<'a> {
    /// Cookie SameSite type.
    pub fn same_site(mut self, same_site: impl Into<CookieSameSite>) -> Self { self.same_site = Some(same_site.into()); self }
    /// Cookie partition key.
    pub fn partition_key(mut self, partition_key: CookiePartitionKey<'a>) -> Self { self.partition_key = Some(partition_key); self }
    /// True if cookie partition key is opaque.
    pub fn partition_key_opaque(mut self, partition_key_opaque: bool) -> Self { self.partition_key_opaque = Some(partition_key_opaque); self }
    pub fn build(self) -> Cookie<'a> {
        Cookie {
            name: self.name,
            value: self.value,
            domain: self.domain,
            path: self.path,
            expires: self.expires,
            size: self.size,
            http_only: self.http_only,
            secure: self.secure,
            session: self.session,
            same_site: self.same_site,
            priority: self.priority,
            source_scheme: self.source_scheme,
            source_port: self.source_port,
            partition_key: self.partition_key,
            partition_key_opaque: self.partition_key_opaque,
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
    #[serde(rename = "blockedReasons")]
    blocked_reasons: Vec<SetCookieBlockedReason>,
    /// The string representing this individual cookie as it would appear in the header.
    /// This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.
    #[serde(rename = "cookieLine")]
    cookie_line: Cow<'a, str>,
    /// The cookie object which represents the cookie which was not stored. It is optional because
    /// sometimes complete cookie information is not available, such as in the case of parsing
    /// errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie: Option<Cookie<'a>>,
}

impl<'a> BlockedSetCookieWithReason<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `blocked_reasons`: The reason(s) this cookie was blocked.
    /// * `cookie_line`: The string representing this individual cookie as it would appear in the header. This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.
    pub fn builder(blocked_reasons: Vec<SetCookieBlockedReason>, cookie_line: impl Into<Cow<'a, str>>) -> BlockedSetCookieWithReasonBuilder<'a> {
        BlockedSetCookieWithReasonBuilder {
            blocked_reasons: blocked_reasons,
            cookie_line: cookie_line.into(),
            cookie: None,
        }
    }
    /// The reason(s) this cookie was blocked.
    pub fn blocked_reasons(&self) -> &[SetCookieBlockedReason] { &self.blocked_reasons }
    /// The string representing this individual cookie as it would appear in the header.
    /// This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.
    pub fn cookie_line(&self) -> &str { self.cookie_line.as_ref() }
    /// The cookie object which represents the cookie which was not stored. It is optional because
    /// sometimes complete cookie information is not available, such as in the case of parsing
    /// errors.
    pub fn cookie(&self) -> Option<&Cookie<'a>> { self.cookie.as_ref() }
}


pub struct BlockedSetCookieWithReasonBuilder<'a> {
    blocked_reasons: Vec<SetCookieBlockedReason>,
    cookie_line: Cow<'a, str>,
    cookie: Option<Cookie<'a>>,
}

impl<'a> BlockedSetCookieWithReasonBuilder<'a> {
    /// The cookie object which represents the cookie which was not stored. It is optional because
    /// sometimes complete cookie information is not available, such as in the case of parsing
    /// errors.
    pub fn cookie(mut self, cookie: Cookie<'a>) -> Self { self.cookie = Some(cookie); self }
    pub fn build(self) -> BlockedSetCookieWithReason<'a> {
        BlockedSetCookieWithReason {
            blocked_reasons: self.blocked_reasons,
            cookie_line: self.cookie_line,
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
    #[serde(rename = "exemptionReason")]
    exemption_reason: CookieExemptionReason,
    /// The string representing this individual cookie as it would appear in the header.
    #[serde(rename = "cookieLine")]
    cookie_line: Cow<'a, str>,
    /// The cookie object representing the cookie.
    cookie: Cookie<'a>,
}

impl<'a> ExemptedSetCookieWithReason<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `exemption_reason`: The reason the cookie was exempted.
    /// * `cookie_line`: The string representing this individual cookie as it would appear in the header.
    /// * `cookie`: The cookie object representing the cookie.
    pub fn builder(exemption_reason: impl Into<CookieExemptionReason>, cookie_line: impl Into<Cow<'a, str>>, cookie: Cookie<'a>) -> ExemptedSetCookieWithReasonBuilder<'a> {
        ExemptedSetCookieWithReasonBuilder {
            exemption_reason: exemption_reason.into(),
            cookie_line: cookie_line.into(),
            cookie: cookie,
        }
    }
    /// The reason the cookie was exempted.
    pub fn exemption_reason(&self) -> &CookieExemptionReason { &self.exemption_reason }
    /// The string representing this individual cookie as it would appear in the header.
    pub fn cookie_line(&self) -> &str { self.cookie_line.as_ref() }
    /// The cookie object representing the cookie.
    pub fn cookie(&self) -> &Cookie<'a> { &self.cookie }
}


pub struct ExemptedSetCookieWithReasonBuilder<'a> {
    exemption_reason: CookieExemptionReason,
    cookie_line: Cow<'a, str>,
    cookie: Cookie<'a>,
}

impl<'a> ExemptedSetCookieWithReasonBuilder<'a> {
    pub fn build(self) -> ExemptedSetCookieWithReason<'a> {
        ExemptedSetCookieWithReason {
            exemption_reason: self.exemption_reason,
            cookie_line: self.cookie_line,
            cookie: self.cookie,
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
    #[serde(rename = "blockedReasons")]
    blocked_reasons: Vec<CookieBlockedReason>,
    /// The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could
    /// only have at most one exemption reason.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exemptionReason")]
    exemption_reason: Option<CookieExemptionReason>,
}

impl<'a> AssociatedCookie<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cookie`: The cookie object representing the cookie which was not sent.
    /// * `blocked_reasons`: The reason(s) the cookie was blocked. If empty means the cookie is included.
    pub fn builder(cookie: Cookie<'a>, blocked_reasons: Vec<CookieBlockedReason>) -> AssociatedCookieBuilder<'a> {
        AssociatedCookieBuilder {
            cookie: cookie,
            blocked_reasons: blocked_reasons,
            exemption_reason: None,
        }
    }
    /// The cookie object representing the cookie which was not sent.
    pub fn cookie(&self) -> &Cookie<'a> { &self.cookie }
    /// The reason(s) the cookie was blocked. If empty means the cookie is included.
    pub fn blocked_reasons(&self) -> &[CookieBlockedReason] { &self.blocked_reasons }
    /// The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could
    /// only have at most one exemption reason.
    pub fn exemption_reason(&self) -> Option<&CookieExemptionReason> { self.exemption_reason.as_ref() }
}


pub struct AssociatedCookieBuilder<'a> {
    cookie: Cookie<'a>,
    blocked_reasons: Vec<CookieBlockedReason>,
    exemption_reason: Option<CookieExemptionReason>,
}

impl<'a> AssociatedCookieBuilder<'a> {
    /// The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could
    /// only have at most one exemption reason.
    pub fn exemption_reason(mut self, exemption_reason: impl Into<CookieExemptionReason>) -> Self { self.exemption_reason = Some(exemption_reason.into()); self }
    pub fn build(self) -> AssociatedCookie<'a> {
        AssociatedCookie {
            cookie: self.cookie,
            blocked_reasons: self.blocked_reasons,
            exemption_reason: self.exemption_reason,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "httpOnly")]
    http_only: Option<bool>,
    /// Cookie SameSite type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
    same_site: Option<CookieSameSite>,
    /// Cookie expiration date, session cookie if not set
    #[serde(skip_serializing_if = "Option::is_none")]
    expires: Option<TimeSinceEpoch>,
    /// Cookie Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<CookiePriority>,
    /// Cookie source scheme type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceScheme")]
    source_scheme: Option<CookieSourceScheme>,
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourcePort")]
    source_port: Option<i64>,
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKey")]
    partition_key: Option<CookiePartitionKey<'a>>,
}

impl<'a> CookieParam<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Cookie name.
    /// * `value`: Cookie value.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> CookieParamBuilder<'a> {
        CookieParamBuilder {
            name: name.into(),
            value: value.into(),
            url: None,
            domain: None,
            path: None,
            secure: None,
            http_only: None,
            same_site: None,
            expires: None,
            priority: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        }
    }
    /// Cookie name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Cookie value.
    pub fn value(&self) -> &str { self.value.as_ref() }
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// Cookie domain.
    pub fn domain(&self) -> Option<&str> { self.domain.as_deref() }
    /// Cookie path.
    pub fn path(&self) -> Option<&str> { self.path.as_deref() }
    /// True if cookie is secure.
    pub fn secure(&self) -> Option<bool> { self.secure }
    /// True if cookie is http-only.
    pub fn http_only(&self) -> Option<bool> { self.http_only }
    /// Cookie SameSite type.
    pub fn same_site(&self) -> Option<&CookieSameSite> { self.same_site.as_ref() }
    /// Cookie expiration date, session cookie if not set
    pub fn expires(&self) -> Option<&TimeSinceEpoch> { self.expires.as_ref() }
    /// Cookie Priority.
    pub fn priority(&self) -> Option<&CookiePriority> { self.priority.as_ref() }
    /// Cookie source scheme type.
    pub fn source_scheme(&self) -> Option<&CookieSourceScheme> { self.source_scheme.as_ref() }
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    pub fn source_port(&self) -> Option<i64> { self.source_port }
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    pub fn partition_key(&self) -> Option<&CookiePartitionKey<'a>> { self.partition_key.as_ref() }
}


pub struct CookieParamBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    url: Option<Cow<'a, str>>,
    domain: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    secure: Option<bool>,
    http_only: Option<bool>,
    same_site: Option<CookieSameSite>,
    expires: Option<TimeSinceEpoch>,
    priority: Option<CookiePriority>,
    source_scheme: Option<CookieSourceScheme>,
    source_port: Option<i64>,
    partition_key: Option<CookiePartitionKey<'a>>,
}

impl<'a> CookieParamBuilder<'a> {
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
    pub fn http_only(mut self, http_only: bool) -> Self { self.http_only = Some(http_only); self }
    /// Cookie SameSite type.
    pub fn same_site(mut self, same_site: impl Into<CookieSameSite>) -> Self { self.same_site = Some(same_site.into()); self }
    /// Cookie expiration date, session cookie if not set
    pub fn expires(mut self, expires: TimeSinceEpoch) -> Self { self.expires = Some(expires); self }
    /// Cookie Priority.
    pub fn priority(mut self, priority: impl Into<CookiePriority>) -> Self { self.priority = Some(priority.into()); self }
    /// Cookie source scheme type.
    pub fn source_scheme(mut self, source_scheme: impl Into<CookieSourceScheme>) -> Self { self.source_scheme = Some(source_scheme.into()); self }
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    pub fn source_port(mut self, source_port: i64) -> Self { self.source_port = Some(source_port); self }
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    pub fn partition_key(mut self, partition_key: CookiePartitionKey<'a>) -> Self { self.partition_key = Some(partition_key); self }
    pub fn build(self) -> CookieParam<'a> {
        CookieParam {
            name: self.name,
            value: self.value,
            url: self.url,
            domain: self.domain,
            path: self.path,
            secure: self.secure,
            http_only: self.http_only,
            same_site: self.same_site,
            expires: self.expires,
            priority: self.priority,
            source_scheme: self.source_scheme,
            source_port: self.source_port,
            partition_key: self.partition_key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Origin of the challenger.
    /// * `scheme`: The authentication scheme used, such as basic or digest
    /// * `realm`: The realm of the challenge. May be empty.
    pub fn builder(origin: impl Into<Cow<'a, str>>, scheme: impl Into<Cow<'a, str>>, realm: impl Into<Cow<'a, str>>) -> AuthChallengeBuilder<'a> {
        AuthChallengeBuilder {
            source: None,
            origin: origin.into(),
            scheme: scheme.into(),
            realm: realm.into(),
        }
    }
    /// Source of the authentication challenge.
    pub fn source(&self) -> Option<&str> { self.source.as_deref() }
    /// Origin of the challenger.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    /// The authentication scheme used, such as basic or digest
    pub fn scheme(&self) -> &str { self.scheme.as_ref() }
    /// The realm of the challenge. May be empty.
    pub fn realm(&self) -> &str { self.realm.as_ref() }
}


pub struct AuthChallengeBuilder<'a> {
    source: Option<Cow<'a, str>>,
    origin: Cow<'a, str>,
    scheme: Cow<'a, str>,
    realm: Cow<'a, str>,
}

impl<'a> AuthChallengeBuilder<'a> {
    /// Source of the authentication challenge.
    pub fn source(mut self, source: impl Into<Cow<'a, str>>) -> Self { self.source = Some(source.into()); self }
    pub fn build(self) -> AuthChallenge<'a> {
        AuthChallenge {
            source: self.source,
            origin: self.origin,
            scheme: self.scheme,
            realm: self.realm,
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
    /// Creates a builder for this type with the required parameters:
    /// * `response`: The decision on what to do in response to the authorization challenge.  Default means deferring to the default behavior of the net stack, which will likely either the Cancel authentication or display a popup dialog box.
    pub fn builder(response: impl Into<Cow<'a, str>>) -> AuthChallengeResponseBuilder<'a> {
        AuthChallengeResponseBuilder {
            response: response.into(),
            username: None,
            password: None,
        }
    }
    /// The decision on what to do in response to the authorization challenge.  Default means
    /// deferring to the default behavior of the net stack, which will likely either the Cancel
    /// authentication or display a popup dialog box.
    pub fn response(&self) -> &str { self.response.as_ref() }
    /// The username to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    pub fn username(&self) -> Option<&str> { self.username.as_deref() }
    /// The password to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    pub fn password(&self) -> Option<&str> { self.password.as_deref() }
}


pub struct AuthChallengeResponseBuilder<'a> {
    response: Cow<'a, str>,
    username: Option<Cow<'a, str>>,
    password: Option<Cow<'a, str>>,
}

impl<'a> AuthChallengeResponseBuilder<'a> {
    /// The username to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    pub fn username(mut self, username: impl Into<Cow<'a, str>>) -> Self { self.username = Some(username.into()); self }
    /// The password to provide, possibly empty. Should only be set if response is
    /// ProvideCredentials.
    pub fn password(mut self, password: impl Into<Cow<'a, str>>) -> Self { self.password = Some(password.into()); self }
    pub fn build(self) -> AuthChallengeResponse<'a> {
        AuthChallengeResponse {
            response: self.response,
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
    /// Wildcards (''*'' -\> zero or more, ''?'' -\> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlPattern")]
    url_pattern: Option<Cow<'a, str>>,
    /// If set, only requests for matching resource types will be intercepted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType")]
    resource_type: Option<ResourceType>,
    /// Stage at which to begin intercepting requests. Default is Request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "interceptionStage")]
    interception_stage: Option<InterceptionStage>,
}

impl<'a> RequestPattern<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> RequestPatternBuilder<'a> {
        RequestPatternBuilder {
            url_pattern: None,
            resource_type: None,
            interception_stage: None,
        }
    }
    /// Wildcards (''*'' -\> zero or more, ''?'' -\> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    pub fn url_pattern(&self) -> Option<&str> { self.url_pattern.as_deref() }
    /// If set, only requests for matching resource types will be intercepted.
    pub fn resource_type(&self) -> Option<&ResourceType> { self.resource_type.as_ref() }
    /// Stage at which to begin intercepting requests. Default is Request.
    pub fn interception_stage(&self) -> Option<&InterceptionStage> { self.interception_stage.as_ref() }
}

#[derive(Default)]
pub struct RequestPatternBuilder<'a> {
    url_pattern: Option<Cow<'a, str>>,
    resource_type: Option<ResourceType>,
    interception_stage: Option<InterceptionStage>,
}

impl<'a> RequestPatternBuilder<'a> {
    /// Wildcards (''*'' -\> zero or more, ''?'' -\> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    pub fn url_pattern(mut self, url_pattern: impl Into<Cow<'a, str>>) -> Self { self.url_pattern = Some(url_pattern.into()); self }
    /// If set, only requests for matching resource types will be intercepted.
    pub fn resource_type(mut self, resource_type: impl Into<ResourceType>) -> Self { self.resource_type = Some(resource_type.into()); self }
    /// Stage at which to begin intercepting requests. Default is Request.
    pub fn interception_stage(mut self, interception_stage: impl Into<InterceptionStage>) -> Self { self.interception_stage = Some(interception_stage.into()); self }
    pub fn build(self) -> RequestPattern<'a> {
        RequestPattern {
            url_pattern: self.url_pattern,
            resource_type: self.resource_type,
            interception_stage: self.interception_stage,
        }
    }
}

/// Information about a signed exchange signature.
/// <https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1>

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
    #[serde(skip_serializing_if = "Option::is_none", rename = "certUrl")]
    cert_url: Option<Cow<'a, str>>,
    /// The hex string of signed exchange signature cert sha256.
    #[serde(skip_serializing_if = "Option::is_none", rename = "certSha256")]
    cert_sha256: Option<Cow<'a, str>>,
    /// Signed exchange signature validity Url.
    #[serde(rename = "validityUrl")]
    validity_url: Cow<'a, str>,
    /// Signed exchange signature date.
    date: i64,
    /// Signed exchange signature expires.
    expires: i64,
    /// The encoded certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    certificates: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SignedExchangeSignature<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `label`: Signed exchange signature label.
    /// * `signature`: The hex string of signed exchange signature.
    /// * `integrity`: Signed exchange signature integrity.
    /// * `validity_url`: Signed exchange signature validity Url.
    /// * `date`: Signed exchange signature date.
    /// * `expires`: Signed exchange signature expires.
    pub fn builder(label: impl Into<Cow<'a, str>>, signature: impl Into<Cow<'a, str>>, integrity: impl Into<Cow<'a, str>>, validity_url: impl Into<Cow<'a, str>>, date: i64, expires: i64) -> SignedExchangeSignatureBuilder<'a> {
        SignedExchangeSignatureBuilder {
            label: label.into(),
            signature: signature.into(),
            integrity: integrity.into(),
            cert_url: None,
            cert_sha256: None,
            validity_url: validity_url.into(),
            date: date,
            expires: expires,
            certificates: None,
        }
    }
    /// Signed exchange signature label.
    pub fn label(&self) -> &str { self.label.as_ref() }
    /// The hex string of signed exchange signature.
    pub fn signature(&self) -> &str { self.signature.as_ref() }
    /// Signed exchange signature integrity.
    pub fn integrity(&self) -> &str { self.integrity.as_ref() }
    /// Signed exchange signature cert Url.
    pub fn cert_url(&self) -> Option<&str> { self.cert_url.as_deref() }
    /// The hex string of signed exchange signature cert sha256.
    pub fn cert_sha256(&self) -> Option<&str> { self.cert_sha256.as_deref() }
    /// Signed exchange signature validity Url.
    pub fn validity_url(&self) -> &str { self.validity_url.as_ref() }
    /// Signed exchange signature date.
    pub fn date(&self) -> i64 { self.date }
    /// Signed exchange signature expires.
    pub fn expires(&self) -> i64 { self.expires }
    /// The encoded certificates.
    pub fn certificates(&self) -> Option<&[Cow<'a, str>]> { self.certificates.as_deref() }
}


pub struct SignedExchangeSignatureBuilder<'a> {
    label: Cow<'a, str>,
    signature: Cow<'a, str>,
    integrity: Cow<'a, str>,
    cert_url: Option<Cow<'a, str>>,
    cert_sha256: Option<Cow<'a, str>>,
    validity_url: Cow<'a, str>,
    date: i64,
    expires: i64,
    certificates: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SignedExchangeSignatureBuilder<'a> {
    /// Signed exchange signature cert Url.
    pub fn cert_url(mut self, cert_url: impl Into<Cow<'a, str>>) -> Self { self.cert_url = Some(cert_url.into()); self }
    /// The hex string of signed exchange signature cert sha256.
    pub fn cert_sha256(mut self, cert_sha256: impl Into<Cow<'a, str>>) -> Self { self.cert_sha256 = Some(cert_sha256.into()); self }
    /// The encoded certificates.
    pub fn certificates(mut self, certificates: Vec<Cow<'a, str>>) -> Self { self.certificates = Some(certificates); self }
    pub fn build(self) -> SignedExchangeSignature<'a> {
        SignedExchangeSignature {
            label: self.label,
            signature: self.signature,
            integrity: self.integrity,
            cert_url: self.cert_url,
            cert_sha256: self.cert_sha256,
            validity_url: self.validity_url,
            date: self.date,
            expires: self.expires,
            certificates: self.certificates,
        }
    }
}

/// Information about a signed exchange header.
/// <https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeHeader<'a> {
    /// Signed exchange request URL.
    #[serde(rename = "requestUrl")]
    request_url: Cow<'a, str>,
    /// Signed exchange response code.
    #[serde(rename = "responseCode")]
    response_code: i64,
    /// Signed exchange response headers.
    #[serde(rename = "responseHeaders")]
    response_headers: Headers,
    /// Signed exchange response signature.
    signatures: Vec<SignedExchangeSignature<'a>>,
    /// Signed exchange header integrity hash in the form of 'sha256-\<base64-hash-value\>'.
    #[serde(rename = "headerIntegrity")]
    header_integrity: Cow<'a, str>,
}

impl<'a> SignedExchangeHeader<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_url`: Signed exchange request URL.
    /// * `response_code`: Signed exchange response code.
    /// * `response_headers`: Signed exchange response headers.
    /// * `signatures`: Signed exchange response signature.
    /// * `header_integrity`: Signed exchange header integrity hash in the form of `sha256-\<base64-hash-value\>`.
    pub fn builder(request_url: impl Into<Cow<'a, str>>, response_code: i64, response_headers: Headers, signatures: Vec<SignedExchangeSignature<'a>>, header_integrity: impl Into<Cow<'a, str>>) -> SignedExchangeHeaderBuilder<'a> {
        SignedExchangeHeaderBuilder {
            request_url: request_url.into(),
            response_code: response_code,
            response_headers: response_headers,
            signatures: signatures,
            header_integrity: header_integrity.into(),
        }
    }
    /// Signed exchange request URL.
    pub fn request_url(&self) -> &str { self.request_url.as_ref() }
    /// Signed exchange response code.
    pub fn response_code(&self) -> i64 { self.response_code }
    /// Signed exchange response headers.
    pub fn response_headers(&self) -> &Headers { &self.response_headers }
    /// Signed exchange response signature.
    pub fn signatures(&self) -> &[SignedExchangeSignature<'a>] { &self.signatures }
    /// Signed exchange header integrity hash in the form of 'sha256-\<base64-hash-value\>'.
    pub fn header_integrity(&self) -> &str { self.header_integrity.as_ref() }
}


pub struct SignedExchangeHeaderBuilder<'a> {
    request_url: Cow<'a, str>,
    response_code: i64,
    response_headers: Headers,
    signatures: Vec<SignedExchangeSignature<'a>>,
    header_integrity: Cow<'a, str>,
}

impl<'a> SignedExchangeHeaderBuilder<'a> {
    pub fn build(self) -> SignedExchangeHeader<'a> {
        SignedExchangeHeader {
            request_url: self.request_url,
            response_code: self.response_code,
            response_headers: self.response_headers,
            signatures: self.signatures,
            header_integrity: self.header_integrity,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "signatureIndex")]
    signature_index: Option<u64>,
    /// The field which caused the error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorField")]
    error_field: Option<SignedExchangeErrorField>,
}

impl<'a> SignedExchangeError<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `message`: Error message.
    pub fn builder(message: impl Into<Cow<'a, str>>) -> SignedExchangeErrorBuilder<'a> {
        SignedExchangeErrorBuilder {
            message: message.into(),
            signature_index: None,
            error_field: None,
        }
    }
    /// Error message.
    pub fn message(&self) -> &str { self.message.as_ref() }
    /// The index of the signature which caused the error.
    pub fn signature_index(&self) -> Option<u64> { self.signature_index }
    /// The field which caused the error.
    pub fn error_field(&self) -> Option<&SignedExchangeErrorField> { self.error_field.as_ref() }
}


pub struct SignedExchangeErrorBuilder<'a> {
    message: Cow<'a, str>,
    signature_index: Option<u64>,
    error_field: Option<SignedExchangeErrorField>,
}

impl<'a> SignedExchangeErrorBuilder<'a> {
    /// The index of the signature which caused the error.
    pub fn signature_index(mut self, signature_index: u64) -> Self { self.signature_index = Some(signature_index); self }
    /// The field which caused the error.
    pub fn error_field(mut self, error_field: impl Into<SignedExchangeErrorField>) -> Self { self.error_field = Some(error_field.into()); self }
    pub fn build(self) -> SignedExchangeError<'a> {
        SignedExchangeError {
            message: self.message,
            signature_index: self.signature_index,
            error_field: self.error_field,
        }
    }
}

/// Information about a signed exchange response.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeInfo<'a> {
    /// The outer response of signed HTTP exchange which was received from network.
    #[serde(rename = "outerResponse")]
    outer_response: Response<'a>,
    /// Whether network response for the signed exchange was accompanied by
    /// extra headers.
    #[serde(rename = "hasExtraInfo")]
    has_extra_info: bool,
    /// Information about the signed exchange header.
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<SignedExchangeHeader<'a>>,
    /// Security details for the signed exchange header.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityDetails")]
    security_details: Option<SecurityDetails<'a>>,
    /// Errors occurred while handling the signed exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<SignedExchangeError<'a>>>,
}

impl<'a> SignedExchangeInfo<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `outer_response`: The outer response of signed HTTP exchange which was received from network.
    /// * `has_extra_info`: Whether network response for the signed exchange was accompanied by extra headers.
    pub fn builder(outer_response: Response<'a>, has_extra_info: bool) -> SignedExchangeInfoBuilder<'a> {
        SignedExchangeInfoBuilder {
            outer_response: outer_response,
            has_extra_info: has_extra_info,
            header: None,
            security_details: None,
            errors: None,
        }
    }
    /// The outer response of signed HTTP exchange which was received from network.
    pub fn outer_response(&self) -> &Response<'a> { &self.outer_response }
    /// Whether network response for the signed exchange was accompanied by
    /// extra headers.
    pub fn has_extra_info(&self) -> bool { self.has_extra_info }
    /// Information about the signed exchange header.
    pub fn header(&self) -> Option<&SignedExchangeHeader<'a>> { self.header.as_ref() }
    /// Security details for the signed exchange header.
    pub fn security_details(&self) -> Option<&SecurityDetails<'a>> { self.security_details.as_ref() }
    /// Errors occurred while handling the signed exchange.
    pub fn errors(&self) -> Option<&[SignedExchangeError<'a>]> { self.errors.as_deref() }
}


pub struct SignedExchangeInfoBuilder<'a> {
    outer_response: Response<'a>,
    has_extra_info: bool,
    header: Option<SignedExchangeHeader<'a>>,
    security_details: Option<SecurityDetails<'a>>,
    errors: Option<Vec<SignedExchangeError<'a>>>,
}

impl<'a> SignedExchangeInfoBuilder<'a> {
    /// Information about the signed exchange header.
    pub fn header(mut self, header: SignedExchangeHeader<'a>) -> Self { self.header = Some(header); self }
    /// Security details for the signed exchange header.
    pub fn security_details(mut self, security_details: SecurityDetails<'a>) -> Self { self.security_details = Some(security_details); self }
    /// Errors occurred while handling the signed exchange.
    pub fn errors(mut self, errors: Vec<SignedExchangeError<'a>>) -> Self { self.errors = Some(errors); self }
    pub fn build(self) -> SignedExchangeInfo<'a> {
        SignedExchangeInfo {
            outer_response: self.outer_response,
            has_extra_info: self.has_extra_info,
            header: self.header,
            security_details: self.security_details,
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
    /// syntax (<https://urlpattern.spec.whatwg.org/>) and must be absolute. If the pattern is empty, all requests are
    /// matched (including p2p connections).
    #[serde(rename = "urlPattern")]
    url_pattern: Cow<'a, str>,
    /// Minimum latency from request sent to response headers received (ms).
    latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    #[serde(rename = "downloadThroughput")]
    download_throughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    #[serde(rename = "uploadThroughput")]
    upload_throughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectionType")]
    connection_type: Option<ConnectionType>,
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetLoss")]
    packet_loss: Option<f64>,
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetQueueLength")]
    packet_queue_length: Option<u64>,
    /// WebRTC packetReordering feature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetReordering")]
    packet_reordering: Option<bool>,
    /// True to emulate internet disconnection.
    #[serde(skip_serializing_if = "Option::is_none")]
    offline: Option<bool>,
}

impl<'a> NetworkConditions<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url_pattern`: Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string syntax (<https://urlpattern.spec.whatwg.org/>) and must be absolute. If the pattern is empty, all requests are matched (including p2p connections).
    /// * `latency`: Minimum latency from request sent to response headers received (ms).
    /// * `download_throughput`: Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    /// * `upload_throughput`: Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub fn builder(url_pattern: impl Into<Cow<'a, str>>, latency: f64, download_throughput: f64, upload_throughput: f64) -> NetworkConditionsBuilder<'a> {
        NetworkConditionsBuilder {
            url_pattern: url_pattern.into(),
            latency: latency,
            download_throughput: download_throughput,
            upload_throughput: upload_throughput,
            connection_type: None,
            packet_loss: None,
            packet_queue_length: None,
            packet_reordering: None,
            offline: None,
        }
    }
    /// Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string
    /// syntax (<https://urlpattern.spec.whatwg.org/>) and must be absolute. If the pattern is empty, all requests are
    /// matched (including p2p connections).
    pub fn url_pattern(&self) -> &str { self.url_pattern.as_ref() }
    /// Minimum latency from request sent to response headers received (ms).
    pub fn latency(&self) -> f64 { self.latency }
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    pub fn download_throughput(&self) -> f64 { self.download_throughput }
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub fn upload_throughput(&self) -> f64 { self.upload_throughput }
    /// Connection type if known.
    pub fn connection_type(&self) -> Option<&ConnectionType> { self.connection_type.as_ref() }
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    pub fn packet_loss(&self) -> Option<f64> { self.packet_loss }
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    pub fn packet_queue_length(&self) -> Option<u64> { self.packet_queue_length }
    /// WebRTC packetReordering feature.
    pub fn packet_reordering(&self) -> Option<bool> { self.packet_reordering }
    /// True to emulate internet disconnection.
    pub fn offline(&self) -> Option<bool> { self.offline }
}


pub struct NetworkConditionsBuilder<'a> {
    url_pattern: Cow<'a, str>,
    latency: f64,
    download_throughput: f64,
    upload_throughput: f64,
    connection_type: Option<ConnectionType>,
    packet_loss: Option<f64>,
    packet_queue_length: Option<u64>,
    packet_reordering: Option<bool>,
    offline: Option<bool>,
}

impl<'a> NetworkConditionsBuilder<'a> {
    /// Connection type if known.
    pub fn connection_type(mut self, connection_type: impl Into<ConnectionType>) -> Self { self.connection_type = Some(connection_type.into()); self }
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    pub fn packet_loss(mut self, packet_loss: f64) -> Self { self.packet_loss = Some(packet_loss); self }
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    pub fn packet_queue_length(mut self, packet_queue_length: u64) -> Self { self.packet_queue_length = Some(packet_queue_length); self }
    /// WebRTC packetReordering feature.
    pub fn packet_reordering(mut self, packet_reordering: bool) -> Self { self.packet_reordering = Some(packet_reordering); self }
    /// True to emulate internet disconnection.
    pub fn offline(mut self, offline: bool) -> Self { self.offline = Some(offline); self }
    pub fn build(self) -> NetworkConditions<'a> {
        NetworkConditions {
            url_pattern: self.url_pattern,
            latency: self.latency,
            download_throughput: self.download_throughput,
            upload_throughput: self.upload_throughput,
            connection_type: self.connection_type,
            packet_loss: self.packet_loss,
            packet_queue_length: self.packet_queue_length,
            packet_reordering: self.packet_reordering,
            offline: self.offline,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockPattern<'a> {
    /// URL pattern to match. Patterns use the URLPattern constructor string syntax
    /// (<https://urlpattern.spec.whatwg.org/>) and must be absolute. Example: '*://*:*/*.css'.
    #[serde(rename = "urlPattern")]
    url_pattern: Cow<'a, str>,
    /// Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later
    /// 'BlockPattern'.
    block: bool,
}

impl<'a> BlockPattern<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url_pattern`: URL pattern to match. Patterns use the URLPattern constructor string syntax (<https://urlpattern.spec.whatwg.org/>) and must be absolute. Example: `*://*:*/*.css`.
    /// * `block`: Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later `BlockPattern`.
    pub fn builder(url_pattern: impl Into<Cow<'a, str>>, block: bool) -> BlockPatternBuilder<'a> {
        BlockPatternBuilder {
            url_pattern: url_pattern.into(),
            block: block,
        }
    }
    /// URL pattern to match. Patterns use the URLPattern constructor string syntax
    /// (<https://urlpattern.spec.whatwg.org/>) and must be absolute. Example: '*://*:*/*.css'.
    pub fn url_pattern(&self) -> &str { self.url_pattern.as_ref() }
    /// Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later
    /// 'BlockPattern'.
    pub fn block(&self) -> bool { self.block }
}


pub struct BlockPatternBuilder<'a> {
    url_pattern: Cow<'a, str>,
    block: bool,
}

impl<'a> BlockPatternBuilder<'a> {
    pub fn build(self) -> BlockPattern<'a> {
        BlockPattern {
            url_pattern: self.url_pattern,
            block: self.block,
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
    #[serde(rename = "noDelay")]
    no_delay: bool,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAliveDelay")]
    keep_alive_delay: Option<f64>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sendBufferSize")]
    send_buffer_size: Option<f64>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "receiveBufferSize")]
    receive_buffer_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dnsQueryType")]
    dns_query_type: Option<DirectSocketDnsQueryType>,
}

impl DirectTCPSocketOptions {
    /// Creates a builder for this type with the required parameters:
    /// * `no_delay`: TCP_NODELAY option
    pub fn builder(no_delay: bool) -> DirectTCPSocketOptionsBuilder {
        DirectTCPSocketOptionsBuilder {
            no_delay: no_delay,
            keep_alive_delay: None,
            send_buffer_size: None,
            receive_buffer_size: None,
            dns_query_type: None,
        }
    }
    /// TCP_NODELAY option
    pub fn no_delay(&self) -> bool { self.no_delay }
    /// Expected to be unsigned integer.
    pub fn keep_alive_delay(&self) -> Option<f64> { self.keep_alive_delay }
    /// Expected to be unsigned integer.
    pub fn send_buffer_size(&self) -> Option<f64> { self.send_buffer_size }
    /// Expected to be unsigned integer.
    pub fn receive_buffer_size(&self) -> Option<f64> { self.receive_buffer_size }
    pub fn dns_query_type(&self) -> Option<&DirectSocketDnsQueryType> { self.dns_query_type.as_ref() }
}


pub struct DirectTCPSocketOptionsBuilder {
    no_delay: bool,
    keep_alive_delay: Option<f64>,
    send_buffer_size: Option<f64>,
    receive_buffer_size: Option<f64>,
    dns_query_type: Option<DirectSocketDnsQueryType>,
}

impl DirectTCPSocketOptionsBuilder {
    /// Expected to be unsigned integer.
    pub fn keep_alive_delay(mut self, keep_alive_delay: f64) -> Self { self.keep_alive_delay = Some(keep_alive_delay); self }
    /// Expected to be unsigned integer.
    pub fn send_buffer_size(mut self, send_buffer_size: f64) -> Self { self.send_buffer_size = Some(send_buffer_size); self }
    /// Expected to be unsigned integer.
    pub fn receive_buffer_size(mut self, receive_buffer_size: f64) -> Self { self.receive_buffer_size = Some(receive_buffer_size); self }
    pub fn dns_query_type(mut self, dns_query_type: impl Into<DirectSocketDnsQueryType>) -> Self { self.dns_query_type = Some(dns_query_type.into()); self }
    pub fn build(self) -> DirectTCPSocketOptions {
        DirectTCPSocketOptions {
            no_delay: self.no_delay,
            keep_alive_delay: self.keep_alive_delay,
            send_buffer_size: self.send_buffer_size,
            receive_buffer_size: self.receive_buffer_size,
            dns_query_type: self.dns_query_type,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectUDPSocketOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "remoteAddr")]
    remote_addr: Option<Cow<'a, str>>,
    /// Unsigned int 16.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remotePort")]
    remote_port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "localAddr")]
    local_addr: Option<Cow<'a, str>>,
    /// Unsigned int 16.
    #[serde(skip_serializing_if = "Option::is_none", rename = "localPort")]
    local_port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dnsQueryType")]
    dns_query_type: Option<DirectSocketDnsQueryType>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sendBufferSize")]
    send_buffer_size: Option<f64>,
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "receiveBufferSize")]
    receive_buffer_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multicastLoopback")]
    multicast_loopback: Option<bool>,
    /// Unsigned int 8.
    #[serde(skip_serializing_if = "Option::is_none", rename = "multicastTimeToLive")]
    multicast_time_to_live: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multicastAllowAddressSharing")]
    multicast_allow_address_sharing: Option<bool>,
}

impl<'a> DirectUDPSocketOptions<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> DirectUDPSocketOptionsBuilder<'a> {
        DirectUDPSocketOptionsBuilder {
            remote_addr: None,
            remote_port: None,
            local_addr: None,
            local_port: None,
            dns_query_type: None,
            send_buffer_size: None,
            receive_buffer_size: None,
            multicast_loopback: None,
            multicast_time_to_live: None,
            multicast_allow_address_sharing: None,
        }
    }
    pub fn remote_addr(&self) -> Option<&str> { self.remote_addr.as_deref() }
    /// Unsigned int 16.
    pub fn remote_port(&self) -> Option<i64> { self.remote_port }
    pub fn local_addr(&self) -> Option<&str> { self.local_addr.as_deref() }
    /// Unsigned int 16.
    pub fn local_port(&self) -> Option<i64> { self.local_port }
    pub fn dns_query_type(&self) -> Option<&DirectSocketDnsQueryType> { self.dns_query_type.as_ref() }
    /// Expected to be unsigned integer.
    pub fn send_buffer_size(&self) -> Option<f64> { self.send_buffer_size }
    /// Expected to be unsigned integer.
    pub fn receive_buffer_size(&self) -> Option<f64> { self.receive_buffer_size }
    pub fn multicast_loopback(&self) -> Option<bool> { self.multicast_loopback }
    /// Unsigned int 8.
    pub fn multicast_time_to_live(&self) -> Option<i64> { self.multicast_time_to_live }
    pub fn multicast_allow_address_sharing(&self) -> Option<bool> { self.multicast_allow_address_sharing }
}

#[derive(Default)]
pub struct DirectUDPSocketOptionsBuilder<'a> {
    remote_addr: Option<Cow<'a, str>>,
    remote_port: Option<i64>,
    local_addr: Option<Cow<'a, str>>,
    local_port: Option<i64>,
    dns_query_type: Option<DirectSocketDnsQueryType>,
    send_buffer_size: Option<f64>,
    receive_buffer_size: Option<f64>,
    multicast_loopback: Option<bool>,
    multicast_time_to_live: Option<i64>,
    multicast_allow_address_sharing: Option<bool>,
}

impl<'a> DirectUDPSocketOptionsBuilder<'a> {
    pub fn remote_addr(mut self, remote_addr: impl Into<Cow<'a, str>>) -> Self { self.remote_addr = Some(remote_addr.into()); self }
    /// Unsigned int 16.
    pub fn remote_port(mut self, remote_port: i64) -> Self { self.remote_port = Some(remote_port); self }
    pub fn local_addr(mut self, local_addr: impl Into<Cow<'a, str>>) -> Self { self.local_addr = Some(local_addr.into()); self }
    /// Unsigned int 16.
    pub fn local_port(mut self, local_port: i64) -> Self { self.local_port = Some(local_port); self }
    pub fn dns_query_type(mut self, dns_query_type: impl Into<DirectSocketDnsQueryType>) -> Self { self.dns_query_type = Some(dns_query_type.into()); self }
    /// Expected to be unsigned integer.
    pub fn send_buffer_size(mut self, send_buffer_size: f64) -> Self { self.send_buffer_size = Some(send_buffer_size); self }
    /// Expected to be unsigned integer.
    pub fn receive_buffer_size(mut self, receive_buffer_size: f64) -> Self { self.receive_buffer_size = Some(receive_buffer_size); self }
    pub fn multicast_loopback(mut self, multicast_loopback: bool) -> Self { self.multicast_loopback = Some(multicast_loopback); self }
    /// Unsigned int 8.
    pub fn multicast_time_to_live(mut self, multicast_time_to_live: i64) -> Self { self.multicast_time_to_live = Some(multicast_time_to_live); self }
    pub fn multicast_allow_address_sharing(mut self, multicast_allow_address_sharing: bool) -> Self { self.multicast_allow_address_sharing = Some(multicast_allow_address_sharing); self }
    pub fn build(self) -> DirectUDPSocketOptions<'a> {
        DirectUDPSocketOptions {
            remote_addr: self.remote_addr,
            remote_port: self.remote_port,
            local_addr: self.local_addr,
            local_port: self.local_port,
            dns_query_type: self.dns_query_type,
            send_buffer_size: self.send_buffer_size,
            receive_buffer_size: self.receive_buffer_size,
            multicast_loopback: self.multicast_loopback,
            multicast_time_to_live: self.multicast_time_to_live,
            multicast_allow_address_sharing: self.multicast_allow_address_sharing,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectUDPMessage<'a> {
    data: Cow<'a, str>,
    /// Null for connected mode.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remoteAddr")]
    remote_addr: Option<Cow<'a, str>>,
    /// Null for connected mode.
    /// Expected to be unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "remotePort")]
    remote_port: Option<i64>,
}

impl<'a> DirectUDPMessage<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `data`: 
    pub fn builder(data: impl Into<Cow<'a, str>>) -> DirectUDPMessageBuilder<'a> {
        DirectUDPMessageBuilder {
            data: data.into(),
            remote_addr: None,
            remote_port: None,
        }
    }
    pub fn data(&self) -> &str { self.data.as_ref() }
    /// Null for connected mode.
    pub fn remote_addr(&self) -> Option<&str> { self.remote_addr.as_deref() }
    /// Null for connected mode.
    /// Expected to be unsigned integer.
    pub fn remote_port(&self) -> Option<i64> { self.remote_port }
}


pub struct DirectUDPMessageBuilder<'a> {
    data: Cow<'a, str>,
    remote_addr: Option<Cow<'a, str>>,
    remote_port: Option<i64>,
}

impl<'a> DirectUDPMessageBuilder<'a> {
    /// Null for connected mode.
    pub fn remote_addr(mut self, remote_addr: impl Into<Cow<'a, str>>) -> Self { self.remote_addr = Some(remote_addr.into()); self }
    /// Null for connected mode.
    /// Expected to be unsigned integer.
    pub fn remote_port(mut self, remote_port: i64) -> Self { self.remote_port = Some(remote_port); self }
    pub fn build(self) -> DirectUDPMessage<'a> {
        DirectUDPMessage {
            data: self.data,
            remote_addr: self.remote_addr,
            remote_port: self.remote_port,
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
    #[serde(rename = "requestTime")]
    request_time: f64,
}

impl ConnectTiming {
    /// Creates a builder for this type with the required parameters:
    /// * `request_time`: Timing's requestTime is a baseline in seconds, while the other numbers are ticks in milliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for the same request (but not for redirected requests).
    pub fn builder(request_time: f64) -> ConnectTimingBuilder {
        ConnectTimingBuilder {
            request_time: request_time,
        }
    }
    /// Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    /// milliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for
    /// the same request (but not for redirected requests).
    pub fn request_time(&self) -> f64 { self.request_time }
}


pub struct ConnectTimingBuilder {
    request_time: f64,
}

impl ConnectTimingBuilder {
    pub fn build(self) -> ConnectTiming {
        ConnectTiming {
            request_time: self.request_time,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientSecurityState {
    #[serde(rename = "initiatorIsSecureContext")]
    initiator_is_secure_context: bool,
    #[serde(rename = "initiatorIPAddressSpace")]
    initiator_ip_address_space: IPAddressSpace,
    #[serde(rename = "localNetworkAccessRequestPolicy")]
    local_network_access_request_policy: LocalNetworkAccessRequestPolicy,
}

impl ClientSecurityState {
    /// Creates a builder for this type with the required parameters:
    /// * `initiator_is_secure_context`: 
    /// * `initiator_ip_address_space`: 
    /// * `local_network_access_request_policy`: 
    pub fn builder(initiator_is_secure_context: bool, initiator_ip_address_space: impl Into<IPAddressSpace>, local_network_access_request_policy: impl Into<LocalNetworkAccessRequestPolicy>) -> ClientSecurityStateBuilder {
        ClientSecurityStateBuilder {
            initiator_is_secure_context: initiator_is_secure_context,
            initiator_ip_address_space: initiator_ip_address_space.into(),
            local_network_access_request_policy: local_network_access_request_policy.into(),
        }
    }
    pub fn initiator_is_secure_context(&self) -> bool { self.initiator_is_secure_context }
    pub fn initiator_ip_address_space(&self) -> &IPAddressSpace { &self.initiator_ip_address_space }
    pub fn local_network_access_request_policy(&self) -> &LocalNetworkAccessRequestPolicy { &self.local_network_access_request_policy }
}


pub struct ClientSecurityStateBuilder {
    initiator_is_secure_context: bool,
    initiator_ip_address_space: IPAddressSpace,
    local_network_access_request_policy: LocalNetworkAccessRequestPolicy,
}

impl ClientSecurityStateBuilder {
    pub fn build(self) -> ClientSecurityState {
        ClientSecurityState {
            initiator_is_secure_context: self.initiator_is_secure_context,
            initiator_ip_address_space: self.initiator_ip_address_space,
            local_network_access_request_policy: self.local_network_access_request_policy,
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
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
    /// V8's debugging ID for the v8::Context.
    #[serde(rename = "debuggerId")]
    debugger_id: crate::runtime::UniqueDebuggerId<'a>,
    /// The script's url (or generated name based on id if inline script).
    name: Cow<'a, str>,
}

impl<'a> AdScriptIdentifier<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: The script's V8 identifier.
    /// * `debugger_id`: V8's debugging ID for the v8::Context.
    /// * `name`: The script's url (or generated name based on id if inline script).
    pub fn builder(script_id: crate::runtime::ScriptId<'a>, debugger_id: crate::runtime::UniqueDebuggerId<'a>, name: impl Into<Cow<'a, str>>) -> AdScriptIdentifierBuilder<'a> {
        AdScriptIdentifierBuilder {
            script_id: script_id,
            debugger_id: debugger_id,
            name: name.into(),
        }
    }
    /// The script's V8 identifier.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
    /// V8's debugging ID for the v8::Context.
    pub fn debugger_id(&self) -> &crate::runtime::UniqueDebuggerId<'a> { &self.debugger_id }
    /// The script's url (or generated name based on id if inline script).
    pub fn name(&self) -> &str { self.name.as_ref() }
}


pub struct AdScriptIdentifierBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
    debugger_id: crate::runtime::UniqueDebuggerId<'a>,
    name: Cow<'a, str>,
}

impl<'a> AdScriptIdentifierBuilder<'a> {
    pub fn build(self) -> AdScriptIdentifier<'a> {
        AdScriptIdentifier {
            script_id: self.script_id,
            debugger_id: self.debugger_id,
            name: self.name,
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
    #[serde(rename = "ancestryChain")]
    ancestry_chain: Vec<AdScriptIdentifier<'a>>,
    /// The filter list rule that caused the root (last) script in
    /// 'ancestryChain' to be tagged as an ad.
    #[serde(skip_serializing_if = "Option::is_none", rename = "rootScriptFilterlistRule")]
    root_script_filterlist_rule: Option<Cow<'a, str>>,
}

impl<'a> AdAncestry<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `ancestry_chain`: A chain of `AdScriptIdentifier`s representing the ancestry of an ad script that led to the creation of a resource or element. The chain is ordered from the script itself (lowest level) up to its root ancestor that was flagged by a filter list.
    pub fn builder(ancestry_chain: Vec<AdScriptIdentifier<'a>>) -> AdAncestryBuilder<'a> {
        AdAncestryBuilder {
            ancestry_chain: ancestry_chain,
            root_script_filterlist_rule: None,
        }
    }
    /// A chain of 'AdScriptIdentifier's representing the ancestry of an ad
    /// script that led to the creation of a resource or element. The chain is
    /// ordered from the script itself (lowest level) up to its root ancestor
    /// that was flagged by a filter list.
    pub fn ancestry_chain(&self) -> &[AdScriptIdentifier<'a>] { &self.ancestry_chain }
    /// The filter list rule that caused the root (last) script in
    /// 'ancestryChain' to be tagged as an ad.
    pub fn root_script_filterlist_rule(&self) -> Option<&str> { self.root_script_filterlist_rule.as_deref() }
}


pub struct AdAncestryBuilder<'a> {
    ancestry_chain: Vec<AdScriptIdentifier<'a>>,
    root_script_filterlist_rule: Option<Cow<'a, str>>,
}

impl<'a> AdAncestryBuilder<'a> {
    /// The filter list rule that caused the root (last) script in
    /// 'ancestryChain' to be tagged as an ad.
    pub fn root_script_filterlist_rule(mut self, root_script_filterlist_rule: impl Into<Cow<'a, str>>) -> Self { self.root_script_filterlist_rule = Some(root_script_filterlist_rule.into()); self }
    pub fn build(self) -> AdAncestry<'a> {
        AdAncestry {
            ancestry_chain: self.ancestry_chain,
            root_script_filterlist_rule: self.root_script_filterlist_rule,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "filterlistRule")]
    filterlist_rule: Option<Cow<'a, str>>,
    /// The script ancestry that created the ad, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "adScriptAncestry")]
    ad_script_ancestry: Option<AdAncestry<'a>>,
}

impl<'a> AdProvenance<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> AdProvenanceBuilder<'a> {
        AdProvenanceBuilder {
            filterlist_rule: None,
            ad_script_ancestry: None,
        }
    }
    /// The filterlist rule that matched, if any.
    pub fn filterlist_rule(&self) -> Option<&str> { self.filterlist_rule.as_deref() }
    /// The script ancestry that created the ad, if any.
    pub fn ad_script_ancestry(&self) -> Option<&AdAncestry<'a>> { self.ad_script_ancestry.as_ref() }
}

#[derive(Default)]
pub struct AdProvenanceBuilder<'a> {
    filterlist_rule: Option<Cow<'a, str>>,
    ad_script_ancestry: Option<AdAncestry<'a>>,
}

impl<'a> AdProvenanceBuilder<'a> {
    /// The filterlist rule that matched, if any.
    pub fn filterlist_rule(mut self, filterlist_rule: impl Into<Cow<'a, str>>) -> Self { self.filterlist_rule = Some(filterlist_rule.into()); self }
    /// The script ancestry that created the ad, if any.
    pub fn ad_script_ancestry(mut self, ad_script_ancestry: AdAncestry<'a>) -> Self { self.ad_script_ancestry = Some(ad_script_ancestry); self }
    pub fn build(self) -> AdProvenance<'a> {
        AdProvenance {
            filterlist_rule: self.filterlist_rule,
            ad_script_ancestry: self.ad_script_ancestry,
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
    #[serde(rename = "reportOnlyValue")]
    report_only_value: CrossOriginOpenerPolicyValue,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportingEndpoint")]
    reporting_endpoint: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportOnlyReportingEndpoint")]
    report_only_reporting_endpoint: Option<Cow<'a, str>>,
}

impl<'a> CrossOriginOpenerPolicyStatus<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `value`: 
    /// * `report_only_value`: 
    pub fn builder(value: impl Into<CrossOriginOpenerPolicyValue>, report_only_value: impl Into<CrossOriginOpenerPolicyValue>) -> CrossOriginOpenerPolicyStatusBuilder<'a> {
        CrossOriginOpenerPolicyStatusBuilder {
            value: value.into(),
            report_only_value: report_only_value.into(),
            reporting_endpoint: None,
            report_only_reporting_endpoint: None,
        }
    }
    pub fn value(&self) -> &CrossOriginOpenerPolicyValue { &self.value }
    pub fn report_only_value(&self) -> &CrossOriginOpenerPolicyValue { &self.report_only_value }
    pub fn reporting_endpoint(&self) -> Option<&str> { self.reporting_endpoint.as_deref() }
    pub fn report_only_reporting_endpoint(&self) -> Option<&str> { self.report_only_reporting_endpoint.as_deref() }
}


pub struct CrossOriginOpenerPolicyStatusBuilder<'a> {
    value: CrossOriginOpenerPolicyValue,
    report_only_value: CrossOriginOpenerPolicyValue,
    reporting_endpoint: Option<Cow<'a, str>>,
    report_only_reporting_endpoint: Option<Cow<'a, str>>,
}

impl<'a> CrossOriginOpenerPolicyStatusBuilder<'a> {
    pub fn reporting_endpoint(mut self, reporting_endpoint: impl Into<Cow<'a, str>>) -> Self { self.reporting_endpoint = Some(reporting_endpoint.into()); self }
    pub fn report_only_reporting_endpoint(mut self, report_only_reporting_endpoint: impl Into<Cow<'a, str>>) -> Self { self.report_only_reporting_endpoint = Some(report_only_reporting_endpoint.into()); self }
    pub fn build(self) -> CrossOriginOpenerPolicyStatus<'a> {
        CrossOriginOpenerPolicyStatus {
            value: self.value,
            report_only_value: self.report_only_value,
            reporting_endpoint: self.reporting_endpoint,
            report_only_reporting_endpoint: self.report_only_reporting_endpoint,
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
    #[serde(rename = "reportOnlyValue")]
    report_only_value: CrossOriginEmbedderPolicyValue,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportingEndpoint")]
    reporting_endpoint: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportOnlyReportingEndpoint")]
    report_only_reporting_endpoint: Option<Cow<'a, str>>,
}

impl<'a> CrossOriginEmbedderPolicyStatus<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `value`: 
    /// * `report_only_value`: 
    pub fn builder(value: impl Into<CrossOriginEmbedderPolicyValue>, report_only_value: impl Into<CrossOriginEmbedderPolicyValue>) -> CrossOriginEmbedderPolicyStatusBuilder<'a> {
        CrossOriginEmbedderPolicyStatusBuilder {
            value: value.into(),
            report_only_value: report_only_value.into(),
            reporting_endpoint: None,
            report_only_reporting_endpoint: None,
        }
    }
    pub fn value(&self) -> &CrossOriginEmbedderPolicyValue { &self.value }
    pub fn report_only_value(&self) -> &CrossOriginEmbedderPolicyValue { &self.report_only_value }
    pub fn reporting_endpoint(&self) -> Option<&str> { self.reporting_endpoint.as_deref() }
    pub fn report_only_reporting_endpoint(&self) -> Option<&str> { self.report_only_reporting_endpoint.as_deref() }
}


pub struct CrossOriginEmbedderPolicyStatusBuilder<'a> {
    value: CrossOriginEmbedderPolicyValue,
    report_only_value: CrossOriginEmbedderPolicyValue,
    reporting_endpoint: Option<Cow<'a, str>>,
    report_only_reporting_endpoint: Option<Cow<'a, str>>,
}

impl<'a> CrossOriginEmbedderPolicyStatusBuilder<'a> {
    pub fn reporting_endpoint(mut self, reporting_endpoint: impl Into<Cow<'a, str>>) -> Self { self.reporting_endpoint = Some(reporting_endpoint.into()); self }
    pub fn report_only_reporting_endpoint(mut self, report_only_reporting_endpoint: impl Into<Cow<'a, str>>) -> Self { self.report_only_reporting_endpoint = Some(report_only_reporting_endpoint.into()); self }
    pub fn build(self) -> CrossOriginEmbedderPolicyStatus<'a> {
        CrossOriginEmbedderPolicyStatus {
            value: self.value,
            report_only_value: self.report_only_value,
            reporting_endpoint: self.reporting_endpoint,
            report_only_reporting_endpoint: self.report_only_reporting_endpoint,
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
    #[serde(rename = "effectiveDirectives")]
    effective_directives: Cow<'a, str>,
    #[serde(rename = "isEnforced")]
    is_enforced: bool,
    source: ContentSecurityPolicySource,
}

impl<'a> ContentSecurityPolicyStatus<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `effective_directives`: 
    /// * `is_enforced`: 
    /// * `source`: 
    pub fn builder(effective_directives: impl Into<Cow<'a, str>>, is_enforced: bool, source: impl Into<ContentSecurityPolicySource>) -> ContentSecurityPolicyStatusBuilder<'a> {
        ContentSecurityPolicyStatusBuilder {
            effective_directives: effective_directives.into(),
            is_enforced: is_enforced,
            source: source.into(),
        }
    }
    pub fn effective_directives(&self) -> &str { self.effective_directives.as_ref() }
    pub fn is_enforced(&self) -> bool { self.is_enforced }
    pub fn source(&self) -> &ContentSecurityPolicySource { &self.source }
}


pub struct ContentSecurityPolicyStatusBuilder<'a> {
    effective_directives: Cow<'a, str>,
    is_enforced: bool,
    source: ContentSecurityPolicySource,
}

impl<'a> ContentSecurityPolicyStatusBuilder<'a> {
    pub fn build(self) -> ContentSecurityPolicyStatus<'a> {
        ContentSecurityPolicyStatus {
            effective_directives: self.effective_directives,
            is_enforced: self.is_enforced,
            source: self.source,
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
    /// Creates a builder for this type.
    pub fn builder() -> SecurityIsolationStatusBuilder<'a> {
        SecurityIsolationStatusBuilder {
            coop: None,
            coep: None,
            csp: None,
        }
    }
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
    #[serde(rename = "initiatorUrl")]
    initiator_url: Cow<'a, str>,
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
    #[serde(rename = "completedAttempts")]
    completed_attempts: i64,
    body: serde_json::Map<String, JsonValue>,
    status: ReportStatus,
}

impl<'a> ReportingApiReport<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: 
    /// * `initiator_url`: The URL of the document that triggered the report.
    /// * `destination`: The name of the endpoint group that should be used to deliver the report.
    /// * `type_`: The type of the report (specifies the set of data that is contained in the report body).
    /// * `timestamp`: When the report was generated.
    /// * `depth`: How many uploads deep the related request was.
    /// * `completed_attempts`: The number of delivery attempts made so far, not including an active attempt.
    /// * `body`: 
    /// * `status`: 
    pub fn builder(id: impl Into<ReportId<'a>>, initiator_url: impl Into<Cow<'a, str>>, destination: impl Into<Cow<'a, str>>, type_: impl Into<Cow<'a, str>>, timestamp: crate::network::TimeSinceEpoch, depth: i64, completed_attempts: i64, body: serde_json::Map<String, JsonValue>, status: impl Into<ReportStatus>) -> ReportingApiReportBuilder<'a> {
        ReportingApiReportBuilder {
            id: id.into(),
            initiator_url: initiator_url.into(),
            destination: destination.into(),
            type_: type_.into(),
            timestamp: timestamp,
            depth: depth,
            completed_attempts: completed_attempts,
            body: body,
            status: status.into(),
        }
    }
    pub fn id(&self) -> &ReportId<'a> { &self.id }
    /// The URL of the document that triggered the report.
    pub fn initiator_url(&self) -> &str { self.initiator_url.as_ref() }
    /// The name of the endpoint group that should be used to deliver the report.
    pub fn destination(&self) -> &str { self.destination.as_ref() }
    /// The type of the report (specifies the set of data that is contained in the report body).
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// When the report was generated.
    pub fn timestamp(&self) -> &crate::network::TimeSinceEpoch { &self.timestamp }
    /// How many uploads deep the related request was.
    pub fn depth(&self) -> i64 { self.depth }
    /// The number of delivery attempts made so far, not including an active attempt.
    pub fn completed_attempts(&self) -> i64 { self.completed_attempts }
    pub fn body(&self) -> &serde_json::Map<String, JsonValue> { &self.body }
    pub fn status(&self) -> &ReportStatus { &self.status }
}


pub struct ReportingApiReportBuilder<'a> {
    id: ReportId<'a>,
    initiator_url: Cow<'a, str>,
    destination: Cow<'a, str>,
    type_: Cow<'a, str>,
    timestamp: crate::network::TimeSinceEpoch,
    depth: i64,
    completed_attempts: i64,
    body: serde_json::Map<String, JsonValue>,
    status: ReportStatus,
}

impl<'a> ReportingApiReportBuilder<'a> {
    pub fn build(self) -> ReportingApiReport<'a> {
        ReportingApiReport {
            id: self.id,
            initiator_url: self.initiator_url,
            destination: self.destination,
            type_: self.type_,
            timestamp: self.timestamp,
            depth: self.depth,
            completed_attempts: self.completed_attempts,
            body: self.body,
            status: self.status,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportingApiEndpoint<'a> {
    /// The URL of the endpoint to which reports may be delivered.
    url: Cow<'a, str>,
    /// Name of the endpoint group.
    #[serde(rename = "groupName")]
    group_name: Cow<'a, str>,
}

impl<'a> ReportingApiEndpoint<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: The URL of the endpoint to which reports may be delivered.
    /// * `group_name`: Name of the endpoint group.
    pub fn builder(url: impl Into<Cow<'a, str>>, group_name: impl Into<Cow<'a, str>>) -> ReportingApiEndpointBuilder<'a> {
        ReportingApiEndpointBuilder {
            url: url.into(),
            group_name: group_name.into(),
        }
    }
    /// The URL of the endpoint to which reports may be delivered.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Name of the endpoint group.
    pub fn group_name(&self) -> &str { self.group_name.as_ref() }
}


pub struct ReportingApiEndpointBuilder<'a> {
    url: Cow<'a, str>,
    group_name: Cow<'a, str>,
}

impl<'a> ReportingApiEndpointBuilder<'a> {
    pub fn build(self) -> ReportingApiEndpoint<'a> {
        ReportingApiEndpoint {
            url: self.url,
            group_name: self.group_name,
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
    /// Creates a builder for this type with the required parameters:
    /// * `site`: The site the session is set up for.
    /// * `id`: The id of the session.
    pub fn builder(site: impl Into<Cow<'a, str>>, id: impl Into<Cow<'a, str>>) -> DeviceBoundSessionKeyBuilder<'a> {
        DeviceBoundSessionKeyBuilder {
            site: site.into(),
            id: id.into(),
        }
    }
    /// The site the session is set up for.
    pub fn site(&self) -> &str { self.site.as_ref() }
    /// The id of the session.
    pub fn id(&self) -> &str { self.id.as_ref() }
}


pub struct DeviceBoundSessionKeyBuilder<'a> {
    site: Cow<'a, str>,
    id: Cow<'a, str>,
}

impl<'a> DeviceBoundSessionKeyBuilder<'a> {
    pub fn build(self) -> DeviceBoundSessionKey<'a> {
        DeviceBoundSessionKey {
            site: self.site,
            id: self.id,
        }
    }
}

/// How a device bound session was used during a request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionWithUsage<'a> {
    /// The key for the session.
    #[serde(rename = "sessionKey")]
    session_key: DeviceBoundSessionKey<'a>,
    /// How the session was used (or not used).
    usage: Cow<'a, str>,
}

impl<'a> DeviceBoundSessionWithUsage<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `session_key`: The key for the session.
    /// * `usage`: How the session was used (or not used).
    pub fn builder(session_key: DeviceBoundSessionKey<'a>, usage: impl Into<Cow<'a, str>>) -> DeviceBoundSessionWithUsageBuilder<'a> {
        DeviceBoundSessionWithUsageBuilder {
            session_key: session_key,
            usage: usage.into(),
        }
    }
    /// The key for the session.
    pub fn session_key(&self) -> &DeviceBoundSessionKey<'a> { &self.session_key }
    /// How the session was used (or not used).
    pub fn usage(&self) -> &str { self.usage.as_ref() }
}


pub struct DeviceBoundSessionWithUsageBuilder<'a> {
    session_key: DeviceBoundSessionKey<'a>,
    usage: Cow<'a, str>,
}

impl<'a> DeviceBoundSessionWithUsageBuilder<'a> {
    pub fn build(self) -> DeviceBoundSessionWithUsage<'a> {
        DeviceBoundSessionWithUsage {
            session_key: self.session_key,
            usage: self.usage,
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
    #[serde(rename = "httpOnly")]
    http_only: bool,
    /// The 'SameSite' attribute of the craving attributes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
    same_site: Option<CookieSameSite>,
}

impl<'a> DeviceBoundSessionCookieCraving<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: The name of the craving.
    /// * `domain`: The domain of the craving.
    /// * `path`: The path of the craving.
    /// * `secure`: The `Secure` attribute of the craving attributes.
    /// * `http_only`: The `HttpOnly` attribute of the craving attributes.
    pub fn builder(name: impl Into<Cow<'a, str>>, domain: impl Into<Cow<'a, str>>, path: impl Into<Cow<'a, str>>, secure: bool, http_only: bool) -> DeviceBoundSessionCookieCravingBuilder<'a> {
        DeviceBoundSessionCookieCravingBuilder {
            name: name.into(),
            domain: domain.into(),
            path: path.into(),
            secure: secure,
            http_only: http_only,
            same_site: None,
        }
    }
    /// The name of the craving.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// The domain of the craving.
    pub fn domain(&self) -> &str { self.domain.as_ref() }
    /// The path of the craving.
    pub fn path(&self) -> &str { self.path.as_ref() }
    /// The 'Secure' attribute of the craving attributes.
    pub fn secure(&self) -> bool { self.secure }
    /// The 'HttpOnly' attribute of the craving attributes.
    pub fn http_only(&self) -> bool { self.http_only }
    /// The 'SameSite' attribute of the craving attributes.
    pub fn same_site(&self) -> Option<&CookieSameSite> { self.same_site.as_ref() }
}


pub struct DeviceBoundSessionCookieCravingBuilder<'a> {
    name: Cow<'a, str>,
    domain: Cow<'a, str>,
    path: Cow<'a, str>,
    secure: bool,
    http_only: bool,
    same_site: Option<CookieSameSite>,
}

impl<'a> DeviceBoundSessionCookieCravingBuilder<'a> {
    /// The 'SameSite' attribute of the craving attributes.
    pub fn same_site(mut self, same_site: impl Into<CookieSameSite>) -> Self { self.same_site = Some(same_site.into()); self }
    pub fn build(self) -> DeviceBoundSessionCookieCraving<'a> {
        DeviceBoundSessionCookieCraving {
            name: self.name,
            domain: self.domain,
            path: self.path,
            secure: self.secure,
            http_only: self.http_only,
            same_site: self.same_site,
        }
    }
}

/// A device bound session's inclusion URL rule.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionUrlRule<'a> {
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type'.
    #[serde(rename = "ruleType")]
    rule_type: Cow<'a, str>,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern'.
    #[serde(rename = "hostPattern")]
    host_pattern: Cow<'a, str>,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix'.
    #[serde(rename = "pathPrefix")]
    path_prefix: Cow<'a, str>,
}

impl<'a> DeviceBoundSessionUrlRule<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `rule_type`: See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type`.
    /// * `host_pattern`: See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern`.
    /// * `path_prefix`: See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix`.
    pub fn builder(rule_type: impl Into<Cow<'a, str>>, host_pattern: impl Into<Cow<'a, str>>, path_prefix: impl Into<Cow<'a, str>>) -> DeviceBoundSessionUrlRuleBuilder<'a> {
        DeviceBoundSessionUrlRuleBuilder {
            rule_type: rule_type.into(),
            host_pattern: host_pattern.into(),
            path_prefix: path_prefix.into(),
        }
    }
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type'.
    pub fn rule_type(&self) -> &str { self.rule_type.as_ref() }
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern'.
    pub fn host_pattern(&self) -> &str { self.host_pattern.as_ref() }
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix'.
    pub fn path_prefix(&self) -> &str { self.path_prefix.as_ref() }
}


pub struct DeviceBoundSessionUrlRuleBuilder<'a> {
    rule_type: Cow<'a, str>,
    host_pattern: Cow<'a, str>,
    path_prefix: Cow<'a, str>,
}

impl<'a> DeviceBoundSessionUrlRuleBuilder<'a> {
    pub fn build(self) -> DeviceBoundSessionUrlRule<'a> {
        DeviceBoundSessionUrlRule {
            rule_type: self.rule_type,
            host_pattern: self.host_pattern,
            path_prefix: self.path_prefix,
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
    #[serde(rename = "includeSite")]
    include_site: bool,
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::url_rules_'.
    #[serde(rename = "urlRules")]
    url_rules: Vec<DeviceBoundSessionUrlRule<'a>>,
}

impl<'a> DeviceBoundSessionInclusionRules<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: See comments on `net::device_bound_sessions::SessionInclusionRules::origin_`.
    /// * `include_site`: Whether the whole site is included. See comments on `net::device_bound_sessions::SessionInclusionRules::include_site_` for more details; this boolean is true if that value is populated.
    /// * `url_rules`: See comments on `net::device_bound_sessions::SessionInclusionRules::url_rules_`.
    pub fn builder(origin: impl Into<Cow<'a, str>>, include_site: bool, url_rules: Vec<DeviceBoundSessionUrlRule<'a>>) -> DeviceBoundSessionInclusionRulesBuilder<'a> {
        DeviceBoundSessionInclusionRulesBuilder {
            origin: origin.into(),
            include_site: include_site,
            url_rules: url_rules,
        }
    }
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::origin_'.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    /// Whether the whole site is included. See comments on
    /// 'net::device_bound_sessions::SessionInclusionRules::include_site_' for more
    /// details; this boolean is true if that value is populated.
    pub fn include_site(&self) -> bool { self.include_site }
    /// See comments on 'net::device_bound_sessions::SessionInclusionRules::url_rules_'.
    pub fn url_rules(&self) -> &[DeviceBoundSessionUrlRule<'a>] { &self.url_rules }
}


pub struct DeviceBoundSessionInclusionRulesBuilder<'a> {
    origin: Cow<'a, str>,
    include_site: bool,
    url_rules: Vec<DeviceBoundSessionUrlRule<'a>>,
}

impl<'a> DeviceBoundSessionInclusionRulesBuilder<'a> {
    pub fn build(self) -> DeviceBoundSessionInclusionRules<'a> {
        DeviceBoundSessionInclusionRules {
            origin: self.origin,
            include_site: self.include_site,
            url_rules: self.url_rules,
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
    #[serde(rename = "refreshUrl")]
    refresh_url: Cow<'a, str>,
    /// See comments on 'net::device_bound_sessions::Session::inclusion_rules_'.
    #[serde(rename = "inclusionRules")]
    inclusion_rules: DeviceBoundSessionInclusionRules<'a>,
    /// See comments on 'net::device_bound_sessions::Session::cookie_cravings_'.
    #[serde(rename = "cookieCravings")]
    cookie_cravings: Vec<DeviceBoundSessionCookieCraving<'a>>,
    /// See comments on 'net::device_bound_sessions::Session::expiry_date_'.
    #[serde(rename = "expiryDate")]
    expiry_date: crate::network::TimeSinceEpoch,
    /// See comments on 'net::device_bound_sessions::Session::cached_challenge__'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cachedChallenge")]
    cached_challenge: Option<Cow<'a, str>>,
    /// See comments on 'net::device_bound_sessions::Session::allowed_refresh_initiators_'.
    #[serde(rename = "allowedRefreshInitiators")]
    allowed_refresh_initiators: Vec<Cow<'a, str>>,
}

impl<'a> DeviceBoundSession<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `key`: The site and session ID of the session.
    /// * `refresh_url`: See comments on `net::device_bound_sessions::Session::refresh_url_`.
    /// * `inclusion_rules`: See comments on `net::device_bound_sessions::Session::inclusion_rules_`.
    /// * `cookie_cravings`: See comments on `net::device_bound_sessions::Session::cookie_cravings_`.
    /// * `expiry_date`: See comments on `net::device_bound_sessions::Session::expiry_date_`.
    /// * `allowed_refresh_initiators`: See comments on `net::device_bound_sessions::Session::allowed_refresh_initiators_`.
    pub fn builder(key: DeviceBoundSessionKey<'a>, refresh_url: impl Into<Cow<'a, str>>, inclusion_rules: DeviceBoundSessionInclusionRules<'a>, cookie_cravings: Vec<DeviceBoundSessionCookieCraving<'a>>, expiry_date: crate::network::TimeSinceEpoch, allowed_refresh_initiators: Vec<Cow<'a, str>>) -> DeviceBoundSessionBuilder<'a> {
        DeviceBoundSessionBuilder {
            key: key,
            refresh_url: refresh_url.into(),
            inclusion_rules: inclusion_rules,
            cookie_cravings: cookie_cravings,
            expiry_date: expiry_date,
            cached_challenge: None,
            allowed_refresh_initiators: allowed_refresh_initiators,
        }
    }
    /// The site and session ID of the session.
    pub fn key(&self) -> &DeviceBoundSessionKey<'a> { &self.key }
    /// See comments on 'net::device_bound_sessions::Session::refresh_url_'.
    pub fn refresh_url(&self) -> &str { self.refresh_url.as_ref() }
    /// See comments on 'net::device_bound_sessions::Session::inclusion_rules_'.
    pub fn inclusion_rules(&self) -> &DeviceBoundSessionInclusionRules<'a> { &self.inclusion_rules }
    /// See comments on 'net::device_bound_sessions::Session::cookie_cravings_'.
    pub fn cookie_cravings(&self) -> &[DeviceBoundSessionCookieCraving<'a>] { &self.cookie_cravings }
    /// See comments on 'net::device_bound_sessions::Session::expiry_date_'.
    pub fn expiry_date(&self) -> &crate::network::TimeSinceEpoch { &self.expiry_date }
    /// See comments on 'net::device_bound_sessions::Session::cached_challenge__'.
    pub fn cached_challenge(&self) -> Option<&str> { self.cached_challenge.as_deref() }
    /// See comments on 'net::device_bound_sessions::Session::allowed_refresh_initiators_'.
    pub fn allowed_refresh_initiators(&self) -> &[Cow<'a, str>] { &self.allowed_refresh_initiators }
}


pub struct DeviceBoundSessionBuilder<'a> {
    key: DeviceBoundSessionKey<'a>,
    refresh_url: Cow<'a, str>,
    inclusion_rules: DeviceBoundSessionInclusionRules<'a>,
    cookie_cravings: Vec<DeviceBoundSessionCookieCraving<'a>>,
    expiry_date: crate::network::TimeSinceEpoch,
    cached_challenge: Option<Cow<'a, str>>,
    allowed_refresh_initiators: Vec<Cow<'a, str>>,
}

impl<'a> DeviceBoundSessionBuilder<'a> {
    /// See comments on 'net::device_bound_sessions::Session::cached_challenge__'.
    pub fn cached_challenge(mut self, cached_challenge: impl Into<Cow<'a, str>>) -> Self { self.cached_challenge = Some(cached_challenge.into()); self }
    pub fn build(self) -> DeviceBoundSession<'a> {
        DeviceBoundSession {
            key: self.key,
            refresh_url: self.refresh_url,
            inclusion_rules: self.inclusion_rules,
            cookie_cravings: self.cookie_cravings,
            expiry_date: self.expiry_date,
            cached_challenge: self.cached_challenge,
            allowed_refresh_initiators: self.allowed_refresh_initiators,
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
}

/// Details about a failed device bound session network request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBoundSessionFailedRequest<'a> {
    /// The failed request URL.
    #[serde(rename = "requestUrl")]
    request_url: Cow<'a, str>,
    /// The net error of the response if it was not OK.
    #[serde(skip_serializing_if = "Option::is_none", rename = "netError")]
    net_error: Option<Cow<'a, str>>,
    /// The response code if the net error was OK and the response code was not
    /// 200.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseError")]
    response_error: Option<i64>,
    /// The body of the response if the net error was OK, the response code was
    /// not 200, and the response body was not empty.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseErrorBody")]
    response_error_body: Option<Cow<'a, str>>,
}

impl<'a> DeviceBoundSessionFailedRequest<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_url`: The failed request URL.
    pub fn builder(request_url: impl Into<Cow<'a, str>>) -> DeviceBoundSessionFailedRequestBuilder<'a> {
        DeviceBoundSessionFailedRequestBuilder {
            request_url: request_url.into(),
            net_error: None,
            response_error: None,
            response_error_body: None,
        }
    }
    /// The failed request URL.
    pub fn request_url(&self) -> &str { self.request_url.as_ref() }
    /// The net error of the response if it was not OK.
    pub fn net_error(&self) -> Option<&str> { self.net_error.as_deref() }
    /// The response code if the net error was OK and the response code was not
    /// 200.
    pub fn response_error(&self) -> Option<i64> { self.response_error }
    /// The body of the response if the net error was OK, the response code was
    /// not 200, and the response body was not empty.
    pub fn response_error_body(&self) -> Option<&str> { self.response_error_body.as_deref() }
}


pub struct DeviceBoundSessionFailedRequestBuilder<'a> {
    request_url: Cow<'a, str>,
    net_error: Option<Cow<'a, str>>,
    response_error: Option<i64>,
    response_error_body: Option<Cow<'a, str>>,
}

impl<'a> DeviceBoundSessionFailedRequestBuilder<'a> {
    /// The net error of the response if it was not OK.
    pub fn net_error(mut self, net_error: impl Into<Cow<'a, str>>) -> Self { self.net_error = Some(net_error.into()); self }
    /// The response code if the net error was OK and the response code was not
    /// 200.
    pub fn response_error(mut self, response_error: i64) -> Self { self.response_error = Some(response_error); self }
    /// The body of the response if the net error was OK, the response code was
    /// not 200, and the response body was not empty.
    pub fn response_error_body(mut self, response_error_body: impl Into<Cow<'a, str>>) -> Self { self.response_error_body = Some(response_error_body.into()); self }
    pub fn build(self) -> DeviceBoundSessionFailedRequest<'a> {
        DeviceBoundSessionFailedRequest {
            request_url: self.request_url,
            net_error: self.net_error,
            response_error: self.response_error,
            response_error_body: self.response_error_body,
        }
    }
}

/// Session event details specific to creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreationEventDetails<'a> {
    /// The result of the fetch attempt.
    #[serde(rename = "fetchResult")]
    fetch_result: DeviceBoundSessionFetchResult,
    /// The session if there was a newly created session. This is populated for
    /// all successful creation events.
    #[serde(skip_serializing_if = "Option::is_none", rename = "newSession")]
    new_session: Option<DeviceBoundSession<'a>>,
    /// Details about a failed device bound session network request if there was
    /// one.
    #[serde(skip_serializing_if = "Option::is_none", rename = "failedRequest")]
    failed_request: Option<DeviceBoundSessionFailedRequest<'a>>,
}

impl<'a> CreationEventDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `fetch_result`: The result of the fetch attempt.
    pub fn builder(fetch_result: impl Into<DeviceBoundSessionFetchResult>) -> CreationEventDetailsBuilder<'a> {
        CreationEventDetailsBuilder {
            fetch_result: fetch_result.into(),
            new_session: None,
            failed_request: None,
        }
    }
    /// The result of the fetch attempt.
    pub fn fetch_result(&self) -> &DeviceBoundSessionFetchResult { &self.fetch_result }
    /// The session if there was a newly created session. This is populated for
    /// all successful creation events.
    pub fn new_session(&self) -> Option<&DeviceBoundSession<'a>> { self.new_session.as_ref() }
    /// Details about a failed device bound session network request if there was
    /// one.
    pub fn failed_request(&self) -> Option<&DeviceBoundSessionFailedRequest<'a>> { self.failed_request.as_ref() }
}


pub struct CreationEventDetailsBuilder<'a> {
    fetch_result: DeviceBoundSessionFetchResult,
    new_session: Option<DeviceBoundSession<'a>>,
    failed_request: Option<DeviceBoundSessionFailedRequest<'a>>,
}

impl<'a> CreationEventDetailsBuilder<'a> {
    /// The session if there was a newly created session. This is populated for
    /// all successful creation events.
    pub fn new_session(mut self, new_session: DeviceBoundSession<'a>) -> Self { self.new_session = Some(new_session); self }
    /// Details about a failed device bound session network request if there was
    /// one.
    pub fn failed_request(mut self, failed_request: DeviceBoundSessionFailedRequest<'a>) -> Self { self.failed_request = Some(failed_request); self }
    pub fn build(self) -> CreationEventDetails<'a> {
        CreationEventDetails {
            fetch_result: self.fetch_result,
            new_session: self.new_session,
            failed_request: self.failed_request,
        }
    }
}

/// Session event details specific to refresh.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RefreshEventDetails<'a> {
    /// The result of a refresh.
    #[serde(rename = "refreshResult")]
    refresh_result: Cow<'a, str>,
    /// If there was a fetch attempt, the result of that.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fetchResult")]
    fetch_result: Option<DeviceBoundSessionFetchResult>,
    /// The session display if there was a newly created session. This is populated
    /// for any refresh event that modifies the session config.
    #[serde(skip_serializing_if = "Option::is_none", rename = "newSession")]
    new_session: Option<DeviceBoundSession<'a>>,
    /// See comments on 'net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh'.
    #[serde(rename = "wasFullyProactiveRefresh")]
    was_fully_proactive_refresh: bool,
    /// Details about a failed device bound session network request if there was
    /// one.
    #[serde(skip_serializing_if = "Option::is_none", rename = "failedRequest")]
    failed_request: Option<DeviceBoundSessionFailedRequest<'a>>,
}

impl<'a> RefreshEventDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `refresh_result`: The result of a refresh.
    /// * `was_fully_proactive_refresh`: See comments on `net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh`.
    pub fn builder(refresh_result: impl Into<Cow<'a, str>>, was_fully_proactive_refresh: bool) -> RefreshEventDetailsBuilder<'a> {
        RefreshEventDetailsBuilder {
            refresh_result: refresh_result.into(),
            fetch_result: None,
            new_session: None,
            was_fully_proactive_refresh: was_fully_proactive_refresh,
            failed_request: None,
        }
    }
    /// The result of a refresh.
    pub fn refresh_result(&self) -> &str { self.refresh_result.as_ref() }
    /// If there was a fetch attempt, the result of that.
    pub fn fetch_result(&self) -> Option<&DeviceBoundSessionFetchResult> { self.fetch_result.as_ref() }
    /// The session display if there was a newly created session. This is populated
    /// for any refresh event that modifies the session config.
    pub fn new_session(&self) -> Option<&DeviceBoundSession<'a>> { self.new_session.as_ref() }
    /// See comments on 'net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh'.
    pub fn was_fully_proactive_refresh(&self) -> bool { self.was_fully_proactive_refresh }
    /// Details about a failed device bound session network request if there was
    /// one.
    pub fn failed_request(&self) -> Option<&DeviceBoundSessionFailedRequest<'a>> { self.failed_request.as_ref() }
}


pub struct RefreshEventDetailsBuilder<'a> {
    refresh_result: Cow<'a, str>,
    fetch_result: Option<DeviceBoundSessionFetchResult>,
    new_session: Option<DeviceBoundSession<'a>>,
    was_fully_proactive_refresh: bool,
    failed_request: Option<DeviceBoundSessionFailedRequest<'a>>,
}

impl<'a> RefreshEventDetailsBuilder<'a> {
    /// If there was a fetch attempt, the result of that.
    pub fn fetch_result(mut self, fetch_result: impl Into<DeviceBoundSessionFetchResult>) -> Self { self.fetch_result = Some(fetch_result.into()); self }
    /// The session display if there was a newly created session. This is populated
    /// for any refresh event that modifies the session config.
    pub fn new_session(mut self, new_session: DeviceBoundSession<'a>) -> Self { self.new_session = Some(new_session); self }
    /// Details about a failed device bound session network request if there was
    /// one.
    pub fn failed_request(mut self, failed_request: DeviceBoundSessionFailedRequest<'a>) -> Self { self.failed_request = Some(failed_request); self }
    pub fn build(self) -> RefreshEventDetails<'a> {
        RefreshEventDetails {
            refresh_result: self.refresh_result,
            fetch_result: self.fetch_result,
            new_session: self.new_session,
            was_fully_proactive_refresh: self.was_fully_proactive_refresh,
            failed_request: self.failed_request,
        }
    }
}

/// Session event details specific to termination.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TerminationEventDetails<'a> {
    /// The reason for a session being deleted.
    #[serde(rename = "deletionReason")]
    deletion_reason: Cow<'a, str>,
}

impl<'a> TerminationEventDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `deletion_reason`: The reason for a session being deleted.
    pub fn builder(deletion_reason: impl Into<Cow<'a, str>>) -> TerminationEventDetailsBuilder<'a> {
        TerminationEventDetailsBuilder {
            deletion_reason: deletion_reason.into(),
        }
    }
    /// The reason for a session being deleted.
    pub fn deletion_reason(&self) -> &str { self.deletion_reason.as_ref() }
}


pub struct TerminationEventDetailsBuilder<'a> {
    deletion_reason: Cow<'a, str>,
}

impl<'a> TerminationEventDetailsBuilder<'a> {
    pub fn build(self) -> TerminationEventDetails<'a> {
        TerminationEventDetails {
            deletion_reason: self.deletion_reason,
        }
    }
}

/// Session event details specific to challenges.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeEventDetails<'a> {
    /// The result of a challenge.
    #[serde(rename = "challengeResult")]
    challenge_result: Cow<'a, str>,
    /// The challenge set.
    challenge: Cow<'a, str>,
}

impl<'a> ChallengeEventDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `challenge_result`: The result of a challenge.
    /// * `challenge`: The challenge set.
    pub fn builder(challenge_result: impl Into<Cow<'a, str>>, challenge: impl Into<Cow<'a, str>>) -> ChallengeEventDetailsBuilder<'a> {
        ChallengeEventDetailsBuilder {
            challenge_result: challenge_result.into(),
            challenge: challenge.into(),
        }
    }
    /// The result of a challenge.
    pub fn challenge_result(&self) -> &str { self.challenge_result.as_ref() }
    /// The challenge set.
    pub fn challenge(&self) -> &str { self.challenge.as_ref() }
}


pub struct ChallengeEventDetailsBuilder<'a> {
    challenge_result: Cow<'a, str>,
    challenge: Cow<'a, str>,
}

impl<'a> ChallengeEventDetailsBuilder<'a> {
    pub fn build(self) -> ChallengeEventDetails<'a> {
        ChallengeEventDetails {
            challenge_result: self.challenge_result,
            challenge: self.challenge,
        }
    }
}

/// An object providing the result of a network resource load.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourcePageResult<'a> {
    success: bool,
    /// Optional values used for error reporting.
    #[serde(skip_serializing_if = "Option::is_none", rename = "netError")]
    net_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "netErrorName")]
    net_error_name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "httpStatusCode")]
    http_status_code: Option<f64>,
    /// If successful, one of the following two fields holds the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<crate::io::StreamHandle<'a>>,
    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<crate::network::Headers>,
}

impl<'a> LoadNetworkResourcePageResult<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `success`: 
    pub fn builder(success: bool) -> LoadNetworkResourcePageResultBuilder<'a> {
        LoadNetworkResourcePageResultBuilder {
            success: success,
            net_error: None,
            net_error_name: None,
            http_status_code: None,
            stream: None,
            headers: None,
        }
    }
    pub fn success(&self) -> bool { self.success }
    /// Optional values used for error reporting.
    pub fn net_error(&self) -> Option<f64> { self.net_error }
    pub fn net_error_name(&self) -> Option<&str> { self.net_error_name.as_deref() }
    pub fn http_status_code(&self) -> Option<f64> { self.http_status_code }
    /// If successful, one of the following two fields holds the result.
    pub fn stream(&self) -> Option<&crate::io::StreamHandle<'a>> { self.stream.as_ref() }
    /// Response headers.
    pub fn headers(&self) -> Option<&crate::network::Headers> { self.headers.as_ref() }
}


pub struct LoadNetworkResourcePageResultBuilder<'a> {
    success: bool,
    net_error: Option<f64>,
    net_error_name: Option<Cow<'a, str>>,
    http_status_code: Option<f64>,
    stream: Option<crate::io::StreamHandle<'a>>,
    headers: Option<crate::network::Headers>,
}

impl<'a> LoadNetworkResourcePageResultBuilder<'a> {
    /// Optional values used for error reporting.
    pub fn net_error(mut self, net_error: f64) -> Self { self.net_error = Some(net_error); self }
    pub fn net_error_name(mut self, net_error_name: impl Into<Cow<'a, str>>) -> Self { self.net_error_name = Some(net_error_name.into()); self }
    pub fn http_status_code(mut self, http_status_code: f64) -> Self { self.http_status_code = Some(http_status_code); self }
    /// If successful, one of the following two fields holds the result.
    pub fn stream(mut self, stream: crate::io::StreamHandle<'a>) -> Self { self.stream = Some(stream); self }
    /// Response headers.
    pub fn headers(mut self, headers: crate::network::Headers) -> Self { self.headers = Some(headers); self }
    pub fn build(self) -> LoadNetworkResourcePageResult<'a> {
        LoadNetworkResourcePageResult {
            success: self.success,
            net_error: self.net_error,
            net_error_name: self.net_error_name,
            http_status_code: self.http_status_code,
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
    #[serde(rename = "disableCache")]
    disable_cache: bool,
    #[serde(rename = "includeCredentials")]
    include_credentials: bool,
}

impl LoadNetworkResourceOptions {
    /// Creates a builder for this type with the required parameters:
    /// * `disable_cache`: 
    /// * `include_credentials`: 
    pub fn builder(disable_cache: bool, include_credentials: bool) -> LoadNetworkResourceOptionsBuilder {
        LoadNetworkResourceOptionsBuilder {
            disable_cache: disable_cache,
            include_credentials: include_credentials,
        }
    }
    pub fn disable_cache(&self) -> bool { self.disable_cache }
    pub fn include_credentials(&self) -> bool { self.include_credentials }
}


pub struct LoadNetworkResourceOptionsBuilder {
    disable_cache: bool,
    include_credentials: bool,
}

impl LoadNetworkResourceOptionsBuilder {
    pub fn build(self) -> LoadNetworkResourceOptions {
        LoadNetworkResourceOptions {
            disable_cache: self.disable_cache,
            include_credentials: self.include_credentials,
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
    /// Creates a builder for this type with the required parameters:
    /// * `encodings`: List of accepted content encodings.
    pub fn builder(encodings: Vec<ContentEncoding>) -> SetAcceptedEncodingsParamsBuilder {
        SetAcceptedEncodingsParamsBuilder {
            encodings: encodings,
        }
    }
    /// List of accepted content encodings.
    pub fn encodings(&self) -> &[ContentEncoding] { &self.encodings }
}


pub struct SetAcceptedEncodingsParamsBuilder {
    encodings: Vec<ContentEncoding>,
}

impl SetAcceptedEncodingsParamsBuilder {
    pub fn build(self) -> SetAcceptedEncodingsParams {
        SetAcceptedEncodingsParams {
            encodings: self.encodings,
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
    /// Creates a builder for this type with the required parameters:
    /// * `result`: True if browser cache can be cleared.
    pub fn builder(result: bool) -> CanClearBrowserCacheReturnsBuilder {
        CanClearBrowserCacheReturnsBuilder {
            result: result,
        }
    }
    /// True if browser cache can be cleared.
    pub fn result(&self) -> bool { self.result }
}


pub struct CanClearBrowserCacheReturnsBuilder {
    result: bool,
}

impl CanClearBrowserCacheReturnsBuilder {
    pub fn build(self) -> CanClearBrowserCacheReturns {
        CanClearBrowserCacheReturns {
            result: self.result,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanClearBrowserCacheParams {}

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
    /// Creates a builder for this type with the required parameters:
    /// * `result`: True if browser cookies can be cleared.
    pub fn builder(result: bool) -> CanClearBrowserCookiesReturnsBuilder {
        CanClearBrowserCookiesReturnsBuilder {
            result: result,
        }
    }
    /// True if browser cookies can be cleared.
    pub fn result(&self) -> bool { self.result }
}


pub struct CanClearBrowserCookiesReturnsBuilder {
    result: bool,
}

impl CanClearBrowserCookiesReturnsBuilder {
    pub fn build(self) -> CanClearBrowserCookiesReturns {
        CanClearBrowserCookiesReturns {
            result: self.result,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanClearBrowserCookiesParams {}

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
    /// Creates a builder for this type with the required parameters:
    /// * `result`: True if emulation of network conditions is supported.
    pub fn builder(result: bool) -> CanEmulateNetworkConditionsReturnsBuilder {
        CanEmulateNetworkConditionsReturnsBuilder {
            result: result,
        }
    }
    /// True if emulation of network conditions is supported.
    pub fn result(&self) -> bool { self.result }
}


pub struct CanEmulateNetworkConditionsReturnsBuilder {
    result: bool,
}

impl CanEmulateNetworkConditionsReturnsBuilder {
    pub fn build(self) -> CanEmulateNetworkConditionsReturns {
        CanEmulateNetworkConditionsReturns {
            result: self.result,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanEmulateNetworkConditionsParams {}

impl CanEmulateNetworkConditionsParams { pub const METHOD: &'static str = "Network.canEmulateNetworkConditions"; }

impl<'a> crate::CdpCommand<'a> for CanEmulateNetworkConditionsParams {
    const METHOD: &'static str = "Network.canEmulateNetworkConditions";
    type Response = CanEmulateNetworkConditionsReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearBrowserCacheParams {}

impl ClearBrowserCacheParams { pub const METHOD: &'static str = "Network.clearBrowserCache"; }

impl<'a> crate::CdpCommand<'a> for ClearBrowserCacheParams {
    const METHOD: &'static str = "Network.clearBrowserCache";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearBrowserCookiesParams {}

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
    #[serde(rename = "interceptionId")]
    interception_id: InterceptionId<'a>,
    /// If set this causes the request to fail with the given reason. Passing 'Aborted' for requests
    /// marked with 'isNavigationRequest' also cancels the navigation. Must not be set in response
    /// to an authChallenge.
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorReason")]
    error_reason: Option<ErrorReason>,
    /// If set the requests completes using with the provided base64 encoded raw response, including
    /// HTTP status line and headers etc... Must not be set in response to an authChallenge. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "rawResponse")]
    raw_response: Option<Cow<'a, str>>,
    /// If set the request url will be modified in a way that's not observable by page. Must not be
    /// set in response to an authChallenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// If set this allows the request method to be overridden. Must not be set in response to an
    /// authChallenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<Cow<'a, str>>,
    /// If set this allows postData to be set. Must not be set in response to an authChallenge.
    #[serde(skip_serializing_if = "Option::is_none", rename = "postData")]
    post_data: Option<Cow<'a, str>>,
    /// If set this allows the request headers to be changed. Must not be set in response to an
    /// authChallenge.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Headers>,
    /// Response to a requestIntercepted with an authChallenge. Must not be set otherwise.
    #[serde(skip_serializing_if = "Option::is_none", rename = "authChallengeResponse")]
    auth_challenge_response: Option<AuthChallengeResponse<'a>>,
}

impl<'a> ContinueInterceptedRequestParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `interception_id`: 
    pub fn builder(interception_id: impl Into<InterceptionId<'a>>) -> ContinueInterceptedRequestParamsBuilder<'a> {
        ContinueInterceptedRequestParamsBuilder {
            interception_id: interception_id.into(),
            error_reason: None,
            raw_response: None,
            url: None,
            method: None,
            post_data: None,
            headers: None,
            auth_challenge_response: None,
        }
    }
    pub fn interception_id(&self) -> &InterceptionId<'a> { &self.interception_id }
    /// If set this causes the request to fail with the given reason. Passing 'Aborted' for requests
    /// marked with 'isNavigationRequest' also cancels the navigation. Must not be set in response
    /// to an authChallenge.
    pub fn error_reason(&self) -> Option<&ErrorReason> { self.error_reason.as_ref() }
    /// If set the requests completes using with the provided base64 encoded raw response, including
    /// HTTP status line and headers etc... Must not be set in response to an authChallenge. (Encoded as a base64 string when passed over JSON)
    pub fn raw_response(&self) -> Option<&str> { self.raw_response.as_deref() }
    /// If set the request url will be modified in a way that's not observable by page. Must not be
    /// set in response to an authChallenge.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// If set this allows the request method to be overridden. Must not be set in response to an
    /// authChallenge.
    pub fn method(&self) -> Option<&str> { self.method.as_deref() }
    /// If set this allows postData to be set. Must not be set in response to an authChallenge.
    pub fn post_data(&self) -> Option<&str> { self.post_data.as_deref() }
    /// If set this allows the request headers to be changed. Must not be set in response to an
    /// authChallenge.
    pub fn headers(&self) -> Option<&Headers> { self.headers.as_ref() }
    /// Response to a requestIntercepted with an authChallenge. Must not be set otherwise.
    pub fn auth_challenge_response(&self) -> Option<&AuthChallengeResponse<'a>> { self.auth_challenge_response.as_ref() }
}


pub struct ContinueInterceptedRequestParamsBuilder<'a> {
    interception_id: InterceptionId<'a>,
    error_reason: Option<ErrorReason>,
    raw_response: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    method: Option<Cow<'a, str>>,
    post_data: Option<Cow<'a, str>>,
    headers: Option<Headers>,
    auth_challenge_response: Option<AuthChallengeResponse<'a>>,
}

impl<'a> ContinueInterceptedRequestParamsBuilder<'a> {
    /// If set this causes the request to fail with the given reason. Passing 'Aborted' for requests
    /// marked with 'isNavigationRequest' also cancels the navigation. Must not be set in response
    /// to an authChallenge.
    pub fn error_reason(mut self, error_reason: impl Into<ErrorReason>) -> Self { self.error_reason = Some(error_reason.into()); self }
    /// If set the requests completes using with the provided base64 encoded raw response, including
    /// HTTP status line and headers etc... Must not be set in response to an authChallenge. (Encoded as a base64 string when passed over JSON)
    pub fn raw_response(mut self, raw_response: impl Into<Cow<'a, str>>) -> Self { self.raw_response = Some(raw_response.into()); self }
    /// If set the request url will be modified in a way that's not observable by page. Must not be
    /// set in response to an authChallenge.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// If set this allows the request method to be overridden. Must not be set in response to an
    /// authChallenge.
    pub fn method(mut self, method: impl Into<Cow<'a, str>>) -> Self { self.method = Some(method.into()); self }
    /// If set this allows postData to be set. Must not be set in response to an authChallenge.
    pub fn post_data(mut self, post_data: impl Into<Cow<'a, str>>) -> Self { self.post_data = Some(post_data.into()); self }
    /// If set this allows the request headers to be changed. Must not be set in response to an
    /// authChallenge.
    pub fn headers(mut self, headers: Headers) -> Self { self.headers = Some(headers); self }
    /// Response to a requestIntercepted with an authChallenge. Must not be set otherwise.
    pub fn auth_challenge_response(mut self, auth_challenge_response: AuthChallengeResponse<'a>) -> Self { self.auth_challenge_response = Some(auth_challenge_response); self }
    pub fn build(self) -> ContinueInterceptedRequestParams<'a> {
        ContinueInterceptedRequestParams {
            interception_id: self.interception_id,
            error_reason: self.error_reason,
            raw_response: self.raw_response,
            url: self.url,
            method: self.method,
            post_data: self.post_data,
            headers: self.headers,
            auth_challenge_response: self.auth_challenge_response,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKey")]
    partition_key: Option<CookiePartitionKey<'a>>,
}

impl<'a> DeleteCookiesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Name of the cookies to remove.
    pub fn builder(name: impl Into<Cow<'a, str>>) -> DeleteCookiesParamsBuilder<'a> {
        DeleteCookiesParamsBuilder {
            name: name.into(),
            url: None,
            domain: None,
            path: None,
            partition_key: None,
        }
    }
    /// Name of the cookies to remove.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// If specified, deletes all the cookies with the given name where domain and path match
    /// provided URL.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// If specified, deletes only cookies with the exact domain.
    pub fn domain(&self) -> Option<&str> { self.domain.as_deref() }
    /// If specified, deletes only cookies with the exact path.
    pub fn path(&self) -> Option<&str> { self.path.as_deref() }
    /// If specified, deletes only cookies with the the given name and partitionKey where
    /// all partition key attributes match the cookie partition key attribute.
    pub fn partition_key(&self) -> Option<&CookiePartitionKey<'a>> { self.partition_key.as_ref() }
}


pub struct DeleteCookiesParamsBuilder<'a> {
    name: Cow<'a, str>,
    url: Option<Cow<'a, str>>,
    domain: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    partition_key: Option<CookiePartitionKey<'a>>,
}

impl<'a> DeleteCookiesParamsBuilder<'a> {
    /// If specified, deletes all the cookies with the given name where domain and path match
    /// provided URL.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// If specified, deletes only cookies with the exact domain.
    pub fn domain(mut self, domain: impl Into<Cow<'a, str>>) -> Self { self.domain = Some(domain.into()); self }
    /// If specified, deletes only cookies with the exact path.
    pub fn path(mut self, path: impl Into<Cow<'a, str>>) -> Self { self.path = Some(path.into()); self }
    /// If specified, deletes only cookies with the the given name and partitionKey where
    /// all partition key attributes match the cookie partition key attribute.
    pub fn partition_key(mut self, partition_key: CookiePartitionKey<'a>) -> Self { self.partition_key = Some(partition_key); self }
    pub fn build(self) -> DeleteCookiesParams<'a> {
        DeleteCookiesParams {
            name: self.name,
            url: self.url,
            domain: self.domain,
            path: self.path,
            partition_key: self.partition_key,
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
    #[serde(rename = "downloadThroughput")]
    download_throughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    #[serde(rename = "uploadThroughput")]
    upload_throughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectionType")]
    connection_type: Option<ConnectionType>,
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetLoss")]
    packet_loss: Option<f64>,
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetQueueLength")]
    packet_queue_length: Option<u64>,
    /// WebRTC packetReordering feature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "packetReordering")]
    packet_reordering: Option<bool>,
}

impl EmulateNetworkConditionsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `offline`: True to emulate internet disconnection.
    /// * `latency`: Minimum latency from request sent to response headers received (ms).
    /// * `download_throughput`: Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    /// * `upload_throughput`: Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub fn builder(offline: bool, latency: f64, download_throughput: f64, upload_throughput: f64) -> EmulateNetworkConditionsParamsBuilder {
        EmulateNetworkConditionsParamsBuilder {
            offline: offline,
            latency: latency,
            download_throughput: download_throughput,
            upload_throughput: upload_throughput,
            connection_type: None,
            packet_loss: None,
            packet_queue_length: None,
            packet_reordering: None,
        }
    }
    /// True to emulate internet disconnection.
    pub fn offline(&self) -> bool { self.offline }
    /// Minimum latency from request sent to response headers received (ms).
    pub fn latency(&self) -> f64 { self.latency }
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    pub fn download_throughput(&self) -> f64 { self.download_throughput }
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub fn upload_throughput(&self) -> f64 { self.upload_throughput }
    /// Connection type if known.
    pub fn connection_type(&self) -> Option<&ConnectionType> { self.connection_type.as_ref() }
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    pub fn packet_loss(&self) -> Option<f64> { self.packet_loss }
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    pub fn packet_queue_length(&self) -> Option<u64> { self.packet_queue_length }
    /// WebRTC packetReordering feature.
    pub fn packet_reordering(&self) -> Option<bool> { self.packet_reordering }
}


pub struct EmulateNetworkConditionsParamsBuilder {
    offline: bool,
    latency: f64,
    download_throughput: f64,
    upload_throughput: f64,
    connection_type: Option<ConnectionType>,
    packet_loss: Option<f64>,
    packet_queue_length: Option<u64>,
    packet_reordering: Option<bool>,
}

impl EmulateNetworkConditionsParamsBuilder {
    /// Connection type if known.
    pub fn connection_type(mut self, connection_type: impl Into<ConnectionType>) -> Self { self.connection_type = Some(connection_type.into()); self }
    /// WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.
    pub fn packet_loss(mut self, packet_loss: f64) -> Self { self.packet_loss = Some(packet_loss); self }
    /// WebRTC packet queue length (packet). 0 removes any queue length limitations.
    pub fn packet_queue_length(mut self, packet_queue_length: u64) -> Self { self.packet_queue_length = Some(packet_queue_length); self }
    /// WebRTC packetReordering feature.
    pub fn packet_reordering(mut self, packet_reordering: bool) -> Self { self.packet_reordering = Some(packet_reordering); self }
    pub fn build(self) -> EmulateNetworkConditionsParams {
        EmulateNetworkConditionsParams {
            offline: self.offline,
            latency: self.latency,
            download_throughput: self.download_throughput,
            upload_throughput: self.upload_throughput,
            connection_type: self.connection_type,
            packet_loss: self.packet_loss,
            packet_queue_length: self.packet_queue_length,
            packet_reordering: self.packet_reordering,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "emulateOfflineServiceWorker")]
    emulate_offline_service_worker: Option<bool>,
    /// Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global
    /// conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are
    /// also applied for throttling of p2p connections.
    #[serde(rename = "matchedNetworkConditions")]
    matched_network_conditions: Vec<NetworkConditions<'a>>,
}

impl<'a> EmulateNetworkConditionsByRuleParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `matched_network_conditions`: Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are also applied for throttling of p2p connections.
    pub fn builder(matched_network_conditions: Vec<NetworkConditions<'a>>) -> EmulateNetworkConditionsByRuleParamsBuilder<'a> {
        EmulateNetworkConditionsByRuleParamsBuilder {
            offline: None,
            emulate_offline_service_worker: None,
            matched_network_conditions: matched_network_conditions,
        }
    }
    /// True to emulate internet disconnection. Deprecated, use the offline property in matchedNetworkConditions
    /// or emulateOfflineServiceWorker instead.
    pub fn offline(&self) -> Option<bool> { self.offline }
    /// True to emulate offline service worker.
    pub fn emulate_offline_service_worker(&self) -> Option<bool> { self.emulate_offline_service_worker }
    /// Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global
    /// conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are
    /// also applied for throttling of p2p connections.
    pub fn matched_network_conditions(&self) -> &[NetworkConditions<'a>] { &self.matched_network_conditions }
}


pub struct EmulateNetworkConditionsByRuleParamsBuilder<'a> {
    offline: Option<bool>,
    emulate_offline_service_worker: Option<bool>,
    matched_network_conditions: Vec<NetworkConditions<'a>>,
}

impl<'a> EmulateNetworkConditionsByRuleParamsBuilder<'a> {
    /// True to emulate internet disconnection. Deprecated, use the offline property in matchedNetworkConditions
    /// or emulateOfflineServiceWorker instead.
    pub fn offline(mut self, offline: bool) -> Self { self.offline = Some(offline); self }
    /// True to emulate offline service worker.
    pub fn emulate_offline_service_worker(mut self, emulate_offline_service_worker: bool) -> Self { self.emulate_offline_service_worker = Some(emulate_offline_service_worker); self }
    pub fn build(self) -> EmulateNetworkConditionsByRuleParams<'a> {
        EmulateNetworkConditionsByRuleParams {
            offline: self.offline,
            emulate_offline_service_worker: self.emulate_offline_service_worker,
            matched_network_conditions: self.matched_network_conditions,
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
    #[serde(rename = "ruleIds")]
    rule_ids: Vec<Cow<'a, str>>,
}

impl<'a> EmulateNetworkConditionsByRuleReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `rule_ids`: An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for requests affected by a rule.
    pub fn builder(rule_ids: Vec<Cow<'a, str>>) -> EmulateNetworkConditionsByRuleReturnsBuilder<'a> {
        EmulateNetworkConditionsByRuleReturnsBuilder {
            rule_ids: rule_ids,
        }
    }
    /// An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for
    /// requests affected by a rule.
    pub fn rule_ids(&self) -> &[Cow<'a, str>] { &self.rule_ids }
}


pub struct EmulateNetworkConditionsByRuleReturnsBuilder<'a> {
    rule_ids: Vec<Cow<'a, str>>,
}

impl<'a> EmulateNetworkConditionsByRuleReturnsBuilder<'a> {
    pub fn build(self) -> EmulateNetworkConditionsByRuleReturns<'a> {
        EmulateNetworkConditionsByRuleReturns {
            rule_ids: self.rule_ids,
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
    #[serde(rename = "downloadThroughput")]
    download_throughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    #[serde(rename = "uploadThroughput")]
    upload_throughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectionType")]
    connection_type: Option<ConnectionType>,
}

impl OverrideNetworkStateParams {
    /// Creates a builder for this type with the required parameters:
    /// * `offline`: True to emulate internet disconnection.
    /// * `latency`: Minimum latency from request sent to response headers received (ms).
    /// * `download_throughput`: Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    /// * `upload_throughput`: Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub fn builder(offline: bool, latency: f64, download_throughput: f64, upload_throughput: f64) -> OverrideNetworkStateParamsBuilder {
        OverrideNetworkStateParamsBuilder {
            offline: offline,
            latency: latency,
            download_throughput: download_throughput,
            upload_throughput: upload_throughput,
            connection_type: None,
        }
    }
    /// True to emulate internet disconnection.
    pub fn offline(&self) -> bool { self.offline }
    /// Minimum latency from request sent to response headers received (ms).
    pub fn latency(&self) -> f64 { self.latency }
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    pub fn download_throughput(&self) -> f64 { self.download_throughput }
    /// Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub fn upload_throughput(&self) -> f64 { self.upload_throughput }
    /// Connection type if known.
    pub fn connection_type(&self) -> Option<&ConnectionType> { self.connection_type.as_ref() }
}


pub struct OverrideNetworkStateParamsBuilder {
    offline: bool,
    latency: f64,
    download_throughput: f64,
    upload_throughput: f64,
    connection_type: Option<ConnectionType>,
}

impl OverrideNetworkStateParamsBuilder {
    /// Connection type if known.
    pub fn connection_type(mut self, connection_type: impl Into<ConnectionType>) -> Self { self.connection_type = Some(connection_type.into()); self }
    pub fn build(self) -> OverrideNetworkStateParams {
        OverrideNetworkStateParams {
            offline: self.offline,
            latency: self.latency,
            download_throughput: self.download_throughput,
            upload_throughput: self.upload_throughput,
            connection_type: self.connection_type,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxTotalBufferSize")]
    max_total_buffer_size: Option<u64>,
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxResourceBufferSize")]
    max_resource_buffer_size: Option<u64>,
    /// Longest post body size (in bytes) that would be included in requestWillBeSent notification
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxPostDataSize")]
    max_post_data_size: Option<u64>,
    /// Whether DirectSocket chunk send/receive events should be reported.
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportDirectSocketTraffic")]
    report_direct_socket_traffic: Option<bool>,
    /// Enable storing response bodies outside of renderer, so that these survive
    /// a cross-process navigation. Requires maxTotalBufferSize to be set.
    /// Currently defaults to false. This field is being deprecated in favor of the dedicated
    /// configureDurableMessages command, due to the possibility of deadlocks when awaiting
    /// Network.enable before issuing Runtime.runIfWaitingForDebugger.
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableDurableMessages")]
    enable_durable_messages: Option<bool>,
}

impl EnableParams {
    /// Creates a builder for this type.
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder {
            max_total_buffer_size: None,
            max_resource_buffer_size: None,
            max_post_data_size: None,
            report_direct_socket_traffic: None,
            enable_durable_messages: None,
        }
    }
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    /// This is the maximum number of bytes that will be collected by this
    /// DevTools session.
    pub fn max_total_buffer_size(&self) -> Option<u64> { self.max_total_buffer_size }
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub fn max_resource_buffer_size(&self) -> Option<u64> { self.max_resource_buffer_size }
    /// Longest post body size (in bytes) that would be included in requestWillBeSent notification
    pub fn max_post_data_size(&self) -> Option<u64> { self.max_post_data_size }
    /// Whether DirectSocket chunk send/receive events should be reported.
    pub fn report_direct_socket_traffic(&self) -> Option<bool> { self.report_direct_socket_traffic }
    /// Enable storing response bodies outside of renderer, so that these survive
    /// a cross-process navigation. Requires maxTotalBufferSize to be set.
    /// Currently defaults to false. This field is being deprecated in favor of the dedicated
    /// configureDurableMessages command, due to the possibility of deadlocks when awaiting
    /// Network.enable before issuing Runtime.runIfWaitingForDebugger.
    pub fn enable_durable_messages(&self) -> Option<bool> { self.enable_durable_messages }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    max_total_buffer_size: Option<u64>,
    max_resource_buffer_size: Option<u64>,
    max_post_data_size: Option<u64>,
    report_direct_socket_traffic: Option<bool>,
    enable_durable_messages: Option<bool>,
}

impl EnableParamsBuilder {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    /// This is the maximum number of bytes that will be collected by this
    /// DevTools session.
    pub fn max_total_buffer_size(mut self, max_total_buffer_size: u64) -> Self { self.max_total_buffer_size = Some(max_total_buffer_size); self }
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub fn max_resource_buffer_size(mut self, max_resource_buffer_size: u64) -> Self { self.max_resource_buffer_size = Some(max_resource_buffer_size); self }
    /// Longest post body size (in bytes) that would be included in requestWillBeSent notification
    pub fn max_post_data_size(mut self, max_post_data_size: u64) -> Self { self.max_post_data_size = Some(max_post_data_size); self }
    /// Whether DirectSocket chunk send/receive events should be reported.
    pub fn report_direct_socket_traffic(mut self, report_direct_socket_traffic: bool) -> Self { self.report_direct_socket_traffic = Some(report_direct_socket_traffic); self }
    /// Enable storing response bodies outside of renderer, so that these survive
    /// a cross-process navigation. Requires maxTotalBufferSize to be set.
    /// Currently defaults to false. This field is being deprecated in favor of the dedicated
    /// configureDurableMessages command, due to the possibility of deadlocks when awaiting
    /// Network.enable before issuing Runtime.runIfWaitingForDebugger.
    pub fn enable_durable_messages(mut self, enable_durable_messages: bool) -> Self { self.enable_durable_messages = Some(enable_durable_messages); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            max_total_buffer_size: self.max_total_buffer_size,
            max_resource_buffer_size: self.max_resource_buffer_size,
            max_post_data_size: self.max_post_data_size,
            report_direct_socket_traffic: self.report_direct_socket_traffic,
            enable_durable_messages: self.enable_durable_messages,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxTotalBufferSize")]
    max_total_buffer_size: Option<u64>,
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxResourceBufferSize")]
    max_resource_buffer_size: Option<u64>,
}

impl ConfigureDurableMessagesParams {
    /// Creates a builder for this type.
    pub fn builder() -> ConfigureDurableMessagesParamsBuilder {
        ConfigureDurableMessagesParamsBuilder {
            max_total_buffer_size: None,
            max_resource_buffer_size: None,
        }
    }
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub fn max_total_buffer_size(&self) -> Option<u64> { self.max_total_buffer_size }
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub fn max_resource_buffer_size(&self) -> Option<u64> { self.max_resource_buffer_size }
}

#[derive(Default)]
pub struct ConfigureDurableMessagesParamsBuilder {
    max_total_buffer_size: Option<u64>,
    max_resource_buffer_size: Option<u64>,
}

impl ConfigureDurableMessagesParamsBuilder {
    /// Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub fn max_total_buffer_size(mut self, max_total_buffer_size: u64) -> Self { self.max_total_buffer_size = Some(max_total_buffer_size); self }
    /// Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub fn max_resource_buffer_size(mut self, max_resource_buffer_size: u64) -> Self { self.max_resource_buffer_size = Some(max_resource_buffer_size); self }
    pub fn build(self) -> ConfigureDurableMessagesParams {
        ConfigureDurableMessagesParams {
            max_total_buffer_size: self.max_total_buffer_size,
            max_resource_buffer_size: self.max_resource_buffer_size,
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
    /// Creates a builder for this type with the required parameters:
    /// * `cookies`: Array of cookie objects.
    pub fn builder(cookies: Vec<Cookie<'a>>) -> GetAllCookiesReturnsBuilder<'a> {
        GetAllCookiesReturnsBuilder {
            cookies: cookies,
        }
    }
    /// Array of cookie objects.
    pub fn cookies(&self) -> &[Cookie<'a>] { &self.cookies }
}


pub struct GetAllCookiesReturnsBuilder<'a> {
    cookies: Vec<Cookie<'a>>,
}

impl<'a> GetAllCookiesReturnsBuilder<'a> {
    pub fn build(self) -> GetAllCookiesReturns<'a> {
        GetAllCookiesReturns {
            cookies: self.cookies,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAllCookiesParams {}

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
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Origin to get certificate for.
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> GetCertificateParamsBuilder<'a> {
        GetCertificateParamsBuilder {
            origin: origin.into(),
        }
    }
    /// Origin to get certificate for.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}


pub struct GetCertificateParamsBuilder<'a> {
    origin: Cow<'a, str>,
}

impl<'a> GetCertificateParamsBuilder<'a> {
    pub fn build(self) -> GetCertificateParams<'a> {
        GetCertificateParams {
            origin: self.origin,
        }
    }
}

/// Returns the DER-encoded certificate.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateReturns<'a> {
    #[serde(rename = "tableNames")]
    table_names: Vec<Cow<'a, str>>,
}

impl<'a> GetCertificateReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `table_names`: 
    pub fn builder(table_names: Vec<Cow<'a, str>>) -> GetCertificateReturnsBuilder<'a> {
        GetCertificateReturnsBuilder {
            table_names: table_names,
        }
    }
    pub fn table_names(&self) -> &[Cow<'a, str>] { &self.table_names }
}


pub struct GetCertificateReturnsBuilder<'a> {
    table_names: Vec<Cow<'a, str>>,
}

impl<'a> GetCertificateReturnsBuilder<'a> {
    pub fn build(self) -> GetCertificateReturns<'a> {
        GetCertificateReturns {
            table_names: self.table_names,
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
    /// Creates a builder for this type.
    pub fn builder() -> GetCookiesParamsBuilder<'a> {
        GetCookiesParamsBuilder {
            urls: None,
        }
    }
    /// The list of URLs for which applicable cookies will be fetched.
    /// If not specified, it's assumed to be set to the list containing
    /// the URLs of the page and all of its subframes.
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
    /// Creates a builder for this type with the required parameters:
    /// * `cookies`: Array of cookie objects.
    pub fn builder(cookies: Vec<Cookie<'a>>) -> GetCookiesReturnsBuilder<'a> {
        GetCookiesReturnsBuilder {
            cookies: cookies,
        }
    }
    /// Array of cookie objects.
    pub fn cookies(&self) -> &[Cookie<'a>] { &self.cookies }
}


pub struct GetCookiesReturnsBuilder<'a> {
    cookies: Vec<Cookie<'a>>,
}

impl<'a> GetCookiesReturnsBuilder<'a> {
    pub fn build(self) -> GetCookiesReturns<'a> {
        GetCookiesReturns {
            cookies: self.cookies,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
}

impl<'a> GetResponseBodyParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: Identifier of the network request to get content for.
    pub fn builder(request_id: impl Into<RequestId<'a>>) -> GetResponseBodyParamsBuilder<'a> {
        GetResponseBodyParamsBuilder {
            request_id: request_id.into(),
        }
    }
    /// Identifier of the network request to get content for.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
}


pub struct GetResponseBodyParamsBuilder<'a> {
    request_id: RequestId<'a>,
}

impl<'a> GetResponseBodyParamsBuilder<'a> {
    pub fn build(self) -> GetResponseBodyParams<'a> {
        GetResponseBodyParams {
            request_id: self.request_id,
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
    #[serde(rename = "base64Encoded")]
    base64_encoded: bool,
}

impl<'a> GetResponseBodyReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `body`: Response body.
    /// * `base64_encoded`: True, if content was sent as base64.
    pub fn builder(body: impl Into<Cow<'a, str>>, base64_encoded: bool) -> GetResponseBodyReturnsBuilder<'a> {
        GetResponseBodyReturnsBuilder {
            body: body.into(),
            base64_encoded: base64_encoded,
        }
    }
    /// Response body.
    pub fn body(&self) -> &str { self.body.as_ref() }
    /// True, if content was sent as base64.
    pub fn base64_encoded(&self) -> bool { self.base64_encoded }
}


pub struct GetResponseBodyReturnsBuilder<'a> {
    body: Cow<'a, str>,
    base64_encoded: bool,
}

impl<'a> GetResponseBodyReturnsBuilder<'a> {
    pub fn build(self) -> GetResponseBodyReturns<'a> {
        GetResponseBodyReturns {
            body: self.body,
            base64_encoded: self.base64_encoded,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
}

impl<'a> GetRequestPostDataParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: Identifier of the network request to get content for.
    pub fn builder(request_id: impl Into<RequestId<'a>>) -> GetRequestPostDataParamsBuilder<'a> {
        GetRequestPostDataParamsBuilder {
            request_id: request_id.into(),
        }
    }
    /// Identifier of the network request to get content for.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
}


pub struct GetRequestPostDataParamsBuilder<'a> {
    request_id: RequestId<'a>,
}

impl<'a> GetRequestPostDataParamsBuilder<'a> {
    pub fn build(self) -> GetRequestPostDataParams<'a> {
        GetRequestPostDataParams {
            request_id: self.request_id,
        }
    }
}

/// Returns post data sent with the request. Returns an error when no data was sent with the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRequestPostDataReturns<'a> {
    /// Request body string, omitting files from multipart requests
    #[serde(rename = "postData")]
    post_data: Cow<'a, str>,
    /// True, if content was sent as base64.
    #[serde(rename = "base64Encoded")]
    base64_encoded: bool,
}

impl<'a> GetRequestPostDataReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `post_data`: Request body string, omitting files from multipart requests
    /// * `base64_encoded`: True, if content was sent as base64.
    pub fn builder(post_data: impl Into<Cow<'a, str>>, base64_encoded: bool) -> GetRequestPostDataReturnsBuilder<'a> {
        GetRequestPostDataReturnsBuilder {
            post_data: post_data.into(),
            base64_encoded: base64_encoded,
        }
    }
    /// Request body string, omitting files from multipart requests
    pub fn post_data(&self) -> &str { self.post_data.as_ref() }
    /// True, if content was sent as base64.
    pub fn base64_encoded(&self) -> bool { self.base64_encoded }
}


pub struct GetRequestPostDataReturnsBuilder<'a> {
    post_data: Cow<'a, str>,
    base64_encoded: bool,
}

impl<'a> GetRequestPostDataReturnsBuilder<'a> {
    pub fn build(self) -> GetRequestPostDataReturns<'a> {
        GetRequestPostDataReturns {
            post_data: self.post_data,
            base64_encoded: self.base64_encoded,
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
    #[serde(rename = "interceptionId")]
    interception_id: InterceptionId<'a>,
}

impl<'a> GetResponseBodyForInterceptionParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `interception_id`: Identifier for the intercepted request to get body for.
    pub fn builder(interception_id: impl Into<InterceptionId<'a>>) -> GetResponseBodyForInterceptionParamsBuilder<'a> {
        GetResponseBodyForInterceptionParamsBuilder {
            interception_id: interception_id.into(),
        }
    }
    /// Identifier for the intercepted request to get body for.
    pub fn interception_id(&self) -> &InterceptionId<'a> { &self.interception_id }
}


pub struct GetResponseBodyForInterceptionParamsBuilder<'a> {
    interception_id: InterceptionId<'a>,
}

impl<'a> GetResponseBodyForInterceptionParamsBuilder<'a> {
    pub fn build(self) -> GetResponseBodyForInterceptionParams<'a> {
        GetResponseBodyForInterceptionParams {
            interception_id: self.interception_id,
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
    #[serde(rename = "base64Encoded")]
    base64_encoded: bool,
}

impl<'a> GetResponseBodyForInterceptionReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `body`: Response body.
    /// * `base64_encoded`: True, if content was sent as base64.
    pub fn builder(body: impl Into<Cow<'a, str>>, base64_encoded: bool) -> GetResponseBodyForInterceptionReturnsBuilder<'a> {
        GetResponseBodyForInterceptionReturnsBuilder {
            body: body.into(),
            base64_encoded: base64_encoded,
        }
    }
    /// Response body.
    pub fn body(&self) -> &str { self.body.as_ref() }
    /// True, if content was sent as base64.
    pub fn base64_encoded(&self) -> bool { self.base64_encoded }
}


pub struct GetResponseBodyForInterceptionReturnsBuilder<'a> {
    body: Cow<'a, str>,
    base64_encoded: bool,
}

impl<'a> GetResponseBodyForInterceptionReturnsBuilder<'a> {
    pub fn build(self) -> GetResponseBodyForInterceptionReturns<'a> {
        GetResponseBodyForInterceptionReturns {
            body: self.body,
            base64_encoded: self.base64_encoded,
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
    #[serde(rename = "interceptionId")]
    interception_id: InterceptionId<'a>,
}

impl<'a> TakeResponseBodyForInterceptionAsStreamParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `interception_id`: 
    pub fn builder(interception_id: impl Into<InterceptionId<'a>>) -> TakeResponseBodyForInterceptionAsStreamParamsBuilder<'a> {
        TakeResponseBodyForInterceptionAsStreamParamsBuilder {
            interception_id: interception_id.into(),
        }
    }
    pub fn interception_id(&self) -> &InterceptionId<'a> { &self.interception_id }
}


pub struct TakeResponseBodyForInterceptionAsStreamParamsBuilder<'a> {
    interception_id: InterceptionId<'a>,
}

impl<'a> TakeResponseBodyForInterceptionAsStreamParamsBuilder<'a> {
    pub fn build(self) -> TakeResponseBodyForInterceptionAsStreamParams<'a> {
        TakeResponseBodyForInterceptionAsStreamParams {
            interception_id: self.interception_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `stream`: 
    pub fn builder(stream: crate::io::StreamHandle<'a>) -> TakeResponseBodyForInterceptionAsStreamReturnsBuilder<'a> {
        TakeResponseBodyForInterceptionAsStreamReturnsBuilder {
            stream: stream,
        }
    }
    pub fn stream(&self) -> &crate::io::StreamHandle<'a> { &self.stream }
}


pub struct TakeResponseBodyForInterceptionAsStreamReturnsBuilder<'a> {
    stream: crate::io::StreamHandle<'a>,
}

impl<'a> TakeResponseBodyForInterceptionAsStreamReturnsBuilder<'a> {
    pub fn build(self) -> TakeResponseBodyForInterceptionAsStreamReturns<'a> {
        TakeResponseBodyForInterceptionAsStreamReturns {
            stream: self.stream,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
}

impl<'a> ReplayXHRParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: Identifier of XHR to replay.
    pub fn builder(request_id: impl Into<RequestId<'a>>) -> ReplayXHRParamsBuilder<'a> {
        ReplayXHRParamsBuilder {
            request_id: request_id.into(),
        }
    }
    /// Identifier of XHR to replay.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
}


pub struct ReplayXHRParamsBuilder<'a> {
    request_id: RequestId<'a>,
}

impl<'a> ReplayXHRParamsBuilder<'a> {
    pub fn build(self) -> ReplayXHRParams<'a> {
        ReplayXHRParams {
            request_id: self.request_id,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
    /// String to search for.
    query: Cow<'a, str>,
    /// If true, search is case sensitive.
    #[serde(skip_serializing_if = "Option::is_none", rename = "caseSensitive")]
    case_sensitive: Option<bool>,
    /// If true, treats string parameter as regex.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isRegex")]
    is_regex: Option<bool>,
}

impl<'a> SearchInResponseBodyParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: Identifier of the network response to search.
    /// * `query`: String to search for.
    pub fn builder(request_id: impl Into<RequestId<'a>>, query: impl Into<Cow<'a, str>>) -> SearchInResponseBodyParamsBuilder<'a> {
        SearchInResponseBodyParamsBuilder {
            request_id: request_id.into(),
            query: query.into(),
            case_sensitive: None,
            is_regex: None,
        }
    }
    /// Identifier of the network response to search.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
    /// String to search for.
    pub fn query(&self) -> &str { self.query.as_ref() }
    /// If true, search is case sensitive.
    pub fn case_sensitive(&self) -> Option<bool> { self.case_sensitive }
    /// If true, treats string parameter as regex.
    pub fn is_regex(&self) -> Option<bool> { self.is_regex }
}


pub struct SearchInResponseBodyParamsBuilder<'a> {
    request_id: RequestId<'a>,
    query: Cow<'a, str>,
    case_sensitive: Option<bool>,
    is_regex: Option<bool>,
}

impl<'a> SearchInResponseBodyParamsBuilder<'a> {
    /// If true, search is case sensitive.
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self { self.case_sensitive = Some(case_sensitive); self }
    /// If true, treats string parameter as regex.
    pub fn is_regex(mut self, is_regex: bool) -> Self { self.is_regex = Some(is_regex); self }
    pub fn build(self) -> SearchInResponseBodyParams<'a> {
        SearchInResponseBodyParams {
            request_id: self.request_id,
            query: self.query,
            case_sensitive: self.case_sensitive,
            is_regex: self.is_regex,
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
    /// Creates a builder for this type with the required parameters:
    /// * `result`: List of search matches.
    pub fn builder(result: Vec<crate::debugger::SearchMatch>) -> SearchInResponseBodyReturnsBuilder {
        SearchInResponseBodyReturnsBuilder {
            result: result,
        }
    }
    /// List of search matches.
    pub fn result(&self) -> &[crate::debugger::SearchMatch] { &self.result }
}


pub struct SearchInResponseBodyReturnsBuilder {
    result: Vec<crate::debugger::SearchMatch>,
}

impl SearchInResponseBodyReturnsBuilder {
    pub fn build(self) -> SearchInResponseBodyReturns {
        SearchInResponseBodyReturns {
            result: self.result,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlPatterns")]
    url_patterns: Option<Vec<BlockPattern<'a>>>,
    /// URL patterns to block. Wildcards ('*') are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    urls: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SetBlockedURLsParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> SetBlockedURLsParamsBuilder<'a> {
        SetBlockedURLsParamsBuilder {
            url_patterns: None,
            urls: None,
        }
    }
    /// Patterns to match in the order in which they are given. These patterns
    /// also take precedence over any wildcard patterns defined in 'urls'.
    pub fn url_patterns(&self) -> Option<&[BlockPattern<'a>]> { self.url_patterns.as_deref() }
    /// URL patterns to block. Wildcards ('*') are allowed.
    pub fn urls(&self) -> Option<&[Cow<'a, str>]> { self.urls.as_deref() }
}

#[derive(Default)]
pub struct SetBlockedURLsParamsBuilder<'a> {
    url_patterns: Option<Vec<BlockPattern<'a>>>,
    urls: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SetBlockedURLsParamsBuilder<'a> {
    /// Patterns to match in the order in which they are given. These patterns
    /// also take precedence over any wildcard patterns defined in 'urls'.
    pub fn url_patterns(mut self, url_patterns: Vec<BlockPattern<'a>>) -> Self { self.url_patterns = Some(url_patterns); self }
    /// URL patterns to block. Wildcards ('*') are allowed.
    pub fn urls(mut self, urls: Vec<Cow<'a, str>>) -> Self { self.urls = Some(urls); self }
    pub fn build(self) -> SetBlockedURLsParams<'a> {
        SetBlockedURLsParams {
            url_patterns: self.url_patterns,
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
    /// Creates a builder for this type with the required parameters:
    /// * `bypass`: Bypass service worker and load from network.
    pub fn builder(bypass: bool) -> SetBypassServiceWorkerParamsBuilder {
        SetBypassServiceWorkerParamsBuilder {
            bypass: bypass,
        }
    }
    /// Bypass service worker and load from network.
    pub fn bypass(&self) -> bool { self.bypass }
}


pub struct SetBypassServiceWorkerParamsBuilder {
    bypass: bool,
}

impl SetBypassServiceWorkerParamsBuilder {
    pub fn build(self) -> SetBypassServiceWorkerParams {
        SetBypassServiceWorkerParams {
            bypass: self.bypass,
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
    #[serde(rename = "cacheDisabled")]
    cache_disabled: bool,
}

impl SetCacheDisabledParams {
    /// Creates a builder for this type with the required parameters:
    /// * `cache_disabled`: Cache disabled state.
    pub fn builder(cache_disabled: bool) -> SetCacheDisabledParamsBuilder {
        SetCacheDisabledParamsBuilder {
            cache_disabled: cache_disabled,
        }
    }
    /// Cache disabled state.
    pub fn cache_disabled(&self) -> bool { self.cache_disabled }
}


pub struct SetCacheDisabledParamsBuilder {
    cache_disabled: bool,
}

impl SetCacheDisabledParamsBuilder {
    pub fn build(self) -> SetCacheDisabledParams {
        SetCacheDisabledParams {
            cache_disabled: self.cache_disabled,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "httpOnly")]
    http_only: Option<bool>,
    /// Cookie SameSite type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sameSite")]
    same_site: Option<CookieSameSite>,
    /// Cookie expiration date, session cookie if not set
    #[serde(skip_serializing_if = "Option::is_none")]
    expires: Option<TimeSinceEpoch>,
    /// Cookie Priority type.
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<CookiePriority>,
    /// Cookie source scheme type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceScheme")]
    source_scheme: Option<CookieSourceScheme>,
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourcePort")]
    source_port: Option<i64>,
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKey")]
    partition_key: Option<CookiePartitionKey<'a>>,
}

impl<'a> SetCookieParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Cookie name.
    /// * `value`: Cookie value.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> SetCookieParamsBuilder<'a> {
        SetCookieParamsBuilder {
            name: name.into(),
            value: value.into(),
            url: None,
            domain: None,
            path: None,
            secure: None,
            http_only: None,
            same_site: None,
            expires: None,
            priority: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        }
    }
    /// Cookie name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Cookie value.
    pub fn value(&self) -> &str { self.value.as_ref() }
    /// The request-URI to associate with the setting of the cookie. This value can affect the
    /// default domain, path, source port, and source scheme values of the created cookie.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// Cookie domain.
    pub fn domain(&self) -> Option<&str> { self.domain.as_deref() }
    /// Cookie path.
    pub fn path(&self) -> Option<&str> { self.path.as_deref() }
    /// True if cookie is secure.
    pub fn secure(&self) -> Option<bool> { self.secure }
    /// True if cookie is http-only.
    pub fn http_only(&self) -> Option<bool> { self.http_only }
    /// Cookie SameSite type.
    pub fn same_site(&self) -> Option<&CookieSameSite> { self.same_site.as_ref() }
    /// Cookie expiration date, session cookie if not set
    pub fn expires(&self) -> Option<&TimeSinceEpoch> { self.expires.as_ref() }
    /// Cookie Priority type.
    pub fn priority(&self) -> Option<&CookiePriority> { self.priority.as_ref() }
    /// Cookie source scheme type.
    pub fn source_scheme(&self) -> Option<&CookieSourceScheme> { self.source_scheme.as_ref() }
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    pub fn source_port(&self) -> Option<i64> { self.source_port }
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    pub fn partition_key(&self) -> Option<&CookiePartitionKey<'a>> { self.partition_key.as_ref() }
}


pub struct SetCookieParamsBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    url: Option<Cow<'a, str>>,
    domain: Option<Cow<'a, str>>,
    path: Option<Cow<'a, str>>,
    secure: Option<bool>,
    http_only: Option<bool>,
    same_site: Option<CookieSameSite>,
    expires: Option<TimeSinceEpoch>,
    priority: Option<CookiePriority>,
    source_scheme: Option<CookieSourceScheme>,
    source_port: Option<i64>,
    partition_key: Option<CookiePartitionKey<'a>>,
}

impl<'a> SetCookieParamsBuilder<'a> {
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
    pub fn http_only(mut self, http_only: bool) -> Self { self.http_only = Some(http_only); self }
    /// Cookie SameSite type.
    pub fn same_site(mut self, same_site: impl Into<CookieSameSite>) -> Self { self.same_site = Some(same_site.into()); self }
    /// Cookie expiration date, session cookie if not set
    pub fn expires(mut self, expires: TimeSinceEpoch) -> Self { self.expires = Some(expires); self }
    /// Cookie Priority type.
    pub fn priority(mut self, priority: impl Into<CookiePriority>) -> Self { self.priority = Some(priority.into()); self }
    /// Cookie source scheme type.
    pub fn source_scheme(mut self, source_scheme: impl Into<CookieSourceScheme>) -> Self { self.source_scheme = Some(source_scheme.into()); self }
    /// Cookie source port. Valid values are {-1, \[1, 65535\]}, -1 indicates an unspecified port.
    /// An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    /// This is a temporary ability and it will be removed in the future.
    pub fn source_port(mut self, source_port: i64) -> Self { self.source_port = Some(source_port); self }
    /// Cookie partition key. If not set, the cookie will be set as not partitioned.
    pub fn partition_key(mut self, partition_key: CookiePartitionKey<'a>) -> Self { self.partition_key = Some(partition_key); self }
    pub fn build(self) -> SetCookieParams<'a> {
        SetCookieParams {
            name: self.name,
            value: self.value,
            url: self.url,
            domain: self.domain,
            path: self.path,
            secure: self.secure,
            http_only: self.http_only,
            same_site: self.same_site,
            expires: self.expires,
            priority: self.priority,
            source_scheme: self.source_scheme,
            source_port: self.source_port,
            partition_key: self.partition_key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `success`: Always set to true. If an error occurs, the response indicates protocol error.
    pub fn builder(success: bool) -> SetCookieReturnsBuilder {
        SetCookieReturnsBuilder {
            success: success,
        }
    }
    /// Always set to true. If an error occurs, the response indicates protocol error.
    pub fn success(&self) -> bool { self.success }
}


pub struct SetCookieReturnsBuilder {
    success: bool,
}

impl SetCookieReturnsBuilder {
    pub fn build(self) -> SetCookieReturns {
        SetCookieReturns {
            success: self.success,
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
    /// Creates a builder for this type with the required parameters:
    /// * `cookies`: Cookies to be set.
    pub fn builder(cookies: Vec<CookieParam<'a>>) -> SetCookiesParamsBuilder<'a> {
        SetCookiesParamsBuilder {
            cookies: cookies,
        }
    }
    /// Cookies to be set.
    pub fn cookies(&self) -> &[CookieParam<'a>] { &self.cookies }
}


pub struct SetCookiesParamsBuilder<'a> {
    cookies: Vec<CookieParam<'a>>,
}

impl<'a> SetCookiesParamsBuilder<'a> {
    pub fn build(self) -> SetCookiesParams<'a> {
        SetCookiesParams {
            cookies: self.cookies,
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
    /// Creates a builder for this type with the required parameters:
    /// * `headers`: Map with extra HTTP headers.
    pub fn builder(headers: Headers) -> SetExtraHTTPHeadersParamsBuilder {
        SetExtraHTTPHeadersParamsBuilder {
            headers: headers,
        }
    }
    /// Map with extra HTTP headers.
    pub fn headers(&self) -> &Headers { &self.headers }
}


pub struct SetExtraHTTPHeadersParamsBuilder {
    headers: Headers,
}

impl SetExtraHTTPHeadersParamsBuilder {
    pub fn build(self) -> SetExtraHTTPHeadersParams {
        SetExtraHTTPHeadersParams {
            headers: self.headers,
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
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: Whether to attach a page script stack for debugging purpose.
    pub fn builder(enabled: bool) -> SetAttachDebugStackParamsBuilder {
        SetAttachDebugStackParamsBuilder {
            enabled: enabled,
        }
    }
    /// Whether to attach a page script stack for debugging purpose.
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetAttachDebugStackParamsBuilder {
    enabled: bool,
}

impl SetAttachDebugStackParamsBuilder {
    pub fn build(self) -> SetAttachDebugStackParams {
        SetAttachDebugStackParams {
            enabled: self.enabled,
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
    /// Creates a builder for this type with the required parameters:
    /// * `patterns`: Requests matching any of these patterns will be forwarded and wait for the corresponding continueInterceptedRequest call.
    pub fn builder(patterns: Vec<RequestPattern<'a>>) -> SetRequestInterceptionParamsBuilder<'a> {
        SetRequestInterceptionParamsBuilder {
            patterns: patterns,
        }
    }
    /// Requests matching any of these patterns will be forwarded and wait for the corresponding
    /// continueInterceptedRequest call.
    pub fn patterns(&self) -> &[RequestPattern<'a>] { &self.patterns }
}


pub struct SetRequestInterceptionParamsBuilder<'a> {
    patterns: Vec<RequestPattern<'a>>,
}

impl<'a> SetRequestInterceptionParamsBuilder<'a> {
    pub fn build(self) -> SetRequestInterceptionParams<'a> {
        SetRequestInterceptionParams {
            patterns: self.patterns,
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
    #[serde(rename = "userAgent")]
    user_agent: Cow<'a, str>,
    /// Browser language to emulate.
    #[serde(skip_serializing_if = "Option::is_none", rename = "acceptLanguage")]
    accept_language: Option<Cow<'a, str>>,
    /// The platform navigator.platform should return.
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<Cow<'a, str>>,
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    #[serde(skip_serializing_if = "Option::is_none", rename = "userAgentMetadata")]
    user_agent_metadata: Option<crate::emulation::UserAgentMetadata<'a>>,
}

impl<'a> SetUserAgentOverrideParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `user_agent`: User agent to use.
    pub fn builder(user_agent: impl Into<Cow<'a, str>>) -> SetUserAgentOverrideParamsBuilder<'a> {
        SetUserAgentOverrideParamsBuilder {
            user_agent: user_agent.into(),
            accept_language: None,
            platform: None,
            user_agent_metadata: None,
        }
    }
    /// User agent to use.
    pub fn user_agent(&self) -> &str { self.user_agent.as_ref() }
    /// Browser language to emulate.
    pub fn accept_language(&self) -> Option<&str> { self.accept_language.as_deref() }
    /// The platform navigator.platform should return.
    pub fn platform(&self) -> Option<&str> { self.platform.as_deref() }
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    pub fn user_agent_metadata(&self) -> Option<&crate::emulation::UserAgentMetadata<'a>> { self.user_agent_metadata.as_ref() }
}


pub struct SetUserAgentOverrideParamsBuilder<'a> {
    user_agent: Cow<'a, str>,
    accept_language: Option<Cow<'a, str>>,
    platform: Option<Cow<'a, str>>,
    user_agent_metadata: Option<crate::emulation::UserAgentMetadata<'a>>,
}

impl<'a> SetUserAgentOverrideParamsBuilder<'a> {
    /// Browser language to emulate.
    pub fn accept_language(mut self, accept_language: impl Into<Cow<'a, str>>) -> Self { self.accept_language = Some(accept_language.into()); self }
    /// The platform navigator.platform should return.
    pub fn platform(mut self, platform: impl Into<Cow<'a, str>>) -> Self { self.platform = Some(platform.into()); self }
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    pub fn user_agent_metadata(mut self, user_agent_metadata: crate::emulation::UserAgentMetadata<'a>) -> Self { self.user_agent_metadata = Some(user_agent_metadata); self }
    pub fn build(self) -> SetUserAgentOverrideParams<'a> {
        SetUserAgentOverrideParams {
            user_agent: self.user_agent,
            accept_language: self.accept_language,
            platform: self.platform,
            user_agent_metadata: self.user_agent_metadata,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
}

impl<'a> StreamResourceContentParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: Identifier of the request to stream.
    pub fn builder(request_id: impl Into<RequestId<'a>>) -> StreamResourceContentParamsBuilder<'a> {
        StreamResourceContentParamsBuilder {
            request_id: request_id.into(),
        }
    }
    /// Identifier of the request to stream.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
}


pub struct StreamResourceContentParamsBuilder<'a> {
    request_id: RequestId<'a>,
}

impl<'a> StreamResourceContentParamsBuilder<'a> {
    pub fn build(self) -> StreamResourceContentParams<'a> {
        StreamResourceContentParams {
            request_id: self.request_id,
        }
    }
}

/// Enables streaming of the response for the given requestId.
/// If enabled, the dataReceived event contains the data that was received during streaming.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StreamResourceContentReturns<'a> {
    /// Data that has been buffered until streaming is enabled. (Encoded as a base64 string when passed over JSON)
    #[serde(rename = "bufferedData")]
    buffered_data: Cow<'a, str>,
}

impl<'a> StreamResourceContentReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `buffered_data`: Data that has been buffered until streaming is enabled. (Encoded as a base64 string when passed over JSON)
    pub fn builder(buffered_data: impl Into<Cow<'a, str>>) -> StreamResourceContentReturnsBuilder<'a> {
        StreamResourceContentReturnsBuilder {
            buffered_data: buffered_data.into(),
        }
    }
    /// Data that has been buffered until streaming is enabled. (Encoded as a base64 string when passed over JSON)
    pub fn buffered_data(&self) -> &str { self.buffered_data.as_ref() }
}


pub struct StreamResourceContentReturnsBuilder<'a> {
    buffered_data: Cow<'a, str>,
}

impl<'a> StreamResourceContentReturnsBuilder<'a> {
    pub fn build(self) -> StreamResourceContentReturns<'a> {
        StreamResourceContentReturns {
            buffered_data: self.buffered_data,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetSecurityIsolationStatusParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetSecurityIsolationStatusParamsBuilder<'a> {
        GetSecurityIsolationStatusParamsBuilder {
            frame_id: None,
        }
    }
    /// If no frameId is provided, the status of the target is provided.
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
}

#[derive(Default)]
pub struct GetSecurityIsolationStatusParamsBuilder<'a> {
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetSecurityIsolationStatusParamsBuilder<'a> {
    /// If no frameId is provided, the status of the target is provided.
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    pub fn build(self) -> GetSecurityIsolationStatusParams<'a> {
        GetSecurityIsolationStatusParams {
            frame_id: self.frame_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `status`: 
    pub fn builder(status: SecurityIsolationStatus<'a>) -> GetSecurityIsolationStatusReturnsBuilder<'a> {
        GetSecurityIsolationStatusReturnsBuilder {
            status: status,
        }
    }
    pub fn status(&self) -> &SecurityIsolationStatus<'a> { &self.status }
}


pub struct GetSecurityIsolationStatusReturnsBuilder<'a> {
    status: SecurityIsolationStatus<'a>,
}

impl<'a> GetSecurityIsolationStatusReturnsBuilder<'a> {
    pub fn build(self) -> GetSecurityIsolationStatusReturns<'a> {
        GetSecurityIsolationStatusReturns {
            status: self.status,
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
    /// Creates a builder for this type with the required parameters:
    /// * `enable`: Whether to enable or disable events for the Reporting API
    pub fn builder(enable: bool) -> EnableReportingApiParamsBuilder {
        EnableReportingApiParamsBuilder {
            enable: enable,
        }
    }
    /// Whether to enable or disable events for the Reporting API
    pub fn enable(&self) -> bool { self.enable }
}


pub struct EnableReportingApiParamsBuilder {
    enable: bool,
}

impl EnableReportingApiParamsBuilder {
    pub fn build(self) -> EnableReportingApiParams {
        EnableReportingApiParams {
            enable: self.enable,
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
    /// Creates a builder for this type with the required parameters:
    /// * `enable`: Whether to enable or disable events.
    pub fn builder(enable: bool) -> EnableDeviceBoundSessionsParamsBuilder {
        EnableDeviceBoundSessionsParamsBuilder {
            enable: enable,
        }
    }
    /// Whether to enable or disable events.
    pub fn enable(&self) -> bool { self.enable }
}


pub struct EnableDeviceBoundSessionsParamsBuilder {
    enable: bool,
}

impl EnableDeviceBoundSessionsParamsBuilder {
    pub fn build(self) -> EnableDeviceBoundSessionsParams {
        EnableDeviceBoundSessionsParams {
            enable: self.enable,
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
    /// Creates a builder for this type with the required parameters:
    /// * `key`: 
    pub fn builder(key: DeviceBoundSessionKey<'a>) -> DeleteDeviceBoundSessionParamsBuilder<'a> {
        DeleteDeviceBoundSessionParamsBuilder {
            key: key,
        }
    }
    pub fn key(&self) -> &DeviceBoundSessionKey<'a> { &self.key }
}


pub struct DeleteDeviceBoundSessionParamsBuilder<'a> {
    key: DeviceBoundSessionKey<'a>,
}

impl<'a> DeleteDeviceBoundSessionParamsBuilder<'a> {
    pub fn build(self) -> DeleteDeviceBoundSessionParams<'a> {
        DeleteDeviceBoundSessionParams {
            key: self.key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: The URL origin.
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> FetchSchemefulSiteParamsBuilder<'a> {
        FetchSchemefulSiteParamsBuilder {
            origin: origin.into(),
        }
    }
    /// The URL origin.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}


pub struct FetchSchemefulSiteParamsBuilder<'a> {
    origin: Cow<'a, str>,
}

impl<'a> FetchSchemefulSiteParamsBuilder<'a> {
    pub fn build(self) -> FetchSchemefulSiteParams<'a> {
        FetchSchemefulSiteParams {
            origin: self.origin,
        }
    }
}

/// Fetches the schemeful site for a specific origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchSchemefulSiteReturns<'a> {
    /// The corresponding schemeful site.
    #[serde(rename = "schemefulSite")]
    schemeful_site: Cow<'a, str>,
}

impl<'a> FetchSchemefulSiteReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `schemeful_site`: The corresponding schemeful site.
    pub fn builder(schemeful_site: impl Into<Cow<'a, str>>) -> FetchSchemefulSiteReturnsBuilder<'a> {
        FetchSchemefulSiteReturnsBuilder {
            schemeful_site: schemeful_site.into(),
        }
    }
    /// The corresponding schemeful site.
    pub fn schemeful_site(&self) -> &str { self.schemeful_site.as_ref() }
}


pub struct FetchSchemefulSiteReturnsBuilder<'a> {
    schemeful_site: Cow<'a, str>,
}

impl<'a> FetchSchemefulSiteReturnsBuilder<'a> {
    pub fn build(self) -> FetchSchemefulSiteReturns<'a> {
        FetchSchemefulSiteReturns {
            schemeful_site: self.schemeful_site,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
    /// URL of the resource to get content for.
    url: Cow<'a, str>,
    /// Options for the request.
    options: LoadNetworkResourceOptions,
}

impl<'a> LoadNetworkResourceParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: URL of the resource to get content for.
    /// * `options`: Options for the request.
    pub fn builder(url: impl Into<Cow<'a, str>>, options: LoadNetworkResourceOptions) -> LoadNetworkResourceParamsBuilder<'a> {
        LoadNetworkResourceParamsBuilder {
            frame_id: None,
            url: url.into(),
            options: options,
        }
    }
    /// Frame id to get the resource for. Mandatory for frame targets, and
    /// should be omitted for worker targets.
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
    /// URL of the resource to get content for.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Options for the request.
    pub fn options(&self) -> &LoadNetworkResourceOptions { &self.options }
}


pub struct LoadNetworkResourceParamsBuilder<'a> {
    frame_id: Option<crate::page::FrameId<'a>>,
    url: Cow<'a, str>,
    options: LoadNetworkResourceOptions,
}

impl<'a> LoadNetworkResourceParamsBuilder<'a> {
    /// Frame id to get the resource for. Mandatory for frame targets, and
    /// should be omitted for worker targets.
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    pub fn build(self) -> LoadNetworkResourceParams<'a> {
        LoadNetworkResourceParams {
            frame_id: self.frame_id,
            url: self.url,
            options: self.options,
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
    /// Creates a builder for this type with the required parameters:
    /// * `resource`: 
    pub fn builder(resource: LoadNetworkResourcePageResult<'a>) -> LoadNetworkResourceReturnsBuilder<'a> {
        LoadNetworkResourceReturnsBuilder {
            resource: resource,
        }
    }
    pub fn resource(&self) -> &LoadNetworkResourcePageResult<'a> { &self.resource }
}


pub struct LoadNetworkResourceReturnsBuilder<'a> {
    resource: LoadNetworkResourcePageResult<'a>,
}

impl<'a> LoadNetworkResourceReturnsBuilder<'a> {
    pub fn build(self) -> LoadNetworkResourceReturns<'a> {
        LoadNetworkResourceReturns {
            resource: self.resource,
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
    #[serde(rename = "enableThirdPartyCookieRestriction")]
    enable_third_party_cookie_restriction: bool,
}

impl SetCookieControlsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `enable_third_party_cookie_restriction`: Whether 3pc restriction is enabled.
    pub fn builder(enable_third_party_cookie_restriction: bool) -> SetCookieControlsParamsBuilder {
        SetCookieControlsParamsBuilder {
            enable_third_party_cookie_restriction: enable_third_party_cookie_restriction,
        }
    }
    /// Whether 3pc restriction is enabled.
    pub fn enable_third_party_cookie_restriction(&self) -> bool { self.enable_third_party_cookie_restriction }
}


pub struct SetCookieControlsParamsBuilder {
    enable_third_party_cookie_restriction: bool,
}

impl SetCookieControlsParamsBuilder {
    pub fn build(self) -> SetCookieControlsParams {
        SetCookieControlsParams {
            enable_third_party_cookie_restriction: self.enable_third_party_cookie_restriction,
        }
    }
}

impl SetCookieControlsParams { pub const METHOD: &'static str = "Network.setCookieControls"; }

impl<'a> crate::CdpCommand<'a> for SetCookieControlsParams {
    const METHOD: &'static str = "Network.setCookieControls";
    type Response = crate::EmptyReturns;
}
