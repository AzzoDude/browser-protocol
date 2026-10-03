use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Unique id

pub type RuleSetId<'a> = Cow<'a, str>;

/// Corresponds to SpeculationRuleSet

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RuleSet<'a> {
    pub id: RuleSetId<'a>,
    /// Identifies a document which the rule set is associated with.
    #[serde(rename = "loaderId")]
    pub loader_id: crate::network::LoaderId<'a>,
    /// Source text of JSON representing the rule set. If it comes from
    /// '\<script\>' tag, it is the textContent of the node. Note that it is
    /// a JSON for valid case.
    /// 
    /// See also:
    /// - <https://wicg.github.io/nav-speculation/speculation-rules.html>
    /// - <https://github.com/WICG/nav-speculation/blob/main/triggers.md>
    #[serde(rename = "sourceText")]
    pub source_text: Cow<'a, str>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "backendNodeId")]
    pub backend_node_id: Option<crate::dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestId")]
    pub request_id: Option<crate::network::RequestId<'a>>,
    /// Error information
    /// 'errorMessage' is null iff 'errorType' is null.
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorType")]
    pub error_type: Option<RuleSetErrorType>,
    /// TODO(<https://crbug.com/1425354>): Replace this property with structured error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorMessage")]
    pub error_message: Option<Cow<'a, str>>,
    /// For more details, see:
    /// <https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum RuleSetErrorType {
    #[default]
    #[serde(rename = "SourceIsNotJsonObject")]
    SourceIsNotJsonObject,
    #[serde(rename = "InvalidRulesSkipped")]
    InvalidRulesSkipped,
    #[serde(rename = "InvalidRulesetLevelTag")]
    InvalidRulesetLevelTag,
}

/// The type of preloading attempted. It corresponds to
/// mojom::SpeculationAction (although PrefetchWithSubresources is omitted as it
/// isn't being used by clients).

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SpeculationAction {
    #[default]
    #[serde(rename = "Prefetch")]
    Prefetch,
    #[serde(rename = "Prerender")]
    Prerender,
    #[serde(rename = "PrerenderUntilScript")]
    PrerenderUntilScript,
}

/// Corresponds to mojom::SpeculationTargetHint.
/// See <https://github.com/WICG/nav-speculation/blob/main/triggers.md#window-name-targeting-hints>

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SpeculationTargetHint {
    #[default]
    #[serde(rename = "Blank")]
    Blank,
    #[serde(rename = "Self")]
    SelfValue,
}

/// A key that identifies a preloading attempt.
/// 
/// The url used is the url specified by the trigger (i.e. the initial URL), and
/// not the final url that is navigated to. For example, prerendering allows
/// same-origin main frame navigations during the attempt, but the attempt is
/// still keyed with the initial URL.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PreloadingAttemptKey<'a> {
    #[serde(rename = "loaderId")]
    pub loader_id: crate::network::LoaderId<'a>,
    pub action: SpeculationAction,
    pub url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "formSubmission")]
    pub form_submission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetHint")]
    pub target_hint: Option<SpeculationTargetHint>,
}
/// Lists sources for a preloading attempt, specifically the ids of rule sets
/// that had a speculation rule that triggered the attempt, and the
/// BackendNodeIds of \<a href\> or \<area href\> elements that triggered the
/// attempt (in the case of attempts triggered by a document rule). It is
/// possible for multiple rule sets and links to trigger a single attempt.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PreloadingAttemptSource<'a> {
    pub key: PreloadingAttemptKey<'a>,
    #[serde(rename = "ruleSetIds")]
    pub rule_set_ids: Vec<RuleSetId<'a>>,
    #[serde(rename = "nodeIds")]
    pub node_ids: Vec<crate::dom::BackendNodeId>,
}
/// Chrome manages different types of preloads together using a
/// concept of preloading pipeline. For example, if a site uses a
/// SpeculationRules for prerender, Chrome first starts a prefetch and
/// then upgrades it to prerender.
/// 
/// CDP events for them are emitted separately but they share
/// 'PreloadPipelineId'.

pub type PreloadPipelineId<'a> = Cow<'a, str>;

