use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSecurityState<'a> {
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
    /// Page certificate.
    certificate: Vec<Cow<'a, str>>,
    /// Certificate subject name.
    #[serde(rename = "subjectName")]
    subject_name: Cow<'a, str>,
    /// Name of the issuing CA.
    issuer: Cow<'a, str>,
    /// Certificate valid from date.
    #[serde(rename = "validFrom")]
    valid_from: crate::network::TimeSinceEpoch,
    /// Certificate valid to (expiration) date
    #[serde(rename = "validTo")]
    valid_to: crate::network::TimeSinceEpoch,
    /// The highest priority network error code, if the certificate has an error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "certificateNetworkError")]
    certificate_network_error: Option<Cow<'a, str>>,
    /// True if the certificate uses a weak signature algorithm.
    #[serde(rename = "certificateHasWeakSignature")]
    certificate_has_weak_signature: bool,
    /// True if the certificate has a SHA1 signature in the chain.
    #[serde(rename = "certificateHasSha1Signature")]
    certificate_has_sha1_signature: bool,
    /// True if modern SSL
    #[serde(rename = "modernSSL")]
    modern_ssl: bool,
    /// True if the connection is using an obsolete SSL protocol.
    #[serde(rename = "obsoleteSslProtocol")]
    obsolete_ssl_protocol: bool,
    /// True if the connection is using an obsolete SSL key exchange.
    #[serde(rename = "obsoleteSslKeyExchange")]
    obsolete_ssl_key_exchange: bool,
    /// True if the connection is using an obsolete SSL cipher.
    #[serde(rename = "obsoleteSslCipher")]
    obsolete_ssl_cipher: bool,
    /// True if the connection is using an obsolete SSL signature.
    #[serde(rename = "obsoleteSslSignature")]
    obsolete_ssl_signature: bool,
}

