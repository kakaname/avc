use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rmp_serde::{Serializer, Deserializer};
use std::fs::{File};
use std::io::{self, BufReader, BufWriter, Write, Read};
use std::io::prelude::*;

use crate::error::raise_error;

#[derive(Serialize, Deserialize, Clone)]
pub struct FileHashMap {
  map : HashMap<String, String>
}
impl FileHashMap{
  pub fn save(self) -> io::Result<()>{
    let file_path = "hashmap.bin";
    match std::fs::remove_file(file_path) { // remove previous versions of the file
      Ok(_) => (),
      Err(_) => raise_error("Error when removing previous version of file"),
    }

    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    self.serialize(&mut Serializer::new(&mut writer)).unwrap();
    writer.flush()?;

    Ok(())
  }

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

  // creates FileHashMap from hashmap
  pub fn get_from_hashmap(map : HashMap<String, String>) -> Self {
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


}