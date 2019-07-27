use std::fs::canonicalize;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process;
use std::process::ExitStatus;

use clap::crate_name;

use crate::command::Command;

pub fn execute_command(cmd: Command, maskfile_path: String) -> Result<ExitStatus> {
    let mut child = match cmd.executor.as_ref() {
        "js" | "javascript" => {
            let mut child = process::Command::new("node");
            child.arg("-e").arg(cmd.source);
            child
        }
        "py" | "python" => {
            let mut child = process::Command::new("python");
            child.arg("-c").arg(cmd.source);
            child
        }
        "rb" | "ruby" => {
            let mut child = process::Command::new("ruby");
            child.arg("-e").arg(cmd.source);
            child
        }
        "php" => {
            let mut child = process::Command::new("php");
            child.arg("-r").arg(cmd.source);
            child
        }
        "bash" | "zsh" | "fish" => {
            let mut child = process::Command::new(cmd.executor);
            child.arg("-c").arg(cmd.source);
            child
        }
        _ => {
            let mut child = process::Command::new("sh");
            child.arg("-c").arg(cmd.source);
            child
        }
    };

    child = add_utility_variables(child, maskfile_path);

    // Add all required args as environment variables
    for arg in cmd.required_args {
        child.env(arg.name, arg.val);
    }

    // Add all optional flags as environment variables if they have a value
    for flag in cmd.option_flags {
        if flag.val != "" {
            child.env(flag.name, flag.val);
        }
    }

    child.spawn()?.wait()
}

// Add some useful environment variables that scripts can use
fn add_utility_variables(mut child: process::Command, maskfile_path: String) -> process::Command {
    let maskfile_path = PathBuf::from(maskfile_path);

    // Find the absolute path to the maskfile
    let absolute_path = canonicalize(&maskfile_path)
        .expect("canonicalize maskfile path failed")
        .to_str()
        .unwrap()
        .to_string();
    let absolute_path = Path::new(&absolute_path);
    let absolute_path_str = absolute_path.to_str().unwrap();

    // Find the absolute path to the maskfile's parent directory
    let parent_dir = absolute_path.parent().unwrap().to_str().unwrap();

    // This allows us to call "$MASK command" instead of "mask --maskfile <path> command"
    // inside scripts so that they can be location-agnostic (not care where they are
    // called from). This is useful for global maskfiles especially.
    child.env("MASK", format!("{} --maskfile {}", crate_name!(), absolute_path_str));
    // This allows us to refer to the directory the maskfile lives in which can be handy
    // for loading relative files to it.
    child.env("MASKFILE_DIR", parent_dir);

    child
}
