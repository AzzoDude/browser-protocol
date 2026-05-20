//! This domain emulates different environments for the page.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SafeAreaInsets {
    /// Overrides safe-area-inset-top.
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<i64>,
    /// Overrides safe-area-max-inset-top.
    #[serde(skip_serializing_if = "Option::is_none")]
    topMax: Option<i64>,
    /// Overrides safe-area-inset-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<i64>,
    /// Overrides safe-area-max-inset-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    leftMax: Option<i64>,
    /// Overrides safe-area-inset-bottom.
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<i64>,
    /// Overrides safe-area-max-inset-bottom.
    #[serde(skip_serializing_if = "Option::is_none")]
    bottomMax: Option<i64>,
    /// Overrides safe-area-inset-right.
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<i64>,
    /// Overrides safe-area-max-inset-right.
    #[serde(skip_serializing_if = "Option::is_none")]
    rightMax: Option<i64>,
}

impl SafeAreaInsets {
    pub fn builder() -> SafeAreaInsetsBuilder {
        SafeAreaInsetsBuilder {
            top: None,
            topMax: None,
            left: None,
            leftMax: None,
            bottom: None,
            bottomMax: None,
            right: None,
            rightMax: None,
        }
    }
    pub fn top(&self) -> Option<i64> { self.top }
    pub fn topMax(&self) -> Option<i64> { self.topMax }
    pub fn left(&self) -> Option<i64> { self.left }
    pub fn leftMax(&self) -> Option<i64> { self.leftMax }
    pub fn bottom(&self) -> Option<i64> { self.bottom }
    pub fn bottomMax(&self) -> Option<i64> { self.bottomMax }
    pub fn right(&self) -> Option<i64> { self.right }
    pub fn rightMax(&self) -> Option<i64> { self.rightMax }
}

#[derive(Default)]
pub struct SafeAreaInsetsBuilder {
    top: Option<i64>,
    topMax: Option<i64>,
    left: Option<i64>,
    leftMax: Option<i64>,
    bottom: Option<i64>,
    bottomMax: Option<i64>,
    right: Option<i64>,
    rightMax: Option<i64>,
}

impl SafeAreaInsetsBuilder {
    /// Overrides safe-area-inset-top.
    pub fn top(mut self, top: i64) -> Self { self.top = Some(top); self }
    /// Overrides safe-area-max-inset-top.
    pub fn topMax(mut self, topMax: i64) -> Self { self.topMax = Some(topMax); self }
    /// Overrides safe-area-inset-left.
    pub fn left(mut self, left: i64) -> Self { self.left = Some(left); self }
    /// Overrides safe-area-max-inset-left.
    pub fn leftMax(mut self, leftMax: i64) -> Self { self.leftMax = Some(leftMax); self }
    /// Overrides safe-area-inset-bottom.
    pub fn bottom(mut self, bottom: i64) -> Self { self.bottom = Some(bottom); self }
    /// Overrides safe-area-max-inset-bottom.
    pub fn bottomMax(mut self, bottomMax: i64) -> Self { self.bottomMax = Some(bottomMax); self }
    /// Overrides safe-area-inset-right.
    pub fn right(mut self, right: i64) -> Self { self.right = Some(right); self }
    /// Overrides safe-area-max-inset-right.
    pub fn rightMax(mut self, rightMax: i64) -> Self { self.rightMax = Some(rightMax); self }
    pub fn build(self) -> SafeAreaInsets {
        SafeAreaInsets {
            top: self.top,
            topMax: self.topMax,
            left: self.left,
            leftMax: self.leftMax,
            bottom: self.bottom,
            bottomMax: self.bottomMax,
            right: self.right,
            rightMax: self.rightMax,
        }
    }
}

/// Screen orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreenOrientation<'a> {
    /// Orientation type.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Orientation angle.
    angle: i64,
}

impl<'a> ScreenOrientation<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>, angle: i64) -> ScreenOrientationBuilder<'a> {
        ScreenOrientationBuilder {
            type_: type_.into(),
            angle: angle,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn angle(&self) -> i64 { self.angle }
}


pub struct ScreenOrientationBuilder<'a> {
    type_: Cow<'a, str>,
    angle: i64,
}

impl<'a> ScreenOrientationBuilder<'a> {
    pub fn build(self) -> ScreenOrientation<'a> {
        ScreenOrientation {
            type_: self.type_,
            angle: self.angle,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisplayFeature<'a> {
    /// Orientation of a display feature in relation to screen
    orientation: Cow<'a, str>,
    /// The offset from the screen origin in either the x (for vertical
    /// orientation) or y (for horizontal orientation) direction.
    offset: i32,
    /// A display feature may mask content such that it is not physically
    /// displayed - this length along with the offset describes this area.
    /// A display feature that only splits content will have a 0 mask_length.
    maskLength: u64,
}

impl<'a> DisplayFeature<'a> {
    pub fn builder(orientation: impl Into<Cow<'a, str>>, offset: i32, maskLength: u64) -> DisplayFeatureBuilder<'a> {
        DisplayFeatureBuilder {
            orientation: orientation.into(),
            offset: offset,
            maskLength: maskLength,
        }
    }
    pub fn orientation(&self) -> &str { self.orientation.as_ref() }
    pub fn offset(&self) -> i32 { self.offset }
    pub fn maskLength(&self) -> u64 { self.maskLength }
}


pub struct DisplayFeatureBuilder<'a> {
    orientation: Cow<'a, str>,
    offset: i32,
    maskLength: u64,
}

impl<'a> DisplayFeatureBuilder<'a> {
    pub fn build(self) -> DisplayFeature<'a> {
        DisplayFeature {
            orientation: self.orientation,
            offset: self.offset,
            maskLength: self.maskLength,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DevicePosture<'a> {
    /// Current posture of the device
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
}

impl<'a> DevicePosture<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> DevicePostureBuilder<'a> {
        DevicePostureBuilder {
            type_: type_.into(),
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}


pub struct DevicePostureBuilder<'a> {
    type_: Cow<'a, str>,
}

impl<'a> DevicePostureBuilder<'a> {
    pub fn build(self) -> DevicePosture<'a> {
        DevicePosture {
            type_: self.type_,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaFeature<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> MediaFeature<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> MediaFeatureBuilder<'a> {
        MediaFeatureBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct MediaFeatureBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> MediaFeatureBuilder<'a> {
    pub fn build(self) -> MediaFeature<'a> {
        MediaFeature {
            name: self.name,
            value: self.value,
        }
    }
}

/// advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to
/// allow the next delayed task (if any) to run; pause: The virtual time base may not advance;
/// pauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending
/// resource fetches.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum VirtualTimePolicy {
    #[default]
    #[serde(rename = "advance")]
    Advance,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "pauseIfNetworkFetchesPending")]
    PauseIfNetworkFetchesPending,
}

/// Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentBrandVersion<'a> {
    brand: Cow<'a, str>,
    version: Cow<'a, str>,
}

impl<'a> UserAgentBrandVersion<'a> {
    pub fn builder(brand: impl Into<Cow<'a, str>>, version: impl Into<Cow<'a, str>>) -> UserAgentBrandVersionBuilder<'a> {
        UserAgentBrandVersionBuilder {
            brand: brand.into(),
            version: version.into(),
        }
    }
    pub fn brand(&self) -> &str { self.brand.as_ref() }
    pub fn version(&self) -> &str { self.version.as_ref() }
}


pub struct UserAgentBrandVersionBuilder<'a> {
    brand: Cow<'a, str>,
    version: Cow<'a, str>,
}

impl<'a> UserAgentBrandVersionBuilder<'a> {
    pub fn build(self) -> UserAgentBrandVersion<'a> {
        UserAgentBrandVersion {
            brand: self.brand,
            version: self.version,
        }
    }
}

/// Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints
/// Missing optional values will be filled in by the target with what it would normally use.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentMetadata<'a> {
    /// Brands appearing in Sec-CH-UA.
    #[serde(skip_serializing_if = "Option::is_none")]
    brands: Option<Vec<UserAgentBrandVersion<'a>>>,
    /// Brands appearing in Sec-CH-UA-Full-Version-List.
    #[serde(skip_serializing_if = "Option::is_none")]
    fullVersionList: Option<Vec<UserAgentBrandVersion<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fullVersion: Option<Cow<'a, str>>,
    platform: Cow<'a, str>,
    platformVersion: Cow<'a, str>,
    architecture: Cow<'a, str>,
    model: Cow<'a, str>,
    mobile: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitness: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wow64: Option<bool>,
    /// Used to specify User Agent form-factor values.
    /// See https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors
    #[serde(skip_serializing_if = "Option::is_none")]
    formFactors: Option<Vec<Cow<'a, str>>>,
}

