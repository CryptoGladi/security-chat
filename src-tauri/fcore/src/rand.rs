//! Module for random

use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

#[must_use]
pub fn get_crypto() -> ChaCha20Rng {
    ChaCha20Rng::from_entropy()
}
