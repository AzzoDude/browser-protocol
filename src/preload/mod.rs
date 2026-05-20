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
    #[serde(rename = "loaderId")]
    loader_id: crate::network::LoaderId<'a>,
    /// Source text of JSON representing the rule set. If it comes from
    /// '\<script\>' tag, it is the textContent of the node. Note that it is
    /// a JSON for valid case.
    /// 
    /// See also:
    /// - <https://wicg.github.io/nav-speculation/speculation-rules.html>
    /// - <https://github.com/WICG/nav-speculation/blob/main/triggers.md>
    #[serde(rename = "sourceText")]
    source_text: Cow<'a, str>,
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
    backend_node_id: Option<crate::dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestId")]
    request_id: Option<crate::network::RequestId<'a>>,
    /// Error information
    /// 'errorMessage' is null iff 'errorType' is null.
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorType")]
    error_type: Option<RuleSetErrorType>,
    /// TODO(<https://crbug.com/1425354>): Replace this property with structured error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorMessage")]
    error_message: Option<Cow<'a, str>>,
    /// For more details, see:
    /// <https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md>
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Cow<'a, str>>,
}

impl<'a> RuleSet<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: 
    /// * `loader_id`: Identifies a document which the rule set is associated with.
    /// * `source_text`: Source text of JSON representing the rule set. If it comes from `\<script\>` tag, it is the textContent of the node. Note that it is a JSON for valid case.  See also: - <https://wicg.github.io/nav-speculation/speculation-rules.html> - <https://github.com/WICG/nav-speculation/blob/main/triggers.md>
    pub fn builder(id: impl Into<RuleSetId<'a>>, loader_id: crate::network::LoaderId<'a>, source_text: impl Into<Cow<'a, str>>) -> RuleSetBuilder<'a> {
        RuleSetBuilder {
            id: id.into(),
            loader_id: loader_id,
            source_text: source_text.into(),
            backend_node_id: None,
            url: None,
            request_id: None,
            error_type: None,
            error_message: None,
            tag: None,
        }
    }
    pub fn id(&self) -> &RuleSetId<'a> { &self.id }
    /// Identifies a document which the rule set is associated with.
    pub fn loader_id(&self) -> &crate::network::LoaderId<'a> { &self.loader_id }
    /// Source text of JSON representing the rule set. If it comes from
    /// '\<script\>' tag, it is the textContent of the node. Note that it is
    /// a JSON for valid case.
    /// 
    /// See also:
    /// - <https://wicg.github.io/nav-speculation/speculation-rules.html>
    /// - <https://github.com/WICG/nav-speculation/blob/main/triggers.md>
    pub fn source_text(&self) -> &str { self.source_text.as_ref() }
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
    pub fn backend_node_id(&self) -> Option<&crate::dom::BackendNodeId> { self.backend_node_id.as_ref() }
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    pub fn request_id(&self) -> Option<&crate::network::RequestId<'a>> { self.request_id.as_ref() }
    /// Error information
    /// 'errorMessage' is null iff 'errorType' is null.
    pub fn error_type(&self) -> Option<&RuleSetErrorType> { self.error_type.as_ref() }
    /// TODO(<https://crbug.com/1425354>): Replace this property with structured error.
    pub fn error_message(&self) -> Option<&str> { self.error_message.as_deref() }
    /// For more details, see:
    /// <https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md>
    pub fn tag(&self) -> Option<&str> { self.tag.as_deref() }
}


pub struct RuleSetBuilder<'a> {
    id: RuleSetId<'a>,
    loader_id: crate::network::LoaderId<'a>,
    source_text: Cow<'a, str>,
    backend_node_id: Option<crate::dom::BackendNodeId>,
    url: Option<Cow<'a, str>>,
    request_id: Option<crate::network::RequestId<'a>>,
    error_type: Option<RuleSetErrorType>,
    error_message: Option<Cow<'a, str>>,
    tag: Option<Cow<'a, str>>,
}

impl<'a> RuleSetBuilder<'a> {
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
    pub fn backend_node_id(mut self, backend_node_id: crate::dom::BackendNodeId) -> Self { self.backend_node_id = Some(backend_node_id); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    pub fn request_id(mut self, request_id: crate::network::RequestId<'a>) -> Self { self.request_id = Some(request_id); self }
    /// Error information
    /// 'errorMessage' is null iff 'errorType' is null.
    pub fn error_type(mut self, error_type: impl Into<RuleSetErrorType>) -> Self { self.error_type = Some(error_type.into()); self }
    /// TODO(<https://crbug.com/1425354>): Replace this property with structured error.
    pub fn error_message(mut self, error_message: impl Into<Cow<'a, str>>) -> Self { self.error_message = Some(error_message.into()); self }
    /// For more details, see:
    /// <https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md>
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self { self.tag = Some(tag.into()); self }
    pub fn build(self) -> RuleSet<'a> {
        RuleSet {
            id: self.id,
            loader_id: self.loader_id,
            source_text: self.source_text,
            backend_node_id: self.backend_node_id,
            url: self.url,
            request_id: self.request_id,
            error_type: self.error_type,
            error_message: self.error_message,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreloadingAttemptKey<'a> {
    #[serde(rename = "loaderId")]
    loader_id: crate::network::LoaderId<'a>,
    action: SpeculationAction,
    url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "formSubmission")]
    form_submission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetHint")]
    target_hint: Option<SpeculationTargetHint>,
}

impl<'a> PreloadingAttemptKey<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `loader_id`: 
    /// * `action`: 
    /// * `url`: 
    pub fn builder(loader_id: crate::network::LoaderId<'a>, action: impl Into<SpeculationAction>, url: impl Into<Cow<'a, str>>) -> PreloadingAttemptKeyBuilder<'a> {
        PreloadingAttemptKeyBuilder {
            loader_id: loader_id,
            action: action.into(),
            url: url.into(),
            form_submission: None,
            target_hint: None,
        }
    }
    pub fn loader_id(&self) -> &crate::network::LoaderId<'a> { &self.loader_id }
    pub fn action(&self) -> &SpeculationAction { &self.action }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn form_submission(&self) -> Option<bool> { self.form_submission }
    pub fn target_hint(&self) -> Option<&SpeculationTargetHint> { self.target_hint.as_ref() }
}


