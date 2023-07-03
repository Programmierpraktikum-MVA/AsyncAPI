mod handler;
mod model;
mod utils;
mod cli; 
use clap::Parser;
use crate::cli::*;
use utils::*;
use crate::handler::*;
use async_nats::jetstream::{self};
use std::{collections::HashMap};
use dotenv::dotenv;
mod config;


#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let env: HashMap<String,String> = config::get_env();
    let args = cli::Args::parse();

    let client = async_nats::connect(env.get("SERVER_URL").unwrap()).await?;
    handle_cli(&client, &args.command, &args.message).await?;

    {{ range .publish_channels }}
        {{ if (index . 1).original_operation.bindings }}
                {{ if (index . 1).original_operation.bindings.nats.queue }}
                    let mut {{ (index . 1).unique_id }} = client.queue_subscribe(env.get("{{ (index . 1).unique_id}}_SUBJECT").unwrap().into(),
                     env.get("{{ (index . 1).unique_id}}_QUEUE").unwrap().into()).await?;
                {{ else  }}
                    let clientcpy = client.clone();
                    let context_jetstream = jetstream::new(clientcpy);
                    let {{ (index . 1).unique_id }} = env.get("{{ (index . 1).unique_id }}_STREAM").unwrap();
                    let consumer = get_consumer(&context_jetstream, &{{ (index . 1).unique_id }}).await?;
                {{end}}
        {{ else }}
            let mut {{ (index . 1).unique_id }} = client.subscribe(env.get("{{ (index . 1).unique_id}}_SUBJECT").unwrap().into()).await?;
        {{end}}
    {{end}}


    tokio::join!(
    {{ range .subscribe_channels }}
        {{ $isStream := false }}
        {{if key_exists (index . 1) "original_operation" "bindings" "nats" "streamname"}}
            {{ $isStream := ((index . 1).original_operation.bindings.nats.streamname) }}
        {{end}}
    {{ end }}
    {{ range .publish_channels  }}
        {{ $isStream := false }}
        {{if key_exists (index . 1) "original_operation" "bindings" "nats" "streamname"}}
            {{ $isStream := ((index . 1).original_operation.bindings.nats.streamname) }}
        {{end}}
        {{if $isStream}}
            stream_listen_for_message(&consumer, stream_handler_{{ (index . 1).unique_id }}, &client),
        {{ else }}
            listen_for_message(&mut  {{ (index . 1).unique_id }}, handler_{{ (index . 1).unique_id }}, &client),
        {{ end }}
    {{ end }}
    );

    println!("fin");
    Ok(())
}
