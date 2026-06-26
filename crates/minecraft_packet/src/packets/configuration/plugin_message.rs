use takumi_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x02)]
pub struct PluginMessagePacket {
    pub channel: String,
    #[protocol(remaining)]
    pub data: Vec<u8>,
}
