//! Supports additional targets discovery and allows to attach to them.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


pub type TargetID<'a> = Cow<'a, str>;

/// Unique identifier of attached debugging session.

pub type SessionID<'a> = Cow<'a, str>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo<'a> {
    targetId: TargetID<'a>,
    /// List of types: https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    title: Cow<'a, str>,
    url: Cow<'a, str>,
    /// Whether the target has an attached client.
    attached: bool,
    /// Opener target Id
    #[serde(skip_serializing_if = "Option::is_none")]
    openerId: Option<TargetID<'a>>,
    /// Whether the target has access to the originating window.
    canAccessOpener: bool,
    /// Frame id of originating window (is only set if target has an opener).
    #[serde(skip_serializing_if = "Option::is_none")]
    openerFrameId: Option<crate::page::FrameId<'a>>,
    /// Id of the parent frame, present for "iframe" and "worker" targets. For nested workers,
    /// this is the "ancestor" frame that created the first worker in the nested chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    parentFrameId: Option<crate::page::FrameId<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
    /// Provides additional details for specific target types. For example, for
    /// the type of "page", this may be set to "prerender".
    #[serde(skip_serializing_if = "Option::is_none")]
    subtype: Option<Cow<'a, str>>,
}

impl<'a> TargetInfo<'a> {
    pub fn builder() -> TargetInfoBuilder<'a> { TargetInfoBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn title(&self) -> &str { self.title.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn attached(&self) -> bool { self.attached }
    pub fn openerId(&self) -> Option<&TargetID<'a>> { self.openerId.as_ref() }
    pub fn canAccessOpener(&self) -> bool { self.canAccessOpener }
    pub fn openerFrameId(&self) -> Option<&crate::page::FrameId<'a>> { self.openerFrameId.as_ref() }
    pub fn parentFrameId(&self) -> Option<&crate::page::FrameId<'a>> { self.parentFrameId.as_ref() }
    pub fn browserContextId(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browserContextId.as_ref() }
    pub fn subtype(&self) -> Option<&str> { self.subtype.as_deref() }
}

#[derive(Default)]
pub struct TargetInfoBuilder<'a> {
    targetId: Option<TargetID<'a>>,
    type_: Option<Cow<'a, str>>,
    title: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    attached: Option<bool>,
    openerId: Option<TargetID<'a>>,
    canAccessOpener: Option<bool>,
    openerFrameId: Option<crate::page::FrameId<'a>>,
    parentFrameId: Option<crate::page::FrameId<'a>>,
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
    subtype: Option<Cow<'a, str>>,
}

impl<'a> TargetInfoBuilder<'a> {
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    /// List of types: https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self { self.title = Some(title.into()); self }
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Whether the target has an attached client.
    pub fn attached(mut self, attached: bool) -> Self { self.attached = Some(attached); self }
    /// Opener target Id
    pub fn openerId(mut self, openerId: TargetID<'a>) -> Self { self.openerId = Some(openerId); self }
    /// Whether the target has access to the originating window.
    pub fn canAccessOpener(mut self, canAccessOpener: bool) -> Self { self.canAccessOpener = Some(canAccessOpener); self }
    /// Frame id of originating window (is only set if target has an opener).
    pub fn openerFrameId(mut self, openerFrameId: crate::page::FrameId<'a>) -> Self { self.openerFrameId = Some(openerFrameId); self }
    /// Id of the parent frame, present for "iframe" and "worker" targets. For nested workers,
    /// this is the "ancestor" frame that created the first worker in the nested chain.
    pub fn parentFrameId(mut self, parentFrameId: crate::page::FrameId<'a>) -> Self { self.parentFrameId = Some(parentFrameId); self }
    pub fn browserContextId(mut self, browserContextId: crate::browser::BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    /// Provides additional details for specific target types. For example, for
    /// the type of "page", this may be set to "prerender".
    pub fn subtype(mut self, subtype: impl Into<Cow<'a, str>>) -> Self { self.subtype = Some(subtype.into()); self }
    pub fn build(self) -> TargetInfo<'a> {
        TargetInfo {
            targetId: self.targetId.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
            title: self.title.unwrap_or_default(),
            url: self.url.unwrap_or_default(),
            attached: self.attached.unwrap_or_default(),
            openerId: self.openerId,
            canAccessOpener: self.canAccessOpener.unwrap_or_default(),
            openerFrameId: self.openerFrameId,
            parentFrameId: self.parentFrameId,
            browserContextId: self.browserContextId,
            subtype: self.subtype,
        }
    }
}

/// A filter used by target query/discovery/auto-attach operations.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilterEntry<'a> {
    /// If set, causes exclusion of matching targets from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<bool>,
    /// If not present, matches any type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    type_: Option<Cow<'a, str>>,
}

impl<'a> FilterEntry<'a> {
    pub fn builder() -> FilterEntryBuilder<'a> { FilterEntryBuilder::default() }
    pub fn exclude(&self) -> Option<bool> { self.exclude }
    pub fn type_(&self) -> Option<&str> { self.type_.as_deref() }
}

#[derive(Default)]
pub struct FilterEntryBuilder<'a> {
    exclude: Option<bool>,
    type_: Option<Cow<'a, str>>,
}

impl<'a> FilterEntryBuilder<'a> {
    /// If set, causes exclusion of matching targets from the list.
    pub fn exclude(mut self, exclude: bool) -> Self { self.exclude = Some(exclude); self }
    /// If not present, matches any type.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    pub fn build(self) -> FilterEntry<'a> {
        FilterEntry {
            exclude: self.exclude,
            type_: self.type_,
        }
    }
}

/// The entries in TargetFilter are matched sequentially against targets and
/// the first entry that matches determines if the target is included or not,
/// depending on the value of 'exclude' field in the entry.
/// If filter is not specified, the one assumed is
/// [{type: "browser", exclude: true}, {type: "tab", exclude: true}, {}]
/// (i.e. include everything but 'browser' and 'tab').

pub type TargetFilter<'a> = Vec<FilterEntry<'a>>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteLocation<'a> {
    host: Cow<'a, str>,
    port: i64,
}

impl<'a> RemoteLocation<'a> {
    pub fn builder() -> RemoteLocationBuilder<'a> { RemoteLocationBuilder::default() }
    pub fn host(&self) -> &str { self.host.as_ref() }
    pub fn port(&self) -> i64 { self.port }
}

