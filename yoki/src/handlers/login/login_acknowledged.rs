use minecraft_packet::packets::login::login_acknowledged::LoginAcknowledgedPacket;
use minecraft_protocol::State;

use crate::{
    ServerState,
    batch::Batch,
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

impl PacketHandler for LoginAcknowledgedPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        batch.queue_both_state_change(State::Configuration);
        Ok(batch)
    }
}
