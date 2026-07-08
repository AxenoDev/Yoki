use yoki_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
pub struct LoginAcknowledgedPacket;
