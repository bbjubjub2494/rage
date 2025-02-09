//! Key structs and serialization.

use age_core::{
    format::FileKey,
    primitives::hkdf,
    secrecy::{ExposeSecret, Secret},
};
use rand::{rngs::OsRng, RngCore};

use crate::{
    error::DecryptError,
    format::HeaderV1,
    primitives::{stream::PayloadKey, HmacKey},
    protocol::Nonce,
};

const HEADER_KEY_LABEL: &[u8] = b"header";
const PAYLOAD_KEY_LABEL: &[u8] = b"payload";

pub fn new_file_key() -> FileKey {
    let mut file_key = [0; 16];
    OsRng.fill_bytes(&mut file_key);
    file_key.into()
}

pub fn mac_key(file_key: &FileKey) -> HmacKey {
    HmacKey(Secret::new(hkdf(
        &[],
        HEADER_KEY_LABEL,
        file_key.expose_secret(),
    )))
}

pub fn v1_payload_key(
    file_key: &FileKey,
    header: &HeaderV1,
    nonce: &Nonce,
) -> Result<PayloadKey, DecryptError> {
    // Verify the MAC
    header.verify_mac(mac_key(file_key))?;

    // Return the payload key
    Ok(PayloadKey(
        hkdf(nonce.as_ref(), PAYLOAD_KEY_LABEL, file_key.expose_secret()).into(),
    ))
}
