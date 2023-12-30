use const_format::concatcp;
use hex;
use sha1::{Digest, Sha1};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs;

pub const GIT_DIR: &str = ".ugit";
const OBJECTS_DIR: &str = concatcp!(GIT_DIR, "/objects");

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(GIT_DIR)?;
    fs::create_dir_all(OBJECTS_DIR)?;
    Ok(())
}

pub fn hash_object(filename: String, type_: Option<&str>) -> std::io::Result<String> {
    let obj_type = type_.unwrap_or("blob");

    let file = fs::File::open(&filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let obj = [obj_type.as_bytes(), &[0x0u8], contents.as_bytes()].concat();

    let mut hasher = Sha1::new();
    hasher.update(&obj);
    let hash = hex::encode(hasher.finalize());

    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        .open(format!("{}/{}", OBJECTS_DIR, hash))?;

    file.write(&obj)?;
    file.flush()?;

    return Ok(hash);
}

pub fn get_object(object: String, expected: Option<&str>) -> std::io::Result<String> {
    let expected_type = expected.unwrap_or("blob");

    let mut file = fs::File::open(format!("{}/{}", OBJECTS_DIR, object))?;
    let mut obj = String::new();
    file.read_to_string(&mut obj)?;

    let tmp_obj = obj.split("\0").take(2).collect::<Vec<&str>>();

    let [file_type, content] = tmp_obj[..] else {
        panic!()
    };

    if !expected.is_none() {
        assert!(
            file_type == expected_type,
            "Expected {}, got {}",
            expected_type,
            file_type,
        )
    }

    Ok(content.to_string())
}
