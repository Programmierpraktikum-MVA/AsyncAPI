use serde::{Deserialize, Serialize};
    
// All models from the specification are defined here
{{ range .model.messages }}
    {{ if .payload }}
        {{ .payload.struct_definition }}
    {{ end }}
{{ end }}

{{ range .model.enums }}
    {{ .struct_definition }}
{{ end }}

