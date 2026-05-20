//! This domain allows interacting with the FedCM dialog.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Account<'a> {
    #[serde(rename = "accountId")]
    account_id: Cow<'a, str>,
    email: Cow<'a, str>,
    name: Cow<'a, str>,
    #[serde(rename = "givenName")]
    given_name: Cow<'a, str>,
    #[serde(rename = "pictureUrl")]
    picture_url: Cow<'a, str>,
    #[serde(rename = "idpConfigUrl")]
    idp_config_url: Cow<'a, str>,
    #[serde(rename = "idpLoginUrl")]
    idp_login_url: Cow<'a, str>,
    #[serde(rename = "loginState")]
    login_state: LoginState,
    /// These two are only set if the loginState is signUp
    #[serde(skip_serializing_if = "Option::is_none", rename = "termsOfServiceUrl")]
    terms_of_service_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "privacyPolicyUrl")]
    privacy_policy_url: Option<Cow<'a, str>>,
}

impl<'a> Account<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `account_id`: 
    /// * `email`: 
    /// * `name`: 
    /// * `given_name`: 
    /// * `picture_url`: 
    /// * `idp_config_url`: 
    /// * `idp_login_url`: 
    /// * `login_state`: 
    pub fn builder(account_id: impl Into<Cow<'a, str>>, email: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, given_name: impl Into<Cow<'a, str>>, picture_url: impl Into<Cow<'a, str>>, idp_config_url: impl Into<Cow<'a, str>>, idp_login_url: impl Into<Cow<'a, str>>, login_state: impl Into<LoginState>) -> AccountBuilder<'a> {
        AccountBuilder {
            account_id: account_id.into(),
            email: email.into(),
            name: name.into(),
            given_name: given_name.into(),
            picture_url: picture_url.into(),
            idp_config_url: idp_config_url.into(),
            idp_login_url: idp_login_url.into(),
            login_state: login_state.into(),
            terms_of_service_url: None,
            privacy_policy_url: None,
        }
    }
    pub fn account_id(&self) -> &str { self.account_id.as_ref() }
    pub fn email(&self) -> &str { self.email.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn given_name(&self) -> &str { self.given_name.as_ref() }
    pub fn picture_url(&self) -> &str { self.picture_url.as_ref() }
    pub fn idp_config_url(&self) -> &str { self.idp_config_url.as_ref() }
    pub fn idp_login_url(&self) -> &str { self.idp_login_url.as_ref() }
    pub fn login_state(&self) -> &LoginState { &self.login_state }
    /// These two are only set if the loginState is signUp
    pub fn terms_of_service_url(&self) -> Option<&str> { self.terms_of_service_url.as_deref() }
    pub fn privacy_policy_url(&self) -> Option<&str> { self.privacy_policy_url.as_deref() }
}


pub struct AccountBuilder<'a> {
    account_id: Cow<'a, str>,
    email: Cow<'a, str>,
    name: Cow<'a, str>,
    given_name: Cow<'a, str>,
    picture_url: Cow<'a, str>,
    idp_config_url: Cow<'a, str>,
    idp_login_url: Cow<'a, str>,
    login_state: LoginState,
    terms_of_service_url: Option<Cow<'a, str>>,
    privacy_policy_url: Option<Cow<'a, str>>,
}

