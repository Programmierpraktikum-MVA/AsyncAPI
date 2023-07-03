use async_nats::{Client, Message, jetstream};
use async_nats::jetstream::Context;
use crate::{publish_message,stream_publish_message,model::*,config::*};
use std::time;



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
        pub fn stream_handler_{{ (index . 1).unique_id }}(message: jetstream::Message, client: &Client) {
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
        pub async fn handler_{{ (index . 1).unique_id }}(message: Message, client: &Client) {
            {{ range (index . 1).messages }}
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
{{ end  }}


{{ range .subscribe_channels }}
    {{ $isStream := false }}
    {{ $channel := . }}

    /// Publish a message in the {{ (index . 1).unique_id }} channel
    /// Channel messages:
    /// {{ range (index . 1).messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    {{ if key_exists (index . 1) "original_operation" "bindings" "nats" "streamname" }}
        {{ $isStream := (index . 1).original_operation.bindings.nats.streamname }}
    {{ end }}
            
    {{ if $isStream }}
        {{ range (index . 1).messages }}
            pub async fn stream_producer_{{ (index $channel 1).unique_id }}(context_stream: &Context, payload : {{ if .payload}} {{ .payload.struct_reference }} {{ else }} () {{ end }}) { //context instead of client
                let subject = get_env().get("{{ (index $channel 1).unique_id }}_SUBJECT").unwrap().clone();
                {{ if .payload }}
                    let payload = match serde_json::to_string(&payload) {
                        Ok(payload) => payload,
                        Err(_) => {
                            println!("Failed to serialize message payload: {{ .payload.struct_reference }}");
                            return;
                        }
                    };
                    stream_publish_message(context_stream, &subject, &payload).await;
                {{else}}
                stream_publish_message(context_stream, &subject, &"").await;
                {{end}}
            }
        {{end}}
    {{ else }}
        {{ range (index . 1).messages }}
            pub async fn producer_{{ (index $channel 1).unique_id }}(client: &Client, payload: {{ if .payload }} {{.payload.struct_reference}} {{else}} () {{end}}) {
                let subject = get_env().get("{{ (index $channel 1).unique_id }}_SUBJECT").unwrap().clone();
                {{ if .payload }}
                    let payload = match serde_json::to_string(&payload) {
                        Ok(payload) => payload,
                        Err(_) => {
                            println!("Failed to serialize message payload: {{ .payload.struct_reference }}");
                            return;
                        }
                    };
                    publish_message(client, &subject, &payload).await;
                {{else}}
                    publish_message(client, &subject, &"").await;
                {{end}}
            }
        {{ end }}
    {{ end }}
{{ end }}
