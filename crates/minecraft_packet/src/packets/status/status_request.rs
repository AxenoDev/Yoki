use takumi_macros::PacketIn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PacketIn)]
#[packet(id = 0x00)]
pub struct StatusRequestPacket;
