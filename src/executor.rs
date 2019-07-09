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

    child.spawn()?.wait()
}
