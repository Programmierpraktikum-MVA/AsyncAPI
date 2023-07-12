use gtmpl_derive::Gtmpl;
use serde::Serialize;
#[derive(Debug, Clone, Serialize, Gtmpl)]
pub struct RustSchemaRepresentation {
    // the unique identifier (e.g. UserSignupMessage)
    pub unique_id: String,
    pub original_key: String,
    // used to reference the model (e.g. UserSignupMessage, but for primitive schemas simply the primitive type e.g. String/f64)
    pub struct_reference: String,
    // model definition (e.g. pub struct UserSignupMessage { ... } or pub enum UserSignupMessage { ... }, is empty for primitive types)
    pub model_definition: String,
    // related models which are referenced by this model
    pub related_models: Vec<RustSchemaRepresentation>,
    // the type of the model (e.g. struct or enum or primitive)
    pub model_type: String,
}

impl RustSchemaRepresentation {
    pub fn get_related_models_recursive(&self) -> Vec<RustSchemaRepresentation> {
        let children_models: Vec<RustSchemaRepresentation> = self
            .related_models
            .clone()
            .iter()
            .flat_map(|x| x.get_related_models_recursive())
            .collect();
        [vec![self.clone()], children_models].concat()
    }
}
