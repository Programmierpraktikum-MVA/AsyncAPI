# Writing your own templates

-   Any templates that in the templates folder at compilation time will be embedded in the compiled binary.
-   If you only have the binary you can put templates in the folder `user-templates`.
    If a file from the `user-templates` folder has the same path as an embedded template, only the template from `user-template` will be rendered.
    -   set the command line argument `--user-templates` or `-u` to set a custom folder
-   the last file extension will be removed e.g: `file.rs.go` will be rendered to `file.rs`.
-   For examples refer to the allready included [templates](https://github.com/Programmierpraktikum-MVA/AsyncAPI/tree/d05d047c5ea9dfb221f31ecbf5af03387103e342/templates)



## What fields are available inside templates?

Any of these fields will be accessible:
```rust,noplayground
    pub struct TemplateContext<'a> {
        pub title: &'a String,
        pub description: &'a Option<String>,
        pub server: &'a Server,
        pub subscribe_channels: Vec<(&'a String, SimplifiedOperation)>,
        pub publish_channels: Vec<(&'a String, SimplifiedOperation)>,
        pub model: Model,
    }
    
    pub struct Model {
        pub message_models: Vec<RustSchemaRepresentation>,
        // pub enums: Vec<MultiStructEnum>,
    }
    
    pub struct SimplifiedOperation {
        pub unique_id: String,
        pub original_operation: Operation,
        // array, da es eine oder mehrere messages geben kann
        pub messages: Vec<SimplifiedMessage>,
        // pub multiple_messages_enum: Option<MultiStructEnum>,
    }
    
    pub struct MultiStructEnum {
        pub unique_id: String,
        pub messages: Vec<SimplifiedMessage>,
        pub struct_definition: String,
    }
    
    pub struct SimplifiedMessage {
        pub unique_id: String,
        pub original_message: Message,
        pub payload: Option<RustSchemaRepresentation>,
    }
```
-   for more information about the fields available from these structs please refer to: [all rust structs](https://github.com/Programmierpraktikum-MVA/AsyncAPI/tree/main/src/asyncapi_model)



## Render to separate files

It is possible to generate files for each specific object in your AsyncAPI documentation. For example, you can specify a filename like `$$handler$$.rs.go` to generate a file for each `publish_channel` defined in your AsyncAPI spec.

This works with file templates that include the following in their name:
- `$$handler$$`
- `$$producer$$`
- `$$model$$`
- `$$schemas$$`



## Functions available inside the templates

- `to_lower(input: String) -> String` converts String to lowercase
- `key_exists(input: String) -> String` checks if key exists
- `camel_to_snake_case(input :String) -> String` converts a String in camelCase to snake_case
- `replace(input: String, from: String, to: String) -> String` replaces `from` with `to` for `input`
  - Side Note: these functions are defined in  `src/generator/template_functions.rs` feel free to extend then, if you have access to the source code.

