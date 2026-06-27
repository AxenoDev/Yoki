use minecraft_packet::packets::{ServerStatus, StatusRequestPacket, StatusResponsePacket};

use crate::{
    ServerState,
    batch::Batch,
    server::packet_registry::PacketRegistry,
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

impl PacketHandler for StatusRequestPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        let status = ServerStatus::yoki_default()
            .with_players(
                server_state.online_players() as u32,
                server_state.max_players() as u32,
            )
            .with_description(server_state.motd());
        let response = StatusResponsePacket::from_status(&status);
        batch.queue_packet(PacketRegistry::StatusResponse(response));
        Ok(batch)
    }
}
