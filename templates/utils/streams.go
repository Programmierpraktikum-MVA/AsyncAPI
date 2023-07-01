use async_nats::jetstream::{self, Context};
use async_nats::jetstream::consumer::{pull::{Config}, Consumer};
use std::time::Duration;
use futures::StreamExt;

pub async fn stream_publish_message(client: &Context, channel: &str, payload: &str) {
	let owned_payload = payload.to_owned().into(); // Convert to Bytes
	client
		.publish(channel.to_string(), owned_payload)
		.await
		.unwrap();
	println!("sent");
}


pub async fn stream_listen_for_message(sub: &Consumer<Config>, handler: impl Fn(jetstream::Message)) -> Result<(), async_nats::Error>{
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