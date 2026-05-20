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
    pub fn builder(vendorId: f64, deviceId: f64, vendorString: impl Into<Cow<'a, str>>, deviceString: impl Into<Cow<'a, str>>, driverVendor: impl Into<Cow<'a, str>>, driverVersion: impl Into<Cow<'a, str>>) -> GPUDeviceBuilder<'a> {
        GPUDeviceBuilder {
            vendorId: vendorId,
            deviceId: deviceId,
            subSysId: None,
            revision: None,
            vendorString: vendorString.into(),
            deviceString: deviceString.into(),
            driverVendor: driverVendor.into(),
            driverVersion: driverVersion.into(),
        }
    }
    pub fn vendorId(&self) -> f64 { self.vendorId }
    pub fn deviceId(&self) -> f64 { self.deviceId }
    pub fn subSysId(&self) -> Option<f64> { self.subSysId }
    pub fn revision(&self) -> Option<f64> { self.revision }
    pub fn vendorString(&self) -> &str { self.vendorString.as_ref() }
    pub fn deviceString(&self) -> &str { self.deviceString.as_ref() }
    pub fn driverVendor(&self) -> &str { self.driverVendor.as_ref() }
    pub fn driverVersion(&self) -> &str { self.driverVersion.as_ref() }
}


pub struct GPUDeviceBuilder<'a> {
    vendorId: f64,
    deviceId: f64,
    subSysId: Option<f64>,
    revision: Option<f64>,
    vendorString: Cow<'a, str>,
    deviceString: Cow<'a, str>,
    driverVendor: Cow<'a, str>,
    driverVersion: Cow<'a, str>,
}

