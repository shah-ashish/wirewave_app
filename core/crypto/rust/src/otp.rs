use hmac::{Hmac, Mac};
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use wirewave_core_config::APP_CONFIG;

type HmacSha256 = Hmac<Sha256>;

/// Generates a cryptographically random 6-digit numeric pairing OTP (`000000`..`999999`).
pub fn generate_otp() -> String {
    let value = OsRng.next_u32() % 1_000_000;
    format!("{value:06}")
}

/// Computes an OTP-bound HMAC proof over the ephemeral X25519 shared secret and transcript.
/// The raw 6-digit OTP is never transmitted over the network.
pub fn compute_otp_proof(shared_secret: &[u8; 32], otp: &str, transcript: &[u8]) -> [u8; 32] {
    let mut mac =
        HmacSha256::new_from_slice(shared_secret).expect("HMAC-SHA256 accepts any key length");
    mac.update(APP_CONFIG.connection.protocol_domain.as_bytes());
    mac.update(b":otp:");
    mac.update(otp.trim().as_bytes());
    mac.update(b":");
    mac.update(transcript);
    let result = mac.finalize().into_bytes();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

/// Constant-time verification of a peer's OTP proof.
pub fn verify_otp_proof(
    shared_secret: &[u8; 32],
    expected_otp: &str,
    transcript: &[u8],
    peer_proof: &[u8],
) -> bool {
    let mut mac =
        HmacSha256::new_from_slice(shared_secret).expect("HMAC-SHA256 accepts any key length");
    mac.update(APP_CONFIG.connection.protocol_domain.as_bytes());
    mac.update(b":otp:");
    mac.update(expected_otp.trim().as_bytes());
    mac.update(b":");
    mac.update(transcript);
    mac.verify_slice(peer_proof).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn otp_proof_matches_only_exact_code_and_transcript() {
        let secret = [7u8; 32];
        let otp = generate_otp();
        assert_eq!(otp.len(), 6);

        let proof = compute_otp_proof(&secret, &otp, b"devA|devB");
        assert!(verify_otp_proof(&secret, &otp, b"devA|devB", &proof));
        assert!(!verify_otp_proof(&secret, "000000", b"devA|devB", &proof));
        assert!(!verify_otp_proof(&secret, &otp, b"devA|devC", &proof));
    }
}
