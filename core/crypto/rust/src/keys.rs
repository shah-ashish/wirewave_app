use crate::CryptoError;
use directories::BaseDirs;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path,
};
use uuid::Uuid;
use wirewave_core_config::APP_CONFIG;
use zeroize::{Zeroize, ZeroizeOnDrop};

const KEYPAIR_FILE_NAME: &str = "keypair.json";

#[derive(Serialize, Deserialize)]
struct StoredKeypair {
    schema_version: u32,
    secret_hex: String,
}

/// Persistent Ed25519 device identity keypair, zeroized from memory on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct IdentityKeypair {
    secret: [u8; 32],
}

impl IdentityKeypair {
    pub fn generate() -> Self {
        let signing = SigningKey::generate(&mut OsRng);
        Self {
            secret: signing.to_bytes(),
        }
    }

    pub fn from_bytes(secret: [u8; 32]) -> Self {
        Self { secret }
    }

    pub fn public_key_hex(&self) -> String {
        let signing = SigningKey::from_bytes(&self.secret);
        hex::encode(signing.verifying_key().to_bytes())
    }

    pub fn sign(&self, payload: &[u8]) -> Vec<u8> {
        let signing = SigningKey::from_bytes(&self.secret);
        signing.sign(payload).to_bytes().to_vec()
    }
}

pub fn verify_peer_signature(
    public_key_hex: &str,
    payload: &[u8],
    signature_bytes: &[u8],
) -> Result<(), CryptoError> {
    let key_bytes: [u8; 32] = hex::decode(public_key_hex)
        .ok()
        .and_then(|v| v.try_into().ok())
        .ok_or(CryptoError::InvalidPublicKey)?;
    let verifying = VerifyingKey::from_bytes(&key_bytes).map_err(|_| CryptoError::InvalidPublicKey)?;
    let sig_bytes: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| CryptoError::InvalidSignature)?;
    let sig = Signature::from_bytes(&sig_bytes);
    verifying
        .verify(payload, &sig)
        .map_err(|_| CryptoError::InvalidSignature)
}

pub fn fetch_or_create_keypair() -> Result<IdentityKeypair, CryptoError> {
    let base = BaseDirs::new().ok_or(CryptoError::NoDataDir)?;
    let path = base
        .data_local_dir()
        .join(APP_CONFIG.app_name)
        .join(KEYPAIR_FILE_NAME);
    fetch_or_create_keypair_at(&path)
}

pub fn fetch_or_create_keypair_at(path: &Path) -> Result<IdentityKeypair, CryptoError> {
    if let Some(existing) = load_keypair(path) {
        return Ok(existing);
    }
    if path.exists() {
        let backup = path.with_extension(format!("json.corrupt-{}", Uuid::new_v4()));
        fs::rename(path, backup)?;
    }

    let created = IdentityKeypair::generate();
    save_keypair(path, &created)?;
    Ok(created)
}

fn load_keypair(path: &Path) -> Option<IdentityKeypair> {
    let raw = fs::read_to_string(path).ok()?;
    let parsed: StoredKeypair = serde_json::from_str(&raw).ok()?;
    let bytes: [u8; 32] = hex::decode(parsed.secret_hex).ok()?.try_into().ok()?;
    Some(IdentityKeypair::from_bytes(bytes))
}

fn save_keypair(path: &Path, keypair: &IdentityKeypair) -> Result<(), CryptoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("json.tmp");
    let stored = StoredKeypair {
        schema_version: 1,
        secret_hex: hex::encode(keypair.secret),
    };
    fs::write(&tmp, serde_json::to_string_pretty(&stored)?)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypair_persists_and_verifies_signatures() {
        let dir = std::env::temp_dir().join(format!("ww_kp_test_{}", Uuid::new_v4()));
        let file = dir.join(KEYPAIR_FILE_NAME);

        let kp1 = fetch_or_create_keypair_at(&file).expect("create");
        let kp2 = fetch_or_create_keypair_at(&file).expect("reload");
        assert_eq!(kp1.public_key_hex(), kp2.public_key_hex());

        let sig = kp1.sign(b"wirewave-challenge");
        assert!(verify_peer_signature(&kp2.public_key_hex(), b"wirewave-challenge", &sig).is_ok());
        assert!(verify_peer_signature(&kp2.public_key_hex(), b"tampered", &sig).is_err());

        let _ = fs::remove_dir_all(dir);
    }
}
