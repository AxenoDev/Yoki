use crate::commands::{ArgKind, ArgSpec, Command, CommandContext, CommandError, CommandRegistry};

pub struct HelpCommand;

impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn description(&self) -> &'static str {
        "Lists available commands"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::optional("command", ArgKind::Word)]
    }

    fn execute(&self, ctx: &mut CommandContext<'_>) -> Result<(), CommandError> {
        let commands = ctx.server_state.commands();

        if let Some(name) = ctx.get_str("command") {
            let Some(command) = commands.get(name) else {
                return Err(CommandError::msg(format!("Unknown command '{name}'")));
            };
            ctx.reply(format!(
                "§e{} §8- §7{}",
                CommandRegistry::usage(command.as_ref()),
                command.description()
            ));
            return Ok(());
        }

        ctx.reply(format!("§eAvailable commands ({}):", commands.len()));
        for command in commands.iter() {
            ctx.reply(format!(
                "§7{} §8- §7{}",
                CommandRegistry::usage(command.as_ref()),
                command.description()
            ));
        }
        Ok(())
    }
}
