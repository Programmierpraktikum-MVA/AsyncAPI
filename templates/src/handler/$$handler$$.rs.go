use async_nats::{Client, Message, jetstream};
use async_nats::jetstream::Context;
use crate::{publish_message,stream_publish_message,model::*,config::*};
use std::time;
use opentelemetry::global;
use opentelemetry::trace::Tracer;
use log::{debug, warn};

    {{ $isStream := false }}
    {{ $channel := . }}

    /// Publish a message in the {{ .unique_id }} channel
    /// Channel messages:
    /// {{ range .messages }}
    ///     {{ .unique_id }}
    /// {{ end }}
    {{ if key_exists  "original_operation" "bindings" "nats" "streamname" }}
        {{ $isStream := .original_operation.bindings.nats.streamname }}
    {{ end }}
            
    {{ if $isStream }}
        {{ range .messages }}
            pub async fn stream_producer_{{ .unique_id }}(context_stream: &Context, payload : {{ if .payload}} {{ .payload.struct_reference }} {{ else }} () {{ end }}) { //context instead of client
    let tracer = global::tracer("{{ .unique_id }}_stream_producer");
        let _span = tracer.start("stream_producer_{{ .unique_id }}");
        let subject = get_env("{{ .unique_id }}_SUBJECT").unwrap().clone();
                {{ if .payload }}
                    let payload = match serde_json::to_string(&payload) {
                        Ok(payload) => payload,
                        Err(_) => {
                            warn!("Failed to serialize message payload: {{ .payload.struct_reference }}");
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
    {{ range .messages }}
    pub async fn producer_{{ .unique_id }}(client: &Client, payload: {{ if .payload }} {{.payload.struct_reference}} {{else}} () {{end}}) {
    let tracer = global::tracer("{{ .unique_id }}_producer");
    let _span = tracer.start("producer_{{ .unique_id }}");
    let subject = get_env("{{ .unique_id }}_SUBJECT").unwrap().clone();
                {{ if .payload }}
                    let payload = match serde_json::to_string(&payload) {
                        Ok(payload) => payload,
                        Err(_) => {
                            warn!("Failed to serialize message payload: {{ .payload.struct_reference }}");
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

