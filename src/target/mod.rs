//! Supports additional targets discovery and allows to attach to them.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


pub type TargetID<'a> = Cow<'a, str>;

/// Unique identifier of attached debugging session.

pub type SessionID<'a> = Cow<'a, str>;


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo<'a> {
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
    /// List of types: <https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22>
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub url: Cow<'a, str>,
    /// Whether the target has an attached client.
    pub attached: bool,
    /// Id of the parent target, if any. For example, "iframe" target may have a "page" parent.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    pub parent_id: Option<TargetID<'a>>,
    /// Opener target Id
    #[serde(skip_serializing_if = "Option::is_none", rename = "openerId")]
    pub opener_id: Option<TargetID<'a>>,
    /// Whether the target has access to the originating window.
    #[serde(rename = "canAccessOpener")]
    pub can_access_opener: bool,
    /// Frame id of originating window (is only set if target has an opener).
    #[serde(skip_serializing_if = "Option::is_none", rename = "openerFrameId")]
    pub opener_frame_id: Option<crate::page::FrameId<'a>>,
    /// Id of the parent frame, present for "iframe" and "worker" targets. For nested workers,
    /// this is the "ancestor" frame that created the first worker in the nested chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentFrameId")]
    pub parent_frame_id: Option<crate::page::FrameId<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
    /// Provides additional details for specific target types. For example, for
    /// the type of "page", this may be set to "prerender".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<Cow<'a, str>>,
    /// Embedder-specific target metadata. This is only set for targets of
    /// type "tab".
    #[serde(skip_serializing_if = "Option::is_none", rename = "embedderData")]
    pub embedder_data: Option<serde_json::Map<String, JsonValue>>,
}
/// A filter used by target query/discovery/auto-attach operations.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FilterEntry<'a> {
    /// If set, causes exclusion of matching targets from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
    /// If not present, matches any type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Cow<'a, str>>,
}
/// The entries in TargetFilter are matched sequentially against targets and
/// the first entry that matches determines if the target is included or not,
/// depending on the value of 'exclude' field in the entry.
/// If filter is not specified, the one assumed is
/// \[{type: "browser", exclude: true}, {type: "tab", exclude: true}, {}\]
/// (i.e. include everything but 'browser' and 'tab').

pub type TargetFilter<'a> = Vec<FilterEntry<'a>>;


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RemoteLocation<'a> {
    pub host: Cow<'a, str>,
    pub port: i64,
}
/// The state of the target window.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum WindowState {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "minimized")]
    Minimized,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "fullscreen")]
    Fullscreen,
}

