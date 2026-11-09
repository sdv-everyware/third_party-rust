//! Build check: the parts of rs-matter the Matter bundle uses are linked, AES-CCM included.

pub use rs_matter;
pub use rs_matter_im_core::{invoke_cmd, read_attr, write_attr};
pub use rs_matter_onboard as onboard;

use rs_matter::crypto::{test_only_crypto, Aead, Crypto, CryptoSensitiveRef, AEAD_TAG_LEN};

/// Encrypts 16 zero bytes with AES-128-CCM under a zero key and nonce and decrypts them again;
/// 0 when the round trip gives the plaintext back.
#[no_mangle]
pub extern "C" fn rsm_smoke_ccm() -> i32 {
    let crypto = test_only_crypto();
    let Ok(mut aead) = crypto.aead() else { return 1 };
    let (key, nonce) = ([0u8; 16], [0u8; 13]);
    let mut buf = [0u8; 16 + AEAD_TAG_LEN];
    let Ok(sealed) = aead.encrypt_in_place(CryptoSensitiveRef::new(&key), CryptoSensitiveRef::new(&nonce), b"aad", &mut buf, 16) else {
        return 2;
    };
    let mut ct = sealed.to_vec();
    match aead.decrypt_in_place(CryptoSensitiveRef::new(&key), CryptoSensitiveRef::new(&nonce), b"aad", &mut ct) {
        Ok(pt) if pt == [0u8; 16] => 0,
        _ => 3,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ccm_round_trip() {
        assert_eq!(super::rsm_smoke_ccm(), 0);
    }
}
