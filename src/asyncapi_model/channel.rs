use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{
    ChannelBinding, ExternalDocumentation, Message, OperationBinding, OperationTrait, Parameter,
    ReferenceOr, Tag,
};

/// Describes the operations available on a single channel.
///
/// # Examples
/// ```json
/// {
///     "description": "This channel is used to exchange messages about users signing up",
///     "subscribe": {
///         "summary": "A user signed up.",
///         "message": {
///         "description": "A longer description of the message",
///         "payload": {
///             "type": "object",
///             "properties": {
///             "user": {
///                 "$ref": "#/components/schemas/user"
///             },
///             "signup": {
///                 "$ref": "#/components/schemas/signup"
///             }
///             }
///         }
///         }
///     },
///     "bindings": {
///         "amqp": {
///         "is": "queue",
///         "queue": {
///             "exclusive": true
///         }
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// description: This channel is used to exchange messages about users signing up
/// subscribe:
///   summary: A user signed up.
///   message:
///     description: A longer description of the message
///     payload:
///       type: object
///       properties:
///         user:
///           $ref: "#/components/schemas/user"
///         signup:
/// bindings:
///   amqp:
///     is: queue
///     queue:
///       exclusive: true
/// ```
///
/// Using `oneOf` to specify multiple messages per operation:
///
/// ```json
/// {
///     "subscribe": {
///         "message": {
///         "oneOf": [
///             { "$ref": "#/components/messages/signup" },
///             { "$ref": "#/components/messages/login" }
///         ]
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// subscribe:
///   message:
///     oneOf:
///       - $ref: '#/components/messages/signup'
///       - $ref: '#/components/messages/login'
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    /// Allows for an external definition of this channel item. The referenced structure
    /// MUST be in the format of a
    /// [Channel Item Object][crate::Channel].
    /// If there are conflicts between the referenced definition and this Channel Item's
    /// definition, the behavior is *undefined*.
    #[deprecated(note = "The $ref field in Channel Item Object is now deprecated
        from AsyncAPI 2.3.0. The current plan is that the $ref field will be 
        removed from Channel Item Object in AsyncAPI 3.0, and replaced with 
        Reference Object.")]
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// An optional description of this channel item.
    /// [CommonMark syntax](https://spec.commonmark.org/) can be used for rich
    /// text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The servers on which this channel is available, specified as an optional unordered
    /// list of names (string keys) of [Server Objects][crate::Server] defined in the
    /// [Servers Object][crate::Server] (a map). If `servers` is absent or empty then this
    /// channel must be available on all servers defined in the [Servers Object][crate::Server].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<String>,
    /// A definition of the SUBSCRIBE operation, which defines the messages produced
    /// by the application and sent to the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<Operation>,
    /// A definition of the PUBLISH operation, which defines the messages consumed
    /// by the application from the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<Operation>,
    /// A map of the parameters included in the channel name. It SHOULD be present only
    /// when using channels with expressions (as defined by
    /// [RFC 6570 section 2.2](https://tools.ietf.org/html/rfc6570#section-2.2)).
    ///
    /// Describes a map of parameters included in a channel name.
    ///
    /// This map MUST contain all the parameters used in the parent channel name.
    ///
    /// # Examples
    ///
    /// ```json
    /// {
    ///     "user/{userId}/signup": {
    ///         "parameters": {
    ///             "userId": {
    ///                 "description": "Id of the user.",
    ///                 "schema": {
    ///                    "type": "string"
    ///                 }
    ///             }
    ///         },
    ///         "subscribe": {
    ///             "$ref": "#/components/messages/userSignedUp"
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
    ///   subscribe:
    ///     $ref: "#/components/messages/userSignedUp"
    /// ```
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub parameters: IndexMap<String, ReferenceOr<Parameter>>,
    /// A map where the keys describe the name of the protocol and the values
    /// describe protocol-specific definitions for the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<ReferenceOr<ChannelBinding>>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// Describes a publish or a subscribe operation. This provides a place to document how
/// and why messages are sent and received.
///
/// For example, an operation might describe a chat application use case where a user sends
/// a text message to a group. A publish operation describes messages that are received by
/// the chat application, whereas a subscribe operation describes messages that are sent by
/// the chat application.
///
/// # Examples
/// ```json
/// {
///     "operationId": "registerUser",
///     "summary": "Action to sign a user up.",
///     "description": "A longer description",
///     "tags": [
///         { "name": "user" },
///         { "name": "signup" },
///         { "name": "register" }
///     ],
///     "message": {
///         "headers": {
///         "type": "object",
///         "properties": {
///             "applicationInstanceId": {
///             "description": "Unique identifier for a given instance of the publishing application",
///             "type": "string"
///             }
///         }
///         },
///         "payload": {
///         "type": "object",
///         "properties": {
///             "user": {
///             "$ref": "#/components/schemas/userCreate"
///             },
///             "signup": {
///             "$ref": "#/components/schemas/signup"
///             }
///         }
///         }
///     },
///     "bindings": {
///         "amqp": {
///         "ack": false
///         }
///     },
///     "traits": [
///         { "$ref": "#/components/operationTraits/kafka" }
///     ]
/// }
/// ```
///
/// ```yaml
/// operationId: registerUser
/// summary: Action to sign a user up.
/// description: A longer description
/// tags:
///   - name: user
///   - name: signup
///   - name: register
/// message:
///   headers:
///     type: object
///     properties:
///       applicationInstanceId:
///         description: Unique identifier for a given instance of the publishing application
///         type: string
///   payload:
///     type: object
///     properties:
///       user:
///         $ref: "#/components/schemas/userCreate"
///       signup:
///         $ref: "#/components/schemas/signup"
/// bindings:
///   amqp:
///     ack: false
/// traits:
///   - $ref: "#/components/operationTraits/kafka"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// Unique string used to identify the operation.
    /// The id MUST be unique among all operations described in the API.
    /// The operationId value is **case-sensitive**.
    /// Tools and libraries MAY use the operationId to uniquely identify an
    /// operation, therefore, it is RECOMMENDED to follow common programming
    /// naming conventions.
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
    /// A map where the keys describe the name of the protocol and the
    /// values describe protocol-specific definitions for the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<ReferenceOr<OperationBinding>>,
    /// A list of traits to apply to the operation object.
    /// Traits MUST be merged into the operation object using the
    /// [JSON Merge Patch](https://tools.ietf.org/html/rfc7386)
    /// algorithm in the same order they are defined here.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub traits: Vec<ReferenceOr<OperationTrait>>,
    /// A definition of the message that will be published or received on
    /// this channel. `oneOf` is allowed here to specify multiple messages, however,
    /// **a message MUST be valid only against one of the referenced message objects.**
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<OperationMessageType>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OperationMessageType {
    Map(IndexMap<String, ReferenceOr<Message>>),
    Single(ReferenceOr<Message>),
}
