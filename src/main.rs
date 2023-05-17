mod generator;
mod model;
mod parser;

use generator::{cargo_add, cargo_fmt, cargo_init_project, write_to_file};
use std::{fs::create_dir_all, path::Path};


fn main() {
    let specfile_path = Path::new("./specs/basic.yaml");
    let spec = parser::parse_asyncapi_yaml_file(specfile_path).unwrap();

    let title = Path::new(&spec.info.title);
    println!("{:?}", spec);
    // let output_path = &Path::new("./output/").join(title);

    // let subscribe_channels = spec.get_subscribe_channels();
    // let publish_channels = spec.get_publish_channels();

    // // generate
    // let publish = PublishTemplate {
    //     server: spec.servers.iter().next().unwrap().1,
    //     publish_data: "asdf",
    //     subscribe_channels: &subscribe_channels,
    //     publish_channels: &publish_channels,
    // };
    // let render = publish.render().unwrap();

    // create_dir_all(output_path.join("src")).unwrap();
    // write_to_file(&render, &output_path.join("src/main.rs")).unwrap();

    // cargo_init_project(output_path);
    // cargo_fmt(&output_path.join("src/main.rs"));
    // cargo_add(output_path, "tokio"); // when there are more crates move to generator.rs
}
