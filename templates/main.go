mod handler;
mod model;
mod utils;
use utils::*;
use crate::handler::*;
use async_nats::jetstream::{self};

pub struct Producer {}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("{{ .server.url }}").await?;

    {{ range .publish_channels }}
        {{ if (index . 1).original_operation.bindings }}
                {{ if (index . 1).original_operation.bindings.nats.queue }}
                    let mut {{ (index . 1).unique_id }} = client.queue_subscribe("{{ index . 0  }}".into(), "{{ (index . 1).original_operation.bindings.nats.queue }}".into()).await?;
                {{ else  }}
                    let clientcpy = client.clone();
                    let context_jetstream = jetstream::new(clientcpy);
                    let {{ (index . 1).unique_id }} = "{{ (index . 1).original_operation.bindings.nats.streamname }}";
                    let consumer = get_consumer(&context_jetstream, &{{ (index . 1).unique_id }}).await?;
                {{end}}
        {{ else }}
            let mut {{ (index . 1).unique_id }} = client.subscribe("{{ index . 0  }}".into()).await?;
        {{end}}
    {{end}}

    //test(&client, "foo").await;

    tokio::join!(
    {{ range .subscribe_channels }}
        {{ if (index . 1).original_operation.bindings }}
            {{if (index . 1).original_operation.bindings.nats.streamname}}
                stream_producer_{{ (index . 1).unique_id }}(&context_jetstream, "{{ (index . 1).original_operation.bindings.nats.streamname }}"),
            {{ else }}
                producer_{{ (index . 1).unique_id }}(&client, "{{ index . 0  }}"),
            {{ end }}
        {{ else }}
            producer_{{ (index . 1).unique_id }}(&client, "{{ index . 0  }}"),
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
