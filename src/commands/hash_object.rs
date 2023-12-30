use hex;
use sha1::{Digest, Sha1};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs;

#[path = "./constants/mod.rs"]
mod constants;

pub fn hash_object(filename: String, format_type: Option<&str>) -> std::io::Result<String> {
  let obj_type = format_type.unwrap_or("blob");

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
      .open(format!("{}/{}", constants::OBJECTS_DIR, hash))?;

  file.write(&obj)?;
  file.flush()?;

  return Ok(hash);
}
