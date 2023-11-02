use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

pub fn get_rand() -> ChaCha20Rng {
    rand_chacha::ChaCha20Rng::from_entropy()
}
