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
    #[serde(skip_serializing_if = "Option::is_none", rename = "topMax")]
    top_max: Option<i64>,
    /// Overrides safe-area-inset-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<i64>,
    /// Overrides safe-area-max-inset-left.
    #[serde(skip_serializing_if = "Option::is_none", rename = "leftMax")]
    left_max: Option<i64>,
    /// Overrides safe-area-inset-bottom.
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<i64>,
    /// Overrides safe-area-max-inset-bottom.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bottomMax")]
    bottom_max: Option<i64>,
    /// Overrides safe-area-inset-right.
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<i64>,
    /// Overrides safe-area-max-inset-right.
    #[serde(skip_serializing_if = "Option::is_none", rename = "rightMax")]
    right_max: Option<i64>,
}

impl SafeAreaInsets {
    /// Creates a builder for this type.
    pub fn builder() -> SafeAreaInsetsBuilder {
        SafeAreaInsetsBuilder {
            top: None,
            top_max: None,
            left: None,
            left_max: None,
            bottom: None,
            bottom_max: None,
            right: None,
            right_max: None,
        }
    }
    /// Overrides safe-area-inset-top.
    pub fn top(&self) -> Option<i64> { self.top }
    /// Overrides safe-area-max-inset-top.
    pub fn top_max(&self) -> Option<i64> { self.top_max }
    /// Overrides safe-area-inset-left.
    pub fn left(&self) -> Option<i64> { self.left }
    /// Overrides safe-area-max-inset-left.
    pub fn left_max(&self) -> Option<i64> { self.left_max }
    /// Overrides safe-area-inset-bottom.
    pub fn bottom(&self) -> Option<i64> { self.bottom }
    /// Overrides safe-area-max-inset-bottom.
    pub fn bottom_max(&self) -> Option<i64> { self.bottom_max }
    /// Overrides safe-area-inset-right.
    pub fn right(&self) -> Option<i64> { self.right }
    /// Overrides safe-area-max-inset-right.
    pub fn right_max(&self) -> Option<i64> { self.right_max }
}

#[derive(Default)]
pub struct SafeAreaInsetsBuilder {
    top: Option<i64>,
    top_max: Option<i64>,
    left: Option<i64>,
    left_max: Option<i64>,
    bottom: Option<i64>,
    bottom_max: Option<i64>,
    right: Option<i64>,
    right_max: Option<i64>,
}

