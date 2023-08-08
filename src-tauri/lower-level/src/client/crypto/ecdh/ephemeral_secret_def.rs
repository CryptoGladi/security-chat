use crate::client::EphemeralSecret;
use crate::built_info::CrateInfo;
use crate::client::crypto::ecdh::NistP384;
use crate::client::crypto::ecdh::NonZeroScalar;
use serde::{Serialize, Deserialize};

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

const CRATE_FOR_MODULE: CrateInfo = CrateInfo {
    name: "p384",
    version: "0.13.0"
};

impl EphemeralSecretDef {
    /// # Safety
    ///
    /// For a safe conversion, the structs must be the same. Therefore, do not upgrade the [`p384`] crate without a good reason
    pub unsafe fn from(x: EphemeralSecret) -> Self {
        debug_assert_eq!(crate::built_info::check_package(CRATE_FOR_MODULE), true);
        std::mem::transmute(x)
    }

    /// # Safety
    ///
    /// For a safe conversion, the structs must be the same. Therefore, do not upgrade the [`p384`] crate without a good reason
    pub unsafe fn get(self) -> EphemeralSecret {
        debug_assert_eq!(crate::built_info::check_package(CRATE_FOR_MODULE), true);
        std::mem::transmute(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_package()  {
        assert_eq!(crate::built_info::check_package(CRATE_FOR_MODULE), true);
    }
}