use yoki_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x01)]
pub struct EncryptionResponsePacket {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}
