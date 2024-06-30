
fn create_directory(path: &str, success_message: &str) {
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