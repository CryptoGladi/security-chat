use crate::client::crypto::ecdh::NistP384;
use crate::client::crypto::ecdh::NonZeroScalar;
use crate::client::EphemeralSecret;
use log::debug;
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
        debug!("from");

        std::mem::transmute(x)
    }

    /// # Safety
    ///
    /// For a safe conversion, the structs must be the same. Therefore, do not upgrade the [`p384`] crate without a good reason
    pub unsafe fn get(self) -> EphemeralSecret {
        debug!("get");

        std::mem::transmute(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::crypto::ecdh::{get_public_info, get_shared_secret};
    use fcore::prelude::{BincodeConfig, Config};
    use temp_dir::TempDir;

    #[test]
    fn test_ecdh_with_ephemeral_secret_def() {
        let (alice_secret, alice_public_key) = get_public_info().unwrap();

        let alice_secret = unsafe {
            let temp_dir = TempDir::new().unwrap();
            let config = BincodeConfig::new(temp_dir.child("secter.temp"));
            let alice_secret_for_save = EphemeralSecretDef::from(alice_secret);

            config.save(&alice_secret_for_save).unwrap();
            config.load().unwrap().get()
        };

        let (bob_secret, bob_public_key) = get_public_info().unwrap();
        let alice_shared_secret = get_shared_secret(&alice_secret, &bob_public_key);
        let bob_shared_secret = get_shared_secret(&bob_secret, &alice_public_key);

        assert_eq!(
            alice_shared_secret.raw_secret_bytes(),
            bob_shared_secret.raw_secret_bytes()
        );
    }
}
