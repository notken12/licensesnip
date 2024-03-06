// license.rs
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

use crate::config::FileTypeConfig;
use ignore::DirEntry;
use mktemp::Temp;
use std::{
    fs,
    fs::File,
    io::Write,
    io::{self, BufReader, Read},
    path::Path,
    str::Bytes,
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
    // Copy file permissions
    let src_metadata = src.metadata()?;
    tmp.set_permissions(src_metadata.permissions())?;
    // Write the data to prepend
    tmp.write_all(data)?;
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
    // Copy file permissions
    let src_metadata = src.metadata()?;
    tmp.set_permissions(src_metadata.permissions())?;

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
        raw_text: &str,
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
            vec.push(License::get_formatted(str, file_name, year))
        }
        vec
    }

    pub fn get_lines(&self) -> Vec<&str> {
        self.raw_text.split('\n').collect::<Vec<&str>>()
    }

    pub fn get_header_text(lines: &Vec<String>, cfg: &FileTypeConfig) -> String {
        let mut text = String::new();

        if !&cfg.before_block.is_empty() {
            text.push_str(&cfg.before_block);
            text.push('\n');
        }

        let mut first = true;

        for line in lines {
            if !first {
                text.push('\n');
            }
            first = false;

            if line.trim().is_empty() {
                let untrimmed = format!("{}{}", cfg.before_line.clone(), cfg.after_line.clone());
                let line = untrimmed.trim_end();
                text.push_str(line);
            } else {
                let mut line_text = cfg.before_line.clone();
                line_text.push_str(line);

                text.push_str(line_text.trim_end());
            }
        }

        if !&cfg.after_block.is_empty() {
            text.push('\n');
            text.push_str(&cfg.after_block);
        }

        text
    }

    pub fn check_file(ent: &DirEntry, header_text: &str) -> Result<bool, AddToFileErr> {
        let path = ent.path();
        let file_text = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return Err(AddToFileErr::ReadFileErr),
        };

        let mut h_bytes = header_text.bytes();
        let mut f_bytes = file_text.bytes();

        let f_match = file_has_matching_header(&mut h_bytes, &mut f_bytes);
        Ok(f_match.matching)
    }

    pub fn add_to_file(ent: &DirEntry, header_text: &str) -> Result<AddToFileResult, AddToFileErr> {
        if !Self::check_file(ent, header_text)? {
            let path = ent.path();
            let mut text_to_add = header_text.to_owned();
            text_to_add.push_str("\n\n");

            // add to top of file
            match prepend_file(text_to_add.as_bytes(), path) {
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
        header_text: &str,
    ) -> Result<RemoveFromFileResult, RemoveFromFileErr> {
        let path = ent.path();
        let file_text = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return Err(RemoveFromFileErr::ReadFileErr),
        };

        let mut h_bytes = header_text.bytes();
        let mut f_bytes = file_text.bytes();

        let f_match = file_has_matching_header(&mut h_bytes, &mut f_bytes);
        let should_remove = f_match.matching;

        let mut r_count: u32 = f_match.header_len;

        if !should_remove {
            return Ok(RemoveFromFileResult::NoChange);
        }

        // Also remove trailing newlines
        for f_byte in f_bytes {
            if f_byte != 13 && f_byte != 10 {
                break;
            }

            r_count += 1;
        }

        // remove from top of file
        match remove_first_chars(r_count, path) {
            Ok(_) => Ok(RemoveFromFileResult::Removed),
            Err(e) => {
                println!("{}", e);
                Err(RemoveFromFileErr::WriteFileErr)
            }
        }
    }
}

struct MatchingHeaderResult {
    matching: bool,
    header_len: u32,
}

fn file_has_matching_header(h_bytes: &mut Bytes, f_bytes: &mut Bytes) -> MatchingHeaderResult {
    let mut has = true;
    let mut f_header_len = 0;

    while let Some(h_byte) = h_bytes.next() {
        let f_byte = match f_bytes.next() {
            Some(b) => b,
            None => {
                // Reached the end of file
                has = false;
                break;
            }
        };
        if f_byte != h_byte {
            // Check if its a line-ending problem
            // LF vs CRLF
            // Skip CR, go to LF
            if f_byte == 13 && h_byte == 10 {
                let _ = f_bytes.next();
                f_header_len += 2;
                continue;
            } else if f_byte == 10 && h_byte == 13 {
                let _ = h_bytes.next();
                continue;
            }
            has = false;
            break;
        }
        f_header_len += 1;
    }

    MatchingHeaderResult {
        matching: has,
        header_len: f_header_len,
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
        Err(_) => Err(ReadLicenseErr::FileReadErr),
    }
}
