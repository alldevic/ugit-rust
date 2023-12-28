use sha1::{Digest, Sha1};
use hex;
use std::{fs, io};

pub const GIT_DIR: &str = ".ugit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(GIT_DIR)?;
    Ok(())
}
pub fn hash_object(filename: String) -> std::io::Result<String> {
    let mut file = fs::File::open(&filename)?;
    let mut hasher = Sha1::new();
    let _ = io::copy(&mut file, &mut hasher)?;
    let hash = hex::encode(hasher.finalize());

    return Ok(hash);
}
