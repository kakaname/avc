use std::{collections::HashMap, fs::File};

use crate::{error::raise_error, macros::{append_to_bin_file, check_dir_exists, check_file_exists, 
            compute_sha1_hash, create_directory, create_file, diff_file, read_from_bin_file,
            replace_hashmap}};
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

/// list all files that are being tracked
pub fn list_targets() {
  let bin_content = read_from_bin_file("./.avc/index.bin").unwrap();
  println!("Tracking Files: \n");
  for filename in bin_content {
    println!("{}\n", filename);
  }

}

// begins tracking a file or updates a files hash
pub fn begin_tracking(arg : &str) {
  if !check_file_exists(arg) { raise_error("file does not exist or cannot be found") }
  let hashed_file  = compute_sha1_hash(arg).unwrap();
  let most_recent_hashmap_object = FileHashMap::get_from_file("./.avc/index.bin"); // uses a wrapper object around the hashmap for more functionality
  let mut most_recent_hashmap = most_recent_hashmap_object.get_map().clone();

  match most_recent_hashmap.entry(arg.to_string()) {
    std::collections::hash_map::Entry::Occupied(mut entry) => {
      if *entry.get() != hashed_file{
        *entry.get_mut() = hashed_file;
        let _ = replace_hashmap(most_recent_hashmap); // should do error propogation management here // update hashmap
      }
    },
    std::collections::hash_map::Entry::Vacant(entry) => {
      entry.insert(hashed_file); // update hashmap
      let _ = replace_hashmap(most_recent_hashmap.clone());
    }
  }



}


/// deletes the repository
pub fn delete_repo() {
  match std::fs::remove_dir_all("./.avc") {
    Ok(_) => (),
    Err(e) => {
      eprintln!("error when deleting repository : {}", e);
      std::process::exit(1);
    },
  }

}


/// initalize a repository
pub fn initalize() {
  // Err should immdeiately exit
  check_dir_exists("./.avc");
  create_directory("./.avc", "");
  create_directory("./.avc/blob", "");
  create_directory("./.avc/tag", "");
  create_directory("./.avc/name", "");
  create_directory("./.avc/message", "");
  create_directory("./.avc/hashes", "Succesfully created avc repository");
  create_file("./.avc/index.bin", "");

  

}