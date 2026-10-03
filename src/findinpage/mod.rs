//! This domain provides commands to trigger the "Find in page" feature.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Forwards 'query' to the find-in-page facility, starting a new find session.
/// Where exactly the search starts from is implementation-specific.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FindInPage.findFirst")]
pub struct FindFirstParams<'a> {
    pub query: Cow<'a, str>,
}
/// Moves to the next match for the query passed to the most recent
/// findFirst() call.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FindInPage.findNext")]
pub struct FindNextParams {

}
/// Moves to the previous match for the query passed to the most recent
/// findFirst() call.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FindInPage.findPrev")]
pub struct FindPrevParams {

}
/// Ends the current find session, if any, and clears its highlighting.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FindInPage.stop")]
pub struct StopParams {

}