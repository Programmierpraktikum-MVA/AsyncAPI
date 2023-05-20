use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::Schema;

/// Map describing protocol-specific definitions for a message.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChannelBinding {
    /// Protocol-specific information for an HTTP channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPChannelBinding>,
    /// Protocol-specific information for a WebSockets channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<WebsocketsChannelBinding>,
    /// Protocol-specific information for a Kafka channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka: Option<KafkaChannelBinding>,
    /// Protocol-specific information for an Anypoint MQ channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anypointmq: Option<AnyPointMQChannelBinding>,
    /// Protocol-specific information for an AMQP 0-9-1 channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amqp: Option<AMQPChannelBinding>,
    /// Protocol-specific information for an AMQP 1.0 channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amqp1: Option<AMQPChannelBinding>,
    /// Protocol-specific information for an MQTT channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt: Option<MQTTChannelBinding>,
    /// Protocol-specific information for an MQTT 5 channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt5: Option<MQTT5ChannelBinding>,
    /// Protocol-specific information for a NATS channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nats: Option<NATSChannelBinding>,
    /// Protocol-specific information for a JMS channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jms: Option<JMSChannelBinding>,
    /// Protocol-specific information for an SNS channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SNSChannelBinding>,
    /// Protocol-specific information for a Solace channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solace: Option<SolaceChannelBinding>,
    /// Protocol-specific information for an SQS channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<SQSChannelBinding>,
    /// Protocol-specific information for a STOMP channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stomp: Option<STOMPChannelBinding>,
    /// Protocol-specific information for a Redis channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis: Option<RedisChannelBinding>,
    /// Protocol-specific information for a Mercure channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mercure: Option<MercureChannelBinding>,
    /// Protocol-specific information for an IBM MQ channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibmmq: Option<IBMMQChannelBinding>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HTTPChannelBinding {}

/// When using WebSockets, the channel represents the connection. Unlike other protocols
/// that support multiple virtual channels (topics, routing keys, etc.) per connection,
/// WebSockets doesn't support virtual channels or, put it another way, there's only one
/// channel and its characteristics are strongly related to the protocol used for the
/// handshake, i.e., HTTP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebsocketsChannelBinding {
    /// The HTTP method to use when establishing the connection.
    /// Its value MUST be either `GET` or `POST`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// A Schema object containing the definitions for each query parameter.
    /// This schema MUST be of type `object` and have a `properties` key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Schema>,
    /// A Schema object containing the definitions of the HTTP headers to use when
    /// establishing the connection. This schema MUST be of type `object` and have
    /// a `properties` key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Schema>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct KafkaChannelBinding {}

