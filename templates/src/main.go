mod handler;
mod model;
use async_nats::{Client, Message, Subscriber};
use async_nats::jetstream::{self, Context, stream};
use async_nats::jetstream::consumer::{pull::{self, Config}, Consumer};
use std::{time::Duration, env, collections::HashMap};
use futures::StreamExt;
use dotenv::dotenv;

use crate::handler::*;

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
    {{ if (index . 1).original_operation.bindings }}
        {{if (index . 1).original_operation.bindings.nats}}
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
        {{end}}
    {{end}}
{{end}} //supports only one stream, otherwise no unique function name(one function is enough for any number of streams, but you have to check if there is one)


{{range .publish_channels}}
    {{ if (index . 1).original_operation.bindings }}
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
{{end}}

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
