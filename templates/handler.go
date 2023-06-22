use async_nats::{Client, Message};
use crate::{publish_message,
{{ range .model.message_models }}
    {{ if ne .model_definition "" }}
        model::{{ .unique_id }},
    {{ end }}
{{ end }}
};
use std::time;


{{ range .publish_channels  }}
    /// This handler is called when a message is received on channel {{ (index . 1).unique_id }}
    /// Channel messages:
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    pub fn handler_{{ (index . 1).unique_id }}(message: Message) {
        {{ range (index . 1).messages }}
        {{ if .payload}}
        match serde_json::from_slice::<{{ .payload.struct_reference }}>(message.payload.as_ref()) {
            Ok(deserialized_message) => {
                println!("Received message {:#?}", deserialized_message);
                // TODO: Replace this with your own handler code
                {{ if eq .payload.model_type "enum"}}
                    match deserialized_message {
                        {{$enumName := .payload.unique_id}}
                        {{ range .payload.related_models }}
                            {{ $enumName }}::{{ .unique_id }}(payload) => {
                            // TODO: Replace this with your own handler code
                            println!("Received message payload {{ .unique_id }} {:?}", payload);
                            }   
                        {{ end }}
                    }
                {{ end }}
            },
            Err(_) => {
                println!("Failed to deserialize message payload: {{ .unique_id }}\nOriginal message: {:#?}", message);
                // TODO: Handle the failed deserialization here
            },
        }
        {{ end }}
        {{ end }}
    }
{{ end  }}

{{ range .subscribe_channels }}
    /// Publish a message in the {{ (index . 1).unique_id }} channel
    /// Channel messages:
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    pub async fn producer_{{ (index . 1).unique_id }}(client: &Client, channel: &str) {
        // This is just an example producer, publishing a message every 2 seconds
        // TODO: replace this with your own producer code
        loop {
            tokio::time::sleep(time::Duration::from_secs(2)).await;
            publish_message(client, channel, "{\"test\":\"serialized\"}").await;
        }
    }
{{ end  }}


pub async fn test(client: &Client, channel: &str) {
    publish_message(client, channel, "from test hello").await;
}
