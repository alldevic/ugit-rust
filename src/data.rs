use sha1::{Digest, Sha1};
use hex;
use std::{fs, io};
use const_format::concatcp;

pub const GIT_DIR: &str = ".ugit";
const OBJECTS_DIR: &str = concatcp!(GIT_DIR, "/objects");

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(GIT_DIR)?;
    fs::create_dir_all(OBJECTS_DIR)?;
    Ok(())
}

pub fn hash_object(filename: String, file_type: Option<&str>) -> std::io::Result<String> {
    let file_type = file_type.unwrap_or("blob");

    let mut file = fs::File::open(&filename)?;

    let mut hasher = Sha1::new();
    let mut contents = String::new();
    
    let _ = io::copy(&mut file, &mut hasher)?;
    let hash  = hex::encode(hasher.finalize());

    file.read_to_string(&mut contents);
    
    let mut obj = [file_type.as_bytes(), b"\x00", ].concat();
    // let mut object_file = fs::File::create(format!("{}/objects/{}", GIT_DIR, hash))?;

    // let mut chunk = file.take(100);
    // io::copy(&mut chunk, &mut object_file)?;

    // object_file.write_all(file.bytes())?;
    let _ = fs::copy(filename, format!("{}/{}", OBJECTS_DIR, hash))?;
    
    return Ok(hash);
}

pub fn get_object(object: String, expected: Option<&str>) -> std::io::Result<String> {
    let expected = expected.unwrap_or("blob");
    let path = format!("{}/{}", OBJECTS_DIR, object);

    let file = fs::read(path);

    Ok(contents)
}
