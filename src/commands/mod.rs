use std::{collections::HashMap, fs::File};

use crate::{error::raise_error, macros::{check_dir_exists, check_file_exists, compress_file, compute_sha1_hash, create_directory, create_file}};
use crate::hashmap::FileHashMap;



pub fn print_help() {
    println!("Usage : avc <argument>");

}

///avc status [options]
///currently detects changes to files but is inefficent in its detection mechanism, should be changed
pub fn status() {
  let hashmap_object = FileHashMap::get_from_file("./.avc/index.bin");
  let hashmap = hashmap_object.get_map(); 
  println!("Printing Status --Debug purposes, should be repurposed");
  for key in hashmap.keys() {
    println!("Tracking File: {} , with hash : {}", key, hashmap[key]);
  }

}

// begins tracking a file or updates a files hash
pub fn begin_tracking(arg : &str) {
  check_file_exists(arg);
  let mut hashmap = FileHashMap::get_from_file("./.avc/index.bin");
  hashmap.update_file(arg);
  let _ = hashmap.save_to_file("./.avc/index.bin");

  // we should change this to work as a commit instead
}

pub fn add_file() {

}


/// deletes the repository
pub fn delete_repo() {
  match std::fs::remove_dir_all("./.avc") {
    Ok(_) => (),
    Err(e) => raise_error("error when deleting repository"),
  }

}

/// initalize a repository
pub fn initalize() {
  // Err should immdeiately exit
  check_dir_exists("./.avc");
  create_directory("./.avc", "");
  create_directory("./.avc/files", "");
  create_directory("./.avc/files/blob", "");
  create_directory("./.avc/files/message", "");
  create_directory("./.avc/temp", "");
  create_directory("./.avc/tag", "");
  create_file("./.avc/index.bin", "Succesfully created avc repository");

}