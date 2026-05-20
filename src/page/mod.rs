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
    adFrameType: AdFrameType,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanations: Option<Vec<AdFrameExplanation>>,
}

impl AdFrameStatus {
    pub fn builder() -> AdFrameStatusBuilder { AdFrameStatusBuilder::default() }
    pub fn adFrameType(&self) -> &AdFrameType { &self.adFrameType }
    pub fn explanations(&self) -> Option<&[AdFrameExplanation]> { self.explanations.as_deref() }
}

#[derive(Default)]
pub struct AdFrameStatusBuilder {
    adFrameType: Option<AdFrameType>,
    explanations: Option<Vec<AdFrameExplanation>>,
}

impl AdFrameStatusBuilder {
    pub fn adFrameType(mut self, adFrameType: AdFrameType) -> Self { self.adFrameType = Some(adFrameType); self }
    pub fn explanations(mut self, explanations: Vec<AdFrameExplanation>) -> Self { self.explanations = Some(explanations); self }
    pub fn build(self) -> AdFrameStatus {
        AdFrameStatus {
            adFrameType: self.adFrameType.unwrap_or_default(),
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
    frameId: FrameId<'a>,
    blockReason: PermissionsPolicyBlockReason,
}

impl<'a> PermissionsPolicyBlockLocator<'a> {
    pub fn builder() -> PermissionsPolicyBlockLocatorBuilder<'a> { PermissionsPolicyBlockLocatorBuilder::default() }
    pub fn frameId(&self) -> &FrameId<'a> { &self.frameId }
    pub fn blockReason(&self) -> &PermissionsPolicyBlockReason { &self.blockReason }
}

#[derive(Default)]
pub struct PermissionsPolicyBlockLocatorBuilder<'a> {
    frameId: Option<FrameId<'a>>,
    blockReason: Option<PermissionsPolicyBlockReason>,
}

impl<'a> PermissionsPolicyBlockLocatorBuilder<'a> {
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn blockReason(mut self, blockReason: PermissionsPolicyBlockReason) -> Self { self.blockReason = Some(blockReason); self }
    pub fn build(self) -> PermissionsPolicyBlockLocator<'a> {
        PermissionsPolicyBlockLocator {
            frameId: self.frameId.unwrap_or_default(),
            blockReason: self.blockReason.unwrap_or_default(),
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
    pub fn builder() -> PermissionsPolicyFeatureStateBuilder<'a> { PermissionsPolicyFeatureStateBuilder::default() }
    pub fn feature(&self) -> &PermissionsPolicyFeature { &self.feature }
    pub fn allowed(&self) -> bool { self.allowed }
    pub fn locator(&self) -> Option<&PermissionsPolicyBlockLocator<'a>> { self.locator.as_ref() }
}

#[derive(Default)]
pub struct PermissionsPolicyFeatureStateBuilder<'a> {
    feature: Option<PermissionsPolicyFeature>,
    allowed: Option<bool>,
    locator: Option<PermissionsPolicyBlockLocator<'a>>,
}

impl<'a> PermissionsPolicyFeatureStateBuilder<'a> {
    pub fn feature(mut self, feature: PermissionsPolicyFeature) -> Self { self.feature = Some(feature); self }
    pub fn allowed(mut self, allowed: bool) -> Self { self.allowed = Some(allowed); self }
    pub fn locator(mut self, locator: PermissionsPolicyBlockLocator<'a>) -> Self { self.locator = Some(locator); self }
    pub fn build(self) -> PermissionsPolicyFeatureState<'a> {
        PermissionsPolicyFeatureState {
            feature: self.feature.unwrap_or_default(),
            allowed: self.allowed.unwrap_or_default(),
            locator: self.locator,
        }
    }
}

/// Origin Trial(https://www.chromium.org/blink/origin-trials) support.
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
    matchSubDomains: bool,
    trialName: Cow<'a, str>,
    expiryTime: crate::network::TimeSinceEpoch,
    isThirdParty: bool,
    usageRestriction: OriginTrialUsageRestriction,
}

impl<'a> OriginTrialToken<'a> {
    pub fn builder() -> OriginTrialTokenBuilder<'a> { OriginTrialTokenBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn matchSubDomains(&self) -> bool { self.matchSubDomains }
    pub fn trialName(&self) -> &str { self.trialName.as_ref() }
    pub fn expiryTime(&self) -> &crate::network::TimeSinceEpoch { &self.expiryTime }
    pub fn isThirdParty(&self) -> bool { self.isThirdParty }
    pub fn usageRestriction(&self) -> &OriginTrialUsageRestriction { &self.usageRestriction }
}

#[derive(Default)]
pub struct OriginTrialTokenBuilder<'a> {
    origin: Option<Cow<'a, str>>,
    matchSubDomains: Option<bool>,
    trialName: Option<Cow<'a, str>>,
    expiryTime: Option<crate::network::TimeSinceEpoch>,
    isThirdParty: Option<bool>,
    usageRestriction: Option<OriginTrialUsageRestriction>,
}

