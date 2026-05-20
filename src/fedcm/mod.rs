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
    pub fn builder() -> AccountBuilder<'a> { AccountBuilder::default() }
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

#[derive(Default)]
pub struct AccountBuilder<'a> {
    accountId: Option<Cow<'a, str>>,
    email: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    givenName: Option<Cow<'a, str>>,
    pictureUrl: Option<Cow<'a, str>>,
    idpConfigUrl: Option<Cow<'a, str>>,
    idpLoginUrl: Option<Cow<'a, str>>,
    loginState: Option<LoginState>,
    termsOfServiceUrl: Option<Cow<'a, str>>,
    privacyPolicyUrl: Option<Cow<'a, str>>,
}

impl<'a> AccountBuilder<'a> {
    pub fn accountId(mut self, accountId: impl Into<Cow<'a, str>>) -> Self { self.accountId = Some(accountId.into()); self }
    pub fn email(mut self, email: impl Into<Cow<'a, str>>) -> Self { self.email = Some(email.into()); self }
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn givenName(mut self, givenName: impl Into<Cow<'a, str>>) -> Self { self.givenName = Some(givenName.into()); self }
    pub fn pictureUrl(mut self, pictureUrl: impl Into<Cow<'a, str>>) -> Self { self.pictureUrl = Some(pictureUrl.into()); self }
    pub fn idpConfigUrl(mut self, idpConfigUrl: impl Into<Cow<'a, str>>) -> Self { self.idpConfigUrl = Some(idpConfigUrl.into()); self }
    pub fn idpLoginUrl(mut self, idpLoginUrl: impl Into<Cow<'a, str>>) -> Self { self.idpLoginUrl = Some(idpLoginUrl.into()); self }
    pub fn loginState(mut self, loginState: LoginState) -> Self { self.loginState = Some(loginState); self }
    /// These two are only set if the loginState is signUp
    pub fn termsOfServiceUrl(mut self, termsOfServiceUrl: impl Into<Cow<'a, str>>) -> Self { self.termsOfServiceUrl = Some(termsOfServiceUrl.into()); self }
    pub fn privacyPolicyUrl(mut self, privacyPolicyUrl: impl Into<Cow<'a, str>>) -> Self { self.privacyPolicyUrl = Some(privacyPolicyUrl.into()); self }
    pub fn build(self) -> Account<'a> {
        Account {
            accountId: self.accountId.unwrap_or_default(),
            email: self.email.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            givenName: self.givenName.unwrap_or_default(),
            pictureUrl: self.pictureUrl.unwrap_or_default(),
            idpConfigUrl: self.idpConfigUrl.unwrap_or_default(),
            idpLoginUrl: self.idpLoginUrl.unwrap_or_default(),
            loginState: self.loginState.unwrap_or_default(),
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
    pub fn builder() -> EnableParamsBuilder { EnableParamsBuilder::default() }
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
    pub fn builder() -> SelectAccountParamsBuilder<'a> { SelectAccountParamsBuilder::default() }
    pub fn dialogId(&self) -> &str { self.dialogId.as_ref() }
    pub fn accountIndex(&self) -> u64 { self.accountIndex }
}

#[derive(Default)]
pub struct SelectAccountParamsBuilder<'a> {
    dialogId: Option<Cow<'a, str>>,
    accountIndex: Option<u64>,
}

impl<'a> SelectAccountParamsBuilder<'a> {
    pub fn dialogId(mut self, dialogId: impl Into<Cow<'a, str>>) -> Self { self.dialogId = Some(dialogId.into()); self }
    pub fn accountIndex(mut self, accountIndex: u64) -> Self { self.accountIndex = Some(accountIndex); self }
    pub fn build(self) -> SelectAccountParams<'a> {
        SelectAccountParams {
            dialogId: self.dialogId.unwrap_or_default(),
            accountIndex: self.accountIndex.unwrap_or_default(),
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
    pub fn builder() -> ClickDialogButtonParamsBuilder<'a> { ClickDialogButtonParamsBuilder::default() }
    pub fn dialogId(&self) -> &str { self.dialogId.as_ref() }
    pub fn dialogButton(&self) -> &DialogButton { &self.dialogButton }
}

#[derive(Default)]
pub struct ClickDialogButtonParamsBuilder<'a> {
    dialogId: Option<Cow<'a, str>>,
    dialogButton: Option<DialogButton>,
}

impl<'a> ClickDialogButtonParamsBuilder<'a> {
    pub fn dialogId(mut self, dialogId: impl Into<Cow<'a, str>>) -> Self { self.dialogId = Some(dialogId.into()); self }
    pub fn dialogButton(mut self, dialogButton: DialogButton) -> Self { self.dialogButton = Some(dialogButton); self }
    pub fn build(self) -> ClickDialogButtonParams<'a> {
        ClickDialogButtonParams {
            dialogId: self.dialogId.unwrap_or_default(),
            dialogButton: self.dialogButton.unwrap_or_default(),
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
    pub fn builder() -> OpenUrlParamsBuilder<'a> { OpenUrlParamsBuilder::default() }
    pub fn dialogId(&self) -> &str { self.dialogId.as_ref() }
    pub fn accountIndex(&self) -> u64 { self.accountIndex }
    pub fn accountUrlType(&self) -> &AccountUrlType { &self.accountUrlType }
}

#[derive(Default)]
pub struct OpenUrlParamsBuilder<'a> {
    dialogId: Option<Cow<'a, str>>,
    accountIndex: Option<u64>,
    accountUrlType: Option<AccountUrlType>,
}

impl<'a> OpenUrlParamsBuilder<'a> {
    pub fn dialogId(mut self, dialogId: impl Into<Cow<'a, str>>) -> Self { self.dialogId = Some(dialogId.into()); self }
    pub fn accountIndex(mut self, accountIndex: u64) -> Self { self.accountIndex = Some(accountIndex); self }
    pub fn accountUrlType(mut self, accountUrlType: AccountUrlType) -> Self { self.accountUrlType = Some(accountUrlType); self }
    pub fn build(self) -> OpenUrlParams<'a> {
        OpenUrlParams {
            dialogId: self.dialogId.unwrap_or_default(),
            accountIndex: self.accountIndex.unwrap_or_default(),
            accountUrlType: self.accountUrlType.unwrap_or_default(),
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
    pub fn builder() -> DismissDialogParamsBuilder<'a> { DismissDialogParamsBuilder::default() }
    pub fn dialogId(&self) -> &str { self.dialogId.as_ref() }
    pub fn triggerCooldown(&self) -> Option<bool> { self.triggerCooldown }
}

#[derive(Default)]
pub struct DismissDialogParamsBuilder<'a> {
    dialogId: Option<Cow<'a, str>>,
    triggerCooldown: Option<bool>,
}

impl<'a> DismissDialogParamsBuilder<'a> {
    pub fn dialogId(mut self, dialogId: impl Into<Cow<'a, str>>) -> Self { self.dialogId = Some(dialogId.into()); self }
    pub fn triggerCooldown(mut self, triggerCooldown: bool) -> Self { self.triggerCooldown = Some(triggerCooldown); self }
    pub fn build(self) -> DismissDialogParams<'a> {
        DismissDialogParams {
            dialogId: self.dialogId.unwrap_or_default(),
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

impl ResetCooldownParams {
    pub fn builder() -> ResetCooldownParamsBuilder {
        ResetCooldownParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ResetCooldownParamsBuilder {}

impl ResetCooldownParamsBuilder {
    pub fn build(self) -> ResetCooldownParams {
        ResetCooldownParams {}
    }
}

impl ResetCooldownParams { pub const METHOD: &'static str = "FedCm.resetCooldown"; }

impl<'a> crate::CdpCommand<'a> for ResetCooldownParams {
    const METHOD: &'static str = "FedCm.resetCooldown";
    type Response = crate::EmptyReturns;
}
