//! A domain for letting clients substitute browser's network layer with client code.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Unique request identifier.
/// Note that this does not identify individual HTTP requests that are part of
/// a network request.

pub type RequestId<'a> = Cow<'a, str>;

/// Stages of the request to handle. Request will intercept before the request is
/// sent. Response will intercept after the response is received (but before response
/// body is received).

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum RequestStage {
    #[default]
    #[serde(rename = "Request")]
    Request,
    #[serde(rename = "Response")]
    Response,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestPattern<'a> {
    /// Wildcards (''*'' -> zero or more, ''?'' -> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    #[serde(skip_serializing_if = "Option::is_none")]
    urlPattern: Option<Cow<'a, str>>,
    /// If set, only requests for matching resource types will be intercepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    resourceType: Option<crate::network::ResourceType>,
    /// Stage at which to begin intercepting requests. Default is Request.
    #[serde(skip_serializing_if = "Option::is_none")]
    requestStage: Option<RequestStage>,
}

impl<'a> RequestPattern<'a> {
    pub fn builder() -> RequestPatternBuilder<'a> {
        RequestPatternBuilder {
            urlPattern: None,
            resourceType: None,
            requestStage: None,
        }
    }
    pub fn urlPattern(&self) -> Option<&str> { self.urlPattern.as_deref() }
    pub fn resourceType(&self) -> Option<&crate::network::ResourceType> { self.resourceType.as_ref() }
    pub fn requestStage(&self) -> Option<&RequestStage> { self.requestStage.as_ref() }
}

#[derive(Default)]
pub struct RequestPatternBuilder<'a> {
    urlPattern: Option<Cow<'a, str>>,
    resourceType: Option<crate::network::ResourceType>,
    requestStage: Option<RequestStage>,
}

impl<'a> RequestPatternBuilder<'a> {
    /// Wildcards (''*'' -> zero or more, ''?'' -> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    pub fn urlPattern(mut self, urlPattern: impl Into<Cow<'a, str>>) -> Self { self.urlPattern = Some(urlPattern.into()); self }
    /// If set, only requests for matching resource types will be intercepted.
    pub fn resourceType(mut self, resourceType: crate::network::ResourceType) -> Self { self.resourceType = Some(resourceType); self }
    /// Stage at which to begin intercepting requests. Default is Request.
    pub fn requestStage(mut self, requestStage: RequestStage) -> Self { self.requestStage = Some(requestStage); self }
    pub fn build(self) -> RequestPattern<'a> {
        RequestPattern {
            urlPattern: self.urlPattern,
            resourceType: self.resourceType,
            requestStage: self.requestStage,
        }
    }
}

/// Response HTTP header entry

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeaderEntry<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> HeaderEntry<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> HeaderEntryBuilder<'a> {
        HeaderEntryBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct HeaderEntryBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> HeaderEntryBuilder<'a> {
    pub fn build(self) -> HeaderEntry<'a> {
        HeaderEntry {
            name: self.name,
            value: self.value,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>, scheme: impl Into<Cow<'a, str>>, realm: impl Into<Cow<'a, str>>) -> AuthChallengeBuilder<'a> {
        AuthChallengeBuilder {
            source: None,
            origin: origin.into(),
            scheme: scheme.into(),
            realm: realm.into(),
        }
    }
    pub fn source(&self) -> Option<&str> { self.source.as_deref() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn scheme(&self) -> &str { self.scheme.as_ref() }
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
    pub fn builder(response: impl Into<Cow<'a, str>>) -> AuthChallengeResponseBuilder<'a> {
        AuthChallengeResponseBuilder {
            response: response.into(),
            username: None,
            password: None,
        }
    }
    pub fn response(&self) -> &str { self.response.as_ref() }
    pub fn username(&self) -> Option<&str> { self.username.as_deref() }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Fetch.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Fetch.disable";
    type Response = crate::EmptyReturns;
}

/// Enables issuing of requestPaused events. A request will be paused until client
/// calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams<'a> {
    /// If specified, only requests matching any of these patterns will produce
    /// fetchRequested event and will be paused until clients response. If not set,
    /// all requests will be affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    patterns: Option<Vec<RequestPattern<'a>>>,
    /// If true, authRequired events will be issued and requests will be paused
    /// expecting a call to continueWithAuth.
    #[serde(skip_serializing_if = "Option::is_none")]
    handleAuthRequests: Option<bool>,
}

