use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! This domain allows configuring virtual authenticators to test the WebAuthn
//! API.


pub type AuthenticatorId = String;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AuthenticatorProtocol {
    #[default]
    U2f,
    Ctap2,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Ctap2Version {
    #[default]
    Ctap20,
    Ctap21,
    Ctap22,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AuthenticatorTransport {
    #[default]
    Usb,
    Nfc,
    Ble,
    Cable,
    Internal,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAuthenticatorOptions {

    pub protocol: AuthenticatorProtocol,
    /// Defaults to ctap2_0. Ignored if |protocol| == u2f.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctap2Version: Option<Ctap2Version>,

    pub transport: AuthenticatorTransport,
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasResidentKey: Option<bool>,
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasUserVerification: Option<bool>,
    /// If set to true, the authenticator will support the largeBlob extension.
    /// https://w3c.github.io/webauthn#largeBlob
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasLargeBlob: Option<bool>,
    /// If set to true, the authenticator will support the credBlob extension.
    /// https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasCredBlob: Option<bool>,
    /// If set to true, the authenticator will support the minPinLength extension.
    /// https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasMinPinLength: Option<bool>,
    /// If set to true, the authenticator will support the prf extension.
    /// https://w3c.github.io/webauthn/#prf-extension
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasPrf: Option<bool>,
    /// If set to true, the authenticator will support the hmac-secret extension.
    /// https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasHmacSecret: Option<bool>,
    /// If set to true, the authenticator will support the hmac-secret-mc extension.
    /// https://fidoalliance.org/specs/fido-v2.2-rd-20241003/fido-client-to-authenticator-protocol-v2.2-rd-20241003.html#sctn-hmac-secret-make-cred-extension
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasHmacSecretMc: Option<bool>,
    /// If set to true, tests of user presence will succeed immediately.
    /// Otherwise, they will not be resolved. Defaults to true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automaticPresenceSimulation: Option<bool>,
    /// Sets whether User Verification succeeds or fails for an authenticator.
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isUserVerified: Option<bool>,
    /// Credentials created by this authenticator will have the backup
    /// eligibility (BE) flag set to this value. Defaults to false.
    /// https://w3c.github.io/webauthn/#sctn-credential-backup

    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaultBackupEligibility: Option<bool>,
    /// Credentials created by this authenticator will have the backup state
    /// (BS) flag set to this value. Defaults to false.
    /// https://w3c.github.io/webauthn/#sctn-credential-backup

    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaultBackupState: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Credential {

    pub credentialId: String,

    pub isResidentCredential: bool,
    /// Relying Party ID the credential is scoped to. Must be set when adding a
    /// credential.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpId: Option<String>,
    /// The ECDSA P-256 private key in PKCS#8 format. (Encoded as a base64 string when passed over JSON)

    pub privateKey: String,
    /// An opaque byte sequence with a maximum size of 64 bytes mapping the
    /// credential to a specific user. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userHandle: Option<String>,
    /// Signature counter. This is incremented by one for each successful
    /// assertion.
    /// See https://w3c.github.io/webauthn/#signature-counter

    pub signCount: u64,
    /// The large blob associated with the credential.
    /// See https://w3c.github.io/webauthn/#sctn-large-blob-extension (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub largeBlob: Option<String>,
    /// Assertions returned by this credential will have the backup eligibility
    /// (BE) flag set to this value. Defaults to the authenticator's
    /// defaultBackupEligibility value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backupEligibility: Option<bool>,
    /// Assertions returned by this credential will have the backup state (BS)
    /// flag set to this value. Defaults to the authenticator's
    /// defaultBackupState value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backupState: Option<bool>,
    /// The credential's user.name property. Equivalent to empty if not set.
    /// https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userName: Option<String>,
    /// The credential's user.displayName property. Equivalent to empty if
    /// not set.
    /// https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userDisplayName: Option<String>,
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
    pub enableUI: Option<bool>,
}

impl EnableParams { pub const METHOD: &'static str = "WebAuthn.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "WebAuthn.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "WebAuthn.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "WebAuthn.disable";
    type Response = crate::EmptyReturns;
}

/// Creates and adds a virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddVirtualAuthenticatorParams {

    pub options: VirtualAuthenticatorOptions,
}

/// Creates and adds a virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddVirtualAuthenticatorReturns {

    pub authenticatorId: AuthenticatorId,
}

impl AddVirtualAuthenticatorParams { pub const METHOD: &'static str = "WebAuthn.addVirtualAuthenticator"; }

impl crate::CdpCommand for AddVirtualAuthenticatorParams {
    const METHOD: &'static str = "WebAuthn.addVirtualAuthenticator";
    type Response = AddVirtualAuthenticatorReturns;
}

/// Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetResponseOverrideBitsParams {

    pub authenticatorId: AuthenticatorId,
    /// If isBogusSignature is set, overrides the signature in the authenticator response to be zero.
    /// Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isBogusSignature: Option<bool>,
    /// If isBadUV is set, overrides the UV bit in the flags in the authenticator response to
    /// be zero. Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isBadUV: Option<bool>,
    /// If isBadUP is set, overrides the UP bit in the flags in the authenticator response to
    /// be zero. Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isBadUP: Option<bool>,
}

impl SetResponseOverrideBitsParams { pub const METHOD: &'static str = "WebAuthn.setResponseOverrideBits"; }

impl crate::CdpCommand for SetResponseOverrideBitsParams {
    const METHOD: &'static str = "WebAuthn.setResponseOverrideBits";
    type Response = crate::EmptyReturns;
}

/// Removes the given authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveVirtualAuthenticatorParams {

    pub authenticatorId: AuthenticatorId,
}

impl RemoveVirtualAuthenticatorParams { pub const METHOD: &'static str = "WebAuthn.removeVirtualAuthenticator"; }

impl crate::CdpCommand for RemoveVirtualAuthenticatorParams {
    const METHOD: &'static str = "WebAuthn.removeVirtualAuthenticator";
    type Response = crate::EmptyReturns;
}

/// Adds the credential to the specified authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddCredentialParams {

    pub authenticatorId: AuthenticatorId,

    pub credential: Credential,
}

impl AddCredentialParams { pub const METHOD: &'static str = "WebAuthn.addCredential"; }

impl crate::CdpCommand for AddCredentialParams {
    const METHOD: &'static str = "WebAuthn.addCredential";
    type Response = crate::EmptyReturns;
}

/// Returns a single credential stored in the given virtual authenticator that
/// matches the credential ID.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialParams {

    pub authenticatorId: AuthenticatorId,

    pub credentialId: String,
}

/// Returns a single credential stored in the given virtual authenticator that
/// matches the credential ID.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialReturns {

    pub credential: Credential,
}

impl GetCredentialParams { pub const METHOD: &'static str = "WebAuthn.getCredential"; }

impl crate::CdpCommand for GetCredentialParams {
    const METHOD: &'static str = "WebAuthn.getCredential";
    type Response = GetCredentialReturns;
}

/// Returns all the credentials stored in the given virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialsParams {

    pub authenticatorId: AuthenticatorId,
}

/// Returns all the credentials stored in the given virtual authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialsReturns {

    pub credentials: Vec<Credential>,
}

