fn main() {
    let machfile_contents = mach::loader::read_machfile();
    let commands = mach::parser::build_command_structure(machfile_contents);
    // dbg!(commands.clone());

    println!("EXECUTING LAST COMMAND");
    let cmd = commands.last().unwrap().clone();
    dbg!(cmd.clone());
    let _result = mach::executor::execute_command(cmd);
}
