//! Module for [ECDH](https://en.wikipedia.org/wiki/Elliptic-curve_Diffie%E2%80%93Hellman)

use crate::client::impl_crypto::error::CryptoError;
pub use ephemeral_secret_def::EphemeralSecretDef;
use fcore::prelude::get_crypto;
use log::debug;
use p384::ecdh::SharedSecret;
pub use p384::ecdh::{EphemeralSecret, SharedSecret as RawSharedSecter};
pub use p384::elliptic_curve::sec1::ToEncodedPoint;
use p384::elliptic_curve::NonZeroScalar;
use p384::NistP384;
pub use p384::{EncodedPoint, PublicKey};

pub mod ephemeral_secret_def;

pub fn get_public_info() -> Result<(EphemeralSecret, PublicKey), CryptoError> {
    debug!("run get_public_info");

    let secret = EphemeralSecret::random(&mut get_crypto());
    let private_key = EncodedPoint::from(secret.public_key());
    let public_key = PublicKey::from_sec1_bytes(private_key.as_ref()).map_err(CryptoError::Ecdh)?;

    Ok((secret, public_key))
}

pub fn get_shared_secret(secret: &EphemeralSecret, public_key: &PublicKey) -> SharedSecret {
    debug!("run get_shared_secret");
    secret.diffie_hellman(public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecdh() {
        let (alice_secret, alice_public_key) = get_public_info().unwrap();
        let (bob_secret, bob_public_key) = get_public_info().unwrap();
        let alice_shared_secret = get_shared_secret(&alice_secret, &bob_public_key);
        let bob_shared_secret = get_shared_secret(&bob_secret, &alice_public_key);

        println!("secret: {:?}", alice_shared_secret.raw_secret_bytes());

        println!(
            "secter len: {}",
            alice_shared_secret.raw_secret_bytes().len()
        );

        assert_eq!(
            alice_shared_secret.raw_secret_bytes(),
            bob_shared_secret.raw_secret_bytes()
        );
    }
}