impl GetCredentialsParams { pub const METHOD: &'static str = "WebAuthn.getCredentials"; }

impl crate::CdpCommand for GetCredentialsParams {
    const METHOD: &'static str = "WebAuthn.getCredentials";
    type Response = GetCredentialsReturns;
}

/// Removes a credential from the authenticator.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCredentialParams {

    pub authenticatorId: AuthenticatorId,

    pub credentialId: String,
}

impl RemoveCredentialParams { pub const METHOD: &'static str = "WebAuthn.removeCredential"; }

impl crate::CdpCommand for RemoveCredentialParams {
    const METHOD: &'static str = "WebAuthn.removeCredential";
    type Response = crate::EmptyReturns;
}

/// Clears all the credentials from the specified device.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCredentialsParams {

    pub authenticatorId: AuthenticatorId,
}

impl ClearCredentialsParams { pub const METHOD: &'static str = "WebAuthn.clearCredentials"; }

impl crate::CdpCommand for ClearCredentialsParams {
    const METHOD: &'static str = "WebAuthn.clearCredentials";
    type Response = crate::EmptyReturns;
}

/// Sets whether User Verification succeeds or fails for an authenticator.
/// The default is true.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetUserVerifiedParams {

    pub authenticatorId: AuthenticatorId,

    pub isUserVerified: bool,
}

impl SetUserVerifiedParams { pub const METHOD: &'static str = "WebAuthn.setUserVerified"; }

impl crate::CdpCommand for SetUserVerifiedParams {
    const METHOD: &'static str = "WebAuthn.setUserVerified";
    type Response = crate::EmptyReturns;
}

/// Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
/// The default is true.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAutomaticPresenceSimulationParams {

    pub authenticatorId: AuthenticatorId,

    pub enabled: bool,
}

impl SetAutomaticPresenceSimulationParams { pub const METHOD: &'static str = "WebAuthn.setAutomaticPresenceSimulation"; }

impl crate::CdpCommand for SetAutomaticPresenceSimulationParams {
    const METHOD: &'static str = "WebAuthn.setAutomaticPresenceSimulation";
    type Response = crate::EmptyReturns;
}

/// Allows setting credential properties.
/// https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCredentialPropertiesParams {

    pub authenticatorId: AuthenticatorId,

    pub credentialId: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backupEligibility: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backupState: Option<bool>,
}

impl SetCredentialPropertiesParams { pub const METHOD: &'static str = "WebAuthn.setCredentialProperties"; }

impl crate::CdpCommand for SetCredentialPropertiesParams {
    const METHOD: &'static str = "WebAuthn.setCredentialProperties";
    type Response = crate::EmptyReturns;
}
