use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


pub type RegistrationID<'a> = Cow<'a, str>;

/// ServiceWorker registration.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRegistration<'a> {
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationID<'a>,
    #[serde(rename = "scopeURL")]
    pub scope_url: Cow<'a, str>,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
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

/// Mostly corresponds to 'RouterCondition' in ServiceWorker spec
/// (<https://www.w3.org/TR/service-workers/#dictdef-routercondition>)

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRouterCondition<'a> {
    /// Plain text, or JSON serialization of URLPatternInit or URLPattern
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlPattern")]
    pub url_pattern: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestMethod")]
    pub request_method: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestMode")]
    pub request_mode: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestDestination")]
    pub request_destination: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "runningStatus")]
    pub running_status: Option<ServiceWorkerVersionRunningStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Box<ServiceWorkerRouterCondition<'a>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<ServiceWorkerRouterCondition<'a>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceWorkerRouterSourceType {
    #[default]
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "fetchEvent")]
    FetchEvent,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "raceNetworkAndFetchHandler")]
    RaceNetworkAndFetchHandler,
    #[serde(rename = "raceNetworkAndCache")]
    RaceNetworkAndCache,
    #[serde(rename = "sourceDict")]
    SourceDict,
}

/// <https://www.w3.org/TR/service-workers/#dictdef-routersourcedict>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRouterSourceDict<'a> {
    #[serde(rename = "cacheName")]
    pub cache_name: Cow<'a, str>,
}
/// Corresponds to 'RouterSource' in the spec while the representation is different as follows.
/// (<https://www.w3.org/TR/service-workers/#typedefdef-routersource>)
/// - 'RouterSourceEnum': 'type' equals 'cache', 'sourceDict' is null.
/// - 'RouterSourceDict': 'type' equals 'sourceDict', 'sourceDict' has valid value.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRouterSource<'a> {
    #[serde(rename = "type")]
    pub type_: ServiceWorkerRouterSourceType,
    /// Non-empty iff 'type' equals "sourceDict".
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceDict")]
    pub source_dict: Option<ServiceWorkerRouterSourceDict<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRouterRule<'a> {
    pub condition: ServiceWorkerRouterCondition<'a>,
    pub source: ServiceWorkerRouterSource<'a>,
    /// Rule ID assigned by the browser. Unique within each ServiceWorkerVersion.
    pub id: u64,
}
/// ServiceWorker version.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerVersion<'a> {
    #[serde(rename = "versionId")]
    pub version_id: Cow<'a, str>,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationID<'a>,
    #[serde(rename = "scriptURL")]
    pub script_url: Cow<'a, str>,
    #[serde(rename = "runningStatus")]
    pub running_status: ServiceWorkerVersionRunningStatus,
    pub status: ServiceWorkerVersionStatus,
    /// The Last-Modified header value of the main script.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptLastModified")]
    pub script_last_modified: Option<f64>,
    /// The time at which the response headers of the main script were received from the server.
    /// For cached script it is the last time the cache entry was validated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptResponseTime")]
    pub script_response_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "controlledClients")]
    pub controlled_clients: Option<Vec<crate::target::TargetID<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    pub target_id: Option<crate::target::TargetID<'a>>,
    /// Migration to 'typedRouterRules' is in progress. The browser sends either
    /// 'routerRules' or 'typedRouterRules'.
    /// TODO(crbug.com/540469610): Remove 'routerRules' after the migration.
    #[serde(skip_serializing_if = "Option::is_none", rename = "routerRules")]
    pub router_rules: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "typedRouterRules")]
    pub typed_router_rules: Option<Vec<ServiceWorkerRouterRule<'a>>>,
}
/// ServiceWorker error message.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerErrorMessage<'a> {
    #[serde(rename = "errorMessage")]
    pub error_message: Cow<'a, str>,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationID<'a>,
    #[serde(rename = "versionId")]
    pub version_id: Cow<'a, str>,
    #[serde(rename = "sourceURL")]
    pub source_url: Cow<'a, str>,
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.deliverPushMessage")]
pub struct DeliverPushMessageParams<'a> {
    pub origin: Cow<'a, str>,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationID<'a>,
    pub data: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.disable")]
pub struct DisableParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.dispatchSyncEvent")]
pub struct DispatchSyncEventParams<'a> {
    pub origin: Cow<'a, str>,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationID<'a>,
    pub tag: Cow<'a, str>,
    #[serde(rename = "lastChance")]
    pub last_chance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.dispatchPeriodicSyncEvent")]
pub struct DispatchPeriodicSyncEventParams<'a> {
    pub origin: Cow<'a, str>,
    #[serde(rename = "registrationId")]
    pub registration_id: RegistrationID<'a>,
    pub tag: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.enable")]
pub struct EnableParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.setForceUpdateOnPageLoad")]
pub struct SetForceUpdateOnPageLoadParams {
    #[serde(rename = "forceUpdateOnPageLoad")]
    pub force_update_on_page_load: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.skipWaiting")]
pub struct SkipWaitingParams<'a> {
    #[serde(rename = "scopeURL")]
    pub scope_url: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.startWorker")]
pub struct StartWorkerParams<'a> {
    #[serde(rename = "scopeURL")]
    pub scope_url: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.stopAllWorkers")]
pub struct StopAllWorkersParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.stopWorker")]
pub struct StopWorkerParams<'a> {
    #[serde(rename = "versionId")]
    pub version_id: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.unregister")]
pub struct UnregisterParams<'a> {
    #[serde(rename = "scopeURL")]
    pub scope_url: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.updateRegistration")]
pub struct UpdateRegistrationParams<'a> {
    #[serde(rename = "scopeURL")]
    pub scope_url: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.workerErrorReported")]
pub struct WorkerErrorReported<'a> {
    #[serde(rename = "errorMessage")]
    pub error_message: ServiceWorkerErrorMessage<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.workerRegistrationUpdated")]
pub struct WorkerRegistrationUpdated<'a> {
    pub registrations: Vec<ServiceWorkerRegistration<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "ServiceWorker.workerVersionUpdated")]
pub struct WorkerVersionUpdated<'a> {
    pub versions: Vec<ServiceWorkerVersion<'a>>,
}