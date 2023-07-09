use cargo_metadata::MetadataCommand;
use warp::Filter;
use serde_json;

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
    //let option = root.metadata["my"]["option"].as_str().unwrap();
    //let version = &root.version;

    //println!("sent root metadata");
    let root =  serde_json::to_string(&root).unwrap();
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

    warp::serve(liveness.or(readiness).or(metadata))
        .run(([127, 0, 0, 1], 8000))
        .await;
}