mod login_acknowledged;
mod login_start;

use minecraft_packet::packets::login::{
    cookie_response_login::CookieResponseLoginPacket,
    encryption_response::EncryptionResponsePacket,
    login_plugin_response::LoginPluginResponsePacket,
};

use crate::{
    ServerState,
    batch::Batch,
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

impl PacketHandler for EncryptionResponsePacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let _ = self;
        Ok(Batch::new())
    }
}

impl PacketHandler for LoginPluginResponsePacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let _ = self;
        Ok(Batch::new())
    }
}

impl PacketHandler for CookieResponseLoginPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let _ = self;
        Ok(Batch::new())
    }
}
