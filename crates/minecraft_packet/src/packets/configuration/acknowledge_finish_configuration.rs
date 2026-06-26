use takumi_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x03)]
pub struct AcknowledgeFinishConfigurationPacket;
