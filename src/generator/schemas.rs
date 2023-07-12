use super::common::render_write_template;
use crate::parser::common::validate_identifier_string;
use crate::template_context::TemplateContext;
use std::path::Path;

pub fn generate_schemas_folder(
    template: impl Into<String> + Clone,
    async_config: &TemplateContext,
    output_dir: &Path,
) {
    async_config
        .publish_channels
        .iter()
        .chain(async_config.subscribe_channels.iter())
        .for_each(|(_key, operation)| {
            let message = operation.messages.first();
            if message.is_none() {
                return;
            }
            let message = message.unwrap();
            if message.payload_schema.is_none() {
                return;
            }
            render_write_template(
                template.clone(),
                &message.clone(),
                &output_dir.join(format!(
                    "{}_payload_schema.json",
                    validate_identifier_string(&message.unique_id, false)
                )),
            );
        });
}