impl<'a> EnableParams<'a> {
    pub fn builder() -> EnableParamsBuilder<'a> {
        EnableParamsBuilder {
            patterns: None,
            handleAuthRequests: None,
        }
    }
    pub fn patterns(&self) -> Option<&[RequestPattern<'a>]> { self.patterns.as_deref() }
    pub fn handleAuthRequests(&self) -> Option<bool> { self.handleAuthRequests }
}

#[derive(Default)]
pub struct EnableParamsBuilder<'a> {
    patterns: Option<Vec<RequestPattern<'a>>>,
    handleAuthRequests: Option<bool>,
}

impl<'a> EnableParamsBuilder<'a> {
    /// If specified, only requests matching any of these patterns will produce
    /// fetchRequested event and will be paused until clients response. If not set,
    /// all requests will be affected.
    pub fn patterns(mut self, patterns: Vec<RequestPattern<'a>>) -> Self { self.patterns = Some(patterns); self }
    /// If true, authRequired events will be issued and requests will be paused
    /// expecting a call to continueWithAuth.
    pub fn handleAuthRequests(mut self, handleAuthRequests: bool) -> Self { self.handleAuthRequests = Some(handleAuthRequests); self }
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            patterns: self.patterns,
            handleAuthRequests: self.handleAuthRequests,
        }
    }
}

impl<'a> EnableParams<'a> { pub const METHOD: &'static str = "Fetch.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams<'a> {
    const METHOD: &'static str = "Fetch.enable";
    type Response = crate::EmptyReturns;
}

/// Causes the request to fail with specified reason.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FailRequestParams<'a> {
    /// An id the client received in requestPaused event.
    requestId: RequestId<'a>,
    /// Causes the request to fail with the given reason.
    errorReason: crate::network::ErrorReason,
}

impl<'a> FailRequestParams<'a> {
    pub fn builder(requestId: RequestId<'a>, errorReason: crate::network::ErrorReason) -> FailRequestParamsBuilder<'a> {
        FailRequestParamsBuilder {
            requestId: requestId,
            errorReason: errorReason,
        }
    }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
    pub fn errorReason(&self) -> &crate::network::ErrorReason { &self.errorReason }
}


pub struct FailRequestParamsBuilder<'a> {
    requestId: RequestId<'a>,
    errorReason: crate::network::ErrorReason,
}

impl<'a> FailRequestParamsBuilder<'a> {
    pub fn build(self) -> FailRequestParams<'a> {
        FailRequestParams {
            requestId: self.requestId,
            errorReason: self.errorReason,
        }
    }
}

impl<'a> FailRequestParams<'a> { pub const METHOD: &'static str = "Fetch.failRequest"; }

impl<'a> crate::CdpCommand<'a> for FailRequestParams<'a> {
    const METHOD: &'static str = "Fetch.failRequest";
    type Response = crate::EmptyReturns;
}

/// Provides response to the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FulfillRequestParams<'a> {
    /// An id the client received in requestPaused event.
    requestId: RequestId<'a>,
    /// An HTTP response code.
    responseCode: i64,
    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    responseHeaders: Option<Vec<HeaderEntry<'a>>>,
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    binaryResponseHeaders: Option<Cow<'a, str>>,
    /// A response body. If absent, original response body will be used if
    /// the request is intercepted at the response stage and empty body
    /// will be used if the request is intercepted at the request stage. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Cow<'a, str>>,
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    responsePhrase: Option<Cow<'a, str>>,
}

