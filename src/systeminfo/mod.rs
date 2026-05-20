//! The SystemInfo domain defines methods and events for querying low-level system information.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Describes a single graphics processor (GPU).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GPUDevice<'a> {
    /// PCI ID of the GPU vendor, if available; 0 otherwise.
    #[serde(rename = "vendorId")]
    vendor_id: f64,
    /// PCI ID of the GPU device, if available; 0 otherwise.
    #[serde(rename = "deviceId")]
    device_id: f64,
    /// Sub sys ID of the GPU, only available on Windows.
    #[serde(skip_serializing_if = "Option::is_none", rename = "subSysId")]
    sub_sys_id: Option<f64>,
    /// Revision of the GPU, only available on Windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<f64>,
    /// String description of the GPU vendor, if the PCI ID is not available.
    #[serde(rename = "vendorString")]
    vendor_string: Cow<'a, str>,
    /// String description of the GPU device, if the PCI ID is not available.
    #[serde(rename = "deviceString")]
    device_string: Cow<'a, str>,
    /// String description of the GPU driver vendor.
    #[serde(rename = "driverVendor")]
    driver_vendor: Cow<'a, str>,
    /// String description of the GPU driver version.
    #[serde(rename = "driverVersion")]
    driver_version: Cow<'a, str>,
}

impl<'a> GPUDevice<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `vendor_id`: PCI ID of the GPU vendor, if available; 0 otherwise.
    /// * `device_id`: PCI ID of the GPU device, if available; 0 otherwise.
    /// * `vendor_string`: String description of the GPU vendor, if the PCI ID is not available.
    /// * `device_string`: String description of the GPU device, if the PCI ID is not available.
    /// * `driver_vendor`: String description of the GPU driver vendor.
    /// * `driver_version`: String description of the GPU driver version.
    pub fn builder(vendor_id: f64, device_id: f64, vendor_string: impl Into<Cow<'a, str>>, device_string: impl Into<Cow<'a, str>>, driver_vendor: impl Into<Cow<'a, str>>, driver_version: impl Into<Cow<'a, str>>) -> GPUDeviceBuilder<'a> {
        GPUDeviceBuilder {
            vendor_id: vendor_id,
            device_id: device_id,
            sub_sys_id: None,
            revision: None,
            vendor_string: vendor_string.into(),
            device_string: device_string.into(),
            driver_vendor: driver_vendor.into(),
            driver_version: driver_version.into(),
        }
    }
    /// PCI ID of the GPU vendor, if available; 0 otherwise.
    pub fn vendor_id(&self) -> f64 { self.vendor_id }
    /// PCI ID of the GPU device, if available; 0 otherwise.
    pub fn device_id(&self) -> f64 { self.device_id }
    /// Sub sys ID of the GPU, only available on Windows.
    pub fn sub_sys_id(&self) -> Option<f64> { self.sub_sys_id }
    /// Revision of the GPU, only available on Windows.
    pub fn revision(&self) -> Option<f64> { self.revision }
    /// String description of the GPU vendor, if the PCI ID is not available.
    pub fn vendor_string(&self) -> &str { self.vendor_string.as_ref() }
    /// String description of the GPU device, if the PCI ID is not available.
    pub fn device_string(&self) -> &str { self.device_string.as_ref() }
    /// String description of the GPU driver vendor.
    pub fn driver_vendor(&self) -> &str { self.driver_vendor.as_ref() }
    /// String description of the GPU driver version.
    pub fn driver_version(&self) -> &str { self.driver_version.as_ref() }
}


pub struct GPUDeviceBuilder<'a> {
    vendor_id: f64,
    device_id: f64,
    sub_sys_id: Option<f64>,
    revision: Option<f64>,
    vendor_string: Cow<'a, str>,
    device_string: Cow<'a, str>,
    driver_vendor: Cow<'a, str>,
    driver_version: Cow<'a, str>,
}

