use yoki_binutils::{
    BinaryError, BinaryReader, BinaryWriter, ProtocolError, ProtocolRead, ProtocolWrite, ReadBytes,
    WriteBytes,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const fn offset_by_face(self, face: i32) -> Self {
        match face {
            0 => Self::new(self.x, self.y - 1, self.z),
            1 => Self::new(self.x, self.y + 1, self.z),
            2 => Self::new(self.x, self.y, self.z - 1),
            3 => Self::new(self.x, self.y, self.z + 1),
            4 => Self::new(self.x - 1, self.y, self.z),
            5 => Self::new(self.x + 1, self.y, self.z),
            _ => self,
        }
    }

    fn to_long(self) -> i64 {
        ((i64::from(self.x) & 0x3FF_FFFF) << 38)
            | ((i64::from(self.z) & 0x3FF_FFFF) << 12)
            | (i64::from(self.y) & 0xFFF)
    }

    fn from_long(value: i64) -> Self {
        Self {
            x: sign_extend((value >> 38) as i32, 26),
            y: sign_extend((value << 52 >> 52) as i32, 12),
            z: sign_extend((value << 26 >> 38) as i32, 26),
        }
    }
}

const fn sign_extend(value: i32, bits: u32) -> i32 {
    let shift = 32 - bits;
    (value << shift) >> shift
}

impl WriteBytes for BlockPos {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.to_long().write(writer)
    }
}

impl ReadBytes for BlockPos {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(Self::from_long(i64::read(reader)?))
    }
}

impl ProtocolWrite for BlockPos {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.write(writer).map_err(Into::into)
    }
}

impl ProtocolRead for BlockPos {
    fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        reader.read().map_err(Into::into)
    }
}
