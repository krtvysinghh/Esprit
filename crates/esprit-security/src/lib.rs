use sha2::{Digest, Sha256};

pub fn checksum(bytes: impl AsRef<[u8]>) -> String {
    let mut h = Sha256::new();
    h.update(bytes.as_ref());
    hex::encode(h.finalize())
}
