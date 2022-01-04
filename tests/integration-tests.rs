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

extern crate assert_cmd;
extern crate predicates;

use std::{collections::HashMap, fs, path::Path};

use assert_cmd::Command;
use predicates::prelude::*;

const TEST_STORE_PATH: &str = "./tests/store.json";

fn setup_cmd(truncate_file: bool) -> Command {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Create the test file before the test, if it does not exist.
    if !Path::new(TEST_STORE_PATH).exists() {
        fs::File::create(TEST_STORE_PATH).unwrap();
    } else {
        if truncate_file {
            std::fs::write(TEST_STORE_PATH, "").unwrap();
        }
    }

    cmd.env("WWT_STORE_PATH", TEST_STORE_PATH);
    cmd
}

#[test]
fn find_nonexistent_key() {
    let mut cmd = setup_cmd(true);

    let assert = cmd.args(&["find", "foo cli"]).assert();
    assert
        .failure()
        .code(1)
        .stderr(predicate::str::contains("No matches found."));
}

#[test]
fn delete_non_existent_command() {
    let mut cmd = setup_cmd(true);
    let assert = cmd.args(&["forget", "foo"]).assert();
    assert
        .failure()
        .code(1)
        .stderr(predicate::str::contains("foo"))
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn set_entry() {
    let mut cmd = setup_cmd(true);
    let assert = cmd.args(&["set", "foo", "A foo cli"]).assert();
    assert.success().code(0);
}

#[test]
fn set_multiple_entries() {
    // Truncate the file before use.
    std::fs::write(TEST_STORE_PATH, "").unwrap();

    let entries = [
        ("foo", "A foo cli"),
        ("bar", "A bar cli"),
        ("baz", "A baz cli"),
    ];
    for (name, description) in entries.iter() {
        // File is not truncated here because the previous additions will be
        // removed if truncated.
        let mut cmd = setup_cmd(false);
        let assert = cmd.args(&["set", name, description]).assert();
        assert.success().code(0);
    }

    let store_contents = serde_json::from_str::<HashMap<String, String>>(
        &fs::read_to_string(TEST_STORE_PATH).unwrap(),
    )
    .unwrap();

    assert_eq!(store_contents.iter().len(), 3);
}

#[test]
fn find_single_entry() {
    let mut set_cmd = setup_cmd(true);
    let assert = set_cmd.args(&["set", "foo", "A foo cli"]).assert();
    assert.success().code(0);

    let mut find_cmd = setup_cmd(false);
    let assert = find_cmd.args(&["find", "foo cli"]).assert();
    assert
        .success()
        .stdout(predicate::str::contains("foo -> A foo cli"));
}

#[test]
fn find_multiple_entries() {
    let entries = [
        ("make-me-a salad", "Makes salad"),
        ("make-me-a cookie", "Makes cookie"),
        ("cat FILE", "Reads FILE and displays contents"),
    ];
    for (name, description) in entries.iter() {
        let mut cmd = setup_cmd(false);
        let assert = cmd.args(&["set", name, description]).assert();
        assert.success().code(0);
    }
    let mut cmd = setup_cmd(false);
    let assert = cmd.args(&["find", "Makes"]).assert();
    assert
        .success()
        .stdout(predicate::str::contains("make-me-a salad -> Makes salad"))
        .stdout(predicate::str::contains("make-me-a cookie -> Makes cookie"));
}

#[test]
fn delete_single_entry() {
    let mut set_cmd = setup_cmd(true);
    let assert = set_cmd.args(&["set", "foo", "A foo cli"]).assert();
    assert.success().code(0);

    let mut delete_cmd = setup_cmd(false);
    let assert = delete_cmd.args(&["delete", "foo"]).assert();
    assert.success().code(0);

    let mut find_cmd = setup_cmd(false);
    let assert = find_cmd.args(&["find", "foo cli"]).assert();
    assert
        .failure()
        .code(1)
        .stderr(predicate::str::contains("No matches found."));
}

#[test]
fn delete_multiple_entries() {
    let entries = [
        ("make-me-a salad", "Makes salad"),
        ("make-me-a cookie", "Makes cookie"),
        ("cat FILE", "Reads FILE and displays contents"),
    ];
    for (name, description) in entries.iter() {
        let mut cmd = setup_cmd(false);
        let assert = cmd.args(&["set", name, description]).assert();
        assert.success().code(0);
    }

    let mut cmd = setup_cmd(false);
    let assert = cmd.args(&["delete", "make-me-a salad"]).assert();
    assert.success().code(0);

    let mut find_cmd = setup_cmd(false);
    let assert = find_cmd.args(&["find", "Makes salad"]).assert();
    assert
        .failure()
        .code(1)
        .stderr(predicate::str::contains("No matches found."));
}

#[test]
fn find_all_entries() {
    setup_cmd(true);
    let entries = [
        ("make-me-a salad", "Makes salad"),
        ("make-me-a cookie", "Makes cookie"),
        ("cat FILE", "Reads FILE and displays contents"),
    ];
    for (name, description) in entries.iter() {
        let mut cmd = setup_cmd(false);
        let assert = cmd.args(&["set", name, description]).assert();
        assert.success().code(0);
    }

    let mut cmd = setup_cmd(false);
    let assert = cmd.args(&["find", ""]).assert();
    assert
        .success()
        .stdout(predicate::str::contains("make-me-a salad -> Makes salad"))
        .stdout(predicate::str::contains("make-me-a cookie -> Makes cookie"))
        .stdout(predicate::str::contains(
            "cat FILE -> Reads FILE and displays contents",
        ));
}
