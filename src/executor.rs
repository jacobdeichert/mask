use std::io::Result;
use std::process;
use std::process::ExitStatus;

use crate::command::Command;

pub fn execute_command(cmd: Command) -> Result<ExitStatus> {
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
