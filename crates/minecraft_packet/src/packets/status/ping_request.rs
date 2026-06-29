use yoki_macros::PacketIn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketIn)]
pub struct PingRequestPacket {
    pub payload: i64,
}
