use yoki_binutils::{ProtocolError, reader::PacketReader};

use crate::packet::IncomingPacket;

#[derive(Debug, Clone)]
pub struct RawPacket {
    pub id: i32,
    pub payload: Vec<u8>,
}

impl RawPacket {
    pub fn decode<P: IncomingPacket>(&self) -> Result<P, ProtocolError> {
        if self.id != P::ID {
            return Err(ProtocolError::UnknownPacket {
                id: self.id,
                conn: None,
            });
        }

        let mut reader = PacketReader::new(&self.payload);
        P::decode_payload(&mut reader)
    }
}
