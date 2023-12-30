use std::fs;

#[path = "./constants/mod.rs"]
mod constants;

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(constants::GIT_DIR)?;
    fs::create_dir_all(constants::OBJECTS_DIR)?;

    Ok(())
}
