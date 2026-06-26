use yoki_macros::PacketOut;

#[derive(Debug, Clone, PartialEq, Eq, PacketOut)]
#[packet(id = 0x00)]
pub struct StatusResponsePacket {
    pub json: String,
}

impl StatusResponsePacket {
    pub fn yoki_default() -> Self {
        Self {
            json: r#"{"version":{"name":"26.2","protocol":776},"players":{"max":100,"online":0},"description":{"text":"Yoki Server"}}"#.into(),
        }
    }
}
