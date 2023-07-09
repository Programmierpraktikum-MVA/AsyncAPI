use async_nats::{Client, Message, Subscriber};
use futures::StreamExt;
use log::debug;

pub async fn listen_for_message<'a, F, Fut>(sub: &mut Subscriber, handler: F, client: &'a Client)
where
    F: Fn(Message,&'a Client) -> Fut + 'a,
    Fut: std::future::Future<Output = ()> + 'a,
{
    while let Some(message) = sub.next().await {
        handler(message, client).await;
        debug!("Message received by Subscriber: {:?}", sub);
    }
}

pub async fn publish_message(client: &Client, channel: &str, payload: &str) {
    let owned_payload = payload.to_owned().into(); // Convert to Bytes
    client
        .publish(channel.to_string(), owned_payload)
        .await
        .unwrap();
    debug!("Published message to channel: {}", channel);
}

