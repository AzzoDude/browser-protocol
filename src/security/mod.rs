use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// An internal certificate ID value.

pub type CertificateId = i64;

/// A description of mixed content (HTTP resources on HTTPS pages), as defined by
/// <https://www.w3.org/TR/mixed-content/#categories>

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MixedContentType {
    #[default]
    #[serde(rename = "blockable")]
    Blockable,
    #[serde(rename = "optionally-blockable")]
    OptionallyBlockable,
    #[serde(rename = "none")]
    None,
}

/// The security level of a page or resource.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SecurityState {
    #[default]
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "insecure")]
    Insecure,
    #[serde(rename = "secure")]
    Secure,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "insecure-broken")]
    InsecureBroken,
}

/// Details about the security state of the page certificate.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSecurityState<'a> {
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
    /// Page certificate.
    pub certificate: Vec<Cow<'a, str>>,
    /// Certificate subject name.
    #[serde(rename = "subjectName")]
    pub subject_name: Cow<'a, str>,
    /// Name of the issuing CA.
    pub issuer: Cow<'a, str>,
    /// Certificate valid from date.
    #[serde(rename = "validFrom")]
    pub valid_from: crate::network::TimeSinceEpoch,
    /// Certificate valid to (expiration) date
    #[serde(rename = "validTo")]
    pub valid_to: crate::network::TimeSinceEpoch,
    /// The highest priority network error code, if the certificate has an error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "certificateNetworkError")]
    pub certificate_network_error: Option<Cow<'a, str>>,
    /// True if the certificate uses a weak signature algorithm.
    #[serde(rename = "certificateHasWeakSignature")]
    pub certificate_has_weak_signature: bool,
    /// True if the certificate has a SHA1 signature in the chain.
    #[serde(rename = "certificateHasSha1Signature")]
    pub certificate_has_sha1_signature: bool,
    /// True if modern SSL
    #[serde(rename = "modernSSL")]
    pub modern_ssl: bool,
    /// True if the connection is using an obsolete SSL protocol.
    #[serde(rename = "obsoleteSslProtocol")]
    pub obsolete_ssl_protocol: bool,
    /// True if the connection is using an obsolete SSL key exchange.
    #[serde(rename = "obsoleteSslKeyExchange")]
    pub obsolete_ssl_key_exchange: bool,
    /// True if the connection is using an obsolete SSL cipher.
    #[serde(rename = "obsoleteSslCipher")]
    pub obsolete_ssl_cipher: bool,
    /// True if the connection is using an obsolete SSL signature.
    #[serde(rename = "obsoleteSslSignature")]
    pub obsolete_ssl_signature: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SafetyTipStatus {
    #[default]
    #[serde(rename = "badReputation")]
    BadReputation,
    #[serde(rename = "lookalike")]
    Lookalike,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SafetyTipInfo<'a> {
    /// Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.
    #[serde(rename = "safetyTipStatus")]
    pub safety_tip_status: SafetyTipStatus,
    /// The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.
    #[serde(skip_serializing_if = "Option::is_none", rename = "safeUrl")]
    pub safe_url: Option<Cow<'a, str>>,
}
/// Security state information about the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct VisibleSecurityState<'a> {
    /// The security level of the page.
    #[serde(rename = "securityState")]
    pub security_state: SecurityState,
    /// Security state details about the page certificate.
    #[serde(skip_serializing_if = "Option::is_none", rename = "certificateSecurityState")]
    pub certificate_security_state: Option<CertificateSecurityState<'a>>,
    /// The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.
    #[serde(skip_serializing_if = "Option::is_none", rename = "safetyTipInfo")]
    pub safety_tip_info: Option<SafetyTipInfo<'a>>,
    /// Array of security state issues ids.
    #[serde(rename = "securityStateIssueIds")]
    pub security_state_issue_ids: Vec<Cow<'a, str>>,
}
/// An explanation of an factor contributing to the security state.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStateExplanation<'a> {
    /// Security state representing the severity of the factor being explained.
    #[serde(rename = "securityState")]
    pub security_state: SecurityState,
    /// Title describing the type of factor.
    pub title: Cow<'a, str>,
    /// Short phrase describing the type of factor.
    pub summary: Cow<'a, str>,
    /// Full text explanation of the factor.
    pub description: Cow<'a, str>,
    /// The type of mixed content described by the explanation.
    #[serde(rename = "mixedContentType")]
    pub mixed_content_type: MixedContentType,
    /// Page certificate.
    pub certificate: Vec<Cow<'a, str>>,
    /// Recommendations to fix any issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<Cow<'a, str>>>,
}
/// Information about insecure content on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InsecureContentStatus {
    /// Always false.
    #[serde(rename = "ranMixedContent")]
    pub ran_mixed_content: bool,
    /// Always false.
    #[serde(rename = "displayedMixedContent")]
    pub displayed_mixed_content: bool,
    /// Always false.
    #[serde(rename = "containedMixedForm")]
    pub contained_mixed_form: bool,
    /// Always false.
    #[serde(rename = "ranContentWithCertErrors")]
    pub ran_content_with_cert_errors: bool,
    /// Always false.
    #[serde(rename = "displayedContentWithCertErrors")]
    pub displayed_content_with_cert_errors: bool,
    /// Always set to unknown.
    #[serde(rename = "ranInsecureContentStyle")]
    pub ran_insecure_content_style: SecurityState,
    /// Always set to unknown.
    #[serde(rename = "displayedInsecureContentStyle")]
    pub displayed_insecure_content_style: SecurityState,
}
/// The action to take when a certificate error occurs. continue will continue processing the
/// request and cancel will cancel the request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CertificateErrorAction {
    #[default]
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "cancel")]
    Cancel,
}

