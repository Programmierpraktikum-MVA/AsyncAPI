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
    let specfile_path = Path::new("./specs/userSignupSubscriber.yaml");
    let spec = parser::parse_asyncapi_yaml_file(specfile_path).unwrap();

    let title = Path::new(&spec.info.title);
    let output_path =
        &Path::new("./output/").join(title.to_str().unwrap().replace(' ', "_").to_lowercase());
    println!("{:?}", spec);

    let template_path = Path::new("./templates/pubsub.tmpl");
    let template = fs::read_to_string(template_path).expect("file could not be read");
    let async_config = parser::spec_to_pubsub_template_type(spec).unwrap();

    let template_result =
        gtmpl::template(&template, async_config).expect("Could not inject template");

    create_dir_all(output_path.join("src")).unwrap();
    utils::write_to_file(&template_result, &output_path.join("src/main.rs")).unwrap();

    cargo_init_project(output_path);
    cargo_fmt(&output_path.join("src/main.rs"));
    cargo_add(output_path, "tokio", Some("rt-multi-thread")); // when there are more crates move to generator.rs
    cargo_add(output_path, "async_nats", None);
    cargo_add(output_path, "futures", None);
    cargo_add(output_path, "serde", None);
}
