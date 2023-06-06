mod handler;
use async_nats::{Client, Message, Subscriber};

use futures::StreamExt;

use crate::handler::*;

pub struct Producer {}

async fn listen_for_message(sub: &mut Subscriber, handler: impl Fn(Message)) {
    while let Some(message) = sub.next().await {
        handler(message);
    }
}
async fn publish_message(client: &Client, channel: &str, payload: &'static str) {
    client
        .publish(channel.into(), payload.into())
        .await
        .unwrap();
    println!("sent");
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("{{ .server.url }}").await?;

    {{ range .subscribe_channels }}
        let mut {{ (index . 1).unique_id }} = client.subscribe("{{ index . 0  }}".into()).await?;
    {{end}}

    test(&client, "foo").await;

    tokio::join!(
    {{ range .publish_channels }}
        producer_{{ (index . 1).unique_id }}(&client, "{{ index . 0  }}"),
    {{ end  }}
    {{ range .subscribe_channels  }}
        listen_for_message(&mut  {{ (index . 1).unique_id }}, handler_{{ (index . 1).unique_id }}),
    {{  end  }}
    );

    println!("fin");
    Ok(())
}
