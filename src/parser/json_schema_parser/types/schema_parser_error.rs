use core::fmt;

#[derive(Debug, Clone)]
pub enum SchemaParserError {
    // error message, property name
    GenericError(String, Option<String>),
}
impl fmt::Display for SchemaParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchemaParserError::GenericError(msg, property_name) => {
                let base_message = match property_name {
                    Some(name) => format!(
                        "Error while parsing schema, inside property:\"{}\";\n Message: {} ",
                        msg, name
                    ),
                    None => "Error while parsing schema".to_string() + &msg.to_string(),
                };
                write!(f, "{}", base_message)
            }
        }
    }
}
impl std::error::Error for SchemaParserError {}
