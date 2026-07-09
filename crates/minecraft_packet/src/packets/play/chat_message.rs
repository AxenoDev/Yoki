use yoki_macros::PacketIn;

#[derive(Debug, PacketIn)]
pub struct ChatMessagePacket {
    content: String,
}

impl ChatMessagePacket {
    pub fn get_message(&self) -> &str {
        &self.content
    }
}
