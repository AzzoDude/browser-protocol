//! A domain for letting clients substitute browser's network layer with client code.

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Unique request identifier.
/// Note that this does not identify individual HTTP requests that are part of
/// a network request.

pub type RequestId = String;

/// Stages of the request to handle. Request will intercept before the request is
/// sent. Response will intercept after the response is received (but before response
/// body is received).

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum RequestStage {
    #[default]
    Request,
    Response,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestPattern {
    /// Wildcards (''*'' -\> zero or more, ''?'' -\> exactly one) are allowed. Escape character is
    /// backslash. Omitting is equivalent to '"*"'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlPattern: Option<String>,
    /// If set, only requests for matching resource types will be intercepted.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resourceType: Option<crate::network::ResourceType>,
    /// Stage at which to begin intercepting requests. Default is Request.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestStage: Option<RequestStage>,
}

/// Response HTTP header entry

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeaderEntry {

    pub name: String,

    pub value: String,
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

/// Enables issuing of requestPaused events. A request will be paused until client
/// calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// If specified, only requests matching any of these patterns will produce
    /// fetchRequested event and will be paused until clients response. If not set,
    /// all requests will be affected.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Vec<RequestPattern>>,
    /// If true, authRequired events will be issued and requests will be paused
    /// expecting a call to continueWithAuth.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub handleAuthRequests: Option<bool>,
}

/// Causes the request to fail with specified reason.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FailRequestParams {
    /// An id the client received in requestPaused event.

    pub requestId: RequestId,
    /// Causes the request to fail with the given reason.

    pub errorReason: crate::network::ErrorReason,
}

/// Provides response to the request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FulfillRequestParams {
    /// An id the client received in requestPaused event.

    pub requestId: RequestId,
    /// An HTTP response code.

    pub responseCode: i64,
    /// Response headers.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseHeaders: Option<Vec<HeaderEntry>>,
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub binaryResponseHeaders: Option<String>,
    /// A response body. If absent, original response body will be used if
    /// the request is intercepted at the response stage and empty body
    /// will be used if the request is intercepted at the request stage. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsePhrase: Option<String>,
}

/// Continues the request, optionally modifying some of its parameters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueRequestParams {
    /// An id the client received in requestPaused event.

    pub requestId: RequestId,
    /// If set, the request url will be modified in a way that's not observable by page.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// If set, the request method is overridden.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// If set, overrides the post data in the request. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postData: Option<String>,
    /// If set, overrides the request headers. Note that the overrides do not
    /// extend to subsequent redirect hops, if a redirect happens. Another override
    /// may be applied to a different request produced by a redirect.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HeaderEntry>>,
    /// If set, overrides response interception behavior for this request.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interceptResponse: Option<bool>,
}

/// Continues a request supplying authChallengeResponse following authRequired event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueWithAuthParams {
    /// An id the client received in authRequired event.

    pub requestId: RequestId,
    /// Response to  with an authChallenge.

    pub authChallengeResponse: AuthChallengeResponse,
}

/// Continues loading of the paused response, optionally modifying the
/// response headers. If either responseCode or headers are modified, all of them
/// must be present.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueResponseParams {
    /// An id the client received in requestPaused event.

    pub requestId: RequestId,
    /// An HTTP response code. If absent, original response code will be used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseCode: Option<i64>,
    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsePhrase: Option<String>,
    /// Response headers. If absent, original response headers will be used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseHeaders: Option<Vec<HeaderEntry>>,
    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs. Prefer the above method unless you
    /// need to represent some non-UTF8 values that can't be transmitted
    /// over the protocol as text. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub binaryResponseHeaders: Option<String>,
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
pub struct GetResponseBodyParams {
    /// Identifier for the intercepted request to get body for.

    pub requestId: RequestId,
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
pub struct GetResponseBodyReturns {
    /// Response body.

    pub body: String,
    /// True, if content was sent as base64.

    pub base64Encoded: bool,
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
pub struct TakeResponseBodyAsStreamParams {

    pub requestId: RequestId,
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
pub struct TakeResponseBodyAsStreamReturns {

    pub stream: crate::io::StreamHandle,
}
