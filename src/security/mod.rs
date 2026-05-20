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
    pub fn builder() -> CertificateSecurityStateBuilder<'a> { CertificateSecurityStateBuilder::default() }
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

#[derive(Default)]
pub struct CertificateSecurityStateBuilder<'a> {
    protocol: Option<Cow<'a, str>>,
    keyExchange: Option<Cow<'a, str>>,
    keyExchangeGroup: Option<Cow<'a, str>>,
    cipher: Option<Cow<'a, str>>,
    mac: Option<Cow<'a, str>>,
    certificate: Option<Vec<Cow<'a, str>>>,
    subjectName: Option<Cow<'a, str>>,
    issuer: Option<Cow<'a, str>>,
    validFrom: Option<crate::network::TimeSinceEpoch>,
    validTo: Option<crate::network::TimeSinceEpoch>,
    certificateNetworkError: Option<Cow<'a, str>>,
    certificateHasWeakSignature: Option<bool>,
    certificateHasSha1Signature: Option<bool>,
    modernSSL: Option<bool>,
    obsoleteSslProtocol: Option<bool>,
    obsoleteSslKeyExchange: Option<bool>,
    obsoleteSslCipher: Option<bool>,
    obsoleteSslSignature: Option<bool>,
}

impl<'a> CertificateSecurityStateBuilder<'a> {
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
    /// Page certificate.
    pub fn certificate(mut self, certificate: Vec<Cow<'a, str>>) -> Self { self.certificate = Some(certificate); self }
    /// Certificate subject name.
    pub fn subjectName(mut self, subjectName: impl Into<Cow<'a, str>>) -> Self { self.subjectName = Some(subjectName.into()); self }
    /// Name of the issuing CA.
    pub fn issuer(mut self, issuer: impl Into<Cow<'a, str>>) -> Self { self.issuer = Some(issuer.into()); self }
    /// Certificate valid from date.
    pub fn validFrom(mut self, validFrom: crate::network::TimeSinceEpoch) -> Self { self.validFrom = Some(validFrom); self }
    /// Certificate valid to (expiration) date
    pub fn validTo(mut self, validTo: crate::network::TimeSinceEpoch) -> Self { self.validTo = Some(validTo); self }
    /// The highest priority network error code, if the certificate has an error.
    pub fn certificateNetworkError(mut self, certificateNetworkError: impl Into<Cow<'a, str>>) -> Self { self.certificateNetworkError = Some(certificateNetworkError.into()); self }
    /// True if the certificate uses a weak signature algorithm.
    pub fn certificateHasWeakSignature(mut self, certificateHasWeakSignature: bool) -> Self { self.certificateHasWeakSignature = Some(certificateHasWeakSignature); self }
    /// True if the certificate has a SHA1 signature in the chain.
    pub fn certificateHasSha1Signature(mut self, certificateHasSha1Signature: bool) -> Self { self.certificateHasSha1Signature = Some(certificateHasSha1Signature); self }
    /// True if modern SSL
    pub fn modernSSL(mut self, modernSSL: bool) -> Self { self.modernSSL = Some(modernSSL); self }
    /// True if the connection is using an obsolete SSL protocol.
    pub fn obsoleteSslProtocol(mut self, obsoleteSslProtocol: bool) -> Self { self.obsoleteSslProtocol = Some(obsoleteSslProtocol); self }
    /// True if the connection is using an obsolete SSL key exchange.
    pub fn obsoleteSslKeyExchange(mut self, obsoleteSslKeyExchange: bool) -> Self { self.obsoleteSslKeyExchange = Some(obsoleteSslKeyExchange); self }
    /// True if the connection is using an obsolete SSL cipher.
    pub fn obsoleteSslCipher(mut self, obsoleteSslCipher: bool) -> Self { self.obsoleteSslCipher = Some(obsoleteSslCipher); self }
    /// True if the connection is using an obsolete SSL signature.
    pub fn obsoleteSslSignature(mut self, obsoleteSslSignature: bool) -> Self { self.obsoleteSslSignature = Some(obsoleteSslSignature); self }
    pub fn build(self) -> CertificateSecurityState<'a> {
        CertificateSecurityState {
            protocol: self.protocol.unwrap_or_default(),
            keyExchange: self.keyExchange.unwrap_or_default(),
            keyExchangeGroup: self.keyExchangeGroup,
            cipher: self.cipher.unwrap_or_default(),
            mac: self.mac,
            certificate: self.certificate.unwrap_or_default(),
            subjectName: self.subjectName.unwrap_or_default(),
            issuer: self.issuer.unwrap_or_default(),
            validFrom: self.validFrom.unwrap_or_default(),
            validTo: self.validTo.unwrap_or_default(),
            certificateNetworkError: self.certificateNetworkError,
            certificateHasWeakSignature: self.certificateHasWeakSignature.unwrap_or_default(),
            certificateHasSha1Signature: self.certificateHasSha1Signature.unwrap_or_default(),
            modernSSL: self.modernSSL.unwrap_or_default(),
            obsoleteSslProtocol: self.obsoleteSslProtocol.unwrap_or_default(),
            obsoleteSslKeyExchange: self.obsoleteSslKeyExchange.unwrap_or_default(),
            obsoleteSslCipher: self.obsoleteSslCipher.unwrap_or_default(),
            obsoleteSslSignature: self.obsoleteSslSignature.unwrap_or_default(),
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
    pub fn builder() -> SafetyTipInfoBuilder<'a> { SafetyTipInfoBuilder::default() }
    pub fn safetyTipStatus(&self) -> &SafetyTipStatus { &self.safetyTipStatus }
    pub fn safeUrl(&self) -> Option<&str> { self.safeUrl.as_deref() }
}

#[derive(Default)]
pub struct SafetyTipInfoBuilder<'a> {
    safetyTipStatus: Option<SafetyTipStatus>,
    safeUrl: Option<Cow<'a, str>>,
}

impl<'a> SafetyTipInfoBuilder<'a> {
    /// Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.
    pub fn safetyTipStatus(mut self, safetyTipStatus: SafetyTipStatus) -> Self { self.safetyTipStatus = Some(safetyTipStatus); self }
    /// The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.
    pub fn safeUrl(mut self, safeUrl: impl Into<Cow<'a, str>>) -> Self { self.safeUrl = Some(safeUrl.into()); self }
    pub fn build(self) -> SafetyTipInfo<'a> {
        SafetyTipInfo {
            safetyTipStatus: self.safetyTipStatus.unwrap_or_default(),
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
    pub fn builder() -> VisibleSecurityStateBuilder<'a> { VisibleSecurityStateBuilder::default() }
    pub fn securityState(&self) -> &SecurityState { &self.securityState }
    pub fn certificateSecurityState(&self) -> Option<&CertificateSecurityState<'a>> { self.certificateSecurityState.as_ref() }
    pub fn safetyTipInfo(&self) -> Option<&SafetyTipInfo<'a>> { self.safetyTipInfo.as_ref() }
    pub fn securityStateIssueIds(&self) -> &[Cow<'a, str>] { &self.securityStateIssueIds }
}

#[derive(Default)]
pub struct VisibleSecurityStateBuilder<'a> {
    securityState: Option<SecurityState>,
    certificateSecurityState: Option<CertificateSecurityState<'a>>,
    safetyTipInfo: Option<SafetyTipInfo<'a>>,
    securityStateIssueIds: Option<Vec<Cow<'a, str>>>,
}

impl<'a> VisibleSecurityStateBuilder<'a> {
    /// The security level of the page.
    pub fn securityState(mut self, securityState: SecurityState) -> Self { self.securityState = Some(securityState); self }
    /// Security state details about the page certificate.
    pub fn certificateSecurityState(mut self, certificateSecurityState: CertificateSecurityState<'a>) -> Self { self.certificateSecurityState = Some(certificateSecurityState); self }
    /// The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.
    pub fn safetyTipInfo(mut self, safetyTipInfo: SafetyTipInfo<'a>) -> Self { self.safetyTipInfo = Some(safetyTipInfo); self }
    /// Array of security state issues ids.
    pub fn securityStateIssueIds(mut self, securityStateIssueIds: Vec<Cow<'a, str>>) -> Self { self.securityStateIssueIds = Some(securityStateIssueIds); self }
    pub fn build(self) -> VisibleSecurityState<'a> {
        VisibleSecurityState {
            securityState: self.securityState.unwrap_or_default(),
            certificateSecurityState: self.certificateSecurityState,
            safetyTipInfo: self.safetyTipInfo,
            securityStateIssueIds: self.securityStateIssueIds.unwrap_or_default(),
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
    pub fn builder() -> SecurityStateExplanationBuilder<'a> { SecurityStateExplanationBuilder::default() }
    pub fn securityState(&self) -> &SecurityState { &self.securityState }
    pub fn title(&self) -> &str { self.title.as_ref() }
    pub fn summary(&self) -> &str { self.summary.as_ref() }
    pub fn description(&self) -> &str { self.description.as_ref() }
    pub fn mixedContentType(&self) -> &MixedContentType { &self.mixedContentType }
    pub fn certificate(&self) -> &[Cow<'a, str>] { &self.certificate }
    pub fn recommendations(&self) -> Option<&[Cow<'a, str>]> { self.recommendations.as_deref() }
}

#[derive(Default)]
pub struct SecurityStateExplanationBuilder<'a> {
    securityState: Option<SecurityState>,
    title: Option<Cow<'a, str>>,
    summary: Option<Cow<'a, str>>,
    description: Option<Cow<'a, str>>,
    mixedContentType: Option<MixedContentType>,
    certificate: Option<Vec<Cow<'a, str>>>,
    recommendations: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SecurityStateExplanationBuilder<'a> {
    /// Security state representing the severity of the factor being explained.
    pub fn securityState(mut self, securityState: SecurityState) -> Self { self.securityState = Some(securityState); self }
    /// Title describing the type of factor.
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self { self.title = Some(title.into()); self }
    /// Short phrase describing the type of factor.
    pub fn summary(mut self, summary: impl Into<Cow<'a, str>>) -> Self { self.summary = Some(summary.into()); self }
    /// Full text explanation of the factor.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self { self.description = Some(description.into()); self }
    /// The type of mixed content described by the explanation.
    pub fn mixedContentType(mut self, mixedContentType: MixedContentType) -> Self { self.mixedContentType = Some(mixedContentType); self }
    /// Page certificate.
    pub fn certificate(mut self, certificate: Vec<Cow<'a, str>>) -> Self { self.certificate = Some(certificate); self }
    /// Recommendations to fix any issues.
    pub fn recommendations(mut self, recommendations: Vec<Cow<'a, str>>) -> Self { self.recommendations = Some(recommendations); self }
    pub fn build(self) -> SecurityStateExplanation<'a> {
        SecurityStateExplanation {
            securityState: self.securityState.unwrap_or_default(),
            title: self.title.unwrap_or_default(),
            summary: self.summary.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            mixedContentType: self.mixedContentType.unwrap_or_default(),
            certificate: self.certificate.unwrap_or_default(),
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
    pub fn builder() -> InsecureContentStatusBuilder { InsecureContentStatusBuilder::default() }
    pub fn ranMixedContent(&self) -> bool { self.ranMixedContent }
    pub fn displayedMixedContent(&self) -> bool { self.displayedMixedContent }
    pub fn containedMixedForm(&self) -> bool { self.containedMixedForm }
    pub fn ranContentWithCertErrors(&self) -> bool { self.ranContentWithCertErrors }
    pub fn displayedContentWithCertErrors(&self) -> bool { self.displayedContentWithCertErrors }
    pub fn ranInsecureContentStyle(&self) -> &SecurityState { &self.ranInsecureContentStyle }
    pub fn displayedInsecureContentStyle(&self) -> &SecurityState { &self.displayedInsecureContentStyle }
}

#[derive(Default)]
pub struct InsecureContentStatusBuilder {
    ranMixedContent: Option<bool>,
    displayedMixedContent: Option<bool>,
    containedMixedForm: Option<bool>,
    ranContentWithCertErrors: Option<bool>,
    displayedContentWithCertErrors: Option<bool>,
    ranInsecureContentStyle: Option<SecurityState>,
    displayedInsecureContentStyle: Option<SecurityState>,
}

impl InsecureContentStatusBuilder {
    /// Always false.
    pub fn ranMixedContent(mut self, ranMixedContent: bool) -> Self { self.ranMixedContent = Some(ranMixedContent); self }
    /// Always false.
    pub fn displayedMixedContent(mut self, displayedMixedContent: bool) -> Self { self.displayedMixedContent = Some(displayedMixedContent); self }
    /// Always false.
    pub fn containedMixedForm(mut self, containedMixedForm: bool) -> Self { self.containedMixedForm = Some(containedMixedForm); self }
    /// Always false.
    pub fn ranContentWithCertErrors(mut self, ranContentWithCertErrors: bool) -> Self { self.ranContentWithCertErrors = Some(ranContentWithCertErrors); self }
    /// Always false.
    pub fn displayedContentWithCertErrors(mut self, displayedContentWithCertErrors: bool) -> Self { self.displayedContentWithCertErrors = Some(displayedContentWithCertErrors); self }
    /// Always set to unknown.
    pub fn ranInsecureContentStyle(mut self, ranInsecureContentStyle: SecurityState) -> Self { self.ranInsecureContentStyle = Some(ranInsecureContentStyle); self }
    /// Always set to unknown.
    pub fn displayedInsecureContentStyle(mut self, displayedInsecureContentStyle: SecurityState) -> Self { self.displayedInsecureContentStyle = Some(displayedInsecureContentStyle); self }
    pub fn build(self) -> InsecureContentStatus {
        InsecureContentStatus {
            ranMixedContent: self.ranMixedContent.unwrap_or_default(),
            displayedMixedContent: self.displayedMixedContent.unwrap_or_default(),
            containedMixedForm: self.containedMixedForm.unwrap_or_default(),
            ranContentWithCertErrors: self.ranContentWithCertErrors.unwrap_or_default(),
            displayedContentWithCertErrors: self.displayedContentWithCertErrors.unwrap_or_default(),
            ranInsecureContentStyle: self.ranInsecureContentStyle.unwrap_or_default(),
            displayedInsecureContentStyle: self.displayedInsecureContentStyle.unwrap_or_default(),
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

impl DisableParams { pub const METHOD: &'static str = "Security.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Security.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct EnableParamsBuilder {}

impl EnableParamsBuilder {
    pub fn build(self) -> EnableParams {
        EnableParams {}
    }
}

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
    pub fn builder() -> SetIgnoreCertificateErrorsParamsBuilder { SetIgnoreCertificateErrorsParamsBuilder::default() }
    pub fn ignore(&self) -> bool { self.ignore }
}

#[derive(Default)]
pub struct SetIgnoreCertificateErrorsParamsBuilder {
    ignore: Option<bool>,
}

impl SetIgnoreCertificateErrorsParamsBuilder {
    /// If true, all certificate errors will be ignored.
    pub fn ignore(mut self, ignore: bool) -> Self { self.ignore = Some(ignore); self }
    pub fn build(self) -> SetIgnoreCertificateErrorsParams {
        SetIgnoreCertificateErrorsParams {
            ignore: self.ignore.unwrap_or_default(),
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
    pub fn builder() -> HandleCertificateErrorParamsBuilder { HandleCertificateErrorParamsBuilder::default() }
    pub fn eventId(&self) -> u64 { self.eventId }
    pub fn action(&self) -> &CertificateErrorAction { &self.action }
}

#[derive(Default)]
pub struct HandleCertificateErrorParamsBuilder {
    eventId: Option<u64>,
    action: Option<CertificateErrorAction>,
}

impl HandleCertificateErrorParamsBuilder {
    /// The ID of the event.
    pub fn eventId(mut self, eventId: u64) -> Self { self.eventId = Some(eventId); self }
    /// The action to take on the certificate error.
    pub fn action(mut self, action: CertificateErrorAction) -> Self { self.action = Some(action); self }
    pub fn build(self) -> HandleCertificateErrorParams {
        HandleCertificateErrorParams {
            eventId: self.eventId.unwrap_or_default(),
            action: self.action.unwrap_or_default(),
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
    pub fn builder() -> SetOverrideCertificateErrorsParamsBuilder { SetOverrideCertificateErrorsParamsBuilder::default() }
    pub fn override_(&self) -> bool { self.override_ }
}

#[derive(Default)]
pub struct SetOverrideCertificateErrorsParamsBuilder {
    override_: Option<bool>,
}

impl SetOverrideCertificateErrorsParamsBuilder {
    /// If true, certificate errors will be overridden.
    pub fn override_(mut self, override_: bool) -> Self { self.override_ = Some(override_); self }
    pub fn build(self) -> SetOverrideCertificateErrorsParams {
        SetOverrideCertificateErrorsParams {
            override_: self.override_.unwrap_or_default(),
        }
    }
}

impl SetOverrideCertificateErrorsParams { pub const METHOD: &'static str = "Security.setOverrideCertificateErrors"; }

impl<'a> crate::CdpCommand<'a> for SetOverrideCertificateErrorsParams {
    const METHOD: &'static str = "Security.setOverrideCertificateErrors";
    type Response = crate::EmptyReturns;
}
