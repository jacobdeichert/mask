use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

fn main() {
    let machfile_contents = mach::loader::read_machfile();
    let root_command = mach::parser::build_command_structure(machfile_contents);
    // dbg!(root_command.clone());

    // println!("EXECUTING LAST COMMAND");
    // let cmd = commands.last().unwrap().clone();
    // dbg!(cmd.clone());
    // let result = mach::executor::execute_command(cmd);

    // dbg!(result.unwrap().clone());
    // println!("exit code {:?}", result.unwrap().code());

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();
}
