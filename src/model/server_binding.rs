use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Map describing protocol-specific definitions for a server.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerBinding {
    /// Protocol-specific information for an HTTP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPServerBinding>,
    /// Protocol-specific information for a WebSockets server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<WebsocketsServerBinding>,
    /// Protocol-specific information for a Kafka server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka: Option<KafkaServerBinding>,
    /// Protocol-specific information for an Anypoint MQ server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anypointmq: Option<AnyPointMQServerBinding>,
    /// Protocol-specific information for an AMQP 0-9-1 server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amqp: Option<AMPQServerBinding>,
    /// Protocol-specific information for an AMQP 1.0 server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ampq1: Option<AMPQ1ServerBinding>,
    ///	Protocol-specific information for an MQTT server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt: Option<MQTTServerBinding>,
    /// Protocol-specific information for an MQTT 5 server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt5: Option<MQTT5ServerBinding>,
    /// Protocol-specific information for a NATS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nats: Option<NATSServerBinding>,
    /// Protocol-specific information for a JMS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jms: Option<JMSServerBinding>,
    /// Protocol-specific information for an SNS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SNSServerBinding>,
    /// Protocol-specific information for a Solace server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solace: Option<SolaceServerBinding>,
    /// Protocol-specific information for an SQS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<SQSServerBinding>,
    /// Protocol-specific information for a STOMP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stomp: Option<STOMPServerBinding>,
    /// Protocol-specific information for a Redis server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis: Option<RedisServerBinding>,
    ///	Protocol-specific information for a Mercure server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mercure: Option<MercureServerBinding>,
    /// Protocol-specific information for an IBM MQ server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibmmq: Option<IBMMQServerBinding>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HTTPServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebsocketsServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct KafkaServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnyPointMQServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AMPQServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AMPQ1ServerBinding {}

/// This object contains information about the server representation in MQTT.
///
/// # Examples
/// ```yaml
/// servers:
///   production:
///     bindings:
///       mqtt:
///         clientId: guest
///         cleanSession: true
///         lastWill:
///           topic: /last-wills
///           qos: 2
///           message: Guest gone offline.
///           retain: false
///         keepAlive: 60
///         bindingVersion: 0.1.0
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MQTTServerBinding {
    /// The client identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Whether to create a persisten connection or not.
    /// When `false`, the connection will be persistent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_session: Option<bool>,
    /// Last Will and Testament configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_will: Option<MQTTServerBindingLasWill>,
    /// Interval in seconds of the longest period of time the broker
    /// and the client can endure without sending a message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<i32>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// Last Will and Testament configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MQTTServerBindingLasWill {
    /// The topic where the Last Will and Testament message will be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// Defines how hard the broker/client will try to ensure that the
    /// Last Will and Testament message is received. Its value MUST be
    /// either 0, 1 or 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// Last Will message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Whether the broker should retain the Last Will and
    /// Testament message or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MQTT5ServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NATSServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct JMSServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SNSServerBinding {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolaceServerBinding {
    /// The current version is 0.2.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
    /// The Virtual Private Network name on the Solace broker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_vpn: Option<String>,
}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SQSServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct STOMPServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RedisServerBinding {}

/// This object MUST NOT contain any properties. Its name is reserved for future use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MercureServerBinding {}

/// This object contains server connection information about the IBM MQ server,
/// referred to as an IBM MQ queue manager. This object contains additional
/// connectivity information not possible to represent within the core
/// AsyncAPI specification.
///
/// # Examples
///
/// Example for multiple endpoints defined in the AsyncAPI configuration
///
/// ```yaml
/// servers:
///   production1:
///     url: ibmmq://qmgr1host:1414/qm1/DEV.APP.SVRCONN
///     protocol: ibmmq-secure
///     description: Production Instance 1
///     bindings:
///       ibmmq:
///         groupId: PRODCLSTR1
///         cipherSpec: ANY_TLS12_OR_HIGHER
///         bindingVersion: 0.1.0
///   production2:
///     url: ibmmq://qmgr2host:1414/qm2/DEV.APP.SVRCONN
///     protocol: ibmmq-secure
///     description: Production Instance 2
///     bindings:
///       ibmmq:
///         groupId: PRODCLSTR1
///         bindingVersion: 0.1.0
/// ```
///
/// Example using combined strategy
///
/// ```yaml
/// servers:
///   production:
///     url: 'http://my-ccdt-json-file'
///     protocol: ibmmq-secure
///     description: Production MQ Instance
///     bindings:
///       ibmmq:
///         ccdtQueueManagerName: qm1
///   test:
///     url: ibmmq://qmgrtest:1414/qm2/DEV.APP.SVRCONN
///     protocol: ibmmq-secure
///     description: Test MQ Instance
///     bindings:
///       ibmmq:
///         cipherSpec: ANY_TLS12_OR_HIGHER
///         bindingVersion: 0.1.0
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IBMMQServerBinding {
    /// Defines a logical group of IBM MQ server objects. This is necessary to specify
    /// multi-endpoint configurations used in high availability deployments.
    /// If omitted, the server object is not part of a group.
    ///
    /// MUST NOT be specified for URI Scheme `http://` or `file://`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The name of the IBM MQ queue manager to bind to in the CCDT file.
    ///
    /// MUST NOT be specified for URI Scheme `ibmmq://`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccdt_queue_manager_name: Option<String>,
    /// The recommended cipher specification used to establish a TLS connection
    /// between the client and the IBM MQ queue manager. More information on
    /// SSL/TLS cipher specifications supported by IBM MQ can be found on this
    /// [page](https://www.ibm.com/support/knowledgecenter/SSFKSJ_latest/com.ibm.mq.dev.doc/q113220_.html)
    /// in the IBM MQ Knowledge Center.
    ///
    /// MUST NOT be specified for protocol ibmmq or URI Scheme `file://` or `http://`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_spec: Option<String>,
    /// If `multiEndpointServer` is `true` then multiple connections can be workload balanced
    /// and applications should not make assumptions as to where messages are processed.
    // Where message ordering, or affinity to specific message resources is necessary, a
    /// single endpoint (`multiEndpointServer` = `false`) may be required.
    ///
    /// MUST NOT be specified for URI Scheme `file://` or `http://`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_endpoint_server: Option<bool>,
    /// The recommended value (in seconds) for the heartbeat sent to the queue manager during
    /// periods of inactivity. A value of zero means that no heart beats are sent. A value of
    /// `1` means that the client will use the value defined by the queue manager. More
    /// information on heart beat interval can be found on this
    /// [page](https://www.ibm.com/support/knowledgecenter/SSFKSJ_latest/com.ibm.mq.ref.dev.doc/q108450_.html)
    /// in the IBM MQ Knowledge Center.
    ///
    /// MUST be `0-999999`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heart_beat_interval: Option<i32>,
    /// The version of this binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}
