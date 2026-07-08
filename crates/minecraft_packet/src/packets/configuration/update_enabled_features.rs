use yoki_identifier::Identifier;
use yoki_macros::PacketOut;

#[derive(Debug, Clone, PacketOut)]
pub struct UpdateEnabledFeaturesPacket {
    pub features: Vec<Identifier>,
}
