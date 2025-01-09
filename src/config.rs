// config.rs
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

use directories::ProjectDirs;

use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

pub static DEFAULT_CONFIG: &str = include_str!("default-config.jsonc");
pub static BASE_CONFIG: &str = include_str!("base-config.jsonc");
pub const CFG_PATH: &str = "licensesnip.config.jsonc";

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
    #[serde(default = "get_true")]
    pub enable: bool,
    #[serde(default = "get_true")]
    pub skip_shebang_line: bool,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub use_gitignore: bool,
    pub file_types: HashMap<String, FileTypeConfig>,
}

impl Config {
    pub fn get_filetype_map(&self) -> HashMap<String, FileTypeConfig> {
        let mut map = HashMap::<String, FileTypeConfig>::new();
        for (types, config) in &self.file_types {
            let split = types.split(',');
            for extension in split {
                map.insert(extension.to_string(), config.clone());
            }
        }

        map
    }

    pub fn assign_partial(target: &Self, source: &PartialConfig) -> Self {
        let mut new = target.clone();

        if let Some(use_gitignore) = source.use_gitignore {
            new.use_gitignore = use_gitignore;
        }

        if let Some(file_types) = &source.file_types {
            for (filetypes, cfg) in file_types {
                new.file_types.insert(filetypes.to_string(), cfg.clone());
            }
        }

        new
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            use_gitignore: true,
            file_types: HashMap::<String, FileTypeConfig>::new(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PartialConfig {
    pub use_gitignore: Option<bool>,
    pub file_types: Option<HashMap<String, FileTypeConfig>>,
}

impl PartialConfig {
    pub fn base() -> Result<Self, LoadConfigErr> {
        match serde_json::from_str(BASE_CONFIG) {
            Ok(config) => Ok(config),
            Err(e) => Err(LoadConfigErr::JsonFormattingErr(e)),
        }
    }

    pub fn from_path(path: &Path, create_default: bool) -> Result<Self, LoadConfigErr> {
        let file_text: String;
        let read_result = fs::read_to_string(path);

        match read_result {
            Ok(str) => file_text = str,
            Err(_) => {
                if create_default {
                    match create_default_config(path) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                            return Err(LoadConfigErr::CreateDefaultConfigErr);
                        }
                    }
                    file_text = String::from(DEFAULT_CONFIG)
                } else {
                    return Err(LoadConfigErr::NotFoundErr);
                }
            }
        }

        match serde_json::from_str(&file_text) {
            Ok(config) => Ok(config),
            Err(e) => Err(LoadConfigErr::JsonFormattingErr(e)),
        }
    }

    pub fn assign(target: &Self, source: &Self) -> Self {
        let mut new = target.clone();

        if let Some(use_gitignore) = source.use_gitignore {
            new.use_gitignore = Some(use_gitignore);
        }

        if let Some(file_types) = &source.file_types {
            for (filetypes, cfg) in file_types {
                if let Some(f) = &mut new.file_types {
                    f.insert(filetypes.to_string(), cfg.clone());
                }
            }
        }

        new
    }
}

fn get_true() -> bool {
    true
}

pub enum LoadConfigErr {
    JsonFormattingErr(serde_json::Error),
    CreateDefaultConfigErr,
    LoadUserConfigErr,
    NotFoundErr,
}

pub fn load_config() -> Result<Config, LoadConfigErr> {
    let config_path = match user_config_path() {
        Ok(d) => d,
        Err(_) => return Err(LoadConfigErr::LoadUserConfigErr),
    };

    let user_config = PartialConfig::from_path(&config_path, true)?;

    let cwd_config = match PartialConfig::from_path(Path::new(CFG_PATH), false) {
        Ok(c) => Some(c),
        Err(e) => match e {
            LoadConfigErr::JsonFormattingErr(e) => return Err(LoadConfigErr::JsonFormattingErr(e)),
            _ => None,
        },
    };

    let base = match PartialConfig::base() {
        Ok(d) => Config::assign_partial(&Config::default(), &d),
        Err(e) => return Err(e),
    };

    let mut assigned = user_config.clone();

    if let Some(c) = cwd_config {
        assigned = PartialConfig::assign(&user_config, &c);
    }

    Ok(Config::assign_partial(&base, &assigned))
}

#[derive(Debug)]
enum CreateDefaultConfigErr {
    MissingPathParentErr,
    #[allow(dead_code)]
    IoErr(std::io::Error),
}

fn create_default_config(path: &Path) -> Result<(), CreateDefaultConfigErr> {
    let dir = match path.parent() {
        Some(p) => p,
        None => return Err(CreateDefaultConfigErr::MissingPathParentErr),
    };
    match fs::create_dir_all(dir) {
        Ok(_) => {}
        Err(e) => return Err(CreateDefaultConfigErr::IoErr(e)),
    };
    match fs::write(path, DEFAULT_CONFIG) {
        Ok(_) => Ok(()),
        Err(e) => Err(CreateDefaultConfigErr::IoErr(e)),
    }
}

#[derive(Debug)]
pub struct NoConfigDirErr;

impl fmt::Display for NoConfigDirErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Couldn't get user's config path")
    }
}

impl std::error::Error for NoConfigDirErr {}

pub fn user_config_path() -> Result<PathBuf, NoConfigDirErr> {
    let proj_dirs;

    let config_dir = if let Some(p) = ProjectDirs::from("io", "notken12", "licensesnip") {
        proj_dirs = p;
        proj_dirs.config_dir()
        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    } else {
        return Err(NoConfigDirErr);
    };

    Ok(config_dir.join(CFG_PATH))
}
