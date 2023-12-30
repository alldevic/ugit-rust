use const_format::concatcp;

pub const GIT_DIR: &str = ".ugit";
pub const OBJECTS_DIR: &str = concatcp!(GIT_DIR, "/objects");
