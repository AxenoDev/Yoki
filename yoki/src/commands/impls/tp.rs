use std::sync::Arc;

use minecraft_packet::packets::play::entity_position_sync::EntityPositionSyncPacket;
use minecraft_packet::packets::play::synchronize_player_position::SynchronizePlayerPositionPacket;

use crate::commands::{ArgKind, ArgSpec, Command, CommandContext, CommandError};
use crate::server::packet_registry::PacketRegistry;
use crate::server::players::ConnectedPlayer;
use crate::ServerState;

pub struct TpCommand;

impl Command for TpCommand {
    fn name(&self) -> &'static str {
        "tp"
    }

    fn description(&self) -> &'static str {
        "Teleports to a player or coordinates"
    }

    fn overloads(&self) -> Vec<Vec<ArgSpec>> {
        vec![
            vec![
                ArgSpec::required("target", ArgKind::Player),
                ArgSpec::required("x", ArgKind::Coordinate),
                ArgSpec::required("y", ArgKind::Coordinate),
                ArgSpec::required("z", ArgKind::Coordinate),
            ],
            vec![
                ArgSpec::required("target", ArgKind::Player),
                ArgSpec::required("destination", ArgKind::Player),
            ],
            vec![
                ArgSpec::required("x", ArgKind::Coordinate),
                ArgSpec::required("y", ArgKind::Coordinate),
                ArgSpec::required("z", ArgKind::Coordinate),
            ],
            vec![ArgSpec::required("destination", ArgKind::Player)],
        ]
    }

    fn execute(&self, ctx: &mut CommandContext<'_>) -> Result<(), CommandError> {
        let subject = ctx
            .get_player("target")
            .unwrap_or_else(|| Arc::clone(&ctx.sender));

        let (x, y, z) = if let (Some(x), Some(y), Some(z)) = (
            ctx.get_double("x"),
            ctx.get_double("y"),
            ctx.get_double("z"),
        ) {
            (x, y, z)
        } else if let Some(destination) = ctx.get_player("destination") {
            let pos = destination.position();
            (pos.x, pos.y, pos.z)
        } else {
            return Err(CommandError::msg("Nothing to teleport to."));
        };

        teleport(ctx.server_state, &subject, x, y, z);

        if subject.entity_id() == ctx.sender.entity_id() {
            ctx.reply(format!("§aTeleported to §e{x:.1} {y:.1} {z:.1}"));
        } else {
            ctx.reply(format!(
                "§aTeleported §e{}§a to §e{x:.1} {y:.1} {z:.1}",
                subject.profile().username()
            ));
        }
        Ok(())
    }
}

fn teleport(server_state: &ServerState, player: &Arc<ConnectedPlayer>, x: f64, y: f64, z: f64) {
    let pos = player.update_position(|position| {
        position.x = x;
        position.y = y;
        position.z = z;
    });

    player.send_packet(PacketRegistry::SynchronizePlayerPosition(
        SynchronizePlayerPositionPacket::spawn(x, y, z, pos.yaw, pos.pitch),
    ));

    for other in server_state.players().others(player.entity_id()) {
        other.send_packet(PacketRegistry::EntityPositionSync(
            EntityPositionSyncPacket::new(
                player.entity_id(),
                x,
                y,
                z,
                pos.yaw,
                pos.pitch,
                pos.on_ground,
            ),
        ));
    }
}
