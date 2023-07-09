use super::common::render_write_template;
use crate::template_context::TemplateContext;
use crate::{parser::common::validate_identifier_string, utils::write_to_path_create_dir};
use std::path::Path;

pub fn generate_models_folder(
    template: impl Into<String> + Clone,
    async_config: &TemplateContext,
    output_dir: &Path,
) {
    async_config
        .model
        .message_models
        .iter()
        .for_each(|message_model| {
            if !message_model.model_definition.is_empty() {
                render_write_template(
                    template.clone(),
                    message_model.clone(),
                    &output_dir.join(format!(
                        "{}.rs",
                        validate_identifier_string(&message_model.unique_id, false)
                    )),
                );
            }
        });

    let imports = async_config
        .model
        .message_models
        .iter()
        .map(|message_model| {
            if !message_model.model_definition.is_empty() {
                format!(
                    "pub mod {}; \n pub use {}::*; \n",
                    validate_identifier_string(&message_model.unique_id, false),
                    validate_identifier_string(&message_model.unique_id, false)
                )
            } else {
                "".to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    write_to_path_create_dir(&imports, &output_dir.join("mod.rs")).unwrap();
}
