use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// An internal certificate ID value.

pub type CertificateId = i64;

/// A description of mixed content (HTTP resources on HTTPS pages), as defined by
/// <https://www.w3.org/TR/mixed-content/#categories>

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MixedContentType {
    #[default]
    Blockable,
    OptionallyBlockable,
    None,
}

/// The security level of a page or resource.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SecurityState {
    #[default]
    Unknown,
    Neutral,
    Insecure,
    Secure,
    Info,
    InsecureBroken,
}

/// Details about the security state of the page certificate.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSecurityState {
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
    /// Page certificate.

    pub certificate: Vec<String>,
    /// Certificate subject name.

    pub subjectName: String,
    /// Name of the issuing CA.

    pub issuer: String,
    /// Certificate valid from date.

    pub validFrom: crate::network::TimeSinceEpoch,
    /// Certificate valid to (expiration) date

    pub validTo: crate::network::TimeSinceEpoch,
    /// The highest priority network error code, if the certificate has an error.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificateNetworkError: Option<String>,
    /// True if the certificate uses a weak signature algorithm.

    pub certificateHasWeakSignature: bool,
    /// True if the certificate has a SHA1 signature in the chain.

    pub certificateHasSha1Signature: bool,
    /// True if modern SSL

    pub modernSSL: bool,
    /// True if the connection is using an obsolete SSL protocol.

    pub obsoleteSslProtocol: bool,
    /// True if the connection is using an obsolete SSL key exchange.

    pub obsoleteSslKeyExchange: bool,
    /// True if the connection is using an obsolete SSL cipher.

    pub obsoleteSslCipher: bool,
    /// True if the connection is using an obsolete SSL signature.

    pub obsoleteSslSignature: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SafetyTipStatus {
    #[default]
    BadReputation,
    Lookalike,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SafetyTipInfo {
    /// Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.

    pub safetyTipStatus: SafetyTipStatus,
    /// The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub safeUrl: Option<String>,
}

/// Security state information about the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VisibleSecurityState {
    /// The security level of the page.

    pub securityState: SecurityState,
    /// Security state details about the page certificate.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificateSecurityState: Option<CertificateSecurityState>,
    /// The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub safetyTipInfo: Option<SafetyTipInfo>,
    /// Array of security state issues ids.

    pub securityStateIssueIds: Vec<String>,
}

/// An explanation of an factor contributing to the security state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStateExplanation {
    /// Security state representing the severity of the factor being explained.

    pub securityState: SecurityState,
    /// Title describing the type of factor.

    pub title: String,
    /// Short phrase describing the type of factor.

    pub summary: String,
    /// Full text explanation of the factor.

    pub description: String,
    /// The type of mixed content described by the explanation.

    pub mixedContentType: MixedContentType,
    /// Page certificate.

    pub certificate: Vec<String>,
    /// Recommendations to fix any issues.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<String>>,
}

/// Information about insecure content on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsecureContentStatus {
    /// Always false.

    pub ranMixedContent: bool,
    /// Always false.

    pub displayedMixedContent: bool,
    /// Always false.

    pub containedMixedForm: bool,
    /// Always false.

    pub ranContentWithCertErrors: bool,
    /// Always false.

    pub displayedContentWithCertErrors: bool,
    /// Always set to unknown.

    pub ranInsecureContentStyle: SecurityState,
    /// Always set to unknown.

    pub displayedInsecureContentStyle: SecurityState,
}

/// The action to take when a certificate error occurs. continue will continue processing the
/// request and cancel will cancel the request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CertificateErrorAction {
    #[default]
    Continue,
    Cancel,
}

/// Enable/disable whether all certificate errors should be ignored.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetIgnoreCertificateErrorsParams {
    /// If true, all certificate errors will be ignored.

    pub ignore: bool,
}

/// Handles a certificate error that fired a certificateError event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HandleCertificateErrorParams {
    /// The ID of the event.

    pub eventId: u64,
    /// The action to take on the certificate error.

    pub action: CertificateErrorAction,
}

/// Enable/disable overriding certificate errors. If enabled, all certificate error events need to
/// be handled by the DevTools client and should be answered with 'handleCertificateError' commands.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetOverrideCertificateErrorsParams {
    /// If true, certificate errors will be overridden.

    #[serde(rename = "override")]
    pub override_: bool,
}
