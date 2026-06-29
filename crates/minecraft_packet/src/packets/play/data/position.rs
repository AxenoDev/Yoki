use yoki_binutils::{
    BinaryError, BinaryReader, BinaryWriter, ProtocolError, ProtocolRead, ProtocolWrite,
    ReadBytes, WriteBytes,
};

#[derive(Clone, Debug, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl ReadBytes for Position {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(Self {
            x: f64::read(reader)?,
            y: f64::read(reader)?,
            z: f64::read(reader)?,
        })
    }
}

impl WriteBytes for Position {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.x.write(writer)?;
        self.y.write(writer)?;
        self.z.write(writer)?;
        Ok(())
    }
}

impl ProtocolWrite for Position {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.write(writer).map_err(Into::into)
    }
}

impl ProtocolRead for Position {
    fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        reader.read().map_err(Into::into)
    }
}
