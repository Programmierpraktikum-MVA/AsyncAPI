use async_nats::{Client, Subscriber};
use futures::StreamExt;

async fn listen_for_message(sub1: &mut Subscriber) {
    while let Some(message) = sub1.next().await {
        println!("Received message {:#?}", message);
    }
}

async fn publish_messages(client: &Client, amount: i32, channel: &str, data: &'static str) {
    for _ in 0..amount {
        client.publish(channel.into(), data.into()).await.unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("{{ .server.url }}").await?;
    // declare subscribers

    // subscribe
    tokio::select! {
    {{range  .subscribe_channels}}
        _=listen_for_message(&mut {{  (index . 1).operationId }}) => {}
    {{end}}
    }
    println!("fin");

    Ok(())
}