impl<'a> GPUDeviceBuilder<'a> {
    /// Sub sys ID of the GPU, only available on Windows.
    pub fn sub_sys_id(mut self, sub_sys_id: f64) -> Self { self.sub_sys_id = Some(sub_sys_id); self }
    /// Revision of the GPU, only available on Windows.
    pub fn revision(mut self, revision: f64) -> Self { self.revision = Some(revision); self }
    pub fn build(self) -> GPUDevice<'a> {
        GPUDevice {
            vendor_id: self.vendor_id,
            device_id: self.device_id,
            sub_sys_id: self.sub_sys_id,
            revision: self.revision,
            vendor_string: self.vendor_string,
            device_string: self.device_string,
            driver_vendor: self.driver_vendor,
            driver_version: self.driver_version,
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
    /// Creates a builder for this type with the required parameters:
    /// * `width`: Width in pixels.
    /// * `height`: Height in pixels.
    pub fn builder(width: u64, height: i64) -> SizeBuilder {
        SizeBuilder {
            width: width,
            height: height,
        }
    }
    /// Width in pixels.
    pub fn width(&self) -> u64 { self.width }
    /// Height in pixels.
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
    #[serde(rename = "maxResolution")]
    max_resolution: Size,
    /// Minimum video dimensions in pixels supported for this |profile|.
    #[serde(rename = "minResolution")]
    min_resolution: Size,
}

impl<'a> VideoDecodeAcceleratorCapability<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `profile`: Video codec profile that is supported, e.g. VP9 Profile 2.
    /// * `max_resolution`: Maximum video dimensions in pixels supported for this |profile|.
    /// * `min_resolution`: Minimum video dimensions in pixels supported for this |profile|.
    pub fn builder(profile: impl Into<Cow<'a, str>>, max_resolution: Size, min_resolution: Size) -> VideoDecodeAcceleratorCapabilityBuilder<'a> {
        VideoDecodeAcceleratorCapabilityBuilder {
            profile: profile.into(),
            max_resolution: max_resolution,
            min_resolution: min_resolution,
        }
    }
    /// Video codec profile that is supported, e.g. VP9 Profile 2.
    pub fn profile(&self) -> &str { self.profile.as_ref() }
    /// Maximum video dimensions in pixels supported for this |profile|.
    pub fn max_resolution(&self) -> &Size { &self.max_resolution }
    /// Minimum video dimensions in pixels supported for this |profile|.
    pub fn min_resolution(&self) -> &Size { &self.min_resolution }
}


pub struct VideoDecodeAcceleratorCapabilityBuilder<'a> {
    profile: Cow<'a, str>,
    max_resolution: Size,
    min_resolution: Size,
}

impl<'a> VideoDecodeAcceleratorCapabilityBuilder<'a> {
    pub fn build(self) -> VideoDecodeAcceleratorCapability<'a> {
        VideoDecodeAcceleratorCapability {
            profile: self.profile,
            max_resolution: self.max_resolution,
            min_resolution: self.min_resolution,
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
    #[serde(rename = "maxResolution")]
    max_resolution: Size,
    /// Maximum encoding framerate in frames per second supported for this
    /// |profile|, as fraction's numerator and denominator, e.g. 24/1 fps,
    /// 24000/1001 fps, etc.
    #[serde(rename = "maxFramerateNumerator")]
    max_framerate_numerator: i64,
    #[serde(rename = "maxFramerateDenominator")]
    max_framerate_denominator: i64,
}

impl<'a> VideoEncodeAcceleratorCapability<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `profile`: Video codec profile that is supported, e.g H264 Main.
    /// * `max_resolution`: Maximum video dimensions in pixels supported for this |profile|.
    /// * `max_framerate_numerator`: Maximum encoding framerate in frames per second supported for this |profile|, as fraction's numerator and denominator, e.g. 24/1 fps, 24000/1001 fps, etc.
    /// * `max_framerate_denominator`: 
    pub fn builder(profile: impl Into<Cow<'a, str>>, max_resolution: Size, max_framerate_numerator: i64, max_framerate_denominator: i64) -> VideoEncodeAcceleratorCapabilityBuilder<'a> {
        VideoEncodeAcceleratorCapabilityBuilder {
            profile: profile.into(),
            max_resolution: max_resolution,
            max_framerate_numerator: max_framerate_numerator,
            max_framerate_denominator: max_framerate_denominator,
        }
    }
    /// Video codec profile that is supported, e.g H264 Main.
    pub fn profile(&self) -> &str { self.profile.as_ref() }
    /// Maximum video dimensions in pixels supported for this |profile|.
    pub fn max_resolution(&self) -> &Size { &self.max_resolution }
    /// Maximum encoding framerate in frames per second supported for this
    /// |profile|, as fraction's numerator and denominator, e.g. 24/1 fps,
    /// 24000/1001 fps, etc.
    pub fn max_framerate_numerator(&self) -> i64 { self.max_framerate_numerator }
    pub fn max_framerate_denominator(&self) -> i64 { self.max_framerate_denominator }
}


pub struct VideoEncodeAcceleratorCapabilityBuilder<'a> {
    profile: Cow<'a, str>,
    max_resolution: Size,
    max_framerate_numerator: i64,
    max_framerate_denominator: i64,
}