/// List of FinalStatus reasons for Prerender2.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PrerenderFinalStatus {
    #[default]
    #[serde(rename = "Activated")]
    Activated,
    #[serde(rename = "Destroyed")]
    Destroyed,
    #[serde(rename = "LowEndDevice")]
    LowEndDevice,
    #[serde(rename = "InvalidSchemeRedirect")]
    InvalidSchemeRedirect,
    #[serde(rename = "InvalidSchemeNavigation")]
    InvalidSchemeNavigation,
    #[serde(rename = "NavigationRequestBlockedByCsp")]
    NavigationRequestBlockedByCsp,
    #[serde(rename = "MojoBinderPolicy")]
    MojoBinderPolicy,
    #[serde(rename = "RendererProcessCrashed")]
    RendererProcessCrashed,
    #[serde(rename = "RendererProcessKilled")]
    RendererProcessKilled,
    #[serde(rename = "Download")]
    Download,
    #[serde(rename = "TriggerDestroyed")]
    TriggerDestroyed,
    #[serde(rename = "NavigationNotCommitted")]
    NavigationNotCommitted,
    #[serde(rename = "NavigationBadHttpStatus")]
    NavigationBadHttpStatus,
    #[serde(rename = "ClientCertRequested")]
    ClientCertRequested,
    #[serde(rename = "NavigationRequestNetworkError")]
    NavigationRequestNetworkError,
    #[serde(rename = "CancelAllHostsForTesting")]
    CancelAllHostsForTesting,
    #[serde(rename = "DidFailLoad")]
    DidFailLoad,
    #[serde(rename = "Stop")]
    Stop,
    #[serde(rename = "SslCertificateError")]
    SslCertificateError,
    #[serde(rename = "LoginAuthRequested")]
    LoginAuthRequested,
    #[serde(rename = "UaChangeRequiresReload")]
    UaChangeRequiresReload,
    #[serde(rename = "BlockedByClient")]
    BlockedByClient,
    #[serde(rename = "AudioOutputDeviceRequested")]
    AudioOutputDeviceRequested,
    #[serde(rename = "MixedContent")]
    MixedContent,
    #[serde(rename = "TriggerBackgrounded")]
    TriggerBackgrounded,
    #[serde(rename = "MemoryLimitExceeded")]
    MemoryLimitExceeded,
    #[serde(rename = "DataSaverEnabled")]
    DataSaverEnabled,
    #[serde(rename = "TriggerUrlHasEffectiveUrl")]
    TriggerUrlHasEffectiveUrl,
    #[serde(rename = "ActivatedBeforeStarted")]
    ActivatedBeforeStarted,
    #[serde(rename = "InactivePageRestriction")]
    InactivePageRestriction,
    #[serde(rename = "StartFailed")]
    StartFailed,
    #[serde(rename = "TimeoutBackgrounded")]
    TimeoutBackgrounded,
    #[serde(rename = "CrossSiteRedirectInInitialNavigation")]
    CrossSiteRedirectInInitialNavigation,
    #[serde(rename = "CrossSiteNavigationInInitialNavigation")]
    CrossSiteNavigationInInitialNavigation,
    #[serde(rename = "SameSiteCrossOriginRedirectNotOptInInInitialNavigation")]
    SameSiteCrossOriginRedirectNotOptInInInitialNavigation,
    #[serde(rename = "SameSiteCrossOriginNavigationNotOptInInInitialNavigation")]
    SameSiteCrossOriginNavigationNotOptInInInitialNavigation,
    #[serde(rename = "ActivationNavigationParameterMismatch")]
    ActivationNavigationParameterMismatch,
    #[serde(rename = "ActivatedInBackground")]
    ActivatedInBackground,
    #[serde(rename = "EmbedderHostDisallowed")]
    EmbedderHostDisallowed,
    #[serde(rename = "ActivationNavigationDestroyedBeforeSuccess")]
    ActivationNavigationDestroyedBeforeSuccess,
    #[serde(rename = "TabClosedByUserGesture")]
    TabClosedByUserGesture,
    #[serde(rename = "TabClosedWithoutUserGesture")]
    TabClosedWithoutUserGesture,
    #[serde(rename = "PrimaryMainFrameRendererProcessCrashed")]
    PrimaryMainFrameRendererProcessCrashed,
    #[serde(rename = "PrimaryMainFrameRendererProcessKilled")]
    PrimaryMainFrameRendererProcessKilled,
    #[serde(rename = "ActivationFramePolicyNotCompatible")]
    ActivationFramePolicyNotCompatible,
    #[serde(rename = "PreloadingDisabled")]
    PreloadingDisabled,
    #[serde(rename = "BatterySaverEnabled")]
    BatterySaverEnabled,
    #[serde(rename = "ActivatedDuringMainFrameNavigation")]
    ActivatedDuringMainFrameNavigation,
    #[serde(rename = "PreloadingUnsupportedByWebContents")]
    PreloadingUnsupportedByWebContents,
    #[serde(rename = "CrossSiteRedirectInMainFrameNavigation")]
    CrossSiteRedirectInMainFrameNavigation,
    #[serde(rename = "CrossSiteNavigationInMainFrameNavigation")]
    CrossSiteNavigationInMainFrameNavigation,
    #[serde(rename = "SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation")]
    SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation,
    #[serde(rename = "SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation")]
    SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation,
    #[serde(rename = "MemoryPressureOnTrigger")]
    MemoryPressureOnTrigger,
    #[serde(rename = "MemoryPressureAfterTriggered")]
    MemoryPressureAfterTriggered,
    #[serde(rename = "PrerenderingDisabledByDevTools")]
    PrerenderingDisabledByDevTools,
    #[serde(rename = "SpeculationRuleRemoved")]
    SpeculationRuleRemoved,
    #[serde(rename = "ActivatedWithAuxiliaryBrowsingContexts")]
    ActivatedWithAuxiliaryBrowsingContexts,
    #[serde(rename = "MaxNumOfRunningEagerPrerendersExceeded")]
    MaxNumOfRunningEagerPrerendersExceeded,
    #[serde(rename = "MaxNumOfRunningNonEagerPrerendersExceeded")]
    MaxNumOfRunningNonEagerPrerendersExceeded,
    #[serde(rename = "MaxNumOfRunningEmbedderPrerendersExceeded")]
    MaxNumOfRunningEmbedderPrerendersExceeded,
    #[serde(rename = "PrerenderingUrlHasEffectiveUrl")]
    PrerenderingUrlHasEffectiveUrl,
    #[serde(rename = "RedirectedPrerenderingUrlHasEffectiveUrl")]
    RedirectedPrerenderingUrlHasEffectiveUrl,
    #[serde(rename = "ActivationUrlHasEffectiveUrl")]
    ActivationUrlHasEffectiveUrl,
    #[serde(rename = "JavaScriptInterfaceAdded")]
    JavaScriptInterfaceAdded,
    #[serde(rename = "JavaScriptInterfaceRemoved")]
    JavaScriptInterfaceRemoved,
    #[serde(rename = "AllPrerenderingCanceled")]
    AllPrerenderingCanceled,
    #[serde(rename = "WindowClosed")]
    WindowClosed,
    #[serde(rename = "SlowNetwork")]
    SlowNetwork,
    #[serde(rename = "OtherPrerenderedPageActivated")]
    OtherPrerenderedPageActivated,
    #[serde(rename = "V8OptimizerDisabled")]
    V8OptimizerDisabled,
    #[serde(rename = "PrerenderFailedDuringPrefetch")]
    PrerenderFailedDuringPrefetch,
    #[serde(rename = "BrowsingDataRemoved")]
    BrowsingDataRemoved,
    #[serde(rename = "PrerenderHostReused")]
    PrerenderHostReused,
    #[serde(rename = "FormSubmitWhenPrerendering")]
    FormSubmitWhenPrerendering,
    #[serde(rename = "CrossDocumentRestart")]
    CrossDocumentRestart,
}

