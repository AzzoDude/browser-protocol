//! Defines events for background web platform features.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// The Background Service that will be associated with the commands/events.
/// Every Background Service operates independently, but they share the same
/// API.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ServiceName {
    #[default]
    #[serde(rename = "backgroundFetch")]
    BackgroundFetch,
    #[serde(rename = "backgroundSync")]
    BackgroundSync,
    #[serde(rename = "pushMessaging")]
    PushMessaging,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "paymentHandler")]
    PaymentHandler,
    #[serde(rename = "periodicBackgroundSync")]
    PeriodicBackgroundSync,
}

/// A key-value pair for additional event information to pass along.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct EventMetadata<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundServiceEvent<'a> {
    /// Timestamp of the event (in seconds).
    pub timestamp: crate::network::TimeSinceEpoch,
    /// The origin this event belongs to.
    pub origin: Cow<'a, str>,
    /// The Service Worker ID that initiated the event.
    #[serde(rename = "serviceWorkerRegistrationId")]
    pub service_worker_registration_id: crate::serviceworker::RegistrationID<'a>,
    /// The Background Service this event belongs to.
    pub service: ServiceName,
    /// A description of the event.
    #[serde(rename = "eventName")]
    pub event_name: Cow<'a, str>,
    /// An identifier that groups related events together.
    #[serde(rename = "instanceId")]
    pub instance_id: Cow<'a, str>,
    /// A list of event-specific information.
    #[serde(rename = "eventMetadata")]
    pub event_metadata: Vec<EventMetadata<'a>>,
    /// Storage key this event belongs to.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
}
/// Enables event updates for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BackgroundService.startObserving")]
pub struct StartObservingParams {
    pub service: ServiceName,
}
/// Disables event updates for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BackgroundService.stopObserving")]
pub struct StopObservingParams {
    pub service: ServiceName,
}
/// Set the recording state for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BackgroundService.setRecording")]
pub struct SetRecordingParams {
    #[serde(rename = "shouldRecord")]
    pub should_record: bool,
    pub service: ServiceName,
}
/// Clears all stored data for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BackgroundService.clearEvents")]
pub struct ClearEventsParams {
    pub service: ServiceName,
}
/// Called when the recording state for the service has been updated.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BackgroundService.recordingStateChanged")]
pub struct RecordingStateChanged {
    #[serde(rename = "isRecording")]
    pub is_recording: bool,
    pub service: ServiceName,
}
/// Called with all existing backgroundServiceEvents when enabled, and all new
/// events afterwards if enabled and recording.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BackgroundService.backgroundServiceEventReceived")]
pub struct BackgroundServiceEventReceived<'a> {
    #[serde(rename = "backgroundServiceEvent")]
    pub background_service_event: BackgroundServiceEvent<'a>,
}