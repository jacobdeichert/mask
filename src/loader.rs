use std::fs::File;
use std::io::prelude::*;

pub fn read_maskfile() -> String {
    // TODO: try to find maskfile in current directory and maybe parent directories?
    let file = File::open("maskfile.md").or(File::open("maskfile"));

    if file.is_err() {
        panic!("Expected a maskfile(.md) to exist in the current directory.")
    }

    let mut file = file.unwrap();
    let mut maskfile_contents = String::new();
    file.read_to_string(&mut maskfile_contents)
        .expect("expected file contents");

    maskfile_contents
}
