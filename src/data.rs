use std::fs;

pub const GIT_DIR: &str  = ".ugit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(GIT_DIR)?;
    Ok(())
}
