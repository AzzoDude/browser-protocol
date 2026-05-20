use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


pub type RegistrationID<'a> = Cow<'a, str>;

/// ServiceWorker registration.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRegistration<'a> {
    #[serde(rename = "registrationId")]
    registration_id: RegistrationID<'a>,
    #[serde(rename = "scopeURL")]
    scope_url: Cow<'a, str>,
    #[serde(rename = "isDeleted")]
    is_deleted: bool,
}

impl<'a> ServiceWorkerRegistration<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `registration_id`: 
    /// * `scope_url`: 
    /// * `is_deleted`: 
    pub fn builder(registration_id: impl Into<RegistrationID<'a>>, scope_url: impl Into<Cow<'a, str>>, is_deleted: bool) -> ServiceWorkerRegistrationBuilder<'a> {
        ServiceWorkerRegistrationBuilder {
            registration_id: registration_id.into(),
            scope_url: scope_url.into(),
            is_deleted: is_deleted,
        }
    }
    pub fn registration_id(&self) -> &RegistrationID<'a> { &self.registration_id }
    pub fn scope_url(&self) -> &str { self.scope_url.as_ref() }
    pub fn is_deleted(&self) -> bool { self.is_deleted }
}


pub struct ServiceWorkerRegistrationBuilder<'a> {
    registration_id: RegistrationID<'a>,
    scope_url: Cow<'a, str>,
    is_deleted: bool,
}

impl<'a> ServiceWorkerRegistrationBuilder<'a> {
    pub fn build(self) -> ServiceWorkerRegistration<'a> {
        ServiceWorkerRegistration {
            registration_id: self.registration_id,
            scope_url: self.scope_url,
            is_deleted: self.is_deleted,
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
    #[serde(rename = "versionId")]
    version_id: Cow<'a, str>,
    #[serde(rename = "registrationId")]
    registration_id: RegistrationID<'a>,
    #[serde(rename = "scriptURL")]
    script_url: Cow<'a, str>,
    #[serde(rename = "runningStatus")]
    running_status: ServiceWorkerVersionRunningStatus,
    status: ServiceWorkerVersionStatus,
    /// The Last-Modified header value of the main script.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptLastModified")]
    script_last_modified: Option<f64>,
    /// The time at which the response headers of the main script were received from the server.
    /// For cached script it is the last time the cache entry was validated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptResponseTime")]
    script_response_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "controlledClients")]
    controlled_clients: Option<Vec<crate::target::TargetID<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    target_id: Option<crate::target::TargetID<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "routerRules")]
    router_rules: Option<Cow<'a, str>>,
}

impl<'a> ServiceWorkerVersion<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `version_id`: 
    /// * `registration_id`: 
    /// * `script_url`: 
    /// * `running_status`: 
    /// * `status`: 
    pub fn builder(version_id: impl Into<Cow<'a, str>>, registration_id: impl Into<RegistrationID<'a>>, script_url: impl Into<Cow<'a, str>>, running_status: impl Into<ServiceWorkerVersionRunningStatus>, status: impl Into<ServiceWorkerVersionStatus>) -> ServiceWorkerVersionBuilder<'a> {
        ServiceWorkerVersionBuilder {
            version_id: version_id.into(),
            registration_id: registration_id.into(),
            script_url: script_url.into(),
            running_status: running_status.into(),
            status: status.into(),
            script_last_modified: None,
            script_response_time: None,
            controlled_clients: None,
            target_id: None,
            router_rules: None,
        }
    }
    pub fn version_id(&self) -> &str { self.version_id.as_ref() }
    pub fn registration_id(&self) -> &RegistrationID<'a> { &self.registration_id }
    pub fn script_url(&self) -> &str { self.script_url.as_ref() }
    pub fn running_status(&self) -> &ServiceWorkerVersionRunningStatus { &self.running_status }
    pub fn status(&self) -> &ServiceWorkerVersionStatus { &self.status }
    /// The Last-Modified header value of the main script.
    pub fn script_last_modified(&self) -> Option<f64> { self.script_last_modified }
    /// The time at which the response headers of the main script were received from the server.
    /// For cached script it is the last time the cache entry was validated.
    pub fn script_response_time(&self) -> Option<f64> { self.script_response_time }
    pub fn controlled_clients(&self) -> Option<&[crate::target::TargetID<'a>]> { self.controlled_clients.as_deref() }
    pub fn target_id(&self) -> Option<&crate::target::TargetID<'a>> { self.target_id.as_ref() }
    pub fn router_rules(&self) -> Option<&str> { self.router_rules.as_deref() }
}


pub struct ServiceWorkerVersionBuilder<'a> {
    version_id: Cow<'a, str>,
    registration_id: RegistrationID<'a>,
    script_url: Cow<'a, str>,
    running_status: ServiceWorkerVersionRunningStatus,
    status: ServiceWorkerVersionStatus,
    script_last_modified: Option<f64>,
    script_response_time: Option<f64>,
    controlled_clients: Option<Vec<crate::target::TargetID<'a>>>,
    target_id: Option<crate::target::TargetID<'a>>,
    router_rules: Option<Cow<'a, str>>,
}

