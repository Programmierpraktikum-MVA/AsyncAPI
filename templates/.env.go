################General Config################

SERVICE_PORT = "8080"
SERVER_URL = "{{ .server.url }}"
LOG_LEVEL = "DEBUG"
OPA_RULES= "path/to/admin/policy"
TRACING_ENABLED = false
SCHEMA_VALIDATION_ENABLED = true

################Channel wise Config################
{{ range .subscribe_channels }}
################{{ (index . 1).unique_id }}################
        {{ if (index . 1).original_operation.bindings }}
                {{ if (index . 1).original_operation.bindings.nats.queue }}
{{ (index . 1).unique_id}}_QUEUE = "{{ (index . 1).original_operation.bindings.nats.queue}}"
                {{else}}
{{ (index . 1).unique_id}}_STREAM = "{{ (index . 1).original_operation.bindings.nats.streamname}}"
                {{ end }}
        {{ end }}
{{ (index . 1).unique_id }}_SUBJECT = "{{ (index . 0) }}"
{{ end }}

{{ range .publish_channels }}
################{{ (index . 1).unique_id }}################
        {{ if (index . 1).original_operation.bindings }}
                {{ if (index . 1).original_operation.bindings.nats.queue }}
{{ (index . 1).unique_id}}_QUEUE = "{{ (index . 1).original_operation.bindings.nats.queue}}"
                {{else}}
{{ (index . 1).unique_id}}_STREAM = "{{ (index . 1).original_operation.bindings.nats.streamname}}"
                {{ end }}
        {{ end }}
{{ (index . 1).unique_id }}_SUBJECT = "{{ (index . 0) }}"
{{ end }}



