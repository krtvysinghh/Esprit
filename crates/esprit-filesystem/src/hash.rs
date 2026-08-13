use anyhow::Result;
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read, path::Path};

pub fn hash(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;

        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
    }

    let digest = hasher.finalize();

    let mut out = String::with_capacity(64);

    for byte in digest {
        use std::fmt::Write;
        let _ = write!(&mut out, "{:02x}", byte);
    }

    Ok(out)
}
