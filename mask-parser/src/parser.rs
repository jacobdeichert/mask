use crate::maskfile::*;
use pulldown_cmark::Event::{Code, End, InlineHtml, Start, Text};
use pulldown_cmark::{Options, Parser, Tag};

pub fn parse(maskfile_contents: String) -> Maskfile {
    let parser = create_markdown_parser(&maskfile_contents);
    let mut commands = vec![];
    let mut current_command = Command::new(1);
    let mut current_option_flag = NamedFlag::new();
    let mut text = "".to_string();
    let mut list_level = 0;

    for event in parser {
        match event {
            Start(tag) => {
                match tag {
                    Tag::Header(heading_level) => {
                        // Add the last command before starting a new one.
                        // Don't add commands for level 1 heading blocks (the title).
                        if heading_level > 1 {
                            commands.push(current_command.build());
                        } else if heading_level == 1 && commands.len() > 0 {
                            // Found another level 1 heading block, so quit parsing.
                            break;
                        }
                        current_command = Command::new(heading_level as u8);
                    }
                    #[cfg(not(windows))]
                    Tag::CodeBlock(lang_code) => {
                        if lang_code.to_string() != "powershell"
                            && lang_code.to_string() != "batch"
                            && lang_code.to_string() != "cmd"
                        {
                            if let Some(s) = &mut current_command.script {
                                s.executor = lang_code.to_string();
                            }
                        }
                    }
                    #[cfg(windows)]
                    Tag::CodeBlock(lang_code) => {
                        if let Some(s) = &mut current_command.script {
                            s.executor = lang_code.to_string();
                        }
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
                    // let (name, required_args) = parse_command_name_and_required_args(text.clone());
                    let (name, required_args, optional_args) =
                        parse_command_name_required_and_optional_args(text.clone());
                    current_command.name = name;
                    current_command.required_args = required_args;
                    current_command.optional_args = optional_args;
                }
                Tag::BlockQuote => {
                    current_command.description = text.clone();
                }
                #[cfg(not(windows))]
                Tag::CodeBlock(lang_code) => {
                    if lang_code.to_string() != "powershell"
                        && lang_code.to_string() != "batch"
                        && lang_code.to_string() != "cmd"
                    {
                        if let Some(s) = &mut current_command.script {
                            s.source = text.to_string();
                        }
                    }
                }
                #[cfg(windows)]
                Tag::CodeBlock(_) => {
                    if let Some(s) = &mut current_command.script {
                        s.source = text.to_string();
                    }
                }
                Tag::List(_) => {
                    // Don't go lower than zero (for cases where it's a non-OPTIONS list)
                    list_level = std::cmp::max(list_level - 1, 0);

                    // Must be finished parsing the current option
                    if list_level == 1 {
                        // Add the current one to the list and start a new one
                        current_command
                            .named_flags
                            .push(current_option_flag.clone());
                        current_option_flag = NamedFlag::new();
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
                        "desc" => current_option_flag.description = val.to_string(),
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
                        "required" => {
                            current_option_flag.required = true;
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

    Maskfile {
        title: root_command.name.clone(),
        description: root_command.description.clone(),
        commands: root_command.subcommands.clone(),
    }
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
        if c.level > current_command.level {
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
        else if c.level == current_command.level {
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

    // the command or any one of its subcommands must have script to be included in the tree
    // root level commands must be retained
    command_tree.retain(|c| c.script.is_some() || !c.subcommands.is_empty() || c.level == 1);

    command_tree
}

fn parse_command_name_required_and_optional_args(
    text: String,
) -> (String, Vec<RequiredArg>, Vec<OptionalArg>) {
    // Checks if any args are present and if not, return early
    let split_idx = match text.find(|c| c == '(' || c == '[') {
        Some(idx) => idx,
        None => return (text.trim().to_string(), vec![], vec![]),
    };

    let (name, args) = text.split_at(split_idx);
    let name = name.trim().to_string();

    // Collects (required_args)
    let required_args = args
        .split(|c| c == '(' || c == ')')
        .filter_map(|arg| match arg.trim() {
            a if !a.is_empty() && !a.contains('[') => Some(RequiredArg::new(a.trim().to_string())),
            _ => None,
        })
        .collect();

    // Collects [optional_args]
    let optional_args = args
        .split(|c| c == '[' || c == ']')
        .filter_map(|arg| match arg.trim() {
            a if !a.is_empty() && !a.contains('(') => Some(OptionalArg::new(a.trim().to_string())),
            _ => None,
        })
        .collect();

    (name, required_args, optional_args)
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

## parent
### parent subcommand
> This is a subcommand

~~~bash
echo hey
~~~

## no_script

This command has no source/script.

## multi (required) [optional]

> Example with optional args

~~~bash
if ! [ -z "$optional" ]; then
 echo "This is optional - $optional"
fi

echo "This is required - $required"
~~~
"#;

#[cfg(test)]
mod parse {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_the_maskfile_structure() {
        let maskfile = parse(TEST_MASKFILE.to_string());

        let verbose_flag = json!({
            "name": "verbose",
            "description": "Sets the level of verbosity",
            "short": "v",
            "long": "verbose",
            "multiple": false,
            "takes_value": false,
            "required": false,
            "validate_as_number": false,
        });

        assert_eq!(
            json!({
                "title": "Document Title",
                "description": "",
                "commands": [
                    {
                        "level": 2,
                        "name": "serve",
                        "description": "Serve the app on the `port`",
                        "script": {
                            "executor": "bash",
                            "source": "echo \"Serving on port $port\"\n",
                        },
                        "subcommands": [],
                        "required_args": [
                            {
                                "name": "port"
                            }
                        ],
                        "optional_args": [],
                        "named_flags": [verbose_flag],
                    },
                    {
                        "level": 2,
                        "name": "node",
                        "description": "An example node script",
                        "script": {
                            "executor": "js",
                            "source": "const { name } = process.env;\nconsole.log(`Hello, ${name}!`);\n",
                        },
                        "subcommands": [],
                        "required_args": [
                            {
                                "name": "name"
                            }
                        ],
                        "optional_args": [],
                        "named_flags": [verbose_flag],
                    },
                    {
                        "level": 2,
                        "name": "parent",
                        "description": "",
                        "script": null,
                        "subcommands": [
                            {
                                "level": 3,
                                "name": "subcommand",
                                "description": "This is a subcommand",
                                "script": {
                                    "executor": "bash",
                                    "source": "echo hey\n",
                                },
                                "subcommands": [],
                                "optional_args": [],
                                "required_args": [],
                                "named_flags": [verbose_flag],
                            }
                        ],
                        "required_args": [],
                        "optional_args": [],
                        "named_flags": [],
                    },
                    {
                        "level": 2,
                        "name": "multi",
                        "description": "Example with optional args",
                        "script": {
                            "executor": "bash",
                            "source": "if ! [ -z \"$optional\" ]; then\n echo \"This is optional - $optional\"\nfi\n\necho \"This is required - $required\"\n",
                        },
                        "subcommands": [],
                        "required_args": [{ "name": "required" }],
                        "optional_args": [{ "name": "optional" }],
                        "named_flags": [verbose_flag],
                    }
                ]
            }),
            maskfile.to_json().expect("should have serialized to json")
        );
    }
}
