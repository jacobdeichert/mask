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
    pub required_args: Vec<RequiredArg>,
    pub option_flags: Vec<OptionFlag>,
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
            required_args: vec![],
            // TODO: don't needlessly add this to commands that have no script https://github.com/jakedeichert/mask/issues/6
            // Auto add common flags like verbose
            option_flags: vec![OptionFlag {
                name: "verbose".to_string(),
                desc: "Sets the level of verbosity".to_string(),
                short: "v".to_string(),
                long: "verbose".to_string(),
                multiple: false,
                takes_value: false,
                val: "".to_string(),
            }],
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequiredArg {
    pub name: String,
    pub val: String,
}

impl RequiredArg {
    pub fn new(name: String) -> Self {
        Self {
            name,
            val: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptionFlag {
    pub name: String,
    pub desc: String,
    pub short: String,     // v        (used as -v)
    pub long: String,      // verbose  (used as --verbose)
    pub multiple: bool,    // Can it have multiple values? (-vvv OR -i one -i two)
    pub takes_value: bool, // Does it take a value? (-i value)
    pub val: String,
}

impl OptionFlag {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            desc: "".to_string(),
            short: "".to_string(),
            long: "".to_string(),
            multiple: false,
            takes_value: false,
            val: "".to_string(),
        }
    }
}
