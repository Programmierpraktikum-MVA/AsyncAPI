use async_nats::{Client, Message, jetstream};
use async_nats::jetstream::Context;
use crate::{publish_message, utils::stream_publish_message, model::*};
use std::time;
use opentelemetry::global;
use opentelemetry::trace::Tracer;

{{ range .publish_channels  }}
    {{ $isStream := false }}

    /// This handler is called when a message is received on channel {{ (index . 1).unique_id }}
    /// Channel messages:
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    {{if key_exists (index . 1) "original_operation" "bindings" "nats" "streamname"}}
        {{ $isStream := ((index . 1).original_operation.bindings.nats.streamname) }}
    {{end}}
    {{if $isStream}}
        pub fn stream_handler_{{ (index . 1).unique_id }}(message: jetstream::Message) {
            let tracer = global::tracer("stream_handler_{{ (index . 1).unique_id }}");
            let _span = tracer.start("{{ (index . 1).unique_id }}_stream_handler");
            {{ range (index . 1).messages }}
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
        pub fn handler_{{ (index . 1).unique_id }}(message: Message) {
            let tracer = global::tracer("handler_{{ (index . 1).unique_id }}");
            let _span = tracer.start("{{ (index . 1).unique_id }}_handler");
            {{ range (index . 1).messages }}
                {{ if .payload}}
                match serde_json::from_slice::<{{ .payload.struct_reference }}>(&message.payload.as_ref()) {
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
    {{end}}
{{ end  }}


{{ range .subscribe_channels }}
    {{ $isStream := false }}

    /// Publish a message in the {{ (index . 1).unique_id }} channel
    /// Channel messages:
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    {{ if key_exists (index . 1) "original_operation" "bindings" "nats" "streamname" }}
        {{ $isStream := (index . 1).original_operation.bindings.nats.streamname }}
    {{ end }}
            
    {{ if $isStream }}
    pub async fn stream_producer_{{ (index . 1).unique_id }}(context_stream: &Context, channel: &str) { //context instead of client
        let tracer = global::tracer("{{ (index . 1).unique_id }}_stream_producer");
        let _span = tracer.start("stream_producer_{{ (index . 1).unique_id }}");
        // This is just an example producer, publishing a message every 2 seconds
        // TODO: replace this with your own producer code
        loop {
            tokio::time::sleep(time::Duration::from_secs(2)).await;
            stream_publish_message(context_stream, channel, "{\"test\":\"serialized\"}").await;
        }
    }
    {{ else }}
        pub async fn producer_{{ (index . 1).unique_id }}(client: &Client, channel: &str) {
            let tracer = global::tracer("{{ (index . 1).unique_id }}_producer");
            let _span = tracer.start("producer_{{ (index . 1).unique_id }}");
            // This is just an example producer, publishing a message every 2 seconds
            // TODO: replace this with your own producer code
            loop {
                tokio::time::sleep(time::Duration::from_secs(2)).await;
                publish_message(client, channel, "{\"test\":\"serialized\"}").await;
            }
        }
    {{ end }}
{{ end  }}
