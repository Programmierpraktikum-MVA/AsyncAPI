{{ range .publish_channels  }}
mod {{ (index . 1).unique_id }};
pub use {{ (index . 1).unique_id }}::*;
{{ end }}
{{ range .subscribe_channels  }}
mod {{ (index . 1).unique_id }};
pub use {{ (index . 1).unique_id }}::*;
{{ end }}
