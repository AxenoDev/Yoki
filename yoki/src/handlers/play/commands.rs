use crate::ServerState;
use crate::batch::Batch;
use crate::server::packet_handler::PacketHandlerError;
use crate::server::{ClientState, PacketHandler};
use minecraft_packet::packets::play::chat_command::ChatCommandPacket;

impl PacketHandler for ChatCommandPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let command = self.get_command();
        tracing::info!(
            "{} issued server command: /{}",
            client_state.get_username(),
            command
        );

        let Some(entity_id) = client_state.entity_id() else {
            tracing::warn!("received command before entering play");
            return Ok(Batch::new());
        };
        let Some(player) = server_state.players().get(entity_id) else {
            tracing::warn!("received command from unregistered player");
            return Ok(Batch::new());
        };

        server_state
            .commands()
            .dispatch(command, player, server_state);

        Ok(Batch::new())
    }
}
