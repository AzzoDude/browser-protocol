//! Actions and events related to the inspected page belong to the page domain.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdFrameStatus {
    #[serde(rename = "adFrameType")]
    pub ad_frame_type: AdFrameType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanations: Option<Vec<AdFrameExplanation>>,
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
    #[serde(rename = "haptics")]
    Haptics,
    #[serde(rename = "hid")]
    Hid,
    #[serde(rename = "identity-credentials-get")]
    IdentityCredentialsGet,
    #[serde(rename = "idle-detection")]
    IdleDetection,
    #[serde(rename = "interest-cohort")]
    InterestCohort,
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
    #[serde(rename = "private-state-token-issuance")]
    PrivateStateTokenIssuance,
    #[serde(rename = "private-state-token-redemption")]
    PrivateStateTokenRedemption,
    #[serde(rename = "publickey-credentials-create")]
    PublickeyCredentialsCreate,
    #[serde(rename = "publickey-credentials-get")]
    PublickeyCredentialsGet,
    #[serde(rename = "publickey-credentials-remote-client-data-json")]
    PublickeyCredentialsRemoteClientDataJson,
    #[serde(rename = "rewriter")]
    Rewriter,
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
    #[serde(rename = "webnn")]
    Webnn,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsPolicyBlockLocator<'a> {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    #[serde(rename = "blockReason")]
    pub block_reason: PermissionsPolicyBlockReason,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsPolicyFeatureState<'a> {
    pub feature: PermissionsPolicyFeature,
    pub allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<PermissionsPolicyBlockLocator<'a>>,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrialToken<'a> {
    pub origin: Cow<'a, str>,
    #[serde(rename = "matchSubDomains")]
    pub match_sub_domains: bool,
    #[serde(rename = "trialName")]
    pub trial_name: Cow<'a, str>,
    #[serde(rename = "expiryTime")]
    pub expiry_time: crate::network::TimeSinceEpoch,
    #[serde(rename = "isThirdParty")]
    pub is_third_party: bool,
    #[serde(rename = "usageRestriction")]
    pub usage_restriction: OriginTrialUsageRestriction,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrialTokenWithStatus<'a> {
    #[serde(rename = "rawTokenText")]
    pub raw_token_text: Cow<'a, str>,
    /// 'parsedToken' is present only when the token is extractable and
    /// parsable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parsedToken")]
    pub parsed_token: Option<OriginTrialToken<'a>>,
    pub status: OriginTrialTokenStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrial<'a> {
    #[serde(rename = "trialName")]
    pub trial_name: Cow<'a, str>,
    pub status: OriginTrialStatus,
    #[serde(rename = "tokensWithStatus")]
    pub tokens_with_status: Vec<OriginTrialTokenWithStatus<'a>>,
}
/// Additional information about the frame document's security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SecurityOriginDetails {
    /// Indicates whether the frame document's security origin is one
    /// of the local hostnames (e.g. "localhost") or IP addresses (IPv4
    /// 127.0.0.0/8 or IPv6 ::1).
    #[serde(rename = "isLocalhost")]
    pub is_localhost: bool,
}
/// Information about the Frame on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Frame<'a> {
    /// Frame unique identifier.
    pub id: FrameId<'a>,
    /// Parent frame identifier.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    pub parent_id: Option<FrameId<'a>>,
    /// Identifier of the loader associated with this frame.
    #[serde(rename = "loaderId")]
    pub loader_id: crate::network::LoaderId<'a>,
    /// Frame's name as specified in the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    /// Frame document's URL without fragment.
    pub url: Cow<'a, str>,
    /// Frame document's URL fragment including the '#'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlFragment")]
    pub url_fragment: Option<Cow<'a, str>>,
    /// Frame document's registered domain, taking the public suffixes list into account.
    /// Extracted from the Frame's url.
    /// Example URLs: <http://www.google.com/file.html> -\> "google.com"
    /// <http://a.b.co.uk/file.html>      -\> "b.co.uk"
    #[serde(rename = "domainAndRegistry")]
    pub domain_and_registry: Cow<'a, str>,
    /// Frame document's security origin.
    #[serde(rename = "securityOrigin")]
    pub security_origin: Cow<'a, str>,
    /// Additional details about the frame document's security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOriginDetails")]
    pub security_origin_details: Option<SecurityOriginDetails>,
    /// Frame document's mimeType as determined by the browser.
    #[serde(rename = "mimeType")]
    pub mime_type: Cow<'a, str>,
    /// If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.
    #[serde(skip_serializing_if = "Option::is_none", rename = "unreachableUrl")]
    pub unreachable_url: Option<Cow<'a, str>>,
    /// Indicates whether this frame was tagged as an ad and why.
    #[serde(skip_serializing_if = "Option::is_none", rename = "adFrameStatus")]
    pub ad_frame_status: Option<AdFrameStatus>,
    /// Indicates whether the main document is a secure context and explains why that is the case.
    #[serde(rename = "secureContextType")]
    pub secure_context_type: SecureContextType,
    /// Indicates whether this is a cross origin isolated context.
    #[serde(rename = "crossOriginIsolatedContextType")]
    pub cross_origin_isolated_context_type: CrossOriginIsolatedContextType,
    /// Indicated which gated APIs / features are available.
    #[serde(rename = "gatedAPIFeatures")]
    pub gated_api_features: Vec<GatedAPIFeatures>,
}
/// Information about the Resource on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FrameResource<'a> {
    /// Resource URL.
    pub url: Cow<'a, str>,
    /// Type of this resource.
    #[serde(rename = "type")]
    pub type_: crate::network::ResourceType,
    /// Resource mimeType as determined by the browser.
    #[serde(rename = "mimeType")]
    pub mime_type: Cow<'a, str>,
    /// last-modified timestamp as reported by server.
    #[serde(skip_serializing_if = "Option::is_none", rename = "lastModified")]
    pub last_modified: Option<crate::network::TimeSinceEpoch>,
    /// Resource content size.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentSize")]
    pub content_size: Option<f64>,
    /// True if the resource failed to load.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    /// True if the resource was canceled during loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<bool>,
}
/// Information about the Frame hierarchy along with their cached resources.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FrameResourceTree<'a> {
    /// Frame information for this tree item.
    pub frame: Frame<'a>,
    /// Child frames.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childFrames")]
    pub child_frames: Option<Vec<Box<FrameResourceTree<'a>>>>,
    /// Information about frame resources.
    pub resources: Vec<FrameResource<'a>>,
}
/// Information about the Frame hierarchy.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FrameTree<'a> {
    /// Frame information for this tree item.
    pub frame: Frame<'a>,
    /// Child frames.
    #[serde(skip_serializing_if = "Option::is_none", rename = "childFrames")]
    pub child_frames: Option<Vec<Box<FrameTree<'a>>>>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEntry<'a> {
    /// Unique id of the navigation history entry.
    pub id: u64,
    /// URL of the navigation history entry.
    pub url: Cow<'a, str>,
    /// URL that the user typed in the url bar.
    #[serde(rename = "userTypedURL")]
    pub user_typed_url: Cow<'a, str>,
    /// Title of the navigation history entry.
    pub title: Cow<'a, str>,
    /// Transition type.
    #[serde(rename = "transitionType")]
    pub transition_type: TransitionType,
}
/// Screencast frame metadata.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameMetadata {
    /// Top offset in DIP.
    #[serde(rename = "offsetTop")]
    pub offset_top: f64,
    /// Page scale factor.
    #[serde(rename = "pageScaleFactor")]
    pub page_scale_factor: f64,
    /// Device screen width in DIP.
    #[serde(rename = "deviceWidth")]
    pub device_width: f64,
    /// Device screen height in DIP.
    #[serde(rename = "deviceHeight")]
    pub device_height: f64,
    /// Position of horizontal scroll in CSS pixels.
    #[serde(rename = "scrollOffsetX")]
    pub scroll_offset_x: f64,
    /// Position of vertical scroll in CSS pixels.
    #[serde(rename = "scrollOffsetY")]
    pub scroll_offset_y: f64,
    /// Frame swap timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::network::TimeSinceEpoch>,
    /// Frame swap timestamp as monotonic time.
    #[serde(skip_serializing_if = "Option::is_none", rename = "monotonicTimestamp")]
    pub monotonic_timestamp: Option<crate::network::MonotonicTime>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AppManifestError<'a> {
    /// Error message.
    pub message: Cow<'a, str>,
    /// If critical, this is a non-recoverable parse error.
    pub critical: i64,
    /// Error line.
    pub line: i64,
    /// Error column.
    pub column: i64,
}
/// Parsed app manifest properties.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AppManifestParsedProperties<'a> {
    /// Computed scope value
    pub scope: Cow<'a, str>,
}
/// Layout viewport position and dimensions.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LayoutViewport {
    /// Horizontal offset relative to the document (CSS pixels).
    #[serde(rename = "pageX")]
    pub page_x: i64,
    /// Vertical offset relative to the document (CSS pixels).
    #[serde(rename = "pageY")]
    pub page_y: i64,
    /// Width (CSS pixels), excludes scrollbar if present.
    #[serde(rename = "clientWidth")]
    pub client_width: u64,
    /// Height (CSS pixels), excludes scrollbar if present.
    #[serde(rename = "clientHeight")]
    pub client_height: i64,
}
/// Visual viewport position, dimensions, and scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct VisualViewport {
    /// Horizontal offset relative to the layout viewport (CSS pixels).
    #[serde(rename = "offsetX")]
    pub offset_x: f64,
    /// Vertical offset relative to the layout viewport (CSS pixels).
    #[serde(rename = "offsetY")]
    pub offset_y: f64,
    /// Horizontal offset relative to the document (CSS pixels).
    #[serde(rename = "pageX")]
    pub page_x: f64,
    /// Vertical offset relative to the document (CSS pixels).
    #[serde(rename = "pageY")]
    pub page_y: f64,
    /// Width (CSS pixels), excludes scrollbar if present.
    #[serde(rename = "clientWidth")]
    pub client_width: f64,
    /// Height (CSS pixels), excludes scrollbar if present.
    #[serde(rename = "clientHeight")]
    pub client_height: f64,
    /// Scale relative to the ideal viewport (size at width=device-width).
    pub scale: f64,
    /// Page zoom factor (CSS to device independent pixels ratio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<f64>,
}
/// Viewport for capturing screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Viewport {
    /// X offset in device independent pixels (dip).
    pub x: f64,
    /// Y offset in device independent pixels (dip).
    pub y: f64,
    /// Rectangle width in device independent pixels (dip).
    pub width: f64,
    /// Rectangle height in device independent pixels (dip).
    pub height: f64,
    /// Page scale factor.
    pub scale: f64,
}
/// Generic font families collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FontFamilies<'a> {
    /// The standard font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Cow<'a, str>>,
    /// The fixed font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Cow<'a, str>>,
    /// The serif font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serif: Option<Cow<'a, str>>,
    /// The sansSerif font-family.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sansSerif")]
    pub sans_serif: Option<Cow<'a, str>>,
    /// The cursive font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursive: Option<Cow<'a, str>>,
    /// The fantasy font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fantasy: Option<Cow<'a, str>>,
    /// The math font-family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<Cow<'a, str>>,
}
/// Font families collection for a script.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScriptFontFamilies<'a> {
    /// Name of the script which these font families are defined for.
    pub script: Cow<'a, str>,
    /// Generic font families collection for the script.
    #[serde(rename = "fontFamilies")]
    pub font_families: FontFamilies<'a>,
}
/// Default font sizes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FontSizes {
    /// Default standard font size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<i64>,
    /// Default fixed font size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<i64>,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityErrorArgument<'a> {
    /// Argument name (e.g. name:'minimum-icon-size-in-pixels').
    pub name: Cow<'a, str>,
    /// Argument value (e.g. value:'64').
    pub value: Cow<'a, str>,
}
/// The installability error

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityError<'a> {
    /// The error id (e.g. 'manifest-missing-suitable-icon').
    #[serde(rename = "errorId")]
    pub error_id: Cow<'a, str>,
    /// The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).
    #[serde(rename = "errorArguments")]
    pub error_arguments: Vec<InstallabilityErrorArgument<'a>>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CompilationCacheParams<'a> {
    /// The URL of the script to produce a compilation cache entry for.
    pub url: Cow<'a, str>,
    /// A hint to the backend whether eager compilation is recommended.
    /// (the actual compilation mode used is upon backend discretion).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eager: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts: Option<Vec<Cow<'a, str>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FileHandler<'a> {
    pub action: Cow<'a, str>,
    pub name: Cow<'a, str>,
    /// Mimic a map, name is the key, accepts is the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts: Option<Vec<FileFilter<'a>>>,
    /// Won't repeat the enums, using string for easy comparison. Same as the
    /// other enums below.
    #[serde(rename = "launchType")]
    pub launch_type: Cow<'a, str>,
}
/// The image definition used in both icon and screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ImageResource<'a> {
    /// The src field in the definition, but changing to url in favor of
    /// consistency.
    pub url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LaunchHandler<'a> {
    #[serde(rename = "clientMode")]
    pub client_mode: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolHandler<'a> {
    pub protocol: Cow<'a, str>,
    pub url: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RelatedApplication<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Cow<'a, str>>,
    pub url: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScopeExtension<'a> {
    /// Instead of using tuple, this field always returns the serialized string
    /// for easy understanding and comparison.
    pub origin: Cow<'a, str>,
    #[serde(rename = "hasOriginWildcard")]
    pub has_origin_wildcard: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Screenshot<'a> {
    pub image: ImageResource<'a>,
    #[serde(rename = "formFactor")]
    pub form_factor: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ShareTarget<'a> {
    pub action: Cow<'a, str>,
    pub method: Cow<'a, str>,
    pub enctype: Cow<'a, str>,
    /// Embed the ShareTargetParams
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileFilter<'a>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Shortcut<'a> {
    pub name: Cow<'a, str>,
    pub url: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct WebAppManifest<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "backgroundColor")]
    pub background_color: Option<Cow<'a, str>>,
    /// The extra description provided by the manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Cow<'a, str>>,
    /// The overrided display mode controlled by the user.
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayOverrides")]
    pub display_overrides: Option<Vec<Cow<'a, str>>>,
    /// The handlers to open files.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fileHandlers")]
    pub file_handlers: Option<Vec<FileHandler<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<ImageResource<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Cow<'a, str>>,
    /// TODO(crbug.com/1231886): This field is non-standard and part of a Chrome
    /// experiment. See:
    /// <https://github.com/WICG/web-app-launch/blob/main/launch_handler.md>
    #[serde(skip_serializing_if = "Option::is_none", rename = "launchHandler")]
    pub launch_handler: Option<LaunchHandler<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "preferRelatedApplications")]
    pub prefer_related_applications: Option<bool>,
    /// The handlers to open protocols.
    #[serde(skip_serializing_if = "Option::is_none", rename = "protocolHandlers")]
    pub protocol_handlers: Option<Vec<ProtocolHandler<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "relatedApplications")]
    pub related_applications: Option<Vec<RelatedApplication<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Cow<'a, str>>,
    /// Non-standard, see
    /// <https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md>
    #[serde(skip_serializing_if = "Option::is_none", rename = "scopeExtensions")]
    pub scope_extensions: Option<Vec<ScopeExtension<'a>>>,
    /// The screenshots used by chromium.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<Vec<Screenshot<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "shareTarget")]
    pub share_target: Option<ShareTarget<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "shortName")]
    pub short_name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcuts: Option<Vec<Shortcut<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startUrl")]
    pub start_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "themeColor")]
    pub theme_color: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SubApp<'a> {
    /// Display name of the sub-app.
    pub name: Cow<'a, str>,
    /// Scope of the sub-app.
    pub scope: Cow<'a, str>,
    /// Manifest id of the sub-app.
    #[serde(rename = "manifestId")]
    pub manifest_id: Cow<'a, str>,
    /// Start URL of the sub-app.
    #[serde(rename = "startUrl")]
    pub start_url: Cow<'a, str>,
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
    #[serde(rename = "EmbedderPrivilegedWebContents")]
    EmbedderPrivilegedWebContents,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheBlockingDetails<'a> {
    /// Url of the file where blockage happened. Optional because of tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// Function name where blockage happened. Optional because of anonymous functions and tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<Cow<'a, str>>,
    /// Line number in the script (0-based).
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    /// Column number in the script (0-based).
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanation<'a> {
    /// Type of the reason
    #[serde(rename = "type")]
    pub type_: BackForwardCacheNotRestoredReasonType,
    /// Not restored reason
    pub reason: BackForwardCacheNotRestoredReason,
    /// Context associated with the reason. The meaning of this context is
    /// dependent on the reason:
    /// - EmbedderExtensionSentMessageToCachedFrame: the extension ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<BackForwardCacheBlockingDetails<'a>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanationTree<'a> {
    /// URL of each frame
    pub url: Cow<'a, str>,
    /// Not restored reasons of each frame
    pub explanations: Vec<BackForwardCacheNotRestoredExplanation<'a>>,
    /// Array of children frame
    pub children: Vec<Box<BackForwardCacheNotRestoredExplanationTree<'a>>>,
}
/// Deprecated, please use addScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.addScriptToEvaluateOnLoad", response = "AddScriptToEvaluateOnLoadReturns<'a>")]
pub struct AddScriptToEvaluateOnLoadParams<'a> {
    #[serde(rename = "scriptSource")]
    pub script_source: Cow<'a, str>,
}
/// Deprecated, please use addScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnLoadReturns<'a> {
    /// Identifier of the added script.
    pub identifier: ScriptIdentifier<'a>,
}
/// Evaluates given script in every frame upon creation (before loading frame's scripts).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.addScriptToEvaluateOnNewDocument", response = "AddScriptToEvaluateOnNewDocumentReturns<'a>")]
pub struct AddScriptToEvaluateOnNewDocumentParams<'a> {
    pub source: Cow<'a, str>,
    /// If specified, creates an isolated world with the given name and evaluates given script in it.
    /// This world name will be used as the ExecutionContextDescription::name when the corresponding
    /// event is emitted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "worldName")]
    pub world_name: Option<Cow<'a, str>>,
    /// Specifies whether command line API should be available to the script, defaults
    /// to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    /// If true, runs the script immediately on existing execution contexts or worlds.
    /// Default: false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "runImmediately")]
    pub run_immediately: Option<bool>,
}
/// Evaluates given script in every frame upon creation (before loading frame's scripts).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnNewDocumentReturns<'a> {
    /// Identifier of the added script.
    pub identifier: ScriptIdentifier<'a>,
}
/// Brings page to front (activates tab).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.bringToFront")]
pub struct BringToFrontParams {

}
/// Capture page screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.captureScreenshot", response = "CaptureScreenshotReturns<'a>")]
pub struct CaptureScreenshotParams<'a> {
    /// Image compression format (defaults to png).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Cow<'a, str>>,
    /// Compression quality from range \[0..100\] (jpeg only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
    /// Capture the screenshot of a given region only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<Viewport>,
    /// Capture the screenshot from the surface, rather than the view. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromSurface")]
    pub from_surface: Option<bool>,
    /// Capture the screenshot beyond the viewport. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "captureBeyondViewport")]
    pub capture_beyond_viewport: Option<bool>,
    /// Optimize image encoding for speed, not for resulting size (defaults to false)
    #[serde(skip_serializing_if = "Option::is_none", rename = "optimizeForSpeed")]
    pub optimize_for_speed: Option<bool>,
}
/// Capture page screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CaptureScreenshotReturns<'a> {
    /// Base64-encoded image data. (Encoded as a base64 string when passed over JSON)
    pub data: Cow<'a, str>,
}
/// Returns a snapshot of the page as a string. For MHTML format, the serialization includes
/// iframes, shadow DOM, external resources, and element-inline styles.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.captureSnapshot", response = "CaptureSnapshotReturns<'a>")]
pub struct CaptureSnapshotParams<'a> {
    /// Format (defaults to mhtml).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Cow<'a, str>>,
}
/// Returns a snapshot of the page as a string. For MHTML format, the serialization includes
/// iframes, shadow DOM, external resources, and element-inline styles.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotReturns<'a> {
    /// Serialized page data.
    pub data: Cow<'a, str>,
}
/// Clears the overridden device metrics.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.clearDeviceMetricsOverride")]
pub struct ClearDeviceMetricsOverrideParams {

}
/// Clears the overridden Device Orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.clearDeviceOrientationOverride")]
pub struct ClearDeviceOrientationOverrideParams {

}
/// Clears the overridden Geolocation Position and Error.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.clearGeolocationOverride")]
pub struct ClearGeolocationOverrideParams {

}
/// Creates an isolated world for the given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.createIsolatedWorld", response = "CreateIsolatedWorldReturns")]
pub struct CreateIsolatedWorldParams<'a> {
    /// Id of the frame in which the isolated world should be created.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// An optional name which is reported in the Execution Context.
    #[serde(skip_serializing_if = "Option::is_none", rename = "worldName")]
    pub world_name: Option<Cow<'a, str>>,
    /// Whether or not universal access should be granted to the isolated world. This is a powerful
    /// option, use with caution.
    #[serde(skip_serializing_if = "Option::is_none", rename = "grantUniveralAccess")]
    pub grant_univeral_access: Option<bool>,
    /// An optional content security policy to set for the isolated world.
    /// If omitted, any existing CSP for the world will be cleared.
    /// Note that clearing or updating the CSP does not immediately affect the active
    /// context in the same document because LocalDOMWindow caches the
    /// ContentSecurityPolicy object. The change takes effect on subsequent
    /// navigations when a new window context is created.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentSecurityPolicy")]
    pub content_security_policy: Option<Cow<'a, str>>,
}
/// Creates an isolated world for the given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CreateIsolatedWorldReturns {
    /// Execution context of the isolated world.
    #[serde(rename = "executionContextId")]
    pub execution_context_id: crate::runtime::ExecutionContextId,
}
/// Deletes browser cookie with given name, domain and path.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.deleteCookie")]
pub struct DeleteCookieParams<'a> {
    /// Name of the cookie to remove.
    #[serde(rename = "cookieName")]
    pub cookie_name: Cow<'a, str>,
    /// URL to match cooke domain and path.
    pub url: Cow<'a, str>,
}
/// Disables page domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.disable")]
pub struct DisableParams {

}
/// Enables page domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.enable")]
pub struct EnableParams {
    /// If true, the 'Page.fileChooserOpened' event will be emitted regardless of the state set by
    /// 'Page.setInterceptFileChooserDialog' command (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableFileChooserOpenedEvent")]
    pub enable_file_chooser_opened_event: Option<bool>,
}
/// Gets the processed manifest for this current document.
/// This API always waits for the manifest to be loaded.
/// If manifestId is provided, and it does not match the manifest of the
/// current document, this API errors out.
/// If there is not a loaded page, this API errors out immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getAppManifest", response = "GetAppManifestReturns<'a>")]
pub struct GetAppManifestParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "manifestId")]
    pub manifest_id: Option<Cow<'a, str>>,
}
/// Gets the processed manifest for this current document.
/// This API always waits for the manifest to be loaded.
/// If manifestId is provided, and it does not match the manifest of the
/// current document, this API errors out.
/// If there is not a loaded page, this API errors out immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAppManifestReturns<'a> {
    /// Manifest location.
    pub url: Cow<'a, str>,
    pub errors: Vec<AppManifestError<'a>>,
    /// Manifest content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Cow<'a, str>>,
    /// Parsed manifest properties. Deprecated, use manifest instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed: Option<AppManifestParsedProperties<'a>>,
    pub manifest: WebAppManifest<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getInstallabilityErrors", response = "GetInstallabilityErrorsReturns<'a>")]
pub struct GetInstallabilityErrorsParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetInstallabilityErrorsReturns<'a> {
    #[serde(rename = "installabilityErrors")]
    pub installability_errors: Vec<InstallabilityError<'a>>,
}
/// Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getManifestIcons", response = "GetManifestIconsReturns<'a>")]
pub struct GetManifestIconsParams {

}
/// Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetManifestIconsReturns<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "primaryIcon")]
    pub primary_icon: Option<Cow<'a, str>>,
}
/// Returns the unique (PWA) app id, along with IWA bundle ID and parent app info.
/// Only returns values if the feature flag 'WebAppEnableManifestId' is enabled

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getAppId", response = "GetAppIdReturns<'a>")]
pub struct GetAppIdParams {

}
/// Returns the unique (PWA) app id, along with IWA bundle ID and parent app info.
/// Only returns values if the feature flag 'WebAppEnableManifestId' is enabled

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAppIdReturns<'a> {
    /// App id, either from manifest's id attribute or computed from start_url
    #[serde(skip_serializing_if = "Option::is_none", rename = "appId")]
    pub app_id: Option<Cow<'a, str>>,
    /// Recommendation for manifest's id attribute to match current id computed from start_url
    #[serde(skip_serializing_if = "Option::is_none", rename = "recommendedId")]
    pub recommended_id: Option<Cow<'a, str>>,
    /// The bundle ID for an Isolated Web App (IWA)
    #[serde(skip_serializing_if = "Option::is_none", rename = "bundleId")]
    pub bundle_id: Option<Cow<'a, str>>,
    /// The name of the parent app if this app is a Sub-App
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentAppName")]
    pub parent_app_name: Option<Cow<'a, str>>,
}
/// Returns the list of installed child Sub-Apps for the inspected parent app.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getSubApps", response = "GetSubAppsReturns<'a>")]
pub struct GetSubAppsParams {

}
/// Returns the list of installed child Sub-Apps for the inspected parent app.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAppsReturns<'a> {
    #[serde(rename = "subApps")]
    pub sub_apps: Vec<SubApp<'a>>,
}
/// Returns the list of sibling Sub-Apps sharing the same parent app if the inspected context is a Sub-App.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getSiblingSubApps", response = "GetSiblingSubAppsReturns<'a>")]
pub struct GetSiblingSubAppsParams {

}
/// Returns the list of sibling Sub-Apps sharing the same parent app if the inspected context is a Sub-App.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetSiblingSubAppsReturns<'a> {
    #[serde(rename = "subApps")]
    pub sub_apps: Vec<SubApp<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getAdScriptAncestry", response = "GetAdScriptAncestryReturns<'a>")]
