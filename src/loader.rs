use std::fs::File;
use std::io::prelude::*;

pub fn read_maskfile() -> String {
    // TODO: try to find maskfile in current directory and maybe parent directories?
    // TODO: search for variations: maskfile, maskfile.md, Maskfile.md (case insensitive)
    let mut file = File::open("test/maskfile").expect("expected maskfile to exist");
    let mut maskfile_contents = String::new();
    file.read_to_string(&mut maskfile_contents)
        .expect("expected file contents");

    maskfile_contents
}
