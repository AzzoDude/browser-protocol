//! This domain allows configuring virtual authenticators to test the WebAuthn
//! API.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


pub type AuthenticatorId<'a> = Cow<'a, str>;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AuthenticatorProtocol {
    #[default]
    #[serde(rename = "u2f")]
    U2f,
    #[serde(rename = "ctap2")]
    Ctap2,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Ctap2Version {
    #[default]
    #[serde(rename = "ctap2_0")]
    Ctap20,
    #[serde(rename = "ctap2_1")]
    Ctap21,
    #[serde(rename = "ctap2_2")]
    Ctap22,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AuthenticatorTransport {
    #[default]
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "ble")]
    Ble,
    #[serde(rename = "cable")]
    Cable,
    #[serde(rename = "internal")]
    Internal,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAuthenticatorOptions {
    protocol: AuthenticatorProtocol,
    /// Defaults to ctap2_0. Ignored if |protocol| == u2f.
    #[serde(skip_serializing_if = "Option::is_none")]
    ctap2Version: Option<Ctap2Version>,
    transport: AuthenticatorTransport,
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasResidentKey: Option<bool>,
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasUserVerification: Option<bool>,
    /// If set to true, the authenticator will support the largeBlob extension.
    /// https://w3c.github.io/webauthn#largeBlob
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasLargeBlob: Option<bool>,
    /// If set to true, the authenticator will support the credBlob extension.
    /// https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasCredBlob: Option<bool>,
    /// If set to true, the authenticator will support the minPinLength extension.
    /// https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasMinPinLength: Option<bool>,
    /// If set to true, the authenticator will support the prf extension.
    /// https://w3c.github.io/webauthn/#prf-extension
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasPrf: Option<bool>,
    /// If set to true, the authenticator will support the hmac-secret extension.
    /// https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasHmacSecret: Option<bool>,
    /// If set to true, the authenticator will support the hmac-secret-mc extension.
    /// https://fidoalliance.org/specs/fido-v2.2-rd-20241003/fido-client-to-authenticator-protocol-v2.2-rd-20241003.html#sctn-hmac-secret-make-cred-extension
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    hasHmacSecretMc: Option<bool>,
    /// If set to true, tests of user presence will succeed immediately.
    /// Otherwise, they will not be resolved. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    automaticPresenceSimulation: Option<bool>,
    /// Sets whether User Verification succeeds or fails for an authenticator.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    isUserVerified: Option<bool>,
    /// Credentials created by this authenticator will have the backup
    /// eligibility (BE) flag set to this value. Defaults to false.
    /// https://w3c.github.io/webauthn/#sctn-credential-backup
    #[serde(skip_serializing_if = "Option::is_none")]
    defaultBackupEligibility: Option<bool>,
    /// Credentials created by this authenticator will have the backup state
    /// (BS) flag set to this value. Defaults to false.
    /// https://w3c.github.io/webauthn/#sctn-credential-backup
    #[serde(skip_serializing_if = "Option::is_none")]
    defaultBackupState: Option<bool>,
}

impl VirtualAuthenticatorOptions {
    pub fn builder() -> VirtualAuthenticatorOptionsBuilder { VirtualAuthenticatorOptionsBuilder::default() }
    pub fn protocol(&self) -> &AuthenticatorProtocol { &self.protocol }
    pub fn ctap2Version(&self) -> Option<&Ctap2Version> { self.ctap2Version.as_ref() }
    pub fn transport(&self) -> &AuthenticatorTransport { &self.transport }
    pub fn hasResidentKey(&self) -> Option<bool> { self.hasResidentKey }
    pub fn hasUserVerification(&self) -> Option<bool> { self.hasUserVerification }
    pub fn hasLargeBlob(&self) -> Option<bool> { self.hasLargeBlob }
    pub fn hasCredBlob(&self) -> Option<bool> { self.hasCredBlob }
    pub fn hasMinPinLength(&self) -> Option<bool> { self.hasMinPinLength }
    pub fn hasPrf(&self) -> Option<bool> { self.hasPrf }
    pub fn hasHmacSecret(&self) -> Option<bool> { self.hasHmacSecret }
    pub fn hasHmacSecretMc(&self) -> Option<bool> { self.hasHmacSecretMc }
    pub fn automaticPresenceSimulation(&self) -> Option<bool> { self.automaticPresenceSimulation }
    pub fn isUserVerified(&self) -> Option<bool> { self.isUserVerified }
    pub fn defaultBackupEligibility(&self) -> Option<bool> { self.defaultBackupEligibility }
    pub fn defaultBackupState(&self) -> Option<bool> { self.defaultBackupState }
}

#[derive(Default)]
pub struct VirtualAuthenticatorOptionsBuilder {
    protocol: Option<AuthenticatorProtocol>,
    ctap2Version: Option<Ctap2Version>,
    transport: Option<AuthenticatorTransport>,
    hasResidentKey: Option<bool>,
    hasUserVerification: Option<bool>,
    hasLargeBlob: Option<bool>,
    hasCredBlob: Option<bool>,
    hasMinPinLength: Option<bool>,
    hasPrf: Option<bool>,
    hasHmacSecret: Option<bool>,
    hasHmacSecretMc: Option<bool>,
    automaticPresenceSimulation: Option<bool>,
    isUserVerified: Option<bool>,
    defaultBackupEligibility: Option<bool>,
    defaultBackupState: Option<bool>,
}

impl VirtualAuthenticatorOptionsBuilder {
    pub fn protocol(mut self, protocol: AuthenticatorProtocol) -> Self { self.protocol = Some(protocol); self }
    /// Defaults to ctap2_0. Ignored if |protocol| == u2f.
    pub fn ctap2Version(mut self, ctap2Version: Ctap2Version) -> Self { self.ctap2Version = Some(ctap2Version); self }
    pub fn transport(mut self, transport: AuthenticatorTransport) -> Self { self.transport = Some(transport); self }
    /// Defaults to false.
    pub fn hasResidentKey(mut self, hasResidentKey: bool) -> Self { self.hasResidentKey = Some(hasResidentKey); self }
    /// Defaults to false.
    pub fn hasUserVerification(mut self, hasUserVerification: bool) -> Self { self.hasUserVerification = Some(hasUserVerification); self }
    /// If set to true, the authenticator will support the largeBlob extension.
    /// https://w3c.github.io/webauthn#largeBlob
    /// Defaults to false.
    pub fn hasLargeBlob(mut self, hasLargeBlob: bool) -> Self { self.hasLargeBlob = Some(hasLargeBlob); self }
    /// If set to true, the authenticator will support the credBlob extension.
    /// https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension
    /// Defaults to false.
    pub fn hasCredBlob(mut self, hasCredBlob: bool) -> Self { self.hasCredBlob = Some(hasCredBlob); self }
    /// If set to true, the authenticator will support the minPinLength extension.
    /// https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension
    /// Defaults to false.
    pub fn hasMinPinLength(mut self, hasMinPinLength: bool) -> Self { self.hasMinPinLength = Some(hasMinPinLength); self }
    /// If set to true, the authenticator will support the prf extension.
    /// https://w3c.github.io/webauthn/#prf-extension
    /// Defaults to false.
    pub fn hasPrf(mut self, hasPrf: bool) -> Self { self.hasPrf = Some(hasPrf); self }
    /// If set to true, the authenticator will support the hmac-secret extension.
    /// https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension
    /// Defaults to false.
    pub fn hasHmacSecret(mut self, hasHmacSecret: bool) -> Self { self.hasHmacSecret = Some(hasHmacSecret); self }
    /// If set to true, the authenticator will support the hmac-secret-mc extension.
    /// https://fidoalliance.org/specs/fido-v2.2-rd-20241003/fido-client-to-authenticator-protocol-v2.2-rd-20241003.html#sctn-hmac-secret-make-cred-extension
    /// Defaults to false.
    pub fn hasHmacSecretMc(mut self, hasHmacSecretMc: bool) -> Self { self.hasHmacSecretMc = Some(hasHmacSecretMc); self }
    /// If set to true, tests of user presence will succeed immediately.
    /// Otherwise, they will not be resolved. Defaults to true.
    pub fn automaticPresenceSimulation(mut self, automaticPresenceSimulation: bool) -> Self { self.automaticPresenceSimulation = Some(automaticPresenceSimulation); self }
    /// Sets whether User Verification succeeds or fails for an authenticator.
    /// Defaults to false.
    pub fn isUserVerified(mut self, isUserVerified: bool) -> Self { self.isUserVerified = Some(isUserVerified); self }
    /// Credentials created by this authenticator will have the backup
    /// eligibility (BE) flag set to this value. Defaults to false.
    /// https://w3c.github.io/webauthn/#sctn-credential-backup
    pub fn defaultBackupEligibility(mut self, defaultBackupEligibility: bool) -> Self { self.defaultBackupEligibility = Some(defaultBackupEligibility); self }
    /// Credentials created by this authenticator will have the backup state
    /// (BS) flag set to this value. Defaults to false.
    /// https://w3c.github.io/webauthn/#sctn-credential-backup
    pub fn defaultBackupState(mut self, defaultBackupState: bool) -> Self { self.defaultBackupState = Some(defaultBackupState); self }
    pub fn build(self) -> VirtualAuthenticatorOptions {
        VirtualAuthenticatorOptions {
            protocol: self.protocol.unwrap_or_default(),
            ctap2Version: self.ctap2Version,
            transport: self.transport.unwrap_or_default(),
            hasResidentKey: self.hasResidentKey,
            hasUserVerification: self.hasUserVerification,
            hasLargeBlob: self.hasLargeBlob,
            hasCredBlob: self.hasCredBlob,
            hasMinPinLength: self.hasMinPinLength,
            hasPrf: self.hasPrf,
            hasHmacSecret: self.hasHmacSecret,
            hasHmacSecretMc: self.hasHmacSecretMc,
            automaticPresenceSimulation: self.automaticPresenceSimulation,
            isUserVerified: self.isUserVerified,
            defaultBackupEligibility: self.defaultBackupEligibility,
            defaultBackupState: self.defaultBackupState,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Credential<'a> {
    credentialId: Cow<'a, str>,
    isResidentCredential: bool,
    /// Relying Party ID the credential is scoped to. Must be set when adding a
    /// credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    rpId: Option<Cow<'a, str>>,
    /// The ECDSA P-256 private key in PKCS#8 format. (Encoded as a base64 string when passed over JSON)
    privateKey: Cow<'a, str>,
    /// An opaque byte sequence with a maximum size of 64 bytes mapping the
    /// credential to a specific user. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    userHandle: Option<Cow<'a, str>>,
    /// Signature counter. This is incremented by one for each successful
    /// assertion.
    /// See https://w3c.github.io/webauthn/#signature-counter
    signCount: u64,
    /// The large blob associated with the credential.
    /// See https://w3c.github.io/webauthn/#sctn-large-blob-extension (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    largeBlob: Option<Cow<'a, str>>,
    /// Assertions returned by this credential will have the backup eligibility
    /// (BE) flag set to this value. Defaults to the authenticator's
    /// defaultBackupEligibility value.
    #[serde(skip_serializing_if = "Option::is_none")]
    backupEligibility: Option<bool>,
    /// Assertions returned by this credential will have the backup state (BS)
    /// flag set to this value. Defaults to the authenticator's
    /// defaultBackupState value.
    #[serde(skip_serializing_if = "Option::is_none")]
    backupState: Option<bool>,
    /// The credential's user.name property. Equivalent to empty if not set.
    /// https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name
    #[serde(skip_serializing_if = "Option::is_none")]
    userName: Option<Cow<'a, str>>,
    /// The credential's user.displayName property. Equivalent to empty if
    /// not set.
    /// https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname
    #[serde(skip_serializing_if = "Option::is_none")]
    userDisplayName: Option<Cow<'a, str>>,
}

impl<'a> Credential<'a> {
    pub fn builder() -> CredentialBuilder<'a> { CredentialBuilder::default() }
    pub fn credentialId(&self) -> &str { self.credentialId.as_ref() }
    pub fn isResidentCredential(&self) -> bool { self.isResidentCredential }
    pub fn rpId(&self) -> Option<&str> { self.rpId.as_deref() }
    pub fn privateKey(&self) -> &str { self.privateKey.as_ref() }
    pub fn userHandle(&self) -> Option<&str> { self.userHandle.as_deref() }
    pub fn signCount(&self) -> u64 { self.signCount }
    pub fn largeBlob(&self) -> Option<&str> { self.largeBlob.as_deref() }
    pub fn backupEligibility(&self) -> Option<bool> { self.backupEligibility }
    pub fn backupState(&self) -> Option<bool> { self.backupState }
    pub fn userName(&self) -> Option<&str> { self.userName.as_deref() }
    pub fn userDisplayName(&self) -> Option<&str> { self.userDisplayName.as_deref() }
}

#[derive(Default)]
pub struct CredentialBuilder<'a> {
    credentialId: Option<Cow<'a, str>>,
    isResidentCredential: Option<bool>,
    rpId: Option<Cow<'a, str>>,
    privateKey: Option<Cow<'a, str>>,
    userHandle: Option<Cow<'a, str>>,
    signCount: Option<u64>,
    largeBlob: Option<Cow<'a, str>>,
    backupEligibility: Option<bool>,
    backupState: Option<bool>,
    userName: Option<Cow<'a, str>>,
    userDisplayName: Option<Cow<'a, str>>,
}

impl<'a> CredentialBuilder<'a> {
    pub fn credentialId(mut self, credentialId: impl Into<Cow<'a, str>>) -> Self { self.credentialId = Some(credentialId.into()); self }
    pub fn isResidentCredential(mut self, isResidentCredential: bool) -> Self { self.isResidentCredential = Some(isResidentCredential); self }
    /// Relying Party ID the credential is scoped to. Must be set when adding a
    /// credential.
    pub fn rpId(mut self, rpId: impl Into<Cow<'a, str>>) -> Self { self.rpId = Some(rpId.into()); self }
    /// The ECDSA P-256 private key in PKCS#8 format. (Encoded as a base64 string when passed over JSON)
    pub fn privateKey(mut self, privateKey: impl Into<Cow<'a, str>>) -> Self { self.privateKey = Some(privateKey.into()); self }
    /// An opaque byte sequence with a maximum size of 64 bytes mapping the
    /// credential to a specific user. (Encoded as a base64 string when passed over JSON)
    pub fn userHandle(mut self, userHandle: impl Into<Cow<'a, str>>) -> Self { self.userHandle = Some(userHandle.into()); self }
    /// Signature counter. This is incremented by one for each successful
    /// assertion.
    /// See https://w3c.github.io/webauthn/#signature-counter
    pub fn signCount(mut self, signCount: u64) -> Self { self.signCount = Some(signCount); self }
    /// The large blob associated with the credential.
    /// See https://w3c.github.io/webauthn/#sctn-large-blob-extension (Encoded as a base64 string when passed over JSON)
    pub fn largeBlob(mut self, largeBlob: impl Into<Cow<'a, str>>) -> Self { self.largeBlob = Some(largeBlob.into()); self }
    /// Assertions returned by this credential will have the backup eligibility
    /// (BE) flag set to this value. Defaults to the authenticator's
    /// defaultBackupEligibility value.
    pub fn backupEligibility(mut self, backupEligibility: bool) -> Self { self.backupEligibility = Some(backupEligibility); self }
    /// Assertions returned by this credential will have the backup state (BS)
    /// flag set to this value. Defaults to the authenticator's
    /// defaultBackupState value.
    pub fn backupState(mut self, backupState: bool) -> Self { self.backupState = Some(backupState); self }
    /// The credential's user.name property. Equivalent to empty if not set.
    /// https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name
    pub fn userName(mut self, userName: impl Into<Cow<'a, str>>) -> Self { self.userName = Some(userName.into()); self }
    /// The credential's user.displayName property. Equivalent to empty if
    /// not set.
    /// https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname
    pub fn userDisplayName(mut self, userDisplayName: impl Into<Cow<'a, str>>) -> Self { self.userDisplayName = Some(userDisplayName.into()); self }
    pub fn build(self) -> Credential<'a> {
        Credential {
            credentialId: self.credentialId.unwrap_or_default(),
            isResidentCredential: self.isResidentCredential.unwrap_or_default(),
            rpId: self.rpId,
            privateKey: self.privateKey.unwrap_or_default(),
            userHandle: self.userHandle,
            signCount: self.signCount.unwrap_or_default(),
            largeBlob: self.largeBlob,
            backupEligibility: self.backupEligibility,
            backupState: self.backupState,
            userName: self.userName,
            userDisplayName: self.userDisplayName,
        }
    }
}

/// Enable the WebAuthn domain and start intercepting credential storage and
/// retrieval with a virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Whether to enable the WebAuthn user interface. Enabling the UI is
    /// recommended for debugging and demo purposes, as it is closer to the real
    /// experience. Disabling the UI is recommended for automated testing.
    /// Supported at the embedder's discretion if UI is available.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    enableUI: Option<bool>,
}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder { EnableParamsBuilder::default() }
    pub fn enableUI(&self) -> Option<bool> { self.enableUI }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    enableUI: Option<bool>,
}

impl EnableParamsBuilder {
    /// Whether to enable the WebAuthn user interface. Enabling the UI is
    /// recommended for debugging and demo purposes, as it is closer to the real
    /// experience. Disabling the UI is recommended for automated testing.
    /// Supported at the embedder's discretion if UI is available.
    /// Defaults to false.
    pub fn enableUI(mut self, enableUI: bool) -> Self { self.enableUI = Some(enableUI); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            enableUI: self.enableUI,
        }
    }
}

impl EnableParams { pub const METHOD: &'static str = "WebAuthn.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "WebAuthn.enable";
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

impl DisableParams { pub const METHOD: &'static str = "WebAuthn.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "WebAuthn.disable";
    type Response = crate::EmptyReturns;
}

/// Creates and adds a virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddVirtualAuthenticatorParams {
    options: VirtualAuthenticatorOptions,
}

impl AddVirtualAuthenticatorParams {
    pub fn builder() -> AddVirtualAuthenticatorParamsBuilder { AddVirtualAuthenticatorParamsBuilder::default() }
    pub fn options(&self) -> &VirtualAuthenticatorOptions { &self.options }
}

#[derive(Default)]
pub struct AddVirtualAuthenticatorParamsBuilder {
    options: Option<VirtualAuthenticatorOptions>,
}

impl AddVirtualAuthenticatorParamsBuilder {
    pub fn options(mut self, options: VirtualAuthenticatorOptions) -> Self { self.options = Some(options); self }
    pub fn build(self) -> AddVirtualAuthenticatorParams {
        AddVirtualAuthenticatorParams {
            options: self.options.unwrap_or_default(),
        }
    }
}

/// Creates and adds a virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddVirtualAuthenticatorReturns<'a> {
    authenticatorId: AuthenticatorId<'a>,
}

impl<'a> AddVirtualAuthenticatorReturns<'a> {
    pub fn builder() -> AddVirtualAuthenticatorReturnsBuilder<'a> { AddVirtualAuthenticatorReturnsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
}

#[derive(Default)]
pub struct AddVirtualAuthenticatorReturnsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
}

impl<'a> AddVirtualAuthenticatorReturnsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn build(self) -> AddVirtualAuthenticatorReturns<'a> {
        AddVirtualAuthenticatorReturns {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
        }
    }
}

impl AddVirtualAuthenticatorParams { pub const METHOD: &'static str = "WebAuthn.addVirtualAuthenticator"; }

impl<'a> crate::CdpCommand<'a> for AddVirtualAuthenticatorParams {
    const METHOD: &'static str = "WebAuthn.addVirtualAuthenticator";
    type Response = AddVirtualAuthenticatorReturns<'a>;
}

/// Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetResponseOverrideBitsParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
    /// If isBogusSignature is set, overrides the signature in the authenticator response to be zero.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    isBogusSignature: Option<bool>,
    /// If isBadUV is set, overrides the UV bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    isBadUV: Option<bool>,
    /// If isBadUP is set, overrides the UP bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    isBadUP: Option<bool>,
}

impl<'a> SetResponseOverrideBitsParams<'a> {
    pub fn builder() -> SetResponseOverrideBitsParamsBuilder<'a> { SetResponseOverrideBitsParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
    pub fn isBogusSignature(&self) -> Option<bool> { self.isBogusSignature }
    pub fn isBadUV(&self) -> Option<bool> { self.isBadUV }
    pub fn isBadUP(&self) -> Option<bool> { self.isBadUP }
}

#[derive(Default)]
pub struct SetResponseOverrideBitsParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
    isBogusSignature: Option<bool>,
    isBadUV: Option<bool>,
    isBadUP: Option<bool>,
}

impl<'a> SetResponseOverrideBitsParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    /// If isBogusSignature is set, overrides the signature in the authenticator response to be zero.
    /// Defaults to false.
    pub fn isBogusSignature(mut self, isBogusSignature: bool) -> Self { self.isBogusSignature = Some(isBogusSignature); self }
    /// If isBadUV is set, overrides the UV bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    pub fn isBadUV(mut self, isBadUV: bool) -> Self { self.isBadUV = Some(isBadUV); self }
    /// If isBadUP is set, overrides the UP bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    pub fn isBadUP(mut self, isBadUP: bool) -> Self { self.isBadUP = Some(isBadUP); self }
    pub fn build(self) -> SetResponseOverrideBitsParams<'a> {
        SetResponseOverrideBitsParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
            isBogusSignature: self.isBogusSignature,
            isBadUV: self.isBadUV,
            isBadUP: self.isBadUP,
        }
    }
}

