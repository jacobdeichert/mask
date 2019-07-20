use assert_cmd::crate_name;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use std::process::Command;

#[test]
fn specifying_a_maskfile_in_a_different_dir() {
    let temp = assert_fs::TempDir::new().unwrap();
    let maskfile = temp.child("maskfile.md");

    maskfile.write_str("
# Integartion tests

> A line describing the integration tests

## foo

<!-- a few more details -->

## bar

<!-- a few more details -->
")
        .unwrap();


    let maskfile_path = maskfile.path().to_str().unwrap();

    let mut mask = Command::cargo_bin(crate_name!()).unwrap();
    mask.arg("--maskfile").arg(maskfile_path).arg("--help").assert().success();
}
