use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    /// A simple object to allow referencing other components in the specification,
    /// internally and externally.
    ///
    /// The Reference Object is defined by
    /// [JSON Reference](https://tools.ietf.org/html/draft-pbryan-zyp-json-ref-03)
    /// and follows the same structure,
    /// behavior and rules. A JSON Reference SHALL only be used to refer to a schema that
    /// is formatted in either JSON or YAML. In the case of a YAML-formatted Schema,
    /// the JSON Reference SHALL be applied to the JSON representation of that schema.
    /// The JSON representation SHALL be made by applying the conversion described
    /// [here](https://www.asyncapi.com/docs/specifications/v2.3.0#format).
    ///
    /// For this specification, reference resolution is done as defined by the
    /// JSON Reference specification and not by the JSON Schema specification.
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Item(T),
}

impl<T> ReferenceOr<T> {
    pub fn ref_(r: &str) -> Self {
        ReferenceOr::Reference {
            reference: r.to_owned(),
        }
    }
    pub fn boxed_item(item: T) -> ReferenceOr<Box<T>> {
        ReferenceOr::Item(Box::new(item))
    }
}

impl<T> ReferenceOr<Box<T>> {
    pub fn unbox(self) -> ReferenceOr<T> {
        match self {
            ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference },
            ReferenceOr::Item(boxed) => ReferenceOr::Item(*boxed),
        }
    }
}
