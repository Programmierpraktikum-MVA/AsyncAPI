use async_nats::{Client, Message};
use crate::{publish_message,
{{ range .model.messages }}
    {{ if .payload }}
        model::{{ .payload.unique_id }},
    {{ end }}
{{ end }}

{{ range .model.enums }}
    model::{{ .unique_id }},
{{ end }}


};
use std::time;


{{ range .publish_channels  }}
    /// This handler is called when a message is received on channel {{ (index . 1).unique_id }}
    /// Accepted messages:
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    pub fn handler_{{ (index . 1).unique_id }}(message: Message) {
        {{ if (index . 1).multiple_messages_enum }}
            match serde_json::from_slice::<{{ (index . 1).multiple_messages_enum.unique_id }}>(&message.payload.as_ref()) {
                Ok(deserialized_message) => {
                    println!("Received message {:#?}", deserialized_message);
                    // Your handler code goes here
                },
                Err(_) => {
                    println!("Failed to deserialize message payload: {{ (index . 1).multiple_messages_enum.unique_id }}");
                    // Handle the failed deserialization here
                },
            }
        {{else}}
            
        {{ end }}
    }
{{ end  }}

{{ range .subscribe_channels }}
    /// publish message to {{ (index . 1).unique_id }}
    ///  messages: 
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    pub async fn producer_{{ (index . 1).unique_id }}(client: &Client, channel: &str) {
        loop {
            tokio::time::sleep(time::Duration::from_secs(2)).await;
            publish_message(client, channel, "{\"usersingnedup\":\"usersingnedupMessage\"}").await;
        }
    }
{{ end  }}


pub async fn test(client: &Client, channel: &str) {
    publish_message(client, channel, "from test hello").await;
}
