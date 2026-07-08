use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use uuid::Uuid;
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt};

const ACTION_ADD_PLAYER: u8 = 0x01;
const ACTION_UPDATE_LISTED: u8 = 0x08;

#[derive(Debug, Clone)]
pub struct PlayerInfoEntry {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct PlayerInfoUpdatePacket {
    entries: Vec<PlayerInfoEntry>,
}

impl PlayerInfoUpdatePacket {
    pub fn add_players(entries: Vec<PlayerInfoEntry>) -> Self {
        Self { entries }
    }
}

impl PacketMeta for PlayerInfoUpdatePacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for PlayerInfoUpdatePacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_byte(ACTION_ADD_PLAYER | ACTION_UPDATE_LISTED);
        VarInt::new(self.entries.len() as i32).write(writer)?;
        for entry in &self.entries {
            entry.uuid.write(writer)?;
            entry.name.write(writer)?;
            VarInt::new(0).write(writer)?; // no game profile properties (offline mode)
            true.write(writer)?; // listed: visible in the tab list
        }
        Ok(())
    }
}
