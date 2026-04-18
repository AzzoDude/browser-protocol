use serde::{Serialize, Deserialize};

/// Unique id

pub type RuleSetId = String;

/// Corresponds to SpeculationRuleSet

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuleSet {

    pub id: RuleSetId,
    /// Identifies a document which the rule set is associated with.

    pub loaderId: crate::network::LoaderId,
    /// Source text of JSON representing the rule set. If it comes from
    /// '\<script\>' tag, it is the textContent of the node. Note that it is
    /// a JSON for valid case.
    /// 
    /// See also:
    /// - <https://wicg.github.io/nav-speculation/speculation-rules.html>
    /// - <https://github.com/WICG/nav-speculation/blob/main/triggers.md>

    pub sourceText: String,
    /// A speculation rule set is either added through an inline
    /// '\<script\>' tag or through an external resource via the
    /// 'Speculation-Rules' HTTP header. For the first case, we include
    /// the BackendNodeId of the relevant '\<script\>' tag. For the second
    /// case, we include the external URL where the rule set was loaded
    /// from, and also RequestId if Network domain is enabled.
    /// 
    /// See also:
    /// - <https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-script>
    /// - <https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-header>

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backendNodeId: Option<crate::dom::BackendNodeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestId: Option<crate::network::RequestId>,
    /// Error information
    /// 'errorMessage' is null iff 'errorType' is null.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorType: Option<RuleSetErrorType>,
    /// TODO(<https://crbug.com/1425354>): Replace this property with structured error.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorMessage: Option<String>,
    /// For more details, see:
    /// <https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md>

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum RuleSetErrorType {
    #[default]
    SourceIsNotJsonObject,
    InvalidRulesSkipped,
    InvalidRulesetLevelTag,
}

/// The type of preloading attempted. It corresponds to
/// mojom::SpeculationAction (although PrefetchWithSubresources is omitted as it
/// isn't being used by clients).

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SpeculationAction {
    #[default]
    Prefetch,
    Prerender,
    PrerenderUntilScript,
}

/// Corresponds to mojom::SpeculationTargetHint.
/// See <https://github.com/WICG/nav-speculation/blob/main/triggers.md#window-name-targeting-hints>

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SpeculationTargetHint {
    #[default]
    Blank,
    SelfValue,
}

/// A key that identifies a preloading attempt.
/// 
/// The url used is the url specified by the trigger (i.e. the initial URL), and
/// not the final url that is navigated to. For example, prerendering allows
/// same-origin main frame navigations during the attempt, but the attempt is
/// still keyed with the initial URL.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreloadingAttemptKey {

    pub loaderId: crate::network::LoaderId,

    pub action: SpeculationAction,

    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub formSubmission: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetHint: Option<SpeculationTargetHint>,
}

/// Lists sources for a preloading attempt, specifically the ids of rule sets
/// that had a speculation rule that triggered the attempt, and the
/// BackendNodeIds of \<a href\> or \<area href\> elements that triggered the
/// attempt (in the case of attempts triggered by a document rule). It is
/// possible for multiple rule sets and links to trigger a single attempt.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreloadingAttemptSource {

    pub key: PreloadingAttemptKey,

    pub ruleSetIds: Vec<RuleSetId>,

    pub nodeIds: Vec<crate::dom::BackendNodeId>,
}

/// Chrome manages different types of preloads together using a
/// concept of preloading pipeline. For example, if a site uses a
/// SpeculationRules for prerender, Chrome first starts a prefetch and
/// then upgrades it to prerender.
/// 
/// CDP events for them are emitted separately but they share
/// 'PreloadPipelineId'.

pub type PreloadPipelineId = String;

