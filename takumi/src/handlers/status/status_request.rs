use minecraft_packet::packets::{StatusRequestPacket, StatusResponsePacket};

use crate::{
    batch::Batch,
    server::packet_registry::PacketRegistry,
    server::{ClientState, PacketHandler, ServerState, packet_handler::PacketHandlerError},
};

impl PacketHandler for StatusRequestPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        let mut response = StatusResponsePacket::takumi_default();
        response.json = response
            .json
            .replace(
                "\"online\":0",
                &format!("\"online\":{}", server_state.online_players()),
            )
            .replace(
                "\"max\":100",
                &format!("\"max\":{}", server_state.max_players()),
            );
        batch.queue_packet(PacketRegistry::StatusResponse(response));
        Ok(batch)
    }
}
