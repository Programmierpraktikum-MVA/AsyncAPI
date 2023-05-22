mod asyncapi_model;
mod generator;
mod parser;
mod template_model;
mod utils;

use generator::{cargo_add, cargo_fmt, cargo_init_project};

use std::{
    fs::{self, create_dir_all},
    path::Path,
};

fn main() {
    let specfile_path = Path::new("./example/specs/basic.yaml");
    let spec = parser::parse_asyncapi_yaml_file(specfile_path).unwrap();

    let title = Path::new(&spec.info.title);
    let output_path =
        &Path::new("./output/").join(title.to_str().unwrap().replace(' ', "_").to_lowercase());
    //println!("{:?}", spec);

    let template_path_main = Path::new("./templates/main.rs");
    let template_path_handler = Path::new("./templates/handler.rs");
    let template_main = fs::read_to_string(template_path_main).expect("file could not be read");
    let template_handler =
        fs::read_to_string(template_path_handler).expect("file could not be read");
    let async_config = parser::spec_to_pubsub_template_type(&spec).unwrap();

    //println!("{:#?}", async_config.server);

    create_dir_all(output_path.join("src")).unwrap();

    let template_result_main =
        gtmpl::template(&template_main, &async_config).expect("Could not inject template");
    let template_result_handler =
        gtmpl::template(&template_handler, &async_config).expect("Could not inject template");

    utils::write_to_file(&template_result_main, &output_path.join("src/main.rs")).unwrap();
    utils::write_to_file(
        &template_result_handler,
        &output_path.join("src/handler.rs"),
    )
    .unwrap();

    cargo_init_project(output_path);
    cargo_fmt(&output_path.join("src/main.rs"));
    cargo_add(output_path, "tokio", Some("rt-multi-thread")); // when there are more crates move to generator.rs
    cargo_add(output_path, "async_nats", None);
    cargo_add(output_path, "futures", None);
    cargo_add(output_path, "serde", None);
}
