//! Actions and events related to the inspected page belong to the page domain.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Unique frame identifier.

pub type FrameId<'a> = Cow<'a, str>;

/// Indicates whether a frame has been identified as an ad.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AdFrameType {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "root")]
    Root,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AdFrameExplanation {
    #[default]
    #[serde(rename = "ParentIsAd")]
    ParentIsAd,
    #[serde(rename = "CreatedByAdScript")]
    CreatedByAdScript,
    #[serde(rename = "MatchedBlockingRule")]
    MatchedBlockingRule,
}

/// Indicates whether a frame has been identified as an ad and why.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdFrameStatus {
    #[serde(rename = "adFrameType")]
    ad_frame_type: AdFrameType,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanations: Option<Vec<AdFrameExplanation>>,
}

impl AdFrameStatus {
    /// Creates a builder for this type with the required parameters:
    /// * `ad_frame_type`: 
    pub fn builder(ad_frame_type: impl Into<AdFrameType>) -> AdFrameStatusBuilder {
        AdFrameStatusBuilder {
            ad_frame_type: ad_frame_type.into(),
            explanations: None,
        }
    }
    pub fn ad_frame_type(&self) -> &AdFrameType { &self.ad_frame_type }
    pub fn explanations(&self) -> Option<&[AdFrameExplanation]> { self.explanations.as_deref() }
}


pub struct AdFrameStatusBuilder {
    ad_frame_type: AdFrameType,
    explanations: Option<Vec<AdFrameExplanation>>,
}

impl AdFrameStatusBuilder {
    pub fn explanations(mut self, explanations: Vec<AdFrameExplanation>) -> Self { self.explanations = Some(explanations); self }
    pub fn build(self) -> AdFrameStatus {
        AdFrameStatus {
            ad_frame_type: self.ad_frame_type,
            explanations: self.explanations,
        }
    }
}

/// Indicates whether the frame is a secure context and why it is the case.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SecureContextType {
    #[default]
    #[serde(rename = "Secure")]
    Secure,
    #[serde(rename = "SecureLocalhost")]
    SecureLocalhost,
    #[serde(rename = "InsecureScheme")]
    InsecureScheme,
    #[serde(rename = "InsecureAncestor")]
    InsecureAncestor,
}

/// Indicates whether the frame is cross-origin isolated and why it is the case.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CrossOriginIsolatedContextType {
    #[default]
    #[serde(rename = "Isolated")]
    Isolated,
    #[serde(rename = "NotIsolated")]
    NotIsolated,
    #[serde(rename = "NotIsolatedFeatureDisabled")]
    NotIsolatedFeatureDisabled,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GatedAPIFeatures {
    #[default]
    #[serde(rename = "SharedArrayBuffers")]
    SharedArrayBuffers,
    #[serde(rename = "SharedArrayBuffersTransferAllowed")]
    SharedArrayBuffersTransferAllowed,
    #[serde(rename = "PerformanceMeasureMemory")]
    PerformanceMeasureMemory,
    #[serde(rename = "PerformanceProfile")]
    PerformanceProfile,
}

/// All Permissions Policy features. This enum should match the one defined
/// in services/network/public/cpp/permissions_policy/permissions_policy_features.json5.
/// LINT.IfChange(PermissionsPolicyFeature)

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionsPolicyFeature {
    #[default]
    #[serde(rename = "accelerometer")]
    Accelerometer,
    #[serde(rename = "all-screens-capture")]
    AllScreensCapture,
    #[serde(rename = "ambient-light-sensor")]
    AmbientLightSensor,
    #[serde(rename = "aria-notify")]
    AriaNotify,
    #[serde(rename = "attribution-reporting")]
    AttributionReporting,
    #[serde(rename = "autofill")]
    Autofill,
    #[serde(rename = "autoplay")]
    Autoplay,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "browsing-topics")]
    BrowsingTopics,
    #[serde(rename = "camera")]
    Camera,
    #[serde(rename = "captured-surface-control")]
    CapturedSurfaceControl,
    #[serde(rename = "ch-dpr")]
    ChDpr,
    #[serde(rename = "ch-device-memory")]
    ChDeviceMemory,
    #[serde(rename = "ch-downlink")]
    ChDownlink,
    #[serde(rename = "ch-ect")]
    ChEct,
    #[serde(rename = "ch-prefers-color-scheme")]
    ChPrefersColorScheme,
    #[serde(rename = "ch-prefers-reduced-motion")]
    ChPrefersReducedMotion,
    #[serde(rename = "ch-prefers-reduced-transparency")]
    ChPrefersReducedTransparency,
    #[serde(rename = "ch-rtt")]
    ChRtt,
    #[serde(rename = "ch-save-data")]
    ChSaveData,
    #[serde(rename = "ch-ua")]
    ChUa,
    #[serde(rename = "ch-ua-arch")]
    ChUaArch,
    #[serde(rename = "ch-ua-bitness")]
    ChUaBitness,
    #[serde(rename = "ch-ua-high-entropy-values")]
    ChUaHighEntropyValues,
    #[serde(rename = "ch-ua-platform")]
    ChUaPlatform,
    #[serde(rename = "ch-ua-model")]
    ChUaModel,
    #[serde(rename = "ch-ua-mobile")]
    ChUaMobile,
    #[serde(rename = "ch-ua-form-factors")]
    ChUaFormFactors,
    #[serde(rename = "ch-ua-full-version")]
    ChUaFullVersion,
    #[serde(rename = "ch-ua-full-version-list")]
    ChUaFullVersionList,
    #[serde(rename = "ch-ua-platform-version")]
    ChUaPlatformVersion,
    #[serde(rename = "ch-ua-wow64")]
    ChUaWow64,
    #[serde(rename = "ch-viewport-height")]
    ChViewportHeight,
    #[serde(rename = "ch-viewport-width")]
    ChViewportWidth,
    #[serde(rename = "ch-width")]
    ChWidth,
    #[serde(rename = "clipboard-read")]
    ClipboardRead,
    #[serde(rename = "clipboard-write")]
    ClipboardWrite,
    #[serde(rename = "compute-pressure")]
    ComputePressure,
    #[serde(rename = "controlled-frame")]
    ControlledFrame,
    #[serde(rename = "cross-origin-isolated")]
    CrossOriginIsolated,
    #[serde(rename = "deferred-fetch")]
    DeferredFetch,
    #[serde(rename = "deferred-fetch-minimal")]
    DeferredFetchMinimal,
    #[serde(rename = "device-attributes")]
    DeviceAttributes,
    #[serde(rename = "digital-credentials-create")]
    DigitalCredentialsCreate,
    #[serde(rename = "digital-credentials-get")]
    DigitalCredentialsGet,
    #[serde(rename = "direct-sockets")]
    DirectSockets,
    #[serde(rename = "direct-sockets-multicast")]
    DirectSocketsMulticast,
    #[serde(rename = "direct-sockets-private")]
    DirectSocketsPrivate,
    #[serde(rename = "display-capture")]
    DisplayCapture,
    #[serde(rename = "document-domain")]
    DocumentDomain,
    #[serde(rename = "encrypted-media")]
    EncryptedMedia,
    #[serde(rename = "execution-while-out-of-viewport")]
    ExecutionWhileOutOfViewport,
    #[serde(rename = "execution-while-not-rendered")]
    ExecutionWhileNotRendered,
    #[serde(rename = "focus-without-user-activation")]
    FocusWithoutUserActivation,
    #[serde(rename = "fullscreen")]
    Fullscreen,
    #[serde(rename = "frobulate")]
    Frobulate,
    #[serde(rename = "gamepad")]
    Gamepad,
    #[serde(rename = "geolocation")]
    Geolocation,
    #[serde(rename = "gyroscope")]
    Gyroscope,
    #[serde(rename = "hid")]
    Hid,
    #[serde(rename = "identity-credentials-get")]
    IdentityCredentialsGet,
    #[serde(rename = "idle-detection")]
    IdleDetection,
    #[serde(rename = "interest-cohort")]
    InterestCohort,
    #[serde(rename = "join-ad-interest-group")]
    JoinAdInterestGroup,
    #[serde(rename = "keyboard-map")]
    KeyboardMap,
    #[serde(rename = "language-detector")]
    LanguageDetector,
    #[serde(rename = "language-model")]
    LanguageModel,
    #[serde(rename = "local-fonts")]
    LocalFonts,
    #[serde(rename = "local-network")]
    LocalNetwork,
    #[serde(rename = "local-network-access")]
    LocalNetworkAccess,
    #[serde(rename = "loopback-network")]
    LoopbackNetwork,
    #[serde(rename = "magnetometer")]
    Magnetometer,
    #[serde(rename = "manual-text")]
    ManualText,
    #[serde(rename = "media-playback-while-not-visible")]
    MediaPlaybackWhileNotVisible,
    #[serde(rename = "microphone")]
    Microphone,
    #[serde(rename = "midi")]
    Midi,
    #[serde(rename = "on-device-speech-recognition")]
    OnDeviceSpeechRecognition,
    #[serde(rename = "otp-credentials")]
    OtpCredentials,
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "picture-in-picture")]
    PictureInPicture,
    #[serde(rename = "private-aggregation")]
    PrivateAggregation,
    #[serde(rename = "private-state-token-issuance")]
    PrivateStateTokenIssuance,
    #[serde(rename = "private-state-token-redemption")]
    PrivateStateTokenRedemption,
    #[serde(rename = "publickey-credentials-create")]
    PublickeyCredentialsCreate,
    #[serde(rename = "publickey-credentials-get")]
    PublickeyCredentialsGet,
    #[serde(rename = "record-ad-auction-events")]
    RecordAdAuctionEvents,
    #[serde(rename = "rewriter")]
    Rewriter,
    #[serde(rename = "run-ad-auction")]
    RunAdAuction,
    #[serde(rename = "screen-wake-lock")]
    ScreenWakeLock,
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "shared-storage")]
    SharedStorage,
    #[serde(rename = "shared-storage-select-url")]
    SharedStorageSelectUrl,
    #[serde(rename = "smart-card")]
    SmartCard,
    #[serde(rename = "speaker-selection")]
    SpeakerSelection,
    #[serde(rename = "storage-access")]
    StorageAccess,
    #[serde(rename = "sub-apps")]
    SubApps,
    #[serde(rename = "summarizer")]
    Summarizer,
    #[serde(rename = "sync-xhr")]
    SyncXhr,
    #[serde(rename = "tools")]
    Tools,
    #[serde(rename = "translator")]
    Translator,
    #[serde(rename = "unload")]
    Unload,
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "usb-unrestricted")]
    UsbUnrestricted,
    #[serde(rename = "vertical-scroll")]
    VerticalScroll,
    #[serde(rename = "web-app-installation")]
    WebAppInstallation,
    #[serde(rename = "web-printing")]
    WebPrinting,
    #[serde(rename = "web-share")]
    WebShare,
    #[serde(rename = "window-management")]
    WindowManagement,
    #[serde(rename = "writer")]
    Writer,
    #[serde(rename = "xr-spatial-tracking")]
    XrSpatialTracking,
}

/// Reason for a permissions policy feature to be disabled.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionsPolicyBlockReason {
    #[default]
    #[serde(rename = "Header")]
    Header,
    #[serde(rename = "IframeAttribute")]
    IframeAttribute,
    #[serde(rename = "InFencedFrameTree")]
    InFencedFrameTree,
    #[serde(rename = "InIsolatedApp")]
    InIsolatedApp,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsPolicyBlockLocator<'a> {
    #[serde(rename = "frameId")]
    frame_id: FrameId<'a>,
    #[serde(rename = "blockReason")]
    block_reason: PermissionsPolicyBlockReason,
}

impl<'a> PermissionsPolicyBlockLocator<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: 
    /// * `block_reason`: 
    pub fn builder(frame_id: impl Into<FrameId<'a>>, block_reason: impl Into<PermissionsPolicyBlockReason>) -> PermissionsPolicyBlockLocatorBuilder<'a> {
        PermissionsPolicyBlockLocatorBuilder {
            frame_id: frame_id.into(),
            block_reason: block_reason.into(),
        }
    }
    pub fn frame_id(&self) -> &FrameId<'a> { &self.frame_id }
    pub fn block_reason(&self) -> &PermissionsPolicyBlockReason { &self.block_reason }
}


pub struct PermissionsPolicyBlockLocatorBuilder<'a> {
    frame_id: FrameId<'a>,
    block_reason: PermissionsPolicyBlockReason,
}

impl<'a> PermissionsPolicyBlockLocatorBuilder<'a> {
    pub fn build(self) -> PermissionsPolicyBlockLocator<'a> {
        PermissionsPolicyBlockLocator {
            frame_id: self.frame_id,
            block_reason: self.block_reason,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsPolicyFeatureState<'a> {
    feature: PermissionsPolicyFeature,
    allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    locator: Option<PermissionsPolicyBlockLocator<'a>>,
}

impl<'a> PermissionsPolicyFeatureState<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `feature`: 
    /// * `allowed`: 
    pub fn builder(feature: impl Into<PermissionsPolicyFeature>, allowed: bool) -> PermissionsPolicyFeatureStateBuilder<'a> {
        PermissionsPolicyFeatureStateBuilder {
            feature: feature.into(),
            allowed: allowed,
            locator: None,
        }
    }
    pub fn feature(&self) -> &PermissionsPolicyFeature { &self.feature }
    pub fn allowed(&self) -> bool { self.allowed }
    pub fn locator(&self) -> Option<&PermissionsPolicyBlockLocator<'a>> { self.locator.as_ref() }
}


pub struct PermissionsPolicyFeatureStateBuilder<'a> {
    feature: PermissionsPolicyFeature,
    allowed: bool,
    locator: Option<PermissionsPolicyBlockLocator<'a>>,
}

impl<'a> PermissionsPolicyFeatureStateBuilder<'a> {
    pub fn locator(mut self, locator: PermissionsPolicyBlockLocator<'a>) -> Self { self.locator = Some(locator); self }
    pub fn build(self) -> PermissionsPolicyFeatureState<'a> {
        PermissionsPolicyFeatureState {
            feature: self.feature,
            allowed: self.allowed,
            locator: self.locator,
        }
    }
}

/// Origin Trial(<https://www.chromium.org/blink/origin-trials>) support.
/// Status for an Origin Trial token.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum OriginTrialTokenStatus {
    #[default]
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "NotSupported")]
    NotSupported,
    #[serde(rename = "Insecure")]
    Insecure,
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "WrongOrigin")]
    WrongOrigin,
    #[serde(rename = "InvalidSignature")]
    InvalidSignature,
    #[serde(rename = "Malformed")]
    Malformed,
    #[serde(rename = "WrongVersion")]
    WrongVersion,
    #[serde(rename = "FeatureDisabled")]
    FeatureDisabled,
    #[serde(rename = "TokenDisabled")]
    TokenDisabled,
    #[serde(rename = "FeatureDisabledForUser")]
    FeatureDisabledForUser,
    #[serde(rename = "UnknownTrial")]
    UnknownTrial,
}

/// Status for an Origin Trial.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum OriginTrialStatus {
    #[default]
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "ValidTokenNotProvided")]
    ValidTokenNotProvided,
    #[serde(rename = "OSNotSupported")]
    OSNotSupported,
    #[serde(rename = "TrialNotAllowed")]
    TrialNotAllowed,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum OriginTrialUsageRestriction {
    #[default]
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Subset")]
    Subset,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrialToken<'a> {
    origin: Cow<'a, str>,
    #[serde(rename = "matchSubDomains")]
    match_sub_domains: bool,
    #[serde(rename = "trialName")]
    trial_name: Cow<'a, str>,
    #[serde(rename = "expiryTime")]
    expiry_time: crate::network::TimeSinceEpoch,
    #[serde(rename = "isThirdParty")]
    is_third_party: bool,
    #[serde(rename = "usageRestriction")]
    usage_restriction: OriginTrialUsageRestriction,
}

impl<'a> OriginTrialToken<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: 
    /// * `match_sub_domains`: 
    /// * `trial_name`: 
    /// * `expiry_time`: 
    /// * `is_third_party`: 
    /// * `usage_restriction`: 
    pub fn builder(origin: impl Into<Cow<'a, str>>, match_sub_domains: bool, trial_name: impl Into<Cow<'a, str>>, expiry_time: crate::network::TimeSinceEpoch, is_third_party: bool, usage_restriction: impl Into<OriginTrialUsageRestriction>) -> OriginTrialTokenBuilder<'a> {
        OriginTrialTokenBuilder {
            origin: origin.into(),
            match_sub_domains: match_sub_domains,
            trial_name: trial_name.into(),
            expiry_time: expiry_time,
            is_third_party: is_third_party,
            usage_restriction: usage_restriction.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn match_sub_domains(&self) -> bool { self.match_sub_domains }
    pub fn trial_name(&self) -> &str { self.trial_name.as_ref() }
    pub fn expiry_time(&self) -> &crate::network::TimeSinceEpoch { &self.expiry_time }
    pub fn is_third_party(&self) -> bool { self.is_third_party }
    pub fn usage_restriction(&self) -> &OriginTrialUsageRestriction { &self.usage_restriction }
}


pub struct OriginTrialTokenBuilder<'a> {
    origin: Cow<'a, str>,
    match_sub_domains: bool,
    trial_name: Cow<'a, str>,
    expiry_time: crate::network::TimeSinceEpoch,
    is_third_party: bool,
    usage_restriction: OriginTrialUsageRestriction,
}

impl<'a> OriginTrialTokenBuilder<'a> {
    pub fn build(self) -> OriginTrialToken<'a> {
        OriginTrialToken {
            origin: self.origin,
            match_sub_domains: self.match_sub_domains,
            trial_name: self.trial_name,
            expiry_time: self.expiry_time,
            is_third_party: self.is_third_party,
            usage_restriction: self.usage_restriction,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrialTokenWithStatus<'a> {
    #[serde(rename = "rawTokenText")]
    raw_token_text: Cow<'a, str>,
    /// 'parsedToken' is present only when the token is extractable and
    /// parsable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parsedToken")]
    parsed_token: Option<OriginTrialToken<'a>>,
    status: OriginTrialTokenStatus,
}

impl<'a> OriginTrialTokenWithStatus<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `raw_token_text`: 
    /// * `status`: 
    pub fn builder(raw_token_text: impl Into<Cow<'a, str>>, status: impl Into<OriginTrialTokenStatus>) -> OriginTrialTokenWithStatusBuilder<'a> {
        OriginTrialTokenWithStatusBuilder {
            raw_token_text: raw_token_text.into(),
            parsed_token: None,
            status: status.into(),
        }
    }
    pub fn raw_token_text(&self) -> &str { self.raw_token_text.as_ref() }
    /// 'parsedToken' is present only when the token is extractable and
    /// parsable.
    pub fn parsed_token(&self) -> Option<&OriginTrialToken<'a>> { self.parsed_token.as_ref() }
    pub fn status(&self) -> &OriginTrialTokenStatus { &self.status }
}


pub struct OriginTrialTokenWithStatusBuilder<'a> {
    raw_token_text: Cow<'a, str>,
    parsed_token: Option<OriginTrialToken<'a>>,
    status: OriginTrialTokenStatus,
}

impl<'a> OriginTrialTokenWithStatusBuilder<'a> {
    /// 'parsedToken' is present only when the token is extractable and
    /// parsable.
    pub fn parsed_token(mut self, parsed_token: OriginTrialToken<'a>) -> Self { self.parsed_token = Some(parsed_token); self }
    pub fn build(self) -> OriginTrialTokenWithStatus<'a> {
        OriginTrialTokenWithStatus {
            raw_token_text: self.raw_token_text,
            parsed_token: self.parsed_token,
            status: self.status,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrial<'a> {
    #[serde(rename = "trialName")]
    trial_name: Cow<'a, str>,
    status: OriginTrialStatus,
    #[serde(rename = "tokensWithStatus")]
    tokens_with_status: Vec<OriginTrialTokenWithStatus<'a>>,
}

impl<'a> OriginTrial<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `trial_name`: 
    /// * `status`: 
    /// * `tokens_with_status`: 
    pub fn builder(trial_name: impl Into<Cow<'a, str>>, status: impl Into<OriginTrialStatus>, tokens_with_status: Vec<OriginTrialTokenWithStatus<'a>>) -> OriginTrialBuilder<'a> {
        OriginTrialBuilder {
            trial_name: trial_name.into(),
            status: status.into(),
            tokens_with_status: tokens_with_status,
        }
    }
    pub fn trial_name(&self) -> &str { self.trial_name.as_ref() }
    pub fn status(&self) -> &OriginTrialStatus { &self.status }
    pub fn tokens_with_status(&self) -> &[OriginTrialTokenWithStatus<'a>] { &self.tokens_with_status }
}


pub struct OriginTrialBuilder<'a> {
    trial_name: Cow<'a, str>,
    status: OriginTrialStatus,
    tokens_with_status: Vec<OriginTrialTokenWithStatus<'a>>,
}

impl<'a> OriginTrialBuilder<'a> {
    pub fn build(self) -> OriginTrial<'a> {
        OriginTrial {
            trial_name: self.trial_name,
            status: self.status,
            tokens_with_status: self.tokens_with_status,
        }
    }
}

/// Additional information about the frame document's security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityOriginDetails {
    /// Indicates whether the frame document's security origin is one
    /// of the local hostnames (e.g. "localhost") or IP addresses (IPv4
    /// 127.0.0.0/8 or IPv6 ::1).
    #[serde(rename = "isLocalhost")]
    is_localhost: bool,
}

impl SecurityOriginDetails {
    /// Creates a builder for this type with the required parameters:
    /// * `is_localhost`: Indicates whether the frame document's security origin is one of the local hostnames (e.g. "localhost") or IP addresses (IPv4 127.0.0.0/8 or IPv6 ::1).
    pub fn builder(is_localhost: bool) -> SecurityOriginDetailsBuilder {
        SecurityOriginDetailsBuilder {
            is_localhost: is_localhost,
        }
    }
    /// Indicates whether the frame document's security origin is one
    /// of the local hostnames (e.g. "localhost") or IP addresses (IPv4
    /// 127.0.0.0/8 or IPv6 ::1).
    pub fn is_localhost(&self) -> bool { self.is_localhost }
}


pub struct SecurityOriginDetailsBuilder {
    is_localhost: bool,
}

impl SecurityOriginDetailsBuilder {
    pub fn build(self) -> SecurityOriginDetails {
        SecurityOriginDetails {
            is_localhost: self.is_localhost,
        }
    }
}

/// Information about the Frame on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Frame<'a> {
    /// Frame unique identifier.
    id: FrameId<'a>,
    /// Parent frame identifier.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    parent_id: Option<FrameId<'a>>,
    /// Identifier of the loader associated with this frame.
    #[serde(rename = "loaderId")]
    loader_id: crate::network::LoaderId<'a>,
    /// Frame's name as specified in the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    /// Frame document's URL without fragment.
    url: Cow<'a, str>,
    /// Frame document's URL fragment including the '#'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlFragment")]
    url_fragment: Option<Cow<'a, str>>,
    /// Frame document's registered domain, taking the public suffixes list into account.
    /// Extracted from the Frame's url.
    /// Example URLs: <http://www.google.com/file.html> -\> "google.com"
    /// <http://a.b.co.uk/file.html>      -\> "b.co.uk"
    #[serde(rename = "domainAndRegistry")]
    domain_and_registry: Cow<'a, str>,
    /// Frame document's security origin.
    #[serde(rename = "securityOrigin")]
    security_origin: Cow<'a, str>,
    /// Additional details about the frame document's security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOriginDetails")]
    security_origin_details: Option<SecurityOriginDetails>,
    /// Frame document's mimeType as determined by the browser.
    #[serde(rename = "mimeType")]
    mime_type: Cow<'a, str>,
    /// If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.
    #[serde(skip_serializing_if = "Option::is_none", rename = "unreachableUrl")]
    unreachable_url: Option<Cow<'a, str>>,
    /// Indicates whether this frame was tagged as an ad and why.
    #[serde(skip_serializing_if = "Option::is_none", rename = "adFrameStatus")]
    ad_frame_status: Option<AdFrameStatus>,
    /// Indicates whether the main document is a secure context and explains why that is the case.
    #[serde(rename = "secureContextType")]
    secure_context_type: SecureContextType,
    /// Indicates whether this is a cross origin isolated context.
    #[serde(rename = "crossOriginIsolatedContextType")]
    cross_origin_isolated_context_type: CrossOriginIsolatedContextType,
    /// Indicated which gated APIs / features are available.
    #[serde(rename = "gatedAPIFeatures")]
    gated_api_features: Vec<GatedAPIFeatures>,
}

impl<'a> Frame<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: Frame unique identifier.
    /// * `loader_id`: Identifier of the loader associated with this frame.
    /// * `url`: Frame document's URL without fragment.
    /// * `domain_and_registry`: Frame document's registered domain, taking the public suffixes list into account. Extracted from the Frame's url. Example URLs: <http://www.google.com/file.html> -\> "google.com"               <http://a.b.co.uk/file.html>      -\> "b.co.uk"
    /// * `security_origin`: Frame document's security origin.
    /// * `mime_type`: Frame document's mimeType as determined by the browser.
    /// * `secure_context_type`: Indicates whether the main document is a secure context and explains why that is the case.
    /// * `cross_origin_isolated_context_type`: Indicates whether this is a cross origin isolated context.
    /// * `gated_api_features`: Indicated which gated APIs / features are available.
    pub fn builder(id: impl Into<FrameId<'a>>, loader_id: crate::network::LoaderId<'a>, url: impl Into<Cow<'a, str>>, domain_and_registry: impl Into<Cow<'a, str>>, security_origin: impl Into<Cow<'a, str>>, mime_type: impl Into<Cow<'a, str>>, secure_context_type: impl Into<SecureContextType>, cross_origin_isolated_context_type: impl Into<CrossOriginIsolatedContextType>, gated_api_features: Vec<GatedAPIFeatures>) -> FrameBuilder<'a> {
        FrameBuilder {
            id: id.into(),
            parent_id: None,
            loader_id: loader_id,
            name: None,
            url: url.into(),
            url_fragment: None,
            domain_and_registry: domain_and_registry.into(),
            security_origin: security_origin.into(),
            security_origin_details: None,
            mime_type: mime_type.into(),
            unreachable_url: None,
            ad_frame_status: None,
            secure_context_type: secure_context_type.into(),
            cross_origin_isolated_context_type: cross_origin_isolated_context_type.into(),
            gated_api_features: gated_api_features,
        }
    }
    /// Frame unique identifier.
    pub fn id(&self) -> &FrameId<'a> { &self.id }
    /// Parent frame identifier.
    pub fn parent_id(&self) -> Option<&FrameId<'a>> { self.parent_id.as_ref() }
    /// Identifier of the loader associated with this frame.
    pub fn loader_id(&self) -> &crate::network::LoaderId<'a> { &self.loader_id }
    /// Frame's name as specified in the tag.
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    /// Frame document's URL without fragment.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Frame document's URL fragment including the '#'.
    pub fn url_fragment(&self) -> Option<&str> { self.url_fragment.as_deref() }
    /// Frame document's registered domain, taking the public suffixes list into account.
    /// Extracted from the Frame's url.
    /// Example URLs: <http://www.google.com/file.html> -\> "google.com"
    /// <http://a.b.co.uk/file.html>      -\> "b.co.uk"
    pub fn domain_and_registry(&self) -> &str { self.domain_and_registry.as_ref() }
    /// Frame document's security origin.
    pub fn security_origin(&self) -> &str { self.security_origin.as_ref() }
    /// Additional details about the frame document's security origin.
    pub fn security_origin_details(&self) -> Option<&SecurityOriginDetails> { self.security_origin_details.as_ref() }
    /// Frame document's mimeType as determined by the browser.
    pub fn mime_type(&self) -> &str { self.mime_type.as_ref() }
    /// If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.
    pub fn unreachable_url(&self) -> Option<&str> { self.unreachable_url.as_deref() }
    /// Indicates whether this frame was tagged as an ad and why.
    pub fn ad_frame_status(&self) -> Option<&AdFrameStatus> { self.ad_frame_status.as_ref() }
    /// Indicates whether the main document is a secure context and explains why that is the case.
    pub fn secure_context_type(&self) -> &SecureContextType { &self.secure_context_type }
    /// Indicates whether this is a cross origin isolated context.
    pub fn cross_origin_isolated_context_type(&self) -> &CrossOriginIsolatedContextType { &self.cross_origin_isolated_context_type }
    /// Indicated which gated APIs / features are available.
    pub fn gated_api_features(&self) -> &[GatedAPIFeatures] { &self.gated_api_features }
}


