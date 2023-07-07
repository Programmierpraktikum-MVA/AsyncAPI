
	use async_nats::{Client, Message, jetstream};
	use async_nats::jetstream::Context;
	use crate::{publish_message,stream_publish_message,model::*,config::*};
	use std::time;

    {{ $isStream := false }}

    /// This handler is called when a message is received on channel {{ .unique_id }}
    /// Channel messages:
    /// {{ range .messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    {{if key_exists  "original_operation" "bindings" "nats" "streamname"}}
        {{ $isStream := (.original_operation.bindings.nats.streamname) }}
    {{end}}
    {{if $isStream}}
        pub fn stream_handler_{{ .unique_id }}(message: jetstream::Message, client: &Client) {
            {{ range .messages }}
                {{ if .payload}}
                    match serde_json::from_slice::<{{ .payload.struct_reference }}>(&message.message.payload.as_ref()) {
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
    {{else}}
        pub async fn handler_{{ .unique_id }}(message: Message, client: &Client) {
            {{ range .messages }}
                {{ if .payload}}
                match serde_json::from_slice::<{{ .payload.struct_reference }}>(&message.payload.as_ref()) {
                    Ok(deserialized_message) => {
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
                        {{else}}
                            println!("Received message {:#?}", deserialized_message);
                            // TODO: Replace this with your own handler code
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
    {{end}}
