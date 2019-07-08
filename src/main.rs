use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

use mach::command::Command;

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

    let mut cli_app = App::new(crate_name!())
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

    cli_app = build_subcommands(cli_app, &root_command.subcommands);


    let _matches = cli_app.get_matches();

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