#[derive(Default)]
pub struct RemoteLocationBuilder<'a> {
    host: Option<Cow<'a, str>>,
    port: Option<i64>,
}

impl<'a> RemoteLocationBuilder<'a> {
    pub fn host(mut self, host: impl Into<Cow<'a, str>>) -> Self { self.host = Some(host.into()); self }
    pub fn port(mut self, port: i64) -> Self { self.port = Some(port); self }
    pub fn build(self) -> RemoteLocation<'a> {
        RemoteLocation {
            host: self.host.unwrap_or_default(),
            port: self.port.unwrap_or_default(),
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActivateTargetParams<'a> {
    targetId: TargetID<'a>,
}

impl<'a> ActivateTargetParams<'a> {
    pub fn builder() -> ActivateTargetParamsBuilder<'a> { ActivateTargetParamsBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
}

#[derive(Default)]
pub struct ActivateTargetParamsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
}

impl<'a> ActivateTargetParamsBuilder<'a> {
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> ActivateTargetParams<'a> {
        ActivateTargetParams {
            targetId: self.targetId.unwrap_or_default(),
        }
    }
}

impl<'a> ActivateTargetParams<'a> { pub const METHOD: &'static str = "Target.activateTarget"; }

impl<'a> crate::CdpCommand<'a> for ActivateTargetParams<'a> {
    const METHOD: &'static str = "Target.activateTarget";
    type Response = crate::EmptyReturns;
}

/// Attaches to the target with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetParams<'a> {
    targetId: TargetID<'a>,
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    #[serde(skip_serializing_if = "Option::is_none")]
    flatten: Option<bool>,
}

impl<'a> AttachToTargetParams<'a> {
    pub fn builder() -> AttachToTargetParamsBuilder<'a> { AttachToTargetParamsBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
    pub fn flatten(&self) -> Option<bool> { self.flatten }
}

#[derive(Default)]
pub struct AttachToTargetParamsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
    flatten: Option<bool>,
}

impl<'a> AttachToTargetParamsBuilder<'a> {
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    pub fn flatten(mut self, flatten: bool) -> Self { self.flatten = Some(flatten); self }
    pub fn build(self) -> AttachToTargetParams<'a> {
        AttachToTargetParams {
            targetId: self.targetId.unwrap_or_default(),
            flatten: self.flatten,
        }
    }
}

/// Attaches to the target with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetReturns<'a> {
    /// Id assigned to the session.
    sessionId: SessionID<'a>,
}

impl<'a> AttachToTargetReturns<'a> {
    pub fn builder() -> AttachToTargetReturnsBuilder<'a> { AttachToTargetReturnsBuilder::default() }
    pub fn sessionId(&self) -> &SessionID<'a> { &self.sessionId }
}

#[derive(Default)]
pub struct AttachToTargetReturnsBuilder<'a> {
    sessionId: Option<SessionID<'a>>,
}

impl<'a> AttachToTargetReturnsBuilder<'a> {
    /// Id assigned to the session.
    pub fn sessionId(mut self, sessionId: SessionID<'a>) -> Self { self.sessionId = Some(sessionId); self }
    pub fn build(self) -> AttachToTargetReturns<'a> {
        AttachToTargetReturns {
            sessionId: self.sessionId.unwrap_or_default(),
        }
    }
}

impl<'a> AttachToTargetParams<'a> { pub const METHOD: &'static str = "Target.attachToTarget"; }

impl<'a> crate::CdpCommand<'a> for AttachToTargetParams<'a> {
    const METHOD: &'static str = "Target.attachToTarget";
    type Response = AttachToTargetReturns<'a>;
}

/// Attaches to the browser target, only uses flat sessionId mode.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttachToBrowserTargetReturns<'a> {
    /// Id assigned to the session.
    sessionId: SessionID<'a>,
}

impl<'a> AttachToBrowserTargetReturns<'a> {
    pub fn builder() -> AttachToBrowserTargetReturnsBuilder<'a> { AttachToBrowserTargetReturnsBuilder::default() }
    pub fn sessionId(&self) -> &SessionID<'a> { &self.sessionId }
}

#[derive(Default)]
pub struct AttachToBrowserTargetReturnsBuilder<'a> {
    sessionId: Option<SessionID<'a>>,
}

