use super::*;

pub fn parse_enum_schema(
    schemas: &[ReferenceOr<Schema>],
    property_name: &str,
) -> Result<RustSchemaRepresentation, SchemaParserError> {
    // get type for each schema
    let schema_representations: Vec<RustSchemaRepresentation> = schemas
        .iter()
        .enumerate()
        .map(|(index, schema)| match schema {
            ReferenceOr::Item(item_schema) => {
                let payload_variant_name = format!("{}Variant{}", property_name, index + 1);
                parse_json_schema_to_rust_type(item_schema, payload_variant_name.as_str()).unwrap()
            }
            ReferenceOr::Reference { reference: _ } => {
                panic!("Refs should be resolved by now");
            }
        })
        .collect();

    let identifyer = validate_identifier_string(format!("{}Enum", property_name).as_str(), true);
    // assemble the enum
    let mut string_builder: String = format!(
        "#[derive(Serialize, Deserialize, Debug)]\n#[serde(untagged)]\npub enum {} {{\n",
        identifyer
    );

    schema_representations.iter().for_each(|schema| {
        let content = format!("{}({}),\n", schema.identifier, schema.struct_reference);
        string_builder.push_str(&content);
    });
    string_builder.push_str("}\n");
    Ok(RustSchemaRepresentation {
        identifier: identifyer.clone(),
        struct_reference: identifyer,
        model_definition: string_builder,
        model_type: "enum".to_string(),
        related_models: schema_representations,
    })
}
