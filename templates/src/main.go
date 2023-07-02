mod handler;
mod model;
mod utils;
use utils::*;
use crate::handler::*;
use async_nats::jetstream::{self};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {

    dotenv().ok();
    let env: HashMap<String,String> = env::vars().collect();

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
        {{ if (index . 1).original_operation.bindings }}
            {{if (index . 1).original_operation.bindings.nats.streamname}}
    stream_producer_{{ (index . 1).unique_id }}(&context_jetstream, env.get("{{ (index . 1).unique_id}}_STREAM").unwrap()),
            {{ end }}
        {{ else }}
    producer_{{ (index . 1).unique_id }}(&client, env.get("{{ (index . 1).unique_id}}_SUBJECT").unwrap() ),
        {{ end }}
    {{ end  }}
    {{ range .publish_channels  }}
        {{ if (index . 1).original_operation.bindings }}
            {{if (index . 1).original_operation.bindings.nats.streamname}}
    stream_listen_for_message(&consumer, stream_handler_{{ (index . 1).unique_id }}),

            {{ else }}
    listen_for_message(&mut  {{ (index . 1).unique_id }}, handler_{{ (index . 1).unique_id }}),
            {{ end }}

        {{ else }}
    listen_for_message(&mut  {{ (index . 1).unique_id }}, handler_{{ (index . 1).unique_id }}),
        {{ end }}
    {{ end }}
    );

    println!("fin");
    Ok(())
}
