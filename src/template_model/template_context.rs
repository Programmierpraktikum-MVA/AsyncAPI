use crate::asyncapi_model::Server;
use gtmpl::Value;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::string::*;

use super::{MultiStructEnum, SimplifiedMessage, SimplifiedOperation};

#[derive(Serialize, Debug)]
pub struct Model {
    pub messages: Vec<SimplifiedMessage>,
    pub enums: Vec<MultiStructEnum>,
}

#[derive(Serialize, Debug)]
pub struct TemplateContext<'a> {
    pub title: &'a String,
    pub description: &'a Option<String>,
    pub server: &'a Server,
    pub subscribe_channels: Vec<(&'a String, SimplifiedOperation)>,
    pub publish_channels: Vec<(&'a String, SimplifiedOperation)>,
    pub model: Model,
}

impl<'a> From<&TemplateContext<'a>> for gtmpl::Value {
    fn from(value: &TemplateContext<'a>) -> Self {
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
