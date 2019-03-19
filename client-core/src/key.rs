//! Key management
use failure::ResultExt;
use rand::rngs::OsRng;
use secp256k1::{PublicKey as SecpPublicKey, SecretKey};

use crate::SECP;
use crate::{Error, ErrorKind};

/// Private key used in Crypto.com Chain
pub struct PrivateKey(SecretKey);

impl PrivateKey {
    /// Generates a new private key
    pub fn new() -> Result<PrivateKey, Error> {
        let mut rng = OsRng::new().context(ErrorKind::KeyGenerationError)?;
        let secret_key = SecretKey::new(&mut rng);

        Ok(PrivateKey(secret_key))
    }
}

/// Public key used in Crypto.com Chain
pub struct PublicKey(SecpPublicKey);

impl From<&PrivateKey> for PublicKey {
    fn from(private_key: &PrivateKey) -> Self {
        let secret_key = &private_key.0;

        let public_key = SECP.with(|secp| SecpPublicKey::from_secret_key(secp, secret_key));

        PublicKey(public_key)
    }
}
