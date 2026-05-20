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
    serviceWorkerRegistrationId: crate::serviceworker::RegistrationID<'a>,
    /// The Background Service this event belongs to.
    service: ServiceName,
    /// A description of the event.
    eventName: Cow<'a, str>,
    /// An identifier that groups related events together.
    instanceId: Cow<'a, str>,
    /// A list of event-specific information.
    eventMetadata: Vec<EventMetadata<'a>>,
    /// Storage key this event belongs to.
    storageKey: Cow<'a, str>,
}

impl<'a> BackgroundServiceEvent<'a> {
    pub fn builder(timestamp: crate::network::TimeSinceEpoch, origin: impl Into<Cow<'a, str>>, serviceWorkerRegistrationId: crate::serviceworker::RegistrationID<'a>, service: impl Into<ServiceName>, eventName: impl Into<Cow<'a, str>>, instanceId: impl Into<Cow<'a, str>>, eventMetadata: Vec<EventMetadata<'a>>, storageKey: impl Into<Cow<'a, str>>) -> BackgroundServiceEventBuilder<'a> {
        BackgroundServiceEventBuilder {
            timestamp: timestamp,
            origin: origin.into(),
            serviceWorkerRegistrationId: serviceWorkerRegistrationId,
            service: service.into(),
            eventName: eventName.into(),
            instanceId: instanceId.into(),
            eventMetadata: eventMetadata,
            storageKey: storageKey.into(),
        }
    }
    pub fn timestamp(&self) -> &crate::network::TimeSinceEpoch { &self.timestamp }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn serviceWorkerRegistrationId(&self) -> &crate::serviceworker::RegistrationID<'a> { &self.serviceWorkerRegistrationId }
    pub fn service(&self) -> &ServiceName { &self.service }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
    pub fn instanceId(&self) -> &str { self.instanceId.as_ref() }
    pub fn eventMetadata(&self) -> &[EventMetadata<'a>] { &self.eventMetadata }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}


pub struct BackgroundServiceEventBuilder<'a> {
    timestamp: crate::network::TimeSinceEpoch,
    origin: Cow<'a, str>,
    serviceWorkerRegistrationId: crate::serviceworker::RegistrationID<'a>,
    service: ServiceName,
    eventName: Cow<'a, str>,
    instanceId: Cow<'a, str>,
    eventMetadata: Vec<EventMetadata<'a>>,
    storageKey: Cow<'a, str>,
}

impl<'a> BackgroundServiceEventBuilder<'a> {
    pub fn build(self) -> BackgroundServiceEvent<'a> {
        BackgroundServiceEvent {
            timestamp: self.timestamp,
            origin: self.origin,
            serviceWorkerRegistrationId: self.serviceWorkerRegistrationId,
            service: self.service,
            eventName: self.eventName,
            instanceId: self.instanceId,
            eventMetadata: self.eventMetadata,
            storageKey: self.storageKey,
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
    shouldRecord: bool,
    service: ServiceName,
}

impl SetRecordingParams {
    pub fn builder(shouldRecord: bool, service: impl Into<ServiceName>) -> SetRecordingParamsBuilder {
        SetRecordingParamsBuilder {
            shouldRecord: shouldRecord,
            service: service.into(),
        }
    }
    pub fn shouldRecord(&self) -> bool { self.shouldRecord }
    pub fn service(&self) -> &ServiceName { &self.service }
}


pub struct SetRecordingParamsBuilder {
    shouldRecord: bool,
    service: ServiceName,
}

impl SetRecordingParamsBuilder {
    pub fn build(self) -> SetRecordingParams {
        SetRecordingParams {
            shouldRecord: self.shouldRecord,
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
