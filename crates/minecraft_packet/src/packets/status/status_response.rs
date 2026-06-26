use takumi_macros::PacketOut;

#[derive(Debug, Clone, PartialEq, Eq, PacketOut)]
#[packet(id = 0x00)]
pub struct StatusResponsePacket {
    pub json: String,
}

impl StatusResponsePacket {
    pub fn takumi_default() -> Self {
        Self {
            json: r#"{"version":{"name":"26.2","protocol":776},"players":{"max":100,"online":0},"description":{"text":"Takumi Server"}}"#.into(),
        }
    }
}
