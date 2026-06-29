use yoki_macros::PacketOut;

use super::PingRequestPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketOut)]
pub struct PingResponsePacket {
    pub payload: i64,
}

impl From<PingRequestPacket> for PingResponsePacket {
    fn from(request: PingRequestPacket) -> Self {
        Self {
            payload: request.payload,
        }
    }
}
