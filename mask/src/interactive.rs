use colored::*;
use dialoguer::{Confirm, Input, Select};
use mask_parser::maskfile::{Command, Maskfile};
use skim::prelude::*;
use std::sync::Arc;

use crate::executor::execute_command;

struct ListItem {
    label: String,
    preview_text: String,
}

impl SkimItem for ListItem {
    fn text(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.label)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.preview_text.clone())
    }
}

fn build_preview(cmd: &Command) -> String {
    let mut preview = String::new();

    if !cmd.description.is_empty() {
        preview.push_str(&format!("Description: {}\n", cmd.description));
    }

    if !cmd.subcommands.is_empty() {
        let subs: Vec<&str> = cmd.subcommands.iter().map(|s| s.name.as_str()).collect();
        preview.push_str(&format!("\nSubcommands: {}\n", subs.join(", ")));
    }

    if !cmd.required_args.is_empty() {
        let args: Vec<&str> = cmd.required_args.iter().map(|a| a.name.as_str()).collect();
        preview.push_str(&format!("\nRequired args: {}\n", args.join(", ")));
    }

    if !cmd.optional_args.is_empty() {
        let args: Vec<&str> = cmd.optional_args.iter().map(|a| a.name.as_str()).collect();
        preview.push_str(&format!("Optional args: {}\n", args.join(", ")));
    }

    let user_flags: Vec<_> = cmd
        .named_flags
        .iter()
        .filter(|f| f.name != "verbose")
        .collect();
    if !user_flags.is_empty() {
        preview.push_str("\nFlags:\n");
        for f in &user_flags {
            let flag_str = if f.short.is_empty() {
                format!("  --{}", f.long)
            } else {
                format!("  -{}, --{}", f.short, f.long)
            };
            if f.description.is_empty() {
                preview.push_str(&format!("{}\n", flag_str));
            } else {
                preview.push_str(&format!("{}  {}\n", flag_str, f.description));
            }
        }
    }

    if let Some(ref script) = cmd.script {
        preview.push_str(&format!("\nScript ({}):\n", script.executor));
        preview.push_str("─────────────\n");
        preview.push_str(&script.source);
        if !script.source.ends_with('\n') {
            preview.push('\n');
        }
    }

    preview
}

fn build_items(commands: &[Command]) -> Vec<Arc<dyn SkimItem>> {
    commands
        .iter()
        .enumerate()
        .map(|(_, cmd)| {
            let label = if !cmd.subcommands.is_empty() {
                format!("{}  ▸", cmd.name)
            } else {
                cmd.name.clone()
            };
            Arc::new(ListItem {
                label,
                preview_text: build_preview(cmd),
            }) as Arc<dyn SkimItem>
        })
        .collect()
}

fn build_header(breadcrumb: &[String]) -> String {
    let nav_hint = "↑↓ navigate  → enter subcommands  ← back  Enter run  Esc quit";
    if breadcrumb.is_empty() {
        nav_hint.to_string()
    } else {
        format!("{}\n\n📂 {}", nav_hint, breadcrumb.join(" > "))
    }
}

fn run_skim(commands: &[Command], breadcrumb: &[String]) -> Option<(usize, Key)> {
    let items = build_items(commands);
    let header = build_header(breadcrumb);

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .preview(Some(""))
        .preview_window(Some("right:50%:wrap"))
        .multi(false)
        .expect(Some("right,left,enter".to_string()))
        .prompt(Some("Command> "))
        .header(Some(&header))
        .build()
        .expect("failed to build skim options");

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in items {
        tx.send(item).unwrap();
    }
    drop(tx);

    let output = Skim::run_with(&options, Some(rx));

    match output {
        Some(out) if !out.is_abort && !out.selected_items.is_empty() => {
            let selected_text = out.selected_items[0].output().to_string();
            // Find the index by matching the label text
            let index = commands.iter().enumerate().find_map(|(i, cmd)| {
                let label = if !cmd.subcommands.is_empty() {
                    format!("{}  ▸", cmd.name)
                } else {
                    cmd.name.clone()
                };
                if label == selected_text.trim_end() {
                    Some(i)
                } else {
                    None
                }
            });

            index.map(|i| (i, out.final_key))
        }
        _ => None,
    }
}

