//! This domain allows interacting with the FedCM dialog.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Whether this is a sign-up or sign-in action for this account, i.e.
/// whether this account has ever been used to sign in to this RP before.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum LoginState {
    #[default]
    #[serde(rename = "SignIn")]
    SignIn,
    #[serde(rename = "SignUp")]
    SignUp,
}

/// The types of FedCM dialogs.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DialogType {
    #[default]
    #[serde(rename = "AccountChooser")]
    AccountChooser,
    #[serde(rename = "AutoReauthn")]
    AutoReauthn,
    #[serde(rename = "ConfirmIdpLogin")]
    ConfirmIdpLogin,
    #[serde(rename = "Error")]
    Error,
}

/// The buttons on the FedCM dialog.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DialogButton {
    #[default]
    #[serde(rename = "ConfirmIdpLoginContinue")]
    ConfirmIdpLoginContinue,
    #[serde(rename = "ErrorGotIt")]
    ErrorGotIt,
    #[serde(rename = "ErrorMoreDetails")]
    ErrorMoreDetails,
}

/// The URLs that each account has

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AccountUrlType {
    #[default]
    #[serde(rename = "TermsOfService")]
    TermsOfService,
    #[serde(rename = "PrivacyPolicy")]
    PrivacyPolicy,
}

/// Corresponds to IdentityRequestAccount

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Account<'a> {
    #[serde(rename = "accountId")]
    pub account_id: Cow<'a, str>,
    pub email: Cow<'a, str>,
    pub name: Cow<'a, str>,
    #[serde(rename = "givenName")]
    pub given_name: Cow<'a, str>,
    #[serde(rename = "pictureUrl")]
    pub picture_url: Cow<'a, str>,
    #[serde(rename = "idpConfigUrl")]
    pub idp_config_url: Cow<'a, str>,
    #[serde(rename = "idpLoginUrl")]
    pub idp_login_url: Cow<'a, str>,
    #[serde(rename = "loginState")]
    pub login_state: LoginState,
    /// These two are only set if the loginState is signUp
    #[serde(skip_serializing_if = "Option::is_none", rename = "termsOfServiceUrl")]
    pub terms_of_service_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "privacyPolicyUrl")]
    pub privacy_policy_url: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FedCm.enable")]
pub struct EnableParams {
    /// Allows callers to disable the promise rejection delay that would
    /// normally happen, if this is unimportant to what's being tested.
    /// (step 4 of <https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in>)
    #[serde(skip_serializing_if = "Option::is_none", rename = "disableRejectionDelay")]
    pub disable_rejection_delay: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FedCm.disable")]
pub struct DisableParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FedCm.selectAccount")]
pub struct SelectAccountParams<'a> {
    #[serde(rename = "dialogId")]
    pub dialog_id: Cow<'a, str>,
    #[serde(rename = "accountIndex")]
    pub account_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FedCm.clickDialogButton")]
pub struct ClickDialogButtonParams<'a> {
    #[serde(rename = "dialogId")]
    pub dialog_id: Cow<'a, str>,
    #[serde(rename = "dialogButton")]
    pub dialog_button: DialogButton,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FedCm.openUrl")]
pub struct OpenUrlParams<'a> {
    #[serde(rename = "dialogId")]
    pub dialog_id: Cow<'a, str>,
    #[serde(rename = "accountIndex")]
    pub account_index: u64,
    #[serde(rename = "accountUrlType")]
    pub account_url_type: AccountUrlType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FedCm.dismissDialog")]
pub struct DismissDialogParams<'a> {
    #[serde(rename = "dialogId")]
    pub dialog_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "triggerCooldown")]
    pub trigger_cooldown: Option<bool>,
}
/// Resets the cooldown time, if any, to allow the next FedCM call to show
/// a dialog even if one was recently dismissed by the user.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FedCm.resetCooldown")]
pub struct ResetCooldownParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FedCm.dialogShown")]
pub struct DialogShown<'a> {
    #[serde(rename = "dialogId")]
    pub dialog_id: Cow<'a, str>,
    #[serde(rename = "dialogType")]
    pub dialog_type: DialogType,
    pub accounts: Vec<Account<'a>>,
    /// These exist primarily so that the caller can verify the
    /// RP context was used appropriately.
    pub title: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<Cow<'a, str>>,
}
/// Triggered when a dialog is closed, either by user action, JS abort,
/// or a command below.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FedCm.dialogClosed")]
pub struct DialogClosed<'a> {
    #[serde(rename = "dialogId")]
    pub dialog_id: Cow<'a, str>,
}