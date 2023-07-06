mod handler;
mod model;
use async_nats::{Client, Message, Subscriber};

use futures::StreamExt;

use crate::handler::*;

pub struct Producer {}

async fn listen_for_message(sub: &mut Subscriber, handler: impl Fn(Message)) {
    while let Some(message) = sub.next().await {
        handler(message);
        println!("Message received by Subscriber: {:?}", sub); 
    }
}
async fn publish_message(client: &Client, channel: &str, payload: &str) {
    let owned_payload = payload.to_owned().into(); // Convert to Bytes
    client
        .publish(channel.to_string(), owned_payload)
        .await
        .unwrap();
    println!("sent");
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("{{ .server.url }}").await?;

    {{ range .publish_channels }}
    {{ if (index . 1).original_operation.bindings }}
    let mut {{ (index . 1).unique_id }} = client.queue_subscribe("{{ index . 0  }}".into(), "{{ (index . 1).original_operation.bindings.nats.queue }}".into()).await?;
       
    {{ else }}
    let mut {{ (index . 1).unique_id }} = client.subscribe("{{ index . 0  }}".into()).await?;
    {{end}}
    {{end}}

    test(&client, "foo").await;

    tokio::join!(
    {{ range .subscribe_channels }}
        producer_{{ (index . 1).unique_id }}(&client, "{{ index . 0  }}"),
    {{ end  }}
    {{ range .publish_channels  }}
        listen_for_message(&mut  {{ (index . 1).unique_id }}, handler_{{ (index . 1).unique_id }}),
    {{  end  }}
    );

    println!("fin");
    Ok(())
}