impl<'a> AttachToBrowserTargetReturnsBuilder<'a> {
    /// Id assigned to the session.
    pub fn sessionId(mut self, sessionId: SessionID<'a>) -> Self { self.sessionId = Some(sessionId); self }
    pub fn build(self) -> AttachToBrowserTargetReturns<'a> {
        AttachToBrowserTargetReturns {
            sessionId: self.sessionId.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachToBrowserTargetParams {}

impl AttachToBrowserTargetParams {
    pub fn builder() -> AttachToBrowserTargetParamsBuilder {
        AttachToBrowserTargetParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct AttachToBrowserTargetParamsBuilder {}

impl AttachToBrowserTargetParamsBuilder {
    pub fn build(self) -> AttachToBrowserTargetParams {
        AttachToBrowserTargetParams {}
    }
}

impl AttachToBrowserTargetParams { pub const METHOD: &'static str = "Target.attachToBrowserTarget"; }

impl<'a> crate::CdpCommand<'a> for AttachToBrowserTargetParams {
    const METHOD: &'static str = "Target.attachToBrowserTarget";
    type Response = AttachToBrowserTargetReturns<'a>;
}

/// Closes the target. If the target is a page that gets closed too.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseTargetParams<'a> {
    targetId: TargetID<'a>,
}

impl<'a> CloseTargetParams<'a> {
    pub fn builder() -> CloseTargetParamsBuilder<'a> { CloseTargetParamsBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
}

#[derive(Default)]
pub struct CloseTargetParamsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
}

impl<'a> CloseTargetParamsBuilder<'a> {
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> CloseTargetParams<'a> {
        CloseTargetParams {
            targetId: self.targetId.unwrap_or_default(),
        }
    }
}

/// Closes the target. If the target is a page that gets closed too.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseTargetReturns {
    /// Always set to true. If an error occurs, the response indicates protocol error.
    success: bool,
}

impl CloseTargetReturns {
    pub fn builder() -> CloseTargetReturnsBuilder { CloseTargetReturnsBuilder::default() }
    pub fn success(&self) -> bool { self.success }
}

#[derive(Default)]
pub struct CloseTargetReturnsBuilder {
    success: Option<bool>,
}

impl CloseTargetReturnsBuilder {
    /// Always set to true. If an error occurs, the response indicates protocol error.
    pub fn success(mut self, success: bool) -> Self { self.success = Some(success); self }
    pub fn build(self) -> CloseTargetReturns {
        CloseTargetReturns {
            success: self.success.unwrap_or_default(),
        }
    }
}

impl<'a> CloseTargetParams<'a> { pub const METHOD: &'static str = "Target.closeTarget"; }

impl<'a> crate::CdpCommand<'a> for CloseTargetParams<'a> {
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
pub struct ExposeDevToolsProtocolParams<'a> {
    targetId: TargetID<'a>,
    /// Binding name, 'cdp' if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    bindingName: Option<Cow<'a, str>>,
    /// If true, inherits the current root session's permissions (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    inheritPermissions: Option<bool>,
}

impl<'a> ExposeDevToolsProtocolParams<'a> {
    pub fn builder() -> ExposeDevToolsProtocolParamsBuilder<'a> { ExposeDevToolsProtocolParamsBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
    pub fn bindingName(&self) -> Option<&str> { self.bindingName.as_deref() }
    pub fn inheritPermissions(&self) -> Option<bool> { self.inheritPermissions }
}

#[derive(Default)]
pub struct ExposeDevToolsProtocolParamsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
    bindingName: Option<Cow<'a, str>>,
    inheritPermissions: Option<bool>,
}

impl<'a> ExposeDevToolsProtocolParamsBuilder<'a> {
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    /// Binding name, 'cdp' if not specified.
    pub fn bindingName(mut self, bindingName: impl Into<Cow<'a, str>>) -> Self { self.bindingName = Some(bindingName.into()); self }
    /// If true, inherits the current root session's permissions (default: false).
    pub fn inheritPermissions(mut self, inheritPermissions: bool) -> Self { self.inheritPermissions = Some(inheritPermissions); self }
    pub fn build(self) -> ExposeDevToolsProtocolParams<'a> {
        ExposeDevToolsProtocolParams {
            targetId: self.targetId.unwrap_or_default(),
            bindingName: self.bindingName,
            inheritPermissions: self.inheritPermissions,
        }
    }
}

impl<'a> ExposeDevToolsProtocolParams<'a> { pub const METHOD: &'static str = "Target.exposeDevToolsProtocol"; }

impl<'a> crate::CdpCommand<'a> for ExposeDevToolsProtocolParams<'a> {
    const METHOD: &'static str = "Target.exposeDevToolsProtocol";
    type Response = crate::EmptyReturns;
}

/// Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
/// one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrowserContextParams<'a> {
    /// If specified, disposes this context when debugging session disconnects.
    #[serde(skip_serializing_if = "Option::is_none")]
    disposeOnDetach: Option<bool>,
    /// Proxy server, similar to the one passed to --proxy-server
    #[serde(skip_serializing_if = "Option::is_none")]
    proxyServer: Option<Cow<'a, str>>,
    /// Proxy bypass list, similar to the one passed to --proxy-bypass-list
    #[serde(skip_serializing_if = "Option::is_none")]
    proxyBypassList: Option<Cow<'a, str>>,
    /// An optional list of origins to grant unlimited cross-origin access to.
    /// Parts of the URL other than those constituting origin are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    originsWithUniversalNetworkAccess: Option<Vec<Cow<'a, str>>>,
}

impl<'a> CreateBrowserContextParams<'a> {
    pub fn builder() -> CreateBrowserContextParamsBuilder<'a> { CreateBrowserContextParamsBuilder::default() }
    pub fn disposeOnDetach(&self) -> Option<bool> { self.disposeOnDetach }
    pub fn proxyServer(&self) -> Option<&str> { self.proxyServer.as_deref() }
    pub fn proxyBypassList(&self) -> Option<&str> { self.proxyBypassList.as_deref() }
    pub fn originsWithUniversalNetworkAccess(&self) -> Option<&[Cow<'a, str>]> { self.originsWithUniversalNetworkAccess.as_deref() }
}

#[derive(Default)]
pub struct CreateBrowserContextParamsBuilder<'a> {
    disposeOnDetach: Option<bool>,
    proxyServer: Option<Cow<'a, str>>,
    proxyBypassList: Option<Cow<'a, str>>,
    originsWithUniversalNetworkAccess: Option<Vec<Cow<'a, str>>>,
}

impl<'a> CreateBrowserContextParamsBuilder<'a> {
    /// If specified, disposes this context when debugging session disconnects.
    pub fn disposeOnDetach(mut self, disposeOnDetach: bool) -> Self { self.disposeOnDetach = Some(disposeOnDetach); self }
    /// Proxy server, similar to the one passed to --proxy-server
    pub fn proxyServer(mut self, proxyServer: impl Into<Cow<'a, str>>) -> Self { self.proxyServer = Some(proxyServer.into()); self }
    /// Proxy bypass list, similar to the one passed to --proxy-bypass-list
    pub fn proxyBypassList(mut self, proxyBypassList: impl Into<Cow<'a, str>>) -> Self { self.proxyBypassList = Some(proxyBypassList.into()); self }
    /// An optional list of origins to grant unlimited cross-origin access to.
    /// Parts of the URL other than those constituting origin are ignored.
    pub fn originsWithUniversalNetworkAccess(mut self, originsWithUniversalNetworkAccess: Vec<Cow<'a, str>>) -> Self { self.originsWithUniversalNetworkAccess = Some(originsWithUniversalNetworkAccess); self }
    pub fn build(self) -> CreateBrowserContextParams<'a> {
        CreateBrowserContextParams {
            disposeOnDetach: self.disposeOnDetach,
            proxyServer: self.proxyServer,
            proxyBypassList: self.proxyBypassList,
            originsWithUniversalNetworkAccess: self.originsWithUniversalNetworkAccess,
        }
    }
}

/// Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
/// one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrowserContextReturns<'a> {
    /// The id of the context created.
    browserContextId: crate::browser::BrowserContextID<'a>,
}

impl<'a> CreateBrowserContextReturns<'a> {
    pub fn builder() -> CreateBrowserContextReturnsBuilder<'a> { CreateBrowserContextReturnsBuilder::default() }
    pub fn browserContextId(&self) -> &crate::browser::BrowserContextID<'a> { &self.browserContextId }
}

#[derive(Default)]
pub struct CreateBrowserContextReturnsBuilder<'a> {
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> CreateBrowserContextReturnsBuilder<'a> {
    /// The id of the context created.
    pub fn browserContextId(mut self, browserContextId: crate::browser::BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> CreateBrowserContextReturns<'a> {
        CreateBrowserContextReturns {
            browserContextId: self.browserContextId.unwrap_or_default(),
        }
    }
}

