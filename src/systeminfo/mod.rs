//! The SystemInfo domain defines methods and events for querying low-level system information.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Describes a single graphics processor (GPU).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GPUDevice<'a> {
    /// PCI ID of the GPU vendor, if available; 0 otherwise.
    vendorId: f64,
    /// PCI ID of the GPU device, if available; 0 otherwise.
    deviceId: f64,
    /// Sub sys ID of the GPU, only available on Windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    subSysId: Option<f64>,
    /// Revision of the GPU, only available on Windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<f64>,
    /// String description of the GPU vendor, if the PCI ID is not available.
    vendorString: Cow<'a, str>,
    /// String description of the GPU device, if the PCI ID is not available.
    deviceString: Cow<'a, str>,
    /// String description of the GPU driver vendor.
    driverVendor: Cow<'a, str>,
    /// String description of the GPU driver version.
    driverVersion: Cow<'a, str>,
}

impl<'a> GPUDevice<'a> {
    pub fn builder() -> GPUDeviceBuilder<'a> { GPUDeviceBuilder::default() }
    pub fn vendorId(&self) -> f64 { self.vendorId }
    pub fn deviceId(&self) -> f64 { self.deviceId }
    pub fn subSysId(&self) -> Option<f64> { self.subSysId }
    pub fn revision(&self) -> Option<f64> { self.revision }
    pub fn vendorString(&self) -> &str { self.vendorString.as_ref() }
    pub fn deviceString(&self) -> &str { self.deviceString.as_ref() }
    pub fn driverVendor(&self) -> &str { self.driverVendor.as_ref() }
    pub fn driverVersion(&self) -> &str { self.driverVersion.as_ref() }
}

#[derive(Default)]
pub struct GPUDeviceBuilder<'a> {
    vendorId: Option<f64>,
    deviceId: Option<f64>,
    subSysId: Option<f64>,
    revision: Option<f64>,
    vendorString: Option<Cow<'a, str>>,
    deviceString: Option<Cow<'a, str>>,
    driverVendor: Option<Cow<'a, str>>,
    driverVersion: Option<Cow<'a, str>>,
}

