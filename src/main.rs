// use std::ffi::OsString;
// use std::path::PathBuf;
pub mod config;
pub mod license;

mod commands;
use clap::{Parser};


use commands::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    if let Some(command) = &args.command {
      match command {
        Commands::Config {directory} => {
          commands::config::execute(*directory) 
        }
      };
    } else {
      commands::default::execute();
    }
  
    
}