impl<'a> CreateBrowserContextParams<'a> { pub const METHOD: &'static str = "Target.createBrowserContext"; }

impl<'a> crate::CdpCommand<'a> for CreateBrowserContextParams<'a> {
    const METHOD: &'static str = "Target.createBrowserContext";
    type Response = CreateBrowserContextReturns<'a>;
}

/// Returns all browser contexts created with 'Target.createBrowserContext' method.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserContextsReturns<'a> {
    /// An array of browser context ids.
    browserContextIds: Vec<crate::browser::BrowserContextID<'a>>,
    /// The id of the default browser context if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    defaultBrowserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> GetBrowserContextsReturns<'a> {
    pub fn builder() -> GetBrowserContextsReturnsBuilder<'a> { GetBrowserContextsReturnsBuilder::default() }
    pub fn browserContextIds(&self) -> &[crate::browser::BrowserContextID<'a>] { &self.browserContextIds }
    pub fn defaultBrowserContextId(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.defaultBrowserContextId.as_ref() }
}

#[derive(Default)]
pub struct GetBrowserContextsReturnsBuilder<'a> {
    browserContextIds: Option<Vec<crate::browser::BrowserContextID<'a>>>,
    defaultBrowserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> GetBrowserContextsReturnsBuilder<'a> {
    /// An array of browser context ids.
    pub fn browserContextIds(mut self, browserContextIds: Vec<crate::browser::BrowserContextID<'a>>) -> Self { self.browserContextIds = Some(browserContextIds); self }
    /// The id of the default browser context if available.
    pub fn defaultBrowserContextId(mut self, defaultBrowserContextId: crate::browser::BrowserContextID<'a>) -> Self { self.defaultBrowserContextId = Some(defaultBrowserContextId); self }
    pub fn build(self) -> GetBrowserContextsReturns<'a> {
        GetBrowserContextsReturns {
            browserContextIds: self.browserContextIds.unwrap_or_default(),
            defaultBrowserContextId: self.defaultBrowserContextId,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBrowserContextsParams {}

impl GetBrowserContextsParams {
    pub fn builder() -> GetBrowserContextsParamsBuilder {
        GetBrowserContextsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetBrowserContextsParamsBuilder {}

impl GetBrowserContextsParamsBuilder {
    pub fn build(self) -> GetBrowserContextsParams {
        GetBrowserContextsParams {}
    }
}

impl GetBrowserContextsParams { pub const METHOD: &'static str = "Target.getBrowserContexts"; }

impl<'a> crate::CdpCommand<'a> for GetBrowserContextsParams {
    const METHOD: &'static str = "Target.getBrowserContexts";
    type Response = GetBrowserContextsReturns<'a>;
}

/// Creates a new page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetParams<'a> {
    /// The initial URL the page will be navigated to. An empty string indicates about:blank.
    url: Cow<'a, str>,
    /// Frame left origin in DIP (requires newWindow to be true or headless shell).
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<i64>,
    /// Frame top origin in DIP (requires newWindow to be true or headless shell).
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<i64>,
    /// Frame width in DIP (requires newWindow to be true or headless shell).
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u64>,
    /// Frame height in DIP (requires newWindow to be true or headless shell).
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
    /// Frame window state (requires newWindow to be true or headless shell).
    /// Default is normal.
    #[serde(skip_serializing_if = "Option::is_none")]
    windowState: Option<WindowState>,
    /// The browser context to create the page in.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
    /// Whether BeginFrames for this target will be controlled via DevTools (headless shell only,
    /// not supported on MacOS yet, false by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    enableBeginFrameControl: Option<bool>,
    /// Whether to create a new Window or Tab (false by default, not supported by headless shell).
    #[serde(skip_serializing_if = "Option::is_none")]
    newWindow: Option<bool>,
    /// Whether to create the target in background or foreground (false by default, not supported
    /// by headless shell).
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<bool>,
    /// Whether to create the target of type "tab".
    #[serde(skip_serializing_if = "Option::is_none")]
    forTab: Option<bool>,
    /// Whether to create a hidden target. The hidden target is observable via protocol, but not
    /// present in the tab UI strip. Cannot be created with 'forTab: true', 'newWindow: true' or
    /// 'background: false'. The life-time of the tab is limited to the life-time of the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden: Option<bool>,
    /// If specified, the option is used to determine if the new target should
    /// be focused or not. By default, the focus behavior depends on the
    /// value of the background field. For example, background=false and focus=false
    /// will result in the target tab being opened but the browser window remain
    /// unchanged (if it was in the background, it will remain in the background)
    /// and background=false with focus=undefined will result in the window being focused.
    /// Using background: true and focus: true is not supported and will result in an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    focus: Option<bool>,
}

impl<'a> CreateTargetParams<'a> {
    pub fn builder() -> CreateTargetParamsBuilder<'a> { CreateTargetParamsBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn left(&self) -> Option<i64> { self.left }
    pub fn top(&self) -> Option<i64> { self.top }
    pub fn width(&self) -> Option<u64> { self.width }
    pub fn height(&self) -> Option<i64> { self.height }
    pub fn windowState(&self) -> Option<&WindowState> { self.windowState.as_ref() }
    pub fn browserContextId(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browserContextId.as_ref() }
    pub fn enableBeginFrameControl(&self) -> Option<bool> { self.enableBeginFrameControl }
    pub fn newWindow(&self) -> Option<bool> { self.newWindow }
    pub fn background(&self) -> Option<bool> { self.background }
    pub fn forTab(&self) -> Option<bool> { self.forTab }
    pub fn hidden(&self) -> Option<bool> { self.hidden }
    pub fn focus(&self) -> Option<bool> { self.focus }
}

#[derive(Default)]
pub struct CreateTargetParamsBuilder<'a> {
    url: Option<Cow<'a, str>>,
    left: Option<i64>,
    top: Option<i64>,
    width: Option<u64>,
    height: Option<i64>,
    windowState: Option<WindowState>,
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
    enableBeginFrameControl: Option<bool>,
    newWindow: Option<bool>,
    background: Option<bool>,
    forTab: Option<bool>,
    hidden: Option<bool>,
    focus: Option<bool>,
}

impl<'a> CreateTargetParamsBuilder<'a> {
    /// The initial URL the page will be navigated to. An empty string indicates about:blank.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Frame left origin in DIP (requires newWindow to be true or headless shell).
    pub fn left(mut self, left: i64) -> Self { self.left = Some(left); self }
    /// Frame top origin in DIP (requires newWindow to be true or headless shell).
    pub fn top(mut self, top: i64) -> Self { self.top = Some(top); self }
    /// Frame width in DIP (requires newWindow to be true or headless shell).
    pub fn width(mut self, width: u64) -> Self { self.width = Some(width); self }
    /// Frame height in DIP (requires newWindow to be true or headless shell).
    pub fn height(mut self, height: i64) -> Self { self.height = Some(height); self }
    /// Frame window state (requires newWindow to be true or headless shell).
    /// Default is normal.
    pub fn windowState(mut self, windowState: WindowState) -> Self { self.windowState = Some(windowState); self }
    /// The browser context to create the page in.
    pub fn browserContextId(mut self, browserContextId: crate::browser::BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    /// Whether BeginFrames for this target will be controlled via DevTools (headless shell only,
    /// not supported on MacOS yet, false by default).
    pub fn enableBeginFrameControl(mut self, enableBeginFrameControl: bool) -> Self { self.enableBeginFrameControl = Some(enableBeginFrameControl); self }
    /// Whether to create a new Window or Tab (false by default, not supported by headless shell).
    pub fn newWindow(mut self, newWindow: bool) -> Self { self.newWindow = Some(newWindow); self }
    /// Whether to create the target in background or foreground (false by default, not supported
    /// by headless shell).
    pub fn background(mut self, background: bool) -> Self { self.background = Some(background); self }
    /// Whether to create the target of type "tab".
    pub fn forTab(mut self, forTab: bool) -> Self { self.forTab = Some(forTab); self }
    /// Whether to create a hidden target. The hidden target is observable via protocol, but not
    /// present in the tab UI strip. Cannot be created with 'forTab: true', 'newWindow: true' or
    /// 'background: false'. The life-time of the tab is limited to the life-time of the session.
    pub fn hidden(mut self, hidden: bool) -> Self { self.hidden = Some(hidden); self }
    /// If specified, the option is used to determine if the new target should
    /// be focused or not. By default, the focus behavior depends on the
    /// value of the background field. For example, background=false and focus=false
    /// will result in the target tab being opened but the browser window remain
    /// unchanged (if it was in the background, it will remain in the background)
    /// and background=false with focus=undefined will result in the window being focused.
    /// Using background: true and focus: true is not supported and will result in an error.
    pub fn focus(mut self, focus: bool) -> Self { self.focus = Some(focus); self }
    pub fn build(self) -> CreateTargetParams<'a> {
        CreateTargetParams {
            url: self.url.unwrap_or_default(),
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            windowState: self.windowState,
            browserContextId: self.browserContextId,
            enableBeginFrameControl: self.enableBeginFrameControl,
            newWindow: self.newWindow,
            background: self.background,
            forTab: self.forTab,
            hidden: self.hidden,
            focus: self.focus,
        }
    }
}

/// Creates a new page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetReturns<'a> {
    /// The id of the page opened.
    targetId: TargetID<'a>,
}

impl<'a> CreateTargetReturns<'a> {
    pub fn builder() -> CreateTargetReturnsBuilder<'a> { CreateTargetReturnsBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
}

#[derive(Default)]
pub struct CreateTargetReturnsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
}

impl<'a> CreateTargetReturnsBuilder<'a> {
    /// The id of the page opened.
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> CreateTargetReturns<'a> {
        CreateTargetReturns {
            targetId: self.targetId.unwrap_or_default(),
        }
    }
}

impl<'a> CreateTargetParams<'a> { pub const METHOD: &'static str = "Target.createTarget"; }

impl<'a> crate::CdpCommand<'a> for CreateTargetParams<'a> {
    const METHOD: &'static str = "Target.createTarget";
    type Response = CreateTargetReturns<'a>;
}

/// Detaches session with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DetachFromTargetParams<'a> {
    /// Session to detach.
    #[serde(skip_serializing_if = "Option::is_none")]
    sessionId: Option<SessionID<'a>>,
    /// Deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    targetId: Option<TargetID<'a>>,
}

