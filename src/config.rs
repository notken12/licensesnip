// config.rs copyright 2022 
// balh blah blah
// 
// mog

// config.rs copyright 2022 
// balh blah blah

// config.rs copyright 2022
// balh blah bla

// config.rs copyright 2022
// balh blh dwblah

use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fs;

static DEFAULT_CONFIG: &'static str = include_str!("default-config.json");
const CFG_PATH: &str = "licensesnip.config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileTypeConfig {
    #[serde(default = "String::new")]
    pub before_block: String,
    #[serde(default = "String::new")]
    pub after_block: String,
    #[serde(default = "String::new")]
    pub before_line: String,
    #[serde(default = "String::new")]
    pub after_line: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "get_true")]
    pub use_gitignore: bool,
    #[serde(default = "HashMap::new")]
    pub file_types: HashMap<String, FileTypeConfig>,
}

impl Config {
    pub fn get_filetype_map(&self) -> HashMap<String, FileTypeConfig> {
        let mut map = HashMap::<String, FileTypeConfig>::new();
        for (types, config) in &self.file_types {
            let split = types.split(",");
            for extension in split {
                map.insert(extension.to_string(), config.clone());
            }
        }
        map
    }
}

fn get_true() -> bool {
    true
}

pub enum LoadConfigErr {
    JsonFormattingErr,
    CouldntCreateDefaultConfig,
}

pub fn load_config() -> Result<Config, LoadConfigErr> {
    let file_text: String;
    let read_result = fs::read_to_string(CFG_PATH);

    match read_result {
        Ok(str) => file_text = str,
        Err(_) => {
            match create_default_config() {
                Ok(_) => {}
                Err(_) => return Err(LoadConfigErr::CouldntCreateDefaultConfig),
            }
            file_text = String::from(DEFAULT_CONFIG)
        }
    }

    match serde_json::from_str(&file_text) {
        Ok(config) => return Ok(config),
        Err(_) => return Err(LoadConfigErr::JsonFormattingErr),
    }
}

fn create_default_config() -> Result<(), std::io::Error> {
    fs::write(CFG_PATH, DEFAULT_CONFIG)
}
