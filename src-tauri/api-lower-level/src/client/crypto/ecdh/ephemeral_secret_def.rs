use crate::client::crypto::ecdh::NistP384;
use crate::client::crypto::ecdh::NonZeroScalar;
use crate::client::EphemeralSecret;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct EphemeralSecretDef {
    pub scalar: NonZeroScalar<NistP384>,
}

impl std::fmt::Debug for EphemeralSecretDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EphemeralSecretDef")
    }
}

static_assertions::assert_eq_size!(EphemeralSecret, EphemeralSecretDef);
static_assertions::assert_eq_align!(EphemeralSecret, EphemeralSecretDef);
static_assertions::assert_fields!(EphemeralSecretDef: scalar);

impl EphemeralSecretDef {
    /// # Safety
    ///
    /// For a safe conversion, the structs must be the same. Therefore, do not upgrade the [`p384`] crate without a good reason
    pub unsafe fn from(x: EphemeralSecret) -> Self {
        std::mem::transmute(x)
    }

    /// # Safety
    ///
    /// For a safe conversion, the structs must be the same. Therefore, do not upgrade the [`p384`] crate without a good reason
    pub unsafe fn get(self) -> EphemeralSecret {
        std::mem::transmute(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

    #[test]
    fn check_transmute() {
        let mut rng = rand_chacha::ChaCha20Rng::from_entropy();

        let d: EphemeralSecretDef =
            totally_safe_transmute::totally_safe_transmute(2); // TODO NOT WORK!

        todo!("make testing");
    }
}
