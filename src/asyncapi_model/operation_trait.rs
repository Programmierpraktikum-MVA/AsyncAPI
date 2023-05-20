use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{ExternalDocumentation, OperationBinding, ReferenceOr, Tag};

/// Describes a trait that MAY be applied to an
/// [Operation Object][crate::Operation].
/// This object MAY contain any property from the
/// [Operation Object][crate::Operation],
/// except `message` and `traits`.
///
/// If you're looking to apply traits to a message, see the
/// [Message Trait Object][crate::MessageTrait].
///
/// # Examples
///
/// ```json
/// {
///     "bindings": {
///         "amqp": {
///         "ack": false
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// bindings:
///   amqp:
///     ack: false
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OperationTrait {
    /// Unique string used to identify the operation. The id MUST be unique among all
    /// operations described in the API. The operationId value is **case-sensitive**.
    /// Tools and libraries MAY use the operationId to uniquely identify an operation,
    /// therefore, it is RECOMMENDED to follow common programming naming conventions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// A short summary of what the operation is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation.
    /// [CommonMark syntax](https://spec.commonmark.org/)
    /// can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// A map where the keys describe the name of the protocol and the values describe
    /// protocol-specific definitions for the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<ReferenceOr<OperationBinding>>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
