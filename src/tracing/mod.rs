use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Configuration for memory dump. Used only when "memory-infra" category is enabled.

pub type MemoryDumpConfig = serde_json::Map<String, JsonValue>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TraceConfig {
    /// Controls how the trace buffer stores data. The default is 'recordUntilFull'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recordMode: Option<String>,
    /// Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value
    /// of 200 MB would be used.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub traceBufferSizeInKb: Option<f64>,
    /// Turns on JavaScript stack sampling.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enableSampling: Option<bool>,
    /// Turns on system tracing.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enableSystrace: Option<bool>,
    /// Turns on argument filter.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enableArgumentFilter: Option<bool>,
    /// Included category filters.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includedCategories: Option<Vec<String>>,
    /// Excluded category filters.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludedCategories: Option<Vec<String>>,
    /// Configuration to synthesize the delays in tracing.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntheticDelays: Option<Vec<String>>,
    /// Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memoryDumpConfig: Option<MemoryDumpConfig>,
}

/// Data format of a trace. Can be either the legacy JSON format or the
/// protocol buffer format. Note that the JSON format will be deprecated soon.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StreamFormat {
    #[default]
    Json,
    Proto,
}

/// Compression type to use for traces returned via streams.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StreamCompression {
    #[default]
    None,
    Gzip,
}

/// Details exposed when memory request explicitly declared.
/// Keep consistent with memory_dump_request_args.h and
/// memory_instrumentation.mojom

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MemoryDumpLevelOfDetail {
    #[default]
    Background,
    Light,
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
    Auto,
    Chrome,
    System,
}

/// Gets supported tracing categories.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoriesReturns {
    /// A list of supported tracing categories.

    pub categories: Vec<String>,
}

/// Return a descriptor for all available tracing categories.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTrackEventDescriptorReturns {
    /// Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message. (Encoded as a base64 string when passed over JSON)

    pub descriptor: String,
}

/// Record a clock sync marker in the trace.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecordClockSyncMarkerParams {
    /// The ID of this clock sync marker

    pub syncId: String,
}

/// Request a global memory dump.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestMemoryDumpParams {
    /// Enables more deterministic results by forcing garbage collection

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deterministic: Option<bool>,
    /// Specifies level of details in memory dump. Defaults to "detailed".

    #[serde(skip_serializing_if = "Option::is_none")]
    pub levelOfDetail: Option<MemoryDumpLevelOfDetail>,
}

/// Request a global memory dump.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestMemoryDumpReturns {
    /// GUID of the resulting global memory dump.

    pub dumpGuid: String,
    /// True iff the global memory dump succeeded.

    pub success: bool,
}

/// Start trace events collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartParams {
    /// Category/tag filter

    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    /// Tracing options

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    /// If set, the agent will issue bufferUsage events at this interval, specified in milliseconds

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bufferUsageReportingInterval: Option<f64>,
    /// Whether to report trace events as series of dataCollected events or to save trace to a
    /// stream (defaults to 'ReportEvents').

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferMode: Option<String>,
    /// Trace data format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'json').

    #[serde(skip_serializing_if = "Option::is_none")]
    pub streamFormat: Option<StreamFormat>,
    /// Compression format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'none')

    #[serde(skip_serializing_if = "Option::is_none")]
    pub streamCompression: Option<StreamCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub traceConfig: Option<TraceConfig>,
    /// Base64-encoded serialized perfetto.protos.TraceConfig protobuf message
    /// When specified, the parameters 'categories', 'options', 'traceConfig'
    /// are ignored. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub perfettoConfig: Option<String>,
    /// Backend type (defaults to 'auto')

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracingBackend: Option<TracingBackend>,
}