/// The Anypoint MQ [Channel Binding Object][ChannelBinding] is defined by a
/// [JSON Schema](https://github.com/asyncapi/bindings/blob/master/anypointmq/json_schemas/channel.json),
/// which defines these fields.
///
/// Note that an Anypoint MQ exchange can only be sent to, not received from.
/// To receive messages sent to an exchange,
/// [an intermediary queue must be defined and bound to the exchange](https://docs.mulesoft.com/mq/mq-understanding#message-exchanges).
/// In this bindings specification, these intermediary queues are not exposed
/// in the AsyncAPI document. Instead, it is simply assumed that whenever
/// messages must be received from an exchange, such an intermediary queue is
/// involved yet invisible in the AsyncAPI document.
///
/// # Examples
///
/// The following example shows a `channels` object with two channels,
/// the second having a channel binding object for `anypointmq`:
///
/// ```yaml
/// channels:
///   user/signup:
///     description: |
///       This application receives command messages from this channel about users to sign up.
///       Minimal configuration, omitting a channel binding object.
///     publish:
///       #...
///   user/signedup:
///     description: |
///       This application sends events to this channel about users that have signed up.
///       Explicitly provides a channel binding object.
///     bindings:
///       anypointmq:
///         destination:     user-signup-exchg
///         destinationType: exchange
///         bindingVersion:  '0.0.1'
///     subscribe:
///       #...
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnyPointMQChannelBinding {
    /// **Optional**, defaults to the channel name. The destination (queue or exchange)
    /// name for this channel. SHOULD only be specified if the channel name differs
    /// from the actual destination name, such as when the channel name is not a valid
    /// destination name in Anypoint MQ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// **Optional**, defaults to `queue`. The type of destination, which MUST be
    /// either `exchange` or `queue` or `fifo-queue`. SHOULD be specified to document
    /// the messaging model (publish/subscribe, point-to-point, strict message
    /// ordering) supported by this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    /// **Optional**, defaults to `latest`. The version of this binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// This object contains information about the channel representation in AMQP.
///
/// # Examples
///
/// ```yaml
/// channels:
///   user/signedup:
///     bindings:
///       amqp:
///         is: routingKey
///         queue:
///           name: my-queue-name
///           durable: true
///           exclusive: true
///           autoDelete: false
///           vhost: /
///         exchange:
///           name: myExchange
///           type: topic
///           durable: true
///           autoDelete: false
///           vhost: /
///         bindingVersion: 0.2.0
///
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AMQPChannelBinding {
    /// Defines what type of channel is it. Can be either `queue` or `routingKey` (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is: Option<String>,
    /// When `is`=`routingKey`, this object defines the exchange properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<AMQPChannelBindingExchange>,
    /// When `is`=`queue`, this object defines the queue properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<AMQPChannelBindingQueue>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AMQPChannelBindingExchange {
    /// The name of the exchange. It MUST NOT exceed 255 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the exchange. Can be either
    /// `topic`, `direct`, `fanout`, `default` or `headers`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    /// Whether the exchange should survive broker restarts or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable: Option<bool>,
    /// Whether the exchange should be deleted when the last queue is unbound from it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete: Option<bool>,
    /// The virtual host of the exchange. Defaults to `/`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vhost: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AMQPChannelBindingQueue {
    /// The name of the queue. It MUST NOT exceed 255 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the queue should survive broker restarts or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable: Option<bool>,
    /// Whether the queue should be used only by one connection or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
    ///  Whether the queue should be deleted when the last consumer unsubscribes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete: Option<bool>,
    /// The virtual host of the queue. Defaults to `/`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vhost: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AMQP1ChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MQTTChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MQTT5ChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NATSChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JMSChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SNSChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolaceChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SQSChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct STOMPChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RedisChannelBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MercureChannelBinding {}

/// This object contains information about the channel representation in IBM MQ. Each channel corresponds to a Queue or Topic within IBM MQ.
///
/// # Examples
///
/// Example for an IBM MQ Topic where topic string is defined by AsyncAPI channel
/// ```yaml
/// channels:
///   user/signedup:
/// ```
///
/// Example for AsyncAPI channel mapping to an IBM MQ topic with a specified MQ Topic object
/// ```yaml
/// channels:
///   user/signedup:
///     bindings:
///       ibmmq:
///         destinationType: topic
///         topic:
///           objectName: myTopicName
///         bindingVersion: 0.1.0
/// ```
///
/// Example for AsyncAPI channel mapping to an IBM MQ Queue
/// ```yaml
/// channels:
///   user/signedup:
///     bindings:
///       ibmmq:
///         destinationType: queue
///         queue:
///           objectName: myQueueName
///           exclusive: true
///         bindingVersion: 0.1.0
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IBMMQChannelBinding {
    /// Defines the type of AsyncAPI channel.
    ///
    /// MUST be either `topic` or `queue`. For type `topic`,
    /// the AsyncAPI channel name MUST be assumed for the
    /// IBM MQ topic string unless overridden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    /// Defines the properties of a queue.
    ///
    /// `queue` and `topic` fields MUST NOT coexist within a channel binding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<IBMMQChannelBindingQueue>,
    /// Defines the properties of a topic.
    ///
    /// `queue` and `topic` fields MUST NOT coexist within a channel binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<IBMMQChannelBindingTopic>,
    /// The maximum length of the physical message (in bytes) accepted
    /// by the Topic or Queue. Messages produced that are greater in size
    /// than this value may fail to be delivered. More information on the
    /// maximum message length can be found on this
    /// [page](https://www.ibm.com/support/knowledgecenter/SSFKSJ_latest/com.ibm.mq.ref.adm.doc/q085520_.html#q085520___maxmsgl)
    /// in the IBM MQ Knowledge Center.
    ///
    /// MUST be `0-104,857,600` bytes (100 MB).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_msg_length: Option<i32>,
    /// The version of this binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IBMMQChannelBindingQueue {
    /// Defines the name of the IBM MQ queue associated with the channel.
    ///
    /// A value MUST be specified. MUST NOT exceed 48 characters in length.
    /// MUST be a valid IBM MQ queue name
    pub object_name: String,
    /// Defines if the queue is a cluster queue and therefore partitioned.
    /// If true, a binding option MAY be specified when accessing the queue.
    /// More information on binding options can be found on this
    /// [page](https://www.ibm.com/support/knowledgecenter/SSFKSJ_latest/com.ibm.mq.ref.dev.doc/q101870_.html#q101870___BIND_ON_OPEN)
    /// in the IBM MQ Knowledge Center.
    ///
    /// If `false`, binding options SHOULD NOT be specified when accessing the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_partitioned: Option<bool>,
    /// Specifies if it is recommended to open the queue exclusively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IBMMQChannelBindingTopic {
    /// The value of the IBM MQ topic string to be used.
    ///
    /// Note: if specified, SHALL override AsyncAPI channel name.
    ///
    /// MUST NOT exceed 10240 characters in length.
    /// MAY coexist with `topic.objectName`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    /// The name of the IBM MQ topic object.
    ///
    /// Note: if specified, SHALL override AsyncAPI channel name.
    ///
    /// MUST NOT exceed 48 characters in length.
    /// MAY coexist with `topic.string`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    /// Defines if the subscription may be durable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_permitted: Option<bool>,
    /// Defines if the last message published will be made
    /// available to new subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_msg_retained: Option<bool>,
}
