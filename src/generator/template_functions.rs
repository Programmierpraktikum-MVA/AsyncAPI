use gtmpl::{gtmpl_fn, Func, FuncError};

pub static TEMPLATE_FUNCTIONS: &[(&str, Func)] = &[
    ("key_exists", key_exists as Func),
    ("campel_to_snake_case", camel_to_snake_case as Func),
    ("to_lower", to_lower as Func),
];

/// TODO: descriptive comment needed about what function does!!
pub fn key_exists(
    args: &[gtmpl_value::Value],
) -> Result<gtmpl_value::Value, gtmpl_value::FuncError> {
    if args.is_empty() {
        return Err(gtmpl_value::FuncError::AtLeastXArgs(
            "Need at least 1 arg for key exists".to_string(),
            1,
        ));
    }
    let map = args[0].clone();
    if args.len() == 1 {
        return Ok(gtmpl_value::Value::Bool(true));
    }
    let keys = args[1..].to_vec();
    // check if keys is empty
    let rest_keys: Vec<gtmpl_value::Value> = match keys.len() > 1 {
        false => vec![],
        _ => keys[1..].to_vec(),
    };

    // extract first key
    if !keys.is_empty() {
        let key = keys[0].clone();
        match key {
            gtmpl_value::Value::String(s) => {
                let res: Result<gtmpl_value::Value, gtmpl_value::FuncError> = match map {
                    gtmpl_value::Value::Object(o) => {
                        // call again with rest of keys
                        key_exists(
                            vec![vec![o.get(&s).unwrap().clone()], rest_keys]
                                .concat()
                                .as_slice(),
                        )
                    }
                    _ => Ok(gtmpl_value::Value::Bool(false)),
                };
                return res;
            }
            _ => {
                return Err(gtmpl_value::FuncError::Generic(
                    "keys need to be string".to_string(),
                ));
            }
        }
    }
    Ok(gtmpl::Value::Bool(true))
}
gtmpl_fn!(
    /// converts an `input` string from camelCase to snake_case
    fn camel_to_snake_case(input: String) -> Result<String, FuncError> {
        let mut snake_case = String::new();
        let mut prev_char_lowercase = false;

        for c in input.chars() {
            if c.is_uppercase() {
                if prev_char_lowercase {
                    snake_case.push('_');
                }
                snake_case.push(c.to_lowercase().next().unwrap());
                prev_char_lowercase = false;
            } else {
                snake_case.push(c);
                prev_char_lowercase = true;
            }
        }

        Ok(snake_case)
    }
);

gtmpl_fn!(
    /// converts an `input` to lowercase
    fn to_lower(input: String) -> Result<String, FuncError> {
        Ok(input.to_lowercase())
    }
);
