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
    path : Vec<PathBuf>,
  },
  Init,
  Status,
  Delete,
}



pub fn parse_args() {
  let args = Cli::parse();
  match args.command {
    Commands::Add { path } => {
      begin_tracking(&path[0].to_string_lossy().into_owned());
    },
    Commands::Delete {} => {
      delete_repo();
    },
    Commands::Init {} => {
      initalize();
    },
    Commands::Status {} => {
      status();
    }
  }
}