use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{Channel, Components, ExternalDocumentation, Info, ReferenceOr, Server, Tag};

/// This is the root document object for the API specification.
/// It combines resource listing and API declaration together into one document.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AsyncAPI {
    /// **Required.** Specifies the AsyncAPI Specification version being used.
    /// It can be used by tooling Specifications and clients to interpret the
    /// version. The structure shall be `major`.`minor`.`patch`, where `patch`
    /// versions must be compatible with the existing `major`.`minor` tooling.
    /// Typically patch versions will be introduced to address errors in the
    /// documentation, and tooling should typically be compatible with the
    /// corresponding `major`.`minor` (1.0.*). Patch versions will correspond
    /// to patches of this document.
    ///
    /// The version string signifies the version of the AsyncAPI Specification
    /// that the document complies to. The format for this string must be
    /// `major`.`minor`.`patch`. The `patch` may be suffixed by a hyphen and
    /// extra alphanumeric characters.
    ///
    /// A `major`.`minor` shall be used to designate the AsyncAPI
    /// Specification version, and will be considered compatible with the
    /// AsyncAPI Specification specified by that `major`.`minor` version. The
    /// patch version will not be considered by tooling, making no distinction
    /// between `1.0.0` and `1.0.1`.
    ///
    /// In subsequent versions of the AsyncAPI Specification, care will be
    /// given such that increments of the `minor` version should not interfere
    /// with operations of tooling developed to a lower minor version. Thus a
    /// hypothetical `1.1.0` specification should be usable with tooling
    /// designed for `1.0.0`.
    pub asyncapi: String,
    /// Identifier of the
    /// [application](https://www.asyncapi.com/docs/specifications/v2.3.0#definitionsApplication)
    /// the AsyncAPI document is defining.
    ///
    /// This field represents a unique universal identifier of the
    /// [application](https://www.asyncapi.com/docs/specifications/v2.3.0#definitionsApplication)
    /// the AsyncAPI document is defining. It must conform to the URI format,
    /// according to [RFC3986](https://tools.ietf.org/html/rfc3986).
    ///
    /// It is RECOMMENDED to use a [URN](https://tools.ietf.org/html/rfc8141)
    /// to globally and uniquely identify the application during long periods
    /// of time, even after it becomes unavailable or ceases to exist.
    ///
    /// # Examples
    ///
    /// ```json
    /// {
    ///     "id": "urn:com:smartylighting:streetlights:server"
    /// }
    /// ```
    ///
    /// ```yaml
    /// id: 'urn:com:smartylighting:streetlights:server'
    /// ```
    ///
    /// ```json
    /// {
    ///     "id": "https://github.com/smartylighting/streetlights-server"
    /// }
    /// ```
    ///
    /// ```yaml
    /// id: 'https://github.com/smartylighting/streetlights-server'
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// **Required.** Provides metadata about the API.
    /// The metadata can be used by the clients if needed.
    pub info: Info,
    /// Provides connection details of servers.
    ///
    /// The Servers Object is a map of
    /// [Server Objects][crate::Server].
    ///
    /// # Examples
    /// ```json
    /// {
    ///     "production": {
    ///         "url": "development.gigantic-server.com",
    ///         "description": "Development server",
    ///         "protocol": "kafka",
    ///         "protocolVersion": "1.0.0"
    ///     }
    /// }
    /// ```
    ///
    /// ```yaml
    /// production:
    ///     url: development.gigantic-server.com
    ///     description: Development server
    ///     protocol: kafka
    ///     protocolVersion: '1.0.0'
    /// ```
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub servers: IndexMap<String, ReferenceOr<Server>>,
    /// Default content type to use when encoding/decoding a message's payload.
    /// A string representing the default content type to use when encoding/decoding a
    /// message's payload. The value MUST be a specific media type (e.g. `application/json`).
    /// This value MUST be used by schema parsers when the
    /// [contentType](https://www.asyncapi.com/docs/specifications/v2.3.0#messageObjectContentType)
    /// property is omitted.
    ///
    /// In case a message can't be encoded/decoded using this value, schema
    /// parsers MUST use their default content type.
    ///
    /// # Examples
    /// ```json
    /// {
    ///   "defaultContentType": "application/json"
    /// }
    /// ```
    ///
    /// ```yaml
    /// defaultContentType: application/json
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_content_type: Option<String>,
    /// **Required** The available channels and messages for the API.
    ///
    /// Holds the relative paths to the individual channel and their operations.
    /// Channel paths are relative to servers.
    ///
    /// Channels are also known as "topics", "routing keys", "event types" or "paths".
    ///
    /// Each item is a relative path to an individual channel. The field name MUST be in
    /// the form of a [RFC 6570 URI template](https://tools.ietf.org/html/rfc6570).
    /// Query parameters and fragments SHALL NOT be used, instead use
    /// [bindings][crate::ChannelBinding] to define them.
    ///
    /// # Examples
    ///
    /// ```json
    /// {
    ///     "user/signedup": {
    ///         "subscribe": {
    ///         "$ref": "#/components/messages/userSignedUp"
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```yaml
    /// user/signedup:
    ///   subscribe:
    ///     $ref: "#/components/messages/userSignedUp"
    /// ```
    pub channels: IndexMap<String, Channel>,
    /// An element to hold various schemas for the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    /// A list of tags used by the specification with additional metadata.
    /// Each tag name in the list MUST be unique.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
