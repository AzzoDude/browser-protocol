use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! Defines events for background web platform features.

/// The Background Service that will be associated with the commands/events.
/// Every Background Service operates independently, but they share the same
/// API.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceName {
    #[default]
    BackgroundFetch,
    BackgroundSync,
    PushMessaging,
    Notifications,
    PaymentHandler,
    PeriodicBackgroundSync,
}

/// A key-value pair for additional event information to pass along.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventMetadata {

    pub key: String,

    pub value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundServiceEvent {
    /// Timestamp of the event (in seconds).

    pub timestamp: crate::network::TimeSinceEpoch,
    /// The origin this event belongs to.

    pub origin: String,
    /// The Service Worker ID that initiated the event.

    pub serviceWorkerRegistrationId: crate::serviceworker::RegistrationID,
    /// The Background Service this event belongs to.

    pub service: ServiceName,
    /// A description of the event.

    pub eventName: String,
    /// An identifier that groups related events together.

    pub instanceId: String,
    /// A list of event-specific information.

    pub eventMetadata: Vec<EventMetadata>,
    /// Storage key this event belongs to.

    pub storageKey: String,
}

/// Enables event updates for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartObservingParams {

    pub service: ServiceName,
}

impl StartObservingParams { pub const METHOD: &'static str = "BackgroundService.startObserving"; }

impl crate::CdpCommand for StartObservingParams {
    const METHOD: &'static str = "BackgroundService.startObserving";
    type Response = crate::EmptyReturns;
}

/// Disables event updates for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopObservingParams {

    pub service: ServiceName,
}

impl StopObservingParams { pub const METHOD: &'static str = "BackgroundService.stopObserving"; }

impl crate::CdpCommand for StopObservingParams {
    const METHOD: &'static str = "BackgroundService.stopObserving";
    type Response = crate::EmptyReturns;
}

/// Set the recording state for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRecordingParams {

    pub shouldRecord: bool,

    pub service: ServiceName,
}

impl SetRecordingParams { pub const METHOD: &'static str = "BackgroundService.setRecording"; }

impl crate::CdpCommand for SetRecordingParams {
    const METHOD: &'static str = "BackgroundService.setRecording";
    type Response = crate::EmptyReturns;
}

/// Clears all stored data for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearEventsParams {

    pub service: ServiceName,
}

impl ClearEventsParams { pub const METHOD: &'static str = "BackgroundService.clearEvents"; }

impl crate::CdpCommand for ClearEventsParams {
    const METHOD: &'static str = "BackgroundService.clearEvents";
    type Response = crate::EmptyReturns;
}
