use minecraft_packet::packets::login::login_start::LoginStartPacket;
use minecraft_packet::packets::login::login_success::LoginSuccessPacket;

use crate::{
    batch::Batch,
    server::packet_registry::PacketRegistry,
    server::{ClientState, PacketHandler, ServerState, packet_handler::PacketHandlerError},
};

impl PacketHandler for LoginStartPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        let protocol_version = _client_state.protocol_version().protocol_number();
        batch.queue_packet(PacketRegistry::LoginSuccess(LoginSuccessPacket::offline(
            self.uuid,
            self.name.clone(),
            protocol_version,
        )));
        Ok(batch)
    }
}