pub struct GetAdScriptAncestryParams<'a> {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAdScriptAncestryReturns<'a> {
    /// The ancestry chain of ad script identifiers leading to this frame's
    /// creation, along with the root script's filterlist rule. The ancestry
    /// chain is ordered from the most immediate script (in the frame creation
    /// stack) to more distant ancestors (that created the immediately preceding
    /// script). Only sent if frame is labelled as an ad and ids are available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "adScriptAncestry")]
    pub ad_script_ancestry: Option<crate::network::AdAncestry<'a>>,
}
/// Returns present frame tree structure.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getFrameTree", response = "GetFrameTreeReturns<'a>")]
pub struct GetFrameTreeParams {

}
/// Returns present frame tree structure.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameTreeReturns<'a> {
    /// Present frame tree structure.
    #[serde(rename = "frameTree")]
    pub frame_tree: FrameTree<'a>,
}
/// Returns metrics relating to the layouting of the page, such as viewport bounds/scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getLayoutMetrics", response = "GetLayoutMetricsReturns")]
pub struct GetLayoutMetricsParams {

}
/// Returns metrics relating to the layouting of the page, such as viewport bounds/scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetLayoutMetricsReturns {
    /// Deprecated metrics relating to the layout viewport. Is in device pixels. Use 'cssLayoutViewport' instead.
    #[serde(rename = "layoutViewport")]
    pub layout_viewport: LayoutViewport,
    /// Deprecated metrics relating to the visual viewport. Is in device pixels. Use 'cssVisualViewport' instead.
    #[serde(rename = "visualViewport")]
    pub visual_viewport: VisualViewport,
    /// Deprecated size of scrollable area. Is in DP. Use 'cssContentSize' instead.
    #[serde(rename = "contentSize")]
    pub content_size: crate::dom::Rect,
    /// Metrics relating to the layout viewport in CSS pixels.
    #[serde(rename = "cssLayoutViewport")]
    pub css_layout_viewport: LayoutViewport,
    /// Metrics relating to the visual viewport in CSS pixels.
    #[serde(rename = "cssVisualViewport")]
    pub css_visual_viewport: VisualViewport,
    /// Size of scrollable area in CSS pixels.
    #[serde(rename = "cssContentSize")]
    pub css_content_size: crate::dom::Rect,
}
/// Returns navigation history for the current page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getNavigationHistory", response = "GetNavigationHistoryReturns<'a>")]
pub struct GetNavigationHistoryParams {

}
/// Returns navigation history for the current page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetNavigationHistoryReturns<'a> {
    /// Index of the current navigation history entry.
    #[serde(rename = "currentIndex")]
    pub current_index: u64,
    /// Array of navigation history entries.
    pub entries: Vec<NavigationEntry<'a>>,
}
/// Resets navigation history for the current page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.resetNavigationHistory")]
pub struct ResetNavigationHistoryParams {

}
/// Returns content of the given resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getResourceContent", response = "GetResourceContentReturns<'a>")]
pub struct GetResourceContentParams<'a> {
    /// Frame id to get resource for.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// URL of the resource to get content for.
    pub url: Cow<'a, str>,
}
/// Returns content of the given resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceContentReturns<'a> {
    /// Resource content.
    pub content: Cow<'a, str>,
    /// True, if content was served as base64.
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
/// Returns present frame / resource tree structure.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getResourceTree", response = "GetResourceTreeReturns<'a>")]
pub struct GetResourceTreeParams {

}
/// Returns present frame / resource tree structure.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceTreeReturns<'a> {
    /// Present frame / resource tree structure.
    #[serde(rename = "frameTree")]
    pub frame_tree: FrameResourceTree<'a>,
}
/// Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.handleJavaScriptDialog")]
pub struct HandleJavaScriptDialogParams<'a> {
    /// Whether to accept or dismiss the dialog.
    pub accept: bool,
    /// The text to enter into the dialog prompt before accepting. Used only if this is a prompt
    /// dialog.
    #[serde(skip_serializing_if = "Option::is_none", rename = "promptText")]
    pub prompt_text: Option<Cow<'a, str>>,
}
/// Navigates current page to the given URL.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.navigate", response = "NavigateReturns<'a>")]
pub struct NavigateParams<'a> {
    /// URL to navigate the page to.
    pub url: Cow<'a, str>,
    /// Referrer URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<Cow<'a, str>>,
    /// Intended transition type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitionType")]
    pub transition_type: Option<TransitionType>,
    /// Frame id to navigate, if not specified navigates the top frame.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<FrameId<'a>>,
    /// Referrer-policy used for the navigation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "referrerPolicy")]
    pub referrer_policy: Option<ReferrerPolicy>,
}
/// Navigates current page to the given URL.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NavigateReturns<'a> {
    /// Frame id that has navigated (or failed to navigate)
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Loader identifier. This is omitted in case of same-document navigation,
    /// as the previously committed loaderId would not change.
    #[serde(skip_serializing_if = "Option::is_none", rename = "loaderId")]
    pub loader_id: Option<crate::network::LoaderId<'a>>,
    /// User friendly error message, present if and only if navigation has failed.
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorText")]
    pub error_text: Option<Cow<'a, str>>,
    /// Whether the navigation resulted in a download.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isDownload")]
    pub is_download: Option<bool>,
}
/// Navigates current page to the given history entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.navigateToHistoryEntry")]
pub struct NavigateToHistoryEntryParams {
    /// Unique id of the entry to navigate to.
    #[serde(rename = "entryId")]
    pub entry_id: u64,
}
/// Print page as PDF.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.printToPDF", response = "PrintToPDFReturns<'a>")]
pub struct PrintToPDFParams<'a> {
    /// Paper orientation. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landscape: Option<bool>,
    /// Display header and footer. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayHeaderFooter")]
    pub display_header_footer: Option<bool>,
    /// Print background graphics. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "printBackground")]
    pub print_background: Option<bool>,
    /// Scale of the webpage rendering. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Paper width in inches. Defaults to 8.5 inches.
    #[serde(skip_serializing_if = "Option::is_none", rename = "paperWidth")]
    pub paper_width: Option<f64>,
    /// Paper height in inches. Defaults to 11 inches.
    #[serde(skip_serializing_if = "Option::is_none", rename = "paperHeight")]
    pub paper_height: Option<f64>,
    /// Top margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginTop")]
    pub margin_top: Option<f64>,
    /// Bottom margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginBottom")]
    pub margin_bottom: Option<f64>,
    /// Left margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginLeft")]
    pub margin_left: Option<f64>,
    /// Right margin in inches. Defaults to 1cm (~0.4 inches).
    #[serde(skip_serializing_if = "Option::is_none", rename = "marginRight")]
    pub margin_right: Option<f64>,
    /// Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are
    /// printed in the document order, not in the order specified, and no
    /// more than once.
    /// Defaults to empty string, which implies the entire document is printed.
    /// The page numbers are quietly capped to actual page count of the
    /// document, and ranges beyond the end of the document are ignored.
    /// If this results in no pages to print, an error is reported.
    /// It is an error to specify a range with start greater than end.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageRanges")]
    pub page_ranges: Option<Cow<'a, str>>,
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
    pub header_template: Option<Cow<'a, str>>,
    /// HTML template for the print footer. Should use the same format as the 'headerTemplate'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "footerTemplate")]
    pub footer_template: Option<Cow<'a, str>>,
    /// Whether or not to prefer page size as defined by css. Defaults to false,
    /// in which case the content will be scaled to fit the paper size.
    #[serde(skip_serializing_if = "Option::is_none", rename = "preferCSSPageSize")]
    pub prefer_css_page_size: Option<bool>,
    /// return as stream
    #[serde(skip_serializing_if = "Option::is_none", rename = "transferMode")]
    pub transfer_mode: Option<Cow<'a, str>>,
    /// Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generateTaggedPDF")]
    pub generate_tagged_pdf: Option<bool>,
    /// Whether or not to embed the document outline into the PDF.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generateDocumentOutline")]
    pub generate_document_outline: Option<bool>,
}
/// Print page as PDF.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PrintToPDFReturns<'a> {
    /// Base64-encoded pdf data. Empty if |returnAsStream| is specified. (Encoded as a base64 string when passed over JSON)
    pub data: Cow<'a, str>,
    /// A handle of the stream that holds resulting PDF data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<crate::io::StreamHandle<'a>>,
}
/// Reloads given page optionally ignoring the cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.reload")]
pub struct ReloadParams<'a> {
    /// If true, browser cache is ignored (as if the user pressed Shift+refresh).
    #[serde(skip_serializing_if = "Option::is_none", rename = "ignoreCache")]
    pub ignore_cache: Option<bool>,
    /// If set, the script will be injected into all frames of the inspected page after reload.
    /// Argument will be ignored if reloading dataURL origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptToEvaluateOnLoad")]
    pub script_to_evaluate_on_load: Option<Cow<'a, str>>,
    /// If set, an error will be thrown if the target page's main frame's
    /// loader id does not match the provided id. This prevents accidentally
    /// reloading an unintended target in case there's a racing navigation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "loaderId")]
    pub loader_id: Option<crate::network::LoaderId<'a>>,
}
/// Deprecated, please use removeScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.removeScriptToEvaluateOnLoad")]
pub struct RemoveScriptToEvaluateOnLoadParams<'a> {
    pub identifier: ScriptIdentifier<'a>,
}
/// Removes given script from the list.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.removeScriptToEvaluateOnNewDocument")]
pub struct RemoveScriptToEvaluateOnNewDocumentParams<'a> {
    pub identifier: ScriptIdentifier<'a>,
}
/// Acknowledges that a screencast frame has been received by the frontend.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.screencastFrameAck")]
pub struct ScreencastFrameAckParams {
    /// Frame number.
    #[serde(rename = "sessionId")]
    pub session_id: u64,
}
/// Searches for given string in resource content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.searchInResource", response = "SearchInResourceReturns")]
pub struct SearchInResourceParams<'a> {
    /// Frame id for resource to search in.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// URL of the resource to search in.
    pub url: Cow<'a, str>,
    /// String to search for.
    pub query: Cow<'a, str>,
    /// If true, search is case sensitive.
    #[serde(skip_serializing_if = "Option::is_none", rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    /// If true, treats string parameter as regex.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isRegex")]
    pub is_regex: Option<bool>,
}
/// Searches for given string in resource content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResourceReturns {
    /// List of search matches.
    pub result: Vec<crate::debugger::SearchMatch>,
}
/// Enable Chrome's experimental ad filter on all sites.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setAdBlockingEnabled")]
pub struct SetAdBlockingEnabledParams {
    /// Whether to block ads.
    pub enabled: bool,
}
/// Enable page Content Security Policy by-passing.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setBypassCSP")]
pub struct SetBypassCSPParams {
    /// Whether to bypass page CSP.
    pub enabled: bool,
}
/// Get Permissions Policy state on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getPermissionsPolicyState", response = "GetPermissionsPolicyStateReturns<'a>")]
pub struct GetPermissionsPolicyStateParams<'a> {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
}
/// Get Permissions Policy state on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetPermissionsPolicyStateReturns<'a> {
    pub states: Vec<PermissionsPolicyFeatureState<'a>>,
}
/// Get Origin Trials on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getOriginTrials", response = "GetOriginTrialsReturns<'a>")]
pub struct GetOriginTrialsParams<'a> {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
}
/// Get Origin Trials on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetOriginTrialsReturns<'a> {
    #[serde(rename = "originTrials")]
    pub origin_trials: Vec<OriginTrial<'a>>,
}
/// Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
/// window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
/// query results).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setDeviceMetricsOverride")]
pub struct SetDeviceMetricsOverrideParams<'a> {
    /// Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub width: u64,
    /// Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub height: i64,
    /// Overriding device scale factor value. 0 disables the override.
    #[serde(rename = "deviceScaleFactor")]
    pub device_scale_factor: f64,
    /// Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    /// autosizing and more.
    pub mobile: bool,
    /// Scale to apply to resulting view image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Overriding screen width value in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenWidth")]
    pub screen_width: Option<u64>,
    /// Overriding screen height value in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenHeight")]
    pub screen_height: Option<i64>,
    /// Overriding view X position on screen in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionX")]
    pub position_x: Option<i64>,
    /// Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionY")]
    pub position_y: Option<i64>,
    /// Do not set visible view size, rely upon explicit setVisibleSize call.
    #[serde(skip_serializing_if = "Option::is_none", rename = "dontSetVisibleSize")]
    pub dont_set_visible_size: Option<bool>,
    /// Screen orientation override.
    #[serde(skip_serializing_if = "Option::is_none", rename = "screenOrientation")]
    pub screen_orientation: Option<crate::emulation::ScreenOrientation<'a>>,
    /// The viewport dimensions and scale. If not set, the override is cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<Viewport>,
}
/// Overrides the Device Orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setDeviceOrientationOverride")]
pub struct SetDeviceOrientationOverrideParams {
    /// Mock alpha
    pub alpha: f64,
    /// Mock beta
    pub beta: f64,
    /// Mock gamma
    pub gamma: f64,
}
/// Set generic font families.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setFontFamilies")]
pub struct SetFontFamiliesParams<'a> {
    /// Specifies font families to set. If a font family is not specified, it won't be changed.
    #[serde(rename = "fontFamilies")]
    pub font_families: FontFamilies<'a>,
    /// Specifies font families to set for individual scripts.
    #[serde(skip_serializing_if = "Option::is_none", rename = "forScripts")]
    pub for_scripts: Option<Vec<ScriptFontFamilies<'a>>>,
}
/// Set default font sizes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setFontSizes")]
pub struct SetFontSizesParams {
    /// Specifies font sizes to set. If a font size is not specified, it won't be changed.
    #[serde(rename = "fontSizes")]
    pub font_sizes: FontSizes,
}
/// Sets given markup as the document's HTML.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setDocumentContent")]
pub struct SetDocumentContentParams<'a> {
    /// Frame id to set HTML for.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// HTML content to set.
    pub html: Cow<'a, str>,
}
/// Set the behavior when downloading a file.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setDownloadBehavior")]
pub struct SetDownloadBehaviorParams<'a> {
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny).
    pub behavior: Cow<'a, str>,
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'
    #[serde(skip_serializing_if = "Option::is_none", rename = "downloadPath")]
    pub download_path: Option<Cow<'a, str>>,
}
/// Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position
/// unavailable.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setGeolocationOverride")]
pub struct SetGeolocationOverrideParams {
    /// Mock latitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Mock longitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Mock accuracy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
}
/// Controls whether page will emit lifecycle events.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setLifecycleEventsEnabled")]
pub struct SetLifecycleEventsEnabledParams {
    /// If true, starts emitting lifecycle events.
    pub enabled: bool,
}
/// Toggles mouse event-based touch event emulation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setTouchEmulationEnabled")]
pub struct SetTouchEmulationEnabledParams<'a> {
    /// Whether the touch event emulation should be enabled.
    pub enabled: bool,
    /// Touch/gesture events configuration. Default: current platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Cow<'a, str>>,
}
/// Starts sending each frame using the 'screencastFrame' event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.startScreencast")]
pub struct StartScreencastParams<'a> {
    /// Image compression format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Cow<'a, str>>,
    /// Compression quality from range \[0..100\].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
    /// Maximum screenshot width.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxWidth")]
    pub max_width: Option<u64>,
    /// Maximum screenshot height.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxHeight")]
    pub max_height: Option<i64>,
    /// Send every n-th frame. Must be a positive integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "everyNthFrame")]
    pub every_nth_frame: Option<i64>,
    /// Maximum number of frames sent until screencastFrameAck is required.
    /// Defaults to 3. Must be a positive integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxFramesInFlight")]
    pub max_frames_in_flight: Option<i64>,
    /// By default, after screencastFrameAck arrives, the next produced frame is sent.
    /// Passing this flag enables storing the last produced frame in memory, which is
    /// immediately sent upon screencastFrameAck. This way, overall performance is
    /// traded for a better latency.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sendLastFrame")]
    pub send_last_frame: Option<bool>,
}
/// Starts screencast video recording.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.startScreenRecording", response = "StartScreenRecordingReturns<'a>")]
pub struct StartScreenRecordingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    /// Maximum frame width in pixels.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxWidth")]
    pub max_width: Option<u64>,
    /// Maximum frame height in pixels.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxHeight")]
    pub max_height: Option<i64>,
    /// Maximum frame rate in frames per second.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameRate")]
    pub frame_rate: Option<i64>,
}
/// Starts screencast video recording.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StartScreenRecordingReturns<'a> {
    /// A handle of the stream that holds resulting screencast data.
    pub stream: crate::io::StreamHandle<'a>,
}
/// Stops screencast video recording.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.stopScreenRecording", response = "StopScreenRecordingReturns<'a>")]
pub struct StopScreenRecordingParams {

}
/// Stops screencast video recording.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StopScreenRecordingReturns<'a> {
    /// A handle of the stream that holds resulting screencast data.
    pub stream: crate::io::StreamHandle<'a>,
}
/// Force the page stop all navigations and pending resource fetches.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.stopLoading")]
pub struct StopLoadingParams {

}
/// Crashes renderer on the IO thread, generates minidumps.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.crash")]
pub struct CrashParams {

}
/// Tries to close page, running its beforeunload hooks, if any.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.close")]
pub struct CloseParams {

}
/// Tries to update the web lifecycle state of the page.
/// It will transition the page to the given state according to:
/// <https://github.com/WICG/web-lifecycle/>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setWebLifecycleState")]
pub struct SetWebLifecycleStateParams<'a> {
    /// Target lifecycle state
    pub state: Cow<'a, str>,
}
/// Stops sending each frame in the 'screencastFrame'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.stopScreencast")]
pub struct StopScreencastParams {

}
/// Requests backend to produce compilation cache for the specified scripts.
/// 'scripts' are appended to the list of scripts for which the cache
/// would be produced. The list may be reset during page navigation.
/// When script with a matching URL is encountered, the cache is optionally
/// produced upon backend discretion, based on internal heuristics.
/// See also: 'Page.compilationCacheProduced'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.produceCompilationCache")]
pub struct ProduceCompilationCacheParams<'a> {
    pub scripts: Vec<CompilationCacheParams<'a>>,
}
/// Seeds compilation cache for given url. Compilation cache does not survive
/// cross-process navigation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.addCompilationCache")]
pub struct AddCompilationCacheParams<'a> {
    pub url: Cow<'a, str>,
    /// Base64-encoded data (Encoded as a base64 string when passed over JSON)
    pub data: Cow<'a, str>,
}
/// Clears seeded compilation cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.clearCompilationCache")]
pub struct ClearCompilationCacheParams {

}
/// Sets the Secure Payment Confirmation transaction mode.
/// <https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setSPCTransactionMode")]
pub struct SetSPCTransactionModeParams<'a> {
    pub mode: Cow<'a, str>,
}
/// Extensions for Custom Handlers API:
/// <https://html.spec.whatwg.org/multipage/system-state.html#rph-automation>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setRPHRegistrationMode")]
pub struct SetRPHRegistrationModeParams<'a> {
    pub mode: Cow<'a, str>,
}
/// Generates a report for testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.generateTestReport")]
pub struct GenerateTestReportParams<'a> {
    /// Message to be displayed in the report.
    pub message: Cow<'a, str>,
    /// Specifies the endpoint group to deliver the report to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Cow<'a, str>>,
}
/// Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.waitForDebugger")]
pub struct WaitForDebuggerParams {

}
/// Intercept file chooser requests and transfer control to protocol clients.
/// When file chooser interception is enabled, native file chooser dialog is not shown.
/// Instead, a protocol event 'Page.fileChooserOpened' is emitted.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setInterceptFileChooserDialog")]
pub struct SetInterceptFileChooserDialogParams {
    pub enabled: bool,
    /// If true, cancels the dialog by emitting relevant events (if any)
    /// in addition to not showing it if the interception is enabled
    /// (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<bool>,
}
/// Enable/disable prerendering manually.
/// 
/// This command is a short-term solution for <https://crbug.com/1440085>.
/// See <https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA>
/// for more details.
/// 
/// TODO(<https://crbug.com/1440085>): Remove this once Puppeteer supports tab targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.setPrerenderingAllowed")]
pub struct SetPrerenderingAllowedParams {
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
}
/// Get the annotated page content for the main frame.
/// This is an experimental command that is subject to change.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.getAnnotatedPageContent", response = "GetAnnotatedPageContentReturns<'a>")]
pub struct GetAnnotatedPageContentParams {
    /// Whether to include actionable information. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeActionableInformation")]
    pub include_actionable_information: Option<bool>,
}
/// Get the annotated page content for the main frame.
/// This is an experimental command that is subject to change.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnotatedPageContentReturns<'a> {
    /// The annotated page content as a base64 encoded protobuf.
    /// The format is defined by the 'AnnotatedPageContent' message in
    /// components/optimization_guide/proto/features/common_quality_data.proto (Encoded as a base64 string when passed over JSON)
    pub content: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.domContentEventFired")]