impl<'a> GPUDeviceBuilder<'a> {
    /// Sub sys ID of the GPU, only available on Windows.
    pub fn subSysId(mut self, subSysId: f64) -> Self { self.subSysId = Some(subSysId); self }
    /// Revision of the GPU, only available on Windows.
    pub fn revision(mut self, revision: f64) -> Self { self.revision = Some(revision); self }
    pub fn build(self) -> GPUDevice<'a> {
        GPUDevice {
            vendorId: self.vendorId,
            deviceId: self.deviceId,
            subSysId: self.subSysId,
            revision: self.revision,
            vendorString: self.vendorString,
            deviceString: self.deviceString,
            driverVendor: self.driverVendor,
            driverVersion: self.driverVersion,
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
    pub fn builder(width: u64, height: i64) -> SizeBuilder {
        SizeBuilder {
            width: width,
            height: height,
        }
    }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> i64 { self.height }
}


pub struct SizeBuilder {
    width: u64,
    height: i64,
}

impl SizeBuilder {
    pub fn build(self) -> Size {
        Size {
            width: self.width,
            height: self.height,
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
    pub fn builder(profile: impl Into<Cow<'a, str>>, maxResolution: Size, minResolution: Size) -> VideoDecodeAcceleratorCapabilityBuilder<'a> {
        VideoDecodeAcceleratorCapabilityBuilder {
            profile: profile.into(),
            maxResolution: maxResolution,
            minResolution: minResolution,
        }
    }
    pub fn profile(&self) -> &str { self.profile.as_ref() }
    pub fn maxResolution(&self) -> &Size { &self.maxResolution }
    pub fn minResolution(&self) -> &Size { &self.minResolution }
}


pub struct VideoDecodeAcceleratorCapabilityBuilder<'a> {
    profile: Cow<'a, str>,
    maxResolution: Size,
    minResolution: Size,
}

impl<'a> VideoDecodeAcceleratorCapabilityBuilder<'a> {
    pub fn build(self) -> VideoDecodeAcceleratorCapability<'a> {
        VideoDecodeAcceleratorCapability {
            profile: self.profile,
            maxResolution: self.maxResolution,
            minResolution: self.minResolution,
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
    pub fn builder(profile: impl Into<Cow<'a, str>>, maxResolution: Size, maxFramerateNumerator: i64, maxFramerateDenominator: i64) -> VideoEncodeAcceleratorCapabilityBuilder<'a> {
        VideoEncodeAcceleratorCapabilityBuilder {
            profile: profile.into(),
            maxResolution: maxResolution,
            maxFramerateNumerator: maxFramerateNumerator,
            maxFramerateDenominator: maxFramerateDenominator,
        }
    }
    pub fn profile(&self) -> &str { self.profile.as_ref() }
    pub fn maxResolution(&self) -> &Size { &self.maxResolution }
    pub fn maxFramerateNumerator(&self) -> i64 { self.maxFramerateNumerator }
    pub fn maxFramerateDenominator(&self) -> i64 { self.maxFramerateDenominator }
}


pub struct VideoEncodeAcceleratorCapabilityBuilder<'a> {
    profile: Cow<'a, str>,
    maxResolution: Size,
    maxFramerateNumerator: i64,
    maxFramerateDenominator: i64,
}

impl<'a> VideoEncodeAcceleratorCapabilityBuilder<'a> {
    pub fn build(self) -> VideoEncodeAcceleratorCapability<'a> {
        VideoEncodeAcceleratorCapability {
            profile: self.profile,
            maxResolution: self.maxResolution,
            maxFramerateNumerator: self.maxFramerateNumerator,
            maxFramerateDenominator: self.maxFramerateDenominator,
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
    pub fn builder(devices: Vec<GPUDevice<'a>>, driverBugWorkarounds: Vec<Cow<'a, str>>, videoDecoding: Vec<VideoDecodeAcceleratorCapability<'a>>, videoEncoding: Vec<VideoEncodeAcceleratorCapability<'a>>) -> GPUInfoBuilder<'a> {
        GPUInfoBuilder {
            devices: devices,
            auxAttributes: None,
            featureStatus: None,
            driverBugWorkarounds: driverBugWorkarounds,
            videoDecoding: videoDecoding,
            videoEncoding: videoEncoding,
        }
    }
    pub fn devices(&self) -> &[GPUDevice<'a>] { &self.devices }
    pub fn auxAttributes(&self) -> Option<&serde_json::Map<String, JsonValue>> { self.auxAttributes.as_ref() }
    pub fn featureStatus(&self) -> Option<&serde_json::Map<String, JsonValue>> { self.featureStatus.as_ref() }
    pub fn driverBugWorkarounds(&self) -> &[Cow<'a, str>] { &self.driverBugWorkarounds }
    pub fn videoDecoding(&self) -> &[VideoDecodeAcceleratorCapability<'a>] { &self.videoDecoding }
    pub fn videoEncoding(&self) -> &[VideoEncodeAcceleratorCapability<'a>] { &self.videoEncoding }
}


pub struct GPUInfoBuilder<'a> {
    devices: Vec<GPUDevice<'a>>,
    auxAttributes: Option<serde_json::Map<String, JsonValue>>,
    featureStatus: Option<serde_json::Map<String, JsonValue>>,
    driverBugWorkarounds: Vec<Cow<'a, str>>,
    videoDecoding: Vec<VideoDecodeAcceleratorCapability<'a>>,
    videoEncoding: Vec<VideoEncodeAcceleratorCapability<'a>>,
}

impl<'a> GPUInfoBuilder<'a> {
    /// An optional dictionary of additional GPU related attributes.
    pub fn auxAttributes(mut self, auxAttributes: serde_json::Map<String, JsonValue>) -> Self { self.auxAttributes = Some(auxAttributes); self }
    /// An optional dictionary of graphics features and their status.
    pub fn featureStatus(mut self, featureStatus: serde_json::Map<String, JsonValue>) -> Self { self.featureStatus = Some(featureStatus); self }
    pub fn build(self) -> GPUInfo<'a> {
        GPUInfo {
            devices: self.devices,
            auxAttributes: self.auxAttributes,
            featureStatus: self.featureStatus,
            driverBugWorkarounds: self.driverBugWorkarounds,
            videoDecoding: self.videoDecoding,
            videoEncoding: self.videoEncoding,
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
    pub fn builder(type_: impl Into<Cow<'a, str>>, id: u64, cpuTime: f64) -> ProcessInfoBuilder<'a> {
        ProcessInfoBuilder {
            type_: type_.into(),
            id: id,
            cpuTime: cpuTime,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn id(&self) -> u64 { self.id }
    pub fn cpuTime(&self) -> f64 { self.cpuTime }
}


pub struct ProcessInfoBuilder<'a> {
    type_: Cow<'a, str>,
    id: u64,
    cpuTime: f64,
}

impl<'a> ProcessInfoBuilder<'a> {
    pub fn build(self) -> ProcessInfo<'a> {
        ProcessInfo {
            type_: self.type_,
            id: self.id,
            cpuTime: self.cpuTime,
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
    pub fn builder(gpu: GPUInfo<'a>, modelName: impl Into<Cow<'a, str>>, modelVersion: impl Into<Cow<'a, str>>, commandLine: impl Into<Cow<'a, str>>) -> GetInfoReturnsBuilder<'a> {
        GetInfoReturnsBuilder {
            gpu: gpu,
            modelName: modelName.into(),
            modelVersion: modelVersion.into(),
            commandLine: commandLine.into(),
        }
    }
    pub fn gpu(&self) -> &GPUInfo<'a> { &self.gpu }
    pub fn modelName(&self) -> &str { self.modelName.as_ref() }
    pub fn modelVersion(&self) -> &str { self.modelVersion.as_ref() }
    pub fn commandLine(&self) -> &str { self.commandLine.as_ref() }
}


pub struct GetInfoReturnsBuilder<'a> {
    gpu: GPUInfo<'a>,
    modelName: Cow<'a, str>,
    modelVersion: Cow<'a, str>,
    commandLine: Cow<'a, str>,
}

impl<'a> GetInfoReturnsBuilder<'a> {
    pub fn build(self) -> GetInfoReturns<'a> {
        GetInfoReturns {
            gpu: self.gpu,
            modelName: self.modelName,
            modelVersion: self.modelVersion,
            commandLine: self.commandLine,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetInfoParams {}

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
    pub fn builder(featureState: impl Into<Cow<'a, str>>) -> GetFeatureStateParamsBuilder<'a> {
        GetFeatureStateParamsBuilder {
            featureState: featureState.into(),
        }
    }
    pub fn featureState(&self) -> &str { self.featureState.as_ref() }
}


pub struct GetFeatureStateParamsBuilder<'a> {
    featureState: Cow<'a, str>,
}

impl<'a> GetFeatureStateParamsBuilder<'a> {
    pub fn build(self) -> GetFeatureStateParams<'a> {
        GetFeatureStateParams {
            featureState: self.featureState,
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
    pub fn builder(featureEnabled: bool) -> GetFeatureStateReturnsBuilder {
        GetFeatureStateReturnsBuilder {
            featureEnabled: featureEnabled,
        }
    }
    pub fn featureEnabled(&self) -> bool { self.featureEnabled }
}


pub struct GetFeatureStateReturnsBuilder {
    featureEnabled: bool,
}

impl GetFeatureStateReturnsBuilder {
    pub fn build(self) -> GetFeatureStateReturns {
        GetFeatureStateReturns {
            featureEnabled: self.featureEnabled,
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
    pub fn builder(processInfo: Vec<ProcessInfo<'a>>) -> GetProcessInfoReturnsBuilder<'a> {
        GetProcessInfoReturnsBuilder {
            processInfo: processInfo,
        }
    }
    pub fn processInfo(&self) -> &[ProcessInfo<'a>] { &self.processInfo }
}


pub struct GetProcessInfoReturnsBuilder<'a> {
    processInfo: Vec<ProcessInfo<'a>>,
}

impl<'a> GetProcessInfoReturnsBuilder<'a> {
    pub fn build(self) -> GetProcessInfoReturns<'a> {
        GetProcessInfoReturns {
            processInfo: self.processInfo,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetProcessInfoParams {}

impl GetProcessInfoParams { pub const METHOD: &'static str = "SystemInfo.getProcessInfo"; }

impl<'a> crate::CdpCommand<'a> for GetProcessInfoParams {
    const METHOD: &'static str = "SystemInfo.getProcessInfo";
    type Response = GetProcessInfoReturns<'a>;
}