pub struct FrameBuilder<'a> {
    id: FrameId<'a>,
    parent_id: Option<FrameId<'a>>,
    loader_id: crate::network::LoaderId<'a>,
    name: Option<Cow<'a, str>>,
    url: Cow<'a, str>,
    url_fragment: Option<Cow<'a, str>>,
    domain_and_registry: Cow<'a, str>,
    security_origin: Cow<'a, str>,
    security_origin_details: Option<SecurityOriginDetails>,
    mime_type: Cow<'a, str>,
    unreachable_url: Option<Cow<'a, str>>,
    ad_frame_status: Option<AdFrameStatus>,
    secure_context_type: SecureContextType,
    cross_origin_isolated_context_type: CrossOriginIsolatedContextType,
    gated_api_features: Vec<GatedAPIFeatures>,
}

impl<'a> FrameBuilder<'a> {
    /// Parent frame identifier.
    pub fn parent_id(mut self, parent_id: impl Into<FrameId<'a>>) -> Self { self.parent_id = Some(parent_id.into()); self }
    /// Frame's name as specified in the tag.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Frame document's URL fragment including the '#'.
    pub fn url_fragment(mut self, url_fragment: impl Into<Cow<'a, str>>) -> Self { self.url_fragment = Some(url_fragment.into()); self }
    /// Additional details about the frame document's security origin.
    pub fn security_origin_details(mut self, security_origin_details: SecurityOriginDetails) -> Self { self.security_origin_details = Some(security_origin_details); self }
    /// If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.
    pub fn unreachable_url(mut self, unreachable_url: impl Into<Cow<'a, str>>) -> Self { self.unreachable_url = Some(unreachable_url.into()); self }
    /// Indicates whether this frame was tagged as an ad and why.
    pub fn ad_frame_status(mut self, ad_frame_status: AdFrameStatus) -> Self { self.ad_frame_status = Some(ad_frame_status); self }
    pub fn build(self) -> Frame<'a> {
        Frame {
            id: self.id,
            parent_id: self.parent_id,
            loader_id: self.loader_id,
            name: self.name,
            url: self.url,
            url_fragment: self.url_fragment,
            domain_and_registry: self.domain_and_registry,
            security_origin: self.security_origin,
            security_origin_details: self.security_origin_details,
            mime_type: self.mime_type,
            unreachable_url: self.unreachable_url,
            ad_frame_status: self.ad_frame_status,
            secure_context_type: self.secure_context_type,
            cross_origin_isolated_context_type: self.cross_origin_isolated_context_type,
            gated_api_features: self.gated_api_features,
        }
    }
}

/// Information about the Resource on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FrameResource<'a> {
    /// Resource URL.
    url: Cow<'a, str>,
    /// Type of this resource.
    #[serde(rename = "type")]
    type_: crate::network::ResourceType,
    /// Resource mimeType as determined by the browser.
    #[serde(rename = "mimeType")]
    mime_type: Cow<'a, str>,
    /// last-modified timestamp as reported by server.
    #[serde(skip_serializing_if = "Option::is_none", rename = "lastModified")]
    last_modified: Option<crate::network::TimeSinceEpoch>,
    /// Resource content size.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentSize")]
    content_size: Option<f64>,
    /// True if the resource failed to load.
    #[serde(skip_serializing_if = "Option::is_none")]
    failed: Option<bool>,
    /// True if the resource was canceled during loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    canceled: Option<bool>,
}

impl<'a> FrameResource<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: Resource URL.
    /// * `type_`: Type of this resource.
    /// * `mime_type`: Resource mimeType as determined by the browser.
    pub fn builder(url: impl Into<Cow<'a, str>>, type_: crate::network::ResourceType, mime_type: impl Into<Cow<'a, str>>) -> FrameResourceBuilder<'a> {
        FrameResourceBuilder {
            url: url.into(),
            type_: type_,
            mime_type: mime_type.into(),
            last_modified: None,
            content_size: None,
            failed: None,
            canceled: None,
        }
    }
    /// Resource URL.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Type of this resource.
    pub fn type_(&self) -> &crate::network::ResourceType { &self.type_ }
    /// Resource mimeType as determined by the browser.
    pub fn mime_type(&self) -> &str { self.mime_type.as_ref() }
    /// last-modified timestamp as reported by server.
    pub fn last_modified(&self) -> Option<&crate::network::TimeSinceEpoch> { self.last_modified.as_ref() }
    /// Resource content size.
    pub fn content_size(&self) -> Option<f64> { self.content_size }
    /// True if the resource failed to load.
    pub fn failed(&self) -> Option<bool> { self.failed }
    /// True if the resource was canceled during loading.
    pub fn canceled(&self) -> Option<bool> { self.canceled }
}


pub struct FrameResourceBuilder<'a> {
    url: Cow<'a, str>,
    type_: crate::network::ResourceType,
    mime_type: Cow<'a, str>,
    last_modified: Option<crate::network::TimeSinceEpoch>,
    content_size: Option<f64>,
    failed: Option<bool>,
    canceled: Option<bool>,
}

impl<'a> FrameResourceBuilder<'a> {
    /// last-modified timestamp as reported by server.
    pub fn last_modified(mut self, last_modified: crate::network::TimeSinceEpoch) -> Self { self.last_modified = Some(last_modified); self }
    /// Resource content size.
    pub fn content_size(mut self, content_size: f64) -> Self { self.content_size = Some(content_size); self }
    /// True if the resource failed to load.
    pub fn failed(mut self, failed: bool) -> Self { self.failed = Some(failed); self }
    /// True if the resource was canceled during loading.
    pub fn canceled(mut self, canceled: bool) -> Self { self.canceled = Some(canceled); self }
    pub fn build(self) -> FrameResource<'a> {
        FrameResource {
            url: self.url,
            type_: self.type_,
            mime_type: self.mime_type,
            last_modified: self.last_modified,
            content_size: self.content_size,
            failed: self.failed,
            canceled: self.canceled,
        }
    }
}

/// Information about the Frame hierarchy along with their cached resources.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FrameResourceTree<'a> {
    /// Frame information for this tree item.
    frame: Frame<'a>,
    /// Child frames.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childFrames")]
    child_frames: Option<Vec<Box<FrameResourceTree<'a>>>>,
    /// Information about frame resources.
    resources: Vec<FrameResource<'a>>,
}

impl<'a> FrameResourceTree<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame`: Frame information for this tree item.
    /// * `resources`: Information about frame resources.
    pub fn builder(frame: Frame<'a>, resources: Vec<FrameResource<'a>>) -> FrameResourceTreeBuilder<'a> {
        FrameResourceTreeBuilder {
            frame: frame,
            child_frames: None,
            resources: resources,
        }
    }
    /// Frame information for this tree item.
    pub fn frame(&self) -> &Frame<'a> { &self.frame }
    /// Child frames.
    pub fn child_frames(&self) -> Option<&[Box<FrameResourceTree<'a>>]> { self.child_frames.as_deref() }
    /// Information about frame resources.
    pub fn resources(&self) -> &[FrameResource<'a>] { &self.resources }
}


pub struct FrameResourceTreeBuilder<'a> {
    frame: Frame<'a>,
    child_frames: Option<Vec<Box<FrameResourceTree<'a>>>>,
    resources: Vec<FrameResource<'a>>,
}

impl<'a> FrameResourceTreeBuilder<'a> {
    /// Child frames.
    pub fn child_frames(mut self, child_frames: Vec<Box<FrameResourceTree<'a>>>) -> Self { self.child_frames = Some(child_frames); self }
    pub fn build(self) -> FrameResourceTree<'a> {
        FrameResourceTree {
            frame: self.frame,
            child_frames: self.child_frames,
            resources: self.resources,
        }
    }
}

/// Information about the Frame hierarchy.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FrameTree<'a> {
    /// Frame information for this tree item.
    frame: Frame<'a>,
    /// Child frames.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childFrames")]
    child_frames: Option<Vec<Box<FrameTree<'a>>>>,
}

impl<'a> FrameTree<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame`: Frame information for this tree item.
    pub fn builder(frame: Frame<'a>) -> FrameTreeBuilder<'a> {
        FrameTreeBuilder {
            frame: frame,
            child_frames: None,
        }
    }
    /// Frame information for this tree item.
    pub fn frame(&self) -> &Frame<'a> { &self.frame }
    /// Child frames.
    pub fn child_frames(&self) -> Option<&[Box<FrameTree<'a>>]> { self.child_frames.as_deref() }
}


pub struct FrameTreeBuilder<'a> {
    frame: Frame<'a>,
    child_frames: Option<Vec<Box<FrameTree<'a>>>>,
}

impl<'a> FrameTreeBuilder<'a> {
    /// Child frames.
    pub fn child_frames(mut self, child_frames: Vec<Box<FrameTree<'a>>>) -> Self { self.child_frames = Some(child_frames); self }
    pub fn build(self) -> FrameTree<'a> {
        FrameTree {
            frame: self.frame,
            child_frames: self.child_frames,
        }
    }
}

/// Unique script identifier.

pub type ScriptIdentifier<'a> = Cow<'a, str>;

/// Transition type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TransitionType {
    #[default]
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "typed")]
    Typed,
    #[serde(rename = "address_bar")]
    AddressBar,
    #[serde(rename = "auto_bookmark")]
    AutoBookmark,
    #[serde(rename = "auto_subframe")]
    AutoSubframe,
    #[serde(rename = "manual_subframe")]
    ManualSubframe,
    #[serde(rename = "generated")]
    Generated,
    #[serde(rename = "auto_toplevel")]
    AutoToplevel,
    #[serde(rename = "form_submit")]
    FormSubmit,
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "keyword")]
    Keyword,
    #[serde(rename = "keyword_generated")]
    KeywordGenerated,
    #[serde(rename = "other")]
    Other,
}

/// Navigation history entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEntry<'a> {
    /// Unique id of the navigation history entry.
    id: u64,
    /// URL of the navigation history entry.
    url: Cow<'a, str>,
    /// URL that the user typed in the url bar.
    #[serde(rename = "userTypedURL")]
    user_typed_url: Cow<'a, str>,
    /// Title of the navigation history entry.
    title: Cow<'a, str>,
    /// Transition type.
    #[serde(rename = "transitionType")]
    transition_type: TransitionType,
}

impl<'a> NavigationEntry<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: Unique id of the navigation history entry.
    /// * `url`: URL of the navigation history entry.
    /// * `user_typed_url`: URL that the user typed in the url bar.
    /// * `title`: Title of the navigation history entry.
    /// * `transition_type`: Transition type.
    pub fn builder(id: u64, url: impl Into<Cow<'a, str>>, user_typed_url: impl Into<Cow<'a, str>>, title: impl Into<Cow<'a, str>>, transition_type: impl Into<TransitionType>) -> NavigationEntryBuilder<'a> {
        NavigationEntryBuilder {
            id: id,
            url: url.into(),
            user_typed_url: user_typed_url.into(),
            title: title.into(),
            transition_type: transition_type.into(),
        }
    }
    /// Unique id of the navigation history entry.
    pub fn id(&self) -> u64 { self.id }
    /// URL of the navigation history entry.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// URL that the user typed in the url bar.
    pub fn user_typed_url(&self) -> &str { self.user_typed_url.as_ref() }
    /// Title of the navigation history entry.
    pub fn title(&self) -> &str { self.title.as_ref() }
    /// Transition type.
    pub fn transition_type(&self) -> &TransitionType { &self.transition_type }
}


pub struct NavigationEntryBuilder<'a> {
    id: u64,
    url: Cow<'a, str>,
    user_typed_url: Cow<'a, str>,
    title: Cow<'a, str>,
    transition_type: TransitionType,
}

impl<'a> NavigationEntryBuilder<'a> {
    pub fn build(self) -> NavigationEntry<'a> {
        NavigationEntry {
            id: self.id,
            url: self.url,
            user_typed_url: self.user_typed_url,
            title: self.title,
            transition_type: self.transition_type,
        }
    }
}

/// Screencast frame metadata.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameMetadata {
    /// Top offset in DIP.
    #[serde(rename = "offsetTop")]
    offset_top: f64,
    /// Page scale factor.
    #[serde(rename = "pageScaleFactor")]
    page_scale_factor: f64,
    /// Device screen width in DIP.
    #[serde(rename = "deviceWidth")]
    device_width: f64,
    /// Device screen height in DIP.
    #[serde(rename = "deviceHeight")]
    device_height: f64,
    /// Position of horizontal scroll in CSS pixels.
    #[serde(rename = "scrollOffsetX")]
    scroll_offset_x: f64,
    /// Position of vertical scroll in CSS pixels.
    #[serde(rename = "scrollOffsetY")]
    scroll_offset_y: f64,
    /// Frame swap timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::network::TimeSinceEpoch>,
}

impl ScreencastFrameMetadata {
    /// Creates a builder for this type with the required parameters:
    /// * `offset_top`: Top offset in DIP.
    /// * `page_scale_factor`: Page scale factor.
    /// * `device_width`: Device screen width in DIP.
    /// * `device_height`: Device screen height in DIP.
    /// * `scroll_offset_x`: Position of horizontal scroll in CSS pixels.
    /// * `scroll_offset_y`: Position of vertical scroll in CSS pixels.
    pub fn builder(offset_top: f64, page_scale_factor: f64, device_width: f64, device_height: f64, scroll_offset_x: f64, scroll_offset_y: f64) -> ScreencastFrameMetadataBuilder {
        ScreencastFrameMetadataBuilder {
            offset_top: offset_top,
            page_scale_factor: page_scale_factor,
            device_width: device_width,
            device_height: device_height,
            scroll_offset_x: scroll_offset_x,
            scroll_offset_y: scroll_offset_y,
            timestamp: None,
        }
    }
    /// Top offset in DIP.
    pub fn offset_top(&self) -> f64 { self.offset_top }
    /// Page scale factor.
    pub fn page_scale_factor(&self) -> f64 { self.page_scale_factor }
    /// Device screen width in DIP.
    pub fn device_width(&self) -> f64 { self.device_width }
    /// Device screen height in DIP.
    pub fn device_height(&self) -> f64 { self.device_height }
    /// Position of horizontal scroll in CSS pixels.
    pub fn scroll_offset_x(&self) -> f64 { self.scroll_offset_x }
    /// Position of vertical scroll in CSS pixels.
    pub fn scroll_offset_y(&self) -> f64 { self.scroll_offset_y }
    /// Frame swap timestamp.
    pub fn timestamp(&self) -> Option<&crate::network::TimeSinceEpoch> { self.timestamp.as_ref() }
}


pub struct ScreencastFrameMetadataBuilder {
    offset_top: f64,
    page_scale_factor: f64,
    device_width: f64,
    device_height: f64,
    scroll_offset_x: f64,
    scroll_offset_y: f64,
    timestamp: Option<crate::network::TimeSinceEpoch>,
}

impl ScreencastFrameMetadataBuilder {
    /// Frame swap timestamp.
    pub fn timestamp(mut self, timestamp: crate::network::TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    pub fn build(self) -> ScreencastFrameMetadata {
        ScreencastFrameMetadata {
            offset_top: self.offset_top,
            page_scale_factor: self.page_scale_factor,
            device_width: self.device_width,
            device_height: self.device_height,
            scroll_offset_x: self.scroll_offset_x,
            scroll_offset_y: self.scroll_offset_y,
            timestamp: self.timestamp,
        }
    }
}

/// Javascript dialog type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DialogType {
    #[default]
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "confirm")]
    Confirm,
    #[serde(rename = "prompt")]
    Prompt,
    #[serde(rename = "beforeunload")]
    Beforeunload,
}

/// Error while paring app manifest.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppManifestError<'a> {
    /// Error message.
    message: Cow<'a, str>,
    /// If critical, this is a non-recoverable parse error.
    critical: i64,
    /// Error line.
    line: i64,
    /// Error column.
    column: i64,
}

impl<'a> AppManifestError<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `message`: Error message.
    /// * `critical`: If critical, this is a non-recoverable parse error.
    /// * `line`: Error line.
    /// * `column`: Error column.
    pub fn builder(message: impl Into<Cow<'a, str>>, critical: i64, line: i64, column: i64) -> AppManifestErrorBuilder<'a> {
        AppManifestErrorBuilder {
            message: message.into(),
            critical: critical,
            line: line,
            column: column,
        }
    }
    /// Error message.
    pub fn message(&self) -> &str { self.message.as_ref() }
    /// If critical, this is a non-recoverable parse error.
    pub fn critical(&self) -> i64 { self.critical }
    /// Error line.
    pub fn line(&self) -> i64 { self.line }
    /// Error column.
    pub fn column(&self) -> i64 { self.column }
}


pub struct AppManifestErrorBuilder<'a> {
    message: Cow<'a, str>,
    critical: i64,
    line: i64,
    column: i64,
}

impl<'a> AppManifestErrorBuilder<'a> {
    pub fn build(self) -> AppManifestError<'a> {
        AppManifestError {
            message: self.message,
            critical: self.critical,
            line: self.line,
            column: self.column,
        }
    }
}

/// Parsed app manifest properties.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppManifestParsedProperties<'a> {
    /// Computed scope value
    scope: Cow<'a, str>,
}

impl<'a> AppManifestParsedProperties<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `scope`: Computed scope value
    pub fn builder(scope: impl Into<Cow<'a, str>>) -> AppManifestParsedPropertiesBuilder<'a> {
        AppManifestParsedPropertiesBuilder {
            scope: scope.into(),
        }
    }
    /// Computed scope value
    pub fn scope(&self) -> &str { self.scope.as_ref() }
}


pub struct AppManifestParsedPropertiesBuilder<'a> {
    scope: Cow<'a, str>,
}

impl<'a> AppManifestParsedPropertiesBuilder<'a> {
    pub fn build(self) -> AppManifestParsedProperties<'a> {
        AppManifestParsedProperties {
            scope: self.scope,
        }
    }
}

/// Layout viewport position and dimensions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutViewport {
    /// Horizontal offset relative to the document (CSS pixels).
    #[serde(rename = "pageX")]
    page_x: i64,
    /// Vertical offset relative to the document (CSS pixels).
    #[serde(rename = "pageY")]
    page_y: i64,
    /// Width (CSS pixels), excludes scrollbar if present.
    #[serde(rename = "clientWidth")]
    client_width: u64,
    /// Height (CSS pixels), excludes scrollbar if present.
    #[serde(rename = "clientHeight")]
    client_height: i64,
}

impl LayoutViewport {
    /// Creates a builder for this type with the required parameters:
    /// * `page_x`: Horizontal offset relative to the document (CSS pixels).
    /// * `page_y`: Vertical offset relative to the document (CSS pixels).
    /// * `client_width`: Width (CSS pixels), excludes scrollbar if present.
    /// * `client_height`: Height (CSS pixels), excludes scrollbar if present.
    pub fn builder(page_x: i64, page_y: i64, client_width: u64, client_height: i64) -> LayoutViewportBuilder {
        LayoutViewportBuilder {
            page_x: page_x,
            page_y: page_y,
            client_width: client_width,
            client_height: client_height,
        }
    }
    /// Horizontal offset relative to the document (CSS pixels).
    pub fn page_x(&self) -> i64 { self.page_x }
    /// Vertical offset relative to the document (CSS pixels).
    pub fn page_y(&self) -> i64 { self.page_y }
    /// Width (CSS pixels), excludes scrollbar if present.
    pub fn client_width(&self) -> u64 { self.client_width }
    /// Height (CSS pixels), excludes scrollbar if present.
    pub fn client_height(&self) -> i64 { self.client_height }
}


pub struct LayoutViewportBuilder {
    page_x: i64,
    page_y: i64,
    client_width: u64,
    client_height: i64,
}

impl LayoutViewportBuilder {
    pub fn build(self) -> LayoutViewport {
        LayoutViewport {
            page_x: self.page_x,
            page_y: self.page_y,
            client_width: self.client_width,
            client_height: self.client_height,
        }
    }
}

/// Visual viewport position, dimensions, and scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VisualViewport {
    /// Horizontal offset relative to the layout viewport (CSS pixels).
    #[serde(rename = "offsetX")]
    offset_x: f64,
    /// Vertical offset relative to the layout viewport (CSS pixels).
    #[serde(rename = "offsetY")]
    offset_y: f64,
    /// Horizontal offset relative to the document (CSS pixels).
    #[serde(rename = "pageX")]
    page_x: f64,
    /// Vertical offset relative to the document (CSS pixels).
    #[serde(rename = "pageY")]
    page_y: f64,
    /// Width (CSS pixels), excludes scrollbar if present.
    #[serde(rename = "clientWidth")]
    client_width: f64,
    /// Height (CSS pixels), excludes scrollbar if present.
    #[serde(rename = "clientHeight")]
    client_height: f64,
    /// Scale relative to the ideal viewport (size at width=device-width).
    scale: f64,
    /// Page zoom factor (CSS to device independent pixels ratio).
    #[serde(skip_serializing_if = "Option::is_none")]
    zoom: Option<f64>,
}

impl VisualViewport {
    /// Creates a builder for this type with the required parameters:
    /// * `offset_x`: Horizontal offset relative to the layout viewport (CSS pixels).
    /// * `offset_y`: Vertical offset relative to the layout viewport (CSS pixels).
    /// * `page_x`: Horizontal offset relative to the document (CSS pixels).
    /// * `page_y`: Vertical offset relative to the document (CSS pixels).
    /// * `client_width`: Width (CSS pixels), excludes scrollbar if present.
    /// * `client_height`: Height (CSS pixels), excludes scrollbar if present.
    /// * `scale`: Scale relative to the ideal viewport (size at width=device-width).
    pub fn builder(offset_x: f64, offset_y: f64, page_x: f64, page_y: f64, client_width: f64, client_height: f64, scale: f64) -> VisualViewportBuilder {
        VisualViewportBuilder {
            offset_x: offset_x,
            offset_y: offset_y,
            page_x: page_x,
            page_y: page_y,
            client_width: client_width,
            client_height: client_height,
            scale: scale,
            zoom: None,
        }
    }
    /// Horizontal offset relative to the layout viewport (CSS pixels).
    pub fn offset_x(&self) -> f64 { self.offset_x }
    /// Vertical offset relative to the layout viewport (CSS pixels).
    pub fn offset_y(&self) -> f64 { self.offset_y }
    /// Horizontal offset relative to the document (CSS pixels).
    pub fn page_x(&self) -> f64 { self.page_x }
    /// Vertical offset relative to the document (CSS pixels).
    pub fn page_y(&self) -> f64 { self.page_y }
    /// Width (CSS pixels), excludes scrollbar if present.
    pub fn client_width(&self) -> f64 { self.client_width }
    /// Height (CSS pixels), excludes scrollbar if present.
    pub fn client_height(&self) -> f64 { self.client_height }
    /// Scale relative to the ideal viewport (size at width=device-width).
    pub fn scale(&self) -> f64 { self.scale }
    /// Page zoom factor (CSS to device independent pixels ratio).
    pub fn zoom(&self) -> Option<f64> { self.zoom }
}


pub struct VisualViewportBuilder {
    offset_x: f64,
    offset_y: f64,
    page_x: f64,
    page_y: f64,
    client_width: f64,
    client_height: f64,
    scale: f64,
    zoom: Option<f64>,
}

impl VisualViewportBuilder {
    /// Page zoom factor (CSS to device independent pixels ratio).
    pub fn zoom(mut self, zoom: f64) -> Self { self.zoom = Some(zoom); self }
    pub fn build(self) -> VisualViewport {
        VisualViewport {
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            page_x: self.page_x,
            page_y: self.page_y,
            client_width: self.client_width,
            client_height: self.client_height,
            scale: self.scale,
            zoom: self.zoom,
        }
    }
}

/// Viewport for capturing screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Viewport {
    /// X offset in device independent pixels (dip).
    x: f64,
    /// Y offset in device independent pixels (dip).
    y: f64,
    /// Rectangle width in device independent pixels (dip).
    width: f64,
    /// Rectangle height in device independent pixels (dip).
    height: f64,
    /// Page scale factor.
    scale: f64,
}

impl Viewport {
    /// Creates a builder for this type with the required parameters:
    /// * `x`: X offset in device independent pixels (dip).
    /// * `y`: Y offset in device independent pixels (dip).
    /// * `width`: Rectangle width in device independent pixels (dip).
    /// * `height`: Rectangle height in device independent pixels (dip).
    /// * `scale`: Page scale factor.
    pub fn builder(x: f64, y: f64, width: f64, height: f64, scale: f64) -> ViewportBuilder {
        ViewportBuilder {
            x: x,
            y: y,
            width: width,
            height: height,
            scale: scale,
        }
    }
    /// X offset in device independent pixels (dip).
    pub fn x(&self) -> f64 { self.x }
    /// Y offset in device independent pixels (dip).
    pub fn y(&self) -> f64 { self.y }
    /// Rectangle width in device independent pixels (dip).
    pub fn width(&self) -> f64 { self.width }
    /// Rectangle height in device independent pixels (dip).
    pub fn height(&self) -> f64 { self.height }
    /// Page scale factor.
    pub fn scale(&self) -> f64 { self.scale }
}


pub struct ViewportBuilder {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    scale: f64,
}

impl ViewportBuilder {
    pub fn build(self) -> Viewport {
        Viewport {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            scale: self.scale,
        }
    }
}

/// Generic font families collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FontFamilies<'a> {
    /// The standard font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    standard: Option<Cow<'a, str>>,
    /// The fixed font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<Cow<'a, str>>,
    /// The serif font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    serif: Option<Cow<'a, str>>,
    /// The sansSerif font-family.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sansSerif")]
    sans_serif: Option<Cow<'a, str>>,
    /// The cursive font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    cursive: Option<Cow<'a, str>>,
    /// The fantasy font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    fantasy: Option<Cow<'a, str>>,
    /// The math font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    math: Option<Cow<'a, str>>,
}

impl<'a> FontFamilies<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> FontFamiliesBuilder<'a> {
        FontFamiliesBuilder {
            standard: None,
            fixed: None,
            serif: None,
            sans_serif: None,
            cursive: None,
            fantasy: None,
            math: None,
        }
    }
    /// The standard font-family.
    pub fn standard(&self) -> Option<&str> { self.standard.as_deref() }
    /// The fixed font-family.
    pub fn fixed(&self) -> Option<&str> { self.fixed.as_deref() }
    /// The serif font-family.
    pub fn serif(&self) -> Option<&str> { self.serif.as_deref() }
    /// The sansSerif font-family.
    pub fn sans_serif(&self) -> Option<&str> { self.sans_serif.as_deref() }
    /// The cursive font-family.
    pub fn cursive(&self) -> Option<&str> { self.cursive.as_deref() }
    /// The fantasy font-family.
    pub fn fantasy(&self) -> Option<&str> { self.fantasy.as_deref() }
    /// The math font-family.
    pub fn math(&self) -> Option<&str> { self.math.as_deref() }
}

#[derive(Default)]
pub struct FontFamiliesBuilder<'a> {
    standard: Option<Cow<'a, str>>,
    fixed: Option<Cow<'a, str>>,
    serif: Option<Cow<'a, str>>,
    sans_serif: Option<Cow<'a, str>>,
    cursive: Option<Cow<'a, str>>,
    fantasy: Option<Cow<'a, str>>,
    math: Option<Cow<'a, str>>,
}

