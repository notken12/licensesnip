// config.rs copyright 2022
// balh blah blah

// mog

use crate::config;

pub fn execute(directory: bool) {
    if directory {
        if let Ok(cwd) = std::env::current_dir() {
            // Create a default config if it doesn't exist already
            let path = cwd.join(config::CFG_PATH);

            if let Ok(_config) = config::PartialConfig::from_path(&path, true) {
                println!("Directory config path: \n{}", path.display());
                std::process::exit(exitcode::OK);
            } else {
                println!("Error: Failed to directory user config path.");
                std::process::exit(exitcode::IOERR);
            }
        }
    } else {
        if let Ok(path) = crate::config::user_config_path() {
            // Create a default config if it doesn't exist already

            if let Ok(_config) = config::PartialConfig::from_path(&path, true) {
                println!("User config path: \n{}", path.display());
                std::process::exit(exitcode::OK);
            } else {
                println!("Error: Failed to get user config path.");
                std::process::exit(exitcode::IOERR);
            }
        }
    }
}
