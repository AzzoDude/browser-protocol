use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! Supports additional targets discovery and allows to attach to them.


pub type TargetID = String;

/// Unique identifier of attached debugging session.

pub type SessionID = String;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {

    pub targetId: TargetID,
    /// List of types: https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22

    #[serde(rename = "type")]
    pub type_: String,

    pub title: String,

    pub url: String,
    /// Whether the target has an attached client.

    pub attached: bool,
    /// Opener target Id

    #[serde(skip_serializing_if = "Option::is_none")]
    pub openerId: Option<TargetID>,
    /// Whether the target has access to the originating window.

    pub canAccessOpener: bool,
    /// Frame id of originating window (is only set if target has an opener).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub openerFrameId: Option<crate::page::FrameId>,
    /// Id of the parent frame, present for "iframe" and "worker" targets. For nested workers,
    /// this is the "ancestor" frame that created the first worker in the nested chain.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentFrameId: Option<crate::page::FrameId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<crate::browser::BrowserContextID>,
    /// Provides additional details for specific target types. For example, for
    /// the type of "page", this may be set to "prerender".

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
}

/// A filter used by target query/discovery/auto-attach operations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilterEntry {
    /// If set, causes exclusion of matching targets from the list.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
    /// If not present, matches any type.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// The entries in TargetFilter are matched sequentially against targets and
/// the first entry that matches determines if the target is included or not,
/// depending on the value of 'exclude' field in the entry.
/// If filter is not specified, the one assumed is
/// [{type: "browser", exclude: true}, {type: "tab", exclude: true}, {}]
/// (i.e. include everything but 'browser' and 'tab').

pub type TargetFilter = Vec<FilterEntry>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteLocation {

    pub host: String,

    pub port: i64,
}

/// The state of the target window.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum WindowState {
    #[default]
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

/// Activates (focuses) the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActivateTargetParams {

    pub targetId: TargetID,
}

impl ActivateTargetParams { pub const METHOD: &'static str = "Target.activateTarget"; }

impl crate::CdpCommand for ActivateTargetParams {
    const METHOD: &'static str = "Target.activateTarget";
    type Response = crate::EmptyReturns;
}

/// Attaches to the target with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetParams {

    pub targetId: TargetID,
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
}

/// Attaches to the target with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetReturns {
    /// Id assigned to the session.

    pub sessionId: SessionID,
}

impl AttachToTargetParams { pub const METHOD: &'static str = "Target.attachToTarget"; }

impl crate::CdpCommand for AttachToTargetParams {
    const METHOD: &'static str = "Target.attachToTarget";
    type Response = AttachToTargetReturns;
}

/// Attaches to the browser target, only uses flat sessionId mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttachToBrowserTargetReturns {
    /// Id assigned to the session.

    pub sessionId: SessionID,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachToBrowserTargetParams {}

impl AttachToBrowserTargetParams { pub const METHOD: &'static str = "Target.attachToBrowserTarget"; }

impl crate::CdpCommand for AttachToBrowserTargetParams {
    const METHOD: &'static str = "Target.attachToBrowserTarget";
    type Response = AttachToBrowserTargetReturns;
}

/// Closes the target. If the target is a page that gets closed too.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseTargetParams {

    pub targetId: TargetID,
}

/// Closes the target. If the target is a page that gets closed too.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseTargetReturns {
    /// Always set to true. If an error occurs, the response indicates protocol error.

    pub success: bool,
}

impl CloseTargetParams { pub const METHOD: &'static str = "Target.closeTarget"; }

impl crate::CdpCommand for CloseTargetParams {
    const METHOD: &'static str = "Target.closeTarget";
    type Response = CloseTargetReturns;
}

/// Inject object to the target's main frame that provides a communication
/// channel with browser target.
/// 
/// Injected object will be available as 'window[bindingName]'.
/// 
/// The object has the following API:
/// - 'binding.send(json)' - a method to send messages over the remote debugging protocol
/// - 'binding.onmessage = json => handleMessage(json)' - a callback that will be called for the protocol notifications and command responses.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExposeDevToolsProtocolParams {

    pub targetId: TargetID,
    /// Binding name, 'cdp' if not specified.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindingName: Option<String>,
    /// If true, inherits the current root session's permissions (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inheritPermissions: Option<bool>,
}

impl ExposeDevToolsProtocolParams { pub const METHOD: &'static str = "Target.exposeDevToolsProtocol"; }

impl crate::CdpCommand for ExposeDevToolsProtocolParams {
    const METHOD: &'static str = "Target.exposeDevToolsProtocol";
    type Response = crate::EmptyReturns;
}

/// Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
/// one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrowserContextParams {
    /// If specified, disposes this context when debugging session disconnects.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposeOnDetach: Option<bool>,
    /// Proxy server, similar to the one passed to --proxy-server

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxyServer: Option<String>,
    /// Proxy bypass list, similar to the one passed to --proxy-bypass-list

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxyBypassList: Option<String>,
    /// An optional list of origins to grant unlimited cross-origin access to.
    /// Parts of the URL other than those constituting origin are ignored.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub originsWithUniversalNetworkAccess: Option<Vec<String>>,
}

/// Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
/// one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrowserContextReturns {
    /// The id of the context created.

    pub browserContextId: crate::browser::BrowserContextID,
}

impl CreateBrowserContextParams { pub const METHOD: &'static str = "Target.createBrowserContext"; }

impl crate::CdpCommand for CreateBrowserContextParams {
    const METHOD: &'static str = "Target.createBrowserContext";
    type Response = CreateBrowserContextReturns;
}

/// Returns all browser contexts created with 'Target.createBrowserContext' method.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserContextsReturns {
    /// An array of browser context ids.

    pub browserContextIds: Vec<crate::browser::BrowserContextID>,
    /// The id of the default browser context if available.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaultBrowserContextId: Option<crate::browser::BrowserContextID>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBrowserContextsParams {}

impl GetBrowserContextsParams { pub const METHOD: &'static str = "Target.getBrowserContexts"; }

impl crate::CdpCommand for GetBrowserContextsParams {
    const METHOD: &'static str = "Target.getBrowserContexts";
    type Response = GetBrowserContextsReturns;
}

/// Creates a new page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetParams {
    /// The initial URL the page will be navigated to. An empty string indicates about:blank.

    pub url: String,
    /// Frame left origin in DIP (requires newWindow to be true or headless shell).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i64>,
    /// Frame top origin in DIP (requires newWindow to be true or headless shell).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    /// Frame width in DIP (requires newWindow to be true or headless shell).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    /// Frame height in DIP (requires newWindow to be true or headless shell).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Frame window state (requires newWindow to be true or headless shell).
    /// Default is normal.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowState: Option<WindowState>,
    /// The browser context to create the page in.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<crate::browser::BrowserContextID>,
    /// Whether BeginFrames for this target will be controlled via DevTools (headless shell only,
    /// not supported on MacOS yet, false by default).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enableBeginFrameControl: Option<bool>,
    /// Whether to create a new Window or Tab (false by default, not supported by headless shell).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub newWindow: Option<bool>,
    /// Whether to create the target in background or foreground (false by default, not supported
    /// by headless shell).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// Whether to create the target of type "tab".

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forTab: Option<bool>,
    /// Whether to create a hidden target. The hidden target is observable via protocol, but not
    /// present in the tab UI strip. Cannot be created with 'forTab: true', 'newWindow: true' or
    /// 'background: false'. The life-time of the tab is limited to the life-time of the session.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// If specified, the option is used to determine if the new target should
    /// be focused or not. By default, the focus behavior depends on the
    /// value of the background field. For example, background=false and focus=false
    /// will result in the target tab being opened but the browser window remain
    /// unchanged (if it was in the background, it will remain in the background)
    /// and background=false with focus=undefined will result in the window being focused.
    /// Using background: true and focus: true is not supported and will result in an error.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<bool>,
}

/// Creates a new page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetReturns {
    /// The id of the page opened.

    pub targetId: TargetID,
}

impl CreateTargetParams { pub const METHOD: &'static str = "Target.createTarget"; }

impl crate::CdpCommand for CreateTargetParams {
    const METHOD: &'static str = "Target.createTarget";
    type Response = CreateTargetReturns;
}

/// Detaches session with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DetachFromTargetParams {
    /// Session to detach.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessionId: Option<SessionID>,
    /// Deprecated.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetId: Option<TargetID>,
}

impl DetachFromTargetParams { pub const METHOD: &'static str = "Target.detachFromTarget"; }

impl crate::CdpCommand for DetachFromTargetParams {
    const METHOD: &'static str = "Target.detachFromTarget";
    type Response = crate::EmptyReturns;
}

/// Deletes a BrowserContext. All the belonging pages will be closed without calling their
/// beforeunload hooks.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisposeBrowserContextParams {

    pub browserContextId: crate::browser::BrowserContextID,
}

impl DisposeBrowserContextParams { pub const METHOD: &'static str = "Target.disposeBrowserContext"; }

impl crate::CdpCommand for DisposeBrowserContextParams {
    const METHOD: &'static str = "Target.disposeBrowserContext";
    type Response = crate::EmptyReturns;
}

/// Returns information about a target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetInfoParams {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetId: Option<TargetID>,
}

/// Returns information about a target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetInfoReturns {

    pub targetInfo: TargetInfo,
}

impl GetTargetInfoParams { pub const METHOD: &'static str = "Target.getTargetInfo"; }

impl crate::CdpCommand for GetTargetInfoParams {
    const METHOD: &'static str = "Target.getTargetInfo";
    type Response = GetTargetInfoReturns;
}

/// Retrieves a list of available targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetsParams {
    /// Only targets matching filter will be reported. If filter is not specified
    /// and target discovery is currently enabled, a filter used for target discovery
    /// is used for consistency.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TargetFilter>,
}

/// Retrieves a list of available targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetsReturns {
    /// The list of targets.

    pub targetInfos: Vec<TargetInfo>,
}

