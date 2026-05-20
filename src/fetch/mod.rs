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
    /// Wildcards (''*'' -\> zero or more, ''?'' -\> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlPattern")]
    url_pattern: Option<Cow<'a, str>>,
    /// If set, only requests for matching resource types will be intercepted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType")]
    resource_type: Option<crate::network::ResourceType>,
    /// Stage at which to begin intercepting requests. Default is Request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestStage")]
    request_stage: Option<RequestStage>,
}

impl<'a> RequestPattern<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> RequestPatternBuilder<'a> {
        RequestPatternBuilder {
            url_pattern: None,
            resource_type: None,
            request_stage: None,
        }
    }
    /// Wildcards (''*'' -\> zero or more, ''?'' -\> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    pub fn url_pattern(&self) -> Option<&str> { self.url_pattern.as_deref() }
    /// If set, only requests for matching resource types will be intercepted.
    pub fn resource_type(&self) -> Option<&crate::network::ResourceType> { self.resource_type.as_ref() }
    /// Stage at which to begin intercepting requests. Default is Request.
    pub fn request_stage(&self) -> Option<&RequestStage> { self.request_stage.as_ref() }
}

#[derive(Default)]
pub struct RequestPatternBuilder<'a> {
    url_pattern: Option<Cow<'a, str>>,
    resource_type: Option<crate::network::ResourceType>,
    request_stage: Option<RequestStage>,
}

impl<'a> RequestPatternBuilder<'a> {
    /// Wildcards (''*'' -\> zero or more, ''?'' -\> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.
    pub fn url_pattern(mut self, url_pattern: impl Into<Cow<'a, str>>) -> Self { self.url_pattern = Some(url_pattern.into()); self }
    /// If set, only requests for matching resource types will be intercepted.
    pub fn resource_type(mut self, resource_type: crate::network::ResourceType) -> Self { self.resource_type = Some(resource_type); self }
    /// Stage at which to begin intercepting requests. Default is Request.
    pub fn request_stage(mut self, request_stage: impl Into<RequestStage>) -> Self { self.request_stage = Some(request_stage.into()); self }
    pub fn build(self) -> RequestPattern<'a> {
        RequestPattern {
            url_pattern: self.url_pattern,
            resource_type: self.resource_type,
            request_stage: self.request_stage,
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: 
    /// * `value`: 
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "handleAuthRequests")]
    handle_auth_requests: Option<bool>,
}

impl<'a> EnableParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> EnableParamsBuilder<'a> {
        EnableParamsBuilder {
            patterns: None,
            handle_auth_requests: None,
        }
    }
    /// If specified, only requests matching any of these patterns will produce
    /// fetchRequested event and will be paused until clients response. If not set,
    /// all requests will be affected.
    pub fn patterns(&self) -> Option<&[RequestPattern<'a>]> { self.patterns.as_deref() }
    /// If true, authRequired events will be issued and requests will be paused
    /// expecting a call to continueWithAuth.
    pub fn handle_auth_requests(&self) -> Option<bool> { self.handle_auth_requests }
}

#[derive(Default)]
pub struct EnableParamsBuilder<'a> {
    patterns: Option<Vec<RequestPattern<'a>>>,
    handle_auth_requests: Option<bool>,
}

