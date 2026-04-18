//! Actions and events related to the inspected page belong to the page domain.

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Unique frame identifier.

pub type FrameId = String;

/// Indicates whether a frame has been identified as an ad.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AdFrameType {
    #[default]
    None,
    Child,
    Root,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AdFrameExplanation {
    #[default]
    ParentIsAd,
    CreatedByAdScript,
    MatchedBlockingRule,
}

/// Indicates whether a frame has been identified as an ad and why.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdFrameStatus {

    pub adFrameType: AdFrameType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanations: Option<Vec<AdFrameExplanation>>,
}

/// Indicates whether the frame is a secure context and why it is the case.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SecureContextType {
    #[default]
    Secure,
    SecureLocalhost,
    InsecureScheme,
    InsecureAncestor,
}

/// Indicates whether the frame is cross-origin isolated and why it is the case.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CrossOriginIsolatedContextType {
    #[default]
    Isolated,
    NotIsolated,
    NotIsolatedFeatureDisabled,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GatedAPIFeatures {
    #[default]
    SharedArrayBuffers,
    SharedArrayBuffersTransferAllowed,
    PerformanceMeasureMemory,
    PerformanceProfile,
}

/// All Permissions Policy features. This enum should match the one defined
/// in services/network/public/cpp/permissions_policy/permissions_policy_features.json5.
/// LINT.IfChange(PermissionsPolicyFeature)

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionsPolicyFeature {
    #[default]
    Accelerometer,
    AllScreensCapture,
    AmbientLightSensor,
    AriaNotify,
    AttributionReporting,
    Autofill,
    Autoplay,
    Bluetooth,
    BrowsingTopics,
    Camera,
    CapturedSurfaceControl,
    ChDpr,
    ChDeviceMemory,
    ChDownlink,
    ChEct,
    ChPrefersColorScheme,
    ChPrefersReducedMotion,
    ChPrefersReducedTransparency,
    ChRtt,
    ChSaveData,
    ChUa,
    ChUaArch,
    ChUaBitness,
    ChUaHighEntropyValues,
    ChUaPlatform,
    ChUaModel,
    ChUaMobile,
    ChUaFormFactors,
    ChUaFullVersion,
    ChUaFullVersionList,
    ChUaPlatformVersion,
    ChUaWow64,
    ChViewportHeight,
    ChViewportWidth,
    ChWidth,
    ClipboardRead,
    ClipboardWrite,
    ComputePressure,
    ControlledFrame,
    CrossOriginIsolated,
    DeferredFetch,
    DeferredFetchMinimal,
    DeviceAttributes,
    DigitalCredentialsCreate,
    DigitalCredentialsGet,
    DirectSockets,
    DirectSocketsMulticast,
    DirectSocketsPrivate,
    DisplayCapture,
    DocumentDomain,
    EncryptedMedia,
    ExecutionWhileOutOfViewport,
    ExecutionWhileNotRendered,
    FocusWithoutUserActivation,
    Fullscreen,
    Frobulate,
    Gamepad,
    Geolocation,
    Gyroscope,
    Hid,
    IdentityCredentialsGet,
    IdleDetection,
    InterestCohort,
    JoinAdInterestGroup,
    KeyboardMap,
    LanguageDetector,
    LanguageModel,
    LocalFonts,
    LocalNetwork,
    LocalNetworkAccess,
    LoopbackNetwork,
    Magnetometer,
    ManualText,
    MediaPlaybackWhileNotVisible,
    Microphone,
    Midi,
    OnDeviceSpeechRecognition,
    OtpCredentials,
    Payment,
    PictureInPicture,
    PrivateAggregation,
    PrivateStateTokenIssuance,
    PrivateStateTokenRedemption,
    PublickeyCredentialsCreate,
    PublickeyCredentialsGet,
    RecordAdAuctionEvents,
    Rewriter,
    RunAdAuction,
    ScreenWakeLock,
    Serial,
    SharedStorage,
    SharedStorageSelectUrl,
    SmartCard,
    SpeakerSelection,
    StorageAccess,
    SubApps,
    Summarizer,
    SyncXhr,
    Translator,
    Unload,
    Usb,
    UsbUnrestricted,
    VerticalScroll,
    WebAppInstallation,
    WebPrinting,
    WebShare,
    WindowManagement,
    Writer,
    XrSpatialTracking,
}

/// Reason for a permissions policy feature to be disabled.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PermissionsPolicyBlockReason {
    #[default]
    Header,
    IframeAttribute,
    InFencedFrameTree,
    InIsolatedApp,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsPolicyBlockLocator {

    pub frameId: FrameId,

    pub blockReason: PermissionsPolicyBlockReason,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsPolicyFeatureState {

    pub feature: PermissionsPolicyFeature,

    pub allowed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<PermissionsPolicyBlockLocator>,
}

/// Origin Trial(<https://www.chromium.org/blink/origin-trials>) support.
/// Status for an Origin Trial token.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum OriginTrialTokenStatus {
    #[default]
    Success,
    NotSupported,
    Insecure,
    Expired,
    WrongOrigin,
    InvalidSignature,
    Malformed,
    WrongVersion,
    FeatureDisabled,
    TokenDisabled,
    FeatureDisabledForUser,
    UnknownTrial,
}

