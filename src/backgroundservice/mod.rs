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
    pub fn builder() -> EventMetadataBuilder<'a> { EventMetadataBuilder::default() }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct EventMetadataBuilder<'a> {
    key: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> EventMetadataBuilder<'a> {
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> EventMetadata<'a> {
        EventMetadata {
            key: self.key.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
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
    pub fn builder() -> BackgroundServiceEventBuilder<'a> { BackgroundServiceEventBuilder::default() }
    pub fn timestamp(&self) -> &crate::network::TimeSinceEpoch { &self.timestamp }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn serviceWorkerRegistrationId(&self) -> &crate::serviceworker::RegistrationID<'a> { &self.serviceWorkerRegistrationId }
    pub fn service(&self) -> &ServiceName { &self.service }
    pub fn eventName(&self) -> &str { self.eventName.as_ref() }
    pub fn instanceId(&self) -> &str { self.instanceId.as_ref() }
    pub fn eventMetadata(&self) -> &[EventMetadata<'a>] { &self.eventMetadata }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}

#[derive(Default)]
pub struct BackgroundServiceEventBuilder<'a> {
    timestamp: Option<crate::network::TimeSinceEpoch>,
    origin: Option<Cow<'a, str>>,
    serviceWorkerRegistrationId: Option<crate::serviceworker::RegistrationID<'a>>,
    service: Option<ServiceName>,
    eventName: Option<Cow<'a, str>>,
    instanceId: Option<Cow<'a, str>>,
    eventMetadata: Option<Vec<EventMetadata<'a>>>,
    storageKey: Option<Cow<'a, str>>,
}

impl<'a> BackgroundServiceEventBuilder<'a> {
    /// Timestamp of the event (in seconds).
    pub fn timestamp(mut self, timestamp: crate::network::TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    /// The origin this event belongs to.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// The Service Worker ID that initiated the event.
    pub fn serviceWorkerRegistrationId(mut self, serviceWorkerRegistrationId: crate::serviceworker::RegistrationID<'a>) -> Self { self.serviceWorkerRegistrationId = Some(serviceWorkerRegistrationId); self }
    /// The Background Service this event belongs to.
    pub fn service(mut self, service: ServiceName) -> Self { self.service = Some(service); self }
    /// A description of the event.
    pub fn eventName(mut self, eventName: impl Into<Cow<'a, str>>) -> Self { self.eventName = Some(eventName.into()); self }
    /// An identifier that groups related events together.
    pub fn instanceId(mut self, instanceId: impl Into<Cow<'a, str>>) -> Self { self.instanceId = Some(instanceId.into()); self }
    /// A list of event-specific information.
    pub fn eventMetadata(mut self, eventMetadata: Vec<EventMetadata<'a>>) -> Self { self.eventMetadata = Some(eventMetadata); self }
    /// Storage key this event belongs to.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    pub fn build(self) -> BackgroundServiceEvent<'a> {
        BackgroundServiceEvent {
            timestamp: self.timestamp.unwrap_or_default(),
            origin: self.origin.unwrap_or_default(),
            serviceWorkerRegistrationId: self.serviceWorkerRegistrationId.unwrap_or_default(),
            service: self.service.unwrap_or_default(),
            eventName: self.eventName.unwrap_or_default(),
            instanceId: self.instanceId.unwrap_or_default(),
            eventMetadata: self.eventMetadata.unwrap_or_default(),
            storageKey: self.storageKey.unwrap_or_default(),
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
    pub fn builder() -> StartObservingParamsBuilder { StartObservingParamsBuilder::default() }
    pub fn service(&self) -> &ServiceName { &self.service }
}

#[derive(Default)]
pub struct StartObservingParamsBuilder {
    service: Option<ServiceName>,
}

impl StartObservingParamsBuilder {
    pub fn service(mut self, service: ServiceName) -> Self { self.service = Some(service); self }
    pub fn build(self) -> StartObservingParams {
        StartObservingParams {
            service: self.service.unwrap_or_default(),
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
    pub fn builder() -> StopObservingParamsBuilder { StopObservingParamsBuilder::default() }
    pub fn service(&self) -> &ServiceName { &self.service }
}

#[derive(Default)]
pub struct StopObservingParamsBuilder {
    service: Option<ServiceName>,
}

impl StopObservingParamsBuilder {
    pub fn service(mut self, service: ServiceName) -> Self { self.service = Some(service); self }
    pub fn build(self) -> StopObservingParams {
        StopObservingParams {
            service: self.service.unwrap_or_default(),
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
    pub fn builder() -> SetRecordingParamsBuilder { SetRecordingParamsBuilder::default() }
    pub fn shouldRecord(&self) -> bool { self.shouldRecord }
    pub fn service(&self) -> &ServiceName { &self.service }
}

#[derive(Default)]
pub struct SetRecordingParamsBuilder {
    shouldRecord: Option<bool>,
    service: Option<ServiceName>,
}

impl SetRecordingParamsBuilder {
    pub fn shouldRecord(mut self, shouldRecord: bool) -> Self { self.shouldRecord = Some(shouldRecord); self }
    pub fn service(mut self, service: ServiceName) -> Self { self.service = Some(service); self }
    pub fn build(self) -> SetRecordingParams {
        SetRecordingParams {
            shouldRecord: self.shouldRecord.unwrap_or_default(),
            service: self.service.unwrap_or_default(),
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
    pub fn builder() -> ClearEventsParamsBuilder { ClearEventsParamsBuilder::default() }
    pub fn service(&self) -> &ServiceName { &self.service }
}

#[derive(Default)]
pub struct ClearEventsParamsBuilder {
    service: Option<ServiceName>,
}

impl ClearEventsParamsBuilder {
    pub fn service(mut self, service: ServiceName) -> Self { self.service = Some(service); self }
    pub fn build(self) -> ClearEventsParams {
        ClearEventsParams {
            service: self.service.unwrap_or_default(),
        }
    }
}

impl ClearEventsParams { pub const METHOD: &'static str = "BackgroundService.clearEvents"; }

impl<'a> crate::CdpCommand<'a> for ClearEventsParams {
    const METHOD: &'static str = "BackgroundService.clearEvents";
    type Response = crate::EmptyReturns;
}
