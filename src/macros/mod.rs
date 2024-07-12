use std::fmt::format;
use std::fs;
use std::io::{self, Read, BufWriter, BufReader, Write};
use tar::{Builder, Archive};
use flate2::write::{GzEncoder, GzDecoder};
use flate2::Compression;
use std::fs::File;
use sha1::{Sha1, Digest};
use crate::error::raise_error;
use std::path::Path;
use walkdir::WalkDir;
use hex::encode;


/// computes a sha1 hash from multiple files of choice
pub fn compute_sha1_hash(file_path: &str) -> io::Result<String> {
  let mut hasher = Sha1::new();
  let mut file = File::open(file_path)?;
  let mut buffer = [0u8; 1024];

  loop {
    let n = file.read(&mut buffer)?;

    if n == 0 {
      break;
    }
    
    hasher.update(&buffer[..n]);
  }

  let result = hasher.finalize();
  Ok(hex::encode(result))
}

/// saves the temporary state of the file
pub fn save_temp_file_state(file_path : &str, file_hash : String) {
   let destination_path = format!("{}{}", "./.avc/tmp/", file_path);
   let new_path = format!("{}{}", "./", file_path);
   match fs::copy(new_path.clone(), destination_path) {
    Ok(arg) => {

    },
    Err(e) => {
      eprintln!("{} ", e);
    }
   }

}


/// check if file exists
pub fn check_file_exists(file : &str) -> bool {
  match fs::metadata(file) {
    Ok(_) => (),
    Err(_) => raise_error("file does not exist or cannot be found"),
  }
  return true;
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

/// creates a directory
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

/// flushes a folder
pub fn flush_folder(folder_path : &str) -> io::Result<()> {
  fs::remove_dir_all(folder_path)?;
  fs::create_dir(folder_path)?;
  fs::File::create("./.avc/tmp/index.bin")?; // currently a placeholder for index replacement
  Ok(())
}

/// compresses a folder
pub fn compress_folder(from_folder : &str, to_folder : &str) -> io::Result<()> {
  let _ = create_directory(to_folder, "");
  let to_file = format!("{}{}", to_folder, "/commit");
  
  let tar_gz = File::create(to_file)?;
  let enc = GzEncoder::new(tar_gz, Compression::default());
  let mut tar = Builder::new(enc);

  for entry in WalkDir::new(from_folder) {
    let entry = entry?;
    let path = entry.path();
    let name = path.strip_prefix(entry.path().parent().unwrap()).unwrap();
    if path.is_file() {
      tar.append_path_with_name(path, name)?;
    } else if path.is_dir() {
      tar.append_dir(name, path)?;
    }
  }

  let _ = tar.finish();
  Ok(())
}

/// decompress file
pub fn decompress_file(src_file : &str, dest_folder : &str) -> io::Result<()> {
  let tar_gz = File::open(src_file)?;
  let dec = GzDecoder::new(tar_gz);
  let mut archive = Archive::new(dec);

  archive.unpack(dest_folder)?;
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

  pub fn test_compression() {
    // tests both file compression and decompression
    let file_path_1 = "./debug/file_1.txt";
    let compressed_file = "./debug/file_2.gz";
    let output_file = "./debug/file_2";
    match File::create(file_path_1){
      Ok(mut e) => {
        let _ = e.write_all(b"hello world");
      },
      Err(e) => {
        panic!("Error when creating file_path_1 : {} ", e);
      }
    }
    let _ = compress_file(&file_path_1, &compressed_file);
    let _ = decompress_file(&compressed_file, &output_file);
    let output_path = Path::new(output_file);
    match fs::read_to_string(output_path) {
      Ok(e) => {
        println!("output_content: {} ", e);
      },
      Err(e) => {
        let _ = std::fs::remove_file(file_path_1);
        let _ = std::fs::remove_file(compressed_file);
        let _ = std::fs::remove_file(output_file);
      }
    }


    let _ = std::fs::remove_file(file_path_1);
    let _ = std::fs::remove_file(compressed_file);
    let _ = std::fs::remove_file(output_file);



  }


}