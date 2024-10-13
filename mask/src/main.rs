mod executor;
mod loader;
use clap::{crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use executor::execute_command;
use mask_parser::maskfile::Command;
use std::env;
use std::path::Path;

fn main() {
    let cli_app = App::new(crate_name!())
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::AllowNegativeNumbers)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .arg(custom_maskfile_path_arg())
        .arg(introspect_arg());

    let (maskfile, maskfile_path) = find_maskfile();
    if maskfile.is_err() {
        // If the maskfile can't be found, at least parse for --version or --help
        cli_app.get_matches();
        return;
    }

    let root = mask_parser::parse(maskfile.unwrap());

    if is_introspecting() {
        let json = root.to_json().expect("to_json failed");
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        return;
    }

    let matches = build_subcommands(cli_app, &root.commands).get_matches();
    let chosen_cmd =
        find_command(&matches, &root.commands).expect("SubcommandRequired failed to work");

    match execute_command(chosen_cmd, maskfile_path) {
        Ok(status) => match status.code() {
            Some(code) => std::process::exit(code),
            None => return,
        },
        Err(err) => {
            eprintln!("{} {}", "ERROR:".red(), err);
            std::process::exit(1)
        }
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

    let maskfile = loader::read_maskfile(maskfile_path);

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

fn is_introspecting() -> bool {
    let args: Vec<String> = env::args().collect();
    for a in args {
        if a == "--introspect" {
            return true;
        }
    }
    false
}

/// Load a maskfile from another directory
fn custom_maskfile_path_arg<'a, 'b>() -> Arg<'a, 'b> {
    // This is needed to prevent clap from complaining about the custom flag check
    // within find_maskfile(). It should be removed once clap 3.x is released.
    // See https://github.com/clap-rs/clap/issues/748
    Arg::with_name("maskfile")
        .help("Path to a different maskfile you want to use")
        .long("maskfile")
        .takes_value(true)
        .multiple(false)
}

/// Print out the maskfile structure in json
fn introspect_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("introspect")
        .help("Print out the maskfile command structure in json")
        .long("introspect")
        .multiple(false)
}

fn build_subcommands<'a, 'b>(
    mut cli_app: App<'a, 'b>,
    subcommands: &'a Vec<Command>,
) -> App<'a, 'b> {
    for c in subcommands {
        let mut subcmd = SubCommand::with_name(&c.name)
            .about(c.description.as_ref())
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::AllowNegativeNumbers);
        if !c.subcommands.is_empty() {
            subcmd = build_subcommands(subcmd, &c.subcommands);
            // If this parent command has no script, require a subcommand.
            if c.script.is_none() {
                subcmd = subcmd.setting(AppSettings::SubcommandRequired);
            }
        }

        // Add all positional arguments
        for a in &c.required_args {
            let arg = Arg::with_name(&a.name).required(true);
            subcmd = subcmd.arg(arg);
        }

        // Add all optional arguments
        for o in &c.optional_args {
            let arg = Arg::with_name(&o.name);
            subcmd = subcmd.arg(arg);
        }

        // Add all named flags
        for f in &c.named_flags {
            let arg = Arg::with_name(&f.name)
                .help(&f.description)
                .short(&f.short)
                .long(&f.long)
                .takes_value(f.takes_value)
                .multiple(f.multiple)
                .required(f.required);
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

    // Check optional args
    for opt_arg in &mut cmd.optional_args {
        opt_arg.val = matches
            .value_of(opt_arg.name.clone())
            .unwrap_or("")
            .to_string();
    }

    // Check all named flags
    for flag in &mut cmd.named_flags {
        flag.val = if flag.takes_value {
            // Extract the value
            let raw_value = matches
                .value_of(flag.name.clone())
                .or(Some(""))
                .unwrap()
                .to_string();

            if !flag.choices.is_empty() && raw_value != "" {
                if !flag.choices.iter().any(|choice| choice == &raw_value) {
                    eprintln!(
                        "{} flag `{}` expects one of {:?}",
                        "ERROR:".red(),
                        flag.name,
                        flag.choices,
                    );
                    std::process::exit(1);
                }
            }

            if flag.validate_as_number && raw_value != "" {
                // Try converting to an integer or float to validate it
                if raw_value.parse::<isize>().is_err() && raw_value.parse::<f32>().is_err() {
                    eprintln!(
                        "{} flag `{}` expects a numerical value",
                        "ERROR:".red(),
                        flag.name
                    );
                    std::process::exit(1);
                }
            }

            raw_value
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
