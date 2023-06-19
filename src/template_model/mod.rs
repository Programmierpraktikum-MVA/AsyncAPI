mod simplified_operation;
mod template_context;
pub use simplified_operation::{
    simplify_message, simplify_operation, simplify_schema, MultiStructEnum, SimplifiedMessage,
    SimplifiedOperation, SimplifiedSchema,
};
pub use template_context::{Model, TemplateContext};