impl<'a> CertificateSecurityState<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `protocol`: Protocol name (e.g. "TLS 1.2" or "QUIC").
    /// * `key_exchange`: Key Exchange used by the connection, or the empty string if not applicable.
    /// * `cipher`: Cipher name.
    /// * `certificate`: Page certificate.
    /// * `subject_name`: Certificate subject name.
    /// * `issuer`: Name of the issuing CA.
    /// * `valid_from`: Certificate valid from date.
    /// * `valid_to`: Certificate valid to (expiration) date
    /// * `certificate_has_weak_signature`: True if the certificate uses a weak signature algorithm.
    /// * `certificate_has_sha1_signature`: True if the certificate has a SHA1 signature in the chain.
    /// * `modern_ssl`: True if modern SSL
    /// * `obsolete_ssl_protocol`: True if the connection is using an obsolete SSL protocol.
    /// * `obsolete_ssl_key_exchange`: True if the connection is using an obsolete SSL key exchange.
    /// * `obsolete_ssl_cipher`: True if the connection is using an obsolete SSL cipher.
    /// * `obsolete_ssl_signature`: True if the connection is using an obsolete SSL signature.
    pub fn builder(protocol: impl Into<Cow<'a, str>>, key_exchange: impl Into<Cow<'a, str>>, cipher: impl Into<Cow<'a, str>>, certificate: Vec<Cow<'a, str>>, subject_name: impl Into<Cow<'a, str>>, issuer: impl Into<Cow<'a, str>>, valid_from: crate::network::TimeSinceEpoch, valid_to: crate::network::TimeSinceEpoch, certificate_has_weak_signature: bool, certificate_has_sha1_signature: bool, modern_ssl: bool, obsolete_ssl_protocol: bool, obsolete_ssl_key_exchange: bool, obsolete_ssl_cipher: bool, obsolete_ssl_signature: bool) -> CertificateSecurityStateBuilder<'a> {
        CertificateSecurityStateBuilder {
            protocol: protocol.into(),
            key_exchange: key_exchange.into(),
            key_exchange_group: None,
            cipher: cipher.into(),
            mac: None,
            certificate: certificate,
            subject_name: subject_name.into(),
            issuer: issuer.into(),
            valid_from: valid_from,
            valid_to: valid_to,
            certificate_network_error: None,
            certificate_has_weak_signature: certificate_has_weak_signature,
            certificate_has_sha1_signature: certificate_has_sha1_signature,
            modern_ssl: modern_ssl,
            obsolete_ssl_protocol: obsolete_ssl_protocol,
            obsolete_ssl_key_exchange: obsolete_ssl_key_exchange,
            obsolete_ssl_cipher: obsolete_ssl_cipher,
            obsolete_ssl_signature: obsolete_ssl_signature,
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
    /// Page certificate.
    pub fn certificate(&self) -> &[Cow<'a, str>] { &self.certificate }
    /// Certificate subject name.
    pub fn subject_name(&self) -> &str { self.subject_name.as_ref() }
    /// Name of the issuing CA.
    pub fn issuer(&self) -> &str { self.issuer.as_ref() }
    /// Certificate valid from date.
    pub fn valid_from(&self) -> &crate::network::TimeSinceEpoch { &self.valid_from }
    /// Certificate valid to (expiration) date
    pub fn valid_to(&self) -> &crate::network::TimeSinceEpoch { &self.valid_to }
    /// The highest priority network error code, if the certificate has an error.
    pub fn certificate_network_error(&self) -> Option<&str> { self.certificate_network_error.as_deref() }
    /// True if the certificate uses a weak signature algorithm.
    pub fn certificate_has_weak_signature(&self) -> bool { self.certificate_has_weak_signature }
    /// True if the certificate has a SHA1 signature in the chain.
    pub fn certificate_has_sha1_signature(&self) -> bool { self.certificate_has_sha1_signature }
    /// True if modern SSL
    pub fn modern_ssl(&self) -> bool { self.modern_ssl }
    /// True if the connection is using an obsolete SSL protocol.
    pub fn obsolete_ssl_protocol(&self) -> bool { self.obsolete_ssl_protocol }
    /// True if the connection is using an obsolete SSL key exchange.
    pub fn obsolete_ssl_key_exchange(&self) -> bool { self.obsolete_ssl_key_exchange }
    /// True if the connection is using an obsolete SSL cipher.
    pub fn obsolete_ssl_cipher(&self) -> bool { self.obsolete_ssl_cipher }
    /// True if the connection is using an obsolete SSL signature.
    pub fn obsolete_ssl_signature(&self) -> bool { self.obsolete_ssl_signature }
}


pub struct CertificateSecurityStateBuilder<'a> {
    protocol: Cow<'a, str>,
    key_exchange: Cow<'a, str>,
    key_exchange_group: Option<Cow<'a, str>>,
    cipher: Cow<'a, str>,
    mac: Option<Cow<'a, str>>,
    certificate: Vec<Cow<'a, str>>,
    subject_name: Cow<'a, str>,
    issuer: Cow<'a, str>,
    valid_from: crate::network::TimeSinceEpoch,
    valid_to: crate::network::TimeSinceEpoch,
    certificate_network_error: Option<Cow<'a, str>>,
    certificate_has_weak_signature: bool,
    certificate_has_sha1_signature: bool,
    modern_ssl: bool,
    obsolete_ssl_protocol: bool,
    obsolete_ssl_key_exchange: bool,
    obsolete_ssl_cipher: bool,
    obsolete_ssl_signature: bool,
}

