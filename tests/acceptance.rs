//! `sear` acceptance test: runs the CLI app as a subprocess and asserts its
//! output for given argument combinations matches what is expected.

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use abscissa_core::testing::prelude::*;
use once_cell::sync::Lazy;
use std::fs;
use tempfile::NamedTempFile;

/// Example files to build a `sear` archive with
const FIXTURE_FILES: &[&str] = &["bar.txt", "baz.txt", "foo.txt"];

/// Command runner (uses `Lazy` to serialize execution via a mutex)
pub static RUNNER: Lazy<CmdRunner> = Lazy::new(|| {
    let mut runner = CmdRunner::default();
    runner.exclusive();
    runner
});

/// Run `sear` while providing no arguments
#[test]
fn run_no_args() {
    let mut runner = RUNNER.clone();
    let cmd = runner.capture_stdout().capture_stderr().run();

    // TODO(tarcieri): ensure stderr displays: `neither -c nor -x specified`
    cmd.wait().unwrap().expect_code(1);
}

#[test]
fn test_create() {
    let output_file = NamedTempFile::new().unwrap();
    let output_path = output_file.into_temp_path();
    fs::remove_file(&output_path).unwrap();
    assert!(!output_path.exists());

    let output_path_str = output_path.to_path_buf().to_str().unwrap().to_owned();

    let mut runner = RUNNER.clone();

    let cmd = runner
        .args(&[
            "-K",
            "tests/fixtures/keys/encryption.key",
            "-C",
            "tests/fixtures/files",
            "-cvf",
            &output_path_str,
        ])
        .args(FIXTURE_FILES)
        .run();

    // TODO: ensure stdout matches what we expect
    cmd.wait().unwrap().expect_success();

    // TODO: check we made a valid archive
    assert!(output_path.exists());
    output_path.close().unwrap();
}
