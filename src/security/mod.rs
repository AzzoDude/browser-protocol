use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// An internal certificate ID value.

pub type CertificateId = i64;

/// A description of mixed content (HTTP resources on HTTPS pages), as defined by
/// https://www.w3.org/TR/mixed-content/#categories

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
    keyExchange: Cow<'a, str>,
    /// (EC)DH group used by the connection, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    keyExchangeGroup: Option<Cow<'a, str>>,
    /// Cipher name.
    cipher: Cow<'a, str>,
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.
    #[serde(skip_serializing_if = "Option::is_none")]
    mac: Option<Cow<'a, str>>,
    /// Page certificate.
    certificate: Vec<Cow<'a, str>>,
    /// Certificate subject name.
    subjectName: Cow<'a, str>,
    /// Name of the issuing CA.
    issuer: Cow<'a, str>,
    /// Certificate valid from date.
    validFrom: crate::network::TimeSinceEpoch,
    /// Certificate valid to (expiration) date
    validTo: crate::network::TimeSinceEpoch,
    /// The highest priority network error code, if the certificate has an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    certificateNetworkError: Option<Cow<'a, str>>,
    /// True if the certificate uses a weak signature algorithm.
    certificateHasWeakSignature: bool,
    /// True if the certificate has a SHA1 signature in the chain.
    certificateHasSha1Signature: bool,
    /// True if modern SSL
    modernSSL: bool,
    /// True if the connection is using an obsolete SSL protocol.
    obsoleteSslProtocol: bool,
    /// True if the connection is using an obsolete SSL key exchange.
    obsoleteSslKeyExchange: bool,
    /// True if the connection is using an obsolete SSL cipher.
    obsoleteSslCipher: bool,
    /// True if the connection is using an obsolete SSL signature.
    obsoleteSslSignature: bool,
}

impl<'a> CertificateSecurityState<'a> {
    pub fn builder(protocol: impl Into<Cow<'a, str>>, keyExchange: impl Into<Cow<'a, str>>, cipher: impl Into<Cow<'a, str>>, certificate: Vec<Cow<'a, str>>, subjectName: impl Into<Cow<'a, str>>, issuer: impl Into<Cow<'a, str>>, validFrom: crate::network::TimeSinceEpoch, validTo: crate::network::TimeSinceEpoch, certificateHasWeakSignature: bool, certificateHasSha1Signature: bool, modernSSL: bool, obsoleteSslProtocol: bool, obsoleteSslKeyExchange: bool, obsoleteSslCipher: bool, obsoleteSslSignature: bool) -> CertificateSecurityStateBuilder<'a> {
        CertificateSecurityStateBuilder {
            protocol: protocol.into(),
            keyExchange: keyExchange.into(),
            keyExchangeGroup: None,
            cipher: cipher.into(),
            mac: None,
            certificate: certificate,
            subjectName: subjectName.into(),
            issuer: issuer.into(),
            validFrom: validFrom,
            validTo: validTo,
            certificateNetworkError: None,
            certificateHasWeakSignature: certificateHasWeakSignature,
            certificateHasSha1Signature: certificateHasSha1Signature,
            modernSSL: modernSSL,
            obsoleteSslProtocol: obsoleteSslProtocol,
            obsoleteSslKeyExchange: obsoleteSslKeyExchange,
            obsoleteSslCipher: obsoleteSslCipher,
            obsoleteSslSignature: obsoleteSslSignature,
        }
    }
    pub fn protocol(&self) -> &str { self.protocol.as_ref() }
    pub fn keyExchange(&self) -> &str { self.keyExchange.as_ref() }
    pub fn keyExchangeGroup(&self) -> Option<&str> { self.keyExchangeGroup.as_deref() }
    pub fn cipher(&self) -> &str { self.cipher.as_ref() }
    pub fn mac(&self) -> Option<&str> { self.mac.as_deref() }
    pub fn certificate(&self) -> &[Cow<'a, str>] { &self.certificate }
    pub fn subjectName(&self) -> &str { self.subjectName.as_ref() }
    pub fn issuer(&self) -> &str { self.issuer.as_ref() }
    pub fn validFrom(&self) -> &crate::network::TimeSinceEpoch { &self.validFrom }
    pub fn validTo(&self) -> &crate::network::TimeSinceEpoch { &self.validTo }
    pub fn certificateNetworkError(&self) -> Option<&str> { self.certificateNetworkError.as_deref() }
    pub fn certificateHasWeakSignature(&self) -> bool { self.certificateHasWeakSignature }
    pub fn certificateHasSha1Signature(&self) -> bool { self.certificateHasSha1Signature }
    pub fn modernSSL(&self) -> bool { self.modernSSL }
    pub fn obsoleteSslProtocol(&self) -> bool { self.obsoleteSslProtocol }
    pub fn obsoleteSslKeyExchange(&self) -> bool { self.obsoleteSslKeyExchange }
    pub fn obsoleteSslCipher(&self) -> bool { self.obsoleteSslCipher }
    pub fn obsoleteSslSignature(&self) -> bool { self.obsoleteSslSignature }
}


