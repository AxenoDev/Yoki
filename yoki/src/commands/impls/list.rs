use crate::commands::{Command, CommandContext, CommandError};

pub struct ListCommand;

impl Command for ListCommand {
    fn name(&self) -> &'static str {
        "list"
    }

    fn description(&self) -> &'static str {
        "Shows the players currently online"
    }

    fn execute(&self, ctx: &mut CommandContext<'_>) -> Result<(), CommandError> {
        let players = ctx.server_state.players().all();
        let names: Vec<&str> = players
            .iter()
            .map(|player| player.profile().username())
            .collect();

        ctx.reply(format!(
            "§ePlayers online ({}): §f{}",
            names.len(),
            names.join(", ")
        ));
        Ok(())
    }
}
