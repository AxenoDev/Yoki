use crate::packet::{IncomingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryReader, ProtocolError, ReadBytes};

pub const FLAG_ON_GROUND: u8 = 0x01;

#[derive(Debug, Clone, Copy)]
pub struct MovePlayerPosPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub flags: u8,
}

impl PacketMeta for MovePlayerPosPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for MovePlayerPosPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            x: f64::read(reader)?,
            y: f64::read(reader)?,
            z: f64::read(reader)?,
            flags: u8::read(reader)?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MovePlayerPosRotPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

impl PacketMeta for MovePlayerPosRotPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for MovePlayerPosRotPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            x: f64::read(reader)?,
            y: f64::read(reader)?,
            z: f64::read(reader)?,
            yaw: f32::read(reader)?,
            pitch: f32::read(reader)?,
            flags: u8::read(reader)?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MovePlayerRotPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

impl PacketMeta for MovePlayerRotPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for MovePlayerRotPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            yaw: f32::read(reader)?,
            pitch: f32::read(reader)?,
            flags: u8::read(reader)?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MovePlayerStatusOnlyPacket {
    pub flags: u8,
}

impl PacketMeta for MovePlayerStatusOnlyPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for MovePlayerStatusOnlyPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            flags: u8::read(reader)?,
        })
    }
}