impl<'a> SetResponseOverrideBitsParams<'a> { pub const METHOD: &'static str = "WebAuthn.setResponseOverrideBits"; }

impl<'a> crate::CdpCommand<'a> for SetResponseOverrideBitsParams<'a> {
    const METHOD: &'static str = "WebAuthn.setResponseOverrideBits";
    type Response = crate::EmptyReturns;
}

/// Removes the given authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveVirtualAuthenticatorParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
}

impl<'a> RemoveVirtualAuthenticatorParams<'a> {
    pub fn builder() -> RemoveVirtualAuthenticatorParamsBuilder<'a> { RemoveVirtualAuthenticatorParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
}

#[derive(Default)]
pub struct RemoveVirtualAuthenticatorParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
}

impl<'a> RemoveVirtualAuthenticatorParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn build(self) -> RemoveVirtualAuthenticatorParams<'a> {
        RemoveVirtualAuthenticatorParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
        }
    }
}

impl<'a> RemoveVirtualAuthenticatorParams<'a> { pub const METHOD: &'static str = "WebAuthn.removeVirtualAuthenticator"; }

impl<'a> crate::CdpCommand<'a> for RemoveVirtualAuthenticatorParams<'a> {
    const METHOD: &'static str = "WebAuthn.removeVirtualAuthenticator";
    type Response = crate::EmptyReturns;
}