impl<'a> CertificateSecurityStateBuilder<'a> {
    /// (EC)DH group used by the connection, if applicable.
    pub fn key_exchange_group(mut self, key_exchange_group: impl Into<Cow<'a, str>>) -> Self { self.key_exchange_group = Some(key_exchange_group.into()); self }
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.
    pub fn mac(mut self, mac: impl Into<Cow<'a, str>>) -> Self { self.mac = Some(mac.into()); self }
    /// The highest priority network error code, if the certificate has an error.
    pub fn certificate_network_error(mut self, certificate_network_error: impl Into<Cow<'a, str>>) -> Self { self.certificate_network_error = Some(certificate_network_error.into()); self }
    pub fn build(self) -> CertificateSecurityState<'a> {
        CertificateSecurityState {
            protocol: self.protocol,
            key_exchange: self.key_exchange,
            key_exchange_group: self.key_exchange_group,
            cipher: self.cipher,
            mac: self.mac,
            certificate: self.certificate,
            subject_name: self.subject_name,
            issuer: self.issuer,
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            certificate_network_error: self.certificate_network_error,
            certificate_has_weak_signature: self.certificate_has_weak_signature,
            certificate_has_sha1_signature: self.certificate_has_sha1_signature,
            modern_ssl: self.modern_ssl,
            obsolete_ssl_protocol: self.obsolete_ssl_protocol,
            obsolete_ssl_key_exchange: self.obsolete_ssl_key_exchange,
            obsolete_ssl_cipher: self.obsolete_ssl_cipher,
            obsolete_ssl_signature: self.obsolete_ssl_signature,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SafetyTipStatus {
    #[default]
    #[serde(rename = "badReputation")]
    BadReputation,
    #[serde(rename = "lookalike")]
    Lookalike,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SafetyTipInfo<'a> {
    /// Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.
    #[serde(rename = "safetyTipStatus")]
    safety_tip_status: SafetyTipStatus,
    /// The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.
    #[serde(skip_serializing_if = "Option::is_none", rename = "safeUrl")]
    safe_url: Option<Cow<'a, str>>,
}

impl<'a> SafetyTipInfo<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `safety_tip_status`: Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.
    pub fn builder(safety_tip_status: impl Into<SafetyTipStatus>) -> SafetyTipInfoBuilder<'a> {
        SafetyTipInfoBuilder {
            safety_tip_status: safety_tip_status.into(),
            safe_url: None,
        }
    }
    /// Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.
    pub fn safety_tip_status(&self) -> &SafetyTipStatus { &self.safety_tip_status }
    /// The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.
    pub fn safe_url(&self) -> Option<&str> { self.safe_url.as_deref() }
}


pub struct SafetyTipInfoBuilder<'a> {
    safety_tip_status: SafetyTipStatus,
    safe_url: Option<Cow<'a, str>>,
}

impl<'a> SafetyTipInfoBuilder<'a> {
    /// The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.
    pub fn safe_url(mut self, safe_url: impl Into<Cow<'a, str>>) -> Self { self.safe_url = Some(safe_url.into()); self }
    pub fn build(self) -> SafetyTipInfo<'a> {
        SafetyTipInfo {
            safety_tip_status: self.safety_tip_status,
            safe_url: self.safe_url,
        }
    }
}

/// Security state information about the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VisibleSecurityState<'a> {
    /// The security level of the page.
    #[serde(rename = "securityState")]
    security_state: SecurityState,
    /// Security state details about the page certificate.
    #[serde(skip_serializing_if = "Option::is_none", rename = "certificateSecurityState")]
    certificate_security_state: Option<CertificateSecurityState<'a>>,
    /// The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.
    #[serde(skip_serializing_if = "Option::is_none", rename = "safetyTipInfo")]
    safety_tip_info: Option<SafetyTipInfo<'a>>,
    /// Array of security state issues ids.
    #[serde(rename = "securityStateIssueIds")]
    security_state_issue_ids: Vec<Cow<'a, str>>,
}

impl<'a> VisibleSecurityState<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `security_state`: The security level of the page.
    /// * `security_state_issue_ids`: Array of security state issues ids.
    pub fn builder(security_state: impl Into<SecurityState>, security_state_issue_ids: Vec<Cow<'a, str>>) -> VisibleSecurityStateBuilder<'a> {
        VisibleSecurityStateBuilder {
            security_state: security_state.into(),
            certificate_security_state: None,
            safety_tip_info: None,
            security_state_issue_ids: security_state_issue_ids,
        }
    }
    /// The security level of the page.
    pub fn security_state(&self) -> &SecurityState { &self.security_state }
    /// Security state details about the page certificate.
    pub fn certificate_security_state(&self) -> Option<&CertificateSecurityState<'a>> { self.certificate_security_state.as_ref() }
    /// The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.
    pub fn safety_tip_info(&self) -> Option<&SafetyTipInfo<'a>> { self.safety_tip_info.as_ref() }
    /// Array of security state issues ids.
    pub fn security_state_issue_ids(&self) -> &[Cow<'a, str>] { &self.security_state_issue_ids }
}


pub struct VisibleSecurityStateBuilder<'a> {
    security_state: SecurityState,
    certificate_security_state: Option<CertificateSecurityState<'a>>,
    safety_tip_info: Option<SafetyTipInfo<'a>>,
    security_state_issue_ids: Vec<Cow<'a, str>>,
}

impl<'a> VisibleSecurityStateBuilder<'a> {
    /// Security state details about the page certificate.
    pub fn certificate_security_state(mut self, certificate_security_state: CertificateSecurityState<'a>) -> Self { self.certificate_security_state = Some(certificate_security_state); self }
    /// The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.
    pub fn safety_tip_info(mut self, safety_tip_info: SafetyTipInfo<'a>) -> Self { self.safety_tip_info = Some(safety_tip_info); self }
    pub fn build(self) -> VisibleSecurityState<'a> {
        VisibleSecurityState {
            security_state: self.security_state,
            certificate_security_state: self.certificate_security_state,
            safety_tip_info: self.safety_tip_info,
            security_state_issue_ids: self.security_state_issue_ids,
        }
    }
}

/// An explanation of an factor contributing to the security state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStateExplanation<'a> {
    /// Security state representing the severity of the factor being explained.
    #[serde(rename = "securityState")]
    security_state: SecurityState,
    /// Title describing the type of factor.
    title: Cow<'a, str>,
    /// Short phrase describing the type of factor.
    summary: Cow<'a, str>,
    /// Full text explanation of the factor.
    description: Cow<'a, str>,
    /// The type of mixed content described by the explanation.
    #[serde(rename = "mixedContentType")]
    mixed_content_type: MixedContentType,
    /// Page certificate.
    certificate: Vec<Cow<'a, str>>,
    /// Recommendations to fix any issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    recommendations: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SecurityStateExplanation<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `security_state`: Security state representing the severity of the factor being explained.
    /// * `title`: Title describing the type of factor.
    /// * `summary`: Short phrase describing the type of factor.
    /// * `description`: Full text explanation of the factor.
    /// * `mixed_content_type`: The type of mixed content described by the explanation.
    /// * `certificate`: Page certificate.
    pub fn builder(security_state: impl Into<SecurityState>, title: impl Into<Cow<'a, str>>, summary: impl Into<Cow<'a, str>>, description: impl Into<Cow<'a, str>>, mixed_content_type: impl Into<MixedContentType>, certificate: Vec<Cow<'a, str>>) -> SecurityStateExplanationBuilder<'a> {
        SecurityStateExplanationBuilder {
            security_state: security_state.into(),
            title: title.into(),
            summary: summary.into(),
            description: description.into(),
            mixed_content_type: mixed_content_type.into(),
            certificate: certificate,
            recommendations: None,
        }
    }
    /// Security state representing the severity of the factor being explained.
    pub fn security_state(&self) -> &SecurityState { &self.security_state }
    /// Title describing the type of factor.
    pub fn title(&self) -> &str { self.title.as_ref() }
    /// Short phrase describing the type of factor.
    pub fn summary(&self) -> &str { self.summary.as_ref() }
    /// Full text explanation of the factor.
    pub fn description(&self) -> &str { self.description.as_ref() }
    /// The type of mixed content described by the explanation.
    pub fn mixed_content_type(&self) -> &MixedContentType { &self.mixed_content_type }
    /// Page certificate.
    pub fn certificate(&self) -> &[Cow<'a, str>] { &self.certificate }
    /// Recommendations to fix any issues.
    pub fn recommendations(&self) -> Option<&[Cow<'a, str>]> { self.recommendations.as_deref() }
}


pub struct SecurityStateExplanationBuilder<'a> {
    security_state: SecurityState,
    title: Cow<'a, str>,
    summary: Cow<'a, str>,
    description: Cow<'a, str>,
    mixed_content_type: MixedContentType,
    certificate: Vec<Cow<'a, str>>,
    recommendations: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SecurityStateExplanationBuilder<'a> {
    /// Recommendations to fix any issues.
    pub fn recommendations(mut self, recommendations: Vec<Cow<'a, str>>) -> Self { self.recommendations = Some(recommendations); self }
    pub fn build(self) -> SecurityStateExplanation<'a> {
        SecurityStateExplanation {
            security_state: self.security_state,
            title: self.title,
            summary: self.summary,
            description: self.description,
            mixed_content_type: self.mixed_content_type,
            certificate: self.certificate,
            recommendations: self.recommendations,
        }
    }
}

/// Information about insecure content on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsecureContentStatus {
    /// Always false.
    #[serde(rename = "ranMixedContent")]
    ran_mixed_content: bool,
    /// Always false.
    #[serde(rename = "displayedMixedContent")]
    displayed_mixed_content: bool,
    /// Always false.
    #[serde(rename = "containedMixedForm")]
    contained_mixed_form: bool,
    /// Always false.
    #[serde(rename = "ranContentWithCertErrors")]
    ran_content_with_cert_errors: bool,
    /// Always false.
    #[serde(rename = "displayedContentWithCertErrors")]
    displayed_content_with_cert_errors: bool,
    /// Always set to unknown.
    #[serde(rename = "ranInsecureContentStyle")]
    ran_insecure_content_style: SecurityState,
    /// Always set to unknown.
    #[serde(rename = "displayedInsecureContentStyle")]
    displayed_insecure_content_style: SecurityState,
}