impl<'a> FontFamiliesBuilder<'a> {
    /// The standard font-family.
    pub fn standard(mut self, standard: impl Into<Cow<'a, str>>) -> Self { self.standard = Some(standard.into()); self }
    /// The fixed font-family.
    pub fn fixed(mut self, fixed: impl Into<Cow<'a, str>>) -> Self { self.fixed = Some(fixed.into()); self }
    /// The serif font-family.
    pub fn serif(mut self, serif: impl Into<Cow<'a, str>>) -> Self { self.serif = Some(serif.into()); self }
    /// The sansSerif font-family.
    pub fn sans_serif(mut self, sans_serif: impl Into<Cow<'a, str>>) -> Self { self.sans_serif = Some(sans_serif.into()); self }
    /// The cursive font-family.
    pub fn cursive(mut self, cursive: impl Into<Cow<'a, str>>) -> Self { self.cursive = Some(cursive.into()); self }
    /// The fantasy font-family.
    pub fn fantasy(mut self, fantasy: impl Into<Cow<'a, str>>) -> Self { self.fantasy = Some(fantasy.into()); self }
    /// The math font-family.
    pub fn math(mut self, math: impl Into<Cow<'a, str>>) -> Self { self.math = Some(math.into()); self }
    pub fn build(self) -> FontFamilies<'a> {
        FontFamilies {
            standard: self.standard,
            fixed: self.fixed,
            serif: self.serif,
            sans_serif: self.sans_serif,
            cursive: self.cursive,
            fantasy: self.fantasy,
            math: self.math,
        }
    }
}

/// Font families collection for a script.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScriptFontFamilies<'a> {
    /// Name of the script which these font families are defined for.
    script: Cow<'a, str>,
    /// Generic font families collection for the script.
    #[serde(rename = "fontFamilies")]
    font_families: FontFamilies<'a>,
}

impl<'a> ScriptFontFamilies<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script`: Name of the script which these font families are defined for.
    /// * `font_families`: Generic font families collection for the script.
    pub fn builder(script: impl Into<Cow<'a, str>>, font_families: FontFamilies<'a>) -> ScriptFontFamiliesBuilder<'a> {
        ScriptFontFamiliesBuilder {
            script: script.into(),
            font_families: font_families,
        }
    }
    /// Name of the script which these font families are defined for.
    pub fn script(&self) -> &str { self.script.as_ref() }
    /// Generic font families collection for the script.
    pub fn font_families(&self) -> &FontFamilies<'a> { &self.font_families }
}


pub struct ScriptFontFamiliesBuilder<'a> {
    script: Cow<'a, str>,
    font_families: FontFamilies<'a>,
}

impl<'a> ScriptFontFamiliesBuilder<'a> {
    pub fn build(self) -> ScriptFontFamilies<'a> {
        ScriptFontFamilies {
            script: self.script,
            font_families: self.font_families,
        }
    }
}

/// Default font sizes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FontSizes {
    /// Default standard font size.
    #[serde(skip_serializing_if = "Option::is_none")]
    standard: Option<i64>,
    /// Default fixed font size.
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<i64>,
}

impl FontSizes {
    /// Creates a builder for this type.
    pub fn builder() -> FontSizesBuilder {
        FontSizesBuilder {
            standard: None,
            fixed: None,
        }
    }
    /// Default standard font size.
    pub fn standard(&self) -> Option<i64> { self.standard }
    /// Default fixed font size.
    pub fn fixed(&self) -> Option<i64> { self.fixed }
}

#[derive(Default)]
pub struct FontSizesBuilder {
    standard: Option<i64>,
    fixed: Option<i64>,
}

impl FontSizesBuilder {
    /// Default standard font size.
    pub fn standard(mut self, standard: i64) -> Self { self.standard = Some(standard); self }
    /// Default fixed font size.
    pub fn fixed(mut self, fixed: i64) -> Self { self.fixed = Some(fixed); self }
    pub fn build(self) -> FontSizes {
        FontSizes {
            standard: self.standard,
            fixed: self.fixed,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ClientNavigationReason {
    #[default]
    #[serde(rename = "anchorClick")]
    AnchorClick,
    #[serde(rename = "formSubmissionGet")]
    FormSubmissionGet,
    #[serde(rename = "formSubmissionPost")]
    FormSubmissionPost,
    #[serde(rename = "httpHeaderRefresh")]
    HttpHeaderRefresh,
    #[serde(rename = "initialFrameNavigation")]
    InitialFrameNavigation,
    #[serde(rename = "metaTagRefresh")]
    MetaTagRefresh,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "pageBlockInterstitial")]
    PageBlockInterstitial,
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "scriptInitiated")]
    ScriptInitiated,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ClientNavigationDisposition {
    #[default]
    #[serde(rename = "currentTab")]
    CurrentTab,
    #[serde(rename = "newTab")]
    NewTab,
    #[serde(rename = "newWindow")]
    NewWindow,
    #[serde(rename = "download")]
    Download,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityErrorArgument<'a> {
    /// Argument name (e.g. name:'minimum-icon-size-in-pixels').
    name: Cow<'a, str>,
    /// Argument value (e.g. value:'64').
    value: Cow<'a, str>,
}

impl<'a> InstallabilityErrorArgument<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Argument name (e.g. name:'minimum-icon-size-in-pixels').
    /// * `value`: Argument value (e.g. value:'64').
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> InstallabilityErrorArgumentBuilder<'a> {
        InstallabilityErrorArgumentBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    /// Argument name (e.g. name:'minimum-icon-size-in-pixels').
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Argument value (e.g. value:'64').
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct InstallabilityErrorArgumentBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> InstallabilityErrorArgumentBuilder<'a> {
    pub fn build(self) -> InstallabilityErrorArgument<'a> {
        InstallabilityErrorArgument {
            name: self.name,
            value: self.value,
        }
    }
}

/// The installability error

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityError<'a> {
    /// The error id (e.g. 'manifest-missing-suitable-icon').
    #[serde(rename = "errorId")]
    error_id: Cow<'a, str>,
    /// The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).
    #[serde(rename = "errorArguments")]
    error_arguments: Vec<InstallabilityErrorArgument<'a>>,
}

impl<'a> InstallabilityError<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `error_id`: The error id (e.g. 'manifest-missing-suitable-icon').
    /// * `error_arguments`: The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).
    pub fn builder(error_id: impl Into<Cow<'a, str>>, error_arguments: Vec<InstallabilityErrorArgument<'a>>) -> InstallabilityErrorBuilder<'a> {
        InstallabilityErrorBuilder {
            error_id: error_id.into(),
            error_arguments: error_arguments,
        }
    }
    /// The error id (e.g. 'manifest-missing-suitable-icon').
    pub fn error_id(&self) -> &str { self.error_id.as_ref() }
    /// The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).
    pub fn error_arguments(&self) -> &[InstallabilityErrorArgument<'a>] { &self.error_arguments }
}


pub struct InstallabilityErrorBuilder<'a> {
    error_id: Cow<'a, str>,
    error_arguments: Vec<InstallabilityErrorArgument<'a>>,
}

impl<'a> InstallabilityErrorBuilder<'a> {
    pub fn build(self) -> InstallabilityError<'a> {
        InstallabilityError {
            error_id: self.error_id,
            error_arguments: self.error_arguments,
        }
    }
}

/// The referring-policy used for the navigation.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ReferrerPolicy {
    #[default]
    #[serde(rename = "noReferrer")]
    NoReferrer,
    #[serde(rename = "noReferrerWhenDowngrade")]
    NoReferrerWhenDowngrade,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "originWhenCrossOrigin")]
    OriginWhenCrossOrigin,
    #[serde(rename = "sameOrigin")]
    SameOrigin,
    #[serde(rename = "strictOrigin")]
    StrictOrigin,
    #[serde(rename = "strictOriginWhenCrossOrigin")]
    StrictOriginWhenCrossOrigin,
    #[serde(rename = "unsafeUrl")]
    UnsafeUrl,
}

/// Per-script compilation cache parameters for 'Page.produceCompilationCache'

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompilationCacheParams<'a> {
    /// The URL of the script to produce a compilation cache entry for.
    url: Cow<'a, str>,
    /// A hint to the backend whether eager compilation is recommended.
    /// (the actual compilation mode used is upon backend discretion).
    #[serde(skip_serializing_if = "Option::is_none")]
    eager: Option<bool>,
}

impl<'a> CompilationCacheParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: The URL of the script to produce a compilation cache entry for.
    pub fn builder(url: impl Into<Cow<'a, str>>) -> CompilationCacheParamsBuilder<'a> {
        CompilationCacheParamsBuilder {
            url: url.into(),
            eager: None,
        }
    }
    /// The URL of the script to produce a compilation cache entry for.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// A hint to the backend whether eager compilation is recommended.
    /// (the actual compilation mode used is upon backend discretion).
    pub fn eager(&self) -> Option<bool> { self.eager }
}


pub struct CompilationCacheParamsBuilder<'a> {
    url: Cow<'a, str>,
    eager: Option<bool>,
}

impl<'a> CompilationCacheParamsBuilder<'a> {
    /// A hint to the backend whether eager compilation is recommended.
    /// (the actual compilation mode used is upon backend discretion).
    pub fn eager(mut self, eager: bool) -> Self { self.eager = Some(eager); self }
    pub fn build(self) -> CompilationCacheParams<'a> {
        CompilationCacheParams {
            url: self.url,
            eager: self.eager,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepts: Option<Vec<Cow<'a, str>>>,
}

impl<'a> FileFilter<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> FileFilterBuilder<'a> {
        FileFilterBuilder {
            name: None,
            accepts: None,
        }
    }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn accepts(&self) -> Option<&[Cow<'a, str>]> { self.accepts.as_deref() }
}

#[derive(Default)]
pub struct FileFilterBuilder<'a> {
    name: Option<Cow<'a, str>>,
    accepts: Option<Vec<Cow<'a, str>>>,
}

impl<'a> FileFilterBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn accepts(mut self, accepts: Vec<Cow<'a, str>>) -> Self { self.accepts = Some(accepts); self }
    pub fn build(self) -> FileFilter<'a> {
        FileFilter {
            name: self.name,
            accepts: self.accepts,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileHandler<'a> {
    action: Cow<'a, str>,
    name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icons: Option<Vec<ImageResource<'a>>>,
    /// Mimic a map, name is the key, accepts is the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    accepts: Option<Vec<FileFilter<'a>>>,
    /// Won't repeat the enums, using string for easy comparison. Same as the
    /// other enums below.
    #[serde(rename = "launchType")]
    launch_type: Cow<'a, str>,
}

impl<'a> FileHandler<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `action`: 
    /// * `name`: 
    /// * `launch_type`: Won't repeat the enums, using string for easy comparison. Same as the other enums below.
    pub fn builder(action: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, launch_type: impl Into<Cow<'a, str>>) -> FileHandlerBuilder<'a> {
        FileHandlerBuilder {
            action: action.into(),
            name: name.into(),
            icons: None,
            accepts: None,
            launch_type: launch_type.into(),
        }
    }
    pub fn action(&self) -> &str { self.action.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn icons(&self) -> Option<&[ImageResource<'a>]> { self.icons.as_deref() }
    /// Mimic a map, name is the key, accepts is the value.
    pub fn accepts(&self) -> Option<&[FileFilter<'a>]> { self.accepts.as_deref() }
    /// Won't repeat the enums, using string for easy comparison. Same as the
    /// other enums below.
    pub fn launch_type(&self) -> &str { self.launch_type.as_ref() }
}


pub struct FileHandlerBuilder<'a> {
    action: Cow<'a, str>,
    name: Cow<'a, str>,
    icons: Option<Vec<ImageResource<'a>>>,
    accepts: Option<Vec<FileFilter<'a>>>,
    launch_type: Cow<'a, str>,
}

impl<'a> FileHandlerBuilder<'a> {
    pub fn icons(mut self, icons: Vec<ImageResource<'a>>) -> Self { self.icons = Some(icons); self }
    /// Mimic a map, name is the key, accepts is the value.
    pub fn accepts(mut self, accepts: Vec<FileFilter<'a>>) -> Self { self.accepts = Some(accepts); self }
    pub fn build(self) -> FileHandler<'a> {
        FileHandler {
            action: self.action,
            name: self.name,
            icons: self.icons,
            accepts: self.accepts,
            launch_type: self.launch_type,
        }
    }
}

/// The image definition used in both icon and screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageResource<'a> {
    /// The src field in the definition, but changing to url in favor of
    /// consistency.
    url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sizes: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    type_: Option<Cow<'a, str>>,
}

impl<'a> ImageResource<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: The src field in the definition, but changing to url in favor of consistency.
    pub fn builder(url: impl Into<Cow<'a, str>>) -> ImageResourceBuilder<'a> {
        ImageResourceBuilder {
            url: url.into(),
            sizes: None,
            type_: None,
        }
    }
    /// The src field in the definition, but changing to url in favor of
    /// consistency.
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn sizes(&self) -> Option<&str> { self.sizes.as_deref() }
    pub fn type_(&self) -> Option<&str> { self.type_.as_deref() }
}


pub struct ImageResourceBuilder<'a> {
    url: Cow<'a, str>,
    sizes: Option<Cow<'a, str>>,
    type_: Option<Cow<'a, str>>,
}

impl<'a> ImageResourceBuilder<'a> {
    pub fn sizes(mut self, sizes: impl Into<Cow<'a, str>>) -> Self { self.sizes = Some(sizes.into()); self }
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    pub fn build(self) -> ImageResource<'a> {
        ImageResource {
            url: self.url,
            sizes: self.sizes,
            type_: self.type_,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchHandler<'a> {
    #[serde(rename = "clientMode")]
    client_mode: Cow<'a, str>,
}

impl<'a> LaunchHandler<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `client_mode`: 
    pub fn builder(client_mode: impl Into<Cow<'a, str>>) -> LaunchHandlerBuilder<'a> {
        LaunchHandlerBuilder {
            client_mode: client_mode.into(),
        }
    }
    pub fn client_mode(&self) -> &str { self.client_mode.as_ref() }
}


pub struct LaunchHandlerBuilder<'a> {
    client_mode: Cow<'a, str>,
}

impl<'a> LaunchHandlerBuilder<'a> {
    pub fn build(self) -> LaunchHandler<'a> {
        LaunchHandler {
            client_mode: self.client_mode,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolHandler<'a> {
    protocol: Cow<'a, str>,
    url: Cow<'a, str>,
}

impl<'a> ProtocolHandler<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `protocol`: 
    /// * `url`: 
    pub fn builder(protocol: impl Into<Cow<'a, str>>, url: impl Into<Cow<'a, str>>) -> ProtocolHandlerBuilder<'a> {
        ProtocolHandlerBuilder {
            protocol: protocol.into(),
            url: url.into(),
        }
    }
    pub fn protocol(&self) -> &str { self.protocol.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}


pub struct ProtocolHandlerBuilder<'a> {
    protocol: Cow<'a, str>,
    url: Cow<'a, str>,
}

impl<'a> ProtocolHandlerBuilder<'a> {
    pub fn build(self) -> ProtocolHandler<'a> {
        ProtocolHandler {
            protocol: self.protocol,
            url: self.url,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelatedApplication<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Cow<'a, str>>,
    url: Cow<'a, str>,
}

impl<'a> RelatedApplication<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: 
    pub fn builder(url: impl Into<Cow<'a, str>>) -> RelatedApplicationBuilder<'a> {
        RelatedApplicationBuilder {
            id: None,
            url: url.into(),
        }
    }
    pub fn id(&self) -> Option<&str> { self.id.as_deref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}


pub struct RelatedApplicationBuilder<'a> {
    id: Option<Cow<'a, str>>,
    url: Cow<'a, str>,
}

impl<'a> RelatedApplicationBuilder<'a> {
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    pub fn build(self) -> RelatedApplication<'a> {
        RelatedApplication {
            id: self.id,
            url: self.url,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScopeExtension<'a> {
    /// Instead of using tuple, this field always returns the serialized string
    /// for easy understanding and comparison.
    origin: Cow<'a, str>,
    #[serde(rename = "hasOriginWildcard")]
    has_origin_wildcard: bool,
}

impl<'a> ScopeExtension<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Instead of using tuple, this field always returns the serialized string for easy understanding and comparison.
    /// * `has_origin_wildcard`: 
    pub fn builder(origin: impl Into<Cow<'a, str>>, has_origin_wildcard: bool) -> ScopeExtensionBuilder<'a> {
        ScopeExtensionBuilder {
            origin: origin.into(),
            has_origin_wildcard: has_origin_wildcard,
        }
    }
    /// Instead of using tuple, this field always returns the serialized string
    /// for easy understanding and comparison.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn has_origin_wildcard(&self) -> bool { self.has_origin_wildcard }
}


pub struct ScopeExtensionBuilder<'a> {
    origin: Cow<'a, str>,
    has_origin_wildcard: bool,
}

impl<'a> ScopeExtensionBuilder<'a> {
    pub fn build(self) -> ScopeExtension<'a> {
        ScopeExtension {
            origin: self.origin,
            has_origin_wildcard: self.has_origin_wildcard,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Screenshot<'a> {
    image: ImageResource<'a>,
    #[serde(rename = "formFactor")]
    form_factor: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Cow<'a, str>>,
}

impl<'a> Screenshot<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `image`: 
    /// * `form_factor`: 
    pub fn builder(image: ImageResource<'a>, form_factor: impl Into<Cow<'a, str>>) -> ScreenshotBuilder<'a> {
        ScreenshotBuilder {
            image: image,
            form_factor: form_factor.into(),
            label: None,
        }
    }
    pub fn image(&self) -> &ImageResource<'a> { &self.image }
    pub fn form_factor(&self) -> &str { self.form_factor.as_ref() }
    pub fn label(&self) -> Option<&str> { self.label.as_deref() }
}


pub struct ScreenshotBuilder<'a> {
    image: ImageResource<'a>,
    form_factor: Cow<'a, str>,
    label: Option<Cow<'a, str>>,
}

impl<'a> ScreenshotBuilder<'a> {
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self { self.label = Some(label.into()); self }
    pub fn build(self) -> Screenshot<'a> {
        Screenshot {
            image: self.image,
            form_factor: self.form_factor,
            label: self.label,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShareTarget<'a> {
    action: Cow<'a, str>,
    method: Cow<'a, str>,
    enctype: Cow<'a, str>,
    /// Embed the ShareTargetParams
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<FileFilter<'a>>>,
}

impl<'a> ShareTarget<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `action`: 
    /// * `method`: 
    /// * `enctype`: 
    pub fn builder(action: impl Into<Cow<'a, str>>, method: impl Into<Cow<'a, str>>, enctype: impl Into<Cow<'a, str>>) -> ShareTargetBuilder<'a> {
        ShareTargetBuilder {
            action: action.into(),
            method: method.into(),
            enctype: enctype.into(),
            title: None,
            text: None,
            url: None,
            files: None,
        }
    }
    pub fn action(&self) -> &str { self.action.as_ref() }
    pub fn method(&self) -> &str { self.method.as_ref() }
    pub fn enctype(&self) -> &str { self.enctype.as_ref() }
    /// Embed the ShareTargetParams
    pub fn title(&self) -> Option<&str> { self.title.as_deref() }
    pub fn text(&self) -> Option<&str> { self.text.as_deref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn files(&self) -> Option<&[FileFilter<'a>]> { self.files.as_deref() }
}


pub struct ShareTargetBuilder<'a> {
    action: Cow<'a, str>,
    method: Cow<'a, str>,
    enctype: Cow<'a, str>,
    title: Option<Cow<'a, str>>,
    text: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    files: Option<Vec<FileFilter<'a>>>,
}

impl<'a> ShareTargetBuilder<'a> {
    /// Embed the ShareTargetParams
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self { self.title = Some(title.into()); self }
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn files(mut self, files: Vec<FileFilter<'a>>) -> Self { self.files = Some(files); self }
    pub fn build(self) -> ShareTarget<'a> {
        ShareTarget {
            action: self.action,
            method: self.method,
            enctype: self.enctype,
            title: self.title,
            text: self.text,
            url: self.url,
            files: self.files,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Shortcut<'a> {
    name: Cow<'a, str>,
    url: Cow<'a, str>,
}

impl<'a> Shortcut<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: 
    /// * `url`: 
    pub fn builder(name: impl Into<Cow<'a, str>>, url: impl Into<Cow<'a, str>>) -> ShortcutBuilder<'a> {
        ShortcutBuilder {
            name: name.into(),
            url: url.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}


pub struct ShortcutBuilder<'a> {
    name: Cow<'a, str>,
    url: Cow<'a, str>,
}

impl<'a> ShortcutBuilder<'a> {
    pub fn build(self) -> Shortcut<'a> {
        Shortcut {
            name: self.name,
            url: self.url,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebAppManifest<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "backgroundColor")]
    background_color: Option<Cow<'a, str>>,
    /// The extra description provided by the manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display: Option<Cow<'a, str>>,
    /// The overrided display mode controlled by the user.
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayOverrides")]
    display_overrides: Option<Vec<Cow<'a, str>>>,
    /// The handlers to open files.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fileHandlers")]
    file_handlers: Option<Vec<FileHandler<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icons: Option<Vec<ImageResource<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<Cow<'a, str>>,
    /// TODO(crbug.com/1231886): This field is non-standard and part of a Chrome
    /// experiment. See:
    /// <https://github.com/WICG/web-app-launch/blob/main/launch_handler.md>
    #[serde(skip_serializing_if = "Option::is_none", rename = "launchHandler")]
    launch_handler: Option<LaunchHandler<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "preferRelatedApplications")]
    prefer_related_applications: Option<bool>,
    /// The handlers to open protocols.
    #[serde(skip_serializing_if = "Option::is_none", rename = "protocolHandlers")]
    protocol_handlers: Option<Vec<ProtocolHandler<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "relatedApplications")]
    related_applications: Option<Vec<RelatedApplication<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Cow<'a, str>>,
    /// Non-standard, see
    /// <https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md>
    #[serde(skip_serializing_if = "Option::is_none", rename = "scopeExtensions")]
    scope_extensions: Option<Vec<ScopeExtension<'a>>>,
    /// The screenshots used by chromium.
    #[serde(skip_serializing_if = "Option::is_none")]
    screenshots: Option<Vec<Screenshot<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "shareTarget")]
    share_target: Option<ShareTarget<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "shortName")]
    short_name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shortcuts: Option<Vec<Shortcut<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startUrl")]
    start_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "themeColor")]
    theme_color: Option<Cow<'a, str>>,
}

