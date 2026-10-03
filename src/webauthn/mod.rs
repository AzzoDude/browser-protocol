//! This domain allows configuring virtual authenticators to test the WebAuthn
//! API.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


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

/// LINT.IfChange(AuthenticatorTransport)

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
    #[serde(rename = "hybrid")]
    Hybrid,
    #[serde(rename = "smart-card")]
    SmartCard,
    #[serde(rename = "internal")]
    Internal,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAuthenticatorOptions {
    pub protocol: AuthenticatorProtocol,
    /// Defaults to ctap2_0. Ignored if |protocol| == u2f.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ctap2Version")]
    pub ctap2_version: Option<Ctap2Version>,
    pub transport: AuthenticatorTransport,
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasResidentKey")]
    pub has_resident_key: Option<bool>,
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasUserVerification")]
    pub has_user_verification: Option<bool>,
    /// If set to true, the authenticator will support the largeBlob extension.
    /// <https://w3c.github.io/webauthn#largeBlob>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasLargeBlob")]
    pub has_large_blob: Option<bool>,
    /// If set to true, the authenticator will support the credBlob extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasCredBlob")]
    pub has_cred_blob: Option<bool>,
    /// If set to true, the authenticator will support the minPinLength extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasMinPinLength")]
    pub has_min_pin_length: Option<bool>,
    /// If set to true, the authenticator will support the prf extension.
    /// <https://w3c.github.io/webauthn/#prf-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasPrf")]
    pub has_prf: Option<bool>,
    /// If set to true, the authenticator will support the hmac-secret extension.
    /// <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasHmacSecret")]
    pub has_hmac_secret: Option<bool>,
    /// If set to true, the authenticator will support the hmac-secret-mc extension.
    /// <https://fidoalliance.org/specs/fido-v2.2-rd-20241003/fido-client-to-authenticator-protocol-v2.2-rd-20241003.html#sctn-hmac-secret-make-cred-extension>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasHmacSecretMc")]
    pub has_hmac_secret_mc: Option<bool>,
    /// If set to true, the authenticator will support the cmtgKey (Credential
    /// Manager Trust Group Key) extension.
    /// <https://github.com/w3c/webauthn/pull/2377>
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasCmtgKey")]
    pub has_cmtg_key: Option<bool>,
    /// If set to true, tests of user presence will succeed immediately.
    /// Otherwise, they will not be resolved. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "automaticPresenceSimulation")]
    pub automatic_presence_simulation: Option<bool>,
    /// Sets whether User Verification succeeds or fails for an authenticator.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isUserVerified")]
    pub is_user_verified: Option<bool>,
    /// Credentials created by this authenticator will have the backup
    /// eligibility (BE) flag set to this value. Defaults to false.
    /// <https://w3c.github.io/webauthn/#sctn-credential-backup>
    #[serde(skip_serializing_if = "Option::is_none", rename = "defaultBackupEligibility")]
    pub default_backup_eligibility: Option<bool>,
    /// Credentials created by this authenticator will have the backup state
    /// (BS) flag set to this value. Defaults to false.
    /// <https://w3c.github.io/webauthn/#sctn-credential-backup>
    #[serde(skip_serializing_if = "Option::is_none", rename = "defaultBackupState")]
    pub default_backup_state: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Credential<'a> {
    #[serde(rename = "credentialId")]
    pub credential_id: Cow<'a, str>,
    #[serde(rename = "isResidentCredential")]
    pub is_resident_credential: bool,
    /// Relying Party ID the credential is scoped to. Must be set when adding a
    /// credential.
    #[serde(skip_serializing_if = "Option::is_none", rename = "rpId")]
    pub rp_id: Option<Cow<'a, str>>,
    /// The ECDSA P-256 private key in PKCS#8 format. (Encoded as a base64 string when passed over JSON)
    #[serde(rename = "privateKey")]
    pub private_key: Cow<'a, str>,
    /// An opaque byte sequence with a maximum size of 64 bytes mapping the
    /// credential to a specific user. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "userHandle")]
    pub user_handle: Option<Cow<'a, str>>,
    /// Signature counter. Must be equal to or greater than -1.
    /// If -1, the credential won't have an associated signature counter, and
    /// every assertion operation will report a value of 0.
    /// See <https://w3c.github.io/webauthn/#signature-counter>
    #[serde(rename = "signCount")]
    pub sign_count: f64,
    /// The large blob associated with the credential.
    /// See <https://w3c.github.io/webauthn/#sctn-large-blob-extension> (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "largeBlob")]
    pub large_blob: Option<Cow<'a, str>>,
    /// Assertions returned by this credential will have the backup eligibility
    /// (BE) flag set to this value. Defaults to the authenticator's
    /// defaultBackupEligibility value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backupEligibility")]
    pub backup_eligibility: Option<bool>,
    /// Assertions returned by this credential will have the backup state (BS)
    /// flag set to this value. Defaults to the authenticator's
    /// defaultBackupState value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backupState")]
    pub backup_state: Option<bool>,
    /// The credential's user.name property. Equivalent to empty if not set.
    /// <https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name>
    #[serde(skip_serializing_if = "Option::is_none", rename = "userName")]
    pub user_name: Option<Cow<'a, str>>,
    /// The credential's user.displayName property. Equivalent to empty if
    /// not set.
    /// <https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname>
    #[serde(skip_serializing_if = "Option::is_none", rename = "userDisplayName")]
    pub user_display_name: Option<Cow<'a, str>>,
    /// The CMTG keys associated with the credential.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmtgKeys")]
    pub cmtg_keys: Option<Vec<Cow<'a, str>>>,
    /// The 0-based index of the active key in cmtgKeys.
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeCmtgKeyIndex")]
    pub active_cmtg_key_index: Option<u64>,
    /// If true, the authenticator will generate a new CMTG key on the next operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generateCmtgKeyOnNextOperation")]
    pub generate_cmtg_key_on_next_operation: Option<bool>,
}
/// Enable the WebAuthn domain and start intercepting credential storage and
/// retrieval with a virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.enable")]
pub struct EnableParams {
    /// Whether to enable the WebAuthn user interface. Enabling the UI is
    /// recommended for debugging and demo purposes, as it is closer to the real
    /// experience. Disabling the UI is recommended for automated testing.
    /// Supported at the embedder's discretion if UI is available.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableUI")]
    pub enable_ui: Option<bool>,
}
/// Disable the WebAuthn domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.disable")]
pub struct DisableParams {

}
/// Creates and adds a virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.addVirtualAuthenticator", response = "AddVirtualAuthenticatorReturns<'a>")]
pub struct AddVirtualAuthenticatorParams {
    pub options: VirtualAuthenticatorOptions,
}
/// Creates and adds a virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddVirtualAuthenticatorReturns<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
}
/// Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.setResponseOverrideBits")]
pub struct SetResponseOverrideBitsParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    /// If isBogusSignature is set, overrides the signature in the authenticator response to be zero.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isBogusSignature")]
    pub is_bogus_signature: Option<bool>,
    /// If isBadUV is set, overrides the UV bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isBadUV")]
    pub is_bad_uv: Option<bool>,
    /// If isBadUP is set, overrides the UP bit in the flags in the authenticator response to
    /// be zero. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isBadUP")]
    pub is_bad_up: Option<bool>,
}
/// Removes the given authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.removeVirtualAuthenticator")]
pub struct RemoveVirtualAuthenticatorParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
}
/// Adds the credential to the specified authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.addCredential")]
pub struct AddCredentialParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    pub credential: Credential<'a>,
}
/// Returns a single credential stored in the given virtual authenticator that
/// matches the credential ID.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.getCredential", response = "GetCredentialReturns<'a>")]
pub struct GetCredentialParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    #[serde(rename = "credentialId")]
    pub credential_id: Cow<'a, str>,
}
/// Returns a single credential stored in the given virtual authenticator that
/// matches the credential ID.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialReturns<'a> {
    pub credential: Credential<'a>,
}
/// Returns all the credentials stored in the given virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.getCredentials", response = "GetCredentialsReturns<'a>")]
pub struct GetCredentialsParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
}
/// Returns all the credentials stored in the given virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialsReturns<'a> {
    pub credentials: Vec<Credential<'a>>,
}
/// Removes a credential from the authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.removeCredential")]
pub struct RemoveCredentialParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    #[serde(rename = "credentialId")]
    pub credential_id: Cow<'a, str>,
}
/// Clears all the credentials from the specified device.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.clearCredentials")]
pub struct ClearCredentialsParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
}
/// Sets whether User Verification succeeds or fails for an authenticator.
/// The default is true.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.setUserVerified")]
pub struct SetUserVerifiedParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    #[serde(rename = "isUserVerified")]
    pub is_user_verified: bool,
}
/// Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
/// The default is true.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.setAutomaticPresenceSimulation")]
pub struct SetAutomaticPresenceSimulationParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    pub enabled: bool,
}
/// Allows setting credential properties.
/// <https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.setCredentialProperties")]
pub struct SetCredentialPropertiesParams<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    #[serde(rename = "credentialId")]
    pub credential_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "backupEligibility")]
    pub backup_eligibility: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "backupState")]
    pub backup_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeCmtgKeyIndex")]
    pub active_cmtg_key_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "generateCmtgKeyOnNextOperation")]
    pub generate_cmtg_key_on_next_operation: Option<bool>,
    /// Must be equal to or greater than -1.
    /// If -1, the signature counter is removed from the credential, and every
    /// assertion operation will report a value of 0.
    /// See <https://w3c.github.io/webauthn/#signature-counter>
    #[serde(skip_serializing_if = "Option::is_none", rename = "signCount")]
    pub sign_count: Option<f64>,
}
/// Triggered when a credential is added to an authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.credentialAdded")]
pub struct CredentialAdded<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    pub credential: Credential<'a>,
}
/// Triggered when a credential is deleted, e.g. through
/// PublicKeyCredential.signalUnknownCredential().

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.credentialDeleted")]
pub struct CredentialDeleted<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    #[serde(rename = "credentialId")]
    pub credential_id: Cow<'a, str>,
}
/// Triggered when a credential is updated, e.g. through
/// PublicKeyCredential.signalCurrentUserDetails().

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.credentialUpdated")]
pub struct CredentialUpdated<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    pub credential: Credential<'a>,
}
/// Triggered when a credential is used in a webauthn assertion.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "WebAuthn.credentialAsserted")]
pub struct CredentialAsserted<'a> {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId<'a>,
    pub credential: Credential<'a>,
}