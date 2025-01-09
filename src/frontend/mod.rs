// mod.rs
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

use std::{collections::HashMap, path::PathBuf};

use colored::Colorize;
use ignore::{Walk, WalkBuilder};

use crate::{
    config::{load_config, Config, FileTypeConfig, LoadConfigErr},
    license::{read_license, License, ReadLicenseErr},
};

pub fn f_load_config() -> Config {
    match load_config() {
        Ok(cfg) => cfg,
        Err(e) => match e {
            LoadConfigErr::JsonFormattingErr(e) => {
                println!(
                    "Error: Your config file wasn't formatted correctly:\n {}",
                    e
                );
                std::process::exit(exitcode::CONFIG);
            }
            LoadConfigErr::CreateDefaultConfigErr => {
                println!("Error: Failed to create default config file.");
                std::process::exit(exitcode::IOERR)
            }
            LoadConfigErr::LoadUserConfigErr => {
                println!("Error: failed to load user config file.");
                std::process::exit(exitcode::IOERR)
            }
            LoadConfigErr::NotFoundErr => std::process::exit(exitcode::IOERR),
        },
    }
}

pub fn f_read_license() -> License {
    match read_license() {
        Ok(l) => l,
        Err(e) => match e {
            ReadLicenseErr::FileReadErr => {
                println!("{}", "Error: Couldn't find a .licensesnip file in the current working directory's root.".red());
                std::process::exit(exitcode::CONFIG)
            }
        },
    }
}

pub struct FileWalk {
    ignore_walk: Walk,
    verbose: bool,
    filetype_map: HashMap<String, FileTypeConfig>,
    pub matched_filetypes_count: u32,
    license: License,
    year: i32,
}

impl FileWalk {
    pub fn new(path: PathBuf, config: Config, license: License, year: i32, verbose: bool) -> Self {
        let filetype_map = config.get_filetype_map();
        let mut builder = WalkBuilder::new(path);
        let walk = builder
            .git_ignore(config.use_gitignore)
            .add_custom_ignore_filename(".licensesnipignore");
        // walk.add_ignore(Path::new("./.licensesnipignore"));
        let ignore_walk = walk.build();

        Self {
            ignore_walk,
            verbose,
            filetype_map,
            license,
            year,
            matched_filetypes_count: 0,
        }
    }
}

pub struct FileData {
    pub formatted_license_lines: Vec<String>,
    pub header_text: String,
    pub entry: ignore::DirEntry,
    pub file_type_config: FileTypeConfig,
}

impl Iterator for FileWalk {
    type Item = FileData;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.ignore_walk.next() {
            match n {
                Ok(entry) => {
                    match entry.file_type() {
                        Some(t) => {
                            if !t.is_file() {
                                return self.next();
                            }
                        }
                        None => return self.next(),
                    }

                    // Get file extension
                    let file_name = entry.file_name().to_string_lossy();
                    let ext = match file_name.split('.').last() {
                        Some(e) => e,
                        None => {
                            if self.verbose {
                                println!(
                                    "(skipped) Invalid file extension - {}",
                                    entry.path().display()
                                )
                            }
                            return self.next();
                        }
                    };

                    let file_type_config = match self.filetype_map.get(ext) {
                        Some(e) => {
                            self.matched_filetypes_count += 1;
                            e
                        }
                        None => {
                            // No configuration for this file type
                            if self.verbose {
                                println!(
                                    "(skipped) No file type configuration found for .{} - {}",
                                    ext,
                                    entry.path().display()
                                );
                            }

                            return self.next();
                        }
                    };

                    if !file_type_config.enable {
                        // Disabled for this filetype
                        if self.verbose {
                            println!(
                                "(skipped) Inserting header is disabled for .{} files - {}",
                                ext,
                                entry.path().display()
                            )
                        }
                        return self.next();
                    }

                    let raw_license_lines = self.license.get_lines();

                    let formatted_license_lines =
                        License::get_formatted_lines(&raw_license_lines, &file_name, self.year);

                    let header_text =
                        License::get_header_text(&formatted_license_lines, file_type_config);

                    return Some(FileData {
                        formatted_license_lines,
                        header_text,
                        entry,
                        file_type_config: file_type_config.clone(),
                    });
                }
                Err(err) => {
                    println!("ERROR: {}", err);
                    return self.next();
                }
            }
        }
        None
    }
}
