use p384::ecdh::EphemeralSecret;

#[allow(clippy::module_name_repetitions)]
pub use crate::unsafe_impl::crypto::ephemeral_secret_def::EphemeralSecretDef as UnsafeEphemeralSecretDef;

#[must_use]
pub fn from(secret: EphemeralSecret) -> UnsafeEphemeralSecretDef {
    // SAFETY:
    // See [`UnsafeEphemeralSecretDef`] doc
    unsafe { UnsafeEphemeralSecretDef::from(secret) }
}

#[must_use]
pub fn get(unsafe_secret: UnsafeEphemeralSecretDef) -> EphemeralSecret {
    // SAFETY:
    // See [`UnsafeEphemeralSecretDef`] doc
    unsafe { unsafe_secret.get() }
}

#[cfg(test)]
mod tests {
    use api_lower_level::client::impl_crypto::ecdh::{get_public_info, get_shared_secret};
    use fcore::prelude::{BincodeConfig, Config};
    use temp_dir::TempDir;

    #[test]
    fn test_ecdh_with_ephemeral_secret_def() {
        let (alice_secret, alice_public_key) = get_public_info().unwrap();

        let alice_secret = {
            let temp_dir = TempDir::new().unwrap();
            let config = BincodeConfig::new(temp_dir.child("secter.temp"));
            let alice_secret_for_save = super::from(alice_secret);

            config.save(&alice_secret_for_save).unwrap();
            let loaded_secter = config.load().unwrap();
            super::get(loaded_secter)
        };

        let (bob_secret, bob_public_key) = get_public_info().unwrap();
        let alice_shared_secret = get_shared_secret(&alice_secret, &bob_public_key);
        let bob_shared_secret = get_shared_secret(&bob_secret, &alice_public_key);

        assert_eq!(
            alice_shared_secret.raw_secret_bytes(),
            bob_shared_secret.raw_secret_bytes()
        );
    }

    #[test]
    fn impl_partial_eq() {
        let (alice_secret, _) = get_public_info().unwrap();
        let (bob_secret, _) = get_public_info().unwrap();

        let (alice_secret, bob_secret) = { (super::from(alice_secret), super::from(bob_secret)) };

        assert_eq!(alice_secret, alice_secret);
        assert_ne!(alice_secret, bob_secret);
    }
}
