fn main() {
    let machfile_contents = mach::loader::read_machfile();
    let commands = mach::parser::build_command_structure(machfile_contents);
    dbg!(commands);

    println!("DONE");
}
