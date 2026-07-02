use minecraft_packet::packets::{
    HandshakePacket, PingRequestPacket, PongResponsePacket, StatusRequestPacket,
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
    play::login::LoginPacket,
};
use yoki_macros::PacketReport;

use crate::{
    ServerState,
    batch::Batch,
    server::{PacketHandler, packet_handler::PacketHandlerError},
};

#[derive(Debug, PacketReport)]
pub enum PacketRegistry {
    #[protocol_id(state = "handshake", bound = "serverbound", id = "minecraft:intention")]
    Handshake(HandshakePacket),

    #[protocol_id(state = "status", bound = "serverbound", id = "minecraft:status_request")]
    StatusRequest(StatusRequestPacket),

    #[protocol_id(state = "status", bound = "clientbound", id = "minecraft:status_response")]
    StatusResponse(StatusResponsePacket),

    #[protocol_id(state = "status", bound = "serverbound", id = "minecraft:ping_request")]
    PingRequest(PingRequestPacket),

    #[protocol_id(state = "status", bound = "clientbound", id = "minecraft:pong_response")]
    PongResponse(PongResponsePacket),

    #[protocol_id(state = "login", bound = "serverbound", id = "minecraft:hello")]
    LoginStart(LoginStartPacket),

    #[protocol_id(state = "login", bound = "clientbound", id = "minecraft:login_finished")]
    LoginSuccess(LoginSuccessPacket),

    #[protocol_id(state = "login", bound = "serverbound", id = "minecraft:key")]
    EncryptionResponse(EncryptionResponsePacket),

    #[protocol_id(state = "login", bound = "serverbound", id = "minecraft:custom_query_answer")]
    LoginPluginResponse(LoginPluginResponsePacket),

    #[protocol_id(state = "login", bound = "serverbound", id = "minecraft:login_acknowledged")]
    LoginAcknowledged(LoginAcknowledgedPacket),

    #[protocol_id(state = "login", bound = "serverbound", id = "minecraft:cookie_response")]
    CookieResponseLogin(CookieResponseLoginPacket),

    #[protocol_id(state = "configuration", bound = "serverbound", id = "minecraft:client_information")]
    ClientInformation(ClientInformationPacket),

    #[protocol_id(state = "configuration", bound = "serverbound", id = "minecraft:custom_payload")]
    PluginMessage(PluginMessagePacket),

    #[protocol_id(state = "configuration", bound = "serverbound", id = "minecraft:finish_configuration")]
    AcknowledgeFinishConfiguration(AcknowledgeFinishConfigurationPacket),

    #[protocol_id(state = "configuration", bound = "clientbound", id = "minecraft:finish_configuration")]
    FinishConfiguration(FinishConfigurationPacket),

    #[protocol_id(state = "play", bound = "clientbound", id = "minecraft:login")]
    Login(LoginPacket),
}

impl PacketHandler for PacketRegistry {
    fn handle(
        &self,
        client_state: &mut super::ClientState,
        server_state: &ServerState,
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