/// Status for an Origin Trial.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum OriginTrialStatus {
    #[default]
    Enabled,
    ValidTokenNotProvided,
    OSNotSupported,
    TrialNotAllowed,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum OriginTrialUsageRestriction {
    #[default]
    None,
    Subset,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrialToken {

    pub origin: String,

    pub matchSubDomains: bool,

    pub trialName: String,

    pub expiryTime: crate::network::TimeSinceEpoch,

    pub isThirdParty: bool,

    pub usageRestriction: OriginTrialUsageRestriction,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrialTokenWithStatus {

    pub rawTokenText: String,
    /// 'parsedToken' is present only when the token is extractable and
    /// parsable.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsedToken: Option<OriginTrialToken>,

    pub status: OriginTrialTokenStatus,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OriginTrial {

    pub trialName: String,

    pub status: OriginTrialStatus,

    pub tokensWithStatus: Vec<OriginTrialTokenWithStatus>,
}

/// Additional information about the frame document's security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityOriginDetails {
    /// Indicates whether the frame document's security origin is one
    /// of the local hostnames (e.g. "localhost") or IP addresses (IPv4
    /// 127.0.0.0/8 or IPv6 ::1).

    pub isLocalhost: bool,
}

/// Information about the Frame on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    /// Frame unique identifier.

    pub id: FrameId,
    /// Parent frame identifier.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentId: Option<FrameId>,
    /// Identifier of the loader associated with this frame.

    pub loaderId: crate::network::LoaderId,
    /// Frame's name as specified in the tag.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Frame document's URL without fragment.

    pub url: String,
    /// Frame document's URL fragment including the '#'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlFragment: Option<String>,
    /// Frame document's registered domain, taking the public suffixes list into account.
    /// Extracted from the Frame's url.
    /// Example URLs: <http://www.google.com/file.html> -\> "google.com"
    /// <http://a.b.co.uk/file.html>      -\> "b.co.uk"

    pub domainAndRegistry: String,
    /// Frame document's security origin.

    pub securityOrigin: String,
    /// Additional details about the frame document's security origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityOriginDetails: Option<SecurityOriginDetails>,
    /// Frame document's mimeType as determined by the browser.

    pub mimeType: String,
    /// If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreachableUrl: Option<String>,
    /// Indicates whether this frame was tagged as an ad and why.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adFrameStatus: Option<AdFrameStatus>,
    /// Indicates whether the main document is a secure context and explains why that is the case.

    pub secureContextType: SecureContextType,
    /// Indicates whether this is a cross origin isolated context.

    pub crossOriginIsolatedContextType: CrossOriginIsolatedContextType,
    /// Indicated which gated APIs / features are available.

    pub gatedAPIFeatures: Vec<GatedAPIFeatures>,
}

/// Information about the Resource on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FrameResource {
    /// Resource URL.

    pub url: String,
    /// Type of this resource.

    #[serde(rename = "type")]
    pub type_: crate::network::ResourceType,
    /// Resource mimeType as determined by the browser.

    pub mimeType: String,
    /// last-modified timestamp as reported by server.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastModified: Option<crate::network::TimeSinceEpoch>,
    /// Resource content size.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentSize: Option<f64>,
    /// True if the resource failed to load.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    /// True if the resource was canceled during loading.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<bool>,
}

/// Information about the Frame hierarchy along with their cached resources.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FrameResourceTree {
    /// Frame information for this tree item.

    pub frame: Frame,
    /// Child frames.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub childFrames: Option<Vec<FrameResourceTree>>,
    /// Information about frame resources.

    pub resources: Vec<FrameResource>,
}

/// Information about the Frame hierarchy.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FrameTree {
    /// Frame information for this tree item.

    pub frame: Frame,
    /// Child frames.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub childFrames: Option<Vec<FrameTree>>,
}

/// Unique script identifier.

pub type ScriptIdentifier = String;

/// Transition type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TransitionType {
    #[default]
    Link,
    Typed,
    AddressBar,
    AutoBookmark,
    AutoSubframe,
    ManualSubframe,
    Generated,
    AutoToplevel,
    FormSubmit,
    Reload,
    Keyword,
    KeywordGenerated,
    Other,
}

/// Navigation history entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEntry {
    /// Unique id of the navigation history entry.

    pub id: u64,
    /// URL of the navigation history entry.

    pub url: String,
    /// URL that the user typed in the url bar.

    pub userTypedURL: String,
    /// Title of the navigation history entry.

    pub title: String,
    /// Transition type.

    pub transitionType: TransitionType,
}

/// Screencast frame metadata.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameMetadata {
    /// Top offset in DIP.

    pub offsetTop: f64,
    /// Page scale factor.

    pub pageScaleFactor: f64,
    /// Device screen width in DIP.

    pub deviceWidth: f64,
    /// Device screen height in DIP.

    pub deviceHeight: f64,
    /// Position of horizontal scroll in CSS pixels.

    pub scrollOffsetX: f64,
    /// Position of vertical scroll in CSS pixels.

    pub scrollOffsetY: f64,
    /// Frame swap timestamp.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::network::TimeSinceEpoch>,
}

