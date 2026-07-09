use std::collections::BTreeMap;
use yoki_binutils::data_types::NbtTag;
use yoki_macros::PacketOut;

#[derive(Debug, Clone, PacketOut)]
pub struct SystemChatMessagePacket {
    content: NbtTag,
    overlay: bool,
}

impl SystemChatMessagePacket {
    pub fn text(message: impl Into<String>) -> Self {
        let mut root = BTreeMap::new();
        root.insert("text".to_string(), NbtTag::string(message));
        Self {
            content: NbtTag::Compound(root),
            overlay: false,
        }
    }
}
