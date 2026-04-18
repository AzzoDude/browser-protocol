use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;


pub type RegistrationID = String;

/// ServiceWorker registration.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRegistration {

    pub registrationId: RegistrationID,

    pub scopeURL: String,

    pub isDeleted: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceWorkerVersionRunningStatus {
    #[default]
    Stopped,
    Starting,
    Running,
    Stopping,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceWorkerVersionStatus {
    #[default]
    New,
    Installing,
    Installed,
    Activating,
    Activated,
    Redundant,
}

/// ServiceWorker version.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerVersion {

    pub versionId: String,

    pub registrationId: RegistrationID,

    pub scriptURL: String,

    pub runningStatus: ServiceWorkerVersionRunningStatus,

    pub status: ServiceWorkerVersionStatus,
    /// The Last-Modified header value of the main script.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriptLastModified: Option<f64>,
    /// The time at which the response headers of the main script were received from the server.
    /// For cached script it is the last time the cache entry was validated.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriptResponseTime: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlledClients: Option<Vec<crate::target::TargetID>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetId: Option<crate::target::TargetID>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub routerRules: Option<String>,
}

/// ServiceWorker error message.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerErrorMessage {

    pub errorMessage: String,

    pub registrationId: RegistrationID,

    pub versionId: String,

    pub sourceURL: String,

    pub lineNumber: i64,

    pub columnNumber: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeliverPushMessageParams {

    pub origin: String,

    pub registrationId: RegistrationID,

    pub data: String,
}

impl DeliverPushMessageParams { pub const METHOD: &'static str = "ServiceWorker.deliverPushMessage"; }

impl crate::CdpCommand for DeliverPushMessageParams {
    const METHOD: &'static str = "ServiceWorker.deliverPushMessage";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "ServiceWorker.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "ServiceWorker.disable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchSyncEventParams {

    pub origin: String,

    pub registrationId: RegistrationID,

    pub tag: String,

    pub lastChance: bool,
}

impl DispatchSyncEventParams { pub const METHOD: &'static str = "ServiceWorker.dispatchSyncEvent"; }

impl crate::CdpCommand for DispatchSyncEventParams {
    const METHOD: &'static str = "ServiceWorker.dispatchSyncEvent";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchPeriodicSyncEventParams {

    pub origin: String,

    pub registrationId: RegistrationID,

    pub tag: String,
}

impl DispatchPeriodicSyncEventParams { pub const METHOD: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent"; }

impl crate::CdpCommand for DispatchPeriodicSyncEventParams {
    const METHOD: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "ServiceWorker.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "ServiceWorker.enable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetForceUpdateOnPageLoadParams {

    pub forceUpdateOnPageLoad: bool,
}

impl SetForceUpdateOnPageLoadParams { pub const METHOD: &'static str = "ServiceWorker.setForceUpdateOnPageLoad"; }

impl crate::CdpCommand for SetForceUpdateOnPageLoadParams {
    const METHOD: &'static str = "ServiceWorker.setForceUpdateOnPageLoad";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SkipWaitingParams {

    pub scopeURL: String,
}

impl SkipWaitingParams { pub const METHOD: &'static str = "ServiceWorker.skipWaiting"; }

impl crate::CdpCommand for SkipWaitingParams {
    const METHOD: &'static str = "ServiceWorker.skipWaiting";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartWorkerParams {

    pub scopeURL: String,
}

impl StartWorkerParams { pub const METHOD: &'static str = "ServiceWorker.startWorker"; }

impl crate::CdpCommand for StartWorkerParams {
    const METHOD: &'static str = "ServiceWorker.startWorker";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopAllWorkersParams {}

impl StopAllWorkersParams { pub const METHOD: &'static str = "ServiceWorker.stopAllWorkers"; }

impl crate::CdpCommand for StopAllWorkersParams {
    const METHOD: &'static str = "ServiceWorker.stopAllWorkers";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopWorkerParams {

    pub versionId: String,
}

impl StopWorkerParams { pub const METHOD: &'static str = "ServiceWorker.stopWorker"; }

impl crate::CdpCommand for StopWorkerParams {
    const METHOD: &'static str = "ServiceWorker.stopWorker";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnregisterParams {

    pub scopeURL: String,
}

impl UnregisterParams { pub const METHOD: &'static str = "ServiceWorker.unregister"; }

impl crate::CdpCommand for UnregisterParams {
    const METHOD: &'static str = "ServiceWorker.unregister";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRegistrationParams {

    pub scopeURL: String,
}

impl UpdateRegistrationParams { pub const METHOD: &'static str = "ServiceWorker.updateRegistration"; }

impl crate::CdpCommand for UpdateRegistrationParams {
    const METHOD: &'static str = "ServiceWorker.updateRegistration";
    type Response = crate::EmptyReturns;
}
