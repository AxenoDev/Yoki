use minecraft_packet::packets::{
    HandshakePacket, PingRequestPacket, PingResponsePacket, StatusRequestPacket,
    StatusResponsePacket,
    configuration::{
        acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket,
        client_information::ClientInformationPacket,
        finish_configuration::FinishConfigurationPacket, plugin_message::PluginMessagePacket,
    },
    login::{
        cookie_response_login::CookieResponseLoginPacket,
        encryption_response::EncryptionResponsePacket, login_acknowledged::LoginAcknowledgedPacket,
        login_plugin_response::LoginPluginResponsePacket, login_start::LoginStartPacket,
        login_success::LoginSuccessPacket,
    },
};
use takumi_macros::PacketReport;

use crate::{
    batch::Batch,
    server::{PacketHandler, packet_handler::PacketHandlerError},
};

#[derive(Debug, PacketReport)]
pub enum PacketRegistry {
    #[protocol_id(state = "handshake", bound = "serverbound", id = 0x00)]
    Handshake(HandshakePacket),

    #[protocol_id(state = "status", bound = "serverbound", id = 0x00)]
    StatusRequest(StatusRequestPacket),

    #[protocol_id(state = "status", bound = "clientbound", id = 0x00)]
    StatusResponse(StatusResponsePacket),

    #[protocol_id(state = "status", bound = "serverbound", id = 0x01)]
    PingRequest(PingRequestPacket),

    #[protocol_id(state = "status", bound = "clientbound", id = 0x01)]
    PingResponse(PingResponsePacket),

    #[protocol_id(state = "login", bound = "serverbound", id = 0x00)]
    LoginStart(LoginStartPacket),

    #[protocol_id(state = "login", bound = "clientbound", id = 0x02)]
    LoginSuccess(LoginSuccessPacket),

    #[protocol_id(state = "login", bound = "serverbound", id = 0x01)]
    EncryptionResponse(EncryptionResponsePacket),

    #[protocol_id(state = "login", bound = "serverbound", id = 0x02)]
    LoginPluginResponse(LoginPluginResponsePacket),

    #[protocol_id(state = "login", bound = "serverbound", id = 0x03)]
    LoginAcknowledged(LoginAcknowledgedPacket),

    #[protocol_id(state = "login", bound = "serverbound", id = 0x04)]
    CookieResponseLogin(CookieResponseLoginPacket),

    #[protocol_id(state = "configuration", bound = "serverbound", id = 0x00)]
    ClientInformation(ClientInformationPacket),

    #[protocol_id(state = "configuration", bound = "serverbound", id = 0x02)]
    PluginMessage(PluginMessagePacket),

    #[protocol_id(state = "configuration", bound = "serverbound", id = 0x03)]
    AcknowledgeFinishConfiguration(AcknowledgeFinishConfigurationPacket),

    #[protocol_id(state = "configuration", bound = "clientbound", id = 0x03)]
    FinishConfiguration(FinishConfigurationPacket),
}

impl PacketHandler for PacketRegistry {
    fn handle(
        &self,
        client_state: &mut super::ClientState,
        server_state: &super::ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        match self {
            Self::Handshake(packet) => packet.handle(client_state, server_state),
            Self::StatusRequest(packet) => packet.handle(client_state, server_state),
            Self::PingRequest(packet) => packet.handle(client_state, server_state),
            Self::LoginStart(packet) => packet.handle(client_state, server_state),
            Self::EncryptionResponse(packet) => packet.handle(client_state, server_state),
            Self::LoginPluginResponse(packet) => packet.handle(client_state, server_state),
            Self::LoginAcknowledged(packet) => packet.handle(client_state, server_state),
            Self::CookieResponseLogin(packet) => packet.handle(client_state, server_state),
            Self::ClientInformation(packet) => packet.handle(client_state, server_state),
            Self::PluginMessage(packet) => packet.handle(client_state, server_state),
            Self::AcknowledgeFinishConfiguration(packet) => {
                packet.handle(client_state, server_state)
            }
            _ => Err(PacketHandlerError::custom("Unhandled packet")),
        }
    }
}
