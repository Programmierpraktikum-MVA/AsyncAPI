use super::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Allows adding meta data to a single tag.
///
/// # Examples
/// ```json
/// {
///     "name": "user",
///     "description": "User-related messages"
/// }
/// ```
///
/// ```yaml
/// name: user
/// description: User-related messages
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Tag {
    /// **Required**. The name of the tag.
    pub name: String,
    /// A short description of the target documentation.
    /// [CommonMark](https://spec.commonmark.org/) syntax can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
