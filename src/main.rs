use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
};

use mach::command::Command;

fn main() {
    let machfile_contents = mach::loader::read_machfile();
    let root_command = mach::parser::build_command_structure(machfile_contents);
    // dbg!(root_command.clone());

    let cli_app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        );

    let matches = build_subcommands(cli_app, &root_command.subcommands).get_matches();

    let chosen_cmd =
        find_command(&matches, &root_command.subcommands).expect("command must have been found");

    let result = mach::executor::execute_command(chosen_cmd);
    println!("exit code {:?}", result.unwrap().code());

}

fn build_subcommands<'a, 'b>(
    mut cli_app: App<'a, 'b>,
    subcommands: &'b Vec<Command>,
) -> App<'a, 'b> {
    for c in subcommands {
        let mut subcmd = SubCommand::with_name(&c.name).about(c.desc.as_ref());
        if !c.subcommands.is_empty() {
            subcmd = build_subcommands(subcmd, &c.subcommands);
        }
        // TODO: build options
        // subcmd.arg(
        //     Arg::with_name("debug")
        //         .short("d")
        //         .long("debug")
        //         .help("print debug information verbosely"),
        // )
        cli_app = cli_app.subcommand(subcmd);
    }
    cli_app
}

fn find_command(matches: &ArgMatches, subcommands: &Vec<Command>) -> Option<Command> {
    let mut command = None;

    // The child subcommand that was used
    if let Some(subcommand_name) = matches.subcommand_name() {
        if let Some(matches) = matches.subcommand_matches(subcommand_name) {
            // TODO: check for arg/option matches

            for c in subcommands {
                if c.name == subcommand_name {
                    // Check if a subcommand was called, otherwise return this command
                    command = find_command(matches, &c.subcommands).or(Some(c.clone()))
                }
            }
        }
    }

    return command;
}
