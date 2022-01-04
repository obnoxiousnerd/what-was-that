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

use std::path::Path;

use clap::Parser;
use cli::Commands;

mod cli;
mod store;
mod util;

extern crate clap;
extern crate fuzzy_matcher;
extern crate serde_json;

fn main() {
    let cli = cli::Cli::parse();

    let store_path = cli.store_path.unwrap_or({
        util::get_config_dir()
            .join("wwt")
            .join("store.json")
            .to_str()
            .unwrap()
            .to_string()
    });

    let mut store = store::Store::new(Path::new(&store_path))
        .unwrap_or_else(|e| util::print_and_exit(e.to_string().as_str()));

    match cli.command {
        Commands::Remember { name, description } => {
            store.set(&name, &description).unwrap_or_else(|e| {
                util::print_and_exit(e.to_string().as_str())
            });
        }
        Commands::Find { description } => {
            let matches = store.find(description.as_str());
            if matches.len() == 0 {
                eprintln!("No matches found.");
                std::process::exit(1);
            } else {
                for [k, v] in matches {
                    println!("{} -> {}", k, v);
                }
            }
        }
        Commands::Forget { name } => {
            store.delete(&name).unwrap_or_else(|e| {
                util::print_and_exit(e.to_string().as_str())
            });
        }
    }
}
