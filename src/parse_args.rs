use std::{env, process};
use crate::commands::{initalize, status, delete_repo, print_help, begin_tracking, list_targets};

pub fn parse_args() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    process::exit(0);
  }

  match args[1].as_str() {
    "init" => initalize(),
    "status" => status(),
    "delete" => delete_repo(),
    "ls-targets" => list_targets(),
    "add" => {
      if args.len() < 3 {
        eprintln!("Correct Usage : avc add <file>");
        process::exit(1);
      }
      begin_tracking(&args[2])
    },
    _ => print_help(),
  }

  process::exit(0);

}

