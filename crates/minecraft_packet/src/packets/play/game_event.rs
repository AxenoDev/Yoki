use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes};

#[derive(Debug, Clone)]
pub struct GameEventPacket {
    event: u8,
    value: f32,
}

impl GameEventPacket {
    pub fn start_waiting_for_chunks() -> Self {
        Self {
            event: 13,
            value: 0.0,
        }
    }

    /// game mode id: 0 survival, 1 creative, 2 adventure, 3 spectator
    pub fn change_game_mode(game_mode: u8) -> Self {
        Self {
            event: 3,
            value: f32::from(game_mode),
        }
    }
}

impl PacketMeta for GameEventPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for GameEventPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.event.write(writer)?;
        self.value.write(writer)?;
        Ok(())
    }
}