impl<'a> DetachFromTargetParams<'a> {
    pub fn builder() -> DetachFromTargetParamsBuilder<'a> { DetachFromTargetParamsBuilder::default() }
    pub fn sessionId(&self) -> Option<&SessionID<'a>> { self.sessionId.as_ref() }
    pub fn targetId(&self) -> Option<&TargetID<'a>> { self.targetId.as_ref() }
}

#[derive(Default)]
pub struct DetachFromTargetParamsBuilder<'a> {
    sessionId: Option<SessionID<'a>>,
    targetId: Option<TargetID<'a>>,
}

impl<'a> DetachFromTargetParamsBuilder<'a> {
    /// Session to detach.
    pub fn sessionId(mut self, sessionId: SessionID<'a>) -> Self { self.sessionId = Some(sessionId); self }
    /// Deprecated.
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> DetachFromTargetParams<'a> {
        DetachFromTargetParams {
            sessionId: self.sessionId,
            targetId: self.targetId,
        }
    }
}

impl<'a> DetachFromTargetParams<'a> { pub const METHOD: &'static str = "Target.detachFromTarget"; }

impl<'a> crate::CdpCommand<'a> for DetachFromTargetParams<'a> {
    const METHOD: &'static str = "Target.detachFromTarget";
    type Response = crate::EmptyReturns;
}

/// Deletes a BrowserContext. All the belonging pages will be closed without calling their
/// beforeunload hooks.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisposeBrowserContextParams<'a> {
    browserContextId: crate::browser::BrowserContextID<'a>,
}

impl<'a> DisposeBrowserContextParams<'a> {
    pub fn builder() -> DisposeBrowserContextParamsBuilder<'a> { DisposeBrowserContextParamsBuilder::default() }
    pub fn browserContextId(&self) -> &crate::browser::BrowserContextID<'a> { &self.browserContextId }
}

