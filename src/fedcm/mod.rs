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
    accountId: Cow<'a, str>,
    email: Cow<'a, str>,
    name: Cow<'a, str>,
    givenName: Cow<'a, str>,
    pictureUrl: Cow<'a, str>,
    idpConfigUrl: Cow<'a, str>,
    idpLoginUrl: Cow<'a, str>,
    loginState: LoginState,
    /// These two are only set if the loginState is signUp
    #[serde(skip_serializing_if = "Option::is_none")]
    termsOfServiceUrl: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacyPolicyUrl: Option<Cow<'a, str>>,
}

impl<'a> Account<'a> {
    pub fn builder(accountId: impl Into<Cow<'a, str>>, email: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, givenName: impl Into<Cow<'a, str>>, pictureUrl: impl Into<Cow<'a, str>>, idpConfigUrl: impl Into<Cow<'a, str>>, idpLoginUrl: impl Into<Cow<'a, str>>, loginState: impl Into<LoginState>) -> AccountBuilder<'a> {
        AccountBuilder {
            accountId: accountId.into(),
            email: email.into(),
            name: name.into(),
            givenName: givenName.into(),
            pictureUrl: pictureUrl.into(),
            idpConfigUrl: idpConfigUrl.into(),
            idpLoginUrl: idpLoginUrl.into(),
            loginState: loginState.into(),
            termsOfServiceUrl: None,
            privacyPolicyUrl: None,
        }
    }
    pub fn accountId(&self) -> &str { self.accountId.as_ref() }
    pub fn email(&self) -> &str { self.email.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn givenName(&self) -> &str { self.givenName.as_ref() }
    pub fn pictureUrl(&self) -> &str { self.pictureUrl.as_ref() }
    pub fn idpConfigUrl(&self) -> &str { self.idpConfigUrl.as_ref() }
    pub fn idpLoginUrl(&self) -> &str { self.idpLoginUrl.as_ref() }
    pub fn loginState(&self) -> &LoginState { &self.loginState }
    pub fn termsOfServiceUrl(&self) -> Option<&str> { self.termsOfServiceUrl.as_deref() }
    pub fn privacyPolicyUrl(&self) -> Option<&str> { self.privacyPolicyUrl.as_deref() }
}


pub struct AccountBuilder<'a> {
    accountId: Cow<'a, str>,
    email: Cow<'a, str>,
    name: Cow<'a, str>,
    givenName: Cow<'a, str>,
    pictureUrl: Cow<'a, str>,
    idpConfigUrl: Cow<'a, str>,
    idpLoginUrl: Cow<'a, str>,
    loginState: LoginState,
    termsOfServiceUrl: Option<Cow<'a, str>>,
    privacyPolicyUrl: Option<Cow<'a, str>>,
}

impl<'a> AccountBuilder<'a> {
    /// These two are only set if the loginState is signUp
    pub fn termsOfServiceUrl(mut self, termsOfServiceUrl: impl Into<Cow<'a, str>>) -> Self { self.termsOfServiceUrl = Some(termsOfServiceUrl.into()); self }
    pub fn privacyPolicyUrl(mut self, privacyPolicyUrl: impl Into<Cow<'a, str>>) -> Self { self.privacyPolicyUrl = Some(privacyPolicyUrl.into()); self }
    pub fn build(self) -> Account<'a> {
        Account {
            accountId: self.accountId,
            email: self.email,
            name: self.name,
            givenName: self.givenName,
            pictureUrl: self.pictureUrl,
            idpConfigUrl: self.idpConfigUrl,
            idpLoginUrl: self.idpLoginUrl,
            loginState: self.loginState,
            termsOfServiceUrl: self.termsOfServiceUrl,
            privacyPolicyUrl: self.privacyPolicyUrl,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Allows callers to disable the promise rejection delay that would
    /// normally happen, if this is unimportant to what's being tested.
    /// (step 4 of https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in)
    #[serde(skip_serializing_if = "Option::is_none")]
    disableRejectionDelay: Option<bool>,
}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder {
            disableRejectionDelay: None,
        }
    }
    pub fn disableRejectionDelay(&self) -> Option<bool> { self.disableRejectionDelay }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    disableRejectionDelay: Option<bool>,
}

impl EnableParamsBuilder {
    /// Allows callers to disable the promise rejection delay that would
    /// normally happen, if this is unimportant to what's being tested.
    /// (step 4 of https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in)
    pub fn disableRejectionDelay(mut self, disableRejectionDelay: bool) -> Self { self.disableRejectionDelay = Some(disableRejectionDelay); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            disableRejectionDelay: self.disableRejectionDelay,
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
    dialogId: Cow<'a, str>,
    accountIndex: u64,
}

impl<'a> SelectAccountParams<'a> {
    pub fn builder(dialogId: impl Into<Cow<'a, str>>, accountIndex: u64) -> SelectAccountParamsBuilder<'a> {
        SelectAccountParamsBuilder {
            dialogId: dialogId.into(),
            accountIndex: accountIndex,
        }
    }
    pub fn dialogId(&self) -> &str { self.dialogId.as_ref() }
    pub fn accountIndex(&self) -> u64 { self.accountIndex }
}


pub struct SelectAccountParamsBuilder<'a> {
    dialogId: Cow<'a, str>,
    accountIndex: u64,
}

impl<'a> SelectAccountParamsBuilder<'a> {
    pub fn build(self) -> SelectAccountParams<'a> {
        SelectAccountParams {
            dialogId: self.dialogId,
            accountIndex: self.accountIndex,
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
    dialogId: Cow<'a, str>,
    dialogButton: DialogButton,
}

impl<'a> ClickDialogButtonParams<'a> {
    pub fn builder(dialogId: impl Into<Cow<'a, str>>, dialogButton: impl Into<DialogButton>) -> ClickDialogButtonParamsBuilder<'a> {
        ClickDialogButtonParamsBuilder {
            dialogId: dialogId.into(),
            dialogButton: dialogButton.into(),
        }
    }
    pub fn dialogId(&self) -> &str { self.dialogId.as_ref() }
    pub fn dialogButton(&self) -> &DialogButton { &self.dialogButton }
}


pub struct ClickDialogButtonParamsBuilder<'a> {
    dialogId: Cow<'a, str>,
    dialogButton: DialogButton,
}

impl<'a> ClickDialogButtonParamsBuilder<'a> {
    pub fn build(self) -> ClickDialogButtonParams<'a> {
        ClickDialogButtonParams {
            dialogId: self.dialogId,
            dialogButton: self.dialogButton,
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
    dialogId: Cow<'a, str>,
    accountIndex: u64,
    accountUrlType: AccountUrlType,
}

impl<'a> OpenUrlParams<'a> {
    pub fn builder(dialogId: impl Into<Cow<'a, str>>, accountIndex: u64, accountUrlType: impl Into<AccountUrlType>) -> OpenUrlParamsBuilder<'a> {
        OpenUrlParamsBuilder {
            dialogId: dialogId.into(),
            accountIndex: accountIndex,
            accountUrlType: accountUrlType.into(),
        }
    }
    pub fn dialogId(&self) -> &str { self.dialogId.as_ref() }
    pub fn accountIndex(&self) -> u64 { self.accountIndex }
    pub fn accountUrlType(&self) -> &AccountUrlType { &self.accountUrlType }
}


pub struct OpenUrlParamsBuilder<'a> {
    dialogId: Cow<'a, str>,
    accountIndex: u64,
    accountUrlType: AccountUrlType,
}

impl<'a> OpenUrlParamsBuilder<'a> {
    pub fn build(self) -> OpenUrlParams<'a> {
        OpenUrlParams {
            dialogId: self.dialogId,
            accountIndex: self.accountIndex,
            accountUrlType: self.accountUrlType,
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
    dialogId: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggerCooldown: Option<bool>,
}

impl<'a> DismissDialogParams<'a> {
    pub fn builder(dialogId: impl Into<Cow<'a, str>>) -> DismissDialogParamsBuilder<'a> {
        DismissDialogParamsBuilder {
            dialogId: dialogId.into(),
            triggerCooldown: None,
        }
    }
    pub fn dialogId(&self) -> &str { self.dialogId.as_ref() }
    pub fn triggerCooldown(&self) -> Option<bool> { self.triggerCooldown }
}


pub struct DismissDialogParamsBuilder<'a> {
    dialogId: Cow<'a, str>,
    triggerCooldown: Option<bool>,
}

impl<'a> DismissDialogParamsBuilder<'a> {
    pub fn triggerCooldown(mut self, triggerCooldown: bool) -> Self { self.triggerCooldown = Some(triggerCooldown); self }
    pub fn build(self) -> DismissDialogParams<'a> {
        DismissDialogParams {
            dialogId: self.dialogId,
            triggerCooldown: self.triggerCooldown,
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