impl InsecureContentStatus {
    /// Creates a builder for this type with the required parameters:
    /// * `ran_mixed_content`: Always false.
    /// * `displayed_mixed_content`: Always false.
    /// * `contained_mixed_form`: Always false.
    /// * `ran_content_with_cert_errors`: Always false.
    /// * `displayed_content_with_cert_errors`: Always false.
    /// * `ran_insecure_content_style`: Always set to unknown.
    /// * `displayed_insecure_content_style`: Always set to unknown.
    pub fn builder(ran_mixed_content: bool, displayed_mixed_content: bool, contained_mixed_form: bool, ran_content_with_cert_errors: bool, displayed_content_with_cert_errors: bool, ran_insecure_content_style: impl Into<SecurityState>, displayed_insecure_content_style: impl Into<SecurityState>) -> InsecureContentStatusBuilder {
        InsecureContentStatusBuilder {
            ran_mixed_content: ran_mixed_content,
            displayed_mixed_content: displayed_mixed_content,
            contained_mixed_form: contained_mixed_form,
            ran_content_with_cert_errors: ran_content_with_cert_errors,
            displayed_content_with_cert_errors: displayed_content_with_cert_errors,
            ran_insecure_content_style: ran_insecure_content_style.into(),
            displayed_insecure_content_style: displayed_insecure_content_style.into(),
        }
    }
    /// Always false.
    pub fn ran_mixed_content(&self) -> bool { self.ran_mixed_content }
    /// Always false.
    pub fn displayed_mixed_content(&self) -> bool { self.displayed_mixed_content }
    /// Always false.
    pub fn contained_mixed_form(&self) -> bool { self.contained_mixed_form }
    /// Always false.
    pub fn ran_content_with_cert_errors(&self) -> bool { self.ran_content_with_cert_errors }
    /// Always false.
    pub fn displayed_content_with_cert_errors(&self) -> bool { self.displayed_content_with_cert_errors }
    /// Always set to unknown.
    pub fn ran_insecure_content_style(&self) -> &SecurityState { &self.ran_insecure_content_style }
    /// Always set to unknown.
    pub fn displayed_insecure_content_style(&self) -> &SecurityState { &self.displayed_insecure_content_style }
}


