use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::commands::{begin_tracking, delete_repo, initalize, status};

#[derive(Debug, Parser)]
#[command(name="avc")]
#[command(about="an AI Version Control tool")]
pub struct Cli {
  #[command(subcommand)]
  command : Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  #[command(arg_required_else_help = true)]
  Add {
    #[arg(required = true)]
    paths : Vec<PathBuf>,
  },
  Init,
  Status,
  Delete,
  Commit {
    #[arg(required = true)]
    message : Option<String>,
  },
}



pub fn parse_args() {
  let args = Cli::parse();
  match args.command {
    Commands::Add { paths } => {
      let str_paths: Vec<String> = paths.iter()
      .map(|path| path.to_string_lossy().to_string())
      .collect();
      begin_tracking(str_paths);
    },
    Commands::Delete {} => {
      delete_repo();
    },
    Commands::Init {} => {
      initalize();
    },
    Commands::Status {} => {
      status();
    },
    Commands::Commit { message } => {
      //commit(str_paths);
    },
  }
}