impl<'a> FulfillRequestParams<'a> {
    pub fn builder(requestId: RequestId<'a>, responseCode: i64) -> FulfillRequestParamsBuilder<'a> {
        FulfillRequestParamsBuilder {
            requestId: requestId,
            responseCode: responseCode,
            responseHeaders: None,
            binaryResponseHeaders: None,
            body: None,
            responsePhrase: None,
        }
    }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
    pub fn responseCode(&self) -> i64 { self.responseCode }
    pub fn responseHeaders(&self) -> Option<&[HeaderEntry<'a>]> { self.responseHeaders.as_deref() }
    pub fn binaryResponseHeaders(&self) -> Option<&str> { self.binaryResponseHeaders.as_deref() }
    pub fn body(&self) -> Option<&str> { self.body.as_deref() }
    pub fn responsePhrase(&self) -> Option<&str> { self.responsePhrase.as_deref() }
}


pub struct FulfillRequestParamsBuilder<'a> {
    requestId: RequestId<'a>,
    responseCode: i64,
    responseHeaders: Option<Vec<HeaderEntry<'a>>>,
    binaryResponseHeaders: Option<Cow<'a, str>>,
    body: Option<Cow<'a, str>>,
    responsePhrase: Option<Cow<'a, str>>,
}

impl<'a> FulfillRequestParamsBuilder<'a> {
    /// Response headers.
    pub fn responseHeaders(mut self, responseHeaders: Vec<HeaderEntry<'a>>) -> Self { self.responseHeaders = Some(responseHeaders); self }
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    pub fn binaryResponseHeaders(mut self, binaryResponseHeaders: impl Into<Cow<'a, str>>) -> Self { self.binaryResponseHeaders = Some(binaryResponseHeaders.into()); self }
    /// A response body. If absent, original response body will be used if
    /// the request is intercepted at the response stage and empty body
    /// will be used if the request is intercepted at the request stage. (Encoded as a base64 string when passed over JSON)
    pub fn body(mut self, body: impl Into<Cow<'a, str>>) -> Self { self.body = Some(body.into()); self }
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    pub fn responsePhrase(mut self, responsePhrase: impl Into<Cow<'a, str>>) -> Self { self.responsePhrase = Some(responsePhrase.into()); self }
    pub fn build(self) -> FulfillRequestParams<'a> {
        FulfillRequestParams {
            requestId: self.requestId,
            responseCode: self.responseCode,
            responseHeaders: self.responseHeaders,
            binaryResponseHeaders: self.binaryResponseHeaders,
            body: self.body,
            responsePhrase: self.responsePhrase,
        }
    }
}

impl<'a> FulfillRequestParams<'a> { pub const METHOD: &'static str = "Fetch.fulfillRequest"; }

impl<'a> crate::CdpCommand<'a> for FulfillRequestParams<'a> {
    const METHOD: &'static str = "Fetch.fulfillRequest";
    type Response = crate::EmptyReturns;
}

/// Continues the request, optionally modifying some of its parameters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueRequestParams<'a> {
    /// An id the client received in requestPaused event.
    requestId: RequestId<'a>,
    /// If set, the request url will be modified in a way that's not observable by page.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// If set, the request method is overridden.
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<Cow<'a, str>>,
    /// If set, overrides the post data in the request. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    postData: Option<Cow<'a, str>>,
    /// If set, overrides the request headers. Note that the overrides do not
    /// extend to subsequent redirect hops, if a redirect happens. Another override
    /// may be applied to a different request produced by a redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<HeaderEntry<'a>>>,
    /// If set, overrides response interception behavior for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    interceptResponse: Option<bool>,
}

impl<'a> ContinueRequestParams<'a> {
    pub fn builder(requestId: RequestId<'a>) -> ContinueRequestParamsBuilder<'a> {
        ContinueRequestParamsBuilder {
            requestId: requestId,
            url: None,
            method: None,
            postData: None,
            headers: None,
            interceptResponse: None,
        }
    }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn method(&self) -> Option<&str> { self.method.as_deref() }
    pub fn postData(&self) -> Option<&str> { self.postData.as_deref() }
    pub fn headers(&self) -> Option<&[HeaderEntry<'a>]> { self.headers.as_deref() }
    pub fn interceptResponse(&self) -> Option<bool> { self.interceptResponse }
}


pub struct ContinueRequestParamsBuilder<'a> {
    requestId: RequestId<'a>,
    url: Option<Cow<'a, str>>,
    method: Option<Cow<'a, str>>,
    postData: Option<Cow<'a, str>>,
    headers: Option<Vec<HeaderEntry<'a>>>,
    interceptResponse: Option<bool>,
}

impl<'a> ContinueRequestParamsBuilder<'a> {
    /// If set, the request url will be modified in a way that's not observable by page.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// If set, the request method is overridden.
    pub fn method(mut self, method: impl Into<Cow<'a, str>>) -> Self { self.method = Some(method.into()); self }
    /// If set, overrides the post data in the request. (Encoded as a base64 string when passed over JSON)
    pub fn postData(mut self, postData: impl Into<Cow<'a, str>>) -> Self { self.postData = Some(postData.into()); self }
    /// If set, overrides the request headers. Note that the overrides do not
    /// extend to subsequent redirect hops, if a redirect happens. Another override
    /// may be applied to a different request produced by a redirect.
    pub fn headers(mut self, headers: Vec<HeaderEntry<'a>>) -> Self { self.headers = Some(headers); self }
    /// If set, overrides response interception behavior for this request.
    pub fn interceptResponse(mut self, interceptResponse: bool) -> Self { self.interceptResponse = Some(interceptResponse); self }
    pub fn build(self) -> ContinueRequestParams<'a> {
        ContinueRequestParams {
            requestId: self.requestId,
            url: self.url,
            method: self.method,
            postData: self.postData,
            headers: self.headers,
            interceptResponse: self.interceptResponse,
        }
    }
}

impl<'a> ContinueRequestParams<'a> { pub const METHOD: &'static str = "Fetch.continueRequest"; }

impl<'a> crate::CdpCommand<'a> for ContinueRequestParams<'a> {
    const METHOD: &'static str = "Fetch.continueRequest";
    type Response = crate::EmptyReturns;
}

/// Continues a request supplying authChallengeResponse following authRequired event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueWithAuthParams<'a> {
    /// An id the client received in authRequired event.
    requestId: RequestId<'a>,
    /// Response to  with an authChallenge.
    authChallengeResponse: AuthChallengeResponse<'a>,
}

impl<'a> ContinueWithAuthParams<'a> {
    pub fn builder(requestId: RequestId<'a>, authChallengeResponse: AuthChallengeResponse<'a>) -> ContinueWithAuthParamsBuilder<'a> {
        ContinueWithAuthParamsBuilder {
            requestId: requestId,
            authChallengeResponse: authChallengeResponse,
        }
    }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
    pub fn authChallengeResponse(&self) -> &AuthChallengeResponse<'a> { &self.authChallengeResponse }
}


pub struct ContinueWithAuthParamsBuilder<'a> {
    requestId: RequestId<'a>,
    authChallengeResponse: AuthChallengeResponse<'a>,
}

impl<'a> ContinueWithAuthParamsBuilder<'a> {
    pub fn build(self) -> ContinueWithAuthParams<'a> {
        ContinueWithAuthParams {
            requestId: self.requestId,
            authChallengeResponse: self.authChallengeResponse,
        }
    }
}

impl<'a> ContinueWithAuthParams<'a> { pub const METHOD: &'static str = "Fetch.continueWithAuth"; }

impl<'a> crate::CdpCommand<'a> for ContinueWithAuthParams<'a> {
    const METHOD: &'static str = "Fetch.continueWithAuth";
    type Response = crate::EmptyReturns;
}

/// Continues loading of the paused response, optionally modifying the
/// response headers. If either responseCode or headers are modified, all of them
/// must be present.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueResponseParams<'a> {
    /// An id the client received in requestPaused event.
    requestId: RequestId<'a>,
    /// An HTTP response code. If absent, original response code will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    responseCode: Option<i64>,
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    responsePhrase: Option<Cow<'a, str>>,
    /// Response headers. If absent, original response headers will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    responseHeaders: Option<Vec<HeaderEntry<'a>>>,
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    binaryResponseHeaders: Option<Cow<'a, str>>,
}

impl<'a> ContinueResponseParams<'a> {
    pub fn builder(requestId: RequestId<'a>) -> ContinueResponseParamsBuilder<'a> {
        ContinueResponseParamsBuilder {
            requestId: requestId,
            responseCode: None,
            responsePhrase: None,
            responseHeaders: None,
            binaryResponseHeaders: None,
        }
    }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
    pub fn responseCode(&self) -> Option<i64> { self.responseCode }
    pub fn responsePhrase(&self) -> Option<&str> { self.responsePhrase.as_deref() }
    pub fn responseHeaders(&self) -> Option<&[HeaderEntry<'a>]> { self.responseHeaders.as_deref() }
    pub fn binaryResponseHeaders(&self) -> Option<&str> { self.binaryResponseHeaders.as_deref() }
}


pub struct ContinueResponseParamsBuilder<'a> {
    requestId: RequestId<'a>,
    responseCode: Option<i64>,
    responsePhrase: Option<Cow<'a, str>>,
    responseHeaders: Option<Vec<HeaderEntry<'a>>>,
    binaryResponseHeaders: Option<Cow<'a, str>>,
}

impl<'a> ContinueResponseParamsBuilder<'a> {
    /// An HTTP response code. If absent, original response code will be used.
    pub fn responseCode(mut self, responseCode: i64) -> Self { self.responseCode = Some(responseCode); self }
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    pub fn responsePhrase(mut self, responsePhrase: impl Into<Cow<'a, str>>) -> Self { self.responsePhrase = Some(responsePhrase.into()); self }
    /// Response headers. If absent, original response headers will be used.
    pub fn responseHeaders(mut self, responseHeaders: Vec<HeaderEntry<'a>>) -> Self { self.responseHeaders = Some(responseHeaders); self }
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    pub fn binaryResponseHeaders(mut self, binaryResponseHeaders: impl Into<Cow<'a, str>>) -> Self { self.binaryResponseHeaders = Some(binaryResponseHeaders.into()); self }
    pub fn build(self) -> ContinueResponseParams<'a> {
        ContinueResponseParams {
            requestId: self.requestId,
            responseCode: self.responseCode,
            responsePhrase: self.responsePhrase,
            responseHeaders: self.responseHeaders,
            binaryResponseHeaders: self.binaryResponseHeaders,
        }
    }
}

impl<'a> ContinueResponseParams<'a> { pub const METHOD: &'static str = "Fetch.continueResponse"; }

impl<'a> crate::CdpCommand<'a> for ContinueResponseParams<'a> {
    const METHOD: &'static str = "Fetch.continueResponse";
    type Response = crate::EmptyReturns;
}

/// Causes the body of the response to be received from the server and
/// returned as a single string. May only be issued for a request that
/// is paused in the Response stage and is mutually exclusive with
/// takeResponseBodyForInterceptionAsStream. Calling other methods that
/// affect the request or disabling fetch domain before body is received
/// results in an undefined behavior.
/// Note that the response body is not available for redirects. Requests
/// paused in the _redirect received_ state may be differentiated by
/// 'responseCode' and presence of 'location' response header, see
/// comments to 'requestPaused' for details.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyParams<'a> {
    /// Identifier for the intercepted request to get body for.
    requestId: RequestId<'a>,
}

impl<'a> GetResponseBodyParams<'a> {
    pub fn builder(requestId: RequestId<'a>) -> GetResponseBodyParamsBuilder<'a> {
        GetResponseBodyParamsBuilder {
            requestId: requestId,
        }
    }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
}


pub struct GetResponseBodyParamsBuilder<'a> {
    requestId: RequestId<'a>,
}

impl<'a> GetResponseBodyParamsBuilder<'a> {
    pub fn build(self) -> GetResponseBodyParams<'a> {
        GetResponseBodyParams {
            requestId: self.requestId,
        }
    }
}

/// Causes the body of the response to be received from the server and
/// returned as a single string. May only be issued for a request that
/// is paused in the Response stage and is mutually exclusive with
/// takeResponseBodyForInterceptionAsStream. Calling other methods that
/// affect the request or disabling fetch domain before body is received
/// results in an undefined behavior.
/// Note that the response body is not available for redirects. Requests
/// paused in the _redirect received_ state may be differentiated by
/// 'responseCode' and presence of 'location' response header, see
/// comments to 'requestPaused' for details.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyReturns<'a> {
    /// Response body.
    body: Cow<'a, str>,
    /// True, if content was sent as base64.
    base64Encoded: bool,
}

impl<'a> GetResponseBodyReturns<'a> {
    pub fn builder(body: impl Into<Cow<'a, str>>, base64Encoded: bool) -> GetResponseBodyReturnsBuilder<'a> {
        GetResponseBodyReturnsBuilder {
            body: body.into(),
            base64Encoded: base64Encoded,
        }
    }
    pub fn body(&self) -> &str { self.body.as_ref() }
    pub fn base64Encoded(&self) -> bool { self.base64Encoded }
}


pub struct GetResponseBodyReturnsBuilder<'a> {
    body: Cow<'a, str>,
    base64Encoded: bool,
}

impl<'a> GetResponseBodyReturnsBuilder<'a> {
    pub fn build(self) -> GetResponseBodyReturns<'a> {
        GetResponseBodyReturns {
            body: self.body,
            base64Encoded: self.base64Encoded,
        }
    }
}

