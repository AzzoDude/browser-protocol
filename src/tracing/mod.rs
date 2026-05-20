use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Configuration for memory dump. Used only when "memory-infra" category is enabled.

pub type MemoryDumpConfig = serde_json::Map<String, JsonValue>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TraceConfig<'a> {
    /// Controls how the trace buffer stores data. The default is 'recordUntilFull'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "recordMode")]
    record_mode: Option<Cow<'a, str>>,
    /// Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value
    /// of 200 MB would be used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "traceBufferSizeInKb")]
    trace_buffer_size_in_kb: Option<f64>,
    /// Turns on JavaScript stack sampling.
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableSampling")]
    enable_sampling: Option<bool>,
    /// Turns on system tracing.
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableSystrace")]
    enable_systrace: Option<bool>,
    /// Turns on argument filter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableArgumentFilter")]
    enable_argument_filter: Option<bool>,
    /// Included category filters.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includedCategories")]
    included_categories: Option<Vec<Cow<'a, str>>>,
    /// Excluded category filters.
    #[serde(skip_serializing_if = "Option::is_none", rename = "excludedCategories")]
    excluded_categories: Option<Vec<Cow<'a, str>>>,
    /// Configuration to synthesize the delays in tracing.
    #[serde(skip_serializing_if = "Option::is_none", rename = "syntheticDelays")]
    synthetic_delays: Option<Vec<Cow<'a, str>>>,
    /// Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.
    #[serde(skip_serializing_if = "Option::is_none", rename = "memoryDumpConfig")]
    memory_dump_config: Option<MemoryDumpConfig>,
}

