mod handler;
mod model;
mod utils;
use utils::*;
use crate::handler::*;
use async_nats::jetstream::{self};
use std::{env, collections::HashMap};
use dotenv::dotenv;
use opentelemetry::global;
use opentelemetry::trace::Tracer;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {

    dotenv().ok();
    let env: HashMap<String,String> = env::vars().collect();

    // Initialize Jaeger Tracer
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
    .with_service_name("my_service_name")
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Failed to initialize Jaeger Tracer");

    let client = async_nats::connect(env.get("SERVER_URL").unwrap()).await?;

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
        {{if $isStream}}
            stream_producer_{{ (index . 1).unique_id }}(&context_jetstream, env.get("{{ (index . 1).unique_id}}_STREAM").unwrap()),
        {{ else }}
            producer_{{ (index . 1).unique_id }}(&client, env.get("{{ (index . 1).unique_id}}_SUBJECT").unwrap() ),
        {{ end  }}
    {{ end }}
    {{ range .publish_channels  }}
        {{ $isStream := false }}
        {{if key_exists (index . 1) "original_operation" "bindings" "nats" "streamname"}}
            {{ $isStream := ((index . 1).original_operation.bindings.nats.streamname) }}
        {{end}}
        {{if $isStream}}
            stream_listen_for_message(&consumer, stream_handler_{{ (index . 1).unique_id }}),
        {{ else }}
            listen_for_message(&mut  {{ (index . 1).unique_id }}, handler_{{ (index . 1).unique_id }}),
        {{ end }}
    {{ end }}
    );
    opentelemetry::global::shutdown_tracer_provider();
    println!("fin");
    Ok(())
}
