use crate::packet::{IncomingPacket, PacketDirection, PacketMeta};
use crate::packets::play::data::BlockPos;
use yoki_binutils::{data_types::VarInt, BinaryReader, ProtocolError, ReadBytes};

#[derive(Debug, Clone, Copy)]
pub struct UseItemOnPacket {
    pub hand: i32,
    pub pos: BlockPos,
    pub face: i32,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub inside_block: bool,
    pub world_border_hit: bool,
    pub sequence: i32,
}

impl PacketMeta for UseItemOnPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for UseItemOnPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            hand: VarInt::read(reader)?.inner(),
            pos: BlockPos::read(reader)?,
            face: VarInt::read(reader)?.inner(),
            cursor_x: f32::read(reader)?,
            cursor_y: f32::read(reader)?,
            cursor_z: f32::read(reader)?,
            inside_block: bool::read(reader)?,
            world_border_hit: bool::read(reader)?,
            sequence: VarInt::read(reader)?.inner(),
        })
    }
}
