use minecraft_packet::packets::{
    PlayerSample, ServerStatus, StatusRequestPacket, StatusResponsePacket,
};

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
        let max_players: usize = server_state.server_properties().max_players;

        tracing::debug!(
            "status request ({}/{} online)",
            server_state.players().get_size(),
            max_players
        );

        let sample: Vec<PlayerSample> = server_state
            .players()
            .all()
            .iter()
            .map(|player| PlayerSample::new(player.profile().username(), player.profile().uuid()))
            .collect();

        let status = ServerStatus::yoki_default()
            .with_players(
                server_state.players().get_size() as u32,
                max_players as u32,
                Some(sample),
            )
            .with_description(server_state.server_properties().motd.to_string())
            .with_favicon(server_state.favicon().unwrap_or_default().to_string());
        let response = StatusResponsePacket::from_status(&status);
        batch.queue_packet(PacketRegistry::StatusResponse(response));
        Ok(batch)
    }
}
