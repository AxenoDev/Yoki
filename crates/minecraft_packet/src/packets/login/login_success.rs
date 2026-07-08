use uuid::Uuid;
use yoki_binutils::{
    BinaryError, BinaryWriter, ProtocolError, ProtocolWrite, WriteBytes, data_types::VarInt,
};

use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl WriteBytes for Property {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.name.write(writer)?;
        self.value.write(writer)?;
        match &self.signature {
            Some(signature) => signature.write(writer)?,
            None => VarInt::from(0).write(writer)?,
        }
        Ok(())
    }
}

impl ProtocolWrite for Property {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.write(writer).map_err(Into::into)
    }
}

#[derive(Debug, Clone)]
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
    pub protocol_version: i32,
}

impl PacketMeta for LoginSuccessPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl LoginSuccessPacket {
    pub fn offline(uuid: Uuid, username: String, protocol_version: i32) -> Self {
        Self {
            uuid,
            username,
            properties: vec![],
            protocol_version,
        }
    }
}

impl OutgoingPacket for LoginSuccessPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.uuid.write_to(writer)?;
        self.username.write_to(writer)?;
        self.properties.write_to(writer)?;

        if self.protocol_version >= 776 {
            let session_id = uuid::Uuid::new_v4();
            let (most, least) = session_id.as_u64_pair();
            (most as i64).write_to(writer)?;
            (least as i64).write_to(writer)?;
        }

        Ok(())
    }
}
