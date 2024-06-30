use crate::macros::{create_directory, check_dir_exists};



pub fn print_help() {
    println!("Usage : avc <argument>");

}

///avc status [options]
///currently detects changes to files but is inefficent in its detection mechanism, should be changed
pub fn status() {
  


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

  

}