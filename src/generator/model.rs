use super::embedded_template_render_write;
use crate::template_context::TemplateContext;
use crate::{parser::common::validate_identifier_string, utils::write_to_path_create_dir};

use std::path::Path;

pub fn generate_models_folder(async_config: &TemplateContext, output_path: &Path) {
    async_config
        .model
        .message_models
        .iter()
        .for_each(|message_model| {
            if !message_model.model_definition.is_empty() {
                embedded_template_render_write(
                    "src/model.go",
                    message_model.clone(),
                    &output_path.join(format!(
                        "src/model/{}.rs",
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

    write_to_path_create_dir(&imports, &output_path.join("src/model/mod.rs")).unwrap();
}
