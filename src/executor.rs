use std::io::Result;
use std::process;
use std::process::ExitStatus;

use crate::command::Command;

pub fn execute_command(cmd: Command) -> Result<ExitStatus> {
    process::Command::new("sh")
        .arg("-c")
        .arg(cmd.source)
        // TODO: loop through cmd options and add any values as env variables
        .env("CHECK", "true")
        .spawn()?
        .wait()
}