impl<'a> AccountBuilder<'a> {
    /// These two are only set if the loginState is signUp
    pub fn terms_of_service_url(mut self, terms_of_service_url: impl Into<Cow<'a, str>>) -> Self { self.terms_of_service_url = Some(terms_of_service_url.into()); self }
    pub fn privacy_policy_url(mut self, privacy_policy_url: impl Into<Cow<'a, str>>) -> Self { self.privacy_policy_url = Some(privacy_policy_url.into()); self }
    pub fn build(self) -> Account<'a> {
        Account {
            account_id: self.account_id,
            email: self.email,
            name: self.name,
            given_name: self.given_name,
            picture_url: self.picture_url,
            idp_config_url: self.idp_config_url,
            idp_login_url: self.idp_login_url,
            login_state: self.login_state,
            terms_of_service_url: self.terms_of_service_url,
            privacy_policy_url: self.privacy_policy_url,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Allows callers to disable the promise rejection delay that would
    /// normally happen, if this is unimportant to what's being tested.
    /// (step 4 of <https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in>)
    #[serde(skip_serializing_if = "Option::is_none", rename = "disableRejectionDelay")]
    disable_rejection_delay: Option<bool>,
}

impl EnableParams {
    /// Creates a builder for this type.
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder {
            disable_rejection_delay: None,
        }
    }
    /// Allows callers to disable the promise rejection delay that would
    /// normally happen, if this is unimportant to what's being tested.
    /// (step 4 of <https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in>)
    pub fn disable_rejection_delay(&self) -> Option<bool> { self.disable_rejection_delay }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    disable_rejection_delay: Option<bool>,
}

impl EnableParamsBuilder {
    /// Allows callers to disable the promise rejection delay that would
    /// normally happen, if this is unimportant to what's being tested.
    /// (step 4 of <https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in>)
    pub fn disable_rejection_delay(mut self, disable_rejection_delay: bool) -> Self { self.disable_rejection_delay = Some(disable_rejection_delay); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            disable_rejection_delay: self.disable_rejection_delay,
        }
    }
}

impl EnableParams { pub const METHOD: &'static str = "FedCm.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "FedCm.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "FedCm.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "FedCm.disable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectAccountParams<'a> {
    #[serde(rename = "dialogId")]
    dialog_id: Cow<'a, str>,
    #[serde(rename = "accountIndex")]
    account_index: u64,
}

impl<'a> SelectAccountParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `dialog_id`: 
    /// * `account_index`: 
    pub fn builder(dialog_id: impl Into<Cow<'a, str>>, account_index: u64) -> SelectAccountParamsBuilder<'a> {
        SelectAccountParamsBuilder {
            dialog_id: dialog_id.into(),
            account_index: account_index,
        }
    }
    pub fn dialog_id(&self) -> &str { self.dialog_id.as_ref() }
    pub fn account_index(&self) -> u64 { self.account_index }
}


pub struct SelectAccountParamsBuilder<'a> {
    dialog_id: Cow<'a, str>,
    account_index: u64,
}

impl<'a> SelectAccountParamsBuilder<'a> {
    pub fn build(self) -> SelectAccountParams<'a> {
        SelectAccountParams {
            dialog_id: self.dialog_id,
            account_index: self.account_index,
        }
    }
}

impl<'a> SelectAccountParams<'a> { pub const METHOD: &'static str = "FedCm.selectAccount"; }

impl<'a> crate::CdpCommand<'a> for SelectAccountParams<'a> {
    const METHOD: &'static str = "FedCm.selectAccount";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClickDialogButtonParams<'a> {
    #[serde(rename = "dialogId")]
    dialog_id: Cow<'a, str>,
    #[serde(rename = "dialogButton")]
    dialog_button: DialogButton,
}

impl<'a> ClickDialogButtonParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `dialog_id`: 
    /// * `dialog_button`: 
    pub fn builder(dialog_id: impl Into<Cow<'a, str>>, dialog_button: impl Into<DialogButton>) -> ClickDialogButtonParamsBuilder<'a> {
        ClickDialogButtonParamsBuilder {
            dialog_id: dialog_id.into(),
            dialog_button: dialog_button.into(),
        }
    }
    pub fn dialog_id(&self) -> &str { self.dialog_id.as_ref() }
    pub fn dialog_button(&self) -> &DialogButton { &self.dialog_button }
}


pub struct ClickDialogButtonParamsBuilder<'a> {
    dialog_id: Cow<'a, str>,
    dialog_button: DialogButton,
}

impl<'a> ClickDialogButtonParamsBuilder<'a> {
    pub fn build(self) -> ClickDialogButtonParams<'a> {
        ClickDialogButtonParams {
            dialog_id: self.dialog_id,
            dialog_button: self.dialog_button,
        }
    }
}