#[derive(Default)]
pub struct DisposeBrowserContextParamsBuilder<'a> {
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> DisposeBrowserContextParamsBuilder<'a> {
    pub fn browserContextId(mut self, browserContextId: crate::browser::BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> DisposeBrowserContextParams<'a> {
        DisposeBrowserContextParams {
            browserContextId: self.browserContextId.unwrap_or_default(),
        }
    }
}

impl<'a> DisposeBrowserContextParams<'a> { pub const METHOD: &'static str = "Target.disposeBrowserContext"; }

impl<'a> crate::CdpCommand<'a> for DisposeBrowserContextParams<'a> {
    const METHOD: &'static str = "Target.disposeBrowserContext";
    type Response = crate::EmptyReturns;
}

/// Returns information about a target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetInfoParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    targetId: Option<TargetID<'a>>,
}

impl<'a> GetTargetInfoParams<'a> {
    pub fn builder() -> GetTargetInfoParamsBuilder<'a> { GetTargetInfoParamsBuilder::default() }
    pub fn targetId(&self) -> Option<&TargetID<'a>> { self.targetId.as_ref() }
}

#[derive(Default)]
pub struct GetTargetInfoParamsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
}

impl<'a> GetTargetInfoParamsBuilder<'a> {
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> GetTargetInfoParams<'a> {
        GetTargetInfoParams {
            targetId: self.targetId,
        }
    }
}

/// Returns information about a target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetInfoReturns<'a> {
    targetInfo: TargetInfo<'a>,
}

impl<'a> GetTargetInfoReturns<'a> {
    pub fn builder() -> GetTargetInfoReturnsBuilder<'a> { GetTargetInfoReturnsBuilder::default() }
    pub fn targetInfo(&self) -> &TargetInfo<'a> { &self.targetInfo }
}

#[derive(Default)]
pub struct GetTargetInfoReturnsBuilder<'a> {
    targetInfo: Option<TargetInfo<'a>>,
}

impl<'a> GetTargetInfoReturnsBuilder<'a> {
    pub fn targetInfo(mut self, targetInfo: TargetInfo<'a>) -> Self { self.targetInfo = Some(targetInfo); self }
    pub fn build(self) -> GetTargetInfoReturns<'a> {
        GetTargetInfoReturns {
            targetInfo: self.targetInfo.unwrap_or_default(),
        }
    }
}

impl<'a> GetTargetInfoParams<'a> { pub const METHOD: &'static str = "Target.getTargetInfo"; }

impl<'a> crate::CdpCommand<'a> for GetTargetInfoParams<'a> {
    const METHOD: &'static str = "Target.getTargetInfo";
    type Response = GetTargetInfoReturns<'a>;
}

/// Retrieves a list of available targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetsParams<'a> {
    /// Only targets matching filter will be reported. If filter is not specified
    /// and target discovery is currently enabled, a filter used for target discovery
    /// is used for consistency.
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<TargetFilter<'a>>,
}

impl<'a> GetTargetsParams<'a> {
    pub fn builder() -> GetTargetsParamsBuilder<'a> { GetTargetsParamsBuilder::default() }
    pub fn filter(&self) -> Option<&TargetFilter<'a>> { self.filter.as_ref() }
}

#[derive(Default)]
pub struct GetTargetsParamsBuilder<'a> {
    filter: Option<TargetFilter<'a>>,
}

impl<'a> GetTargetsParamsBuilder<'a> {
    /// Only targets matching filter will be reported. If filter is not specified
    /// and target discovery is currently enabled, a filter used for target discovery
    /// is used for consistency.
    pub fn filter(mut self, filter: TargetFilter<'a>) -> Self { self.filter = Some(filter); self }
    pub fn build(self) -> GetTargetsParams<'a> {
        GetTargetsParams {
            filter: self.filter,
        }
    }
}

/// Retrieves a list of available targets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetsReturns<'a> {
    /// The list of targets.
    targetInfos: Vec<TargetInfo<'a>>,
}

impl<'a> GetTargetsReturns<'a> {
    pub fn builder() -> GetTargetsReturnsBuilder<'a> { GetTargetsReturnsBuilder::default() }
    pub fn targetInfos(&self) -> &[TargetInfo<'a>] { &self.targetInfos }
}

#[derive(Default)]
pub struct GetTargetsReturnsBuilder<'a> {
    targetInfos: Option<Vec<TargetInfo<'a>>>,
}

impl<'a> GetTargetsReturnsBuilder<'a> {
    /// The list of targets.
    pub fn targetInfos(mut self, targetInfos: Vec<TargetInfo<'a>>) -> Self { self.targetInfos = Some(targetInfos); self }
    pub fn build(self) -> GetTargetsReturns<'a> {
        GetTargetsReturns {
            targetInfos: self.targetInfos.unwrap_or_default(),
        }
    }
}

impl<'a> GetTargetsParams<'a> { pub const METHOD: &'static str = "Target.getTargets"; }

impl<'a> crate::CdpCommand<'a> for GetTargetsParams<'a> {
    const METHOD: &'static str = "Target.getTargets";
    type Response = GetTargetsReturns<'a>;
}

/// Sends protocol message over session with given id.
/// Consider using flat mode instead; see commands attachToTarget, setAutoAttach,
/// and crbug.com/991325.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageToTargetParams<'a> {
    message: Cow<'a, str>,
    /// Identifier of the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    sessionId: Option<SessionID<'a>>,
    /// Deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    targetId: Option<TargetID<'a>>,
}

impl<'a> SendMessageToTargetParams<'a> {
    pub fn builder() -> SendMessageToTargetParamsBuilder<'a> { SendMessageToTargetParamsBuilder::default() }
    pub fn message(&self) -> &str { self.message.as_ref() }
    pub fn sessionId(&self) -> Option<&SessionID<'a>> { self.sessionId.as_ref() }
    pub fn targetId(&self) -> Option<&TargetID<'a>> { self.targetId.as_ref() }
}

#[derive(Default)]
pub struct SendMessageToTargetParamsBuilder<'a> {
    message: Option<Cow<'a, str>>,
    sessionId: Option<SessionID<'a>>,
    targetId: Option<TargetID<'a>>,
}

