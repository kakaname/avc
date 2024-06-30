use std::{env, process};
use crate::commands::{initalize, status, delete_repo, print_help};

pub fn parse_args() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Usage : avc <argument>");
    process::exit(0);
  }

  match args[1].as_str() {
    "init" => initalize(),
    "status" => status(),
    "delete" => delete_repo(),
    _ => print_help(),
  }

  process::exit(0);

}

