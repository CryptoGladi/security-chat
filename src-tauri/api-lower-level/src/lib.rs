//! Crate for low-level API management between client and server.
//!
//! Includes:
//! 1. A wrapper for [gRPC](https://grpc.io/) calls
//! 2. tools for data encryption decryption

pub(crate) mod built_info;
pub mod client;

#[cfg(test)]
pub mod test_utils;