use minecraft_packet::packets::HandshakePacket;
use minecraft_protocol::State;
use protocol_version::protocol_version::ProtocolVersion;
use thiserror::Error;

use crate::{
    ServerState,
    batch::Batch,
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

impl PacketHandler for HandshakePacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        client_state
            .set_protocol_version(self.get_protocol(server_state.allow_unsupported_versions()));

        let next_state = self.get_next_state().map_err(|err| {
            PacketHandlerError::invalid_state(format!("Unsupported next state {}", err.0))
        })?;

        batch.queue_both_state_change(next_state);

        match next_state {
            State::Status => {
                if server_state.reply_to_status() {
                    Ok(batch)
                } else {
                    Err(PacketHandlerError::disconnect("Ignoring status request"))
                }
            }
            State::Login => {
                if client_state.protocol_version().is_unsupported() {
                    return Err(PacketHandlerError::invalid_state(format!(
                        "Unsupported protocol version {}",
                        client_state.protocol_version().protocol_number()
                    )));
                }
                Ok(batch)
            }
            State::Transfer => {
                if server_state.accept_transfers() {
                    batch.queue_both_state_change(State::Login);
                    if client_state.protocol_version().is_unsupported() {
                        return Err(PacketHandlerError::invalid_state(format!(
                            "Unsupported protocol version {}",
                            client_state.protocol_version().protocol_number()
                        )));
                    }
                    Ok(batch)
                } else {
                    Err(PacketHandlerError::disconnect("Transfers disabled"))
                }
            }
            state => Err(PacketHandlerError::invalid_state(format!(
                "Invalid intention {state}"
            ))),
        }
    }
}

#[derive(Error, Debug)]
#[error("unknown state {0}")]
struct UnknownStateError(i32);

trait GetStateProtocol {
    fn get_next_state(&self) -> Result<State, UnknownStateError>;
    fn get_protocol(&self, allow_unsupported_versions: bool) -> ProtocolVersion;
}

impl GetStateProtocol for HandshakePacket {
    fn get_next_state(&self) -> Result<State, UnknownStateError> {
        match self.intent.inner() {
            1 => Ok(State::Status),
            2 => Ok(State::Login),
            3 => Ok(State::Transfer),
            value => Err(UnknownStateError(value)),
        }
    }

    fn get_protocol(&self, allow_unsupported_versions: bool) -> ProtocolVersion {
        if self.protocol_version.inner() == -1 {
            ProtocolVersion::Any
        } else {
            let pvn = self.protocol_version.inner();
            if allow_unsupported_versions {
                ProtocolVersion::from(pvn)
            } else {
                ProtocolVersion::try_from(pvn).unwrap_or(ProtocolVersion::Unsupported)
            }
        }
    }
}
