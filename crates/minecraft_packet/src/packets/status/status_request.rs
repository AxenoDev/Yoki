use yoki_macros::PacketIn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PacketIn)]
pub struct StatusRequestPacket;
