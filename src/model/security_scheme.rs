use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Defines a security scheme that can be used by the operations. Supported schemes are:
///
/// * User/Password.
/// * API key (either as user or as password).
/// * X.509 certificate.
/// * End-to-end encryption (either symmetric or asymmetric).
/// * HTTP authentication.
/// * HTTP API key.
/// * OAuth2's common flows (Implicit, Resource Owner Protected Credentials, Client Credentials and Authorization Code) as defined in RFC6749.
/// * OpenID Connect Discovery.
/// * SASL (Simple Authentication and Security Layer) as defined in RFC4422.
///
/// # Examples
///
/// ## User/Password Authentication Sample
/// ```json
/// {
///     "type": "userPassword"
/// }
/// ```
///
/// ```yaml
/// type: userPassword
/// ```
///
/// ## API Key Authentication Sample
/// ```json
/// {
///     "type": "apiKey",
///     "in": "user"
/// }
/// ```
///
/// ```yaml
/// type: apiKey,
/// in: user
/// ```
///
/// ## X.509 Authentication Sample
/// ```json
/// {
///     "type": "X509"
/// }
/// ```
///
/// ```yaml
/// type: X509
/// ```
///
/// ## End-to-end Encryption Authentication Sample
/// ```json
/// {
///     "type": "symmetricEncryption"
/// }
/// ```
///
/// ```yaml
/// type: symmetricEncryption
/// ```
///
/// ## Basic Authentication Sample
/// ```json
/// {
///     "type": "http",
///     "scheme": "basic"
/// }
/// ```
///
/// ```yaml
/// type: http
/// scheme: basic
/// ```
///
/// ## API Key Sample
/// ```json
/// {
///     "type": "httpApiKey",
///     "name": "api_key",
///     "in": "header"
/// }
/// ```
///
/// ```yaml
/// type: httpApiKey
/// name: api_key
/// in: header
/// ```
///
/// ## JWT Bearer Sample
/// ```json
/// {
///     "type": "http",
///     "scheme": "bearer",
///     "bearerFormat": "JWT"
/// }
/// ```
///
/// ```yaml
/// type: http
/// scheme: bearer
/// bearerFormat: JWT
/// ```
///
/// ## Implicit OAuth2 Sample
/// ```json
/// {
///     "type": "oauth2",
///     "flows": {
///         "implicit": {
///         "authorizationUrl": "https://example.com/api/oauth/dialog",
///         "scopes": {
///             "write:pets": "modify pets in your account",
///             "read:pets": "read your pets"
///         }
///         }
///     }
/// }
/// ```
///
/// ```yaml
/// type: oauth2
/// flows:
///   implicit:
///     authorizationUrl: https://example.com/api/oauth/dialog
///     scopes:
///       write:pets: modify pets in your account
///       read:pets: read your pets
/// ```
///
/// ## SASL Sample
/// ```json
/// {
///     "type": "scramSha512"
/// }
/// ```
///
/// ```yaml
/// type: scramSha512
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "userPassword")]
    UserPassword {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "apiKey")]
    ApiKey {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// REQUIRED. The location of the API key.
        /// Valid values are `"user"` and `"password"`.
        #[serde(rename = "in")]
        location: String,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    X509 {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "symmetricEncryption")]
    SymmetricEncryption {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "asymmetricEncryption")]
    AsymmetricEncryption {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "httpApiKey")]
    HttpApiKey {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// **REQUIRED**. The name of the header,
        /// query or cookie parameter to be used.
        name: String,
        /// REQUIRED. The location of the API key.
        /// Valid values are `"query"`, `"header"` or `"cookie"`.
        #[serde(rename = "in")]
        location: String,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "http", rename_all = "camelCase")]
    Http {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// **REQUIRED**. The name of the HTTP Authorization scheme
        /// to be used in the Authorization header as defined in
        /// [RFC7235](https://tools.ietf.org/html/rfc7235#section-5.1).
        scheme: String,
        /// A hint to the client to identify how the bearer token is formatted.
        /// Bearer tokens are usually generated by an authorization server,
        /// so this information is primarily for documentation purposes.
        #[serde(skip_serializing_if = "Option::is_none")]
        bearer_format: Option<String>,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    /// # Examples
    /// ```json
    /// {
    ///     "type": "oauth2",
    ///     "flows": {
    ///         "implicit": {
    ///         "authorizationUrl": "https://example.com/api/oauth/dialog",
    ///         "scopes": {
    ///             "write:pets": "modify pets in your account",
    ///             "read:pets": "read your pets"
    ///         }
    ///         },
    ///         "authorizationCode": {
    ///         "authorizationUrl": "https://example.com/api/oauth/dialog",
    ///         "tokenUrl": "https://example.com/api/oauth/token",
    ///         "scopes": {
    ///             "write:pets": "modify pets in your account",
    ///             "read:pets": "read your pets"
    ///         }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```yaml
    /// type: oauth2
    /// flows:
    ///   implicit:
    ///     authorizationUrl: https://example.com/api/oauth/dialog
    ///     scopes:
    ///       write:pets: modify pets in your account
    ///       read:pets: read your pets
    ///   authorizationCode:
    ///     authorizationUrl: https://example.com/api/oauth/dialog
    ///     tokenUrl: https://example.com/api/oauth/token
    ///     scopes:
    ///       write:pets: modify pets in your account
    ///       read:pets: read your pets
    /// ```
    #[serde(rename = "oauth2")]
    OAuth2 {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// **REQUIRED**. An object containing configuration
        /// information for the flow types supported.
        flows: OAuthFlows,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "openIdConnect", rename_all = "camelCase")]
    OpenIdConnect {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// **REQUIRED**. OpenId Connect URL to discover
        /// OAuth2 configuration values. This MUST be in the form of a URL.
        open_id_connect_url: String,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "plain")]
    Plain {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "scramSha256")]
    ScramSha256 {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "scramSha512")]
    ScramSha512 {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "gssapi")]
    Gssapi {
        /// A short description for security scheme.
        /// [CommonMark syntax](https://spec.commonmark.org/)
        /// MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// This object MAY be extended with
        /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
}

