


/// raises an error and quits
/// This function also serves to log all the data before it exits / though it isn't implemented yet
pub fn raise_error(arg : &str) -> ! {
  // this function should also include the ability to log
  eprintln!("ERROR : {}", arg);
  std::process::exit(1);
}