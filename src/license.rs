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
    io::{self, BufRead, BufReader, Read},
    ops::Range,
    path::Path,
};

const LICENSE_PATH: &str = ".licensesnip";

fn prepend_file(header_text: &[u8], file_path: &Path, skip_shebang_line: bool) -> io::Result<()> {
    // Create a temporary file
    let tmp = Temp::new_file()?;
    let tmp_path = tmp.to_path_buf();
    // Stop the temp file being automatically deleted when the variable
    // is dropped, by releasing it.
    tmp.release();
    // Open temp file for writing
    let mut tmp = File::create(&tmp_path)?;
    // Open source file for reading
    let mut src = File::open(file_path)?;
    // Copy file permissions
    let src_metadata = src.metadata()?;
    tmp.set_permissions(src_metadata.permissions())?;
    if skip_shebang_line {
        let mut shebang_line = String::new();
        let src = BufReader::new(&src);
        let mut lines = src.lines();
        if let Some(line) = lines.next() {
            shebang_line = line.unwrap();
        }
        shebang_line.push('\n');
        if shebang_line.starts_with("#!") {
            // Write the shebang line first.
            tmp.write_all(shebang_line.as_bytes())?;
            tmp.write_all(header_text)?;
        } else {
            tmp.write_all(header_text)?;
            // Write the first line that we've already read.
            tmp.write_all(shebang_line.as_bytes())?;
        }
        for line in lines {
            tmp.write_all(line.unwrap().as_bytes())?;
            tmp.write_all(b"\n")?;
        }
    } else {
        tmp.write_all(header_text)?;
        // Copy the rest of the source file
        io::copy(&mut src, &mut tmp)?;
    }
    drop(tmp);
    drop(src);
    fs::remove_file(file_path)?;
    fs::copy(&tmp_path, file_path)?;
    fs::remove_file(&tmp_path)?;
    Ok(())
}

fn remove_range_of_chars(omit_range: Range<u32>, file_path: &Path) -> io::Result<()> {
    // Create a temporary file
    let tmp = Temp::new_file()?;
    let tmp_path = tmp.to_path_buf();
    // Stop the temp file being automatically deleted when the variable
    // is dropped, by releasing it.
    tmp.release();
    // Open temp file for writing
    let mut tmp = File::create(&tmp_path)?;
    // Open source file for reading
    let mut src = File::open(file_path)?;
    // Copy file permissions
    let src_metadata = src.metadata()?;
    tmp.set_permissions(src_metadata.permissions())?;

    let mut i = 0;
    let mut text = Vec::<u8>::new();
    let full_text = BufReader::new(&src).bytes();

    for byte in full_text {
        match byte {
            Ok(byte) => {
                if !omit_range.contains(&i) {
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
    drop(src);
    drop(tmp);
    fs::remove_file(file_path)?;
    fs::copy(&tmp_path, file_path)?;
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

    pub fn check_file(
        ent: &DirEntry,
        file_type_config: &FileTypeConfig,
        header_text: &str,
    ) -> Result<bool, AddToFileErr> {
        let path = ent.path();
        let file_text = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return Err(AddToFileErr::ReadFileErr),
        };

        let h_bytes = header_text.as_bytes();
        let f_bytes = file_text.as_bytes();

        let matching_header =
            file_has_matching_header(h_bytes, f_bytes, file_type_config.skip_shebang_line);
        Ok(matches!(
            matching_header,
            MatchingHeaderResult::MatchingHeaderAt(_)
        ))
    }

    pub fn add_to_file(
        ent: &DirEntry,
        file_type_config: &FileTypeConfig,
        header_text: &str,
    ) -> Result<AddToFileResult, AddToFileErr> {
        if !Self::check_file(ent, file_type_config, header_text)? {
            let path = ent.path();
            let mut text_to_add = header_text.to_owned();
            text_to_add.push_str("\n\n");

            // add to top of file
            match prepend_file(
                text_to_add.as_bytes(),
                path,
                file_type_config.skip_shebang_line,
            ) {
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
        file_type_config: &FileTypeConfig,
        header_text: &str,
    ) -> Result<RemoveFromFileResult, RemoveFromFileErr> {
        let path = ent.path();
        let file_text = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return Err(RemoveFromFileErr::ReadFileErr),
        };

        let h_bytes = header_text.as_bytes();
        let f_bytes = file_text.as_bytes();

        let f_match =
            file_has_matching_header(h_bytes, f_bytes, file_type_config.skip_shebang_line);

        let MatchingHeaderResult::MatchingHeaderAt(header_range) = f_match else {
            return Ok(RemoveFromFileResult::NoChange);
        };

        // remove from top of file
        match remove_range_of_chars(header_range, path) {
            Ok(_) => Ok(RemoveFromFileResult::Removed),
            Err(e) => {
                println!("{}", e);
                Err(RemoveFromFileErr::WriteFileErr)
            }
        }
    }
}

#[derive(Debug)]
enum MatchingHeaderResult {
    MatchingHeaderAt(Range<u32>),
    NotMatching,
}

fn file_has_matching_header(
    header: &[u8],
    mut file: &[u8],
    skip_shebang_line: bool,
) -> MatchingHeaderResult {
    let mut header_start: u32 = 0;

    if skip_shebang_line && file.len() >= 2 && &file[0..2] == b"#!" {
        // Skip the shebang line
        while !file.is_empty() {
            header_start += 1;
            let ch = file[0];
            file = &file[1..];
            if ch == b'\n' {
                break;
            }
        }
    }

    let mut header_end: u32 = header_start;
    if file.len() < header.len() || header != &file[0..header.len()] {
        return MatchingHeaderResult::NotMatching;
    }

    header_end += header.len() as u32;
    file = &file[header.len()..];

    while !file.is_empty() {
        let ch = file[0];
        if ch == b'\n' || ch == b'\r' {
            header_end += 1;
            file = &file[1..];
            continue;
        }
        break;
    }

    MatchingHeaderResult::MatchingHeaderAt(header_start..header_end)
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
        Ok(str) => Ok(License {
            raw_text: str.trim().to_string(),
        }),
        Err(_) => Err(ReadLicenseErr::FileReadErr),
    }
}
