mod commands;
mod cli;
mod utils;
mod types;

fn main() {
    let command = types::UgitCommandsStruct::new();

    cli::register(command);
}
