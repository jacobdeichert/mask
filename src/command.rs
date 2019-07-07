#[derive(Debug, Clone)]
pub struct Command {
    pub cmd_level: u8,
    pub name: String,
    pub desc: String,
    // The executor to run the source with
    pub executor: String, // shell, node, ruby, python, etc...
    // The script source to execute
    pub source: String,
    pub subcommands: Vec<Command>,
    // pub options: Vec<CommandOption>,
    // pub required_args: Vec<RequiredArg>,
}

impl Command {
    pub fn new(cmd_level: u8) -> Self {
        Self {
            cmd_level,
            name: "".to_string(),
            desc: "".to_string(),
            executor: "".to_string(),
            source: "".to_string(),
            subcommands: vec![],
        }
    }
}