impl<'a> OriginTrialTokenBuilder<'a> {
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn matchSubDomains(mut self, matchSubDomains: bool) -> Self { self.matchSubDomains = Some(matchSubDomains); self }
    pub fn trialName(mut self, trialName: impl Into<Cow<'a, str>>) -> Self { self.trialName = Some(trialName.into()); self }
    pub fn expiryTime(mut self, expiryTime: crate::network::TimeSinceEpoch) -> Self { self.expiryTime = Some(expiryTime); self }
    pub fn isThirdParty(mut self, isThirdParty: bool) -> Self { self.isThirdParty = Some(isThirdParty); self }
    pub fn usageRestriction(mut self, usageRestriction: OriginTrialUsageRestriction) -> Self { self.usageRestriction = Some(usageRestriction); self }
    pub fn build(self) -> OriginTrialToken<'a> {
        OriginTrialToken {
            origin: self.origin.unwrap_or_default(),
            matchSubDomains: self.matchSubDomains.unwrap_or_default(),
            trialName: self.trialName.unwrap_or_default(),
            expiryTime: self.expiryTime.unwrap_or_default(),
            isThirdParty: self.isThirdParty.unwrap_or_default(),
            usageRestriction: self.usageRestriction.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrialTokenWithStatus<'a> {
    rawTokenText: Cow<'a, str>,
    /// 'parsedToken' is present only when the token is extractable and
    /// parsable.
    #[serde(skip_serializing_if = "Option::is_none")]
    parsedToken: Option<OriginTrialToken<'a>>,
    status: OriginTrialTokenStatus,
}

impl<'a> OriginTrialTokenWithStatus<'a> {
    pub fn builder() -> OriginTrialTokenWithStatusBuilder<'a> { OriginTrialTokenWithStatusBuilder::default() }
    pub fn rawTokenText(&self) -> &str { self.rawTokenText.as_ref() }
    pub fn parsedToken(&self) -> Option<&OriginTrialToken<'a>> { self.parsedToken.as_ref() }
    pub fn status(&self) -> &OriginTrialTokenStatus { &self.status }
}

#[derive(Default)]
pub struct OriginTrialTokenWithStatusBuilder<'a> {
    rawTokenText: Option<Cow<'a, str>>,
    parsedToken: Option<OriginTrialToken<'a>>,
    status: Option<OriginTrialTokenStatus>,
}

impl<'a> OriginTrialTokenWithStatusBuilder<'a> {
    pub fn rawTokenText(mut self, rawTokenText: impl Into<Cow<'a, str>>) -> Self { self.rawTokenText = Some(rawTokenText.into()); self }
    /// 'parsedToken' is present only when the token is extractable and
    /// parsable.
    pub fn parsedToken(mut self, parsedToken: OriginTrialToken<'a>) -> Self { self.parsedToken = Some(parsedToken); self }
    pub fn status(mut self, status: OriginTrialTokenStatus) -> Self { self.status = Some(status); self }
    pub fn build(self) -> OriginTrialTokenWithStatus<'a> {
        OriginTrialTokenWithStatus {
            rawTokenText: self.rawTokenText.unwrap_or_default(),
            parsedToken: self.parsedToken,
            status: self.status.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrial<'a> {
    trialName: Cow<'a, str>,
    status: OriginTrialStatus,
    tokensWithStatus: Vec<OriginTrialTokenWithStatus<'a>>,
}

impl<'a> OriginTrial<'a> {
    pub fn builder() -> OriginTrialBuilder<'a> { OriginTrialBuilder::default() }
    pub fn trialName(&self) -> &str { self.trialName.as_ref() }
    pub fn status(&self) -> &OriginTrialStatus { &self.status }
    pub fn tokensWithStatus(&self) -> &[OriginTrialTokenWithStatus<'a>] { &self.tokensWithStatus }
}

#[derive(Default)]
pub struct OriginTrialBuilder<'a> {
    trialName: Option<Cow<'a, str>>,
    status: Option<OriginTrialStatus>,
    tokensWithStatus: Option<Vec<OriginTrialTokenWithStatus<'a>>>,
}

impl<'a> OriginTrialBuilder<'a> {
    pub fn trialName(mut self, trialName: impl Into<Cow<'a, str>>) -> Self { self.trialName = Some(trialName.into()); self }
    pub fn status(mut self, status: OriginTrialStatus) -> Self { self.status = Some(status); self }
    pub fn tokensWithStatus(mut self, tokensWithStatus: Vec<OriginTrialTokenWithStatus<'a>>) -> Self { self.tokensWithStatus = Some(tokensWithStatus); self }
    pub fn build(self) -> OriginTrial<'a> {
        OriginTrial {
            trialName: self.trialName.unwrap_or_default(),
            status: self.status.unwrap_or_default(),
            tokensWithStatus: self.tokensWithStatus.unwrap_or_default(),
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
    isLocalhost: bool,
}

impl SecurityOriginDetails {
    pub fn builder() -> SecurityOriginDetailsBuilder { SecurityOriginDetailsBuilder::default() }
    pub fn isLocalhost(&self) -> bool { self.isLocalhost }
}

#[derive(Default)]
pub struct SecurityOriginDetailsBuilder {
    isLocalhost: Option<bool>,
}

impl SecurityOriginDetailsBuilder {
    /// Indicates whether the frame document's security origin is one
    /// of the local hostnames (e.g. "localhost") or IP addresses (IPv4
    /// 127.0.0.0/8 or IPv6 ::1).
    pub fn isLocalhost(mut self, isLocalhost: bool) -> Self { self.isLocalhost = Some(isLocalhost); self }
    pub fn build(self) -> SecurityOriginDetails {
        SecurityOriginDetails {
            isLocalhost: self.isLocalhost.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    parentId: Option<FrameId<'a>>,
    /// Identifier of the loader associated with this frame.
    loaderId: crate::network::LoaderId<'a>,
    /// Frame's name as specified in the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    /// Frame document's URL without fragment.
    url: Cow<'a, str>,
    /// Frame document's URL fragment including the '#'.
    #[serde(skip_serializing_if = "Option::is_none")]
    urlFragment: Option<Cow<'a, str>>,
    /// Frame document's registered domain, taking the public suffixes list into account.
    /// Extracted from the Frame's url.
    /// Example URLs: http://www.google.com/file.html -> "google.com"
    /// http://a.b.co.uk/file.html      -> "b.co.uk"
    domainAndRegistry: Cow<'a, str>,
    /// Frame document's security origin.
    securityOrigin: Cow<'a, str>,
    /// Additional details about the frame document's security origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    securityOriginDetails: Option<SecurityOriginDetails>,
    /// Frame document's mimeType as determined by the browser.
    mimeType: Cow<'a, str>,
    /// If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.
    #[serde(skip_serializing_if = "Option::is_none")]
    unreachableUrl: Option<Cow<'a, str>>,
    /// Indicates whether this frame was tagged as an ad and why.
    #[serde(skip_serializing_if = "Option::is_none")]
    adFrameStatus: Option<AdFrameStatus>,
    /// Indicates whether the main document is a secure context and explains why that is the case.
    secureContextType: SecureContextType,
    /// Indicates whether this is a cross origin isolated context.
    crossOriginIsolatedContextType: CrossOriginIsolatedContextType,
    /// Indicated which gated APIs / features are available.
    gatedAPIFeatures: Vec<GatedAPIFeatures>,
}

impl<'a> Frame<'a> {
    pub fn builder() -> FrameBuilder<'a> { FrameBuilder::default() }
    pub fn id(&self) -> &FrameId<'a> { &self.id }
    pub fn parentId(&self) -> Option<&FrameId<'a>> { self.parentId.as_ref() }
    pub fn loaderId(&self) -> &crate::network::LoaderId<'a> { &self.loaderId }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn urlFragment(&self) -> Option<&str> { self.urlFragment.as_deref() }
    pub fn domainAndRegistry(&self) -> &str { self.domainAndRegistry.as_ref() }
    pub fn securityOrigin(&self) -> &str { self.securityOrigin.as_ref() }
    pub fn securityOriginDetails(&self) -> Option<&SecurityOriginDetails> { self.securityOriginDetails.as_ref() }
    pub fn mimeType(&self) -> &str { self.mimeType.as_ref() }
    pub fn unreachableUrl(&self) -> Option<&str> { self.unreachableUrl.as_deref() }
    pub fn adFrameStatus(&self) -> Option<&AdFrameStatus> { self.adFrameStatus.as_ref() }
    pub fn secureContextType(&self) -> &SecureContextType { &self.secureContextType }
    pub fn crossOriginIsolatedContextType(&self) -> &CrossOriginIsolatedContextType { &self.crossOriginIsolatedContextType }
    pub fn gatedAPIFeatures(&self) -> &[GatedAPIFeatures] { &self.gatedAPIFeatures }
}

#[derive(Default)]
pub struct FrameBuilder<'a> {
    id: Option<FrameId<'a>>,
    parentId: Option<FrameId<'a>>,
    loaderId: Option<crate::network::LoaderId<'a>>,
    name: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    urlFragment: Option<Cow<'a, str>>,
    domainAndRegistry: Option<Cow<'a, str>>,
    securityOrigin: Option<Cow<'a, str>>,
    securityOriginDetails: Option<SecurityOriginDetails>,
    mimeType: Option<Cow<'a, str>>,
    unreachableUrl: Option<Cow<'a, str>>,
    adFrameStatus: Option<AdFrameStatus>,
    secureContextType: Option<SecureContextType>,
    crossOriginIsolatedContextType: Option<CrossOriginIsolatedContextType>,
    gatedAPIFeatures: Option<Vec<GatedAPIFeatures>>,
}

impl<'a> FrameBuilder<'a> {
    /// Frame unique identifier.
    pub fn id(mut self, id: FrameId<'a>) -> Self { self.id = Some(id); self }
    /// Parent frame identifier.
    pub fn parentId(mut self, parentId: FrameId<'a>) -> Self { self.parentId = Some(parentId); self }
    /// Identifier of the loader associated with this frame.
    pub fn loaderId(mut self, loaderId: crate::network::LoaderId<'a>) -> Self { self.loaderId = Some(loaderId); self }
    /// Frame's name as specified in the tag.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Frame document's URL without fragment.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Frame document's URL fragment including the '#'.
    pub fn urlFragment(mut self, urlFragment: impl Into<Cow<'a, str>>) -> Self { self.urlFragment = Some(urlFragment.into()); self }
    /// Frame document's registered domain, taking the public suffixes list into account.
    /// Extracted from the Frame's url.
    /// Example URLs: http://www.google.com/file.html -> "google.com"
    /// http://a.b.co.uk/file.html      -> "b.co.uk"
    pub fn domainAndRegistry(mut self, domainAndRegistry: impl Into<Cow<'a, str>>) -> Self { self.domainAndRegistry = Some(domainAndRegistry.into()); self }
    /// Frame document's security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Additional details about the frame document's security origin.
    pub fn securityOriginDetails(mut self, securityOriginDetails: SecurityOriginDetails) -> Self { self.securityOriginDetails = Some(securityOriginDetails); self }
    /// Frame document's mimeType as determined by the browser.
    pub fn mimeType(mut self, mimeType: impl Into<Cow<'a, str>>) -> Self { self.mimeType = Some(mimeType.into()); self }
    /// If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.
    pub fn unreachableUrl(mut self, unreachableUrl: impl Into<Cow<'a, str>>) -> Self { self.unreachableUrl = Some(unreachableUrl.into()); self }
    /// Indicates whether this frame was tagged as an ad and why.
    pub fn adFrameStatus(mut self, adFrameStatus: AdFrameStatus) -> Self { self.adFrameStatus = Some(adFrameStatus); self }
    /// Indicates whether the main document is a secure context and explains why that is the case.
    pub fn secureContextType(mut self, secureContextType: SecureContextType) -> Self { self.secureContextType = Some(secureContextType); self }
    /// Indicates whether this is a cross origin isolated context.
    pub fn crossOriginIsolatedContextType(mut self, crossOriginIsolatedContextType: CrossOriginIsolatedContextType) -> Self { self.crossOriginIsolatedContextType = Some(crossOriginIsolatedContextType); self }
    /// Indicated which gated APIs / features are available.
    pub fn gatedAPIFeatures(mut self, gatedAPIFeatures: Vec<GatedAPIFeatures>) -> Self { self.gatedAPIFeatures = Some(gatedAPIFeatures); self }
    pub fn build(self) -> Frame<'a> {
        Frame {
            id: self.id.unwrap_or_default(),
            parentId: self.parentId,
            loaderId: self.loaderId.unwrap_or_default(),
            name: self.name,
            url: self.url.unwrap_or_default(),
            urlFragment: self.urlFragment,
            domainAndRegistry: self.domainAndRegistry.unwrap_or_default(),
            securityOrigin: self.securityOrigin.unwrap_or_default(),
            securityOriginDetails: self.securityOriginDetails,
            mimeType: self.mimeType.unwrap_or_default(),
            unreachableUrl: self.unreachableUrl,
            adFrameStatus: self.adFrameStatus,
            secureContextType: self.secureContextType.unwrap_or_default(),
            crossOriginIsolatedContextType: self.crossOriginIsolatedContextType.unwrap_or_default(),
            gatedAPIFeatures: self.gatedAPIFeatures.unwrap_or_default(),
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
    mimeType: Cow<'a, str>,
    /// last-modified timestamp as reported by server.
    #[serde(skip_serializing_if = "Option::is_none")]
    lastModified: Option<crate::network::TimeSinceEpoch>,
    /// Resource content size.
    #[serde(skip_serializing_if = "Option::is_none")]
    contentSize: Option<f64>,
    /// True if the resource failed to load.
    #[serde(skip_serializing_if = "Option::is_none")]
    failed: Option<bool>,
    /// True if the resource was canceled during loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    canceled: Option<bool>,
}

impl<'a> FrameResource<'a> {
    pub fn builder() -> FrameResourceBuilder<'a> { FrameResourceBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn type_(&self) -> &crate::network::ResourceType { &self.type_ }
    pub fn mimeType(&self) -> &str { self.mimeType.as_ref() }
    pub fn lastModified(&self) -> Option<&crate::network::TimeSinceEpoch> { self.lastModified.as_ref() }
    pub fn contentSize(&self) -> Option<f64> { self.contentSize }
    pub fn failed(&self) -> Option<bool> { self.failed }
    pub fn canceled(&self) -> Option<bool> { self.canceled }
}

#[derive(Default)]
pub struct FrameResourceBuilder<'a> {
    url: Option<Cow<'a, str>>,
    type_: Option<crate::network::ResourceType>,
    mimeType: Option<Cow<'a, str>>,
    lastModified: Option<crate::network::TimeSinceEpoch>,
    contentSize: Option<f64>,
    failed: Option<bool>,
    canceled: Option<bool>,
}

impl<'a> FrameResourceBuilder<'a> {
    /// Resource URL.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Type of this resource.
    pub fn type_(mut self, type_: crate::network::ResourceType) -> Self { self.type_ = Some(type_); self }
    /// Resource mimeType as determined by the browser.
    pub fn mimeType(mut self, mimeType: impl Into<Cow<'a, str>>) -> Self { self.mimeType = Some(mimeType.into()); self }
    /// last-modified timestamp as reported by server.
    pub fn lastModified(mut self, lastModified: crate::network::TimeSinceEpoch) -> Self { self.lastModified = Some(lastModified); self }
    /// Resource content size.
    pub fn contentSize(mut self, contentSize: f64) -> Self { self.contentSize = Some(contentSize); self }
    /// True if the resource failed to load.
    pub fn failed(mut self, failed: bool) -> Self { self.failed = Some(failed); self }
    /// True if the resource was canceled during loading.
    pub fn canceled(mut self, canceled: bool) -> Self { self.canceled = Some(canceled); self }
    pub fn build(self) -> FrameResource<'a> {
        FrameResource {
            url: self.url.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
            mimeType: self.mimeType.unwrap_or_default(),
            lastModified: self.lastModified,
            contentSize: self.contentSize,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    childFrames: Option<Vec<Box<FrameResourceTree<'a>>>>,
    /// Information about frame resources.
    resources: Vec<FrameResource<'a>>,
}

impl<'a> FrameResourceTree<'a> {
    pub fn builder() -> FrameResourceTreeBuilder<'a> { FrameResourceTreeBuilder::default() }
    pub fn frame(&self) -> &Frame<'a> { &self.frame }
    pub fn childFrames(&self) -> Option<&[Box<FrameResourceTree<'a>>]> { self.childFrames.as_deref() }
    pub fn resources(&self) -> &[FrameResource<'a>] { &self.resources }
}

#[derive(Default)]
pub struct FrameResourceTreeBuilder<'a> {
    frame: Option<Frame<'a>>,
    childFrames: Option<Vec<Box<FrameResourceTree<'a>>>>,
    resources: Option<Vec<FrameResource<'a>>>,
}

impl<'a> FrameResourceTreeBuilder<'a> {
    /// Frame information for this tree item.
    pub fn frame(mut self, frame: Frame<'a>) -> Self { self.frame = Some(frame); self }
    /// Child frames.
    pub fn childFrames(mut self, childFrames: Vec<Box<FrameResourceTree<'a>>>) -> Self { self.childFrames = Some(childFrames); self }
    /// Information about frame resources.
    pub fn resources(mut self, resources: Vec<FrameResource<'a>>) -> Self { self.resources = Some(resources); self }
    pub fn build(self) -> FrameResourceTree<'a> {
        FrameResourceTree {
            frame: self.frame.unwrap_or_default(),
            childFrames: self.childFrames,
            resources: self.resources.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    childFrames: Option<Vec<Box<FrameTree<'a>>>>,
}

impl<'a> FrameTree<'a> {
    pub fn builder() -> FrameTreeBuilder<'a> { FrameTreeBuilder::default() }
    pub fn frame(&self) -> &Frame<'a> { &self.frame }
    pub fn childFrames(&self) -> Option<&[Box<FrameTree<'a>>]> { self.childFrames.as_deref() }
}

#[derive(Default)]
pub struct FrameTreeBuilder<'a> {
    frame: Option<Frame<'a>>,
    childFrames: Option<Vec<Box<FrameTree<'a>>>>,
}

impl<'a> FrameTreeBuilder<'a> {
    /// Frame information for this tree item.
    pub fn frame(mut self, frame: Frame<'a>) -> Self { self.frame = Some(frame); self }
    /// Child frames.
    pub fn childFrames(mut self, childFrames: Vec<Box<FrameTree<'a>>>) -> Self { self.childFrames = Some(childFrames); self }
    pub fn build(self) -> FrameTree<'a> {
        FrameTree {
            frame: self.frame.unwrap_or_default(),
            childFrames: self.childFrames,
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
    userTypedURL: Cow<'a, str>,
    /// Title of the navigation history entry.
    title: Cow<'a, str>,
    /// Transition type.
    transitionType: TransitionType,
}

impl<'a> NavigationEntry<'a> {
    pub fn builder() -> NavigationEntryBuilder<'a> { NavigationEntryBuilder::default() }
    pub fn id(&self) -> u64 { self.id }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn userTypedURL(&self) -> &str { self.userTypedURL.as_ref() }
    pub fn title(&self) -> &str { self.title.as_ref() }
    pub fn transitionType(&self) -> &TransitionType { &self.transitionType }
}

#[derive(Default)]
pub struct NavigationEntryBuilder<'a> {
    id: Option<u64>,
    url: Option<Cow<'a, str>>,
    userTypedURL: Option<Cow<'a, str>>,
    title: Option<Cow<'a, str>>,
    transitionType: Option<TransitionType>,
}

impl<'a> NavigationEntryBuilder<'a> {
    /// Unique id of the navigation history entry.
    pub fn id(mut self, id: u64) -> Self { self.id = Some(id); self }
    /// URL of the navigation history entry.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// URL that the user typed in the url bar.
    pub fn userTypedURL(mut self, userTypedURL: impl Into<Cow<'a, str>>) -> Self { self.userTypedURL = Some(userTypedURL.into()); self }
    /// Title of the navigation history entry.
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self { self.title = Some(title.into()); self }
    /// Transition type.
    pub fn transitionType(mut self, transitionType: TransitionType) -> Self { self.transitionType = Some(transitionType); self }
    pub fn build(self) -> NavigationEntry<'a> {
        NavigationEntry {
            id: self.id.unwrap_or_default(),
            url: self.url.unwrap_or_default(),
            userTypedURL: self.userTypedURL.unwrap_or_default(),
            title: self.title.unwrap_or_default(),
            transitionType: self.transitionType.unwrap_or_default(),
        }
    }
}

/// Screencast frame metadata.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameMetadata {
    /// Top offset in DIP.
    offsetTop: f64,
    /// Page scale factor.
    pageScaleFactor: f64,
    /// Device screen width in DIP.
    deviceWidth: f64,
    /// Device screen height in DIP.
    deviceHeight: f64,
    /// Position of horizontal scroll in CSS pixels.
    scrollOffsetX: f64,
    /// Position of vertical scroll in CSS pixels.
    scrollOffsetY: f64,
    /// Frame swap timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::network::TimeSinceEpoch>,
}

impl ScreencastFrameMetadata {
    pub fn builder() -> ScreencastFrameMetadataBuilder { ScreencastFrameMetadataBuilder::default() }
    pub fn offsetTop(&self) -> f64 { self.offsetTop }
    pub fn pageScaleFactor(&self) -> f64 { self.pageScaleFactor }
    pub fn deviceWidth(&self) -> f64 { self.deviceWidth }
    pub fn deviceHeight(&self) -> f64 { self.deviceHeight }
    pub fn scrollOffsetX(&self) -> f64 { self.scrollOffsetX }
    pub fn scrollOffsetY(&self) -> f64 { self.scrollOffsetY }
    pub fn timestamp(&self) -> Option<&crate::network::TimeSinceEpoch> { self.timestamp.as_ref() }
}

#[derive(Default)]
pub struct ScreencastFrameMetadataBuilder {
    offsetTop: Option<f64>,
    pageScaleFactor: Option<f64>,
    deviceWidth: Option<f64>,
    deviceHeight: Option<f64>,
    scrollOffsetX: Option<f64>,
    scrollOffsetY: Option<f64>,
    timestamp: Option<crate::network::TimeSinceEpoch>,
}

impl ScreencastFrameMetadataBuilder {
    /// Top offset in DIP.
    pub fn offsetTop(mut self, offsetTop: f64) -> Self { self.offsetTop = Some(offsetTop); self }
    /// Page scale factor.
    pub fn pageScaleFactor(mut self, pageScaleFactor: f64) -> Self { self.pageScaleFactor = Some(pageScaleFactor); self }
    /// Device screen width in DIP.
    pub fn deviceWidth(mut self, deviceWidth: f64) -> Self { self.deviceWidth = Some(deviceWidth); self }
    /// Device screen height in DIP.
    pub fn deviceHeight(mut self, deviceHeight: f64) -> Self { self.deviceHeight = Some(deviceHeight); self }
    /// Position of horizontal scroll in CSS pixels.
    pub fn scrollOffsetX(mut self, scrollOffsetX: f64) -> Self { self.scrollOffsetX = Some(scrollOffsetX); self }
    /// Position of vertical scroll in CSS pixels.
    pub fn scrollOffsetY(mut self, scrollOffsetY: f64) -> Self { self.scrollOffsetY = Some(scrollOffsetY); self }
    /// Frame swap timestamp.
    pub fn timestamp(mut self, timestamp: crate::network::TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    pub fn build(self) -> ScreencastFrameMetadata {
        ScreencastFrameMetadata {
            offsetTop: self.offsetTop.unwrap_or_default(),
            pageScaleFactor: self.pageScaleFactor.unwrap_or_default(),
            deviceWidth: self.deviceWidth.unwrap_or_default(),
            deviceHeight: self.deviceHeight.unwrap_or_default(),
            scrollOffsetX: self.scrollOffsetX.unwrap_or_default(),
            scrollOffsetY: self.scrollOffsetY.unwrap_or_default(),
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
    pub fn builder() -> AppManifestErrorBuilder<'a> { AppManifestErrorBuilder::default() }
    pub fn message(&self) -> &str { self.message.as_ref() }
    pub fn critical(&self) -> i64 { self.critical }
    pub fn line(&self) -> i64 { self.line }
    pub fn column(&self) -> i64 { self.column }
}

#[derive(Default)]
pub struct AppManifestErrorBuilder<'a> {
    message: Option<Cow<'a, str>>,
    critical: Option<i64>,
    line: Option<i64>,
    column: Option<i64>,
}

impl<'a> AppManifestErrorBuilder<'a> {
    /// Error message.
    pub fn message(mut self, message: impl Into<Cow<'a, str>>) -> Self { self.message = Some(message.into()); self }
    /// If critical, this is a non-recoverable parse error.
    pub fn critical(mut self, critical: i64) -> Self { self.critical = Some(critical); self }
    /// Error line.
    pub fn line(mut self, line: i64) -> Self { self.line = Some(line); self }
    /// Error column.
    pub fn column(mut self, column: i64) -> Self { self.column = Some(column); self }
    pub fn build(self) -> AppManifestError<'a> {
        AppManifestError {
            message: self.message.unwrap_or_default(),
            critical: self.critical.unwrap_or_default(),
            line: self.line.unwrap_or_default(),
            column: self.column.unwrap_or_default(),
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
    pub fn builder() -> AppManifestParsedPropertiesBuilder<'a> { AppManifestParsedPropertiesBuilder::default() }
    pub fn scope(&self) -> &str { self.scope.as_ref() }
}

#[derive(Default)]
pub struct AppManifestParsedPropertiesBuilder<'a> {
    scope: Option<Cow<'a, str>>,
}

impl<'a> AppManifestParsedPropertiesBuilder<'a> {
    /// Computed scope value
    pub fn scope(mut self, scope: impl Into<Cow<'a, str>>) -> Self { self.scope = Some(scope.into()); self }
    pub fn build(self) -> AppManifestParsedProperties<'a> {
        AppManifestParsedProperties {
            scope: self.scope.unwrap_or_default(),
        }
    }
}

/// Layout viewport position and dimensions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutViewport {
    /// Horizontal offset relative to the document (CSS pixels).
    pageX: i64,
    /// Vertical offset relative to the document (CSS pixels).
    pageY: i64,
    /// Width (CSS pixels), excludes scrollbar if present.
    clientWidth: u64,
    /// Height (CSS pixels), excludes scrollbar if present.
    clientHeight: i64,
}

impl LayoutViewport {
    pub fn builder() -> LayoutViewportBuilder { LayoutViewportBuilder::default() }
    pub fn pageX(&self) -> i64 { self.pageX }
    pub fn pageY(&self) -> i64 { self.pageY }
    pub fn clientWidth(&self) -> u64 { self.clientWidth }
    pub fn clientHeight(&self) -> i64 { self.clientHeight }
}

#[derive(Default)]
pub struct LayoutViewportBuilder {
    pageX: Option<i64>,
    pageY: Option<i64>,
    clientWidth: Option<u64>,
    clientHeight: Option<i64>,
}

impl LayoutViewportBuilder {
    /// Horizontal offset relative to the document (CSS pixels).
    pub fn pageX(mut self, pageX: i64) -> Self { self.pageX = Some(pageX); self }
    /// Vertical offset relative to the document (CSS pixels).
    pub fn pageY(mut self, pageY: i64) -> Self { self.pageY = Some(pageY); self }
    /// Width (CSS pixels), excludes scrollbar if present.
    pub fn clientWidth(mut self, clientWidth: u64) -> Self { self.clientWidth = Some(clientWidth); self }
    /// Height (CSS pixels), excludes scrollbar if present.
    pub fn clientHeight(mut self, clientHeight: i64) -> Self { self.clientHeight = Some(clientHeight); self }
    pub fn build(self) -> LayoutViewport {
        LayoutViewport {
            pageX: self.pageX.unwrap_or_default(),
            pageY: self.pageY.unwrap_or_default(),
            clientWidth: self.clientWidth.unwrap_or_default(),
            clientHeight: self.clientHeight.unwrap_or_default(),
        }
    }
}

/// Visual viewport position, dimensions, and scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VisualViewport {
    /// Horizontal offset relative to the layout viewport (CSS pixels).
    offsetX: f64,
    /// Vertical offset relative to the layout viewport (CSS pixels).
    offsetY: f64,
    /// Horizontal offset relative to the document (CSS pixels).
    pageX: f64,
    /// Vertical offset relative to the document (CSS pixels).
    pageY: f64,
    /// Width (CSS pixels), excludes scrollbar if present.
    clientWidth: f64,
    /// Height (CSS pixels), excludes scrollbar if present.
    clientHeight: f64,
    /// Scale relative to the ideal viewport (size at width=device-width).
    scale: f64,
    /// Page zoom factor (CSS to device independent pixels ratio).
    #[serde(skip_serializing_if = "Option::is_none")]
    zoom: Option<f64>,
}

impl VisualViewport {
    pub fn builder() -> VisualViewportBuilder { VisualViewportBuilder::default() }
    pub fn offsetX(&self) -> f64 { self.offsetX }
    pub fn offsetY(&self) -> f64 { self.offsetY }
    pub fn pageX(&self) -> f64 { self.pageX }
    pub fn pageY(&self) -> f64 { self.pageY }
    pub fn clientWidth(&self) -> f64 { self.clientWidth }
    pub fn clientHeight(&self) -> f64 { self.clientHeight }
    pub fn scale(&self) -> f64 { self.scale }
    pub fn zoom(&self) -> Option<f64> { self.zoom }
}

#[derive(Default)]
pub struct VisualViewportBuilder {
    offsetX: Option<f64>,
    offsetY: Option<f64>,
    pageX: Option<f64>,
    pageY: Option<f64>,
    clientWidth: Option<f64>,
    clientHeight: Option<f64>,
    scale: Option<f64>,
    zoom: Option<f64>,
}

impl VisualViewportBuilder {
    /// Horizontal offset relative to the layout viewport (CSS pixels).
    pub fn offsetX(mut self, offsetX: f64) -> Self { self.offsetX = Some(offsetX); self }
    /// Vertical offset relative to the layout viewport (CSS pixels).
    pub fn offsetY(mut self, offsetY: f64) -> Self { self.offsetY = Some(offsetY); self }
    /// Horizontal offset relative to the document (CSS pixels).
    pub fn pageX(mut self, pageX: f64) -> Self { self.pageX = Some(pageX); self }
    /// Vertical offset relative to the document (CSS pixels).
    pub fn pageY(mut self, pageY: f64) -> Self { self.pageY = Some(pageY); self }
    /// Width (CSS pixels), excludes scrollbar if present.
    pub fn clientWidth(mut self, clientWidth: f64) -> Self { self.clientWidth = Some(clientWidth); self }
    /// Height (CSS pixels), excludes scrollbar if present.
    pub fn clientHeight(mut self, clientHeight: f64) -> Self { self.clientHeight = Some(clientHeight); self }
    /// Scale relative to the ideal viewport (size at width=device-width).
    pub fn scale(mut self, scale: f64) -> Self { self.scale = Some(scale); self }
    /// Page zoom factor (CSS to device independent pixels ratio).
    pub fn zoom(mut self, zoom: f64) -> Self { self.zoom = Some(zoom); self }
    pub fn build(self) -> VisualViewport {
        VisualViewport {
            offsetX: self.offsetX.unwrap_or_default(),
            offsetY: self.offsetY.unwrap_or_default(),
            pageX: self.pageX.unwrap_or_default(),
            pageY: self.pageY.unwrap_or_default(),
            clientWidth: self.clientWidth.unwrap_or_default(),
            clientHeight: self.clientHeight.unwrap_or_default(),
            scale: self.scale.unwrap_or_default(),
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
    pub fn builder() -> ViewportBuilder { ViewportBuilder::default() }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn width(&self) -> f64 { self.width }
    pub fn height(&self) -> f64 { self.height }
    pub fn scale(&self) -> f64 { self.scale }
}

#[derive(Default)]
pub struct ViewportBuilder {
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    scale: Option<f64>,
}

impl ViewportBuilder {
    /// X offset in device independent pixels (dip).
    pub fn x(mut self, x: f64) -> Self { self.x = Some(x); self }
    /// Y offset in device independent pixels (dip).
    pub fn y(mut self, y: f64) -> Self { self.y = Some(y); self }
    /// Rectangle width in device independent pixels (dip).
    pub fn width(mut self, width: f64) -> Self { self.width = Some(width); self }
    /// Rectangle height in device independent pixels (dip).
    pub fn height(mut self, height: f64) -> Self { self.height = Some(height); self }
    /// Page scale factor.
    pub fn scale(mut self, scale: f64) -> Self { self.scale = Some(scale); self }
    pub fn build(self) -> Viewport {
        Viewport {
            x: self.x.unwrap_or_default(),
            y: self.y.unwrap_or_default(),
            width: self.width.unwrap_or_default(),
            height: self.height.unwrap_or_default(),
            scale: self.scale.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    sansSerif: Option<Cow<'a, str>>,
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
    pub fn builder() -> FontFamiliesBuilder<'a> { FontFamiliesBuilder::default() }
    pub fn standard(&self) -> Option<&str> { self.standard.as_deref() }
    pub fn fixed(&self) -> Option<&str> { self.fixed.as_deref() }
    pub fn serif(&self) -> Option<&str> { self.serif.as_deref() }
    pub fn sansSerif(&self) -> Option<&str> { self.sansSerif.as_deref() }
    pub fn cursive(&self) -> Option<&str> { self.cursive.as_deref() }
    pub fn fantasy(&self) -> Option<&str> { self.fantasy.as_deref() }
    pub fn math(&self) -> Option<&str> { self.math.as_deref() }
}

#[derive(Default)]
pub struct FontFamiliesBuilder<'a> {
    standard: Option<Cow<'a, str>>,
    fixed: Option<Cow<'a, str>>,
    serif: Option<Cow<'a, str>>,
    sansSerif: Option<Cow<'a, str>>,
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
    pub fn sansSerif(mut self, sansSerif: impl Into<Cow<'a, str>>) -> Self { self.sansSerif = Some(sansSerif.into()); self }
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
            sansSerif: self.sansSerif,
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
    fontFamilies: FontFamilies<'a>,
}

impl<'a> ScriptFontFamilies<'a> {
    pub fn builder() -> ScriptFontFamiliesBuilder<'a> { ScriptFontFamiliesBuilder::default() }
    pub fn script(&self) -> &str { self.script.as_ref() }
    pub fn fontFamilies(&self) -> &FontFamilies<'a> { &self.fontFamilies }
}

#[derive(Default)]
pub struct ScriptFontFamiliesBuilder<'a> {
    script: Option<Cow<'a, str>>,
    fontFamilies: Option<FontFamilies<'a>>,
}

impl<'a> ScriptFontFamiliesBuilder<'a> {
    /// Name of the script which these font families are defined for.
    pub fn script(mut self, script: impl Into<Cow<'a, str>>) -> Self { self.script = Some(script.into()); self }
    /// Generic font families collection for the script.
    pub fn fontFamilies(mut self, fontFamilies: FontFamilies<'a>) -> Self { self.fontFamilies = Some(fontFamilies); self }
    pub fn build(self) -> ScriptFontFamilies<'a> {
        ScriptFontFamilies {
            script: self.script.unwrap_or_default(),
            fontFamilies: self.fontFamilies.unwrap_or_default(),
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
    pub fn builder() -> FontSizesBuilder { FontSizesBuilder::default() }
    pub fn standard(&self) -> Option<i64> { self.standard }
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
    pub fn builder() -> InstallabilityErrorArgumentBuilder<'a> { InstallabilityErrorArgumentBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct InstallabilityErrorArgumentBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> InstallabilityErrorArgumentBuilder<'a> {
    /// Argument name (e.g. name:'minimum-icon-size-in-pixels').
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Argument value (e.g. value:'64').
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> InstallabilityErrorArgument<'a> {
        InstallabilityErrorArgument {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}

/// The installability error

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityError<'a> {
    /// The error id (e.g. 'manifest-missing-suitable-icon').
    errorId: Cow<'a, str>,
    /// The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).
    errorArguments: Vec<InstallabilityErrorArgument<'a>>,
}

impl<'a> InstallabilityError<'a> {
    pub fn builder() -> InstallabilityErrorBuilder<'a> { InstallabilityErrorBuilder::default() }
    pub fn errorId(&self) -> &str { self.errorId.as_ref() }
    pub fn errorArguments(&self) -> &[InstallabilityErrorArgument<'a>] { &self.errorArguments }
}

#[derive(Default)]
pub struct InstallabilityErrorBuilder<'a> {
    errorId: Option<Cow<'a, str>>,
    errorArguments: Option<Vec<InstallabilityErrorArgument<'a>>>,
}

impl<'a> InstallabilityErrorBuilder<'a> {
    /// The error id (e.g. 'manifest-missing-suitable-icon').
    pub fn errorId(mut self, errorId: impl Into<Cow<'a, str>>) -> Self { self.errorId = Some(errorId.into()); self }
    /// The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).
    pub fn errorArguments(mut self, errorArguments: Vec<InstallabilityErrorArgument<'a>>) -> Self { self.errorArguments = Some(errorArguments); self }
    pub fn build(self) -> InstallabilityError<'a> {
        InstallabilityError {
            errorId: self.errorId.unwrap_or_default(),
            errorArguments: self.errorArguments.unwrap_or_default(),
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
    pub fn builder() -> CompilationCacheParamsBuilder<'a> { CompilationCacheParamsBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn eager(&self) -> Option<bool> { self.eager }
}

#[derive(Default)]
pub struct CompilationCacheParamsBuilder<'a> {
    url: Option<Cow<'a, str>>,
    eager: Option<bool>,
}

impl<'a> CompilationCacheParamsBuilder<'a> {
    /// The URL of the script to produce a compilation cache entry for.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// A hint to the backend whether eager compilation is recommended.
    /// (the actual compilation mode used is upon backend discretion).
    pub fn eager(mut self, eager: bool) -> Self { self.eager = Some(eager); self }
    pub fn build(self) -> CompilationCacheParams<'a> {
        CompilationCacheParams {
            url: self.url.unwrap_or_default(),
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
    pub fn builder() -> FileFilterBuilder<'a> { FileFilterBuilder::default() }
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
    launchType: Cow<'a, str>,
}

impl<'a> FileHandler<'a> {
    pub fn builder() -> FileHandlerBuilder<'a> { FileHandlerBuilder::default() }
    pub fn action(&self) -> &str { self.action.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn icons(&self) -> Option<&[ImageResource<'a>]> { self.icons.as_deref() }
    pub fn accepts(&self) -> Option<&[FileFilter<'a>]> { self.accepts.as_deref() }
    pub fn launchType(&self) -> &str { self.launchType.as_ref() }
}

#[derive(Default)]
pub struct FileHandlerBuilder<'a> {
    action: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    icons: Option<Vec<ImageResource<'a>>>,
    accepts: Option<Vec<FileFilter<'a>>>,
    launchType: Option<Cow<'a, str>>,
}

impl<'a> FileHandlerBuilder<'a> {
    pub fn action(mut self, action: impl Into<Cow<'a, str>>) -> Self { self.action = Some(action.into()); self }
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn icons(mut self, icons: Vec<ImageResource<'a>>) -> Self { self.icons = Some(icons); self }
    /// Mimic a map, name is the key, accepts is the value.
    pub fn accepts(mut self, accepts: Vec<FileFilter<'a>>) -> Self { self.accepts = Some(accepts); self }
    /// Won't repeat the enums, using string for easy comparison. Same as the
    /// other enums below.
    pub fn launchType(mut self, launchType: impl Into<Cow<'a, str>>) -> Self { self.launchType = Some(launchType.into()); self }
    pub fn build(self) -> FileHandler<'a> {
        FileHandler {
            action: self.action.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            icons: self.icons,
            accepts: self.accepts,
            launchType: self.launchType.unwrap_or_default(),
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
    pub fn builder() -> ImageResourceBuilder<'a> { ImageResourceBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn sizes(&self) -> Option<&str> { self.sizes.as_deref() }
    pub fn type_(&self) -> Option<&str> { self.type_.as_deref() }
}

#[derive(Default)]
pub struct ImageResourceBuilder<'a> {
    url: Option<Cow<'a, str>>,
    sizes: Option<Cow<'a, str>>,
    type_: Option<Cow<'a, str>>,
}

impl<'a> ImageResourceBuilder<'a> {
    /// The src field in the definition, but changing to url in favor of
    /// consistency.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn sizes(mut self, sizes: impl Into<Cow<'a, str>>) -> Self { self.sizes = Some(sizes.into()); self }
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    pub fn build(self) -> ImageResource<'a> {
        ImageResource {
            url: self.url.unwrap_or_default(),
            sizes: self.sizes,
            type_: self.type_,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchHandler<'a> {
    clientMode: Cow<'a, str>,
}

impl<'a> LaunchHandler<'a> {
    pub fn builder() -> LaunchHandlerBuilder<'a> { LaunchHandlerBuilder::default() }
    pub fn clientMode(&self) -> &str { self.clientMode.as_ref() }
}

#[derive(Default)]
pub struct LaunchHandlerBuilder<'a> {
    clientMode: Option<Cow<'a, str>>,
}

impl<'a> LaunchHandlerBuilder<'a> {
    pub fn clientMode(mut self, clientMode: impl Into<Cow<'a, str>>) -> Self { self.clientMode = Some(clientMode.into()); self }
    pub fn build(self) -> LaunchHandler<'a> {
        LaunchHandler {
            clientMode: self.clientMode.unwrap_or_default(),
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
    pub fn builder() -> ProtocolHandlerBuilder<'a> { ProtocolHandlerBuilder::default() }
    pub fn protocol(&self) -> &str { self.protocol.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}

#[derive(Default)]
pub struct ProtocolHandlerBuilder<'a> {
    protocol: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
}

impl<'a> ProtocolHandlerBuilder<'a> {
    pub fn protocol(mut self, protocol: impl Into<Cow<'a, str>>) -> Self { self.protocol = Some(protocol.into()); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> ProtocolHandler<'a> {
        ProtocolHandler {
            protocol: self.protocol.unwrap_or_default(),
            url: self.url.unwrap_or_default(),
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
    pub fn builder() -> RelatedApplicationBuilder<'a> { RelatedApplicationBuilder::default() }
    pub fn id(&self) -> Option<&str> { self.id.as_deref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}

#[derive(Default)]
pub struct RelatedApplicationBuilder<'a> {
    id: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
}

impl<'a> RelatedApplicationBuilder<'a> {
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> RelatedApplication<'a> {
        RelatedApplication {
            id: self.id,
            url: self.url.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScopeExtension<'a> {
    /// Instead of using tuple, this field always returns the serialized string
    /// for easy understanding and comparison.
    origin: Cow<'a, str>,
    hasOriginWildcard: bool,
}

impl<'a> ScopeExtension<'a> {
    pub fn builder() -> ScopeExtensionBuilder<'a> { ScopeExtensionBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn hasOriginWildcard(&self) -> bool { self.hasOriginWildcard }
}

#[derive(Default)]
pub struct ScopeExtensionBuilder<'a> {
    origin: Option<Cow<'a, str>>,
    hasOriginWildcard: Option<bool>,
}

impl<'a> ScopeExtensionBuilder<'a> {
    /// Instead of using tuple, this field always returns the serialized string
    /// for easy understanding and comparison.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn hasOriginWildcard(mut self, hasOriginWildcard: bool) -> Self { self.hasOriginWildcard = Some(hasOriginWildcard); self }
    pub fn build(self) -> ScopeExtension<'a> {
        ScopeExtension {
            origin: self.origin.unwrap_or_default(),
            hasOriginWildcard: self.hasOriginWildcard.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Screenshot<'a> {
    image: ImageResource<'a>,
    formFactor: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Cow<'a, str>>,
}

impl<'a> Screenshot<'a> {
    pub fn builder() -> ScreenshotBuilder<'a> { ScreenshotBuilder::default() }
    pub fn image(&self) -> &ImageResource<'a> { &self.image }
    pub fn formFactor(&self) -> &str { self.formFactor.as_ref() }
    pub fn label(&self) -> Option<&str> { self.label.as_deref() }
}

#[derive(Default)]
pub struct ScreenshotBuilder<'a> {
    image: Option<ImageResource<'a>>,
    formFactor: Option<Cow<'a, str>>,
    label: Option<Cow<'a, str>>,
}

impl<'a> ScreenshotBuilder<'a> {
    pub fn image(mut self, image: ImageResource<'a>) -> Self { self.image = Some(image); self }
    pub fn formFactor(mut self, formFactor: impl Into<Cow<'a, str>>) -> Self { self.formFactor = Some(formFactor.into()); self }
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self { self.label = Some(label.into()); self }
    pub fn build(self) -> Screenshot<'a> {
        Screenshot {
            image: self.image.unwrap_or_default(),
            formFactor: self.formFactor.unwrap_or_default(),
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
    pub fn builder() -> ShareTargetBuilder<'a> { ShareTargetBuilder::default() }
    pub fn action(&self) -> &str { self.action.as_ref() }
    pub fn method(&self) -> &str { self.method.as_ref() }
    pub fn enctype(&self) -> &str { self.enctype.as_ref() }
    pub fn title(&self) -> Option<&str> { self.title.as_deref() }
    pub fn text(&self) -> Option<&str> { self.text.as_deref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn files(&self) -> Option<&[FileFilter<'a>]> { self.files.as_deref() }
}

#[derive(Default)]
pub struct ShareTargetBuilder<'a> {
    action: Option<Cow<'a, str>>,
    method: Option<Cow<'a, str>>,
    enctype: Option<Cow<'a, str>>,
    title: Option<Cow<'a, str>>,
    text: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    files: Option<Vec<FileFilter<'a>>>,
}

impl<'a> ShareTargetBuilder<'a> {
    pub fn action(mut self, action: impl Into<Cow<'a, str>>) -> Self { self.action = Some(action.into()); self }
    pub fn method(mut self, method: impl Into<Cow<'a, str>>) -> Self { self.method = Some(method.into()); self }
    pub fn enctype(mut self, enctype: impl Into<Cow<'a, str>>) -> Self { self.enctype = Some(enctype.into()); self }
    /// Embed the ShareTargetParams
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self { self.title = Some(title.into()); self }
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn files(mut self, files: Vec<FileFilter<'a>>) -> Self { self.files = Some(files); self }
    pub fn build(self) -> ShareTarget<'a> {
        ShareTarget {
            action: self.action.unwrap_or_default(),
            method: self.method.unwrap_or_default(),
            enctype: self.enctype.unwrap_or_default(),
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
    pub fn builder() -> ShortcutBuilder<'a> { ShortcutBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}

#[derive(Default)]
pub struct ShortcutBuilder<'a> {
    name: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
}

impl<'a> ShortcutBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> Shortcut<'a> {
        Shortcut {
            name: self.name.unwrap_or_default(),
            url: self.url.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebAppManifest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    backgroundColor: Option<Cow<'a, str>>,
    /// The extra description provided by the manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display: Option<Cow<'a, str>>,
    /// The overrided display mode controlled by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    displayOverrides: Option<Vec<Cow<'a, str>>>,
    /// The handlers to open files.
    #[serde(skip_serializing_if = "Option::is_none")]
    fileHandlers: Option<Vec<FileHandler<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icons: Option<Vec<ImageResource<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<Cow<'a, str>>,
    /// TODO(crbug.com/1231886): This field is non-standard and part of a Chrome
    /// experiment. See:
    /// https://github.com/WICG/web-app-launch/blob/main/launch_handler.md
    #[serde(skip_serializing_if = "Option::is_none")]
    launchHandler: Option<LaunchHandler<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferRelatedApplications: Option<bool>,
    /// The handlers to open protocols.
    #[serde(skip_serializing_if = "Option::is_none")]
    protocolHandlers: Option<Vec<ProtocolHandler<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relatedApplications: Option<Vec<RelatedApplication<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Cow<'a, str>>,
    /// Non-standard, see
    /// https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md
    #[serde(skip_serializing_if = "Option::is_none")]
    scopeExtensions: Option<Vec<ScopeExtension<'a>>>,
    /// The screenshots used by chromium.
    #[serde(skip_serializing_if = "Option::is_none")]
    screenshots: Option<Vec<Screenshot<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shareTarget: Option<ShareTarget<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shortName: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shortcuts: Option<Vec<Shortcut<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startUrl: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    themeColor: Option<Cow<'a, str>>,
}

impl<'a> WebAppManifest<'a> {
    pub fn builder() -> WebAppManifestBuilder<'a> { WebAppManifestBuilder::default() }
    pub fn backgroundColor(&self) -> Option<&str> { self.backgroundColor.as_deref() }
    pub fn description(&self) -> Option<&str> { self.description.as_deref() }
    pub fn dir(&self) -> Option<&str> { self.dir.as_deref() }
    pub fn display(&self) -> Option<&str> { self.display.as_deref() }
    pub fn displayOverrides(&self) -> Option<&[Cow<'a, str>]> { self.displayOverrides.as_deref() }
    pub fn fileHandlers(&self) -> Option<&[FileHandler<'a>]> { self.fileHandlers.as_deref() }
    pub fn icons(&self) -> Option<&[ImageResource<'a>]> { self.icons.as_deref() }
    pub fn id(&self) -> Option<&str> { self.id.as_deref() }
    pub fn lang(&self) -> Option<&str> { self.lang.as_deref() }
    pub fn launchHandler(&self) -> Option<&LaunchHandler<'a>> { self.launchHandler.as_ref() }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn orientation(&self) -> Option<&str> { self.orientation.as_deref() }
    pub fn preferRelatedApplications(&self) -> Option<bool> { self.preferRelatedApplications }
    pub fn protocolHandlers(&self) -> Option<&[ProtocolHandler<'a>]> { self.protocolHandlers.as_deref() }
    pub fn relatedApplications(&self) -> Option<&[RelatedApplication<'a>]> { self.relatedApplications.as_deref() }
    pub fn scope(&self) -> Option<&str> { self.scope.as_deref() }
    pub fn scopeExtensions(&self) -> Option<&[ScopeExtension<'a>]> { self.scopeExtensions.as_deref() }
    pub fn screenshots(&self) -> Option<&[Screenshot<'a>]> { self.screenshots.as_deref() }
    pub fn shareTarget(&self) -> Option<&ShareTarget<'a>> { self.shareTarget.as_ref() }
    pub fn shortName(&self) -> Option<&str> { self.shortName.as_deref() }
    pub fn shortcuts(&self) -> Option<&[Shortcut<'a>]> { self.shortcuts.as_deref() }
    pub fn startUrl(&self) -> Option<&str> { self.startUrl.as_deref() }
    pub fn themeColor(&self) -> Option<&str> { self.themeColor.as_deref() }
}

#[derive(Default)]
pub struct WebAppManifestBuilder<'a> {
    backgroundColor: Option<Cow<'a, str>>,
    description: Option<Cow<'a, str>>,
    dir: Option<Cow<'a, str>>,
    display: Option<Cow<'a, str>>,
    displayOverrides: Option<Vec<Cow<'a, str>>>,
    fileHandlers: Option<Vec<FileHandler<'a>>>,
    icons: Option<Vec<ImageResource<'a>>>,
    id: Option<Cow<'a, str>>,
    lang: Option<Cow<'a, str>>,
    launchHandler: Option<LaunchHandler<'a>>,
    name: Option<Cow<'a, str>>,
    orientation: Option<Cow<'a, str>>,
    preferRelatedApplications: Option<bool>,
    protocolHandlers: Option<Vec<ProtocolHandler<'a>>>,
    relatedApplications: Option<Vec<RelatedApplication<'a>>>,
    scope: Option<Cow<'a, str>>,
    scopeExtensions: Option<Vec<ScopeExtension<'a>>>,
    screenshots: Option<Vec<Screenshot<'a>>>,
    shareTarget: Option<ShareTarget<'a>>,
    shortName: Option<Cow<'a, str>>,
    shortcuts: Option<Vec<Shortcut<'a>>>,
    startUrl: Option<Cow<'a, str>>,
    themeColor: Option<Cow<'a, str>>,
}

impl<'a> WebAppManifestBuilder<'a> {
    pub fn backgroundColor(mut self, backgroundColor: impl Into<Cow<'a, str>>) -> Self { self.backgroundColor = Some(backgroundColor.into()); self }
    /// The extra description provided by the manifest.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self { self.description = Some(description.into()); self }
    pub fn dir(mut self, dir: impl Into<Cow<'a, str>>) -> Self { self.dir = Some(dir.into()); self }
    pub fn display(mut self, display: impl Into<Cow<'a, str>>) -> Self { self.display = Some(display.into()); self }
    /// The overrided display mode controlled by the user.
    pub fn displayOverrides(mut self, displayOverrides: Vec<Cow<'a, str>>) -> Self { self.displayOverrides = Some(displayOverrides); self }
    /// The handlers to open files.
    pub fn fileHandlers(mut self, fileHandlers: Vec<FileHandler<'a>>) -> Self { self.fileHandlers = Some(fileHandlers); self }
    pub fn icons(mut self, icons: Vec<ImageResource<'a>>) -> Self { self.icons = Some(icons); self }
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    pub fn lang(mut self, lang: impl Into<Cow<'a, str>>) -> Self { self.lang = Some(lang.into()); self }
    /// TODO(crbug.com/1231886): This field is non-standard and part of a Chrome
    /// experiment. See:
    /// https://github.com/WICG/web-app-launch/blob/main/launch_handler.md
    pub fn launchHandler(mut self, launchHandler: LaunchHandler<'a>) -> Self { self.launchHandler = Some(launchHandler); self }
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn orientation(mut self, orientation: impl Into<Cow<'a, str>>) -> Self { self.orientation = Some(orientation.into()); self }
    pub fn preferRelatedApplications(mut self, preferRelatedApplications: bool) -> Self { self.preferRelatedApplications = Some(preferRelatedApplications); self }
    /// The handlers to open protocols.
    pub fn protocolHandlers(mut self, protocolHandlers: Vec<ProtocolHandler<'a>>) -> Self { self.protocolHandlers = Some(protocolHandlers); self }
    pub fn relatedApplications(mut self, relatedApplications: Vec<RelatedApplication<'a>>) -> Self { self.relatedApplications = Some(relatedApplications); self }
    pub fn scope(mut self, scope: impl Into<Cow<'a, str>>) -> Self { self.scope = Some(scope.into()); self }
    /// Non-standard, see
    /// https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md
    pub fn scopeExtensions(mut self, scopeExtensions: Vec<ScopeExtension<'a>>) -> Self { self.scopeExtensions = Some(scopeExtensions); self }
    /// The screenshots used by chromium.
    pub fn screenshots(mut self, screenshots: Vec<Screenshot<'a>>) -> Self { self.screenshots = Some(screenshots); self }
    pub fn shareTarget(mut self, shareTarget: ShareTarget<'a>) -> Self { self.shareTarget = Some(shareTarget); self }
    pub fn shortName(mut self, shortName: impl Into<Cow<'a, str>>) -> Self { self.shortName = Some(shortName.into()); self }
    pub fn shortcuts(mut self, shortcuts: Vec<Shortcut<'a>>) -> Self { self.shortcuts = Some(shortcuts); self }
    pub fn startUrl(mut self, startUrl: impl Into<Cow<'a, str>>) -> Self { self.startUrl = Some(startUrl.into()); self }
    pub fn themeColor(mut self, themeColor: impl Into<Cow<'a, str>>) -> Self { self.themeColor = Some(themeColor.into()); self }
    pub fn build(self) -> WebAppManifest<'a> {
        WebAppManifest {
            backgroundColor: self.backgroundColor,
            description: self.description,
            dir: self.dir,
            display: self.display,
            displayOverrides: self.displayOverrides,
            fileHandlers: self.fileHandlers,
            icons: self.icons,
            id: self.id,
            lang: self.lang,
            launchHandler: self.launchHandler,
            name: self.name,
            orientation: self.orientation,
            preferRelatedApplications: self.preferRelatedApplications,
            protocolHandlers: self.protocolHandlers,
            relatedApplications: self.relatedApplications,
            scope: self.scope,
            scopeExtensions: self.scopeExtensions,
            screenshots: self.screenshots,
            shareTarget: self.shareTarget,
            shortName: self.shortName,
            shortcuts: self.shortcuts,
            startUrl: self.startUrl,
            themeColor: self.themeColor,
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
    lineNumber: i64,
    /// Column number in the script (0-based).
    columnNumber: i64,
}

impl<'a> BackForwardCacheBlockingDetails<'a> {
    pub fn builder() -> BackForwardCacheBlockingDetailsBuilder<'a> { BackForwardCacheBlockingDetailsBuilder::default() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn function(&self) -> Option<&str> { self.function.as_deref() }
    pub fn lineNumber(&self) -> i64 { self.lineNumber }
    pub fn columnNumber(&self) -> i64 { self.columnNumber }
}

#[derive(Default)]
pub struct BackForwardCacheBlockingDetailsBuilder<'a> {
    url: Option<Cow<'a, str>>,
    function: Option<Cow<'a, str>>,
    lineNumber: Option<i64>,
    columnNumber: Option<i64>,
}

impl<'a> BackForwardCacheBlockingDetailsBuilder<'a> {
    /// Url of the file where blockage happened. Optional because of tests.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Function name where blockage happened. Optional because of anonymous functions and tests.
    pub fn function(mut self, function: impl Into<Cow<'a, str>>) -> Self { self.function = Some(function.into()); self }
    /// Line number in the script (0-based).
    pub fn lineNumber(mut self, lineNumber: i64) -> Self { self.lineNumber = Some(lineNumber); self }
    /// Column number in the script (0-based).
    pub fn columnNumber(mut self, columnNumber: i64) -> Self { self.columnNumber = Some(columnNumber); self }
    pub fn build(self) -> BackForwardCacheBlockingDetails<'a> {
        BackForwardCacheBlockingDetails {
            url: self.url,
            function: self.function,
            lineNumber: self.lineNumber.unwrap_or_default(),
            columnNumber: self.columnNumber.unwrap_or_default(),
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
    pub fn builder() -> BackForwardCacheNotRestoredExplanationBuilder<'a> { BackForwardCacheNotRestoredExplanationBuilder::default() }
    pub fn type_(&self) -> &BackForwardCacheNotRestoredReasonType { &self.type_ }
    pub fn reason(&self) -> &BackForwardCacheNotRestoredReason { &self.reason }
    pub fn context(&self) -> Option<&str> { self.context.as_deref() }
    pub fn details(&self) -> Option<&[BackForwardCacheBlockingDetails<'a>]> { self.details.as_deref() }
}

#[derive(Default)]
pub struct BackForwardCacheNotRestoredExplanationBuilder<'a> {
    type_: Option<BackForwardCacheNotRestoredReasonType>,
    reason: Option<BackForwardCacheNotRestoredReason>,
    context: Option<Cow<'a, str>>,
    details: Option<Vec<BackForwardCacheBlockingDetails<'a>>>,
}

impl<'a> BackForwardCacheNotRestoredExplanationBuilder<'a> {
    /// Type of the reason
    pub fn type_(mut self, type_: BackForwardCacheNotRestoredReasonType) -> Self { self.type_ = Some(type_); self }
    /// Not restored reason
    pub fn reason(mut self, reason: BackForwardCacheNotRestoredReason) -> Self { self.reason = Some(reason); self }
    /// Context associated with the reason. The meaning of this context is
    /// dependent on the reason:
    /// - EmbedderExtensionSentMessageToCachedFrame: the extension ID.
    pub fn context(mut self, context: impl Into<Cow<'a, str>>) -> Self { self.context = Some(context.into()); self }
    pub fn details(mut self, details: Vec<BackForwardCacheBlockingDetails<'a>>) -> Self { self.details = Some(details); self }
    pub fn build(self) -> BackForwardCacheNotRestoredExplanation<'a> {
        BackForwardCacheNotRestoredExplanation {
            type_: self.type_.unwrap_or_default(),
            reason: self.reason.unwrap_or_default(),
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
    pub fn builder() -> BackForwardCacheNotRestoredExplanationTreeBuilder<'a> { BackForwardCacheNotRestoredExplanationTreeBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn explanations(&self) -> &[BackForwardCacheNotRestoredExplanation<'a>] { &self.explanations }
    pub fn children(&self) -> &[Box<BackForwardCacheNotRestoredExplanationTree<'a>>] { &self.children }
}

#[derive(Default)]
pub struct BackForwardCacheNotRestoredExplanationTreeBuilder<'a> {
    url: Option<Cow<'a, str>>,
    explanations: Option<Vec<BackForwardCacheNotRestoredExplanation<'a>>>,
    children: Option<Vec<Box<BackForwardCacheNotRestoredExplanationTree<'a>>>>,
}

impl<'a> BackForwardCacheNotRestoredExplanationTreeBuilder<'a> {
    /// URL of each frame
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Not restored reasons of each frame
    pub fn explanations(mut self, explanations: Vec<BackForwardCacheNotRestoredExplanation<'a>>) -> Self { self.explanations = Some(explanations); self }
    /// Array of children frame
    pub fn children(mut self, children: Vec<Box<BackForwardCacheNotRestoredExplanationTree<'a>>>) -> Self { self.children = Some(children); self }
    pub fn build(self) -> BackForwardCacheNotRestoredExplanationTree<'a> {
        BackForwardCacheNotRestoredExplanationTree {
            url: self.url.unwrap_or_default(),
            explanations: self.explanations.unwrap_or_default(),
            children: self.children.unwrap_or_default(),
        }
    }
}

/// Deprecated, please use addScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnLoadParams<'a> {
    scriptSource: Cow<'a, str>,
}

impl<'a> AddScriptToEvaluateOnLoadParams<'a> {
    pub fn builder() -> AddScriptToEvaluateOnLoadParamsBuilder<'a> { AddScriptToEvaluateOnLoadParamsBuilder::default() }
    pub fn scriptSource(&self) -> &str { self.scriptSource.as_ref() }
}

#[derive(Default)]
pub struct AddScriptToEvaluateOnLoadParamsBuilder<'a> {
    scriptSource: Option<Cow<'a, str>>,
}

impl<'a> AddScriptToEvaluateOnLoadParamsBuilder<'a> {
    pub fn scriptSource(mut self, scriptSource: impl Into<Cow<'a, str>>) -> Self { self.scriptSource = Some(scriptSource.into()); self }
    pub fn build(self) -> AddScriptToEvaluateOnLoadParams<'a> {
        AddScriptToEvaluateOnLoadParams {
            scriptSource: self.scriptSource.unwrap_or_default(),
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
    pub fn builder() -> AddScriptToEvaluateOnLoadReturnsBuilder<'a> { AddScriptToEvaluateOnLoadReturnsBuilder::default() }
    pub fn identifier(&self) -> &ScriptIdentifier<'a> { &self.identifier }
}

#[derive(Default)]
pub struct AddScriptToEvaluateOnLoadReturnsBuilder<'a> {
    identifier: Option<ScriptIdentifier<'a>>,
}

impl<'a> AddScriptToEvaluateOnLoadReturnsBuilder<'a> {
    /// Identifier of the added script.
    pub fn identifier(mut self, identifier: ScriptIdentifier<'a>) -> Self { self.identifier = Some(identifier); self }
    pub fn build(self) -> AddScriptToEvaluateOnLoadReturns<'a> {
        AddScriptToEvaluateOnLoadReturns {
            identifier: self.identifier.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    worldName: Option<Cow<'a, str>>,
    /// Specifies whether command line API should be available to the script, defaults
    /// to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    includeCommandLineAPI: Option<bool>,
    /// If true, runs the script immediately on existing execution contexts or worlds.
    /// Default: false.
    #[serde(skip_serializing_if = "Option::is_none")]
    runImmediately: Option<bool>,
}

impl<'a> AddScriptToEvaluateOnNewDocumentParams<'a> {
    pub fn builder() -> AddScriptToEvaluateOnNewDocumentParamsBuilder<'a> { AddScriptToEvaluateOnNewDocumentParamsBuilder::default() }
    pub fn source(&self) -> &str { self.source.as_ref() }
    pub fn worldName(&self) -> Option<&str> { self.worldName.as_deref() }
    pub fn includeCommandLineAPI(&self) -> Option<bool> { self.includeCommandLineAPI }
    pub fn runImmediately(&self) -> Option<bool> { self.runImmediately }
}

#[derive(Default)]
pub struct AddScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
    source: Option<Cow<'a, str>>,
    worldName: Option<Cow<'a, str>>,
    includeCommandLineAPI: Option<bool>,
    runImmediately: Option<bool>,
}

impl<'a> AddScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
    pub fn source(mut self, source: impl Into<Cow<'a, str>>) -> Self { self.source = Some(source.into()); self }
    /// If specified, creates an isolated world with the given name and evaluates given script in it.
    /// This world name will be used as the ExecutionContextDescription::name when the corresponding
    /// event is emitted.
    pub fn worldName(mut self, worldName: impl Into<Cow<'a, str>>) -> Self { self.worldName = Some(worldName.into()); self }
    /// Specifies whether command line API should be available to the script, defaults
    /// to false.
    pub fn includeCommandLineAPI(mut self, includeCommandLineAPI: bool) -> Self { self.includeCommandLineAPI = Some(includeCommandLineAPI); self }
    /// If true, runs the script immediately on existing execution contexts or worlds.
    /// Default: false.
    pub fn runImmediately(mut self, runImmediately: bool) -> Self { self.runImmediately = Some(runImmediately); self }
    pub fn build(self) -> AddScriptToEvaluateOnNewDocumentParams<'a> {
        AddScriptToEvaluateOnNewDocumentParams {
            source: self.source.unwrap_or_default(),
            worldName: self.worldName,
            includeCommandLineAPI: self.includeCommandLineAPI,
            runImmediately: self.runImmediately,
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
    pub fn builder() -> AddScriptToEvaluateOnNewDocumentReturnsBuilder<'a> { AddScriptToEvaluateOnNewDocumentReturnsBuilder::default() }
    pub fn identifier(&self) -> &ScriptIdentifier<'a> { &self.identifier }
}

#[derive(Default)]
pub struct AddScriptToEvaluateOnNewDocumentReturnsBuilder<'a> {
    identifier: Option<ScriptIdentifier<'a>>,
}

impl<'a> AddScriptToEvaluateOnNewDocumentReturnsBuilder<'a> {
    /// Identifier of the added script.
    pub fn identifier(mut self, identifier: ScriptIdentifier<'a>) -> Self { self.identifier = Some(identifier); self }
    pub fn build(self) -> AddScriptToEvaluateOnNewDocumentReturns<'a> {
        AddScriptToEvaluateOnNewDocumentReturns {
            identifier: self.identifier.unwrap_or_default(),
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

impl BringToFrontParams {
    pub fn builder() -> BringToFrontParamsBuilder {
        BringToFrontParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct BringToFrontParamsBuilder {}

impl BringToFrontParamsBuilder {
    pub fn build(self) -> BringToFrontParams {
        BringToFrontParams {}
    }
}

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
    /// Compression quality from range [0..100] (jpeg only).
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<i64>,
    /// Capture the screenshot of a given region only.
    #[serde(skip_serializing_if = "Option::is_none")]
    clip: Option<Viewport>,
    /// Capture the screenshot from the surface, rather than the view. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    fromSurface: Option<bool>,
    /// Capture the screenshot beyond the viewport. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    captureBeyondViewport: Option<bool>,
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    #[serde(skip_serializing_if = "Option::is_none")]
    optimizeForSpeed: Option<bool>,
}

impl<'a> CaptureScreenshotParams<'a> {
    pub fn builder() -> CaptureScreenshotParamsBuilder<'a> { CaptureScreenshotParamsBuilder::default() }
    pub fn format(&self) -> Option<&str> { self.format.as_deref() }
    pub fn quality(&self) -> Option<i64> { self.quality }
    pub fn clip(&self) -> Option<&Viewport> { self.clip.as_ref() }
    pub fn fromSurface(&self) -> Option<bool> { self.fromSurface }
    pub fn captureBeyondViewport(&self) -> Option<bool> { self.captureBeyondViewport }
    pub fn optimizeForSpeed(&self) -> Option<bool> { self.optimizeForSpeed }
}

#[derive(Default)]
pub struct CaptureScreenshotParamsBuilder<'a> {
    format: Option<Cow<'a, str>>,
    quality: Option<i64>,
    clip: Option<Viewport>,
    fromSurface: Option<bool>,
    captureBeyondViewport: Option<bool>,
    optimizeForSpeed: Option<bool>,
}

impl<'a> CaptureScreenshotParamsBuilder<'a> {
    /// Image compression format (defaults to png).
    pub fn format(mut self, format: impl Into<Cow<'a, str>>) -> Self { self.format = Some(format.into()); self }
    /// Compression quality from range [0..100] (jpeg only).
    pub fn quality(mut self, quality: i64) -> Self { self.quality = Some(quality); self }
    /// Capture the screenshot of a given region only.
    pub fn clip(mut self, clip: Viewport) -> Self { self.clip = Some(clip); self }
    /// Capture the screenshot from the surface, rather than the view. Defaults to true.
    pub fn fromSurface(mut self, fromSurface: bool) -> Self { self.fromSurface = Some(fromSurface); self }
    /// Capture the screenshot beyond the viewport. Defaults to false.
    pub fn captureBeyondViewport(mut self, captureBeyondViewport: bool) -> Self { self.captureBeyondViewport = Some(captureBeyondViewport); self }
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    pub fn optimizeForSpeed(mut self, optimizeForSpeed: bool) -> Self { self.optimizeForSpeed = Some(optimizeForSpeed); self }
    pub fn build(self) -> CaptureScreenshotParams<'a> {
        CaptureScreenshotParams {
            format: self.format,
            quality: self.quality,
            clip: self.clip,
            fromSurface: self.fromSurface,
            captureBeyondViewport: self.captureBeyondViewport,
            optimizeForSpeed: self.optimizeForSpeed,
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
    pub fn builder() -> CaptureScreenshotReturnsBuilder<'a> { CaptureScreenshotReturnsBuilder::default() }
    pub fn data(&self) -> &str { self.data.as_ref() }
}

#[derive(Default)]
pub struct CaptureScreenshotReturnsBuilder<'a> {
    data: Option<Cow<'a, str>>,
}

impl<'a> CaptureScreenshotReturnsBuilder<'a> {
    /// Base64-encoded image data. (Encoded as a base64 string when passed over JSON)
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> CaptureScreenshotReturns<'a> {
        CaptureScreenshotReturns {
            data: self.data.unwrap_or_default(),
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
    pub fn builder() -> CaptureSnapshotParamsBuilder<'a> { CaptureSnapshotParamsBuilder::default() }
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
    pub fn builder() -> CaptureSnapshotReturnsBuilder<'a> { CaptureSnapshotReturnsBuilder::default() }
    pub fn data(&self) -> &str { self.data.as_ref() }
}

#[derive(Default)]
pub struct CaptureSnapshotReturnsBuilder<'a> {
    data: Option<Cow<'a, str>>,
}

impl<'a> CaptureSnapshotReturnsBuilder<'a> {
    /// Serialized page data.
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> CaptureSnapshotReturns<'a> {
        CaptureSnapshotReturns {
            data: self.data.unwrap_or_default(),
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

impl ClearDeviceMetricsOverrideParams {
    pub fn builder() -> ClearDeviceMetricsOverrideParamsBuilder {
        ClearDeviceMetricsOverrideParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ClearDeviceMetricsOverrideParamsBuilder {}

impl ClearDeviceMetricsOverrideParamsBuilder {
    pub fn build(self) -> ClearDeviceMetricsOverrideParams {
        ClearDeviceMetricsOverrideParams {}
    }
}

impl ClearDeviceMetricsOverrideParams { pub const METHOD: &'static str = "Page.clearDeviceMetricsOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearDeviceMetricsOverrideParams {
    const METHOD: &'static str = "Page.clearDeviceMetricsOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearDeviceOrientationOverrideParams {}

impl ClearDeviceOrientationOverrideParams {
    pub fn builder() -> ClearDeviceOrientationOverrideParamsBuilder {
        ClearDeviceOrientationOverrideParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ClearDeviceOrientationOverrideParamsBuilder {}

impl ClearDeviceOrientationOverrideParamsBuilder {
    pub fn build(self) -> ClearDeviceOrientationOverrideParams {
        ClearDeviceOrientationOverrideParams {}
    }
}

impl ClearDeviceOrientationOverrideParams { pub const METHOD: &'static str = "Page.clearDeviceOrientationOverride"; }

impl<'a> crate::CdpCommand<'a> for ClearDeviceOrientationOverrideParams {
    const METHOD: &'static str = "Page.clearDeviceOrientationOverride";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearGeolocationOverrideParams {}

impl ClearGeolocationOverrideParams {
    pub fn builder() -> ClearGeolocationOverrideParamsBuilder {
        ClearGeolocationOverrideParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ClearGeolocationOverrideParamsBuilder {}

impl ClearGeolocationOverrideParamsBuilder {
    pub fn build(self) -> ClearGeolocationOverrideParams {
        ClearGeolocationOverrideParams {}
    }
}

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
    frameId: FrameId<'a>,
    /// An optional name which is reported in the Execution Context.
    #[serde(skip_serializing_if = "Option::is_none")]
    worldName: Option<Cow<'a, str>>,
    /// Whether or not universal access should be granted to the isolated world. This is a powerful
    /// option, use with caution.
    #[serde(skip_serializing_if = "Option::is_none")]
    grantUniveralAccess: Option<bool>,
}

impl<'a> CreateIsolatedWorldParams<'a> {
    pub fn builder() -> CreateIsolatedWorldParamsBuilder<'a> { CreateIsolatedWorldParamsBuilder::default() }
    pub fn frameId(&self) -> &FrameId<'a> { &self.frameId }
    pub fn worldName(&self) -> Option<&str> { self.worldName.as_deref() }
    pub fn grantUniveralAccess(&self) -> Option<bool> { self.grantUniveralAccess }
}

#[derive(Default)]
pub struct CreateIsolatedWorldParamsBuilder<'a> {
    frameId: Option<FrameId<'a>>,
    worldName: Option<Cow<'a, str>>,
    grantUniveralAccess: Option<bool>,
}

impl<'a> CreateIsolatedWorldParamsBuilder<'a> {
    /// Id of the frame in which the isolated world should be created.
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// An optional name which is reported in the Execution Context.
    pub fn worldName(mut self, worldName: impl Into<Cow<'a, str>>) -> Self { self.worldName = Some(worldName.into()); self }
    /// Whether or not universal access should be granted to the isolated world. This is a powerful
    /// option, use with caution.
    pub fn grantUniveralAccess(mut self, grantUniveralAccess: bool) -> Self { self.grantUniveralAccess = Some(grantUniveralAccess); self }
    pub fn build(self) -> CreateIsolatedWorldParams<'a> {
        CreateIsolatedWorldParams {
            frameId: self.frameId.unwrap_or_default(),
            worldName: self.worldName,
            grantUniveralAccess: self.grantUniveralAccess,
        }
    }
}

/// Creates an isolated world for the given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateIsolatedWorldReturns {
    /// Execution context of the isolated world.
    executionContextId: crate::runtime::ExecutionContextId,
}

impl CreateIsolatedWorldReturns {
    pub fn builder() -> CreateIsolatedWorldReturnsBuilder { CreateIsolatedWorldReturnsBuilder::default() }
    pub fn executionContextId(&self) -> &crate::runtime::ExecutionContextId { &self.executionContextId }
}

#[derive(Default)]
pub struct CreateIsolatedWorldReturnsBuilder {
    executionContextId: Option<crate::runtime::ExecutionContextId>,
}

impl CreateIsolatedWorldReturnsBuilder {
    /// Execution context of the isolated world.
    pub fn executionContextId(mut self, executionContextId: crate::runtime::ExecutionContextId) -> Self { self.executionContextId = Some(executionContextId); self }
    pub fn build(self) -> CreateIsolatedWorldReturns {
        CreateIsolatedWorldReturns {
            executionContextId: self.executionContextId.unwrap_or_default(),
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
    cookieName: Cow<'a, str>,
    /// URL to match cooke domain and path.
    url: Cow<'a, str>,
}

impl<'a> DeleteCookieParams<'a> {
    pub fn builder() -> DeleteCookieParamsBuilder<'a> { DeleteCookieParamsBuilder::default() }
    pub fn cookieName(&self) -> &str { self.cookieName.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
}

#[derive(Default)]
pub struct DeleteCookieParamsBuilder<'a> {
    cookieName: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
}

impl<'a> DeleteCookieParamsBuilder<'a> {
    /// Name of the cookie to remove.
    pub fn cookieName(mut self, cookieName: impl Into<Cow<'a, str>>) -> Self { self.cookieName = Some(cookieName.into()); self }
    /// URL to match cooke domain and path.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> DeleteCookieParams<'a> {
        DeleteCookieParams {
            cookieName: self.cookieName.unwrap_or_default(),
            url: self.url.unwrap_or_default(),
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

impl DisableParams {
    pub fn builder() -> DisableParamsBuilder {
        DisableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct DisableParamsBuilder {}

impl DisableParamsBuilder {
    pub fn build(self) -> DisableParams {
        DisableParams {}
    }
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    enableFileChooserOpenedEvent: Option<bool>,
}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder { EnableParamsBuilder::default() }
    pub fn enableFileChooserOpenedEvent(&self) -> Option<bool> { self.enableFileChooserOpenedEvent }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    enableFileChooserOpenedEvent: Option<bool>,
}

impl EnableParamsBuilder {
    /// If true, the 'Page.fileChooserOpened' event will be emitted regardless of the state set by
    /// 'Page.setInterceptFileChooserDialog' command (default: false).
    pub fn enableFileChooserOpenedEvent(mut self, enableFileChooserOpenedEvent: bool) -> Self { self.enableFileChooserOpenedEvent = Some(enableFileChooserOpenedEvent); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            enableFileChooserOpenedEvent: self.enableFileChooserOpenedEvent,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    manifestId: Option<Cow<'a, str>>,
}

impl<'a> GetAppManifestParams<'a> {
    pub fn builder() -> GetAppManifestParamsBuilder<'a> { GetAppManifestParamsBuilder::default() }
    pub fn manifestId(&self) -> Option<&str> { self.manifestId.as_deref() }
}

#[derive(Default)]
pub struct GetAppManifestParamsBuilder<'a> {
    manifestId: Option<Cow<'a, str>>,
}

impl<'a> GetAppManifestParamsBuilder<'a> {
    pub fn manifestId(mut self, manifestId: impl Into<Cow<'a, str>>) -> Self { self.manifestId = Some(manifestId.into()); self }
    pub fn build(self) -> GetAppManifestParams<'a> {
        GetAppManifestParams {
            manifestId: self.manifestId,
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
    pub fn builder() -> GetAppManifestReturnsBuilder<'a> { GetAppManifestReturnsBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn errors(&self) -> &[AppManifestError<'a>] { &self.errors }
    pub fn data(&self) -> Option<&str> { self.data.as_deref() }
    pub fn parsed(&self) -> Option<&AppManifestParsedProperties<'a>> { self.parsed.as_ref() }
    pub fn manifest(&self) -> &WebAppManifest<'a> { &self.manifest }
}

#[derive(Default)]
pub struct GetAppManifestReturnsBuilder<'a> {
    url: Option<Cow<'a, str>>,
    errors: Option<Vec<AppManifestError<'a>>>,
    data: Option<Cow<'a, str>>,
    parsed: Option<AppManifestParsedProperties<'a>>,
    manifest: Option<WebAppManifest<'a>>,
}

impl<'a> GetAppManifestReturnsBuilder<'a> {
    /// Manifest location.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn errors(mut self, errors: Vec<AppManifestError<'a>>) -> Self { self.errors = Some(errors); self }
    /// Manifest content.
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    /// Parsed manifest properties. Deprecated, use manifest instead.
    pub fn parsed(mut self, parsed: AppManifestParsedProperties<'a>) -> Self { self.parsed = Some(parsed); self }
    pub fn manifest(mut self, manifest: WebAppManifest<'a>) -> Self { self.manifest = Some(manifest); self }
    pub fn build(self) -> GetAppManifestReturns<'a> {
        GetAppManifestReturns {
            url: self.url.unwrap_or_default(),
            errors: self.errors.unwrap_or_default(),
            data: self.data,
            parsed: self.parsed,
            manifest: self.manifest.unwrap_or_default(),
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
    installabilityErrors: Vec<InstallabilityError<'a>>,
}

impl<'a> GetInstallabilityErrorsReturns<'a> {
    pub fn builder() -> GetInstallabilityErrorsReturnsBuilder<'a> { GetInstallabilityErrorsReturnsBuilder::default() }
    pub fn installabilityErrors(&self) -> &[InstallabilityError<'a>] { &self.installabilityErrors }
}

#[derive(Default)]
pub struct GetInstallabilityErrorsReturnsBuilder<'a> {
    installabilityErrors: Option<Vec<InstallabilityError<'a>>>,
}

impl<'a> GetInstallabilityErrorsReturnsBuilder<'a> {
    pub fn installabilityErrors(mut self, installabilityErrors: Vec<InstallabilityError<'a>>) -> Self { self.installabilityErrors = Some(installabilityErrors); self }
    pub fn build(self) -> GetInstallabilityErrorsReturns<'a> {
        GetInstallabilityErrorsReturns {
            installabilityErrors: self.installabilityErrors.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetInstallabilityErrorsParams {}

impl GetInstallabilityErrorsParams {
    pub fn builder() -> GetInstallabilityErrorsParamsBuilder {
        GetInstallabilityErrorsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetInstallabilityErrorsParamsBuilder {}

impl GetInstallabilityErrorsParamsBuilder {
    pub fn build(self) -> GetInstallabilityErrorsParams {
        GetInstallabilityErrorsParams {}
    }
}

impl GetInstallabilityErrorsParams { pub const METHOD: &'static str = "Page.getInstallabilityErrors"; }

impl<'a> crate::CdpCommand<'a> for GetInstallabilityErrorsParams {
    const METHOD: &'static str = "Page.getInstallabilityErrors";
    type Response = GetInstallabilityErrorsReturns<'a>;
}

/// Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetManifestIconsReturns<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    primaryIcon: Option<Cow<'a, str>>,
}

impl<'a> GetManifestIconsReturns<'a> {
    pub fn builder() -> GetManifestIconsReturnsBuilder<'a> { GetManifestIconsReturnsBuilder::default() }
    pub fn primaryIcon(&self) -> Option<&str> { self.primaryIcon.as_deref() }
}

#[derive(Default)]
pub struct GetManifestIconsReturnsBuilder<'a> {
    primaryIcon: Option<Cow<'a, str>>,
}

impl<'a> GetManifestIconsReturnsBuilder<'a> {
    pub fn primaryIcon(mut self, primaryIcon: impl Into<Cow<'a, str>>) -> Self { self.primaryIcon = Some(primaryIcon.into()); self }
    pub fn build(self) -> GetManifestIconsReturns<'a> {
        GetManifestIconsReturns {
            primaryIcon: self.primaryIcon,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetManifestIconsParams {}

impl GetManifestIconsParams {
    pub fn builder() -> GetManifestIconsParamsBuilder {
        GetManifestIconsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetManifestIconsParamsBuilder {}

impl GetManifestIconsParamsBuilder {
    pub fn build(self) -> GetManifestIconsParams {
        GetManifestIconsParams {}
    }
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    appId: Option<Cow<'a, str>>,
    /// Recommendation for manifest's id attribute to match current id computed from start_url
    #[serde(skip_serializing_if = "Option::is_none")]
    recommendedId: Option<Cow<'a, str>>,
}

impl<'a> GetAppIdReturns<'a> {
    pub fn builder() -> GetAppIdReturnsBuilder<'a> { GetAppIdReturnsBuilder::default() }
    pub fn appId(&self) -> Option<&str> { self.appId.as_deref() }
    pub fn recommendedId(&self) -> Option<&str> { self.recommendedId.as_deref() }
}

#[derive(Default)]
pub struct GetAppIdReturnsBuilder<'a> {
    appId: Option<Cow<'a, str>>,
    recommendedId: Option<Cow<'a, str>>,
}

impl<'a> GetAppIdReturnsBuilder<'a> {
    /// App id, either from manifest's id attribute or computed from start_url
    pub fn appId(mut self, appId: impl Into<Cow<'a, str>>) -> Self { self.appId = Some(appId.into()); self }
    /// Recommendation for manifest's id attribute to match current id computed from start_url
    pub fn recommendedId(mut self, recommendedId: impl Into<Cow<'a, str>>) -> Self { self.recommendedId = Some(recommendedId.into()); self }
    pub fn build(self) -> GetAppIdReturns<'a> {
        GetAppIdReturns {
            appId: self.appId,
            recommendedId: self.recommendedId,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAppIdParams {}

impl GetAppIdParams {
    pub fn builder() -> GetAppIdParamsBuilder {
        GetAppIdParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetAppIdParamsBuilder {}

impl GetAppIdParamsBuilder {
    pub fn build(self) -> GetAppIdParams {
        GetAppIdParams {}
    }
}

impl GetAppIdParams { pub const METHOD: &'static str = "Page.getAppId"; }

impl<'a> crate::CdpCommand<'a> for GetAppIdParams {
    const METHOD: &'static str = "Page.getAppId";
    type Response = GetAppIdReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAdScriptAncestryParams<'a> {
    frameId: FrameId<'a>,
}

impl<'a> GetAdScriptAncestryParams<'a> {
    pub fn builder() -> GetAdScriptAncestryParamsBuilder<'a> { GetAdScriptAncestryParamsBuilder::default() }
    pub fn frameId(&self) -> &FrameId<'a> { &self.frameId }
}

#[derive(Default)]
pub struct GetAdScriptAncestryParamsBuilder<'a> {
    frameId: Option<FrameId<'a>>,
}

impl<'a> GetAdScriptAncestryParamsBuilder<'a> {
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetAdScriptAncestryParams<'a> {
        GetAdScriptAncestryParams {
            frameId: self.frameId.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    adScriptAncestry: Option<crate::network::AdAncestry<'a>>,
}

impl<'a> GetAdScriptAncestryReturns<'a> {
    pub fn builder() -> GetAdScriptAncestryReturnsBuilder<'a> { GetAdScriptAncestryReturnsBuilder::default() }
    pub fn adScriptAncestry(&self) -> Option<&crate::network::AdAncestry<'a>> { self.adScriptAncestry.as_ref() }
}

#[derive(Default)]
pub struct GetAdScriptAncestryReturnsBuilder<'a> {
    adScriptAncestry: Option<crate::network::AdAncestry<'a>>,
}

impl<'a> GetAdScriptAncestryReturnsBuilder<'a> {
    /// The ancestry chain of ad script identifiers leading to this frame's
    /// creation, along with the root script's filterlist rule. The ancestry
    /// chain is ordered from the most immediate script (in the frame creation
    /// stack) to more distant ancestors (that created the immediately preceding
    /// script). Only sent if frame is labelled as an ad and ids are available.
    pub fn adScriptAncestry(mut self, adScriptAncestry: crate::network::AdAncestry<'a>) -> Self { self.adScriptAncestry = Some(adScriptAncestry); self }
    pub fn build(self) -> GetAdScriptAncestryReturns<'a> {
        GetAdScriptAncestryReturns {
            adScriptAncestry: self.adScriptAncestry,
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
    frameTree: FrameTree<'a>,
}

impl<'a> GetFrameTreeReturns<'a> {
    pub fn builder() -> GetFrameTreeReturnsBuilder<'a> { GetFrameTreeReturnsBuilder::default() }
    pub fn frameTree(&self) -> &FrameTree<'a> { &self.frameTree }
}

#[derive(Default)]
pub struct GetFrameTreeReturnsBuilder<'a> {
    frameTree: Option<FrameTree<'a>>,
}

impl<'a> GetFrameTreeReturnsBuilder<'a> {
    /// Present frame tree structure.
    pub fn frameTree(mut self, frameTree: FrameTree<'a>) -> Self { self.frameTree = Some(frameTree); self }
    pub fn build(self) -> GetFrameTreeReturns<'a> {
        GetFrameTreeReturns {
            frameTree: self.frameTree.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetFrameTreeParams {}

impl GetFrameTreeParams {
    pub fn builder() -> GetFrameTreeParamsBuilder {
        GetFrameTreeParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetFrameTreeParamsBuilder {}

impl GetFrameTreeParamsBuilder {
    pub fn build(self) -> GetFrameTreeParams {
        GetFrameTreeParams {}
    }
}

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
    layoutViewport: LayoutViewport,
    /// Deprecated metrics relating to the visual viewport. Is in device pixels. Use 'cssVisualViewport' instead.
    visualViewport: VisualViewport,
    /// Deprecated size of scrollable area. Is in DP. Use 'cssContentSize' instead.
    contentSize: crate::dom::Rect,
    /// Metrics relating to the layout viewport in CSS pixels.
    cssLayoutViewport: LayoutViewport,
    /// Metrics relating to the visual viewport in CSS pixels.
    cssVisualViewport: VisualViewport,
    /// Size of scrollable area in CSS pixels.
    cssContentSize: crate::dom::Rect,
}

impl GetLayoutMetricsReturns {
    pub fn builder() -> GetLayoutMetricsReturnsBuilder { GetLayoutMetricsReturnsBuilder::default() }
    pub fn layoutViewport(&self) -> &LayoutViewport { &self.layoutViewport }
    pub fn visualViewport(&self) -> &VisualViewport { &self.visualViewport }
    pub fn contentSize(&self) -> &crate::dom::Rect { &self.contentSize }
    pub fn cssLayoutViewport(&self) -> &LayoutViewport { &self.cssLayoutViewport }
    pub fn cssVisualViewport(&self) -> &VisualViewport { &self.cssVisualViewport }
    pub fn cssContentSize(&self) -> &crate::dom::Rect { &self.cssContentSize }
}

#[derive(Default)]
pub struct GetLayoutMetricsReturnsBuilder {
    layoutViewport: Option<LayoutViewport>,
    visualViewport: Option<VisualViewport>,
    contentSize: Option<crate::dom::Rect>,
    cssLayoutViewport: Option<LayoutViewport>,
    cssVisualViewport: Option<VisualViewport>,
    cssContentSize: Option<crate::dom::Rect>,
}

impl GetLayoutMetricsReturnsBuilder {
    /// Deprecated metrics relating to the layout viewport. Is in device pixels. Use 'cssLayoutViewport' instead.
    pub fn layoutViewport(mut self, layoutViewport: LayoutViewport) -> Self { self.layoutViewport = Some(layoutViewport); self }
    /// Deprecated metrics relating to the visual viewport. Is in device pixels. Use 'cssVisualViewport' instead.
    pub fn visualViewport(mut self, visualViewport: VisualViewport) -> Self { self.visualViewport = Some(visualViewport); self }
    /// Deprecated size of scrollable area. Is in DP. Use 'cssContentSize' instead.
    pub fn contentSize(mut self, contentSize: crate::dom::Rect) -> Self { self.contentSize = Some(contentSize); self }
    /// Metrics relating to the layout viewport in CSS pixels.
    pub fn cssLayoutViewport(mut self, cssLayoutViewport: LayoutViewport) -> Self { self.cssLayoutViewport = Some(cssLayoutViewport); self }
    /// Metrics relating to the visual viewport in CSS pixels.
    pub fn cssVisualViewport(mut self, cssVisualViewport: VisualViewport) -> Self { self.cssVisualViewport = Some(cssVisualViewport); self }
    /// Size of scrollable area in CSS pixels.
    pub fn cssContentSize(mut self, cssContentSize: crate::dom::Rect) -> Self { self.cssContentSize = Some(cssContentSize); self }
    pub fn build(self) -> GetLayoutMetricsReturns {
        GetLayoutMetricsReturns {
            layoutViewport: self.layoutViewport.unwrap_or_default(),
            visualViewport: self.visualViewport.unwrap_or_default(),
            contentSize: self.contentSize.unwrap_or_default(),
            cssLayoutViewport: self.cssLayoutViewport.unwrap_or_default(),
            cssVisualViewport: self.cssVisualViewport.unwrap_or_default(),
            cssContentSize: self.cssContentSize.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetLayoutMetricsParams {}

impl GetLayoutMetricsParams {
    pub fn builder() -> GetLayoutMetricsParamsBuilder {
        GetLayoutMetricsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetLayoutMetricsParamsBuilder {}

impl GetLayoutMetricsParamsBuilder {
    pub fn build(self) -> GetLayoutMetricsParams {
        GetLayoutMetricsParams {}
    }
}

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
    currentIndex: u64,
    /// Array of navigation history entries.
    entries: Vec<NavigationEntry<'a>>,
}

impl<'a> GetNavigationHistoryReturns<'a> {
    pub fn builder() -> GetNavigationHistoryReturnsBuilder<'a> { GetNavigationHistoryReturnsBuilder::default() }
    pub fn currentIndex(&self) -> u64 { self.currentIndex }
    pub fn entries(&self) -> &[NavigationEntry<'a>] { &self.entries }
}

#[derive(Default)]
pub struct GetNavigationHistoryReturnsBuilder<'a> {
    currentIndex: Option<u64>,
    entries: Option<Vec<NavigationEntry<'a>>>,
}

impl<'a> GetNavigationHistoryReturnsBuilder<'a> {
    /// Index of the current navigation history entry.
    pub fn currentIndex(mut self, currentIndex: u64) -> Self { self.currentIndex = Some(currentIndex); self }
    /// Array of navigation history entries.
    pub fn entries(mut self, entries: Vec<NavigationEntry<'a>>) -> Self { self.entries = Some(entries); self }
    pub fn build(self) -> GetNavigationHistoryReturns<'a> {
        GetNavigationHistoryReturns {
            currentIndex: self.currentIndex.unwrap_or_default(),
            entries: self.entries.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetNavigationHistoryParams {}

impl GetNavigationHistoryParams {
    pub fn builder() -> GetNavigationHistoryParamsBuilder {
        GetNavigationHistoryParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetNavigationHistoryParamsBuilder {}

impl GetNavigationHistoryParamsBuilder {
    pub fn build(self) -> GetNavigationHistoryParams {
        GetNavigationHistoryParams {}
    }
}

impl GetNavigationHistoryParams { pub const METHOD: &'static str = "Page.getNavigationHistory"; }

impl<'a> crate::CdpCommand<'a> for GetNavigationHistoryParams {
    const METHOD: &'static str = "Page.getNavigationHistory";
    type Response = GetNavigationHistoryReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResetNavigationHistoryParams {}

impl ResetNavigationHistoryParams {
    pub fn builder() -> ResetNavigationHistoryParamsBuilder {
        ResetNavigationHistoryParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ResetNavigationHistoryParamsBuilder {}

impl ResetNavigationHistoryParamsBuilder {
    pub fn build(self) -> ResetNavigationHistoryParams {
        ResetNavigationHistoryParams {}
    }
}

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
    frameId: FrameId<'a>,
    /// URL of the resource to get content for.
    url: Cow<'a, str>,
}

impl<'a> GetResourceContentParams<'a> {
    pub fn builder() -> GetResourceContentParamsBuilder<'a> { GetResourceContentParamsBuilder::default() }
    pub fn frameId(&self) -> &FrameId<'a> { &self.frameId }
    pub fn url(&self) -> &str { self.url.as_ref() }
}

#[derive(Default)]
pub struct GetResourceContentParamsBuilder<'a> {
    frameId: Option<FrameId<'a>>,
    url: Option<Cow<'a, str>>,
}

impl<'a> GetResourceContentParamsBuilder<'a> {
    /// Frame id to get resource for.
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// URL of the resource to get content for.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn build(self) -> GetResourceContentParams<'a> {
        GetResourceContentParams {
            frameId: self.frameId.unwrap_or_default(),
            url: self.url.unwrap_or_default(),
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
    base64Encoded: bool,
}

impl<'a> GetResourceContentReturns<'a> {
    pub fn builder() -> GetResourceContentReturnsBuilder<'a> { GetResourceContentReturnsBuilder::default() }
    pub fn content(&self) -> &str { self.content.as_ref() }
    pub fn base64Encoded(&self) -> bool { self.base64Encoded }
}

#[derive(Default)]
pub struct GetResourceContentReturnsBuilder<'a> {
    content: Option<Cow<'a, str>>,
    base64Encoded: Option<bool>,
}

impl<'a> GetResourceContentReturnsBuilder<'a> {
    /// Resource content.
    pub fn content(mut self, content: impl Into<Cow<'a, str>>) -> Self { self.content = Some(content.into()); self }
    /// True, if content was served as base64.
    pub fn base64Encoded(mut self, base64Encoded: bool) -> Self { self.base64Encoded = Some(base64Encoded); self }
    pub fn build(self) -> GetResourceContentReturns<'a> {
        GetResourceContentReturns {
            content: self.content.unwrap_or_default(),
            base64Encoded: self.base64Encoded.unwrap_or_default(),
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
    frameTree: FrameResourceTree<'a>,
}

impl<'a> GetResourceTreeReturns<'a> {
    pub fn builder() -> GetResourceTreeReturnsBuilder<'a> { GetResourceTreeReturnsBuilder::default() }
    pub fn frameTree(&self) -> &FrameResourceTree<'a> { &self.frameTree }
}

#[derive(Default)]
pub struct GetResourceTreeReturnsBuilder<'a> {
    frameTree: Option<FrameResourceTree<'a>>,
}

impl<'a> GetResourceTreeReturnsBuilder<'a> {
    /// Present frame / resource tree structure.
    pub fn frameTree(mut self, frameTree: FrameResourceTree<'a>) -> Self { self.frameTree = Some(frameTree); self }
    pub fn build(self) -> GetResourceTreeReturns<'a> {
        GetResourceTreeReturns {
            frameTree: self.frameTree.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetResourceTreeParams {}

impl GetResourceTreeParams {
    pub fn builder() -> GetResourceTreeParamsBuilder {
        GetResourceTreeParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetResourceTreeParamsBuilder {}

impl GetResourceTreeParamsBuilder {
    pub fn build(self) -> GetResourceTreeParams {
        GetResourceTreeParams {}
    }
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    promptText: Option<Cow<'a, str>>,
}

impl<'a> HandleJavaScriptDialogParams<'a> {
    pub fn builder() -> HandleJavaScriptDialogParamsBuilder<'a> { HandleJavaScriptDialogParamsBuilder::default() }
    pub fn accept(&self) -> bool { self.accept }
    pub fn promptText(&self) -> Option<&str> { self.promptText.as_deref() }
}

#[derive(Default)]
pub struct HandleJavaScriptDialogParamsBuilder<'a> {
    accept: Option<bool>,
    promptText: Option<Cow<'a, str>>,
}

impl<'a> HandleJavaScriptDialogParamsBuilder<'a> {
    /// Whether to accept or dismiss the dialog.
    pub fn accept(mut self, accept: bool) -> Self { self.accept = Some(accept); self }
    /// The text to enter into the dialog prompt before accepting. Used only if this is a prompt
    /// dialog.
    pub fn promptText(mut self, promptText: impl Into<Cow<'a, str>>) -> Self { self.promptText = Some(promptText.into()); self }
    pub fn build(self) -> HandleJavaScriptDialogParams<'a> {
        HandleJavaScriptDialogParams {
            accept: self.accept.unwrap_or_default(),
            promptText: self.promptText,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    transitionType: Option<TransitionType>,
    /// Frame id to navigate, if not specified navigates the top frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<FrameId<'a>>,
    /// Referrer-policy used for the navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    referrerPolicy: Option<ReferrerPolicy>,
}

impl<'a> NavigateParams<'a> {
    pub fn builder() -> NavigateParamsBuilder<'a> { NavigateParamsBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn referrer(&self) -> Option<&str> { self.referrer.as_deref() }
    pub fn transitionType(&self) -> Option<&TransitionType> { self.transitionType.as_ref() }
    pub fn frameId(&self) -> Option<&FrameId<'a>> { self.frameId.as_ref() }
    pub fn referrerPolicy(&self) -> Option<&ReferrerPolicy> { self.referrerPolicy.as_ref() }
}

#[derive(Default)]
pub struct NavigateParamsBuilder<'a> {
    url: Option<Cow<'a, str>>,
    referrer: Option<Cow<'a, str>>,
    transitionType: Option<TransitionType>,
    frameId: Option<FrameId<'a>>,
    referrerPolicy: Option<ReferrerPolicy>,
}

impl<'a> NavigateParamsBuilder<'a> {
    /// URL to navigate the page to.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Referrer URL.
    pub fn referrer(mut self, referrer: impl Into<Cow<'a, str>>) -> Self { self.referrer = Some(referrer.into()); self }
    /// Intended transition type.
    pub fn transitionType(mut self, transitionType: TransitionType) -> Self { self.transitionType = Some(transitionType); self }
    /// Frame id to navigate, if not specified navigates the top frame.
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// Referrer-policy used for the navigation.
    pub fn referrerPolicy(mut self, referrerPolicy: ReferrerPolicy) -> Self { self.referrerPolicy = Some(referrerPolicy); self }
    pub fn build(self) -> NavigateParams<'a> {
        NavigateParams {
            url: self.url.unwrap_or_default(),
            referrer: self.referrer,
            transitionType: self.transitionType,
            frameId: self.frameId,
            referrerPolicy: self.referrerPolicy,
        }
    }
}

/// Navigates current page to the given URL.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigateReturns<'a> {
    /// Frame id that has navigated (or failed to navigate)
    frameId: FrameId<'a>,
    /// Loader identifier. This is omitted in case of same-document navigation,
    /// as the previously committed loaderId would not change.
    #[serde(skip_serializing_if = "Option::is_none")]
    loaderId: Option<crate::network::LoaderId<'a>>,
    /// User friendly error message, present if and only if navigation has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    errorText: Option<Cow<'a, str>>,
    /// Whether the navigation resulted in a download.
    #[serde(skip_serializing_if = "Option::is_none")]
    isDownload: Option<bool>,
}

impl<'a> NavigateReturns<'a> {
    pub fn builder() -> NavigateReturnsBuilder<'a> { NavigateReturnsBuilder::default() }
    pub fn frameId(&self) -> &FrameId<'a> { &self.frameId }
    pub fn loaderId(&self) -> Option<&crate::network::LoaderId<'a>> { self.loaderId.as_ref() }
    pub fn errorText(&self) -> Option<&str> { self.errorText.as_deref() }
    pub fn isDownload(&self) -> Option<bool> { self.isDownload }
}

#[derive(Default)]
pub struct NavigateReturnsBuilder<'a> {
    frameId: Option<FrameId<'a>>,
    loaderId: Option<crate::network::LoaderId<'a>>,
    errorText: Option<Cow<'a, str>>,
    isDownload: Option<bool>,
}

impl<'a> NavigateReturnsBuilder<'a> {
    /// Frame id that has navigated (or failed to navigate)
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// Loader identifier. This is omitted in case of same-document navigation,
    /// as the previously committed loaderId would not change.
    pub fn loaderId(mut self, loaderId: crate::network::LoaderId<'a>) -> Self { self.loaderId = Some(loaderId); self }
    /// User friendly error message, present if and only if navigation has failed.
    pub fn errorText(mut self, errorText: impl Into<Cow<'a, str>>) -> Self { self.errorText = Some(errorText.into()); self }
    /// Whether the navigation resulted in a download.
    pub fn isDownload(mut self, isDownload: bool) -> Self { self.isDownload = Some(isDownload); self }
    pub fn build(self) -> NavigateReturns<'a> {
        NavigateReturns {
            frameId: self.frameId.unwrap_or_default(),
            loaderId: self.loaderId,
            errorText: self.errorText,
            isDownload: self.isDownload,
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
    entryId: u64,
}

impl NavigateToHistoryEntryParams {
    pub fn builder() -> NavigateToHistoryEntryParamsBuilder { NavigateToHistoryEntryParamsBuilder::default() }
    pub fn entryId(&self) -> u64 { self.entryId }
}

#[derive(Default)]
pub struct NavigateToHistoryEntryParamsBuilder {
    entryId: Option<u64>,
}

impl NavigateToHistoryEntryParamsBuilder {
    /// Unique id of the entry to navigate to.
    pub fn entryId(mut self, entryId: u64) -> Self { self.entryId = Some(entryId); self }
    pub fn build(self) -> NavigateToHistoryEntryParams {
        NavigateToHistoryEntryParams {
            entryId: self.entryId.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    displayHeaderFooter: Option<bool>,
    /// Print background graphics. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    printBackground: Option<bool>,
    /// Scale of the webpage rendering. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f64>,
    /// Paper width in inches. Defaults to 8.5 inches.
    #[serde(skip_serializing_if = "Option::is_none")]
    paperWidth: Option<f64>,
    /// Paper height in inches. Defaults to 11 inches.
    #[serde(skip_serializing_if = "Option::is_none")]
    paperHeight: Option<f64>,
    /// Top margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none")]
    marginTop: Option<f64>,
    /// Bottom margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none")]
    marginBottom: Option<f64>,
    /// Left margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none")]
    marginLeft: Option<f64>,
    /// Right margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none")]
    marginRight: Option<f64>,
    /// Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are
    /// printed in the document order, not in the order specified, and no
    /// more than once.
    /// Defaults to empty string, which implies the entire document is printed.
    /// The page numbers are quietly capped to actual page count of the
    /// document, and ranges beyond the end of the document are ignored.
    /// If this results in no pages to print, an error is reported.
    /// It is an error to specify a range with start greater than end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pageRanges: Option<Cow<'a, str>>,
    /// HTML template for the print header. Should be valid HTML markup with following
    /// classes used to inject printing values into them:
    /// - 'date': formatted print date
    /// - 'title': document title
    /// - 'url': document location
    /// - 'pageNumber': current page number
    /// - 'totalPages': total pages in the document
    /// 
    /// For example, '<span class=title></span>' would generate span containing the title.
    #[serde(skip_serializing_if = "Option::is_none")]
    headerTemplate: Option<Cow<'a, str>>,
    /// HTML template for the print footer. Should use the same format as the 'headerTemplate'.
    #[serde(skip_serializing_if = "Option::is_none")]
    footerTemplate: Option<Cow<'a, str>>,
    /// Whether or not to prefer page size as defined by css. Defaults to false,
    /// in which case the content will be scaled to fit the paper size.
    #[serde(skip_serializing_if = "Option::is_none")]
    preferCSSPageSize: Option<bool>,
    /// return as stream
    #[serde(skip_serializing_if = "Option::is_none")]
    transferMode: Option<Cow<'a, str>>,
    /// Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    generateTaggedPDF: Option<bool>,
    /// Whether or not to embed the document outline into the PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    generateDocumentOutline: Option<bool>,
}

impl<'a> PrintToPDFParams<'a> {
    pub fn builder() -> PrintToPDFParamsBuilder<'a> { PrintToPDFParamsBuilder::default() }
    pub fn landscape(&self) -> Option<bool> { self.landscape }
    pub fn displayHeaderFooter(&self) -> Option<bool> { self.displayHeaderFooter }
    pub fn printBackground(&self) -> Option<bool> { self.printBackground }
    pub fn scale(&self) -> Option<f64> { self.scale }
    pub fn paperWidth(&self) -> Option<f64> { self.paperWidth }
    pub fn paperHeight(&self) -> Option<f64> { self.paperHeight }
    pub fn marginTop(&self) -> Option<f64> { self.marginTop }
    pub fn marginBottom(&self) -> Option<f64> { self.marginBottom }
    pub fn marginLeft(&self) -> Option<f64> { self.marginLeft }
    pub fn marginRight(&self) -> Option<f64> { self.marginRight }
    pub fn pageRanges(&self) -> Option<&str> { self.pageRanges.as_deref() }
    pub fn headerTemplate(&self) -> Option<&str> { self.headerTemplate.as_deref() }
    pub fn footerTemplate(&self) -> Option<&str> { self.footerTemplate.as_deref() }
    pub fn preferCSSPageSize(&self) -> Option<bool> { self.preferCSSPageSize }
    pub fn transferMode(&self) -> Option<&str> { self.transferMode.as_deref() }
    pub fn generateTaggedPDF(&self) -> Option<bool> { self.generateTaggedPDF }
    pub fn generateDocumentOutline(&self) -> Option<bool> { self.generateDocumentOutline }
}

#[derive(Default)]
pub struct PrintToPDFParamsBuilder<'a> {
    landscape: Option<bool>,
    displayHeaderFooter: Option<bool>,
    printBackground: Option<bool>,
    scale: Option<f64>,
    paperWidth: Option<f64>,
    paperHeight: Option<f64>,
    marginTop: Option<f64>,
    marginBottom: Option<f64>,
    marginLeft: Option<f64>,
    marginRight: Option<f64>,
    pageRanges: Option<Cow<'a, str>>,
    headerTemplate: Option<Cow<'a, str>>,
    footerTemplate: Option<Cow<'a, str>>,
    preferCSSPageSize: Option<bool>,
    transferMode: Option<Cow<'a, str>>,
    generateTaggedPDF: Option<bool>,
    generateDocumentOutline: Option<bool>,
}

impl<'a> PrintToPDFParamsBuilder<'a> {
    /// Paper orientation. Defaults to false.
    pub fn landscape(mut self, landscape: bool) -> Self { self.landscape = Some(landscape); self }
    /// Display header and footer. Defaults to false.
    pub fn displayHeaderFooter(mut self, displayHeaderFooter: bool) -> Self { self.displayHeaderFooter = Some(displayHeaderFooter); self }
    /// Print background graphics. Defaults to false.
    pub fn printBackground(mut self, printBackground: bool) -> Self { self.printBackground = Some(printBackground); self }
    /// Scale of the webpage rendering. Defaults to 1.
    pub fn scale(mut self, scale: f64) -> Self { self.scale = Some(scale); self }
    /// Paper width in inches. Defaults to 8.5 inches.
    pub fn paperWidth(mut self, paperWidth: f64) -> Self { self.paperWidth = Some(paperWidth); self }
    /// Paper height in inches. Defaults to 11 inches.
    pub fn paperHeight(mut self, paperHeight: f64) -> Self { self.paperHeight = Some(paperHeight); self }
    /// Top margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn marginTop(mut self, marginTop: f64) -> Self { self.marginTop = Some(marginTop); self }
    /// Bottom margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn marginBottom(mut self, marginBottom: f64) -> Self { self.marginBottom = Some(marginBottom); self }
    /// Left margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn marginLeft(mut self, marginLeft: f64) -> Self { self.marginLeft = Some(marginLeft); self }
    /// Right margin in inches. Defaults to 1cm (~0.4 inches).
    pub fn marginRight(mut self, marginRight: f64) -> Self { self.marginRight = Some(marginRight); self }
    /// Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are
    /// printed in the document order, not in the order specified, and no
    /// more than once.
    /// Defaults to empty string, which implies the entire document is printed.
    /// The page numbers are quietly capped to actual page count of the
    /// document, and ranges beyond the end of the document are ignored.
    /// If this results in no pages to print, an error is reported.
    /// It is an error to specify a range with start greater than end.
    pub fn pageRanges(mut self, pageRanges: impl Into<Cow<'a, str>>) -> Self { self.pageRanges = Some(pageRanges.into()); self }
    /// HTML template for the print header. Should be valid HTML markup with following
    /// classes used to inject printing values into them:
    /// - 'date': formatted print date
    /// - 'title': document title
    /// - 'url': document location
    /// - 'pageNumber': current page number
    /// - 'totalPages': total pages in the document
    /// 
    /// For example, '<span class=title></span>' would generate span containing the title.
    pub fn headerTemplate(mut self, headerTemplate: impl Into<Cow<'a, str>>) -> Self { self.headerTemplate = Some(headerTemplate.into()); self }
    /// HTML template for the print footer. Should use the same format as the 'headerTemplate'.
    pub fn footerTemplate(mut self, footerTemplate: impl Into<Cow<'a, str>>) -> Self { self.footerTemplate = Some(footerTemplate.into()); self }
    /// Whether or not to prefer page size as defined by css. Defaults to false,
    /// in which case the content will be scaled to fit the paper size.
    pub fn preferCSSPageSize(mut self, preferCSSPageSize: bool) -> Self { self.preferCSSPageSize = Some(preferCSSPageSize); self }
    /// return as stream
    pub fn transferMode(mut self, transferMode: impl Into<Cow<'a, str>>) -> Self { self.transferMode = Some(transferMode.into()); self }
    /// Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice.
    pub fn generateTaggedPDF(mut self, generateTaggedPDF: bool) -> Self { self.generateTaggedPDF = Some(generateTaggedPDF); self }
    /// Whether or not to embed the document outline into the PDF.
    pub fn generateDocumentOutline(mut self, generateDocumentOutline: bool) -> Self { self.generateDocumentOutline = Some(generateDocumentOutline); self }
    pub fn build(self) -> PrintToPDFParams<'a> {
        PrintToPDFParams {
            landscape: self.landscape,
            displayHeaderFooter: self.displayHeaderFooter,
            printBackground: self.printBackground,
            scale: self.scale,
            paperWidth: self.paperWidth,
            paperHeight: self.paperHeight,
            marginTop: self.marginTop,
            marginBottom: self.marginBottom,
            marginLeft: self.marginLeft,
            marginRight: self.marginRight,
            pageRanges: self.pageRanges,
            headerTemplate: self.headerTemplate,
            footerTemplate: self.footerTemplate,
            preferCSSPageSize: self.preferCSSPageSize,
            transferMode: self.transferMode,
            generateTaggedPDF: self.generateTaggedPDF,
            generateDocumentOutline: self.generateDocumentOutline,
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
    pub fn builder() -> PrintToPDFReturnsBuilder<'a> { PrintToPDFReturnsBuilder::default() }
    pub fn data(&self) -> &str { self.data.as_ref() }
    pub fn stream(&self) -> Option<&crate::io::StreamHandle<'a>> { self.stream.as_ref() }
}

#[derive(Default)]
pub struct PrintToPDFReturnsBuilder<'a> {
    data: Option<Cow<'a, str>>,
    stream: Option<crate::io::StreamHandle<'a>>,
}

impl<'a> PrintToPDFReturnsBuilder<'a> {
    /// Base64-encoded pdf data. Empty if |returnAsStream| is specified. (Encoded as a base64 string when passed over JSON)
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    /// A handle of the stream that holds resulting PDF data.
    pub fn stream(mut self, stream: crate::io::StreamHandle<'a>) -> Self { self.stream = Some(stream); self }
    pub fn build(self) -> PrintToPDFReturns<'a> {
        PrintToPDFReturns {
            data: self.data.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    ignoreCache: Option<bool>,
    /// If set, the script will be injected into all frames of the inspected page after reload.
    /// Argument will be ignored if reloading dataURL origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    scriptToEvaluateOnLoad: Option<Cow<'a, str>>,
    /// If set, an error will be thrown if the target page's main frame's
    /// loader id does not match the provided id. This prevents accidentally
    /// reloading an unintended target in case there's a racing navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    loaderId: Option<crate::network::LoaderId<'a>>,
}

impl<'a> ReloadParams<'a> {
    pub fn builder() -> ReloadParamsBuilder<'a> { ReloadParamsBuilder::default() }
    pub fn ignoreCache(&self) -> Option<bool> { self.ignoreCache }
    pub fn scriptToEvaluateOnLoad(&self) -> Option<&str> { self.scriptToEvaluateOnLoad.as_deref() }
    pub fn loaderId(&self) -> Option<&crate::network::LoaderId<'a>> { self.loaderId.as_ref() }
}

#[derive(Default)]
pub struct ReloadParamsBuilder<'a> {
    ignoreCache: Option<bool>,
    scriptToEvaluateOnLoad: Option<Cow<'a, str>>,
    loaderId: Option<crate::network::LoaderId<'a>>,
}

impl<'a> ReloadParamsBuilder<'a> {
    /// If true, browser cache is ignored (as if the user pressed Shift+refresh).
    pub fn ignoreCache(mut self, ignoreCache: bool) -> Self { self.ignoreCache = Some(ignoreCache); self }
    /// If set, the script will be injected into all frames of the inspected page after reload.
    /// Argument will be ignored if reloading dataURL origin.
    pub fn scriptToEvaluateOnLoad(mut self, scriptToEvaluateOnLoad: impl Into<Cow<'a, str>>) -> Self { self.scriptToEvaluateOnLoad = Some(scriptToEvaluateOnLoad.into()); self }
    /// If set, an error will be thrown if the target page's main frame's
    /// loader id does not match the provided id. This prevents accidentally
    /// reloading an unintended target in case there's a racing navigation.
    pub fn loaderId(mut self, loaderId: crate::network::LoaderId<'a>) -> Self { self.loaderId = Some(loaderId); self }
    pub fn build(self) -> ReloadParams<'a> {
        ReloadParams {
            ignoreCache: self.ignoreCache,
            scriptToEvaluateOnLoad: self.scriptToEvaluateOnLoad,
            loaderId: self.loaderId,
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
    pub fn builder() -> RemoveScriptToEvaluateOnLoadParamsBuilder<'a> { RemoveScriptToEvaluateOnLoadParamsBuilder::default() }
    pub fn identifier(&self) -> &ScriptIdentifier<'a> { &self.identifier }
}

#[derive(Default)]
pub struct RemoveScriptToEvaluateOnLoadParamsBuilder<'a> {
    identifier: Option<ScriptIdentifier<'a>>,
}

impl<'a> RemoveScriptToEvaluateOnLoadParamsBuilder<'a> {
    pub fn identifier(mut self, identifier: ScriptIdentifier<'a>) -> Self { self.identifier = Some(identifier); self }
    pub fn build(self) -> RemoveScriptToEvaluateOnLoadParams<'a> {
        RemoveScriptToEvaluateOnLoadParams {
            identifier: self.identifier.unwrap_or_default(),
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
    pub fn builder() -> RemoveScriptToEvaluateOnNewDocumentParamsBuilder<'a> { RemoveScriptToEvaluateOnNewDocumentParamsBuilder::default() }
    pub fn identifier(&self) -> &ScriptIdentifier<'a> { &self.identifier }
}

#[derive(Default)]
pub struct RemoveScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
    identifier: Option<ScriptIdentifier<'a>>,
}

impl<'a> RemoveScriptToEvaluateOnNewDocumentParamsBuilder<'a> {
    pub fn identifier(mut self, identifier: ScriptIdentifier<'a>) -> Self { self.identifier = Some(identifier); self }
    pub fn build(self) -> RemoveScriptToEvaluateOnNewDocumentParams<'a> {
        RemoveScriptToEvaluateOnNewDocumentParams {
            identifier: self.identifier.unwrap_or_default(),
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
    sessionId: u64,
}

impl ScreencastFrameAckParams {
    pub fn builder() -> ScreencastFrameAckParamsBuilder { ScreencastFrameAckParamsBuilder::default() }
    pub fn sessionId(&self) -> u64 { self.sessionId }
}

#[derive(Default)]
pub struct ScreencastFrameAckParamsBuilder {
    sessionId: Option<u64>,
}

impl ScreencastFrameAckParamsBuilder {
    /// Frame number.
    pub fn sessionId(mut self, sessionId: u64) -> Self { self.sessionId = Some(sessionId); self }
    pub fn build(self) -> ScreencastFrameAckParams {
        ScreencastFrameAckParams {
            sessionId: self.sessionId.unwrap_or_default(),
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
    frameId: FrameId<'a>,
    /// URL of the resource to search in.
    url: Cow<'a, str>,
    /// String to search for.
    query: Cow<'a, str>,
    /// If true, search is case sensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    caseSensitive: Option<bool>,
    /// If true, treats string parameter as regex.
    #[serde(skip_serializing_if = "Option::is_none")]
    isRegex: Option<bool>,
}

impl<'a> SearchInResourceParams<'a> {
    pub fn builder() -> SearchInResourceParamsBuilder<'a> { SearchInResourceParamsBuilder::default() }
    pub fn frameId(&self) -> &FrameId<'a> { &self.frameId }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn query(&self) -> &str { self.query.as_ref() }
    pub fn caseSensitive(&self) -> Option<bool> { self.caseSensitive }
    pub fn isRegex(&self) -> Option<bool> { self.isRegex }
}

#[derive(Default)]
pub struct SearchInResourceParamsBuilder<'a> {
    frameId: Option<FrameId<'a>>,
    url: Option<Cow<'a, str>>,
    query: Option<Cow<'a, str>>,
    caseSensitive: Option<bool>,
    isRegex: Option<bool>,
}

impl<'a> SearchInResourceParamsBuilder<'a> {
    /// Frame id for resource to search in.
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// URL of the resource to search in.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// String to search for.
    pub fn query(mut self, query: impl Into<Cow<'a, str>>) -> Self { self.query = Some(query.into()); self }
    /// If true, search is case sensitive.
    pub fn caseSensitive(mut self, caseSensitive: bool) -> Self { self.caseSensitive = Some(caseSensitive); self }
    /// If true, treats string parameter as regex.
    pub fn isRegex(mut self, isRegex: bool) -> Self { self.isRegex = Some(isRegex); self }
    pub fn build(self) -> SearchInResourceParams<'a> {
        SearchInResourceParams {
            frameId: self.frameId.unwrap_or_default(),
            url: self.url.unwrap_or_default(),
            query: self.query.unwrap_or_default(),
            caseSensitive: self.caseSensitive,
            isRegex: self.isRegex,
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
    pub fn builder() -> SearchInResourceReturnsBuilder { SearchInResourceReturnsBuilder::default() }
    pub fn result(&self) -> &[crate::debugger::SearchMatch] { &self.result }
}

#[derive(Default)]
pub struct SearchInResourceReturnsBuilder {
    result: Option<Vec<crate::debugger::SearchMatch>>,
}

impl SearchInResourceReturnsBuilder {
    /// List of search matches.
    pub fn result(mut self, result: Vec<crate::debugger::SearchMatch>) -> Self { self.result = Some(result); self }
    pub fn build(self) -> SearchInResourceReturns {
        SearchInResourceReturns {
            result: self.result.unwrap_or_default(),
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
    pub fn builder() -> SetAdBlockingEnabledParamsBuilder { SetAdBlockingEnabledParamsBuilder::default() }
    pub fn enabled(&self) -> bool { self.enabled }
}

#[derive(Default)]
pub struct SetAdBlockingEnabledParamsBuilder {
    enabled: Option<bool>,
}

impl SetAdBlockingEnabledParamsBuilder {
    /// Whether to block ads.
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    pub fn build(self) -> SetAdBlockingEnabledParams {
        SetAdBlockingEnabledParams {
            enabled: self.enabled.unwrap_or_default(),
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
    pub fn builder() -> SetBypassCSPParamsBuilder { SetBypassCSPParamsBuilder::default() }
    pub fn enabled(&self) -> bool { self.enabled }
}

#[derive(Default)]
pub struct SetBypassCSPParamsBuilder {
    enabled: Option<bool>,
}

impl SetBypassCSPParamsBuilder {
    /// Whether to bypass page CSP.
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    pub fn build(self) -> SetBypassCSPParams {
        SetBypassCSPParams {
            enabled: self.enabled.unwrap_or_default(),
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
    frameId: FrameId<'a>,
}

impl<'a> GetPermissionsPolicyStateParams<'a> {
    pub fn builder() -> GetPermissionsPolicyStateParamsBuilder<'a> { GetPermissionsPolicyStateParamsBuilder::default() }
    pub fn frameId(&self) -> &FrameId<'a> { &self.frameId }
}

#[derive(Default)]
pub struct GetPermissionsPolicyStateParamsBuilder<'a> {
    frameId: Option<FrameId<'a>>,
}

impl<'a> GetPermissionsPolicyStateParamsBuilder<'a> {
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetPermissionsPolicyStateParams<'a> {
        GetPermissionsPolicyStateParams {
            frameId: self.frameId.unwrap_or_default(),
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
    pub fn builder() -> GetPermissionsPolicyStateReturnsBuilder<'a> { GetPermissionsPolicyStateReturnsBuilder::default() }
    pub fn states(&self) -> &[PermissionsPolicyFeatureState<'a>] { &self.states }
}

#[derive(Default)]
pub struct GetPermissionsPolicyStateReturnsBuilder<'a> {
    states: Option<Vec<PermissionsPolicyFeatureState<'a>>>,
}

impl<'a> GetPermissionsPolicyStateReturnsBuilder<'a> {
    pub fn states(mut self, states: Vec<PermissionsPolicyFeatureState<'a>>) -> Self { self.states = Some(states); self }
    pub fn build(self) -> GetPermissionsPolicyStateReturns<'a> {
        GetPermissionsPolicyStateReturns {
            states: self.states.unwrap_or_default(),
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
    frameId: FrameId<'a>,
}

impl<'a> GetOriginTrialsParams<'a> {
    pub fn builder() -> GetOriginTrialsParamsBuilder<'a> { GetOriginTrialsParamsBuilder::default() }
    pub fn frameId(&self) -> &FrameId<'a> { &self.frameId }
}

#[derive(Default)]
pub struct GetOriginTrialsParamsBuilder<'a> {
    frameId: Option<FrameId<'a>>,
}

impl<'a> GetOriginTrialsParamsBuilder<'a> {
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetOriginTrialsParams<'a> {
        GetOriginTrialsParams {
            frameId: self.frameId.unwrap_or_default(),
        }
    }
}

/// Get Origin Trials on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOriginTrialsReturns<'a> {
    originTrials: Vec<OriginTrial<'a>>,
}

impl<'a> GetOriginTrialsReturns<'a> {
    pub fn builder() -> GetOriginTrialsReturnsBuilder<'a> { GetOriginTrialsReturnsBuilder::default() }
    pub fn originTrials(&self) -> &[OriginTrial<'a>] { &self.originTrials }
}

#[derive(Default)]
pub struct GetOriginTrialsReturnsBuilder<'a> {
    originTrials: Option<Vec<OriginTrial<'a>>>,
}

impl<'a> GetOriginTrialsReturnsBuilder<'a> {
    pub fn originTrials(mut self, originTrials: Vec<OriginTrial<'a>>) -> Self { self.originTrials = Some(originTrials); self }
    pub fn build(self) -> GetOriginTrialsReturns<'a> {
        GetOriginTrialsReturns {
            originTrials: self.originTrials.unwrap_or_default(),
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
    screenOrientation: Option<crate::emulation::ScreenOrientation<'a>>,
    /// The viewport dimensions and scale. If not set, the override is cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    viewport: Option<Viewport>,
}

impl<'a> SetDeviceMetricsOverrideParams<'a> {
    pub fn builder() -> SetDeviceMetricsOverrideParamsBuilder<'a> { SetDeviceMetricsOverrideParamsBuilder::default() }
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
    pub fn screenOrientation(&self) -> Option<&crate::emulation::ScreenOrientation<'a>> { self.screenOrientation.as_ref() }
    pub fn viewport(&self) -> Option<&Viewport> { self.viewport.as_ref() }
}

#[derive(Default)]
pub struct SetDeviceMetricsOverrideParamsBuilder<'a> {
    width: Option<u64>,
    height: Option<i64>,
    deviceScaleFactor: Option<f64>,
    mobile: Option<bool>,
    scale: Option<f64>,
    screenWidth: Option<u64>,
    screenHeight: Option<i64>,
    positionX: Option<i64>,
    positionY: Option<i64>,
    dontSetVisibleSize: Option<bool>,
    screenOrientation: Option<crate::emulation::ScreenOrientation<'a>>,
    viewport: Option<Viewport>,
}

impl<'a> SetDeviceMetricsOverrideParamsBuilder<'a> {
    /// Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub fn width(mut self, width: u64) -> Self { self.width = Some(width); self }
    /// Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub fn height(mut self, height: i64) -> Self { self.height = Some(height); self }
    /// Overriding device scale factor value. 0 disables the override.
    pub fn deviceScaleFactor(mut self, deviceScaleFactor: f64) -> Self { self.deviceScaleFactor = Some(deviceScaleFactor); self }
    /// Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    /// autosizing and more.
    pub fn mobile(mut self, mobile: bool) -> Self { self.mobile = Some(mobile); self }
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
    pub fn screenOrientation(mut self, screenOrientation: crate::emulation::ScreenOrientation<'a>) -> Self { self.screenOrientation = Some(screenOrientation); self }
    /// The viewport dimensions and scale. If not set, the override is cleared.
    pub fn viewport(mut self, viewport: Viewport) -> Self { self.viewport = Some(viewport); self }
    pub fn build(self) -> SetDeviceMetricsOverrideParams<'a> {
        SetDeviceMetricsOverrideParams {
            width: self.width.unwrap_or_default(),
            height: self.height.unwrap_or_default(),
            deviceScaleFactor: self.deviceScaleFactor.unwrap_or_default(),
            mobile: self.mobile.unwrap_or_default(),
            scale: self.scale,
            screenWidth: self.screenWidth,
            screenHeight: self.screenHeight,
            positionX: self.positionX,
            positionY: self.positionY,
            dontSetVisibleSize: self.dontSetVisibleSize,
            screenOrientation: self.screenOrientation,
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
    pub fn builder() -> SetDeviceOrientationOverrideParamsBuilder { SetDeviceOrientationOverrideParamsBuilder::default() }
    pub fn alpha(&self) -> f64 { self.alpha }
    pub fn beta(&self) -> f64 { self.beta }
    pub fn gamma(&self) -> f64 { self.gamma }
}

#[derive(Default)]
pub struct SetDeviceOrientationOverrideParamsBuilder {
    alpha: Option<f64>,
    beta: Option<f64>,
    gamma: Option<f64>,
}

impl SetDeviceOrientationOverrideParamsBuilder {
    /// Mock alpha
    pub fn alpha(mut self, alpha: f64) -> Self { self.alpha = Some(alpha); self }
    /// Mock beta
    pub fn beta(mut self, beta: f64) -> Self { self.beta = Some(beta); self }
    /// Mock gamma
    pub fn gamma(mut self, gamma: f64) -> Self { self.gamma = Some(gamma); self }
    pub fn build(self) -> SetDeviceOrientationOverrideParams {
        SetDeviceOrientationOverrideParams {
            alpha: self.alpha.unwrap_or_default(),
            beta: self.beta.unwrap_or_default(),
            gamma: self.gamma.unwrap_or_default(),
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
    fontFamilies: FontFamilies<'a>,
    /// Specifies font families to set for individual scripts.
    #[serde(skip_serializing_if = "Option::is_none")]
    forScripts: Option<Vec<ScriptFontFamilies<'a>>>,
}

impl<'a> SetFontFamiliesParams<'a> {
    pub fn builder() -> SetFontFamiliesParamsBuilder<'a> { SetFontFamiliesParamsBuilder::default() }
    pub fn fontFamilies(&self) -> &FontFamilies<'a> { &self.fontFamilies }
    pub fn forScripts(&self) -> Option<&[ScriptFontFamilies<'a>]> { self.forScripts.as_deref() }
}

#[derive(Default)]
pub struct SetFontFamiliesParamsBuilder<'a> {
    fontFamilies: Option<FontFamilies<'a>>,
    forScripts: Option<Vec<ScriptFontFamilies<'a>>>,
}

impl<'a> SetFontFamiliesParamsBuilder<'a> {
    /// Specifies font families to set. If a font family is not specified, it won't be changed.
    pub fn fontFamilies(mut self, fontFamilies: FontFamilies<'a>) -> Self { self.fontFamilies = Some(fontFamilies); self }
    /// Specifies font families to set for individual scripts.
    pub fn forScripts(mut self, forScripts: Vec<ScriptFontFamilies<'a>>) -> Self { self.forScripts = Some(forScripts); self }
    pub fn build(self) -> SetFontFamiliesParams<'a> {
        SetFontFamiliesParams {
            fontFamilies: self.fontFamilies.unwrap_or_default(),
            forScripts: self.forScripts,
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
    fontSizes: FontSizes,
}

impl SetFontSizesParams {
    pub fn builder() -> SetFontSizesParamsBuilder { SetFontSizesParamsBuilder::default() }
    pub fn fontSizes(&self) -> &FontSizes { &self.fontSizes }
}

#[derive(Default)]
pub struct SetFontSizesParamsBuilder {
    fontSizes: Option<FontSizes>,
}

impl SetFontSizesParamsBuilder {
    /// Specifies font sizes to set. If a font size is not specified, it won't be changed.
    pub fn fontSizes(mut self, fontSizes: FontSizes) -> Self { self.fontSizes = Some(fontSizes); self }
    pub fn build(self) -> SetFontSizesParams {
        SetFontSizesParams {
            fontSizes: self.fontSizes.unwrap_or_default(),
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
    frameId: FrameId<'a>,
    /// HTML content to set.
    html: Cow<'a, str>,
}

impl<'a> SetDocumentContentParams<'a> {
    pub fn builder() -> SetDocumentContentParamsBuilder<'a> { SetDocumentContentParamsBuilder::default() }
    pub fn frameId(&self) -> &FrameId<'a> { &self.frameId }
    pub fn html(&self) -> &str { self.html.as_ref() }
}

#[derive(Default)]
pub struct SetDocumentContentParamsBuilder<'a> {
    frameId: Option<FrameId<'a>>,
    html: Option<Cow<'a, str>>,
}

impl<'a> SetDocumentContentParamsBuilder<'a> {
    /// Frame id to set HTML for.
    pub fn frameId(mut self, frameId: FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// HTML content to set.
    pub fn html(mut self, html: impl Into<Cow<'a, str>>) -> Self { self.html = Some(html.into()); self }
    pub fn build(self) -> SetDocumentContentParams<'a> {
        SetDocumentContentParams {
            frameId: self.frameId.unwrap_or_default(),
            html: self.html.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    downloadPath: Option<Cow<'a, str>>,
}

impl<'a> SetDownloadBehaviorParams<'a> {
    pub fn builder() -> SetDownloadBehaviorParamsBuilder<'a> { SetDownloadBehaviorParamsBuilder::default() }
    pub fn behavior(&self) -> &str { self.behavior.as_ref() }
    pub fn downloadPath(&self) -> Option<&str> { self.downloadPath.as_deref() }
}

#[derive(Default)]
pub struct SetDownloadBehaviorParamsBuilder<'a> {
    behavior: Option<Cow<'a, str>>,
    downloadPath: Option<Cow<'a, str>>,
}

impl<'a> SetDownloadBehaviorParamsBuilder<'a> {
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny).
    pub fn behavior(mut self, behavior: impl Into<Cow<'a, str>>) -> Self { self.behavior = Some(behavior.into()); self }
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    pub fn downloadPath(mut self, downloadPath: impl Into<Cow<'a, str>>) -> Self { self.downloadPath = Some(downloadPath.into()); self }
    pub fn build(self) -> SetDownloadBehaviorParams<'a> {
        SetDownloadBehaviorParams {
            behavior: self.behavior.unwrap_or_default(),
            downloadPath: self.downloadPath,
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
    pub fn builder() -> SetGeolocationOverrideParamsBuilder { SetGeolocationOverrideParamsBuilder::default() }
    pub fn latitude(&self) -> Option<f64> { self.latitude }
    pub fn longitude(&self) -> Option<f64> { self.longitude }
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
    pub fn builder() -> SetLifecycleEventsEnabledParamsBuilder { SetLifecycleEventsEnabledParamsBuilder::default() }
    pub fn enabled(&self) -> bool { self.enabled }
}

#[derive(Default)]
pub struct SetLifecycleEventsEnabledParamsBuilder {
    enabled: Option<bool>,
}

impl SetLifecycleEventsEnabledParamsBuilder {
    /// If true, starts emitting lifecycle events.
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    pub fn build(self) -> SetLifecycleEventsEnabledParams {
        SetLifecycleEventsEnabledParams {
            enabled: self.enabled.unwrap_or_default(),
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
    pub fn builder() -> SetTouchEmulationEnabledParamsBuilder<'a> { SetTouchEmulationEnabledParamsBuilder::default() }
    pub fn enabled(&self) -> bool { self.enabled }
    pub fn configuration(&self) -> Option<&str> { self.configuration.as_deref() }
}

#[derive(Default)]
pub struct SetTouchEmulationEnabledParamsBuilder<'a> {
    enabled: Option<bool>,
    configuration: Option<Cow<'a, str>>,
}

impl<'a> SetTouchEmulationEnabledParamsBuilder<'a> {
    /// Whether the touch event emulation should be enabled.
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    /// Touch/gesture events configuration. Default: current platform.
    pub fn configuration(mut self, configuration: impl Into<Cow<'a, str>>) -> Self { self.configuration = Some(configuration.into()); self }
    pub fn build(self) -> SetTouchEmulationEnabledParams<'a> {
        SetTouchEmulationEnabledParams {
            enabled: self.enabled.unwrap_or_default(),
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
    /// Compression quality from range [0..100].
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<i64>,
    /// Maximum screenshot width.
    #[serde(skip_serializing_if = "Option::is_none")]
    maxWidth: Option<u64>,
    /// Maximum screenshot height.
    #[serde(skip_serializing_if = "Option::is_none")]
    maxHeight: Option<i64>,
    /// Send every n-th frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    everyNthFrame: Option<i64>,
}

impl<'a> StartScreencastParams<'a> {
    pub fn builder() -> StartScreencastParamsBuilder<'a> { StartScreencastParamsBuilder::default() }
    pub fn format(&self) -> Option<&str> { self.format.as_deref() }
    pub fn quality(&self) -> Option<i64> { self.quality }
    pub fn maxWidth(&self) -> Option<u64> { self.maxWidth }
    pub fn maxHeight(&self) -> Option<i64> { self.maxHeight }
    pub fn everyNthFrame(&self) -> Option<i64> { self.everyNthFrame }
}

#[derive(Default)]
pub struct StartScreencastParamsBuilder<'a> {
    format: Option<Cow<'a, str>>,
    quality: Option<i64>,
    maxWidth: Option<u64>,
    maxHeight: Option<i64>,
    everyNthFrame: Option<i64>,
}

impl<'a> StartScreencastParamsBuilder<'a> {
    /// Image compression format.
    pub fn format(mut self, format: impl Into<Cow<'a, str>>) -> Self { self.format = Some(format.into()); self }
    /// Compression quality from range [0..100].
    pub fn quality(mut self, quality: i64) -> Self { self.quality = Some(quality); self }
    /// Maximum screenshot width.
    pub fn maxWidth(mut self, maxWidth: u64) -> Self { self.maxWidth = Some(maxWidth); self }
    /// Maximum screenshot height.
    pub fn maxHeight(mut self, maxHeight: i64) -> Self { self.maxHeight = Some(maxHeight); self }
    /// Send every n-th frame.
    pub fn everyNthFrame(mut self, everyNthFrame: i64) -> Self { self.everyNthFrame = Some(everyNthFrame); self }
    pub fn build(self) -> StartScreencastParams<'a> {
        StartScreencastParams {
            format: self.format,
            quality: self.quality,
            maxWidth: self.maxWidth,
            maxHeight: self.maxHeight,
            everyNthFrame: self.everyNthFrame,
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

impl StopLoadingParams {
    pub fn builder() -> StopLoadingParamsBuilder {
        StopLoadingParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct StopLoadingParamsBuilder {}

impl StopLoadingParamsBuilder {
    pub fn build(self) -> StopLoadingParams {
        StopLoadingParams {}
    }
}

impl StopLoadingParams { pub const METHOD: &'static str = "Page.stopLoading"; }

impl<'a> crate::CdpCommand<'a> for StopLoadingParams {
    const METHOD: &'static str = "Page.stopLoading";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrashParams {}

impl CrashParams {
    pub fn builder() -> CrashParamsBuilder {
        CrashParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct CrashParamsBuilder {}

impl CrashParamsBuilder {
    pub fn build(self) -> CrashParams {
        CrashParams {}
    }
}

impl CrashParams { pub const METHOD: &'static str = "Page.crash"; }

impl<'a> crate::CdpCommand<'a> for CrashParams {
    const METHOD: &'static str = "Page.crash";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloseParams {}

impl CloseParams {
    pub fn builder() -> CloseParamsBuilder {
        CloseParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct CloseParamsBuilder {}

impl CloseParamsBuilder {
    pub fn build(self) -> CloseParams {
        CloseParams {}
    }
}

impl CloseParams { pub const METHOD: &'static str = "Page.close"; }

impl<'a> crate::CdpCommand<'a> for CloseParams {
    const METHOD: &'static str = "Page.close";
    type Response = crate::EmptyReturns;
}

/// Tries to update the web lifecycle state of the page.
/// It will transition the page to the given state according to:
/// https://github.com/WICG/web-lifecycle/

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetWebLifecycleStateParams<'a> {
    /// Target lifecycle state
    state: Cow<'a, str>,
}

impl<'a> SetWebLifecycleStateParams<'a> {
    pub fn builder() -> SetWebLifecycleStateParamsBuilder<'a> { SetWebLifecycleStateParamsBuilder::default() }
    pub fn state(&self) -> &str { self.state.as_ref() }
}

#[derive(Default)]
pub struct SetWebLifecycleStateParamsBuilder<'a> {
    state: Option<Cow<'a, str>>,
}

impl<'a> SetWebLifecycleStateParamsBuilder<'a> {
    /// Target lifecycle state
    pub fn state(mut self, state: impl Into<Cow<'a, str>>) -> Self { self.state = Some(state.into()); self }
    pub fn build(self) -> SetWebLifecycleStateParams<'a> {
        SetWebLifecycleStateParams {
            state: self.state.unwrap_or_default(),
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

impl StopScreencastParams {
    pub fn builder() -> StopScreencastParamsBuilder {
        StopScreencastParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct StopScreencastParamsBuilder {}

impl StopScreencastParamsBuilder {
    pub fn build(self) -> StopScreencastParams {
        StopScreencastParams {}
    }
}

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
    pub fn builder() -> ProduceCompilationCacheParamsBuilder<'a> { ProduceCompilationCacheParamsBuilder::default() }
    pub fn scripts(&self) -> &[CompilationCacheParams<'a>] { &self.scripts }
}

#[derive(Default)]
pub struct ProduceCompilationCacheParamsBuilder<'a> {
    scripts: Option<Vec<CompilationCacheParams<'a>>>,
}

impl<'a> ProduceCompilationCacheParamsBuilder<'a> {
    pub fn scripts(mut self, scripts: Vec<CompilationCacheParams<'a>>) -> Self { self.scripts = Some(scripts); self }
    pub fn build(self) -> ProduceCompilationCacheParams<'a> {
        ProduceCompilationCacheParams {
            scripts: self.scripts.unwrap_or_default(),
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
    pub fn builder() -> AddCompilationCacheParamsBuilder<'a> { AddCompilationCacheParamsBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn data(&self) -> &str { self.data.as_ref() }
}

#[derive(Default)]
pub struct AddCompilationCacheParamsBuilder<'a> {
    url: Option<Cow<'a, str>>,
    data: Option<Cow<'a, str>>,
}

impl<'a> AddCompilationCacheParamsBuilder<'a> {
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Base64-encoded data (Encoded as a base64 string when passed over JSON)
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> AddCompilationCacheParams<'a> {
        AddCompilationCacheParams {
            url: self.url.unwrap_or_default(),
            data: self.data.unwrap_or_default(),
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

impl ClearCompilationCacheParams {
    pub fn builder() -> ClearCompilationCacheParamsBuilder {
        ClearCompilationCacheParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ClearCompilationCacheParamsBuilder {}

impl ClearCompilationCacheParamsBuilder {
    pub fn build(self) -> ClearCompilationCacheParams {
        ClearCompilationCacheParams {}
    }
}

impl ClearCompilationCacheParams { pub const METHOD: &'static str = "Page.clearCompilationCache"; }

impl<'a> crate::CdpCommand<'a> for ClearCompilationCacheParams {
    const METHOD: &'static str = "Page.clearCompilationCache";
    type Response = crate::EmptyReturns;
}

/// Sets the Secure Payment Confirmation transaction mode.
/// https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSPCTransactionModeParams<'a> {
    mode: Cow<'a, str>,
}

impl<'a> SetSPCTransactionModeParams<'a> {
    pub fn builder() -> SetSPCTransactionModeParamsBuilder<'a> { SetSPCTransactionModeParamsBuilder::default() }
    pub fn mode(&self) -> &str { self.mode.as_ref() }
}

#[derive(Default)]
pub struct SetSPCTransactionModeParamsBuilder<'a> {
    mode: Option<Cow<'a, str>>,
}

impl<'a> SetSPCTransactionModeParamsBuilder<'a> {
    pub fn mode(mut self, mode: impl Into<Cow<'a, str>>) -> Self { self.mode = Some(mode.into()); self }
    pub fn build(self) -> SetSPCTransactionModeParams<'a> {
        SetSPCTransactionModeParams {
            mode: self.mode.unwrap_or_default(),
        }
    }
}

impl<'a> SetSPCTransactionModeParams<'a> { pub const METHOD: &'static str = "Page.setSPCTransactionMode"; }

impl<'a> crate::CdpCommand<'a> for SetSPCTransactionModeParams<'a> {
    const METHOD: &'static str = "Page.setSPCTransactionMode";
    type Response = crate::EmptyReturns;
}

/// Extensions for Custom Handlers API:
/// https://html.spec.whatwg.org/multipage/system-state.html#rph-automation

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRPHRegistrationModeParams<'a> {
    mode: Cow<'a, str>,
}

impl<'a> SetRPHRegistrationModeParams<'a> {
    pub fn builder() -> SetRPHRegistrationModeParamsBuilder<'a> { SetRPHRegistrationModeParamsBuilder::default() }
    pub fn mode(&self) -> &str { self.mode.as_ref() }
}

#[derive(Default)]
pub struct SetRPHRegistrationModeParamsBuilder<'a> {
    mode: Option<Cow<'a, str>>,
}

impl<'a> SetRPHRegistrationModeParamsBuilder<'a> {
    pub fn mode(mut self, mode: impl Into<Cow<'a, str>>) -> Self { self.mode = Some(mode.into()); self }
    pub fn build(self) -> SetRPHRegistrationModeParams<'a> {
        SetRPHRegistrationModeParams {
            mode: self.mode.unwrap_or_default(),
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
    pub fn builder() -> GenerateTestReportParamsBuilder<'a> { GenerateTestReportParamsBuilder::default() }
    pub fn message(&self) -> &str { self.message.as_ref() }
    pub fn group(&self) -> Option<&str> { self.group.as_deref() }
}

#[derive(Default)]
pub struct GenerateTestReportParamsBuilder<'a> {
    message: Option<Cow<'a, str>>,
    group: Option<Cow<'a, str>>,
}

impl<'a> GenerateTestReportParamsBuilder<'a> {
    /// Message to be displayed in the report.
    pub fn message(mut self, message: impl Into<Cow<'a, str>>) -> Self { self.message = Some(message.into()); self }
    /// Specifies the endpoint group to deliver the report to.
    pub fn group(mut self, group: impl Into<Cow<'a, str>>) -> Self { self.group = Some(group.into()); self }
    pub fn build(self) -> GenerateTestReportParams<'a> {
        GenerateTestReportParams {
            message: self.message.unwrap_or_default(),
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

impl WaitForDebuggerParams {
    pub fn builder() -> WaitForDebuggerParamsBuilder {
        WaitForDebuggerParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct WaitForDebuggerParamsBuilder {}

impl WaitForDebuggerParamsBuilder {
    pub fn build(self) -> WaitForDebuggerParams {
        WaitForDebuggerParams {}
    }
}

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
    pub fn builder() -> SetInterceptFileChooserDialogParamsBuilder { SetInterceptFileChooserDialogParamsBuilder::default() }
    pub fn enabled(&self) -> bool { self.enabled }
    pub fn cancel(&self) -> Option<bool> { self.cancel }
}

#[derive(Default)]
pub struct SetInterceptFileChooserDialogParamsBuilder {
    enabled: Option<bool>,
    cancel: Option<bool>,
}

impl SetInterceptFileChooserDialogParamsBuilder {
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = Some(enabled); self }
    /// If true, cancels the dialog by emitting relevant events (if any)
    /// in addition to not showing it if the interception is enabled
    /// (default: false).
    pub fn cancel(mut self, cancel: bool) -> Self { self.cancel = Some(cancel); self }
    pub fn build(self) -> SetInterceptFileChooserDialogParams {
        SetInterceptFileChooserDialogParams {
            enabled: self.enabled.unwrap_or_default(),
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
/// This command is a short-term solution for https://crbug.com/1440085.
/// See https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA
/// for more details.
/// 
/// TODO(https://crbug.com/1440085): Remove this once Puppeteer supports tab targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPrerenderingAllowedParams {
    isAllowed: bool,
}

impl SetPrerenderingAllowedParams {
    pub fn builder() -> SetPrerenderingAllowedParamsBuilder { SetPrerenderingAllowedParamsBuilder::default() }
    pub fn isAllowed(&self) -> bool { self.isAllowed }
}

#[derive(Default)]
pub struct SetPrerenderingAllowedParamsBuilder {
    isAllowed: Option<bool>,
}

impl SetPrerenderingAllowedParamsBuilder {
    pub fn isAllowed(mut self, isAllowed: bool) -> Self { self.isAllowed = Some(isAllowed); self }
    pub fn build(self) -> SetPrerenderingAllowedParams {
        SetPrerenderingAllowedParams {
            isAllowed: self.isAllowed.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    includeActionableInformation: Option<bool>,
}

impl GetAnnotatedPageContentParams {
    pub fn builder() -> GetAnnotatedPageContentParamsBuilder { GetAnnotatedPageContentParamsBuilder::default() }
    pub fn includeActionableInformation(&self) -> Option<bool> { self.includeActionableInformation }
}

#[derive(Default)]
pub struct GetAnnotatedPageContentParamsBuilder {
    includeActionableInformation: Option<bool>,
}

impl GetAnnotatedPageContentParamsBuilder {
    /// Whether to include actionable information. Defaults to true.
    pub fn includeActionableInformation(mut self, includeActionableInformation: bool) -> Self { self.includeActionableInformation = Some(includeActionableInformation); self }
    pub fn build(self) -> GetAnnotatedPageContentParams {
        GetAnnotatedPageContentParams {
            includeActionableInformation: self.includeActionableInformation,
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
    pub fn builder() -> GetAnnotatedPageContentReturnsBuilder<'a> { GetAnnotatedPageContentReturnsBuilder::default() }
    pub fn content(&self) -> &str { self.content.as_ref() }
}

#[derive(Default)]
pub struct GetAnnotatedPageContentReturnsBuilder<'a> {
    content: Option<Cow<'a, str>>,
}

impl<'a> GetAnnotatedPageContentReturnsBuilder<'a> {
    /// The annotated page content as a base64 encoded protobuf.
    /// The format is defined by the 'AnnotatedPageContent' message in
    /// components/optimization_guide/proto/features/common_quality_data.proto (Encoded as a base64 string when passed over JSON)
    pub fn content(mut self, content: impl Into<Cow<'a, str>>) -> Self { self.content = Some(content.into()); self }
    pub fn build(self) -> GetAnnotatedPageContentReturns<'a> {
        GetAnnotatedPageContentReturns {
            content: self.content.unwrap_or_default(),
        }
    }
}

impl GetAnnotatedPageContentParams { pub const METHOD: &'static str = "Page.getAnnotatedPageContent"; }

impl<'a> crate::CdpCommand<'a> for GetAnnotatedPageContentParams {
    const METHOD: &'static str = "Page.getAnnotatedPageContent";
    type Response = GetAnnotatedPageContentReturns<'a>;
}