/// Adds the credential to the specified authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddCredentialParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
    credential: Credential<'a>,
}

impl<'a> AddCredentialParams<'a> {
    pub fn builder() -> AddCredentialParamsBuilder<'a> { AddCredentialParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
    pub fn credential(&self) -> &Credential<'a> { &self.credential }
}

#[derive(Default)]
pub struct AddCredentialParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
    credential: Option<Credential<'a>>,
}

impl<'a> AddCredentialParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn credential(mut self, credential: Credential<'a>) -> Self { self.credential = Some(credential); self }
    pub fn build(self) -> AddCredentialParams<'a> {
        AddCredentialParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
            credential: self.credential.unwrap_or_default(),
        }
    }
}

impl<'a> AddCredentialParams<'a> { pub const METHOD: &'static str = "WebAuthn.addCredential"; }

impl<'a> crate::CdpCommand<'a> for AddCredentialParams<'a> {
    const METHOD: &'static str = "WebAuthn.addCredential";
    type Response = crate::EmptyReturns;
}

/// Returns a single credential stored in the given virtual authenticator that
/// matches the credential ID.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
    credentialId: Cow<'a, str>,
}

impl<'a> GetCredentialParams<'a> {
    pub fn builder() -> GetCredentialParamsBuilder<'a> { GetCredentialParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
    pub fn credentialId(&self) -> &str { self.credentialId.as_ref() }
}