impl<'a> EnableParamsBuilder<'a> {
    /// If specified, only requests matching any of these patterns will produce
    /// fetchRequested event and will be paused until clients response. If not set,
    /// all requests will be affected.
    pub fn patterns(mut self, patterns: Vec<RequestPattern<'a>>) -> Self { self.patterns = Some(patterns); self }
    /// If true, authRequired events will be issued and requests will be paused
    /// expecting a call to continueWithAuth.
    pub fn handle_auth_requests(mut self, handle_auth_requests: bool) -> Self { self.handle_auth_requests = Some(handle_auth_requests); self }
    pub fn build(self) -> EnableParams<'a> {
        EnableParams {
            patterns: self.patterns,
            handle_auth_requests: self.handle_auth_requests,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
    /// Causes the request to fail with the given reason.
    #[serde(rename = "errorReason")]
    error_reason: crate::network::ErrorReason,
}

impl<'a> FailRequestParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: An id the client received in requestPaused event.
    /// * `error_reason`: Causes the request to fail with the given reason.
    pub fn builder(request_id: impl Into<RequestId<'a>>, error_reason: crate::network::ErrorReason) -> FailRequestParamsBuilder<'a> {
        FailRequestParamsBuilder {
            request_id: request_id.into(),
            error_reason: error_reason,
        }
    }
    /// An id the client received in requestPaused event.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
    /// Causes the request to fail with the given reason.
    pub fn error_reason(&self) -> &crate::network::ErrorReason { &self.error_reason }
}


pub struct FailRequestParamsBuilder<'a> {
    request_id: RequestId<'a>,
    error_reason: crate::network::ErrorReason,
}

impl<'a> FailRequestParamsBuilder<'a> {
    pub fn build(self) -> FailRequestParams<'a> {
        FailRequestParams {
            request_id: self.request_id,
            error_reason: self.error_reason,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
    /// An HTTP response code.
    #[serde(rename = "responseCode")]
    response_code: i64,
    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseHeaders")]
    response_headers: Option<Vec<HeaderEntry<'a>>>,
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "binaryResponseHeaders")]
    binary_response_headers: Option<Cow<'a, str>>,
    /// A response body. If absent, original response body will be used if
    /// the request is intercepted at the response stage and empty body
    /// will be used if the request is intercepted at the request stage. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Cow<'a, str>>,
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responsePhrase")]
    response_phrase: Option<Cow<'a, str>>,
}

impl<'a> FulfillRequestParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: An id the client received in requestPaused event.
    /// * `response_code`: An HTTP response code.
    pub fn builder(request_id: impl Into<RequestId<'a>>, response_code: i64) -> FulfillRequestParamsBuilder<'a> {
        FulfillRequestParamsBuilder {
            request_id: request_id.into(),
            response_code: response_code,
            response_headers: None,
            binary_response_headers: None,
            body: None,
            response_phrase: None,
        }
    }
    /// An id the client received in requestPaused event.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
    /// An HTTP response code.
    pub fn response_code(&self) -> i64 { self.response_code }
    /// Response headers.
    pub fn response_headers(&self) -> Option<&[HeaderEntry<'a>]> { self.response_headers.as_deref() }
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    pub fn binary_response_headers(&self) -> Option<&str> { self.binary_response_headers.as_deref() }
    /// A response body. If absent, original response body will be used if
    /// the request is intercepted at the response stage and empty body
    /// will be used if the request is intercepted at the request stage. (Encoded as a base64 string when passed over JSON)
    pub fn body(&self) -> Option<&str> { self.body.as_deref() }
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    pub fn response_phrase(&self) -> Option<&str> { self.response_phrase.as_deref() }
}


pub struct FulfillRequestParamsBuilder<'a> {
    request_id: RequestId<'a>,
    response_code: i64,
    response_headers: Option<Vec<HeaderEntry<'a>>>,
    binary_response_headers: Option<Cow<'a, str>>,
    body: Option<Cow<'a, str>>,
    response_phrase: Option<Cow<'a, str>>,
}

