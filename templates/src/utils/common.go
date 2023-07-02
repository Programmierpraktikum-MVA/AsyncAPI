
use async_nats::{Client, Message, Subscriber};
use futures::StreamExt;

pub async fn listen_for_message(sub: &mut Subscriber, handler: impl Fn(Message)) {
    while let Some(message) = sub.next().await {
        handler(message);
        println!("Message received by Subscriber: {:?}", sub); 
    }
}
pub async fn publish_message(client: &Client, channel: &str, payload: &str) {
    let owned_payload = payload.to_owned().into(); // Convert to Bytes
    client
        .publish(channel.to_string(), owned_payload)
        .await
        .unwrap();
    println!("sent");
}

