use std::{env, process};
use crate::command;



pub fn parse_args() -> command::Command{
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Usage : avc <argument>");
    process::exit(0);
  }

  if args[1] == "init" {
    return command::Command::initalize();
  }

  process::exit(0);

}

