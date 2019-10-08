#[derive(Debug, Clone)]
pub struct Command {
    pub cmd_level: u8,
    pub name: String,
    pub desc: String,
    pub script: Script,
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
            script: Script::new(),
            subcommands: vec![],
            required_args: vec![],
            option_flags: vec![],
        }
    }

    pub fn build(mut self) -> Self {
        // Auto add common flags like verbose for commands that have a script source
        if !self.script.source.is_empty() {
            self.option_flags.push(OptionFlag {
                name: "verbose".to_string(),
                desc: "Sets the level of verbosity".to_string(),
                short: "v".to_string(),
                long: "verbose".to_string(),
                multiple: false,
                takes_value: false,
                validate_as_number: false,
                val: "".to_string(),
            });
        }
        self
    }
}

#[derive(Debug, Clone)]
pub struct Script {
    // The executor to run the source with
    pub executor: String, // shell, node, ruby, python, etc...
    // The script source to execute
    pub source: String,
}

impl Script {
    pub fn new() -> Self {
        Self {
            executor: "".to_string(),
            source: "".to_string(),
        }
    }

    pub fn has_script(&self) -> bool {
        self.source != "" && self.executor != ""
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
    pub short: String,            // v        (used as -v)
    pub long: String,             // verbose  (used as --verbose)
    pub multiple: bool,           // Can it have multiple values? (-vvv OR -i one -i two)
    pub takes_value: bool,        // Does it take a value? (-i value)
    pub validate_as_number: bool, // Should we validate it as a number?
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
            validate_as_number: false,
            val: "".to_string(),
        }
    }
}