impl<'a> WebAppManifest<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> WebAppManifestBuilder<'a> {
        WebAppManifestBuilder {
            background_color: None,
            description: None,
            dir: None,
            display: None,
            display_overrides: None,
            file_handlers: None,
            icons: None,
            id: None,
            lang: None,
            launch_handler: None,
            name: None,
            orientation: None,
            prefer_related_applications: None,
            protocol_handlers: None,
            related_applications: None,
            scope: None,
            scope_extensions: None,
            screenshots: None,
            share_target: None,
            short_name: None,
            shortcuts: None,
            start_url: None,
            theme_color: None,
        }
    }
    pub fn background_color(&self) -> Option<&str> { self.background_color.as_deref() }
    /// The extra description provided by the manifest.
    pub fn description(&self) -> Option<&str> { self.description.as_deref() }
    pub fn dir(&self) -> Option<&str> { self.dir.as_deref() }
    pub fn display(&self) -> Option<&str> { self.display.as_deref() }
    /// The overrided display mode controlled by the user.
    pub fn display_overrides(&self) -> Option<&[Cow<'a, str>]> { self.display_overrides.as_deref() }
    /// The handlers to open files.
    pub fn file_handlers(&self) -> Option<&[FileHandler<'a>]> { self.file_handlers.as_deref() }
    pub fn icons(&self) -> Option<&[ImageResource<'a>]> { self.icons.as_deref() }
    pub fn id(&self) -> Option<&str> { self.id.as_deref() }
    pub fn lang(&self) -> Option<&str> { self.lang.as_deref() }
    /// TODO(crbug.com/1231886): This field is non-standard and part of a Chrome
    /// experiment. See:
    /// <https://github.com/WICG/web-app-launch/blob/main/launch_handler.md>
    pub fn launch_handler(&self) -> Option<&LaunchHandler<'a>> { self.launch_handler.as_ref() }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn orientation(&self) -> Option<&str> { self.orientation.as_deref() }
    pub fn prefer_related_applications(&self) -> Option<bool> { self.prefer_related_applications }
    /// The handlers to open protocols.
    pub fn protocol_handlers(&self) -> Option<&[ProtocolHandler<'a>]> { self.protocol_handlers.as_deref() }
    pub fn related_applications(&self) -> Option<&[RelatedApplication<'a>]> { self.related_applications.as_deref() }
    pub fn scope(&self) -> Option<&str> { self.scope.as_deref() }
    /// Non-standard, see
    /// <https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md>
    pub fn scope_extensions(&self) -> Option<&[ScopeExtension<'a>]> { self.scope_extensions.as_deref() }
    /// The screenshots used by chromium.
    pub fn screenshots(&self) -> Option<&[Screenshot<'a>]> { self.screenshots.as_deref() }
    pub fn share_target(&self) -> Option<&ShareTarget<'a>> { self.share_target.as_ref() }
    pub fn short_name(&self) -> Option<&str> { self.short_name.as_deref() }
    pub fn shortcuts(&self) -> Option<&[Shortcut<'a>]> { self.shortcuts.as_deref() }
    pub fn start_url(&self) -> Option<&str> { self.start_url.as_deref() }
    pub fn theme_color(&self) -> Option<&str> { self.theme_color.as_deref() }
}

#[derive(Default)]
pub struct WebAppManifestBuilder<'a> {
    background_color: Option<Cow<'a, str>>,
    description: Option<Cow<'a, str>>,
    dir: Option<Cow<'a, str>>,
    display: Option<Cow<'a, str>>,
    display_overrides: Option<Vec<Cow<'a, str>>>,
    file_handlers: Option<Vec<FileHandler<'a>>>,
    icons: Option<Vec<ImageResource<'a>>>,
    id: Option<Cow<'a, str>>,
    lang: Option<Cow<'a, str>>,
    launch_handler: Option<LaunchHandler<'a>>,
    name: Option<Cow<'a, str>>,
    orientation: Option<Cow<'a, str>>,
    prefer_related_applications: Option<bool>,
    protocol_handlers: Option<Vec<ProtocolHandler<'a>>>,
    related_applications: Option<Vec<RelatedApplication<'a>>>,
    scope: Option<Cow<'a, str>>,
    scope_extensions: Option<Vec<ScopeExtension<'a>>>,
    screenshots: Option<Vec<Screenshot<'a>>>,
    share_target: Option<ShareTarget<'a>>,
    short_name: Option<Cow<'a, str>>,
    shortcuts: Option<Vec<Shortcut<'a>>>,
    start_url: Option<Cow<'a, str>>,
    theme_color: Option<Cow<'a, str>>,
}

impl<'a> WebAppManifestBuilder<'a> {
    pub fn background_color(mut self, background_color: impl Into<Cow<'a, str>>) -> Self { self.background_color = Some(background_color.into()); self }
    /// The extra description provided by the manifest.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self { self.description = Some(description.into()); self }
    pub fn dir(mut self, dir: impl Into<Cow<'a, str>>) -> Self { self.dir = Some(dir.into()); self }
    pub fn display(mut self, display: impl Into<Cow<'a, str>>) -> Self { self.display = Some(display.into()); self }
    /// The overrided display mode controlled by the user.
    pub fn display_overrides(mut self, display_overrides: Vec<Cow<'a, str>>) -> Self { self.display_overrides = Some(display_overrides); self }
    /// The handlers to open files.
    pub fn file_handlers(mut self, file_handlers: Vec<FileHandler<'a>>) -> Self { self.file_handlers = Some(file_handlers); self }
    pub fn icons(mut self, icons: Vec<ImageResource<'a>>) -> Self { self.icons = Some(icons); self }
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    pub fn lang(mut self, lang: impl Into<Cow<'a, str>>) -> Self { self.lang = Some(lang.into()); self }
    /// TODO(crbug.com/1231886): This field is non-standard and part of a Chrome
    /// experiment. See:
    /// <https://github.com/WICG/web-app-launch/blob/main/launch_handler.md>
    pub fn launch_handler(mut self, launch_handler: LaunchHandler<'a>) -> Self { self.launch_handler = Some(launch_handler); self }
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn orientation(mut self, orientation: impl Into<Cow<'a, str>>) -> Self { self.orientation = Some(orientation.into()); self }
    pub fn prefer_related_applications(mut self, prefer_related_applications: bool) -> Self { self.prefer_related_applications = Some(prefer_related_applications); self }
    /// The handlers to open protocols.
    pub fn protocol_handlers(mut self, protocol_handlers: Vec<ProtocolHandler<'a>>) -> Self { self.protocol_handlers = Some(protocol_handlers); self }
    pub fn related_applications(mut self, related_applications: Vec<RelatedApplication<'a>>) -> Self { self.related_applications = Some(related_applications); self }
    pub fn scope(mut self, scope: impl Into<Cow<'a, str>>) -> Self { self.scope = Some(scope.into()); self }
    /// Non-standard, see
    /// <https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md>
    pub fn scope_extensions(mut self, scope_extensions: Vec<ScopeExtension<'a>>) -> Self { self.scope_extensions = Some(scope_extensions); self }
    /// The screenshots used by chromium.
    pub fn screenshots(mut self, screenshots: Vec<Screenshot<'a>>) -> Self { self.screenshots = Some(screenshots); self }
    pub fn share_target(mut self, share_target: ShareTarget<'a>) -> Self { self.share_target = Some(share_target); self }
    pub fn short_name(mut self, short_name: impl Into<Cow<'a, str>>) -> Self { self.short_name = Some(short_name.into()); self }
    pub fn shortcuts(mut self, shortcuts: Vec<Shortcut<'a>>) -> Self { self.shortcuts = Some(shortcuts); self }
    pub fn start_url(mut self, start_url: impl Into<Cow<'a, str>>) -> Self { self.start_url = Some(start_url.into()); self }
    pub fn theme_color(mut self, theme_color: impl Into<Cow<'a, str>>) -> Self { self.theme_color = Some(theme_color.into()); self }
    pub fn build(self) -> WebAppManifest<'a> {
        WebAppManifest {
            background_color: self.background_color,
            description: self.description,
            dir: self.dir,
            display: self.display,
            display_overrides: self.display_overrides,
            file_handlers: self.file_handlers,
            icons: self.icons,
            id: self.id,
            lang: self.lang,
            launch_handler: self.launch_handler,
            name: self.name,
            orientation: self.orientation,
            prefer_related_applications: self.prefer_related_applications,
            protocol_handlers: self.protocol_handlers,
            related_applications: self.related_applications,
            scope: self.scope,
            scope_extensions: self.scope_extensions,
            screenshots: self.screenshots,
            share_target: self.share_target,
            short_name: self.short_name,
            shortcuts: self.shortcuts,
            start_url: self.start_url,
            theme_color: self.theme_color,
        }
    }
}

/// The type of a frameNavigated event.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum NavigationType {
    #[default]
    #[serde(rename = "Navigation")]
    Navigation,
    #[serde(rename = "BackForwardCacheRestore")]
    BackForwardCacheRestore,
}

/// List of not restored reasons for back-forward cache.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BackForwardCacheNotRestoredReason {
    #[default]
    #[serde(rename = "NotPrimaryMainFrame")]
    NotPrimaryMainFrame,
    #[serde(rename = "BackForwardCacheDisabled")]
    BackForwardCacheDisabled,
    #[serde(rename = "RelatedActiveContentsExist")]
    RelatedActiveContentsExist,
    #[serde(rename = "HTTPStatusNotOK")]
    HTTPStatusNotOK,
    #[serde(rename = "SchemeNotHTTPOrHTTPS")]
    SchemeNotHTTPOrHTTPS,
    #[serde(rename = "Loading")]
    Loading,
    #[serde(rename = "WasGrantedMediaAccess")]
    WasGrantedMediaAccess,
    #[serde(rename = "DisableForRenderFrameHostCalled")]
    DisableForRenderFrameHostCalled,
    #[serde(rename = "DomainNotAllowed")]
    DomainNotAllowed,
    #[serde(rename = "HTTPMethodNotGET")]
    HTTPMethodNotGET,
    #[serde(rename = "SubframeIsNavigating")]
    SubframeIsNavigating,
    #[serde(rename = "Timeout")]
    Timeout,
    #[serde(rename = "CacheLimit")]
    CacheLimit,
    #[serde(rename = "JavaScriptExecution")]
    JavaScriptExecution,
    #[serde(rename = "RendererProcessKilled")]
    RendererProcessKilled,
    #[serde(rename = "RendererProcessCrashed")]
    RendererProcessCrashed,
    #[serde(rename = "SchedulerTrackedFeatureUsed")]
    SchedulerTrackedFeatureUsed,
    #[serde(rename = "ConflictingBrowsingInstance")]
    ConflictingBrowsingInstance,
    #[serde(rename = "CacheFlushed")]
    CacheFlushed,
    #[serde(rename = "ServiceWorkerVersionActivation")]
    ServiceWorkerVersionActivation,
    #[serde(rename = "SessionRestored")]
    SessionRestored,
    #[serde(rename = "ServiceWorkerPostMessage")]
    ServiceWorkerPostMessage,
    #[serde(rename = "EnteredBackForwardCacheBeforeServiceWorkerHostAdded")]
    EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
    #[serde(rename = "RenderFrameHostReused_SameSite")]
    RenderFrameHostReusedSameSite,
    #[serde(rename = "RenderFrameHostReused_CrossSite")]
    RenderFrameHostReusedCrossSite,
    #[serde(rename = "ServiceWorkerClaim")]
    ServiceWorkerClaim,
    #[serde(rename = "IgnoreEventAndEvict")]
    IgnoreEventAndEvict,
    #[serde(rename = "HaveInnerContents")]
    HaveInnerContents,
    #[serde(rename = "TimeoutPuttingInCache")]
    TimeoutPuttingInCache,
    #[serde(rename = "BackForwardCacheDisabledByLowMemory")]
    BackForwardCacheDisabledByLowMemory,
    #[serde(rename = "BackForwardCacheDisabledByCommandLine")]
    BackForwardCacheDisabledByCommandLine,
    #[serde(rename = "NetworkRequestDatapipeDrainedAsBytesConsumer")]
    NetworkRequestDatapipeDrainedAsBytesConsumer,
    #[serde(rename = "NetworkRequestRedirected")]
    NetworkRequestRedirected,
    #[serde(rename = "NetworkRequestTimeout")]
    NetworkRequestTimeout,
    #[serde(rename = "NetworkExceedsBufferLimit")]
    NetworkExceedsBufferLimit,
    #[serde(rename = "NavigationCancelledWhileRestoring")]
    NavigationCancelledWhileRestoring,
    #[serde(rename = "NotMostRecentNavigationEntry")]
    NotMostRecentNavigationEntry,
    #[serde(rename = "BackForwardCacheDisabledForPrerender")]
    BackForwardCacheDisabledForPrerender,
    #[serde(rename = "UserAgentOverrideDiffers")]
    UserAgentOverrideDiffers,
    #[serde(rename = "ForegroundCacheLimit")]
    ForegroundCacheLimit,
    #[serde(rename = "ForwardCacheDisabled")]
    ForwardCacheDisabled,
    #[serde(rename = "BrowsingInstanceNotSwapped")]
    BrowsingInstanceNotSwapped,
    #[serde(rename = "BackForwardCacheDisabledForDelegate")]
    BackForwardCacheDisabledForDelegate,
    #[serde(rename = "UnloadHandlerExistsInMainFrame")]
    UnloadHandlerExistsInMainFrame,
    #[serde(rename = "UnloadHandlerExistsInSubFrame")]
    UnloadHandlerExistsInSubFrame,
    #[serde(rename = "ServiceWorkerUnregistration")]
    ServiceWorkerUnregistration,
    #[serde(rename = "CacheControlNoStore")]
    CacheControlNoStore,
    #[serde(rename = "CacheControlNoStoreCookieModified")]
    CacheControlNoStoreCookieModified,
    #[serde(rename = "CacheControlNoStoreHTTPOnlyCookieModified")]
    CacheControlNoStoreHTTPOnlyCookieModified,
    #[serde(rename = "NoResponseHead")]
    NoResponseHead,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "ActivationNavigationsDisallowedForBug1234857")]
    ActivationNavigationsDisallowedForBug1234857,
    #[serde(rename = "ErrorDocument")]
    ErrorDocument,
    #[serde(rename = "FencedFramesEmbedder")]
    FencedFramesEmbedder,
    #[serde(rename = "CookieDisabled")]
    CookieDisabled,
    #[serde(rename = "HTTPAuthRequired")]
    HTTPAuthRequired,
    #[serde(rename = "CookieFlushed")]
    CookieFlushed,
    #[serde(rename = "BroadcastChannelOnMessage")]
    BroadcastChannelOnMessage,
    #[serde(rename = "WebViewSettingsChanged")]
    WebViewSettingsChanged,
    #[serde(rename = "WebViewJavaScriptObjectChanged")]
    WebViewJavaScriptObjectChanged,
    #[serde(rename = "WebViewMessageListenerInjected")]
    WebViewMessageListenerInjected,
    #[serde(rename = "WebViewSafeBrowsingAllowlistChanged")]
    WebViewSafeBrowsingAllowlistChanged,
    #[serde(rename = "WebViewDocumentStartJavascriptChanged")]
    WebViewDocumentStartJavascriptChanged,
    #[serde(rename = "WebSocket")]
    WebSocket,
    #[serde(rename = "WebTransport")]
    WebTransport,
    #[serde(rename = "WebRTC")]
    WebRTC,
    #[serde(rename = "MainResourceHasCacheControlNoStore")]
    MainResourceHasCacheControlNoStore,
    #[serde(rename = "MainResourceHasCacheControlNoCache")]
    MainResourceHasCacheControlNoCache,
    #[serde(rename = "SubresourceHasCacheControlNoStore")]
    SubresourceHasCacheControlNoStore,
    #[serde(rename = "SubresourceHasCacheControlNoCache")]
    SubresourceHasCacheControlNoCache,
    #[serde(rename = "ContainsPlugins")]
    ContainsPlugins,
    #[serde(rename = "DocumentLoaded")]
    DocumentLoaded,
    #[serde(rename = "OutstandingNetworkRequestOthers")]
    OutstandingNetworkRequestOthers,
    #[serde(rename = "RequestedMIDIPermission")]
    RequestedMIDIPermission,
    #[serde(rename = "RequestedAudioCapturePermission")]
    RequestedAudioCapturePermission,
    #[serde(rename = "RequestedVideoCapturePermission")]
    RequestedVideoCapturePermission,
    #[serde(rename = "RequestedBackForwardCacheBlockedSensors")]
    RequestedBackForwardCacheBlockedSensors,
    #[serde(rename = "RequestedBackgroundWorkPermission")]
    RequestedBackgroundWorkPermission,
    #[serde(rename = "BroadcastChannel")]
    BroadcastChannel,
    #[serde(rename = "WebXR")]
    WebXR,
    #[serde(rename = "SharedWorker")]
    SharedWorker,
    #[serde(rename = "SharedWorkerMessage")]
    SharedWorkerMessage,
    #[serde(rename = "SharedWorkerWithNoActiveClient")]
    SharedWorkerWithNoActiveClient,
    #[serde(rename = "WebLocks")]
    WebLocks,
    #[serde(rename = "WebLocksContention")]
    WebLocksContention,
    #[serde(rename = "WebHID")]
    WebHID,
    #[serde(rename = "WebBluetooth")]
    WebBluetooth,
    #[serde(rename = "WebShare")]
    WebShare,
    #[serde(rename = "RequestedStorageAccessGrant")]
    RequestedStorageAccessGrant,
    #[serde(rename = "WebNfc")]
    WebNfc,
    #[serde(rename = "OutstandingNetworkRequestFetch")]
    OutstandingNetworkRequestFetch,
    #[serde(rename = "OutstandingNetworkRequestXHR")]
    OutstandingNetworkRequestXHR,
    #[serde(rename = "AppBanner")]
    AppBanner,
    #[serde(rename = "Printing")]
    Printing,
    #[serde(rename = "WebDatabase")]
    WebDatabase,
    #[serde(rename = "PictureInPicture")]
    PictureInPicture,
    #[serde(rename = "SpeechRecognizer")]
    SpeechRecognizer,
    #[serde(rename = "IdleManager")]
    IdleManager,
    #[serde(rename = "PaymentManager")]
    PaymentManager,
    #[serde(rename = "SpeechSynthesis")]
    SpeechSynthesis,
    #[serde(rename = "KeyboardLock")]
    KeyboardLock,
    #[serde(rename = "WebOTPService")]
    WebOTPService,
    #[serde(rename = "OutstandingNetworkRequestDirectSocket")]
    OutstandingNetworkRequestDirectSocket,
    #[serde(rename = "InjectedJavascript")]
    InjectedJavascript,
    #[serde(rename = "InjectedStyleSheet")]
    InjectedStyleSheet,
    #[serde(rename = "KeepaliveRequest")]
    KeepaliveRequest,
    #[serde(rename = "IndexedDBEvent")]
    IndexedDBEvent,
    #[serde(rename = "Dummy")]
    Dummy,
    #[serde(rename = "JsNetworkRequestReceivedCacheControlNoStoreResource")]
    JsNetworkRequestReceivedCacheControlNoStoreResource,
    #[serde(rename = "WebRTCUsedWithCCNS")]
    WebRTCUsedWithCCNS,
    #[serde(rename = "WebTransportUsedWithCCNS")]
    WebTransportUsedWithCCNS,
    #[serde(rename = "WebSocketUsedWithCCNS")]
    WebSocketUsedWithCCNS,
    #[serde(rename = "SmartCard")]
    SmartCard,
    #[serde(rename = "LiveMediaStreamTrack")]
    LiveMediaStreamTrack,
    #[serde(rename = "UnloadHandler")]
    UnloadHandler,
    #[serde(rename = "ParserAborted")]
    ParserAborted,
    #[serde(rename = "ContentSecurityHandler")]
    ContentSecurityHandler,
    #[serde(rename = "ContentWebAuthenticationAPI")]
    ContentWebAuthenticationAPI,
    #[serde(rename = "ContentFileChooser")]
    ContentFileChooser,
    #[serde(rename = "ContentSerial")]
    ContentSerial,
    #[serde(rename = "ContentFileSystemAccess")]
    ContentFileSystemAccess,
    #[serde(rename = "ContentMediaDevicesDispatcherHost")]
    ContentMediaDevicesDispatcherHost,
    #[serde(rename = "ContentWebBluetooth")]
    ContentWebBluetooth,
    #[serde(rename = "ContentWebUSB")]
    ContentWebUSB,
    #[serde(rename = "ContentMediaSessionService")]
    ContentMediaSessionService,
    #[serde(rename = "ContentScreenReader")]
    ContentScreenReader,
    #[serde(rename = "ContentDiscarded")]
    ContentDiscarded,
    #[serde(rename = "EmbedderPopupBlockerTabHelper")]
    EmbedderPopupBlockerTabHelper,
    #[serde(rename = "EmbedderSafeBrowsingTriggeredPopupBlocker")]
    EmbedderSafeBrowsingTriggeredPopupBlocker,
    #[serde(rename = "EmbedderSafeBrowsingThreatDetails")]
    EmbedderSafeBrowsingThreatDetails,
    #[serde(rename = "EmbedderAppBannerManager")]
    EmbedderAppBannerManager,
    #[serde(rename = "EmbedderDomDistillerViewerSource")]
    EmbedderDomDistillerViewerSource,
    #[serde(rename = "EmbedderDomDistillerSelfDeletingRequestDelegate")]
    EmbedderDomDistillerSelfDeletingRequestDelegate,
    #[serde(rename = "EmbedderOomInterventionTabHelper")]
    EmbedderOomInterventionTabHelper,
    #[serde(rename = "EmbedderOfflinePage")]
    EmbedderOfflinePage,
    #[serde(rename = "EmbedderChromePasswordManagerClientBindCredentialManager")]
    EmbedderChromePasswordManagerClientBindCredentialManager,
    #[serde(rename = "EmbedderPermissionRequestManager")]
    EmbedderPermissionRequestManager,
    #[serde(rename = "EmbedderModalDialog")]
    EmbedderModalDialog,
    #[serde(rename = "EmbedderExtensions")]
    EmbedderExtensions,
    #[serde(rename = "EmbedderExtensionMessaging")]
    EmbedderExtensionMessaging,
    #[serde(rename = "EmbedderExtensionMessagingForOpenPort")]
    EmbedderExtensionMessagingForOpenPort,
    #[serde(rename = "EmbedderExtensionSentMessageToCachedFrame")]
    EmbedderExtensionSentMessageToCachedFrame,
    #[serde(rename = "EmbedderExtensionFrame")]
    EmbedderExtensionFrame,
    #[serde(rename = "RequestedByWebViewClient")]
    RequestedByWebViewClient,
    #[serde(rename = "PostMessageByWebViewClient")]
    PostMessageByWebViewClient,
    #[serde(rename = "CacheControlNoStoreDeviceBoundSessionTerminated")]
    CacheControlNoStoreDeviceBoundSessionTerminated,
    #[serde(rename = "CacheLimitPrunedOnModerateMemoryPressure")]
    CacheLimitPrunedOnModerateMemoryPressure,
    #[serde(rename = "CacheLimitPrunedOnCriticalMemoryPressure")]
    CacheLimitPrunedOnCriticalMemoryPressure,
}

/// Types of not restored reasons for back-forward cache.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BackForwardCacheNotRestoredReasonType {
    #[default]
    #[serde(rename = "SupportPending")]
    SupportPending,
    #[serde(rename = "PageSupportNeeded")]
    PageSupportNeeded,
    #[serde(rename = "Circumstantial")]
    Circumstantial,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheBlockingDetails<'a> {
    /// Url of the file where blockage happened. Optional because of tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// Function name where blockage happened. Optional because of anonymous functions and tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    function: Option<Cow<'a, str>>,
    /// Line number in the script (0-based).
    #[serde(rename = "lineNumber")]
    line_number: i64,
    /// Column number in the script (0-based).
    #[serde(rename = "columnNumber")]
    column_number: i64,
}

impl<'a> BackForwardCacheBlockingDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `line_number`: Line number in the script (0-based).
    /// * `column_number`: Column number in the script (0-based).
    pub fn builder(line_number: i64, column_number: i64) -> BackForwardCacheBlockingDetailsBuilder<'a> {
        BackForwardCacheBlockingDetailsBuilder {
            url: None,
            function: None,
            line_number: line_number,
            column_number: column_number,
        }
    }
    /// Url of the file where blockage happened. Optional because of tests.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// Function name where blockage happened. Optional because of anonymous functions and tests.
    pub fn function(&self) -> Option<&str> { self.function.as_deref() }
    /// Line number in the script (0-based).
    pub fn line_number(&self) -> i64 { self.line_number }
    /// Column number in the script (0-based).
    pub fn column_number(&self) -> i64 { self.column_number }
}


pub struct BackForwardCacheBlockingDetailsBuilder<'a> {
    url: Option<Cow<'a, str>>,
    function: Option<Cow<'a, str>>,
    line_number: i64,
    column_number: i64,
}

impl<'a> BackForwardCacheBlockingDetailsBuilder<'a> {
    /// Url of the file where blockage happened. Optional because of tests.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Function name where blockage happened. Optional because of anonymous functions and tests.
    pub fn function(mut self, function: impl Into<Cow<'a, str>>) -> Self { self.function = Some(function.into()); self }
    pub fn build(self) -> BackForwardCacheBlockingDetails<'a> {
        BackForwardCacheBlockingDetails {
            url: self.url,
            function: self.function,
            line_number: self.line_number,
            column_number: self.column_number,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanation<'a> {
    /// Type of the reason
    #[serde(rename = "type")]
    type_: BackForwardCacheNotRestoredReasonType,
    /// Not restored reason
    reason: BackForwardCacheNotRestoredReason,
    /// Context associated with the reason. The meaning of this context is
    /// dependent on the reason:
    /// - EmbedderExtensionSentMessageToCachedFrame: the extension ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Vec<BackForwardCacheBlockingDetails<'a>>>,
}

impl<'a> BackForwardCacheNotRestoredExplanation<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Type of the reason
    /// * `reason`: Not restored reason
    pub fn builder(type_: impl Into<BackForwardCacheNotRestoredReasonType>, reason: impl Into<BackForwardCacheNotRestoredReason>) -> BackForwardCacheNotRestoredExplanationBuilder<'a> {
        BackForwardCacheNotRestoredExplanationBuilder {
            type_: type_.into(),
            reason: reason.into(),
            context: None,
            details: None,
        }
    }
    /// Type of the reason
    pub fn type_(&self) -> &BackForwardCacheNotRestoredReasonType { &self.type_ }
    /// Not restored reason
    pub fn reason(&self) -> &BackForwardCacheNotRestoredReason { &self.reason }
    /// Context associated with the reason. The meaning of this context is
    /// dependent on the reason:
    /// - EmbedderExtensionSentMessageToCachedFrame: the extension ID.
    pub fn context(&self) -> Option<&str> { self.context.as_deref() }
    pub fn details(&self) -> Option<&[BackForwardCacheBlockingDetails<'a>]> { self.details.as_deref() }
}


pub struct BackForwardCacheNotRestoredExplanationBuilder<'a> {
    type_: BackForwardCacheNotRestoredReasonType,
    reason: BackForwardCacheNotRestoredReason,
    context: Option<Cow<'a, str>>,
    details: Option<Vec<BackForwardCacheBlockingDetails<'a>>>,
}

impl<'a> BackForwardCacheNotRestoredExplanationBuilder<'a> {
    /// Context associated with the reason. The meaning of this context is
    /// dependent on the reason:
    /// - EmbedderExtensionSentMessageToCachedFrame: the extension ID.
    pub fn context(mut self, context: impl Into<Cow<'a, str>>) -> Self { self.context = Some(context.into()); self }
    pub fn details(mut self, details: Vec<BackForwardCacheBlockingDetails<'a>>) -> Self { self.details = Some(details); self }
    pub fn build(self) -> BackForwardCacheNotRestoredExplanation<'a> {
        BackForwardCacheNotRestoredExplanation {
            type_: self.type_,
            reason: self.reason,
            context: self.context,
            details: self.details,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanationTree<'a> {
    /// URL of each frame
    url: Cow<'a, str>,
    /// Not restored reasons of each frame
    explanations: Vec<BackForwardCacheNotRestoredExplanation<'a>>,
    /// Array of children frame
    children: Vec<Box<BackForwardCacheNotRestoredExplanationTree<'a>>>,
}

impl<'a> BackForwardCacheNotRestoredExplanationTree<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: URL of each frame
    /// * `explanations`: Not restored reasons of each frame
    /// * `children`: Array of children frame
    pub fn builder(url: impl Into<Cow<'a, str>>, explanations: Vec<BackForwardCacheNotRestoredExplanation<'a>>, children: Vec<Box<BackForwardCacheNotRestoredExplanationTree<'a>>>) -> BackForwardCacheNotRestoredExplanationTreeBuilder<'a> {
        BackForwardCacheNotRestoredExplanationTreeBuilder {
            url: url.into(),
            explanations: explanations,
            children: children,
        }
    }
    /// URL of each frame
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Not restored reasons of each frame
    pub fn explanations(&self) -> &[BackForwardCacheNotRestoredExplanation<'a>] { &self.explanations }
    /// Array of children frame
    pub fn children(&self) -> &[Box<BackForwardCacheNotRestoredExplanationTree<'a>>] { &self.children }
}


pub struct BackForwardCacheNotRestoredExplanationTreeBuilder<'a> {
    url: Cow<'a, str>,
    explanations: Vec<BackForwardCacheNotRestoredExplanation<'a>>,
    children: Vec<Box<BackForwardCacheNotRestoredExplanationTree<'a>>>,
}

impl<'a> BackForwardCacheNotRestoredExplanationTreeBuilder<'a> {
    pub fn build(self) -> BackForwardCacheNotRestoredExplanationTree<'a> {
        BackForwardCacheNotRestoredExplanationTree {
            url: self.url,
            explanations: self.explanations,
            children: self.children,
        }
    }
}

/// Deprecated, please use addScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnLoadParams<'a> {
    #[serde(rename = "scriptSource")]
    script_source: Cow<'a, str>,
}

impl<'a> AddScriptToEvaluateOnLoadParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_source`: 
    pub fn builder(script_source: impl Into<Cow<'a, str>>) -> AddScriptToEvaluateOnLoadParamsBuilder<'a> {
        AddScriptToEvaluateOnLoadParamsBuilder {
            script_source: script_source.into(),
        }
    }
    pub fn script_source(&self) -> &str { self.script_source.as_ref() }
}


pub struct AddScriptToEvaluateOnLoadParamsBuilder<'a> {
    script_source: Cow<'a, str>,
}

impl<'a> AddScriptToEvaluateOnLoadParamsBuilder<'a> {
    pub fn build(self) -> AddScriptToEvaluateOnLoadParams<'a> {
        AddScriptToEvaluateOnLoadParams {
            script_source: self.script_source,
        }
    }
}

/// Deprecated, please use addScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnLoadReturns<'a> {
    /// Identifier of the added script.
    identifier: ScriptIdentifier<'a>,
}

impl<'a> AddScriptToEvaluateOnLoadReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `identifier`: Identifier of the added script.
    pub fn builder(identifier: impl Into<ScriptIdentifier<'a>>) -> AddScriptToEvaluateOnLoadReturnsBuilder<'a> {
        AddScriptToEvaluateOnLoadReturnsBuilder {
            identifier: identifier.into(),
        }
    }
    /// Identifier of the added script.
    pub fn identifier(&self) -> &ScriptIdentifier<'a> { &self.identifier }
}


pub struct AddScriptToEvaluateOnLoadReturnsBuilder<'a> {
    identifier: ScriptIdentifier<'a>,
}

impl<'a> AddScriptToEvaluateOnLoadReturnsBuilder<'a> {
    pub fn build(self) -> AddScriptToEvaluateOnLoadReturns<'a> {
        AddScriptToEvaluateOnLoadReturns {
            identifier: self.identifier,
        }
    }
}

impl<'a> AddScriptToEvaluateOnLoadParams<'a> { pub const METHOD: &'static str = "Page.addScriptToEvaluateOnLoad"; }

impl<'a> crate::CdpCommand<'a> for AddScriptToEvaluateOnLoadParams<'a> {
    const METHOD: &'static str = "Page.addScriptToEvaluateOnLoad";
    type Response = AddScriptToEvaluateOnLoadReturns<'a>;
}

/// Evaluates given script in every frame upon creation (before loading frame's scripts).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnNewDocumentParams<'a> {
    source: Cow<'a, str>,
    /// If specified, creates an isolated world with the given name and evaluates given script in it.
    /// This world name will be used as the ExecutionContextDescription::name when the corresponding
    /// event is emitted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "worldName")]
    world_name: Option<Cow<'a, str>>,
    /// Specifies whether command line API should be available to the script, defaults
    /// to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeCommandLineAPI")]
    include_command_line_api: Option<bool>,
    /// If true, runs the script immediately on existing execution contexts or worlds.
    /// Default: false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "runImmediately")]
    run_immediately: Option<bool>,
}

impl<'a> AddScriptToEvaluateOnNewDocumentParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `source`: 
    pub fn builder(source: impl Into<Cow<'a, str>>) -> AddScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
        AddScriptToEvaluateOnNewDocumentParamsBuilder {
            source: source.into(),
            world_name: None,
            include_command_line_api: None,
            run_immediately: None,
        }
    }
    pub fn source(&self) -> &str { self.source.as_ref() }
    /// If specified, creates an isolated world with the given name and evaluates given script in it.
    /// This world name will be used as the ExecutionContextDescription::name when the corresponding
    /// event is emitted.
    pub fn world_name(&self) -> Option<&str> { self.world_name.as_deref() }
    /// Specifies whether command line API should be available to the script, defaults
    /// to false.
    pub fn include_command_line_api(&self) -> Option<bool> { self.include_command_line_api }
    /// If true, runs the script immediately on existing execution contexts or worlds.
    /// Default: false.
    pub fn run_immediately(&self) -> Option<bool> { self.run_immediately }
}


pub struct AddScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
    source: Cow<'a, str>,
    world_name: Option<Cow<'a, str>>,
    include_command_line_api: Option<bool>,
    run_immediately: Option<bool>,
}

impl<'a> AddScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
    /// If specified, creates an isolated world with the given name and evaluates given script in it.
    /// This world name will be used as the ExecutionContextDescription::name when the corresponding
    /// event is emitted.
    pub fn world_name(mut self, world_name: impl Into<Cow<'a, str>>) -> Self { self.world_name = Some(world_name.into()); self }
    /// Specifies whether command line API should be available to the script, defaults
    /// to false.
    pub fn include_command_line_api(mut self, include_command_line_api: bool) -> Self { self.include_command_line_api = Some(include_command_line_api); self }
    /// If true, runs the script immediately on existing execution contexts or worlds.
    /// Default: false.
    pub fn run_immediately(mut self, run_immediately: bool) -> Self { self.run_immediately = Some(run_immediately); self }
    pub fn build(self) -> AddScriptToEvaluateOnNewDocumentParams<'a> {
        AddScriptToEvaluateOnNewDocumentParams {
            source: self.source,
            world_name: self.world_name,
            include_command_line_api: self.include_command_line_api,
            run_immediately: self.run_immediately,
        }
    }
}

/// Evaluates given script in every frame upon creation (before loading frame's scripts).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnNewDocumentReturns<'a> {
    /// Identifier of the added script.
    identifier: ScriptIdentifier<'a>,
}

impl<'a> AddScriptToEvaluateOnNewDocumentReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `identifier`: Identifier of the added script.
    pub fn builder(identifier: impl Into<ScriptIdentifier<'a>>) -> AddScriptToEvaluateOnNewDocumentReturnsBuilder<'a> {
        AddScriptToEvaluateOnNewDocumentReturnsBuilder {
            identifier: identifier.into(),
        }
    }
    /// Identifier of the added script.
    pub fn identifier(&self) -> &ScriptIdentifier<'a> { &self.identifier }
}


pub struct AddScriptToEvaluateOnNewDocumentReturnsBuilder<'a> {
    identifier: ScriptIdentifier<'a>,
}

impl<'a> AddScriptToEvaluateOnNewDocumentReturnsBuilder<'a> {
    pub fn build(self) -> AddScriptToEvaluateOnNewDocumentReturns<'a> {
        AddScriptToEvaluateOnNewDocumentReturns {
            identifier: self.identifier,
        }
    }
}

impl<'a> AddScriptToEvaluateOnNewDocumentParams<'a> { pub const METHOD: &'static str = "Page.addScriptToEvaluateOnNewDocument"; }

impl<'a> crate::CdpCommand<'a> for AddScriptToEvaluateOnNewDocumentParams<'a> {
    const METHOD: &'static str = "Page.addScriptToEvaluateOnNewDocument";
    type Response = AddScriptToEvaluateOnNewDocumentReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BringToFrontParams {}

impl BringToFrontParams { pub const METHOD: &'static str = "Page.bringToFront"; }

impl<'a> crate::CdpCommand<'a> for BringToFrontParams {
    const METHOD: &'static str = "Page.bringToFront";
    type Response = crate::EmptyReturns;
}

/// Capture page screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureScreenshotParams<'a> {
    /// Image compression format (defaults to png).
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<Cow<'a, str>>,
    /// Compression quality from range \[0..100\] (jpeg only).
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<i64>,
    /// Capture the screenshot of a given region only.
    #[serde(skip_serializing_if = "Option::is_none")]
    clip: Option<Viewport>,
    /// Capture the screenshot from the surface, rather than the view. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromSurface")]
    from_surface: Option<bool>,
    /// Capture the screenshot beyond the viewport. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "captureBeyondViewport")]
    capture_beyond_viewport: Option<bool>,
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    #[serde(skip_serializing_if = "Option::is_none", rename = "optimizeForSpeed")]
    optimize_for_speed: Option<bool>,
}

impl<'a> CaptureScreenshotParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> CaptureScreenshotParamsBuilder<'a> {
        CaptureScreenshotParamsBuilder {
            format: None,
            quality: None,
            clip: None,
            from_surface: None,
            capture_beyond_viewport: None,
            optimize_for_speed: None,
        }
    }
    /// Image compression format (defaults to png).
    pub fn format(&self) -> Option<&str> { self.format.as_deref() }
    /// Compression quality from range \[0..100\] (jpeg only).
    pub fn quality(&self) -> Option<i64> { self.quality }
    /// Capture the screenshot of a given region only.
    pub fn clip(&self) -> Option<&Viewport> { self.clip.as_ref() }
    /// Capture the screenshot from the surface, rather than the view. Defaults to true.
    pub fn from_surface(&self) -> Option<bool> { self.from_surface }
    /// Capture the screenshot beyond the viewport. Defaults to false.
    pub fn capture_beyond_viewport(&self) -> Option<bool> { self.capture_beyond_viewport }
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    pub fn optimize_for_speed(&self) -> Option<bool> { self.optimize_for_speed }
}

#[derive(Default)]
pub struct CaptureScreenshotParamsBuilder<'a> {
    format: Option<Cow<'a, str>>,
    quality: Option<i64>,
    clip: Option<Viewport>,
    from_surface: Option<bool>,
    capture_beyond_viewport: Option<bool>,
    optimize_for_speed: Option<bool>,
}

impl<'a> CaptureScreenshotParamsBuilder<'a> {
    /// Image compression format (defaults to png).
    pub fn format(mut self, format: impl Into<Cow<'a, str>>) -> Self { self.format = Some(format.into()); self }
    /// Compression quality from range \[0..100\] (jpeg only).
    pub fn quality(mut self, quality: i64) -> Self { self.quality = Some(quality); self }
    /// Capture the screenshot of a given region only.
    pub fn clip(mut self, clip: Viewport) -> Self { self.clip = Some(clip); self }
    /// Capture the screenshot from the surface, rather than the view. Defaults to true.
    pub fn from_surface(mut self, from_surface: bool) -> Self { self.from_surface = Some(from_surface); self }
    /// Capture the screenshot beyond the viewport. Defaults to false.
    pub fn capture_beyond_viewport(mut self, capture_beyond_viewport: bool) -> Self { self.capture_beyond_viewport = Some(capture_beyond_viewport); self }
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    pub fn optimize_for_speed(mut self, optimize_for_speed: bool) -> Self { self.optimize_for_speed = Some(optimize_for_speed); self }
    pub fn build(self) -> CaptureScreenshotParams<'a> {
        CaptureScreenshotParams {
            format: self.format,
            quality: self.quality,
            clip: self.clip,
            from_surface: self.from_surface,
            capture_beyond_viewport: self.capture_beyond_viewport,
            optimize_for_speed: self.optimize_for_speed,
        }
    }
}

/// Capture page screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureScreenshotReturns<'a> {
    /// Base64-encoded image data. (Encoded as a base64 string when passed over JSON)
    data: Cow<'a, str>,
}

impl<'a> CaptureScreenshotReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `data`: Base64-encoded image data. (Encoded as a base64 string when passed over JSON)
    pub fn builder(data: impl Into<Cow<'a, str>>) -> CaptureScreenshotReturnsBuilder<'a> {
        CaptureScreenshotReturnsBuilder {
            data: data.into(),
        }
    }
    /// Base64-encoded image data. (Encoded as a base64 string when passed over JSON)
    pub fn data(&self) -> &str { self.data.as_ref() }
}


pub struct CaptureScreenshotReturnsBuilder<'a> {
    data: Cow<'a, str>,
}

impl<'a> CaptureScreenshotReturnsBuilder<'a> {
    pub fn build(self) -> CaptureScreenshotReturns<'a> {
        CaptureScreenshotReturns {
            data: self.data,
        }
    }
}

impl<'a> CaptureScreenshotParams<'a> { pub const METHOD: &'static str = "Page.captureScreenshot"; }

impl<'a> crate::CdpCommand<'a> for CaptureScreenshotParams<'a> {
    const METHOD: &'static str = "Page.captureScreenshot";
    type Response = CaptureScreenshotReturns<'a>;
}

/// Returns a snapshot of the page as a string. For MHTML format, the serialization includes
/// iframes, shadow DOM, external resources, and element-inline styles.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotParams<'a> {
    /// Format (defaults to mhtml).
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<Cow<'a, str>>,
}

impl<'a> CaptureSnapshotParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> CaptureSnapshotParamsBuilder<'a> {
        CaptureSnapshotParamsBuilder {
            format: None,
        }
    }
    /// Format (defaults to mhtml).
    pub fn format(&self) -> Option<&str> { self.format.as_deref() }
}