pub struct InsecureContentStatusBuilder {
    ran_mixed_content: bool,
    displayed_mixed_content: bool,
    contained_mixed_form: bool,
    ran_content_with_cert_errors: bool,
    displayed_content_with_cert_errors: bool,
    ran_insecure_content_style: SecurityState,
    displayed_insecure_content_style: SecurityState,
}

impl InsecureContentStatusBuilder {
    pub fn build(self) -> InsecureContentStatus {
        InsecureContentStatus {
            ran_mixed_content: self.ran_mixed_content,
            displayed_mixed_content: self.displayed_mixed_content,
            contained_mixed_form: self.contained_mixed_form,
            ran_content_with_cert_errors: self.ran_content_with_cert_errors,
            displayed_content_with_cert_errors: self.displayed_content_with_cert_errors,
            ran_insecure_content_style: self.ran_insecure_content_style,
            displayed_insecure_content_style: self.displayed_insecure_content_style,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Security.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Security.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Security.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Security.enable";
    type Response = crate::EmptyReturns;
}

/// Enable/disable whether all certificate errors should be ignored.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetIgnoreCertificateErrorsParams {
    /// If true, all certificate errors will be ignored.
    ignore: bool,
}

impl SetIgnoreCertificateErrorsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `ignore`: If true, all certificate errors will be ignored.
    pub fn builder(ignore: bool) -> SetIgnoreCertificateErrorsParamsBuilder {
        SetIgnoreCertificateErrorsParamsBuilder {
            ignore: ignore,
        }
    }
    /// If true, all certificate errors will be ignored.
    pub fn ignore(&self) -> bool { self.ignore }
}


