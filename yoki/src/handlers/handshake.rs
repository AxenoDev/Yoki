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
        let protocol_version = self.get_protocol(server_state.allow_unsupported_versions());
        let rejected =
            protocol_version.is_unsupported() || !server_state.is_version_allowed(protocol_version);
        let next_state = self.get_next_state().map_err(|err| {
            PacketHandlerError::invalid_state(format!("Unsupported next state {}", err.0))
        })?;

        tracing::debug!(
            "handshake: protocol {} -> {next_state:?} (rejected: {rejected})",
            self.protocol_version.inner()
        );

        let mut protocol_version = if rejected {
            ProtocolVersion::Unsupported
        } else {
            protocol_version
        };

        let joins = matches!(next_state, State::Login | State::Transfer);
        if rejected && joins {
            if next_state == State::Transfer || !server_state.accept_transfers() {
                return Err(PacketHandlerError::disconnect("Transfers disabled"));
            }

            let know = self.get_protocol(false);
            let label = if know.is_unsupported() {
                format!("protocol {}", self.protocol_version.inner())
            } else {
                know.humanize().to_string()
            };
            protocol_version = if know.is_unsupported() {
                ProtocolVersion::latest()
            } else {
                know
            };
            tracing::debug!("version {label} is not allowed, client will be kicked");
            client_state.kick(server_state.version_kick_message(&label));
        }

        client_state.set_protocol_version(protocol_version);
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
                tracing::debug!(
                    "handshake: client is logging in with protocol {}",
                    protocol_version.humanize()
                );
                Ok(batch)
            }
            State::Transfer => {
                if server_state.accept_transfers() {
                    batch.queue_both_state_change(State::Login);
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
