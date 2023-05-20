use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// The asyncapi spec doesn't describe a structured example object.
///
/// This library, however, tries to serialize examples into
/// this struct for easier handling.
/// See [issue #606](https://github.com/asyncapi/spec/issues/606) for a
/// proposal
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Example {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A machine-friendly name.
    pub name: Option<String>,
    /// A short summary of what the example is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Long description for the example.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Payload as described in the `websocket-gemini` example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// field name proposed in the [issue #606](https://github.com/asyncapi/spec/issues/606)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// Inline extensions to this object.
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