/// Activates (focuses) the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.activateTarget")]
pub struct ActivateTargetParams<'a> {
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
}
/// Attaches to the target with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.attachToTarget", response = "AttachToTargetReturns<'a>")]
pub struct AttachToTargetParams<'a> {
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
}
/// Attaches to the target with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetReturns<'a> {
    /// Id assigned to the session.
    #[serde(rename = "sessionId")]
    pub session_id: SessionID<'a>,
}
/// Attaches to the browser target, only uses flat sessionId mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.attachToBrowserTarget", response = "AttachToBrowserTargetReturns<'a>")]
pub struct AttachToBrowserTargetParams {

}
/// Attaches to the browser target, only uses flat sessionId mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AttachToBrowserTargetReturns<'a> {
    /// Id assigned to the session.
    #[serde(rename = "sessionId")]
    pub session_id: SessionID<'a>,
}
/// Closes the target. If the target is a page that gets closed too.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.closeTarget", response = "CloseTargetReturns")]
pub struct CloseTargetParams<'a> {
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
}
/// Closes the target. If the target is a page that gets closed too.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CloseTargetReturns {
    /// Always set to true. If an error occurs, the response indicates protocol error.
    pub success: bool,
}
/// Inject object to the target's main frame that provides a communication
/// channel with browser target.
/// 
/// Injected object will be available as 'window\[bindingName\]'.
/// 
/// The object has the following API:
/// - 'binding.send(json)' - a method to send messages over the remote debugging protocol
/// - 'binding.onmessage = json =\> handleMessage(json)' - a callback that will be called for the protocol notifications and command responses.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.exposeDevToolsProtocol")]
pub struct ExposeDevToolsProtocolParams<'a> {
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
    /// Binding name, 'cdp' if not specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bindingName")]
    pub binding_name: Option<Cow<'a, str>>,
    /// If true, inherits the current root session's permissions (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "inheritPermissions")]
    pub inherit_permissions: Option<bool>,
}
/// Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
/// one.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.createBrowserContext", response = "CreateBrowserContextReturns<'a>")]
pub struct CreateBrowserContextParams<'a> {
    /// If specified, disposes this context when debugging session disconnects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "disposeOnDetach")]
    pub dispose_on_detach: Option<bool>,
    /// Proxy server, similar to the one passed to --proxy-server
    #[serde(skip_serializing_if = "Option::is_none", rename = "proxyServer")]
    pub proxy_server: Option<Cow<'a, str>>,
    /// Proxy bypass list, similar to the one passed to --proxy-bypass-list
    #[serde(skip_serializing_if = "Option::is_none", rename = "proxyBypassList")]
    pub proxy_bypass_list: Option<Cow<'a, str>>,
    /// An optional list of origins to grant unlimited cross-origin access to.
    /// Parts of the URL other than those constituting origin are ignored.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originsWithUniversalNetworkAccess")]
    pub origins_with_universal_network_access: Option<Vec<Cow<'a, str>>>,
}
/// Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
/// one.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrowserContextReturns<'a> {
    /// The id of the context created.
    #[serde(rename = "browserContextId")]
    pub browser_context_id: crate::browser::BrowserContextID<'a>,
}
/// Returns all browser contexts created with 'Target.createBrowserContext' method.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.getBrowserContexts", response = "GetBrowserContextsReturns<'a>")]
pub struct GetBrowserContextsParams {

}
/// Returns all browser contexts created with 'Target.createBrowserContext' method.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserContextsReturns<'a> {
    /// An array of browser context ids.
    #[serde(rename = "browserContextIds")]
    pub browser_context_ids: Vec<crate::browser::BrowserContextID<'a>>,
    /// The id of the default browser context if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "defaultBrowserContextId")]
    pub default_browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}