impl GetTargetsParams { pub const METHOD: &'static str = "Target.getTargets"; }

impl crate::CdpCommand for GetTargetsParams {
    const METHOD: &'static str = "Target.getTargets";
    type Response = GetTargetsReturns;
}

/// Sends protocol message over session with given id.
/// Consider using flat mode instead; see commands attachToTarget, setAutoAttach,
/// and crbug.com/991325.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageToTargetParams {

    pub message: String,
    /// Identifier of the session.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessionId: Option<SessionID>,
    /// Deprecated.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetId: Option<TargetID>,
}

impl SendMessageToTargetParams { pub const METHOD: &'static str = "Target.sendMessageToTarget"; }

impl crate::CdpCommand for SendMessageToTargetParams {
    const METHOD: &'static str = "Target.sendMessageToTarget";
    type Response = crate::EmptyReturns;
}

/// Controls whether to automatically attach to new targets which are considered
/// to be directly related to this one (for example, iframes or workers).
/// When turned on, attaches to all existing related targets as well. When turned off,
/// automatically detaches from all currently attached targets.
/// This also clears all targets added by 'autoAttachRelated' from the list of targets to watch
/// for creation of related targets.
/// You might want to call this recursively for auto-attached targets to attach
/// to all available targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoAttachParams {
    /// Whether to auto-attach to related targets.

    pub autoAttach: bool,
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.

    pub waitForDebuggerOnStart: bool,
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
    /// Only targets matching filter will be attached.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TargetFilter>,
}

impl SetAutoAttachParams { pub const METHOD: &'static str = "Target.setAutoAttach"; }

impl crate::CdpCommand for SetAutoAttachParams {
    const METHOD: &'static str = "Target.setAutoAttach";
    type Response = crate::EmptyReturns;
}

/// Adds the specified target to the list of targets that will be monitored for any related target
/// creation (such as child frames, child workers and new versions of service worker) and reported
/// through 'attachedToTarget'. The specified target is also auto-attached.
/// This cancels the effect of any previous 'setAutoAttach' and is also cancelled by subsequent
/// 'setAutoAttach'. Only available at the Browser target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AutoAttachRelatedParams {

    pub targetId: TargetID,
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.

    pub waitForDebuggerOnStart: bool,
    /// Only targets matching filter will be attached.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TargetFilter>,
}

impl AutoAttachRelatedParams { pub const METHOD: &'static str = "Target.autoAttachRelated"; }

impl crate::CdpCommand for AutoAttachRelatedParams {
    const METHOD: &'static str = "Target.autoAttachRelated";
    type Response = crate::EmptyReturns;
}

/// Controls whether to discover available targets and notify via
/// 'targetCreated/targetInfoChanged/targetDestroyed' events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDiscoverTargetsParams {
    /// Whether to discover available targets.

    pub discover: bool,
    /// Only targets matching filter will be attached. If 'discover' is false,
    /// 'filter' must be omitted or empty.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TargetFilter>,
}

impl SetDiscoverTargetsParams { pub const METHOD: &'static str = "Target.setDiscoverTargets"; }

impl crate::CdpCommand for SetDiscoverTargetsParams {
    const METHOD: &'static str = "Target.setDiscoverTargets";
    type Response = crate::EmptyReturns;
}

/// Enables target discovery for the specified locations, when 'setDiscoverTargets' was set to
/// 'true'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRemoteLocationsParams {
    /// List of remote locations.

    pub locations: Vec<RemoteLocation>,
}

impl SetRemoteLocationsParams { pub const METHOD: &'static str = "Target.setRemoteLocations"; }

impl crate::CdpCommand for SetRemoteLocationsParams {
    const METHOD: &'static str = "Target.setRemoteLocations";
    type Response = crate::EmptyReturns;
}

/// Gets the targetId of the DevTools page target opened for the given target
/// (if any).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDevToolsTargetParams {
    /// Page or tab target ID.

    pub targetId: TargetID,
}

/// Gets the targetId of the DevTools page target opened for the given target
/// (if any).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDevToolsTargetReturns {
    /// The targetId of DevTools page target if exists.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetId: Option<TargetID>,
}

impl GetDevToolsTargetParams { pub const METHOD: &'static str = "Target.getDevToolsTarget"; }

impl crate::CdpCommand for GetDevToolsTargetParams {
    const METHOD: &'static str = "Target.getDevToolsTarget";
    type Response = GetDevToolsTargetReturns;
}

/// Opens a DevTools window for the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenDevToolsParams {
    /// This can be the page or tab target ID.

    pub targetId: TargetID,
    /// The id of the panel we want DevTools to open initially. Currently
    /// supported panels are elements, console, network, sources, resources
    /// and performance.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub panelId: Option<String>,
}

/// Opens a DevTools window for the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenDevToolsReturns {
    /// The targetId of DevTools page target.

    pub targetId: TargetID,
}

impl OpenDevToolsParams { pub const METHOD: &'static str = "Target.openDevTools"; }

impl crate::CdpCommand for OpenDevToolsParams {
    const METHOD: &'static str = "Target.openDevTools";
    type Response = OpenDevToolsReturns;
}