impl<'a> GPUDeviceBuilder<'a> {
    /// PCI ID of the GPU vendor, if available; 0 otherwise.
    pub fn vendorId(mut self, vendorId: f64) -> Self { self.vendorId = Some(vendorId); self }
    /// PCI ID of the GPU device, if available; 0 otherwise.
    pub fn deviceId(mut self, deviceId: f64) -> Self { self.deviceId = Some(deviceId); self }
    /// Sub sys ID of the GPU, only available on Windows.
    pub fn subSysId(mut self, subSysId: f64) -> Self { self.subSysId = Some(subSysId); self }
    /// Revision of the GPU, only available on Windows.
    pub fn revision(mut self, revision: f64) -> Self { self.revision = Some(revision); self }
    /// String description of the GPU vendor, if the PCI ID is not available.
    pub fn vendorString(mut self, vendorString: impl Into<Cow<'a, str>>) -> Self { self.vendorString = Some(vendorString.into()); self }
    /// String description of the GPU device, if the PCI ID is not available.
    pub fn deviceString(mut self, deviceString: impl Into<Cow<'a, str>>) -> Self { self.deviceString = Some(deviceString.into()); self }
    /// String description of the GPU driver vendor.
    pub fn driverVendor(mut self, driverVendor: impl Into<Cow<'a, str>>) -> Self { self.driverVendor = Some(driverVendor.into()); self }
    /// String description of the GPU driver version.
    pub fn driverVersion(mut self, driverVersion: impl Into<Cow<'a, str>>) -> Self { self.driverVersion = Some(driverVersion.into()); self }
    pub fn build(self) -> GPUDevice<'a> {
        GPUDevice {
            vendorId: self.vendorId.unwrap_or_default(),
            deviceId: self.deviceId.unwrap_or_default(),
            subSysId: self.subSysId,
            revision: self.revision,
            vendorString: self.vendorString.unwrap_or_default(),
            deviceString: self.deviceString.unwrap_or_default(),
            driverVendor: self.driverVendor.unwrap_or_default(),
            driverVersion: self.driverVersion.unwrap_or_default(),
        }
    }
}

/// Describes the width and height dimensions of an entity.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    /// Width in pixels.
    width: u64,
    /// Height in pixels.
    height: i64,
}

impl Size {
    pub fn builder() -> SizeBuilder { SizeBuilder::default() }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> i64 { self.height }
}

#[derive(Default)]
pub struct SizeBuilder {
    width: Option<u64>,
    height: Option<i64>,
}

impl SizeBuilder {
    /// Width in pixels.
    pub fn width(mut self, width: u64) -> Self { self.width = Some(width); self }
    /// Height in pixels.
    pub fn height(mut self, height: i64) -> Self { self.height = Some(height); self }
    pub fn build(self) -> Size {
        Size {
            width: self.width.unwrap_or_default(),
            height: self.height.unwrap_or_default(),
        }
    }
}

/// Describes a supported video decoding profile with its associated minimum and
/// maximum resolutions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoDecodeAcceleratorCapability<'a> {
    /// Video codec profile that is supported, e.g. VP9 Profile 2.
    profile: Cow<'a, str>,
    /// Maximum video dimensions in pixels supported for this |profile|.
    maxResolution: Size,
    /// Minimum video dimensions in pixels supported for this |profile|.
    minResolution: Size,
}

impl<'a> VideoDecodeAcceleratorCapability<'a> {
    pub fn builder() -> VideoDecodeAcceleratorCapabilityBuilder<'a> { VideoDecodeAcceleratorCapabilityBuilder::default() }
    pub fn profile(&self) -> &str { self.profile.as_ref() }
    pub fn maxResolution(&self) -> &Size { &self.maxResolution }
    pub fn minResolution(&self) -> &Size { &self.minResolution }
}

#[derive(Default)]
pub struct VideoDecodeAcceleratorCapabilityBuilder<'a> {
    profile: Option<Cow<'a, str>>,
    maxResolution: Option<Size>,
    minResolution: Option<Size>,
}

impl<'a> VideoDecodeAcceleratorCapabilityBuilder<'a> {
    /// Video codec profile that is supported, e.g. VP9 Profile 2.
    pub fn profile(mut self, profile: impl Into<Cow<'a, str>>) -> Self { self.profile = Some(profile.into()); self }
    /// Maximum video dimensions in pixels supported for this |profile|.
    pub fn maxResolution(mut self, maxResolution: Size) -> Self { self.maxResolution = Some(maxResolution); self }
    /// Minimum video dimensions in pixels supported for this |profile|.
    pub fn minResolution(mut self, minResolution: Size) -> Self { self.minResolution = Some(minResolution); self }
    pub fn build(self) -> VideoDecodeAcceleratorCapability<'a> {
        VideoDecodeAcceleratorCapability {
            profile: self.profile.unwrap_or_default(),
            maxResolution: self.maxResolution.unwrap_or_default(),
            minResolution: self.minResolution.unwrap_or_default(),
        }
    }
}

/// Describes a supported video encoding profile with its associated maximum
/// resolution and maximum framerate.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoEncodeAcceleratorCapability<'a> {
    /// Video codec profile that is supported, e.g H264 Main.
    profile: Cow<'a, str>,
    /// Maximum video dimensions in pixels supported for this |profile|.
    maxResolution: Size,
    /// Maximum encoding framerate in frames per second supported for this
    /// |profile|, as fraction's numerator and denominator, e.g. 24/1 fps,
    /// 24000/1001 fps, etc.
    maxFramerateNumerator: i64,
    maxFramerateDenominator: i64,
}

