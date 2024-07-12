use core::panic;
use std::{collections::HashMap, fs::File};

use crate::{error::raise_error, 
  macros::{check_dir_exists, check_file_exists, 
          compute_sha1_hash, create_directory, 
          create_file, save_temp_file_state,
          compress_folder, flush_folder}};
use crate::hashmap::FileHashMap;

use std::io::{self, Write};


pub fn print_help() {
    println!("Usage : avc <argument>");

}

///avc status [options]
///currently detects changes to files but is inefficent in its detection mechanism, should be changed
pub fn status() {
  let hashmap_object = FileHashMap::get_from_file("./.avc/tmp/index.bin");
  let hashmap = hashmap_object.get_map(); 
  println!("Printing Status --Debug purposes, should be repurposed");
  for key in hashmap.keys() {
    println!("Tracking File: {} , with hash : {}", key, hashmap[key]);
  }

}

// begins tracking a file or updates a files hash
pub fn begin_tracking(files : Vec<String>) {
  let mut hashmap = FileHashMap::get_from_file("./.avc/tmp/index.bin");
  for file in files {
    hashmap.update_hashmap(&file);
    match compute_sha1_hash(&file) {
      Ok(arg) => {
        let _ = save_temp_file_state(&file, arg);
      },
      Err(e) => {
        raise_error("error when computing sha1_file_hash");
      }
    }
  }

  let _ = hashmap.save_to_file("./.avc/tmp/index.bin");
}

pub fn commit(message : String) {
  // message + file-blobs => hash
  // message -> bin
  // performance stats 
  // flush current index
  let folder_hash = compute_sha1_hash("./.avc/tmp/index.bin").unwrap();
  let commit_path = format!("{}{}", "./.avc/commits/", folder_hash);
  match compress_folder("./.avc/tmp/", &commit_path ) {
    Ok(arg) => {
    },
    Err(e) => {
      eprintln!("{}", e);
    }
  }

  // write message file
  let message_path = format!("{}/message.txt", commit_path);
  match File::create(message_path) {
    Ok(mut file) => {
      let _ = file.write_all(message.as_bytes());
    },
    Err(e) => {
      eprintln!("error when writing to file {}", e);

    }
  }

  // flush /tmp
  let _ = flush_folder("./.avc/tmp/");

  // make note of previous commit
  // write message file
  let previous_hash = "./.avc/tmp/hash.txt";
  match File::create(previous_hash) {
    Ok(mut file) => {
      let _ = file.write_all(folder_hash.as_bytes());
    },
    Err(e) => {
      eprintln!("error when writing to file {}", e);

    }
  }
  
  

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
  create_directory("./.avc/commits", "");
  create_directory("./.avc/tmp", "");
  create_directory("./.avc/tag", "");
  create_file("./.avc/tmp/index.bin", "Succesfully created avc repository");

}


#[cfg(test)]
mod test_commands {
  use super::*;

  


}