/// List of FinalStatus reasons for Prerender2.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PrerenderFinalStatus {
    #[default]
    Activated,
    Destroyed,
    LowEndDevice,
    InvalidSchemeRedirect,
    InvalidSchemeNavigation,
    NavigationRequestBlockedByCsp,
    MojoBinderPolicy,
    RendererProcessCrashed,
    RendererProcessKilled,
    Download,
    TriggerDestroyed,
    NavigationNotCommitted,
    NavigationBadHttpStatus,
    ClientCertRequested,
    NavigationRequestNetworkError,
    CancelAllHostsForTesting,
    DidFailLoad,
    Stop,
    SslCertificateError,
    LoginAuthRequested,
    UaChangeRequiresReload,
    BlockedByClient,
    AudioOutputDeviceRequested,
    MixedContent,
    TriggerBackgrounded,
    MemoryLimitExceeded,
    DataSaverEnabled,
    TriggerUrlHasEffectiveUrl,
    ActivatedBeforeStarted,
    InactivePageRestriction,
    StartFailed,
    TimeoutBackgrounded,
    CrossSiteRedirectInInitialNavigation,
    CrossSiteNavigationInInitialNavigation,
    SameSiteCrossOriginRedirectNotOptInInInitialNavigation,
    SameSiteCrossOriginNavigationNotOptInInInitialNavigation,
    ActivationNavigationParameterMismatch,
    ActivatedInBackground,
    EmbedderHostDisallowed,
    ActivationNavigationDestroyedBeforeSuccess,
    TabClosedByUserGesture,
    TabClosedWithoutUserGesture,
    PrimaryMainFrameRendererProcessCrashed,
    PrimaryMainFrameRendererProcessKilled,
    ActivationFramePolicyNotCompatible,
    PreloadingDisabled,
    BatterySaverEnabled,
    ActivatedDuringMainFrameNavigation,
    PreloadingUnsupportedByWebContents,
    CrossSiteRedirectInMainFrameNavigation,
    CrossSiteNavigationInMainFrameNavigation,
    SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation,
    SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation,
    MemoryPressureOnTrigger,
    MemoryPressureAfterTriggered,
    PrerenderingDisabledByDevTools,
    SpeculationRuleRemoved,
    ActivatedWithAuxiliaryBrowsingContexts,
    MaxNumOfRunningEagerPrerendersExceeded,
    MaxNumOfRunningNonEagerPrerendersExceeded,
    MaxNumOfRunningEmbedderPrerendersExceeded,
    PrerenderingUrlHasEffectiveUrl,
    RedirectedPrerenderingUrlHasEffectiveUrl,
    ActivationUrlHasEffectiveUrl,
    JavaScriptInterfaceAdded,
    JavaScriptInterfaceRemoved,
    AllPrerenderingCanceled,
    WindowClosed,
    SlowNetwork,
    OtherPrerenderedPageActivated,
    V8OptimizerDisabled,
    PrerenderFailedDuringPrefetch,
    BrowsingDataRemoved,
    PrerenderHostReused,
    FormSubmitWhenPrerendering,
}

/// Preloading status values, see also PreloadingTriggeringOutcome. This
/// status is shared by prefetchStatusUpdated and prerenderStatusUpdated.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PreloadingStatus {
    #[default]
    Pending,
    Running,
    Ready,
    Success,
    Failure,
    NotSupported,
}

/// TODO(<https://crbug.com/1384419>): revisit the list of PrefetchStatus and
/// filter out the ones that aren't necessary to the developers.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PrefetchStatus {
    #[default]
    PrefetchAllowed,
    PrefetchFailedIneligibleRedirect,
    PrefetchFailedInvalidRedirect,
    PrefetchFailedMIMENotSupported,
    PrefetchFailedNetError,
    PrefetchFailedNon2XX,
    PrefetchEvictedAfterBrowsingDataRemoved,
    PrefetchEvictedAfterCandidateRemoved,
    PrefetchEvictedForNewerPrefetch,
    PrefetchHeldback,
    PrefetchIneligibleRetryAfter,
    PrefetchIsPrivacyDecoy,
    PrefetchIsStale,
    PrefetchNotEligibleBrowserContextOffTheRecord,
    PrefetchNotEligibleDataSaverEnabled,
    PrefetchNotEligibleExistingProxy,
    PrefetchNotEligibleHostIsNonUnique,
    PrefetchNotEligibleNonDefaultStoragePartition,
    PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy,
    PrefetchNotEligibleSchemeIsNotHttps,
    PrefetchNotEligibleUserHasCookies,
    PrefetchNotEligibleUserHasServiceWorker,
    PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler,
    PrefetchNotEligibleRedirectFromServiceWorker,
    PrefetchNotEligibleRedirectToServiceWorker,
    PrefetchNotEligibleBatterySaverEnabled,
    PrefetchNotEligiblePreloadingDisabled,
    PrefetchNotFinishedInTime,
    PrefetchNotStarted,
    PrefetchNotUsedCookiesChanged,
    PrefetchProxyNotAvailable,
    PrefetchResponseUsed,
    PrefetchSuccessfulButNotUsed,
    PrefetchNotUsedProbeFailed,
}

/// Information of headers to be displayed when the header mismatch occurred.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrerenderMismatchedHeaders {

    pub headerName: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialValue: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub activationValue: Option<String>,
}
