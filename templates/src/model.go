use serde::{Deserialize, Serialize};
    
// All models from the specification are defined here
{{ range .model.message_models }}
        {{ .model_definition }}
{{ end }}

