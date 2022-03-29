use directories::ProjectDirs;

use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
    #[serde(default= "get_true")]
    pub enable: bool
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
                let split = types.split(",");
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

  pub fn default() -> Self {
    Self {
      use_gitignore: true,
      file_types: HashMap::<String, FileTypeConfig>::new()
    }
  }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PartialConfig {
    pub use_gitignore: Option<bool>,
    pub file_types: Option<HashMap<String, FileTypeConfig>>,
}

impl PartialConfig {
  pub fn default() -> Result<Self, LoadConfigErr> {
      match serde_json::from_str(&DEFAULT_CONFIG) {
            Ok(config) => return Ok(config),
            Err(_) => return Err(LoadConfigErr::JsonFormattingErr),
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
                            println!("{}", e);
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
            Ok(config) => return Ok(config),
            Err(_) => return Err(LoadConfigErr::JsonFormattingErr),
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
    JsonFormattingErr,
    CreateDefaultConfigErr,
    LoadUserConfigErr,
    NotFoundErr,
}

pub fn load_config() -> Result<Config, LoadConfigErr> {
    let config_dir;

    let proj_dirs;

    if let Some(p) = ProjectDirs::from("io", "notken12", "licensesnip") {
        proj_dirs = p;
        config_dir = proj_dirs.config_dir()
        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    } else {
        return Err(LoadConfigErr::LoadUserConfigErr);
    };

    println!("{}", config_dir.join(CFG_PATH).display());

    let user_config = match PartialConfig::from_path(&config_dir.join(CFG_PATH), true) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let cwd_config = match PartialConfig::from_path(&Path::new(CFG_PATH), false) {
        Ok(c) => Some(c),
        Err(e) => match e {
            LoadConfigErr::JsonFormattingErr => return Err(e),
            _ => None,
        },
    };
  
    let default;
    
    match PartialConfig::default() {
      Ok(d) => default = Config::assign_partial(&Config::default(), &d),
      Err(e) => return Err(e)
    };
  
    let mut assigned = user_config.clone();

    match cwd_config {
        Some(c) => {
            assigned = PartialConfig::assign(&user_config, &c);
        }
        None => {}
    };


    Ok(Config::assign_partial(&default, &assigned))
}

fn create_default_config(path: &Path) -> Result<(), std::io::Error> {
    println!("creating default config at {}", path.display());
    fs::write(path, DEFAULT_CONFIG)
}
