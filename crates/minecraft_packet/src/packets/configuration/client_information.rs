use yoki_binutils::data_types::VarInt;
use yoki_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
pub struct ClientInformationPacket {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}
