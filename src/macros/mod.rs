use std::fs;
use std::io::{self, Read, BufWriter, BufReader, Write};
use std::fs::File;
use sha1::{Sha1, Digest};

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
    Err(_) => {
      eprintln!("file {} does not exist or cannot be found", arg);
      return false;
    },
  }
}

/// check if a file is being tracked by checking ./avc/index
pub fn check_file_tracked(arg : &str) -> bool {
  let output: Vec<String> = read_from_bin_file("./.avc/index").unwrap();
  if output.contains(&String::from(arg)) {
    return true;
  }
  return false
}

/// reads filenames from a binary file
pub fn read_from_bin_file(arg : &str) -> io::Result<Vec<String>> {
  let file = File::open(arg)?;
  let mut reader = BufReader::new(file);
  let mut filenames = Vec::new();

  loop {
      let mut length_buf = [0; 4];
      if reader.read_exact(&mut length_buf).is_err() {
          break;
      }
      let length = u32::from_le_bytes(length_buf) as usize;

      let mut filename_buf = vec![0; length];
      reader.read_exact(&mut filename_buf)?;

      let filename = String::from_utf8(filename_buf).expect("Invalid UTF-8 sequence");
      filenames.push(filename);
  }
  Ok(filenames)
}


/// writes to a binary file
pub fn append_to_bin_file(filename: &str, file_path: &str) -> io::Result<()> {
  let file = fs::OpenOptions::new().append(true).create(true).open(file_path)?;
  let mut writer = BufWriter::new(file);

  let length = filename.len() as u32;
  writer.write_all(&length.to_le_bytes())?;
  writer.write_all(filename.as_bytes())?;

  Ok(())
}

/// adds a file to be tracked
pub fn add_file_tracked(arg : &str) {
  check_file_tracked(arg);
  match append_to_bin_file(arg, "./.avc/index") {
    Ok(_) => (),
    Err(e) => {
      eprintln!("error when writing to file: {}", e);
      std::process::exit(1);
    },
  }

}

pub fn diff_file(arg : &str, a : io::Result<String>) {
  check_file_tracked(arg);

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