#[derive(Default)]
pub struct GetCredentialParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
    credentialId: Option<Cow<'a, str>>,
}

impl<'a> GetCredentialParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn credentialId(mut self, credentialId: impl Into<Cow<'a, str>>) -> Self { self.credentialId = Some(credentialId.into()); self }
    pub fn build(self) -> GetCredentialParams<'a> {
        GetCredentialParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
            credentialId: self.credentialId.unwrap_or_default(),
        }
    }
}

/// Returns a single credential stored in the given virtual authenticator that
/// matches the credential ID.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialReturns<'a> {
    credential: Credential<'a>,
}

impl<'a> GetCredentialReturns<'a> {
    pub fn builder() -> GetCredentialReturnsBuilder<'a> { GetCredentialReturnsBuilder::default() }
    pub fn credential(&self) -> &Credential<'a> { &self.credential }
}

#[derive(Default)]
pub struct GetCredentialReturnsBuilder<'a> {
    credential: Option<Credential<'a>>,
}

impl<'a> GetCredentialReturnsBuilder<'a> {
    pub fn credential(mut self, credential: Credential<'a>) -> Self { self.credential = Some(credential); self }
    pub fn build(self) -> GetCredentialReturns<'a> {
        GetCredentialReturns {
            credential: self.credential.unwrap_or_default(),
        }
    }
}

