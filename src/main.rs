fn main() {
    let machfile_contents = mach::loader::read_machfile();
    mach::parser::build_cli_structure(machfile_contents);

    println!("DONE");
}
