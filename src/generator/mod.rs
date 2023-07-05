mod common;
pub use common::{
    check_for_overwrite, embedded_template_render_write, template_render_write,
    write_multiple_embedded_templates,
};

mod model;
pub use model::generate_models_folder;

mod template_functions;
