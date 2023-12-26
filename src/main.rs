use clap::{Parser, Subcommand, ValueEnum};

mod data;
mod utils;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl std::fmt::Display for ColorWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}


fn init() -> () {
    let _ = data::init();
    let current_dir = utils::get_current_working_dir();
    println!("Initialized empty ugit repository in {}//{}", current_dir, data::GIT_DIR);
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init => {
            init();
        }
    }
}
