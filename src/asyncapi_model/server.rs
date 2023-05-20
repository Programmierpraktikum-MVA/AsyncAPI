use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{ReferenceOr, ServerBinding};

/// An object representing a message broker, a server or any other kind of
/// computer program capable of sending and/or receiving data. This object is
/// used to capture details such as URIs, protocols and security configuration.
/// Variable substitution can be used so that some details, for example
/// usernames and passwords, can be injected by code generation tools.
///
/// # Examples
///
/// A single server would be described as:
/// ```json
/// {
///     "url": "development.gigantic-server.com",
///     "description": "Development server",
///     "protocol": "kafka",
///     "protocolVersion": "1.0.0"
/// }
/// ```
///
/// ```yaml
/// url: development.gigantic-server.com
/// description: Development server
/// protocol: kafka
/// protocolVersion: '1.0.0'
/// ```
///
/// The following shows how multiple servers can be described, for example,
/// at the AsyncAPI Object's `servers`:
///
/// ```json
/// {
///     "servers": {
///         "development": {
///         "url": "development.gigantic-server.com",
///         "description": "Development server",
///         "protocol": "amqp",
///         "protocolVersion": "0.9.1"
///         },
///         "staging": {
///         "url": "staging.gigantic-server.com",
///         "description": "Staging server",
///         "protocol": "amqp",
///         "protocolVersion": "0.9.1"
///         },
///         "production": {
///         "url": "api.gigantic-server.com",
///         "description": "Production server",
///         "protocol": "amqp",
///         "protocolVersion": "0.9.1"
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// servers:
///   development:
///     url: development.gigantic-server.com
///     description: Development server
///     protocol: amqp
///     protocolVersion: 0.9.1
///   staging:
///     url: staging.gigantic-server.com
///     description: Staging server
///     protocol: amqp
///     protocolVersion: 0.9.1
///   production:
///     url: api.gigantic-server.com
///     description: Production server
///     protocol: amqp
///     protocolVersion: 0.9.1
/// ```
///
/// The following shows how variables can be used for a server configuration:
///
/// ```json
/// {
///     "servers": {
///         "production": {
///             "url": "{username}.gigantic-server.com:{port}/{basePath}",
///             "description": "The production API server",
///             "protocol": "secure-mqtt",
///             "variables": {
///                 "username": {
///                 "default": "demo",
///                 "description": "This value is assigned by the service provider, in this example `gigantic-server.com`"
///                 },
///                 "port": {
///                 "enum": [
///                     "8883",
///                     "8884"
///                 ],
///                 "default": "8883"
///                 },
///                 "basePath": {
///                 "default": "v2"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// servers:
///   production:
///     url: '{username}.gigantic-server.com:{port}/{basePath}'
///     description: The production API server
///     protocol: secure-mqtt
///     variables:
///       username:
///         # note! no enum here means it is an open value
///         default: demo
///         description: This value is assigned by the service provider, in this example `gigantic-server.com`
///       port:
///         enum:
///           - '8883'
///           - '8884'
///         default: '8883'
///       basePath:
///         # open meaning there is the opportunity to use special base paths as assigned by the provider, default is `v2`
///         default: v2
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    /// **REQUIRED.** A URL to the target host. This URL supports Server
    /// Variables and MAY be relative, to indicate that the host location is
    /// relative to the location where the AsyncAPI document is being served.
    /// Variable substitutions will be made when a variable is named in
    /// `{`brackets`}`.
    pub url: String,
    /// **REQUIRED.** The protocol this URL supports for connection.
    /// Supported protocol include, but are not limited to:
    /// `amqp`, `amqps`, `http`, `https`, `ibmmq`, `jms`, `kafka`,
    /// `kafka-secure`, `mqtt`, `secure-mqtt`, `stomp`, `stomps`, `ws`,
    /// `wss`, `mercure`.
    pub protocol: String,
    /// The version of the protocol used for connection.
    /// For instance: AMQP `0.9.1`, HTTP `2.0`, Kafka `1.0.0`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
    /// An optional string describing the host designated by the URL.
    /// [CommonMark syntax](https://spec.commonmark.org/) MAY be used
    /// for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A map between a variable name and its value. The value is used
    /// for substitution in the server's URL template.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub variables: IndexMap<String, ServerVariable>,
    /// A declaration of which security mechanisms can be used with this
    /// server. The list of values includes alternative security requirement
    /// objects that can be used. Only one of the security requirement objects
    /// need to be satisfied to authorize a connection or operation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<SecurityRequirement>,
    /// A map where the keys describe the name of the protocol and the values
    /// describe protocol-specific definitions for the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<ReferenceOr<ServerBinding>>,
    /// This object MAY be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// An object representing a Server Variable for server URL
/// template substitution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerVariable {
    /// An enumeration of string values to be used if the substitution options are from a limited set.
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<Vec<String>>,
    /// The default value to use for substitution, and to send,
    /// if an alternate value is not supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// An optional description for the server variable.
    /// [CommonMark syntax](https://spec.commonmark.org/)
    /// MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An array of examples of the server variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
    /// This object MAY be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// Lists the required security schemes to execute this operation. The name
/// used for each property MUST correspond to a security scheme declared in the
/// Security Schemes under the Components Object.
///
/// When a list of Security Requirement Objects is defined on a Server object,
/// only one of the Security Requirement Objects in the list needs to be
/// satisfied to authorize the connection.
///
/// # Examples
/// ## User/Password Security Requirement
///
/// ```json
/// {
///     "user_pass": []
/// }
/// ```
///
/// ```yaml
/// user_pass: []
/// ```
///
/// ## API Key Security Requirement
///
/// ```json
/// {
///     "api_key": []
/// }
/// ```
///
/// ```yaml
/// api_key: []
/// ```
///
/// ## OAuth2 Security Requirement
///
/// ```json
/// {
///     "petstore_auth": [
///         "write:pets",
///         "read:pets"
///     ]
/// }
/// ```
///
/// ```yaml
/// petstore_auth:
/// - write:pets
/// - read:pets
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRequirement {
    /// Each name MUST correspond to a security scheme which is declared in the
    /// [Security Schemes][crate::SecurityScheme]
    /// under the [Components Object][crate::Components].
    /// If the security scheme is of type `"oauth2"` or `"openIdConnect"`, then
    /// the value is a list of scope names. Provide scopes that are required to
    /// establish successful connection with the server. If scopes are not
    /// needed, the list can be empty. For other security scheme types, the
    /// array MUST be empty.
    #[serde(flatten)]
    pub values: IndexMap<String, Vec<String>>,
}
