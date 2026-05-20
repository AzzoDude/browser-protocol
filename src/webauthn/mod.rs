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
    #[serde(skip_serializing_if = "Option::is_none", rename = "ctap2Version")]
    ctap2_version: Option<Ctap2Version>,
    transport: AuthenticatorTransport,
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasResidentKey")]
    has_resident_key: Option<bool>,
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasUserVerification")]
    has_user_verification: Option<bool>,
    /// If set to true, the authenticator will support the largeBlob extension.
    /// <https://w3c.github.io/webauthn#largeBlob>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasLargeBlob")]
    has_large_blob: Option<bool>,
    /// If set to true, the authenticator will support the credBlob extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasCredBlob")]
    has_cred_blob: Option<bool>,
    /// If set to true, the authenticator will support the minPinLength extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasMinPinLength")]
    has_min_pin_length: Option<bool>,
    /// If set to true, the authenticator will support the prf extension.
    /// <https://w3c.github.io/webauthn/#prf-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasPrf")]
    has_prf: Option<bool>,
    /// If set to true, the authenticator will support the hmac-secret extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasHmacSecret")]
    has_hmac_secret: Option<bool>,
    /// If set to true, the authenticator will support the hmac-secret-mc extension.
    /// <https://fidoalliance.org/specs/fido-v2.2-rd-20241003/fido-client-to-authenticator-protocol-v2.2-rd-20241003.html#sctn-hmac-secret-make-cred-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasHmacSecretMc")]
    has_hmac_secret_mc: Option<bool>,
    /// If set to true, tests of user presence will succeed immediately.
    /// Otherwise, they will not be resolved. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "automaticPresenceSimulation")]
    automatic_presence_simulation: Option<bool>,
    /// Sets whether User Verification succeeds or fails for an authenticator.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isUserVerified")]
    is_user_verified: Option<bool>,
    /// Credentials created by this authenticator will have the backup
    /// eligibility (BE) flag set to this value. Defaults to false.
    /// <https://w3c.github.io/webauthn/#sctn-credential-backup>
    #[serde(skip_serializing_if = "Option::is_none", rename = "defaultBackupEligibility")]
    default_backup_eligibility: Option<bool>,
    /// Credentials created by this authenticator will have the backup state
    /// (BS) flag set to this value. Defaults to false.
    /// <https://w3c.github.io/webauthn/#sctn-credential-backup>
    #[serde(skip_serializing_if = "Option::is_none", rename = "defaultBackupState")]
    default_backup_state: Option<bool>,
}

