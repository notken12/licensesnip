// license.rs

// MIT License

// Copyright (c) 2022 Ken Zhou

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::config::FileTypeConfig;
use ignore::DirEntry;
use mktemp::Temp;
use std::{
    fs,
    fs::File,
    io::Write,
    io::{self, BufReader, Read},
    path::Path,
};

const LICENSE_PATH: &str = ".licensesnip";

fn prepend_file(data: &[u8], file_path: &Path) -> io::Result<()> {
    // Create a temporary file
    let tmp = Temp::new_file()?;
    let tmp_path = tmp.to_path_buf();
    // Stop the temp file being automatically deleted when the variable
    // is dropped, by releasing it.
    tmp.release();
    // Open temp file for writing
    let mut tmp = File::create(&tmp_path)?;
    // Open source file for reading
    let mut src = File::open(&file_path)?;
    // Write the data to prepend
    tmp.write_all(&data)?;
    // Copy the rest of the source file
    io::copy(&mut src, &mut tmp)?;
    fs::remove_file(&file_path)?;
    fs::copy(&tmp_path, &file_path)?;
    fs::remove_file(&tmp_path)?;
    Ok(())
}

fn remove_first_chars(count: u32, file_path: &Path) -> io::Result<()> {
    // Create a temporary file
    let tmp = Temp::new_file()?;
    let tmp_path = tmp.to_path_buf();
    // Stop the temp file being automatically deleted when the variable
    // is dropped, by releasing it.
    tmp.release();
    // Open temp file for writing
    let mut tmp = File::create(&tmp_path)?;
    // Open source file for reading
    let mut src = File::open(&file_path)?;

    let mut i = 0;
    let mut text = Vec::<u8>::new();
    let full_text = BufReader::new(&src).bytes();

    for byte in full_text {
        match byte {
            Ok(byte) => {
                if i >= count {
                    text.push(byte)
                }
                i += 1;
            }
            Err(e) => {
                println!("{}", e);
                std::process::exit(exitcode::IOERR);
            }
        }
    }

    // Write the data to prepend
    tmp.write_all(text.as_slice())?;
    // Copy the rest of the source file
    io::copy(&mut src, &mut tmp)?;
    fs::remove_file(&file_path)?;
    fs::copy(&tmp_path, &file_path)?;
    fs::remove_file(&tmp_path)?;
    Ok(())
}

pub enum ReadLicenseErr {
    FileReadErr,
}

pub struct License {
    pub raw_text: String,
}

impl License {
    pub fn get_formatted<Y: std::fmt::Display + Copy>(
        raw_text: &String,
        file_name: &str,
        year: Y,
    ) -> String {
        raw_text
            .replace("%FILENAME%", file_name)
            .replace("%YEAR%", &year.to_string())
    }

    pub fn get_formatted_lines<Y: std::fmt::Display + Copy>(
        lines: &Vec<&str>,
        file_name: &str,
        year: Y,
    ) -> Vec<String> {
        let mut vec = Vec::<String>::new();
        for str in lines {
            vec.push(License::get_formatted(&str.to_string(), file_name, year))
        }
        vec
    }

    pub fn get_lines(&self) -> Vec<&str> {
        self.raw_text.split("\n").collect::<Vec<&str>>()
    }

    pub fn get_header_text(lines: &Vec<String>, cfg: &FileTypeConfig) -> String {
        let mut text = String::new();
        text.push_str(&cfg.before_block);

        let mut first = true;

        for line in lines {
            if line.trim().is_empty() {
                text.push('\n')
            } else {
                if !first {
                    text.push('\n');
                }
                let mut line_text = cfg.before_line.clone();
                line_text.push_str(line);

                text.push_str(&line_text);

                first = false;
            }
        }

        text.push_str(&cfg.after_block);

        text
    }

    pub fn add_to_file(
        ent: &DirEntry,
        header_text: &String,
    ) -> Result<AddToFileResult, AddToFileErr> {
        let path = ent.path();
        let file_text = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return Err(AddToFileErr::ReadFileErr),
        };

        let mut f_bytes = file_text.bytes();

        let mut should_add = false;

        for h_byte in header_text.bytes() {
            let f_byte = match f_bytes.next() {
                Some(c) => c,
                None => {
                    // Reached the end of file
                    should_add = true;
                    break;
                }
            };
            if f_byte != h_byte {
                should_add = true;
                break;
            }
        }

        if should_add {
            let mut text_to_add = header_text.clone();
            text_to_add.push_str("\n\n");

            // add to top of file
            match prepend_file(text_to_add.as_bytes(), &path) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    return Err(AddToFileErr::WriteFileErr);
                }
            };

            return Ok(AddToFileResult::Added);
        }

        Ok(AddToFileResult::NoChange)
    }

    pub fn remove_from_file(
        ent: &DirEntry,
        header_text: &String,
    ) -> Result<RemoveFromFileResult, RemoveFromFileErr> {
        let path = ent.path();
        let file_text = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return Err(RemoveFromFileErr::ReadFileErr),
        };

        let mut f_bytes = file_text.bytes();

        let mut should_remove = true;
        let mut r_count: u32 = 0;

        // Remove if the top of the file matches the header text
        for h_byte in header_text.bytes() {
            let f_byte = match f_bytes.next() {
                Some(c) => c,
                None => {
                    // Reached the end of file
                    should_remove = false;
                    break;
                }
            };
            if f_byte != h_byte {
                should_remove = false;
                break;
            }
            r_count += 1;
        }

        if !should_remove {
            return Ok(RemoveFromFileResult::NoChange);
        }

        // Also remove trailing newlines
        loop {
            let f_byte = match f_bytes.next() {
                Some(c) => c,
                None => {
                    // Reached the end of file
                    break;
                }
            };

            if f_byte != b'\n' {
                break;
            }

            r_count += 1;
        }

        // add to top of file
        match remove_first_chars(r_count, &path) {
            Ok(_) => return Ok(RemoveFromFileResult::Removed),
            Err(e) => {
                println!("{}", e);
                return Err(RemoveFromFileErr::WriteFileErr);
            }
        }
    }
}

pub enum AddToFileResult {
    Added,
    NoChange,
}

pub enum RemoveFromFileResult {
    Removed,
    NoChange,
}

#[derive(Debug)]
pub enum AddToFileErr {
    ReadFileErr,
    WriteFileErr,
}

#[derive(Debug)]
pub enum RemoveFromFileErr {
    ReadFileErr,
    WriteFileErr,
}

pub fn read_license() -> Result<License, ReadLicenseErr> {
    let read_result = fs::read_to_string(LICENSE_PATH);

    match read_result {
        Ok(str) => {
            return Ok(License {
                raw_text: str.trim().to_string(),
            })
        }
        Err(_) => return Err(ReadLicenseErr::FileReadErr),
    }
}
