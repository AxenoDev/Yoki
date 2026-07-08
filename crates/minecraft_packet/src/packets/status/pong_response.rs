use yoki_macros::PacketOut;

use super::PingRequestPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketOut)]
pub struct PongResponsePacket {
    pub payload: i64,
}

impl From<PingRequestPacket> for PongResponsePacket {
    fn from(request: PingRequestPacket) -> Self {
        Self {
            payload: request.payload,
        }
    }
}
