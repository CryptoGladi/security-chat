use log::warn;

pub fn env_var(name: &str) -> String {
    let env = std::env::var(name).unwrap_or_else(|_| panic!("{} must be set", name));
    warn!("get var from env. {} = {}", name, env);

    env
}

#[cfg(test)]
mod tests {
    #[test]
    fn env_var() {
        std::env::set_var("TEST_ENV", "TEST_VALUE");
        assert_eq!(super::env_var("TEST_ENV"), "TEST_VALUE");
    }

    #[test]
    #[should_panic(expected = "NOT_ENV! must be set")]
    fn env_var_with_error_must_be_set() {
        super::env_var("NOT_ENV!");
    }
}
