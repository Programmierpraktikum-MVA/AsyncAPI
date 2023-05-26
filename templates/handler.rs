use std::time;
use async_nats::{Client, Message};
use crate::publish_message;

{{ .schema }}

 {{ range .subscribe_channels  }}
    pub fn handler_{{ (index . 1).operationId }}(message: Message) {
        println!("Received message {:#?}", message)
    }
{{ end  }}

{{ range .publish_channels }}
    pub async fn producer_{{ (index . 1).operationId }}(client: &Client, channel: &str) {
        loop {
            tokio::time::sleep(time::Duration::from_secs(2)).await;
            publish_message(client, channel, "test").await;
        }
    }
{{ end  }}


pub async fn test(client: &Client, channel: &str) {
    publish_message(client, channel, "from test hello").await;
}
