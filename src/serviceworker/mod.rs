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
    pub fn builder(registrationId: impl Into<RegistrationID<'a>>, scopeURL: impl Into<Cow<'a, str>>, isDeleted: bool) -> ServiceWorkerRegistrationBuilder<'a> {
        ServiceWorkerRegistrationBuilder {
            registrationId: registrationId.into(),
            scopeURL: scopeURL.into(),
            isDeleted: isDeleted,
        }
    }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
    pub fn isDeleted(&self) -> bool { self.isDeleted }
}


pub struct ServiceWorkerRegistrationBuilder<'a> {
    registrationId: RegistrationID<'a>,
    scopeURL: Cow<'a, str>,
    isDeleted: bool,
}

impl<'a> ServiceWorkerRegistrationBuilder<'a> {
    pub fn build(self) -> ServiceWorkerRegistration<'a> {
        ServiceWorkerRegistration {
            registrationId: self.registrationId,
            scopeURL: self.scopeURL,
            isDeleted: self.isDeleted,
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
    pub fn builder(versionId: impl Into<Cow<'a, str>>, registrationId: impl Into<RegistrationID<'a>>, scriptURL: impl Into<Cow<'a, str>>, runningStatus: impl Into<ServiceWorkerVersionRunningStatus>, status: impl Into<ServiceWorkerVersionStatus>) -> ServiceWorkerVersionBuilder<'a> {
        ServiceWorkerVersionBuilder {
            versionId: versionId.into(),
            registrationId: registrationId.into(),
            scriptURL: scriptURL.into(),
            runningStatus: runningStatus.into(),
            status: status.into(),
            scriptLastModified: None,
            scriptResponseTime: None,
            controlledClients: None,
            targetId: None,
            routerRules: None,
        }
    }
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


pub struct ServiceWorkerVersionBuilder<'a> {
    versionId: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    scriptURL: Cow<'a, str>,
    runningStatus: ServiceWorkerVersionRunningStatus,
    status: ServiceWorkerVersionStatus,
    scriptLastModified: Option<f64>,
    scriptResponseTime: Option<f64>,
    controlledClients: Option<Vec<crate::target::TargetID<'a>>>,
    targetId: Option<crate::target::TargetID<'a>>,
    routerRules: Option<Cow<'a, str>>,
}

impl<'a> ServiceWorkerVersionBuilder<'a> {
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
            versionId: self.versionId,
            registrationId: self.registrationId,
            scriptURL: self.scriptURL,
            runningStatus: self.runningStatus,
            status: self.status,
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
    pub fn builder(errorMessage: impl Into<Cow<'a, str>>, registrationId: impl Into<RegistrationID<'a>>, versionId: impl Into<Cow<'a, str>>, sourceURL: impl Into<Cow<'a, str>>, lineNumber: i64, columnNumber: i64) -> ServiceWorkerErrorMessageBuilder<'a> {
        ServiceWorkerErrorMessageBuilder {
            errorMessage: errorMessage.into(),
            registrationId: registrationId.into(),
            versionId: versionId.into(),
            sourceURL: sourceURL.into(),
            lineNumber: lineNumber,
            columnNumber: columnNumber,
        }
    }
    pub fn errorMessage(&self) -> &str { self.errorMessage.as_ref() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn versionId(&self) -> &str { self.versionId.as_ref() }
    pub fn sourceURL(&self) -> &str { self.sourceURL.as_ref() }
    pub fn lineNumber(&self) -> i64 { self.lineNumber }
    pub fn columnNumber(&self) -> i64 { self.columnNumber }
}


pub struct ServiceWorkerErrorMessageBuilder<'a> {
    errorMessage: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    versionId: Cow<'a, str>,
    sourceURL: Cow<'a, str>,
    lineNumber: i64,
    columnNumber: i64,
}

impl<'a> ServiceWorkerErrorMessageBuilder<'a> {
    pub fn build(self) -> ServiceWorkerErrorMessage<'a> {
        ServiceWorkerErrorMessage {
            errorMessage: self.errorMessage,
            registrationId: self.registrationId,
            versionId: self.versionId,
            sourceURL: self.sourceURL,
            lineNumber: self.lineNumber,
            columnNumber: self.columnNumber,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>, registrationId: impl Into<RegistrationID<'a>>, data: impl Into<Cow<'a, str>>) -> DeliverPushMessageParamsBuilder<'a> {
        DeliverPushMessageParamsBuilder {
            origin: origin.into(),
            registrationId: registrationId.into(),
            data: data.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn data(&self) -> &str { self.data.as_ref() }
}


pub struct DeliverPushMessageParamsBuilder<'a> {
    origin: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    data: Cow<'a, str>,
}

impl<'a> DeliverPushMessageParamsBuilder<'a> {
    pub fn build(self) -> DeliverPushMessageParams<'a> {
        DeliverPushMessageParams {
            origin: self.origin,
            registrationId: self.registrationId,
            data: self.data,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>, registrationId: impl Into<RegistrationID<'a>>, tag: impl Into<Cow<'a, str>>, lastChance: bool) -> DispatchSyncEventParamsBuilder<'a> {
        DispatchSyncEventParamsBuilder {
            origin: origin.into(),
            registrationId: registrationId.into(),
            tag: tag.into(),
            lastChance: lastChance,
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn tag(&self) -> &str { self.tag.as_ref() }
    pub fn lastChance(&self) -> bool { self.lastChance }
}


pub struct DispatchSyncEventParamsBuilder<'a> {
    origin: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    tag: Cow<'a, str>,
    lastChance: bool,
}

impl<'a> DispatchSyncEventParamsBuilder<'a> {
    pub fn build(self) -> DispatchSyncEventParams<'a> {
        DispatchSyncEventParams {
            origin: self.origin,
            registrationId: self.registrationId,
            tag: self.tag,
            lastChance: self.lastChance,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>, registrationId: impl Into<RegistrationID<'a>>, tag: impl Into<Cow<'a, str>>) -> DispatchPeriodicSyncEventParamsBuilder<'a> {
        DispatchPeriodicSyncEventParamsBuilder {
            origin: origin.into(),
            registrationId: registrationId.into(),
            tag: tag.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn registrationId(&self) -> &RegistrationID<'a> { &self.registrationId }
    pub fn tag(&self) -> &str { self.tag.as_ref() }
}


pub struct DispatchPeriodicSyncEventParamsBuilder<'a> {
    origin: Cow<'a, str>,
    registrationId: RegistrationID<'a>,
    tag: Cow<'a, str>,
}

impl<'a> DispatchPeriodicSyncEventParamsBuilder<'a> {
    pub fn build(self) -> DispatchPeriodicSyncEventParams<'a> {
        DispatchPeriodicSyncEventParams {
            origin: self.origin,
            registrationId: self.registrationId,
            tag: self.tag,
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
    pub fn builder(forceUpdateOnPageLoad: bool) -> SetForceUpdateOnPageLoadParamsBuilder {
        SetForceUpdateOnPageLoadParamsBuilder {
            forceUpdateOnPageLoad: forceUpdateOnPageLoad,
        }
    }
    pub fn forceUpdateOnPageLoad(&self) -> bool { self.forceUpdateOnPageLoad }
}


pub struct SetForceUpdateOnPageLoadParamsBuilder {
    forceUpdateOnPageLoad: bool,
}

impl SetForceUpdateOnPageLoadParamsBuilder {
    pub fn build(self) -> SetForceUpdateOnPageLoadParams {
        SetForceUpdateOnPageLoadParams {
            forceUpdateOnPageLoad: self.forceUpdateOnPageLoad,
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
    pub fn builder(scopeURL: impl Into<Cow<'a, str>>) -> SkipWaitingParamsBuilder<'a> {
        SkipWaitingParamsBuilder {
            scopeURL: scopeURL.into(),
        }
    }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
}


pub struct SkipWaitingParamsBuilder<'a> {
    scopeURL: Cow<'a, str>,
}

impl<'a> SkipWaitingParamsBuilder<'a> {
    pub fn build(self) -> SkipWaitingParams<'a> {
        SkipWaitingParams {
            scopeURL: self.scopeURL,
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
    pub fn builder(scopeURL: impl Into<Cow<'a, str>>) -> StartWorkerParamsBuilder<'a> {
        StartWorkerParamsBuilder {
            scopeURL: scopeURL.into(),
        }
    }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
}


pub struct StartWorkerParamsBuilder<'a> {
    scopeURL: Cow<'a, str>,
}

impl<'a> StartWorkerParamsBuilder<'a> {
    pub fn build(self) -> StartWorkerParams<'a> {
        StartWorkerParams {
            scopeURL: self.scopeURL,
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
    pub fn builder(versionId: impl Into<Cow<'a, str>>) -> StopWorkerParamsBuilder<'a> {
        StopWorkerParamsBuilder {
            versionId: versionId.into(),
        }
    }
    pub fn versionId(&self) -> &str { self.versionId.as_ref() }
}


pub struct StopWorkerParamsBuilder<'a> {
    versionId: Cow<'a, str>,
}

impl<'a> StopWorkerParamsBuilder<'a> {
    pub fn build(self) -> StopWorkerParams<'a> {
        StopWorkerParams {
            versionId: self.versionId,
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
    pub fn builder(scopeURL: impl Into<Cow<'a, str>>) -> UnregisterParamsBuilder<'a> {
        UnregisterParamsBuilder {
            scopeURL: scopeURL.into(),
        }
    }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
}


pub struct UnregisterParamsBuilder<'a> {
    scopeURL: Cow<'a, str>,
}

impl<'a> UnregisterParamsBuilder<'a> {
    pub fn build(self) -> UnregisterParams<'a> {
        UnregisterParams {
            scopeURL: self.scopeURL,
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
    pub fn builder(scopeURL: impl Into<Cow<'a, str>>) -> UpdateRegistrationParamsBuilder<'a> {
        UpdateRegistrationParamsBuilder {
            scopeURL: scopeURL.into(),
        }
    }
    pub fn scopeURL(&self) -> &str { self.scopeURL.as_ref() }
}


pub struct UpdateRegistrationParamsBuilder<'a> {
    scopeURL: Cow<'a, str>,
}

impl<'a> UpdateRegistrationParamsBuilder<'a> {
    pub fn build(self) -> UpdateRegistrationParams<'a> {
        UpdateRegistrationParams {
            scopeURL: self.scopeURL,
        }
    }
}

impl<'a> UpdateRegistrationParams<'a> { pub const METHOD: &'static str = "ServiceWorker.updateRegistration"; }

impl<'a> crate::CdpCommand<'a> for UpdateRegistrationParams<'a> {
    const METHOD: &'static str = "ServiceWorker.updateRegistration";
    type Response = crate::EmptyReturns;
}
