pub fn execute(directory: bool) {
  if directory {
    if let Ok(cwd) = std::env::current_dir() {
      let path = cwd.join(crate::config::CFG_PATH);
    println!("Directory config path: \n{}", path.display());
    }
  } 
  else {
    if let Ok(path) = crate::config::user_config_path() {
      println!("User config path: \n{}", path.display())
    }
  }
}