impl<'a> GetCredentialParams<'a> { pub const METHOD: &'static str = "WebAuthn.getCredential"; }

impl<'a> crate::CdpCommand<'a> for GetCredentialParams<'a> {
    const METHOD: &'static str = "WebAuthn.getCredential";
    type Response = GetCredentialReturns<'a>;
}

/// Returns all the credentials stored in the given virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialsParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
}

impl<'a> GetCredentialsParams<'a> {
    pub fn builder() -> GetCredentialsParamsBuilder<'a> { GetCredentialsParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
}

#[derive(Default)]
pub struct GetCredentialsParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
}

impl<'a> GetCredentialsParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn build(self) -> GetCredentialsParams<'a> {
        GetCredentialsParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
        }
    }
}

/// Returns all the credentials stored in the given virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialsReturns<'a> {
    credentials: Vec<Credential<'a>>,
}

impl<'a> GetCredentialsReturns<'a> {
    pub fn builder() -> GetCredentialsReturnsBuilder<'a> { GetCredentialsReturnsBuilder::default() }
    pub fn credentials(&self) -> &[Credential<'a>] { &self.credentials }
}

#[derive(Default)]
pub struct GetCredentialsReturnsBuilder<'a> {
    credentials: Option<Vec<Credential<'a>>>,
}

