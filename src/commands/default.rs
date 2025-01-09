// default.rs
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

use std::path::PathBuf;

use chrono;
use chrono::Datelike;

use crate::frontend::{f_load_config, f_read_license, FileData, FileWalk};
use crate::license::{AddToFileResult, License};

use colored::*;

use super::Cli;

pub fn execute(args: Cli) {
    let verbose = args.verbose;
    let file = args.file.unwrap_or(PathBuf::from("."));
    let config = f_load_config();
    let license = f_read_license();

    let mut changed_files_count: u32 = 0;

    let year = chrono::Utc::now().date().year();

    let mut walk = FileWalk::new(file, config, license, year, verbose);

    for file_data in &mut walk {
        let FileData {
            header_text,
            formatted_license_lines: _,
            entry,
            file_type_config,
        } = file_data;

        match License::add_to_file(&entry, &file_type_config, &header_text) {
            Ok(r) => {
                match r {
                    AddToFileResult::Added => {
                        if verbose {
                            println!("(ok) Added license header - {}", entry.path().display());
                        }
                        changed_files_count += 1;
                    }
                    AddToFileResult::NoChange => {
                        if verbose {
                            println!(
                                "(skipped) Header already present - {}",
                                entry.path().display()
                            );
                        }
                    }
                };
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }

    let status_str = format!("✔ Added license header to {} files.", changed_files_count);
    let status_str_colored = status_str.green();

    println!("{}", status_str_colored);

    if walk.matched_filetypes_count == 0 {
        let warning = format!("{}\n\n{}\n\n{}", "⚠ No supported file types were found. You may need to add styling rules for your filetypes in your user/local config file. Run".yellow(), "licensesnip help", "for more info.".yellow());

        println!("{}", warning);
    }

    std::process::exit(exitcode::OK);
}
