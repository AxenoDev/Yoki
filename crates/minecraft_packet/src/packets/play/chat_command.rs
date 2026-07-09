use yoki_macros::PacketIn;

#[derive(Debug, PacketIn)]
pub struct ChatCommandPacket {
    command: String,
}

impl ChatCommandPacket {
    pub fn get_command(&self) -> &str {
        &self.command
    }
}
