// pub mod types {
// };

pub struct UgitCommandsStruct {
    pub init: fn () -> std::io::Result<()>,
    pub hash_object: fn (file: String, format_type: Option<&str>) -> std::io::Result<String>,
    pub get_object: fn (file: String, expected: Option<&str>) -> std::io::Result<String>,
}