/// Creates a new page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.createTarget", response = "CreateTargetReturns<'a>")]
pub struct CreateTargetParams<'a> {
    /// The initial URL the page will be navigated to. An empty string indicates about:blank.
    pub url: Cow<'a, str>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "windowState")]
    pub window_state: Option<WindowState>,
    /// The browser context to create the page in.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
    /// Whether BeginFrames for this target will be controlled via DevTools (headless shell only,
    /// not supported on MacOS yet, false by default).
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableBeginFrameControl")]
    pub enable_begin_frame_control: Option<bool>,
    /// Whether to create a new Window or Tab (false by default, not supported by headless shell).
    #[serde(skip_serializing_if = "Option::is_none", rename = "newWindow")]
    pub new_window: Option<bool>,
    /// Whether to create the target in background or foreground (false by default, not supported
    /// by headless shell).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// Whether to create the target of type "tab".
    #[serde(skip_serializing_if = "Option::is_none", rename = "forTab")]
    pub for_tab: Option<bool>,
    /// Whether to create a hidden target. The hidden target is observable via protocol, but not
    /// present in the tab UI strip. Cannot be created with 'forTab: true', 'newWindow: true' or
    /// 'background: false'. The life-time of the tab is limited to the life-time of the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// If specified, determines whether the new target should be focused.
    /// By default, the focus behavior depends on the 'background' parameter:
    /// - If 'background' is false (default) and 'focus' is omitted, the new target is focused and the browser window is brought to the foreground.
    /// - If 'background' is false and 'focus' is false, the target is opened but the browser window's focus remains unchanged (e.g., if the window was in the background, it stays there).
    /// - If 'background' is true, setting 'focus' to true is not supported and will result in an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<bool>,
}
/// Creates a new page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetReturns<'a> {
    /// The id of the page opened.
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
}
/// Detaches session with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.detachFromTarget")]
pub struct DetachFromTargetParams<'a> {
    /// Session to detach.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sessionId")]
    pub session_id: Option<SessionID<'a>>,
    /// Deprecated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    pub target_id: Option<TargetID<'a>>,
}
/// Deletes a BrowserContext. All the belonging pages will be closed without calling their
/// beforeunload hooks.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.disposeBrowserContext")]
pub struct DisposeBrowserContextParams<'a> {
    #[serde(rename = "browserContextId")]
    pub browser_context_id: crate::browser::BrowserContextID<'a>,
}
/// Returns information about a target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.getTargetInfo", response = "GetTargetInfoReturns<'a>")]
pub struct GetTargetInfoParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    pub target_id: Option<TargetID<'a>>,
}
/// Returns information about a target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetInfoReturns<'a> {
    #[serde(rename = "targetInfo")]
    pub target_info: TargetInfo<'a>,
}
/// Retrieves a list of available targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.getTargets", response = "GetTargetsReturns<'a>")]
pub struct GetTargetsParams<'a> {
    /// Only targets matching filter will be reported. If filter is not specified
    /// and target discovery is currently enabled, a filter used for target discovery
    /// is used for consistency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TargetFilter<'a>>,
}
/// Retrieves a list of available targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetsReturns<'a> {
    /// The list of targets.
    #[serde(rename = "targetInfos")]
    pub target_infos: Vec<TargetInfo<'a>>,
}
/// Sends protocol message over session with given id.
/// Consider using flat mode instead; see commands attachToTarget, setAutoAttach,
/// and crbug.com/991325.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.sendMessageToTarget")]
pub struct SendMessageToTargetParams<'a> {
    pub message: Cow<'a, str>,
    /// Identifier of the session.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sessionId")]
    pub session_id: Option<SessionID<'a>>,
    /// Deprecated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    pub target_id: Option<TargetID<'a>>,
}
/// Controls whether to automatically attach to new targets which are considered
/// to be directly related to this one (for example, iframes or workers).
/// When turned on, attaches to all existing related targets as well. When turned off,
/// automatically detaches from all currently attached targets.
/// This also clears all targets added by 'autoAttachRelated' from the list of targets to watch
/// for creation of related targets.
/// You might want to call this recursively for auto-attached targets to attach
/// to all available targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.setAutoAttach")]
pub struct SetAutoAttachParams<'a> {
    /// Whether to auto-attach to related targets.
    #[serde(rename = "autoAttach")]
    pub auto_attach: bool,
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    #[serde(rename = "waitForDebuggerOnStart")]
    pub wait_for_debugger_on_start: bool,
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
    /// Only targets matching filter will be attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TargetFilter<'a>>,
}
/// Adds the specified target to the list of targets that will be monitored for any related target
/// creation (such as child frames, child workers and new versions of service worker) and reported
/// through 'attachedToTarget'. The specified target is also auto-attached.
/// This cancels the effect of any previous 'setAutoAttach' and is also cancelled by subsequent
/// 'setAutoAttach'. Only available at the Browser target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.autoAttachRelated")]
pub struct AutoAttachRelatedParams<'a> {
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    #[serde(rename = "waitForDebuggerOnStart")]
    pub wait_for_debugger_on_start: bool,
    /// Only targets matching filter will be attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TargetFilter<'a>>,
}
/// Controls whether to discover available targets and notify via
/// 'targetCreated/targetInfoChanged/targetDestroyed' events.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.setDiscoverTargets")]
pub struct SetDiscoverTargetsParams<'a> {
    /// Whether to discover available targets.
    pub discover: bool,
    /// Only targets matching filter will be attached. If 'discover' is false,
    /// 'filter' must be omitted or empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TargetFilter<'a>>,
}
/// Enables target discovery for the specified locations, when 'setDiscoverTargets' was set to
/// 'true'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.setRemoteLocations")]
pub struct SetRemoteLocationsParams<'a> {
    /// List of remote locations.
    pub locations: Vec<RemoteLocation<'a>>,
}
/// Gets the targetId of the DevTools page target opened for the given target
/// (if any).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.getDevToolsTarget", response = "GetDevToolsTargetReturns<'a>")]
pub struct GetDevToolsTargetParams<'a> {
    /// Page or tab target ID.
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
}
/// Gets the targetId of the DevTools page target opened for the given target
/// (if any).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetDevToolsTargetReturns<'a> {
    /// The targetId of DevTools page target if exists.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    pub target_id: Option<TargetID<'a>>,
}
/// Opens a DevTools window for the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.openDevTools", response = "OpenDevToolsReturns<'a>")]
pub struct OpenDevToolsParams<'a> {
    /// This can be the page or tab target ID.
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
    /// The id of the panel we want DevTools to open initially. Currently
    /// supported panels are elements, console, network, sources, resources,
    /// timeline, chrome-recorder, heap-profiler, lighthouse, and security.
    #[serde(skip_serializing_if = "Option::is_none", rename = "panelId")]
    pub panel_id: Option<Cow<'a, str>>,
}
/// Opens a DevTools window for the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct OpenDevToolsReturns<'a> {
    /// The targetId of DevTools page target.
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
}
/// Issued when attached to target because of auto-attach or 'attachToTarget' command.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.attachedToTarget")]
pub struct AttachedToTarget<'a> {
    /// Identifier assigned to the session used to send/receive messages.
    #[serde(rename = "sessionId")]
    pub session_id: SessionID<'a>,
    #[serde(rename = "targetInfo")]
    pub target_info: TargetInfo<'a>,
    #[serde(rename = "waitingForDebugger")]
    pub waiting_for_debugger: bool,
}
/// Issued when detached from target for any reason (including 'detachFromTarget' command). Can be
/// issued multiple times per target if multiple sessions have been attached to it.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.detachedFromTarget")]
pub struct DetachedFromTarget<'a> {
    /// Detached session identifier.
    #[serde(rename = "sessionId")]
    pub session_id: SessionID<'a>,
    /// Deprecated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    pub target_id: Option<TargetID<'a>>,
}
/// Notifies about a new protocol message received from the session (as reported in
/// 'attachedToTarget' event).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.receivedMessageFromTarget")]
pub struct ReceivedMessageFromTarget<'a> {
    /// Identifier of a session which sends a message.
    #[serde(rename = "sessionId")]
    pub session_id: SessionID<'a>,
    pub message: Cow<'a, str>,
    /// Deprecated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    pub target_id: Option<TargetID<'a>>,
}
/// Issued when a possible inspection target is created.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.targetCreated")]
pub struct TargetCreated<'a> {
    #[serde(rename = "targetInfo")]
    pub target_info: TargetInfo<'a>,
}
/// Issued when a target is destroyed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.targetDestroyed")]
pub struct TargetDestroyed<'a> {
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
}
/// Issued when a target has crashed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.targetCrashed")]
pub struct TargetCrashed<'a> {
    #[serde(rename = "targetId")]
    pub target_id: TargetID<'a>,
    /// Termination status type.
    pub status: Cow<'a, str>,
    /// Termination error code.
    #[serde(rename = "errorCode")]
    pub error_code: i64,
}
/// Issued when some information about a target has changed. This only happens between
/// 'targetCreated' and 'targetDestroyed'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Target.targetInfoChanged")]
pub struct TargetInfoChanged<'a> {
    #[serde(rename = "targetInfo")]
    pub target_info: TargetInfo<'a>,
}