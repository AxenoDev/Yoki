use takumi_macros::PacketIn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketIn)]
#[packet(id = 0x01)]
pub struct PingRequestPacket {
    pub payload: i64,
}