impl<'a> GetCredentialsReturnsBuilder<'a> {
    pub fn credentials(mut self, credentials: Vec<Credential<'a>>) -> Self { self.credentials = Some(credentials); self }
    pub fn build(self) -> GetCredentialsReturns<'a> {
        GetCredentialsReturns {
            credentials: self.credentials.unwrap_or_default(),
        }
    }
}

impl<'a> GetCredentialsParams<'a> { pub const METHOD: &'static str = "WebAuthn.getCredentials"; }

impl<'a> crate::CdpCommand<'a> for GetCredentialsParams<'a> {
    const METHOD: &'static str = "WebAuthn.getCredentials";
    type Response = GetCredentialsReturns<'a>;
}

/// Removes a credential from the authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCredentialParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
    credentialId: Cow<'a, str>,
}

impl<'a> RemoveCredentialParams<'a> {
    pub fn builder() -> RemoveCredentialParamsBuilder<'a> { RemoveCredentialParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
    pub fn credentialId(&self) -> &str { self.credentialId.as_ref() }
}

#[derive(Default)]
pub struct RemoveCredentialParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
    credentialId: Option<Cow<'a, str>>,
}

impl<'a> RemoveCredentialParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn credentialId(mut self, credentialId: impl Into<Cow<'a, str>>) -> Self { self.credentialId = Some(credentialId.into()); self }
    pub fn build(self) -> RemoveCredentialParams<'a> {
        RemoveCredentialParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
            credentialId: self.credentialId.unwrap_or_default(),
        }
    }
}

