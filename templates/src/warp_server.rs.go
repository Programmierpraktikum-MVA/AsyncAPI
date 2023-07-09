use cargo_metadata::MetadataCommand;
use warp::Filter;
use serde_json;

use crate::config::get_env;

async fn life() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("alive\n"))
}

async fn ready_func() -> Result<impl warp::Reply, warp::Rejection> {
    //TODO: actually check if service is ready
    Ok(format!("ready\n"))
}

async fn metamessage() -> Result<impl warp::Reply, warp::Rejection>{
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let meta = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(&path)
        .exec()
        .unwrap();

    let root = meta.root_package().unwrap();
    let root = serde_json::to_string(&root).unwrap();
    Ok(root)
}

pub async fn server() {

    let metadata = warp::get()
        .and(warp::path("root"))
        .and(warp::path::end())
        .and_then(metamessage);


    let liveness = warp::get()
        .and(warp::path("healthz"))
        .and(warp::path::end())
        .and_then(life);

    let readiness = warp::get()
        .and(warp::path("readyz"))
        .and(warp::path::end())
        .and_then(ready_func);

    let port = match get_env("SERVICE_PORT") {
        Some(port) => match port.parse::<u16>() {
            Ok(port) => port,
            Err(_) => panic!("SERVICE_PORT is not a valid port number"),
        }
        None => panic!("SERVICE_PORT not set"),
    };

    warp::serve(liveness.or(readiness).or(metadata))
        .run(([127, 0, 0, 1], port))
        .await;
}