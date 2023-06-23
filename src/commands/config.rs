// config.rs
//
// MIT License
//
// Copyright (c) 2023 Ken Zhou
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::config;

use super::Commands;

pub fn execute(args: Commands) {
    let directory = match args {
        Commands::Config { directory } => directory,
        _ => panic!("Wrong command type"),
    };
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
    } else if let Ok(path) = crate::config::user_config_path() {
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