impl<'a> SendMessageToTargetParamsBuilder<'a> {
    pub fn message(mut self, message: impl Into<Cow<'a, str>>) -> Self { self.message = Some(message.into()); self }
    /// Identifier of the session.
    pub fn sessionId(mut self, sessionId: SessionID<'a>) -> Self { self.sessionId = Some(sessionId); self }
    /// Deprecated.
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> SendMessageToTargetParams<'a> {
        SendMessageToTargetParams {
            message: self.message.unwrap_or_default(),
            sessionId: self.sessionId,
            targetId: self.targetId,
        }
    }
}

impl<'a> SendMessageToTargetParams<'a> { pub const METHOD: &'static str = "Target.sendMessageToTarget"; }

impl<'a> crate::CdpCommand<'a> for SendMessageToTargetParams<'a> {
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
pub struct SetAutoAttachParams<'a> {
    /// Whether to auto-attach to related targets.
    autoAttach: bool,
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    waitForDebuggerOnStart: bool,
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    #[serde(skip_serializing_if = "Option::is_none")]
    flatten: Option<bool>,
    /// Only targets matching filter will be attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<TargetFilter<'a>>,
}

impl<'a> SetAutoAttachParams<'a> {
    pub fn builder() -> SetAutoAttachParamsBuilder<'a> { SetAutoAttachParamsBuilder::default() }
    pub fn autoAttach(&self) -> bool { self.autoAttach }
    pub fn waitForDebuggerOnStart(&self) -> bool { self.waitForDebuggerOnStart }
    pub fn flatten(&self) -> Option<bool> { self.flatten }
    pub fn filter(&self) -> Option<&TargetFilter<'a>> { self.filter.as_ref() }
}

#[derive(Default)]
pub struct SetAutoAttachParamsBuilder<'a> {
    autoAttach: Option<bool>,
    waitForDebuggerOnStart: Option<bool>,
    flatten: Option<bool>,
    filter: Option<TargetFilter<'a>>,
}

impl<'a> SetAutoAttachParamsBuilder<'a> {
    /// Whether to auto-attach to related targets.
    pub fn autoAttach(mut self, autoAttach: bool) -> Self { self.autoAttach = Some(autoAttach); self }
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    pub fn waitForDebuggerOnStart(mut self, waitForDebuggerOnStart: bool) -> Self { self.waitForDebuggerOnStart = Some(waitForDebuggerOnStart); self }
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    pub fn flatten(mut self, flatten: bool) -> Self { self.flatten = Some(flatten); self }
    /// Only targets matching filter will be attached.
    pub fn filter(mut self, filter: TargetFilter<'a>) -> Self { self.filter = Some(filter); self }
    pub fn build(self) -> SetAutoAttachParams<'a> {
        SetAutoAttachParams {
            autoAttach: self.autoAttach.unwrap_or_default(),
            waitForDebuggerOnStart: self.waitForDebuggerOnStart.unwrap_or_default(),
            flatten: self.flatten,
            filter: self.filter,
        }
    }
}

impl<'a> SetAutoAttachParams<'a> { pub const METHOD: &'static str = "Target.setAutoAttach"; }

impl<'a> crate::CdpCommand<'a> for SetAutoAttachParams<'a> {
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
pub struct AutoAttachRelatedParams<'a> {
    targetId: TargetID<'a>,
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    waitForDebuggerOnStart: bool,
    /// Only targets matching filter will be attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<TargetFilter<'a>>,
}

impl<'a> AutoAttachRelatedParams<'a> {
    pub fn builder() -> AutoAttachRelatedParamsBuilder<'a> { AutoAttachRelatedParamsBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
    pub fn waitForDebuggerOnStart(&self) -> bool { self.waitForDebuggerOnStart }
    pub fn filter(&self) -> Option<&TargetFilter<'a>> { self.filter.as_ref() }
}

#[derive(Default)]
pub struct AutoAttachRelatedParamsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
    waitForDebuggerOnStart: Option<bool>,
    filter: Option<TargetFilter<'a>>,
}

impl<'a> AutoAttachRelatedParamsBuilder<'a> {
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    pub fn waitForDebuggerOnStart(mut self, waitForDebuggerOnStart: bool) -> Self { self.waitForDebuggerOnStart = Some(waitForDebuggerOnStart); self }
    /// Only targets matching filter will be attached.
    pub fn filter(mut self, filter: TargetFilter<'a>) -> Self { self.filter = Some(filter); self }
    pub fn build(self) -> AutoAttachRelatedParams<'a> {
        AutoAttachRelatedParams {
            targetId: self.targetId.unwrap_or_default(),
            waitForDebuggerOnStart: self.waitForDebuggerOnStart.unwrap_or_default(),
            filter: self.filter,
        }
    }
}

impl<'a> AutoAttachRelatedParams<'a> { pub const METHOD: &'static str = "Target.autoAttachRelated"; }

impl<'a> crate::CdpCommand<'a> for AutoAttachRelatedParams<'a> {
    const METHOD: &'static str = "Target.autoAttachRelated";
    type Response = crate::EmptyReturns;
}

/// Controls whether to discover available targets and notify via
/// 'targetCreated/targetInfoChanged/targetDestroyed' events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetDiscoverTargetsParams<'a> {
    /// Whether to discover available targets.
    discover: bool,
    /// Only targets matching filter will be attached. If 'discover' is false,
    /// 'filter' must be omitted or empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<TargetFilter<'a>>,
}

impl<'a> SetDiscoverTargetsParams<'a> {
    pub fn builder() -> SetDiscoverTargetsParamsBuilder<'a> { SetDiscoverTargetsParamsBuilder::default() }
    pub fn discover(&self) -> bool { self.discover }
    pub fn filter(&self) -> Option<&TargetFilter<'a>> { self.filter.as_ref() }
}

#[derive(Default)]
pub struct SetDiscoverTargetsParamsBuilder<'a> {
    discover: Option<bool>,
    filter: Option<TargetFilter<'a>>,
}

impl<'a> SetDiscoverTargetsParamsBuilder<'a> {
    /// Whether to discover available targets.
    pub fn discover(mut self, discover: bool) -> Self { self.discover = Some(discover); self }
    /// Only targets matching filter will be attached. If 'discover' is false,
    /// 'filter' must be omitted or empty.
    pub fn filter(mut self, filter: TargetFilter<'a>) -> Self { self.filter = Some(filter); self }
    pub fn build(self) -> SetDiscoverTargetsParams<'a> {
        SetDiscoverTargetsParams {
            discover: self.discover.unwrap_or_default(),
            filter: self.filter,
        }
    }
}

