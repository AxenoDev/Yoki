use minecraft_packet::packets::configuration::{
    acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket,
    client_information::ClientInformationPacket, plugin_message::PluginMessagePacket,
};

use crate::{
    batch::Batch,
    server::{ClientState, PacketHandler, ServerState, packet_handler::PacketHandlerError},
};

impl PacketHandler for ClientInformationPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let _ = self;
        Ok(Batch::new())
    }
}

impl PacketHandler for PluginMessagePacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let _ = self;
        Ok(Batch::new())
    }
}

impl PacketHandler for AcknowledgeFinishConfigurationPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let _ = self;
        Ok(Batch::new())
    }
}
