//! A domain for ad-related metrics and data.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Ad frame data.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdFrameData<'a> {
    /// The DevTools frame token.
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
    /// The initial origin of the frame. To minimize the payload size, this is
    /// only sent once per frame.
    #[serde(skip_serializing_if = "Option::is_none", rename = "initialOrigin")]
    pub initial_origin: Option<Cow<'a, str>>,
    /// The network bytes of the frame.
    #[serde(rename = "networkBytes")]
    pub network_bytes: f64,
    /// The CPU time of the frame, in milliseconds.
    #[serde(rename = "cpuTime")]
    pub cpu_time: f64,
}
/// Ad metrics for a page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdMetrics<'a> {
    /// The viewport ad density by area, represented as a percentage (an integer
    /// between 0 and 100).
    #[serde(rename = "viewportAdDensityByArea")]
    pub viewport_ad_density_by_area: i64,
    /// The time-weighted average of the viewport ad density by area, measured
    /// across the duration of the page.
    #[serde(rename = "averageViewportAdDensityByArea")]
    pub average_viewport_ad_density_by_area: f64,
    /// The number of ads currently visible within the viewport.
    #[serde(rename = "viewportAdCount")]
    pub viewport_ad_count: u64,
    /// The time-weighted average of the viewport ad count, measured across the
    /// duration of the page.
    #[serde(rename = "averageViewportAdCount")]
    pub average_viewport_ad_count: f64,
    /// The total ad CPU usage, in milliseconds.
    #[serde(rename = "totalAdCpuTime")]
    pub total_ad_cpu_time: f64,
    /// The total ad network bytes.
    #[serde(rename = "totalAdNetworkBytes")]
    pub total_ad_network_bytes: f64,
    /// The list of ad frames that have been updated since the last event.
    #[serde(rename = "updateAdFrames")]
    pub update_ad_frames: Vec<AdFrameData<'a>>,
    /// The list of ad frame IDs that have been removed since the last event.
    #[serde(rename = "removeAdFrames")]
    pub remove_ad_frames: Vec<crate::page::FrameId<'a>>,
}
/// An ad script.
/// Note: when the script is a transitive ad script, we only fill in the
/// immediate ancestor script in the provenance's adScriptAncestry field (as its
/// first entry), rather than filling in the full ancestry. This saves work for
/// the backend, and the frontend can reconstruct the full ancestry if
/// necessary.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AdScript<'a> {
    /// The script ID.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// The ad provenance.
    pub provenance: crate::network::AdProvenance<'a>,
}
/// Retrieves ad metrics for the current page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Ads.getAdMetrics", response = "GetAdMetricsReturns<'a>")]
pub struct GetAdMetricsParams {

}
/// Retrieves ad metrics for the current page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAdMetricsReturns<'a> {
    pub metrics: AdMetrics<'a>,
}
/// Retrieves ad scripts for the current page. To minimize payload size, this
/// only returns the newly tracked ad scripts since the last call to
/// getAdScripts (i.e., the delta).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Ads.getAdScripts", response = "GetAdScriptsReturns<'a>")]
pub struct GetAdScriptsParams {

}
/// Retrieves ad scripts for the current page. To minimize payload size, this
/// only returns the newly tracked ad scripts since the last call to
/// getAdScripts (i.e., the delta).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetAdScriptsReturns<'a> {
    #[serde(rename = "newScripts")]
    pub new_scripts: Vec<AdScript<'a>>,
}