use sha1::{Digest, Sha1};
use hex;
use std::{fs, io};
use const_format::concatcp;
// use std::io::prelude::*;

pub const GIT_DIR: &str = ".ugit";
const OBJECTS_DIR: &str = concatcp!(GIT_DIR, "/objects");

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(GIT_DIR)?;
    fs::create_dir_all(OBJECTS_DIR)?;
    Ok(())
}

pub fn hash_object(filename: String) -> std::io::Result<String> {
    let mut file = fs::File::open(&filename)?;
    let mut hasher = Sha1::new();
    let _ = io::copy(&mut file, &mut hasher)?;
    let hash  = hex::encode(hasher.finalize());
    
    // let mut object_file = fs::File::create(format!("{}/objects/{}", GIT_DIR, hash))?;

    // let mut chunk = file.take(100);
    // io::copy(&mut chunk, &mut object_file)?;

    // object_file.write_all(file.bytes())?;
    let _ = fs::copy(filename, format!("{}/{}", OBJECTS_DIR, hash))?;
    
    return Ok(hash);
}