impl<'a> ServiceWorkerVersionBuilder<'a> {
    /// The Last-Modified header value of the main script.
    pub fn script_last_modified(mut self, script_last_modified: f64) -> Self { self.script_last_modified = Some(script_last_modified); self }
    /// The time at which the response headers of the main script were received from the server.
    /// For cached script it is the last time the cache entry was validated.
    pub fn script_response_time(mut self, script_response_time: f64) -> Self { self.script_response_time = Some(script_response_time); self }
    pub fn controlled_clients(mut self, controlled_clients: Vec<crate::target::TargetID<'a>>) -> Self { self.controlled_clients = Some(controlled_clients); self }
    pub fn target_id(mut self, target_id: crate::target::TargetID<'a>) -> Self { self.target_id = Some(target_id); self }
    pub fn router_rules(mut self, router_rules: impl Into<Cow<'a, str>>) -> Self { self.router_rules = Some(router_rules.into()); self }
    pub fn build(self) -> ServiceWorkerVersion<'a> {
        ServiceWorkerVersion {
            version_id: self.version_id,
            registration_id: self.registration_id,
            script_url: self.script_url,
            running_status: self.running_status,
            status: self.status,
            script_last_modified: self.script_last_modified,
            script_response_time: self.script_response_time,
            controlled_clients: self.controlled_clients,
            target_id: self.target_id,
            router_rules: self.router_rules,
        }
    }
}

/// ServiceWorker error message.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerErrorMessage<'a> {
    #[serde(rename = "errorMessage")]
    error_message: Cow<'a, str>,
    #[serde(rename = "registrationId")]
    registration_id: RegistrationID<'a>,
    #[serde(rename = "versionId")]
    version_id: Cow<'a, str>,
    #[serde(rename = "sourceURL")]
    source_url: Cow<'a, str>,
    #[serde(rename = "lineNumber")]
    line_number: i64,
    #[serde(rename = "columnNumber")]
    column_number: i64,
}

impl<'a> ServiceWorkerErrorMessage<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `error_message`: 
    /// * `registration_id`: 
    /// * `version_id`: 
    /// * `source_url`: 
    /// * `line_number`: 
    /// * `column_number`: 
    pub fn builder(error_message: impl Into<Cow<'a, str>>, registration_id: impl Into<RegistrationID<'a>>, version_id: impl Into<Cow<'a, str>>, source_url: impl Into<Cow<'a, str>>, line_number: i64, column_number: i64) -> ServiceWorkerErrorMessageBuilder<'a> {
        ServiceWorkerErrorMessageBuilder {
            error_message: error_message.into(),
            registration_id: registration_id.into(),
            version_id: version_id.into(),
            source_url: source_url.into(),
            line_number: line_number,
            column_number: column_number,
        }
    }
    pub fn error_message(&self) -> &str { self.error_message.as_ref() }
    pub fn registration_id(&self) -> &RegistrationID<'a> { &self.registration_id }
    pub fn version_id(&self) -> &str { self.version_id.as_ref() }
    pub fn source_url(&self) -> &str { self.source_url.as_ref() }
    pub fn line_number(&self) -> i64 { self.line_number }
    pub fn column_number(&self) -> i64 { self.column_number }
}


pub struct ServiceWorkerErrorMessageBuilder<'a> {
    error_message: Cow<'a, str>,
    registration_id: RegistrationID<'a>,
    version_id: Cow<'a, str>,
    source_url: Cow<'a, str>,
    line_number: i64,
    column_number: i64,
}

