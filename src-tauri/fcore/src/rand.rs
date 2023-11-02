//! Module for random

use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

pub fn get_crypto_rand() -> ChaCha20Rng {
    rand_chacha::ChaCha20Rng::from_entropy()
}
