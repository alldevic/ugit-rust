use clap::{Parser, Subcommand, ValueEnum};
use crate::types::UgitCommandsStruct;

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
  HashObject { file: String },
  CatFile {object: String}
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

pub fn register(commands: &UgitCommandsStruct) {
  let args = Cli::parse();

  match args.command {
    Commands::Init => {
      commands.init();
    }
    Commands::HashObject { file } => {
      commands.hash_object(file, None);
    }
    Commands::CatFile { object } => {
      commands.get_object(object, None);
    }
  }
}