fn prompt_arguments(cmd: &mut Command) {
    for arg in &mut cmd.required_args {
        let val: String = Input::new()
            .with_prompt(&arg.name)
            .interact_text()
            .unwrap_or_else(|_| std::process::exit(0));
        arg.val = val;
    }

    for arg in &mut cmd.optional_args {
        let val: String = Input::new()
            .with_prompt(format!("{} (optional)", arg.name))
            .allow_empty(true)
            .interact_text()
            .unwrap_or_else(|_| std::process::exit(0));
        arg.val = val;
    }

    for flag in &mut cmd.named_flags {
        if flag.name == "verbose" {
            continue;
        }

        if !flag.choices.is_empty() {
            let selection = Select::new()
                .with_prompt(&flag.name)
                .items(&flag.choices)
                .default(0)
                .interact_opt()
                .unwrap_or_else(|_| std::process::exit(0));
            if let Some(idx) = selection {
                flag.val = flag.choices[idx].clone();
            }
        } else if flag.takes_value {
            let prompt_label = if flag.description.is_empty() {
                flag.name.clone()
            } else {
                format!("{} ({})", flag.name, flag.description)
            };
            let val: String = Input::new()
                .with_prompt(prompt_label)
                .allow_empty(!flag.required)
                .interact_text()
                .unwrap_or_else(|_| std::process::exit(0));

            if flag.validate_as_number && !val.is_empty() {
                if val.parse::<isize>().is_err() && val.parse::<f32>().is_err() {
                    eprintln!(
                        "{} flag `{}` expects a numerical value",
                        "ERROR:".red(),
                        flag.name
                    );
                    std::process::exit(1);
                }
            }
            flag.val = val;
        } else {
            let enabled = Confirm::new()
                .with_prompt(&flag.name)
                .default(false)
                .interact_opt()
                .unwrap_or_else(|_| std::process::exit(0));
            if enabled == Some(true) {
                flag.val = "true".to_string();
            }
        }
    }
}

fn execute(cmd: Command, display_path: &str, maskfile_path: String) {
    let mut cmd = cmd;

    let has_prompts = !cmd.required_args.is_empty()
        || !cmd.optional_args.is_empty()
        || cmd.named_flags.iter().any(|f| f.name != "verbose");

    if has_prompts {
        println!("\n{} {}\n", "Selected:".green().bold(), display_path.bold());
        prompt_arguments(&mut cmd);
    }

    println!("\n{} {}\n", "Running:".cyan().bold(), display_path.bold());

    match execute_command(cmd, maskfile_path) {
        Ok(status) => match status.code() {
            Some(code) => std::process::exit(code),
            None => return,
        },
        Err(err) => {
            eprintln!("{} {}", "ERROR:".red(), err);
            std::process::exit(1);
        }
    }
}

pub fn run(root: &Maskfile, maskfile_path: String) {
    if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
        eprintln!("{} interactive mode requires a TTY", "ERROR:".red());
        std::process::exit(1);
    }

    if root.commands.is_empty() {
        eprintln!("No executable commands found in maskfile");
        std::process::exit(1);
    }

    // Navigation stack: each entry is (commands at this level, breadcrumb)
    let mut stack: Vec<(&[Command], Vec<String>)> = vec![];
    let mut current_commands: &[Command] = &root.commands;
    let mut breadcrumb: Vec<String> = vec![];

    loop {
        let result = run_skim(current_commands, &breadcrumb);

        let (index, key) = match result {
            Some(r) => r,
            None => {
                // Esc pressed: go back or quit
                if let Some((prev_commands, prev_breadcrumb)) = stack.pop() {
                    current_commands = prev_commands;
                    breadcrumb = prev_breadcrumb;
                    continue;
                }
                return;
            }
        };

        let cmd = &current_commands[index];

        match key {
            // Left arrow: go back
            Key::Left => {
                if let Some((prev_commands, prev_breadcrumb)) = stack.pop() {
                    current_commands = prev_commands;
                    breadcrumb = prev_breadcrumb;
                }
            }
            // Right arrow: enter subcommands if any
            Key::Right => {
                if !cmd.subcommands.is_empty() {
                    stack.push((current_commands, breadcrumb.clone()));
                    breadcrumb.push(cmd.name.clone());
                    current_commands = &current_commands[index].subcommands;
                }
            }
            // Enter: execute if script, or enter subcommands
            Key::Enter | _ => {
                if cmd.script.is_some() {
                    let display_path = if breadcrumb.is_empty() {
                        cmd.name.clone()
                    } else {
                        format!("{} > {}", breadcrumb.join(" > "), cmd.name)
                    };
                    execute(cmd.clone(), &display_path, maskfile_path);
                    return;
                } else if !cmd.subcommands.is_empty() {
                    stack.push((current_commands, breadcrumb.clone()));
                    breadcrumb.push(cmd.name.clone());
                    current_commands = &current_commands[index].subcommands;
                }
            }
        }
    }
}
