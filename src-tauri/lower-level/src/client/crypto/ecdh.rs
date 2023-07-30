use super::{aes, common::get_rand, CryptoError};
use log::info;
pub use p384::ecdh::{EphemeralSecret, SharedSecret as RawSharedSecter};
pub use p384::elliptic_curve::sec1::ToEncodedPoint;
pub use p384::{EncodedPoint, PublicKey};

pub struct SharedSecret(pub p384::ecdh::SharedSecret);

impl SharedSecret {
    pub fn get_key_for_aes_256(&self) -> &[u8] {
        &self.0.raw_secret_bytes()[..aes::SIZE_KEY]
    }
}

pub fn get_public_info() -> Result<(EphemeralSecret, PublicKey), CryptoError> {
    info!("run get_public_info");
    let secret = EphemeralSecret::random(&mut get_rand());
    let private_key = EncodedPoint::from(secret.public_key());
    let public_key = PublicKey::from_sec1_bytes(private_key.as_ref()).map_err(CryptoError::Ecdh)?;

    Ok((secret, public_key))
}

pub fn get_shared_secret(secret: &EphemeralSecret, public_key: &PublicKey) -> SharedSecret {
    info!("run get_shared_secret");
    SharedSecret(secret.diffie_hellman(public_key))
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

        println!("secret: {:?}", alice_shared_secret.0.raw_secret_bytes());
        println!(
            "secter len: {}",
            alice_shared_secret.0.raw_secret_bytes().len()
        );

        assert_eq!(
            alice_shared_secret.0.raw_secret_bytes(),
            bob_shared_secret.0.raw_secret_bytes()
        );
        assert_eq!(alice_shared_secret.get_key_for_aes_256().len(), 32);
    }
}
