use serde::{Deserialize, Serialize};
    

{{ range .model.messages }}
    {{ if .payload }}
        {{ .payload.struct_definition }}
    {{ end }}
{{ end }}

{{ range .model.enums }}
    {{ .struct_definition }}
{{ end }}

