use minecraft_packet::packets::play::game_event::GameEventPacket;

use crate::commands::{ArgKind, ArgSpec, Command, CommandContext, CommandError};
use crate::server::gamemode::GameMode;
use crate::server::packet_registry::PacketRegistry;

pub struct GameModeCommand;

impl Command for GameModeCommand {
    fn name(&self) -> &'static str {
        "gamemode"
    }

    fn description(&self) -> &'static str {
        "Changes your game mode"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::required("mode", ArgKind::Word)]
    }

    fn execute(&self, ctx: &mut CommandContext<'_>) -> Result<(), CommandError> {
        let input = ctx.get_str("mode").unwrap_or_default();
        let Some(mode) = GameMode::parse(input) else {
            return Err(CommandError::msg(
                "Unknown game mode. Use survival, creative, adventure or spectator.",
            ));
        };

        let player = &ctx.sender;
        player.set_game_mode(mode);

        player.send_packet(PacketRegistry::GameEvent(
            GameEventPacket::change_game_mode(mode.id()),
        ));
        player.send_packet(PacketRegistry::PlayerAbilities(
            mode.abilities(player.is_flying()),
        ));

        ctx.reply(format!("§aGame mode set to §e{}", mode.name()));
        Ok(())
    }
}
