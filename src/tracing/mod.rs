use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Configuration for memory dump. Used only when "memory-infra" category is enabled.

pub type MemoryDumpConfig = serde_json::Map<String, JsonValue>;


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TraceConfig<'a> {
    /// Controls how the trace buffer stores data. The default is 'recordUntilFull'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "recordMode")]
    pub record_mode: Option<Cow<'a, str>>,
    /// Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value
    /// of 200 MB would be used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "traceBufferSizeInKb")]
    pub trace_buffer_size_in_kb: Option<f64>,
    /// Turns on JavaScript stack sampling.
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableSampling")]
    pub enable_sampling: Option<bool>,
    /// Turns on system tracing.
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableSystrace")]
    pub enable_systrace: Option<bool>,
    /// Turns on argument filter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableArgumentFilter")]
    pub enable_argument_filter: Option<bool>,
    /// Included category filters.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includedCategories")]
    pub included_categories: Option<Vec<Cow<'a, str>>>,
    /// Excluded category filters.
    #[serde(skip_serializing_if = "Option::is_none", rename = "excludedCategories")]
    pub excluded_categories: Option<Vec<Cow<'a, str>>>,
    /// Configuration to synthesize the delays in tracing.
    #[serde(skip_serializing_if = "Option::is_none", rename = "syntheticDelays")]
    pub synthetic_delays: Option<Vec<Cow<'a, str>>>,
    /// Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.
    #[serde(skip_serializing_if = "Option::is_none", rename = "memoryDumpConfig")]
    pub memory_dump_config: Option<MemoryDumpConfig>,
}
/// Data format of a trace. Can be either the legacy JSON format or the
/// protocol buffer format. Note that the JSON format will be deprecated soon.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StreamFormat {
    #[default]
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "proto")]
    Proto,
}

/// Compression type to use for traces returned via streams.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StreamCompression {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "gzip")]
    Gzip,
}

/// Details exposed when memory request explicitly declared.
/// Keep consistent with memory_dump_request_args.h and
/// memory_instrumentation.mojom

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MemoryDumpLevelOfDetail {
    #[default]
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "detailed")]
    Detailed,
}

/// Backend type to use for tracing. 'chrome' uses the Chrome-integrated
/// tracing service and is supported on all platforms. 'system' is only
/// supported on Chrome OS and uses the Perfetto system tracing service.
/// 'auto' chooses 'system' when the perfettoConfig provided to Tracing.start
/// specifies at least one non-Chrome data source; otherwise uses 'chrome'.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TracingBackend {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "chrome")]
    Chrome,
    #[serde(rename = "system")]
    System,
}