impl VirtualAuthenticatorOptions {
    /// Creates a builder for this type with the required parameters:
    /// * `protocol`: 
    /// * `transport`: 
    pub fn builder(protocol: impl Into<AuthenticatorProtocol>, transport: impl Into<AuthenticatorTransport>) -> VirtualAuthenticatorOptionsBuilder {
        VirtualAuthenticatorOptionsBuilder {
            protocol: protocol.into(),
            ctap2_version: None,
            transport: transport.into(),
            has_resident_key: None,
            has_user_verification: None,
            has_large_blob: None,
            has_cred_blob: None,
            has_min_pin_length: None,
            has_prf: None,
            has_hmac_secret: None,
            has_hmac_secret_mc: None,
            automatic_presence_simulation: None,
            is_user_verified: None,
            default_backup_eligibility: None,
            default_backup_state: None,
        }
    }
    pub fn protocol(&self) -> &AuthenticatorProtocol { &self.protocol }
    /// Defaults to ctap2_0. Ignored if |protocol| == u2f.
    pub fn ctap2_version(&self) -> Option<&Ctap2Version> { self.ctap2_version.as_ref() }
    pub fn transport(&self) -> &AuthenticatorTransport { &self.transport }
    /// Defaults to false.
    pub fn has_resident_key(&self) -> Option<bool> { self.has_resident_key }
    /// Defaults to false.
    pub fn has_user_verification(&self) -> Option<bool> { self.has_user_verification }
    /// If set to true, the authenticator will support the largeBlob extension.
    /// <https://w3c.github.io/webauthn#largeBlob>
    /// Defaults to false.
    pub fn has_large_blob(&self) -> Option<bool> { self.has_large_blob }
    /// If set to true, the authenticator will support the credBlob extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension>
    /// Defaults to false.
    pub fn has_cred_blob(&self) -> Option<bool> { self.has_cred_blob }
    /// If set to true, the authenticator will support the minPinLength extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension>
    /// Defaults to false.
    pub fn has_min_pin_length(&self) -> Option<bool> { self.has_min_pin_length }
    /// If set to true, the authenticator will support the prf extension.
    /// <https://w3c.github.io/webauthn/#prf-extension>
    /// Defaults to false.
    pub fn has_prf(&self) -> Option<bool> { self.has_prf }
    /// If set to true, the authenticator will support the hmac-secret extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension>
    /// Defaults to false.
    pub fn has_hmac_secret(&self) -> Option<bool> { self.has_hmac_secret }
    /// If set to true, the authenticator will support the hmac-secret-mc extension.
    /// <https://fidoalliance.org/specs/fido-v2.2-rd-20241003/fido-client-to-authenticator-protocol-v2.2-rd-20241003.html#sctn-hmac-secret-make-cred-extension>
    /// Defaults to false.
    pub fn has_hmac_secret_mc(&self) -> Option<bool> { self.has_hmac_secret_mc }
    /// If set to true, tests of user presence will succeed immediately.
    /// Otherwise, they will not be resolved. Defaults to true.
    pub fn automatic_presence_simulation(&self) -> Option<bool> { self.automatic_presence_simulation }
    /// Sets whether User Verification succeeds or fails for an authenticator.
    /// Defaults to false.
    pub fn is_user_verified(&self) -> Option<bool> { self.is_user_verified }
    /// Credentials created by this authenticator will have the backup
    /// eligibility (BE) flag set to this value. Defaults to false.
    /// <https://w3c.github.io/webauthn/#sctn-credential-backup>
    pub fn default_backup_eligibility(&self) -> Option<bool> { self.default_backup_eligibility }
    /// Credentials created by this authenticator will have the backup state
    /// (BS) flag set to this value. Defaults to false.
    /// <https://w3c.github.io/webauthn/#sctn-credential-backup>
    pub fn default_backup_state(&self) -> Option<bool> { self.default_backup_state }
}


pub struct VirtualAuthenticatorOptionsBuilder {
    protocol: AuthenticatorProtocol,
    ctap2_version: Option<Ctap2Version>,
    transport: AuthenticatorTransport,
    has_resident_key: Option<bool>,
    has_user_verification: Option<bool>,
    has_large_blob: Option<bool>,
    has_cred_blob: Option<bool>,
    has_min_pin_length: Option<bool>,
    has_prf: Option<bool>,
    has_hmac_secret: Option<bool>,
    has_hmac_secret_mc: Option<bool>,
    automatic_presence_simulation: Option<bool>,
    is_user_verified: Option<bool>,
    default_backup_eligibility: Option<bool>,
    default_backup_state: Option<bool>,
}

