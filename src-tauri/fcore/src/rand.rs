//! Module for random

use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

pub fn get_crypto_rand() -> ChaCha20Rng {
    ChaCha20Rng::from_entropy()
}
