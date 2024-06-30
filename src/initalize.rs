use std::fs;
use std::io;


// initalize a repository
pub fn initalize() {
  match fs::create_dir_all("./.avc/"){
  Ok(_) => println!("repository created"),
  Err(e) => println!("failed to create repository: {}", e),
  }

  return;
  

}