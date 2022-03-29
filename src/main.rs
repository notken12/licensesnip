// main.rs copyright 2022 
// balh blah blah
// 
// mog

// main.rs copyright 2022 
// balh blah blah

// main.rs copyright 2022
// balh blah blah

mod config;
mod license;

use config::{load_config, Config, LoadConfigErr};
use license::{read_license, License, ReadLicenseErr};

use ignore::Walk;

use colored::*;

use crate::license::AddToFileResult;
fn main() {
    let config: Config;
    match load_config() {
        Ok(cfg) => config = cfg,
        Err(e) => match e {
            LoadConfigErr::JsonFormattingErr => {
                println!("Error: Your config file wasn't formatted correctly.");
                std::process::exit(exitcode::CONFIG);
            }
            LoadConfigErr::CouldntCreateDefaultConfig => {
                println!("Error: Failed to create default config file.");
                std::process::exit(exitcode::IOERR)
            }
        },
    };

    let license: License;

    match read_license() {
        Ok(l) => license = l,
        Err(e) => match e {
            ReadLicenseErr::FileReadErr => {
                println!("Error: Couldn't find a .licensesnip file in the current working directory's root.");
                std::process::exit(exitcode::CONFIG)
            }
        },
    }

    println!("License raw text: \n{}", license.raw_text);

    let filetype_map = config.get_filetype_map();
    let mut changed_files_count: u32 = 0;

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

                let file_name = entry.file_name().to_string_lossy();
                let ext;
                match file_name.split(".").last() {
                    Some(e) => ext = e,
                    None => return,
                }

                let filetype_cfg = match filetype_map.get(ext) {
                    Some(e) => e,
                    None => {
                        // No configuration for this file type
                        return;
                    }
                };

                let raw_lines = license.get_lines();

                let f_lines = License::get_formatted_lines(&raw_lines, &file_name, 2022);

                let header_text = License::get_header_text(&f_lines, filetype_cfg);
                println!("{}", header_text);

                match License::add_to_file(&entry, &header_text) {
                    Ok(r) => {
                        match r {
                            AddToFileResult::Added => {
                                changed_files_count += 1;
                            }
                            _ => {}
                        };
                    }
                    Err(e) => {
                        println!("{:?}", e)
                    }
                }
            })(entry),
            Err(err) => println!("ERROR: {}", err),
        }
    }

    let status_str = format!("✔ Added license header to {} files.", changed_files_count);
    let status_str_colored = status_str.green();

    println!("{}", status_str_colored);

    std::process::exit(exitcode::OK);
}
