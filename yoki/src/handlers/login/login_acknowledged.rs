use minecraft_packet::packets::configuration::known_packs::SelectKnownPacksClientboundPacket;
use minecraft_packet::packets::login::login_acknowledged::LoginAcknowledgedPacket;
use minecraft_protocol::State;

use crate::{
    ServerState,
    batch::Batch,
    registry::known_packs_for_version,
    server::packet_registry::PacketRegistry,
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

impl PacketHandler for LoginAcknowledgedPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();

        batch.queue_both_state_change(State::Configuration);
        batch.queue_packet(PacketRegistry::SelectKnownPacksClientbound(
            SelectKnownPacksClientboundPacket {
                known_packs: known_packs_for_version(client_state.protocol_version()),
            },
        ));
        Ok(batch)
    }
}