#[derive(Default)]
pub struct CaptureSnapshotParamsBuilder<'a> {
    format: Option<Cow<'a, str>>,
}

impl<'a> CaptureSnapshotParamsBuilder<'a> {
    /// Format (defaults to mhtml).
    pub fn format(mut self, format: impl Into<Cow<'a, str>>) -> Self { self.format = Some(format.into()); self }
    pub fn build(self) -> CaptureSnapshotParams<'a> {
        CaptureSnapshotParams {
            format: self.format,
        }
    }
}

/// Returns a snapshot of the page as a string. For MHTML format, the serialization includes
/// iframes, shadow DOM, external resources, and element-inline styles.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotReturns<'a> {
    /// Serialized page data.
    data: Cow<'a, str>,
}

impl<'a> CaptureSnapshotReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `data`: Serialized page data.
    pub fn builder(data: impl Into<Cow<'a, str>>) -> CaptureSnapshotReturnsBuilder<'a> {
        CaptureSnapshotReturnsBuilder {
            data: data.into(),
        }
    }
    /// Serialized page data.
    pub fn data(&self) -> &str { self.data.as_ref() }
}


pub struct CaptureSnapshotReturnsBuilder<'a> {
    data: Cow<'a, str>,
}

impl<'a> CaptureSnapshotReturnsBuilder<'a> {
    pub fn build(self) -> CaptureSnapshotReturns<'a> {
        CaptureSnapshotReturns {
            data: self.data,
        }
    }
}

impl<'a> CaptureSnapshotParams<'a> { pub const METHOD: &'static str = "Page.captureSnapshot"; }

impl<'a> crate::CdpCommand<'a> for CaptureSnapshotParams<'a> {
    const METHOD: &'static str = "Page.captureSnapshot";
    type Response = CaptureSnapshotReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDeviceMetricsOverrideParams {}

impl ClearDeviceMetricsOverrideParams { pub const METHOD: &'static str = "Page.clearDeviceMetricsOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearDeviceMetricsOverrideParams {
    const METHOD: &'static str = "Page.clearDeviceMetricsOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDeviceOrientationOverrideParams {}

impl ClearDeviceOrientationOverrideParams { pub const METHOD: &'static str = "Page.clearDeviceOrientationOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearDeviceOrientationOverrideParams {
    const METHOD: &'static str = "Page.clearDeviceOrientationOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearGeolocationOverrideParams {}

impl ClearGeolocationOverrideParams { pub const METHOD: &'static str = "Page.clearGeolocationOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearGeolocationOverrideParams {
    const METHOD: &'static str = "Page.clearGeolocationOverride";
    type Response = crate::EmptyReturns;
}

/// Creates an isolated world for the given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateIsolatedWorldParams<'a> {
    /// Id of the frame in which the isolated world should be created.
    #[serde(rename = "frameId")]
    frame_id: FrameId<'a>,
    /// An optional name which is reported in the Execution Context.
    #[serde(skip_serializing_if = "Option::is_none", rename = "worldName")]
    world_name: Option<Cow<'a, str>>,
    /// Whether or not universal access should be granted to the isolated world. This is a powerful
    /// option, use with caution.
    #[serde(skip_serializing_if = "Option::is_none", rename = "grantUniveralAccess")]
    grant_univeral_access: Option<bool>,
}

impl<'a> CreateIsolatedWorldParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: Id of the frame in which the isolated world should be created.
    pub fn builder(frame_id: impl Into<FrameId<'a>>) -> CreateIsolatedWorldParamsBuilder<'a> {
        CreateIsolatedWorldParamsBuilder {
            frame_id: frame_id.into(),
            world_name: None,
            grant_univeral_access: None,
        }
    }
    /// Id of the frame in which the isolated world should be created.
    pub fn frame_id(&self) -> &FrameId<'a> { &self.frame_id }
    /// An optional name which is reported in the Execution Context.
    pub fn world_name(&self) -> Option<&str> { self.world_name.as_deref() }
    /// Whether or not universal access should be granted to the isolated world. This is a powerful
    /// option, use with caution.
    pub fn grant_univeral_access(&self) -> Option<bool> { self.grant_univeral_access }
}


pub struct CreateIsolatedWorldParamsBuilder<'a> {
    frame_id: FrameId<'a>,
    world_name: Option<Cow<'a, str>>,
    grant_univeral_access: Option<bool>,
}

impl<'a> CreateIsolatedWorldParamsBuilder<'a> {
    /// An optional name which is reported in the Execution Context.
    pub fn world_name(mut self, world_name: impl Into<Cow<'a, str>>) -> Self { self.world_name = Some(world_name.into()); self }
    /// Whether or not universal access should be granted to the isolated world. This is a powerful
    /// option, use with caution.
    pub fn grant_univeral_access(mut self, grant_univeral_access: bool) -> Self { self.grant_univeral_access = Some(grant_univeral_access); self }
    pub fn build(self) -> CreateIsolatedWorldParams<'a> {
        CreateIsolatedWorldParams {
            frame_id: self.frame_id,
            world_name: self.world_name,
            grant_univeral_access: self.grant_univeral_access,
        }
    }
}

/// Creates an isolated world for the given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateIsolatedWorldReturns {
    /// Execution context of the isolated world.
    #[serde(rename = "executionContextId")]
    execution_context_id: crate::runtime::ExecutionContextId,
}

impl CreateIsolatedWorldReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `execution_context_id`: Execution context of the isolated world.
    pub fn builder(execution_context_id: crate::runtime::ExecutionContextId) -> CreateIsolatedWorldReturnsBuilder {
        CreateIsolatedWorldReturnsBuilder {
            execution_context_id: execution_context_id,
        }
    }
    /// Execution context of the isolated world.
    pub fn execution_context_id(&self) -> &crate::runtime::ExecutionContextId { &self.execution_context_id }
}


pub struct CreateIsolatedWorldReturnsBuilder {
    execution_context_id: crate::runtime::ExecutionContextId,
}

impl CreateIsolatedWorldReturnsBuilder {
    pub fn build(self) -> CreateIsolatedWorldReturns {
        CreateIsolatedWorldReturns {
            execution_context_id: self.execution_context_id,
        }
    }
}

impl<'a> CreateIsolatedWorldParams<'a> { pub const METHOD: &'static str = "Page.createIsolatedWorld"; }

impl<'a> crate::CdpCommand<'a> for CreateIsolatedWorldParams<'a> {
    const METHOD: &'static str = "Page.createIsolatedWorld";
    type Response = CreateIsolatedWorldReturns;
}

/// Deletes browser cookie with given name, domain and path.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCookieParams<'a> {
    /// Name of the cookie to remove.
    #[serde(rename = "cookieName")]
    cookie_name: Cow<'a, str>,
    /// URL to match cooke domain and path.
    url: Cow<'a, str>,
}

impl<'a> DeleteCookieParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cookie_name`: Name of the cookie to remove.
    /// * `url`: URL to match cooke domain and path.
    pub fn builder(cookie_name: impl Into<Cow<'a, str>>, url: impl Into<Cow<'a, str>>) -> DeleteCookieParamsBuilder<'a> {
        DeleteCookieParamsBuilder {
            cookie_name: cookie_name.into(),
            url: url.into(),
        }
    }
    /// Name of the cookie to remove.
    pub fn cookie_name(&self) -> &str { self.cookie_name.as_ref() }
    /// URL to match cooke domain and path.
    pub fn url(&self) -> &str { self.url.as_ref() }
}


pub struct DeleteCookieParamsBuilder<'a> {
    cookie_name: Cow<'a, str>,
    url: Cow<'a, str>,
}

impl<'a> DeleteCookieParamsBuilder<'a> {
    pub fn build(self) -> DeleteCookieParams<'a> {
        DeleteCookieParams {
            cookie_name: self.cookie_name,
            url: self.url,
        }
    }
}

impl<'a> DeleteCookieParams<'a> { pub const METHOD: &'static str = "Page.deleteCookie"; }

impl<'a> crate::CdpCommand<'a> for DeleteCookieParams<'a> {
    const METHOD: &'static str = "Page.deleteCookie";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Page.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Page.disable";
    type Response = crate::EmptyReturns;
}

/// Enables page domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// If true, the 'Page.fileChooserOpened' event will be emitted regardless of the state set by
    /// 'Page.setInterceptFileChooserDialog' command (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableFileChooserOpenedEvent")]
    enable_file_chooser_opened_event: Option<bool>,
}

impl EnableParams {
    /// Creates a builder for this type.
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder {
            enable_file_chooser_opened_event: None,
        }
    }
    /// If true, the 'Page.fileChooserOpened' event will be emitted regardless of the state set by
    /// 'Page.setInterceptFileChooserDialog' command (default: false).
    pub fn enable_file_chooser_opened_event(&self) -> Option<bool> { self.enable_file_chooser_opened_event }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    enable_file_chooser_opened_event: Option<bool>,
}

impl EnableParamsBuilder {
    /// If true, the 'Page.fileChooserOpened' event will be emitted regardless of the state set by
    /// 'Page.setInterceptFileChooserDialog' command (default: false).
    pub fn enable_file_chooser_opened_event(mut self, enable_file_chooser_opened_event: bool) -> Self { self.enable_file_chooser_opened_event = Some(enable_file_chooser_opened_event); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            enable_file_chooser_opened_event: self.enable_file_chooser_opened_event,
        }
    }
}

impl EnableParams { pub const METHOD: &'static str = "Page.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Page.enable";
    type Response = crate::EmptyReturns;
}

/// Gets the processed manifest for this current document.
/// This API always waits for the manifest to be loaded.
/// If manifestId is provided, and it does not match the manifest of the
/// current document, this API errors out.
/// If there is not a loaded page, this API errors out immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAppManifestParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "manifestId")]
    manifest_id: Option<Cow<'a, str>>,
}

impl<'a> GetAppManifestParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetAppManifestParamsBuilder<'a> {
        GetAppManifestParamsBuilder {
            manifest_id: None,
        }
    }
    pub fn manifest_id(&self) -> Option<&str> { self.manifest_id.as_deref() }
}

#[derive(Default)]
pub struct GetAppManifestParamsBuilder<'a> {
    manifest_id: Option<Cow<'a, str>>,
}

impl<'a> GetAppManifestParamsBuilder<'a> {
    pub fn manifest_id(mut self, manifest_id: impl Into<Cow<'a, str>>) -> Self { self.manifest_id = Some(manifest_id.into()); self }
    pub fn build(self) -> GetAppManifestParams<'a> {
        GetAppManifestParams {
            manifest_id: self.manifest_id,
        }
    }
}

/// Gets the processed manifest for this current document.
/// This API always waits for the manifest to be loaded.
/// If manifestId is provided, and it does not match the manifest of the
/// current document, this API errors out.
/// If there is not a loaded page, this API errors out immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAppManifestReturns<'a> {
    /// Manifest location.
    url: Cow<'a, str>,
    errors: Vec<AppManifestError<'a>>,
    /// Manifest content.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Cow<'a, str>>,
    /// Parsed manifest properties. Deprecated, use manifest instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    parsed: Option<AppManifestParsedProperties<'a>>,
    manifest: WebAppManifest<'a>,
}

impl<'a> GetAppManifestReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: Manifest location.
    /// * `errors`: 
    /// * `manifest`: 
    pub fn builder(url: impl Into<Cow<'a, str>>, errors: Vec<AppManifestError<'a>>, manifest: WebAppManifest<'a>) -> GetAppManifestReturnsBuilder<'a> {
        GetAppManifestReturnsBuilder {
            url: url.into(),
            errors: errors,
            data: None,
            parsed: None,
            manifest: manifest,
        }
    }
    /// Manifest location.
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn errors(&self) -> &[AppManifestError<'a>] { &self.errors }
    /// Manifest content.
    pub fn data(&self) -> Option<&str> { self.data.as_deref() }
    /// Parsed manifest properties. Deprecated, use manifest instead.
    pub fn parsed(&self) -> Option<&AppManifestParsedProperties<'a>> { self.parsed.as_ref() }
    pub fn manifest(&self) -> &WebAppManifest<'a> { &self.manifest }
}


pub struct GetAppManifestReturnsBuilder<'a> {
    url: Cow<'a, str>,
    errors: Vec<AppManifestError<'a>>,
    data: Option<Cow<'a, str>>,
    parsed: Option<AppManifestParsedProperties<'a>>,
    manifest: WebAppManifest<'a>,
}

impl<'a> GetAppManifestReturnsBuilder<'a> {
    /// Manifest content.
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    /// Parsed manifest properties. Deprecated, use manifest instead.
    pub fn parsed(mut self, parsed: AppManifestParsedProperties<'a>) -> Self { self.parsed = Some(parsed); self }
    pub fn build(self) -> GetAppManifestReturns<'a> {
        GetAppManifestReturns {
            url: self.url,
            errors: self.errors,
            data: self.data,
            parsed: self.parsed,
            manifest: self.manifest,
        }
    }
}

impl<'a> GetAppManifestParams<'a> { pub const METHOD: &'static str = "Page.getAppManifest"; }

impl<'a> crate::CdpCommand<'a> for GetAppManifestParams<'a> {
    const METHOD: &'static str = "Page.getAppManifest";
    type Response = GetAppManifestReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstallabilityErrorsReturns<'a> {
    #[serde(rename = "installabilityErrors")]
    installability_errors: Vec<InstallabilityError<'a>>,
}

impl<'a> GetInstallabilityErrorsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `installability_errors`: 
    pub fn builder(installability_errors: Vec<InstallabilityError<'a>>) -> GetInstallabilityErrorsReturnsBuilder<'a> {
        GetInstallabilityErrorsReturnsBuilder {
            installability_errors: installability_errors,
        }
    }
    pub fn installability_errors(&self) -> &[InstallabilityError<'a>] { &self.installability_errors }
}


pub struct GetInstallabilityErrorsReturnsBuilder<'a> {
    installability_errors: Vec<InstallabilityError<'a>>,
}

impl<'a> GetInstallabilityErrorsReturnsBuilder<'a> {
    pub fn build(self) -> GetInstallabilityErrorsReturns<'a> {
        GetInstallabilityErrorsReturns {
            installability_errors: self.installability_errors,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetInstallabilityErrorsParams {}

impl GetInstallabilityErrorsParams { pub const METHOD: &'static str = "Page.getInstallabilityErrors"; }

impl<'a> crate::CdpCommand<'a> for GetInstallabilityErrorsParams {
    const METHOD: &'static str = "Page.getInstallabilityErrors";
    type Response = GetInstallabilityErrorsReturns<'a>;
}

/// Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetManifestIconsReturns<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "primaryIcon")]
    primary_icon: Option<Cow<'a, str>>,
}

impl<'a> GetManifestIconsReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetManifestIconsReturnsBuilder<'a> {
        GetManifestIconsReturnsBuilder {
            primary_icon: None,
        }
    }
    pub fn primary_icon(&self) -> Option<&str> { self.primary_icon.as_deref() }
}

#[derive(Default)]
pub struct GetManifestIconsReturnsBuilder<'a> {
    primary_icon: Option<Cow<'a, str>>,
}

