use sha2::{Digest, Sha256};

pub fn sha256(data: impl AsRef<[u8]>) -> String {
    let mut h = Sha256::new();
    h.update(data.as_ref());
    hex::encode(h.finalize())
}

// dummy change 21

// dummy change 22

// dummy change 23

// dummy change 24

// dummy change 25