impl<'a> TraceConfig<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> TraceConfigBuilder<'a> {
        TraceConfigBuilder {
            record_mode: None,
            trace_buffer_size_in_kb: None,
            enable_sampling: None,
            enable_systrace: None,
            enable_argument_filter: None,
            included_categories: None,
            excluded_categories: None,
            synthetic_delays: None,
            memory_dump_config: None,
        }
    }
    /// Controls how the trace buffer stores data. The default is 'recordUntilFull'.
    pub fn record_mode(&self) -> Option<&str> { self.record_mode.as_deref() }
    /// Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value
    /// of 200 MB would be used.
    pub fn trace_buffer_size_in_kb(&self) -> Option<f64> { self.trace_buffer_size_in_kb }
    /// Turns on JavaScript stack sampling.
    pub fn enable_sampling(&self) -> Option<bool> { self.enable_sampling }
    /// Turns on system tracing.
    pub fn enable_systrace(&self) -> Option<bool> { self.enable_systrace }
    /// Turns on argument filter.
    pub fn enable_argument_filter(&self) -> Option<bool> { self.enable_argument_filter }
    /// Included category filters.
    pub fn included_categories(&self) -> Option<&[Cow<'a, str>]> { self.included_categories.as_deref() }
    /// Excluded category filters.
    pub fn excluded_categories(&self) -> Option<&[Cow<'a, str>]> { self.excluded_categories.as_deref() }
    /// Configuration to synthesize the delays in tracing.
    pub fn synthetic_delays(&self) -> Option<&[Cow<'a, str>]> { self.synthetic_delays.as_deref() }
    /// Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.
    pub fn memory_dump_config(&self) -> Option<&MemoryDumpConfig> { self.memory_dump_config.as_ref() }
}

#[derive(Default)]
pub struct TraceConfigBuilder<'a> {
    record_mode: Option<Cow<'a, str>>,
    trace_buffer_size_in_kb: Option<f64>,
    enable_sampling: Option<bool>,
    enable_systrace: Option<bool>,
    enable_argument_filter: Option<bool>,
    included_categories: Option<Vec<Cow<'a, str>>>,
    excluded_categories: Option<Vec<Cow<'a, str>>>,
    synthetic_delays: Option<Vec<Cow<'a, str>>>,
    memory_dump_config: Option<MemoryDumpConfig>,
}

impl<'a> TraceConfigBuilder<'a> {
    /// Controls how the trace buffer stores data. The default is 'recordUntilFull'.
    pub fn record_mode(mut self, record_mode: impl Into<Cow<'a, str>>) -> Self { self.record_mode = Some(record_mode.into()); self }
    /// Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value
    /// of 200 MB would be used.
    pub fn trace_buffer_size_in_kb(mut self, trace_buffer_size_in_kb: f64) -> Self { self.trace_buffer_size_in_kb = Some(trace_buffer_size_in_kb); self }
    /// Turns on JavaScript stack sampling.
    pub fn enable_sampling(mut self, enable_sampling: bool) -> Self { self.enable_sampling = Some(enable_sampling); self }
    /// Turns on system tracing.
    pub fn enable_systrace(mut self, enable_systrace: bool) -> Self { self.enable_systrace = Some(enable_systrace); self }
    /// Turns on argument filter.
    pub fn enable_argument_filter(mut self, enable_argument_filter: bool) -> Self { self.enable_argument_filter = Some(enable_argument_filter); self }
    /// Included category filters.
    pub fn included_categories(mut self, included_categories: Vec<Cow<'a, str>>) -> Self { self.included_categories = Some(included_categories); self }
    /// Excluded category filters.
    pub fn excluded_categories(mut self, excluded_categories: Vec<Cow<'a, str>>) -> Self { self.excluded_categories = Some(excluded_categories); self }
    /// Configuration to synthesize the delays in tracing.
    pub fn synthetic_delays(mut self, synthetic_delays: Vec<Cow<'a, str>>) -> Self { self.synthetic_delays = Some(synthetic_delays); self }
    /// Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.
    pub fn memory_dump_config(mut self, memory_dump_config: MemoryDumpConfig) -> Self { self.memory_dump_config = Some(memory_dump_config); self }
    pub fn build(self) -> TraceConfig<'a> {
        TraceConfig {
            record_mode: self.record_mode,
            trace_buffer_size_in_kb: self.trace_buffer_size_in_kb,
            enable_sampling: self.enable_sampling,
            enable_systrace: self.enable_systrace,
            enable_argument_filter: self.enable_argument_filter,
            included_categories: self.included_categories,
            excluded_categories: self.excluded_categories,
            synthetic_delays: self.synthetic_delays,
            memory_dump_config: self.memory_dump_config,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EndParams {}

impl EndParams { pub const METHOD: &'static str = "Tracing.end"; }

impl<'a> crate::CdpCommand<'a> for EndParams {
    const METHOD: &'static str = "Tracing.end";
    type Response = crate::EmptyReturns;
}

/// Gets supported tracing categories.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoriesReturns<'a> {
    /// A list of supported tracing categories.
    categories: Vec<Cow<'a, str>>,
}

impl<'a> GetCategoriesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `categories`: A list of supported tracing categories.
    pub fn builder(categories: Vec<Cow<'a, str>>) -> GetCategoriesReturnsBuilder<'a> {
        GetCategoriesReturnsBuilder {
            categories: categories,
        }
    }
    /// A list of supported tracing categories.
    pub fn categories(&self) -> &[Cow<'a, str>] { &self.categories }
}


pub struct GetCategoriesReturnsBuilder<'a> {
    categories: Vec<Cow<'a, str>>,
}

impl<'a> GetCategoriesReturnsBuilder<'a> {
    pub fn build(self) -> GetCategoriesReturns<'a> {
        GetCategoriesReturns {
            categories: self.categories,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetCategoriesParams {}

impl GetCategoriesParams { pub const METHOD: &'static str = "Tracing.getCategories"; }

impl<'a> crate::CdpCommand<'a> for GetCategoriesParams {
    const METHOD: &'static str = "Tracing.getCategories";
    type Response = GetCategoriesReturns<'a>;
}

/// Return a descriptor for all available tracing categories.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTrackEventDescriptorReturns<'a> {
    /// Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message. (Encoded as a base64 string when passed over JSON)
    descriptor: Cow<'a, str>,
}

impl<'a> GetTrackEventDescriptorReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `descriptor`: Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message. (Encoded as a base64 string when passed over JSON)
    pub fn builder(descriptor: impl Into<Cow<'a, str>>) -> GetTrackEventDescriptorReturnsBuilder<'a> {
        GetTrackEventDescriptorReturnsBuilder {
            descriptor: descriptor.into(),
        }
    }
    /// Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message. (Encoded as a base64 string when passed over JSON)
    pub fn descriptor(&self) -> &str { self.descriptor.as_ref() }
}


pub struct GetTrackEventDescriptorReturnsBuilder<'a> {
    descriptor: Cow<'a, str>,
}

impl<'a> GetTrackEventDescriptorReturnsBuilder<'a> {
    pub fn build(self) -> GetTrackEventDescriptorReturns<'a> {
        GetTrackEventDescriptorReturns {
            descriptor: self.descriptor,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTrackEventDescriptorParams {}

impl GetTrackEventDescriptorParams { pub const METHOD: &'static str = "Tracing.getTrackEventDescriptor"; }

impl<'a> crate::CdpCommand<'a> for GetTrackEventDescriptorParams {
    const METHOD: &'static str = "Tracing.getTrackEventDescriptor";
    type Response = GetTrackEventDescriptorReturns<'a>;
}

/// Record a clock sync marker in the trace.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecordClockSyncMarkerParams<'a> {
    /// The ID of this clock sync marker
    #[serde(rename = "syncId")]
    sync_id: Cow<'a, str>,
}

impl<'a> RecordClockSyncMarkerParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `sync_id`: The ID of this clock sync marker
    pub fn builder(sync_id: impl Into<Cow<'a, str>>) -> RecordClockSyncMarkerParamsBuilder<'a> {
        RecordClockSyncMarkerParamsBuilder {
            sync_id: sync_id.into(),
        }
    }
    /// The ID of this clock sync marker
    pub fn sync_id(&self) -> &str { self.sync_id.as_ref() }
}


pub struct RecordClockSyncMarkerParamsBuilder<'a> {
    sync_id: Cow<'a, str>,
}

impl<'a> RecordClockSyncMarkerParamsBuilder<'a> {
    pub fn build(self) -> RecordClockSyncMarkerParams<'a> {
        RecordClockSyncMarkerParams {
            sync_id: self.sync_id,
        }
    }
}

impl<'a> RecordClockSyncMarkerParams<'a> { pub const METHOD: &'static str = "Tracing.recordClockSyncMarker"; }

impl<'a> crate::CdpCommand<'a> for RecordClockSyncMarkerParams<'a> {
    const METHOD: &'static str = "Tracing.recordClockSyncMarker";
    type Response = crate::EmptyReturns;
}

/// Request a global memory dump.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestMemoryDumpParams {
    /// Enables more deterministic results by forcing garbage collection
    #[serde(skip_serializing_if = "Option::is_none")]
    deterministic: Option<bool>,
    /// Specifies level of details in memory dump. Defaults to "detailed".
    #[serde(skip_serializing_if = "Option::is_none", rename = "levelOfDetail")]
    level_of_detail: Option<MemoryDumpLevelOfDetail>,
}

impl RequestMemoryDumpParams {
    /// Creates a builder for this type.
    pub fn builder() -> RequestMemoryDumpParamsBuilder {
        RequestMemoryDumpParamsBuilder {
            deterministic: None,
            level_of_detail: None,
        }
    }
    /// Enables more deterministic results by forcing garbage collection
    pub fn deterministic(&self) -> Option<bool> { self.deterministic }
    /// Specifies level of details in memory dump. Defaults to "detailed".
    pub fn level_of_detail(&self) -> Option<&MemoryDumpLevelOfDetail> { self.level_of_detail.as_ref() }
}

#[derive(Default)]
pub struct RequestMemoryDumpParamsBuilder {
    deterministic: Option<bool>,
    level_of_detail: Option<MemoryDumpLevelOfDetail>,
}

impl RequestMemoryDumpParamsBuilder {
    /// Enables more deterministic results by forcing garbage collection
    pub fn deterministic(mut self, deterministic: bool) -> Self { self.deterministic = Some(deterministic); self }
    /// Specifies level of details in memory dump. Defaults to "detailed".
    pub fn level_of_detail(mut self, level_of_detail: impl Into<MemoryDumpLevelOfDetail>) -> Self { self.level_of_detail = Some(level_of_detail.into()); self }
    pub fn build(self) -> RequestMemoryDumpParams {
        RequestMemoryDumpParams {
            deterministic: self.deterministic,
            level_of_detail: self.level_of_detail,
        }
    }
}

/// Request a global memory dump.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestMemoryDumpReturns<'a> {
    /// GUID of the resulting global memory dump.
    #[serde(rename = "dumpGuid")]
    dump_guid: Cow<'a, str>,
    /// True iff the global memory dump succeeded.
    success: bool,
}

impl<'a> RequestMemoryDumpReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `dump_guid`: GUID of the resulting global memory dump.
    /// * `success`: True iff the global memory dump succeeded.
    pub fn builder(dump_guid: impl Into<Cow<'a, str>>, success: bool) -> RequestMemoryDumpReturnsBuilder<'a> {
        RequestMemoryDumpReturnsBuilder {
            dump_guid: dump_guid.into(),
            success: success,
        }
    }
    /// GUID of the resulting global memory dump.
    pub fn dump_guid(&self) -> &str { self.dump_guid.as_ref() }
    /// True iff the global memory dump succeeded.
    pub fn success(&self) -> bool { self.success }
}


pub struct RequestMemoryDumpReturnsBuilder<'a> {
    dump_guid: Cow<'a, str>,
    success: bool,
}

impl<'a> RequestMemoryDumpReturnsBuilder<'a> {
    pub fn build(self) -> RequestMemoryDumpReturns<'a> {
        RequestMemoryDumpReturns {
            dump_guid: self.dump_guid,
            success: self.success,
        }
    }
}

impl RequestMemoryDumpParams { pub const METHOD: &'static str = "Tracing.requestMemoryDump"; }

impl<'a> crate::CdpCommand<'a> for RequestMemoryDumpParams {
    const METHOD: &'static str = "Tracing.requestMemoryDump";
    type Response = RequestMemoryDumpReturns<'a>;
}

/// Start trace events collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartParams<'a> {
    /// Category/tag filter
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Cow<'a, str>>,
    /// Tracing options
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Cow<'a, str>>,
    /// If set, the agent will issue bufferUsage events at this interval, specified in milliseconds
    #[serde(skip_serializing_if = "Option::is_none", rename = "bufferUsageReportingInterval")]
    buffer_usage_reporting_interval: Option<f64>,
    /// Whether to report trace events as series of dataCollected events or to save trace to a
    /// stream (defaults to 'ReportEvents').
    #[serde(skip_serializing_if = "Option::is_none", rename = "transferMode")]
    transfer_mode: Option<Cow<'a, str>>,
    /// Trace data format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'json').
    #[serde(skip_serializing_if = "Option::is_none", rename = "streamFormat")]
    stream_format: Option<StreamFormat>,
    /// Compression format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'none')
    #[serde(skip_serializing_if = "Option::is_none", rename = "streamCompression")]
    stream_compression: Option<StreamCompression>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "traceConfig")]
    trace_config: Option<TraceConfig<'a>>,
    /// Base64-encoded serialized perfetto.protos.TraceConfig protobuf message
    /// When specified, the parameters 'categories', 'options', 'traceConfig'
    /// are ignored. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none", rename = "perfettoConfig")]
    perfetto_config: Option<Cow<'a, str>>,
    /// Backend type (defaults to 'auto')
    #[serde(skip_serializing_if = "Option::is_none", rename = "tracingBackend")]
    tracing_backend: Option<TracingBackend>,
}

impl<'a> StartParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> StartParamsBuilder<'a> {
        StartParamsBuilder {
            categories: None,
            options: None,
            buffer_usage_reporting_interval: None,
            transfer_mode: None,
            stream_format: None,
            stream_compression: None,
            trace_config: None,
            perfetto_config: None,
            tracing_backend: None,
        }
    }
    /// Category/tag filter
    pub fn categories(&self) -> Option<&str> { self.categories.as_deref() }
    /// Tracing options
    pub fn options(&self) -> Option<&str> { self.options.as_deref() }
    /// If set, the agent will issue bufferUsage events at this interval, specified in milliseconds
    pub fn buffer_usage_reporting_interval(&self) -> Option<f64> { self.buffer_usage_reporting_interval }
    /// Whether to report trace events as series of dataCollected events or to save trace to a
    /// stream (defaults to 'ReportEvents').
    pub fn transfer_mode(&self) -> Option<&str> { self.transfer_mode.as_deref() }
    /// Trace data format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'json').
    pub fn stream_format(&self) -> Option<&StreamFormat> { self.stream_format.as_ref() }
    /// Compression format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'none')
    pub fn stream_compression(&self) -> Option<&StreamCompression> { self.stream_compression.as_ref() }
    pub fn trace_config(&self) -> Option<&TraceConfig<'a>> { self.trace_config.as_ref() }
    /// Base64-encoded serialized perfetto.protos.TraceConfig protobuf message
    /// When specified, the parameters 'categories', 'options', 'traceConfig'
    /// are ignored. (Encoded as a base64 string when passed over JSON)
    pub fn perfetto_config(&self) -> Option<&str> { self.perfetto_config.as_deref() }
    /// Backend type (defaults to 'auto')
    pub fn tracing_backend(&self) -> Option<&TracingBackend> { self.tracing_backend.as_ref() }
}

#[derive(Default)]
pub struct StartParamsBuilder<'a> {
    categories: Option<Cow<'a, str>>,
    options: Option<Cow<'a, str>>,
    buffer_usage_reporting_interval: Option<f64>,
    transfer_mode: Option<Cow<'a, str>>,
    stream_format: Option<StreamFormat>,
    stream_compression: Option<StreamCompression>,
    trace_config: Option<TraceConfig<'a>>,
    perfetto_config: Option<Cow<'a, str>>,
    tracing_backend: Option<TracingBackend>,
}

