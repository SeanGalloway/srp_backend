use std::collections::HashMap;
use std::env;
use config::Map;
use crate::util::error_handling::application_error::ApplicationError;

pub fn get_item(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(val) => Some(val),
        Err(_) => None}
}

pub fn require_item(key: &str) -> Result<String, ApplicationError> {
    match env::var(key) {
        Ok(key) => Ok(key),
        Err(_) => match local_defaults().get(&key) {
            Some(val) => Ok(val.to_string()),
            None => Err(ApplicationError::new(format!("Env variable not found for key {0}", key).as_str()))
        }
    }
}


fn local_defaults() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("DATABASE_URL", "postgres://local_admin:admin_pwd@localhost:32760/srp"),
        ("SERVER_PORT", "8080")
    ])
}