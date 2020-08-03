use pulldown_cmark::{
    Event::{Code, End, InlineHtml, Start, Text},
    Options, Parser, Tag,
};

use crate::command::{Command, OptionFlag, RequiredArg};

// Woof. This is ugly. I'm planning on giving this a rewrite at some point...
// At least we have some decent tests in place.
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
                            commands.push(current_command.build());
                        }
                        current_command = Command::new(heading_level as u8);
                    }
                    #[cfg(not(windows))]
                    Tag::CodeBlock(lang_code) => {
                        if lang_code.to_string() != "powershell"
                            && lang_code.to_string() != "batch"
                            && lang_code.to_string() != "cmd"
                        {
                            current_command.script.executor = lang_code.to_string();
                        }
                    }
                    #[cfg(windows)]
                    Tag::CodeBlock(lang_code) => {
                        current_command.script.executor = lang_code.to_string();
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
                Tag::Header(_) => {
                    let (name, required_args) = parse_command_name_and_required_args(text.clone());
                    current_command.name = name;
                    current_command.required_args = required_args;
                }
                Tag::BlockQuote => {
                    current_command.desc = text.clone();
                }
                #[cfg(not(windows))]
                Tag::CodeBlock(lang_code) => {
                    if lang_code.to_string() != "powershell"
                        && lang_code.to_string() != "batch"
                        && lang_code.to_string() != "cmd"
                    {
                        current_command.script.source = text.to_string();
                    }
                }
                #[cfg(windows)]
                Tag::CodeBlock(_) => {
                    current_command.script.source = text.to_string();
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

                // Options level 1 is the flag name
                if list_level == 1 {
                    current_option_flag.name = text.clone();
                }
                // Options level 2 is the flag config
                else if list_level == 2 {
                    let mut config_split = text.splitn(2, ":");
                    let param = config_split.next().unwrap_or("").trim();
                    let val = config_split.next().unwrap_or("").trim();
                    match param {
                        "desc" => current_option_flag.desc = val.to_string(),
                        "type" => {
                            if val == "string" || val == "number" {
                                current_option_flag.takes_value = true;
                            }

                            if val == "number" {
                                current_option_flag.validate_as_number = true;
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
    commands.push(current_command.build());

    // Convert the flat commands array and to a tree of subcommands based on level
    let all = treeify_commands(commands);
    let root_command = all.first().expect("root command must exist");

    // The command root and a possible init script
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
        let mut c = commands[i].clone();

        // This must be a subcommand
        if c.cmd_level > current_command.cmd_level {
            if c.name.starts_with(&current_command.name) {
                // remove parent command name prefixes from subcommand
                c.name = c
                    .name
                    .strip_prefix(&current_command.name)
                    .unwrap()
                    .trim()
                    .to_string();
            }
            current_command.subcommands.push(c);
        }
        // This must be a sibling command
        else if c.cmd_level == current_command.cmd_level {
            // Make sure the initial command doesn't skip itself before it finds children
            if i > 0 {
                // Found a sibling, so the current command has found all children.
                command_tree.push(current_command);
                current_command = c;
            }
        }
    }

    // Adding last command which was not added in the above loop
    command_tree.push(current_command);

    // Treeify all subcommands recursively
    for c in &mut command_tree {
        if !c.subcommands.is_empty() {
            c.subcommands = treeify_commands(c.subcommands.clone());
        }
    }

    command_tree
}

fn parse_command_name_and_required_args(text: String) -> (String, Vec<RequiredArg>) {
    // Find any required arguments. They look like this: (required_arg_name)
    let name_and_args: Vec<&str> = text.split(|c| c == '(' || c == ')').collect();
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

## serve (port)

> Serve the app on the `port`

~~~bash
echo "Serving on port $port"
~~~


## node (name)

> An example node script

Valid lang codes: js, javascript

```js
const { name } = process.env;
console.log(`Hello, ${name}!`);
```


## no_script

This command has no source/script.
"#;

#[cfg(test)]
mod build_command_structure {
    use super::*;

    #[test]
    fn parses_serve_command_name() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree
            .subcommands
            .iter()
            .find(|cmd| cmd.name == "serve")
            .expect("serve command missing");
        assert_eq!(serve_command.name, "serve");
    }

    #[test]
    fn parses_serve_command_description() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree
            .subcommands
            .iter()
            .find(|cmd| cmd.name == "serve")
            .expect("serve command missing");
        assert_eq!(serve_command.desc, "Serve the app on the `port`");
    }

    #[test]
    fn parses_serve_required_positional_arguments() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree
            .subcommands
            .iter()
            .find(|cmd| cmd.name == "serve")
            .expect("serve command missing");
        assert_eq!(serve_command.required_args.len(), 1);
        assert_eq!(serve_command.required_args[0].name, "port");
    }

    #[test]
    fn parses_serve_command_executor() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree
            .subcommands
            .iter()
            .find(|cmd| cmd.name == "serve")
            .expect("serve command missing");
        assert_eq!(serve_command.script.executor, "bash");
    }

    #[test]
    fn parses_serve_command_source_with_tildes() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let serve_command = &tree
            .subcommands
            .iter()
            .find(|cmd| cmd.name == "serve")
            .expect("serve command missing");
        assert_eq!(
            serve_command.script.source,
            "echo \"Serving on port $port\"\n"
        );
    }

    #[test]
    fn parses_node_command_source_with_backticks() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let node_command = &tree
            .subcommands
            .iter()
            .find(|cmd| cmd.name == "node")
            .expect("node command missing");
        assert_eq!(
            node_command.script.source,
            "const { name } = process.env;\nconsole.log(`Hello, ${name}!`);\n"
        );
    }

    #[test]
    fn adds_verbose_optional_flag_to_command_with_script() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let node_command = tree
            .subcommands
            .iter()
            .find(|cmd| cmd.name == "node")
            .expect("node command missing");

        assert_eq!(node_command.option_flags.len(), 1);
        assert_eq!(node_command.option_flags[0].name, "verbose");
        assert_eq!(
            node_command.option_flags[0].desc,
            "Sets the level of verbosity"
        );
        assert_eq!(node_command.option_flags[0].short, "v");
        assert_eq!(node_command.option_flags[0].long, "verbose");
        assert_eq!(node_command.option_flags[0].multiple, false);
        assert_eq!(node_command.option_flags[0].takes_value, false);
    }

    #[test]
    fn does_not_add_verbose_optional_flag_to_command_with_no_script() {
        let tree = build_command_structure(TEST_MASKFILE.to_string());
        let no_script_command = tree
            .subcommands
            .iter()
            .find(|cmd| cmd.name == "no_script")
            .expect("no_script command missing");

        assert_eq!(no_script_command.option_flags.len(), 0);
    }
}
