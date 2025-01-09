// mod.rs
//
// MIT License
//
// Copyright (c) 2025 Ken Zhou
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

pub mod check;
pub mod config;
pub mod default;
pub mod remove;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

// Note: this requires the `derive` feature
#[derive(Parser)]
#[clap(name = "licensesnip")]
#[clap(bin_name = "licensesnip")]
pub struct Cli {
    /// The file(s) to add the license header to
    pub file: Option<PathBuf>,
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
        directory: bool,
    },
    /// Remove all license headers from directory files
    #[clap(arg_required_else_help = false)]
    Remove {
        /// The file(s) to remove the license header to
        file: Option<PathBuf>,
        /// Display more information
        #[clap(short, long)]
        verbose: bool,
    },
    /// Check if license header exists in files
    #[clap(arg_required_else_help = false)]
    Check {
        /// The file(s) to check
        file: Option<PathBuf>,
        /// Display more information
        #[clap(short, long)]
        verbose: bool,
    },
}