pub struct DomContentEventFired {
    pub timestamp: crate::network::MonotonicTime,
}
/// Emitted only when 'page.interceptFileChooser' is enabled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.fileChooserOpened")]
pub struct FileChooserOpened<'a> {
    /// Id of the frame containing input node.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Input mode.
    pub mode: Cow<'a, str>,
    /// Input node id. Only present for file choosers opened via an '\<input type="file"\>' element.
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
}
/// Fired when frame has been attached to its parent.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameAttached")]
pub struct FrameAttached<'a> {
    /// Id of the frame that has been attached.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Parent frame identifier.
    #[serde(rename = "parentFrameId")]
    pub parent_frame_id: FrameId<'a>,
    /// JavaScript stack trace of when frame was attached, only set if frame initiated from script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<crate::runtime::StackTrace>,
}
/// Fired when frame no longer has a scheduled navigation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameClearedScheduledNavigation")]
pub struct FrameClearedScheduledNavigation<'a> {
    /// Id of the frame that has cleared its scheduled navigation.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
}
/// Fired when frame has been detached from its parent.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameDetached")]
pub struct FrameDetached<'a> {
    /// Id of the frame that has been detached.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    pub reason: Cow<'a, str>,
}
/// Fired before frame subtree is detached. Emitted before any frame of the
/// subtree is actually detached.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameSubtreeWillBeDetached")]
pub struct FrameSubtreeWillBeDetached<'a> {
    /// Id of the frame that is the root of the subtree that will be detached.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
}
/// Fired once navigation of the frame has completed. Frame is now associated with the new loader.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameNavigated")]
pub struct FrameNavigated<'a> {
    /// Frame object.
    pub frame: Frame<'a>,
    #[serde(rename = "type")]
    pub type_: NavigationType,
}
/// Fired when opening document to write to.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.documentOpened")]
pub struct DocumentOpened<'a> {
    /// Frame object.
    pub frame: Frame<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameResized")]
