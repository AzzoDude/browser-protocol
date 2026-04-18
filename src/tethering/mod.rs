//! The Tethering domain defines methods and events for browser port binding.

use serde::{Serialize, Deserialize};

/// Request browser port binding.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindParams {
    /// Port number to bind.

    pub port: i64,
}

/// Request browser port unbinding.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnbindParams {
    /// Port number to unbind.

    pub port: i64,
}
