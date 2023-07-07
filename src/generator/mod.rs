mod common;
pub use common::{check_for_overwrite, render_write_all_embedded_templates};

mod model;
pub use model::generate_models_folder;

mod template_functions;