pub struct FrameResized {

}
/// Fired when a navigation starts. This event is fired for both
/// renderer-initiated and browser-initiated navigations. For renderer-initiated
/// navigations, the event is fired after 'frameRequestedNavigation'.
/// Navigation may still be cancelled after the event is issued. Multiple events
/// can be fired for a single navigation, for example, when a same-document
/// navigation becomes a cross-document navigation (such as in the case of a
/// frameset).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameStartedNavigating")]
pub struct FrameStartedNavigating<'a> {
    /// ID of the frame that is being navigated.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// The URL the navigation started with. The final URL can be different.
    pub url: Cow<'a, str>,
    /// Loader identifier. Even though it is present in case of same-document
    /// navigation, the previously committed loaderId would not change unless
    /// the navigation changes from a same-document to a cross-document
    /// navigation.
    #[serde(rename = "loaderId")]
    pub loader_id: crate::network::LoaderId<'a>,
    #[serde(rename = "navigationType")]
    pub navigation_type: Cow<'a, str>,
}
/// Fired when a renderer-initiated navigation is requested.
/// Navigation may still be cancelled after the event is issued.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameRequestedNavigation")]
pub struct FrameRequestedNavigation<'a> {
    /// Id of the frame that is being navigated.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// The reason for the navigation.
    pub reason: ClientNavigationReason,
    /// The destination URL for the requested navigation.
    pub url: Cow<'a, str>,
    /// The disposition for the navigation.
    pub disposition: ClientNavigationDisposition,
}
/// Fired when frame schedules a potential navigation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameScheduledNavigation")]
pub struct FrameScheduledNavigation<'a> {
    /// Id of the frame that has scheduled a navigation.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Delay (in seconds) until the navigation is scheduled to begin. The navigation is not
    /// guaranteed to start.
    pub delay: f64,
    /// The reason for the navigation.
    pub reason: ClientNavigationReason,
    /// The destination URL for the scheduled navigation.
    pub url: Cow<'a, str>,
}
/// Fired when frame has started loading.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameStartedLoading")]
pub struct FrameStartedLoading<'a> {
    /// Id of the frame that has started loading.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
}
/// Fired when frame has stopped loading.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.frameStoppedLoading")]
pub struct FrameStoppedLoading<'a> {
    /// Id of the frame that has stopped loading.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
}
/// Fired when page is about to start a download.
/// Deprecated. Use Browser.downloadWillBegin instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.downloadWillBegin")]
pub struct DownloadWillBegin<'a> {
    /// Id of the frame that caused download to begin.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Global unique identifier of the download.
    pub guid: Cow<'a, str>,
    /// URL of the resource being downloaded.
    pub url: Cow<'a, str>,
    /// Suggested file name of the resource (the actual name of the file saved on disk may differ).
    #[serde(rename = "suggestedFilename")]
    pub suggested_filename: Cow<'a, str>,
}
/// Fired when download makes progress. Last call has |done| == true.
/// Deprecated. Use Browser.downloadProgress instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.downloadProgress")]
pub struct DownloadProgress<'a> {
    /// Global unique identifier of the download.
    pub guid: Cow<'a, str>,
    /// Total expected bytes to download.
    #[serde(rename = "totalBytes")]
    pub total_bytes: f64,
    /// Total bytes received.
    #[serde(rename = "receivedBytes")]
    pub received_bytes: f64,
    /// Download status.
    pub state: Cow<'a, str>,
}
/// Fired when interstitial page was hidden

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.interstitialHidden")]
pub struct InterstitialHidden {

}
/// Fired when interstitial page was shown

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.interstitialShown")]
pub struct InterstitialShown {

}
/// Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been
/// closed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.javascriptDialogClosed")]
pub struct JavascriptDialogClosed<'a> {
    /// Frame id.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Whether dialog was confirmed.
    pub result: bool,
    /// User input in case of prompt.
    #[serde(rename = "userInput")]
    pub user_input: Cow<'a, str>,
}
/// Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to
/// open.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.javascriptDialogOpening")]
pub struct JavascriptDialogOpening<'a> {
    /// Frame url.
    pub url: Cow<'a, str>,
    /// Frame id.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Message that will be displayed by the dialog.
    pub message: Cow<'a, str>,
    /// Dialog type.
    #[serde(rename = "type")]
    pub type_: DialogType,
    /// True iff browser is capable showing or acting on the given dialog. When browser has no
    /// dialog handler for given target, calling alert while Page domain is engaged will stall
    /// the page execution. Execution can be resumed via calling Page.handleJavaScriptDialog.
    #[serde(rename = "hasBrowserHandler")]
    pub has_browser_handler: bool,
    /// Default dialog prompt.
    #[serde(skip_serializing_if = "Option::is_none", rename = "defaultPrompt")]
    pub default_prompt: Option<Cow<'a, str>>,
}
/// Fired for lifecycle events (navigation, load, paint, etc) in the current
/// target (including local frames).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.lifecycleEvent")]
pub struct LifecycleEvent<'a> {
    /// Id of the frame.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Loader identifier. Empty string if the request is fetched from worker.
    #[serde(rename = "loaderId")]
    pub loader_id: crate::network::LoaderId<'a>,
    pub name: Cow<'a, str>,
    pub timestamp: crate::network::MonotonicTime,
}
/// Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do
/// not assume any ordering with the Page.frameNavigated event. This event is fired only for
/// main-frame history navigation where the document changes (non-same-document navigations),
/// when bfcache navigation fails.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.backForwardCacheNotUsed")]
pub struct BackForwardCacheNotUsed<'a> {
    /// The loader id for the associated navigation.
    #[serde(rename = "loaderId")]
    pub loader_id: crate::network::LoaderId<'a>,
    /// The frame id of the associated frame.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Array of reasons why the page could not be cached. This must not be empty.
    #[serde(rename = "notRestoredExplanations")]
    pub not_restored_explanations: Vec<BackForwardCacheNotRestoredExplanation<'a>>,
    /// Tree structure of reasons why the page could not be cached for each frame.
    #[serde(skip_serializing_if = "Option::is_none", rename = "notRestoredExplanationsTree")]
    pub not_restored_explanations_tree: Option<BackForwardCacheNotRestoredExplanationTree<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.loadEventFired")]