impl<'a> VideoEncodeAcceleratorCapabilityBuilder<'a> {
    pub fn build(self) -> VideoEncodeAcceleratorCapability<'a> {
        VideoEncodeAcceleratorCapability {
            profile: self.profile,
            max_resolution: self.max_resolution,
            max_framerate_numerator: self.max_framerate_numerator,
            max_framerate_denominator: self.max_framerate_denominator,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "auxAttributes")]
    aux_attributes: Option<serde_json::Map<String, JsonValue>>,
    /// An optional dictionary of graphics features and their status.
    #[serde(skip_serializing_if = "Option::is_none", rename = "featureStatus")]
    feature_status: Option<serde_json::Map<String, JsonValue>>,
    /// An optional array of GPU driver bug workarounds.
    #[serde(rename = "driverBugWorkarounds")]
    driver_bug_workarounds: Vec<Cow<'a, str>>,
    /// Supported accelerated video decoding capabilities.
    #[serde(rename = "videoDecoding")]
    video_decoding: Vec<VideoDecodeAcceleratorCapability<'a>>,
    /// Supported accelerated video encoding capabilities.
    #[serde(rename = "videoEncoding")]
    video_encoding: Vec<VideoEncodeAcceleratorCapability<'a>>,
}

impl<'a> GPUInfo<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `devices`: The graphics devices on the system. Element 0 is the primary GPU.
    /// * `driver_bug_workarounds`: An optional array of GPU driver bug workarounds.
    /// * `video_decoding`: Supported accelerated video decoding capabilities.
    /// * `video_encoding`: Supported accelerated video encoding capabilities.
    pub fn builder(devices: Vec<GPUDevice<'a>>, driver_bug_workarounds: Vec<Cow<'a, str>>, video_decoding: Vec<VideoDecodeAcceleratorCapability<'a>>, video_encoding: Vec<VideoEncodeAcceleratorCapability<'a>>) -> GPUInfoBuilder<'a> {
        GPUInfoBuilder {
            devices: devices,
            aux_attributes: None,
            feature_status: None,
            driver_bug_workarounds: driver_bug_workarounds,
            video_decoding: video_decoding,
            video_encoding: video_encoding,
        }
    }
    /// The graphics devices on the system. Element 0 is the primary GPU.
    pub fn devices(&self) -> &[GPUDevice<'a>] { &self.devices }
    /// An optional dictionary of additional GPU related attributes.
    pub fn aux_attributes(&self) -> Option<&serde_json::Map<String, JsonValue>> { self.aux_attributes.as_ref() }
    /// An optional dictionary of graphics features and their status.
    pub fn feature_status(&self) -> Option<&serde_json::Map<String, JsonValue>> { self.feature_status.as_ref() }
    /// An optional array of GPU driver bug workarounds.
    pub fn driver_bug_workarounds(&self) -> &[Cow<'a, str>] { &self.driver_bug_workarounds }
    /// Supported accelerated video decoding capabilities.
    pub fn video_decoding(&self) -> &[VideoDecodeAcceleratorCapability<'a>] { &self.video_decoding }
    /// Supported accelerated video encoding capabilities.
    pub fn video_encoding(&self) -> &[VideoEncodeAcceleratorCapability<'a>] { &self.video_encoding }
}


pub struct GPUInfoBuilder<'a> {
    devices: Vec<GPUDevice<'a>>,
    aux_attributes: Option<serde_json::Map<String, JsonValue>>,
    feature_status: Option<serde_json::Map<String, JsonValue>>,
    driver_bug_workarounds: Vec<Cow<'a, str>>,
    video_decoding: Vec<VideoDecodeAcceleratorCapability<'a>>,
    video_encoding: Vec<VideoEncodeAcceleratorCapability<'a>>,
}

impl<'a> GPUInfoBuilder<'a> {
    /// An optional dictionary of additional GPU related attributes.
    pub fn aux_attributes(mut self, aux_attributes: serde_json::Map<String, JsonValue>) -> Self { self.aux_attributes = Some(aux_attributes); self }
    /// An optional dictionary of graphics features and their status.
    pub fn feature_status(mut self, feature_status: serde_json::Map<String, JsonValue>) -> Self { self.feature_status = Some(feature_status); self }
    pub fn build(self) -> GPUInfo<'a> {
        GPUInfo {
            devices: self.devices,
            aux_attributes: self.aux_attributes,
            feature_status: self.feature_status,
            driver_bug_workarounds: self.driver_bug_workarounds,
            video_decoding: self.video_decoding,
            video_encoding: self.video_encoding,
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
    #[serde(rename = "cpuTime")]
    cpu_time: f64,
}

impl<'a> ProcessInfo<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Specifies process type.
    /// * `id`: Specifies process id.
    /// * `cpu_time`: Specifies cumulative CPU usage in seconds across all threads of the process since the process start.
    pub fn builder(type_: impl Into<Cow<'a, str>>, id: u64, cpu_time: f64) -> ProcessInfoBuilder<'a> {
        ProcessInfoBuilder {
            type_: type_.into(),
            id: id,
            cpu_time: cpu_time,
        }
    }
    /// Specifies process type.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Specifies process id.
    pub fn id(&self) -> u64 { self.id }
    /// Specifies cumulative CPU usage in seconds across all threads of the
    /// process since the process start.
    pub fn cpu_time(&self) -> f64 { self.cpu_time }
}


pub struct ProcessInfoBuilder<'a> {
    type_: Cow<'a, str>,
    id: u64,
    cpu_time: f64,
}

impl<'a> ProcessInfoBuilder<'a> {
    pub fn build(self) -> ProcessInfo<'a> {
        ProcessInfo {
            type_: self.type_,
            id: self.id,
            cpu_time: self.cpu_time,
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
    #[serde(rename = "modelName")]
    model_name: Cow<'a, str>,
    /// A platform-dependent description of the version of the machine. On Mac OS, this is, for
    /// example, '10.1'. Will be the empty string if not supported.
    #[serde(rename = "modelVersion")]
    model_version: Cow<'a, str>,
    /// The command line string used to launch the browser. Will be the empty string if not
    /// supported.
    #[serde(rename = "commandLine")]
    command_line: Cow<'a, str>,
}

impl<'a> GetInfoReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `gpu`: Information about the GPUs on the system.
    /// * `model_name`: A platform-dependent description of the model of the machine. On Mac OS, this is, for example, 'MacBookPro'. Will be the empty string if not supported.
    /// * `model_version`: A platform-dependent description of the version of the machine. On Mac OS, this is, for example, '10.1'. Will be the empty string if not supported.
    /// * `command_line`: The command line string used to launch the browser. Will be the empty string if not supported.
    pub fn builder(gpu: GPUInfo<'a>, model_name: impl Into<Cow<'a, str>>, model_version: impl Into<Cow<'a, str>>, command_line: impl Into<Cow<'a, str>>) -> GetInfoReturnsBuilder<'a> {
        GetInfoReturnsBuilder {
            gpu: gpu,
            model_name: model_name.into(),
            model_version: model_version.into(),
            command_line: command_line.into(),
        }
    }
    /// Information about the GPUs on the system.
    pub fn gpu(&self) -> &GPUInfo<'a> { &self.gpu }
    /// A platform-dependent description of the model of the machine. On Mac OS, this is, for
    /// example, 'MacBookPro'. Will be the empty string if not supported.
    pub fn model_name(&self) -> &str { self.model_name.as_ref() }
    /// A platform-dependent description of the version of the machine. On Mac OS, this is, for
    /// example, '10.1'. Will be the empty string if not supported.
    pub fn model_version(&self) -> &str { self.model_version.as_ref() }
    /// The command line string used to launch the browser. Will be the empty string if not
    /// supported.
    pub fn command_line(&self) -> &str { self.command_line.as_ref() }
}


pub struct GetInfoReturnsBuilder<'a> {
    gpu: GPUInfo<'a>,
    model_name: Cow<'a, str>,
    model_version: Cow<'a, str>,
    command_line: Cow<'a, str>,
}

