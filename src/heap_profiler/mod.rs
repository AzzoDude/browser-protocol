use serde::{Serialize, Deserialize};
pub type RemoteObjectId = String;
pub type RemoteObject = serde_json::Value;
pub type ScriptId = String;
pub type StackTrace = serde_json::Value;
pub type UniqueDebuggerId = String;
pub type SearchMatch = serde_json::Value;
pub type ExecutionContextId = i64;
pub type Timestamp = f64;
