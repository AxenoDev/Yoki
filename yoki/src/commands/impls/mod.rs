mod help;
mod list;
mod tps;

use crate::commands::CommandRegistry;

pub fn register_all() -> CommandRegistry {
    let mut registry = CommandRegistry::new();
    registry.register(help::HelpCommand);
    registry.register(list::ListCommand);
    registry.register(tps::TpsCommand);
    registry
}
