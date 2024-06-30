use std::fs;
use std::io;
mod macros;
use crate::macros::{create_directory};

pub fn print_help() {

}

///avc status [options]
///currently detects changes to files but is inefficent in its detection mechanism, should be changed
pub fn status() {
  


}

pub fn delete_repo() {

}


/// initalize a repository
pub fn initalize() {
  // Err should immdeiately exit
  create_directory("./.avc", "");
  create_directory("./.avc/blob", "");
  create_directory("./.avc/tag", "");
  create_directory("./.avc/name", "");
  create_directory("./.avc/message", "");
  create_directory("./.avc/hashes", "Succesfully created avc repository");

  

}