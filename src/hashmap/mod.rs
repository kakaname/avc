use std::collections::HashMap;
use std::hash::Hash;
use serde::{Serialize, Deserialize};
use rmp_serde::{Serializer, Deserializer};
use std::fs::{File};
use std::io::{self, BufReader, BufWriter, Write, Read};
use std::io::prelude::*;

use crate::error::raise_error;
use crate::macros::compute_sha1_hash;

#[derive(Serialize, Deserialize, Clone)]
pub struct FileHashMap {
  map : HashMap<String, String>
}
impl FileHashMap{
  pub fn set_map(&mut self, new_map : HashMap<String, String>) {
    self.map = new_map
  }

  pub fn get_map(self) -> HashMap<String, String> {
    self.map.clone()
  }
  
  pub fn empty() -> Self {
    let map : HashMap<String, String> = HashMap::new();
    Self { 
      map,
    }
  }

  // gets FileHashMap from File
  pub fn get_from_file(file_path : &str) -> Self {
    // Read the MessagePack data from the file and deserialize it back to a MyHashMap struct
    if let Ok(file) = File::open(file_path) {
      if file.metadata().unwrap().len() > 0{
        let mut reader = BufReader::new(file);
        let mut buf = Vec::new();
        match reader.read_to_end(&mut buf) {
          Ok(_) => (),
          Err(_) => (),
        }
        let mut de = Deserializer::new(&buf[..]);
        let deserialized = Deserialize::deserialize(&mut de).unwrap();

        deserialized
      }else {
        FileHashMap::empty()
      }
    }else{
      raise_error("Error opening file");
    }

  }

  pub fn save_to_file(&self, file_path : &str) -> Result<(), std::io::Error>{
    if let Ok(serialized_data) = rmp_serde::to_vec(&self) { // matches for error from rmp_serde
      let mut file = File::create(file_path)?;
      file.write_all(&serialized_data)?;

    }else {
      raise_error("failed to serialize data when replacing hashmap");
    }
    Ok(())
  }

  pub fn update_hashmap(&mut self, file_path : &str) {
    let hashed_file  = compute_sha1_hash(file_path).unwrap();

    match self.map.entry(file_path.to_string()) {
      std::collections::hash_map::Entry::Occupied(mut entry) => {
        if *entry.get() != hashed_file{
          *entry.get_mut() = hashed_file;
        }
      },
      std::collections::hash_map::Entry::Vacant(entry) => {
        entry.insert(hashed_file); // update hashmap
      }
    }
  }


}