/// Javascript dialog type.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DialogType {
    #[default]
    Alert,
    Confirm,
    Prompt,
    Beforeunload,
}

/// Error while paring app manifest.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppManifestError {
    /// Error message.

    pub message: String,
    /// If critical, this is a non-recoverable parse error.

    pub critical: i64,
    /// Error line.

    pub line: i64,
    /// Error column.

    pub column: i64,
}

/// Parsed app manifest properties.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppManifestParsedProperties {
    /// Computed scope value

    pub scope: String,
}

/// Layout viewport position and dimensions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LayoutViewport {
    /// Horizontal offset relative to the document (CSS pixels).

    pub pageX: i64,
    /// Vertical offset relative to the document (CSS pixels).

    pub pageY: i64,
    /// Width (CSS pixels), excludes scrollbar if present.

    pub clientWidth: u64,
    /// Height (CSS pixels), excludes scrollbar if present.

    pub clientHeight: i64,
}

/// Visual viewport position, dimensions, and scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VisualViewport {
    /// Horizontal offset relative to the layout viewport (CSS pixels).

    pub offsetX: f64,
    /// Vertical offset relative to the layout viewport (CSS pixels).

    pub offsetY: f64,
    /// Horizontal offset relative to the document (CSS pixels).

    pub pageX: f64,
    /// Vertical offset relative to the document (CSS pixels).

    pub pageY: f64,
    /// Width (CSS pixels), excludes scrollbar if present.

    pub clientWidth: f64,
    /// Height (CSS pixels), excludes scrollbar if present.

    pub clientHeight: f64,
    /// Scale relative to the ideal viewport (size at width=device-width).

    pub scale: f64,
    /// Page zoom factor (CSS to device independent pixels ratio).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<f64>,
}

/// Viewport for capturing screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FontFamilies {
    /// The standard font-family.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<String>,
    /// The fixed font-family.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<String>,
    /// The serif font-family.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serif: Option<String>,
    /// The sansSerif font-family.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sansSerif: Option<String>,
    /// The cursive font-family.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursive: Option<String>,
    /// The fantasy font-family.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fantasy: Option<String>,
    /// The math font-family.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<String>,
}

/// Font families collection for a script.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScriptFontFamilies {
    /// Name of the script which these font families are defined for.

    pub script: String,
    /// Generic font families collection for the script.

    pub fontFamilies: FontFamilies,
}

