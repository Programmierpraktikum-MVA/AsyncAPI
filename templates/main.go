mod handler;
mod model;
use async_nats::{Client, Message, Subscriber};
use async_nats::jetstream::{self, Context, stream};
use async_nats::jetstream::consumer::{pull::{self, Config}, Consumer};
use std::time::Duration;
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

pub async fn get_consumer(jetstream: &Context, stream_name: &str) -> Result<Consumer<Config>, async_nats::Error>{

    let stream = jetstream.get_or_create_stream(jetstream::stream::Config {
        name: stream_name.to_string(),
        ..Default::default()
    }).await?;
    let consumer = stream.get_or_create_consumer("consumer", Config {
        durable_name: Some("consumer".to_string()),
        ..Default::default()
    }).await?;
    return Ok(consumer);
}

{{ range .subscribe_channels }}
{{ if (index . 1).original_operation.bindings.nats.streamname }}
async fn stream_publish_message(client: &Context, channel: &str, payload: &str) {
    let owned_payload = payload.to_owned().into(); // Convert to Bytes
    client
        .publish(channel.to_string(), owned_payload)
        .await
        .unwrap();
    println!("sent");
}
{{end}}
{{end}} //supports only one stream, otherwise no unique function name(one function is enough for any number of streams, but you have to check if there is one)


{{range .publish_channels}}
{{ if (index . 1).original_operation.bindings.nats.streamname }}
async fn stream_listen_for_message(sub: &Consumer<Config>, handler: impl Fn(jetstream::Message)) -> Result<(), async_nats::Error>{
    loop{
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let mut messages = sub.messages().await?.take(10);
        while let Some(message) = messages.next().await {
            let message = message?;
            handler(message);
            println!("Message received by Subscriber: {:?}", sub.cached_info().name); // if you show sub its a mess, is now a Context
        }
    }
    Ok(())
}
{{end}}
{{end}}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("{{ .server.url }}").await?;

    {{ range .publish_channels }}
    {{ if (index . 1).original_operation.bindings.nats.queue }}
        let mut {{ (index . 1).unique_id }} = client.queue_subscribe("{{ index . 0  }}".into(), "{{ (index . 1).original_operation.bindings.nats.queue }}".into()).await?;
       
    {{ else if (index . 1).original_operation.bindings.nats.streamname }}
        let context_jetstream = jetstream::new(client);
        let {{ (index . 1).unique_id }} = "{{ (index . 1).original_operation.bindings.nats.streamname }}";
        let consumer = get_consumer(&context_jetstream, &{{ (index . 1).unique_id }}).await?;
    {{ else }}
        let mut {{ (index . 1).unique_id }} = client.subscribe("{{ index . 0  }}".into()).await?;
    {{end}}
    {{end}}

    //test(&client, "foo").await;

    tokio::join!(
    {{ range .subscribe_channels }}
        {{if (index . 1).original_operation.bindings.nats.streamname}}
        stream_producer_{{ (index . 1).unique_id }}(&context_jetstream, "{{ (index . 1).original_operation.bindings.nats.streamname }}"),
        {{ else }}
        producer_{{ (index . 1).unique_id }}(&client, "{{ index . 0  }}"),
        {{ end }}
    {{ end  }}
    {{ range .publish_channels  }}
        {{if (index . 1).original_operation.bindings.nats.streamname}}
        stream_listen_for_message(&consumer, stream_handler_{{ (index . 1).unique_id }}),
        {{ else }}
        listen_for_message(&mut  {{ (index . 1).unique_id }}, handler_{{ (index . 1).unique_id }}),
        {{end}}
    {{  end  }}
    );

    println!("fin");
    Ok(())
}
