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
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
    /// List of types: <https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22>
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    title: Cow<'a, str>,
    url: Cow<'a, str>,
    /// Whether the target has an attached client.
    attached: bool,
    /// Id of the parent target, if any. For example, "iframe" target may have a "page" parent.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    parent_id: Option<TargetID<'a>>,
    /// Opener target Id
    #[serde(skip_serializing_if = "Option::is_none", rename = "openerId")]
    opener_id: Option<TargetID<'a>>,
    /// Whether the target has access to the originating window.
    #[serde(rename = "canAccessOpener")]
    can_access_opener: bool,
    /// Frame id of originating window (is only set if target has an opener).
    #[serde(skip_serializing_if = "Option::is_none", rename = "openerFrameId")]
    opener_frame_id: Option<crate::page::FrameId<'a>>,
    /// Id of the parent frame, present for "iframe" and "worker" targets. For nested workers,
    /// this is the "ancestor" frame that created the first worker in the nested chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentFrameId")]
    parent_frame_id: Option<crate::page::FrameId<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
    /// Provides additional details for specific target types. For example, for
    /// the type of "page", this may be set to "prerender".
    #[serde(skip_serializing_if = "Option::is_none")]
    subtype: Option<Cow<'a, str>>,
}

impl<'a> TargetInfo<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: 
    /// * `type_`: List of types: <https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22>
    /// * `title`: 
    /// * `url`: 
    /// * `attached`: Whether the target has an attached client.
    /// * `can_access_opener`: Whether the target has access to the originating window.
    pub fn builder(target_id: impl Into<TargetID<'a>>, type_: impl Into<Cow<'a, str>>, title: impl Into<Cow<'a, str>>, url: impl Into<Cow<'a, str>>, attached: bool, can_access_opener: bool) -> TargetInfoBuilder<'a> {
        TargetInfoBuilder {
            target_id: target_id.into(),
            type_: type_.into(),
            title: title.into(),
            url: url.into(),
            attached: attached,
            parent_id: None,
            opener_id: None,
            can_access_opener: can_access_opener,
            opener_frame_id: None,
            parent_frame_id: None,
            browser_context_id: None,
            subtype: None,
        }
    }
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
    /// List of types: <https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22>
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn title(&self) -> &str { self.title.as_ref() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Whether the target has an attached client.
    pub fn attached(&self) -> bool { self.attached }
    /// Id of the parent target, if any. For example, "iframe" target may have a "page" parent.
    pub fn parent_id(&self) -> Option<&TargetID<'a>> { self.parent_id.as_ref() }
    /// Opener target Id
    pub fn opener_id(&self) -> Option<&TargetID<'a>> { self.opener_id.as_ref() }
    /// Whether the target has access to the originating window.
    pub fn can_access_opener(&self) -> bool { self.can_access_opener }
    /// Frame id of originating window (is only set if target has an opener).
    pub fn opener_frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.opener_frame_id.as_ref() }
    /// Id of the parent frame, present for "iframe" and "worker" targets. For nested workers,
    /// this is the "ancestor" frame that created the first worker in the nested chain.
    pub fn parent_frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.parent_frame_id.as_ref() }
    pub fn browser_context_id(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browser_context_id.as_ref() }
    /// Provides additional details for specific target types. For example, for
    /// the type of "page", this may be set to "prerender".
    pub fn subtype(&self) -> Option<&str> { self.subtype.as_deref() }
}


pub struct TargetInfoBuilder<'a> {
    target_id: TargetID<'a>,
    type_: Cow<'a, str>,
    title: Cow<'a, str>,
    url: Cow<'a, str>,
    attached: bool,
    parent_id: Option<TargetID<'a>>,
    opener_id: Option<TargetID<'a>>,
    can_access_opener: bool,
    opener_frame_id: Option<crate::page::FrameId<'a>>,
    parent_frame_id: Option<crate::page::FrameId<'a>>,
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
    subtype: Option<Cow<'a, str>>,
}