/// Default font sizes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    AnchorClick,
    FormSubmissionGet,
    FormSubmissionPost,
    HttpHeaderRefresh,
    InitialFrameNavigation,
    MetaTagRefresh,
    Other,
    PageBlockInterstitial,
    Reload,
    ScriptInitiated,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ClientNavigationDisposition {
    #[default]
    CurrentTab,
    NewTab,
    NewWindow,
    Download,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityErrorArgument {
    /// Argument name (e.g. name:'minimum-icon-size-in-pixels').

    pub name: String,
    /// Argument value (e.g. value:'64').

    pub value: String,
}

/// The installability error

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityError {
    /// The error id (e.g. 'manifest-missing-suitable-icon').

    pub errorId: String,
    /// The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).

    pub errorArguments: Vec<InstallabilityErrorArgument>,
}

/// The referring-policy used for the navigation.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ReferrerPolicy {
    #[default]
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

/// Per-script compilation cache parameters for 'Page.produceCompilationCache'

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompilationCacheParams {
    /// The URL of the script to produce a compilation cache entry for.

    pub url: String,
    /// A hint to the backend whether eager compilation is recommended.
    /// (the actual compilation mode used is upon backend discretion).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eager: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts: Option<Vec<String>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileHandler {

    pub action: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<ImageResource>>,
    /// Mimic a map, name is the key, accepts is the value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts: Option<Vec<FileFilter>>,
    /// Won't repeat the enums, using string for easy comparison. Same as the
    /// other enums below.

    pub launchType: String,
}

/// The image definition used in both icon and screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageResource {
    /// The src field in the definition, but changing to url in favor of
    /// consistency.

    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchHandler {

    pub clientMode: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolHandler {

    pub protocol: String,

    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelatedApplication {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScopeExtension {
    /// Instead of using tuple, this field always returns the serialized string
    /// for easy understanding and comparison.

    pub origin: String,

    pub hasOriginWildcard: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Screenshot {

    pub image: ImageResource,

    pub formFactor: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShareTarget {

    pub action: String,

    pub method: String,

    pub enctype: String,
    /// Embed the ShareTargetParams

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileFilter>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Shortcut {

    pub name: String,

    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebAppManifest {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backgroundColor: Option<String>,
    /// The extra description provided by the manifest.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// The overrided display mode controlled by the user.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayOverrides: Option<Vec<String>>,
    /// The handlers to open files.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fileHandlers: Option<Vec<FileHandler>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<ImageResource>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// TODO(crbug.com/1231886): This field is non-standard and part of a Chrome
    /// experiment. See:
    /// <https://github.com/WICG/web-app-launch/blob/main/launch_handler.md>

    #[serde(skip_serializing_if = "Option::is_none")]
    pub launchHandler: Option<LaunchHandler>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferRelatedApplications: Option<bool>,
    /// The handlers to open protocols.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocolHandlers: Option<Vec<ProtocolHandler>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relatedApplications: Option<Vec<RelatedApplication>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Non-standard, see
    /// <https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md>

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopeExtensions: Option<Vec<ScopeExtension>>,
    /// The screenshots used by chromium.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<Vec<Screenshot>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareTarget: Option<ShareTarget>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortName: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcuts: Option<Vec<Shortcut>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub startUrl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub themeColor: Option<String>,
}

/// The type of a frameNavigated event.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum NavigationType {
    #[default]
    Navigation,
    BackForwardCacheRestore,
}

/// List of not restored reasons for back-forward cache.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BackForwardCacheNotRestoredReason {
    #[default]
    NotPrimaryMainFrame,
    BackForwardCacheDisabled,
    RelatedActiveContentsExist,
    HTTPStatusNotOK,
    SchemeNotHTTPOrHTTPS,
    Loading,
    WasGrantedMediaAccess,
    DisableForRenderFrameHostCalled,
    DomainNotAllowed,
    HTTPMethodNotGET,
    SubframeIsNavigating,
    Timeout,
    CacheLimit,
    JavaScriptExecution,
    RendererProcessKilled,
    RendererProcessCrashed,
    SchedulerTrackedFeatureUsed,
    ConflictingBrowsingInstance,
    CacheFlushed,
    ServiceWorkerVersionActivation,
    SessionRestored,
    ServiceWorkerPostMessage,
    EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
    RenderFrameHostReusedSameSite,
    RenderFrameHostReusedCrossSite,
    ServiceWorkerClaim,
    IgnoreEventAndEvict,
    HaveInnerContents,
    TimeoutPuttingInCache,
    BackForwardCacheDisabledByLowMemory,
    BackForwardCacheDisabledByCommandLine,
    NetworkRequestDatapipeDrainedAsBytesConsumer,
    NetworkRequestRedirected,
    NetworkRequestTimeout,
    NetworkExceedsBufferLimit,
    NavigationCancelledWhileRestoring,
    NotMostRecentNavigationEntry,
    BackForwardCacheDisabledForPrerender,
    UserAgentOverrideDiffers,
    ForegroundCacheLimit,
    ForwardCacheDisabled,
    BrowsingInstanceNotSwapped,
    BackForwardCacheDisabledForDelegate,
    UnloadHandlerExistsInMainFrame,
    UnloadHandlerExistsInSubFrame,
    ServiceWorkerUnregistration,
    CacheControlNoStore,
    CacheControlNoStoreCookieModified,
    CacheControlNoStoreHTTPOnlyCookieModified,
    NoResponseHead,
    Unknown,
    ActivationNavigationsDisallowedForBug1234857,
    ErrorDocument,
    FencedFramesEmbedder,
    CookieDisabled,
    HTTPAuthRequired,
    CookieFlushed,
    BroadcastChannelOnMessage,
    WebViewSettingsChanged,
    WebViewJavaScriptObjectChanged,
    WebViewMessageListenerInjected,
    WebViewSafeBrowsingAllowlistChanged,
    WebViewDocumentStartJavascriptChanged,
    WebSocket,
    WebTransport,
    WebRTC,
    MainResourceHasCacheControlNoStore,
    MainResourceHasCacheControlNoCache,
    SubresourceHasCacheControlNoStore,
    SubresourceHasCacheControlNoCache,
    ContainsPlugins,
    DocumentLoaded,
    OutstandingNetworkRequestOthers,
    RequestedMIDIPermission,
    RequestedAudioCapturePermission,
    RequestedVideoCapturePermission,
    RequestedBackForwardCacheBlockedSensors,
    RequestedBackgroundWorkPermission,
    BroadcastChannel,
    WebXR,
    SharedWorker,
    SharedWorkerMessage,
    SharedWorkerWithNoActiveClient,
    WebLocks,
    WebLocksContention,
    WebHID,
    WebBluetooth,
    WebShare,
    RequestedStorageAccessGrant,
    WebNfc,
    OutstandingNetworkRequestFetch,
    OutstandingNetworkRequestXHR,
    AppBanner,
    Printing,
    WebDatabase,
    PictureInPicture,
    SpeechRecognizer,
    IdleManager,
    PaymentManager,
    SpeechSynthesis,
    KeyboardLock,
    WebOTPService,
    OutstandingNetworkRequestDirectSocket,
    InjectedJavascript,
    InjectedStyleSheet,
    KeepaliveRequest,
    IndexedDBEvent,
    Dummy,
    JsNetworkRequestReceivedCacheControlNoStoreResource,
    WebRTCUsedWithCCNS,
    WebTransportUsedWithCCNS,
    WebSocketUsedWithCCNS,
    SmartCard,
    LiveMediaStreamTrack,
    UnloadHandler,
    ParserAborted,
    ContentSecurityHandler,
    ContentWebAuthenticationAPI,
    ContentFileChooser,
    ContentSerial,
    ContentFileSystemAccess,
    ContentMediaDevicesDispatcherHost,
    ContentWebBluetooth,
    ContentWebUSB,
    ContentMediaSessionService,
    ContentScreenReader,
    ContentDiscarded,
    EmbedderPopupBlockerTabHelper,
    EmbedderSafeBrowsingTriggeredPopupBlocker,
    EmbedderSafeBrowsingThreatDetails,
    EmbedderAppBannerManager,
    EmbedderDomDistillerViewerSource,
    EmbedderDomDistillerSelfDeletingRequestDelegate,
    EmbedderOomInterventionTabHelper,
    EmbedderOfflinePage,
    EmbedderChromePasswordManagerClientBindCredentialManager,
    EmbedderPermissionRequestManager,
    EmbedderModalDialog,
    EmbedderExtensions,
    EmbedderExtensionMessaging,
    EmbedderExtensionMessagingForOpenPort,
    EmbedderExtensionSentMessageToCachedFrame,
    RequestedByWebViewClient,
    PostMessageByWebViewClient,
    CacheControlNoStoreDeviceBoundSessionTerminated,
    CacheLimitPrunedOnModerateMemoryPressure,
    CacheLimitPrunedOnCriticalMemoryPressure,
}

/// Types of not restored reasons for back-forward cache.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum BackForwardCacheNotRestoredReasonType {
    #[default]
    SupportPending,
    PageSupportNeeded,
    Circumstantial,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheBlockingDetails {
    /// Url of the file where blockage happened. Optional because of tests.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Function name where blockage happened. Optional because of anonymous functions and tests.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// Line number in the script (0-based).

    pub lineNumber: i64,
    /// Column number in the script (0-based).

    pub columnNumber: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanation {
    /// Type of the reason

    #[serde(rename = "type")]
    pub type_: BackForwardCacheNotRestoredReasonType,
    /// Not restored reason

    pub reason: BackForwardCacheNotRestoredReason,
    /// Context associated with the reason. The meaning of this context is
    /// dependent on the reason:
    /// - EmbedderExtensionSentMessageToCachedFrame: the extension ID.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<BackForwardCacheBlockingDetails>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanationTree {
    /// URL of each frame

    pub url: String,
    /// Not restored reasons of each frame

    pub explanations: Vec<BackForwardCacheNotRestoredExplanation>,
    /// Array of children frame

    pub children: Vec<BackForwardCacheNotRestoredExplanationTree>,
}

/// Deprecated, please use addScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnLoadParams {

    pub scriptSource: String,
}

/// Deprecated, please use addScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnLoadReturns {
    /// Identifier of the added script.

    pub identifier: ScriptIdentifier,
}

/// Evaluates given script in every frame upon creation (before loading frame's scripts).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnNewDocumentParams {

    pub source: String,
    /// If specified, creates an isolated world with the given name and evaluates given script in it.
    /// This world name will be used as the ExecutionContextDescription::name when the corresponding
    /// event is emitted.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worldName: Option<String>,
    /// Specifies whether command line API should be available to the script, defaults
    /// to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeCommandLineAPI: Option<bool>,
    /// If true, runs the script immediately on existing execution contexts or worlds.
    /// Default: false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub runImmediately: Option<bool>,
}

/// Evaluates given script in every frame upon creation (before loading frame's scripts).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnNewDocumentReturns {
    /// Identifier of the added script.

    pub identifier: ScriptIdentifier,
}

/// Capture page screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureScreenshotParams {
    /// Image compression format (defaults to png).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Compression quality from range \[0..100\] (jpeg only).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
    /// Capture the screenshot of a given region only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<Viewport>,
    /// Capture the screenshot from the surface, rather than the view. Defaults to true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromSurface: Option<bool>,
    /// Capture the screenshot beyond the viewport. Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub captureBeyondViewport: Option<bool>,
    /// Optimize image encoding for speed, not for resulting size (defaults to false)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizeForSpeed: Option<bool>,
}

/// Capture page screenshot.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureScreenshotReturns {
    /// Base64-encoded image data. (Encoded as a base64 string when passed over JSON)

    pub data: String,
}

/// Returns a snapshot of the page as a string. For MHTML format, the serialization includes
/// iframes, shadow DOM, external resources, and element-inline styles.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotParams {
    /// Format (defaults to mhtml).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Returns a snapshot of the page as a string. For MHTML format, the serialization includes
/// iframes, shadow DOM, external resources, and element-inline styles.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotReturns {
    /// Serialized page data.

    pub data: String,
}

