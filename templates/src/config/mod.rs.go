use dotenv::dotenv;
use lazy_static::lazy_static;
use std::{collections::HashMap, env, sync::RwLock};


lazy_static! {
    static ref ENV_VARS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

pub fn initialize_env() {
    dotenv().ok();
    let env: HashMap<String, String> = env::vars().collect();
    ENV_VARS.try_write().unwrap().clear();
    ENV_VARS.try_write().unwrap().extend(env);
}

pub fn get_env(key: &str) -> Option<String> {
    ENV_VARS.read().unwrap().get(key).cloned()
}
