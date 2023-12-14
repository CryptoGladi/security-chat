use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct ConnectionParameters {
    pub timeout: Duration,
}

impl Default for ConnectionParameters {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
        }
    }
}
