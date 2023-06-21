mod handler;
mod model;


use std::time::Duration;
use futures::StreamExt;
use async_nats::jetstream::{self, Context, stream};
use async_nats::jetstream::consumer::{pull::{self, Config}, Consumer};
use futures::stream::Take;

use crate::handler::*;

pub struct Producer {}

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

///heavily modified, because jetstream isn't really async
async fn listen_for_message(sub: &Consumer<Config>, handler: impl Fn(jetstream::Message)) -> Result<(), async_nats::Error>{
    loop{
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let mut messages = sub.messages().await?.take(10);
        while let Some(message) = messages.next().await {
            let message = message?;
            handler(message);
            println!("Message received by Subscriber: {:?}", sub); // if you show sub its a mess, is now a Context
        }
    }
    Ok(())
}

async fn publish_message(client: &Context, channel: &str, payload: &str) {
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

    let context_jetstream = jetstream::new(client);

	//as I am not sure how this will be named, or where it will be defined, supposed to be number of streams
	{{range .publish_channels}}
	    let {{ (index . 1).unique_id }} = "{{ index . 0}}"; // {{ (index . 1).unique_id }} should contain the streams name
		//TODO: Needs unique name:
        let consumer = get_consumer(& context_jetstream, & {{ (index . 1).unique_id }}).await?;
	{{end}}

    test(&context_jetstream, "{{ index . 0}}").await;

    // this is terrible, cause no error handling in listen for message.
    // as of now i don't really have a solution, since it is in a join block 
    tokio::join!(
		{{ range .subscribe_channels }}
            producer_{{ (index . 1).unique_id }}(&context_jetstream, "{{ index . 0}}"),
		{{end}}
		{{ range .publish_channels  }}
		//TODO: Needs unique consumer name
            listen_for_message(&consumer, handler_usersignedup),
		{{  end  }}
    );


    println!("fin");
    Ok(())
}
