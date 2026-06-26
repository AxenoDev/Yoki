use minecraft_packet::packets::PingRequestPacket;
use minecraft_packet::packets::PingResponsePacket;

use crate::{
    batch::Batch,
    server::packet_registry::PacketRegistry,
    server::{ClientState, PacketHandler, ServerState, packet_handler::PacketHandlerError},
};

impl PacketHandler for PingRequestPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        batch.queue_packet(PacketRegistry::PingResponse(PingResponsePacket::from(
            *self,
        )));
        Ok(batch)
    }
}