pub struct PreloadingAttemptKeyBuilder<'a> {
    loader_id: crate::network::LoaderId<'a>,
    action: SpeculationAction,
    url: Cow<'a, str>,
    form_submission: Option<bool>,
    target_hint: Option<SpeculationTargetHint>,
}

impl<'a> PreloadingAttemptKeyBuilder<'a> {
    pub fn form_submission(mut self, form_submission: bool) -> Self { self.form_submission = Some(form_submission); self }
    pub fn target_hint(mut self, target_hint: impl Into<SpeculationTargetHint>) -> Self { self.target_hint = Some(target_hint.into()); self }
    pub fn build(self) -> PreloadingAttemptKey<'a> {
        PreloadingAttemptKey {
            loader_id: self.loader_id,
            action: self.action,
            url: self.url,
            form_submission: self.form_submission,
            target_hint: self.target_hint,
        }
    }
}

/// Lists sources for a preloading attempt, specifically the ids of rule sets
/// that had a speculation rule that triggered the attempt, and the
/// BackendNodeIds of \<a href\> or \<area href\> elements that triggered the
/// attempt (in the case of attempts triggered by a document rule). It is
/// possible for multiple rule sets and links to trigger a single attempt.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreloadingAttemptSource<'a> {
    key: PreloadingAttemptKey<'a>,
    #[serde(rename = "ruleSetIds")]
    rule_set_ids: Vec<RuleSetId<'a>>,
    #[serde(rename = "nodeIds")]
    node_ids: Vec<crate::dom::BackendNodeId>,
}

impl<'a> PreloadingAttemptSource<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `key`: 
    /// * `rule_set_ids`: 
    /// * `node_ids`: 
    pub fn builder(key: PreloadingAttemptKey<'a>, rule_set_ids: Vec<RuleSetId<'a>>, node_ids: Vec<crate::dom::BackendNodeId>) -> PreloadingAttemptSourceBuilder<'a> {
        PreloadingAttemptSourceBuilder {
            key: key,
            rule_set_ids: rule_set_ids,
            node_ids: node_ids,
        }
    }
    pub fn key(&self) -> &PreloadingAttemptKey<'a> { &self.key }
    pub fn rule_set_ids(&self) -> &[RuleSetId<'a>] { &self.rule_set_ids }
    pub fn node_ids(&self) -> &[crate::dom::BackendNodeId] { &self.node_ids }
}


pub struct PreloadingAttemptSourceBuilder<'a> {
    key: PreloadingAttemptKey<'a>,
    rule_set_ids: Vec<RuleSetId<'a>>,
    node_ids: Vec<crate::dom::BackendNodeId>,
}

impl<'a> PreloadingAttemptSourceBuilder<'a> {
    pub fn build(self) -> PreloadingAttemptSource<'a> {
        PreloadingAttemptSource {
            key: self.key,
            rule_set_ids: self.rule_set_ids,
            node_ids: self.node_ids,
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
    #[serde(rename = "headerName")]
    header_name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "initialValue")]
    initial_value: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activationValue")]
    activation_value: Option<Cow<'a, str>>,
}

impl<'a> PrerenderMismatchedHeaders<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `header_name`: 
    pub fn builder(header_name: impl Into<Cow<'a, str>>) -> PrerenderMismatchedHeadersBuilder<'a> {
        PrerenderMismatchedHeadersBuilder {
            header_name: header_name.into(),
            initial_value: None,
            activation_value: None,
        }
    }
    pub fn header_name(&self) -> &str { self.header_name.as_ref() }
    pub fn initial_value(&self) -> Option<&str> { self.initial_value.as_deref() }
    pub fn activation_value(&self) -> Option<&str> { self.activation_value.as_deref() }
}


pub struct PrerenderMismatchedHeadersBuilder<'a> {
    header_name: Cow<'a, str>,
    initial_value: Option<Cow<'a, str>>,
    activation_value: Option<Cow<'a, str>>,
}

impl<'a> PrerenderMismatchedHeadersBuilder<'a> {
    pub fn initial_value(mut self, initial_value: impl Into<Cow<'a, str>>) -> Self { self.initial_value = Some(initial_value.into()); self }
    pub fn activation_value(mut self, activation_value: impl Into<Cow<'a, str>>) -> Self { self.activation_value = Some(activation_value.into()); self }
    pub fn build(self) -> PrerenderMismatchedHeaders<'a> {
        PrerenderMismatchedHeaders {
            header_name: self.header_name,
            initial_value: self.initial_value,
            activation_value: self.activation_value,
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
