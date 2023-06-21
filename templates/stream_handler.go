use async_nats::{Client, Message, jetstream};
use async_nats::jetstream::Context;
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
    /// Channel messages:
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    pub fn handler_{{ (index . 1).unique_id }}(message: jetstream::Message) {
        {{ if (index . 1).multiple_messages_enum }}
            match serde_json::from_slice::<{{ (index . 1).multiple_messages_enum.unique_id }}>(&message.message.payload.as_ref()) {
                Ok(deserialized_message) => {
                    // TODO: Replace this with your own handler code
                    println!("Received message {:#?}", deserialized_message);
                },
                Err(_) => {
                    println!("Failed to deserialize message payload: {{ (index . 1).multiple_messages_enum.unique_id }}\nOriginal message: {:#?}", message);
                    // TODO: Handle the failed deserialization here
                },
            }
        {{else}}
        {{ range (index . 1).messages }}
        match serde_json::from_slice::<{{ .unique_id }}>(&message.message.payload.as_ref()) {
            Ok(deserialized_message) => {
                println!("Received message {:#?}", deserialized_message);
                // TODO: Replace this with your own handler code
                {{ if .payload}}
                {{ if .payload.multiple_payload_enum}}
                    // TODO: this is always None for now (unreachable),
                    // take a look the comment in src/template_model/simplified_operation.rs/simplify_schema
                    match deserialized_message.payload {
                        {{$enumName := .payload.multiple_payload_enum.unique_id}}
                        {{ range .payload.multiple_payload_enum.messages }}
                            {{ $enumName }}::{{ .unique_id }}(payload) => {
                            println!("Received message payload {{ .unique_id }}", payload);
                            }   
                        {{ end }}
                    }
                {{ end }}
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
    pub async fn producer_{{ (index . 1).unique_id }}(context_stream: &Context, channel: &str) { //context instead of client
        // This is just an example producer, publishing a message every 2 seconds
        // TODO: replace this with your own producer code
        loop {
            tokio::time::sleep(time::Duration::from_secs(2)).await;
            publish_message(context_stream, channel, "{\"test\":\"serialized\"}").await;
        }
    }
{{ end  }}


pub async fn test(client: &Context, channel: &str) {
    publish_message(client, channel, "from test hello").await;
}