pub struct SetIgnoreCertificateErrorsParamsBuilder {
    ignore: bool,
}

impl SetIgnoreCertificateErrorsParamsBuilder {
    pub fn build(self) -> SetIgnoreCertificateErrorsParams {
        SetIgnoreCertificateErrorsParams {
            ignore: self.ignore,
        }
    }
}

impl SetIgnoreCertificateErrorsParams { pub const METHOD: &'static str = "Security.setIgnoreCertificateErrors"; }

impl<'a> crate::CdpCommand<'a> for SetIgnoreCertificateErrorsParams {
    const METHOD: &'static str = "Security.setIgnoreCertificateErrors";
    type Response = crate::EmptyReturns;
}

/// Handles a certificate error that fired a certificateError event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HandleCertificateErrorParams {
    /// The ID of the event.
    #[serde(rename = "eventId")]
    event_id: u64,
    /// The action to take on the certificate error.
    action: CertificateErrorAction,
}

impl HandleCertificateErrorParams {
    /// Creates a builder for this type with the required parameters:
    /// * `event_id`: The ID of the event.
    /// * `action`: The action to take on the certificate error.
    pub fn builder(event_id: u64, action: impl Into<CertificateErrorAction>) -> HandleCertificateErrorParamsBuilder {
        HandleCertificateErrorParamsBuilder {
            event_id: event_id,
            action: action.into(),
        }
    }
    /// The ID of the event.
    pub fn event_id(&self) -> u64 { self.event_id }
    /// The action to take on the certificate error.
    pub fn action(&self) -> &CertificateErrorAction { &self.action }
}


pub struct HandleCertificateErrorParamsBuilder {
    event_id: u64,
    action: CertificateErrorAction,
}

impl HandleCertificateErrorParamsBuilder {
    pub fn build(self) -> HandleCertificateErrorParams {
        HandleCertificateErrorParams {
            event_id: self.event_id,
            action: self.action,
        }
    }
}

impl HandleCertificateErrorParams { pub const METHOD: &'static str = "Security.handleCertificateError"; }

impl<'a> crate::CdpCommand<'a> for HandleCertificateErrorParams {
    const METHOD: &'static str = "Security.handleCertificateError";
    type Response = crate::EmptyReturns;
}

/// Enable/disable overriding certificate errors. If enabled, all certificate error events need to
/// be handled by the DevTools client and should be answered with 'handleCertificateError' commands.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetOverrideCertificateErrorsParams {
    /// If true, certificate errors will be overridden.
    #[serde(rename = "override")]
    override_: bool,
}

impl SetOverrideCertificateErrorsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `override_`: If true, certificate errors will be overridden.
    pub fn builder(override_: bool) -> SetOverrideCertificateErrorsParamsBuilder {
        SetOverrideCertificateErrorsParamsBuilder {
            override_: override_,
        }
    }
    /// If true, certificate errors will be overridden.
    pub fn override_(&self) -> bool { self.override_ }
}


pub struct SetOverrideCertificateErrorsParamsBuilder {
    override_: bool,
}

impl SetOverrideCertificateErrorsParamsBuilder {
    pub fn build(self) -> SetOverrideCertificateErrorsParams {
        SetOverrideCertificateErrorsParams {
            override_: self.override_,
        }
    }
}

impl SetOverrideCertificateErrorsParams { pub const METHOD: &'static str = "Security.setOverrideCertificateErrors"; }

impl<'a> crate::CdpCommand<'a> for SetOverrideCertificateErrorsParams {
    const METHOD: &'static str = "Security.setOverrideCertificateErrors";
    type Response = crate::EmptyReturns;
}
