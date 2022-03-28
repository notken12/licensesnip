use std::fs;

const LICENSE_PATH: &str = ".licensesnip";

pub enum ReadLicenseErr {
  FileReadErr
}

pub struct License {
  pub raw_text: String
}

impl License {
  pub fn get_formatted(raw_text: &String, file_name: &str, year: u32) -> String {
     raw_text.replace("%FILENAME%", file_name).replace("%YEAR%", &year.to_string())
  }

  pub fn get_formatted_lines(lines: &Vec<String>) -> Vec<String> {
    let vec = Vec::<String>::new();
    for str in lines {
      vec.push(License.get_formatted(&str))
    }
  }

  pub fn get_lines(&self) -> Vec<String> {
    self.raw_text.split("\n").into_iter().collect()
  }
}

pub fn read_license() -> Result<License, ReadLicenseErr> {
  let read_result = fs::read_to_string(LICENSE_PATH);

  match read_result {
    Ok(str) => return Ok(License {
      raw_text: str
    }),
    Err(_) => return Err(ReadLicenseErr::FileReadErr)
  }
}