/// Preloading status values, see also PreloadingTriggeringOutcome. This
/// status is shared by prefetchStatusUpdated and prerenderStatusUpdated.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PreloadingStatus {
    #[default]
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Ready")]
    Ready,
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "Failure")]
    Failure,
    #[serde(rename = "NotSupported")]
    NotSupported,
}

/// TODO(<https://crbug.com/1384419>): revisit the list of PrefetchStatus and
/// filter out the ones that aren't necessary to the developers.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PrefetchStatus {
    #[default]
    #[serde(rename = "PrefetchAllowed")]
    PrefetchAllowed,
    #[serde(rename = "PrefetchFailedIneligibleRedirect")]
    PrefetchFailedIneligibleRedirect,
    #[serde(rename = "PrefetchFailedInvalidRedirect")]
    PrefetchFailedInvalidRedirect,
    #[serde(rename = "PrefetchFailedMIMENotSupported")]
    PrefetchFailedMIMENotSupported,
    #[serde(rename = "PrefetchFailedNetError")]
    PrefetchFailedNetError,
    #[serde(rename = "PrefetchFailedNon2XX")]
    PrefetchFailedNon2XX,
    #[serde(rename = "PrefetchEvictedAfterBrowsingDataRemoved")]
    PrefetchEvictedAfterBrowsingDataRemoved,
    #[serde(rename = "PrefetchEvictedAfterCandidateRemoved")]
    PrefetchEvictedAfterCandidateRemoved,
    #[serde(rename = "PrefetchEvictedForNewerPrefetch")]
    PrefetchEvictedForNewerPrefetch,
    #[serde(rename = "PrefetchHeldback")]
    PrefetchHeldback,
    #[serde(rename = "PrefetchIneligibleRetryAfter")]
    PrefetchIneligibleRetryAfter,
    #[serde(rename = "PrefetchIsPrivacyDecoy")]
    PrefetchIsPrivacyDecoy,
    #[serde(rename = "PrefetchIsStale")]
    PrefetchIsStale,
    #[serde(rename = "PrefetchNotEligibleBlockedByConnectionAllowlist")]
    PrefetchNotEligibleBlockedByConnectionAllowlist,
    #[serde(rename = "PrefetchNotEligibleBrowserContextOffTheRecord")]
    PrefetchNotEligibleBrowserContextOffTheRecord,
    #[serde(rename = "PrefetchNotEligibleCrossOrigin")]
    PrefetchNotEligibleCrossOrigin,
    #[serde(rename = "PrefetchNotEligibleDataSaverEnabled")]
    PrefetchNotEligibleDataSaverEnabled,
    #[serde(rename = "PrefetchNotEligibleExistingProxy")]
    PrefetchNotEligibleExistingProxy,
    #[serde(rename = "PrefetchNotEligibleHostIsNonUnique")]
    PrefetchNotEligibleHostIsNonUnique,
    #[serde(rename = "PrefetchNotEligibleNonDefaultStoragePartition")]
    PrefetchNotEligibleNonDefaultStoragePartition,
    #[serde(rename = "PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy")]
    PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy,
    #[serde(rename = "PrefetchNotEligibleSchemeIsNotHttps")]
    PrefetchNotEligibleSchemeIsNotHttps,
    #[serde(rename = "PrefetchNotEligibleUserHasCookies")]
    PrefetchNotEligibleUserHasCookies,
    #[serde(rename = "PrefetchNotEligibleUserHasServiceWorker")]
    PrefetchNotEligibleUserHasServiceWorker,
    #[serde(rename = "PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler")]
    PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler,
    #[serde(rename = "PrefetchNotEligibleRedirectFromServiceWorker")]
    PrefetchNotEligibleRedirectFromServiceWorker,
    #[serde(rename = "PrefetchNotEligibleRedirectToServiceWorker")]
    PrefetchNotEligibleRedirectToServiceWorker,
    #[serde(rename = "PrefetchNotEligibleBatterySaverEnabled")]
    PrefetchNotEligibleBatterySaverEnabled,
    #[serde(rename = "PrefetchNotEligiblePreloadingDisabled")]
    PrefetchNotEligiblePreloadingDisabled,
    #[serde(rename = "PrefetchNotFinishedInTime")]
    PrefetchNotFinishedInTime,
    #[serde(rename = "PrefetchNotStarted")]
    PrefetchNotStarted,
    #[serde(rename = "PrefetchNotUsedCookiesChanged")]
    PrefetchNotUsedCookiesChanged,
    #[serde(rename = "PrefetchProxyNotAvailable")]
    PrefetchProxyNotAvailable,
    #[serde(rename = "PrefetchResponseUsed")]
    PrefetchResponseUsed,
    #[serde(rename = "PrefetchSuccessfulButNotUsed")]
    PrefetchSuccessfulButNotUsed,
    #[serde(rename = "PrefetchNotUsedProbeFailed")]
    PrefetchNotUsedProbeFailed,
    #[serde(rename = "PrefetchCancelledOnUserNavigation")]
    PrefetchCancelledOnUserNavigation,
}