impl<'a> RemoveCredentialParams<'a> { pub const METHOD: &'static str = "WebAuthn.removeCredential"; }

impl<'a> crate::CdpCommand<'a> for RemoveCredentialParams<'a> {
    const METHOD: &'static str = "WebAuthn.removeCredential";
    type Response = crate::EmptyReturns;
}

/// Clears all the credentials from the specified device.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCredentialsParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
}

impl<'a> ClearCredentialsParams<'a> {
    pub fn builder() -> ClearCredentialsParamsBuilder<'a> { ClearCredentialsParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
}

#[derive(Default)]
pub struct ClearCredentialsParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
}

impl<'a> ClearCredentialsParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn build(self) -> ClearCredentialsParams<'a> {
        ClearCredentialsParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
        }
    }
}

impl<'a> ClearCredentialsParams<'a> { pub const METHOD: &'static str = "WebAuthn.clearCredentials"; }

impl<'a> crate::CdpCommand<'a> for ClearCredentialsParams<'a> {
    const METHOD: &'static str = "WebAuthn.clearCredentials";
    type Response = crate::EmptyReturns;
}

/// Sets whether User Verification succeeds or fails for an authenticator.
/// The default is true.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetUserVerifiedParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
    isUserVerified: bool,
}

impl<'a> SetUserVerifiedParams<'a> {
    pub fn builder() -> SetUserVerifiedParamsBuilder<'a> { SetUserVerifiedParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
    pub fn isUserVerified(&self) -> bool { self.isUserVerified }
}

