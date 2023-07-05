mod common;
mod model;
pub use common::{
    cargo_fix, cargo_fmt, cargo_generate_rustdoc, cargo_init_project, check_for_overwrite,
    template_render_write, write_multiple_templates,
};
mod template_functions;
pub use model::generate_models_folder;