/// Stop trace events collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tracing.end")]
pub struct EndParams {

}
/// Gets supported tracing categories.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tracing.getCategories", response = "GetCategoriesReturns<'a>")]
pub struct GetCategoriesParams {

}
/// Gets supported tracing categories.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoriesReturns<'a> {
    /// A list of supported tracing categories.
    pub categories: Vec<Cow<'a, str>>,
}
/// Return a descriptor for all available tracing categories.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tracing.getTrackEventDescriptor", response = "GetTrackEventDescriptorReturns<'a>")]
pub struct GetTrackEventDescriptorParams {

}
/// Return a descriptor for all available tracing categories.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetTrackEventDescriptorReturns<'a> {
    /// Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message. (Encoded as a base64 string when passed over JSON)
    pub descriptor: Cow<'a, str>,
}
/// Record a clock sync marker in the trace.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tracing.recordClockSyncMarker")]
pub struct RecordClockSyncMarkerParams<'a> {
    /// The ID of this clock sync marker
    #[serde(rename = "syncId")]
    pub sync_id: Cow<'a, str>,
}
/// Request a global memory dump.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tracing.requestMemoryDump", response = "RequestMemoryDumpReturns<'a>")]
pub struct RequestMemoryDumpParams {
    /// Enables more deterministic results by forcing garbage collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deterministic: Option<bool>,
    /// Specifies level of details in memory dump. Defaults to "detailed".
    #[serde(skip_serializing_if = "Option::is_none", rename = "levelOfDetail")]
    pub level_of_detail: Option<MemoryDumpLevelOfDetail>,
}
/// Request a global memory dump.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RequestMemoryDumpReturns<'a> {
    /// GUID of the resulting global memory dump.
    #[serde(rename = "dumpGuid")]
    pub dump_guid: Cow<'a, str>,
    /// True iff the global memory dump succeeded.
    pub success: bool,
}
/// Start trace events collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tracing.start")]
pub struct StartParams<'a> {
    /// Category/tag filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Cow<'a, str>>,
    /// Tracing options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Cow<'a, str>>,
    /// If set, the agent will issue bufferUsage events at this interval, specified in milliseconds
    #[serde(skip_serializing_if = "Option::is_none", rename = "bufferUsageReportingInterval")]
    pub buffer_usage_reporting_interval: Option<f64>,
    /// Whether to report trace events as series of dataCollected events or to save trace to a
    /// stream (defaults to 'ReportEvents').
    #[serde(skip_serializing_if = "Option::is_none", rename = "transferMode")]
    pub transfer_mode: Option<Cow<'a, str>>,
    /// Trace data format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'json').
    #[serde(skip_serializing_if = "Option::is_none", rename = "streamFormat")]
    pub stream_format: Option<StreamFormat>,
    /// Compression format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'none')
    #[serde(skip_serializing_if = "Option::is_none", rename = "streamCompression")]
    pub stream_compression: Option<StreamCompression>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "traceConfig")]
    pub trace_config: Option<TraceConfig<'a>>,
    /// Base64-encoded serialized perfetto.protos.TraceConfig protobuf message
    /// When specified, the parameters 'categories', 'options', 'traceConfig'
    /// are ignored. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "perfettoConfig")]
    pub perfetto_config: Option<Cow<'a, str>>,
    /// Backend type (defaults to 'auto')
    #[serde(skip_serializing_if = "Option::is_none", rename = "tracingBackend")]
    pub tracing_backend: Option<TracingBackend>,
    /// Maximum width and height (in pixels) of each captured screenshot.
    /// Only used when the 'disabled-by-default-devtools.screenshot' category is
    /// enabled. Defaults to 500. The combined memory footprint of screenshots
    /// ('screenshotMaxSize' * 'screenshotMaxSize' * 4 * 'screenshotMaxCount')
    /// is clamped to the existing per-session budget.
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenshotMaxSize")]
    pub screenshot_max_size: Option<u64>,
    /// Maximum number of screenshots captured during a single tracing session.
    /// Only used when the 'disabled-by-default-devtools.screenshot' category is
    /// enabled. Defaults to 450. Clamped together with 'screenshotMaxSize' to
    /// stay within the per-session screenshot memory budget.
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenshotMaxCount")]
    pub screenshot_max_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tracing.bufferUsage")]
pub struct BufferUsage {
    /// A number in range \[0..1\] that indicates the used size of event buffer as a fraction of its
    /// total size.
    #[serde(skip_serializing_if = "Option::is_none", rename = "percentFull")]
    pub percent_full: Option<f64>,
    /// An approximate number of events in the trace log.
    #[serde(skip_serializing_if = "Option::is_none", rename = "eventCount")]
    pub event_count: Option<f64>,
    /// A number in range \[0..1\] that indicates the used size of event buffer as a fraction of its
    /// total size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
/// Contains a bucket of collected trace events. When tracing is stopped collected events will be
/// sent as a sequence of dataCollected events followed by tracingComplete event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tracing.dataCollected")]
pub struct DataCollected {
    pub value: Vec<serde_json::Map<String, JsonValue>>,
}
/// Signals that tracing is stopped and there is no trace buffers pending flush, all data were
/// delivered via dataCollected events.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Tracing.tracingComplete")]
pub struct TracingComplete<'a> {
    /// Indicates whether some trace data is known to have been lost, e.g. because the trace ring
    /// buffer wrapped around.
    #[serde(rename = "dataLossOccurred")]
    pub data_loss_occurred: bool,
    /// A handle of the stream that holds resulting trace data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<crate::io::StreamHandle<'a>>,
    /// Trace data format of returned stream.
    #[serde(skip_serializing_if = "Option::is_none", rename = "traceFormat")]
    pub trace_format: Option<StreamFormat>,
    /// Compression format of returned stream.
    #[serde(skip_serializing_if = "Option::is_none", rename = "streamCompression")]
    pub stream_compression: Option<StreamCompression>,
}