impl<'a> VideoEncodeAcceleratorCapability<'a> {
    pub fn builder() -> VideoEncodeAcceleratorCapabilityBuilder<'a> { VideoEncodeAcceleratorCapabilityBuilder::default() }
    pub fn profile(&self) -> &str { self.profile.as_ref() }
    pub fn maxResolution(&self) -> &Size { &self.maxResolution }
    pub fn maxFramerateNumerator(&self) -> i64 { self.maxFramerateNumerator }
    pub fn maxFramerateDenominator(&self) -> i64 { self.maxFramerateDenominator }
}

#[derive(Default)]
pub struct VideoEncodeAcceleratorCapabilityBuilder<'a> {
    profile: Option<Cow<'a, str>>,
    maxResolution: Option<Size>,
    maxFramerateNumerator: Option<i64>,
    maxFramerateDenominator: Option<i64>,
}

impl<'a> VideoEncodeAcceleratorCapabilityBuilder<'a> {
    /// Video codec profile that is supported, e.g H264 Main.
    pub fn profile(mut self, profile: impl Into<Cow<'a, str>>) -> Self { self.profile = Some(profile.into()); self }
    /// Maximum video dimensions in pixels supported for this |profile|.
    pub fn maxResolution(mut self, maxResolution: Size) -> Self { self.maxResolution = Some(maxResolution); self }
    /// Maximum encoding framerate in frames per second supported for this
    /// |profile|, as fraction's numerator and denominator, e.g. 24/1 fps,
    /// 24000/1001 fps, etc.
    pub fn maxFramerateNumerator(mut self, maxFramerateNumerator: i64) -> Self { self.maxFramerateNumerator = Some(maxFramerateNumerator); self }
    pub fn maxFramerateDenominator(mut self, maxFramerateDenominator: i64) -> Self { self.maxFramerateDenominator = Some(maxFramerateDenominator); self }
    pub fn build(self) -> VideoEncodeAcceleratorCapability<'a> {
        VideoEncodeAcceleratorCapability {
            profile: self.profile.unwrap_or_default(),
            maxResolution: self.maxResolution.unwrap_or_default(),
            maxFramerateNumerator: self.maxFramerateNumerator.unwrap_or_default(),
            maxFramerateDenominator: self.maxFramerateDenominator.unwrap_or_default(),
        }
    }
}

/// YUV subsampling type of the pixels of a given image.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SubsamplingFormat {
    #[default]
    #[serde(rename = "yuv420")]
    Yuv420,
    #[serde(rename = "yuv422")]
    Yuv422,
    #[serde(rename = "yuv444")]
    Yuv444,
}

/// Image format of a given image.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ImageType {
    #[default]
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "unknown")]
    Unknown,
}

/// Provides information about the GPU(s) on the system.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GPUInfo<'a> {
    /// The graphics devices on the system. Element 0 is the primary GPU.
    devices: Vec<GPUDevice<'a>>,
    /// An optional dictionary of additional GPU related attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    auxAttributes: Option<serde_json::Map<String, JsonValue>>,
    /// An optional dictionary of graphics features and their status.
    #[serde(skip_serializing_if = "Option::is_none")]
    featureStatus: Option<serde_json::Map<String, JsonValue>>,
    /// An optional array of GPU driver bug workarounds.
    driverBugWorkarounds: Vec<Cow<'a, str>>,
    /// Supported accelerated video decoding capabilities.
    videoDecoding: Vec<VideoDecodeAcceleratorCapability<'a>>,
    /// Supported accelerated video encoding capabilities.
    videoEncoding: Vec<VideoEncodeAcceleratorCapability<'a>>,
}

