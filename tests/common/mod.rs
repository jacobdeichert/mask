use assert_fs::prelude::*;
use assert_cmd::{crate_name, prelude::*};
use std::process::Command;
use std::path::PathBuf;

pub trait MaskCommandExt {
    fn command(&mut self, c: &'static str) -> &mut Command;
    fn cli(&mut self, arguments: &'static str) -> &mut Command;
}

impl MaskCommandExt for Command {
    fn command(&mut self, c: &'static str) -> &mut Command {
        self.arg(c);
        self
    }

    fn cli(&mut self, arguments: &'static str) -> &mut Command {
        let args: Vec<&str> = arguments.split(" ").collect();
        for arg in args {
            self.arg(arg);
        }
        self
    }
}

pub fn maskfile(content: &'static str) -> (assert_fs::TempDir, PathBuf) {
    let temp = assert_fs::TempDir::new().unwrap();
    let maskfile = temp.child("maskfile.md");

    maskfile.write_str(content).unwrap();

    let maskfile_path = maskfile.path().to_path_buf();

    (temp, maskfile_path)
}

pub fn run_mask(maskfile: &PathBuf) -> Command {
    let mut mask = Command::cargo_bin(crate_name!()).expect("Was not able to find binary");

    mask.arg("--maskfile").arg(maskfile);

    mask
}