impl SafeAreaInsetsBuilder {
    /// Overrides safe-area-inset-top.
    pub fn top(mut self, top: i64) -> Self { self.top = Some(top); self }
    /// Overrides safe-area-max-inset-top.
    pub fn top_max(mut self, top_max: i64) -> Self { self.top_max = Some(top_max); self }
    /// Overrides safe-area-inset-left.
    pub fn left(mut self, left: i64) -> Self { self.left = Some(left); self }
    /// Overrides safe-area-max-inset-left.
    pub fn left_max(mut self, left_max: i64) -> Self { self.left_max = Some(left_max); self }
    /// Overrides safe-area-inset-bottom.
    pub fn bottom(mut self, bottom: i64) -> Self { self.bottom = Some(bottom); self }
    /// Overrides safe-area-max-inset-bottom.
    pub fn bottom_max(mut self, bottom_max: i64) -> Self { self.bottom_max = Some(bottom_max); self }
    /// Overrides safe-area-inset-right.
    pub fn right(mut self, right: i64) -> Self { self.right = Some(right); self }
    /// Overrides safe-area-max-inset-right.
    pub fn right_max(mut self, right_max: i64) -> Self { self.right_max = Some(right_max); self }
    pub fn build(self) -> SafeAreaInsets {
        SafeAreaInsets {
            top: self.top,
            top_max: self.top_max,
            left: self.left,
            left_max: self.left_max,
            bottom: self.bottom,
            bottom_max: self.bottom_max,
            right: self.right,
            right_max: self.right_max,
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
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Orientation type.
    /// * `angle`: Orientation angle.
    pub fn builder(type_: impl Into<Cow<'a, str>>, angle: i64) -> ScreenOrientationBuilder<'a> {
        ScreenOrientationBuilder {
            type_: type_.into(),
            angle: angle,
        }
    }
    /// Orientation type.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Orientation angle.
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
    #[serde(rename = "maskLength")]
    mask_length: u64,
}

impl<'a> DisplayFeature<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `orientation`: Orientation of a display feature in relation to screen
    /// * `offset`: The offset from the screen origin in either the x (for vertical orientation) or y (for horizontal orientation) direction.
    /// * `mask_length`: A display feature may mask content such that it is not physically displayed - this length along with the offset describes this area. A display feature that only splits content will have a 0 mask_length.
    pub fn builder(orientation: impl Into<Cow<'a, str>>, offset: i32, mask_length: u64) -> DisplayFeatureBuilder<'a> {
        DisplayFeatureBuilder {
            orientation: orientation.into(),
            offset: offset,
            mask_length: mask_length,
        }
    }
    /// Orientation of a display feature in relation to screen
    pub fn orientation(&self) -> &str { self.orientation.as_ref() }
    /// The offset from the screen origin in either the x (for vertical
    /// orientation) or y (for horizontal orientation) direction.
    pub fn offset(&self) -> i32 { self.offset }
    /// A display feature may mask content such that it is not physically
    /// displayed - this length along with the offset describes this area.
    /// A display feature that only splits content will have a 0 mask_length.
    pub fn mask_length(&self) -> u64 { self.mask_length }
}


pub struct DisplayFeatureBuilder<'a> {
    orientation: Cow<'a, str>,
    offset: i32,
    mask_length: u64,
}

impl<'a> DisplayFeatureBuilder<'a> {
    pub fn build(self) -> DisplayFeature<'a> {
        DisplayFeature {
            orientation: self.orientation,
            offset: self.offset,
            mask_length: self.mask_length,
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
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Current posture of the device
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> DevicePostureBuilder<'a> {
        DevicePostureBuilder {
            type_: type_.into(),
        }
    }
    /// Current posture of the device
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
    /// Creates a builder for this type with the required parameters:
    /// * `name`: 
    /// * `value`: 
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

/// Used to specify User Agent Client Hints to emulate. See <https://wicg.github.io/ua-client-hints>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentBrandVersion<'a> {
    brand: Cow<'a, str>,
    version: Cow<'a, str>,
}

impl<'a> UserAgentBrandVersion<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `brand`: 
    /// * `version`: 
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

/// Used to specify User Agent Client Hints to emulate. See <https://wicg.github.io/ua-client-hints>
/// Missing optional values will be filled in by the target with what it would normally use.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentMetadata<'a> {
    /// Brands appearing in Sec-CH-UA.
    #[serde(skip_serializing_if = "Option::is_none")]
    brands: Option<Vec<UserAgentBrandVersion<'a>>>,
    /// Brands appearing in Sec-CH-UA-Full-Version-List.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fullVersionList")]
    full_version_list: Option<Vec<UserAgentBrandVersion<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fullVersion")]
    full_version: Option<Cow<'a, str>>,
    platform: Cow<'a, str>,
    #[serde(rename = "platformVersion")]
    platform_version: Cow<'a, str>,
    architecture: Cow<'a, str>,
    model: Cow<'a, str>,
    mobile: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitness: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wow64: Option<bool>,
    /// Used to specify User Agent form-factor values.
    /// See <https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors>
    #[serde(skip_serializing_if = "Option::is_none", rename = "formFactors")]
    form_factors: Option<Vec<Cow<'a, str>>>,
}

impl<'a> UserAgentMetadata<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `platform`: 
    /// * `platform_version`: 
    /// * `architecture`: 
    /// * `model`: 
    /// * `mobile`: 
    pub fn builder(platform: impl Into<Cow<'a, str>>, platform_version: impl Into<Cow<'a, str>>, architecture: impl Into<Cow<'a, str>>, model: impl Into<Cow<'a, str>>, mobile: bool) -> UserAgentMetadataBuilder<'a> {
        UserAgentMetadataBuilder {
            brands: None,
            full_version_list: None,
            full_version: None,
            platform: platform.into(),
            platform_version: platform_version.into(),
            architecture: architecture.into(),
            model: model.into(),
            mobile: mobile,
            bitness: None,
            wow64: None,
            form_factors: None,
        }
    }
    /// Brands appearing in Sec-CH-UA.
    pub fn brands(&self) -> Option<&[UserAgentBrandVersion<'a>]> { self.brands.as_deref() }
    /// Brands appearing in Sec-CH-UA-Full-Version-List.
    pub fn full_version_list(&self) -> Option<&[UserAgentBrandVersion<'a>]> { self.full_version_list.as_deref() }
    pub fn full_version(&self) -> Option<&str> { self.full_version.as_deref() }
    pub fn platform(&self) -> &str { self.platform.as_ref() }
    pub fn platform_version(&self) -> &str { self.platform_version.as_ref() }
    pub fn architecture(&self) -> &str { self.architecture.as_ref() }
    pub fn model(&self) -> &str { self.model.as_ref() }
    pub fn mobile(&self) -> bool { self.mobile }
    pub fn bitness(&self) -> Option<&str> { self.bitness.as_deref() }
    pub fn wow64(&self) -> Option<bool> { self.wow64 }
    /// Used to specify User Agent form-factor values.
    /// See <https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors>
    pub fn form_factors(&self) -> Option<&[Cow<'a, str>]> { self.form_factors.as_deref() }
}


pub struct UserAgentMetadataBuilder<'a> {
    brands: Option<Vec<UserAgentBrandVersion<'a>>>,
    full_version_list: Option<Vec<UserAgentBrandVersion<'a>>>,
    full_version: Option<Cow<'a, str>>,
    platform: Cow<'a, str>,
    platform_version: Cow<'a, str>,
    architecture: Cow<'a, str>,
    model: Cow<'a, str>,
    mobile: bool,
    bitness: Option<Cow<'a, str>>,
    wow64: Option<bool>,
    form_factors: Option<Vec<Cow<'a, str>>>,
}

impl<'a> UserAgentMetadataBuilder<'a> {
    /// Brands appearing in Sec-CH-UA.
    pub fn brands(mut self, brands: Vec<UserAgentBrandVersion<'a>>) -> Self { self.brands = Some(brands); self }
    /// Brands appearing in Sec-CH-UA-Full-Version-List.
    pub fn full_version_list(mut self, full_version_list: Vec<UserAgentBrandVersion<'a>>) -> Self { self.full_version_list = Some(full_version_list); self }
    pub fn full_version(mut self, full_version: impl Into<Cow<'a, str>>) -> Self { self.full_version = Some(full_version.into()); self }
    pub fn bitness(mut self, bitness: impl Into<Cow<'a, str>>) -> Self { self.bitness = Some(bitness.into()); self }
    pub fn wow64(mut self, wow64: bool) -> Self { self.wow64 = Some(wow64); self }
    /// Used to specify User Agent form-factor values.
    /// See <https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors>
    pub fn form_factors(mut self, form_factors: Vec<Cow<'a, str>>) -> Self { self.form_factors = Some(form_factors); self }
    pub fn build(self) -> UserAgentMetadata<'a> {
        UserAgentMetadata {
            brands: self.brands,
            full_version_list: self.full_version_list,
            full_version: self.full_version,
            platform: self.platform,
            platform_version: self.platform_version,
            architecture: self.architecture,
            model: self.model,
            mobile: self.mobile,
            bitness: self.bitness,
            wow64: self.wow64,
            form_factors: self.form_factors,
        }
    }
}

/// Used to specify sensor types to emulate.
/// See <https://w3c.github.io/sensors/#automation> for more information.

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
    #[serde(skip_serializing_if = "Option::is_none", rename = "minimumFrequency")]
    minimum_frequency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maximumFrequency")]
    maximum_frequency: Option<f64>,
}

impl SensorMetadata {
    /// Creates a builder for this type.
    pub fn builder() -> SensorMetadataBuilder {
        SensorMetadataBuilder {
            available: None,
            minimum_frequency: None,
            maximum_frequency: None,
        }
    }
    pub fn available(&self) -> Option<bool> { self.available }
    pub fn minimum_frequency(&self) -> Option<f64> { self.minimum_frequency }
    pub fn maximum_frequency(&self) -> Option<f64> { self.maximum_frequency }
}

#[derive(Default)]
pub struct SensorMetadataBuilder {
    available: Option<bool>,
    minimum_frequency: Option<f64>,
    maximum_frequency: Option<f64>,
}

impl SensorMetadataBuilder {
    pub fn available(mut self, available: bool) -> Self { self.available = Some(available); self }
    pub fn minimum_frequency(mut self, minimum_frequency: f64) -> Self { self.minimum_frequency = Some(minimum_frequency); self }
    pub fn maximum_frequency(mut self, maximum_frequency: f64) -> Self { self.maximum_frequency = Some(maximum_frequency); self }
    pub fn build(self) -> SensorMetadata {
        SensorMetadata {
            available: self.available,
            minimum_frequency: self.minimum_frequency,
            maximum_frequency: self.maximum_frequency,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingSingle {
    value: f64,
}

impl SensorReadingSingle {
    /// Creates a builder for this type with the required parameters:
    /// * `value`: 
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
    /// Creates a builder for this type with the required parameters:
    /// * `x`: 
    /// * `y`: 
    /// * `z`: 
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
    /// Creates a builder for this type with the required parameters:
    /// * `x`: 
    /// * `y`: 
    /// * `z`: 
    /// * `w`: 
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
    /// Creates a builder for this type.
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
    /// Creates a builder for this type.
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
    /// Creates a builder for this type.
    pub fn builder() -> WorkAreaInsetsBuilder {
        WorkAreaInsetsBuilder {
            top: None,
            left: None,
            bottom: None,
            right: None,
        }
    }
    /// Work area top inset in pixels. Default is 0;
    pub fn top(&self) -> Option<i64> { self.top }
    /// Work area left inset in pixels. Default is 0;
    pub fn left(&self) -> Option<i64> { self.left }
    /// Work area bottom inset in pixels. Default is 0;
    pub fn bottom(&self) -> Option<i64> { self.bottom }
    /// Work area right inset in pixels. Default is 0;
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
/// see <https://w3c.github.io/window-management/#screendetailed>.

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
    #[serde(rename = "availLeft")]
    avail_left: i64,
    /// Offset of the top edge of the available screen area.
    #[serde(rename = "availTop")]
    avail_top: i64,
    /// Width of the available screen area.
    #[serde(rename = "availWidth")]
    avail_width: u64,
    /// Height of the available screen area.
    #[serde(rename = "availHeight")]
    avail_height: i64,
    /// Specifies the screen's device pixel ratio.
    #[serde(rename = "devicePixelRatio")]
    device_pixel_ratio: f64,
    /// Specifies the screen's orientation.
    orientation: ScreenOrientation<'a>,
    /// Specifies the screen's color depth in bits.
    #[serde(rename = "colorDepth")]
    color_depth: i64,
    /// Indicates whether the device has multiple screens.
    #[serde(rename = "isExtended")]
    is_extended: bool,
    /// Indicates whether the screen is internal to the device or external, attached to the device.
    #[serde(rename = "isInternal")]
    is_internal: bool,
    /// Indicates whether the screen is set as the the operating system primary screen.
    #[serde(rename = "isPrimary")]
    is_primary: bool,
    /// Specifies the descriptive label for the screen.
    label: Cow<'a, str>,
    /// Specifies the unique identifier of the screen.
    id: ScreenId<'a>,
}

impl<'a> ScreenInfo<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `left`: Offset of the left edge of the screen.
    /// * `top`: Offset of the top edge of the screen.
    /// * `width`: Width of the screen.
    /// * `height`: Height of the screen.
    /// * `avail_left`: Offset of the left edge of the available screen area.
    /// * `avail_top`: Offset of the top edge of the available screen area.
    /// * `avail_width`: Width of the available screen area.
    /// * `avail_height`: Height of the available screen area.
    /// * `device_pixel_ratio`: Specifies the screen's device pixel ratio.
    /// * `orientation`: Specifies the screen's orientation.
    /// * `color_depth`: Specifies the screen's color depth in bits.
    /// * `is_extended`: Indicates whether the device has multiple screens.
    /// * `is_internal`: Indicates whether the screen is internal to the device or external, attached to the device.
    /// * `is_primary`: Indicates whether the screen is set as the the operating system primary screen.
    /// * `label`: Specifies the descriptive label for the screen.
    /// * `id`: Specifies the unique identifier of the screen.
    pub fn builder(left: i64, top: i64, width: u64, height: i64, avail_left: i64, avail_top: i64, avail_width: u64, avail_height: i64, device_pixel_ratio: f64, orientation: ScreenOrientation<'a>, color_depth: i64, is_extended: bool, is_internal: bool, is_primary: bool, label: impl Into<Cow<'a, str>>, id: impl Into<ScreenId<'a>>) -> ScreenInfoBuilder<'a> {
        ScreenInfoBuilder {
            left: left,
            top: top,
            width: width,
            height: height,
            avail_left: avail_left,
            avail_top: avail_top,
            avail_width: avail_width,
            avail_height: avail_height,
            device_pixel_ratio: device_pixel_ratio,
            orientation: orientation,
            color_depth: color_depth,
            is_extended: is_extended,
            is_internal: is_internal,
            is_primary: is_primary,
            label: label.into(),
            id: id.into(),
        }
    }
    /// Offset of the left edge of the screen.
    pub fn left(&self) -> i64 { self.left }
    /// Offset of the top edge of the screen.
    pub fn top(&self) -> i64 { self.top }
    /// Width of the screen.
    pub fn width(&self) -> u64 { self.width }
    /// Height of the screen.
    pub fn height(&self) -> i64 { self.height }
    /// Offset of the left edge of the available screen area.
    pub fn avail_left(&self) -> i64 { self.avail_left }
    /// Offset of the top edge of the available screen area.
    pub fn avail_top(&self) -> i64 { self.avail_top }
    /// Width of the available screen area.
    pub fn avail_width(&self) -> u64 { self.avail_width }
    /// Height of the available screen area.
    pub fn avail_height(&self) -> i64 { self.avail_height }
    /// Specifies the screen's device pixel ratio.
    pub fn device_pixel_ratio(&self) -> f64 { self.device_pixel_ratio }
    /// Specifies the screen's orientation.
    pub fn orientation(&self) -> &ScreenOrientation<'a> { &self.orientation }
    /// Specifies the screen's color depth in bits.
    pub fn color_depth(&self) -> i64 { self.color_depth }
    /// Indicates whether the device has multiple screens.
    pub fn is_extended(&self) -> bool { self.is_extended }
    /// Indicates whether the screen is internal to the device or external, attached to the device.
    pub fn is_internal(&self) -> bool { self.is_internal }
    /// Indicates whether the screen is set as the the operating system primary screen.
    pub fn is_primary(&self) -> bool { self.is_primary }
    /// Specifies the descriptive label for the screen.
    pub fn label(&self) -> &str { self.label.as_ref() }
    /// Specifies the unique identifier of the screen.
    pub fn id(&self) -> &ScreenId<'a> { &self.id }
}


pub struct ScreenInfoBuilder<'a> {
    left: i64,
    top: i64,
    width: u64,
    height: i64,
    avail_left: i64,
    avail_top: i64,
    avail_width: u64,
    avail_height: i64,
    device_pixel_ratio: f64,
    orientation: ScreenOrientation<'a>,
    color_depth: i64,
    is_extended: bool,
    is_internal: bool,
    is_primary: bool,
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
            avail_left: self.avail_left,
            avail_top: self.avail_top,
            avail_width: self.avail_width,
            avail_height: self.avail_height,
            device_pixel_ratio: self.device_pixel_ratio,
            orientation: self.orientation,
            color_depth: self.color_depth,
            is_extended: self.is_extended,
            is_internal: self.is_internal,
            is_primary: self.is_primary,
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
    /// Creates a builder for this type with the required parameters:
    /// * `result`: True if emulation is supported.
    pub fn builder(result: bool) -> CanEmulateReturnsBuilder {
        CanEmulateReturnsBuilder {
            result: result,
        }
    }
    /// True if emulation is supported.
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
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: Whether to enable to disable focus emulation.
    pub fn builder(enabled: bool) -> SetFocusEmulationEnabledParamsBuilder {
        SetFocusEmulationEnabledParamsBuilder {
            enabled: enabled,
        }
    }
    /// Whether to enable to disable focus emulation.
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
    /// Creates a builder for this type.
    pub fn builder() -> SetAutoDarkModeOverrideParamsBuilder {
        SetAutoDarkModeOverrideParamsBuilder {
            enabled: None,
        }
    }
    /// Whether to enable or disable automatic dark mode.
    /// If not specified, any existing override will be cleared.
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
    /// Creates a builder for this type with the required parameters:
    /// * `rate`: Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc).
    pub fn builder(rate: f64) -> SetCPUThrottlingRateParamsBuilder {
        SetCPUThrottlingRateParamsBuilder {
            rate: rate,
        }
    }
    /// Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc).
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
    /// Creates a builder for this type.
    pub fn builder() -> SetDefaultBackgroundColorOverrideParamsBuilder {
        SetDefaultBackgroundColorOverrideParamsBuilder {
            color: None,
        }
    }
    /// RGBA of the default background color. If not specified, any existing override will be
    /// cleared.
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
    /// Creates a builder for this type with the required parameters:
    /// * `insets`: 
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
    #[serde(rename = "deviceScaleFactor")]
    device_scale_factor: f64,
    /// Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    /// autosizing and more.
    mobile: bool,
    /// Scale to apply to resulting view image.
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f64>,
    /// Overriding screen width value in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenWidth")]
    screen_width: Option<u64>,
    /// Overriding screen height value in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenHeight")]
    screen_height: Option<i64>,
    /// Overriding view X position on screen in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionX")]
    position_x: Option<i64>,
    /// Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionY")]
    position_y: Option<i64>,
    /// Do not set visible view size, rely upon explicit setVisibleSize call.
    #[serde(skip_serializing_if = "Option::is_none", rename = "dontSetVisibleSize")]
    dont_set_visible_size: Option<bool>,
    /// Screen orientation override.
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenOrientation")]
    screen_orientation: Option<ScreenOrientation<'a>>,
    /// If set, the visible area of the page will be overridden to this viewport. This viewport
    /// change is not observed by the page, e.g. viewport-relative elements do not change positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    viewport: Option<crate::page::Viewport>,
    /// If set, the display feature of a multi-segment screen. If not set, multi-segment support
    /// is turned-off.
    /// Deprecated, use Emulation.setDisplayFeaturesOverride.
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayFeature")]
    display_feature: Option<DisplayFeature<'a>>,
    /// If set, the posture of a foldable device. If not set the posture is set
    /// to continuous.
    /// Deprecated, use Emulation.setDevicePostureOverride.
    #[serde(skip_serializing_if = "Option::is_none", rename = "devicePosture")]
    device_posture: Option<DevicePosture<'a>>,
    /// Scrollbar type. Default: 'default'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scrollbarType")]
    scrollbar_type: Option<Cow<'a, str>>,
    /// If set to true, enables screen orientation lock emulation, which
    /// intercepts screen.orientation.lock() calls from the page and reports
    /// orientation changes via screenOrientationLockChanged events. This is
    /// useful for emulating mobile device orientation lock behavior in
    /// responsive design mode.
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenOrientationLockEmulation")]
    screen_orientation_lock_emulation: Option<bool>,
}

impl<'a> SetDeviceMetricsOverrideParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `width`: Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    /// * `height`: Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    /// * `device_scale_factor`: Overriding device scale factor value. 0 disables the override.
    /// * `mobile`: Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text autosizing and more.
    pub fn builder(width: u64, height: i64, device_scale_factor: f64, mobile: bool) -> SetDeviceMetricsOverrideParamsBuilder<'a> {
        SetDeviceMetricsOverrideParamsBuilder {
            width: width,
            height: height,
            device_scale_factor: device_scale_factor,
            mobile: mobile,
            scale: None,
            screen_width: None,
            screen_height: None,
            position_x: None,
            position_y: None,
            dont_set_visible_size: None,
            screen_orientation: None,
            viewport: None,
            display_feature: None,
            device_posture: None,
            scrollbar_type: None,
            screen_orientation_lock_emulation: None,
        }
    }
    /// Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub fn width(&self) -> u64 { self.width }
    /// Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub fn height(&self) -> i64 { self.height }
    /// Overriding device scale factor value. 0 disables the override.
    pub fn device_scale_factor(&self) -> f64 { self.device_scale_factor }
    /// Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    /// autosizing and more.
    pub fn mobile(&self) -> bool { self.mobile }
    /// Scale to apply to resulting view image.
    pub fn scale(&self) -> Option<f64> { self.scale }
    /// Overriding screen width value in pixels (minimum 0, maximum 10000000).
    pub fn screen_width(&self) -> Option<u64> { self.screen_width }
    /// Overriding screen height value in pixels (minimum 0, maximum 10000000).
    pub fn screen_height(&self) -> Option<i64> { self.screen_height }
    /// Overriding view X position on screen in pixels (minimum 0, maximum 10000000).
    pub fn position_x(&self) -> Option<i64> { self.position_x }
    /// Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).
    pub fn position_y(&self) -> Option<i64> { self.position_y }
    /// Do not set visible view size, rely upon explicit setVisibleSize call.
    pub fn dont_set_visible_size(&self) -> Option<bool> { self.dont_set_visible_size }
    /// Screen orientation override.
    pub fn screen_orientation(&self) -> Option<&ScreenOrientation<'a>> { self.screen_orientation.as_ref() }
    /// If set, the visible area of the page will be overridden to this viewport. This viewport
    /// change is not observed by the page, e.g. viewport-relative elements do not change positions.
    pub fn viewport(&self) -> Option<&crate::page::Viewport> { self.viewport.as_ref() }
    /// If set, the display feature of a multi-segment screen. If not set, multi-segment support
    /// is turned-off.
    /// Deprecated, use Emulation.setDisplayFeaturesOverride.
    pub fn display_feature(&self) -> Option<&DisplayFeature<'a>> { self.display_feature.as_ref() }
    /// If set, the posture of a foldable device. If not set the posture is set
    /// to continuous.
    /// Deprecated, use Emulation.setDevicePostureOverride.
    pub fn device_posture(&self) -> Option<&DevicePosture<'a>> { self.device_posture.as_ref() }
    /// Scrollbar type. Default: 'default'.
    pub fn scrollbar_type(&self) -> Option<&str> { self.scrollbar_type.as_deref() }
    /// If set to true, enables screen orientation lock emulation, which
    /// intercepts screen.orientation.lock() calls from the page and reports
    /// orientation changes via screenOrientationLockChanged events. This is
    /// useful for emulating mobile device orientation lock behavior in
    /// responsive design mode.
    pub fn screen_orientation_lock_emulation(&self) -> Option<bool> { self.screen_orientation_lock_emulation }
}


pub struct SetDeviceMetricsOverrideParamsBuilder<'a> {
    width: u64,
    height: i64,
    device_scale_factor: f64,
    mobile: bool,
    scale: Option<f64>,
    screen_width: Option<u64>,
    screen_height: Option<i64>,
    position_x: Option<i64>,
    position_y: Option<i64>,
    dont_set_visible_size: Option<bool>,
    screen_orientation: Option<ScreenOrientation<'a>>,
    viewport: Option<crate::page::Viewport>,
    display_feature: Option<DisplayFeature<'a>>,
    device_posture: Option<DevicePosture<'a>>,
    scrollbar_type: Option<Cow<'a, str>>,
    screen_orientation_lock_emulation: Option<bool>,
}

impl<'a> SetDeviceMetricsOverrideParamsBuilder<'a> {
    /// Scale to apply to resulting view image.
    pub fn scale(mut self, scale: f64) -> Self { self.scale = Some(scale); self }
    /// Overriding screen width value in pixels (minimum 0, maximum 10000000).
    pub fn screen_width(mut self, screen_width: u64) -> Self { self.screen_width = Some(screen_width); self }
    /// Overriding screen height value in pixels (minimum 0, maximum 10000000).
    pub fn screen_height(mut self, screen_height: i64) -> Self { self.screen_height = Some(screen_height); self }
    /// Overriding view X position on screen in pixels (minimum 0, maximum 10000000).
    pub fn position_x(mut self, position_x: i64) -> Self { self.position_x = Some(position_x); self }
    /// Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).
    pub fn position_y(mut self, position_y: i64) -> Self { self.position_y = Some(position_y); self }
    /// Do not set visible view size, rely upon explicit setVisibleSize call.
    pub fn dont_set_visible_size(mut self, dont_set_visible_size: bool) -> Self { self.dont_set_visible_size = Some(dont_set_visible_size); self }
    /// Screen orientation override.
    pub fn screen_orientation(mut self, screen_orientation: ScreenOrientation<'a>) -> Self { self.screen_orientation = Some(screen_orientation); self }
    /// If set, the visible area of the page will be overridden to this viewport. This viewport
    /// change is not observed by the page, e.g. viewport-relative elements do not change positions.
    pub fn viewport(mut self, viewport: crate::page::Viewport) -> Self { self.viewport = Some(viewport); self }
    /// If set, the display feature of a multi-segment screen. If not set, multi-segment support
    /// is turned-off.
    /// Deprecated, use Emulation.setDisplayFeaturesOverride.
    pub fn display_feature(mut self, display_feature: DisplayFeature<'a>) -> Self { self.display_feature = Some(display_feature); self }
    /// If set, the posture of a foldable device. If not set the posture is set
    /// to continuous.
    /// Deprecated, use Emulation.setDevicePostureOverride.
    pub fn device_posture(mut self, device_posture: DevicePosture<'a>) -> Self { self.device_posture = Some(device_posture); self }
    /// Scrollbar type. Default: 'default'.
    pub fn scrollbar_type(mut self, scrollbar_type: impl Into<Cow<'a, str>>) -> Self { self.scrollbar_type = Some(scrollbar_type.into()); self }
    /// If set to true, enables screen orientation lock emulation, which
    /// intercepts screen.orientation.lock() calls from the page and reports
    /// orientation changes via screenOrientationLockChanged events. This is
    /// useful for emulating mobile device orientation lock behavior in
    /// responsive design mode.
    pub fn screen_orientation_lock_emulation(mut self, screen_orientation_lock_emulation: bool) -> Self { self.screen_orientation_lock_emulation = Some(screen_orientation_lock_emulation); self }
    pub fn build(self) -> SetDeviceMetricsOverrideParams<'a> {
        SetDeviceMetricsOverrideParams {
            width: self.width,
            height: self.height,
            device_scale_factor: self.device_scale_factor,
            mobile: self.mobile,
            scale: self.scale,
            screen_width: self.screen_width,
            screen_height: self.screen_height,
            position_x: self.position_x,
            position_y: self.position_y,
            dont_set_visible_size: self.dont_set_visible_size,
            screen_orientation: self.screen_orientation,
            viewport: self.viewport,
            display_feature: self.display_feature,
            device_posture: self.device_posture,
            scrollbar_type: self.scrollbar_type,
            screen_orientation_lock_emulation: self.screen_orientation_lock_emulation,
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
    /// Creates a builder for this type with the required parameters:
    /// * `posture`: 
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
    /// Creates a builder for this type with the required parameters:
    /// * `features`: 
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
    /// Creates a builder for this type with the required parameters:
    /// * `hidden`: Whether scrollbars should be always hidden.
    pub fn builder(hidden: bool) -> SetScrollbarsHiddenParamsBuilder {
        SetScrollbarsHiddenParamsBuilder {
            hidden: hidden,
        }
    }
    /// Whether scrollbars should be always hidden.
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
    /// Creates a builder for this type with the required parameters:
    /// * `disabled`: Whether document.coookie API should be disabled.
    pub fn builder(disabled: bool) -> SetDocumentCookieDisabledParamsBuilder {
        SetDocumentCookieDisabledParamsBuilder {
            disabled: disabled,
        }
    }
    /// Whether document.coookie API should be disabled.
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
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: Whether touch emulation based on mouse input should be enabled.
    pub fn builder(enabled: bool) -> SetEmitTouchEventsForMouseParamsBuilder<'a> {
        SetEmitTouchEventsForMouseParamsBuilder {
            enabled: enabled,
            configuration: None,
        }
    }
    /// Whether touch emulation based on mouse input should be enabled.
    pub fn enabled(&self) -> bool { self.enabled }
    /// Touch/gesture events configuration. Default: current platform.
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
    /// Creates a builder for this type.
    pub fn builder() -> SetEmulatedMediaParamsBuilder<'a> {
        SetEmulatedMediaParamsBuilder {
            media: None,
            features: None,
        }
    }
    /// Media type to emulate. Empty string disables the override.
    pub fn media(&self) -> Option<&str> { self.media.as_deref() }
    /// Media features to emulate.
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
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Vision deficiency to emulate. Order: best-effort emulations come first, followed by any physiologically accurate emulations for medically recognized color vision deficiencies.
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> SetEmulatedVisionDeficiencyParamsBuilder<'a> {
        SetEmulatedVisionDeficiencyParamsBuilder {
            type_: type_.into(),
        }
    }
    /// Vision deficiency to emulate. Order: best-effort emulations come first, followed by any
    /// physiologically accurate emulations for medically recognized color vision deficiencies.
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
    /// Creates a builder for this type.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "altitudeAccuracy")]
    altitude_accuracy: Option<f64>,
    /// Mock heading
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<f64>,
    /// Mock speed
    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<f64>,
}

impl SetGeolocationOverrideParams {
    /// Creates a builder for this type.
    pub fn builder() -> SetGeolocationOverrideParamsBuilder {
        SetGeolocationOverrideParamsBuilder {
            latitude: None,
            longitude: None,
            accuracy: None,
            altitude: None,
            altitude_accuracy: None,
            heading: None,
            speed: None,
        }
    }
    /// Mock latitude
    pub fn latitude(&self) -> Option<f64> { self.latitude }
    /// Mock longitude
    pub fn longitude(&self) -> Option<f64> { self.longitude }
    /// Mock accuracy
    pub fn accuracy(&self) -> Option<f64> { self.accuracy }
    /// Mock altitude
    pub fn altitude(&self) -> Option<f64> { self.altitude }
    /// Mock altitudeAccuracy
    pub fn altitude_accuracy(&self) -> Option<f64> { self.altitude_accuracy }
    /// Mock heading
    pub fn heading(&self) -> Option<f64> { self.heading }
    /// Mock speed
    pub fn speed(&self) -> Option<f64> { self.speed }
}

#[derive(Default)]
pub struct SetGeolocationOverrideParamsBuilder {
    latitude: Option<f64>,
    longitude: Option<f64>,
    accuracy: Option<f64>,
    altitude: Option<f64>,
    altitude_accuracy: Option<f64>,
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
    pub fn altitude_accuracy(mut self, altitude_accuracy: f64) -> Self { self.altitude_accuracy = Some(altitude_accuracy); self }
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
            altitude_accuracy: self.altitude_accuracy,
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
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: 
    pub fn builder(type_: impl Into<SensorType>) -> GetOverriddenSensorInformationParamsBuilder {
        GetOverriddenSensorInformationParamsBuilder {
            type_: type_.into(),
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
    #[serde(rename = "requestedSamplingFrequency")]
    requested_sampling_frequency: f64,
}

impl GetOverriddenSensorInformationReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `requested_sampling_frequency`: 
    pub fn builder(requested_sampling_frequency: f64) -> GetOverriddenSensorInformationReturnsBuilder {
        GetOverriddenSensorInformationReturnsBuilder {
            requested_sampling_frequency: requested_sampling_frequency,
        }
    }
    pub fn requested_sampling_frequency(&self) -> f64 { self.requested_sampling_frequency }
}


pub struct GetOverriddenSensorInformationReturnsBuilder {
    requested_sampling_frequency: f64,
}

impl GetOverriddenSensorInformationReturnsBuilder {
    pub fn build(self) -> GetOverriddenSensorInformationReturns {
        GetOverriddenSensorInformationReturns {
            requested_sampling_frequency: self.requested_sampling_frequency,
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
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: 
    /// * `type_`: 
    pub fn builder(enabled: bool, type_: impl Into<SensorType>) -> SetSensorOverrideEnabledParamsBuilder {
        SetSensorOverrideEnabledParamsBuilder {
            enabled: enabled,
            type_: type_.into(),
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
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: 
    /// * `reading`: 
    pub fn builder(type_: impl Into<SensorType>, reading: SensorReading) -> SetSensorOverrideReadingsParamsBuilder {
        SetSensorOverrideReadingsParamsBuilder {
            type_: type_.into(),
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
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: 
    /// * `source`: 
    pub fn builder(enabled: bool, source: impl Into<PressureSource>) -> SetPressureSourceOverrideEnabledParamsBuilder {
        SetPressureSourceOverrideEnabledParamsBuilder {
            enabled: enabled,
            source: source.into(),
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
    /// Creates a builder for this type with the required parameters:
    /// * `source`: 
    /// * `state`: 
    pub fn builder(source: impl Into<PressureSource>, state: impl Into<PressureState>) -> SetPressureStateOverrideParamsBuilder {
        SetPressureStateOverrideParamsBuilder {
            source: source.into(),
            state: state.into(),
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "ownContributionEstimate")]
    own_contribution_estimate: Option<f64>,
}

impl SetPressureDataOverrideParams {
    /// Creates a builder for this type with the required parameters:
    /// * `source`: 
    /// * `state`: 
    pub fn builder(source: impl Into<PressureSource>, state: impl Into<PressureState>) -> SetPressureDataOverrideParamsBuilder {
        SetPressureDataOverrideParamsBuilder {
            source: source.into(),
            state: state.into(),
            own_contribution_estimate: None,
        }
    }
    pub fn source(&self) -> &PressureSource { &self.source }
    pub fn state(&self) -> &PressureState { &self.state }
    pub fn own_contribution_estimate(&self) -> Option<f64> { self.own_contribution_estimate }
}


pub struct SetPressureDataOverrideParamsBuilder {
    source: PressureSource,
    state: PressureState,
    own_contribution_estimate: Option<f64>,
}

impl SetPressureDataOverrideParamsBuilder {
    pub fn own_contribution_estimate(mut self, own_contribution_estimate: f64) -> Self { self.own_contribution_estimate = Some(own_contribution_estimate); self }
    pub fn build(self) -> SetPressureDataOverrideParams {
        SetPressureDataOverrideParams {
            source: self.source,
            state: self.state,
            own_contribution_estimate: self.own_contribution_estimate,
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
    #[serde(rename = "isUserActive")]
    is_user_active: bool,
    /// Mock isScreenUnlocked
    #[serde(rename = "isScreenUnlocked")]
    is_screen_unlocked: bool,
}

impl SetIdleOverrideParams {
    /// Creates a builder for this type with the required parameters:
    /// * `is_user_active`: Mock isUserActive
    /// * `is_screen_unlocked`: Mock isScreenUnlocked
    pub fn builder(is_user_active: bool, is_screen_unlocked: bool) -> SetIdleOverrideParamsBuilder {
        SetIdleOverrideParamsBuilder {
            is_user_active: is_user_active,
            is_screen_unlocked: is_screen_unlocked,
        }
    }
    /// Mock isUserActive
    pub fn is_user_active(&self) -> bool { self.is_user_active }
    /// Mock isScreenUnlocked
    pub fn is_screen_unlocked(&self) -> bool { self.is_screen_unlocked }
}


pub struct SetIdleOverrideParamsBuilder {
    is_user_active: bool,
    is_screen_unlocked: bool,
}

impl SetIdleOverrideParamsBuilder {
    pub fn build(self) -> SetIdleOverrideParams {
        SetIdleOverrideParams {
            is_user_active: self.is_user_active,
            is_screen_unlocked: self.is_screen_unlocked,
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
    /// Creates a builder for this type with the required parameters:
    /// * `platform`: The platform navigator.platform should return.
    pub fn builder(platform: impl Into<Cow<'a, str>>) -> SetNavigatorOverridesParamsBuilder<'a> {
        SetNavigatorOverridesParamsBuilder {
            platform: platform.into(),
        }
    }
    /// The platform navigator.platform should return.
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
    #[serde(rename = "pageScaleFactor")]
    page_scale_factor: f64,
}

impl SetPageScaleFactorParams {
    /// Creates a builder for this type with the required parameters:
    /// * `page_scale_factor`: Page scale factor.
    pub fn builder(page_scale_factor: f64) -> SetPageScaleFactorParamsBuilder {
        SetPageScaleFactorParamsBuilder {
            page_scale_factor: page_scale_factor,
        }
    }
    /// Page scale factor.
    pub fn page_scale_factor(&self) -> f64 { self.page_scale_factor }
}


pub struct SetPageScaleFactorParamsBuilder {
    page_scale_factor: f64,
}

impl SetPageScaleFactorParamsBuilder {
    pub fn build(self) -> SetPageScaleFactorParams {
        SetPageScaleFactorParams {
            page_scale_factor: self.page_scale_factor,
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
    /// Creates a builder for this type with the required parameters:
    /// * `value`: Whether script execution should be disabled in the page.
    pub fn builder(value: bool) -> SetScriptExecutionDisabledParamsBuilder {
        SetScriptExecutionDisabledParamsBuilder {
            value: value,
        }
    }
    /// Whether script execution should be disabled in the page.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxTouchPoints")]
    max_touch_points: Option<i64>,
}

impl SetTouchEmulationEnabledParams {
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: Whether the touch event emulation should be enabled.
    pub fn builder(enabled: bool) -> SetTouchEmulationEnabledParamsBuilder {
        SetTouchEmulationEnabledParamsBuilder {
            enabled: enabled,
            max_touch_points: None,
        }
    }
    /// Whether the touch event emulation should be enabled.
    pub fn enabled(&self) -> bool { self.enabled }
    /// Maximum touch points supported. Defaults to one.
    pub fn max_touch_points(&self) -> Option<i64> { self.max_touch_points }
}


pub struct SetTouchEmulationEnabledParamsBuilder {
    enabled: bool,
    max_touch_points: Option<i64>,
}

impl SetTouchEmulationEnabledParamsBuilder {
    /// Maximum touch points supported. Defaults to one.
    pub fn max_touch_points(mut self, max_touch_points: i64) -> Self { self.max_touch_points = Some(max_touch_points); self }
    pub fn build(self) -> SetTouchEmulationEnabledParams {
        SetTouchEmulationEnabledParams {
            enabled: self.enabled,
            max_touch_points: self.max_touch_points,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxVirtualTimeTaskStarvationCount")]
    max_virtual_time_task_starvation_count: Option<u64>,
    /// If set, base::Time::Now will be overridden to initially return this value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "initialVirtualTime")]
    initial_virtual_time: Option<crate::network::TimeSinceEpoch>,
}

impl SetVirtualTimePolicyParams {
    /// Creates a builder for this type with the required parameters:
    /// * `policy`: 
    pub fn builder(policy: impl Into<VirtualTimePolicy>) -> SetVirtualTimePolicyParamsBuilder {
        SetVirtualTimePolicyParamsBuilder {
            policy: policy.into(),
            budget: None,
            max_virtual_time_task_starvation_count: None,
            initial_virtual_time: None,
        }
    }
    pub fn policy(&self) -> &VirtualTimePolicy { &self.policy }
    /// If set, after this many virtual milliseconds have elapsed virtual time will be paused and a
    /// virtualTimeBudgetExpired event is sent.
    pub fn budget(&self) -> Option<f64> { self.budget }
    /// If set this specifies the maximum number of tasks that can be run before virtual is forced
    /// forwards to prevent deadlock.
    pub fn max_virtual_time_task_starvation_count(&self) -> Option<u64> { self.max_virtual_time_task_starvation_count }
    /// If set, base::Time::Now will be overridden to initially return this value.
    pub fn initial_virtual_time(&self) -> Option<&crate::network::TimeSinceEpoch> { self.initial_virtual_time.as_ref() }
}


pub struct SetVirtualTimePolicyParamsBuilder {
    policy: VirtualTimePolicy,
    budget: Option<f64>,
    max_virtual_time_task_starvation_count: Option<u64>,
    initial_virtual_time: Option<crate::network::TimeSinceEpoch>,
}

impl SetVirtualTimePolicyParamsBuilder {
    /// If set, after this many virtual milliseconds have elapsed virtual time will be paused and a
    /// virtualTimeBudgetExpired event is sent.
    pub fn budget(mut self, budget: f64) -> Self { self.budget = Some(budget); self }
    /// If set this specifies the maximum number of tasks that can be run before virtual is forced
    /// forwards to prevent deadlock.
    pub fn max_virtual_time_task_starvation_count(mut self, max_virtual_time_task_starvation_count: u64) -> Self { self.max_virtual_time_task_starvation_count = Some(max_virtual_time_task_starvation_count); self }
    /// If set, base::Time::Now will be overridden to initially return this value.
    pub fn initial_virtual_time(mut self, initial_virtual_time: crate::network::TimeSinceEpoch) -> Self { self.initial_virtual_time = Some(initial_virtual_time); self }
    pub fn build(self) -> SetVirtualTimePolicyParams {
        SetVirtualTimePolicyParams {
            policy: self.policy,
            budget: self.budget,
            max_virtual_time_task_starvation_count: self.max_virtual_time_task_starvation_count,
            initial_virtual_time: self.initial_virtual_time,
        }
    }
}

/// Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
/// the current virtual time policy.  Note this supersedes any previous time budget.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVirtualTimePolicyReturns {
    /// Absolute timestamp at which virtual time was first enabled (up time in milliseconds).
    #[serde(rename = "virtualTimeTicksBase")]
    virtual_time_ticks_base: f64,
}

impl SetVirtualTimePolicyReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `virtual_time_ticks_base`: Absolute timestamp at which virtual time was first enabled (up time in milliseconds).
    pub fn builder(virtual_time_ticks_base: f64) -> SetVirtualTimePolicyReturnsBuilder {
        SetVirtualTimePolicyReturnsBuilder {
            virtual_time_ticks_base: virtual_time_ticks_base,
        }
    }
    /// Absolute timestamp at which virtual time was first enabled (up time in milliseconds).
    pub fn virtual_time_ticks_base(&self) -> f64 { self.virtual_time_ticks_base }
}


pub struct SetVirtualTimePolicyReturnsBuilder {
    virtual_time_ticks_base: f64,
}

impl SetVirtualTimePolicyReturnsBuilder {
    pub fn build(self) -> SetVirtualTimePolicyReturns {
        SetVirtualTimePolicyReturns {
            virtual_time_ticks_base: self.virtual_time_ticks_base,
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
    /// Creates a builder for this type.
    pub fn builder() -> SetLocaleOverrideParamsBuilder<'a> {
        SetLocaleOverrideParamsBuilder {
            locale: None,
        }
    }
    /// ICU style C locale (e.g. "en_US"). If not specified or empty, disables the override and
    /// restores default host system locale.
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
    /// <https://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt>
    /// If empty, disables the override and restores default host system timezone.
    #[serde(rename = "timezoneId")]
    timezone_id: Cow<'a, str>,
}

impl<'a> SetTimezoneOverrideParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `timezone_id`: The timezone identifier. List of supported timezones: <https://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt> If empty, disables the override and restores default host system timezone.
    pub fn builder(timezone_id: impl Into<Cow<'a, str>>) -> SetTimezoneOverrideParamsBuilder<'a> {
        SetTimezoneOverrideParamsBuilder {
            timezone_id: timezone_id.into(),
        }
    }
    /// The timezone identifier. List of supported timezones:
    /// <https://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt>
    /// If empty, disables the override and restores default host system timezone.
    pub fn timezone_id(&self) -> &str { self.timezone_id.as_ref() }
}


pub struct SetTimezoneOverrideParamsBuilder<'a> {
    timezone_id: Cow<'a, str>,
}

impl<'a> SetTimezoneOverrideParamsBuilder<'a> {
    pub fn build(self) -> SetTimezoneOverrideParams<'a> {
        SetTimezoneOverrideParams {
            timezone_id: self.timezone_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `width`: Frame width (DIP).
    /// * `height`: Frame height (DIP).
    pub fn builder(width: u64, height: i64) -> SetVisibleSizeParamsBuilder {
        SetVisibleSizeParamsBuilder {
            width: width,
            height: height,
        }
    }
    /// Frame width (DIP).
    pub fn width(&self) -> u64 { self.width }
    /// Frame height (DIP).
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
    #[serde(rename = "imageTypes")]
    image_types: Vec<DisabledImageType>,
}

impl SetDisabledImageTypesParams {
    /// Creates a builder for this type with the required parameters:
    /// * `image_types`: Image types to disable.
    pub fn builder(image_types: Vec<DisabledImageType>) -> SetDisabledImageTypesParamsBuilder {
        SetDisabledImageTypesParamsBuilder {
            image_types: image_types,
        }
    }
    /// Image types to disable.
    pub fn image_types(&self) -> &[DisabledImageType] { &self.image_types }
}


pub struct SetDisabledImageTypesParamsBuilder {
    image_types: Vec<DisabledImageType>,
}

impl SetDisabledImageTypesParamsBuilder {
    pub fn build(self) -> SetDisabledImageTypesParams {
        SetDisabledImageTypesParams {
            image_types: self.image_types,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataSaverEnabled")]
    data_saver_enabled: Option<bool>,
}

impl SetDataSaverOverrideParams {
    /// Creates a builder for this type.
    pub fn builder() -> SetDataSaverOverrideParamsBuilder {
        SetDataSaverOverrideParamsBuilder {
            data_saver_enabled: None,
        }
    }
    /// Override value. Omitting the parameter disables the override.
    pub fn data_saver_enabled(&self) -> Option<bool> { self.data_saver_enabled }
}

#[derive(Default)]
pub struct SetDataSaverOverrideParamsBuilder {
    data_saver_enabled: Option<bool>,
}

impl SetDataSaverOverrideParamsBuilder {
    /// Override value. Omitting the parameter disables the override.
    pub fn data_saver_enabled(mut self, data_saver_enabled: bool) -> Self { self.data_saver_enabled = Some(data_saver_enabled); self }
    pub fn build(self) -> SetDataSaverOverrideParams {
        SetDataSaverOverrideParams {
            data_saver_enabled: self.data_saver_enabled,
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
    #[serde(rename = "hardwareConcurrency")]
    hardware_concurrency: i64,
}

impl SetHardwareConcurrencyOverrideParams {
    /// Creates a builder for this type with the required parameters:
    /// * `hardware_concurrency`: Hardware concurrency to report
    pub fn builder(hardware_concurrency: i64) -> SetHardwareConcurrencyOverrideParamsBuilder {
        SetHardwareConcurrencyOverrideParamsBuilder {
            hardware_concurrency: hardware_concurrency,
        }
    }
    /// Hardware concurrency to report
    pub fn hardware_concurrency(&self) -> i64 { self.hardware_concurrency }
}


pub struct SetHardwareConcurrencyOverrideParamsBuilder {
    hardware_concurrency: i64,
}

impl SetHardwareConcurrencyOverrideParamsBuilder {
    pub fn build(self) -> SetHardwareConcurrencyOverrideParams {
        SetHardwareConcurrencyOverrideParams {
            hardware_concurrency: self.hardware_concurrency,
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
    #[serde(rename = "userAgent")]
    user_agent: Cow<'a, str>,
    /// Browser language to emulate.
    #[serde(skip_serializing_if = "Option::is_none", rename = "acceptLanguage")]
    accept_language: Option<Cow<'a, str>>,
    /// The platform navigator.platform should return.
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<Cow<'a, str>>,
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    #[serde(skip_serializing_if = "Option::is_none", rename = "userAgentMetadata")]
    user_agent_metadata: Option<UserAgentMetadata<'a>>,
}

impl<'a> SetUserAgentOverrideParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `user_agent`: User agent to use.
    pub fn builder(user_agent: impl Into<Cow<'a, str>>) -> SetUserAgentOverrideParamsBuilder<'a> {
        SetUserAgentOverrideParamsBuilder {
            user_agent: user_agent.into(),
            accept_language: None,
            platform: None,
            user_agent_metadata: None,
        }
    }
    /// User agent to use.
    pub fn user_agent(&self) -> &str { self.user_agent.as_ref() }
    /// Browser language to emulate.
    pub fn accept_language(&self) -> Option<&str> { self.accept_language.as_deref() }
    /// The platform navigator.platform should return.
    pub fn platform(&self) -> Option<&str> { self.platform.as_deref() }
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    pub fn user_agent_metadata(&self) -> Option<&UserAgentMetadata<'a>> { self.user_agent_metadata.as_ref() }
}


pub struct SetUserAgentOverrideParamsBuilder<'a> {
    user_agent: Cow<'a, str>,
    accept_language: Option<Cow<'a, str>>,
    platform: Option<Cow<'a, str>>,
    user_agent_metadata: Option<UserAgentMetadata<'a>>,
}

impl<'a> SetUserAgentOverrideParamsBuilder<'a> {
    /// Browser language to emulate.
    pub fn accept_language(mut self, accept_language: impl Into<Cow<'a, str>>) -> Self { self.accept_language = Some(accept_language.into()); self }
    /// The platform navigator.platform should return.
    pub fn platform(mut self, platform: impl Into<Cow<'a, str>>) -> Self { self.platform = Some(platform.into()); self }
    /// To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    pub fn user_agent_metadata(mut self, user_agent_metadata: UserAgentMetadata<'a>) -> Self { self.user_agent_metadata = Some(user_agent_metadata); self }
    pub fn build(self) -> SetUserAgentOverrideParams<'a> {
        SetUserAgentOverrideParams {
            user_agent: self.user_agent,
            accept_language: self.accept_language,
            platform: self.platform,
            user_agent_metadata: self.user_agent_metadata,
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
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: Whether the override should be enabled.
    pub fn builder(enabled: bool) -> SetAutomationOverrideParamsBuilder {
        SetAutomationOverrideParamsBuilder {
            enabled: enabled,
        }
    }
    /// Whether the override should be enabled.
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
    /// Creates a builder for this type with the required parameters:
    /// * `difference`: This will cause an element of size 100svh to be `difference` pixels smaller than an element of size 100lvh.
    pub fn builder(difference: i64) -> SetSmallViewportHeightDifferenceOverrideParamsBuilder {
        SetSmallViewportHeightDifferenceOverrideParamsBuilder {
            difference: difference,
        }
    }
    /// This will cause an element of size 100svh to be 'difference' pixels smaller than an element
    /// of size 100lvh.
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
    #[serde(rename = "screenInfos")]
    screen_infos: Vec<ScreenInfo<'a>>,
}

impl<'a> GetScreenInfosReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `screen_infos`: 
    pub fn builder(screen_infos: Vec<ScreenInfo<'a>>) -> GetScreenInfosReturnsBuilder<'a> {
        GetScreenInfosReturnsBuilder {
            screen_infos: screen_infos,
        }
    }
    pub fn screen_infos(&self) -> &[ScreenInfo<'a>] { &self.screen_infos }
}


pub struct GetScreenInfosReturnsBuilder<'a> {
    screen_infos: Vec<ScreenInfo<'a>>,
}

impl<'a> GetScreenInfosReturnsBuilder<'a> {
    pub fn build(self) -> GetScreenInfosReturns<'a> {
        GetScreenInfosReturns {
            screen_infos: self.screen_infos,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "workAreaInsets")]
    work_area_insets: Option<WorkAreaInsets>,
    /// Specifies the screen's device pixel ratio. Default is 1.
    #[serde(skip_serializing_if = "Option::is_none", rename = "devicePixelRatio")]
    device_pixel_ratio: Option<f64>,
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<i64>,
    /// Specifies the screen's color depth in bits. Default is 24.
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorDepth")]
    color_depth: Option<i64>,
    /// Specifies the descriptive label for the screen. Default is none.
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Cow<'a, str>>,
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isInternal")]
    is_internal: Option<bool>,
}

impl<'a> AddScreenParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `left`: Offset of the left edge of the screen in pixels.
    /// * `top`: Offset of the top edge of the screen in pixels.
    /// * `width`: The width of the screen in pixels.
    /// * `height`: The height of the screen in pixels.
    pub fn builder(left: i64, top: i64, width: u64, height: i64) -> AddScreenParamsBuilder<'a> {
        AddScreenParamsBuilder {
            left: left,
            top: top,
            width: width,
            height: height,
            work_area_insets: None,
            device_pixel_ratio: None,
            rotation: None,
            color_depth: None,
            label: None,
            is_internal: None,
        }
    }
    /// Offset of the left edge of the screen in pixels.
    pub fn left(&self) -> i64 { self.left }
    /// Offset of the top edge of the screen in pixels.
    pub fn top(&self) -> i64 { self.top }
    /// The width of the screen in pixels.
    pub fn width(&self) -> u64 { self.width }
    /// The height of the screen in pixels.
    pub fn height(&self) -> i64 { self.height }
    /// Specifies the screen's work area. Default is entire screen.
    pub fn work_area_insets(&self) -> Option<&WorkAreaInsets> { self.work_area_insets.as_ref() }
    /// Specifies the screen's device pixel ratio. Default is 1.
    pub fn device_pixel_ratio(&self) -> Option<f64> { self.device_pixel_ratio }
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0.
    pub fn rotation(&self) -> Option<i64> { self.rotation }
    /// Specifies the screen's color depth in bits. Default is 24.
    pub fn color_depth(&self) -> Option<i64> { self.color_depth }
    /// Specifies the descriptive label for the screen. Default is none.
    pub fn label(&self) -> Option<&str> { self.label.as_deref() }
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    pub fn is_internal(&self) -> Option<bool> { self.is_internal }
}


pub struct AddScreenParamsBuilder<'a> {
    left: i64,
    top: i64,
    width: u64,
    height: i64,
    work_area_insets: Option<WorkAreaInsets>,
    device_pixel_ratio: Option<f64>,
    rotation: Option<i64>,
    color_depth: Option<i64>,
    label: Option<Cow<'a, str>>,
    is_internal: Option<bool>,
}

impl<'a> AddScreenParamsBuilder<'a> {
    /// Specifies the screen's work area. Default is entire screen.
    pub fn work_area_insets(mut self, work_area_insets: WorkAreaInsets) -> Self { self.work_area_insets = Some(work_area_insets); self }
    /// Specifies the screen's device pixel ratio. Default is 1.
    pub fn device_pixel_ratio(mut self, device_pixel_ratio: f64) -> Self { self.device_pixel_ratio = Some(device_pixel_ratio); self }
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0.
    pub fn rotation(mut self, rotation: i64) -> Self { self.rotation = Some(rotation); self }
    /// Specifies the screen's color depth in bits. Default is 24.
    pub fn color_depth(mut self, color_depth: i64) -> Self { self.color_depth = Some(color_depth); self }
    /// Specifies the descriptive label for the screen. Default is none.
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self { self.label = Some(label.into()); self }
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    pub fn is_internal(mut self, is_internal: bool) -> Self { self.is_internal = Some(is_internal); self }
    pub fn build(self) -> AddScreenParams<'a> {
        AddScreenParams {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            work_area_insets: self.work_area_insets,
            device_pixel_ratio: self.device_pixel_ratio,
            rotation: self.rotation,
            color_depth: self.color_depth,
            label: self.label,
            is_internal: self.is_internal,
        }
    }
}

/// Add a new screen to the device. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScreenReturns<'a> {
    #[serde(rename = "screenInfo")]
    screen_info: ScreenInfo<'a>,
}

impl<'a> AddScreenReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `screen_info`: 
    pub fn builder(screen_info: ScreenInfo<'a>) -> AddScreenReturnsBuilder<'a> {
        AddScreenReturnsBuilder {
            screen_info: screen_info,
        }
    }
    pub fn screen_info(&self) -> &ScreenInfo<'a> { &self.screen_info }
}


pub struct AddScreenReturnsBuilder<'a> {
    screen_info: ScreenInfo<'a>,
}

impl<'a> AddScreenReturnsBuilder<'a> {
    pub fn build(self) -> AddScreenReturns<'a> {
        AddScreenReturns {
            screen_info: self.screen_info,
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
    #[serde(rename = "screenId")]
    screen_id: ScreenId<'a>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "workAreaInsets")]
    work_area_insets: Option<WorkAreaInsets>,
    /// Specifies the screen's device pixel ratio.
    #[serde(skip_serializing_if = "Option::is_none", rename = "devicePixelRatio")]
    device_pixel_ratio: Option<f64>,
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270.
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<i64>,
    /// Specifies the screen's color depth in bits.
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorDepth")]
    color_depth: Option<i64>,
    /// Specifies the descriptive label for the screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Cow<'a, str>>,
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isInternal")]
    is_internal: Option<bool>,
}

impl<'a> UpdateScreenParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `screen_id`: Target screen identifier.
    pub fn builder(screen_id: impl Into<ScreenId<'a>>) -> UpdateScreenParamsBuilder<'a> {
        UpdateScreenParamsBuilder {
            screen_id: screen_id.into(),
            left: None,
            top: None,
            width: None,
            height: None,
            work_area_insets: None,
            device_pixel_ratio: None,
            rotation: None,
            color_depth: None,
            label: None,
            is_internal: None,
        }
    }
    /// Target screen identifier.
    pub fn screen_id(&self) -> &ScreenId<'a> { &self.screen_id }
    /// Offset of the left edge of the screen in pixels.
    pub fn left(&self) -> Option<i64> { self.left }
    /// Offset of the top edge of the screen in pixels.
    pub fn top(&self) -> Option<i64> { self.top }
    /// The width of the screen in pixels.
    pub fn width(&self) -> Option<u64> { self.width }
    /// The height of the screen in pixels.
    pub fn height(&self) -> Option<i64> { self.height }
    /// Specifies the screen's work area.
    pub fn work_area_insets(&self) -> Option<&WorkAreaInsets> { self.work_area_insets.as_ref() }
    /// Specifies the screen's device pixel ratio.
    pub fn device_pixel_ratio(&self) -> Option<f64> { self.device_pixel_ratio }
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270.
    pub fn rotation(&self) -> Option<i64> { self.rotation }
    /// Specifies the screen's color depth in bits.
    pub fn color_depth(&self) -> Option<i64> { self.color_depth }
    /// Specifies the descriptive label for the screen.
    pub fn label(&self) -> Option<&str> { self.label.as_deref() }
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    pub fn is_internal(&self) -> Option<bool> { self.is_internal }
}


pub struct UpdateScreenParamsBuilder<'a> {
    screen_id: ScreenId<'a>,
    left: Option<i64>,
    top: Option<i64>,
    width: Option<u64>,
    height: Option<i64>,
    work_area_insets: Option<WorkAreaInsets>,
    device_pixel_ratio: Option<f64>,
    rotation: Option<i64>,
    color_depth: Option<i64>,
    label: Option<Cow<'a, str>>,
    is_internal: Option<bool>,
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
    pub fn work_area_insets(mut self, work_area_insets: WorkAreaInsets) -> Self { self.work_area_insets = Some(work_area_insets); self }
    /// Specifies the screen's device pixel ratio.
    pub fn device_pixel_ratio(mut self, device_pixel_ratio: f64) -> Self { self.device_pixel_ratio = Some(device_pixel_ratio); self }
    /// Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270.
    pub fn rotation(mut self, rotation: i64) -> Self { self.rotation = Some(rotation); self }
    /// Specifies the screen's color depth in bits.
    pub fn color_depth(mut self, color_depth: i64) -> Self { self.color_depth = Some(color_depth); self }
    /// Specifies the descriptive label for the screen.
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self { self.label = Some(label.into()); self }
    /// Indicates whether the screen is internal to the device or external, attached to the device. Default is false.
    pub fn is_internal(mut self, is_internal: bool) -> Self { self.is_internal = Some(is_internal); self }
    pub fn build(self) -> UpdateScreenParams<'a> {
        UpdateScreenParams {
            screen_id: self.screen_id,
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            work_area_insets: self.work_area_insets,
            device_pixel_ratio: self.device_pixel_ratio,
            rotation: self.rotation,
            color_depth: self.color_depth,
            label: self.label,
            is_internal: self.is_internal,
        }
    }
}

/// Updates specified screen parameters. Only supported in headless mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScreenReturns<'a> {
    #[serde(rename = "screenInfo")]
    screen_info: ScreenInfo<'a>,
}

impl<'a> UpdateScreenReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `screen_info`: 
    pub fn builder(screen_info: ScreenInfo<'a>) -> UpdateScreenReturnsBuilder<'a> {
        UpdateScreenReturnsBuilder {
            screen_info: screen_info,
        }
    }
    pub fn screen_info(&self) -> &ScreenInfo<'a> { &self.screen_info }
}


pub struct UpdateScreenReturnsBuilder<'a> {
    screen_info: ScreenInfo<'a>,
}

impl<'a> UpdateScreenReturnsBuilder<'a> {
    pub fn build(self) -> UpdateScreenReturns<'a> {
        UpdateScreenReturns {
            screen_info: self.screen_info,
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
    #[serde(rename = "screenId")]
    screen_id: ScreenId<'a>,
}

impl<'a> RemoveScreenParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `screen_id`: 
    pub fn builder(screen_id: impl Into<ScreenId<'a>>) -> RemoveScreenParamsBuilder<'a> {
        RemoveScreenParamsBuilder {
            screen_id: screen_id.into(),
        }
    }
    pub fn screen_id(&self) -> &ScreenId<'a> { &self.screen_id }
}


pub struct RemoveScreenParamsBuilder<'a> {
    screen_id: ScreenId<'a>,
}

impl<'a> RemoveScreenParamsBuilder<'a> {
    pub fn build(self) -> RemoveScreenParams<'a> {
        RemoveScreenParams {
            screen_id: self.screen_id,
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
    #[serde(rename = "screenId")]
    screen_id: ScreenId<'a>,
}

impl<'a> SetPrimaryScreenParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `screen_id`: 
    pub fn builder(screen_id: impl Into<ScreenId<'a>>) -> SetPrimaryScreenParamsBuilder<'a> {
        SetPrimaryScreenParamsBuilder {
            screen_id: screen_id.into(),
        }
    }
    pub fn screen_id(&self) -> &ScreenId<'a> { &self.screen_id }
}


pub struct SetPrimaryScreenParamsBuilder<'a> {
    screen_id: ScreenId<'a>,
}

impl<'a> SetPrimaryScreenParamsBuilder<'a> {
    pub fn build(self) -> SetPrimaryScreenParams<'a> {
        SetPrimaryScreenParams {
            screen_id: self.screen_id,
        }
    }
}

impl<'a> SetPrimaryScreenParams<'a> { pub const METHOD: &'static str = "Emulation.setPrimaryScreen"; }

impl<'a> crate::CdpCommand<'a> for SetPrimaryScreenParams<'a> {
    const METHOD: &'static str = "Emulation.setPrimaryScreen";
    type Response = crate::EmptyReturns;
}
