use minecraft_packet::packets::login::login_start::LoginStartPacket;
use minecraft_packet::packets::login::login_success::LoginSuccessPacket;

use crate::{
    ServerState,
    batch::Batch,
    server::game_profile::GameProfile,
    server::packet_registry::PacketRegistry,
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

impl PacketHandler for LoginStartPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        let protocol_version = client_state.protocol_version().protocol_number();
        tracing::info!("login start: {} (uuid {})", self.name, self.uuid);
        client_state.set_game_profile(GameProfile::new(&self.name, self.uuid));
        batch.queue_packet(PacketRegistry::LoginSuccess(LoginSuccessPacket::offline(
            self.uuid,
            self.name.clone(),
            protocol_version,
        )));
        Ok(batch)
    }
}