impl<'a> ClickDialogButtonParams<'a> { pub const METHOD: &'static str = "FedCm.clickDialogButton"; }

impl<'a> crate::CdpCommand<'a> for ClickDialogButtonParams<'a> {
    const METHOD: &'static str = "FedCm.clickDialogButton";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenUrlParams<'a> {
    #[serde(rename = "dialogId")]
    dialog_id: Cow<'a, str>,
    #[serde(rename = "accountIndex")]
    account_index: u64,
    #[serde(rename = "accountUrlType")]
    account_url_type: AccountUrlType,
}

impl<'a> OpenUrlParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `dialog_id`: 
    /// * `account_index`: 
    /// * `account_url_type`: 
    pub fn builder(dialog_id: impl Into<Cow<'a, str>>, account_index: u64, account_url_type: impl Into<AccountUrlType>) -> OpenUrlParamsBuilder<'a> {
        OpenUrlParamsBuilder {
            dialog_id: dialog_id.into(),
            account_index: account_index,
            account_url_type: account_url_type.into(),
        }
    }
    pub fn dialog_id(&self) -> &str { self.dialog_id.as_ref() }
    pub fn account_index(&self) -> u64 { self.account_index }
    pub fn account_url_type(&self) -> &AccountUrlType { &self.account_url_type }
}


pub struct OpenUrlParamsBuilder<'a> {
    dialog_id: Cow<'a, str>,
    account_index: u64,
    account_url_type: AccountUrlType,
}

impl<'a> OpenUrlParamsBuilder<'a> {
    pub fn build(self) -> OpenUrlParams<'a> {
        OpenUrlParams {
            dialog_id: self.dialog_id,
            account_index: self.account_index,
            account_url_type: self.account_url_type,
        }
    }
}

impl<'a> OpenUrlParams<'a> { pub const METHOD: &'static str = "FedCm.openUrl"; }

impl<'a> crate::CdpCommand<'a> for OpenUrlParams<'a> {
    const METHOD: &'static str = "FedCm.openUrl";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DismissDialogParams<'a> {
    #[serde(rename = "dialogId")]
    dialog_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "triggerCooldown")]
    trigger_cooldown: Option<bool>,
}

impl<'a> DismissDialogParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `dialog_id`: 
    pub fn builder(dialog_id: impl Into<Cow<'a, str>>) -> DismissDialogParamsBuilder<'a> {
        DismissDialogParamsBuilder {
            dialog_id: dialog_id.into(),
            trigger_cooldown: None,
        }
    }
    pub fn dialog_id(&self) -> &str { self.dialog_id.as_ref() }
    pub fn trigger_cooldown(&self) -> Option<bool> { self.trigger_cooldown }
}


pub struct DismissDialogParamsBuilder<'a> {
    dialog_id: Cow<'a, str>,
    trigger_cooldown: Option<bool>,
}

impl<'a> DismissDialogParamsBuilder<'a> {
    pub fn trigger_cooldown(mut self, trigger_cooldown: bool) -> Self { self.trigger_cooldown = Some(trigger_cooldown); self }
    pub fn build(self) -> DismissDialogParams<'a> {
        DismissDialogParams {
            dialog_id: self.dialog_id,
            trigger_cooldown: self.trigger_cooldown,
        }
    }
}

impl<'a> DismissDialogParams<'a> { pub const METHOD: &'static str = "FedCm.dismissDialog"; }

impl<'a> crate::CdpCommand<'a> for DismissDialogParams<'a> {
    const METHOD: &'static str = "FedCm.dismissDialog";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResetCooldownParams {}

impl ResetCooldownParams { pub const METHOD: &'static str = "FedCm.resetCooldown"; }

impl<'a> crate::CdpCommand<'a> for ResetCooldownParams {
    const METHOD: &'static str = "FedCm.resetCooldown";
    type Response = crate::EmptyReturns;
}