impl VirtualAuthenticatorOptionsBuilder {
    /// Defaults to ctap2_0. Ignored if |protocol| == u2f.
    pub fn ctap2_version(mut self, ctap2_version: impl Into<Ctap2Version>) -> Self { self.ctap2_version = Some(ctap2_version.into()); self }
    /// Defaults to false.
    pub fn has_resident_key(mut self, has_resident_key: bool) -> Self { self.has_resident_key = Some(has_resident_key); self }
    /// Defaults to false.
    pub fn has_user_verification(mut self, has_user_verification: bool) -> Self { self.has_user_verification = Some(has_user_verification); self }
    /// If set to true, the authenticator will support the largeBlob extension.
    /// <https://w3c.github.io/webauthn#largeBlob>
    /// Defaults to false.
    pub fn has_large_blob(mut self, has_large_blob: bool) -> Self { self.has_large_blob = Some(has_large_blob); self }
    /// If set to true, the authenticator will support the credBlob extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension>
    /// Defaults to false.
    pub fn has_cred_blob(mut self, has_cred_blob: bool) -> Self { self.has_cred_blob = Some(has_cred_blob); self }
    /// If set to true, the authenticator will support the minPinLength extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension>
    /// Defaults to false.
    pub fn has_min_pin_length(mut self, has_min_pin_length: bool) -> Self { self.has_min_pin_length = Some(has_min_pin_length); self }
    /// If set to true, the authenticator will support the prf extension.
    /// <https://w3c.github.io/webauthn/#prf-extension>
    /// Defaults to false.
    pub fn has_prf(mut self, has_prf: bool) -> Self { self.has_prf = Some(has_prf); self }
    /// If set to true, the authenticator will support the hmac-secret extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension>
    /// Defaults to false.
    pub fn has_hmac_secret(mut self, has_hmac_secret: bool) -> Self { self.has_hmac_secret = Some(has_hmac_secret); self }
    /// If set to true, the authenticator will support the hmac-secret-mc extension.
    /// <https://fidoalliance.org/specs/fido-v2.2-rd-20241003/fido-client-to-authenticator-protocol-v2.2-rd-20241003.html#sctn-hmac-secret-make-cred-extension>
    /// Defaults to false.
    pub fn has_hmac_secret_mc(mut self, has_hmac_secret_mc: bool) -> Self { self.has_hmac_secret_mc = Some(has_hmac_secret_mc); self }
    /// If set to true, tests of user presence will succeed immediately.
    /// Otherwise, they will not be resolved. Defaults to true.
    pub fn automatic_presence_simulation(mut self, automatic_presence_simulation: bool) -> Self { self.automatic_presence_simulation = Some(automatic_presence_simulation); self }
    /// Sets whether User Verification succeeds or fails for an authenticator.
    /// Defaults to false.
    pub fn is_user_verified(mut self, is_user_verified: bool) -> Self { self.is_user_verified = Some(is_user_verified); self }
    /// Credentials created by this authenticator will have the backup
    /// eligibility (BE) flag set to this value. Defaults to false.
    /// <https://w3c.github.io/webauthn/#sctn-credential-backup>
    pub fn default_backup_eligibility(mut self, default_backup_eligibility: bool) -> Self { self.default_backup_eligibility = Some(default_backup_eligibility); self }
    /// Credentials created by this authenticator will have the backup state
    /// (BS) flag set to this value. Defaults to false.
    /// <https://w3c.github.io/webauthn/#sctn-credential-backup>
    pub fn default_backup_state(mut self, default_backup_state: bool) -> Self { self.default_backup_state = Some(default_backup_state); self }
    pub fn build(self) -> VirtualAuthenticatorOptions {
        VirtualAuthenticatorOptions {
            protocol: self.protocol,
            ctap2_version: self.ctap2_version,
            transport: self.transport,
            has_resident_key: self.has_resident_key,
            has_user_verification: self.has_user_verification,
            has_large_blob: self.has_large_blob,
            has_cred_blob: self.has_cred_blob,
            has_min_pin_length: self.has_min_pin_length,
            has_prf: self.has_prf,
            has_hmac_secret: self.has_hmac_secret,
            has_hmac_secret_mc: self.has_hmac_secret_mc,
            automatic_presence_simulation: self.automatic_presence_simulation,
            is_user_verified: self.is_user_verified,
            default_backup_eligibility: self.default_backup_eligibility,
            default_backup_state: self.default_backup_state,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Credential<'a> {
    #[serde(rename = "credentialId")]
    credential_id: Cow<'a, str>,
    #[serde(rename = "isResidentCredential")]
    is_resident_credential: bool,
    /// Relying Party ID the credential is scoped to. Must be set when adding a
    /// credential.
    #[serde(skip_serializing_if = "Option::is_none", rename = "rpId")]
    rp_id: Option<Cow<'a, str>>,
    /// The ECDSA P-256 private key in PKCS#8 format. (Encoded as a base64 string when passed over JSON)
    #[serde(rename = "privateKey")]
    private_key: Cow<'a, str>,
    /// An opaque byte sequence with a maximum size of 64 bytes mapping the
    /// credential to a specific user. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "userHandle")]
    user_handle: Option<Cow<'a, str>>,
    /// Signature counter. This is incremented by one for each successful
    /// assertion.
    /// See <https://w3c.github.io/webauthn/#signature-counter>
    #[serde(rename = "signCount")]
    sign_count: u64,
    /// The large blob associated with the credential.
    /// See <https://w3c.github.io/webauthn/#sctn-large-blob-extension> (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "largeBlob")]
    large_blob: Option<Cow<'a, str>>,
    /// Assertions returned by this credential will have the backup eligibility
    /// (BE) flag set to this value. Defaults to the authenticator's
    /// defaultBackupEligibility value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backupEligibility")]
    backup_eligibility: Option<bool>,
    /// Assertions returned by this credential will have the backup state (BS)
    /// flag set to this value. Defaults to the authenticator's
    /// defaultBackupState value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backupState")]
    backup_state: Option<bool>,
    /// The credential's user.name property. Equivalent to empty if not set.
    /// <https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name>
    #[serde(skip_serializing_if = "Option::is_none", rename = "userName")]
    user_name: Option<Cow<'a, str>>,
    /// The credential's user.displayName property. Equivalent to empty if
    /// not set.
    /// <https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname>
    #[serde(skip_serializing_if = "Option::is_none", rename = "userDisplayName")]
    user_display_name: Option<Cow<'a, str>>,
}

impl<'a> Credential<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `credential_id`: 
    /// * `is_resident_credential`: 
    /// * `private_key`: The ECDSA P-256 private key in PKCS#8 format. (Encoded as a base64 string when passed over JSON)
    /// * `sign_count`: Signature counter. This is incremented by one for each successful assertion. See <https://w3c.github.io/webauthn/#signature-counter>
    pub fn builder(credential_id: impl Into<Cow<'a, str>>, is_resident_credential: bool, private_key: impl Into<Cow<'a, str>>, sign_count: u64) -> CredentialBuilder<'a> {
        CredentialBuilder {
            credential_id: credential_id.into(),
            is_resident_credential: is_resident_credential,
            rp_id: None,
            private_key: private_key.into(),
            user_handle: None,
            sign_count: sign_count,
            large_blob: None,
            backup_eligibility: None,
            backup_state: None,
            user_name: None,
            user_display_name: None,
        }
    }
    pub fn credential_id(&self) -> &str { self.credential_id.as_ref() }
    pub fn is_resident_credential(&self) -> bool { self.is_resident_credential }
    /// Relying Party ID the credential is scoped to. Must be set when adding a
    /// credential.
    pub fn rp_id(&self) -> Option<&str> { self.rp_id.as_deref() }
    /// The ECDSA P-256 private key in PKCS#8 format. (Encoded as a base64 string when passed over JSON)
    pub fn private_key(&self) -> &str { self.private_key.as_ref() }
    /// An opaque byte sequence with a maximum size of 64 bytes mapping the
    /// credential to a specific user. (Encoded as a base64 string when passed over JSON)
    pub fn user_handle(&self) -> Option<&str> { self.user_handle.as_deref() }
    /// Signature counter. This is incremented by one for each successful
    /// assertion.
    /// See <https://w3c.github.io/webauthn/#signature-counter>
    pub fn sign_count(&self) -> u64 { self.sign_count }
    /// The large blob associated with the credential.
    /// See <https://w3c.github.io/webauthn/#sctn-large-blob-extension> (Encoded as a base64 string when passed over JSON)
    pub fn large_blob(&self) -> Option<&str> { self.large_blob.as_deref() }
    /// Assertions returned by this credential will have the backup eligibility
    /// (BE) flag set to this value. Defaults to the authenticator's
    /// defaultBackupEligibility value.
    pub fn backup_eligibility(&self) -> Option<bool> { self.backup_eligibility }
    /// Assertions returned by this credential will have the backup state (BS)
    /// flag set to this value. Defaults to the authenticator's
    /// defaultBackupState value.
    pub fn backup_state(&self) -> Option<bool> { self.backup_state }
    /// The credential's user.name property. Equivalent to empty if not set.
    /// <https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name>
    pub fn user_name(&self) -> Option<&str> { self.user_name.as_deref() }
    /// The credential's user.displayName property. Equivalent to empty if
    /// not set.
    /// <https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname>
    pub fn user_display_name(&self) -> Option<&str> { self.user_display_name.as_deref() }
}


