use yoki_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
pub struct CookieResponseLoginPacket {
    pub key: String,
    #[protocol(remaining_option)]
    pub payload: Option<Vec<u8>>,
}