impl<'a> GetInfoReturnsBuilder<'a> {
    pub fn build(self) -> GetInfoReturns<'a> {
        GetInfoReturns {
            gpu: self.gpu,
            model_name: self.model_name,
            model_version: self.model_version,
            command_line: self.command_line,
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
    #[serde(rename = "featureState")]
    feature_state: Cow<'a, str>,
}

impl<'a> GetFeatureStateParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `feature_state`: 
    pub fn builder(feature_state: impl Into<Cow<'a, str>>) -> GetFeatureStateParamsBuilder<'a> {
        GetFeatureStateParamsBuilder {
            feature_state: feature_state.into(),
        }
    }
    pub fn feature_state(&self) -> &str { self.feature_state.as_ref() }
}


pub struct GetFeatureStateParamsBuilder<'a> {
    feature_state: Cow<'a, str>,
}

impl<'a> GetFeatureStateParamsBuilder<'a> {
    pub fn build(self) -> GetFeatureStateParams<'a> {
        GetFeatureStateParams {
            feature_state: self.feature_state,
        }
    }
}

/// Returns information about the feature state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeatureStateReturns {
    #[serde(rename = "featureEnabled")]
    feature_enabled: bool,
}

impl GetFeatureStateReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `feature_enabled`: 
    pub fn builder(feature_enabled: bool) -> GetFeatureStateReturnsBuilder {
        GetFeatureStateReturnsBuilder {
            feature_enabled: feature_enabled,
        }
    }
    pub fn feature_enabled(&self) -> bool { self.feature_enabled }
}


pub struct GetFeatureStateReturnsBuilder {
    feature_enabled: bool,
}

impl GetFeatureStateReturnsBuilder {
    pub fn build(self) -> GetFeatureStateReturns {
        GetFeatureStateReturns {
            feature_enabled: self.feature_enabled,
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
    #[serde(rename = "processInfo")]
    process_info: Vec<ProcessInfo<'a>>,
}

impl<'a> GetProcessInfoReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `process_info`: An array of process info blocks.
    pub fn builder(process_info: Vec<ProcessInfo<'a>>) -> GetProcessInfoReturnsBuilder<'a> {
        GetProcessInfoReturnsBuilder {
            process_info: process_info,
        }
    }
    /// An array of process info blocks.
    pub fn process_info(&self) -> &[ProcessInfo<'a>] { &self.process_info }
}


pub struct GetProcessInfoReturnsBuilder<'a> {
    process_info: Vec<ProcessInfo<'a>>,
}

impl<'a> GetProcessInfoReturnsBuilder<'a> {
    pub fn build(self) -> GetProcessInfoReturns<'a> {
        GetProcessInfoReturns {
            process_info: self.process_info,
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
