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

use std::collections::HashMap;
use std::fmt;
use std::path::Path;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Debug)]
pub enum StoreError {
    Io(std::io::Error),
    Json(serde_json::Error),
    App(StoreErrorKind),
}

// Implement empty format for StoreError
impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoreError::Io(e) => match e.kind() {
                std::io::ErrorKind::NotFound => write!(f, "File not found"),
                _ => write!(f, "IO error: {}", e),
            },
            StoreError::Json(e) => write!(f, "JSON error: {}", e),
            StoreError::App(e) => {
                write!(f, "Application error: {}", e.to_string())
            }
        }
    }
}

// Implement error conversion for StoreError
impl From<std::io::Error> for StoreError {
    fn from(err: std::io::Error) -> Self {
        StoreError::Io(err)
    }
}
impl From<serde_json::Error> for StoreError {
    fn from(err: serde_json::Error) -> Self {
        StoreError::Json(err)
    }
}

#[derive(Debug)]
/// List of possible custom errors that can occur when using the store.
pub enum StoreErrorKind {
    /// The specified key does not exist in the store.
    KeyNotFound(String),
}

impl StoreErrorKind {
    /// Convert the error to a string.
    pub fn to_string(&self) -> String {
        match self {
            StoreErrorKind::KeyNotFound(key) => {
                format!("Key not found: {}", key)
            }
        }
    }
}

/// Store Implementation for the CLI.

pub struct Store<'a> {
    /// The path to the store file.
    pub store_path: &'a Path,
    /// The in-memory store loaded from the store file.
    store: HashMap<String, String>,
}

impl Store<'_> {
    /// Creates a new Store instance.
    pub fn new(store_path: &Path) -> Result<Store, StoreError> {
        let mut store = Store {
            store_path,
            store: HashMap::new(),
        };
        store.load()?;
        Ok(store)
    }

    /// Loads the store from the store file.
    fn load(&mut self) -> Result<(), StoreError> {
        // If the parent directory of the store file does not exist, create it.
        match self.store_path.parent() {
            Some(parent_dir) => {
                if !parent_dir.exists() {
                    std::fs::create_dir_all(parent_dir)?;
                }
            }
            None => {}
        }
        // If the store file does not exist, create it.
        if !self.store_path.exists() {
            std::fs::File::create(self.store_path)?;
        }

        let content = std::fs::read_to_string(self.store_path)?;
        if content.is_empty() {
            // If the store file is empty, there is no point in going further
            // to parse it, so return.
            return Ok(());
        }

        let store = serde_json::from_str::<HashMap<String, String>>(&content)?;
        for (k, v) in store.iter() {
            self.store.insert(k.to_string(), v.to_string());
        }
        Ok(())
    }

    /// Saves the store to the store file.
    fn save(&mut self) -> Result<(), StoreError> {
        let content = serde_json::to_string(&self.store)?;
        std::fs::write(self.store_path, content.as_bytes())?;
        Ok(())
    }

    /// Adds/modifies an entry in the store and saves it to the store file.
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), StoreError> {
        self.store.insert(key.to_string(), value.to_string());
        self.save()?;
        Ok(())
    }

    /// Finds the matches for the given description.
    pub fn find(&self, description: &str) -> Vec<[String; 2]> {
        let matcher = SkimMatcherV2::default();
        let mut matches = Vec::new();
        for (k, v) in self.store.iter() {
            let score = matcher.fuzzy_match(&v, &description);
            match score {
                Some(_) => {
                    matches.push([k.to_string(), v.to_string()]);
                }
                None => {}
            }
        }
        matches
    }

    /// Deletes an entry from the store and saves the store to the store file.
    pub fn delete(&mut self, key: &str) -> Result<(), StoreError> {
        if self.store.contains_key(key) {
            self.store.remove(key);
            self.save()?;
            Ok(())
        } else {
            Err(StoreError::App(StoreErrorKind::KeyNotFound(
                key.to_string(),
            )))
        }
    }
}

// Tests for CLI store.
#[cfg(test)]
mod tests {
    extern crate tempfile;

    use super::*;

    fn run_test(test: fn(Store)) {
        // Setup
        let store_file = tempfile::NamedTempFile::new().unwrap();
        let store = Store::new(store_file.path()).unwrap();
        // Run the test
        test(store);
        // Teardown
        // ...
    }

    #[test]
    fn test_load() {
        run_test(|store| {
            assert_eq!(store.store.len(), 0);
        });
    }

    #[test]
    fn test_set() {
        run_test(|mut store| {
            store.set("key", "value").unwrap();
            assert_eq!(store.store.get("key").unwrap(), "value");
        });
    }

    #[test]
    fn test_find_single_result() {
        run_test(|mut store| {
            store.set("key", "value").unwrap();
            let matches = store.find("value");
            assert_eq!(matches.len(), 1);
            assert_eq!(matches[0][0], "key".to_string());
            assert_eq!(matches[0][1], "value".to_string());
        });
    }

    #[test]
    fn test_find_multiple_results() {
        run_test(|mut store| {
            store.set("key1", "value1").unwrap();
            store.set("key2", "value2").unwrap();

            let matches = store.find("value");
            assert_eq!(matches.len(), 2);
            for [key, _] in matches {
                // We don't know which key is added first, so check for both
                // keys at the same time.
                assert!(["key1", "key2"].contains(&key.as_str()));
            }
        })
    }
}
