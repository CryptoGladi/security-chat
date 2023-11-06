//! Main API crate which is needed to store encryption keys, high level functions (i.e. api-lower-level crate implementation).

pub mod client;
pub mod prelude;

#[cfg(any(test, feature = "benchmarking"))]
pub mod test_utils;
