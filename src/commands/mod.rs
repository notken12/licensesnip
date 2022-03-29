pub mod config;
pub mod default;

use clap::{Parser, Subcommand};

// Note: this requires the `derive` feature
#[derive(Parser)]
#[clap(name = "licensesnip")]
#[clap(bin_name = "licensesnip")]
pub struct Cli {
  // Whether to display extra detailed output
    #[clap(short, long)]
    pub verbose: bool,
  
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Get path to config file
  #[clap(arg_required_else_help = false)]
Config {
    /// Get path of directory's local config
    #[clap(short, long)]
    directory: bool
},
}