use std::env;

/// Get environment variable
pub fn get_env(e: &str) -> String {
    env::var(e).unwrap()
}
