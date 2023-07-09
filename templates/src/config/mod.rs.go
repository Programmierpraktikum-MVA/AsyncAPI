use dotenv::dotenv;
use std::{collections::HashMap, env};

pub fn get_env() -> HashMap<String, String> {
    dotenv().ok();
    env::vars().collect()
}