/// Creates an isolated world for the given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateIsolatedWorldParams {
    /// Id of the frame in which the isolated world should be created.

    pub frameId: FrameId,
    /// An optional name which is reported in the Execution Context.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worldName: Option<String>,
    /// Whether or not universal access should be granted to the isolated world. This is a powerful
    /// option, use with caution.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantUniveralAccess: Option<bool>,
}

/// Creates an isolated world for the given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateIsolatedWorldReturns {
    /// Execution context of the isolated world.

    pub executionContextId: crate::runtime::ExecutionContextId,
}

/// Deletes browser cookie with given name, domain and path.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCookieParams {
    /// Name of the cookie to remove.

    pub cookieName: String,
    /// URL to match cooke domain and path.

    pub url: String,
}

/// Enables page domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// If true, the 'Page.fileChooserOpened' event will be emitted regardless of the state set by
    /// 'Page.setInterceptFileChooserDialog' command (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enableFileChooserOpenedEvent: Option<bool>,
}

/// Gets the processed manifest for this current document.
/// This API always waits for the manifest to be loaded.
/// If manifestId is provided, and it does not match the manifest of the
/// current document, this API errors out.
/// If there is not a loaded page, this API errors out immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAppManifestParams {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifestId: Option<String>,
}

/// Gets the processed manifest for this current document.
/// This API always waits for the manifest to be loaded.
/// If manifestId is provided, and it does not match the manifest of the
/// current document, this API errors out.
/// If there is not a loaded page, this API errors out immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAppManifestReturns {
    /// Manifest location.

    pub url: String,

    pub errors: Vec<AppManifestError>,
    /// Manifest content.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Parsed manifest properties. Deprecated, use manifest instead.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed: Option<AppManifestParsedProperties>,

    pub manifest: WebAppManifest,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstallabilityErrorsReturns {

    pub installabilityErrors: Vec<InstallabilityError>,
}

/// Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetManifestIconsReturns {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primaryIcon: Option<String>,
}

/// Returns the unique (PWA) app id.
/// Only returns values if the feature flag 'WebAppEnableManifestId' is enabled

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAppIdReturns {
    /// App id, either from manifest's id attribute or computed from start_url

    #[serde(skip_serializing_if = "Option::is_none")]
    pub appId: Option<String>,
    /// Recommendation for manifest's id attribute to match current id computed from start_url

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendedId: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAdScriptAncestryParams {

    pub frameId: FrameId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAdScriptAncestryReturns {
    /// The ancestry chain of ad script identifiers leading to this frame's
    /// creation, along with the root script's filterlist rule. The ancestry
    /// chain is ordered from the most immediate script (in the frame creation
    /// stack) to more distant ancestors (that created the immediately preceding
    /// script). Only sent if frame is labelled as an ad and ids are available.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adScriptAncestry: Option<crate::network::AdAncestry>,
}

/// Returns present frame tree structure.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameTreeReturns {
    /// Present frame tree structure.

    pub frameTree: FrameTree,
}

/// Returns metrics relating to the layouting of the page, such as viewport bounds/scale.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLayoutMetricsReturns {
    /// Deprecated metrics relating to the layout viewport. Is in device pixels. Use 'cssLayoutViewport' instead.

    pub layoutViewport: LayoutViewport,
    /// Deprecated metrics relating to the visual viewport. Is in device pixels. Use 'cssVisualViewport' instead.

    pub visualViewport: VisualViewport,
    /// Deprecated size of scrollable area. Is in DP. Use 'cssContentSize' instead.

    pub contentSize: crate::dom::Rect,
    /// Metrics relating to the layout viewport in CSS pixels.

    pub cssLayoutViewport: LayoutViewport,
    /// Metrics relating to the visual viewport in CSS pixels.

    pub cssVisualViewport: VisualViewport,
    /// Size of scrollable area in CSS pixels.

    pub cssContentSize: crate::dom::Rect,
}

/// Returns navigation history for the current page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNavigationHistoryReturns {
    /// Index of the current navigation history entry.

    pub currentIndex: u64,
    /// Array of navigation history entries.

    pub entries: Vec<NavigationEntry>,
}

/// Returns content of the given resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceContentParams {
    /// Frame id to get resource for.

    pub frameId: FrameId,
    /// URL of the resource to get content for.

    pub url: String,
}

/// Returns content of the given resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceContentReturns {
    /// Resource content.

    pub content: String,
    /// True, if content was served as base64.

    pub base64Encoded: bool,
}

/// Returns present frame / resource tree structure.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceTreeReturns {
    /// Present frame / resource tree structure.

    pub frameTree: FrameResourceTree,
}

/// Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HandleJavaScriptDialogParams {
    /// Whether to accept or dismiss the dialog.

    pub accept: bool,
    /// The text to enter into the dialog prompt before accepting. Used only if this is a prompt
    /// dialog.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptText: Option<String>,
}

/// Navigates current page to the given URL.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigateParams {
    /// URL to navigate the page to.

    pub url: String,
    /// Referrer URL.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    /// Intended transition type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitionType: Option<TransitionType>,
    /// Frame id to navigate, if not specified navigates the top frame.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<FrameId>,
    /// Referrer-policy used for the navigation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrerPolicy: Option<ReferrerPolicy>,
}

/// Navigates current page to the given URL.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigateReturns {
    /// Frame id that has navigated (or failed to navigate)

    pub frameId: FrameId,
    /// Loader identifier. This is omitted in case of same-document navigation,
    /// as the previously committed loaderId would not change.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaderId: Option<crate::network::LoaderId>,
    /// User friendly error message, present if and only if navigation has failed.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorText: Option<String>,
    /// Whether the navigation resulted in a download.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isDownload: Option<bool>,
}

