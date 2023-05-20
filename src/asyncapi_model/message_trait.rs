use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{
    message_binding::MessageBinding, CorrelationId, ExternalDocumentation, ReferenceOr, Schema, Tag,
};

/// Describes a trait that MAY be applied to a
/// [Message Object][crate::Message].
/// This object MAY contain any property from the
/// [Message Object][crate::Message],
/// except `payload` and `traits`.
///
/// If you're looking to apply traits to an operation, see the
/// [Operation Trait Object][crate::OperationTrait].
///
/// # Examples
///
/// ```json
/// {
///     "schemaFormat": "application/vnd.apache.avro+json;version=1.9.0",
///     "contentType": "application/json"
/// }
/// ```
///
/// ```yaml
/// schemaFormat: 'application/vnd.apache.avro+yaml;version=1.9.0'
/// contentType: application/json
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MessageTrait {
    /// Schema definition of the application headers.
    /// Schema MUST be of type "object".
    /// It **MUST NOT** define the protocol headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<ReferenceOr<Schema>>,
    /// Definition of the correlation ID used for message tracing or matching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<ReferenceOr<CorrelationId>>,
    /// A string containing the name of the schema format/language used to define
    /// the message payload. If omitted, implementations should parse the payload as a
    /// [Schema object][crate::Schema].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_format: Option<String>,
    /// The content type to use when encoding/decoding a message's payload.
    /// The value MUST be a specific media type (e.g. `application/json`).
    /// When omitted, the value MUST be the one specified on the
    /// [defaultContentType](https://www.asyncapi.com/docs/specifications/v2.3.0#defaultContentTypeString)
    /// field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// A machine-friendly name for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A human-friendly title for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A short summary of what the message is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the message.
    /// [CommonMark syntax](https://spec.commonmark.org/)
    /// can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of messages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// A map where the keys describe the name of the protocol
    /// and the values describe protocol-specific definitions for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<ReferenceOr<MessageBinding>>,
    /// List of examples.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<MessageExample>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// Message Example Object represents an example of a
/// [Message Object][crate::Message] and MUST contain either **headers**
/// and/or **payload** fields.
///
/// # Examples
///
/// ```json
/// {
///     "name": "SimpleSignup",
///     "summary": "A simple UserSignup example message",
///     "headers": {
///         "correlationId": "my-correlation-id",
///         "applicationInstanceId": "myInstanceId"
///     },
///     "payload": {
///         "user": {
///         "someUserKey": "someUserValue"
///         },
///         "signup": {
///         "someSignupKey": "someSignupValue"
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// name: SimpleSignup
/// summary: A simple UserSignup example message
/// headers:
///   correlationId: my-correlation-id
///   applicationInstanceId: myInstanceId
/// payload:
///   user:
///     someUserKey: someUserValue
///   signup:
///     someSignupKey: someSignupValue
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MessageExample {
    /// The value of this field MUST validate against the
    /// [Message Object's][crate::Message] headers field.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<String, serde_json::Value>,
    /// The value of this field MUST validate against the
    /// [Message Object's][crate::Message] payload field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// A machine-friendly name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A short summary of what the example is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
