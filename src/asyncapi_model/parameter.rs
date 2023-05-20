use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{ReferenceOr, Schema};

/// Describes a parameter included in a channel name.
///
/// # Examples
///
/// ```json
/// {
///     "user/{userId}/signup": {
///         "parameters": {
///         "userId": {
///             "description": "Id of the user.",
///             "schema": {
///             "type": "string"
///             },
///             "location": "$message.payload#/user/id"
///         }
///         },
///         "subscribe": {
///         "$ref": "#/components/messages/userSignedUp"
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// user/{userId}/signup:
///   parameters:
///     userId:
///       description: Id of the user.
///       schema:
///         type: string
///       location: $message.payload#/user/id
///   subscribe:
///     $ref: "#/components/messages/userSignedUp"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// A verbose explanation of the parameter.
    /// [CommonMark syntax](https://spec.commonmark.org/)
    /// can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Definition of the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceOr<Schema>>,
    /// A [runtime expression](https://www.asyncapi.com/docs/specifications/v2.3.0#runtimeExpression)
    /// that specifies the location of the parameter value.
    /// Even when a definition for the target field exists,
    /// it MUST NOT be used to validate this parameter but,
    /// instead, the `schema` property MUST be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
