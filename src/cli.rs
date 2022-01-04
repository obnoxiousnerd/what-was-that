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

use clap::{Parser, Subcommand};

/// CLI Implementation.
#[derive(Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(env = "WWT_STORE_PATH", help_heading = "ENVIRONMENT")]
    /// Custom path to the store file.
    pub store_path: Option<String>,

    #[clap(subcommand)]
    /// Executed subcommand.
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(alias = "set", verbatim_doc_comment)]
    /// Remember a thing and its description
    ///
    /// After adding the description and the thing to the store successfully,
    /// it will return a 0 status code.
    ///
    /// Examples:
    /// what-was-that remember "ls" "list files"
    Remember {
        /// The name of the thing
        name: String,
        /// The description of the thing
        description: String,
    },

    #[clap(alias = "get", verbatim_doc_comment)]
    /// Find the thing using a description
    ///
    /// Examples:
    /// 1. With a single entry containing the term:
    /// $ what-was-that find "list files"
    /// ls -> list files
    ///
    /// 2. With multiple entries containing the term:
    /// $ what-was-that find "list files"
    /// ls -> list files
    /// ls -l -> list files with longer format
    Find {
        /// Expected description of the thing
        description: String,
    },

    #[clap(alias = "delete", verbatim_doc_comment)]
    /// Forget a thing from the store
    ///
    /// After removing the thing from the store successfully,
    /// it will return a 0 status code. Note that you will have to enter the
    /// exact thing to forget it. If you can't remember the thing itself,
    /// use `what-was-that find` to get the thing, and then run this.
    ///
    /// Examples:
    /// what-was-that forget "ls"
    Forget {
        /// The thing
        name: String,
    },
}