impl<'a> ServiceWorkerErrorMessageBuilder<'a> {
    pub fn build(self) -> ServiceWorkerErrorMessage<'a> {
        ServiceWorkerErrorMessage {
            error_message: self.error_message,
            registration_id: self.registration_id,
            version_id: self.version_id,
            source_url: self.source_url,
            line_number: self.line_number,
            column_number: self.column_number,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeliverPushMessageParams<'a> {
    origin: Cow<'a, str>,
    #[serde(rename = "registrationId")]
    registration_id: RegistrationID<'a>,
    data: Cow<'a, str>,
}

impl<'a> DeliverPushMessageParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: 
    /// * `registration_id`: 
    /// * `data`: 
    pub fn builder(origin: impl Into<Cow<'a, str>>, registration_id: impl Into<RegistrationID<'a>>, data: impl Into<Cow<'a, str>>) -> DeliverPushMessageParamsBuilder<'a> {
        DeliverPushMessageParamsBuilder {
            origin: origin.into(),
            registration_id: registration_id.into(),
            data: data.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn registration_id(&self) -> &RegistrationID<'a> { &self.registration_id }
    pub fn data(&self) -> &str { self.data.as_ref() }
}


pub struct DeliverPushMessageParamsBuilder<'a> {
    origin: Cow<'a, str>,
    registration_id: RegistrationID<'a>,
    data: Cow<'a, str>,
}

impl<'a> DeliverPushMessageParamsBuilder<'a> {
    pub fn build(self) -> DeliverPushMessageParams<'a> {
        DeliverPushMessageParams {
            origin: self.origin,
            registration_id: self.registration_id,
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
    #[serde(rename = "registrationId")]
    registration_id: RegistrationID<'a>,
    tag: Cow<'a, str>,
    #[serde(rename = "lastChance")]
    last_chance: bool,
}

impl<'a> DispatchSyncEventParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: 
    /// * `registration_id`: 
    /// * `tag`: 
    /// * `last_chance`: 
    pub fn builder(origin: impl Into<Cow<'a, str>>, registration_id: impl Into<RegistrationID<'a>>, tag: impl Into<Cow<'a, str>>, last_chance: bool) -> DispatchSyncEventParamsBuilder<'a> {
        DispatchSyncEventParamsBuilder {
            origin: origin.into(),
            registration_id: registration_id.into(),
            tag: tag.into(),
            last_chance: last_chance,
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn registration_id(&self) -> &RegistrationID<'a> { &self.registration_id }
    pub fn tag(&self) -> &str { self.tag.as_ref() }
    pub fn last_chance(&self) -> bool { self.last_chance }
}


pub struct DispatchSyncEventParamsBuilder<'a> {
    origin: Cow<'a, str>,
    registration_id: RegistrationID<'a>,
    tag: Cow<'a, str>,
    last_chance: bool,
}

impl<'a> DispatchSyncEventParamsBuilder<'a> {
    pub fn build(self) -> DispatchSyncEventParams<'a> {
        DispatchSyncEventParams {
            origin: self.origin,
            registration_id: self.registration_id,
            tag: self.tag,
            last_chance: self.last_chance,
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
    #[serde(rename = "registrationId")]
    registration_id: RegistrationID<'a>,
    tag: Cow<'a, str>,
}

impl<'a> DispatchPeriodicSyncEventParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: 
    /// * `registration_id`: 
    /// * `tag`: 
    pub fn builder(origin: impl Into<Cow<'a, str>>, registration_id: impl Into<RegistrationID<'a>>, tag: impl Into<Cow<'a, str>>) -> DispatchPeriodicSyncEventParamsBuilder<'a> {
        DispatchPeriodicSyncEventParamsBuilder {
            origin: origin.into(),
            registration_id: registration_id.into(),
            tag: tag.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn registration_id(&self) -> &RegistrationID<'a> { &self.registration_id }
    pub fn tag(&self) -> &str { self.tag.as_ref() }
}


pub struct DispatchPeriodicSyncEventParamsBuilder<'a> {
    origin: Cow<'a, str>,
    registration_id: RegistrationID<'a>,
    tag: Cow<'a, str>,
}

impl<'a> DispatchPeriodicSyncEventParamsBuilder<'a> {
    pub fn build(self) -> DispatchPeriodicSyncEventParams<'a> {
        DispatchPeriodicSyncEventParams {
            origin: self.origin,
            registration_id: self.registration_id,
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
    #[serde(rename = "forceUpdateOnPageLoad")]
    force_update_on_page_load: bool,
}

impl SetForceUpdateOnPageLoadParams {
    /// Creates a builder for this type with the required parameters:
    /// * `force_update_on_page_load`: 
    pub fn builder(force_update_on_page_load: bool) -> SetForceUpdateOnPageLoadParamsBuilder {
        SetForceUpdateOnPageLoadParamsBuilder {
            force_update_on_page_load: force_update_on_page_load,
        }
    }
    pub fn force_update_on_page_load(&self) -> bool { self.force_update_on_page_load }
}


pub struct SetForceUpdateOnPageLoadParamsBuilder {
    force_update_on_page_load: bool,
}

impl SetForceUpdateOnPageLoadParamsBuilder {
    pub fn build(self) -> SetForceUpdateOnPageLoadParams {
        SetForceUpdateOnPageLoadParams {
            force_update_on_page_load: self.force_update_on_page_load,
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
    #[serde(rename = "scopeURL")]
    scope_url: Cow<'a, str>,
}

impl<'a> SkipWaitingParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `scope_url`: 
    pub fn builder(scope_url: impl Into<Cow<'a, str>>) -> SkipWaitingParamsBuilder<'a> {
        SkipWaitingParamsBuilder {
            scope_url: scope_url.into(),
        }
    }
    pub fn scope_url(&self) -> &str { self.scope_url.as_ref() }
}


pub struct SkipWaitingParamsBuilder<'a> {
    scope_url: Cow<'a, str>,
}

impl<'a> SkipWaitingParamsBuilder<'a> {
    pub fn build(self) -> SkipWaitingParams<'a> {
        SkipWaitingParams {
            scope_url: self.scope_url,
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
    #[serde(rename = "scopeURL")]
    scope_url: Cow<'a, str>,
}

impl<'a> StartWorkerParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `scope_url`: 
    pub fn builder(scope_url: impl Into<Cow<'a, str>>) -> StartWorkerParamsBuilder<'a> {
        StartWorkerParamsBuilder {
            scope_url: scope_url.into(),
        }
    }
    pub fn scope_url(&self) -> &str { self.scope_url.as_ref() }
}


pub struct StartWorkerParamsBuilder<'a> {
    scope_url: Cow<'a, str>,
}

impl<'a> StartWorkerParamsBuilder<'a> {
    pub fn build(self) -> StartWorkerParams<'a> {
        StartWorkerParams {
            scope_url: self.scope_url,
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
    #[serde(rename = "versionId")]
    version_id: Cow<'a, str>,
}

impl<'a> StopWorkerParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `version_id`: 
    pub fn builder(version_id: impl Into<Cow<'a, str>>) -> StopWorkerParamsBuilder<'a> {
        StopWorkerParamsBuilder {
            version_id: version_id.into(),
        }
    }
    pub fn version_id(&self) -> &str { self.version_id.as_ref() }
}


pub struct StopWorkerParamsBuilder<'a> {
    version_id: Cow<'a, str>,
}

impl<'a> StopWorkerParamsBuilder<'a> {
    pub fn build(self) -> StopWorkerParams<'a> {
        StopWorkerParams {
            version_id: self.version_id,
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
    #[serde(rename = "scopeURL")]
    scope_url: Cow<'a, str>,
}

impl<'a> UnregisterParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `scope_url`: 
    pub fn builder(scope_url: impl Into<Cow<'a, str>>) -> UnregisterParamsBuilder<'a> {
        UnregisterParamsBuilder {
            scope_url: scope_url.into(),
        }
    }
    pub fn scope_url(&self) -> &str { self.scope_url.as_ref() }
}


pub struct UnregisterParamsBuilder<'a> {
    scope_url: Cow<'a, str>,
}

impl<'a> UnregisterParamsBuilder<'a> {
    pub fn build(self) -> UnregisterParams<'a> {
        UnregisterParams {
            scope_url: self.scope_url,
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
    #[serde(rename = "scopeURL")]
    scope_url: Cow<'a, str>,
}

impl<'a> UpdateRegistrationParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `scope_url`: 
    pub fn builder(scope_url: impl Into<Cow<'a, str>>) -> UpdateRegistrationParamsBuilder<'a> {
        UpdateRegistrationParamsBuilder {
            scope_url: scope_url.into(),
        }
    }
    pub fn scope_url(&self) -> &str { self.scope_url.as_ref() }
}


pub struct UpdateRegistrationParamsBuilder<'a> {
    scope_url: Cow<'a, str>,
}

impl<'a> UpdateRegistrationParamsBuilder<'a> {
    pub fn build(self) -> UpdateRegistrationParams<'a> {
        UpdateRegistrationParams {
            scope_url: self.scope_url,
        }
    }
}

impl<'a> UpdateRegistrationParams<'a> { pub const METHOD: &'static str = "ServiceWorker.updateRegistration"; }

impl<'a> crate::CdpCommand<'a> for UpdateRegistrationParams<'a> {
    const METHOD: &'static str = "ServiceWorker.updateRegistration";
    type Response = crate::EmptyReturns;
}
