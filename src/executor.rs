use std::io::Result;
use std::process;
use std::process::ExitStatus;

use crate::command::Command;

pub fn execute_command(cmd: Command) -> Result<ExitStatus> {
    let mut child = process::Command::new("sh");
    child.arg("-c").arg(cmd.source);

    // Add all required args as environment variables
    for arg in cmd.required_args {
        child.env(arg.name, arg.val);
    }

    child.spawn()?.wait()
}