/// Disables tracking security state changes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Security.disable")]
pub struct DisableParams {

}
/// Enables tracking security state changes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Security.enable")]
pub struct EnableParams {

}
/// Enable/disable whether all certificate errors should be ignored.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Security.setIgnoreCertificateErrors")]
pub struct SetIgnoreCertificateErrorsParams {
    /// If true, all certificate errors will be ignored.
    pub ignore: bool,
}
/// Handles a certificate error that fired a certificateError event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Security.handleCertificateError")]
pub struct HandleCertificateErrorParams {
    /// The ID of the event.
    #[serde(rename = "eventId")]
    pub event_id: u64,
    /// The action to take on the certificate error.
    pub action: CertificateErrorAction,
}
/// Enable/disable overriding certificate errors. If enabled, all certificate error events need to
/// be handled by the DevTools client and should be answered with 'handleCertificateError' commands.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Security.setOverrideCertificateErrors")]
pub struct SetOverrideCertificateErrorsParams {
    /// If true, certificate errors will be overridden.
    #[serde(rename = "override")]
    pub override_: bool,
}
/// There is a certificate error. If overriding certificate errors is enabled, then it should be
/// handled with the 'handleCertificateError' command. Note: this event does not fire if the
/// certificate error has been allowed internally. Only one client per target should override
/// certificate errors at the same time.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Security.certificateError")]
pub struct CertificateError<'a> {
    /// The ID of the event.
    #[serde(rename = "eventId")]
    pub event_id: u64,
    /// The type of the error.
    #[serde(rename = "errorType")]
    pub error_type: Cow<'a, str>,
    /// The url that was requested.
    #[serde(rename = "requestURL")]
    pub request_url: Cow<'a, str>,
}
/// The security state of the page changed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Security.visibleSecurityStateChanged")]
pub struct VisibleSecurityStateChanged<'a> {
    /// Security state information about the page.
    #[serde(rename = "visibleSecurityState")]
    pub visible_security_state: VisibleSecurityState<'a>,
}
/// The security state of the page changed. No longer being sent.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Security.securityStateChanged")]
pub struct SecurityStateChanged<'a> {
    /// Security state.
    #[serde(rename = "securityState")]
    pub security_state: SecurityState,
    /// True if the page was loaded over cryptographic transport such as HTTPS.
    #[serde(rename = "schemeIsCryptographic")]
    pub scheme_is_cryptographic: bool,
    /// Previously a list of explanations for the security state. Now always
    /// empty.
    pub explanations: Vec<SecurityStateExplanation<'a>>,
    /// Information about insecure content on the page.
    #[serde(rename = "insecureContentStatus")]
    pub insecure_content_status: InsecureContentStatus,
    /// Overrides user-visible description of the state. Always omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Cow<'a, str>>,
}