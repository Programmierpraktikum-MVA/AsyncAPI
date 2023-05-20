use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// An object that specifies an identifier at design time that can used for
/// message tracing and correlation.
///
/// For specifying and computing the location of a Correlation ID, a
/// [runtime expression](https://www.asyncapi.com/docs/specifications/v2.3.0#runtimeExpression)
/// is used.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CorrelationId {
    /// An optional description of the identifier.
    /// [CommonMark syntax](https://spec.commonmark.org/)
    /// can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// **REQUIRED**.
    /// A [runtime expression](https://www.asyncapi.com/docs/specifications/v2.3.0#runtimeExpression)
    /// that specifies the location of the correlation ID.
    pub location: String,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