#[derive(Default)]
pub struct SetUserVerifiedParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
    isUserVerified: Option<bool>,
}

impl<'a> SetUserVerifiedParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn isUserVerified(mut self, isUserVerified: bool) -> Self { self.isUserVerified = Some(isUserVerified); self }
    pub fn build(self) -> SetUserVerifiedParams<'a> {
        SetUserVerifiedParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
            isUserVerified: self.isUserVerified.unwrap_or_default(),
        }
    }
}

impl<'a> SetUserVerifiedParams<'a> { pub const METHOD: &'static str = "WebAuthn.setUserVerified"; }

impl<'a> crate::CdpCommand<'a> for SetUserVerifiedParams<'a> {
    const METHOD: &'static str = "WebAuthn.setUserVerified";
    type Response = crate::EmptyReturns;
}

/// Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
/// The default is true.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAutomaticPresenceSimulationParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
    enabled: bool,
}

impl<'a> SetAutomaticPresenceSimulationParams<'a> {
    pub fn builder() -> SetAutomaticPresenceSimulationParamsBuilder<'a> { SetAutomaticPresenceSimulationParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
    pub fn enabled(&self) -> bool { self.enabled }
}

#[derive(Default)]
pub struct SetAutomaticPresenceSimulationParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
    enabled: Option<bool>,
}

impl<'a> SetAutomaticPresenceSimulationParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    pub fn build(self) -> SetAutomaticPresenceSimulationParams<'a> {
        SetAutomaticPresenceSimulationParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
            enabled: self.enabled.unwrap_or_default(),
        }
    }
}

impl<'a> SetAutomaticPresenceSimulationParams<'a> { pub const METHOD: &'static str = "WebAuthn.setAutomaticPresenceSimulation"; }

impl<'a> crate::CdpCommand<'a> for SetAutomaticPresenceSimulationParams<'a> {
    const METHOD: &'static str = "WebAuthn.setAutomaticPresenceSimulation";
    type Response = crate::EmptyReturns;
}

/// Allows setting credential properties.
/// https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCredentialPropertiesParams<'a> {
    authenticatorId: AuthenticatorId<'a>,
    credentialId: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backupEligibility: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backupState: Option<bool>,
}

impl<'a> SetCredentialPropertiesParams<'a> {
    pub fn builder() -> SetCredentialPropertiesParamsBuilder<'a> { SetCredentialPropertiesParamsBuilder::default() }
    pub fn authenticatorId(&self) -> &AuthenticatorId<'a> { &self.authenticatorId }
    pub fn credentialId(&self) -> &str { self.credentialId.as_ref() }
    pub fn backupEligibility(&self) -> Option<bool> { self.backupEligibility }
    pub fn backupState(&self) -> Option<bool> { self.backupState }
}

#[derive(Default)]
pub struct SetCredentialPropertiesParamsBuilder<'a> {
    authenticatorId: Option<AuthenticatorId<'a>>,
    credentialId: Option<Cow<'a, str>>,
    backupEligibility: Option<bool>,
    backupState: Option<bool>,
}

impl<'a> SetCredentialPropertiesParamsBuilder<'a> {
    pub fn authenticatorId(mut self, authenticatorId: AuthenticatorId<'a>) -> Self { self.authenticatorId = Some(authenticatorId); self }
    pub fn credentialId(mut self, credentialId: impl Into<Cow<'a, str>>) -> Self { self.credentialId = Some(credentialId.into()); self }
    pub fn backupEligibility(mut self, backupEligibility: bool) -> Self { self.backupEligibility = Some(backupEligibility); self }
    pub fn backupState(mut self, backupState: bool) -> Self { self.backupState = Some(backupState); self }
    pub fn build(self) -> SetCredentialPropertiesParams<'a> {
        SetCredentialPropertiesParams {
            authenticatorId: self.authenticatorId.unwrap_or_default(),
            credentialId: self.credentialId.unwrap_or_default(),
            backupEligibility: self.backupEligibility,
            backupState: self.backupState,
        }
    }
}

impl<'a> SetCredentialPropertiesParams<'a> { pub const METHOD: &'static str = "WebAuthn.setCredentialProperties"; }

impl<'a> crate::CdpCommand<'a> for SetCredentialPropertiesParams<'a> {
    const METHOD: &'static str = "WebAuthn.setCredentialProperties";
    type Response = crate::EmptyReturns;
}