impl<'a> FulfillRequestParamsBuilder<'a> {
    /// Response headers.
    pub fn response_headers(mut self, response_headers: Vec<HeaderEntry<'a>>) -> Self { self.response_headers = Some(response_headers); self }
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    pub fn binary_response_headers(mut self, binary_response_headers: impl Into<Cow<'a, str>>) -> Self { self.binary_response_headers = Some(binary_response_headers.into()); self }
    /// A response body. If absent, original response body will be used if
    /// the request is intercepted at the response stage and empty body
    /// will be used if the request is intercepted at the request stage. (Encoded as a base64 string when passed over JSON)
    pub fn body(mut self, body: impl Into<Cow<'a, str>>) -> Self { self.body = Some(body.into()); self }
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    pub fn response_phrase(mut self, response_phrase: impl Into<Cow<'a, str>>) -> Self { self.response_phrase = Some(response_phrase.into()); self }
    pub fn build(self) -> FulfillRequestParams<'a> {
        FulfillRequestParams {
            request_id: self.request_id,
            response_code: self.response_code,
            response_headers: self.response_headers,
            binary_response_headers: self.binary_response_headers,
            body: self.body,
            response_phrase: self.response_phrase,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
    /// If set, the request url will be modified in a way that's not observable by page.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// If set, the request method is overridden.
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<Cow<'a, str>>,
    /// If set, overrides the post data in the request. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "postData")]
    post_data: Option<Cow<'a, str>>,
    /// If set, overrides the request headers. Note that the overrides do not
    /// extend to subsequent redirect hops, if a redirect happens. Another override
    /// may be applied to a different request produced by a redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<HeaderEntry<'a>>>,
    /// If set, overrides response interception behavior for this request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "interceptResponse")]
    intercept_response: Option<bool>,
}

impl<'a> ContinueRequestParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: An id the client received in requestPaused event.
    pub fn builder(request_id: impl Into<RequestId<'a>>) -> ContinueRequestParamsBuilder<'a> {
        ContinueRequestParamsBuilder {
            request_id: request_id.into(),
            url: None,
            method: None,
            post_data: None,
            headers: None,
            intercept_response: None,
        }
    }
    /// An id the client received in requestPaused event.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
    /// If set, the request url will be modified in a way that's not observable by page.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// If set, the request method is overridden.
    pub fn method(&self) -> Option<&str> { self.method.as_deref() }
    /// If set, overrides the post data in the request. (Encoded as a base64 string when passed over JSON)
    pub fn post_data(&self) -> Option<&str> { self.post_data.as_deref() }
    /// If set, overrides the request headers. Note that the overrides do not
    /// extend to subsequent redirect hops, if a redirect happens. Another override
    /// may be applied to a different request produced by a redirect.
    pub fn headers(&self) -> Option<&[HeaderEntry<'a>]> { self.headers.as_deref() }
    /// If set, overrides response interception behavior for this request.
    pub fn intercept_response(&self) -> Option<bool> { self.intercept_response }
}


pub struct ContinueRequestParamsBuilder<'a> {
    request_id: RequestId<'a>,
    url: Option<Cow<'a, str>>,
    method: Option<Cow<'a, str>>,
    post_data: Option<Cow<'a, str>>,
    headers: Option<Vec<HeaderEntry<'a>>>,
    intercept_response: Option<bool>,
}

impl<'a> ContinueRequestParamsBuilder<'a> {
    /// If set, the request url will be modified in a way that's not observable by page.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// If set, the request method is overridden.
    pub fn method(mut self, method: impl Into<Cow<'a, str>>) -> Self { self.method = Some(method.into()); self }
    /// If set, overrides the post data in the request. (Encoded as a base64 string when passed over JSON)
    pub fn post_data(mut self, post_data: impl Into<Cow<'a, str>>) -> Self { self.post_data = Some(post_data.into()); self }
    /// If set, overrides the request headers. Note that the overrides do not
    /// extend to subsequent redirect hops, if a redirect happens. Another override
    /// may be applied to a different request produced by a redirect.
    pub fn headers(mut self, headers: Vec<HeaderEntry<'a>>) -> Self { self.headers = Some(headers); self }
    /// If set, overrides response interception behavior for this request.
    pub fn intercept_response(mut self, intercept_response: bool) -> Self { self.intercept_response = Some(intercept_response); self }
    pub fn build(self) -> ContinueRequestParams<'a> {
        ContinueRequestParams {
            request_id: self.request_id,
            url: self.url,
            method: self.method,
            post_data: self.post_data,
            headers: self.headers,
            intercept_response: self.intercept_response,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
    /// Response to  with an authChallenge.
    #[serde(rename = "authChallengeResponse")]
    auth_challenge_response: AuthChallengeResponse<'a>,
}

impl<'a> ContinueWithAuthParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: An id the client received in authRequired event.
    /// * `auth_challenge_response`: Response to  with an authChallenge.
    pub fn builder(request_id: impl Into<RequestId<'a>>, auth_challenge_response: AuthChallengeResponse<'a>) -> ContinueWithAuthParamsBuilder<'a> {
        ContinueWithAuthParamsBuilder {
            request_id: request_id.into(),
            auth_challenge_response: auth_challenge_response,
        }
    }
    /// An id the client received in authRequired event.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
    /// Response to  with an authChallenge.
    pub fn auth_challenge_response(&self) -> &AuthChallengeResponse<'a> { &self.auth_challenge_response }
}


pub struct ContinueWithAuthParamsBuilder<'a> {
    request_id: RequestId<'a>,
    auth_challenge_response: AuthChallengeResponse<'a>,
}

impl<'a> ContinueWithAuthParamsBuilder<'a> {
    pub fn build(self) -> ContinueWithAuthParams<'a> {
        ContinueWithAuthParams {
            request_id: self.request_id,
            auth_challenge_response: self.auth_challenge_response,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
    /// An HTTP response code. If absent, original response code will be used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseCode")]
    response_code: Option<i64>,
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responsePhrase")]
    response_phrase: Option<Cow<'a, str>>,
    /// Response headers. If absent, original response headers will be used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseHeaders")]
    response_headers: Option<Vec<HeaderEntry<'a>>>,
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "binaryResponseHeaders")]
    binary_response_headers: Option<Cow<'a, str>>,
}

impl<'a> ContinueResponseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: An id the client received in requestPaused event.
    pub fn builder(request_id: impl Into<RequestId<'a>>) -> ContinueResponseParamsBuilder<'a> {
        ContinueResponseParamsBuilder {
            request_id: request_id.into(),
            response_code: None,
            response_phrase: None,
            response_headers: None,
            binary_response_headers: None,
        }
    }
    /// An id the client received in requestPaused event.
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
    /// An HTTP response code. If absent, original response code will be used.
    pub fn response_code(&self) -> Option<i64> { self.response_code }
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    pub fn response_phrase(&self) -> Option<&str> { self.response_phrase.as_deref() }
    /// Response headers. If absent, original response headers will be used.
    pub fn response_headers(&self) -> Option<&[HeaderEntry<'a>]> { self.response_headers.as_deref() }
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    pub fn binary_response_headers(&self) -> Option<&str> { self.binary_response_headers.as_deref() }
}


pub struct ContinueResponseParamsBuilder<'a> {
    request_id: RequestId<'a>,
    response_code: Option<i64>,
    response_phrase: Option<Cow<'a, str>>,
    response_headers: Option<Vec<HeaderEntry<'a>>>,
    binary_response_headers: Option<Cow<'a, str>>,
}

impl<'a> ContinueResponseParamsBuilder<'a> {
    /// An HTTP response code. If absent, original response code will be used.
    pub fn response_code(mut self, response_code: i64) -> Self { self.response_code = Some(response_code); self }
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    pub fn response_phrase(mut self, response_phrase: impl Into<Cow<'a, str>>) -> Self { self.response_phrase = Some(response_phrase.into()); self }
    /// Response headers. If absent, original response headers will be used.
    pub fn response_headers(mut self, response_headers: Vec<HeaderEntry<'a>>) -> Self { self.response_headers = Some(response_headers); self }
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)
    pub fn binary_response_headers(mut self, binary_response_headers: impl Into<Cow<'a, str>>) -> Self { self.binary_response_headers = Some(binary_response_headers.into()); self }
    pub fn build(self) -> ContinueResponseParams<'a> {
        ContinueResponseParams {
            request_id: self.request_id,
            response_code: self.response_code,
            response_phrase: self.response_phrase,
            response_headers: self.response_headers,
            binary_response_headers: self.binary_response_headers,
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
}

impl<'a> GetResponseBodyParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: Identifier for the intercepted request to get body for.
    pub fn builder(request_id: impl Into<RequestId<'a>>) -> GetResponseBodyParamsBuilder<'a> {
        GetResponseBodyParamsBuilder {
            request_id: request_id.into(),
        }
    }
    /// Identifier for the intercepted request to get body for.
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
    #[serde(rename = "requestId")]
    request_id: RequestId<'a>,
}

impl<'a> TakeResponseBodyAsStreamParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_id`: 
    pub fn builder(request_id: impl Into<RequestId<'a>>) -> TakeResponseBodyAsStreamParamsBuilder<'a> {
        TakeResponseBodyAsStreamParamsBuilder {
            request_id: request_id.into(),
        }
    }
    pub fn request_id(&self) -> &RequestId<'a> { &self.request_id }
}


pub struct TakeResponseBodyAsStreamParamsBuilder<'a> {
    request_id: RequestId<'a>,
}

impl<'a> TakeResponseBodyAsStreamParamsBuilder<'a> {
    pub fn build(self) -> TakeResponseBodyAsStreamParams<'a> {
        TakeResponseBodyAsStreamParams {
            request_id: self.request_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `stream`: 
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
