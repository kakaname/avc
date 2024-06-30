use std::{env, process};
use crate::initalize::initalize;
use crate::help::print_help;


pub fn parse_args() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Usage : avc <argument>");
    process::exit(0);
  }

  match args[1].as_str() {
    "init" => initalize(),
    _ => print_help(),
  }

  process::exit(0);

}

