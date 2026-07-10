use crate::packet::{IncomingPacket, OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryReader, BinaryWriter, ProtocolError, ReadBytes, WriteBytes};

pub const ABILITY_INVULNERABLE: i8 = 0x01;
pub const ABILITY_FLYING: i8 = 0x02;
pub const ABILITY_ALLOW_FLYING: i8 = 0x04;
pub const ABILITY_CREATIVE_MODE: i8 = 0x08;

#[derive(Debug, Clone)]
pub struct PlayerAbilitiesPacket {
    flags: i8,
    flying_speed: f32,
    field_of_view_modifier: f32,
}

impl Default for PlayerAbilitiesPacket {
    fn default() -> Self {
        Self {
            flags: 0,
            flying_speed: 0.05,
            field_of_view_modifier: 0.1,
        }
    }
}

impl PlayerAbilitiesPacket {
    pub fn new(flags: i8) -> Self {
        Self {
            flags,
            ..Self::default()
        }
    }
}

impl PacketMeta for PlayerAbilitiesPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for PlayerAbilitiesPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.flags.write(writer)?;
        self.flying_speed.write(writer)?;
        self.field_of_view_modifier.write(writer)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ServerboundPlayerAbilitiesPacket {
    pub flags: i8,
}

impl ServerboundPlayerAbilitiesPacket {
    pub fn is_flying(&self) -> bool {
        self.flags & ABILITY_FLYING != 0
    }
}

impl PacketMeta for ServerboundPlayerAbilitiesPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for ServerboundPlayerAbilitiesPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            flags: i8::read(reader)?,
        })
    }
}