impl<'a> GetResponseBodyParams<'a> { pub const METHOD: &'static str = "Fetch.getResponseBody"; }

impl<'a> crate::CdpCommand<'a> for GetResponseBodyParams<'a> {
    const METHOD: &'static str = "Fetch.getResponseBody";
    type Response = GetResponseBodyReturns<'a>;
}

/// Returns a handle to the stream representing the response body.
/// The request must be paused in the HeadersReceived stage.
/// Note that after this command the request can't be continued
/// as is -- client either needs to cancel it or to provide the
/// response body.
/// The stream only supports sequential read, IO.read will fail if the position
/// is specified.
/// This method is mutually exclusive with getResponseBody.
/// Calling other methods that affect the request or disabling fetch
/// domain before body is received results in an undefined behavior.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeResponseBodyAsStreamParams<'a> {
    requestId: RequestId<'a>,
}

impl<'a> TakeResponseBodyAsStreamParams<'a> {
    pub fn builder(requestId: RequestId<'a>) -> TakeResponseBodyAsStreamParamsBuilder<'a> {
        TakeResponseBodyAsStreamParamsBuilder {
            requestId: requestId,
        }
    }
    pub fn requestId(&self) -> &RequestId<'a> { &self.requestId }
}


pub struct TakeResponseBodyAsStreamParamsBuilder<'a> {
    requestId: RequestId<'a>,
}

