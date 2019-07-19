use pulldown_cmark::{
    Event::{Code, End, InlineHtml, Start, Text},
    Options, Parser, Tag,
};

use crate::command::{Command, OptionFlag, RequiredArg};


pub fn build_command_structure(maskfile_contents: String) -> Command {
    let parser = create_markdown_parser(&maskfile_contents);
    let mut commands = vec![];
    let mut current_command = Command::new(1);
    let mut current_option_flag = OptionFlag::new();
    let mut text = "".to_string();
    let mut list_level = 0;

    for event in parser {
        match event {
            Start(tag) => {
                match tag {
                    Tag::Header(heading_level) => {
                        // Add the last command before starting a new one.
                        // Don't add the first command during the first iteration.
                        if heading_level > 1 {
                            commands.push(current_command);
                        }
                        current_command = Command::new(heading_level as u8);
                    }
                    Tag::CodeBlock(lang_code) => {
                        current_command.executor = lang_code.to_string();
                    }
                    Tag::List(_) => {
                        // We're in an options list if the current text above it is "OPTIONS"
                        if text == "OPTIONS" || list_level > 0 {
                            list_level += 1;
                        }
                    }
                    _ => (),
                };

                // Reset all state
                text = "".to_string();
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
                Tag::List(_) => {
                    // Don't go lower than zero (for cases where it's a non-OPTIONS list)
                    list_level = std::cmp::max(list_level - 1, 0);

                    // Must be finished parsing the current option
                    if list_level == 1 {
                        // Add the current one to the list and start a new one
                        current_command
                            .option_flags
                            .push(current_option_flag.clone());
                        current_option_flag = OptionFlag::new();
                    }
                }
                _ => (),
            },
            Text(body) => {
                text += &body.to_string();
                // Level 1 is the flag name
                if list_level == 1 {
                    current_option_flag.name = text.clone();
                }
                // Level 2 is the flag config
                else if list_level == 2 {
                    let mut config_split = text.splitn(2, ":");
                    let param = config_split.next().unwrap_or("").trim();
                    let val = config_split.next().unwrap_or("").trim();
                    match param {
                        "desc" => current_option_flag.desc = val.to_string(),
                        // TODO: allow "number" type for input validation purposes https://github.com/jakedeichert/mask/issues/3
                        "type" => {
                            if val == "string" {
                                current_option_flag.takes_value = true;
                            }
                        }
                        // Parse out the short and long flag names
                        "flags" => {
                            let short_and_long_flags: Vec<&str> = val.splitn(2, " ").collect();
                            for flag in short_and_long_flags {
                                // Must be a long flag name
                                if flag.starts_with("--") {
                                    let name = flag.split("--").collect::<Vec<&str>>().join("");
                                    current_option_flag.long = name;
                                }
                                // Must be a short flag name
                                else if flag.starts_with("-") {
                                    // Get the single char
                                    let name = flag.get(1..2).unwrap_or("");
                                    current_option_flag.short = name.to_string();
                                }
                            }
                        }
                        _ => (),
                    };
                }
            }
            InlineHtml(html) => {
                text += &html.to_string();
            }
            Code(inline_code) => {
                text += &format!("`{}`", inline_code);
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


fn create_markdown_parser<'a>(maskfile_contents: &'a String) -> Parser<'a> {
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&maskfile_contents, options);
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

    // Find any required arguments. They look like this: <required_arg_name>
    let name_and_args: Vec<&str> = name.split(|c| c == '<' || c == '>').collect();
    let (name, args) = name_and_args.split_at(1);
    let name = name.join(" ").trim().to_string();
    let mut required_args: Vec<RequiredArg> = vec![];

    // TODO: some how support infinite args? https://github.com/jakedeichert/mask/issues/4
    // TODO: support optional args https://github.com/jakedeichert/mask/issues/5
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

#[cfg(test)]
const TEST_MASKFILE: &str = r#"
# Document Title

This is an example maskfile for the tests below.

## serve <port>

> Serve the app on the `port`

~~~bash
echo "Serving on port $port"
~~~

## node <name>

> An example node script

Valid lang codes: js, javascript

```js
const { name } = process.env;
console.log(`Hello, ${name}!`);
```
"#;

#[cfg(test)]
mod build_command_structure {
    use super::*;

    #[test]
    fn parses_serve_command_name() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree.subcommands[0];
        assert_eq!(serve_command.name, "serve");
    }

    #[test]
    fn parses_serve_command_description() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree.subcommands[0];
        assert_eq!(serve_command.desc, "Serve the app on the `port`");
    }

    #[test]
    fn parses_serve_required_positional_arguments() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree.subcommands[0];
        assert_eq!(serve_command.required_args.len(), 1);
        assert_eq!(serve_command.required_args[0].name, "port");
    }

    #[test]
    fn adds_default_verbose_optional_flag() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree.subcommands[0];
        assert_eq!(serve_command.option_flags.len(), 1);
        assert_eq!(serve_command.option_flags[0].name, "verbose");
        assert_eq!(
            serve_command.option_flags[0].desc,
            "Sets the level of verbosity"
        );
        assert_eq!(serve_command.option_flags[0].short, "v");
        assert_eq!(serve_command.option_flags[0].long, "verbose");
        assert_eq!(serve_command.option_flags[0].multiple, false);
        assert_eq!(serve_command.option_flags[0].takes_value, false);
    }

    #[test]
    fn parses_serve_command_executor() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree.subcommands[0];
        assert_eq!(serve_command.executor, "bash");
    }

    #[test]
    fn parses_serve_command_source_with_tildes() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree.subcommands[0];
        assert_eq!(serve_command.source, "echo \"Serving on port $port\"\n");
    }

    #[test]
    fn parses_node_command_source_with_backticks() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let node_command = &tree.subcommands[1];
        assert_eq!(
            node_command.source,
            "const { name } = process.env;\nconsole.log(`Hello, ${name}!`);\n"
        );
    }
}
