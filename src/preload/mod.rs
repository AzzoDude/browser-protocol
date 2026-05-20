use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Unique id

pub type RuleSetId<'a> = Cow<'a, str>;

/// Corresponds to SpeculationRuleSet

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuleSet<'a> {
    id: RuleSetId<'a>,
    /// Identifies a document which the rule set is associated with.
    loaderId: crate::network::LoaderId<'a>,
    /// Source text of JSON representing the rule set. If it comes from
    /// '<script>' tag, it is the textContent of the node. Note that it is
    /// a JSON for valid case.
    /// 
    /// See also:
    /// - https://wicg.github.io/nav-speculation/speculation-rules.html
    /// - https://github.com/WICG/nav-speculation/blob/main/triggers.md
    sourceText: Cow<'a, str>,
    /// A speculation rule set is either added through an inline
    /// '<script>' tag or through an external resource via the
    /// 'Speculation-Rules' HTTP header. For the first case, we include
    /// the BackendNodeId of the relevant '<script>' tag. For the second
    /// case, we include the external URL where the rule set was loaded
    /// from, and also RequestId if Network domain is enabled.
    /// 
    /// See also:
    /// - https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-script
    /// - https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-header
    #[serde(skip_serializing_if = "Option::is_none")]
    backendNodeId: Option<crate::dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requestId: Option<crate::network::RequestId<'a>>,
    /// Error information
    /// 'errorMessage' is null iff 'errorType' is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    errorType: Option<RuleSetErrorType>,
    /// TODO(https://crbug.com/1425354): Replace this property with structured error.
    #[serde(skip_serializing_if = "Option::is_none")]
    errorMessage: Option<Cow<'a, str>>,
    /// For more details, see:
    /// https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Cow<'a, str>>,
}

impl<'a> RuleSet<'a> {
    pub fn builder(id: impl Into<RuleSetId<'a>>, loaderId: crate::network::LoaderId<'a>, sourceText: impl Into<Cow<'a, str>>) -> RuleSetBuilder<'a> {
        RuleSetBuilder {
            id: id.into(),
            loaderId: loaderId,
            sourceText: sourceText.into(),
            backendNodeId: None,
            url: None,
            requestId: None,
            errorType: None,
            errorMessage: None,
            tag: None,
        }
    }
    pub fn id(&self) -> &RuleSetId<'a> { &self.id }
    pub fn loaderId(&self) -> &crate::network::LoaderId<'a> { &self.loaderId }
    pub fn sourceText(&self) -> &str { self.sourceText.as_ref() }
    pub fn backendNodeId(&self) -> Option<&crate::dom::BackendNodeId> { self.backendNodeId.as_ref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn requestId(&self) -> Option<&crate::network::RequestId<'a>> { self.requestId.as_ref() }
    pub fn errorType(&self) -> Option<&RuleSetErrorType> { self.errorType.as_ref() }
    pub fn errorMessage(&self) -> Option<&str> { self.errorMessage.as_deref() }
    pub fn tag(&self) -> Option<&str> { self.tag.as_deref() }
}


pub struct RuleSetBuilder<'a> {
    id: RuleSetId<'a>,
    loaderId: crate::network::LoaderId<'a>,
    sourceText: Cow<'a, str>,
    backendNodeId: Option<crate::dom::BackendNodeId>,
    url: Option<Cow<'a, str>>,
    requestId: Option<crate::network::RequestId<'a>>,
    errorType: Option<RuleSetErrorType>,
    errorMessage: Option<Cow<'a, str>>,
    tag: Option<Cow<'a, str>>,
}

impl<'a> RuleSetBuilder<'a> {
    /// A speculation rule set is either added through an inline
    /// '<script>' tag or through an external resource via the
    /// 'Speculation-Rules' HTTP header. For the first case, we include
    /// the BackendNodeId of the relevant '<script>' tag. For the second
    /// case, we include the external URL where the rule set was loaded
    /// from, and also RequestId if Network domain is enabled.
    /// 
    /// See also:
    /// - https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-script
    /// - https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-header
    pub fn backendNodeId(mut self, backendNodeId: crate::dom::BackendNodeId) -> Self { self.backendNodeId = Some(backendNodeId); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn requestId(mut self, requestId: crate::network::RequestId<'a>) -> Self { self.requestId = Some(requestId); self }
    /// Error information
    /// 'errorMessage' is null iff 'errorType' is null.
    pub fn errorType(mut self, errorType: impl Into<RuleSetErrorType>) -> Self { self.errorType = Some(errorType.into()); self }
    /// TODO(https://crbug.com/1425354): Replace this property with structured error.
    pub fn errorMessage(mut self, errorMessage: impl Into<Cow<'a, str>>) -> Self { self.errorMessage = Some(errorMessage.into()); self }
    /// For more details, see:
    /// https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self { self.tag = Some(tag.into()); self }
    pub fn build(self) -> RuleSet<'a> {
        RuleSet {
            id: self.id,
            loaderId: self.loaderId,
            sourceText: self.sourceText,
            backendNodeId: self.backendNodeId,
            url: self.url,
            requestId: self.requestId,
            errorType: self.errorType,
            errorMessage: self.errorMessage,
            tag: self.tag,
        }
    }
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
/// See https://github.com/WICG/nav-speculation/blob/main/triggers.md#window-name-targeting-hints

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreloadingAttemptKey<'a> {
    loaderId: crate::network::LoaderId<'a>,
    action: SpeculationAction,
    url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formSubmission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targetHint: Option<SpeculationTargetHint>,
}

impl<'a> PreloadingAttemptKey<'a> {
    pub fn builder(loaderId: crate::network::LoaderId<'a>, action: impl Into<SpeculationAction>, url: impl Into<Cow<'a, str>>) -> PreloadingAttemptKeyBuilder<'a> {
        PreloadingAttemptKeyBuilder {
            loaderId: loaderId,
            action: action.into(),
            url: url.into(),
            formSubmission: None,
            targetHint: None,
        }
    }
    pub fn loaderId(&self) -> &crate::network::LoaderId<'a> { &self.loaderId }
    pub fn action(&self) -> &SpeculationAction { &self.action }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn formSubmission(&self) -> Option<bool> { self.formSubmission }
    pub fn targetHint(&self) -> Option<&SpeculationTargetHint> { self.targetHint.as_ref() }
}


pub struct PreloadingAttemptKeyBuilder<'a> {
    loaderId: crate::network::LoaderId<'a>,
    action: SpeculationAction,
    url: Cow<'a, str>,
    formSubmission: Option<bool>,
    targetHint: Option<SpeculationTargetHint>,
}

impl<'a> PreloadingAttemptKeyBuilder<'a> {
    pub fn formSubmission(mut self, formSubmission: bool) -> Self { self.formSubmission = Some(formSubmission); self }
    pub fn targetHint(mut self, targetHint: impl Into<SpeculationTargetHint>) -> Self { self.targetHint = Some(targetHint.into()); self }
    pub fn build(self) -> PreloadingAttemptKey<'a> {
        PreloadingAttemptKey {
            loaderId: self.loaderId,
            action: self.action,
            url: self.url,
            formSubmission: self.formSubmission,
            targetHint: self.targetHint,
        }
    }
}

