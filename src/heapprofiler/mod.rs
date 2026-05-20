use serde::{Serialize, Deserialize};
pub type RemoteObjectId<'a> = std::borrow::Cow<'a, str>;
pub type RemoteObject = serde_json::Value;
pub type ScriptId<'a> = std::borrow::Cow<'a, str>;
pub type StackTrace = serde_json::Value;
pub type UniqueDebuggerId<'a> = std::borrow::Cow<'a, str>;
pub type SearchMatch = serde_json::Value;
pub type ExecutionContextId = i64;
pub type Timestamp = f64;
