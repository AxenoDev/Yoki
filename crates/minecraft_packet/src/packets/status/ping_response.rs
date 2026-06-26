use takumi_macros::PacketOut;

use super::PingRequestPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketOut)]
#[packet(id = 0x01)]
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
