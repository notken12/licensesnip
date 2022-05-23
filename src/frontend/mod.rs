// mod.rs
//
// MIT License
//
// Copyright (c) 2022 Ken Zhou
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

use crate::config::{load_config, Config, LoadConfigErr};

pub fn f_load_config() -> Config {
    match load_config() {
        Ok(cfg) => cfg,
        Err(e) => match e {
            LoadConfigErr::JsonFormattingErr(e) => {
                println!("Error: Your config file wasn't formatted correctly:\n {}", e);
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
