use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExternalDocumentation {
    /// A short description of the target documentation.
    /// [CommonMark syntax](https://spec.commonmark.org/) can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// **Required**. The URL for the target documentation.
    /// Value MUST be in the format of a URL.
    pub url: String,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
