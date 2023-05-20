use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// The object provides metadata about the API. The metadata can be used by the clients if needed.
///
/// # Examples
/// ```json
/// {
///     "title": "AsyncAPI Sample App",
///     "description": "This is a sample server.",
///     "termsOfService": "https://asyncapi.org/terms/",
///     "contact": {
///         "name": "API Support",
///         "url": "https://www.asyncapi.org/support",
///         "email": "support@asyncapi.org"
///     },
///     "license": {
///         "name": "Apache 2.0",
///         "url": "https://www.apache.org/licenses/LICENSE-2.0.html"
///     },
///     "version": "1.0.1"
/// }
/// ```
///
/// ```yaml
/// title: AsyncAPI Sample App
/// description: This is a sample server.
/// termsOfService: https://asyncapi.org/terms/
/// contact:
///   name: API Support
///   url: https://www.asyncapi.org/support
///   email: support@asyncapi.org
/// license:
///   name: Apache 2.0
///   url: https://www.apache.org/licenses/LICENSE-2.0.html
/// version: 1.0.1
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// **Required.** The title of the application.
    pub title: String,
    /// **Required** Provides the version of the application API
    /// (not to be confused with the specification version).
    pub version: String,
    /// A short description of the application.
    /// CommonMark syntax can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API.
    /// MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// Contact information for the exposed API.
///
/// # Examples
/// ```json
/// {
///     "name": "API Support",
///     "url": "https://www.example.com/support",
///     "email": "support@example.com"
/// }
/// ```
///
/// ```yaml
/// name: API Support
/// url: https://www.example.com/support
/// email: support@example.com
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    /// MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The email address of the contact person/organization.
    /// MUST be in the format of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// License information for the exposed API.
///
/// # Examples
/// ```json
/// {
///     "name": "Apache 2.0",
///     "url": "https://www.apache.org/licenses/LICENSE-2.0.html"
/// }
/// ```
///
/// ```yaml
/// name: Apache 2.0
/// url: https://www.apache.org/licenses/LICENSE-2.0.html
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    /// **Required.** The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API.
    /// MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// This object can be extended with
    /// [Specification Extensions](https://www.asyncapi.com/docs/specifications/v2.3.0#specificationExtensions).
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