pub struct CredentialBuilder<'a> {
    credential_id: Cow<'a, str>,
    is_resident_credential: bool,
    rp_id: Option<Cow<'a, str>>,
    private_key: Cow<'a, str>,
    user_handle: Option<Cow<'a, str>>,
    sign_count: u64,
    large_blob: Option<Cow<'a, str>>,
    backup_eligibility: Option<bool>,
    backup_state: Option<bool>,
    user_name: Option<Cow<'a, str>>,
    user_display_name: Option<Cow<'a, str>>,
}

impl<'a> CredentialBuilder<'a> {
    /// Relying Party ID the credential is scoped to. Must be set when adding a
    /// credential.
    pub fn rp_id(mut self, rp_id: impl Into<Cow<'a, str>>) -> Self { self.rp_id = Some(rp_id.into()); self }
    /// An opaque byte sequence with a maximum size of 64 bytes mapping the
    /// credential to a specific user. (Encoded as a base64 string when passed over JSON)
    pub fn user_handle(mut self, user_handle: impl Into<Cow<'a, str>>) -> Self { self.user_handle = Some(user_handle.into()); self }
    /// The large blob associated with the credential.
    /// See <https://w3c.github.io/webauthn/#sctn-large-blob-extension> (Encoded as a base64 string when passed over JSON)
    pub fn large_blob(mut self, large_blob: impl Into<Cow<'a, str>>) -> Self { self.large_blob = Some(large_blob.into()); self }
    /// Assertions returned by this credential will have the backup eligibility
    /// (BE) flag set to this value. Defaults to the authenticator's
    /// defaultBackupEligibility value.
    pub fn backup_eligibility(mut self, backup_eligibility: bool) -> Self { self.backup_eligibility = Some(backup_eligibility); self }
    /// Assertions returned by this credential will have the backup state (BS)
    /// flag set to this value. Defaults to the authenticator's
    /// defaultBackupState value.
    pub fn backup_state(mut self, backup_state: bool) -> Self { self.backup_state = Some(backup_state); self }
    /// The credential's user.name property. Equivalent to empty if not set.
    /// <https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name>
    pub fn user_name(mut self, user_name: impl Into<Cow<'a, str>>) -> Self { self.user_name = Some(user_name.into()); self }
    /// The credential's user.displayName property. Equivalent to empty if
    /// not set.
    /// <https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname>
    pub fn user_display_name(mut self, user_display_name: impl Into<Cow<'a, str>>) -> Self { self.user_display_name = Some(user_display_name.into()); self }
    pub fn build(self) -> Credential<'a> {
        Credential {
            credential_id: self.credential_id,
            is_resident_credential: self.is_resident_credential,
            rp_id: self.rp_id,
            private_key: self.private_key,
            user_handle: self.user_handle,
            sign_count: self.sign_count,
            large_blob: self.large_blob,
            backup_eligibility: self.backup_eligibility,
            backup_state: self.backup_state,
            user_name: self.user_name,
            user_display_name: self.user_display_name,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableUI")]
    enable_ui: Option<bool>,
}

impl EnableParams {
    /// Creates a builder for this type.
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder {
            enable_ui: None,
        }
    }
    /// Whether to enable the WebAuthn user interface. Enabling the UI is
    /// recommended for debugging and demo purposes, as it is closer to the real
    /// experience. Disabling the UI is recommended for automated testing.
    /// Supported at the embedder's discretion if UI is available.
    /// Defaults to false.
    pub fn enable_ui(&self) -> Option<bool> { self.enable_ui }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    enable_ui: Option<bool>,
}

impl EnableParamsBuilder {
    /// Whether to enable the WebAuthn user interface. Enabling the UI is
    /// recommended for debugging and demo purposes, as it is closer to the real
    /// experience. Disabling the UI is recommended for automated testing.
    /// Supported at the embedder's discretion if UI is available.
    /// Defaults to false.
    pub fn enable_ui(mut self, enable_ui: bool) -> Self { self.enable_ui = Some(enable_ui); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            enable_ui: self.enable_ui,
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
    /// Creates a builder for this type with the required parameters:
    /// * `options`: 
    pub fn builder(options: VirtualAuthenticatorOptions) -> AddVirtualAuthenticatorParamsBuilder {
        AddVirtualAuthenticatorParamsBuilder {
            options: options,
        }
    }
    pub fn options(&self) -> &VirtualAuthenticatorOptions { &self.options }
}


pub struct AddVirtualAuthenticatorParamsBuilder {
    options: VirtualAuthenticatorOptions,
}

impl AddVirtualAuthenticatorParamsBuilder {
    pub fn build(self) -> AddVirtualAuthenticatorParams {
        AddVirtualAuthenticatorParams {
            options: self.options,
        }
    }
}

/// Creates and adds a virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddVirtualAuthenticatorReturns<'a> {
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
}

impl<'a> AddVirtualAuthenticatorReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>) -> AddVirtualAuthenticatorReturnsBuilder<'a> {
        AddVirtualAuthenticatorReturnsBuilder {
            authenticator_id: authenticator_id.into(),
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
}


pub struct AddVirtualAuthenticatorReturnsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
}

impl<'a> AddVirtualAuthenticatorReturnsBuilder<'a> {
    pub fn build(self) -> AddVirtualAuthenticatorReturns<'a> {
        AddVirtualAuthenticatorReturns {
            authenticator_id: self.authenticator_id,
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
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
    /// If isBogusSignature is set, overrides the signature in the authenticator response to be zero.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isBogusSignature")]
    is_bogus_signature: Option<bool>,
    /// If isBadUV is set, overrides the UV bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isBadUV")]
    is_bad_uv: Option<bool>,
    /// If isBadUP is set, overrides the UP bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isBadUP")]
    is_bad_up: Option<bool>,
}

impl<'a> SetResponseOverrideBitsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>) -> SetResponseOverrideBitsParamsBuilder<'a> {
        SetResponseOverrideBitsParamsBuilder {
            authenticator_id: authenticator_id.into(),
            is_bogus_signature: None,
            is_bad_uv: None,
            is_bad_up: None,
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
    /// If isBogusSignature is set, overrides the signature in the authenticator response to be zero.
    /// Defaults to false.
    pub fn is_bogus_signature(&self) -> Option<bool> { self.is_bogus_signature }
    /// If isBadUV is set, overrides the UV bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    pub fn is_bad_uv(&self) -> Option<bool> { self.is_bad_uv }
    /// If isBadUP is set, overrides the UP bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    pub fn is_bad_up(&self) -> Option<bool> { self.is_bad_up }
}


pub struct SetResponseOverrideBitsParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
    is_bogus_signature: Option<bool>,
    is_bad_uv: Option<bool>,
    is_bad_up: Option<bool>,
}

impl<'a> SetResponseOverrideBitsParamsBuilder<'a> {
    /// If isBogusSignature is set, overrides the signature in the authenticator response to be zero.
    /// Defaults to false.
    pub fn is_bogus_signature(mut self, is_bogus_signature: bool) -> Self { self.is_bogus_signature = Some(is_bogus_signature); self }
    /// If isBadUV is set, overrides the UV bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    pub fn is_bad_uv(mut self, is_bad_uv: bool) -> Self { self.is_bad_uv = Some(is_bad_uv); self }
    /// If isBadUP is set, overrides the UP bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    pub fn is_bad_up(mut self, is_bad_up: bool) -> Self { self.is_bad_up = Some(is_bad_up); self }
    pub fn build(self) -> SetResponseOverrideBitsParams<'a> {
        SetResponseOverrideBitsParams {
            authenticator_id: self.authenticator_id,
            is_bogus_signature: self.is_bogus_signature,
            is_bad_uv: self.is_bad_uv,
            is_bad_up: self.is_bad_up,
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
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
}

impl<'a> RemoveVirtualAuthenticatorParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>) -> RemoveVirtualAuthenticatorParamsBuilder<'a> {
        RemoveVirtualAuthenticatorParamsBuilder {
            authenticator_id: authenticator_id.into(),
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
}


pub struct RemoveVirtualAuthenticatorParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
}

impl<'a> RemoveVirtualAuthenticatorParamsBuilder<'a> {
    pub fn build(self) -> RemoveVirtualAuthenticatorParams<'a> {
        RemoveVirtualAuthenticatorParams {
            authenticator_id: self.authenticator_id,
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
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
    credential: Credential<'a>,
}

impl<'a> AddCredentialParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    /// * `credential`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>, credential: Credential<'a>) -> AddCredentialParamsBuilder<'a> {
        AddCredentialParamsBuilder {
            authenticator_id: authenticator_id.into(),
            credential: credential,
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
    pub fn credential(&self) -> &Credential<'a> { &self.credential }
}


pub struct AddCredentialParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
    credential: Credential<'a>,
}

impl<'a> AddCredentialParamsBuilder<'a> {
    pub fn build(self) -> AddCredentialParams<'a> {
        AddCredentialParams {
            authenticator_id: self.authenticator_id,
            credential: self.credential,
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
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
    #[serde(rename = "credentialId")]
    credential_id: Cow<'a, str>,
}

impl<'a> GetCredentialParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    /// * `credential_id`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>, credential_id: impl Into<Cow<'a, str>>) -> GetCredentialParamsBuilder<'a> {
        GetCredentialParamsBuilder {
            authenticator_id: authenticator_id.into(),
            credential_id: credential_id.into(),
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
    pub fn credential_id(&self) -> &str { self.credential_id.as_ref() }
}


pub struct GetCredentialParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
    credential_id: Cow<'a, str>,
}

impl<'a> GetCredentialParamsBuilder<'a> {
    pub fn build(self) -> GetCredentialParams<'a> {
        GetCredentialParams {
            authenticator_id: self.authenticator_id,
            credential_id: self.credential_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `credential`: 
    pub fn builder(credential: Credential<'a>) -> GetCredentialReturnsBuilder<'a> {
        GetCredentialReturnsBuilder {
            credential: credential,
        }
    }
    pub fn credential(&self) -> &Credential<'a> { &self.credential }
}


pub struct GetCredentialReturnsBuilder<'a> {
    credential: Credential<'a>,
}

impl<'a> GetCredentialReturnsBuilder<'a> {
    pub fn build(self) -> GetCredentialReturns<'a> {
        GetCredentialReturns {
            credential: self.credential,
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
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
}

impl<'a> GetCredentialsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>) -> GetCredentialsParamsBuilder<'a> {
        GetCredentialsParamsBuilder {
            authenticator_id: authenticator_id.into(),
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
}


pub struct GetCredentialsParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
}

impl<'a> GetCredentialsParamsBuilder<'a> {
    pub fn build(self) -> GetCredentialsParams<'a> {
        GetCredentialsParams {
            authenticator_id: self.authenticator_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `credentials`: 
    pub fn builder(credentials: Vec<Credential<'a>>) -> GetCredentialsReturnsBuilder<'a> {
        GetCredentialsReturnsBuilder {
            credentials: credentials,
        }
    }
    pub fn credentials(&self) -> &[Credential<'a>] { &self.credentials }
}


pub struct GetCredentialsReturnsBuilder<'a> {
    credentials: Vec<Credential<'a>>,
}

impl<'a> GetCredentialsReturnsBuilder<'a> {
    pub fn build(self) -> GetCredentialsReturns<'a> {
        GetCredentialsReturns {
            credentials: self.credentials,
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
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
    #[serde(rename = "credentialId")]
    credential_id: Cow<'a, str>,
}

impl<'a> RemoveCredentialParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    /// * `credential_id`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>, credential_id: impl Into<Cow<'a, str>>) -> RemoveCredentialParamsBuilder<'a> {
        RemoveCredentialParamsBuilder {
            authenticator_id: authenticator_id.into(),
            credential_id: credential_id.into(),
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
    pub fn credential_id(&self) -> &str { self.credential_id.as_ref() }
}


pub struct RemoveCredentialParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
    credential_id: Cow<'a, str>,
}

impl<'a> RemoveCredentialParamsBuilder<'a> {
    pub fn build(self) -> RemoveCredentialParams<'a> {
        RemoveCredentialParams {
            authenticator_id: self.authenticator_id,
            credential_id: self.credential_id,
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
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
}

impl<'a> ClearCredentialsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>) -> ClearCredentialsParamsBuilder<'a> {
        ClearCredentialsParamsBuilder {
            authenticator_id: authenticator_id.into(),
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
}


pub struct ClearCredentialsParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
}

impl<'a> ClearCredentialsParamsBuilder<'a> {
    pub fn build(self) -> ClearCredentialsParams<'a> {
        ClearCredentialsParams {
            authenticator_id: self.authenticator_id,
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
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
    #[serde(rename = "isUserVerified")]
    is_user_verified: bool,
}

impl<'a> SetUserVerifiedParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    /// * `is_user_verified`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>, is_user_verified: bool) -> SetUserVerifiedParamsBuilder<'a> {
        SetUserVerifiedParamsBuilder {
            authenticator_id: authenticator_id.into(),
            is_user_verified: is_user_verified,
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
    pub fn is_user_verified(&self) -> bool { self.is_user_verified }
}


pub struct SetUserVerifiedParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
    is_user_verified: bool,
}

impl<'a> SetUserVerifiedParamsBuilder<'a> {
    pub fn build(self) -> SetUserVerifiedParams<'a> {
        SetUserVerifiedParams {
            authenticator_id: self.authenticator_id,
            is_user_verified: self.is_user_verified,
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
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
    enabled: bool,
}

impl<'a> SetAutomaticPresenceSimulationParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    /// * `enabled`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>, enabled: bool) -> SetAutomaticPresenceSimulationParamsBuilder<'a> {
        SetAutomaticPresenceSimulationParamsBuilder {
            authenticator_id: authenticator_id.into(),
            enabled: enabled,
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetAutomaticPresenceSimulationParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
    enabled: bool,
}

impl<'a> SetAutomaticPresenceSimulationParamsBuilder<'a> {
    pub fn build(self) -> SetAutomaticPresenceSimulationParams<'a> {
        SetAutomaticPresenceSimulationParams {
            authenticator_id: self.authenticator_id,
            enabled: self.enabled,
        }
    }
}

impl<'a> SetAutomaticPresenceSimulationParams<'a> { pub const METHOD: &'static str = "WebAuthn.setAutomaticPresenceSimulation"; }

impl<'a> crate::CdpCommand<'a> for SetAutomaticPresenceSimulationParams<'a> {
    const METHOD: &'static str = "WebAuthn.setAutomaticPresenceSimulation";
    type Response = crate::EmptyReturns;
}

/// Allows setting credential properties.
/// <https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCredentialPropertiesParams<'a> {
    #[serde(rename = "authenticatorId")]
    authenticator_id: AuthenticatorId<'a>,
    #[serde(rename = "credentialId")]
    credential_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "backupEligibility")]
    backup_eligibility: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "backupState")]
    backup_state: Option<bool>,
}

impl<'a> SetCredentialPropertiesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `authenticator_id`: 
    /// * `credential_id`: 
    pub fn builder(authenticator_id: impl Into<AuthenticatorId<'a>>, credential_id: impl Into<Cow<'a, str>>) -> SetCredentialPropertiesParamsBuilder<'a> {
        SetCredentialPropertiesParamsBuilder {
            authenticator_id: authenticator_id.into(),
            credential_id: credential_id.into(),
            backup_eligibility: None,
            backup_state: None,
        }
    }
    pub fn authenticator_id(&self) -> &AuthenticatorId<'a> { &self.authenticator_id }
    pub fn credential_id(&self) -> &str { self.credential_id.as_ref() }
    pub fn backup_eligibility(&self) -> Option<bool> { self.backup_eligibility }
    pub fn backup_state(&self) -> Option<bool> { self.backup_state }
}


pub struct SetCredentialPropertiesParamsBuilder<'a> {
    authenticator_id: AuthenticatorId<'a>,
    credential_id: Cow<'a, str>,
    backup_eligibility: Option<bool>,
    backup_state: Option<bool>,
}

impl<'a> SetCredentialPropertiesParamsBuilder<'a> {
    pub fn backup_eligibility(mut self, backup_eligibility: bool) -> Self { self.backup_eligibility = Some(backup_eligibility); self }
    pub fn backup_state(mut self, backup_state: bool) -> Self { self.backup_state = Some(backup_state); self }
    pub fn build(self) -> SetCredentialPropertiesParams<'a> {
        SetCredentialPropertiesParams {
            authenticator_id: self.authenticator_id,
            credential_id: self.credential_id,
            backup_eligibility: self.backup_eligibility,
            backup_state: self.backup_state,
        }
    }
}

impl<'a> SetCredentialPropertiesParams<'a> { pub const METHOD: &'static str = "WebAuthn.setCredentialProperties"; }

impl<'a> crate::CdpCommand<'a> for SetCredentialPropertiesParams<'a> {
    const METHOD: &'static str = "WebAuthn.setCredentialProperties";
    type Response = crate::EmptyReturns;
}
