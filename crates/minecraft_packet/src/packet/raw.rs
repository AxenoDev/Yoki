use yoki_binutils::{BinaryReader, ProtocolError};

use crate::packet::IncomingPacket;

#[derive(Debug, Clone)]
pub struct RawPacket {
    pub id: i32,
    pub payload: Vec<u8>,
}

impl RawPacket {
    pub fn decode<P: IncomingPacket>(&self) -> Result<P, ProtocolError> {
        let mut reader = BinaryReader::new(&self.payload);
        P::decode_payload(&mut reader)
    }
}
