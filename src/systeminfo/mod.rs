//! The SystemInfo domain defines methods and events for querying low-level system information.

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Describes a single graphics processor (GPU).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GPUDevice {
    /// PCI ID of the GPU vendor, if available; 0 otherwise.

    pub vendorId: f64,
    /// PCI ID of the GPU device, if available; 0 otherwise.

    pub deviceId: f64,
    /// Sub sys ID of the GPU, only available on Windows.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subSysId: Option<f64>,
    /// Revision of the GPU, only available on Windows.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<f64>,
    /// String description of the GPU vendor, if the PCI ID is not available.

    pub vendorString: String,
    /// String description of the GPU device, if the PCI ID is not available.

    pub deviceString: String,
    /// String description of the GPU driver vendor.

    pub driverVendor: String,
    /// String description of the GPU driver version.

    pub driverVersion: String,
}

/// Describes the width and height dimensions of an entity.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    /// Width in pixels.

    pub width: u64,
    /// Height in pixels.

    pub height: i64,
}

/// Describes a supported video decoding profile with its associated minimum and
/// maximum resolutions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoDecodeAcceleratorCapability {
    /// Video codec profile that is supported, e.g. VP9 Profile 2.

    pub profile: String,
    /// Maximum video dimensions in pixels supported for this |profile|.

    pub maxResolution: Size,
    /// Minimum video dimensions in pixels supported for this |profile|.

    pub minResolution: Size,
}

/// Describes a supported video encoding profile with its associated maximum
/// resolution and maximum framerate.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoEncodeAcceleratorCapability {
    /// Video codec profile that is supported, e.g H264 Main.

    pub profile: String,
    /// Maximum video dimensions in pixels supported for this |profile|.

    pub maxResolution: Size,
    /// Maximum encoding framerate in frames per second supported for this
    /// |profile|, as fraction's numerator and denominator, e.g. 24/1 fps,
    /// 24000/1001 fps, etc.

    pub maxFramerateNumerator: i64,

    pub maxFramerateDenominator: i64,
}

/// YUV subsampling type of the pixels of a given image.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SubsamplingFormat {
    #[default]
    Yuv420,
    Yuv422,
    Yuv444,
}

/// Image format of a given image.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ImageType {
    #[default]
    Jpeg,
    Webp,
    Unknown,
}

/// Provides information about the GPU(s) on the system.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GPUInfo {
    /// The graphics devices on the system. Element 0 is the primary GPU.

    pub devices: Vec<GPUDevice>,
    /// An optional dictionary of additional GPU related attributes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxAttributes: Option<serde_json::Map<String, JsonValue>>,
    /// An optional dictionary of graphics features and their status.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub featureStatus: Option<serde_json::Map<String, JsonValue>>,
    /// An optional array of GPU driver bug workarounds.

    pub driverBugWorkarounds: Vec<String>,
    /// Supported accelerated video decoding capabilities.

    pub videoDecoding: Vec<VideoDecodeAcceleratorCapability>,
    /// Supported accelerated video encoding capabilities.

    pub videoEncoding: Vec<VideoEncodeAcceleratorCapability>,
}

/// Represents process info.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    /// Specifies process type.

    #[serde(rename = "type")]
    pub type_: String,
    /// Specifies process id.

    pub id: u64,
    /// Specifies cumulative CPU usage in seconds across all threads of the
    /// process since the process start.

    pub cpuTime: f64,
}

/// Returns information about the system.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoReturns {
    /// Information about the GPUs on the system.

    pub gpu: GPUInfo,
    /// A platform-dependent description of the model of the machine. On Mac OS, this is, for
    /// example, 'MacBookPro'. Will be the empty string if not supported.

    pub modelName: String,
    /// A platform-dependent description of the version of the machine. On Mac OS, this is, for
    /// example, '10.1'. Will be the empty string if not supported.

    pub modelVersion: String,
    /// The command line string used to launch the browser. Will be the empty string if not
    /// supported.

    pub commandLine: String,
}

/// Returns information about the feature state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeatureStateParams {

    pub featureState: String,
}

/// Returns information about the feature state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeatureStateReturns {

    pub featureEnabled: bool,
}

/// Returns information about all running processes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessInfoReturns {
    /// An array of process info blocks.

    pub processInfo: Vec<ProcessInfo>,
}
