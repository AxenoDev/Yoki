use crate::packet::{IncomingPacket, PacketDirection, PacketMeta};
use crate::packets::play::data::BlockPos;
use yoki_binutils::{data_types::VarInt, BinaryReader, ProtocolError, ReadBytes};

pub const STATUS_START_DIGGING: i32 = 0;
pub const STATUS_FINISH_DIGGING: i32 = 2;

#[derive(Debug, Clone, Copy)]
pub struct PlayerActionPacket {
    pub status: i32,
    pub pos: BlockPos,
    pub face: i8,
    pub sequence: i32,
}

impl PlayerActionPacket {
    pub fn is_break(&self) -> bool {
        self.status == STATUS_START_DIGGING || self.status == STATUS_FINISH_DIGGING
    }
}

impl PacketMeta for PlayerActionPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for PlayerActionPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            status: VarInt::read(reader)?.inner(),
            pos: BlockPos::read(reader)?,
            face: i8::read(reader)?,
            sequence: VarInt::read(reader)?.inner(),
        })
    }
}
