mod args;
mod command;
mod context;
mod error;

pub mod brigadier;
pub mod impls;

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

pub use args::{ArgKind, ArgSpec, ArgValue};
pub use command::Command;
pub use context::CommandContext;
pub use error::{ArgError, CommandError};

use crate::ServerState;
use crate::server::players::ConnectedPlayer;

#[derive(Default)]
pub struct CommandRegistry {
    commands: BTreeMap<&'static str, Arc<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, command: impl Command + 'static) {
        args::validate_specs(command.name(), &command.args());
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
        let mut usage = format!("/{}", command.name());
        for spec in command.args() {
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

        match args::parse_args(&command.args(), rest, server_state) {
            Ok(values) => {
                let mut ctx = CommandContext::new(Arc::clone(&sender), server_state, values);
                if let Err(err) = command.execute(&mut ctx) {
                    sender.send(format!("§c{err}"));
                }
            }
            Err(err) => {
                sender.send(format!("§c{err}"));
                sender.send(format!("§cUsage: {}", Self::usage(command.as_ref())));
            }
        }
    }
}

impl fmt::Debug for CommandRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.commands.keys()).finish()
    }
}