/// Navigates current page to the given history entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigateToHistoryEntryParams {
    /// Unique id of the entry to navigate to.

    pub entryId: u64,
}

/// Print page as PDF.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrintToPDFParams {
    /// Paper orientation. Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub landscape: Option<bool>,
    /// Display header and footer. Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayHeaderFooter: Option<bool>,
    /// Print background graphics. Defaults to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub printBackground: Option<bool>,
    /// Scale of the webpage rendering. Defaults to 1.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Paper width in inches. Defaults to 8.5 inches.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paperWidth: Option<f64>,
    /// Paper height in inches. Defaults to 11 inches.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paperHeight: Option<f64>,
    /// Top margin in inches. Defaults to 1cm (~0.4 inches).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub marginTop: Option<f64>,
    /// Bottom margin in inches. Defaults to 1cm (~0.4 inches).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub marginBottom: Option<f64>,
    /// Left margin in inches. Defaults to 1cm (~0.4 inches).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub marginLeft: Option<f64>,
    /// Right margin in inches. Defaults to 1cm (~0.4 inches).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub marginRight: Option<f64>,
    /// Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are
    /// printed in the document order, not in the order specified, and no
    /// more than once.
    /// Defaults to empty string, which implies the entire document is printed.
    /// The page numbers are quietly capped to actual page count of the
    /// document, and ranges beyond the end of the document are ignored.
    /// If this results in no pages to print, an error is reported.
    /// It is an error to specify a range with start greater than end.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageRanges: Option<String>,
    /// HTML template for the print header. Should be valid HTML markup with following
    /// classes used to inject printing values into them:
    /// - 'date': formatted print date
    /// - 'title': document title
    /// - 'url': document location
    /// - 'pageNumber': current page number
    /// - 'totalPages': total pages in the document
    /// 
    /// For example, '\<span class=title\>\</span\>' would generate span containing the title.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headerTemplate: Option<String>,
    /// HTML template for the print footer. Should use the same format as the 'headerTemplate'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub footerTemplate: Option<String>,
    /// Whether or not to prefer page size as defined by css. Defaults to false,
    /// in which case the content will be scaled to fit the paper size.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferCSSPageSize: Option<bool>,
    /// return as stream

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferMode: Option<String>,
    /// Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generateTaggedPDF: Option<bool>,
    /// Whether or not to embed the document outline into the PDF.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generateDocumentOutline: Option<bool>,
}

/// Print page as PDF.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrintToPDFReturns {
    /// Base64-encoded pdf data. Empty if |returnAsStream| is specified. (Encoded as a base64 string when passed over JSON)

    pub data: String,
    /// A handle of the stream that holds resulting PDF data.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<crate::io::StreamHandle>,
}

/// Reloads given page optionally ignoring the cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReloadParams {
    /// If true, browser cache is ignored (as if the user pressed Shift+refresh).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignoreCache: Option<bool>,
    /// If set, the script will be injected into all frames of the inspected page after reload.
    /// Argument will be ignored if reloading dataURL origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriptToEvaluateOnLoad: Option<String>,
    /// If set, an error will be thrown if the target page's main frame's
    /// loader id does not match the provided id. This prevents accidentally
    /// reloading an unintended target in case there's a racing navigation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaderId: Option<crate::network::LoaderId>,
}

/// Deprecated, please use removeScriptToEvaluateOnNewDocument instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScriptToEvaluateOnLoadParams {

    pub identifier: ScriptIdentifier,
}

/// Removes given script from the list.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScriptToEvaluateOnNewDocumentParams {

    pub identifier: ScriptIdentifier,
}

/// Acknowledges that a screencast frame has been received by the frontend.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameAckParams {
    /// Frame number.

    pub sessionId: u64,
}

/// Searches for given string in resource content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResourceParams {
    /// Frame id for resource to search in.

    pub frameId: FrameId,
    /// URL of the resource to search in.

    pub url: String,
    /// String to search for.

    pub query: String,
    /// If true, search is case sensitive.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caseSensitive: Option<bool>,
    /// If true, treats string parameter as regex.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isRegex: Option<bool>,
}

/// Searches for given string in resource content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResourceReturns {
    /// List of search matches.

    pub result: Vec<crate::debugger::SearchMatch>,
}

/// Enable Chrome's experimental ad filter on all sites.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAdBlockingEnabledParams {
    /// Whether to block ads.

    pub enabled: bool,
}

/// Enable page Content Security Policy by-passing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBypassCSPParams {
    /// Whether to bypass page CSP.

    pub enabled: bool,
}

/// Get Permissions Policy state on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPermissionsPolicyStateParams {

    pub frameId: FrameId,
}

/// Get Permissions Policy state on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPermissionsPolicyStateReturns {

    pub states: Vec<PermissionsPolicyFeatureState>,
}

/// Get Origin Trials on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOriginTrialsParams {

    pub frameId: FrameId,
}

/// Get Origin Trials on given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOriginTrialsReturns {

    pub originTrials: Vec<OriginTrial>,
}

/// Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
/// window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
/// query results).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceMetricsOverrideParams {
    /// Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.

    pub width: u64,
    /// Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.

    pub height: i64,
    /// Overriding device scale factor value. 0 disables the override.

    pub deviceScaleFactor: f64,
    /// Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    /// autosizing and more.

    pub mobile: bool,
    /// Scale to apply to resulting view image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Overriding screen width value in pixels (minimum 0, maximum 10000000).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenWidth: Option<u64>,
    /// Overriding screen height value in pixels (minimum 0, maximum 10000000).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenHeight: Option<i64>,
    /// Overriding view X position on screen in pixels (minimum 0, maximum 10000000).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub positionX: Option<i64>,
    /// Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub positionY: Option<i64>,
    /// Do not set visible view size, rely upon explicit setVisibleSize call.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dontSetVisibleSize: Option<bool>,
    /// Screen orientation override.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenOrientation: Option<crate::emulation::ScreenOrientation>,
    /// The viewport dimensions and scale. If not set, the override is cleared.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<Viewport>,
}

/// Overrides the Device Orientation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceOrientationOverrideParams {
    /// Mock alpha

    pub alpha: f64,
    /// Mock beta

    pub beta: f64,
    /// Mock gamma

    pub gamma: f64,
}

/// Set generic font families.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFontFamiliesParams {
    /// Specifies font families to set. If a font family is not specified, it won't be changed.

    pub fontFamilies: FontFamilies,
    /// Specifies font families to set for individual scripts.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forScripts: Option<Vec<ScriptFontFamilies>>,
}

/// Set default font sizes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFontSizesParams {
    /// Specifies font sizes to set. If a font size is not specified, it won't be changed.

    pub fontSizes: FontSizes,
}

/// Sets given markup as the document's HTML.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentContentParams {
    /// Frame id to set HTML for.

    pub frameId: FrameId,
    /// HTML content to set.

    pub html: String,
}

/// Set the behavior when downloading a file.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDownloadBehaviorParams {
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny).

    pub behavior: String,
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'

    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloadPath: Option<String>,
}

/// Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position
/// unavailable.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetLifecycleEventsEnabledParams {
    /// If true, starts emitting lifecycle events.

    pub enabled: bool,
}

/// Toggles mouse event-based touch event emulation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTouchEmulationEnabledParams {
    /// Whether the touch event emulation should be enabled.

    pub enabled: bool,
    /// Touch/gesture events configuration. Default: current platform.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
}

/// Starts sending each frame using the 'screencastFrame' event.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartScreencastParams {
    /// Image compression format.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Compression quality from range \[0..100\].

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
    /// Maximum screenshot width.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxWidth: Option<u64>,
    /// Maximum screenshot height.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxHeight: Option<i64>,
    /// Send every n-th frame.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub everyNthFrame: Option<i64>,
}

/// Tries to update the web lifecycle state of the page.
/// It will transition the page to the given state according to:
/// <https://github.com/WICG/web-lifecycle/>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetWebLifecycleStateParams {
    /// Target lifecycle state

    pub state: String,
}

/// Requests backend to produce compilation cache for the specified scripts.
/// 'scripts' are appended to the list of scripts for which the cache
/// would be produced. The list may be reset during page navigation.
/// When script with a matching URL is encountered, the cache is optionally
/// produced upon backend discretion, based on internal heuristics.
/// See also: 'Page.compilationCacheProduced'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProduceCompilationCacheParams {

    pub scripts: Vec<CompilationCacheParams>,
}

/// Seeds compilation cache for given url. Compilation cache does not survive
/// cross-process navigation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddCompilationCacheParams {

    pub url: String,
    /// Base64-encoded data (Encoded as a base64 string when passed over JSON)

    pub data: String,
}

/// Sets the Secure Payment Confirmation transaction mode.
/// <https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSPCTransactionModeParams {

    pub mode: String,
}

/// Extensions for Custom Handlers API:
/// <https://html.spec.whatwg.org/multipage/system-state.html#rph-automation>

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRPHRegistrationModeParams {

    pub mode: String,
}

/// Generates a report for testing.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTestReportParams {
    /// Message to be displayed in the report.

    pub message: String,
    /// Specifies the endpoint group to deliver the report to.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// Intercept file chooser requests and transfer control to protocol clients.
/// When file chooser interception is enabled, native file chooser dialog is not shown.
/// Instead, a protocol event 'Page.fileChooserOpened' is emitted.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
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
/// This command is a short-term solution for <https://crbug.com/1440085.>
/// See <https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA>
/// for more details.
/// 
/// TODO(<https://crbug.com/1440085>): Remove this once Puppeteer supports tab targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPrerenderingAllowedParams {

    pub isAllowed: bool,
}

/// Get the annotated page content for the main frame.
/// This is an experimental command that is subject to change.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnotatedPageContentParams {
    /// Whether to include actionable information. Defaults to true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeActionableInformation: Option<bool>,
}

/// Get the annotated page content for the main frame.
/// This is an experimental command that is subject to change.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnotatedPageContentReturns {
    /// The annotated page content as a base64 encoded protobuf.
    /// The format is defined by the 'AnnotatedPageContent' message in
    /// components/optimization_guide/proto/features/common_quality_data.proto (Encoded as a base64 string when passed over JSON)

    pub content: String,
}
