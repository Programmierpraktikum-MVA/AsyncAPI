use crate::{
    asyncapi_model::{Message, Operation, Server},
    parser::json_schema_parser::types::RustSchemaRepresentation,
};
use gtmpl::Value;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::string::*;

#[derive(Serialize, Debug)]
pub struct TemplateContext<'a> {
    pub title: &'a String,
    pub description: &'a Option<String>,
    pub server: &'a Server,
    pub subscribe_channels: Vec<(&'a String, SimplifiedOperation)>,
    pub publish_channels: Vec<(&'a String, SimplifiedOperation)>,
    pub model: Model,
}

#[derive(Serialize, Debug)]
pub struct Model {
    pub message_models: Vec<RustSchemaRepresentation>,
    // pub enums: Vec<MultiStructEnum>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SimplifiedOperation {
    pub unique_id: String,
    pub original_operation: Operation,
    // array, da es eine oder mehrere messages geben kann
    pub messages: Vec<SimplifiedMessage>,
    // pub multiple_messages_enum: Option<MultiStructEnum>,
}
#[derive(Serialize, Debug, Clone)]

pub struct MultiStructEnum {
    pub unique_id: String,
    pub messages: Vec<SimplifiedMessage>,
    pub struct_definition: String,
}
#[derive(Serialize, Debug, Clone)]

pub struct SimplifiedMessage {
    pub unique_id: String,
    pub original_message: Message,
    pub payload: Option<RustSchemaRepresentation>,
    pub payload_schema: Option<String>,
}

/// FIXME: these are just a quick workaround until gtmpl::Value supports `From<impl Serialize> for gtmpl::Value`
impl<'a> From<&TemplateContext<'a>> for gtmpl::Value {
    fn from(value: &TemplateContext<'a>) -> Self {
        let json_value: serde_json::Value = serde_json::to_value(value).unwrap();
        serde_value_to_gtmpl_value(&json_value)
    }
}
impl From<&SimplifiedOperation> for gtmpl::Value {
    fn from(value: &SimplifiedOperation) -> Self {
        let json_value: serde_json::Value = serde_json::to_value(value).unwrap();
        serde_value_to_gtmpl_value(&json_value)
    }
}

impl From<&SimplifiedMessage> for gtmpl::Value {
    fn from(value: &SimplifiedMessage) -> Self {
        let json_value: serde_json::Value = serde_json::to_value(value).unwrap();
        serde_value_to_gtmpl_value(&json_value)
    }
}

/// converts any serde serializable value to a gtmpl value
/// WARNING: clones objects, so not exactly zero cost abstraction 🤷‍♂️
fn serde_value_to_gtmpl_value(value: &serde_json::Value) -> gtmpl::Value {
    match value {
        JsonValue::Null => Value::Nil,
        JsonValue::Bool(b) => Value::Bool(*b),
        JsonValue::Number(n) => {
            if let Some(int_val) = n.as_i64() {
                Value::Number(int_val.into())
            } else if let Some(float_val) = n.as_f64() {
                Value::Number(float_val.into())
            } else if let Some(uint_val) = n.as_u64() {
                Value::Number(uint_val.into())
            } else {
                Value::Nil
            }
        }
        JsonValue::String(s) => Value::String(s.clone()),
        JsonValue::Array(arr) => {
            let converted: Vec<gtmpl::Value> = arr.iter().map(serde_value_to_gtmpl_value).collect();
            Value::Array(converted)
        }
        JsonValue::Object(obj) => {
            let converted = obj
                .iter()
                .map(|(k, v)| (k.clone(), serde_value_to_gtmpl_value(v)))
                .collect();
            Value::Object(converted)
        }
    }
}
