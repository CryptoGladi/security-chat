//! Module for [env variables](https://en.wikipedia.org/wiki/Environment_variable)

use log::warn;

/// Get env variables by key
pub fn get_env_var(key: &str) -> String {
    let env = std::env::var(key).unwrap_or_else(|_| panic!("{} env value must be set", key));
    warn!("get var from env. {} = {}", key, env);

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
        super::get_env_var("NOT_ENV!");
    }
}