impl<'a> SetDiscoverTargetsParams<'a> { pub const METHOD: &'static str = "Target.setDiscoverTargets"; }

impl<'a> crate::CdpCommand<'a> for SetDiscoverTargetsParams<'a> {
    const METHOD: &'static str = "Target.setDiscoverTargets";
    type Response = crate::EmptyReturns;
}

/// Enables target discovery for the specified locations, when 'setDiscoverTargets' was set to
/// 'true'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetRemoteLocationsParams<'a> {
    /// List of remote locations.
    locations: Vec<RemoteLocation<'a>>,
}

impl<'a> SetRemoteLocationsParams<'a> {
    pub fn builder() -> SetRemoteLocationsParamsBuilder<'a> { SetRemoteLocationsParamsBuilder::default() }
    pub fn locations(&self) -> &[RemoteLocation<'a>] { &self.locations }
}

#[derive(Default)]
pub struct SetRemoteLocationsParamsBuilder<'a> {
    locations: Option<Vec<RemoteLocation<'a>>>,
}

impl<'a> SetRemoteLocationsParamsBuilder<'a> {
    /// List of remote locations.
    pub fn locations(mut self, locations: Vec<RemoteLocation<'a>>) -> Self { self.locations = Some(locations); self }
    pub fn build(self) -> SetRemoteLocationsParams<'a> {
        SetRemoteLocationsParams {
            locations: self.locations.unwrap_or_default(),
        }
    }
}

impl<'a> SetRemoteLocationsParams<'a> { pub const METHOD: &'static str = "Target.setRemoteLocations"; }

impl<'a> crate::CdpCommand<'a> for SetRemoteLocationsParams<'a> {
    const METHOD: &'static str = "Target.setRemoteLocations";
    type Response = crate::EmptyReturns;
}

/// Gets the targetId of the DevTools page target opened for the given target
/// (if any).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDevToolsTargetParams<'a> {
    /// Page or tab target ID.
    targetId: TargetID<'a>,
}

impl<'a> GetDevToolsTargetParams<'a> {
    pub fn builder() -> GetDevToolsTargetParamsBuilder<'a> { GetDevToolsTargetParamsBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
}

#[derive(Default)]
pub struct GetDevToolsTargetParamsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
}

impl<'a> GetDevToolsTargetParamsBuilder<'a> {
    /// Page or tab target ID.
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> GetDevToolsTargetParams<'a> {
        GetDevToolsTargetParams {
            targetId: self.targetId.unwrap_or_default(),
        }
    }
}

/// Gets the targetId of the DevTools page target opened for the given target
/// (if any).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDevToolsTargetReturns<'a> {
    /// The targetId of DevTools page target if exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    targetId: Option<TargetID<'a>>,
}

impl<'a> GetDevToolsTargetReturns<'a> {
    pub fn builder() -> GetDevToolsTargetReturnsBuilder<'a> { GetDevToolsTargetReturnsBuilder::default() }
    pub fn targetId(&self) -> Option<&TargetID<'a>> { self.targetId.as_ref() }
}

#[derive(Default)]
pub struct GetDevToolsTargetReturnsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
}

impl<'a> GetDevToolsTargetReturnsBuilder<'a> {
    /// The targetId of DevTools page target if exists.
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> GetDevToolsTargetReturns<'a> {
        GetDevToolsTargetReturns {
            targetId: self.targetId,
        }
    }
}

impl<'a> GetDevToolsTargetParams<'a> { pub const METHOD: &'static str = "Target.getDevToolsTarget"; }

impl<'a> crate::CdpCommand<'a> for GetDevToolsTargetParams<'a> {
    const METHOD: &'static str = "Target.getDevToolsTarget";
    type Response = GetDevToolsTargetReturns<'a>;
}

/// Opens a DevTools window for the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenDevToolsParams<'a> {
    /// This can be the page or tab target ID.
    targetId: TargetID<'a>,
    /// The id of the panel we want DevTools to open initially. Currently
    /// supported panels are elements, console, network, sources, resources
    /// and performance.
    #[serde(skip_serializing_if = "Option::is_none")]
    panelId: Option<Cow<'a, str>>,
}

impl<'a> OpenDevToolsParams<'a> {
    pub fn builder() -> OpenDevToolsParamsBuilder<'a> { OpenDevToolsParamsBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
    pub fn panelId(&self) -> Option<&str> { self.panelId.as_deref() }
}

#[derive(Default)]
pub struct OpenDevToolsParamsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
    panelId: Option<Cow<'a, str>>,
}

impl<'a> OpenDevToolsParamsBuilder<'a> {
    /// This can be the page or tab target ID.
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    /// The id of the panel we want DevTools to open initially. Currently
    /// supported panels are elements, console, network, sources, resources
    /// and performance.
    pub fn panelId(mut self, panelId: impl Into<Cow<'a, str>>) -> Self { self.panelId = Some(panelId.into()); self }
    pub fn build(self) -> OpenDevToolsParams<'a> {
        OpenDevToolsParams {
            targetId: self.targetId.unwrap_or_default(),
            panelId: self.panelId,
        }
    }
}

/// Opens a DevTools window for the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenDevToolsReturns<'a> {
    /// The targetId of DevTools page target.
    targetId: TargetID<'a>,
}

impl<'a> OpenDevToolsReturns<'a> {
    pub fn builder() -> OpenDevToolsReturnsBuilder<'a> { OpenDevToolsReturnsBuilder::default() }
    pub fn targetId(&self) -> &TargetID<'a> { &self.targetId }
}

#[derive(Default)]
pub struct OpenDevToolsReturnsBuilder<'a> {
    targetId: Option<TargetID<'a>>,
}

impl<'a> OpenDevToolsReturnsBuilder<'a> {
    /// The targetId of DevTools page target.
    pub fn targetId(mut self, targetId: TargetID<'a>) -> Self { self.targetId = Some(targetId); self }
    pub fn build(self) -> OpenDevToolsReturns<'a> {
        OpenDevToolsReturns {
            targetId: self.targetId.unwrap_or_default(),
        }
    }
}

impl<'a> OpenDevToolsParams<'a> { pub const METHOD: &'static str = "Target.openDevTools"; }

impl<'a> crate::CdpCommand<'a> for OpenDevToolsParams<'a> {
    const METHOD: &'static str = "Target.openDevTools";
    type Response = OpenDevToolsReturns<'a>;
}
