// check.rs
//
// MIT License
//
// Copyright (c) 2022 Ken Zhou
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

use chrono;
use chrono::Datelike;

use crate::frontend::f_load_config;
use crate::license::{read_license, License, ReadLicenseErr};

use ignore::Walk;

use colored::*;

pub fn execute(verbose: bool) {
    let config = f_load_config();

    let license: License;

    match read_license() {
        Ok(l) => license = l,
        Err(e) => match e {
            ReadLicenseErr::FileReadErr => {
                println!("{}", "Error: Couldn't find a .licensesnip file in the current working directory's root.".red());
                std::process::exit(exitcode::CONFIG)
            }
        },
    }

    let filetype_map = config.get_filetype_map();

    let mut checked_files_count: u32 = 0;
    let mut matched_filetypes_count: u32 = 0;

    let year = chrono::Utc::now().date().year();

    for result in Walk::new("./") {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => (|entry: ignore::DirEntry| {
                match entry.file_type() {
                    Some(t) => {
                        if !t.is_file() {
                            return;
                        }
                    }
                    None => return,
                }

                // Get file extension
                let file_name = entry.file_name().to_string_lossy();
                let ext;
                match file_name.split(".").last() {
                    Some(e) => ext = e,
                    None => {
                        if verbose {
                            println!(
                                "(skipped) Invalid file extension - {}",
                                entry.path().display()
                            )
                        }
                        return;
                    }
                }

                let filetype_cfg = match filetype_map.get(ext) {
                    Some(e) => {
                        matched_filetypes_count += 1;
                        e
                    }
                    None => {
                        // No configuration for this file type
                        if verbose {
                            println!(
                                "(skipped) No file type configuration found for .{} - {}",
                                ext,
                                entry.path().display()
                            );
                        }

                        return;
                    }
                };

                if !filetype_cfg.enable {
                    // Disabled for this filetype
                    if verbose {
                        println!(
                            "(skipped) Inserting header is disabled for .{} files - {}",
                            ext,
                            entry.path().display()
                        )
                    }
                    return;
                }

                let raw_lines = license.get_lines();

                let f_lines = License::get_formatted_lines(&raw_lines, &file_name, year);

                let header_text = License::get_header_text(&f_lines, filetype_cfg);

                match License::check_file(&entry, &header_text) {
                    Ok(r) => {
                        if r {
                            if verbose {
                                println!(
                                    "(ok) License header present - {}",
                                    entry.path().display()
                                );
                            }
                            checked_files_count += 1;
                        } else {
                            println!("(err) License header missing - {}. \nDid you forget to run `licensesnip`?", entry.path().display());
                            std::process::exit(1);
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e)
                    }
                }
            })(entry),
            Err(err) => println!("ERROR: {}", err),
        }
    }

    let status_str = format!(
        "✔ License header present in all {} files.",
        checked_files_count
    );
    let status_str_colored = status_str.green();

    println!("{}", status_str_colored);

    if matched_filetypes_count == 0 {
        let warning = format!("{}\n\n{}\n\n{}", "⚠ No supported file types were found. You may need to add styling rules for your filetypes in your user/local config file. Run".yellow(), "licensesnip help", "for more info.".yellow());

        println!("{}", warning);
    }

    std::process::exit(exitcode::OK);
}
