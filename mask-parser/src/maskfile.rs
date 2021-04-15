use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Maskfile {
    pub title: String,
    pub description: String,
    pub commands: Vec<Command>,
}

impl Maskfile {
    pub fn to_json(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(&self)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Command {
    pub level: u8,
    pub name: String,
    pub description: String,
    pub script: Option<Script>,
    pub subcommands: Vec<Command>,
    pub required_args: Vec<RequiredArg>,
    pub named_flags: Vec<NamedFlag>,
}

impl Command {
    pub fn new(level: u8) -> Self {
        Self {
            level,
            name: "".to_string(),
            description: "".to_string(),
            script: Some(Script::new()),
            subcommands: vec![],
            required_args: vec![],
            named_flags: vec![],
        }
    }

    pub fn build(mut self) -> Self {
        // Set to None if there is no source and executor
        if let Some(s) = &mut self.script {
            if s.source.is_empty() && s.executor.is_empty() {
                self.script = None;
            }
        }

        // Auto add common flags like verbose for commands that have a script source
        if self.script.is_some() {
            self.named_flags.push(NamedFlag {
                name: "verbose".to_string(),
                description: "Sets the level of verbosity".to_string(),
                short: "v".to_string(),
                long: "verbose".to_string(),
                multiple: false,
                takes_value: false,
                required: false,
                validate_as_number: false,
                val: "".to_string(),
            });
        }
        self
    }
}

#[derive(Debug, Serialize, Clone)]
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
}

#[derive(Debug, Serialize, Clone)]
pub struct RequiredArg {
    pub name: String,
    /// Used within mask. TODO: store in a different place within mask instead of here.
    #[serde(skip)]
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

#[derive(Debug, Serialize, Clone)]
pub struct NamedFlag {
    pub name: String,
    pub description: String,
    pub short: String,            // v        (used as -v)
    pub long: String,             // verbose  (used as --verbose)
    pub multiple: bool,           // Can it have multiple values? (-vvv OR -i one -i two)
    pub takes_value: bool,        // Does it take a value? (-i value)
    pub validate_as_number: bool, // Should we validate it as a number?
    pub required: bool,
    /// Used within mask. TODO: store in a different place within mask instead of here.
    #[serde(skip)]
    pub val: String,
}

impl NamedFlag {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            description: "".to_string(),
            short: "".to_string(),
            long: "".to_string(),
            multiple: false,
            takes_value: false,
            required: false,
            validate_as_number: false,
            val: "".to_string(),
        }
    }
}
