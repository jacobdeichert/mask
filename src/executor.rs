use std::fs::canonicalize;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process;
use std::process::ExitStatus;

use clap::crate_name;

use crate::command::{Command, Script};

pub fn execute_command(
    init_script: Script,
    cmd: Command,
    maskfile_path: String,
) -> Result<ExitStatus> {
    let mut child;
    if init_script.has_script() {
        if !validate_init_script(&init_script) {
            let msg = "ON::INIT must be a shell-based script executor.";
            return Err(Error::new(ErrorKind::Other, msg));
        }
        child = prepare_command_with_init_script(init_script, &cmd);
    } else {
        child = prepare_command_without_init_script(&cmd);
    }

    child = add_utility_variables(child, maskfile_path);
    child = add_flag_variables(child, &cmd);

    child.spawn()?.wait()
}

fn prepare_command_without_init_script(cmd: &Command) -> process::Command {
    let executor = cmd.script.executor.clone();
    let source = cmd.script.source.clone();

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
        "bash" | "zsh" | "fish" => {
            let mut child = process::Command::new(executor);
            child.arg("-c").arg(source);
            child
        }
        _ => {
            let mut child = process::Command::new("sh");
            child.arg("-c").arg(source);
            child
        }
    }
}

fn prepare_command_with_init_script(init_script: Script, cmd: &Command) -> process::Command {
    let executor = cmd.script.executor.clone();

    match executor.as_ref() {
        "js" | "javascript" => run_with_init_script(&init_script, &cmd, "node -e"),
        "py" | "python" => run_with_init_script(&init_script, &cmd, "python -c"),
        "rb" | "ruby" => run_with_init_script(&init_script, &cmd, "ruby -e"),
        "php" => run_with_init_script(&init_script, &cmd, "php -r"),
        "bash" | "zsh" | "fish" => {
            run_with_init_script(&init_script, &cmd, &format!("{} -c", executor))
        }
        _ => run_with_init_script(&init_script, &cmd, "sh -c"),
    }
}

fn run_with_init_script(
    init_script: &Script,
    cmd: &Command,
    executor_invocation: &str,
) -> process::Command {
    let mut child = process::Command::new(init_script.executor.clone());
    // Combine the init script with the command to run
    let source = format!(
        "{}\n{} \"{}\"",
        init_script.source.clone(),
        executor_invocation,
        "$MASK_CMD_SOURCE"
    );
    child
        .env("MASK_CMD_SOURCE", cmd.script.source.clone())
        .arg("-c")
        .arg(source);
    child
}

// Validate the subshell init script is shell-based
fn validate_init_script(init_script: &Script) -> bool {
    match init_script.executor.as_ref() {
        "js" | "javascript" | "py" | "python" | "rb" | "ruby" | "php" => false,
        _ => true,
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

    // Add all optional flags as environment variables if they have a value
    for flag in &cmd.option_flags {
        if flag.val != "" {
            child.env(flag.name.clone(), flag.val.clone());
        }
    }

    child
}