/// Lists sources for a preloading attempt, specifically the ids of rule sets
/// that had a speculation rule that triggered the attempt, and the
/// BackendNodeIds of <a href> or <area href> elements that triggered the
/// attempt (in the case of attempts triggered by a document rule). It is
/// possible for multiple rule sets and links to trigger a single attempt.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreloadingAttemptSource<'a> {
    key: PreloadingAttemptKey<'a>,
    ruleSetIds: Vec<RuleSetId<'a>>,
    nodeIds: Vec<crate::dom::BackendNodeId>,
}

impl<'a> PreloadingAttemptSource<'a> {
    pub fn builder(key: PreloadingAttemptKey<'a>, ruleSetIds: Vec<RuleSetId<'a>>, nodeIds: Vec<crate::dom::BackendNodeId>) -> PreloadingAttemptSourceBuilder<'a> {
        PreloadingAttemptSourceBuilder {
            key: key,
            ruleSetIds: ruleSetIds,
            nodeIds: nodeIds,
        }
    }
    pub fn key(&self) -> &PreloadingAttemptKey<'a> { &self.key }
    pub fn ruleSetIds(&self) -> &[RuleSetId<'a>] { &self.ruleSetIds }
    pub fn nodeIds(&self) -> &[crate::dom::BackendNodeId] { &self.nodeIds }
}


pub struct PreloadingAttemptSourceBuilder<'a> {
    key: PreloadingAttemptKey<'a>,
    ruleSetIds: Vec<RuleSetId<'a>>,
    nodeIds: Vec<crate::dom::BackendNodeId>,
}

impl<'a> PreloadingAttemptSourceBuilder<'a> {
    pub fn build(self) -> PreloadingAttemptSource<'a> {
        PreloadingAttemptSource {
            key: self.key,
            ruleSetIds: self.ruleSetIds,
            nodeIds: self.nodeIds,
        }
    }
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

/// TODO(https://crbug.com/1384419): revisit the list of PrefetchStatus and
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
    #[serde(rename = "PrefetchNotEligibleBrowserContextOffTheRecord")]
    PrefetchNotEligibleBrowserContextOffTheRecord,
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
}

/// Information of headers to be displayed when the header mismatch occurred.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrerenderMismatchedHeaders<'a> {
    headerName: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initialValue: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activationValue: Option<Cow<'a, str>>,
}

impl<'a> PrerenderMismatchedHeaders<'a> {
    pub fn builder(headerName: impl Into<Cow<'a, str>>) -> PrerenderMismatchedHeadersBuilder<'a> {
        PrerenderMismatchedHeadersBuilder {
            headerName: headerName.into(),
            initialValue: None,
            activationValue: None,
        }
    }
    pub fn headerName(&self) -> &str { self.headerName.as_ref() }
    pub fn initialValue(&self) -> Option<&str> { self.initialValue.as_deref() }
    pub fn activationValue(&self) -> Option<&str> { self.activationValue.as_deref() }
}


pub struct PrerenderMismatchedHeadersBuilder<'a> {
    headerName: Cow<'a, str>,
    initialValue: Option<Cow<'a, str>>,
    activationValue: Option<Cow<'a, str>>,
}

impl<'a> PrerenderMismatchedHeadersBuilder<'a> {
    pub fn initialValue(mut self, initialValue: impl Into<Cow<'a, str>>) -> Self { self.initialValue = Some(initialValue.into()); self }
    pub fn activationValue(mut self, activationValue: impl Into<Cow<'a, str>>) -> Self { self.activationValue = Some(activationValue.into()); self }
    pub fn build(self) -> PrerenderMismatchedHeaders<'a> {
        PrerenderMismatchedHeaders {
            headerName: self.headerName,
            initialValue: self.initialValue,
            activationValue: self.activationValue,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Preload.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Preload.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Preload.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Preload.disable";
    type Response = crate::EmptyReturns;
}