impl<'a> GetManifestIconsReturnsBuilder<'a> {
    pub fn primary_icon(mut self, primary_icon: impl Into<Cow<'a, str>>) -> Self { self.primary_icon = Some(primary_icon.into()); self }
    pub fn build(self) -> GetManifestIconsReturns<'a> {
        GetManifestIconsReturns {
            primary_icon: self.primary_icon,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetManifestIconsParams {}

impl GetManifestIconsParams { pub const METHOD: &'static str = "Page.getManifestIcons"; }

impl<'a> crate::CdpCommand<'a> for GetManifestIconsParams {
    const METHOD: &'static str = "Page.getManifestIcons";
    type Response = GetManifestIconsReturns<'a>;
}

/// Returns the unique (PWA) app id.
/// Only returns values if the feature flag 'WebAppEnableManifestId' is enabled

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAppIdReturns<'a> {
    /// App id, either from manifest's id attribute or computed from start_url
    #[serde(skip_serializing_if = "Option::is_none", rename = "appId")]
    app_id: Option<Cow<'a, str>>,
    /// Recommendation for manifest's id attribute to match current id computed from start_url
    #[serde(skip_serializing_if = "Option::is_none", rename = "recommendedId")]
    recommended_id: Option<Cow<'a, str>>,
}

impl<'a> GetAppIdReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetAppIdReturnsBuilder<'a> {
        GetAppIdReturnsBuilder {
            app_id: None,
            recommended_id: None,
        }
    }
    /// App id, either from manifest's id attribute or computed from start_url
    pub fn app_id(&self) -> Option<&str> { self.app_id.as_deref() }
    /// Recommendation for manifest's id attribute to match current id computed from start_url
    pub fn recommended_id(&self) -> Option<&str> { self.recommended_id.as_deref() }
}

#[derive(Default)]
pub struct GetAppIdReturnsBuilder<'a> {
    app_id: Option<Cow<'a, str>>,
    recommended_id: Option<Cow<'a, str>>,
}

impl<'a> GetAppIdReturnsBuilder<'a> {
    /// App id, either from manifest's id attribute or computed from start_url
    pub fn app_id(mut self, app_id: impl Into<Cow<'a, str>>) -> Self { self.app_id = Some(app_id.into()); self }
    /// Recommendation for manifest's id attribute to match current id computed from start_url
    pub fn recommended_id(mut self, recommended_id: impl Into<Cow<'a, str>>) -> Self { self.recommended_id = Some(recommended_id.into()); self }
    pub fn build(self) -> GetAppIdReturns<'a> {
        GetAppIdReturns {
            app_id: self.app_id,
            recommended_id: self.recommended_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAppIdParams {}

impl GetAppIdParams { pub const METHOD: &'static str = "Page.getAppId"; }

impl<'a> crate::CdpCommand<'a> for GetAppIdParams {
    const METHOD: &'static str = "Page.getAppId";
    type Response = GetAppIdReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAdScriptAncestryParams<'a> {
    #[serde(rename = "frameId")]
    frame_id: FrameId<'a>,
}

impl<'a> GetAdScriptAncestryParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: 
    pub fn builder(frame_id: impl Into<FrameId<'a>>) -> GetAdScriptAncestryParamsBuilder<'a> {
        GetAdScriptAncestryParamsBuilder {
            frame_id: frame_id.into(),
        }
    }
    pub fn frame_id(&self) -> &FrameId<'a> { &self.frame_id }
}


pub struct GetAdScriptAncestryParamsBuilder<'a> {
    frame_id: FrameId<'a>,
}

impl<'a> GetAdScriptAncestryParamsBuilder<'a> {
    pub fn build(self) -> GetAdScriptAncestryParams<'a> {
        GetAdScriptAncestryParams {
            frame_id: self.frame_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAdScriptAncestryReturns<'a> {
    /// The ancestry chain of ad script identifiers leading to this frame's
    /// creation, along with the root script's filterlist rule. The ancestry
    /// chain is ordered from the most immediate script (in the frame creation
    /// stack) to more distant ancestors (that created the immediately preceding
    /// script). Only sent if frame is labelled as an ad and ids are available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "adScriptAncestry")]
    ad_script_ancestry: Option<crate::network::AdAncestry<'a>>,
}

impl<'a> GetAdScriptAncestryReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetAdScriptAncestryReturnsBuilder<'a> {
        GetAdScriptAncestryReturnsBuilder {
            ad_script_ancestry: None,
        }
    }
    /// The ancestry chain of ad script identifiers leading to this frame's
    /// creation, along with the root script's filterlist rule. The ancestry
    /// chain is ordered from the most immediate script (in the frame creation
    /// stack) to more distant ancestors (that created the immediately preceding
    /// script). Only sent if frame is labelled as an ad and ids are available.
    pub fn ad_script_ancestry(&self) -> Option<&crate::network::AdAncestry<'a>> { self.ad_script_ancestry.as_ref() }
}

#[derive(Default)]
pub struct GetAdScriptAncestryReturnsBuilder<'a> {
    ad_script_ancestry: Option<crate::network::AdAncestry<'a>>,
}

impl<'a> GetAdScriptAncestryReturnsBuilder<'a> {
    /// The ancestry chain of ad script identifiers leading to this frame's
    /// creation, along with the root script's filterlist rule. The ancestry
    /// chain is ordered from the most immediate script (in the frame creation
    /// stack) to more distant ancestors (that created the immediately preceding
    /// script). Only sent if frame is labelled as an ad and ids are available.
    pub fn ad_script_ancestry(mut self, ad_script_ancestry: crate::network::AdAncestry<'a>) -> Self { self.ad_script_ancestry = Some(ad_script_ancestry); self }
    pub fn build(self) -> GetAdScriptAncestryReturns<'a> {
        GetAdScriptAncestryReturns {
            ad_script_ancestry: self.ad_script_ancestry,
        }
    }
}

impl<'a> GetAdScriptAncestryParams<'a> { pub const METHOD: &'static str = "Page.getAdScriptAncestry"; }

impl<'a> crate::CdpCommand<'a> for GetAdScriptAncestryParams<'a> {
    const METHOD: &'static str = "Page.getAdScriptAncestry";
    type Response = GetAdScriptAncestryReturns<'a>;
}

/// Returns present frame tree structure.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameTreeReturns<'a> {
    /// Present frame tree structure.
    #[serde(rename = "frameTree")]
    frame_tree: FrameTree<'a>,
}

impl<'a> GetFrameTreeReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_tree`: Present frame tree structure.
    pub fn builder(frame_tree: FrameTree<'a>) -> GetFrameTreeReturnsBuilder<'a> {
        GetFrameTreeReturnsBuilder {
            frame_tree: frame_tree,
        }
    }
    /// Present frame tree structure.
    pub fn frame_tree(&self) -> &FrameTree<'a> { &self.frame_tree }
}


pub struct GetFrameTreeReturnsBuilder<'a> {
    frame_tree: FrameTree<'a>,
}

impl<'a> GetFrameTreeReturnsBuilder<'a> {
    pub fn build(self) -> GetFrameTreeReturns<'a> {
        GetFrameTreeReturns {
            frame_tree: self.frame_tree,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetFrameTreeParams {}

impl GetFrameTreeParams { pub const METHOD: &'static str = "Page.getFrameTree"; }

impl<'a> crate::CdpCommand<'a> for GetFrameTreeParams {
    const METHOD: &'static str = "Page.getFrameTree";
    type Response = GetFrameTreeReturns<'a>;
}

/// Returns metrics relating to the layouting of the page, such as viewport bounds/scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLayoutMetricsReturns {
    /// Deprecated metrics relating to the layout viewport. Is in device pixels. Use 'cssLayoutViewport' instead.
    #[serde(rename = "layoutViewport")]
    layout_viewport: LayoutViewport,
    /// Deprecated metrics relating to the visual viewport. Is in device pixels. Use 'cssVisualViewport' instead.
    #[serde(rename = "visualViewport")]
    visual_viewport: VisualViewport,
    /// Deprecated size of scrollable area. Is in DP. Use 'cssContentSize' instead.
    #[serde(rename = "contentSize")]
    content_size: crate::dom::Rect,
    /// Metrics relating to the layout viewport in CSS pixels.
    #[serde(rename = "cssLayoutViewport")]
    css_layout_viewport: LayoutViewport,
    /// Metrics relating to the visual viewport in CSS pixels.
    #[serde(rename = "cssVisualViewport")]
    css_visual_viewport: VisualViewport,
    /// Size of scrollable area in CSS pixels.
    #[serde(rename = "cssContentSize")]
    css_content_size: crate::dom::Rect,
}

impl GetLayoutMetricsReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `layout_viewport`: Deprecated metrics relating to the layout viewport. Is in device pixels. Use `cssLayoutViewport` instead.
    /// * `visual_viewport`: Deprecated metrics relating to the visual viewport. Is in device pixels. Use `cssVisualViewport` instead.
    /// * `content_size`: Deprecated size of scrollable area. Is in DP. Use `cssContentSize` instead.
    /// * `css_layout_viewport`: Metrics relating to the layout viewport in CSS pixels.
    /// * `css_visual_viewport`: Metrics relating to the visual viewport in CSS pixels.
    /// * `css_content_size`: Size of scrollable area in CSS pixels.
    pub fn builder(layout_viewport: LayoutViewport, visual_viewport: VisualViewport, content_size: crate::dom::Rect, css_layout_viewport: LayoutViewport, css_visual_viewport: VisualViewport, css_content_size: crate::dom::Rect) -> GetLayoutMetricsReturnsBuilder {
        GetLayoutMetricsReturnsBuilder {
            layout_viewport: layout_viewport,
            visual_viewport: visual_viewport,
            content_size: content_size,
            css_layout_viewport: css_layout_viewport,
            css_visual_viewport: css_visual_viewport,
            css_content_size: css_content_size,
        }
    }
    /// Deprecated metrics relating to the layout viewport. Is in device pixels. Use 'cssLayoutViewport' instead.
    pub fn layout_viewport(&self) -> &LayoutViewport { &self.layout_viewport }
    /// Deprecated metrics relating to the visual viewport. Is in device pixels. Use 'cssVisualViewport' instead.
    pub fn visual_viewport(&self) -> &VisualViewport { &self.visual_viewport }
    /// Deprecated size of scrollable area. Is in DP. Use 'cssContentSize' instead.
    pub fn content_size(&self) -> &crate::dom::Rect { &self.content_size }
    /// Metrics relating to the layout viewport in CSS pixels.
    pub fn css_layout_viewport(&self) -> &LayoutViewport { &self.css_layout_viewport }
    /// Metrics relating to the visual viewport in CSS pixels.
    pub fn css_visual_viewport(&self) -> &VisualViewport { &self.css_visual_viewport }
    /// Size of scrollable area in CSS pixels.
    pub fn css_content_size(&self) -> &crate::dom::Rect { &self.css_content_size }
}


pub struct GetLayoutMetricsReturnsBuilder {
    layout_viewport: LayoutViewport,
    visual_viewport: VisualViewport,
    content_size: crate::dom::Rect,
    css_layout_viewport: LayoutViewport,
    css_visual_viewport: VisualViewport,
    css_content_size: crate::dom::Rect,
}

impl GetLayoutMetricsReturnsBuilder {
    pub fn build(self) -> GetLayoutMetricsReturns {
        GetLayoutMetricsReturns {
            layout_viewport: self.layout_viewport,
            visual_viewport: self.visual_viewport,
            content_size: self.content_size,
            css_layout_viewport: self.css_layout_viewport,
            css_visual_viewport: self.css_visual_viewport,
            css_content_size: self.css_content_size,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetLayoutMetricsParams {}

impl GetLayoutMetricsParams { pub const METHOD: &'static str = "Page.getLayoutMetrics"; }

impl<'a> crate::CdpCommand<'a> for GetLayoutMetricsParams {
    const METHOD: &'static str = "Page.getLayoutMetrics";
    type Response = GetLayoutMetricsReturns;
}

/// Returns navigation history for the current page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNavigationHistoryReturns<'a> {
    /// Index of the current navigation history entry.
    #[serde(rename = "currentIndex")]
    current_index: u64,
    /// Array of navigation history entries.
    entries: Vec<NavigationEntry<'a>>,
}

impl<'a> GetNavigationHistoryReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `current_index`: Index of the current navigation history entry.
    /// * `entries`: Array of navigation history entries.
    pub fn builder(current_index: u64, entries: Vec<NavigationEntry<'a>>) -> GetNavigationHistoryReturnsBuilder<'a> {
        GetNavigationHistoryReturnsBuilder {
            current_index: current_index,
            entries: entries,
        }
    }
    /// Index of the current navigation history entry.
    pub fn current_index(&self) -> u64 { self.current_index }
    /// Array of navigation history entries.
    pub fn entries(&self) -> &[NavigationEntry<'a>] { &self.entries }
}


pub struct GetNavigationHistoryReturnsBuilder<'a> {
    current_index: u64,
    entries: Vec<NavigationEntry<'a>>,
}

impl<'a> GetNavigationHistoryReturnsBuilder<'a> {
    pub fn build(self) -> GetNavigationHistoryReturns<'a> {
        GetNavigationHistoryReturns {
            current_index: self.current_index,
            entries: self.entries,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetNavigationHistoryParams {}

impl GetNavigationHistoryParams { pub const METHOD: &'static str = "Page.getNavigationHistory"; }

impl<'a> crate::CdpCommand<'a> for GetNavigationHistoryParams {
    const METHOD: &'static str = "Page.getNavigationHistory";
    type Response = GetNavigationHistoryReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResetNavigationHistoryParams {}

impl ResetNavigationHistoryParams { pub const METHOD: &'static str = "Page.resetNavigationHistory"; }

impl<'a> crate::CdpCommand<'a> for ResetNavigationHistoryParams {
    const METHOD: &'static str = "Page.resetNavigationHistory";
    type Response = crate::EmptyReturns;
}

/// Returns content of the given resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceContentParams<'a> {
    /// Frame id to get resource for.
    #[serde(rename = "frameId")]
    frame_id: FrameId<'a>,
    /// URL of the resource to get content for.
    url: Cow<'a, str>,
}

impl<'a> GetResourceContentParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: Frame id to get resource for.
    /// * `url`: URL of the resource to get content for.
    pub fn builder(frame_id: impl Into<FrameId<'a>>, url: impl Into<Cow<'a, str>>) -> GetResourceContentParamsBuilder<'a> {
        GetResourceContentParamsBuilder {
            frame_id: frame_id.into(),
            url: url.into(),
        }
    }
    /// Frame id to get resource for.
    pub fn frame_id(&self) -> &FrameId<'a> { &self.frame_id }
    /// URL of the resource to get content for.
    pub fn url(&self) -> &str { self.url.as_ref() }
}


pub struct GetResourceContentParamsBuilder<'a> {
    frame_id: FrameId<'a>,
    url: Cow<'a, str>,
}

impl<'a> GetResourceContentParamsBuilder<'a> {
    pub fn build(self) -> GetResourceContentParams<'a> {
        GetResourceContentParams {
            frame_id: self.frame_id,
            url: self.url,
        }
    }
}

/// Returns content of the given resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceContentReturns<'a> {
    /// Resource content.
    content: Cow<'a, str>,
    /// True, if content was served as base64.
    #[serde(rename = "base64Encoded")]
    base64_encoded: bool,
}

impl<'a> GetResourceContentReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `content`: Resource content.
    /// * `base64_encoded`: True, if content was served as base64.
    pub fn builder(content: impl Into<Cow<'a, str>>, base64_encoded: bool) -> GetResourceContentReturnsBuilder<'a> {
        GetResourceContentReturnsBuilder {
            content: content.into(),
            base64_encoded: base64_encoded,
        }
    }
    /// Resource content.
    pub fn content(&self) -> &str { self.content.as_ref() }
    /// True, if content was served as base64.
    pub fn base64_encoded(&self) -> bool { self.base64_encoded }
}


pub struct GetResourceContentReturnsBuilder<'a> {
    content: Cow<'a, str>,
    base64_encoded: bool,
}

impl<'a> GetResourceContentReturnsBuilder<'a> {
    pub fn build(self) -> GetResourceContentReturns<'a> {
        GetResourceContentReturns {
            content: self.content,
            base64_encoded: self.base64_encoded,
        }
    }
}

impl<'a> GetResourceContentParams<'a> { pub const METHOD: &'static str = "Page.getResourceContent"; }

impl<'a> crate::CdpCommand<'a> for GetResourceContentParams<'a> {
    const METHOD: &'static str = "Page.getResourceContent";
    type Response = GetResourceContentReturns<'a>;
}

/// Returns present frame / resource tree structure.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceTreeReturns<'a> {
    /// Present frame / resource tree structure.
    #[serde(rename = "frameTree")]
    frame_tree: FrameResourceTree<'a>,
}

impl<'a> GetResourceTreeReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_tree`: Present frame / resource tree structure.
    pub fn builder(frame_tree: FrameResourceTree<'a>) -> GetResourceTreeReturnsBuilder<'a> {
        GetResourceTreeReturnsBuilder {
            frame_tree: frame_tree,
        }
    }
    /// Present frame / resource tree structure.
    pub fn frame_tree(&self) -> &FrameResourceTree<'a> { &self.frame_tree }
}


pub struct GetResourceTreeReturnsBuilder<'a> {
    frame_tree: FrameResourceTree<'a>,
}

impl<'a> GetResourceTreeReturnsBuilder<'a> {
    pub fn build(self) -> GetResourceTreeReturns<'a> {
        GetResourceTreeReturns {
            frame_tree: self.frame_tree,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetResourceTreeParams {}

impl GetResourceTreeParams { pub const METHOD: &'static str = "Page.getResourceTree"; }

impl<'a> crate::CdpCommand<'a> for GetResourceTreeParams {
    const METHOD: &'static str = "Page.getResourceTree";
    type Response = GetResourceTreeReturns<'a>;
}

/// Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HandleJavaScriptDialogParams<'a> {
    /// Whether to accept or dismiss the dialog.
    accept: bool,
    /// The text to enter into the dialog prompt before accepting. Used only if this is a prompt
    /// dialog.
    #[serde(skip_serializing_if = "Option::is_none", rename = "promptText")]
    prompt_text: Option<Cow<'a, str>>,
}

impl<'a> HandleJavaScriptDialogParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `accept`: Whether to accept or dismiss the dialog.
    pub fn builder(accept: bool) -> HandleJavaScriptDialogParamsBuilder<'a> {
        HandleJavaScriptDialogParamsBuilder {
            accept: accept,
            prompt_text: None,
        }
    }
    /// Whether to accept or dismiss the dialog.
    pub fn accept(&self) -> bool { self.accept }
    /// The text to enter into the dialog prompt before accepting. Used only if this is a prompt
    /// dialog.
    pub fn prompt_text(&self) -> Option<&str> { self.prompt_text.as_deref() }
}


pub struct HandleJavaScriptDialogParamsBuilder<'a> {
    accept: bool,
    prompt_text: Option<Cow<'a, str>>,
}

impl<'a> HandleJavaScriptDialogParamsBuilder<'a> {
    /// The text to enter into the dialog prompt before accepting. Used only if this is a prompt
    /// dialog.
    pub fn prompt_text(mut self, prompt_text: impl Into<Cow<'a, str>>) -> Self { self.prompt_text = Some(prompt_text.into()); self }
    pub fn build(self) -> HandleJavaScriptDialogParams<'a> {
        HandleJavaScriptDialogParams {
            accept: self.accept,
            prompt_text: self.prompt_text,
        }
    }
}

impl<'a> HandleJavaScriptDialogParams<'a> { pub const METHOD: &'static str = "Page.handleJavaScriptDialog"; }

impl<'a> crate::CdpCommand<'a> for HandleJavaScriptDialogParams<'a> {
    const METHOD: &'static str = "Page.handleJavaScriptDialog";
    type Response = crate::EmptyReturns;
}

/// Navigates current page to the given URL.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigateParams<'a> {
    /// URL to navigate the page to.
    url: Cow<'a, str>,
    /// Referrer URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    referrer: Option<Cow<'a, str>>,
    /// Intended transition type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitionType")]
    transition_type: Option<TransitionType>,
    /// Frame id to navigate, if not specified navigates the top frame.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<FrameId<'a>>,
    /// Referrer-policy used for the navigation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "referrerPolicy")]
    referrer_policy: Option<ReferrerPolicy>,
}

impl<'a> NavigateParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: URL to navigate the page to.
    pub fn builder(url: impl Into<Cow<'a, str>>) -> NavigateParamsBuilder<'a> {
        NavigateParamsBuilder {
            url: url.into(),
            referrer: None,
            transition_type: None,
            frame_id: None,
            referrer_policy: None,
        }
    }
    /// URL to navigate the page to.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Referrer URL.
    pub fn referrer(&self) -> Option<&str> { self.referrer.as_deref() }
    /// Intended transition type.
    pub fn transition_type(&self) -> Option<&TransitionType> { self.transition_type.as_ref() }
    /// Frame id to navigate, if not specified navigates the top frame.
    pub fn frame_id(&self) -> Option<&FrameId<'a>> { self.frame_id.as_ref() }
    /// Referrer-policy used for the navigation.
    pub fn referrer_policy(&self) -> Option<&ReferrerPolicy> { self.referrer_policy.as_ref() }
}


pub struct NavigateParamsBuilder<'a> {
    url: Cow<'a, str>,
    referrer: Option<Cow<'a, str>>,
    transition_type: Option<TransitionType>,
    frame_id: Option<FrameId<'a>>,
    referrer_policy: Option<ReferrerPolicy>,
}

impl<'a> NavigateParamsBuilder<'a> {
    /// Referrer URL.
    pub fn referrer(mut self, referrer: impl Into<Cow<'a, str>>) -> Self { self.referrer = Some(referrer.into()); self }
    /// Intended transition type.
    pub fn transition_type(mut self, transition_type: impl Into<TransitionType>) -> Self { self.transition_type = Some(transition_type.into()); self }
    /// Frame id to navigate, if not specified navigates the top frame.
    pub fn frame_id(mut self, frame_id: impl Into<FrameId<'a>>) -> Self { self.frame_id = Some(frame_id.into()); self }
    /// Referrer-policy used for the navigation.
    pub fn referrer_policy(mut self, referrer_policy: impl Into<ReferrerPolicy>) -> Self { self.referrer_policy = Some(referrer_policy.into()); self }
    pub fn build(self) -> NavigateParams<'a> {
        NavigateParams {
            url: self.url,
            referrer: self.referrer,
            transition_type: self.transition_type,
            frame_id: self.frame_id,
            referrer_policy: self.referrer_policy,
        }
    }
}

/// Navigates current page to the given URL.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigateReturns<'a> {
    /// Frame id that has navigated (or failed to navigate)
    #[serde(rename = "frameId")]
    frame_id: FrameId<'a>,
    /// Loader identifier. This is omitted in case of same-document navigation,
    /// as the previously committed loaderId would not change.
    #[serde(skip_serializing_if = "Option::is_none", rename = "loaderId")]
    loader_id: Option<crate::network::LoaderId<'a>>,
    /// User friendly error message, present if and only if navigation has failed.
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorText")]
    error_text: Option<Cow<'a, str>>,
    /// Whether the navigation resulted in a download.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isDownload")]
    is_download: Option<bool>,
}

impl<'a> NavigateReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: Frame id that has navigated (or failed to navigate)
    pub fn builder(frame_id: impl Into<FrameId<'a>>) -> NavigateReturnsBuilder<'a> {
        NavigateReturnsBuilder {
            frame_id: frame_id.into(),
            loader_id: None,
            error_text: None,
            is_download: None,
        }
    }
    /// Frame id that has navigated (or failed to navigate)
    pub fn frame_id(&self) -> &FrameId<'a> { &self.frame_id }
    /// Loader identifier. This is omitted in case of same-document navigation,
    /// as the previously committed loaderId would not change.
    pub fn loader_id(&self) -> Option<&crate::network::LoaderId<'a>> { self.loader_id.as_ref() }
    /// User friendly error message, present if and only if navigation has failed.
    pub fn error_text(&self) -> Option<&str> { self.error_text.as_deref() }
    /// Whether the navigation resulted in a download.
    pub fn is_download(&self) -> Option<bool> { self.is_download }
}


pub struct NavigateReturnsBuilder<'a> {
    frame_id: FrameId<'a>,
    loader_id: Option<crate::network::LoaderId<'a>>,
    error_text: Option<Cow<'a, str>>,
    is_download: Option<bool>,
}

impl<'a> NavigateReturnsBuilder<'a> {
    /// Loader identifier. This is omitted in case of same-document navigation,
    /// as the previously committed loaderId would not change.
    pub fn loader_id(mut self, loader_id: crate::network::LoaderId<'a>) -> Self { self.loader_id = Some(loader_id); self }
    /// User friendly error message, present if and only if navigation has failed.
    pub fn error_text(mut self, error_text: impl Into<Cow<'a, str>>) -> Self { self.error_text = Some(error_text.into()); self }
    /// Whether the navigation resulted in a download.
    pub fn is_download(mut self, is_download: bool) -> Self { self.is_download = Some(is_download); self }
    pub fn build(self) -> NavigateReturns<'a> {
        NavigateReturns {
            frame_id: self.frame_id,
            loader_id: self.loader_id,
            error_text: self.error_text,
            is_download: self.is_download,
        }
    }
}

impl<'a> NavigateParams<'a> { pub const METHOD: &'static str = "Page.navigate"; }

impl<'a> crate::CdpCommand<'a> for NavigateParams<'a> {
    const METHOD: &'static str = "Page.navigate";
    type Response = NavigateReturns<'a>;
}

/// Navigates current page to the given history entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigateToHistoryEntryParams {
    /// Unique id of the entry to navigate to.
    #[serde(rename = "entryId")]
    entry_id: u64,
}

impl NavigateToHistoryEntryParams {
    /// Creates a builder for this type with the required parameters:
    /// * `entry_id`: Unique id of the entry to navigate to.
    pub fn builder(entry_id: u64) -> NavigateToHistoryEntryParamsBuilder {
        NavigateToHistoryEntryParamsBuilder {
            entry_id: entry_id,
        }
    }
    /// Unique id of the entry to navigate to.
    pub fn entry_id(&self) -> u64 { self.entry_id }
}


pub struct NavigateToHistoryEntryParamsBuilder {
    entry_id: u64,
}

impl NavigateToHistoryEntryParamsBuilder {
    pub fn build(self) -> NavigateToHistoryEntryParams {
        NavigateToHistoryEntryParams {
            entry_id: self.entry_id,
        }
    }
}

impl NavigateToHistoryEntryParams { pub const METHOD: &'static str = "Page.navigateToHistoryEntry"; }

impl<'a> crate::CdpCommand<'a> for NavigateToHistoryEntryParams {
    const METHOD: &'static str = "Page.navigateToHistoryEntry";
    type Response = crate::EmptyReturns;
}

/// Print page as PDF.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrintToPDFParams<'a> {
    /// Paper orientation. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    landscape: Option<bool>,
    /// Display header and footer. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayHeaderFooter")]
    display_header_footer: Option<bool>,
    /// Print background graphics. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "printBackground")]
    print_background: Option<bool>,
    /// Scale of the webpage rendering. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f64>,
    /// Paper width in inches. Defaults to 8.5 inches.
    #[serde(skip_serializing_if = "Option::is_none", rename = "paperWidth")]
    paper_width: Option<f64>,
    /// Paper height in inches. Defaults to 11 inches.
    #[serde(skip_serializing_if = "Option::is_none", rename = "paperHeight")]
    paper_height: Option<f64>,
    /// Top margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginTop")]
    margin_top: Option<f64>,
    /// Bottom margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginBottom")]
    margin_bottom: Option<f64>,
    /// Left margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginLeft")]
    margin_left: Option<f64>,
    /// Right margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginRight")]
    margin_right: Option<f64>,
    /// Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are
    /// printed in the document order, not in the order specified, and no
    /// more than once.
    /// Defaults to empty string, which implies the entire document is printed.
    /// The page numbers are quietly capped to actual page count of the
    /// document, and ranges beyond the end of the document are ignored.
    /// If this results in no pages to print, an error is reported.
    /// It is an error to specify a range with start greater than end.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageRanges")]
    page_ranges: Option<Cow<'a, str>>,
    /// HTML template for the print header. Should be valid HTML markup with following
    /// classes used to inject printing values into them:
    /// - 'date': formatted print date
    /// - 'title': document title
    /// - 'url': document location
    /// - 'pageNumber': current page number
    /// - 'totalPages': total pages in the document
    /// 
    /// For example, '\<span class=title\>\</span\>' would generate span containing the title.
    #[serde(skip_serializing_if = "Option::is_none", rename = "headerTemplate")]
    header_template: Option<Cow<'a, str>>,
    /// HTML template for the print footer. Should use the same format as the 'headerTemplate'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "footerTemplate")]
    footer_template: Option<Cow<'a, str>>,
    /// Whether or not to prefer page size as defined by css. Defaults to false,
    /// in which case the content will be scaled to fit the paper size.
    #[serde(skip_serializing_if = "Option::is_none", rename = "preferCSSPageSize")]
    prefer_css_page_size: Option<bool>,
    /// return as stream
    #[serde(skip_serializing_if = "Option::is_none", rename = "transferMode")]
    transfer_mode: Option<Cow<'a, str>>,
    /// Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generateTaggedPDF")]
    generate_tagged_pdf: Option<bool>,
    /// Whether or not to embed the document outline into the PDF.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generateDocumentOutline")]
    generate_document_outline: Option<bool>,
}

impl<'a> PrintToPDFParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> PrintToPDFParamsBuilder<'a> {
        PrintToPDFParamsBuilder {
            landscape: None,
            display_header_footer: None,
            print_background: None,
            scale: None,
            paper_width: None,
            paper_height: None,
            margin_top: None,
            margin_bottom: None,
            margin_left: None,
            margin_right: None,
            page_ranges: None,
            header_template: None,
            footer_template: None,
            prefer_css_page_size: None,
            transfer_mode: None,
            generate_tagged_pdf: None,
            generate_document_outline: None,
        }
    }
    /// Paper orientation. Defaults to false.
    pub fn landscape(&self) -> Option<bool> { self.landscape }
    /// Display header and footer. Defaults to false.
    pub fn display_header_footer(&self) -> Option<bool> { self.display_header_footer }
    /// Print background graphics. Defaults to false.
    pub fn print_background(&self) -> Option<bool> { self.print_background }
    /// Scale of the webpage rendering. Defaults to 1.
    pub fn scale(&self) -> Option<f64> { self.scale }
    /// Paper width in inches. Defaults to 8.5 inches.
    pub fn paper_width(&self) -> Option<f64> { self.paper_width }
    /// Paper height in inches. Defaults to 11 inches.
    pub fn paper_height(&self) -> Option<f64> { self.paper_height }
    /// Top margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn margin_top(&self) -> Option<f64> { self.margin_top }
    /// Bottom margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn margin_bottom(&self) -> Option<f64> { self.margin_bottom }
    /// Left margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn margin_left(&self) -> Option<f64> { self.margin_left }
    /// Right margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn margin_right(&self) -> Option<f64> { self.margin_right }
    /// Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are
    /// printed in the document order, not in the order specified, and no
    /// more than once.
    /// Defaults to empty string, which implies the entire document is printed.
    /// The page numbers are quietly capped to actual page count of the
    /// document, and ranges beyond the end of the document are ignored.
    /// If this results in no pages to print, an error is reported.
    /// It is an error to specify a range with start greater than end.
    pub fn page_ranges(&self) -> Option<&str> { self.page_ranges.as_deref() }
    /// HTML template for the print header. Should be valid HTML markup with following
    /// classes used to inject printing values into them:
    /// - 'date': formatted print date
    /// - 'title': document title
    /// - 'url': document location
    /// - 'pageNumber': current page number
    /// - 'totalPages': total pages in the document
    /// 
    /// For example, '\<span class=title\>\</span\>' would generate span containing the title.
    pub fn header_template(&self) -> Option<&str> { self.header_template.as_deref() }
    /// HTML template for the print footer. Should use the same format as the 'headerTemplate'.
    pub fn footer_template(&self) -> Option<&str> { self.footer_template.as_deref() }
    /// Whether or not to prefer page size as defined by css. Defaults to false,
    /// in which case the content will be scaled to fit the paper size.
    pub fn prefer_css_page_size(&self) -> Option<bool> { self.prefer_css_page_size }
    /// return as stream
    pub fn transfer_mode(&self) -> Option<&str> { self.transfer_mode.as_deref() }
    /// Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice.
    pub fn generate_tagged_pdf(&self) -> Option<bool> { self.generate_tagged_pdf }
    /// Whether or not to embed the document outline into the PDF.
    pub fn generate_document_outline(&self) -> Option<bool> { self.generate_document_outline }
}

#[derive(Default)]
pub struct PrintToPDFParamsBuilder<'a> {
    landscape: Option<bool>,
    display_header_footer: Option<bool>,
    print_background: Option<bool>,
    scale: Option<f64>,
    paper_width: Option<f64>,
    paper_height: Option<f64>,
    margin_top: Option<f64>,
    margin_bottom: Option<f64>,
    margin_left: Option<f64>,
    margin_right: Option<f64>,
    page_ranges: Option<Cow<'a, str>>,
    header_template: Option<Cow<'a, str>>,
    footer_template: Option<Cow<'a, str>>,
    prefer_css_page_size: Option<bool>,
    transfer_mode: Option<Cow<'a, str>>,
    generate_tagged_pdf: Option<bool>,
    generate_document_outline: Option<bool>,
}

