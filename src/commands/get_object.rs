use std::io::prelude::*;
use std::fs;

#[path = "./constants/mod.rs"]
mod constants;

pub fn get_object(object: String, expected: Option<&str>) -> std::io::Result<String> {
  let expected_type = expected.unwrap_or("blob");

  let mut file = fs::File::open(format!("{}/{}", constants::OBJECTS_DIR, object))?;
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
