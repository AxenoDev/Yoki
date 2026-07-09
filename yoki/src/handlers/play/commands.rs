use crate::batch::Batch;
use crate::server::packet_handler::PacketHandlerError;
use crate::server::{ClientState, PacketHandler};
use crate::ServerState;
use minecraft_packet::packets::play::chat_command::ChatCommandPacket;

impl PacketHandler for ChatCommandPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        println!("Command {}", self.get_command());
        Ok(batch)
    }
}
