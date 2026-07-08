use yoki_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
pub struct EncryptionResponsePacket {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}
