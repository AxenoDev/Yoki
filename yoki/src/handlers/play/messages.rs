use crate::ServerState;
use crate::batch::Batch;
use crate::server::PacketRegistry::SystemChatMessage;
use crate::server::packet_handler::PacketHandlerError;
use crate::server::{ClientState, PacketHandler, PacketRegistry};
use minecraft_packet::packets::play::chat_message::ChatMessagePacket;
use minecraft_packet::packets::play::system_chat::SystemChatMessagePacket;

impl PacketHandler for ChatMessagePacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        println!("{}", self.get_message());

        let mut batch = Batch::new();

        Ok(batch)
    }
}
