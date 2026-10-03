//! The SystemInfo domain defines methods and events for querying low-level system information.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Describes a single graphics processor (GPU).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GPUDevice<'a> {
    /// PCI ID of the GPU vendor, if available; 0 otherwise.
    #[serde(rename = "vendorId")]
    pub vendor_id: f64,
    /// PCI ID of the GPU device, if available; 0 otherwise.
    #[serde(rename = "deviceId")]
    pub device_id: f64,
    /// Sub sys ID of the GPU, only available on Windows.
    #[serde(skip_serializing_if = "Option::is_none", rename = "subSysId")]
    pub sub_sys_id: Option<f64>,
    /// Revision of the GPU, only available on Windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<f64>,
    /// String description of the GPU vendor, if the PCI ID is not available.
    #[serde(rename = "vendorString")]
    pub vendor_string: Cow<'a, str>,
    /// String description of the GPU device, if the PCI ID is not available.
    #[serde(rename = "deviceString")]
    pub device_string: Cow<'a, str>,
    /// String description of the GPU driver vendor.
    #[serde(rename = "driverVendor")]
    pub driver_vendor: Cow<'a, str>,
    /// String description of the GPU driver version.
    #[serde(rename = "driverVersion")]
    pub driver_version: Cow<'a, str>,
}
/// Describes the width and height dimensions of an entity.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    /// Width in pixels.
    pub width: u64,
    /// Height in pixels.
    pub height: i64,
}
/// Describes a supported video decoding profile with its associated minimum and
/// maximum resolutions.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct VideoDecodeAcceleratorCapability<'a> {
    /// Video codec profile that is supported, e.g. VP9 Profile 2.
    pub profile: Cow<'a, str>,
    /// Maximum video dimensions in pixels supported for this |profile|.
    #[serde(rename = "maxResolution")]
    pub max_resolution: Size,
    /// Minimum video dimensions in pixels supported for this |profile|.
    #[serde(rename = "minResolution")]
    pub min_resolution: Size,
}
/// Describes a supported video encoding profile with its associated maximum
/// resolution and maximum framerate.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct VideoEncodeAcceleratorCapability<'a> {
    /// Video codec profile that is supported, e.g H264 Main.
    pub profile: Cow<'a, str>,
    /// Maximum video dimensions in pixels supported for this |profile|.
    #[serde(rename = "maxResolution")]
    pub max_resolution: Size,
    /// Maximum encoding framerate in frames per second supported for this
    /// |profile|, as fraction's numerator and denominator, e.g. 24/1 fps,
    /// 24000/1001 fps, etc.
    #[serde(rename = "maxFramerateNumerator")]
    pub max_framerate_numerator: i64,
    #[serde(rename = "maxFramerateDenominator")]
    pub max_framerate_denominator: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GPUInfo<'a> {
    /// The graphics devices on the system. Element 0 is the primary GPU.
    pub devices: Vec<GPUDevice<'a>>,
    /// An optional dictionary of additional GPU related attributes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "auxAttributes")]
    pub aux_attributes: Option<serde_json::Map<String, JsonValue>>,
    /// An optional dictionary of graphics features and their status.
    #[serde(skip_serializing_if = "Option::is_none", rename = "featureStatus")]
    pub feature_status: Option<serde_json::Map<String, JsonValue>>,
    /// An optional array of GPU driver bug workarounds.
    #[serde(rename = "driverBugWorkarounds")]
    pub driver_bug_workarounds: Vec<Cow<'a, str>>,
    /// Supported accelerated video decoding capabilities.
    #[serde(rename = "videoDecoding")]
    pub video_decoding: Vec<VideoDecodeAcceleratorCapability<'a>>,
    /// Supported accelerated video encoding capabilities.
    #[serde(rename = "videoEncoding")]
    pub video_encoding: Vec<VideoEncodeAcceleratorCapability<'a>>,
}
/// Represents process info.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo<'a> {
    /// Specifies process type.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Specifies process id.
    pub id: u64,
    /// Specifies cumulative CPU usage in seconds across all threads of the
    /// process since the process start.
    #[serde(rename = "cpuTime")]
    pub cpu_time: f64,
}
/// Returns information about the system.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SystemInfo.getInfo", response = "GetInfoReturns<'a>")]
pub struct GetInfoParams {

}
/// Returns information about the system.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoReturns<'a> {
    /// Information about the GPUs on the system.
    pub gpu: GPUInfo<'a>,
    /// A platform-dependent description of the model of the machine. On Mac OS, this is, for
    /// example, 'MacBookPro'. Will be the empty string if not supported.
    #[serde(rename = "modelName")]
    pub model_name: Cow<'a, str>,
    /// A platform-dependent description of the version of the machine. On Mac OS, this is, for
    /// example, '10.1'. Will be the empty string if not supported.
    #[serde(rename = "modelVersion")]
    pub model_version: Cow<'a, str>,
    /// The command line string used to launch the browser. Will be the empty string if not
    /// supported.
    #[serde(rename = "commandLine")]
    pub command_line: Cow<'a, str>,
}
/// Returns information about the feature state.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SystemInfo.getFeatureState", response = "GetFeatureStateReturns")]
pub struct GetFeatureStateParams<'a> {
    #[serde(rename = "featureState")]
    pub feature_state: Cow<'a, str>,
}
/// Returns information about the feature state.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetFeatureStateReturns {
    #[serde(rename = "featureEnabled")]
    pub feature_enabled: bool,
}
/// Returns information about all running processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SystemInfo.getProcessInfo", response = "GetProcessInfoReturns<'a>")]
pub struct GetProcessInfoParams {

}
/// Returns information about all running processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessInfoReturns<'a> {
    /// An array of process info blocks.
    #[serde(rename = "processInfo")]
    pub process_info: Vec<ProcessInfo<'a>>,
}