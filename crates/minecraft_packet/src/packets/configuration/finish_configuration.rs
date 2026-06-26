use yoki_macros::PacketOut;

#[derive(Debug, Clone, PacketOut)]
#[packet(id = 0x03)]
pub struct FinishConfigurationPacket;