impl<'a> GPUInfo<'a> {
    pub fn builder() -> GPUInfoBuilder<'a> { GPUInfoBuilder::default() }
    pub fn devices(&self) -> &[GPUDevice<'a>] { &self.devices }
    pub fn auxAttributes(&self) -> Option<&serde_json::Map<String, JsonValue>> { self.auxAttributes.as_ref() }
    pub fn featureStatus(&self) -> Option<&serde_json::Map<String, JsonValue>> { self.featureStatus.as_ref() }
    pub fn driverBugWorkarounds(&self) -> &[Cow<'a, str>] { &self.driverBugWorkarounds }
    pub fn videoDecoding(&self) -> &[VideoDecodeAcceleratorCapability<'a>] { &self.videoDecoding }
    pub fn videoEncoding(&self) -> &[VideoEncodeAcceleratorCapability<'a>] { &self.videoEncoding }
}

#[derive(Default)]
pub struct GPUInfoBuilder<'a> {
    devices: Option<Vec<GPUDevice<'a>>>,
    auxAttributes: Option<serde_json::Map<String, JsonValue>>,
    featureStatus: Option<serde_json::Map<String, JsonValue>>,
    driverBugWorkarounds: Option<Vec<Cow<'a, str>>>,
    videoDecoding: Option<Vec<VideoDecodeAcceleratorCapability<'a>>>,
    videoEncoding: Option<Vec<VideoEncodeAcceleratorCapability<'a>>>,
}

impl<'a> GPUInfoBuilder<'a> {
    /// The graphics devices on the system. Element 0 is the primary GPU.
    pub fn devices(mut self, devices: Vec<GPUDevice<'a>>) -> Self { self.devices = Some(devices); self }
    /// An optional dictionary of additional GPU related attributes.
    pub fn auxAttributes(mut self, auxAttributes: serde_json::Map<String, JsonValue>) -> Self { self.auxAttributes = Some(auxAttributes); self }
    /// An optional dictionary of graphics features and their status.
    pub fn featureStatus(mut self, featureStatus: serde_json::Map<String, JsonValue>) -> Self { self.featureStatus = Some(featureStatus); self }
    /// An optional array of GPU driver bug workarounds.
    pub fn driverBugWorkarounds(mut self, driverBugWorkarounds: Vec<Cow<'a, str>>) -> Self { self.driverBugWorkarounds = Some(driverBugWorkarounds); self }
    /// Supported accelerated video decoding capabilities.
    pub fn videoDecoding(mut self, videoDecoding: Vec<VideoDecodeAcceleratorCapability<'a>>) -> Self { self.videoDecoding = Some(videoDecoding); self }
    /// Supported accelerated video encoding capabilities.
    pub fn videoEncoding(mut self, videoEncoding: Vec<VideoEncodeAcceleratorCapability<'a>>) -> Self { self.videoEncoding = Some(videoEncoding); self }
    pub fn build(self) -> GPUInfo<'a> {
        GPUInfo {
            devices: self.devices.unwrap_or_default(),
            auxAttributes: self.auxAttributes,
            featureStatus: self.featureStatus,
            driverBugWorkarounds: self.driverBugWorkarounds.unwrap_or_default(),
            videoDecoding: self.videoDecoding.unwrap_or_default(),
            videoEncoding: self.videoEncoding.unwrap_or_default(),
        }
    }
}

/// Represents process info.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo<'a> {
    /// Specifies process type.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Specifies process id.
    id: u64,
    /// Specifies cumulative CPU usage in seconds across all threads of the
    /// process since the process start.
    cpuTime: f64,
}

impl<'a> ProcessInfo<'a> {
    pub fn builder() -> ProcessInfoBuilder<'a> { ProcessInfoBuilder::default() }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn id(&self) -> u64 { self.id }
    pub fn cpuTime(&self) -> f64 { self.cpuTime }
}

#[derive(Default)]
pub struct ProcessInfoBuilder<'a> {
    type_: Option<Cow<'a, str>>,
    id: Option<u64>,
    cpuTime: Option<f64>,
}

impl<'a> ProcessInfoBuilder<'a> {
    /// Specifies process type.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    /// Specifies process id.
    pub fn id(mut self, id: u64) -> Self { self.id = Some(id); self }
    /// Specifies cumulative CPU usage in seconds across all threads of the
    /// process since the process start.
    pub fn cpuTime(mut self, cpuTime: f64) -> Self { self.cpuTime = Some(cpuTime); self }
    pub fn build(self) -> ProcessInfo<'a> {
        ProcessInfo {
            type_: self.type_.unwrap_or_default(),
            id: self.id.unwrap_or_default(),
            cpuTime: self.cpuTime.unwrap_or_default(),
        }
    }
}

/// Returns information about the system.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoReturns<'a> {
    /// Information about the GPUs on the system.
    gpu: GPUInfo<'a>,
    /// A platform-dependent description of the model of the machine. On Mac OS, this is, for
    /// example, 'MacBookPro'. Will be the empty string if not supported.
    modelName: Cow<'a, str>,
    /// A platform-dependent description of the version of the machine. On Mac OS, this is, for
    /// example, '10.1'. Will be the empty string if not supported.
    modelVersion: Cow<'a, str>,
    /// The command line string used to launch the browser. Will be the empty string if not
    /// supported.
    commandLine: Cow<'a, str>,
}

impl<'a> GetInfoReturns<'a> {
    pub fn builder() -> GetInfoReturnsBuilder<'a> { GetInfoReturnsBuilder::default() }
    pub fn gpu(&self) -> &GPUInfo<'a> { &self.gpu }
    pub fn modelName(&self) -> &str { self.modelName.as_ref() }
    pub fn modelVersion(&self) -> &str { self.modelVersion.as_ref() }
    pub fn commandLine(&self) -> &str { self.commandLine.as_ref() }
}

#[derive(Default)]
pub struct GetInfoReturnsBuilder<'a> {
    gpu: Option<GPUInfo<'a>>,
    modelName: Option<Cow<'a, str>>,
    modelVersion: Option<Cow<'a, str>>,
    commandLine: Option<Cow<'a, str>>,
}

impl<'a> GetInfoReturnsBuilder<'a> {
    /// Information about the GPUs on the system.
    pub fn gpu(mut self, gpu: GPUInfo<'a>) -> Self { self.gpu = Some(gpu); self }
    /// A platform-dependent description of the model of the machine. On Mac OS, this is, for
    /// example, 'MacBookPro'. Will be the empty string if not supported.
    pub fn modelName(mut self, modelName: impl Into<Cow<'a, str>>) -> Self { self.modelName = Some(modelName.into()); self }
    /// A platform-dependent description of the version of the machine. On Mac OS, this is, for
    /// example, '10.1'. Will be the empty string if not supported.
    pub fn modelVersion(mut self, modelVersion: impl Into<Cow<'a, str>>) -> Self { self.modelVersion = Some(modelVersion.into()); self }
    /// The command line string used to launch the browser. Will be the empty string if not
    /// supported.
    pub fn commandLine(mut self, commandLine: impl Into<Cow<'a, str>>) -> Self { self.commandLine = Some(commandLine.into()); self }
    pub fn build(self) -> GetInfoReturns<'a> {
        GetInfoReturns {
            gpu: self.gpu.unwrap_or_default(),
            modelName: self.modelName.unwrap_or_default(),
            modelVersion: self.modelVersion.unwrap_or_default(),
            commandLine: self.commandLine.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetInfoParams {}

impl GetInfoParams {
    pub fn builder() -> GetInfoParamsBuilder {
        GetInfoParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetInfoParamsBuilder {}

impl GetInfoParamsBuilder {
    pub fn build(self) -> GetInfoParams {
        GetInfoParams {}
    }
}

impl GetInfoParams { pub const METHOD: &'static str = "SystemInfo.getInfo"; }

impl<'a> crate::CdpCommand<'a> for GetInfoParams {
    const METHOD: &'static str = "SystemInfo.getInfo";
    type Response = GetInfoReturns<'a>;
}

/// Returns information about the feature state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeatureStateParams<'a> {
    featureState: Cow<'a, str>,
}

impl<'a> GetFeatureStateParams<'a> {
    pub fn builder() -> GetFeatureStateParamsBuilder<'a> { GetFeatureStateParamsBuilder::default() }
    pub fn featureState(&self) -> &str { self.featureState.as_ref() }
}

#[derive(Default)]
pub struct GetFeatureStateParamsBuilder<'a> {
    featureState: Option<Cow<'a, str>>,
}

impl<'a> GetFeatureStateParamsBuilder<'a> {
    pub fn featureState(mut self, featureState: impl Into<Cow<'a, str>>) -> Self { self.featureState = Some(featureState.into()); self }
    pub fn build(self) -> GetFeatureStateParams<'a> {
        GetFeatureStateParams {
            featureState: self.featureState.unwrap_or_default(),
        }
    }
}

/// Returns information about the feature state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeatureStateReturns {
    featureEnabled: bool,
}

impl GetFeatureStateReturns {
    pub fn builder() -> GetFeatureStateReturnsBuilder { GetFeatureStateReturnsBuilder::default() }
    pub fn featureEnabled(&self) -> bool { self.featureEnabled }
}

#[derive(Default)]
pub struct GetFeatureStateReturnsBuilder {
    featureEnabled: Option<bool>,
}

impl GetFeatureStateReturnsBuilder {
    pub fn featureEnabled(mut self, featureEnabled: bool) -> Self { self.featureEnabled = Some(featureEnabled); self }
    pub fn build(self) -> GetFeatureStateReturns {
        GetFeatureStateReturns {
            featureEnabled: self.featureEnabled.unwrap_or_default(),
        }
    }
}

impl<'a> GetFeatureStateParams<'a> { pub const METHOD: &'static str = "SystemInfo.getFeatureState"; }

impl<'a> crate::CdpCommand<'a> for GetFeatureStateParams<'a> {
    const METHOD: &'static str = "SystemInfo.getFeatureState";
    type Response = GetFeatureStateReturns;
}

/// Returns information about all running processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessInfoReturns<'a> {
    /// An array of process info blocks.
    processInfo: Vec<ProcessInfo<'a>>,
}

impl<'a> GetProcessInfoReturns<'a> {
    pub fn builder() -> GetProcessInfoReturnsBuilder<'a> { GetProcessInfoReturnsBuilder::default() }
    pub fn processInfo(&self) -> &[ProcessInfo<'a>] { &self.processInfo }
}

#[derive(Default)]
pub struct GetProcessInfoReturnsBuilder<'a> {
    processInfo: Option<Vec<ProcessInfo<'a>>>,
}

impl<'a> GetProcessInfoReturnsBuilder<'a> {
    /// An array of process info blocks.
    pub fn processInfo(mut self, processInfo: Vec<ProcessInfo<'a>>) -> Self { self.processInfo = Some(processInfo); self }
    pub fn build(self) -> GetProcessInfoReturns<'a> {
        GetProcessInfoReturns {
            processInfo: self.processInfo.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetProcessInfoParams {}

impl GetProcessInfoParams {
    pub fn builder() -> GetProcessInfoParamsBuilder {
        GetProcessInfoParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetProcessInfoParamsBuilder {}

impl GetProcessInfoParamsBuilder {
    pub fn build(self) -> GetProcessInfoParams {
        GetProcessInfoParams {}
    }
}

impl GetProcessInfoParams { pub const METHOD: &'static str = "SystemInfo.getProcessInfo"; }

impl<'a> crate::CdpCommand<'a> for GetProcessInfoParams {
    const METHOD: &'static str = "SystemInfo.getProcessInfo";
    type Response = GetProcessInfoReturns<'a>;
}