impl<'a> StartParamsBuilder<'a> {
    /// Category/tag filter
    pub fn categories(mut self, categories: impl Into<Cow<'a, str>>) -> Self { self.categories = Some(categories.into()); self }
    /// Tracing options
    pub fn options(mut self, options: impl Into<Cow<'a, str>>) -> Self { self.options = Some(options.into()); self }
    /// If set, the agent will issue bufferUsage events at this interval, specified in milliseconds
    pub fn buffer_usage_reporting_interval(mut self, buffer_usage_reporting_interval: f64) -> Self { self.buffer_usage_reporting_interval = Some(buffer_usage_reporting_interval); self }
    /// Whether to report trace events as series of dataCollected events or to save trace to a
    /// stream (defaults to 'ReportEvents').
    pub fn transfer_mode(mut self, transfer_mode: impl Into<Cow<'a, str>>) -> Self { self.transfer_mode = Some(transfer_mode.into()); self }
    /// Trace data format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'json').
    pub fn stream_format(mut self, stream_format: impl Into<StreamFormat>) -> Self { self.stream_format = Some(stream_format.into()); self }
    /// Compression format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'none')
    pub fn stream_compression(mut self, stream_compression: impl Into<StreamCompression>) -> Self { self.stream_compression = Some(stream_compression.into()); self }
    pub fn trace_config(mut self, trace_config: TraceConfig<'a>) -> Self { self.trace_config = Some(trace_config); self }
    /// Base64-encoded serialized perfetto.protos.TraceConfig protobuf message
    /// When specified, the parameters 'categories', 'options', 'traceConfig'
    /// are ignored. (Encoded as a base64 string when passed over JSON)
    pub fn perfetto_config(mut self, perfetto_config: impl Into<Cow<'a, str>>) -> Self { self.perfetto_config = Some(perfetto_config.into()); self }
    /// Backend type (defaults to 'auto')
    pub fn tracing_backend(mut self, tracing_backend: impl Into<TracingBackend>) -> Self { self.tracing_backend = Some(tracing_backend.into()); self }
    pub fn build(self) -> StartParams<'a> {
        StartParams {
            categories: self.categories,
            options: self.options,
            buffer_usage_reporting_interval: self.buffer_usage_reporting_interval,
            transfer_mode: self.transfer_mode,
            stream_format: self.stream_format,
            stream_compression: self.stream_compression,
            trace_config: self.trace_config,
            perfetto_config: self.perfetto_config,
            tracing_backend: self.tracing_backend,
        }
    }
}

impl<'a> StartParams<'a> { pub const METHOD: &'static str = "Tracing.start"; }

impl<'a> crate::CdpCommand<'a> for StartParams<'a> {
    const METHOD: &'static str = "Tracing.start";
    type Response = crate::EmptyReturns;
}
