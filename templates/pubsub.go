use futures::StreamExt;
use serde::{Deserialize, Serialize};


{{ .schema }}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("{{ .server_url }}").await?;
    let mut subscriber = client.subscribe("{{ .channel_name }}".into()).await?.take(10);

    for _ in 0..10 {
        client.publish("{{ .channel_name }}".into(), "Hi mom".into()).await?;
    }

    while let Some(message) = subscriber.next().await {
      println!("Received message {:?}", message);
    }

    Ok(())
}