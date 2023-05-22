use crate::asyncapi_model::{Operation, Server};

use gtmpl::Value;
use serde::Serialize;

use serde_json::Value as JsonValue;
use std::string::*;

// macro_rules! impl_gtmpl_value {
//     ($struct_type:ty) => {
//         impl gtmpl::Value for $struct_type {
//             fn as_value(&self) -> gtmpl::Value {
//                 let json_value = serde_json::to_value(self).unwrap();
//                 serde_value_to_gtmpl_value(&json_value)
//             }
//         }
//     };
// }
#[derive(Serialize, Debug)]
pub struct PubsubTemplate<'a> {
    pub server: &'a Server,
    pub subscribe_channels: Vec<(&'a String, &'a Operation)>,
    pub publish_channels: Vec<(&'a String, &'a Operation)>,
}
// #[derive(Serialize, Debug)]
// pub struct PubsubTemplate<'a> {
//     pub server: &'a Server,
//     pub subscribe_channels: Vec<Channel<'a>>,
//     pub publish_channels: Vec<Channel<'a>>,
// }

// #[derive(Serialize, Debug)]
// pub struct Channel<'a> {
//     pub channel_name: &'a String,
//     pub operation: &'a Operation,
// }

impl<'a> From<&PubsubTemplate<'a>> for gtmpl::Value {
    fn from(value: &PubsubTemplate<'a>) -> Self {
        let json_value: serde_json::Value = serde_json::to_value(value).unwrap();
        serde_value_to_gtmpl_value(&json_value)
    }
}

/// converts any serde serializable value to a gtmpl value
/// WARNING: clones objects, so not exactly zero cost abstraction 🤷‍♂️
pub fn serde_value_to_gtmpl_value(value: &serde_json::Value) -> gtmpl::Value {
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
