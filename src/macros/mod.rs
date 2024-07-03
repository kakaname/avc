use std::fs;
use std::io::{self, Read, BufWriter, BufReader, Write};
use std::fs::File;
use sha1::{Sha1, Digest};
use crate::error::raise_error;
use crate::hashmap::FileHashMap;
use std::collections::HashMap;


/// computes a sha1 hash from a file of choice
pub fn compute_sha1_hash(file_path: &str) -> io::Result<String> {
  let mut file = File::open(file_path)?;
  let mut hasher = Sha1::new();
  let mut buffer = [0; 1024];

  loop {
      let bytes_read = file.read(&mut buffer)?;
      if bytes_read == 0 {
          break;
      }
      hasher.update(&buffer[..bytes_read]);
  }

  let hash = hasher.finalize();
  Ok(format!("{:x}", hash))
}

/// check if file exists
pub fn check_file_exists(arg : &str) -> bool {
  match fs::metadata(arg) {
    Ok(_) => return true,
    Err(_) => raise_error("file does not exist or cannot be found"),
  }
}


/// creates a file
pub fn create_file(path : &str, success_message: &str) {
  match File::create(path) {
    Ok(_) => (),
    Err(e) => eprintln!("Error creating file : {}, e : {} ", path, e),
  }

}


/// check that a directory exists
pub fn check_dir_exists(path : &str) {
  match fs::metadata(path) {
    Ok(_) => {
      println!("repository already exists");
      std::process::exit(1);
    },
    Err(_) => (),
  }
}

pub fn create_directory(path: &str, success_message: &str) {
  match fs::create_dir_all(path) {
      Ok(_) => {
          if !success_message.is_empty() {
              println!("{}", success_message);
          }
      }
      Err(e) => {
          eprintln!("failed to create repository: {}", e);
          std::process::exit(1);
      }
  }
}

/// replaces current hashmap version with new hashmap
pub fn replace_hashmap(hashmap : HashMap<String, String>) -> Result<(), std::io::Error> {
  let map_to_save = FileHashMap::get_from_hashmap(hashmap);
  if let Ok(serialized_data) = rmp_serde::to_vec(&map_to_save) { // matches for error from rmp_serde
    let mut file = File::create("./.avc/index.bin")?;
    file.write_all(&serialized_data)?;

  }else {
    raise_error("failed to serialize data when replacing hashmap");
  }

  Ok(())
  
}


#[cfg(test)]
mod test_macros {

  use super::*;

  #[test]
  pub fn test_sha_computation() {
    let file_path_1 = "./debug/file_1.txt";
    let file_path_2 = "./debug/file_2.txt";
    let mut f1 = File::create(file_path_1);
    let mut f2 = File::create(file_path_2);

    let hash_1 = compute_sha1_hash(&file_path_1).unwrap();
    let hash_2 = compute_sha1_hash(&file_path_2).unwrap();

    assert_eq!(hash_1, hash_2); // two hashes of different files with same content should be the same

    let _ = f2.unwrap().write_all(b"hello world");

    let hash_1 = compute_sha1_hash(&file_path_1).unwrap();
    let hash_2 = compute_sha1_hash(&file_path_2).unwrap();

    assert_ne!(hash_1, hash_2); // two hashes of different files with different content should be the different

    let hash_1 = compute_sha1_hash(&file_path_1).unwrap();
    let hash_2 = compute_sha1_hash(&file_path_1).unwrap();

    assert_eq!(hash_1, hash_2); // two hashes at different times of a single file should be equivalent

    // clean up after file creation
    let _ = std::fs::remove_file(file_path_1);
    let _ = std::fs::remove_file(file_path_2);

  }

  #[test]
  pub fn test_check_file() { 
    let file_path_1 = "./debug/file_1.txt";
    let mut f1 = File::create(file_path_1);
    assert!(check_file_exists(&file_path_1));

  }



}