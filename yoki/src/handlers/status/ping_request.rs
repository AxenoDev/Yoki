use minecraft_packet::packets::{PingRequestPacket, PongResponsePacket};

use crate::{
    ServerState,
    batch::Batch,
    server::packet_registry::PacketRegistry,
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

impl PacketHandler for PingRequestPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        batch.queue_packet(PacketRegistry::PongResponse(PongResponsePacket::from(
            *self,
        )));
        Ok(batch)
    }
}
