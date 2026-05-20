use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


pub type RegistrationID<'a> = Cow<'a, str>;

/// ServiceWorker registration.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRegistration<'a> {
    registrationId: RegistrationID<'a>,
    scopeURL: Cow<'a, str>,
    isDeleted: bool,
}

impl<'a> ServiceWorkerRegistration<'a> {
    pub fn builder() -> ServiceWorkerRegistrationBuilder<'a> { ServiceWorkerRegistrationBuilder::default() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
    pub fn isDeleted(&self) -> bool { self.isDeleted }
}

#[derive(Default)]
pub struct ServiceWorkerRegistrationBuilder<'a> {
    registrationId: Option<RegistrationID<'a>>,
    scopeURL: Option<Cow<'a, str>>,
    isDeleted: Option<bool>,
}

impl<'a> ServiceWorkerRegistrationBuilder<'a> {
    pub fn registrationId(mut self, registrationId: RegistrationID<'a>) -> Self { self.registrationId = Some(registrationId); self }
    pub fn scopeURL(mut self, scopeURL: impl Into<Cow<'a, str>>) -> Self { self.scopeURL = Some(scopeURL.into()); self }
    pub fn isDeleted(mut self, isDeleted: bool) -> Self { self.isDeleted = Some(isDeleted); self }
    pub fn build(self) -> ServiceWorkerRegistration<'a> {
        ServiceWorkerRegistration {
            registrationId: self.registrationId.unwrap_or_default(),
            scopeURL: self.scopeURL.unwrap_or_default(),
            isDeleted: self.isDeleted.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceWorkerVersionRunningStatus {
    #[default]
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceWorkerVersionStatus {
    #[default]
    #[serde(rename = "new")]
    New,
    #[serde(rename = "installing")]
    Installing,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "activating")]
    Activating,
    #[serde(rename = "activated")]
    Activated,
    #[serde(rename = "redundant")]
    Redundant,
}

/// ServiceWorker version.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerVersion<'a> {
    versionId: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    scriptURL: Cow<'a, str>,
    runningStatus: ServiceWorkerVersionRunningStatus,
    status: ServiceWorkerVersionStatus,
    /// The Last-Modified header value of the main script.
    #[serde(skip_serializing_if = "Option::is_none")]
    scriptLastModified: Option<f64>,
    /// The time at which the response headers of the main script were received from the server.
    /// For cached script it is the last time the cache entry was validated.
    #[serde(skip_serializing_if = "Option::is_none")]
    scriptResponseTime: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    controlledClients: Option<Vec<crate::target::TargetID<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targetId: Option<crate::target::TargetID<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routerRules: Option<Cow<'a, str>>,
}

impl<'a> ServiceWorkerVersion<'a> {
    pub fn builder() -> ServiceWorkerVersionBuilder<'a> { ServiceWorkerVersionBuilder::default() }
    pub fn versionId(&self) -> &str { self.versionId.as_ref() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn scriptURL(&self) -> &str { self.scriptURL.as_ref() }
    pub fn runningStatus(&self) -> &ServiceWorkerVersionRunningStatus { &self.runningStatus }
    pub fn status(&self) -> &ServiceWorkerVersionStatus { &self.status }
    pub fn scriptLastModified(&self) -> Option<f64> { self.scriptLastModified }
    pub fn scriptResponseTime(&self) -> Option<f64> { self.scriptResponseTime }
    pub fn controlledClients(&self) -> Option<&[crate::target::TargetID<'a>]> { self.controlledClients.as_deref() }
    pub fn targetId(&self) -> Option<&crate::target::TargetID<'a>> { self.targetId.as_ref() }
    pub fn routerRules(&self) -> Option<&str> { self.routerRules.as_deref() }
}

#[derive(Default)]
pub struct ServiceWorkerVersionBuilder<'a> {
    versionId: Option<Cow<'a, str>>,
    registrationId: Option<RegistrationID<'a>>,
    scriptURL: Option<Cow<'a, str>>,
    runningStatus: Option<ServiceWorkerVersionRunningStatus>,
    status: Option<ServiceWorkerVersionStatus>,
    scriptLastModified: Option<f64>,
    scriptResponseTime: Option<f64>,
    controlledClients: Option<Vec<crate::target::TargetID<'a>>>,
    targetId: Option<crate::target::TargetID<'a>>,
    routerRules: Option<Cow<'a, str>>,
}

impl<'a> ServiceWorkerVersionBuilder<'a> {
    pub fn versionId(mut self, versionId: impl Into<Cow<'a, str>>) -> Self { self.versionId = Some(versionId.into()); self }
    pub fn registrationId(mut self, registrationId: RegistrationID<'a>) -> Self { self.registrationId = Some(registrationId); self }
    pub fn scriptURL(mut self, scriptURL: impl Into<Cow<'a, str>>) -> Self { self.scriptURL = Some(scriptURL.into()); self }
    pub fn runningStatus(mut self, runningStatus: ServiceWorkerVersionRunningStatus) -> Self { self.runningStatus = Some(runningStatus); self }
    pub fn status(mut self, status: ServiceWorkerVersionStatus) -> Self { self.status = Some(status); self }
    /// The Last-Modified header value of the main script.
    pub fn scriptLastModified(mut self, scriptLastModified: f64) -> Self { self.scriptLastModified = Some(scriptLastModified); self }
    /// The time at which the response headers of the main script were received from the server.
    /// For cached script it is the last time the cache entry was validated.
    pub fn scriptResponseTime(mut self, scriptResponseTime: f64) -> Self { self.scriptResponseTime = Some(scriptResponseTime); self }
    pub fn controlledClients(mut self, controlledClients: Vec<crate::target::TargetID<'a>>) -> Self { self.controlledClients = Some(controlledClients); self }
    pub fn targetId(mut self, targetId: crate::target::TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn routerRules(mut self, routerRules: impl Into<Cow<'a, str>>) -> Self { self.routerRules = Some(routerRules.into()); self }
    pub fn build(self) -> ServiceWorkerVersion<'a> {
        ServiceWorkerVersion {
            versionId: self.versionId.unwrap_or_default(),
            registrationId: self.registrationId.unwrap_or_default(),
            scriptURL: self.scriptURL.unwrap_or_default(),
            runningStatus: self.runningStatus.unwrap_or_default(),
            status: self.status.unwrap_or_default(),
            scriptLastModified: self.scriptLastModified,
            scriptResponseTime: self.scriptResponseTime,
            controlledClients: self.controlledClients,
            targetId: self.targetId,
            routerRules: self.routerRules,
        }
    }
}

/// ServiceWorker error message.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerErrorMessage<'a> {
    errorMessage: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    versionId: Cow<'a, str>,
    sourceURL: Cow<'a, str>,
    lineNumber: i64,
    columnNumber: i64,
}

impl<'a> ServiceWorkerErrorMessage<'a> {
    pub fn builder() -> ServiceWorkerErrorMessageBuilder<'a> { ServiceWorkerErrorMessageBuilder::default() }
    pub fn errorMessage(&self) -> &str { self.errorMessage.as_ref() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn versionId(&self) -> &str { self.versionId.as_ref() }
    pub fn sourceURL(&self) -> &str { self.sourceURL.as_ref() }
    pub fn lineNumber(&self) -> i64 { self.lineNumber }
    pub fn columnNumber(&self) -> i64 { self.columnNumber }
}

#[derive(Default)]
pub struct ServiceWorkerErrorMessageBuilder<'a> {
    errorMessage: Option<Cow<'a, str>>,
    registrationId: Option<RegistrationID<'a>>,
    versionId: Option<Cow<'a, str>>,
    sourceURL: Option<Cow<'a, str>>,
    lineNumber: Option<i64>,
    columnNumber: Option<i64>,
}

impl<'a> ServiceWorkerErrorMessageBuilder<'a> {
    pub fn errorMessage(mut self, errorMessage: impl Into<Cow<'a, str>>) -> Self { self.errorMessage = Some(errorMessage.into()); self }
    pub fn registrationId(mut self, registrationId: RegistrationID<'a>) -> Self { self.registrationId = Some(registrationId); self }
    pub fn versionId(mut self, versionId: impl Into<Cow<'a, str>>) -> Self { self.versionId = Some(versionId.into()); self }
    pub fn sourceURL(mut self, sourceURL: impl Into<Cow<'a, str>>) -> Self { self.sourceURL = Some(sourceURL.into()); self }
    pub fn lineNumber(mut self, lineNumber: i64) -> Self { self.lineNumber = Some(lineNumber); self }
    pub fn columnNumber(mut self, columnNumber: i64) -> Self { self.columnNumber = Some(columnNumber); self }
    pub fn build(self) -> ServiceWorkerErrorMessage<'a> {
        ServiceWorkerErrorMessage {
            errorMessage: self.errorMessage.unwrap_or_default(),
            registrationId: self.registrationId.unwrap_or_default(),
            versionId: self.versionId.unwrap_or_default(),
            sourceURL: self.sourceURL.unwrap_or_default(),
            lineNumber: self.lineNumber.unwrap_or_default(),
            columnNumber: self.columnNumber.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeliverPushMessageParams<'a> {
    origin: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    data: Cow<'a, str>,
}

impl<'a> DeliverPushMessageParams<'a> {
    pub fn builder() -> DeliverPushMessageParamsBuilder<'a> { DeliverPushMessageParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn data(&self) -> &str { self.data.as_ref() }
}

#[derive(Default)]
pub struct DeliverPushMessageParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
    registrationId: Option<RegistrationID<'a>>,
    data: Option<Cow<'a, str>>,
}

impl<'a> DeliverPushMessageParamsBuilder<'a> {
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn registrationId(mut self, registrationId: RegistrationID<'a>) -> Self { self.registrationId = Some(registrationId); self }
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> DeliverPushMessageParams<'a> {
        DeliverPushMessageParams {
            origin: self.origin.unwrap_or_default(),
            registrationId: self.registrationId.unwrap_or_default(),
            data: self.data.unwrap_or_default(),
        }
    }
}

impl<'a> DeliverPushMessageParams<'a> { pub const METHOD: &'static str = "ServiceWorker.deliverPushMessage"; }

impl<'a> crate::CdpCommand<'a> for DeliverPushMessageParams<'a> {
    const METHOD: &'static str = "ServiceWorker.deliverPushMessage";
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

impl DisableParams { pub const METHOD: &'static str = "ServiceWorker.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "ServiceWorker.disable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchSyncEventParams<'a> {
    origin: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    tag: Cow<'a, str>,
    lastChance: bool,
}

impl<'a> DispatchSyncEventParams<'a> {
    pub fn builder() -> DispatchSyncEventParamsBuilder<'a> { DispatchSyncEventParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn tag(&self) -> &str { self.tag.as_ref() }
    pub fn lastChance(&self) -> bool { self.lastChance }
}

#[derive(Default)]
pub struct DispatchSyncEventParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
    registrationId: Option<RegistrationID<'a>>,
    tag: Option<Cow<'a, str>>,
    lastChance: Option<bool>,
}

impl<'a> DispatchSyncEventParamsBuilder<'a> {
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn registrationId(mut self, registrationId: RegistrationID<'a>) -> Self { self.registrationId = Some(registrationId); self }
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self { self.tag = Some(tag.into()); self }
    pub fn lastChance(mut self, lastChance: bool) -> Self { self.lastChance = Some(lastChance); self }
    pub fn build(self) -> DispatchSyncEventParams<'a> {
        DispatchSyncEventParams {
            origin: self.origin.unwrap_or_default(),
            registrationId: self.registrationId.unwrap_or_default(),
            tag: self.tag.unwrap_or_default(),
            lastChance: self.lastChance.unwrap_or_default(),
        }
    }
}

impl<'a> DispatchSyncEventParams<'a> { pub const METHOD: &'static str = "ServiceWorker.dispatchSyncEvent"; }

impl<'a> crate::CdpCommand<'a> for DispatchSyncEventParams<'a> {
    const METHOD: &'static str = "ServiceWorker.dispatchSyncEvent";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchPeriodicSyncEventParams<'a> {
    origin: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    tag: Cow<'a, str>,
}

impl<'a> DispatchPeriodicSyncEventParams<'a> {
    pub fn builder() -> DispatchPeriodicSyncEventParamsBuilder<'a> { DispatchPeriodicSyncEventParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn tag(&self) -> &str { self.tag.as_ref() }
}

#[derive(Default)]
pub struct DispatchPeriodicSyncEventParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
    registrationId: Option<RegistrationID<'a>>,
    tag: Option<Cow<'a, str>>,
}

impl<'a> DispatchPeriodicSyncEventParamsBuilder<'a> {
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn registrationId(mut self, registrationId: RegistrationID<'a>) -> Self { self.registrationId = Some(registrationId); self }
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self { self.tag = Some(tag.into()); self }
    pub fn build(self) -> DispatchPeriodicSyncEventParams<'a> {
        DispatchPeriodicSyncEventParams {
            origin: self.origin.unwrap_or_default(),
            registrationId: self.registrationId.unwrap_or_default(),
            tag: self.tag.unwrap_or_default(),
        }
    }
}

impl<'a> DispatchPeriodicSyncEventParams<'a> { pub const METHOD: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent"; }

impl<'a> crate::CdpCommand<'a> for DispatchPeriodicSyncEventParams<'a> {
    const METHOD: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent";
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

impl EnableParams { pub const METHOD: &'static str = "ServiceWorker.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "ServiceWorker.enable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetForceUpdateOnPageLoadParams {
    forceUpdateOnPageLoad: bool,
}

impl SetForceUpdateOnPageLoadParams {
    pub fn builder() -> SetForceUpdateOnPageLoadParamsBuilder { SetForceUpdateOnPageLoadParamsBuilder::default() }
    pub fn forceUpdateOnPageLoad(&self) -> bool { self.forceUpdateOnPageLoad }
}

#[derive(Default)]
pub struct SetForceUpdateOnPageLoadParamsBuilder {
    forceUpdateOnPageLoad: Option<bool>,
}

impl SetForceUpdateOnPageLoadParamsBuilder {
    pub fn forceUpdateOnPageLoad(mut self, forceUpdateOnPageLoad: bool) -> Self { self.forceUpdateOnPageLoad = Some(forceUpdateOnPageLoad); self }
    pub fn build(self) -> SetForceUpdateOnPageLoadParams {
        SetForceUpdateOnPageLoadParams {
            forceUpdateOnPageLoad: self.forceUpdateOnPageLoad.unwrap_or_default(),
        }
    }
}

impl SetForceUpdateOnPageLoadParams { pub const METHOD: &'static str = "ServiceWorker.setForceUpdateOnPageLoad"; }

impl<'a> crate::CdpCommand<'a> for SetForceUpdateOnPageLoadParams {
    const METHOD: &'static str = "ServiceWorker.setForceUpdateOnPageLoad";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SkipWaitingParams<'a> {
    scopeURL: Cow<'a, str>,
}

impl<'a> SkipWaitingParams<'a> {
    pub fn builder() -> SkipWaitingParamsBuilder<'a> { SkipWaitingParamsBuilder::default() }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
}

#[derive(Default)]
pub struct SkipWaitingParamsBuilder<'a> {
    scopeURL: Option<Cow<'a, str>>,
}

impl<'a> SkipWaitingParamsBuilder<'a> {
    pub fn scopeURL(mut self, scopeURL: impl Into<Cow<'a, str>>) -> Self { self.scopeURL = Some(scopeURL.into()); self }
    pub fn build(self) -> SkipWaitingParams<'a> {
        SkipWaitingParams {
            scopeURL: self.scopeURL.unwrap_or_default(),
        }
    }
}

impl<'a> SkipWaitingParams<'a> { pub const METHOD: &'static str = "ServiceWorker.skipWaiting"; }

impl<'a> crate::CdpCommand<'a> for SkipWaitingParams<'a> {
    const METHOD: &'static str = "ServiceWorker.skipWaiting";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartWorkerParams<'a> {
    scopeURL: Cow<'a, str>,
}

impl<'a> StartWorkerParams<'a> {
    pub fn builder() -> StartWorkerParamsBuilder<'a> { StartWorkerParamsBuilder::default() }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
}

#[derive(Default)]
pub struct StartWorkerParamsBuilder<'a> {
    scopeURL: Option<Cow<'a, str>>,
}

impl<'a> StartWorkerParamsBuilder<'a> {
    pub fn scopeURL(mut self, scopeURL: impl Into<Cow<'a, str>>) -> Self { self.scopeURL = Some(scopeURL.into()); self }
    pub fn build(self) -> StartWorkerParams<'a> {
        StartWorkerParams {
            scopeURL: self.scopeURL.unwrap_or_default(),
        }
    }
}

impl<'a> StartWorkerParams<'a> { pub const METHOD: &'static str = "ServiceWorker.startWorker"; }

impl<'a> crate::CdpCommand<'a> for StartWorkerParams<'a> {
    const METHOD: &'static str = "ServiceWorker.startWorker";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopAllWorkersParams {}

impl StopAllWorkersParams {
    pub fn builder() -> StopAllWorkersParamsBuilder {
        StopAllWorkersParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct StopAllWorkersParamsBuilder {}

impl StopAllWorkersParamsBuilder {
    pub fn build(self) -> StopAllWorkersParams {
        StopAllWorkersParams {}
    }
}

impl StopAllWorkersParams { pub const METHOD: &'static str = "ServiceWorker.stopAllWorkers"; }

impl<'a> crate::CdpCommand<'a> for StopAllWorkersParams {
    const METHOD: &'static str = "ServiceWorker.stopAllWorkers";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopWorkerParams<'a> {
    versionId: Cow<'a, str>,
}

impl<'a> StopWorkerParams<'a> {
    pub fn builder() -> StopWorkerParamsBuilder<'a> { StopWorkerParamsBuilder::default() }
    pub fn versionId(&self) -> &str { self.versionId.as_ref() }
}

#[derive(Default)]
pub struct StopWorkerParamsBuilder<'a> {
    versionId: Option<Cow<'a, str>>,
}

impl<'a> StopWorkerParamsBuilder<'a> {
    pub fn versionId(mut self, versionId: impl Into<Cow<'a, str>>) -> Self { self.versionId = Some(versionId.into()); self }
    pub fn build(self) -> StopWorkerParams<'a> {
        StopWorkerParams {
            versionId: self.versionId.unwrap_or_default(),
        }
    }
}

impl<'a> StopWorkerParams<'a> { pub const METHOD: &'static str = "ServiceWorker.stopWorker"; }

impl<'a> crate::CdpCommand<'a> for StopWorkerParams<'a> {
    const METHOD: &'static str = "ServiceWorker.stopWorker";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnregisterParams<'a> {
    scopeURL: Cow<'a, str>,
}

impl<'a> UnregisterParams<'a> {
    pub fn builder() -> UnregisterParamsBuilder<'a> { UnregisterParamsBuilder::default() }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
}

#[derive(Default)]
pub struct UnregisterParamsBuilder<'a> {
    scopeURL: Option<Cow<'a, str>>,
}

impl<'a> UnregisterParamsBuilder<'a> {
    pub fn scopeURL(mut self, scopeURL: impl Into<Cow<'a, str>>) -> Self { self.scopeURL = Some(scopeURL.into()); self }
    pub fn build(self) -> UnregisterParams<'a> {
        UnregisterParams {
            scopeURL: self.scopeURL.unwrap_or_default(),
        }
    }
}

impl<'a> UnregisterParams<'a> { pub const METHOD: &'static str = "ServiceWorker.unregister"; }

impl<'a> crate::CdpCommand<'a> for UnregisterParams<'a> {
    const METHOD: &'static str = "ServiceWorker.unregister";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRegistrationParams<'a> {
    scopeURL: Cow<'a, str>,
}

impl<'a> UpdateRegistrationParams<'a> {
    pub fn builder() -> UpdateRegistrationParamsBuilder<'a> { UpdateRegistrationParamsBuilder::default() }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
}

#[derive(Default)]
pub struct UpdateRegistrationParamsBuilder<'a> {
    scopeURL: Option<Cow<'a, str>>,
}

impl<'a> UpdateRegistrationParamsBuilder<'a> {
    pub fn scopeURL(mut self, scopeURL: impl Into<Cow<'a, str>>) -> Self { self.scopeURL = Some(scopeURL.into()); self }
    pub fn build(self) -> UpdateRegistrationParams<'a> {
        UpdateRegistrationParams {
            scopeURL: self.scopeURL.unwrap_or_default(),
        }
    }
}

impl<'a> UpdateRegistrationParams<'a> { pub const METHOD: &'static str = "ServiceWorker.updateRegistration"; }

impl<'a> crate::CdpCommand<'a> for UpdateRegistrationParams<'a> {
    const METHOD: &'static str = "ServiceWorker.updateRegistration";
    type Response = crate::EmptyReturns;
}
