use clap::crate_name;
use mask_parser::maskfile::Command;
use std::fs::canonicalize;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process;
use std::process::ExitStatus;

pub fn execute_command(cmd: Command, maskfile_path: String) -> Result<ExitStatus> {
    let script = cmd.script.clone().expect("script should exist");
    if script.source.is_empty() || script.executor.is_empty() {
        let msg = "Command is missing script or lang code which determines which executor to use.";
        return Err(Error::new(ErrorKind::Other, msg));
    }

    let mut child = prepare_command(&cmd);
    child = add_utility_variables(child, maskfile_path);
    child = add_flag_variables(child, &cmd);

    child
        .spawn()
        .map_err(|e| {
            if e.kind() != ErrorKind::NotFound {
                return e;
            }
            Error::new(
                ErrorKind::NotFound,
                format!(
                    "program '{}' for executor '{}' not in PATH",
                    child.get_program().to_string_lossy(),
                    script.executor
                ),
            )
        })?
        .wait()
}

fn prepare_command(cmd: &Command) -> process::Command {
    let script = cmd.script.clone().expect("script should exist");
    let executor = script.executor.clone();
    let source = script.source.clone();

    match executor.as_ref() {
        "js" | "javascript" => {
            let mut child;
            child = process::Command::new("node");
            child.arg("-e").arg(source);
            child
        }
        "py" | "python" => {
            let mut child = process::Command::new("python");
            child.arg("-c").arg(source);
            child
        }
        "lua" => {
            let mut child = process::Command::new("lua");
            child.arg("-e").arg(source);
            child
        }
        "rb" | "ruby" => {
            let mut child = process::Command::new("ruby");
            child.arg("-e").arg(source);
            child
        }
        "php" => {
            let mut child = process::Command::new("php");
            child.arg("-r").arg(source);
            child
        }
        "swift" => {
            let mut child = process::Command::new("swift");
            child.arg("-e").arg(source);
            child
        }
        #[cfg(windows)]
        "cmd" | "batch" => {
            let mut child = process::Command::new("cmd.exe");
            child.arg("/c").arg(source);
            child
        }
        #[cfg(windows)]
        "powershell" => {
            let mut child = process::Command::new("powershell.exe");
            child.arg("-c").arg(source);
            child
        }
        // Any other executor that supports -c (sh, bash, zsh, fish, dash, etc...)
        _ => {
            let mut child = process::Command::new(executor);
            child.arg("-c").arg(source);
            child
        }
    }
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
    child.env(
        "MASK",
        format!("{} --maskfile {}", crate_name!(), absolute_path_str),
    );
    // This allows us to refer to the directory the maskfile lives in which can be handy
    // for loading relative files to it.
    child.env("MASKFILE_DIR", parent_dir);

    child
}

fn add_flag_variables(mut child: process::Command, cmd: &Command) -> process::Command {
    // Add all required args as environment variables
    for arg in &cmd.required_args {
        child.env(arg.name.clone(), arg.val.clone());
    }

    // Add all optional args
    for opt_arg in &cmd.optional_args {
        child.env(opt_arg.name.clone(), opt_arg.val.clone());
    }

    // Add all named flags as environment variables if they have a value
    for flag in &cmd.named_flags {
        if flag.val != "" {
            child.env(flag.name.clone(), flag.val.clone());
        }
    }

    child
}
