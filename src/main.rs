use std::env;
use std::path::Path;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
    SubCommand,
};
use colored::*;

use mask::command::Command;
use mask::executor::execute_command;

fn main() {
    let cli_app = App::new(crate_name!())
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(custom_maskfile_path_arg());

    let (maskfile, maskfile_path) = find_maskfile();
    if maskfile.is_err() {
        // If the maskfile can't be found, at least parse for --version or --help
        cli_app.get_matches();
        return;
    }

    let root_command = mask::parser::build_command_structure(maskfile.unwrap());
    let matches = build_subcommands(cli_app, &root_command.subcommands).get_matches();
    let chosen_cmd = find_command(&matches, &root_command.subcommands)
        .expect("SubcommandRequired failed to work");

    match execute_command(chosen_cmd, maskfile_path) {
        Ok(status) => match status.code() {
            Some(code) => std::process::exit(code),
            None => return,
        },
        Err(err) => eprintln!("{} {}", "ERROR:".red(), err),
    }
}

fn find_maskfile() -> (Result<String, String>, String) {
    let args: Vec<String> = env::args().collect();

    let maybe_maskfile = args.get(1);
    let maybe_path = args.get(2);

    // Check for a custom --maskfile arg
    let maskfile_path = match (maybe_maskfile, maybe_path) {
        (Some(a), Some(path)) if a == "--maskfile" => Path::new(path),
        _ => Path::new("./maskfile.md"),
    };

    let maskfile = mask::loader::read_maskfile(maskfile_path);

    if maskfile.is_err() {
        if let Some(p) = maskfile_path.to_str() {
            // Check if this is a custom maskfile
            if p != "./maskfile.md" {
                // Exit with an error it's not found
                eprintln!("{} specified maskfile not found", "ERROR:".red());
                std::process::exit(1);
            } else {
                // Just log a warning and let the process continue
                println!("{} no maskfile.md found", "WARNING:".yellow());
            }
        }
    }

    (maskfile, maskfile_path.to_str().unwrap().to_string())
}

fn custom_maskfile_path_arg<'a, 'b>() -> Arg<'a, 'b> {
    // This is needed to prevent clap from complaining about the custom flag check
    // within find_maskfile(). It should be removed once clap 3.x is released.
    // See https://github.com/clap-rs/clap/issues/748
    let custom_maskfile_path = Arg::with_name("maskfile")
        .help("Path to a different maskfile you want to use")
        .long("maskfile")
        .takes_value(true)
        .multiple(false);

    custom_maskfile_path
}

fn build_subcommands<'a, 'b>(
    mut cli_app: App<'a, 'b>,
    subcommands: &'a Vec<Command>,
) -> App<'a, 'b> {
    for c in subcommands {
        let mut subcmd = SubCommand::with_name(&c.name)
            .about(c.desc.as_ref())
            .setting(AppSettings::ColoredHelp);
        if !c.subcommands.is_empty() {
            subcmd = build_subcommands(subcmd, &c.subcommands);
            // If this parent command has no script source, require a subcommand.
            if c.source == "" {
                subcmd = subcmd.setting(AppSettings::SubcommandRequired);
            }
        }

        // Add all required arguments
        for a in &c.required_args {
            let arg = Arg::with_name(&a.name).required(true);
            subcmd = subcmd.arg(arg);
        }

        // Add all optional flags
        for f in &c.option_flags {
            let arg = Arg::with_name(&f.name)
                .help(&f.desc)
                .short(&f.short)
                .long(&f.long)
                .takes_value(f.takes_value)
                .multiple(f.multiple);
            subcmd = subcmd.arg(arg);
        }
        cli_app = cli_app.subcommand(subcmd);
    }

    cli_app
}

fn find_command<'a>(matches: &ArgMatches, subcommands: &Vec<Command>) -> Option<Command> {
    let mut command = None;

    // The child subcommand that was used
    if let Some(subcommand_name) = matches.subcommand_name() {
        if let Some(matches) = matches.subcommand_matches(subcommand_name) {
            for c in subcommands {
                if c.name == subcommand_name {
                    // Check if a subcommand was called, otherwise return this command
                    command = find_command(matches, &c.subcommands)
                        .or(Some(c.clone()).map(|c| get_command_options(c, &matches)));
                }
            }
        }
    }

    return command;
}

fn get_command_options(mut cmd: Command, matches: &ArgMatches) -> Command {
    // Check all required args
    for arg in &mut cmd.required_args {
        arg.val = matches.value_of(arg.name.clone()).unwrap().to_string();
    }

    // Check all optional flags
    for flag in &mut cmd.option_flags {
        flag.val = if flag.takes_value {
            // Extract the value
            matches
                .value_of(flag.name.clone())
                .or(Some(""))
                .unwrap()
                .to_string()
        } else {
            // Check if the boolean flag is present and set to "true".
            // It's a string since it's set as an environment variable.
            let val = if matches.is_present(flag.name.clone()) {
                "true".to_string()
            } else {
                "".to_string()
            };
            val
        };
    }

    cmd
}