/// Information of headers to be displayed when the header mismatch occurred.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PrerenderMismatchedHeaders<'a> {
    #[serde(rename = "headerName")]
    pub header_name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "initialValue")]
    pub initial_value: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activationValue")]
    pub activation_value: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Preload.enable")]
pub struct EnableParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Preload.disable")]
pub struct DisableParams {

}
/// Upsert. Currently, it is only emitted when a rule set added.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Preload.ruleSetUpdated")]
pub struct RuleSetUpdated<'a> {
    #[serde(rename = "ruleSet")]
    pub rule_set: RuleSet<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Preload.ruleSetRemoved")]
pub struct RuleSetRemoved<'a> {
    pub id: RuleSetId<'a>,
}
/// Fired when a preload enabled state is updated.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Preload.preloadEnabledStateUpdated")]
pub struct PreloadEnabledStateUpdated {
    #[serde(rename = "disabledByPreference")]
    pub disabled_by_preference: bool,
    #[serde(rename = "disabledByDataSaver")]
    pub disabled_by_data_saver: bool,
    #[serde(rename = "disabledByBatterySaver")]
    pub disabled_by_battery_saver: bool,
    #[serde(rename = "disabledByHoldbackPrefetchSpeculationRules")]
    pub disabled_by_holdback_prefetch_speculation_rules: bool,
    #[serde(rename = "disabledByHoldbackPrerenderSpeculationRules")]
    pub disabled_by_holdback_prerender_speculation_rules: bool,
}
/// Fired when a prefetch attempt is updated.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Preload.prefetchStatusUpdated")]
pub struct PrefetchStatusUpdated<'a> {
    pub key: PreloadingAttemptKey<'a>,
    #[serde(rename = "pipelineId")]
    pub pipeline_id: PreloadPipelineId<'a>,
    /// The frame id of the frame initiating prefetch.
    #[serde(rename = "initiatingFrameId")]
    pub initiating_frame_id: crate::page::FrameId<'a>,
    #[serde(rename = "prefetchUrl")]
    pub prefetch_url: Cow<'a, str>,
    pub status: PreloadingStatus,
    #[serde(rename = "prefetchStatus")]
    pub prefetch_status: PrefetchStatus,
    #[serde(rename = "requestId")]
    pub request_id: crate::network::RequestId<'a>,
}
/// Fired when a prerender attempt is updated.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Preload.prerenderStatusUpdated")]
pub struct PrerenderStatusUpdated<'a> {
    pub key: PreloadingAttemptKey<'a>,
    #[serde(rename = "pipelineId")]
    pub pipeline_id: PreloadPipelineId<'a>,
    pub status: PreloadingStatus,
    /// The action currently performed by this attempt. This differs from
    /// 'key.action' after a prerender-until-script attempt is upgraded in place
    /// to a full prerender.
    #[serde(skip_serializing_if = "Option::is_none", rename = "effectiveAction")]
    pub effective_action: Option<SpeculationAction>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "prerenderStatus")]
    pub prerender_status: Option<PrerenderFinalStatus>,
    /// This is used to give users more information about the name of Mojo interface
    /// that is incompatible with prerender and has caused the cancellation of the attempt.
    #[serde(skip_serializing_if = "Option::is_none", rename = "disallowedMojoInterface")]
    pub disallowed_mojo_interface: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mismatchedHeaders")]
    pub mismatched_headers: Option<Vec<PrerenderMismatchedHeaders<'a>>>,
}
/// Send a list of sources for all preloading attempts in a document.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Preload.preloadingAttemptSourcesUpdated")]
pub struct PreloadingAttemptSourcesUpdated<'a> {
    #[serde(rename = "loaderId")]
    pub loader_id: crate::network::LoaderId<'a>,
    #[serde(rename = "preloadingAttemptSources")]
    pub preloading_attempt_sources: Vec<PreloadingAttemptSource<'a>>,
}