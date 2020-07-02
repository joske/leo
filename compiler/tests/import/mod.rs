use crate::{get_error, integers::u32::output_one, parse_program};

use std::env::{current_dir, set_current_dir};

static TEST_SOURCE_DIRECTORY: &str = "tests/import";

// Import tests rely on knowledge of local directories. They should be run locally only.

fn set_local_dir() {
    let mut local = current_dir().unwrap();
    local.push(TEST_SOURCE_DIRECTORY);

    set_current_dir(local).unwrap();
}

#[test]
#[ignore]
fn test_basic() {
    let bytes = include_bytes!("basic.leo");
    let program = parse_program(bytes).unwrap();

    set_local_dir();

    output_one(program);
}

#[test]
#[ignore]
fn test_multiple() {
    let bytes = include_bytes!("multiple.leo");
    let program = parse_program(bytes).unwrap();

    set_local_dir();

    output_one(program);
}

#[test]
#[ignore]
fn test_star() {
    let bytes = include_bytes!("star.leo");
    let program = parse_program(bytes).unwrap();

    set_local_dir();

    output_one(program);
}

#[test]
#[ignore]
fn test_star_fail() {
    let bytes = include_bytes!("star_fail.leo");
    let program = parse_program(bytes).unwrap();

    set_local_dir();

    let _err = get_error(program);
}

#[test]
#[ignore]
fn test_alias() {
    let bytes = include_bytes!("alias.leo");
    let program = parse_program(bytes).unwrap();

    set_local_dir();

    output_one(program);
}

// more complex tests
#[test]
#[ignore]
fn test_many_import() {
    let bytes = include_bytes!("many_import.leo");
    let program = parse_program(bytes).unwrap();

    set_local_dir();

    output_one(program);
}

#[test]
#[ignore]
fn test_many_import_star() {
    let bytes = include_bytes!("many_import_star.leo");
    let program = parse_program(bytes).unwrap();

    set_local_dir();

    output_one(program);
}
