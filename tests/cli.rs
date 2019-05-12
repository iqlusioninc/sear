//! Integration tests for the `sear` CLI

extern crate tempfile;

use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Output};
use tempfile::NamedTempFile;

const SEAR_EXE_PATH: &str = "./target/debug/sear";

const FIXTURE_FILES: &[&str] = &["bar.txt", "baz.txt", "foo.txt"];

/// Run the `sear` CLI command with the given arguments
pub fn run<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(SEAR_EXE_PATH).args(args).output().unwrap()
}

/// Run the `sear` CLI command with the expectation that it will exit successfully,
/// printing stdout/stderr and panicking if it does not
pub fn run_successfully<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = run(args);
    let status_code = output.status.code().unwrap();

    if status_code == 0 {
        output
    } else {
        io::stdout().write(&output.stdout).unwrap();
        io::stderr().write(&output.stderr).unwrap();
        panic!(
            "{} exited with error status: {}",
            SEAR_EXE_PATH, status_code
        );
    }
}

#[test]
fn test_usage() {
    let status_code = run(&[] as &[&OsStr]).status.code().unwrap();
    assert_eq!(status_code, 2);
}

#[test]
fn test_create() {
    let output_file = NamedTempFile::new().unwrap();
    let output_path = output_file.into_temp_path();
    fs::remove_file(&output_path).unwrap();
    assert!(!output_path.exists());

    let output_path_str = output_path.to_path_buf().to_str().unwrap().to_owned();

    let mut args = vec![
        "-K",
        "tests/fixtures/example.key",
        "-C",
        "tests/fixtures",
        "-cvf",
        &output_path_str,
    ];

    args.extend_from_slice(&FIXTURE_FILES);

    // TODO: ensure stdout matches what we expect
    let _output = run_successfully(&args);

    // TODO: check we made a valid archive
    assert!(output_path.exists());
    output_path.close().unwrap();
}
