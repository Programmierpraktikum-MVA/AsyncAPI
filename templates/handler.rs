use std::time;
use async_nats::{Client, Message};
use crate::publish_message;
use serde::{Deserialize, Serialize};



 {{ range .subscribe_channels  }}
    {{ range (index . 1).messages }}
        {{ if .payload }}
         // payload
            {{ .payload.struct_definition }}
        {{ end }}
    {{ end }}
    /// this handler is called when a message is received on channel {{ (index . 1).unique_id }}
    /// messages: 
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    pub fn handler_{{ (index . 1).unique_id }}(message: Message) {
        println!("Received message {:#?}", message)
    }
{{ end  }}

{{ range .publish_channels }}
    /// publish message to {{ (index . 1).unique_id }}
    ///  messages: 
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    pub async fn producer_{{ (index . 1).unique_id }}(client: &Client, channel: &str) {
        loop {
            tokio::time::sleep(time::Duration::from_secs(2)).await;
            publish_message(client, channel, "test").await;
        }
    }
{{ end  }}


pub async fn test(client: &Client, channel: &str) {
    publish_message(client, channel, "from test hello").await;
}
