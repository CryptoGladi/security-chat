//! Module for [env variables](https://en.wikipedia.org/wiki/Environment_variable)

use log::info;

/// Get env variables by key
///
/// # Panics
///
/// If the env value has not been set, there will be a panic
#[must_use]
pub fn get_env_var(key: &str) -> String {
    let env = std::env::var(key).unwrap_or_else(|_| panic!("{key} env value must be set"));
    info!("get var from env. {} = {}", key, env);

    env
}

#[cfg(test)]
mod tests {
    #[test]
    fn env_var() {
        std::env::set_var("TEST_ENV", "TEST_VALUE");
        assert_eq!(super::get_env_var("TEST_ENV"), "TEST_VALUE");
    }

    #[test]
    #[should_panic(expected = "NOT_ENV! env value must be set")]
    fn env_var_with_error_must_be_set() {
        let _ = super::get_env_var("NOT_ENV!");
    }
}