pub struct LoadEventFired {
    pub timestamp: crate::network::MonotonicTime,
}
/// Fired when same-document navigation happens, e.g. due to history API usage or anchor navigation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.navigatedWithinDocument")]
pub struct NavigatedWithinDocument<'a> {
    /// Id of the frame.
    #[serde(rename = "frameId")]
    pub frame_id: FrameId<'a>,
    /// Frame's new url.
    pub url: Cow<'a, str>,
    /// Navigation type
    #[serde(rename = "navigationType")]
    pub navigation_type: Cow<'a, str>,
}
/// Compressed image data requested by the 'startScreencast'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.screencastFrame")]
pub struct ScreencastFrame<'a> {
    /// Base64-encoded compressed image. (Encoded as a base64 string when passed over JSON)
    pub data: Cow<'a, str>,
    /// Screencast frame metadata.
    pub metadata: ScreencastFrameMetadata,
    /// Frame number.
    #[serde(rename = "sessionId")]
    pub session_id: u64,
}
/// Fired when the page with currently enabled screencast was shown or hidden '.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.screencastVisibilityChanged")]
pub struct ScreencastVisibilityChanged {
    /// True if the page is visible.
    pub visible: bool,
}
/// Fired when a new window is going to be opened, via window.open(), link click, form submission,
/// etc.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.windowOpen")]
pub struct WindowOpen<'a> {
    /// The URL for the new window.
    pub url: Cow<'a, str>,
    /// Window name.
    #[serde(rename = "windowName")]
    pub window_name: Cow<'a, str>,
    /// An array of enabled window features.
    #[serde(rename = "windowFeatures")]
    pub window_features: Vec<Cow<'a, str>>,
    /// Whether or not it was triggered by user gesture.
    #[serde(rename = "userGesture")]
    pub user_gesture: bool,
}
/// Issued for every compilation cache generated.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Page.compilationCacheProduced")]
pub struct CompilationCacheProduced<'a> {
    pub url: Cow<'a, str>,
    /// Base64-encoded data (Encoded as a base64 string when passed over JSON)
    pub data: Cow<'a, str>,
}