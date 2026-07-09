use crate::ServerState;
use crate::batch::Batch;
use crate::server::packet_handler::PacketHandlerError;
use crate::server::{ClientState, PacketHandler};
use minecraft_packet::packets::play::chat_message::ChatMessagePacket;

impl PacketHandler for ChatMessagePacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let line = format!("<{}> {}", client_state.get_username(), self.get_message());
        tracing::info!("{line}");
        server_state.broadcast(line);

        Ok(Batch::new())
    }
}
