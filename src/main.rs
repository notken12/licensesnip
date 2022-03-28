mod config;
mod license;

use config::{load_config, Config, LoadConfigErr};
use license::{read_license, ReadLicenseErr, License};

use ignore::Walk;

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
    }
  }

  println!("License raw text: {}", license.raw_text);

  let filetype_map = config.get_filetype_map();
  
    for result in Walk::new("./") {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => (|entry: ignore::DirEntry|{
              if !entry.file_type().unwrap().is_file() {return};
              let file_name = entry.file_name();

              let formatted = license.get_formatted(&file_name.to_str().unwrap(), 2022);
              println!("{}", formatted)
            })(entry),
            Err(err) => println!("ERROR: {}", err),
        }
    }

    std::process::exit(exitcode::OK);
}
