use takumi_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x04)]
pub struct CookieResponseLoginPacket {
    pub key: String,
    #[protocol(remaining_option)]
    pub payload: Option<Vec<u8>>,
}
