use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Configuration for memory dump. Used only when "memory-infra" category is enabled.

pub type MemoryDumpConfig = serde_json::Map<String, JsonValue>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TraceConfig<'a> {
    /// Controls how the trace buffer stores data. The default is 'recordUntilFull'.
    #[serde(skip_serializing_if = "Option::is_none")]
    recordMode: Option<Cow<'a, str>>,
    /// Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value
    /// of 200 MB would be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    traceBufferSizeInKb: Option<f64>,
    /// Turns on JavaScript stack sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    enableSampling: Option<bool>,
    /// Turns on system tracing.
    #[serde(skip_serializing_if = "Option::is_none")]
    enableSystrace: Option<bool>,
    /// Turns on argument filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    enableArgumentFilter: Option<bool>,
    /// Included category filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    includedCategories: Option<Vec<Cow<'a, str>>>,
    /// Excluded category filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    excludedCategories: Option<Vec<Cow<'a, str>>>,
    /// Configuration to synthesize the delays in tracing.
    #[serde(skip_serializing_if = "Option::is_none")]
    syntheticDelays: Option<Vec<Cow<'a, str>>>,
    /// Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    memoryDumpConfig: Option<MemoryDumpConfig>,
}

impl<'a> TraceConfig<'a> {
    pub fn builder() -> TraceConfigBuilder<'a> { TraceConfigBuilder::default() }
    pub fn recordMode(&self) -> Option<&str> { self.recordMode.as_deref() }
    pub fn traceBufferSizeInKb(&self) -> Option<f64> { self.traceBufferSizeInKb }
    pub fn enableSampling(&self) -> Option<bool> { self.enableSampling }
    pub fn enableSystrace(&self) -> Option<bool> { self.enableSystrace }
    pub fn enableArgumentFilter(&self) -> Option<bool> { self.enableArgumentFilter }
    pub fn includedCategories(&self) -> Option<&[Cow<'a, str>]> { self.includedCategories.as_deref() }
    pub fn excludedCategories(&self) -> Option<&[Cow<'a, str>]> { self.excludedCategories.as_deref() }
    pub fn syntheticDelays(&self) -> Option<&[Cow<'a, str>]> { self.syntheticDelays.as_deref() }
    pub fn memoryDumpConfig(&self) -> Option<&MemoryDumpConfig> { self.memoryDumpConfig.as_ref() }
}

#[derive(Default)]
pub struct TraceConfigBuilder<'a> {
    recordMode: Option<Cow<'a, str>>,
    traceBufferSizeInKb: Option<f64>,
    enableSampling: Option<bool>,
    enableSystrace: Option<bool>,
    enableArgumentFilter: Option<bool>,
    includedCategories: Option<Vec<Cow<'a, str>>>,
    excludedCategories: Option<Vec<Cow<'a, str>>>,
    syntheticDelays: Option<Vec<Cow<'a, str>>>,
    memoryDumpConfig: Option<MemoryDumpConfig>,
}

impl<'a> TraceConfigBuilder<'a> {
    /// Controls how the trace buffer stores data. The default is 'recordUntilFull'.
    pub fn recordMode(mut self, recordMode: impl Into<Cow<'a, str>>) -> Self { self.recordMode = Some(recordMode.into()); self }
    /// Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value
    /// of 200 MB would be used.
    pub fn traceBufferSizeInKb(mut self, traceBufferSizeInKb: f64) -> Self { self.traceBufferSizeInKb = Some(traceBufferSizeInKb); self }
    /// Turns on JavaScript stack sampling.
    pub fn enableSampling(mut self, enableSampling: bool) -> Self { self.enableSampling = Some(enableSampling); self }
    /// Turns on system tracing.
    pub fn enableSystrace(mut self, enableSystrace: bool) -> Self { self.enableSystrace = Some(enableSystrace); self }
    /// Turns on argument filter.
    pub fn enableArgumentFilter(mut self, enableArgumentFilter: bool) -> Self { self.enableArgumentFilter = Some(enableArgumentFilter); self }
    /// Included category filters.
    pub fn includedCategories(mut self, includedCategories: Vec<Cow<'a, str>>) -> Self { self.includedCategories = Some(includedCategories); self }
    /// Excluded category filters.
    pub fn excludedCategories(mut self, excludedCategories: Vec<Cow<'a, str>>) -> Self { self.excludedCategories = Some(excludedCategories); self }
    /// Configuration to synthesize the delays in tracing.
    pub fn syntheticDelays(mut self, syntheticDelays: Vec<Cow<'a, str>>) -> Self { self.syntheticDelays = Some(syntheticDelays); self }
    /// Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.
    pub fn memoryDumpConfig(mut self, memoryDumpConfig: MemoryDumpConfig) -> Self { self.memoryDumpConfig = Some(memoryDumpConfig); self }
    pub fn build(self) -> TraceConfig<'a> {
        TraceConfig {
            recordMode: self.recordMode,
            traceBufferSizeInKb: self.traceBufferSizeInKb,
            enableSampling: self.enableSampling,
            enableSystrace: self.enableSystrace,
            enableArgumentFilter: self.enableArgumentFilter,
            includedCategories: self.includedCategories,
            excludedCategories: self.excludedCategories,
            syntheticDelays: self.syntheticDelays,
            memoryDumpConfig: self.memoryDumpConfig,
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

impl EndParams {
    pub fn builder() -> EndParamsBuilder {
        EndParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct EndParamsBuilder {}

impl EndParamsBuilder {
    pub fn build(self) -> EndParams {
        EndParams {}
    }
}

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
    pub fn builder() -> GetCategoriesReturnsBuilder<'a> { GetCategoriesReturnsBuilder::default() }
    pub fn categories(&self) -> &[Cow<'a, str>] { &self.categories }
}

#[derive(Default)]
pub struct GetCategoriesReturnsBuilder<'a> {
    categories: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GetCategoriesReturnsBuilder<'a> {
    /// A list of supported tracing categories.
    pub fn categories(mut self, categories: Vec<Cow<'a, str>>) -> Self { self.categories = Some(categories); self }
    pub fn build(self) -> GetCategoriesReturns<'a> {
        GetCategoriesReturns {
            categories: self.categories.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetCategoriesParams {}

impl GetCategoriesParams {
    pub fn builder() -> GetCategoriesParamsBuilder {
        GetCategoriesParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetCategoriesParamsBuilder {}

impl GetCategoriesParamsBuilder {
    pub fn build(self) -> GetCategoriesParams {
        GetCategoriesParams {}
    }
}

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
    pub fn builder() -> GetTrackEventDescriptorReturnsBuilder<'a> { GetTrackEventDescriptorReturnsBuilder::default() }
    pub fn descriptor(&self) -> &str { self.descriptor.as_ref() }
}

#[derive(Default)]
pub struct GetTrackEventDescriptorReturnsBuilder<'a> {
    descriptor: Option<Cow<'a, str>>,
}

impl<'a> GetTrackEventDescriptorReturnsBuilder<'a> {
    /// Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message. (Encoded as a base64 string when passed over JSON)
    pub fn descriptor(mut self, descriptor: impl Into<Cow<'a, str>>) -> Self { self.descriptor = Some(descriptor.into()); self }
    pub fn build(self) -> GetTrackEventDescriptorReturns<'a> {
        GetTrackEventDescriptorReturns {
            descriptor: self.descriptor.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTrackEventDescriptorParams {}

impl GetTrackEventDescriptorParams {
    pub fn builder() -> GetTrackEventDescriptorParamsBuilder {
        GetTrackEventDescriptorParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetTrackEventDescriptorParamsBuilder {}

impl GetTrackEventDescriptorParamsBuilder {
    pub fn build(self) -> GetTrackEventDescriptorParams {
        GetTrackEventDescriptorParams {}
    }
}

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
    syncId: Cow<'a, str>,
}

impl<'a> RecordClockSyncMarkerParams<'a> {
    pub fn builder() -> RecordClockSyncMarkerParamsBuilder<'a> { RecordClockSyncMarkerParamsBuilder::default() }
    pub fn syncId(&self) -> &str { self.syncId.as_ref() }
}

#[derive(Default)]
pub struct RecordClockSyncMarkerParamsBuilder<'a> {
    syncId: Option<Cow<'a, str>>,
}

impl<'a> RecordClockSyncMarkerParamsBuilder<'a> {
    /// The ID of this clock sync marker
    pub fn syncId(mut self, syncId: impl Into<Cow<'a, str>>) -> Self { self.syncId = Some(syncId.into()); self }
    pub fn build(self) -> RecordClockSyncMarkerParams<'a> {
        RecordClockSyncMarkerParams {
            syncId: self.syncId.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    levelOfDetail: Option<MemoryDumpLevelOfDetail>,
}

impl RequestMemoryDumpParams {
    pub fn builder() -> RequestMemoryDumpParamsBuilder { RequestMemoryDumpParamsBuilder::default() }
    pub fn deterministic(&self) -> Option<bool> { self.deterministic }
    pub fn levelOfDetail(&self) -> Option<&MemoryDumpLevelOfDetail> { self.levelOfDetail.as_ref() }
}

#[derive(Default)]
pub struct RequestMemoryDumpParamsBuilder {
    deterministic: Option<bool>,
    levelOfDetail: Option<MemoryDumpLevelOfDetail>,
}

impl RequestMemoryDumpParamsBuilder {
    /// Enables more deterministic results by forcing garbage collection
    pub fn deterministic(mut self, deterministic: bool) -> Self { self.deterministic = Some(deterministic); self }
    /// Specifies level of details in memory dump. Defaults to "detailed".
    pub fn levelOfDetail(mut self, levelOfDetail: MemoryDumpLevelOfDetail) -> Self { self.levelOfDetail = Some(levelOfDetail); self }
    pub fn build(self) -> RequestMemoryDumpParams {
        RequestMemoryDumpParams {
            deterministic: self.deterministic,
            levelOfDetail: self.levelOfDetail,
        }
    }
}

/// Request a global memory dump.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestMemoryDumpReturns<'a> {
    /// GUID of the resulting global memory dump.
    dumpGuid: Cow<'a, str>,
    /// True iff the global memory dump succeeded.
    success: bool,
}

impl<'a> RequestMemoryDumpReturns<'a> {
    pub fn builder() -> RequestMemoryDumpReturnsBuilder<'a> { RequestMemoryDumpReturnsBuilder::default() }
    pub fn dumpGuid(&self) -> &str { self.dumpGuid.as_ref() }
    pub fn success(&self) -> bool { self.success }
}

#[derive(Default)]
pub struct RequestMemoryDumpReturnsBuilder<'a> {
    dumpGuid: Option<Cow<'a, str>>,
    success: Option<bool>,
}

impl<'a> RequestMemoryDumpReturnsBuilder<'a> {
    /// GUID of the resulting global memory dump.
    pub fn dumpGuid(mut self, dumpGuid: impl Into<Cow<'a, str>>) -> Self { self.dumpGuid = Some(dumpGuid.into()); self }
    /// True iff the global memory dump succeeded.
    pub fn success(mut self, success: bool) -> Self { self.success = Some(success); self }
    pub fn build(self) -> RequestMemoryDumpReturns<'a> {
        RequestMemoryDumpReturns {
            dumpGuid: self.dumpGuid.unwrap_or_default(),
            success: self.success.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    bufferUsageReportingInterval: Option<f64>,
    /// Whether to report trace events as series of dataCollected events or to save trace to a
    /// stream (defaults to 'ReportEvents').
    #[serde(skip_serializing_if = "Option::is_none")]
    transferMode: Option<Cow<'a, str>>,
    /// Trace data format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'json').
    #[serde(skip_serializing_if = "Option::is_none")]
    streamFormat: Option<StreamFormat>,
    /// Compression format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'none')
    #[serde(skip_serializing_if = "Option::is_none")]
    streamCompression: Option<StreamCompression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traceConfig: Option<TraceConfig<'a>>,
    /// Base64-encoded serialized perfetto.protos.TraceConfig protobuf message
    /// When specified, the parameters 'categories', 'options', 'traceConfig'
    /// are ignored. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    perfettoConfig: Option<Cow<'a, str>>,
    /// Backend type (defaults to 'auto')
    #[serde(skip_serializing_if = "Option::is_none")]
    tracingBackend: Option<TracingBackend>,
}

impl<'a> StartParams<'a> {
    pub fn builder() -> StartParamsBuilder<'a> { StartParamsBuilder::default() }
    pub fn categories(&self) -> Option<&str> { self.categories.as_deref() }
    pub fn options(&self) -> Option<&str> { self.options.as_deref() }
    pub fn bufferUsageReportingInterval(&self) -> Option<f64> { self.bufferUsageReportingInterval }
    pub fn transferMode(&self) -> Option<&str> { self.transferMode.as_deref() }
    pub fn streamFormat(&self) -> Option<&StreamFormat> { self.streamFormat.as_ref() }
    pub fn streamCompression(&self) -> Option<&StreamCompression> { self.streamCompression.as_ref() }
    pub fn traceConfig(&self) -> Option<&TraceConfig<'a>> { self.traceConfig.as_ref() }
    pub fn perfettoConfig(&self) -> Option<&str> { self.perfettoConfig.as_deref() }
    pub fn tracingBackend(&self) -> Option<&TracingBackend> { self.tracingBackend.as_ref() }
}

#[derive(Default)]
pub struct StartParamsBuilder<'a> {
    categories: Option<Cow<'a, str>>,
    options: Option<Cow<'a, str>>,
    bufferUsageReportingInterval: Option<f64>,
    transferMode: Option<Cow<'a, str>>,
    streamFormat: Option<StreamFormat>,
    streamCompression: Option<StreamCompression>,
    traceConfig: Option<TraceConfig<'a>>,
    perfettoConfig: Option<Cow<'a, str>>,
    tracingBackend: Option<TracingBackend>,
}

impl<'a> StartParamsBuilder<'a> {
    /// Category/tag filter
    pub fn categories(mut self, categories: impl Into<Cow<'a, str>>) -> Self { self.categories = Some(categories.into()); self }
    /// Tracing options
    pub fn options(mut self, options: impl Into<Cow<'a, str>>) -> Self { self.options = Some(options.into()); self }
    /// If set, the agent will issue bufferUsage events at this interval, specified in milliseconds
    pub fn bufferUsageReportingInterval(mut self, bufferUsageReportingInterval: f64) -> Self { self.bufferUsageReportingInterval = Some(bufferUsageReportingInterval); self }
    /// Whether to report trace events as series of dataCollected events or to save trace to a
    /// stream (defaults to 'ReportEvents').
    pub fn transferMode(mut self, transferMode: impl Into<Cow<'a, str>>) -> Self { self.transferMode = Some(transferMode.into()); self }
    /// Trace data format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'json').
    pub fn streamFormat(mut self, streamFormat: StreamFormat) -> Self { self.streamFormat = Some(streamFormat); self }
    /// Compression format to use. This only applies when using 'ReturnAsStream'
    /// transfer mode (defaults to 'none')
    pub fn streamCompression(mut self, streamCompression: StreamCompression) -> Self { self.streamCompression = Some(streamCompression); self }
    pub fn traceConfig(mut self, traceConfig: TraceConfig<'a>) -> Self { self.traceConfig = Some(traceConfig); self }
    /// Base64-encoded serialized perfetto.protos.TraceConfig protobuf message
    /// When specified, the parameters 'categories', 'options', 'traceConfig'
    /// are ignored. (Encoded as a base64 string when passed over JSON)
    pub fn perfettoConfig(mut self, perfettoConfig: impl Into<Cow<'a, str>>) -> Self { self.perfettoConfig = Some(perfettoConfig.into()); self }
    /// Backend type (defaults to 'auto')
    pub fn tracingBackend(mut self, tracingBackend: TracingBackend) -> Self { self.tracingBackend = Some(tracingBackend); self }
    pub fn build(self) -> StartParams<'a> {
        StartParams {
            categories: self.categories,
            options: self.options,
            bufferUsageReportingInterval: self.bufferUsageReportingInterval,
            transferMode: self.transferMode,
            streamFormat: self.streamFormat,
            streamCompression: self.streamCompression,
            traceConfig: self.traceConfig,
            perfettoConfig: self.perfettoConfig,
            tracingBackend: self.tracingBackend,
        }
    }
}

impl<'a> StartParams<'a> { pub const METHOD: &'static str = "Tracing.start"; }

impl<'a> crate::CdpCommand<'a> for StartParams<'a> {
    const METHOD: &'static str = "Tracing.start";
    type Response = crate::EmptyReturns;
}
