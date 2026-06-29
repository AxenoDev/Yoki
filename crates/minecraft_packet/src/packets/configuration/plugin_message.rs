use yoki_identifier::Identifier;
use yoki_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
pub struct PluginMessagePacket {
    pub channel: Identifier,
    pub data: Vec<u8>,
}
