//! Defines events for background web platform features.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventMetadata<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> EventMetadata<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `key`: 
    /// * `value`: 
    pub fn builder(key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> EventMetadataBuilder<'a> {
        EventMetadataBuilder {
            key: key.into(),
            value: value.into(),
        }
    }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct EventMetadataBuilder<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> EventMetadataBuilder<'a> {
    pub fn build(self) -> EventMetadata<'a> {
        EventMetadata {
            key: self.key,
            value: self.value,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundServiceEvent<'a> {
    /// Timestamp of the event (in seconds).
    timestamp: crate::network::TimeSinceEpoch,
    /// The origin this event belongs to.
    origin: Cow<'a, str>,
    /// The Service Worker ID that initiated the event.
    #[serde(rename = "serviceWorkerRegistrationId")]
    service_worker_registration_id: crate::serviceworker::RegistrationID<'a>,
    /// The Background Service this event belongs to.
    service: ServiceName,
    /// A description of the event.
    #[serde(rename = "eventName")]
    event_name: Cow<'a, str>,
    /// An identifier that groups related events together.
    #[serde(rename = "instanceId")]
    instance_id: Cow<'a, str>,
    /// A list of event-specific information.
    #[serde(rename = "eventMetadata")]
    event_metadata: Vec<EventMetadata<'a>>,
    /// Storage key this event belongs to.
    #[serde(rename = "storageKey")]
    storage_key: Cow<'a, str>,
}

impl<'a> BackgroundServiceEvent<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `timestamp`: Timestamp of the event (in seconds).
    /// * `origin`: The origin this event belongs to.
    /// * `service_worker_registration_id`: The Service Worker ID that initiated the event.
    /// * `service`: The Background Service this event belongs to.
    /// * `event_name`: A description of the event.
    /// * `instance_id`: An identifier that groups related events together.
    /// * `event_metadata`: A list of event-specific information.
    /// * `storage_key`: Storage key this event belongs to.
    pub fn builder(timestamp: crate::network::TimeSinceEpoch, origin: impl Into<Cow<'a, str>>, service_worker_registration_id: crate::serviceworker::RegistrationID<'a>, service: impl Into<ServiceName>, event_name: impl Into<Cow<'a, str>>, instance_id: impl Into<Cow<'a, str>>, event_metadata: Vec<EventMetadata<'a>>, storage_key: impl Into<Cow<'a, str>>) -> BackgroundServiceEventBuilder<'a> {
        BackgroundServiceEventBuilder {
            timestamp: timestamp,
            origin: origin.into(),
            service_worker_registration_id: service_worker_registration_id,
            service: service.into(),
            event_name: event_name.into(),
            instance_id: instance_id.into(),
            event_metadata: event_metadata,
            storage_key: storage_key.into(),
        }
    }
    /// Timestamp of the event (in seconds).
    pub fn timestamp(&self) -> &crate::network::TimeSinceEpoch { &self.timestamp }
    /// The origin this event belongs to.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    /// The Service Worker ID that initiated the event.
    pub fn service_worker_registration_id(&self) -> &crate::serviceworker::RegistrationID<'a> { &self.service_worker_registration_id }
    /// The Background Service this event belongs to.
    pub fn service(&self) -> &ServiceName { &self.service }
    /// A description of the event.
    pub fn event_name(&self) -> &str { self.event_name.as_ref() }
    /// An identifier that groups related events together.
    pub fn instance_id(&self) -> &str { self.instance_id.as_ref() }
    /// A list of event-specific information.
    pub fn event_metadata(&self) -> &[EventMetadata<'a>] { &self.event_metadata }
    /// Storage key this event belongs to.
    pub fn storage_key(&self) -> &str { self.storage_key.as_ref() }
}


pub struct BackgroundServiceEventBuilder<'a> {
    timestamp: crate::network::TimeSinceEpoch,
    origin: Cow<'a, str>,
    service_worker_registration_id: crate::serviceworker::RegistrationID<'a>,
    service: ServiceName,
    event_name: Cow<'a, str>,
    instance_id: Cow<'a, str>,
    event_metadata: Vec<EventMetadata<'a>>,
    storage_key: Cow<'a, str>,
}

