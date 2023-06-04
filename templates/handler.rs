use std::time;
use async_nats::{Client, Message};
use crate::publish_message;
use serde::{Deserialize, Serialize};

{{ .schema }}

 {{ range .subscribe_channels  }}
    /// this handler is called when a message is received on {{ (index . 1).operationId }}
    pub fn handler_{{ (index . 1).operationId }}(message: Message) {
        println!("Received message {:#?}", message)
    }
{{ end  }}

{{ range .publish_channels }}
    /// publish message to {{ (index . 1).operationId }}
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