/// Allows configuration of the supported OAuth Flows.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlows {
    /// Configuration for the OAuth Implicit flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlowImplicit>,
    /// Configuration for the OAuth Resource Owner Protected Credentials flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlowPassword>,
    /// Configuration for the OAuth Client Credentials flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlowClientCredentials>,
    /// Configuration for the OAuth Authorization Code flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlowAuthorizationCode>,
    /// This object MAY be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// Configuration details for a supported OAuth Flow
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowImplicit {
    /// **REQUIRED**. The authorization URL to be used for this flow.
    /// This MUST be in the form of a URL.
    pub authorization_url: String,
    /// The URL to be used for obtaining refresh tokens.
    /// This MUST be in the form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// **REQUIRED**. The available scopes for the OAuth2 security scheme.
    /// A map between the scope name and a short description for it.
    pub scopes: IndexMap<String, String>,
    /// This object MAY be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// Configuration details for a supported OAuth Flow
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowPassword {
    /// **REQUIRED**. The token URL to be used for this flow.
    /// This MUST be in the form of a URL.
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens.
    /// This MUST be in the form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// **REQUIRED**. The available scopes for the OAuth2 security scheme.
    /// A map between the scope name and a short description for it.
    pub scopes: IndexMap<String, String>,
    /// This object MAY be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// Configuration details for a supported OAuth Flow
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowClientCredentials {
    /// **REQUIRED**. The token URL to be used for this flow.
    /// This MUST be in the form of a URL.
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens.
    /// This MUST be in the form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// **REQUIRED**. The available scopes for the OAuth2 security scheme.
    /// A map between the scope name and a short description for it.
    pub scopes: IndexMap<String, String>,
    /// This object MAY be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// Configuration details for a supported OAuth Flow
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowAuthorizationCode {
    /// **REQUIRED**. The authorization URL to be used for this flow.
    /// This MUST be in the form of a URL.
    pub authorization_url: String,
    /// **REQUIRED**. The token URL to be used for this flow.
    /// This MUST be in the form of a URL.
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens.
    /// This MUST be in the form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// **REQUIRED**. The available scopes for the OAuth2 security scheme.
    /// A map between the scope name and a short description for it.
    pub scopes: IndexMap<String, String>,
    /// This object MAY be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[test]
fn test_deserialize_security_scheme() {
    use super::ReferenceOr;

    let example = r#"
    type: apiKey
    in: user
    description: Provide your API key as the user and leave the password empty.
    "#;
    let asyncapi: ReferenceOr<SecurityScheme> = serde_yaml::from_str(example)
        .expect(&format!("Could not deserialize api key security scheme"));
    assert_eq!(
        ReferenceOr::Item(SecurityScheme::ApiKey {
            location: "user".to_string(),
            description: Some(
                "Provide your API key as the user and leave the password empty.".to_string(),
            ),
            extensions: Default::default(),
        }),
        asyncapi
    );
}