impl<'a> BackgroundServiceEventBuilder<'a> {
    pub fn build(self) -> BackgroundServiceEvent<'a> {
        BackgroundServiceEvent {
            timestamp: self.timestamp,
            origin: self.origin,
            service_worker_registration_id: self.service_worker_registration_id,
            service: self.service,
            event_name: self.event_name,
            instance_id: self.instance_id,
            event_metadata: self.event_metadata,
            storage_key: self.storage_key,
        }
    }
}

/// Enables event updates for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartObservingParams {
    service: ServiceName,
}

impl StartObservingParams {
    /// Creates a builder for this type with the required parameters:
    /// * `service`: 
    pub fn builder(service: impl Into<ServiceName>) -> StartObservingParamsBuilder {
        StartObservingParamsBuilder {
            service: service.into(),
        }
    }
    pub fn service(&self) -> &ServiceName { &self.service }
}


pub struct StartObservingParamsBuilder {
    service: ServiceName,
}

impl StartObservingParamsBuilder {
    pub fn build(self) -> StartObservingParams {
        StartObservingParams {
            service: self.service,
        }
    }
}

impl StartObservingParams { pub const METHOD: &'static str = "BackgroundService.startObserving"; }

impl<'a> crate::CdpCommand<'a> for StartObservingParams {
    const METHOD: &'static str = "BackgroundService.startObserving";
    type Response = crate::EmptyReturns;
}

/// Disables event updates for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopObservingParams {
    service: ServiceName,
}

impl StopObservingParams {
    /// Creates a builder for this type with the required parameters:
    /// * `service`: 
    pub fn builder(service: impl Into<ServiceName>) -> StopObservingParamsBuilder {
        StopObservingParamsBuilder {
            service: service.into(),
        }
    }
    pub fn service(&self) -> &ServiceName { &self.service }
}


pub struct StopObservingParamsBuilder {
    service: ServiceName,
}

impl StopObservingParamsBuilder {
    pub fn build(self) -> StopObservingParams {
        StopObservingParams {
            service: self.service,
        }
    }
}

impl StopObservingParams { pub const METHOD: &'static str = "BackgroundService.stopObserving"; }

impl<'a> crate::CdpCommand<'a> for StopObservingParams {
    const METHOD: &'static str = "BackgroundService.stopObserving";
    type Response = crate::EmptyReturns;
}

/// Set the recording state for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRecordingParams {
    #[serde(rename = "shouldRecord")]
    should_record: bool,
    service: ServiceName,
}

impl SetRecordingParams {
    /// Creates a builder for this type with the required parameters:
    /// * `should_record`: 
    /// * `service`: 
    pub fn builder(should_record: bool, service: impl Into<ServiceName>) -> SetRecordingParamsBuilder {
        SetRecordingParamsBuilder {
            should_record: should_record,
            service: service.into(),
        }
    }
    pub fn should_record(&self) -> bool { self.should_record }
    pub fn service(&self) -> &ServiceName { &self.service }
}


pub struct SetRecordingParamsBuilder {
    should_record: bool,
    service: ServiceName,
}

impl SetRecordingParamsBuilder {
    pub fn build(self) -> SetRecordingParams {
        SetRecordingParams {
            should_record: self.should_record,
            service: self.service,
        }
    }
}

impl SetRecordingParams { pub const METHOD: &'static str = "BackgroundService.setRecording"; }

impl<'a> crate::CdpCommand<'a> for SetRecordingParams {
    const METHOD: &'static str = "BackgroundService.setRecording";
    type Response = crate::EmptyReturns;
}

/// Clears all stored data for the service.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearEventsParams {
    service: ServiceName,
}

impl ClearEventsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `service`: 
    pub fn builder(service: impl Into<ServiceName>) -> ClearEventsParamsBuilder {
        ClearEventsParamsBuilder {
            service: service.into(),
        }
    }
    pub fn service(&self) -> &ServiceName { &self.service }
}


pub struct ClearEventsParamsBuilder {
    service: ServiceName,
}

impl ClearEventsParamsBuilder {
    pub fn build(self) -> ClearEventsParams {
        ClearEventsParams {
            service: self.service,
        }
    }
}

impl ClearEventsParams { pub const METHOD: &'static str = "BackgroundService.clearEvents"; }

impl<'a> crate::CdpCommand<'a> for ClearEventsParams {
    const METHOD: &'static str = "BackgroundService.clearEvents";
    type Response = crate::EmptyReturns;
}
