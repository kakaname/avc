use std::fs;
use std::io;



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