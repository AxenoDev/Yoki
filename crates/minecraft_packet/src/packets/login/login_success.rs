use uuid::Uuid;
use yoki_binutils::{ProtocolError, ProtocolWrite, writer::PacketWriter};

use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl ProtocolWrite for Property {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.name);
        writer.write_string(&self.value);
        writer.write_prefixed_optional_string(self.signature.as_deref());
        Ok(())
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
    const ID: i32 = 0x02;
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
    fn encode_payload(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        self.uuid.write_to(writer)?;
        self.username.write_to(writer)?;
        self.properties.write_to(writer)?;

        if self.protocol_version >= 776 {
            let session_id = uuid::Uuid::new_v4();
            let (most, least) = session_id.as_u64_pair();
            writer.write_i64(most as i64);
            writer.write_i64(least as i64);
        }

        Ok(())
    }
}
