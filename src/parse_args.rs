use std::{env, process};
use crate::{commands::{begin_tracking, delete_repo, initalize, print_help, status}, error::raise_error};

pub fn parse_args() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    process::exit(0);
  }

  match args[1].as_str() {
    "init" => initalize(),
    "status" => status(),
    "delete" => delete_repo(),
    "add" => {
      if args.len() < 3 { raise_error("Correct Usage : avc add <file>") }
      begin_tracking(&args[2]);
      },
    _ => print_help(),
  }

  process::exit(0);

}