impl<'a> UserAgentMetadata<'a> {
    pub fn builder(platform: impl Into<Cow<'a, str>>, platformVersion: impl Into<Cow<'a, str>>, architecture: impl Into<Cow<'a, str>>, model: impl Into<Cow<'a, str>>, mobile: bool) -> UserAgentMetadataBuilder<'a> {
        UserAgentMetadataBuilder {
            brands: None,
            fullVersionList: None,
            fullVersion: None,
            platform: platform.into(),
            platformVersion: platformVersion.into(),
            architecture: architecture.into(),
            model: model.into(),
            mobile: mobile,
            bitness: None,
            wow64: None,
            formFactors: None,
        }
    }
    pub fn brands(&self) -> Option<&[UserAgentBrandVersion<'a>]> { self.brands.as_deref() }
    pub fn fullVersionList(&self) -> Option<&[UserAgentBrandVersion<'a>]> { self.fullVersionList.as_deref() }
    pub fn fullVersion(&self) -> Option<&str> { self.fullVersion.as_deref() }
    pub fn platform(&self) -> &str { self.platform.as_ref() }
    pub fn platformVersion(&self) -> &str { self.platformVersion.as_ref() }
    pub fn architecture(&self) -> &str { self.architecture.as_ref() }
    pub fn model(&self) -> &str { self.model.as_ref() }
    pub fn mobile(&self) -> bool { self.mobile }
    pub fn bitness(&self) -> Option<&str> { self.bitness.as_deref() }
    pub fn wow64(&self) -> Option<bool> { self.wow64 }
    pub fn formFactors(&self) -> Option<&[Cow<'a, str>]> { self.formFactors.as_deref() }
}


pub struct UserAgentMetadataBuilder<'a> {
    brands: Option<Vec<UserAgentBrandVersion<'a>>>,
    fullVersionList: Option<Vec<UserAgentBrandVersion<'a>>>,
    fullVersion: Option<Cow<'a, str>>,
    platform: Cow<'a, str>,
    platformVersion: Cow<'a, str>,
    architecture: Cow<'a, str>,
    model: Cow<'a, str>,
    mobile: bool,
    bitness: Option<Cow<'a, str>>,
    wow64: Option<bool>,
    formFactors: Option<Vec<Cow<'a, str>>>,
}

impl<'a> UserAgentMetadataBuilder<'a> {
    /// Brands appearing in Sec-CH-UA.
    pub fn brands(mut self, brands: Vec<UserAgentBrandVersion<'a>>) -> Self { self.brands = Some(brands); self }
    /// Brands appearing in Sec-CH-UA-Full-Version-List.
    pub fn fullVersionList(mut self, fullVersionList: Vec<UserAgentBrandVersion<'a>>) -> Self { self.fullVersionList = Some(fullVersionList); self }
    pub fn fullVersion(mut self, fullVersion: impl Into<Cow<'a, str>>) -> Self { self.fullVersion = Some(fullVersion.into()); self }
    pub fn bitness(mut self, bitness: impl Into<Cow<'a, str>>) -> Self { self.bitness = Some(bitness.into()); self }
    pub fn wow64(mut self, wow64: bool) -> Self { self.wow64 = Some(wow64); self }
    /// Used to specify User Agent form-factor values.
    /// See https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors
    pub fn formFactors(mut self, formFactors: Vec<Cow<'a, str>>) -> Self { self.formFactors = Some(formFactors); self }
    pub fn build(self) -> UserAgentMetadata<'a> {
        UserAgentMetadata {
            brands: self.brands,
            fullVersionList: self.fullVersionList,
            fullVersion: self.fullVersion,
            platform: self.platform,
            platformVersion: self.platformVersion,
            architecture: self.architecture,
            model: self.model,
            mobile: self.mobile,
            bitness: self.bitness,
            wow64: self.wow64,
            formFactors: self.formFactors,
        }
    }
}

/// Used to specify sensor types to emulate.
/// See https://w3c.github.io/sensors/#automation for more information.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SensorType {
    #[default]
    #[serde(rename = "absolute-orientation")]
    AbsoluteOrientation,
    #[serde(rename = "accelerometer")]
    Accelerometer,
    #[serde(rename = "ambient-light")]
    AmbientLight,
    #[serde(rename = "gravity")]
    Gravity,
    #[serde(rename = "gyroscope")]
    Gyroscope,
    #[serde(rename = "linear-acceleration")]
    LinearAcceleration,
    #[serde(rename = "magnetometer")]
    Magnetometer,
    #[serde(rename = "relative-orientation")]
    RelativeOrientation,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimumFrequency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximumFrequency: Option<f64>,
}

impl SensorMetadata {
    pub fn builder() -> SensorMetadataBuilder {
        SensorMetadataBuilder {
            available: None,
            minimumFrequency: None,
            maximumFrequency: None,
        }
    }
    pub fn available(&self) -> Option<bool> { self.available }
    pub fn minimumFrequency(&self) -> Option<f64> { self.minimumFrequency }
    pub fn maximumFrequency(&self) -> Option<f64> { self.maximumFrequency }
}

#[derive(Default)]
pub struct SensorMetadataBuilder {
    available: Option<bool>,
    minimumFrequency: Option<f64>,
    maximumFrequency: Option<f64>,
}

impl SensorMetadataBuilder {
    pub fn available(mut self, available: bool) -> Self { self.available = Some(available); self }
    pub fn minimumFrequency(mut self, minimumFrequency: f64) -> Self { self.minimumFrequency = Some(minimumFrequency); self }
    pub fn maximumFrequency(mut self, maximumFrequency: f64) -> Self { self.maximumFrequency = Some(maximumFrequency); self }
    pub fn build(self) -> SensorMetadata {
        SensorMetadata {
            available: self.available,
            minimumFrequency: self.minimumFrequency,
            maximumFrequency: self.maximumFrequency,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingSingle {
    value: f64,
}

impl SensorReadingSingle {
    pub fn builder(value: f64) -> SensorReadingSingleBuilder {
        SensorReadingSingleBuilder {
            value: value,
        }
    }
    pub fn value(&self) -> f64 { self.value }
}


pub struct SensorReadingSingleBuilder {
    value: f64,
}

impl SensorReadingSingleBuilder {
    pub fn build(self) -> SensorReadingSingle {
        SensorReadingSingle {
            value: self.value,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingXYZ {
    x: f64,
    y: f64,
    z: f64,
}

impl SensorReadingXYZ {
    pub fn builder(x: f64, y: f64, z: f64) -> SensorReadingXYZBuilder {
        SensorReadingXYZBuilder {
            x: x,
            y: y,
            z: z,
        }
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }
}


pub struct SensorReadingXYZBuilder {
    x: f64,
    y: f64,
    z: f64,
}

impl SensorReadingXYZBuilder {
    pub fn build(self) -> SensorReadingXYZ {
        SensorReadingXYZ {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingQuaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl SensorReadingQuaternion {
    pub fn builder(x: f64, y: f64, z: f64, w: f64) -> SensorReadingQuaternionBuilder {
        SensorReadingQuaternionBuilder {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }
    pub fn w(&self) -> f64 { self.w }
}


pub struct SensorReadingQuaternionBuilder {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl SensorReadingQuaternionBuilder {
    pub fn build(self) -> SensorReadingQuaternion {
        SensorReadingQuaternion {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorReading {
    #[serde(skip_serializing_if = "Option::is_none")]
    single: Option<SensorReadingSingle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xyz: Option<SensorReadingXYZ>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quaternion: Option<SensorReadingQuaternion>,
}

impl SensorReading {
    pub fn builder() -> SensorReadingBuilder {
        SensorReadingBuilder {
            single: None,
            xyz: None,
            quaternion: None,
        }
    }
    pub fn single(&self) -> Option<&SensorReadingSingle> { self.single.as_ref() }
    pub fn xyz(&self) -> Option<&SensorReadingXYZ> { self.xyz.as_ref() }
    pub fn quaternion(&self) -> Option<&SensorReadingQuaternion> { self.quaternion.as_ref() }
}

#[derive(Default)]
pub struct SensorReadingBuilder {
    single: Option<SensorReadingSingle>,
    xyz: Option<SensorReadingXYZ>,
    quaternion: Option<SensorReadingQuaternion>,
}

impl SensorReadingBuilder {
    pub fn single(mut self, single: SensorReadingSingle) -> Self { self.single = Some(single); self }
    pub fn xyz(mut self, xyz: SensorReadingXYZ) -> Self { self.xyz = Some(xyz); self }
    pub fn quaternion(mut self, quaternion: SensorReadingQuaternion) -> Self { self.quaternion = Some(quaternion); self }
    pub fn build(self) -> SensorReading {
        SensorReading {
            single: self.single,
            xyz: self.xyz,
            quaternion: self.quaternion,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PressureSource {
    #[default]
    #[serde(rename = "cpu")]
    Cpu,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PressureState {
    #[default]
    #[serde(rename = "nominal")]
    Nominal,
    #[serde(rename = "fair")]
    Fair,
    #[serde(rename = "serious")]
    Serious,
    #[serde(rename = "critical")]
    Critical,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PressureMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    available: Option<bool>,
}

impl PressureMetadata {
    pub fn builder() -> PressureMetadataBuilder {
        PressureMetadataBuilder {
            available: None,
        }
    }
    pub fn available(&self) -> Option<bool> { self.available }
}

#[derive(Default)]
pub struct PressureMetadataBuilder {
    available: Option<bool>,
}

impl PressureMetadataBuilder {
    pub fn available(mut self, available: bool) -> Self { self.available = Some(available); self }
    pub fn build(self) -> PressureMetadata {
        PressureMetadata {
            available: self.available,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorkAreaInsets {
    /// Work area top inset in pixels. Default is 0;
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<i64>,
    /// Work area left inset in pixels. Default is 0;
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<i64>,
    /// Work area bottom inset in pixels. Default is 0;
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<i64>,
    /// Work area right inset in pixels. Default is 0;
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<i64>,
}

impl WorkAreaInsets {
    pub fn builder() -> WorkAreaInsetsBuilder {
        WorkAreaInsetsBuilder {
            top: None,
            left: None,
            bottom: None,
            right: None,
        }
    }
    pub fn top(&self) -> Option<i64> { self.top }
    pub fn left(&self) -> Option<i64> { self.left }
    pub fn bottom(&self) -> Option<i64> { self.bottom }
    pub fn right(&self) -> Option<i64> { self.right }
}

#[derive(Default)]
pub struct WorkAreaInsetsBuilder {
    top: Option<i64>,
    left: Option<i64>,
    bottom: Option<i64>,
    right: Option<i64>,
}

impl WorkAreaInsetsBuilder {
    /// Work area top inset in pixels. Default is 0;
    pub fn top(mut self, top: i64) -> Self { self.top = Some(top); self }
    /// Work area left inset in pixels. Default is 0;
    pub fn left(mut self, left: i64) -> Self { self.left = Some(left); self }
    /// Work area bottom inset in pixels. Default is 0;
    pub fn bottom(mut self, bottom: i64) -> Self { self.bottom = Some(bottom); self }
    /// Work area right inset in pixels. Default is 0;
    pub fn right(mut self, right: i64) -> Self { self.right = Some(right); self }
    pub fn build(self) -> WorkAreaInsets {
        WorkAreaInsets {
            top: self.top,
            left: self.left,
            bottom: self.bottom,
            right: self.right,
        }
    }
}


pub type ScreenId<'a> = Cow<'a, str>;

/// Screen information similar to the one returned by window.getScreenDetails() method,
/// see https://w3c.github.io/window-management/#screendetailed.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreenInfo<'a> {
    /// Offset of the left edge of the screen.
    left: i64,
    /// Offset of the top edge of the screen.
    top: i64,
    /// Width of the screen.
    width: u64,
    /// Height of the screen.
    height: i64,
    /// Offset of the left edge of the available screen area.
    availLeft: i64,
    /// Offset of the top edge of the available screen area.
    availTop: i64,
    /// Width of the available screen area.
    availWidth: u64,
    /// Height of the available screen area.
    availHeight: i64,
    /// Specifies the screen's device pixel ratio.
    devicePixelRatio: f64,
    /// Specifies the screen's orientation.
    orientation: ScreenOrientation<'a>,
    /// Specifies the screen's color depth in bits.
    colorDepth: i64,
    /// Indicates whether the device has multiple screens.
    isExtended: bool,
    /// Indicates whether the screen is internal to the device or external, attached to the device.
    isInternal: bool,
    /// Indicates whether the screen is set as the the operating system primary screen.
    isPrimary: bool,
    /// Specifies the descriptive label for the screen.
    label: Cow<'a, str>,
    /// Specifies the unique identifier of the screen.
    id: ScreenId<'a>,
}

impl<'a> ScreenInfo<'a> {
    pub fn builder(left: i64, top: i64, width: u64, height: i64, availLeft: i64, availTop: i64, availWidth: u64, availHeight: i64, devicePixelRatio: f64, orientation: ScreenOrientation<'a>, colorDepth: i64, isExtended: bool, isInternal: bool, isPrimary: bool, label: impl Into<Cow<'a, str>>, id: ScreenId<'a>) -> ScreenInfoBuilder<'a> {
        ScreenInfoBuilder {
            left: left,
            top: top,
            width: width,
            height: height,
            availLeft: availLeft,
            availTop: availTop,
            availWidth: availWidth,
            availHeight: availHeight,
            devicePixelRatio: devicePixelRatio,
            orientation: orientation,
            colorDepth: colorDepth,
            isExtended: isExtended,
            isInternal: isInternal,
            isPrimary: isPrimary,
            label: label.into(),
            id: id,
        }
    }
    pub fn left(&self) -> i64 { self.left }
    pub fn top(&self) -> i64 { self.top }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> i64 { self.height }
    pub fn availLeft(&self) -> i64 { self.availLeft }
    pub fn availTop(&self) -> i64 { self.availTop }
    pub fn availWidth(&self) -> u64 { self.availWidth }
    pub fn availHeight(&self) -> i64 { self.availHeight }
    pub fn devicePixelRatio(&self) -> f64 { self.devicePixelRatio }
    pub fn orientation(&self) -> &ScreenOrientation<'a> { &self.orientation }
    pub fn colorDepth(&self) -> i64 { self.colorDepth }
    pub fn isExtended(&self) -> bool { self.isExtended }
    pub fn isInternal(&self) -> bool { self.isInternal }
    pub fn isPrimary(&self) -> bool { self.isPrimary }
    pub fn label(&self) -> &str { self.label.as_ref() }
    pub fn id(&self) -> &ScreenId<'a> { &self.id }
}


pub struct ScreenInfoBuilder<'a> {
    left: i64,
    top: i64,
    width: u64,
    height: i64,
    availLeft: i64,
    availTop: i64,
    availWidth: u64,
    availHeight: i64,
    devicePixelRatio: f64,
    orientation: ScreenOrientation<'a>,
    colorDepth: i64,
    isExtended: bool,
    isInternal: bool,
    isPrimary: bool,
    label: Cow<'a, str>,
    id: ScreenId<'a>,
}

impl<'a> ScreenInfoBuilder<'a> {
    pub fn build(self) -> ScreenInfo<'a> {
        ScreenInfo {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            availLeft: self.availLeft,
            availTop: self.availTop,
            availWidth: self.availWidth,
            availHeight: self.availHeight,
            devicePixelRatio: self.devicePixelRatio,
            orientation: self.orientation,
            colorDepth: self.colorDepth,
            isExtended: self.isExtended,
            isInternal: self.isInternal,
            isPrimary: self.isPrimary,
            label: self.label,
            id: self.id,
        }
    }
}

/// Enum of image types that can be disabled.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DisabledImageType {
    #[default]
    #[serde(rename = "avif")]
    Avif,
    #[serde(rename = "jxl")]
    Jxl,
    #[serde(rename = "webp")]
    Webp,
}

/// Tells whether emulation is supported.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulateReturns {
    /// True if emulation is supported.
    result: bool,
}

impl CanEmulateReturns {
    pub fn builder(result: bool) -> CanEmulateReturnsBuilder {
        CanEmulateReturnsBuilder {
            result: result,
        }
    }
    pub fn result(&self) -> bool { self.result }
}


pub struct CanEmulateReturnsBuilder {
    result: bool,
}

impl CanEmulateReturnsBuilder {
    pub fn build(self) -> CanEmulateReturns {
        CanEmulateReturns {
            result: self.result,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanEmulateParams {}

impl CanEmulateParams { pub const METHOD: &'static str = "Emulation.canEmulate"; }

impl<'a> crate::CdpCommand<'a> for CanEmulateParams {
    const METHOD: &'static str = "Emulation.canEmulate";
    type Response = CanEmulateReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDeviceMetricsOverrideParams {}

impl ClearDeviceMetricsOverrideParams { pub const METHOD: &'static str = "Emulation.clearDeviceMetricsOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearDeviceMetricsOverrideParams {
    const METHOD: &'static str = "Emulation.clearDeviceMetricsOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearGeolocationOverrideParams {}

impl ClearGeolocationOverrideParams { pub const METHOD: &'static str = "Emulation.clearGeolocationOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearGeolocationOverrideParams {
    const METHOD: &'static str = "Emulation.clearGeolocationOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResetPageScaleFactorParams {}

impl ResetPageScaleFactorParams { pub const METHOD: &'static str = "Emulation.resetPageScaleFactor"; }

impl<'a> crate::CdpCommand<'a> for ResetPageScaleFactorParams {
    const METHOD: &'static str = "Emulation.resetPageScaleFactor";
    type Response = crate::EmptyReturns;
}

/// Enables or disables simulating a focused and active page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFocusEmulationEnabledParams {
    /// Whether to enable to disable focus emulation.
    enabled: bool,
}

impl SetFocusEmulationEnabledParams {
    pub fn builder(enabled: bool) -> SetFocusEmulationEnabledParamsBuilder {
        SetFocusEmulationEnabledParamsBuilder {
            enabled: enabled,
        }
    }
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetFocusEmulationEnabledParamsBuilder {
    enabled: bool,
}

impl SetFocusEmulationEnabledParamsBuilder {
    pub fn build(self) -> SetFocusEmulationEnabledParams {
        SetFocusEmulationEnabledParams {
            enabled: self.enabled,
        }
    }
}

impl SetFocusEmulationEnabledParams { pub const METHOD: &'static str = "Emulation.setFocusEmulationEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetFocusEmulationEnabledParams {
    const METHOD: &'static str = "Emulation.setFocusEmulationEnabled";
    type Response = crate::EmptyReturns;
}

/// Automatically render all web contents using a dark theme.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoDarkModeOverrideParams {
    /// Whether to enable or disable automatic dark mode.
    /// If not specified, any existing override will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}

impl SetAutoDarkModeOverrideParams {
    pub fn builder() -> SetAutoDarkModeOverrideParamsBuilder {
        SetAutoDarkModeOverrideParamsBuilder {
            enabled: None,
        }
    }
    pub fn enabled(&self) -> Option<bool> { self.enabled }
}

#[derive(Default)]
pub struct SetAutoDarkModeOverrideParamsBuilder {
    enabled: Option<bool>,
}

impl SetAutoDarkModeOverrideParamsBuilder {
    /// Whether to enable or disable automatic dark mode.
    /// If not specified, any existing override will be cleared.
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    pub fn build(self) -> SetAutoDarkModeOverrideParams {
        SetAutoDarkModeOverrideParams {
            enabled: self.enabled,
        }
    }
}

impl SetAutoDarkModeOverrideParams { pub const METHOD: &'static str = "Emulation.setAutoDarkModeOverride"; }

impl<'a> crate::CdpCommand<'a> for SetAutoDarkModeOverrideParams {
    const METHOD: &'static str = "Emulation.setAutoDarkModeOverride";
    type Response = crate::EmptyReturns;
}

/// Enables CPU throttling to emulate slow CPUs.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCPUThrottlingRateParams {
    /// Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc).
    rate: f64,
}

impl SetCPUThrottlingRateParams {
    pub fn builder(rate: f64) -> SetCPUThrottlingRateParamsBuilder {
        SetCPUThrottlingRateParamsBuilder {
            rate: rate,
        }
    }
    pub fn rate(&self) -> f64 { self.rate }
}


pub struct SetCPUThrottlingRateParamsBuilder {
    rate: f64,
}

impl SetCPUThrottlingRateParamsBuilder {
    pub fn build(self) -> SetCPUThrottlingRateParams {
        SetCPUThrottlingRateParams {
            rate: self.rate,
        }
    }
}

impl SetCPUThrottlingRateParams { pub const METHOD: &'static str = "Emulation.setCPUThrottlingRate"; }

impl<'a> crate::CdpCommand<'a> for SetCPUThrottlingRateParams {
    const METHOD: &'static str = "Emulation.setCPUThrottlingRate";
    type Response = crate::EmptyReturns;
}

/// Sets or clears an override of the default background color of the frame. This override is used
/// if the content does not specify one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultBackgroundColorOverrideParams {
    /// RGBA of the default background color. If not specified, any existing override will be
    /// cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<crate::dom::RGBA>,
}

impl SetDefaultBackgroundColorOverrideParams {
    pub fn builder() -> SetDefaultBackgroundColorOverrideParamsBuilder {
        SetDefaultBackgroundColorOverrideParamsBuilder {
            color: None,
        }
    }
    pub fn color(&self) -> Option<&crate::dom::RGBA> { self.color.as_ref() }
}

#[derive(Default)]
pub struct SetDefaultBackgroundColorOverrideParamsBuilder {
    color: Option<crate::dom::RGBA>,
}

impl SetDefaultBackgroundColorOverrideParamsBuilder {
    /// RGBA of the default background color. If not specified, any existing override will be
    /// cleared.
    pub fn color(mut self, color: crate::dom::RGBA) -> Self { self.color = Some(color); self }
    pub fn build(self) -> SetDefaultBackgroundColorOverrideParams {
        SetDefaultBackgroundColorOverrideParams {
            color: self.color,
        }
    }
}

impl SetDefaultBackgroundColorOverrideParams { pub const METHOD: &'static str = "Emulation.setDefaultBackgroundColorOverride"; }

impl<'a> crate::CdpCommand<'a> for SetDefaultBackgroundColorOverrideParams {
    const METHOD: &'static str = "Emulation.setDefaultBackgroundColorOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the
/// respective variables to be undefined, even if previously overridden.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSafeAreaInsetsOverrideParams {
    insets: SafeAreaInsets,
}

impl SetSafeAreaInsetsOverrideParams {
    pub fn builder(insets: SafeAreaInsets) -> SetSafeAreaInsetsOverrideParamsBuilder {
        SetSafeAreaInsetsOverrideParamsBuilder {
            insets: insets,
        }
    }
    pub fn insets(&self) -> &SafeAreaInsets { &self.insets }
}


pub struct SetSafeAreaInsetsOverrideParamsBuilder {
    insets: SafeAreaInsets,
}

impl SetSafeAreaInsetsOverrideParamsBuilder {
    pub fn build(self) -> SetSafeAreaInsetsOverrideParams {
        SetSafeAreaInsetsOverrideParams {
            insets: self.insets,
        }
    }
}

impl SetSafeAreaInsetsOverrideParams { pub const METHOD: &'static str = "Emulation.setSafeAreaInsetsOverride"; }

impl<'a> crate::CdpCommand<'a> for SetSafeAreaInsetsOverrideParams {
    const METHOD: &'static str = "Emulation.setSafeAreaInsetsOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
/// window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
/// query results).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceMetricsOverrideParams<'a> {
    /// Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    width: u64,
    /// Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    height: i64,
    /// Overriding device scale factor value. 0 disables the override.
    deviceScaleFactor: f64,
    /// Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    /// autosizing and more.
    mobile: bool,
    /// Scale to apply to resulting view image.
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f64>,
    /// Overriding screen width value in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none")]
    screenWidth: Option<u64>,
    /// Overriding screen height value in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none")]
    screenHeight: Option<i64>,
    /// Overriding view X position on screen in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none")]
    positionX: Option<i64>,
    /// Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none")]
    positionY: Option<i64>,
    /// Do not set visible view size, rely upon explicit setVisibleSize call.
    #[serde(skip_serializing_if = "Option::is_none")]
    dontSetVisibleSize: Option<bool>,
    /// Screen orientation override.
    #[serde(skip_serializing_if = "Option::is_none")]
    screenOrientation: Option<ScreenOrientation<'a>>,
    /// If set, the visible area of the page will be overridden to this viewport. This viewport
    /// change is not observed by the page, e.g. viewport-relative elements do not change positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    viewport: Option<crate::page::Viewport>,
    /// If set, the display feature of a multi-segment screen. If not set, multi-segment support
    /// is turned-off.
    /// Deprecated, use Emulation.setDisplayFeaturesOverride.
    #[serde(skip_serializing_if = "Option::is_none")]
    displayFeature: Option<DisplayFeature<'a>>,
    /// If set, the posture of a foldable device. If not set the posture is set
    /// to continuous.
    /// Deprecated, use Emulation.setDevicePostureOverride.
    #[serde(skip_serializing_if = "Option::is_none")]
    devicePosture: Option<DevicePosture<'a>>,
    /// Scrollbar type. Default: 'default'.
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollbarType: Option<Cow<'a, str>>,
    /// If set to true, enables screen orientation lock emulation, which
    /// intercepts screen.orientation.lock() calls from the page and reports
    /// orientation changes via screenOrientationLockChanged events. This is
    /// useful for emulating mobile device orientation lock behavior in
    /// responsive design mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    screenOrientationLockEmulation: Option<bool>,
}

impl<'a> SetDeviceMetricsOverrideParams<'a> {
    pub fn builder(width: u64, height: i64, deviceScaleFactor: f64, mobile: bool) -> SetDeviceMetricsOverrideParamsBuilder<'a> {
        SetDeviceMetricsOverrideParamsBuilder {
            width: width,
            height: height,
            deviceScaleFactor: deviceScaleFactor,
            mobile: mobile,
            scale: None,
            screenWidth: None,
            screenHeight: None,
            positionX: None,
            positionY: None,
            dontSetVisibleSize: None,
            screenOrientation: None,
            viewport: None,
            displayFeature: None,
            devicePosture: None,
            scrollbarType: None,
            screenOrientationLockEmulation: None,
        }
    }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> i64 { self.height }
    pub fn deviceScaleFactor(&self) -> f64 { self.deviceScaleFactor }
    pub fn mobile(&self) -> bool { self.mobile }
    pub fn scale(&self) -> Option<f64> { self.scale }
    pub fn screenWidth(&self) -> Option<u64> { self.screenWidth }
    pub fn screenHeight(&self) -> Option<i64> { self.screenHeight }
    pub fn positionX(&self) -> Option<i64> { self.positionX }
    pub fn positionY(&self) -> Option<i64> { self.positionY }
    pub fn dontSetVisibleSize(&self) -> Option<bool> { self.dontSetVisibleSize }
    pub fn screenOrientation(&self) -> Option<&ScreenOrientation<'a>> { self.screenOrientation.as_ref() }
    pub fn viewport(&self) -> Option<&crate::page::Viewport> { self.viewport.as_ref() }
    pub fn displayFeature(&self) -> Option<&DisplayFeature<'a>> { self.displayFeature.as_ref() }
    pub fn devicePosture(&self) -> Option<&DevicePosture<'a>> { self.devicePosture.as_ref() }
    pub fn scrollbarType(&self) -> Option<&str> { self.scrollbarType.as_deref() }
    pub fn screenOrientationLockEmulation(&self) -> Option<bool> { self.screenOrientationLockEmulation }
}


pub struct SetDeviceMetricsOverrideParamsBuilder<'a> {
    width: u64,
    height: i64,
    deviceScaleFactor: f64,
    mobile: bool,
    scale: Option<f64>,
    screenWidth: Option<u64>,
    screenHeight: Option<i64>,
    positionX: Option<i64>,
    positionY: Option<i64>,
    dontSetVisibleSize: Option<bool>,
    screenOrientation: Option<ScreenOrientation<'a>>,
    viewport: Option<crate::page::Viewport>,
    displayFeature: Option<DisplayFeature<'a>>,
    devicePosture: Option<DevicePosture<'a>>,
    scrollbarType: Option<Cow<'a, str>>,
    screenOrientationLockEmulation: Option<bool>,
}

impl<'a> SetDeviceMetricsOverrideParamsBuilder<'a> {
    /// Scale to apply to resulting view image.
    pub fn scale(mut self, scale: f64) -> Self { self.scale = Some(scale); self }
    /// Overriding screen width value in pixels (minimum 0, maximum 10000000).
    pub fn screenWidth(mut self, screenWidth: u64) -> Self { self.screenWidth = Some(screenWidth); self }
    /// Overriding screen height value in pixels (minimum 0, maximum 10000000).
    pub fn screenHeight(mut self, screenHeight: i64) -> Self { self.screenHeight = Some(screenHeight); self }
    /// Overriding view X position on screen in pixels (minimum 0, maximum 10000000).
    pub fn positionX(mut self, positionX: i64) -> Self { self.positionX = Some(positionX); self }
    /// Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).
    pub fn positionY(mut self, positionY: i64) -> Self { self.positionY = Some(positionY); self }
    /// Do not set visible view size, rely upon explicit setVisibleSize call.
    pub fn dontSetVisibleSize(mut self, dontSetVisibleSize: bool) -> Self { self.dontSetVisibleSize = Some(dontSetVisibleSize); self }
    /// Screen orientation override.
    pub fn screenOrientation(mut self, screenOrientation: ScreenOrientation<'a>) -> Self { self.screenOrientation = Some(screenOrientation); self }
    /// If set, the visible area of the page will be overridden to this viewport. This viewport
    /// change is not observed by the page, e.g. viewport-relative elements do not change positions.
    pub fn viewport(mut self, viewport: crate::page::Viewport) -> Self { self.viewport = Some(viewport); self }
    /// If set, the display feature of a multi-segment screen. If not set, multi-segment support
    /// is turned-off.
    /// Deprecated, use Emulation.setDisplayFeaturesOverride.
    pub fn displayFeature(mut self, displayFeature: DisplayFeature<'a>) -> Self { self.displayFeature = Some(displayFeature); self }
    /// If set, the posture of a foldable device. If not set the posture is set
    /// to continuous.
    /// Deprecated, use Emulation.setDevicePostureOverride.
    pub fn devicePosture(mut self, devicePosture: DevicePosture<'a>) -> Self { self.devicePosture = Some(devicePosture); self }
    /// Scrollbar type. Default: 'default'.
    pub fn scrollbarType(mut self, scrollbarType: impl Into<Cow<'a, str>>) -> Self { self.scrollbarType = Some(scrollbarType.into()); self }
    /// If set to true, enables screen orientation lock emulation, which
    /// intercepts screen.orientation.lock() calls from the page and reports
    /// orientation changes via screenOrientationLockChanged events. This is
    /// useful for emulating mobile device orientation lock behavior in
    /// responsive design mode.
    pub fn screenOrientationLockEmulation(mut self, screenOrientationLockEmulation: bool) -> Self { self.screenOrientationLockEmulation = Some(screenOrientationLockEmulation); self }
    pub fn build(self) -> SetDeviceMetricsOverrideParams<'a> {
        SetDeviceMetricsOverrideParams {
            width: self.width,
            height: self.height,
            deviceScaleFactor: self.deviceScaleFactor,
            mobile: self.mobile,
            scale: self.scale,
            screenWidth: self.screenWidth,
            screenHeight: self.screenHeight,
            positionX: self.positionX,
            positionY: self.positionY,
            dontSetVisibleSize: self.dontSetVisibleSize,
            screenOrientation: self.screenOrientation,
            viewport: self.viewport,
            displayFeature: self.displayFeature,
            devicePosture: self.devicePosture,
            scrollbarType: self.scrollbarType,
            screenOrientationLockEmulation: self.screenOrientationLockEmulation,
        }
    }
}

impl<'a> SetDeviceMetricsOverrideParams<'a> { pub const METHOD: &'static str = "Emulation.setDeviceMetricsOverride"; }

impl<'a> crate::CdpCommand<'a> for SetDeviceMetricsOverrideParams<'a> {
    const METHOD: &'static str = "Emulation.setDeviceMetricsOverride";
    type Response = crate::EmptyReturns;
}

/// Start reporting the given posture value to the Device Posture API.
/// This override can also be set in setDeviceMetricsOverride().

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDevicePostureOverrideParams<'a> {
    posture: DevicePosture<'a>,
}

impl<'a> SetDevicePostureOverrideParams<'a> {
    pub fn builder(posture: DevicePosture<'a>) -> SetDevicePostureOverrideParamsBuilder<'a> {
        SetDevicePostureOverrideParamsBuilder {
            posture: posture,
        }
    }
    pub fn posture(&self) -> &DevicePosture<'a> { &self.posture }
}


pub struct SetDevicePostureOverrideParamsBuilder<'a> {
    posture: DevicePosture<'a>,
}

impl<'a> SetDevicePostureOverrideParamsBuilder<'a> {
    pub fn build(self) -> SetDevicePostureOverrideParams<'a> {
        SetDevicePostureOverrideParams {
            posture: self.posture,
        }
    }
}

impl<'a> SetDevicePostureOverrideParams<'a> { pub const METHOD: &'static str = "Emulation.setDevicePostureOverride"; }

impl<'a> crate::CdpCommand<'a> for SetDevicePostureOverrideParams<'a> {
    const METHOD: &'static str = "Emulation.setDevicePostureOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDevicePostureOverrideParams {}

impl ClearDevicePostureOverrideParams { pub const METHOD: &'static str = "Emulation.clearDevicePostureOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearDevicePostureOverrideParams {
    const METHOD: &'static str = "Emulation.clearDevicePostureOverride";
    type Response = crate::EmptyReturns;
}

/// Start using the given display features to pupulate the Viewport Segments API.
/// This override can also be set in setDeviceMetricsOverride().

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayFeaturesOverrideParams<'a> {
    features: Vec<DisplayFeature<'a>>,
}

impl<'a> SetDisplayFeaturesOverrideParams<'a> {
    pub fn builder(features: Vec<DisplayFeature<'a>>) -> SetDisplayFeaturesOverrideParamsBuilder<'a> {
        SetDisplayFeaturesOverrideParamsBuilder {
            features: features,
        }
    }
    pub fn features(&self) -> &[DisplayFeature<'a>] { &self.features }
}


pub struct SetDisplayFeaturesOverrideParamsBuilder<'a> {
    features: Vec<DisplayFeature<'a>>,
}

impl<'a> SetDisplayFeaturesOverrideParamsBuilder<'a> {
    pub fn build(self) -> SetDisplayFeaturesOverrideParams<'a> {
        SetDisplayFeaturesOverrideParams {
            features: self.features,
        }
    }
}

impl<'a> SetDisplayFeaturesOverrideParams<'a> { pub const METHOD: &'static str = "Emulation.setDisplayFeaturesOverride"; }

impl<'a> crate::CdpCommand<'a> for SetDisplayFeaturesOverrideParams<'a> {
    const METHOD: &'static str = "Emulation.setDisplayFeaturesOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDisplayFeaturesOverrideParams {}

impl ClearDisplayFeaturesOverrideParams { pub const METHOD: &'static str = "Emulation.clearDisplayFeaturesOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearDisplayFeaturesOverrideParams {
    const METHOD: &'static str = "Emulation.clearDisplayFeaturesOverride";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScrollbarsHiddenParams {
    /// Whether scrollbars should be always hidden.
    hidden: bool,
}

impl SetScrollbarsHiddenParams {
    pub fn builder(hidden: bool) -> SetScrollbarsHiddenParamsBuilder {
        SetScrollbarsHiddenParamsBuilder {
            hidden: hidden,
        }
    }
    pub fn hidden(&self) -> bool { self.hidden }
}


pub struct SetScrollbarsHiddenParamsBuilder {
    hidden: bool,
}

impl SetScrollbarsHiddenParamsBuilder {
    pub fn build(self) -> SetScrollbarsHiddenParams {
        SetScrollbarsHiddenParams {
            hidden: self.hidden,
        }
    }
}

impl SetScrollbarsHiddenParams { pub const METHOD: &'static str = "Emulation.setScrollbarsHidden"; }

impl<'a> crate::CdpCommand<'a> for SetScrollbarsHiddenParams {
    const METHOD: &'static str = "Emulation.setScrollbarsHidden";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentCookieDisabledParams {
    /// Whether document.coookie API should be disabled.
    disabled: bool,
}

impl SetDocumentCookieDisabledParams {
    pub fn builder(disabled: bool) -> SetDocumentCookieDisabledParamsBuilder {
        SetDocumentCookieDisabledParamsBuilder {
            disabled: disabled,
        }
    }
    pub fn disabled(&self) -> bool { self.disabled }
}


pub struct SetDocumentCookieDisabledParamsBuilder {
    disabled: bool,
}

impl SetDocumentCookieDisabledParamsBuilder {
    pub fn build(self) -> SetDocumentCookieDisabledParams {
        SetDocumentCookieDisabledParams {
            disabled: self.disabled,
        }
    }
}

impl SetDocumentCookieDisabledParams { pub const METHOD: &'static str = "Emulation.setDocumentCookieDisabled"; }

impl<'a> crate::CdpCommand<'a> for SetDocumentCookieDisabledParams {
    const METHOD: &'static str = "Emulation.setDocumentCookieDisabled";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEmitTouchEventsForMouseParams<'a> {
    /// Whether touch emulation based on mouse input should be enabled.
    enabled: bool,
    /// Touch/gesture events configuration. Default: current platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Cow<'a, str>>,
}

impl<'a> SetEmitTouchEventsForMouseParams<'a> {
    pub fn builder(enabled: bool) -> SetEmitTouchEventsForMouseParamsBuilder<'a> {
        SetEmitTouchEventsForMouseParamsBuilder {
            enabled: enabled,
            configuration: None,
        }
    }
    pub fn enabled(&self) -> bool { self.enabled }
    pub fn configuration(&self) -> Option<&str> { self.configuration.as_deref() }
}


pub struct SetEmitTouchEventsForMouseParamsBuilder<'a> {
    enabled: bool,
    configuration: Option<Cow<'a, str>>,
}

impl<'a> SetEmitTouchEventsForMouseParamsBuilder<'a> {
    /// Touch/gesture events configuration. Default: current platform.
    pub fn configuration(mut self, configuration: impl Into<Cow<'a, str>>) -> Self { self.configuration = Some(configuration.into()); self }
    pub fn build(self) -> SetEmitTouchEventsForMouseParams<'a> {
        SetEmitTouchEventsForMouseParams {
            enabled: self.enabled,
            configuration: self.configuration,
        }
    }
}

impl<'a> SetEmitTouchEventsForMouseParams<'a> { pub const METHOD: &'static str = "Emulation.setEmitTouchEventsForMouse"; }

impl<'a> crate::CdpCommand<'a> for SetEmitTouchEventsForMouseParams<'a> {
    const METHOD: &'static str = "Emulation.setEmitTouchEventsForMouse";
    type Response = crate::EmptyReturns;
}

/// Emulates the given media type or media feature for CSS media queries.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedMediaParams<'a> {
    /// Media type to emulate. Empty string disables the override.
    #[serde(skip_serializing_if = "Option::is_none")]
    media: Option<Cow<'a, str>>,
    /// Media features to emulate.
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<Vec<MediaFeature<'a>>>,
}

impl<'a> SetEmulatedMediaParams<'a> {
    pub fn builder() -> SetEmulatedMediaParamsBuilder<'a> {
        SetEmulatedMediaParamsBuilder {
            media: None,
            features: None,
        }
    }
    pub fn media(&self) -> Option<&str> { self.media.as_deref() }
    pub fn features(&self) -> Option<&[MediaFeature<'a>]> { self.features.as_deref() }
}

#[derive(Default)]
pub struct SetEmulatedMediaParamsBuilder<'a> {
    media: Option<Cow<'a, str>>,
    features: Option<Vec<MediaFeature<'a>>>,
}

impl<'a> SetEmulatedMediaParamsBuilder<'a> {
    /// Media type to emulate. Empty string disables the override.
    pub fn media(mut self, media: impl Into<Cow<'a, str>>) -> Self { self.media = Some(media.into()); self }
    /// Media features to emulate.
    pub fn features(mut self, features: Vec<MediaFeature<'a>>) -> Self { self.features = Some(features); self }
    pub fn build(self) -> SetEmulatedMediaParams<'a> {
        SetEmulatedMediaParams {
            media: self.media,
            features: self.features,
        }
    }
}

impl<'a> SetEmulatedMediaParams<'a> { pub const METHOD: &'static str = "Emulation.setEmulatedMedia"; }

impl<'a> crate::CdpCommand<'a> for SetEmulatedMediaParams<'a> {
    const METHOD: &'static str = "Emulation.setEmulatedMedia";
    type Response = crate::EmptyReturns;
}

/// Emulates the given vision deficiency.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedVisionDeficiencyParams<'a> {
    /// Vision deficiency to emulate. Order: best-effort emulations come first, followed by any
    /// physiologically accurate emulations for medically recognized color vision deficiencies.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
}

impl<'a> SetEmulatedVisionDeficiencyParams<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> SetEmulatedVisionDeficiencyParamsBuilder<'a> {
        SetEmulatedVisionDeficiencyParamsBuilder {
            type_: type_.into(),
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}


pub struct SetEmulatedVisionDeficiencyParamsBuilder<'a> {
    type_: Cow<'a, str>,
}

impl<'a> SetEmulatedVisionDeficiencyParamsBuilder<'a> {
    pub fn build(self) -> SetEmulatedVisionDeficiencyParams<'a> {
        SetEmulatedVisionDeficiencyParams {
            type_: self.type_,
        }
    }
}

impl<'a> SetEmulatedVisionDeficiencyParams<'a> { pub const METHOD: &'static str = "Emulation.setEmulatedVisionDeficiency"; }

impl<'a> crate::CdpCommand<'a> for SetEmulatedVisionDeficiencyParams<'a> {
    const METHOD: &'static str = "Emulation.setEmulatedVisionDeficiency";
    type Response = crate::EmptyReturns;
}

/// Emulates the given OS text scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedOSTextScaleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f64>,
}

impl SetEmulatedOSTextScaleParams {
    pub fn builder() -> SetEmulatedOSTextScaleParamsBuilder {
        SetEmulatedOSTextScaleParamsBuilder {
            scale: None,
        }
    }
    pub fn scale(&self) -> Option<f64> { self.scale }
}

#[derive(Default)]
pub struct SetEmulatedOSTextScaleParamsBuilder {
    scale: Option<f64>,
}

impl SetEmulatedOSTextScaleParamsBuilder {
    pub fn scale(mut self, scale: f64) -> Self { self.scale = Some(scale); self }
    pub fn build(self) -> SetEmulatedOSTextScaleParams {
        SetEmulatedOSTextScaleParams {
            scale: self.scale,
        }
    }
}

impl SetEmulatedOSTextScaleParams { pub const METHOD: &'static str = "Emulation.setEmulatedOSTextScale"; }

impl<'a> crate::CdpCommand<'a> for SetEmulatedOSTextScaleParams {
    const METHOD: &'static str = "Emulation.setEmulatedOSTextScale";
    type Response = crate::EmptyReturns;
}

/// Overrides the Geolocation Position or Error. Omitting latitude, longitude or
/// accuracy emulates position unavailable.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetGeolocationOverrideParams {
    /// Mock latitude
    #[serde(skip_serializing_if = "Option::is_none")]
    latitude: Option<f64>,
    /// Mock longitude
    #[serde(skip_serializing_if = "Option::is_none")]
    longitude: Option<f64>,
    /// Mock accuracy
    #[serde(skip_serializing_if = "Option::is_none")]
    accuracy: Option<f64>,
    /// Mock altitude
    #[serde(skip_serializing_if = "Option::is_none")]
    altitude: Option<f64>,
    /// Mock altitudeAccuracy
    #[serde(skip_serializing_if = "Option::is_none")]
    altitudeAccuracy: Option<f64>,
    /// Mock heading
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<f64>,
    /// Mock speed
    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<f64>,
}

impl SetGeolocationOverrideParams {
    pub fn builder() -> SetGeolocationOverrideParamsBuilder {
        SetGeolocationOverrideParamsBuilder {
            latitude: None,
            longitude: None,
            accuracy: None,
            altitude: None,
            altitudeAccuracy: None,
            heading: None,
            speed: None,
        }
    }
    pub fn latitude(&self) -> Option<f64> { self.latitude }
    pub fn longitude(&self) -> Option<f64> { self.longitude }
    pub fn accuracy(&self) -> Option<f64> { self.accuracy }
    pub fn altitude(&self) -> Option<f64> { self.altitude }
    pub fn altitudeAccuracy(&self) -> Option<f64> { self.altitudeAccuracy }
    pub fn heading(&self) -> Option<f64> { self.heading }
    pub fn speed(&self) -> Option<f64> { self.speed }
}

#[derive(Default)]
pub struct SetGeolocationOverrideParamsBuilder {
    latitude: Option<f64>,
    longitude: Option<f64>,
    accuracy: Option<f64>,
    altitude: Option<f64>,
    altitudeAccuracy: Option<f64>,
    heading: Option<f64>,
    speed: Option<f64>,
}

impl SetGeolocationOverrideParamsBuilder {
    /// Mock latitude
    pub fn latitude(mut self, latitude: f64) -> Self { self.latitude = Some(latitude); self }
    /// Mock longitude
    pub fn longitude(mut self, longitude: f64) -> Self { self.longitude = Some(longitude); self }
    /// Mock accuracy
    pub fn accuracy(mut self, accuracy: f64) -> Self { self.accuracy = Some(accuracy); self }
    /// Mock altitude
    pub fn altitude(mut self, altitude: f64) -> Self { self.altitude = Some(altitude); self }
    /// Mock altitudeAccuracy
    pub fn altitudeAccuracy(mut self, altitudeAccuracy: f64) -> Self { self.altitudeAccuracy = Some(altitudeAccuracy); self }
    /// Mock heading
    pub fn heading(mut self, heading: f64) -> Self { self.heading = Some(heading); self }
    /// Mock speed
    pub fn speed(mut self, speed: f64) -> Self { self.speed = Some(speed); self }
    pub fn build(self) -> SetGeolocationOverrideParams {
        SetGeolocationOverrideParams {
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy: self.accuracy,
            altitude: self.altitude,
            altitudeAccuracy: self.altitudeAccuracy,
            heading: self.heading,
            speed: self.speed,
        }
    }
}

impl SetGeolocationOverrideParams { pub const METHOD: &'static str = "Emulation.setGeolocationOverride"; }

impl<'a> crate::CdpCommand<'a> for SetGeolocationOverrideParams {
    const METHOD: &'static str = "Emulation.setGeolocationOverride";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOverriddenSensorInformationParams {
    #[serde(rename = "type")]
    type_: SensorType,
}

impl GetOverriddenSensorInformationParams {
    pub fn builder(type_: SensorType) -> GetOverriddenSensorInformationParamsBuilder {
        GetOverriddenSensorInformationParamsBuilder {
            type_: type_,
        }
    }
    pub fn type_(&self) -> &SensorType { &self.type_ }
}


pub struct GetOverriddenSensorInformationParamsBuilder {
    type_: SensorType,
}

impl GetOverriddenSensorInformationParamsBuilder {
    pub fn build(self) -> GetOverriddenSensorInformationParams {
        GetOverriddenSensorInformationParams {
            type_: self.type_,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOverriddenSensorInformationReturns {
    requestedSamplingFrequency: f64,
}

impl GetOverriddenSensorInformationReturns {
    pub fn builder(requestedSamplingFrequency: f64) -> GetOverriddenSensorInformationReturnsBuilder {
        GetOverriddenSensorInformationReturnsBuilder {
            requestedSamplingFrequency: requestedSamplingFrequency,
        }
    }
    pub fn requestedSamplingFrequency(&self) -> f64 { self.requestedSamplingFrequency }
}


pub struct GetOverriddenSensorInformationReturnsBuilder {
    requestedSamplingFrequency: f64,
}

impl GetOverriddenSensorInformationReturnsBuilder {
    pub fn build(self) -> GetOverriddenSensorInformationReturns {
        GetOverriddenSensorInformationReturns {
            requestedSamplingFrequency: self.requestedSamplingFrequency,
        }
    }
}

impl GetOverriddenSensorInformationParams { pub const METHOD: &'static str = "Emulation.getOverriddenSensorInformation"; }

impl<'a> crate::CdpCommand<'a> for GetOverriddenSensorInformationParams {
    const METHOD: &'static str = "Emulation.getOverriddenSensorInformation";
    type Response = GetOverriddenSensorInformationReturns;
}

/// Overrides a platform sensor of a given type. If |enabled| is true, calls to
/// Sensor.start() will use a virtual sensor as backend rather than fetching
/// data from a real hardware sensor. Otherwise, existing virtual
/// sensor-backend Sensor objects will fire an error event and new calls to
/// Sensor.start() will attempt to use a real sensor instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSensorOverrideEnabledParams {
    enabled: bool,
    #[serde(rename = "type")]
    type_: SensorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<SensorMetadata>,
}

impl SetSensorOverrideEnabledParams {
    pub fn builder(enabled: bool, type_: SensorType) -> SetSensorOverrideEnabledParamsBuilder {
        SetSensorOverrideEnabledParamsBuilder {
            enabled: enabled,
            type_: type_,
            metadata: None,
        }
    }
    pub fn enabled(&self) -> bool { self.enabled }
    pub fn type_(&self) -> &SensorType { &self.type_ }
    pub fn metadata(&self) -> Option<&SensorMetadata> { self.metadata.as_ref() }
}


pub struct SetSensorOverrideEnabledParamsBuilder {
    enabled: bool,
    type_: SensorType,
    metadata: Option<SensorMetadata>,
}

impl SetSensorOverrideEnabledParamsBuilder {
    pub fn metadata(mut self, metadata: SensorMetadata) -> Self { self.metadata = Some(metadata); self }
    pub fn build(self) -> SetSensorOverrideEnabledParams {
        SetSensorOverrideEnabledParams {
            enabled: self.enabled,
            type_: self.type_,
            metadata: self.metadata,
        }
    }
}

impl SetSensorOverrideEnabledParams { pub const METHOD: &'static str = "Emulation.setSensorOverrideEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetSensorOverrideEnabledParams {
    const METHOD: &'static str = "Emulation.setSensorOverrideEnabled";
    type Response = crate::EmptyReturns;
}

/// Updates the sensor readings reported by a sensor type previously overridden
/// by setSensorOverrideEnabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSensorOverrideReadingsParams {
    #[serde(rename = "type")]
    type_: SensorType,
    reading: SensorReading,
}

impl SetSensorOverrideReadingsParams {
    pub fn builder(type_: SensorType, reading: SensorReading) -> SetSensorOverrideReadingsParamsBuilder {
        SetSensorOverrideReadingsParamsBuilder {
            type_: type_,
            reading: reading,
        }
    }
    pub fn type_(&self) -> &SensorType { &self.type_ }
    pub fn reading(&self) -> &SensorReading { &self.reading }
}


pub struct SetSensorOverrideReadingsParamsBuilder {
    type_: SensorType,
    reading: SensorReading,
}

impl SetSensorOverrideReadingsParamsBuilder {
    pub fn build(self) -> SetSensorOverrideReadingsParams {
        SetSensorOverrideReadingsParams {
            type_: self.type_,
            reading: self.reading,
        }
    }
}

impl SetSensorOverrideReadingsParams { pub const METHOD: &'static str = "Emulation.setSensorOverrideReadings"; }

impl<'a> crate::CdpCommand<'a> for SetSensorOverrideReadingsParams {
    const METHOD: &'static str = "Emulation.setSensorOverrideReadings";
    type Response = crate::EmptyReturns;
}

/// Overrides a pressure source of a given type, as used by the Compute
/// Pressure API, so that updates to PressureObserver.observe() are provided
/// via setPressureStateOverride instead of being retrieved from
/// platform-provided telemetry data.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureSourceOverrideEnabledParams {
    enabled: bool,
    source: PressureSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PressureMetadata>,
}

impl SetPressureSourceOverrideEnabledParams {
    pub fn builder(enabled: bool, source: PressureSource) -> SetPressureSourceOverrideEnabledParamsBuilder {
        SetPressureSourceOverrideEnabledParamsBuilder {
            enabled: enabled,
            source: source,
            metadata: None,
        }
    }
    pub fn enabled(&self) -> bool { self.enabled }
    pub fn source(&self) -> &PressureSource { &self.source }
    pub fn metadata(&self) -> Option<&PressureMetadata> { self.metadata.as_ref() }
}


pub struct SetPressureSourceOverrideEnabledParamsBuilder {
    enabled: bool,
    source: PressureSource,
    metadata: Option<PressureMetadata>,
}

impl SetPressureSourceOverrideEnabledParamsBuilder {
    pub fn metadata(mut self, metadata: PressureMetadata) -> Self { self.metadata = Some(metadata); self }
    pub fn build(self) -> SetPressureSourceOverrideEnabledParams {
        SetPressureSourceOverrideEnabledParams {
            enabled: self.enabled,
            source: self.source,
            metadata: self.metadata,
        }
    }
}

impl SetPressureSourceOverrideEnabledParams { pub const METHOD: &'static str = "Emulation.setPressureSourceOverrideEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetPressureSourceOverrideEnabledParams {
    const METHOD: &'static str = "Emulation.setPressureSourceOverrideEnabled";
    type Response = crate::EmptyReturns;
}

/// TODO: OBSOLETE: To remove when setPressureDataOverride is merged.
/// Provides a given pressure state that will be processed and eventually be
/// delivered to PressureObserver users. |source| must have been previously
/// overridden by setPressureSourceOverrideEnabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureStateOverrideParams {
    source: PressureSource,
    state: PressureState,
}

impl SetPressureStateOverrideParams {
    pub fn builder(source: PressureSource, state: PressureState) -> SetPressureStateOverrideParamsBuilder {
        SetPressureStateOverrideParamsBuilder {
            source: source,
            state: state,
        }
    }
    pub fn source(&self) -> &PressureSource { &self.source }
    pub fn state(&self) -> &PressureState { &self.state }
}


pub struct SetPressureStateOverrideParamsBuilder {
    source: PressureSource,
    state: PressureState,
}

impl SetPressureStateOverrideParamsBuilder {
    pub fn build(self) -> SetPressureStateOverrideParams {
        SetPressureStateOverrideParams {
            source: self.source,
            state: self.state,
        }
    }
}

impl SetPressureStateOverrideParams { pub const METHOD: &'static str = "Emulation.setPressureStateOverride"; }

impl<'a> crate::CdpCommand<'a> for SetPressureStateOverrideParams {
    const METHOD: &'static str = "Emulation.setPressureStateOverride";
    type Response = crate::EmptyReturns;
}

/// Provides a given pressure data set that will be processed and eventually be
/// delivered to PressureObserver users. |source| must have been previously
/// overridden by setPressureSourceOverrideEnabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureDataOverrideParams {
    source: PressureSource,
    state: PressureState,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownContributionEstimate: Option<f64>,
}

impl SetPressureDataOverrideParams {
    pub fn builder(source: PressureSource, state: PressureState) -> SetPressureDataOverrideParamsBuilder {
        SetPressureDataOverrideParamsBuilder {
            source: source,
            state: state,
            ownContributionEstimate: None,
        }
    }
    pub fn source(&self) -> &PressureSource { &self.source }
    pub fn state(&self) -> &PressureState { &self.state }
    pub fn ownContributionEstimate(&self) -> Option<f64> { self.ownContributionEstimate }
}


pub struct SetPressureDataOverrideParamsBuilder {
    source: PressureSource,
    state: PressureState,
    ownContributionEstimate: Option<f64>,
}

impl SetPressureDataOverrideParamsBuilder {
    pub fn ownContributionEstimate(mut self, ownContributionEstimate: f64) -> Self { self.ownContributionEstimate = Some(ownContributionEstimate); self }
    pub fn build(self) -> SetPressureDataOverrideParams {
        SetPressureDataOverrideParams {
            source: self.source,
            state: self.state,
            ownContributionEstimate: self.ownContributionEstimate,
        }
    }
}

impl SetPressureDataOverrideParams { pub const METHOD: &'static str = "Emulation.setPressureDataOverride"; }

impl<'a> crate::CdpCommand<'a> for SetPressureDataOverrideParams {
    const METHOD: &'static str = "Emulation.setPressureDataOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides the Idle state.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetIdleOverrideParams {
    /// Mock isUserActive
    isUserActive: bool,
    /// Mock isScreenUnlocked
    isScreenUnlocked: bool,
}

impl SetIdleOverrideParams {
    pub fn builder(isUserActive: bool, isScreenUnlocked: bool) -> SetIdleOverrideParamsBuilder {
        SetIdleOverrideParamsBuilder {
            isUserActive: isUserActive,
            isScreenUnlocked: isScreenUnlocked,
        }
    }
    pub fn isUserActive(&self) -> bool { self.isUserActive }
    pub fn isScreenUnlocked(&self) -> bool { self.isScreenUnlocked }
}


pub struct SetIdleOverrideParamsBuilder {
    isUserActive: bool,
    isScreenUnlocked: bool,
}

impl SetIdleOverrideParamsBuilder {
    pub fn build(self) -> SetIdleOverrideParams {
        SetIdleOverrideParams {
            isUserActive: self.isUserActive,
            isScreenUnlocked: self.isScreenUnlocked,
        }
    }
}

impl SetIdleOverrideParams { pub const METHOD: &'static str = "Emulation.setIdleOverride"; }

impl<'a> crate::CdpCommand<'a> for SetIdleOverrideParams {
    const METHOD: &'static str = "Emulation.setIdleOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearIdleOverrideParams {}

impl ClearIdleOverrideParams { pub const METHOD: &'static str = "Emulation.clearIdleOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearIdleOverrideParams {
    const METHOD: &'static str = "Emulation.clearIdleOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides value returned by the javascript navigator object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetNavigatorOverridesParams<'a> {
    /// The platform navigator.platform should return.
    platform: Cow<'a, str>,
}

impl<'a> SetNavigatorOverridesParams<'a> {
    pub fn builder(platform: impl Into<Cow<'a, str>>) -> SetNavigatorOverridesParamsBuilder<'a> {
        SetNavigatorOverridesParamsBuilder {
            platform: platform.into(),
        }
    }
    pub fn platform(&self) -> &str { self.platform.as_ref() }
}


pub struct SetNavigatorOverridesParamsBuilder<'a> {
    platform: Cow<'a, str>,
}

impl<'a> SetNavigatorOverridesParamsBuilder<'a> {
    pub fn build(self) -> SetNavigatorOverridesParams<'a> {
        SetNavigatorOverridesParams {
            platform: self.platform,
        }
    }
}

impl<'a> SetNavigatorOverridesParams<'a> { pub const METHOD: &'static str = "Emulation.setNavigatorOverrides"; }

impl<'a> crate::CdpCommand<'a> for SetNavigatorOverridesParams<'a> {
    const METHOD: &'static str = "Emulation.setNavigatorOverrides";
    type Response = crate::EmptyReturns;
}

/// Sets a specified page scale factor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPageScaleFactorParams {
    /// Page scale factor.
    pageScaleFactor: f64,
}

impl SetPageScaleFactorParams {
    pub fn builder(pageScaleFactor: f64) -> SetPageScaleFactorParamsBuilder {
        SetPageScaleFactorParamsBuilder {
            pageScaleFactor: pageScaleFactor,
        }
    }
    pub fn pageScaleFactor(&self) -> f64 { self.pageScaleFactor }
}


pub struct SetPageScaleFactorParamsBuilder {
    pageScaleFactor: f64,
}

impl SetPageScaleFactorParamsBuilder {
    pub fn build(self) -> SetPageScaleFactorParams {
        SetPageScaleFactorParams {
            pageScaleFactor: self.pageScaleFactor,
        }
    }
}

impl SetPageScaleFactorParams { pub const METHOD: &'static str = "Emulation.setPageScaleFactor"; }

impl<'a> crate::CdpCommand<'a> for SetPageScaleFactorParams {
    const METHOD: &'static str = "Emulation.setPageScaleFactor";
    type Response = crate::EmptyReturns;
}

/// Switches script execution in the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScriptExecutionDisabledParams {
    /// Whether script execution should be disabled in the page.
    value: bool,
}

impl SetScriptExecutionDisabledParams {
    pub fn builder(value: bool) -> SetScriptExecutionDisabledParamsBuilder {
        SetScriptExecutionDisabledParamsBuilder {
            value: value,
        }
    }
    pub fn value(&self) -> bool { self.value }
}


pub struct SetScriptExecutionDisabledParamsBuilder {
    value: bool,
}

impl SetScriptExecutionDisabledParamsBuilder {
    pub fn build(self) -> SetScriptExecutionDisabledParams {
        SetScriptExecutionDisabledParams {
            value: self.value,
        }
    }
}

impl SetScriptExecutionDisabledParams { pub const METHOD: &'static str = "Emulation.setScriptExecutionDisabled"; }

impl<'a> crate::CdpCommand<'a> for SetScriptExecutionDisabledParams {
    const METHOD: &'static str = "Emulation.setScriptExecutionDisabled";
    type Response = crate::EmptyReturns;
}

/// Enables touch on platforms which do not support them.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTouchEmulationEnabledParams {
    /// Whether the touch event emulation should be enabled.
    enabled: bool,
    /// Maximum touch points supported. Defaults to one.
    #[serde(skip_serializing_if = "Option::is_none")]
    maxTouchPoints: Option<i64>,
}

impl SetTouchEmulationEnabledParams {
    pub fn builder(enabled: bool) -> SetTouchEmulationEnabledParamsBuilder {
        SetTouchEmulationEnabledParamsBuilder {
            enabled: enabled,
            maxTouchPoints: None,
        }
    }
    pub fn enabled(&self) -> bool { self.enabled }
    pub fn maxTouchPoints(&self) -> Option<i64> { self.maxTouchPoints }
}


pub struct SetTouchEmulationEnabledParamsBuilder {
    enabled: bool,
    maxTouchPoints: Option<i64>,
}

impl SetTouchEmulationEnabledParamsBuilder {
    /// Maximum touch points supported. Defaults to one.
    pub fn maxTouchPoints(mut self, maxTouchPoints: i64) -> Self { self.maxTouchPoints = Some(maxTouchPoints); self }
    pub fn build(self) -> SetTouchEmulationEnabledParams {
        SetTouchEmulationEnabledParams {
            enabled: self.enabled,
            maxTouchPoints: self.maxTouchPoints,
        }
    }
}

impl SetTouchEmulationEnabledParams { pub const METHOD: &'static str = "Emulation.setTouchEmulationEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetTouchEmulationEnabledParams {
    const METHOD: &'static str = "Emulation.setTouchEmulationEnabled";
    type Response = crate::EmptyReturns;
}

/// Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
/// the current virtual time policy.  Note this supersedes any previous time budget.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVirtualTimePolicyParams {
    policy: VirtualTimePolicy,
    /// If set, after this many virtual milliseconds have elapsed virtual time will be paused and a
    /// virtualTimeBudgetExpired event is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    budget: Option<f64>,
    /// If set this specifies the maximum number of tasks that can be run before virtual is forced
    /// forwards to prevent deadlock.
    #[serde(skip_serializing_if = "Option::is_none")]
    maxVirtualTimeTaskStarvationCount: Option<u64>,
    /// If set, base::Time::Now will be overridden to initially return this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    initialVirtualTime: Option<crate::network::TimeSinceEpoch>,
}

impl SetVirtualTimePolicyParams {
    pub fn builder(policy: VirtualTimePolicy) -> SetVirtualTimePolicyParamsBuilder {
        SetVirtualTimePolicyParamsBuilder {
            policy: policy,
            budget: None,
            maxVirtualTimeTaskStarvationCount: None,
            initialVirtualTime: None,
        }
    }
    pub fn policy(&self) -> &VirtualTimePolicy { &self.policy }
    pub fn budget(&self) -> Option<f64> { self.budget }
    pub fn maxVirtualTimeTaskStarvationCount(&self) -> Option<u64> { self.maxVirtualTimeTaskStarvationCount }
    pub fn initialVirtualTime(&self) -> Option<&crate::network::TimeSinceEpoch> { self.initialVirtualTime.as_ref() }
}


pub struct SetVirtualTimePolicyParamsBuilder {
    policy: VirtualTimePolicy,
    budget: Option<f64>,
    maxVirtualTimeTaskStarvationCount: Option<u64>,
    initialVirtualTime: Option<crate::network::TimeSinceEpoch>,
}

impl SetVirtualTimePolicyParamsBuilder {
    /// If set, after this many virtual milliseconds have elapsed virtual time will be paused and a
    /// virtualTimeBudgetExpired event is sent.
    pub fn budget(mut self, budget: f64) -> Self { self.budget = Some(budget); self }
    /// If set this specifies the maximum number of tasks that can be run before virtual is forced
    /// forwards to prevent deadlock.
    pub fn maxVirtualTimeTaskStarvationCount(mut self, maxVirtualTimeTaskStarvationCount: u64) -> Self { self.maxVirtualTimeTaskStarvationCount = Some(maxVirtualTimeTaskStarvationCount); self }
    /// If set, base::Time::Now will be overridden to initially return this value.
    pub fn initialVirtualTime(mut self, initialVirtualTime: crate::network::TimeSinceEpoch) -> Self { self.initialVirtualTime = Some(initialVirtualTime); self }
    pub fn build(self) -> SetVirtualTimePolicyParams {
        SetVirtualTimePolicyParams {
            policy: self.policy,
            budget: self.budget,
            maxVirtualTimeTaskStarvationCount: self.maxVirtualTimeTaskStarvationCount,
            initialVirtualTime: self.initialVirtualTime,
        }
    }
}

/// Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
/// the current virtual time policy.  Note this supersedes any previous time budget.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVirtualTimePolicyReturns {
    /// Absolute timestamp at which virtual time was first enabled (up time in milliseconds).
    virtualTimeTicksBase: f64,
}

impl SetVirtualTimePolicyReturns {
    pub fn builder(virtualTimeTicksBase: f64) -> SetVirtualTimePolicyReturnsBuilder {
        SetVirtualTimePolicyReturnsBuilder {
            virtualTimeTicksBase: virtualTimeTicksBase,
        }
    }
    pub fn virtualTimeTicksBase(&self) -> f64 { self.virtualTimeTicksBase }
}


pub struct SetVirtualTimePolicyReturnsBuilder {
    virtualTimeTicksBase: f64,
}

impl SetVirtualTimePolicyReturnsBuilder {
    pub fn build(self) -> SetVirtualTimePolicyReturns {
        SetVirtualTimePolicyReturns {
            virtualTimeTicksBase: self.virtualTimeTicksBase,
        }
    }
}

impl SetVirtualTimePolicyParams { pub const METHOD: &'static str = "Emulation.setVirtualTimePolicy"; }

impl<'a> crate::CdpCommand<'a> for SetVirtualTimePolicyParams {
    const METHOD: &'static str = "Emulation.setVirtualTimePolicy";
    type Response = SetVirtualTimePolicyReturns;
}

/// Overrides default host system locale with the specified one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetLocaleOverrideParams<'a> {
    /// ICU style C locale (e.g. "en_US"). If not specified or empty, disables the override and
    /// restores default host system locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<Cow<'a, str>>,
}

impl<'a> SetLocaleOverrideParams<'a> {
    pub fn builder() -> SetLocaleOverrideParamsBuilder<'a> {
        SetLocaleOverrideParamsBuilder {
            locale: None,
        }
    }
    pub fn locale(&self) -> Option<&str> { self.locale.as_deref() }
}

#[derive(Default)]
pub struct SetLocaleOverrideParamsBuilder<'a> {
    locale: Option<Cow<'a, str>>,
}

impl<'a> SetLocaleOverrideParamsBuilder<'a> {
    /// ICU style C locale (e.g. "en_US"). If not specified or empty, disables the override and
    /// restores default host system locale.
    pub fn locale(mut self, locale: impl Into<Cow<'a, str>>) -> Self { self.locale = Some(locale.into()); self }
    pub fn build(self) -> SetLocaleOverrideParams<'a> {
        SetLocaleOverrideParams {
            locale: self.locale,
        }
    }
}

impl<'a> SetLocaleOverrideParams<'a> { pub const METHOD: &'static str = "Emulation.setLocaleOverride"; }

impl<'a> crate::CdpCommand<'a> for SetLocaleOverrideParams<'a> {
    const METHOD: &'static str = "Emulation.setLocaleOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides default host system timezone with the specified one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTimezoneOverrideParams<'a> {
    /// The timezone identifier. List of supported timezones:
    /// https://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt
    /// If empty, disables the override and restores default host system timezone.
    timezoneId: Cow<'a, str>,
}

impl<'a> SetTimezoneOverrideParams<'a> {
    pub fn builder(timezoneId: impl Into<Cow<'a, str>>) -> SetTimezoneOverrideParamsBuilder<'a> {
        SetTimezoneOverrideParamsBuilder {
            timezoneId: timezoneId.into(),
        }
    }
    pub fn timezoneId(&self) -> &str { self.timezoneId.as_ref() }
}


pub struct SetTimezoneOverrideParamsBuilder<'a> {
    timezoneId: Cow<'a, str>,
}

impl<'a> SetTimezoneOverrideParamsBuilder<'a> {
    pub fn build(self) -> SetTimezoneOverrideParams<'a> {
        SetTimezoneOverrideParams {
            timezoneId: self.timezoneId,
        }
    }
}

impl<'a> SetTimezoneOverrideParams<'a> { pub const METHOD: &'static str = "Emulation.setTimezoneOverride"; }

impl<'a> crate::CdpCommand<'a> for SetTimezoneOverrideParams<'a> {
    const METHOD: &'static str = "Emulation.setTimezoneOverride";
    type Response = crate::EmptyReturns;
}

/// Resizes the frame/viewport of the page. Note that this does not affect the frame's container
/// (e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
/// on Android.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVisibleSizeParams {
    /// Frame width (DIP).
    width: u64,
    /// Frame height (DIP).
    height: i64,
}

impl SetVisibleSizeParams {
    pub fn builder(width: u64, height: i64) -> SetVisibleSizeParamsBuilder {
        SetVisibleSizeParamsBuilder {
            width: width,
            height: height,
        }
    }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> i64 { self.height }
}


pub struct SetVisibleSizeParamsBuilder {
    width: u64,
    height: i64,
}

impl SetVisibleSizeParamsBuilder {
    pub fn build(self) -> SetVisibleSizeParams {
        SetVisibleSizeParams {
            width: self.width,
            height: self.height,
        }
    }
}

impl SetVisibleSizeParams { pub const METHOD: &'static str = "Emulation.setVisibleSize"; }

impl<'a> crate::CdpCommand<'a> for SetVisibleSizeParams {
    const METHOD: &'static str = "Emulation.setVisibleSize";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDisabledImageTypesParams {
    /// Image types to disable.
    imageTypes: Vec<DisabledImageType>,
}

impl SetDisabledImageTypesParams {
    pub fn builder(imageTypes: Vec<DisabledImageType>) -> SetDisabledImageTypesParamsBuilder {
        SetDisabledImageTypesParamsBuilder {
            imageTypes: imageTypes,
        }
    }
    pub fn imageTypes(&self) -> &[DisabledImageType] { &self.imageTypes }
}


pub struct SetDisabledImageTypesParamsBuilder {
    imageTypes: Vec<DisabledImageType>,
}

impl SetDisabledImageTypesParamsBuilder {
    pub fn build(self) -> SetDisabledImageTypesParams {
        SetDisabledImageTypesParams {
            imageTypes: self.imageTypes,
        }
    }
}

impl SetDisabledImageTypesParams { pub const METHOD: &'static str = "Emulation.setDisabledImageTypes"; }

impl<'a> crate::CdpCommand<'a> for SetDisabledImageTypesParams {
    const METHOD: &'static str = "Emulation.setDisabledImageTypes";
    type Response = crate::EmptyReturns;
}

/// Override the value of navigator.connection.saveData

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDataSaverOverrideParams {
    /// Override value. Omitting the parameter disables the override.
    #[serde(skip_serializing_if = "Option::is_none")]
    dataSaverEnabled: Option<bool>,
}

impl SetDataSaverOverrideParams {
    pub fn builder() -> SetDataSaverOverrideParamsBuilder {
        SetDataSaverOverrideParamsBuilder {
            dataSaverEnabled: None,
        }
    }
    pub fn dataSaverEnabled(&self) -> Option<bool> { self.dataSaverEnabled }
}

#[derive(Default)]
pub struct SetDataSaverOverrideParamsBuilder {
    dataSaverEnabled: Option<bool>,
}

impl SetDataSaverOverrideParamsBuilder {
    /// Override value. Omitting the parameter disables the override.
    pub fn dataSaverEnabled(mut self, dataSaverEnabled: bool) -> Self { self.dataSaverEnabled = Some(dataSaverEnabled); self }
    pub fn build(self) -> SetDataSaverOverrideParams {
        SetDataSaverOverrideParams {
            dataSaverEnabled: self.dataSaverEnabled,
        }
    }
}

impl SetDataSaverOverrideParams { pub const METHOD: &'static str = "Emulation.setDataSaverOverride"; }

impl<'a> crate::CdpCommand<'a> for SetDataSaverOverrideParams {
    const METHOD: &'static str = "Emulation.setDataSaverOverride";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetHardwareConcurrencyOverrideParams {
    /// Hardware concurrency to report
    hardwareConcurrency: i64,
}

impl SetHardwareConcurrencyOverrideParams {
    pub fn builder(hardwareConcurrency: i64) -> SetHardwareConcurrencyOverrideParamsBuilder {
        SetHardwareConcurrencyOverrideParamsBuilder {
            hardwareConcurrency: hardwareConcurrency,
        }
    }
    pub fn hardwareConcurrency(&self) -> i64 { self.hardwareConcurrency }
}


pub struct SetHardwareConcurrencyOverrideParamsBuilder {
    hardwareConcurrency: i64,
}

impl SetHardwareConcurrencyOverrideParamsBuilder {
    pub fn build(self) -> SetHardwareConcurrencyOverrideParams {
        SetHardwareConcurrencyOverrideParams {
            hardwareConcurrency: self.hardwareConcurrency,
        }
    }
}

impl SetHardwareConcurrencyOverrideParams { pub const METHOD: &'static str = "Emulation.setHardwareConcurrencyOverride"; }

impl<'a> crate::CdpCommand<'a> for SetHardwareConcurrencyOverrideParams {
    const METHOD: &'static str = "Emulation.setHardwareConcurrencyOverride";
    type Response = crate::EmptyReturns;
}

/// Allows overriding user agent with the given string.
/// 'userAgentMetadata' must be set for Client Hint headers to be sent.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAgentOverrideParams<'a> {
    /// User agent to use.
    userAgent: Cow<'a, str>,
    /// Browser language to emulate.
    #[serde(skip_serializing_if = "Option::is_none")]
    acceptLanguage: Option<Cow<'a, str>>,
    /// The platform navigator.platform should return.
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<Cow<'a, str>>,
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    #[serde(skip_serializing_if = "Option::is_none")]
    userAgentMetadata: Option<UserAgentMetadata<'a>>,
}

impl<'a> SetUserAgentOverrideParams<'a> {
    pub fn builder(userAgent: impl Into<Cow<'a, str>>) -> SetUserAgentOverrideParamsBuilder<'a> {
        SetUserAgentOverrideParamsBuilder {
            userAgent: userAgent.into(),
            acceptLanguage: None,
            platform: None,
            userAgentMetadata: None,
        }
    }
    pub fn userAgent(&self) -> &str { self.userAgent.as_ref() }
    pub fn acceptLanguage(&self) -> Option<&str> { self.acceptLanguage.as_deref() }
    pub fn platform(&self) -> Option<&str> { self.platform.as_deref() }
    pub fn userAgentMetadata(&self) -> Option<&UserAgentMetadata<'a>> { self.userAgentMetadata.as_ref() }
}


pub struct SetUserAgentOverrideParamsBuilder<'a> {
    userAgent: Cow<'a, str>,
    acceptLanguage: Option<Cow<'a, str>>,
    platform: Option<Cow<'a, str>>,
    userAgentMetadata: Option<UserAgentMetadata<'a>>,
}

impl<'a> SetUserAgentOverrideParamsBuilder<'a> {
    /// Browser language to emulate.
    pub fn acceptLanguage(mut self, acceptLanguage: impl Into<Cow<'a, str>>) -> Self { self.acceptLanguage = Some(acceptLanguage.into()); self }
    /// The platform navigator.platform should return.
    pub fn platform(mut self, platform: impl Into<Cow<'a, str>>) -> Self { self.platform = Some(platform.into()); self }
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    pub fn userAgentMetadata(mut self, userAgentMetadata: UserAgentMetadata<'a>) -> Self { self.userAgentMetadata = Some(userAgentMetadata); self }
    pub fn build(self) -> SetUserAgentOverrideParams<'a> {
        SetUserAgentOverrideParams {
            userAgent: self.userAgent,
            acceptLanguage: self.acceptLanguage,
            platform: self.platform,
            userAgentMetadata: self.userAgentMetadata,
        }
    }
}

impl<'a> SetUserAgentOverrideParams<'a> { pub const METHOD: &'static str = "Emulation.setUserAgentOverride"; }

impl<'a> crate::CdpCommand<'a> for SetUserAgentOverrideParams<'a> {
    const METHOD: &'static str = "Emulation.setUserAgentOverride";
    type Response = crate::EmptyReturns;
}

/// Allows overriding the automation flag.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAutomationOverrideParams {
    /// Whether the override should be enabled.
    enabled: bool,
}

impl SetAutomationOverrideParams {
    pub fn builder(enabled: bool) -> SetAutomationOverrideParamsBuilder {
        SetAutomationOverrideParamsBuilder {
            enabled: enabled,
        }
    }
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetAutomationOverrideParamsBuilder {
    enabled: bool,
}

impl SetAutomationOverrideParamsBuilder {
    pub fn build(self) -> SetAutomationOverrideParams {
        SetAutomationOverrideParams {
            enabled: self.enabled,
        }
    }
}

impl SetAutomationOverrideParams { pub const METHOD: &'static str = "Emulation.setAutomationOverride"; }

impl<'a> crate::CdpCommand<'a> for SetAutomationOverrideParams {
    const METHOD: &'static str = "Emulation.setAutomationOverride";
    type Response = crate::EmptyReturns;
}

/// Allows overriding the difference between the small and large viewport sizes, which determine the
/// value of the 'svh' and 'lvh' unit, respectively. Only supported for top-level frames.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSmallViewportHeightDifferenceOverrideParams {
    /// This will cause an element of size 100svh to be 'difference' pixels smaller than an element
    /// of size 100lvh.
    difference: i64,
}

impl SetSmallViewportHeightDifferenceOverrideParams {
    pub fn builder(difference: i64) -> SetSmallViewportHeightDifferenceOverrideParamsBuilder {
        SetSmallViewportHeightDifferenceOverrideParamsBuilder {
            difference: difference,
        }
    }
    pub fn difference(&self) -> i64 { self.difference }
}


pub struct SetSmallViewportHeightDifferenceOverrideParamsBuilder {
    difference: i64,
}

impl SetSmallViewportHeightDifferenceOverrideParamsBuilder {
    pub fn build(self) -> SetSmallViewportHeightDifferenceOverrideParams {
        SetSmallViewportHeightDifferenceOverrideParams {
            difference: self.difference,
        }
    }
}

impl SetSmallViewportHeightDifferenceOverrideParams { pub const METHOD: &'static str = "Emulation.setSmallViewportHeightDifferenceOverride"; }

impl<'a> crate::CdpCommand<'a> for SetSmallViewportHeightDifferenceOverrideParams {
    const METHOD: &'static str = "Emulation.setSmallViewportHeightDifferenceOverride";
    type Response = crate::EmptyReturns;
}

/// Returns device's screen configuration. In headful mode, the physical screens configuration is returned,
/// whereas in headless mode, a virtual headless screen configuration is provided instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetScreenInfosReturns<'a> {
    screenInfos: Vec<ScreenInfo<'a>>,
}

impl<'a> GetScreenInfosReturns<'a> {
    pub fn builder(screenInfos: Vec<ScreenInfo<'a>>) -> GetScreenInfosReturnsBuilder<'a> {
        GetScreenInfosReturnsBuilder {
            screenInfos: screenInfos,
        }
    }
    pub fn screenInfos(&self) -> &[ScreenInfo<'a>] { &self.screenInfos }
}


pub struct GetScreenInfosReturnsBuilder<'a> {
    screenInfos: Vec<ScreenInfo<'a>>,
}

impl<'a> GetScreenInfosReturnsBuilder<'a> {
    pub fn build(self) -> GetScreenInfosReturns<'a> {
        GetScreenInfosReturns {
            screenInfos: self.screenInfos,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetScreenInfosParams {}

impl GetScreenInfosParams { pub const METHOD: &'static str = "Emulation.getScreenInfos"; }

impl<'a> crate::CdpCommand<'a> for GetScreenInfosParams {
    const METHOD: &'static str = "Emulation.getScreenInfos";
    type Response = GetScreenInfosReturns<'a>;
}

/// Add a new screen to the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScreenParams<'a> {
    /// Offset of the left edge of the screen in pixels.
    left: i64,
    /// Offset of the top edge of the screen in pixels.
    top: i64,
    /// The width of the screen in pixels.
    width: u64,
    /// The height of the screen in pixels.
    height: i64,
    /// Specifies the screen's work area. Default is entire screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    workAreaInsets: Option<WorkAreaInsets>,
    /// Specifies the screen's device pixel ratio. Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    devicePixelRatio: Option<f64>,
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<i64>,
    /// Specifies the screen's color depth in bits. Default is 24.
    #[serde(skip_serializing_if = "Option::is_none")]
    colorDepth: Option<i64>,
    /// Specifies the descriptive label for the screen. Default is none.
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Cow<'a, str>>,
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    isInternal: Option<bool>,
}

impl<'a> AddScreenParams<'a> {
    pub fn builder(left: i64, top: i64, width: u64, height: i64) -> AddScreenParamsBuilder<'a> {
        AddScreenParamsBuilder {
            left: left,
            top: top,
            width: width,
            height: height,
            workAreaInsets: None,
            devicePixelRatio: None,
            rotation: None,
            colorDepth: None,
            label: None,
            isInternal: None,
        }
    }
    pub fn left(&self) -> i64 { self.left }
    pub fn top(&self) -> i64 { self.top }
    pub fn width(&self) -> u64 { self.width }
    pub fn height(&self) -> i64 { self.height }
    pub fn workAreaInsets(&self) -> Option<&WorkAreaInsets> { self.workAreaInsets.as_ref() }
    pub fn devicePixelRatio(&self) -> Option<f64> { self.devicePixelRatio }
    pub fn rotation(&self) -> Option<i64> { self.rotation }
    pub fn colorDepth(&self) -> Option<i64> { self.colorDepth }
    pub fn label(&self) -> Option<&str> { self.label.as_deref() }
    pub fn isInternal(&self) -> Option<bool> { self.isInternal }
}


pub struct AddScreenParamsBuilder<'a> {
    left: i64,
    top: i64,
    width: u64,
    height: i64,
    workAreaInsets: Option<WorkAreaInsets>,
    devicePixelRatio: Option<f64>,
    rotation: Option<i64>,
    colorDepth: Option<i64>,
    label: Option<Cow<'a, str>>,
    isInternal: Option<bool>,
}

impl<'a> AddScreenParamsBuilder<'a> {
    /// Specifies the screen's work area. Default is entire screen.
    pub fn workAreaInsets(mut self, workAreaInsets: WorkAreaInsets) -> Self { self.workAreaInsets = Some(workAreaInsets); self }
    /// Specifies the screen's device pixel ratio. Default is 1.
    pub fn devicePixelRatio(mut self, devicePixelRatio: f64) -> Self { self.devicePixelRatio = Some(devicePixelRatio); self }
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0.
    pub fn rotation(mut self, rotation: i64) -> Self { self.rotation = Some(rotation); self }
    /// Specifies the screen's color depth in bits. Default is 24.
    pub fn colorDepth(mut self, colorDepth: i64) -> Self { self.colorDepth = Some(colorDepth); self }
    /// Specifies the descriptive label for the screen. Default is none.
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self { self.label = Some(label.into()); self }
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    pub fn isInternal(mut self, isInternal: bool) -> Self { self.isInternal = Some(isInternal); self }
    pub fn build(self) -> AddScreenParams<'a> {
        AddScreenParams {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            workAreaInsets: self.workAreaInsets,
            devicePixelRatio: self.devicePixelRatio,
            rotation: self.rotation,
            colorDepth: self.colorDepth,
            label: self.label,
            isInternal: self.isInternal,
        }
    }
}

/// Add a new screen to the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScreenReturns<'a> {
    screenInfo: ScreenInfo<'a>,
}

impl<'a> AddScreenReturns<'a> {
    pub fn builder(screenInfo: ScreenInfo<'a>) -> AddScreenReturnsBuilder<'a> {
        AddScreenReturnsBuilder {
            screenInfo: screenInfo,
        }
    }
    pub fn screenInfo(&self) -> &ScreenInfo<'a> { &self.screenInfo }
}


pub struct AddScreenReturnsBuilder<'a> {
    screenInfo: ScreenInfo<'a>,
}

impl<'a> AddScreenReturnsBuilder<'a> {
    pub fn build(self) -> AddScreenReturns<'a> {
        AddScreenReturns {
            screenInfo: self.screenInfo,
        }
    }
}

impl<'a> AddScreenParams<'a> { pub const METHOD: &'static str = "Emulation.addScreen"; }

impl<'a> crate::CdpCommand<'a> for AddScreenParams<'a> {
    const METHOD: &'static str = "Emulation.addScreen";
    type Response = AddScreenReturns<'a>;
}

/// Updates specified screen parameters. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScreenParams<'a> {
    /// Target screen identifier.
    screenId: ScreenId<'a>,
    /// Offset of the left edge of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<i64>,
    /// Offset of the top edge of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<i64>,
    /// The width of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u64>,
    /// The height of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
    /// Specifies the screen's work area.
    #[serde(skip_serializing_if = "Option::is_none")]
    workAreaInsets: Option<WorkAreaInsets>,
    /// Specifies the screen's device pixel ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    devicePixelRatio: Option<f64>,
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270.
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<i64>,
    /// Specifies the screen's color depth in bits.
    #[serde(skip_serializing_if = "Option::is_none")]
    colorDepth: Option<i64>,
    /// Specifies the descriptive label for the screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Cow<'a, str>>,
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    isInternal: Option<bool>,
}

impl<'a> UpdateScreenParams<'a> {
    pub fn builder(screenId: ScreenId<'a>) -> UpdateScreenParamsBuilder<'a> {
        UpdateScreenParamsBuilder {
            screenId: screenId,
            left: None,
            top: None,
            width: None,
            height: None,
            workAreaInsets: None,
            devicePixelRatio: None,
            rotation: None,
            colorDepth: None,
            label: None,
            isInternal: None,
        }
    }
    pub fn screenId(&self) -> &ScreenId<'a> { &self.screenId }
    pub fn left(&self) -> Option<i64> { self.left }
    pub fn top(&self) -> Option<i64> { self.top }
    pub fn width(&self) -> Option<u64> { self.width }
    pub fn height(&self) -> Option<i64> { self.height }
    pub fn workAreaInsets(&self) -> Option<&WorkAreaInsets> { self.workAreaInsets.as_ref() }
    pub fn devicePixelRatio(&self) -> Option<f64> { self.devicePixelRatio }
    pub fn rotation(&self) -> Option<i64> { self.rotation }
    pub fn colorDepth(&self) -> Option<i64> { self.colorDepth }
    pub fn label(&self) -> Option<&str> { self.label.as_deref() }
    pub fn isInternal(&self) -> Option<bool> { self.isInternal }
}


pub struct UpdateScreenParamsBuilder<'a> {
    screenId: ScreenId<'a>,
    left: Option<i64>,
    top: Option<i64>,
    width: Option<u64>,
    height: Option<i64>,
    workAreaInsets: Option<WorkAreaInsets>,
    devicePixelRatio: Option<f64>,
    rotation: Option<i64>,
    colorDepth: Option<i64>,
    label: Option<Cow<'a, str>>,
    isInternal: Option<bool>,
}

impl<'a> UpdateScreenParamsBuilder<'a> {
    /// Offset of the left edge of the screen in pixels.
    pub fn left(mut self, left: i64) -> Self { self.left = Some(left); self }
    /// Offset of the top edge of the screen in pixels.
    pub fn top(mut self, top: i64) -> Self { self.top = Some(top); self }
    /// The width of the screen in pixels.
    pub fn width(mut self, width: u64) -> Self { self.width = Some(width); self }
    /// The height of the screen in pixels.
    pub fn height(mut self, height: i64) -> Self { self.height = Some(height); self }
    /// Specifies the screen's work area.
    pub fn workAreaInsets(mut self, workAreaInsets: WorkAreaInsets) -> Self { self.workAreaInsets = Some(workAreaInsets); self }
    /// Specifies the screen's device pixel ratio.
    pub fn devicePixelRatio(mut self, devicePixelRatio: f64) -> Self { self.devicePixelRatio = Some(devicePixelRatio); self }
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270.
    pub fn rotation(mut self, rotation: i64) -> Self { self.rotation = Some(rotation); self }
    /// Specifies the screen's color depth in bits.
    pub fn colorDepth(mut self, colorDepth: i64) -> Self { self.colorDepth = Some(colorDepth); self }
    /// Specifies the descriptive label for the screen.
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self { self.label = Some(label.into()); self }
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    pub fn isInternal(mut self, isInternal: bool) -> Self { self.isInternal = Some(isInternal); self }
    pub fn build(self) -> UpdateScreenParams<'a> {
        UpdateScreenParams {
            screenId: self.screenId,
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            workAreaInsets: self.workAreaInsets,
            devicePixelRatio: self.devicePixelRatio,
            rotation: self.rotation,
            colorDepth: self.colorDepth,
            label: self.label,
            isInternal: self.isInternal,
        }
    }
}

/// Updates specified screen parameters. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScreenReturns<'a> {
    screenInfo: ScreenInfo<'a>,
}

impl<'a> UpdateScreenReturns<'a> {
    pub fn builder(screenInfo: ScreenInfo<'a>) -> UpdateScreenReturnsBuilder<'a> {
        UpdateScreenReturnsBuilder {
            screenInfo: screenInfo,
        }
    }
    pub fn screenInfo(&self) -> &ScreenInfo<'a> { &self.screenInfo }
}


pub struct UpdateScreenReturnsBuilder<'a> {
    screenInfo: ScreenInfo<'a>,
}

impl<'a> UpdateScreenReturnsBuilder<'a> {
    pub fn build(self) -> UpdateScreenReturns<'a> {
        UpdateScreenReturns {
            screenInfo: self.screenInfo,
        }
    }
}

impl<'a> UpdateScreenParams<'a> { pub const METHOD: &'static str = "Emulation.updateScreen"; }

impl<'a> crate::CdpCommand<'a> for UpdateScreenParams<'a> {
    const METHOD: &'static str = "Emulation.updateScreen";
    type Response = UpdateScreenReturns<'a>;
}

/// Remove screen from the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScreenParams<'a> {
    screenId: ScreenId<'a>,
}

impl<'a> RemoveScreenParams<'a> {
    pub fn builder(screenId: ScreenId<'a>) -> RemoveScreenParamsBuilder<'a> {
        RemoveScreenParamsBuilder {
            screenId: screenId,
        }
    }
    pub fn screenId(&self) -> &ScreenId<'a> { &self.screenId }
}


pub struct RemoveScreenParamsBuilder<'a> {
    screenId: ScreenId<'a>,
}

impl<'a> RemoveScreenParamsBuilder<'a> {
    pub fn build(self) -> RemoveScreenParams<'a> {
        RemoveScreenParams {
            screenId: self.screenId,
        }
    }
}

impl<'a> RemoveScreenParams<'a> { pub const METHOD: &'static str = "Emulation.removeScreen"; }

impl<'a> crate::CdpCommand<'a> for RemoveScreenParams<'a> {
    const METHOD: &'static str = "Emulation.removeScreen";
    type Response = crate::EmptyReturns;
}

/// Set primary screen. Only supported in headless mode.
/// Note that this changes the coordinate system origin to the top-left
/// of the new primary screen, updating the bounds and work areas
/// of all existing screens accordingly.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPrimaryScreenParams<'a> {
    screenId: ScreenId<'a>,
}

impl<'a> SetPrimaryScreenParams<'a> {
    pub fn builder(screenId: ScreenId<'a>) -> SetPrimaryScreenParamsBuilder<'a> {
        SetPrimaryScreenParamsBuilder {
            screenId: screenId,
        }
    }
    pub fn screenId(&self) -> &ScreenId<'a> { &self.screenId }
}


pub struct SetPrimaryScreenParamsBuilder<'a> {
    screenId: ScreenId<'a>,
}

impl<'a> SetPrimaryScreenParamsBuilder<'a> {
    pub fn build(self) -> SetPrimaryScreenParams<'a> {
        SetPrimaryScreenParams {
            screenId: self.screenId,
        }
    }
}

impl<'a> SetPrimaryScreenParams<'a> { pub const METHOD: &'static str = "Emulation.setPrimaryScreen"; }

impl<'a> crate::CdpCommand<'a> for SetPrimaryScreenParams<'a> {
    const METHOD: &'static str = "Emulation.setPrimaryScreen";
    type Response = crate::EmptyReturns;
}
