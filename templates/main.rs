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

    {{ range .subscribe_channels  }}
    {{ if (index . 1).bindings }}
        let mut {{ (index . 1).operationId }} = client.queue_subscribe("{{ index . 0  }}".into(), "{{ (index . 1).bindings.nats.queue }}".into()).await?;
    {{ else }}
        let mut {{ (index . 1).operationId }} = client.subscribe("{{ index . 0  }}".into()).await?;
    {{end}}

    {{ end  }}}

    test(&client, "foo").await;

    tokio::join!(
    {{ range .publish_channels }}
        producer_{{ (index . 1).operationId }}(&client, "{{ index . 0  }}"),
    {{ end  }}
    {{ range .subscribe_channels  }}
        listen_for_message(&mut  {{ (index . 1).operationId }}, handler_{{ (index . 1).operationId }}),
    {{  end  }}
    );

    println!("fin");
    Ok(())
}
