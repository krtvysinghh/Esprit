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

// dummy change 26

// dummy change 27

// dummy change 28

// dummy change 29

// dummy change 30

// dummy change 31

// dummy change 32

// dummy change 33

// dummy change 34

// dummy change 36

// dummy change 37

// dummy change 38

// dummy change 39

/// Core package version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Global application identifier
pub const APP_NAME: &str = "Esprit";

// padding 40

// padding 42

pub fn is_debug_mode() -> bool { cfg!(debug_assertions) }