impl<'a> TargetInfoBuilder<'a> {
    /// Id of the parent target, if any. For example, "iframe" target may have a "page" parent.
    pub fn parent_id(mut self, parent_id: impl Into<TargetID<'a>>) -> Self { self.parent_id = Some(parent_id.into()); self }
    /// Opener target Id
    pub fn opener_id(mut self, opener_id: impl Into<TargetID<'a>>) -> Self { self.opener_id = Some(opener_id.into()); self }
    /// Frame id of originating window (is only set if target has an opener).
    pub fn opener_frame_id(mut self, opener_frame_id: crate::page::FrameId<'a>) -> Self { self.opener_frame_id = Some(opener_frame_id); self }
    /// Id of the parent frame, present for "iframe" and "worker" targets. For nested workers,
    /// this is the "ancestor" frame that created the first worker in the nested chain.
    pub fn parent_frame_id(mut self, parent_frame_id: crate::page::FrameId<'a>) -> Self { self.parent_frame_id = Some(parent_frame_id); self }
    pub fn browser_context_id(mut self, browser_context_id: crate::browser::BrowserContextID<'a>) -> Self { self.browser_context_id = Some(browser_context_id); self }
    /// Provides additional details for specific target types. For example, for
    /// the type of "page", this may be set to "prerender".
    pub fn subtype(mut self, subtype: impl Into<Cow<'a, str>>) -> Self { self.subtype = Some(subtype.into()); self }
    pub fn build(self) -> TargetInfo<'a> {
        TargetInfo {
            target_id: self.target_id,
            type_: self.type_,
            title: self.title,
            url: self.url,
            attached: self.attached,
            parent_id: self.parent_id,
            opener_id: self.opener_id,
            can_access_opener: self.can_access_opener,
            opener_frame_id: self.opener_frame_id,
            parent_frame_id: self.parent_frame_id,
            browser_context_id: self.browser_context_id,
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
    /// Creates a builder for this type.
    pub fn builder() -> FilterEntryBuilder<'a> {
        FilterEntryBuilder {
            exclude: None,
            type_: None,
        }
    }
    /// If set, causes exclusion of matching targets from the list.
    pub fn exclude(&self) -> Option<bool> { self.exclude }
    /// If not present, matches any type.
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
/// \[{type: "browser", exclude: true}, {type: "tab", exclude: true}, {}\]
/// (i.e. include everything but 'browser' and 'tab').

pub type TargetFilter<'a> = Vec<FilterEntry<'a>>;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteLocation<'a> {
    host: Cow<'a, str>,
    port: i64,
}

impl<'a> RemoteLocation<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `host`: 
    /// * `port`: 
    pub fn builder(host: impl Into<Cow<'a, str>>, port: i64) -> RemoteLocationBuilder<'a> {
        RemoteLocationBuilder {
            host: host.into(),
            port: port,
        }
    }
    pub fn host(&self) -> &str { self.host.as_ref() }
    pub fn port(&self) -> i64 { self.port }
}


pub struct RemoteLocationBuilder<'a> {
    host: Cow<'a, str>,
    port: i64,
}

impl<'a> RemoteLocationBuilder<'a> {
    pub fn build(self) -> RemoteLocation<'a> {
        RemoteLocation {
            host: self.host,
            port: self.port,
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
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
}

impl<'a> ActivateTargetParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: 
    pub fn builder(target_id: impl Into<TargetID<'a>>) -> ActivateTargetParamsBuilder<'a> {
        ActivateTargetParamsBuilder {
            target_id: target_id.into(),
        }
    }
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
}


pub struct ActivateTargetParamsBuilder<'a> {
    target_id: TargetID<'a>,
}

impl<'a> ActivateTargetParamsBuilder<'a> {
    pub fn build(self) -> ActivateTargetParams<'a> {
        ActivateTargetParams {
            target_id: self.target_id,
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
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    #[serde(skip_serializing_if = "Option::is_none")]
    flatten: Option<bool>,
}

impl<'a> AttachToTargetParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: 
    pub fn builder(target_id: impl Into<TargetID<'a>>) -> AttachToTargetParamsBuilder<'a> {
        AttachToTargetParamsBuilder {
            target_id: target_id.into(),
            flatten: None,
        }
    }
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    pub fn flatten(&self) -> Option<bool> { self.flatten }
}


pub struct AttachToTargetParamsBuilder<'a> {
    target_id: TargetID<'a>,
    flatten: Option<bool>,
}

impl<'a> AttachToTargetParamsBuilder<'a> {
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    pub fn flatten(mut self, flatten: bool) -> Self { self.flatten = Some(flatten); self }
    pub fn build(self) -> AttachToTargetParams<'a> {
        AttachToTargetParams {
            target_id: self.target_id,
            flatten: self.flatten,
        }
    }
}

/// Attaches to the target with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetReturns<'a> {
    /// Id assigned to the session.
    #[serde(rename = "sessionId")]
    session_id: SessionID<'a>,
}

impl<'a> AttachToTargetReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `session_id`: Id assigned to the session.
    pub fn builder(session_id: impl Into<SessionID<'a>>) -> AttachToTargetReturnsBuilder<'a> {
        AttachToTargetReturnsBuilder {
            session_id: session_id.into(),
        }
    }
    /// Id assigned to the session.
    pub fn session_id(&self) -> &SessionID<'a> { &self.session_id }
}


pub struct AttachToTargetReturnsBuilder<'a> {
    session_id: SessionID<'a>,
}

impl<'a> AttachToTargetReturnsBuilder<'a> {
    pub fn build(self) -> AttachToTargetReturns<'a> {
        AttachToTargetReturns {
            session_id: self.session_id,
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
    #[serde(rename = "sessionId")]
    session_id: SessionID<'a>,
}

impl<'a> AttachToBrowserTargetReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `session_id`: Id assigned to the session.
    pub fn builder(session_id: impl Into<SessionID<'a>>) -> AttachToBrowserTargetReturnsBuilder<'a> {
        AttachToBrowserTargetReturnsBuilder {
            session_id: session_id.into(),
        }
    }
    /// Id assigned to the session.
    pub fn session_id(&self) -> &SessionID<'a> { &self.session_id }
}


pub struct AttachToBrowserTargetReturnsBuilder<'a> {
    session_id: SessionID<'a>,
}

impl<'a> AttachToBrowserTargetReturnsBuilder<'a> {
    pub fn build(self) -> AttachToBrowserTargetReturns<'a> {
        AttachToBrowserTargetReturns {
            session_id: self.session_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachToBrowserTargetParams {}

impl AttachToBrowserTargetParams { pub const METHOD: &'static str = "Target.attachToBrowserTarget"; }

impl<'a> crate::CdpCommand<'a> for AttachToBrowserTargetParams {
    const METHOD: &'static str = "Target.attachToBrowserTarget";
    type Response = AttachToBrowserTargetReturns<'a>;
}

/// Closes the target. If the target is a page that gets closed too.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseTargetParams<'a> {
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
}

impl<'a> CloseTargetParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: 
    pub fn builder(target_id: impl Into<TargetID<'a>>) -> CloseTargetParamsBuilder<'a> {
        CloseTargetParamsBuilder {
            target_id: target_id.into(),
        }
    }
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
}


pub struct CloseTargetParamsBuilder<'a> {
    target_id: TargetID<'a>,
}

impl<'a> CloseTargetParamsBuilder<'a> {
    pub fn build(self) -> CloseTargetParams<'a> {
        CloseTargetParams {
            target_id: self.target_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `success`: Always set to true. If an error occurs, the response indicates protocol error.
    pub fn builder(success: bool) -> CloseTargetReturnsBuilder {
        CloseTargetReturnsBuilder {
            success: success,
        }
    }
    /// Always set to true. If an error occurs, the response indicates protocol error.
    pub fn success(&self) -> bool { self.success }
}


pub struct CloseTargetReturnsBuilder {
    success: bool,
}

impl CloseTargetReturnsBuilder {
    pub fn build(self) -> CloseTargetReturns {
        CloseTargetReturns {
            success: self.success,
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
/// Injected object will be available as 'window\[bindingName\]'.
/// 
/// The object has the following API:
/// - 'binding.send(json)' - a method to send messages over the remote debugging protocol
/// - 'binding.onmessage = json =\> handleMessage(json)' - a callback that will be called for the protocol notifications and command responses.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExposeDevToolsProtocolParams<'a> {
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
    /// Binding name, 'cdp' if not specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bindingName")]
    binding_name: Option<Cow<'a, str>>,
    /// If true, inherits the current root session's permissions (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "inheritPermissions")]
    inherit_permissions: Option<bool>,
}

impl<'a> ExposeDevToolsProtocolParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: 
    pub fn builder(target_id: impl Into<TargetID<'a>>) -> ExposeDevToolsProtocolParamsBuilder<'a> {
        ExposeDevToolsProtocolParamsBuilder {
            target_id: target_id.into(),
            binding_name: None,
            inherit_permissions: None,
        }
    }
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
    /// Binding name, 'cdp' if not specified.
    pub fn binding_name(&self) -> Option<&str> { self.binding_name.as_deref() }
    /// If true, inherits the current root session's permissions (default: false).
    pub fn inherit_permissions(&self) -> Option<bool> { self.inherit_permissions }
}


pub struct ExposeDevToolsProtocolParamsBuilder<'a> {
    target_id: TargetID<'a>,
    binding_name: Option<Cow<'a, str>>,
    inherit_permissions: Option<bool>,
}

impl<'a> ExposeDevToolsProtocolParamsBuilder<'a> {
    /// Binding name, 'cdp' if not specified.
    pub fn binding_name(mut self, binding_name: impl Into<Cow<'a, str>>) -> Self { self.binding_name = Some(binding_name.into()); self }
    /// If true, inherits the current root session's permissions (default: false).
    pub fn inherit_permissions(mut self, inherit_permissions: bool) -> Self { self.inherit_permissions = Some(inherit_permissions); self }
    pub fn build(self) -> ExposeDevToolsProtocolParams<'a> {
        ExposeDevToolsProtocolParams {
            target_id: self.target_id,
            binding_name: self.binding_name,
            inherit_permissions: self.inherit_permissions,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "disposeOnDetach")]
    dispose_on_detach: Option<bool>,
    /// Proxy server, similar to the one passed to --proxy-server
    #[serde(skip_serializing_if = "Option::is_none", rename = "proxyServer")]
    proxy_server: Option<Cow<'a, str>>,
    /// Proxy bypass list, similar to the one passed to --proxy-bypass-list
    #[serde(skip_serializing_if = "Option::is_none", rename = "proxyBypassList")]
    proxy_bypass_list: Option<Cow<'a, str>>,
    /// An optional list of origins to grant unlimited cross-origin access to.
    /// Parts of the URL other than those constituting origin are ignored.
    #[serde(skip_serializing_if = "Option::is_none", rename = "originsWithUniversalNetworkAccess")]
    origins_with_universal_network_access: Option<Vec<Cow<'a, str>>>,
}

impl<'a> CreateBrowserContextParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> CreateBrowserContextParamsBuilder<'a> {
        CreateBrowserContextParamsBuilder {
            dispose_on_detach: None,
            proxy_server: None,
            proxy_bypass_list: None,
            origins_with_universal_network_access: None,
        }
    }
    /// If specified, disposes this context when debugging session disconnects.
    pub fn dispose_on_detach(&self) -> Option<bool> { self.dispose_on_detach }
    /// Proxy server, similar to the one passed to --proxy-server
    pub fn proxy_server(&self) -> Option<&str> { self.proxy_server.as_deref() }
    /// Proxy bypass list, similar to the one passed to --proxy-bypass-list
    pub fn proxy_bypass_list(&self) -> Option<&str> { self.proxy_bypass_list.as_deref() }
    /// An optional list of origins to grant unlimited cross-origin access to.
    /// Parts of the URL other than those constituting origin are ignored.
    pub fn origins_with_universal_network_access(&self) -> Option<&[Cow<'a, str>]> { self.origins_with_universal_network_access.as_deref() }
}

#[derive(Default)]
pub struct CreateBrowserContextParamsBuilder<'a> {
    dispose_on_detach: Option<bool>,
    proxy_server: Option<Cow<'a, str>>,
    proxy_bypass_list: Option<Cow<'a, str>>,
    origins_with_universal_network_access: Option<Vec<Cow<'a, str>>>,
}

impl<'a> CreateBrowserContextParamsBuilder<'a> {
    /// If specified, disposes this context when debugging session disconnects.
    pub fn dispose_on_detach(mut self, dispose_on_detach: bool) -> Self { self.dispose_on_detach = Some(dispose_on_detach); self }
    /// Proxy server, similar to the one passed to --proxy-server
    pub fn proxy_server(mut self, proxy_server: impl Into<Cow<'a, str>>) -> Self { self.proxy_server = Some(proxy_server.into()); self }
    /// Proxy bypass list, similar to the one passed to --proxy-bypass-list
    pub fn proxy_bypass_list(mut self, proxy_bypass_list: impl Into<Cow<'a, str>>) -> Self { self.proxy_bypass_list = Some(proxy_bypass_list.into()); self }
    /// An optional list of origins to grant unlimited cross-origin access to.
    /// Parts of the URL other than those constituting origin are ignored.
    pub fn origins_with_universal_network_access(mut self, origins_with_universal_network_access: Vec<Cow<'a, str>>) -> Self { self.origins_with_universal_network_access = Some(origins_with_universal_network_access); self }
    pub fn build(self) -> CreateBrowserContextParams<'a> {
        CreateBrowserContextParams {
            dispose_on_detach: self.dispose_on_detach,
            proxy_server: self.proxy_server,
            proxy_bypass_list: self.proxy_bypass_list,
            origins_with_universal_network_access: self.origins_with_universal_network_access,
        }
    }
}

/// Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
/// one.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrowserContextReturns<'a> {
    /// The id of the context created.
    #[serde(rename = "browserContextId")]
    browser_context_id: crate::browser::BrowserContextID<'a>,
}

impl<'a> CreateBrowserContextReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `browser_context_id`: The id of the context created.
    pub fn builder(browser_context_id: crate::browser::BrowserContextID<'a>) -> CreateBrowserContextReturnsBuilder<'a> {
        CreateBrowserContextReturnsBuilder {
            browser_context_id: browser_context_id,
        }
    }
    /// The id of the context created.
    pub fn browser_context_id(&self) -> &crate::browser::BrowserContextID<'a> { &self.browser_context_id }
}


pub struct CreateBrowserContextReturnsBuilder<'a> {
    browser_context_id: crate::browser::BrowserContextID<'a>,
}

impl<'a> CreateBrowserContextReturnsBuilder<'a> {
    pub fn build(self) -> CreateBrowserContextReturns<'a> {
        CreateBrowserContextReturns {
            browser_context_id: self.browser_context_id,
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
    #[serde(rename = "browserContextIds")]
    browser_context_ids: Vec<crate::browser::BrowserContextID<'a>>,
    /// The id of the default browser context if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "defaultBrowserContextId")]
    default_browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> GetBrowserContextsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `browser_context_ids`: An array of browser context ids.
    pub fn builder(browser_context_ids: Vec<crate::browser::BrowserContextID<'a>>) -> GetBrowserContextsReturnsBuilder<'a> {
        GetBrowserContextsReturnsBuilder {
            browser_context_ids: browser_context_ids,
            default_browser_context_id: None,
        }
    }
    /// An array of browser context ids.
    pub fn browser_context_ids(&self) -> &[crate::browser::BrowserContextID<'a>] { &self.browser_context_ids }
    /// The id of the default browser context if available.
    pub fn default_browser_context_id(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.default_browser_context_id.as_ref() }
}


pub struct GetBrowserContextsReturnsBuilder<'a> {
    browser_context_ids: Vec<crate::browser::BrowserContextID<'a>>,
    default_browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> GetBrowserContextsReturnsBuilder<'a> {
    /// The id of the default browser context if available.
    pub fn default_browser_context_id(mut self, default_browser_context_id: crate::browser::BrowserContextID<'a>) -> Self { self.default_browser_context_id = Some(default_browser_context_id); self }
    pub fn build(self) -> GetBrowserContextsReturns<'a> {
        GetBrowserContextsReturns {
            browser_context_ids: self.browser_context_ids,
            default_browser_context_id: self.default_browser_context_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBrowserContextsParams {}

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
    #[serde(skip_serializing_if = "Option::is_none", rename = "windowState")]
    window_state: Option<WindowState>,
    /// The browser context to create the page in.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
    /// Whether BeginFrames for this target will be controlled via DevTools (headless shell only,
    /// not supported on MacOS yet, false by default).
    #[serde(skip_serializing_if = "Option::is_none", rename = "enableBeginFrameControl")]
    enable_begin_frame_control: Option<bool>,
    /// Whether to create a new Window or Tab (false by default, not supported by headless shell).
    #[serde(skip_serializing_if = "Option::is_none", rename = "newWindow")]
    new_window: Option<bool>,
    /// Whether to create the target in background or foreground (false by default, not supported
    /// by headless shell).
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<bool>,
    /// Whether to create the target of type "tab".
    #[serde(skip_serializing_if = "Option::is_none", rename = "forTab")]
    for_tab: Option<bool>,
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
    /// Creates a builder for this type with the required parameters:
    /// * `url`: The initial URL the page will be navigated to. An empty string indicates about:blank.
    pub fn builder(url: impl Into<Cow<'a, str>>) -> CreateTargetParamsBuilder<'a> {
        CreateTargetParamsBuilder {
            url: url.into(),
            left: None,
            top: None,
            width: None,
            height: None,
            window_state: None,
            browser_context_id: None,
            enable_begin_frame_control: None,
            new_window: None,
            background: None,
            for_tab: None,
            hidden: None,
            focus: None,
        }
    }
    /// The initial URL the page will be navigated to. An empty string indicates about:blank.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Frame left origin in DIP (requires newWindow to be true or headless shell).
    pub fn left(&self) -> Option<i64> { self.left }
    /// Frame top origin in DIP (requires newWindow to be true or headless shell).
    pub fn top(&self) -> Option<i64> { self.top }
    /// Frame width in DIP (requires newWindow to be true or headless shell).
    pub fn width(&self) -> Option<u64> { self.width }
    /// Frame height in DIP (requires newWindow to be true or headless shell).
    pub fn height(&self) -> Option<i64> { self.height }
    /// Frame window state (requires newWindow to be true or headless shell).
    /// Default is normal.
    pub fn window_state(&self) -> Option<&WindowState> { self.window_state.as_ref() }
    /// The browser context to create the page in.
    pub fn browser_context_id(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browser_context_id.as_ref() }
    /// Whether BeginFrames for this target will be controlled via DevTools (headless shell only,
    /// not supported on MacOS yet, false by default).
    pub fn enable_begin_frame_control(&self) -> Option<bool> { self.enable_begin_frame_control }
    /// Whether to create a new Window or Tab (false by default, not supported by headless shell).
    pub fn new_window(&self) -> Option<bool> { self.new_window }
    /// Whether to create the target in background or foreground (false by default, not supported
    /// by headless shell).
    pub fn background(&self) -> Option<bool> { self.background }
    /// Whether to create the target of type "tab".
    pub fn for_tab(&self) -> Option<bool> { self.for_tab }
    /// Whether to create a hidden target. The hidden target is observable via protocol, but not
    /// present in the tab UI strip. Cannot be created with 'forTab: true', 'newWindow: true' or
    /// 'background: false'. The life-time of the tab is limited to the life-time of the session.
    pub fn hidden(&self) -> Option<bool> { self.hidden }
    /// If specified, the option is used to determine if the new target should
    /// be focused or not. By default, the focus behavior depends on the
    /// value of the background field. For example, background=false and focus=false
    /// will result in the target tab being opened but the browser window remain
    /// unchanged (if it was in the background, it will remain in the background)
    /// and background=false with focus=undefined will result in the window being focused.
    /// Using background: true and focus: true is not supported and will result in an error.
    pub fn focus(&self) -> Option<bool> { self.focus }
}


pub struct CreateTargetParamsBuilder<'a> {
    url: Cow<'a, str>,
    left: Option<i64>,
    top: Option<i64>,
    width: Option<u64>,
    height: Option<i64>,
    window_state: Option<WindowState>,
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
    enable_begin_frame_control: Option<bool>,
    new_window: Option<bool>,
    background: Option<bool>,
    for_tab: Option<bool>,
    hidden: Option<bool>,
    focus: Option<bool>,
}

impl<'a> CreateTargetParamsBuilder<'a> {
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
    pub fn window_state(mut self, window_state: impl Into<WindowState>) -> Self { self.window_state = Some(window_state.into()); self }
    /// The browser context to create the page in.
    pub fn browser_context_id(mut self, browser_context_id: crate::browser::BrowserContextID<'a>) -> Self { self.browser_context_id = Some(browser_context_id); self }
    /// Whether BeginFrames for this target will be controlled via DevTools (headless shell only,
    /// not supported on MacOS yet, false by default).
    pub fn enable_begin_frame_control(mut self, enable_begin_frame_control: bool) -> Self { self.enable_begin_frame_control = Some(enable_begin_frame_control); self }
    /// Whether to create a new Window or Tab (false by default, not supported by headless shell).
    pub fn new_window(mut self, new_window: bool) -> Self { self.new_window = Some(new_window); self }
    /// Whether to create the target in background or foreground (false by default, not supported
    /// by headless shell).
    pub fn background(mut self, background: bool) -> Self { self.background = Some(background); self }
    /// Whether to create the target of type "tab".
    pub fn for_tab(mut self, for_tab: bool) -> Self { self.for_tab = Some(for_tab); self }
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
            url: self.url,
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            window_state: self.window_state,
            browser_context_id: self.browser_context_id,
            enable_begin_frame_control: self.enable_begin_frame_control,
            new_window: self.new_window,
            background: self.background,
            for_tab: self.for_tab,
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
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
}

impl<'a> CreateTargetReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: The id of the page opened.
    pub fn builder(target_id: impl Into<TargetID<'a>>) -> CreateTargetReturnsBuilder<'a> {
        CreateTargetReturnsBuilder {
            target_id: target_id.into(),
        }
    }
    /// The id of the page opened.
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
}


pub struct CreateTargetReturnsBuilder<'a> {
    target_id: TargetID<'a>,
}

impl<'a> CreateTargetReturnsBuilder<'a> {
    pub fn build(self) -> CreateTargetReturns<'a> {
        CreateTargetReturns {
            target_id: self.target_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "sessionId")]
    session_id: Option<SessionID<'a>>,
    /// Deprecated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    target_id: Option<TargetID<'a>>,
}

impl<'a> DetachFromTargetParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> DetachFromTargetParamsBuilder<'a> {
        DetachFromTargetParamsBuilder {
            session_id: None,
            target_id: None,
        }
    }
    /// Session to detach.
    pub fn session_id(&self) -> Option<&SessionID<'a>> { self.session_id.as_ref() }
    /// Deprecated.
    pub fn target_id(&self) -> Option<&TargetID<'a>> { self.target_id.as_ref() }
}

#[derive(Default)]
pub struct DetachFromTargetParamsBuilder<'a> {
    session_id: Option<SessionID<'a>>,
    target_id: Option<TargetID<'a>>,
}

impl<'a> DetachFromTargetParamsBuilder<'a> {
    /// Session to detach.
    pub fn session_id(mut self, session_id: impl Into<SessionID<'a>>) -> Self { self.session_id = Some(session_id.into()); self }
    /// Deprecated.
    pub fn target_id(mut self, target_id: impl Into<TargetID<'a>>) -> Self { self.target_id = Some(target_id.into()); self }
    pub fn build(self) -> DetachFromTargetParams<'a> {
        DetachFromTargetParams {
            session_id: self.session_id,
            target_id: self.target_id,
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
    #[serde(rename = "browserContextId")]
    browser_context_id: crate::browser::BrowserContextID<'a>,
}

impl<'a> DisposeBrowserContextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `browser_context_id`: 
    pub fn builder(browser_context_id: crate::browser::BrowserContextID<'a>) -> DisposeBrowserContextParamsBuilder<'a> {
        DisposeBrowserContextParamsBuilder {
            browser_context_id: browser_context_id,
        }
    }
    pub fn browser_context_id(&self) -> &crate::browser::BrowserContextID<'a> { &self.browser_context_id }
}


pub struct DisposeBrowserContextParamsBuilder<'a> {
    browser_context_id: crate::browser::BrowserContextID<'a>,
}

impl<'a> DisposeBrowserContextParamsBuilder<'a> {
    pub fn build(self) -> DisposeBrowserContextParams<'a> {
        DisposeBrowserContextParams {
            browser_context_id: self.browser_context_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    target_id: Option<TargetID<'a>>,
}

impl<'a> GetTargetInfoParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetTargetInfoParamsBuilder<'a> {
        GetTargetInfoParamsBuilder {
            target_id: None,
        }
    }
    pub fn target_id(&self) -> Option<&TargetID<'a>> { self.target_id.as_ref() }
}

#[derive(Default)]
pub struct GetTargetInfoParamsBuilder<'a> {
    target_id: Option<TargetID<'a>>,
}

impl<'a> GetTargetInfoParamsBuilder<'a> {
    pub fn target_id(mut self, target_id: impl Into<TargetID<'a>>) -> Self { self.target_id = Some(target_id.into()); self }
    pub fn build(self) -> GetTargetInfoParams<'a> {
        GetTargetInfoParams {
            target_id: self.target_id,
        }
    }
}

/// Returns information about a target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetInfoReturns<'a> {
    #[serde(rename = "targetInfo")]
    target_info: TargetInfo<'a>,
}

impl<'a> GetTargetInfoReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_info`: 
    pub fn builder(target_info: TargetInfo<'a>) -> GetTargetInfoReturnsBuilder<'a> {
        GetTargetInfoReturnsBuilder {
            target_info: target_info,
        }
    }
    pub fn target_info(&self) -> &TargetInfo<'a> { &self.target_info }
}


pub struct GetTargetInfoReturnsBuilder<'a> {
    target_info: TargetInfo<'a>,
}

impl<'a> GetTargetInfoReturnsBuilder<'a> {
    pub fn build(self) -> GetTargetInfoReturns<'a> {
        GetTargetInfoReturns {
            target_info: self.target_info,
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
    /// Creates a builder for this type.
    pub fn builder() -> GetTargetsParamsBuilder<'a> {
        GetTargetsParamsBuilder {
            filter: None,
        }
    }
    /// Only targets matching filter will be reported. If filter is not specified
    /// and target discovery is currently enabled, a filter used for target discovery
    /// is used for consistency.
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
    #[serde(rename = "targetInfos")]
    target_infos: Vec<TargetInfo<'a>>,
}

impl<'a> GetTargetsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_infos`: The list of targets.
    pub fn builder(target_infos: Vec<TargetInfo<'a>>) -> GetTargetsReturnsBuilder<'a> {
        GetTargetsReturnsBuilder {
            target_infos: target_infos,
        }
    }
    /// The list of targets.
    pub fn target_infos(&self) -> &[TargetInfo<'a>] { &self.target_infos }
}


pub struct GetTargetsReturnsBuilder<'a> {
    target_infos: Vec<TargetInfo<'a>>,
}

impl<'a> GetTargetsReturnsBuilder<'a> {
    pub fn build(self) -> GetTargetsReturns<'a> {
        GetTargetsReturns {
            target_infos: self.target_infos,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "sessionId")]
    session_id: Option<SessionID<'a>>,
    /// Deprecated.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    target_id: Option<TargetID<'a>>,
}

impl<'a> SendMessageToTargetParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `message`: 
    pub fn builder(message: impl Into<Cow<'a, str>>) -> SendMessageToTargetParamsBuilder<'a> {
        SendMessageToTargetParamsBuilder {
            message: message.into(),
            session_id: None,
            target_id: None,
        }
    }
    pub fn message(&self) -> &str { self.message.as_ref() }
    /// Identifier of the session.
    pub fn session_id(&self) -> Option<&SessionID<'a>> { self.session_id.as_ref() }
    /// Deprecated.
    pub fn target_id(&self) -> Option<&TargetID<'a>> { self.target_id.as_ref() }
}


pub struct SendMessageToTargetParamsBuilder<'a> {
    message: Cow<'a, str>,
    session_id: Option<SessionID<'a>>,
    target_id: Option<TargetID<'a>>,
}

impl<'a> SendMessageToTargetParamsBuilder<'a> {
    /// Identifier of the session.
    pub fn session_id(mut self, session_id: impl Into<SessionID<'a>>) -> Self { self.session_id = Some(session_id.into()); self }
    /// Deprecated.
    pub fn target_id(mut self, target_id: impl Into<TargetID<'a>>) -> Self { self.target_id = Some(target_id.into()); self }
    pub fn build(self) -> SendMessageToTargetParams<'a> {
        SendMessageToTargetParams {
            message: self.message,
            session_id: self.session_id,
            target_id: self.target_id,
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
    #[serde(rename = "autoAttach")]
    auto_attach: bool,
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    #[serde(rename = "waitForDebuggerOnStart")]
    wait_for_debugger_on_start: bool,
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
    /// Creates a builder for this type with the required parameters:
    /// * `auto_attach`: Whether to auto-attach to related targets.
    /// * `wait_for_debugger_on_start`: Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger` to run paused targets.
    pub fn builder(auto_attach: bool, wait_for_debugger_on_start: bool) -> SetAutoAttachParamsBuilder<'a> {
        SetAutoAttachParamsBuilder {
            auto_attach: auto_attach,
            wait_for_debugger_on_start: wait_for_debugger_on_start,
            flatten: None,
            filter: None,
        }
    }
    /// Whether to auto-attach to related targets.
    pub fn auto_attach(&self) -> bool { self.auto_attach }
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    pub fn wait_for_debugger_on_start(&self) -> bool { self.wait_for_debugger_on_start }
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    pub fn flatten(&self) -> Option<bool> { self.flatten }
    /// Only targets matching filter will be attached.
    pub fn filter(&self) -> Option<&TargetFilter<'a>> { self.filter.as_ref() }
}


pub struct SetAutoAttachParamsBuilder<'a> {
    auto_attach: bool,
    wait_for_debugger_on_start: bool,
    flatten: Option<bool>,
    filter: Option<TargetFilter<'a>>,
}

impl<'a> SetAutoAttachParamsBuilder<'a> {
    /// Enables "flat" access to the session via specifying sessionId attribute in the commands.
    /// We plan to make this the default, deprecate non-flattened mode,
    /// and eventually retire it. See crbug.com/991325.
    pub fn flatten(mut self, flatten: bool) -> Self { self.flatten = Some(flatten); self }
    /// Only targets matching filter will be attached.
    pub fn filter(mut self, filter: TargetFilter<'a>) -> Self { self.filter = Some(filter); self }
    pub fn build(self) -> SetAutoAttachParams<'a> {
        SetAutoAttachParams {
            auto_attach: self.auto_attach,
            wait_for_debugger_on_start: self.wait_for_debugger_on_start,
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
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    #[serde(rename = "waitForDebuggerOnStart")]
    wait_for_debugger_on_start: bool,
    /// Only targets matching filter will be attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<TargetFilter<'a>>,
}

impl<'a> AutoAttachRelatedParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: 
    /// * `wait_for_debugger_on_start`: Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger` to run paused targets.
    pub fn builder(target_id: impl Into<TargetID<'a>>, wait_for_debugger_on_start: bool) -> AutoAttachRelatedParamsBuilder<'a> {
        AutoAttachRelatedParamsBuilder {
            target_id: target_id.into(),
            wait_for_debugger_on_start: wait_for_debugger_on_start,
            filter: None,
        }
    }
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
    /// Whether to pause new targets when attaching to them. Use 'Runtime.runIfWaitingForDebugger'
    /// to run paused targets.
    pub fn wait_for_debugger_on_start(&self) -> bool { self.wait_for_debugger_on_start }
    /// Only targets matching filter will be attached.
    pub fn filter(&self) -> Option<&TargetFilter<'a>> { self.filter.as_ref() }
}


pub struct AutoAttachRelatedParamsBuilder<'a> {
    target_id: TargetID<'a>,
    wait_for_debugger_on_start: bool,
    filter: Option<TargetFilter<'a>>,
}

impl<'a> AutoAttachRelatedParamsBuilder<'a> {
    /// Only targets matching filter will be attached.
    pub fn filter(mut self, filter: TargetFilter<'a>) -> Self { self.filter = Some(filter); self }
    pub fn build(self) -> AutoAttachRelatedParams<'a> {
        AutoAttachRelatedParams {
            target_id: self.target_id,
            wait_for_debugger_on_start: self.wait_for_debugger_on_start,
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
    /// Creates a builder for this type with the required parameters:
    /// * `discover`: Whether to discover available targets.
    pub fn builder(discover: bool) -> SetDiscoverTargetsParamsBuilder<'a> {
        SetDiscoverTargetsParamsBuilder {
            discover: discover,
            filter: None,
        }
    }
    /// Whether to discover available targets.
    pub fn discover(&self) -> bool { self.discover }
    /// Only targets matching filter will be attached. If 'discover' is false,
    /// 'filter' must be omitted or empty.
    pub fn filter(&self) -> Option<&TargetFilter<'a>> { self.filter.as_ref() }
}


pub struct SetDiscoverTargetsParamsBuilder<'a> {
    discover: bool,
    filter: Option<TargetFilter<'a>>,
}

impl<'a> SetDiscoverTargetsParamsBuilder<'a> {
    /// Only targets matching filter will be attached. If 'discover' is false,
    /// 'filter' must be omitted or empty.
    pub fn filter(mut self, filter: TargetFilter<'a>) -> Self { self.filter = Some(filter); self }
    pub fn build(self) -> SetDiscoverTargetsParams<'a> {
        SetDiscoverTargetsParams {
            discover: self.discover,
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
    /// Creates a builder for this type with the required parameters:
    /// * `locations`: List of remote locations.
    pub fn builder(locations: Vec<RemoteLocation<'a>>) -> SetRemoteLocationsParamsBuilder<'a> {
        SetRemoteLocationsParamsBuilder {
            locations: locations,
        }
    }
    /// List of remote locations.
    pub fn locations(&self) -> &[RemoteLocation<'a>] { &self.locations }
}


pub struct SetRemoteLocationsParamsBuilder<'a> {
    locations: Vec<RemoteLocation<'a>>,
}

impl<'a> SetRemoteLocationsParamsBuilder<'a> {
    pub fn build(self) -> SetRemoteLocationsParams<'a> {
        SetRemoteLocationsParams {
            locations: self.locations,
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
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
}

impl<'a> GetDevToolsTargetParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: Page or tab target ID.
    pub fn builder(target_id: impl Into<TargetID<'a>>) -> GetDevToolsTargetParamsBuilder<'a> {
        GetDevToolsTargetParamsBuilder {
            target_id: target_id.into(),
        }
    }
    /// Page or tab target ID.
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
}


pub struct GetDevToolsTargetParamsBuilder<'a> {
    target_id: TargetID<'a>,
}

impl<'a> GetDevToolsTargetParamsBuilder<'a> {
    pub fn build(self) -> GetDevToolsTargetParams<'a> {
        GetDevToolsTargetParams {
            target_id: self.target_id,
        }
    }
}

/// Gets the targetId of the DevTools page target opened for the given target
/// (if any).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDevToolsTargetReturns<'a> {
    /// The targetId of DevTools page target if exists.
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetId")]
    target_id: Option<TargetID<'a>>,
}

impl<'a> GetDevToolsTargetReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetDevToolsTargetReturnsBuilder<'a> {
        GetDevToolsTargetReturnsBuilder {
            target_id: None,
        }
    }
    /// The targetId of DevTools page target if exists.
    pub fn target_id(&self) -> Option<&TargetID<'a>> { self.target_id.as_ref() }
}

#[derive(Default)]
pub struct GetDevToolsTargetReturnsBuilder<'a> {
    target_id: Option<TargetID<'a>>,
}

impl<'a> GetDevToolsTargetReturnsBuilder<'a> {
    /// The targetId of DevTools page target if exists.
    pub fn target_id(mut self, target_id: impl Into<TargetID<'a>>) -> Self { self.target_id = Some(target_id.into()); self }
    pub fn build(self) -> GetDevToolsTargetReturns<'a> {
        GetDevToolsTargetReturns {
            target_id: self.target_id,
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
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
    /// The id of the panel we want DevTools to open initially. Currently
    /// supported panels are elements, console, network, sources, resources
    /// and performance.
    #[serde(skip_serializing_if = "Option::is_none", rename = "panelId")]
    panel_id: Option<Cow<'a, str>>,
}

impl<'a> OpenDevToolsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: This can be the page or tab target ID.
    pub fn builder(target_id: impl Into<TargetID<'a>>) -> OpenDevToolsParamsBuilder<'a> {
        OpenDevToolsParamsBuilder {
            target_id: target_id.into(),
            panel_id: None,
        }
    }
    /// This can be the page or tab target ID.
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
    /// The id of the panel we want DevTools to open initially. Currently
    /// supported panels are elements, console, network, sources, resources
    /// and performance.
    pub fn panel_id(&self) -> Option<&str> { self.panel_id.as_deref() }
}


pub struct OpenDevToolsParamsBuilder<'a> {
    target_id: TargetID<'a>,
    panel_id: Option<Cow<'a, str>>,
}

impl<'a> OpenDevToolsParamsBuilder<'a> {
    /// The id of the panel we want DevTools to open initially. Currently
    /// supported panels are elements, console, network, sources, resources
    /// and performance.
    pub fn panel_id(mut self, panel_id: impl Into<Cow<'a, str>>) -> Self { self.panel_id = Some(panel_id.into()); self }
    pub fn build(self) -> OpenDevToolsParams<'a> {
        OpenDevToolsParams {
            target_id: self.target_id,
            panel_id: self.panel_id,
        }
    }
}

/// Opens a DevTools window for the target.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenDevToolsReturns<'a> {
    /// The targetId of DevTools page target.
    #[serde(rename = "targetId")]
    target_id: TargetID<'a>,
}

impl<'a> OpenDevToolsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `target_id`: The targetId of DevTools page target.
    pub fn builder(target_id: impl Into<TargetID<'a>>) -> OpenDevToolsReturnsBuilder<'a> {
        OpenDevToolsReturnsBuilder {
            target_id: target_id.into(),
        }
    }
    /// The targetId of DevTools page target.
    pub fn target_id(&self) -> &TargetID<'a> { &self.target_id }
}


pub struct OpenDevToolsReturnsBuilder<'a> {
    target_id: TargetID<'a>,
}

impl<'a> OpenDevToolsReturnsBuilder<'a> {
    pub fn build(self) -> OpenDevToolsReturns<'a> {
        OpenDevToolsReturns {
            target_id: self.target_id,
        }
    }
}

impl<'a> OpenDevToolsParams<'a> { pub const METHOD: &'static str = "Target.openDevTools"; }

impl<'a> crate::CdpCommand<'a> for OpenDevToolsParams<'a> {
    const METHOD: &'static str = "Target.openDevTools";
    type Response = OpenDevToolsReturns<'a>;
}