impl<'a> PrintToPDFParamsBuilder<'a> {
    /// Paper orientation. Defaults to false.
    pub fn landscape(mut self, landscape: bool) -> Self { self.landscape = Some(landscape); self }
    /// Display header and footer. Defaults to false.
    pub fn display_header_footer(mut self, display_header_footer: bool) -> Self { self.display_header_footer = Some(display_header_footer); self }
    /// Print background graphics. Defaults to false.
    pub fn print_background(mut self, print_background: bool) -> Self { self.print_background = Some(print_background); self }
    /// Scale of the webpage rendering. Defaults to 1.
    pub fn scale(mut self, scale: f64) -> Self { self.scale = Some(scale); self }
    /// Paper width in inches. Defaults to 8.5 inches.
    pub fn paper_width(mut self, paper_width: f64) -> Self { self.paper_width = Some(paper_width); self }
    /// Paper height in inches. Defaults to 11 inches.
    pub fn paper_height(mut self, paper_height: f64) -> Self { self.paper_height = Some(paper_height); self }
    /// Top margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn margin_top(mut self, margin_top: f64) -> Self { self.margin_top = Some(margin_top); self }
    /// Bottom margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn margin_bottom(mut self, margin_bottom: f64) -> Self { self.margin_bottom = Some(margin_bottom); self }
    /// Left margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn margin_left(mut self, margin_left: f64) -> Self { self.margin_left = Some(margin_left); self }
    /// Right margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn margin_right(mut self, margin_right: f64) -> Self { self.margin_right = Some(margin_right); self }
    /// Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are
    /// printed in the document order, not in the order specified, and no
    /// more than once.
    /// Defaults to empty string, which implies the entire document is printed.
    /// The page numbers are quietly capped to actual page count of the
    /// document, and ranges beyond the end of the document are ignored.
    /// If this results in no pages to print, an error is reported.
    /// It is an error to specify a range with start greater than end.
    pub fn page_ranges(mut self, page_ranges: impl Into<Cow<'a, str>>) -> Self { self.page_ranges = Some(page_ranges.into()); self }
    /// HTML template for the print header. Should be valid HTML markup with following
    /// classes used to inject printing values into them:
    /// - 'date': formatted print date
    /// - 'title': document title
    /// - 'url': document location
    /// - 'pageNumber': current page number
    /// - 'totalPages': total pages in the document
    /// 
    /// For example, '\<span class=title\>\</span\>' would generate span containing the title.
    pub fn header_template(mut self, header_template: impl Into<Cow<'a, str>>) -> Self { self.header_template = Some(header_template.into()); self }
    /// HTML template for the print footer. Should use the same format as the 'headerTemplate'.
    pub fn footer_template(mut self, footer_template: impl Into<Cow<'a, str>>) -> Self { self.footer_template = Some(footer_template.into()); self }
    /// Whether or not to prefer page size as defined by css. Defaults to false,
    /// in which case the content will be scaled to fit the paper size.
    pub fn prefer_css_page_size(mut self, prefer_css_page_size: bool) -> Self { self.prefer_css_page_size = Some(prefer_css_page_size); self }
    /// return as stream
    pub fn transfer_mode(mut self, transfer_mode: impl Into<Cow<'a, str>>) -> Self { self.transfer_mode = Some(transfer_mode.into()); self }
    /// Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice.
    pub fn generate_tagged_pdf(mut self, generate_tagged_pdf: bool) -> Self { self.generate_tagged_pdf = Some(generate_tagged_pdf); self }
    /// Whether or not to embed the document outline into the PDF.
    pub fn generate_document_outline(mut self, generate_document_outline: bool) -> Self { self.generate_document_outline = Some(generate_document_outline); self }
    pub fn build(self) -> PrintToPDFParams<'a> {
        PrintToPDFParams {
            landscape: self.landscape,
            display_header_footer: self.display_header_footer,
            print_background: self.print_background,
            scale: self.scale,
            paper_width: self.paper_width,
            paper_height: self.paper_height,
            margin_top: self.margin_top,
            margin_bottom: self.margin_bottom,
            margin_left: self.margin_left,
            margin_right: self.margin_right,
            page_ranges: self.page_ranges,
            header_template: self.header_template,
            footer_template: self.footer_template,
            prefer_css_page_size: self.prefer_css_page_size,
            transfer_mode: self.transfer_mode,
            generate_tagged_pdf: self.generate_tagged_pdf,
            generate_document_outline: self.generate_document_outline,
        }
    }
}

/// Print page as PDF.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrintToPDFReturns<'a> {
    /// Base64-encoded pdf data. Empty if |returnAsStream| is specified. (Encoded as a base64 string when passed over JSON)
    data: Cow<'a, str>,
    /// A handle of the stream that holds resulting PDF data.
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<crate::io::StreamHandle<'a>>,
}

impl<'a> PrintToPDFReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `data`: Base64-encoded pdf data. Empty if |returnAsStream| is specified. (Encoded as a base64 string when passed over JSON)
    pub fn builder(data: impl Into<Cow<'a, str>>) -> PrintToPDFReturnsBuilder<'a> {
        PrintToPDFReturnsBuilder {
            data: data.into(),
            stream: None,
        }
    }
    /// Base64-encoded pdf data. Empty if |returnAsStream| is specified. (Encoded as a base64 string when passed over JSON)
    pub fn data(&self) -> &str { self.data.as_ref() }
    /// A handle of the stream that holds resulting PDF data.
    pub fn stream(&self) -> Option<&crate::io::StreamHandle<'a>> { self.stream.as_ref() }
}


pub struct PrintToPDFReturnsBuilder<'a> {
    data: Cow<'a, str>,
    stream: Option<crate::io::StreamHandle<'a>>,
}

impl<'a> PrintToPDFReturnsBuilder<'a> {
    /// A handle of the stream that holds resulting PDF data.
    pub fn stream(mut self, stream: crate::io::StreamHandle<'a>) -> Self { self.stream = Some(stream); self }
    pub fn build(self) -> PrintToPDFReturns<'a> {
        PrintToPDFReturns {
            data: self.data,
            stream: self.stream,
        }
    }
}

impl<'a> PrintToPDFParams<'a> { pub const METHOD: &'static str = "Page.printToPDF"; }

impl<'a> crate::CdpCommand<'a> for PrintToPDFParams<'a> {
    const METHOD: &'static str = "Page.printToPDF";
    type Response = PrintToPDFReturns<'a>;
}

/// Reloads given page optionally ignoring the cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReloadParams<'a> {
    /// If true, browser cache is ignored (as if the user pressed Shift+refresh).
    #[serde(skip_serializing_if = "Option::is_none", rename = "ignoreCache")]
    ignore_cache: Option<bool>,
    /// If set, the script will be injected into all frames of the inspected page after reload.
    /// Argument will be ignored if reloading dataURL origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptToEvaluateOnLoad")]
    script_to_evaluate_on_load: Option<Cow<'a, str>>,
    /// If set, an error will be thrown if the target page's main frame's
    /// loader id does not match the provided id. This prevents accidentally
    /// reloading an unintended target in case there's a racing navigation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "loaderId")]
    loader_id: Option<crate::network::LoaderId<'a>>,
}

impl<'a> ReloadParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> ReloadParamsBuilder<'a> {
        ReloadParamsBuilder {
            ignore_cache: None,
            script_to_evaluate_on_load: None,
            loader_id: None,
        }
    }
    /// If true, browser cache is ignored (as if the user pressed Shift+refresh).
    pub fn ignore_cache(&self) -> Option<bool> { self.ignore_cache }
    /// If set, the script will be injected into all frames of the inspected page after reload.
    /// Argument will be ignored if reloading dataURL origin.
    pub fn script_to_evaluate_on_load(&self) -> Option<&str> { self.script_to_evaluate_on_load.as_deref() }
    /// If set, an error will be thrown if the target page's main frame's
    /// loader id does not match the provided id. This prevents accidentally
    /// reloading an unintended target in case there's a racing navigation.
    pub fn loader_id(&self) -> Option<&crate::network::LoaderId<'a>> { self.loader_id.as_ref() }
}

#[derive(Default)]
pub struct ReloadParamsBuilder<'a> {
    ignore_cache: Option<bool>,
    script_to_evaluate_on_load: Option<Cow<'a, str>>,
    loader_id: Option<crate::network::LoaderId<'a>>,
}

impl<'a> ReloadParamsBuilder<'a> {
    /// If true, browser cache is ignored (as if the user pressed Shift+refresh).
    pub fn ignore_cache(mut self, ignore_cache: bool) -> Self { self.ignore_cache = Some(ignore_cache); self }
    /// If set, the script will be injected into all frames of the inspected page after reload.
    /// Argument will be ignored if reloading dataURL origin.
    pub fn script_to_evaluate_on_load(mut self, script_to_evaluate_on_load: impl Into<Cow<'a, str>>) -> Self { self.script_to_evaluate_on_load = Some(script_to_evaluate_on_load.into()); self }
    /// If set, an error will be thrown if the target page's main frame's
    /// loader id does not match the provided id. This prevents accidentally
    /// reloading an unintended target in case there's a racing navigation.
    pub fn loader_id(mut self, loader_id: crate::network::LoaderId<'a>) -> Self { self.loader_id = Some(loader_id); self }
    pub fn build(self) -> ReloadParams<'a> {
        ReloadParams {
            ignore_cache: self.ignore_cache,
            script_to_evaluate_on_load: self.script_to_evaluate_on_load,
            loader_id: self.loader_id,
        }
    }
}

impl<'a> ReloadParams<'a> { pub const METHOD: &'static str = "Page.reload"; }

impl<'a> crate::CdpCommand<'a> for ReloadParams<'a> {
    const METHOD: &'static str = "Page.reload";
    type Response = crate::EmptyReturns;
}

/// Deprecated, please use removeScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScriptToEvaluateOnLoadParams<'a> {
    identifier: ScriptIdentifier<'a>,
}

impl<'a> RemoveScriptToEvaluateOnLoadParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `identifier`: 
    pub fn builder(identifier: impl Into<ScriptIdentifier<'a>>) -> RemoveScriptToEvaluateOnLoadParamsBuilder<'a> {
        RemoveScriptToEvaluateOnLoadParamsBuilder {
            identifier: identifier.into(),
        }
    }
    pub fn identifier(&self) -> &ScriptIdentifier<'a> { &self.identifier }
}


pub struct RemoveScriptToEvaluateOnLoadParamsBuilder<'a> {
    identifier: ScriptIdentifier<'a>,
}

impl<'a> RemoveScriptToEvaluateOnLoadParamsBuilder<'a> {
    pub fn build(self) -> RemoveScriptToEvaluateOnLoadParams<'a> {
        RemoveScriptToEvaluateOnLoadParams {
            identifier: self.identifier,
        }
    }
}

impl<'a> RemoveScriptToEvaluateOnLoadParams<'a> { pub const METHOD: &'static str = "Page.removeScriptToEvaluateOnLoad"; }

impl<'a> crate::CdpCommand<'a> for RemoveScriptToEvaluateOnLoadParams<'a> {
    const METHOD: &'static str = "Page.removeScriptToEvaluateOnLoad";
    type Response = crate::EmptyReturns;
}

/// Removes given script from the list.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScriptToEvaluateOnNewDocumentParams<'a> {
    identifier: ScriptIdentifier<'a>,
}

impl<'a> RemoveScriptToEvaluateOnNewDocumentParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `identifier`: 
    pub fn builder(identifier: impl Into<ScriptIdentifier<'a>>) -> RemoveScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
        RemoveScriptToEvaluateOnNewDocumentParamsBuilder {
            identifier: identifier.into(),
        }
    }
    pub fn identifier(&self) -> &ScriptIdentifier<'a> { &self.identifier }
}


pub struct RemoveScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
    identifier: ScriptIdentifier<'a>,
}

impl<'a> RemoveScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
    pub fn build(self) -> RemoveScriptToEvaluateOnNewDocumentParams<'a> {
        RemoveScriptToEvaluateOnNewDocumentParams {
            identifier: self.identifier,
        }
    }
}

impl<'a> RemoveScriptToEvaluateOnNewDocumentParams<'a> { pub const METHOD: &'static str = "Page.removeScriptToEvaluateOnNewDocument"; }

impl<'a> crate::CdpCommand<'a> for RemoveScriptToEvaluateOnNewDocumentParams<'a> {
    const METHOD: &'static str = "Page.removeScriptToEvaluateOnNewDocument";
    type Response = crate::EmptyReturns;
}

/// Acknowledges that a screencast frame has been received by the frontend.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameAckParams {
    /// Frame number.
    #[serde(rename = "sessionId")]
    session_id: u64,
}

impl ScreencastFrameAckParams {
    /// Creates a builder for this type with the required parameters:
    /// * `session_id`: Frame number.
    pub fn builder(session_id: u64) -> ScreencastFrameAckParamsBuilder {
        ScreencastFrameAckParamsBuilder {
            session_id: session_id,
        }
    }
    /// Frame number.
    pub fn session_id(&self) -> u64 { self.session_id }
}


pub struct ScreencastFrameAckParamsBuilder {
    session_id: u64,
}

impl ScreencastFrameAckParamsBuilder {
    pub fn build(self) -> ScreencastFrameAckParams {
        ScreencastFrameAckParams {
            session_id: self.session_id,
        }
    }
}

impl ScreencastFrameAckParams { pub const METHOD: &'static str = "Page.screencastFrameAck"; }

impl<'a> crate::CdpCommand<'a> for ScreencastFrameAckParams {
    const METHOD: &'static str = "Page.screencastFrameAck";
    type Response = crate::EmptyReturns;
}

/// Searches for given string in resource content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResourceParams<'a> {
    /// Frame id for resource to search in.
    #[serde(rename = "frameId")]
    frame_id: FrameId<'a>,
    /// URL of the resource to search in.
    url: Cow<'a, str>,
    /// String to search for.
    query: Cow<'a, str>,
    /// If true, search is case sensitive.
    #[serde(skip_serializing_if = "Option::is_none", rename = "caseSensitive")]
    case_sensitive: Option<bool>,
    /// If true, treats string parameter as regex.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isRegex")]
    is_regex: Option<bool>,
}

impl<'a> SearchInResourceParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: Frame id for resource to search in.
    /// * `url`: URL of the resource to search in.
    /// * `query`: String to search for.
    pub fn builder(frame_id: impl Into<FrameId<'a>>, url: impl Into<Cow<'a, str>>, query: impl Into<Cow<'a, str>>) -> SearchInResourceParamsBuilder<'a> {
        SearchInResourceParamsBuilder {
            frame_id: frame_id.into(),
            url: url.into(),
            query: query.into(),
            case_sensitive: None,
            is_regex: None,
        }
    }
    /// Frame id for resource to search in.
    pub fn frame_id(&self) -> &FrameId<'a> { &self.frame_id }
    /// URL of the resource to search in.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// String to search for.
    pub fn query(&self) -> &str { self.query.as_ref() }
    /// If true, search is case sensitive.
    pub fn case_sensitive(&self) -> Option<bool> { self.case_sensitive }
    /// If true, treats string parameter as regex.
    pub fn is_regex(&self) -> Option<bool> { self.is_regex }
}


pub struct SearchInResourceParamsBuilder<'a> {
    frame_id: FrameId<'a>,
    url: Cow<'a, str>,
    query: Cow<'a, str>,
    case_sensitive: Option<bool>,
    is_regex: Option<bool>,
}

impl<'a> SearchInResourceParamsBuilder<'a> {
    /// If true, search is case sensitive.
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self { self.case_sensitive = Some(case_sensitive); self }
    /// If true, treats string parameter as regex.
    pub fn is_regex(mut self, is_regex: bool) -> Self { self.is_regex = Some(is_regex); self }
    pub fn build(self) -> SearchInResourceParams<'a> {
        SearchInResourceParams {
            frame_id: self.frame_id,
            url: self.url,
            query: self.query,
            case_sensitive: self.case_sensitive,
            is_regex: self.is_regex,
        }
    }
}

/// Searches for given string in resource content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResourceReturns {
    /// List of search matches.
    result: Vec<crate::debugger::SearchMatch>,
}

impl SearchInResourceReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: List of search matches.
    pub fn builder(result: Vec<crate::debugger::SearchMatch>) -> SearchInResourceReturnsBuilder {
        SearchInResourceReturnsBuilder {
            result: result,
        }
    }
    /// List of search matches.
    pub fn result(&self) -> &[crate::debugger::SearchMatch] { &self.result }
}


pub struct SearchInResourceReturnsBuilder {
    result: Vec<crate::debugger::SearchMatch>,
}

impl SearchInResourceReturnsBuilder {
    pub fn build(self) -> SearchInResourceReturns {
        SearchInResourceReturns {
            result: self.result,
        }
    }
}

impl<'a> SearchInResourceParams<'a> { pub const METHOD: &'static str = "Page.searchInResource"; }

impl<'a> crate::CdpCommand<'a> for SearchInResourceParams<'a> {
    const METHOD: &'static str = "Page.searchInResource";
    type Response = SearchInResourceReturns;
}

/// Enable Chrome's experimental ad filter on all sites.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAdBlockingEnabledParams {
    /// Whether to block ads.
    enabled: bool,
}

impl SetAdBlockingEnabledParams {
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: Whether to block ads.
    pub fn builder(enabled: bool) -> SetAdBlockingEnabledParamsBuilder {
        SetAdBlockingEnabledParamsBuilder {
            enabled: enabled,
        }
    }
    /// Whether to block ads.
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetAdBlockingEnabledParamsBuilder {
    enabled: bool,
}

impl SetAdBlockingEnabledParamsBuilder {
    pub fn build(self) -> SetAdBlockingEnabledParams {
        SetAdBlockingEnabledParams {
            enabled: self.enabled,
        }
    }
}

impl SetAdBlockingEnabledParams { pub const METHOD: &'static str = "Page.setAdBlockingEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetAdBlockingEnabledParams {
    const METHOD: &'static str = "Page.setAdBlockingEnabled";
    type Response = crate::EmptyReturns;
}

/// Enable page Content Security Policy by-passing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBypassCSPParams {
    /// Whether to bypass page CSP.
    enabled: bool,
}

impl SetBypassCSPParams {
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: Whether to bypass page CSP.
    pub fn builder(enabled: bool) -> SetBypassCSPParamsBuilder {
        SetBypassCSPParamsBuilder {
            enabled: enabled,
        }
    }
    /// Whether to bypass page CSP.
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetBypassCSPParamsBuilder {
    enabled: bool,
}

impl SetBypassCSPParamsBuilder {
    pub fn build(self) -> SetBypassCSPParams {
        SetBypassCSPParams {
            enabled: self.enabled,
        }
    }
}

impl SetBypassCSPParams { pub const METHOD: &'static str = "Page.setBypassCSP"; }

impl<'a> crate::CdpCommand<'a> for SetBypassCSPParams {
    const METHOD: &'static str = "Page.setBypassCSP";
    type Response = crate::EmptyReturns;
}

/// Get Permissions Policy state on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPermissionsPolicyStateParams<'a> {
    #[serde(rename = "frameId")]
    frame_id: FrameId<'a>,
}

impl<'a> GetPermissionsPolicyStateParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: 
    pub fn builder(frame_id: impl Into<FrameId<'a>>) -> GetPermissionsPolicyStateParamsBuilder<'a> {
        GetPermissionsPolicyStateParamsBuilder {
            frame_id: frame_id.into(),
        }
    }
    pub fn frame_id(&self) -> &FrameId<'a> { &self.frame_id }
}


pub struct GetPermissionsPolicyStateParamsBuilder<'a> {
    frame_id: FrameId<'a>,
}

impl<'a> GetPermissionsPolicyStateParamsBuilder<'a> {
    pub fn build(self) -> GetPermissionsPolicyStateParams<'a> {
        GetPermissionsPolicyStateParams {
            frame_id: self.frame_id,
        }
    }
}

/// Get Permissions Policy state on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPermissionsPolicyStateReturns<'a> {
    states: Vec<PermissionsPolicyFeatureState<'a>>,
}

impl<'a> GetPermissionsPolicyStateReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `states`: 
    pub fn builder(states: Vec<PermissionsPolicyFeatureState<'a>>) -> GetPermissionsPolicyStateReturnsBuilder<'a> {
        GetPermissionsPolicyStateReturnsBuilder {
            states: states,
        }
    }
    pub fn states(&self) -> &[PermissionsPolicyFeatureState<'a>] { &self.states }
}


pub struct GetPermissionsPolicyStateReturnsBuilder<'a> {
    states: Vec<PermissionsPolicyFeatureState<'a>>,
}

impl<'a> GetPermissionsPolicyStateReturnsBuilder<'a> {
    pub fn build(self) -> GetPermissionsPolicyStateReturns<'a> {
        GetPermissionsPolicyStateReturns {
            states: self.states,
        }
    }
}

impl<'a> GetPermissionsPolicyStateParams<'a> { pub const METHOD: &'static str = "Page.getPermissionsPolicyState"; }

impl<'a> crate::CdpCommand<'a> for GetPermissionsPolicyStateParams<'a> {
    const METHOD: &'static str = "Page.getPermissionsPolicyState";
    type Response = GetPermissionsPolicyStateReturns<'a>;
}

/// Get Origin Trials on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOriginTrialsParams<'a> {
    #[serde(rename = "frameId")]
    frame_id: FrameId<'a>,
}

impl<'a> GetOriginTrialsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: 
    pub fn builder(frame_id: impl Into<FrameId<'a>>) -> GetOriginTrialsParamsBuilder<'a> {
        GetOriginTrialsParamsBuilder {
            frame_id: frame_id.into(),
        }
    }
    pub fn frame_id(&self) -> &FrameId<'a> { &self.frame_id }
}


pub struct GetOriginTrialsParamsBuilder<'a> {
    frame_id: FrameId<'a>,
}

impl<'a> GetOriginTrialsParamsBuilder<'a> {
    pub fn build(self) -> GetOriginTrialsParams<'a> {
        GetOriginTrialsParams {
            frame_id: self.frame_id,
        }
    }
}

/// Get Origin Trials on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOriginTrialsReturns<'a> {
    #[serde(rename = "originTrials")]
    origin_trials: Vec<OriginTrial<'a>>,
}

impl<'a> GetOriginTrialsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin_trials`: 
    pub fn builder(origin_trials: Vec<OriginTrial<'a>>) -> GetOriginTrialsReturnsBuilder<'a> {
        GetOriginTrialsReturnsBuilder {
            origin_trials: origin_trials,
        }
    }
    pub fn origin_trials(&self) -> &[OriginTrial<'a>] { &self.origin_trials }
}


pub struct GetOriginTrialsReturnsBuilder<'a> {
    origin_trials: Vec<OriginTrial<'a>>,
}

impl<'a> GetOriginTrialsReturnsBuilder<'a> {
    pub fn build(self) -> GetOriginTrialsReturns<'a> {
        GetOriginTrialsReturns {
            origin_trials: self.origin_trials,
        }
    }
}

impl<'a> GetOriginTrialsParams<'a> { pub const METHOD: &'static str = "Page.getOriginTrials"; }

impl<'a> crate::CdpCommand<'a> for GetOriginTrialsParams<'a> {
    const METHOD: &'static str = "Page.getOriginTrials";
    type Response = GetOriginTrialsReturns<'a>;
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
    screen_orientation: Option<crate::emulation::ScreenOrientation<'a>>,
    /// The viewport dimensions and scale. If not set, the override is cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    viewport: Option<Viewport>,
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
    pub fn screen_orientation(&self) -> Option<&crate::emulation::ScreenOrientation<'a>> { self.screen_orientation.as_ref() }
    /// The viewport dimensions and scale. If not set, the override is cleared.
    pub fn viewport(&self) -> Option<&Viewport> { self.viewport.as_ref() }
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
    screen_orientation: Option<crate::emulation::ScreenOrientation<'a>>,
    viewport: Option<Viewport>,
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
    pub fn screen_orientation(mut self, screen_orientation: crate::emulation::ScreenOrientation<'a>) -> Self { self.screen_orientation = Some(screen_orientation); self }
    /// The viewport dimensions and scale. If not set, the override is cleared.
    pub fn viewport(mut self, viewport: Viewport) -> Self { self.viewport = Some(viewport); self }
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
        }
    }
}

impl<'a> SetDeviceMetricsOverrideParams<'a> { pub const METHOD: &'static str = "Page.setDeviceMetricsOverride"; }

impl<'a> crate::CdpCommand<'a> for SetDeviceMetricsOverrideParams<'a> {
    const METHOD: &'static str = "Page.setDeviceMetricsOverride";
    type Response = crate::EmptyReturns;
}

/// Overrides the Device Orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceOrientationOverrideParams {
    /// Mock alpha
    alpha: f64,
    /// Mock beta
    beta: f64,
    /// Mock gamma
    gamma: f64,
}

impl SetDeviceOrientationOverrideParams {
    /// Creates a builder for this type with the required parameters:
    /// * `alpha`: Mock alpha
    /// * `beta`: Mock beta
    /// * `gamma`: Mock gamma
    pub fn builder(alpha: f64, beta: f64, gamma: f64) -> SetDeviceOrientationOverrideParamsBuilder {
        SetDeviceOrientationOverrideParamsBuilder {
            alpha: alpha,
            beta: beta,
            gamma: gamma,
        }
    }
    /// Mock alpha
    pub fn alpha(&self) -> f64 { self.alpha }
    /// Mock beta
    pub fn beta(&self) -> f64 { self.beta }
    /// Mock gamma
    pub fn gamma(&self) -> f64 { self.gamma }
}


pub struct SetDeviceOrientationOverrideParamsBuilder {
    alpha: f64,
    beta: f64,
    gamma: f64,
}

impl SetDeviceOrientationOverrideParamsBuilder {
    pub fn build(self) -> SetDeviceOrientationOverrideParams {
        SetDeviceOrientationOverrideParams {
            alpha: self.alpha,
            beta: self.beta,
            gamma: self.gamma,
        }
    }
}

impl SetDeviceOrientationOverrideParams { pub const METHOD: &'static str = "Page.setDeviceOrientationOverride"; }

impl<'a> crate::CdpCommand<'a> for SetDeviceOrientationOverrideParams {
    const METHOD: &'static str = "Page.setDeviceOrientationOverride";
    type Response = crate::EmptyReturns;
}

/// Set generic font families.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFontFamiliesParams<'a> {
    /// Specifies font families to set. If a font family is not specified, it won't be changed.
    #[serde(rename = "fontFamilies")]
    font_families: FontFamilies<'a>,
    /// Specifies font families to set for individual scripts.
    #[serde(skip_serializing_if = "Option::is_none", rename = "forScripts")]
    for_scripts: Option<Vec<ScriptFontFamilies<'a>>>,
}

impl<'a> SetFontFamiliesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `font_families`: Specifies font families to set. If a font family is not specified, it won't be changed.
    pub fn builder(font_families: FontFamilies<'a>) -> SetFontFamiliesParamsBuilder<'a> {
        SetFontFamiliesParamsBuilder {
            font_families: font_families,
            for_scripts: None,
        }
    }
    /// Specifies font families to set. If a font family is not specified, it won't be changed.
    pub fn font_families(&self) -> &FontFamilies<'a> { &self.font_families }
    /// Specifies font families to set for individual scripts.
    pub fn for_scripts(&self) -> Option<&[ScriptFontFamilies<'a>]> { self.for_scripts.as_deref() }
}


pub struct SetFontFamiliesParamsBuilder<'a> {
    font_families: FontFamilies<'a>,
    for_scripts: Option<Vec<ScriptFontFamilies<'a>>>,
}

impl<'a> SetFontFamiliesParamsBuilder<'a> {
    /// Specifies font families to set for individual scripts.
    pub fn for_scripts(mut self, for_scripts: Vec<ScriptFontFamilies<'a>>) -> Self { self.for_scripts = Some(for_scripts); self }
    pub fn build(self) -> SetFontFamiliesParams<'a> {
        SetFontFamiliesParams {
            font_families: self.font_families,
            for_scripts: self.for_scripts,
        }
    }
}

impl<'a> SetFontFamiliesParams<'a> { pub const METHOD: &'static str = "Page.setFontFamilies"; }

impl<'a> crate::CdpCommand<'a> for SetFontFamiliesParams<'a> {
    const METHOD: &'static str = "Page.setFontFamilies";
    type Response = crate::EmptyReturns;
}

