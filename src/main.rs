mod generator;
mod model;
mod parser;

use generator::{cargo_add, cargo_fmt, cargo_init_project, write_to_file};
use std::{
    fs::{self, create_dir_all},
    path::Path,
};

fn main() {
    let specfile_path = Path::new("./specs/basic.yaml");
    let spec = parser::parse_asyncapi_yaml_file(specfile_path).unwrap();

    let title = Path::new(&spec.info.title);
    println!("{:?}", spec);
    let output_path = &Path::new("./output/").join(title);

    // let subscribe_channels = spec.get_subscribe_channels();
    // let publish_channels = spec.get_publish_channels();

    // parse correct templates with config as context with gtmpl
    let template_path = Path::new("./templates/pubsub.tmpl");
    let template = fs::read_to_string(template_path).expect("file could not be read");
    let async_config = parser::spec_to_pubsub_template_type(spec).unwrap();
    // println!("async_config: {:?}", async_config);

    println!("Template: {:?}", template);

    let template_result =
        gtmpl::template(&template, async_config).expect("Could not inject template");

    create_dir_all(output_path.join("src")).unwrap();
    write_to_file(&template_result, &output_path.join("src/main.rs")).unwrap();

    cargo_init_project(output_path);
    cargo_fmt(&output_path.join("src/main.rs"));
    cargo_add(output_path, "tokio", Some("rt-multi-thread")); // when there are more crates move to generator.rs
    cargo_add(output_path, "async_nats", None);
    cargo_add(output_path, "futures", None);
}
