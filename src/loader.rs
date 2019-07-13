use std::fs::File;
use std::io::prelude::*;

pub fn read_maskfile() -> Result<String, String> {
    // TODO: try to find maskfile in current directory and maybe parent directories?
    // https://github.com/jakedeichert/mask/issues/7
    let file = File::open("maskfile.md").or(File::open("maskfile"));

    if file.is_err() {
        return Err("Expected a maskfile(.md) to exist in the current directory.".to_string());
    }

    let mut file = file.unwrap();
    let mut maskfile_contents = String::new();
    file.read_to_string(&mut maskfile_contents)
        .expect("expected file contents");

    Ok(maskfile_contents)
}
