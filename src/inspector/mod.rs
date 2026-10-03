use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Disables inspector domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Inspector.disable")]
pub struct DisableParams {

}
/// Enables inspector domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Inspector.enable")]
pub struct EnableParams {

}
/// Fired when remote debugging connection is about to be terminated. Contains detach reason.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Inspector.detached")]
pub struct Detached<'a> {
    /// The reason why connection has been terminated.
    pub reason: Cow<'a, str>,
}
/// Fired when debugging target has crashed

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Inspector.targetCrashed")]
pub struct TargetCrashed {

}
/// Fired when debugging target has reloaded after crash

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Inspector.targetReloadedAfterCrash")]
pub struct TargetReloadedAfterCrash {

}
/// Fired on worker targets when main worker script and any imported scripts have been evaluated.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Inspector.workerScriptLoaded")]
pub struct WorkerScriptLoaded {

}