mod args;
mod command;
mod context;
mod error;

pub mod brigadier;
pub mod impls;

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

pub use args::{ArgKind, ArgSpec, ArgValue, Suggest};
pub use command::Command;
pub use context::CommandContext;
pub use error::{ArgError, CommandError};

use crate::server::players::ConnectedPlayer;
use crate::ServerState;

#[derive(Default)]
pub struct CommandRegistry {
    commands: BTreeMap<&'static str, Arc<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, command: impl Command + 'static) {
        for overload in command.overloads() {
            args::validate_specs(command.name(), &overload);
        }
        self.commands.insert(command.name(), Arc::new(command));
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Command>> {
        self.commands.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn Command>> {
        self.commands.values()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn usage(command: &dyn Command) -> String {
        command
            .overloads()
            .iter()
            .map(|specs| Self::usage_line(command.name(), specs))
            .collect::<Vec<_>>()
            .join("\n§c")
    }

    fn usage_line(name: &str, specs: &[ArgSpec]) -> String {
        let mut usage = format!("/{name}");
        for spec in specs {
            if spec.required {
                usage.push_str(&format!(" <{}>", spec.name));
            } else {
                usage.push_str(&format!(" [{}]", spec.name));
            }
        }
        usage
    }

    pub fn dispatch(&self, input: &str, sender: Arc<ConnectedPlayer>, server_state: &ServerState) {
        let trimmed = input.trim();
        let (name, rest) = match trimmed.find(char::is_whitespace) {
            Some(index) => (&trimmed[..index], &trimmed[index..]),
            None => (trimmed, ""),
        };

        let Some(command) = self.commands.get(name) else {
            sender.send("§cUnknown command. Type /help for a list of commands.");
            return;
        };

        let mut last_error = None;
        for overload in command.overloads() {
            match args::parse_args(&overload, rest, server_state) {
                Ok(values) => {
                    let mut ctx = CommandContext::new(Arc::clone(&sender), server_state, values);
                    if let Err(err) = command.execute(&mut ctx) {
                        sender.send(format!("§c{err}"));
                    }
                    return;
                }
                Err(err) => last_error = Some(err),
            }
        }

        if let Some(err) = last_error {
            sender.send(format!("§c{err}"));
        }
        sender.send(format!("§cUsage: {}", Self::usage(command.as_ref())));
    }

    pub fn suggest(&self, text: &str, server_state: &ServerState) -> (i32, i32, Vec<String>) {
        let body = text.strip_prefix('/').unwrap_or(text);
        let base = (text.len() - body.len()) as i32;

        let Some((name, rest)) = body.split_once(char::is_whitespace) else {
            let matches = self
                .commands
                .keys()
                .filter(|command| command.starts_with(body))
                .map(|command| (*command).to_string())
                .collect();
            return (base, body.len() as i32, matches);
        };
        let Some(command) = self.commands.get(name) else {
            return empty_suggestion(text);
        };

        let args_offset = base + name.len() as i32 + 1;
        let (token_start, token) = last_token(rest);
        let arg_index = tokens_before(rest);

        let mut matches: Vec<String> = command
            .overloads()
            .iter()
            .filter_map(|specs| specs.get(arg_index))
            .flat_map(|spec| spec.candidates(token, server_state))
            .collect();
        matches.sort();
        matches.dedup();

        (
            args_offset + token_start as i32,
            token.len() as i32,
            matches,
        )
    }
}

fn empty_suggestion(text: &str) -> (i32, i32, Vec<String>) {
    (text.len() as i32, 0, Vec::new())
}

fn last_token(rest: &str) -> (usize, &str) {
    match rest.rfind(char::is_whitespace) {
        Some(index) => (index + 1, &rest[index + 1..]),
        None => (0, rest),
    }
}

fn tokens_before(rest: &str) -> usize {
    rest.split_whitespace().count().saturating_sub(
        if rest.ends_with(char::is_whitespace) || rest.is_empty() {
            0
        } else {
            1
        },
    )
}

impl fmt::Debug for CommandRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.commands.keys()).finish()
    }
}
