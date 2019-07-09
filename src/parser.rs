use pulldown_cmark::{
    Event::{End, Start, Text},
    Options, Parser, Tag,
};

use crate::command::{Command, RequiredArg};


pub fn build_command_structure(machfile_contents: String) -> Command {
    let parser = create_markdown_parser(&machfile_contents);
    let mut commands = vec![];
    let mut current_command = Command::new(1);
    let mut text = "".to_string();

    for event in parser {
        match event {
            Start(tag) => {
                // Reset all state
                text = "".to_string();

                match tag {
                    Tag::Header(heading_level) => {
                        // Add the last command before starting a new one.
                        // Don't add the first command during the first iteration.
                        if heading_level > 1 {
                            commands.push(current_command);
                        }
                        current_command = Command::new(heading_level as u8);
                        // println!("Start Header {}", heading_level);
                    }
                    Tag::CodeBlock(lang_code) => {
                        current_command.executor = lang_code.to_string();
                    }
                    _ => (),
                }
            }
            End(tag) => match tag {
                Tag::Header(heading_level) => {
                    let (name, required_args) =
                        parse_command_name_and_required_args(heading_level, text.clone());
                    current_command.name = name;
                    current_command.required_args = required_args;
                }
                Tag::BlockQuote => {
                    current_command.desc = text.clone();
                }
                Tag::CodeBlock(_) => {
                    current_command.source = text.to_string();
                }
                _ => (),
            },
            Text(body) => {
                text += &body.to_string();
                // println!("BODY {}", body);
            }
            _ => (),
        };
    }

    // Add the last command
    commands.push(current_command);

    // Convert the flat commands array and to a tree of subcommands based on level
    let all = treeify_commands(commands);
    let root_command = all.first().expect("root command must exist");
    root_command.clone()
}


fn create_markdown_parser<'a>(machfile_contents: &'a String) -> Parser<'a> {
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&machfile_contents, options);
    parser
}


fn treeify_commands(commands: Vec<Command>) -> Vec<Command> {
    let mut command_tree = vec![];
    let mut current_command = commands.first().expect("command should exist").clone();
    let num_commands = commands.len();

    for i in 0..num_commands {
        let c = &commands[i];
        let is_last_cmd = i == num_commands - 1;

        // This must be a subcommand
        if c.cmd_level > current_command.cmd_level {
            current_command.subcommands.push(c.clone());
        }
        // This must be a sibling command
        else if c.cmd_level == current_command.cmd_level {
            // Make sure the initial command doesn't skip itself before it finds children
            if i > 0 {
                // Found a sibling, so the current command has found all children.
                command_tree.push(current_command.clone());
                current_command = c.clone();
            }
        }

        // The last command needs to be added regardless, since there's not another iteration of the loop to do so
        if is_last_cmd && c.cmd_level >= current_command.cmd_level {
            command_tree.push(current_command.clone());
        }
    }

    // Treeify all subcommands recursively
    for c in &mut command_tree {
        if !c.subcommands.is_empty() {
            c.subcommands = treeify_commands(c.subcommands.clone());
        }
    }

    command_tree
}

fn parse_command_name_and_required_args(
    heading_level: i32,
    text: String,
) -> (String, Vec<RequiredArg>) {
    // Why heading_level > 2? Because level 1 is the root command title (unused)
    // and level 2 can't be a subcommand so no need to split.
    let name = if heading_level > 2 {
        // Takes a subcommand name like this:
        // "#### db flush postgres <required_arg>"
        // and returns "postgres <required_arg>" as the actual name
        text.clone()
            .split(" ")
            .collect::<Vec<&str>>()
            // Get subcommand after the parent command name
            .split_at(heading_level as usize - 2)
            .1
            .join(" ")
    } else {
        text.clone()
    };

    // Find any required arguments
    let name_and_args: Vec<&str> = name.split(|c| c == '<' || c == '>').collect();
    let (name, args) = name_and_args.split_at(1);
    let name = name.join(" ").trim().to_string();
    let mut required_args: Vec<RequiredArg> = vec![];

    // TODO: some how support infinite args?
    // Maybe something like <files...>
    if !args.is_empty() {
        let args = args.join("");
        let args: Vec<&str> = args.split(" ").collect();
        required_args = args
            .iter()
            .map(|a| RequiredArg::new(a.to_string()))
            .collect();
    }

    (name, required_args)
}