impl<'a> TakeResponseBodyAsStreamParamsBuilder<'a> {
    pub fn build(self) -> TakeResponseBodyAsStreamParams<'a> {
        TakeResponseBodyAsStreamParams {
            requestId: self.requestId,
        }
    }
}

/// Returns a handle to the stream representing the response body.
/// The request must be paused in the HeadersReceived stage.
/// Note that after this command the request can't be continued
/// as is -- client either needs to cancel it or to provide the
/// response body.
/// The stream only supports sequential read, IO.read will fail if the position
/// is specified.
/// This method is mutually exclusive with getResponseBody.
/// Calling other methods that affect the request or disabling fetch
/// domain before body is received results in an undefined behavior.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeResponseBodyAsStreamReturns<'a> {
    stream: crate::io::StreamHandle<'a>,
}

impl<'a> TakeResponseBodyAsStreamReturns<'a> {
    pub fn builder(stream: crate::io::StreamHandle<'a>) -> TakeResponseBodyAsStreamReturnsBuilder<'a> {
        TakeResponseBodyAsStreamReturnsBuilder {
            stream: stream,
        }
    }
    pub fn stream(&self) -> &crate::io::StreamHandle<'a> { &self.stream }
}


pub struct TakeResponseBodyAsStreamReturnsBuilder<'a> {
    stream: crate::io::StreamHandle<'a>,
}

impl<'a> TakeResponseBodyAsStreamReturnsBuilder<'a> {
    pub fn build(self) -> TakeResponseBodyAsStreamReturns<'a> {
        TakeResponseBodyAsStreamReturns {
            stream: self.stream,
        }
    }
}

impl<'a> TakeResponseBodyAsStreamParams<'a> { pub const METHOD: &'static str = "Fetch.takeResponseBodyAsStream"; }

impl<'a> crate::CdpCommand<'a> for TakeResponseBodyAsStreamParams<'a> {
    const METHOD: &'static str = "Fetch.takeResponseBodyAsStream";
    type Response = TakeResponseBodyAsStreamReturns<'a>;
}
