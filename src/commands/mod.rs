use crate::types::UgitCommandsStruct;

pub mod init;
pub mod hash_object;
pub mod get_object;

impl UgitCommandsStruct {
    pub fn new() -> Self {
        Self {
            init: init::init,
            hash_object: hash_object::hash_object,
            get_object: get_object::get_object,
        }
    }
}
