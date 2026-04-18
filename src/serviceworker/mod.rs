use serde::{Serialize, Deserialize};


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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchSyncEventParams {

    pub origin: String,

    pub registrationId: RegistrationID,

    pub tag: String,

    pub lastChance: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchPeriodicSyncEventParams {

    pub origin: String,

    pub registrationId: RegistrationID,

    pub tag: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetForceUpdateOnPageLoadParams {

    pub forceUpdateOnPageLoad: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SkipWaitingParams {

    pub scopeURL: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartWorkerParams {

    pub scopeURL: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopWorkerParams {

    pub versionId: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnregisterParams {

    pub scopeURL: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRegistrationParams {

    pub scopeURL: String,
}
