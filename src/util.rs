// Copyright 2022 Pranav Karawale
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    env,
    path::{Path, PathBuf},
};

/// Returns the path to the system config directory.
pub fn get_config_dir() -> PathBuf {
    match env::consts::OS.to_string().as_str() {
        "windows" => Path::new(&env::var("APPDATA").unwrap()).to_path_buf(),
        "macos" => Path::new(&env::var("HOME").unwrap())
            .join("Library")
            .join("Application Support"),
        "linux" => Path::new(&env::var("HOME").unwrap()).join(".config"),
        _ => Path::new(&env::var("HOME").unwrap()).join(".config"),
    }
}

/// Prints the given error message and exits the program.
pub fn print_and_exit(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}
