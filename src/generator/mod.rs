mod common;
pub use common::{check_for_overwrite, template_render_write, write_multiple_templates};

mod model;
pub use model::generate_models_folder;

mod template_functions;
