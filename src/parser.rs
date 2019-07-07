use pulldown_cmark::{
    Event::{End, Start, Text},
    Options, Parser, Tag,
};

use crate::command::Command;


pub fn build_command_structure(machfile_contents: String) -> Vec<Command> {
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
                        println!("Start Header {}", heading_level);
                    }
                    Tag::CodeBlock(lang_code) => {
                        current_command.executor = lang_code.to_string();
                    }
                    _ => (),
                }
            }
            End(tag) => match tag {
                Tag::Header(_) => {
                    current_command.name = text.clone();
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
                println!("BODY {}", body);
            }
            _ => (),
        };
    }

    // Add the last command
    commands.push(current_command);

    commands
}


fn create_markdown_parser<'a>(machfile_contents: &'a String) -> Parser<'a> {
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&machfile_contents, options);
    parser
}
