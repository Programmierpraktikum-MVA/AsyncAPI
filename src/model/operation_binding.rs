use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::Schema;

/// Map describing protocol-specific definitions for an operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OperationBinding {
    /// Protocol-specific information for an HTTP operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPOperationBinding>,
    /// Protocol-specific information for a WebSockets operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<WebSocketsOperationBinding>,
    /// Protocol-specific information for a Kafka operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka: Option<KafkaOperationBinding>,
    /// Protocol-specific information for an Anypoint MQ operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anypointmq: Option<AnyPointMQOperationBinding>,
    /// Protocol-specific information for an AMPQ operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amqp: Option<AMQPOperationBinding>,
    /// Protocol-specific information for an AMQP 1.0 operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amqp1: Option<AMQP1OperationBinding>,
    /// Protocol-specific information for an MQTT operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt: Option<MQTTOperationBinding>,
    /// Protocol-specific information for an MQTT 5 operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt5: Option<MQTT5OperationBinding>,
    /// Protocol-specific information for a NATS operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nats: Option<NATSOperationBinding>,
    /// Protocol-specific information for a JMS operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jms: Option<JMSOperationBinding>,
    /// Protocol-specific information for an SNS operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SNSOperationBinding>,
    /// Protocol-specific information for a Solace operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solace: Option<SolaceOperationBinding>,
    /// Protocol-specific information for an SQS operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<SQSOperationBinding>,
    /// Protocol-specific information for a STOMP operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stomp: Option<STOMPOperationBinding>,
    /// Protocol-specific information for a Redis operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis: Option<RedisOperationBinding>,
    /// Protocol-specific information for a Mercure operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mercure: Option<MercureOperationBinding>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

///
///
/// # Examples
/// ```yaml
/// channels:
///   /employees:
///     subscribe:
///       bindings:
///         http:
///           type: request
///           method: GET
///           query:
///             type: object
///             required:
///               - companyId
///             properties:
///               companyId:
///                 type: number
///                 minimum: 1
///                 description: The Id of the company.
///             additionalProperties: false
///           bindingVersion: '0.1.0'
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HTTPOperationBinding {
    /// Required. Type of operation. Its value MUST be either `request` or `response`.
    #[serde(rename = "type")]
    pub typ: String,
    /// When `type` is `request`, this is the HTTP method, otherwise it MUST be ignored.
    /// Its value MUST be one of `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `HEAD`,
    /// `OPTIONS`, `CONNECT`, and `TRACE`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// A Schema object containing the definitions for each query parameter.
    /// This schema MUST be of type `object` and have a `properties` key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Schema>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebSocketsOperationBinding {}

/// This object contains information about the operation representation in Kafka.
///
/// # Examples
///
/// ```yaml
/// channels:
///   user-signedup:
///     publish:
///       bindings:
///         kafka:
///           groupId:
///             type: string
///             enum: ['myGroupId']
///           clientId:
///             type: string
///             enum: ['myClientId']
///           bindingVersion: '0.1.0'
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KafkaOperationBinding {
    /// Id of the consumer group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Schema>,
    /// Id of the consumer inside a consumer group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<Schema>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnyPointMQOperationBinding {}

/// This object contains information about the operation representation in AMQP.
///
/// # Examples
///
/// ```yaml
/// channels:
///   user/signup:
///     publish:
///       bindings:
///         amqp:
///           expiration: 100000
///           userId: guest
///           cc: ['user.logs']
///           priority: 10
///           deliveryMode: 2
///           mandatory: false
///           bcc: ['external.audit']
///           replyTo: user.signedup
///           timestamp: true
///           ack: false
///           bindingVersion: 0.2.0
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AMQPOperationBinding {
    /// TTL (Time-To-Live) for the message. It MUST be greater than or equal to zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i32>,
    /// Identifies the user who has sent the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The routing keys the message should be routed to at the time of publishing.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<String>,
    /// A priority for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Delivery mode of the message. Its value MUST be either 1 (transient) or 2 (persistent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<i32>,
    /// Whether the message is mandatory or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    /// Like [cc](https://github.com/asyncapi/bindings/blob/master/amqp/README.md#operationBindingObjectCC) but consumers will not receive this information.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bcc: Vec<String>,
    /// Name of the queue where the consumer should send the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Whether the message should include a timestamp or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<bool>,
    /// Whether the consumer should ack the message or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ack: Option<bool>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AMQP1OperationBinding {}

/// This object contains information about the operation representation in MQTT.
///
/// # Examples
///
/// ```yaml
/// channels:
///   user/signup:
///     publish:
///       bindings:
///         mqtt:
///           qos: 2
///           retain: true
///           bindingVersion: 0.1.0
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MQTTOperationBinding {
    /// Defines the Quality of Service (QoS) levels for the message flow between client
    /// and server. Its value MUST be either 0 (At most once delivery),
    /// 1 (At least once delivery), or 2 (Exactly once delivery).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// Whether the broker should retain the message or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MQTT5OperationBinding {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NATSOperationBinding {
    /// Defines the name of the queue to use. It MUST NOT exceed 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct JMSOperationBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SNSOperationBinding {}

/// We need the ability to support several bindings for each operation, see the
/// [Example](https://github.com/asyncapi/bindings/tree/master/solace#example)
/// section for details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolaceOperationBinding {
    /// The current version is 0.2.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destinations: Vec<SolaceDestination>,
}

/// Each destination has the following structure. Note that bindings under a
/// 'subscribe' operation define the behaviour of publishers, and those under a
/// 'publish' operation define how subscribers are configured.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolaceDestination {
    /// 'queue' or 'topic'. If the type is queue, then the subscriber can bind
    /// to the queue, which in turn will subscribe to the topic as represented
    /// by the channel name or to the provided topicSubscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_type: Option<SolaceDestinationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_mode: Option<SolaceDestinationDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue: Option<SolaceDestinationQueue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<SolaceDestinationTopic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SolaceDestinationType {
    Queue,
    Topic,
}

/// 'direct' or 'persistent'. This determines the quality of service for
/// publishing messages as documented
/// [here](https://docs.solace.com/PubSub-Basics/Core-Concepts-Message-Delivery-Modes.htm).
/// Default is 'persistent'.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SolaceDestinationDeliveryMode {
    Direct,
    Persistent,
}

impl Default for SolaceDestinationDeliveryMode {
    fn default() -> Self {
        SolaceDestinationDeliveryMode::Persistent
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolaceDestinationQueue {
    /// The name of the queue, only applicable when destinationType is 'queue'.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// A list of topics that the queue subscribes to, only applicable when
    /// destinationType is 'queue'. If none is given, the queue subscribes to
    /// the topic as represented by the channel name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    topic_subscriptions: Vec<String>,
    /// 'exclusive' or 'nonexclusive'. This is documented
    /// [here](https://docs.solace.com/PubSub-Basics/Endpoints.htm).
    /// Only applicable when destinationType is 'queue'.
    #[serde(skip_serializing_if = "Option::is_none")]
    access_type: Option<SolaceDestinationQueueAccessType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SolaceDestinationQueueAccessType {
    Exclusive,
    Nonexclusive,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolaceDestinationTopic {
    /// A list of topics that the client subscribes to, only applicable when
    /// destinationType is 'topic'. If none is given, the client subscribes to
    /// the topic as represented by the channel name.
    topic_subscriptions: Vec<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SQSOperationBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct STOMPOperationBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RedisOperationBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MercureOperationBinding {}
