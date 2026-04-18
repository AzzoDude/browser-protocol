//! This domain allows interacting with the FedCM dialog.

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Whether this is a sign-up or sign-in action for this account, i.e.
/// whether this account has ever been used to sign in to this RP before.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum LoginState {
    #[default]
    SignIn,
    SignUp,
}

/// The types of FedCM dialogs.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DialogType {
    #[default]
    AccountChooser,
    AutoReauthn,
    ConfirmIdpLogin,
    Error,
}

/// The buttons on the FedCM dialog.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DialogButton {
    #[default]
    ConfirmIdpLoginContinue,
    ErrorGotIt,
    ErrorMoreDetails,
}

/// The URLs that each account has

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AccountUrlType {
    #[default]
    TermsOfService,
    PrivacyPolicy,
}

/// Corresponds to IdentityRequestAccount

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Account {

    pub accountId: String,

    pub email: String,

    pub name: String,

    pub givenName: String,

    pub pictureUrl: String,

    pub idpConfigUrl: String,

    pub idpLoginUrl: String,

    pub loginState: LoginState,
    /// These two are only set if the loginState is signUp

    #[serde(skip_serializing_if = "Option::is_none")]
    pub termsOfServiceUrl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacyPolicyUrl: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Allows callers to disable the promise rejection delay that would
    /// normally happen, if this is unimportant to what's being tested.
    /// (step 4 of <https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in>)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disableRejectionDelay: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectAccountParams {

    pub dialogId: String,

    pub accountIndex: u64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClickDialogButtonParams {

    pub dialogId: String,

    pub dialogButton: DialogButton,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenUrlParams {

    pub dialogId: String,

    pub accountIndex: u64,

    pub accountUrlType: AccountUrlType,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DismissDialogParams {

    pub dialogId: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggerCooldown: Option<bool>,
}
