use std::fs::File;
use std::io::prelude::*;

pub fn read_machfile() -> String {
    // TODO: try to find machfile in current directory and maybe parent directories?
    // TODO: search for variations: machfile, machfile.md, Machfile.md (case insensitive)
    let mut file = File::open("test/machfile").expect("expected machfile to exist");
    let mut machfile_contents = String::new();
    file.read_to_string(&mut machfile_contents)
        .expect("expected file contents");

    machfile_contents
}