/// Set default font sizes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFontSizesParams {
    /// Specifies font sizes to set. If a font size is not specified, it won't be changed.
    #[serde(rename = "fontSizes")]
    font_sizes: FontSizes,
}

impl SetFontSizesParams {
    /// Creates a builder for this type with the required parameters:
    /// * `font_sizes`: Specifies font sizes to set. If a font size is not specified, it won't be changed.
    pub fn builder(font_sizes: FontSizes) -> SetFontSizesParamsBuilder {
        SetFontSizesParamsBuilder {
            font_sizes: font_sizes,
        }
    }
    /// Specifies font sizes to set. If a font size is not specified, it won't be changed.
    pub fn font_sizes(&self) -> &FontSizes { &self.font_sizes }
}


pub struct SetFontSizesParamsBuilder {
    font_sizes: FontSizes,
}

impl SetFontSizesParamsBuilder {
    pub fn build(self) -> SetFontSizesParams {
        SetFontSizesParams {
            font_sizes: self.font_sizes,
        }
    }
}

impl SetFontSizesParams { pub const METHOD: &'static str = "Page.setFontSizes"; }

impl<'a> crate::CdpCommand<'a> for SetFontSizesParams {
    const METHOD: &'static str = "Page.setFontSizes";
    type Response = crate::EmptyReturns;
}

/// Sets given markup as the document's HTML.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentContentParams<'a> {
    /// Frame id to set HTML for.
    #[serde(rename = "frameId")]
    frame_id: FrameId<'a>,
    /// HTML content to set.
    html: Cow<'a, str>,
}

impl<'a> SetDocumentContentParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: Frame id to set HTML for.
    /// * `html`: HTML content to set.
    pub fn builder(frame_id: impl Into<FrameId<'a>>, html: impl Into<Cow<'a, str>>) -> SetDocumentContentParamsBuilder<'a> {
        SetDocumentContentParamsBuilder {
            frame_id: frame_id.into(),
            html: html.into(),
        }
    }
    /// Frame id to set HTML for.
    pub fn frame_id(&self) -> &FrameId<'a> { &self.frame_id }
    /// HTML content to set.
    pub fn html(&self) -> &str { self.html.as_ref() }
}


pub struct SetDocumentContentParamsBuilder<'a> {
    frame_id: FrameId<'a>,
    html: Cow<'a, str>,
}

impl<'a> SetDocumentContentParamsBuilder<'a> {
    pub fn build(self) -> SetDocumentContentParams<'a> {
        SetDocumentContentParams {
            frame_id: self.frame_id,
            html: self.html,
        }
    }
}

impl<'a> SetDocumentContentParams<'a> { pub const METHOD: &'static str = "Page.setDocumentContent"; }

impl<'a> crate::CdpCommand<'a> for SetDocumentContentParams<'a> {
    const METHOD: &'static str = "Page.setDocumentContent";
    type Response = crate::EmptyReturns;
}

/// Set the behavior when downloading a file.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDownloadBehaviorParams<'a> {
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny).
    behavior: Cow<'a, str>,
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    #[serde(skip_serializing_if = "Option::is_none", rename = "downloadPath")]
    download_path: Option<Cow<'a, str>>,
}

impl<'a> SetDownloadBehaviorParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `behavior`: Whether to allow all or deny all download requests, or use default Chrome behavior if available (otherwise deny).
    pub fn builder(behavior: impl Into<Cow<'a, str>>) -> SetDownloadBehaviorParamsBuilder<'a> {
        SetDownloadBehaviorParamsBuilder {
            behavior: behavior.into(),
            download_path: None,
        }
    }
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny).
    pub fn behavior(&self) -> &str { self.behavior.as_ref() }
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    pub fn download_path(&self) -> Option<&str> { self.download_path.as_deref() }
}


pub struct SetDownloadBehaviorParamsBuilder<'a> {
    behavior: Cow<'a, str>,
    download_path: Option<Cow<'a, str>>,
}

impl<'a> SetDownloadBehaviorParamsBuilder<'a> {
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    pub fn download_path(mut self, download_path: impl Into<Cow<'a, str>>) -> Self { self.download_path = Some(download_path.into()); self }
    pub fn build(self) -> SetDownloadBehaviorParams<'a> {
        SetDownloadBehaviorParams {
            behavior: self.behavior,
            download_path: self.download_path,
        }
    }
}

impl<'a> SetDownloadBehaviorParams<'a> { pub const METHOD: &'static str = "Page.setDownloadBehavior"; }

impl<'a> crate::CdpCommand<'a> for SetDownloadBehaviorParams<'a> {
    const METHOD: &'static str = "Page.setDownloadBehavior";
    type Response = crate::EmptyReturns;
}

/// Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position
/// unavailable.

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
}

impl SetGeolocationOverrideParams {
    /// Creates a builder for this type.
    pub fn builder() -> SetGeolocationOverrideParamsBuilder {
        SetGeolocationOverrideParamsBuilder {
            latitude: None,
            longitude: None,
            accuracy: None,
        }
    }
    /// Mock latitude
    pub fn latitude(&self) -> Option<f64> { self.latitude }
    /// Mock longitude
    pub fn longitude(&self) -> Option<f64> { self.longitude }
    /// Mock accuracy
    pub fn accuracy(&self) -> Option<f64> { self.accuracy }
}

#[derive(Default)]
pub struct SetGeolocationOverrideParamsBuilder {
    latitude: Option<f64>,
    longitude: Option<f64>,
    accuracy: Option<f64>,
}

impl SetGeolocationOverrideParamsBuilder {
    /// Mock latitude
    pub fn latitude(mut self, latitude: f64) -> Self { self.latitude = Some(latitude); self }
    /// Mock longitude
    pub fn longitude(mut self, longitude: f64) -> Self { self.longitude = Some(longitude); self }
    /// Mock accuracy
    pub fn accuracy(mut self, accuracy: f64) -> Self { self.accuracy = Some(accuracy); self }
    pub fn build(self) -> SetGeolocationOverrideParams {
        SetGeolocationOverrideParams {
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy: self.accuracy,
        }
    }
}

impl SetGeolocationOverrideParams { pub const METHOD: &'static str = "Page.setGeolocationOverride"; }

impl<'a> crate::CdpCommand<'a> for SetGeolocationOverrideParams {
    const METHOD: &'static str = "Page.setGeolocationOverride";
    type Response = crate::EmptyReturns;
}

/// Controls whether page will emit lifecycle events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetLifecycleEventsEnabledParams {
    /// If true, starts emitting lifecycle events.
    enabled: bool,
}

impl SetLifecycleEventsEnabledParams {
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: If true, starts emitting lifecycle events.
    pub fn builder(enabled: bool) -> SetLifecycleEventsEnabledParamsBuilder {
        SetLifecycleEventsEnabledParamsBuilder {
            enabled: enabled,
        }
    }
    /// If true, starts emitting lifecycle events.
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetLifecycleEventsEnabledParamsBuilder {
    enabled: bool,
}

impl SetLifecycleEventsEnabledParamsBuilder {
    pub fn build(self) -> SetLifecycleEventsEnabledParams {
        SetLifecycleEventsEnabledParams {
            enabled: self.enabled,
        }
    }
}

impl SetLifecycleEventsEnabledParams { pub const METHOD: &'static str = "Page.setLifecycleEventsEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetLifecycleEventsEnabledParams {
    const METHOD: &'static str = "Page.setLifecycleEventsEnabled";
    type Response = crate::EmptyReturns;
}

/// Toggles mouse event-based touch event emulation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTouchEmulationEnabledParams<'a> {
    /// Whether the touch event emulation should be enabled.
    enabled: bool,
    /// Touch/gesture events configuration. Default: current platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Cow<'a, str>>,
}

impl<'a> SetTouchEmulationEnabledParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: Whether the touch event emulation should be enabled.
    pub fn builder(enabled: bool) -> SetTouchEmulationEnabledParamsBuilder<'a> {
        SetTouchEmulationEnabledParamsBuilder {
            enabled: enabled,
            configuration: None,
        }
    }
    /// Whether the touch event emulation should be enabled.
    pub fn enabled(&self) -> bool { self.enabled }
    /// Touch/gesture events configuration. Default: current platform.
    pub fn configuration(&self) -> Option<&str> { self.configuration.as_deref() }
}


pub struct SetTouchEmulationEnabledParamsBuilder<'a> {
    enabled: bool,
    configuration: Option<Cow<'a, str>>,
}

impl<'a> SetTouchEmulationEnabledParamsBuilder<'a> {
    /// Touch/gesture events configuration. Default: current platform.
    pub fn configuration(mut self, configuration: impl Into<Cow<'a, str>>) -> Self { self.configuration = Some(configuration.into()); self }
    pub fn build(self) -> SetTouchEmulationEnabledParams<'a> {
        SetTouchEmulationEnabledParams {
            enabled: self.enabled,
            configuration: self.configuration,
        }
    }
}

impl<'a> SetTouchEmulationEnabledParams<'a> { pub const METHOD: &'static str = "Page.setTouchEmulationEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetTouchEmulationEnabledParams<'a> {
    const METHOD: &'static str = "Page.setTouchEmulationEnabled";
    type Response = crate::EmptyReturns;
}

/// Starts sending each frame using the 'screencastFrame' event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartScreencastParams<'a> {
    /// Image compression format.
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<Cow<'a, str>>,
    /// Compression quality from range \[0..100\].
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<i64>,
    /// Maximum screenshot width.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxWidth")]
    max_width: Option<u64>,
    /// Maximum screenshot height.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxHeight")]
    max_height: Option<i64>,
    /// Send every n-th frame.
    #[serde(skip_serializing_if = "Option::is_none", rename = "everyNthFrame")]
    every_nth_frame: Option<i64>,
}

impl<'a> StartScreencastParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> StartScreencastParamsBuilder<'a> {
        StartScreencastParamsBuilder {
            format: None,
            quality: None,
            max_width: None,
            max_height: None,
            every_nth_frame: None,
        }
    }
    /// Image compression format.
    pub fn format(&self) -> Option<&str> { self.format.as_deref() }
    /// Compression quality from range \[0..100\].
    pub fn quality(&self) -> Option<i64> { self.quality }
    /// Maximum screenshot width.
    pub fn max_width(&self) -> Option<u64> { self.max_width }
    /// Maximum screenshot height.
    pub fn max_height(&self) -> Option<i64> { self.max_height }
    /// Send every n-th frame.
    pub fn every_nth_frame(&self) -> Option<i64> { self.every_nth_frame }
}

#[derive(Default)]
pub struct StartScreencastParamsBuilder<'a> {
    format: Option<Cow<'a, str>>,
    quality: Option<i64>,
    max_width: Option<u64>,
    max_height: Option<i64>,
    every_nth_frame: Option<i64>,
}

impl<'a> StartScreencastParamsBuilder<'a> {
    /// Image compression format.
    pub fn format(mut self, format: impl Into<Cow<'a, str>>) -> Self { self.format = Some(format.into()); self }
    /// Compression quality from range \[0..100\].
    pub fn quality(mut self, quality: i64) -> Self { self.quality = Some(quality); self }
    /// Maximum screenshot width.
    pub fn max_width(mut self, max_width: u64) -> Self { self.max_width = Some(max_width); self }
    /// Maximum screenshot height.
    pub fn max_height(mut self, max_height: i64) -> Self { self.max_height = Some(max_height); self }
    /// Send every n-th frame.
    pub fn every_nth_frame(mut self, every_nth_frame: i64) -> Self { self.every_nth_frame = Some(every_nth_frame); self }
    pub fn build(self) -> StartScreencastParams<'a> {
        StartScreencastParams {
            format: self.format,
            quality: self.quality,
            max_width: self.max_width,
            max_height: self.max_height,
            every_nth_frame: self.every_nth_frame,
        }
    }
}

impl<'a> StartScreencastParams<'a> { pub const METHOD: &'static str = "Page.startScreencast"; }

impl<'a> crate::CdpCommand<'a> for StartScreencastParams<'a> {
    const METHOD: &'static str = "Page.startScreencast";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopLoadingParams {}

impl StopLoadingParams { pub const METHOD: &'static str = "Page.stopLoading"; }

impl<'a> crate::CdpCommand<'a> for StopLoadingParams {
    const METHOD: &'static str = "Page.stopLoading";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrashParams {}

impl CrashParams { pub const METHOD: &'static str = "Page.crash"; }

impl<'a> crate::CdpCommand<'a> for CrashParams {
    const METHOD: &'static str = "Page.crash";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloseParams {}

impl CloseParams { pub const METHOD: &'static str = "Page.close"; }

impl<'a> crate::CdpCommand<'a> for CloseParams {
    const METHOD: &'static str = "Page.close";
    type Response = crate::EmptyReturns;
}

/// Tries to update the web lifecycle state of the page.
/// It will transition the page to the given state according to:
/// <https://github.com/WICG/web-lifecycle/>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetWebLifecycleStateParams<'a> {
    /// Target lifecycle state
    state: Cow<'a, str>,
}

impl<'a> SetWebLifecycleStateParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `state`: Target lifecycle state
    pub fn builder(state: impl Into<Cow<'a, str>>) -> SetWebLifecycleStateParamsBuilder<'a> {
        SetWebLifecycleStateParamsBuilder {
            state: state.into(),
        }
    }
    /// Target lifecycle state
    pub fn state(&self) -> &str { self.state.as_ref() }
}


pub struct SetWebLifecycleStateParamsBuilder<'a> {
    state: Cow<'a, str>,
}

impl<'a> SetWebLifecycleStateParamsBuilder<'a> {
    pub fn build(self) -> SetWebLifecycleStateParams<'a> {
        SetWebLifecycleStateParams {
            state: self.state,
        }
    }
}

impl<'a> SetWebLifecycleStateParams<'a> { pub const METHOD: &'static str = "Page.setWebLifecycleState"; }

impl<'a> crate::CdpCommand<'a> for SetWebLifecycleStateParams<'a> {
    const METHOD: &'static str = "Page.setWebLifecycleState";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopScreencastParams {}

impl StopScreencastParams { pub const METHOD: &'static str = "Page.stopScreencast"; }

impl<'a> crate::CdpCommand<'a> for StopScreencastParams {
    const METHOD: &'static str = "Page.stopScreencast";
    type Response = crate::EmptyReturns;
}

/// Requests backend to produce compilation cache for the specified scripts.
/// 'scripts' are appended to the list of scripts for which the cache
/// would be produced. The list may be reset during page navigation.
/// When script with a matching URL is encountered, the cache is optionally
/// produced upon backend discretion, based on internal heuristics.
/// See also: 'Page.compilationCacheProduced'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProduceCompilationCacheParams<'a> {
    scripts: Vec<CompilationCacheParams<'a>>,
}

impl<'a> ProduceCompilationCacheParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `scripts`: 
    pub fn builder(scripts: Vec<CompilationCacheParams<'a>>) -> ProduceCompilationCacheParamsBuilder<'a> {
        ProduceCompilationCacheParamsBuilder {
            scripts: scripts,
        }
    }
    pub fn scripts(&self) -> &[CompilationCacheParams<'a>] { &self.scripts }
}


pub struct ProduceCompilationCacheParamsBuilder<'a> {
    scripts: Vec<CompilationCacheParams<'a>>,
}

impl<'a> ProduceCompilationCacheParamsBuilder<'a> {
    pub fn build(self) -> ProduceCompilationCacheParams<'a> {
        ProduceCompilationCacheParams {
            scripts: self.scripts,
        }
    }
}

impl<'a> ProduceCompilationCacheParams<'a> { pub const METHOD: &'static str = "Page.produceCompilationCache"; }

impl<'a> crate::CdpCommand<'a> for ProduceCompilationCacheParams<'a> {
    const METHOD: &'static str = "Page.produceCompilationCache";
    type Response = crate::EmptyReturns;
}

/// Seeds compilation cache for given url. Compilation cache does not survive
/// cross-process navigation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddCompilationCacheParams<'a> {
    url: Cow<'a, str>,
    /// Base64-encoded data (Encoded as a base64 string when passed over JSON)
    data: Cow<'a, str>,
}

impl<'a> AddCompilationCacheParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: 
    /// * `data`: Base64-encoded data (Encoded as a base64 string when passed over JSON)
    pub fn builder(url: impl Into<Cow<'a, str>>, data: impl Into<Cow<'a, str>>) -> AddCompilationCacheParamsBuilder<'a> {
        AddCompilationCacheParamsBuilder {
            url: url.into(),
            data: data.into(),
        }
    }
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Base64-encoded data (Encoded as a base64 string when passed over JSON)
    pub fn data(&self) -> &str { self.data.as_ref() }
}


pub struct AddCompilationCacheParamsBuilder<'a> {
    url: Cow<'a, str>,
    data: Cow<'a, str>,
}

impl<'a> AddCompilationCacheParamsBuilder<'a> {
    pub fn build(self) -> AddCompilationCacheParams<'a> {
        AddCompilationCacheParams {
            url: self.url,
            data: self.data,
        }
    }
}

impl<'a> AddCompilationCacheParams<'a> { pub const METHOD: &'static str = "Page.addCompilationCache"; }

impl<'a> crate::CdpCommand<'a> for AddCompilationCacheParams<'a> {
    const METHOD: &'static str = "Page.addCompilationCache";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearCompilationCacheParams {}

impl ClearCompilationCacheParams { pub const METHOD: &'static str = "Page.clearCompilationCache"; }

impl<'a> crate::CdpCommand<'a> for ClearCompilationCacheParams {
    const METHOD: &'static str = "Page.clearCompilationCache";
    type Response = crate::EmptyReturns;
}

/// Sets the Secure Payment Confirmation transaction mode.
/// <https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSPCTransactionModeParams<'a> {
    mode: Cow<'a, str>,
}

impl<'a> SetSPCTransactionModeParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `mode`: 
    pub fn builder(mode: impl Into<Cow<'a, str>>) -> SetSPCTransactionModeParamsBuilder<'a> {
        SetSPCTransactionModeParamsBuilder {
            mode: mode.into(),
        }
    }
    pub fn mode(&self) -> &str { self.mode.as_ref() }
}


pub struct SetSPCTransactionModeParamsBuilder<'a> {
    mode: Cow<'a, str>,
}

impl<'a> SetSPCTransactionModeParamsBuilder<'a> {
    pub fn build(self) -> SetSPCTransactionModeParams<'a> {
        SetSPCTransactionModeParams {
            mode: self.mode,
        }
    }
}

impl<'a> SetSPCTransactionModeParams<'a> { pub const METHOD: &'static str = "Page.setSPCTransactionMode"; }

impl<'a> crate::CdpCommand<'a> for SetSPCTransactionModeParams<'a> {
    const METHOD: &'static str = "Page.setSPCTransactionMode";
    type Response = crate::EmptyReturns;
}

/// Extensions for Custom Handlers API:
/// <https://html.spec.whatwg.org/multipage/system-state.html#rph-automation>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRPHRegistrationModeParams<'a> {
    mode: Cow<'a, str>,
}

impl<'a> SetRPHRegistrationModeParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `mode`: 
    pub fn builder(mode: impl Into<Cow<'a, str>>) -> SetRPHRegistrationModeParamsBuilder<'a> {
        SetRPHRegistrationModeParamsBuilder {
            mode: mode.into(),
        }
    }
    pub fn mode(&self) -> &str { self.mode.as_ref() }
}


pub struct SetRPHRegistrationModeParamsBuilder<'a> {
    mode: Cow<'a, str>,
}

impl<'a> SetRPHRegistrationModeParamsBuilder<'a> {
    pub fn build(self) -> SetRPHRegistrationModeParams<'a> {
        SetRPHRegistrationModeParams {
            mode: self.mode,
        }
    }
}

impl<'a> SetRPHRegistrationModeParams<'a> { pub const METHOD: &'static str = "Page.setRPHRegistrationMode"; }

impl<'a> crate::CdpCommand<'a> for SetRPHRegistrationModeParams<'a> {
    const METHOD: &'static str = "Page.setRPHRegistrationMode";
    type Response = crate::EmptyReturns;
}

/// Generates a report for testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTestReportParams<'a> {
    /// Message to be displayed in the report.
    message: Cow<'a, str>,
    /// Specifies the endpoint group to deliver the report to.
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<Cow<'a, str>>,
}

impl<'a> GenerateTestReportParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `message`: Message to be displayed in the report.
    pub fn builder(message: impl Into<Cow<'a, str>>) -> GenerateTestReportParamsBuilder<'a> {
        GenerateTestReportParamsBuilder {
            message: message.into(),
            group: None,
        }
    }
    /// Message to be displayed in the report.
    pub fn message(&self) -> &str { self.message.as_ref() }
    /// Specifies the endpoint group to deliver the report to.
    pub fn group(&self) -> Option<&str> { self.group.as_deref() }
}


pub struct GenerateTestReportParamsBuilder<'a> {
    message: Cow<'a, str>,
    group: Option<Cow<'a, str>>,
}

impl<'a> GenerateTestReportParamsBuilder<'a> {
    /// Specifies the endpoint group to deliver the report to.
    pub fn group(mut self, group: impl Into<Cow<'a, str>>) -> Self { self.group = Some(group.into()); self }
    pub fn build(self) -> GenerateTestReportParams<'a> {
        GenerateTestReportParams {
            message: self.message,
            group: self.group,
        }
    }
}

impl<'a> GenerateTestReportParams<'a> { pub const METHOD: &'static str = "Page.generateTestReport"; }

impl<'a> crate::CdpCommand<'a> for GenerateTestReportParams<'a> {
    const METHOD: &'static str = "Page.generateTestReport";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WaitForDebuggerParams {}

impl WaitForDebuggerParams { pub const METHOD: &'static str = "Page.waitForDebugger"; }

impl<'a> crate::CdpCommand<'a> for WaitForDebuggerParams {
    const METHOD: &'static str = "Page.waitForDebugger";
    type Response = crate::EmptyReturns;
}

/// Intercept file chooser requests and transfer control to protocol clients.
/// When file chooser interception is enabled, native file chooser dialog is not shown.
/// Instead, a protocol event 'Page.fileChooserOpened' is emitted.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInterceptFileChooserDialogParams {
    enabled: bool,
    /// If true, cancels the dialog by emitting relevant events (if any)
    /// in addition to not showing it if the interception is enabled
    /// (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel: Option<bool>,
}

impl SetInterceptFileChooserDialogParams {
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: 
    pub fn builder(enabled: bool) -> SetInterceptFileChooserDialogParamsBuilder {
        SetInterceptFileChooserDialogParamsBuilder {
            enabled: enabled,
            cancel: None,
        }
    }
    pub fn enabled(&self) -> bool { self.enabled }
    /// If true, cancels the dialog by emitting relevant events (if any)
    /// in addition to not showing it if the interception is enabled
    /// (default: false).
    pub fn cancel(&self) -> Option<bool> { self.cancel }
}


pub struct SetInterceptFileChooserDialogParamsBuilder {
    enabled: bool,
    cancel: Option<bool>,
}

impl SetInterceptFileChooserDialogParamsBuilder {
    /// If true, cancels the dialog by emitting relevant events (if any)
    /// in addition to not showing it if the interception is enabled
    /// (default: false).
    pub fn cancel(mut self, cancel: bool) -> Self { self.cancel = Some(cancel); self }
    pub fn build(self) -> SetInterceptFileChooserDialogParams {
        SetInterceptFileChooserDialogParams {
            enabled: self.enabled,
            cancel: self.cancel,
        }
    }
}

impl SetInterceptFileChooserDialogParams { pub const METHOD: &'static str = "Page.setInterceptFileChooserDialog"; }

impl<'a> crate::CdpCommand<'a> for SetInterceptFileChooserDialogParams {
    const METHOD: &'static str = "Page.setInterceptFileChooserDialog";
    type Response = crate::EmptyReturns;
}

/// Enable/disable prerendering manually.
/// 
/// This command is a short-term solution for <https://crbug.com/1440085>.
/// See <https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA>
/// for more details.
/// 
/// TODO(<https://crbug.com/1440085>): Remove this once Puppeteer supports tab targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPrerenderingAllowedParams {
    #[serde(rename = "isAllowed")]
    is_allowed: bool,
}

impl SetPrerenderingAllowedParams {
    /// Creates a builder for this type with the required parameters:
    /// * `is_allowed`: 
    pub fn builder(is_allowed: bool) -> SetPrerenderingAllowedParamsBuilder {
        SetPrerenderingAllowedParamsBuilder {
            is_allowed: is_allowed,
        }
    }
    pub fn is_allowed(&self) -> bool { self.is_allowed }
}


pub struct SetPrerenderingAllowedParamsBuilder {
    is_allowed: bool,
}

impl SetPrerenderingAllowedParamsBuilder {
    pub fn build(self) -> SetPrerenderingAllowedParams {
        SetPrerenderingAllowedParams {
            is_allowed: self.is_allowed,
        }
    }
}

impl SetPrerenderingAllowedParams { pub const METHOD: &'static str = "Page.setPrerenderingAllowed"; }

impl<'a> crate::CdpCommand<'a> for SetPrerenderingAllowedParams {
    const METHOD: &'static str = "Page.setPrerenderingAllowed";
    type Response = crate::EmptyReturns;
}

/// Get the annotated page content for the main frame.
/// This is an experimental command that is subject to change.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnotatedPageContentParams {
    /// Whether to include actionable information. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeActionableInformation")]
    include_actionable_information: Option<bool>,
}

impl GetAnnotatedPageContentParams {
    /// Creates a builder for this type.
    pub fn builder() -> GetAnnotatedPageContentParamsBuilder {
        GetAnnotatedPageContentParamsBuilder {
            include_actionable_information: None,
        }
    }
    /// Whether to include actionable information. Defaults to true.
    pub fn include_actionable_information(&self) -> Option<bool> { self.include_actionable_information }
}

#[derive(Default)]
pub struct GetAnnotatedPageContentParamsBuilder {
    include_actionable_information: Option<bool>,
}

impl GetAnnotatedPageContentParamsBuilder {
    /// Whether to include actionable information. Defaults to true.
    pub fn include_actionable_information(mut self, include_actionable_information: bool) -> Self { self.include_actionable_information = Some(include_actionable_information); self }
    pub fn build(self) -> GetAnnotatedPageContentParams {
        GetAnnotatedPageContentParams {
            include_actionable_information: self.include_actionable_information,
        }
    }
}

/// Get the annotated page content for the main frame.
/// This is an experimental command that is subject to change.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnotatedPageContentReturns<'a> {
    /// The annotated page content as a base64 encoded protobuf.
    /// The format is defined by the 'AnnotatedPageContent' message in
    /// components/optimization_guide/proto/features/common_quality_data.proto (Encoded as a base64 string when passed over JSON)
    content: Cow<'a, str>,
}

impl<'a> GetAnnotatedPageContentReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `content`: The annotated page content as a base64 encoded protobuf. The format is defined by the `AnnotatedPageContent` message in components/optimization_guide/proto/features/common_quality_data.proto (Encoded as a base64 string when passed over JSON)
    pub fn builder(content: impl Into<Cow<'a, str>>) -> GetAnnotatedPageContentReturnsBuilder<'a> {
        GetAnnotatedPageContentReturnsBuilder {
            content: content.into(),
        }
    }
    /// The annotated page content as a base64 encoded protobuf.
    /// The format is defined by the 'AnnotatedPageContent' message in
    /// components/optimization_guide/proto/features/common_quality_data.proto (Encoded as a base64 string when passed over JSON)
    pub fn content(&self) -> &str { self.content.as_ref() }
}


pub struct GetAnnotatedPageContentReturnsBuilder<'a> {
    content: Cow<'a, str>,
}

impl<'a> GetAnnotatedPageContentReturnsBuilder<'a> {
    pub fn build(self) -> GetAnnotatedPageContentReturns<'a> {
        GetAnnotatedPageContentReturns {
            content: self.content,
        }
    }
}

impl GetAnnotatedPageContentParams { pub const METHOD: &'static str = "Page.getAnnotatedPageContent"; }

impl<'a> crate::CdpCommand<'a> for GetAnnotatedPageContentParams {
    const METHOD: &'static str = "Page.getAnnotatedPageContent";
    type Response = GetAnnotatedPageContentReturns<'a>;
}
