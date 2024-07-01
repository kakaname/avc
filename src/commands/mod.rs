use crate::macros::{append_to_bin_file, check_dir_exists, check_file_exists, compute_sha1_hash, create_directory, create_file, diff_file, read_from_bin_file};




pub fn print_help() {
    println!("Usage : avc <argument>");

}

///avc status [options]
///currently detects changes to files but is inefficent in its detection mechanism, should be changed
pub fn status() {
  


}

/// list all files that are being tracked
pub fn list_targets() {
  let bin_content = read_from_bin_file("./.avc/index").unwrap();
  println!("Tracking Files: \n");
  for filename in bin_content {
    println!("{}\n", filename);
  }

}


pub fn begin_tracking(arg : &str) {
  if !check_file_exists(arg) {
    eprintln!("file does not exist");
    std::process::exit(1);
  }
  let a  = compute_sha1_hash(arg);
  match append_to_bin_file(arg, "./.avc/index") {
    Ok(_) => (),
    Err(e) => eprintln!("Error was found when beginning to track file: {}", e),
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
  create_file("./.avc/index", "");

  

}