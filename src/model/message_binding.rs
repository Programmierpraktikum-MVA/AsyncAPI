use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::Schema;

/// Map describing protocol-specific definitions for a message.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MessageBinding {
    /// Protocol-specific information for an HTTP message, i.e., a request or a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPMessageBinding>,
    /// Protocol-specific information for a WebSockets message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<WebSocketMessageBinding>,
    /// Protocol-specific information for a Kafka message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka: Option<KafkaMessageBinding>,
    /// Protocol-specific information for an Anypoint MQ message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anypointmq: Option<AnyPointMQMessageBinding>,
    /// Protocol-specific information for an AMQP 0-9-1 message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amqp: Option<AMQPMessageBinding>,
    /// Protocol-specific information for an AMQP 1.0 message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qmqp1: Option<AMQP1MessageBinding>,
    /// Protocol-specific information for an MQTT message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt: Option<MQTTMessageBinding>,
    /// Protocol-specific information for an MQTT 5 message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt5: Option<MQTT5MessageBinding>,
    /// Protocol-specific information for a NATS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nats: Option<NATSMessageBinding>,
    /// Protocol-specific information for a JMS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jms: Option<JMSMessageBinding>,
    /// Protocol-specific information for an SNS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SNSMessageBinding>,
    /// Protocol-specific information for a Solace message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solace: Option<SolaceMessageBinding>,
    /// Protocol-specific information for an SQS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<SQSMessageBinding>,
    /// Protocol-specific information for a STOMP message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stomp: Option<STOMPMessageBinding>,
    /// Protocol-specific information for a Redis message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis: Option<RedisMessageBinding>,
    /// Protocol-specific information for a Mercure message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mercure: Option<MercureMessageBinding>,
    /// Protocol-specific information for an IBM MQ message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibmmq: Option<IBMMQMessageBinding>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// This object contains information about the message representation in HTTP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HTTPMessageBinding {
    /// A Schema object containing the definitions for HTTP-specific headers.
    /// This schema MUST be of type object and have a properties key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Schema>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// When using WebSockets, the channel represents the connection.
/// Unlike other protocols that support multiple virtual channels
/// (topics, routing keys, etc.) per connection, WebSockets doesn't
/// support virtual channels or, put it another way, there's only one channel
/// and its characteristics are strongly related to the protocol used for the handshake, i.e., HTTP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMessageBinding {
    /// The HTTP method to use when establishing the connection. Its value MUST be either GET or POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// A Schema object containing the definitions for each query parameter.
    /// This schema MUST be of type object and have a properties key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Schema>,
    /// A Schema object containing the definitions of the HTTP headers to use when establishing the connection.
    /// This schema MUST be of type object and have a properties key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Schema>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object contains information about the message representation in Kafka.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KafkaMessageBinding {
    /// The message key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Schema>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// The Anypoint MQ [Message Binding Object][MessageBinding] is defined by a
/// [JSON Schema](https://github.com/asyncapi/bindings/blob/master/anypointmq/json_schemas/message.json),
/// which defines these fields.
///
/// Note that application headers must be specified in the
/// [`headers` field of the standard Message Object](https://github.com/asyncapi/spec/blob/master/spec/asyncapi.md#messageObjectHeaders)
/// and are transmitted in the
/// [`properties` section of the Anypoint MQ message](https://anypoint.mulesoft.com/exchange/portals/anypoint-platform/f1e97bc6-315a-4490-82a7-23abe036327a.anypoint-platform/anypoint-mq-broker/).
/// In contrast, protocol headers such as `messageId` must be specified in the
/// [`headers` field of the message binding object](https://github.com/asyncapi/bindings/blob/master/anypointmq/README.md#messageBindingObjectHeaders)
/// and are transmitted in the [`headers` section of the Anypoint MQ message](https://anypoint.mulesoft.com/exchange/portals/anypoint-platform/f1e97bc6-315a-4490-82a7-23abe036327a.anypoint-platform/anypoint-mq-broker/).
///
/// # Examples
///
/// The following example shows a `channels` object with two channels, each having one operation (`subscribe` or `publish`) with one message. Only the message of the `subscribe` operation has a message binding object for `anypointmq`:
///
/// ```yaml
/// channels:
///   user/signup:
///     publish:
///       message:
///         #...
///   user/signedup:
///     subscribe:
///       message:
///         headers:
///           type: object
///           properties:
///             correlationId:
///               description: Correlation ID set by application
///               type: string
///         payload:
///           type: object
///           properties:
///             #...
///         correlationId:
///           description: Correlation ID is specified as a header and transmitted in the Anypoint MQ message properties section
///           location:    $message.header#/correlationId
///         bindings:
///           anypointmq:
///             headers:
///               type: object
///               properties:
///                 messageId:
///                   type: string
///             bindingVersion: '0.0.1'
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnyPointMQMessageBinding {
    /// **Optional**. A Schema object containing the definitions for Anypoint MQ-specific headers
    /// (so-called protocol headers). This schema MUST be of type object and have a properties key.
    /// Examples of Anypoint MQ protocol headers are messageId and messageGroupId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Schema>,
    /// **Optional**, defaults to `latest`. The version of this binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object contains information about the message representation in AMQP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AMQPMessageBinding {
    /// A MIME encoding for the message content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
    /// Application-specific message type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AMQP1MessageBinding {}

/// This object contains information about the message representation in MQTT.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MQTTMessageBinding {
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MQTT5MessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NATSMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JMSMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SNSMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolaceMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SQSMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct STOMPMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RedisMessageBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MercureMessageBinding {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IBMMQMessageBinding {
    #[serde(rename = "type")]
    pub typ: Option<String>,
}
