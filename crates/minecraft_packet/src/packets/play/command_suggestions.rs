use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{data_types::VarInt, BinaryWriter, ProtocolError, WriteBytes};

#[derive(Debug, Clone)]
pub struct CommandSuggestionsPacket {
    id: VarInt,
    start: VarInt,
    length: VarInt,
    matches: Vec<String>,
}

impl CommandSuggestionsPacket {
    pub fn new(id: i32, start: i32, length: i32, matches: Vec<String>) -> Self {
        Self {
            id: VarInt::new(id),
            start: VarInt::new(start),
            length: VarInt::new(length),
            matches,
        }
    }
}

impl PacketMeta for CommandSuggestionsPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for CommandSuggestionsPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.id.write(writer)?;
        self.start.write(writer)?;
        self.length.write(writer)?;
        VarInt::new(self.matches.len() as i32).write(writer)?;
        for candidate in &self.matches {
            candidate.write(writer)?;
            false.write(writer)?;
        }
        Ok(())
    }
}