pub struct CertificateSecurityStateBuilder<'a> {
    protocol: Cow<'a, str>,
    keyExchange: Cow<'a, str>,
    keyExchangeGroup: Option<Cow<'a, str>>,
    cipher: Cow<'a, str>,
    mac: Option<Cow<'a, str>>,
    certificate: Vec<Cow<'a, str>>,
    subjectName: Cow<'a, str>,
    issuer: Cow<'a, str>,
    validFrom: crate::network::TimeSinceEpoch,
    validTo: crate::network::TimeSinceEpoch,
    certificateNetworkError: Option<Cow<'a, str>>,
    certificateHasWeakSignature: bool,
    certificateHasSha1Signature: bool,
    modernSSL: bool,
    obsoleteSslProtocol: bool,
    obsoleteSslKeyExchange: bool,
    obsoleteSslCipher: bool,
    obsoleteSslSignature: bool,
}

impl<'a> CertificateSecurityStateBuilder<'a> {
    /// (EC)DH group used by the connection, if applicable.
    pub fn keyExchangeGroup(mut self, keyExchangeGroup: impl Into<Cow<'a, str>>) -> Self { self.keyExchangeGroup = Some(keyExchangeGroup.into()); self }
    /// TLS MAC. Note that AEAD ciphers do not have separate MACs.
    pub fn mac(mut self, mac: impl Into<Cow<'a, str>>) -> Self { self.mac = Some(mac.into()); self }
    /// The highest priority network error code, if the certificate has an error.
    pub fn certificateNetworkError(mut self, certificateNetworkError: impl Into<Cow<'a, str>>) -> Self { self.certificateNetworkError = Some(certificateNetworkError.into()); self }
    pub fn build(self) -> CertificateSecurityState<'a> {
        CertificateSecurityState {
            protocol: self.protocol,
            keyExchange: self.keyExchange,
            keyExchangeGroup: self.keyExchangeGroup,
            cipher: self.cipher,
            mac: self.mac,
            certificate: self.certificate,
            subjectName: self.subjectName,
            issuer: self.issuer,
            validFrom: self.validFrom,
            validTo: self.validTo,
            certificateNetworkError: self.certificateNetworkError,
            certificateHasWeakSignature: self.certificateHasWeakSignature,
            certificateHasSha1Signature: self.certificateHasSha1Signature,
            modernSSL: self.modernSSL,
            obsoleteSslProtocol: self.obsoleteSslProtocol,
            obsoleteSslKeyExchange: self.obsoleteSslKeyExchange,
            obsoleteSslCipher: self.obsoleteSslCipher,
            obsoleteSslSignature: self.obsoleteSslSignature,
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
    safetyTipStatus: SafetyTipStatus,
    /// The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    safeUrl: Option<Cow<'a, str>>,
}

impl<'a> SafetyTipInfo<'a> {
    pub fn builder(safetyTipStatus: SafetyTipStatus) -> SafetyTipInfoBuilder<'a> {
        SafetyTipInfoBuilder {
            safetyTipStatus: safetyTipStatus,
            safeUrl: None,
        }
    }
    pub fn safetyTipStatus(&self) -> &SafetyTipStatus { &self.safetyTipStatus }
    pub fn safeUrl(&self) -> Option<&str> { self.safeUrl.as_deref() }
}


pub struct SafetyTipInfoBuilder<'a> {
    safetyTipStatus: SafetyTipStatus,
    safeUrl: Option<Cow<'a, str>>,
}

impl<'a> SafetyTipInfoBuilder<'a> {
    /// The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.
    pub fn safeUrl(mut self, safeUrl: impl Into<Cow<'a, str>>) -> Self { self.safeUrl = Some(safeUrl.into()); self }
    pub fn build(self) -> SafetyTipInfo<'a> {
        SafetyTipInfo {
            safetyTipStatus: self.safetyTipStatus,
            safeUrl: self.safeUrl,
        }
    }
}

/// Security state information about the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VisibleSecurityState<'a> {
    /// The security level of the page.
    securityState: SecurityState,
    /// Security state details about the page certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    certificateSecurityState: Option<CertificateSecurityState<'a>>,
    /// The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    safetyTipInfo: Option<SafetyTipInfo<'a>>,
    /// Array of security state issues ids.
    securityStateIssueIds: Vec<Cow<'a, str>>,
}

impl<'a> VisibleSecurityState<'a> {
    pub fn builder(securityState: SecurityState, securityStateIssueIds: Vec<Cow<'a, str>>) -> VisibleSecurityStateBuilder<'a> {
        VisibleSecurityStateBuilder {
            securityState: securityState,
            certificateSecurityState: None,
            safetyTipInfo: None,
            securityStateIssueIds: securityStateIssueIds,
        }
    }
    pub fn securityState(&self) -> &SecurityState { &self.securityState }
    pub fn certificateSecurityState(&self) -> Option<&CertificateSecurityState<'a>> { self.certificateSecurityState.as_ref() }
    pub fn safetyTipInfo(&self) -> Option<&SafetyTipInfo<'a>> { self.safetyTipInfo.as_ref() }
    pub fn securityStateIssueIds(&self) -> &[Cow<'a, str>] { &self.securityStateIssueIds }
}


pub struct VisibleSecurityStateBuilder<'a> {
    securityState: SecurityState,
    certificateSecurityState: Option<CertificateSecurityState<'a>>,
    safetyTipInfo: Option<SafetyTipInfo<'a>>,
    securityStateIssueIds: Vec<Cow<'a, str>>,
}

impl<'a> VisibleSecurityStateBuilder<'a> {
    /// Security state details about the page certificate.
    pub fn certificateSecurityState(mut self, certificateSecurityState: CertificateSecurityState<'a>) -> Self { self.certificateSecurityState = Some(certificateSecurityState); self }
    /// The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.
    pub fn safetyTipInfo(mut self, safetyTipInfo: SafetyTipInfo<'a>) -> Self { self.safetyTipInfo = Some(safetyTipInfo); self }
    pub fn build(self) -> VisibleSecurityState<'a> {
        VisibleSecurityState {
            securityState: self.securityState,
            certificateSecurityState: self.certificateSecurityState,
            safetyTipInfo: self.safetyTipInfo,
            securityStateIssueIds: self.securityStateIssueIds,
        }
    }
}

/// An explanation of an factor contributing to the security state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStateExplanation<'a> {
    /// Security state representing the severity of the factor being explained.
    securityState: SecurityState,
    /// Title describing the type of factor.
    title: Cow<'a, str>,
    /// Short phrase describing the type of factor.
    summary: Cow<'a, str>,
    /// Full text explanation of the factor.
    description: Cow<'a, str>,
    /// The type of mixed content described by the explanation.
    mixedContentType: MixedContentType,
    /// Page certificate.
    certificate: Vec<Cow<'a, str>>,
    /// Recommendations to fix any issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    recommendations: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SecurityStateExplanation<'a> {
    pub fn builder(securityState: SecurityState, title: impl Into<Cow<'a, str>>, summary: impl Into<Cow<'a, str>>, description: impl Into<Cow<'a, str>>, mixedContentType: MixedContentType, certificate: Vec<Cow<'a, str>>) -> SecurityStateExplanationBuilder<'a> {
        SecurityStateExplanationBuilder {
            securityState: securityState,
            title: title.into(),
            summary: summary.into(),
            description: description.into(),
            mixedContentType: mixedContentType,
            certificate: certificate,
            recommendations: None,
        }
    }
    pub fn securityState(&self) -> &SecurityState { &self.securityState }
    pub fn title(&self) -> &str { self.title.as_ref() }
    pub fn summary(&self) -> &str { self.summary.as_ref() }
    pub fn description(&self) -> &str { self.description.as_ref() }
    pub fn mixedContentType(&self) -> &MixedContentType { &self.mixedContentType }
    pub fn certificate(&self) -> &[Cow<'a, str>] { &self.certificate }
    pub fn recommendations(&self) -> Option<&[Cow<'a, str>]> { self.recommendations.as_deref() }
}


pub struct SecurityStateExplanationBuilder<'a> {
    securityState: SecurityState,
    title: Cow<'a, str>,
    summary: Cow<'a, str>,
    description: Cow<'a, str>,
    mixedContentType: MixedContentType,
    certificate: Vec<Cow<'a, str>>,
    recommendations: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SecurityStateExplanationBuilder<'a> {
    /// Recommendations to fix any issues.
    pub fn recommendations(mut self, recommendations: Vec<Cow<'a, str>>) -> Self { self.recommendations = Some(recommendations); self }
    pub fn build(self) -> SecurityStateExplanation<'a> {
        SecurityStateExplanation {
            securityState: self.securityState,
            title: self.title,
            summary: self.summary,
            description: self.description,
            mixedContentType: self.mixedContentType,
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
    ranMixedContent: bool,
    /// Always false.
    displayedMixedContent: bool,
    /// Always false.
    containedMixedForm: bool,
    /// Always false.
    ranContentWithCertErrors: bool,
    /// Always false.
    displayedContentWithCertErrors: bool,
    /// Always set to unknown.
    ranInsecureContentStyle: SecurityState,
    /// Always set to unknown.
    displayedInsecureContentStyle: SecurityState,
}

impl InsecureContentStatus {
    pub fn builder(ranMixedContent: bool, displayedMixedContent: bool, containedMixedForm: bool, ranContentWithCertErrors: bool, displayedContentWithCertErrors: bool, ranInsecureContentStyle: SecurityState, displayedInsecureContentStyle: SecurityState) -> InsecureContentStatusBuilder {
        InsecureContentStatusBuilder {
            ranMixedContent: ranMixedContent,
            displayedMixedContent: displayedMixedContent,
            containedMixedForm: containedMixedForm,
            ranContentWithCertErrors: ranContentWithCertErrors,
            displayedContentWithCertErrors: displayedContentWithCertErrors,
            ranInsecureContentStyle: ranInsecureContentStyle,
            displayedInsecureContentStyle: displayedInsecureContentStyle,
        }
    }
    pub fn ranMixedContent(&self) -> bool { self.ranMixedContent }
    pub fn displayedMixedContent(&self) -> bool { self.displayedMixedContent }
    pub fn containedMixedForm(&self) -> bool { self.containedMixedForm }
    pub fn ranContentWithCertErrors(&self) -> bool { self.ranContentWithCertErrors }
    pub fn displayedContentWithCertErrors(&self) -> bool { self.displayedContentWithCertErrors }
    pub fn ranInsecureContentStyle(&self) -> &SecurityState { &self.ranInsecureContentStyle }
    pub fn displayedInsecureContentStyle(&self) -> &SecurityState { &self.displayedInsecureContentStyle }
}


pub struct InsecureContentStatusBuilder {
    ranMixedContent: bool,
    displayedMixedContent: bool,
    containedMixedForm: bool,
    ranContentWithCertErrors: bool,
    displayedContentWithCertErrors: bool,
    ranInsecureContentStyle: SecurityState,
    displayedInsecureContentStyle: SecurityState,
}

impl InsecureContentStatusBuilder {
    pub fn build(self) -> InsecureContentStatus {
        InsecureContentStatus {
            ranMixedContent: self.ranMixedContent,
            displayedMixedContent: self.displayedMixedContent,
            containedMixedForm: self.containedMixedForm,
            ranContentWithCertErrors: self.ranContentWithCertErrors,
            displayedContentWithCertErrors: self.displayedContentWithCertErrors,
            ranInsecureContentStyle: self.ranInsecureContentStyle,
            displayedInsecureContentStyle: self.displayedInsecureContentStyle,
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
    pub fn builder(ignore: bool) -> SetIgnoreCertificateErrorsParamsBuilder {
        SetIgnoreCertificateErrorsParamsBuilder {
            ignore: ignore,
        }
    }
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
    eventId: u64,
    /// The action to take on the certificate error.
    action: CertificateErrorAction,
}

impl HandleCertificateErrorParams {
    pub fn builder(eventId: u64, action: CertificateErrorAction) -> HandleCertificateErrorParamsBuilder {
        HandleCertificateErrorParamsBuilder {
            eventId: eventId,
            action: action,
        }
    }
    pub fn eventId(&self) -> u64 { self.eventId }
    pub fn action(&self) -> &CertificateErrorAction { &self.action }
}


pub struct HandleCertificateErrorParamsBuilder {
    eventId: u64,
    action: CertificateErrorAction,
}

impl HandleCertificateErrorParamsBuilder {
    pub fn build(self) -> HandleCertificateErrorParams {
        HandleCertificateErrorParams {
            eventId: self.eventId,
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
    pub fn builder(override_: bool) -> SetOverrideCertificateErrorsParamsBuilder {
        SetOverrideCertificateErrorsParamsBuilder {
            override_: override_,
        }
    }
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
