mod parse_args;
mod commands;
mod macros;
mod hashmap;
mod error;

use crate::{commands::{begin_tracking, delete_repo, initalize, print_help, status}, error::raise_error};
use crate::parse_args::parse_args;


fn main() {
  parse_args();

}
