use yoki_macros::PacketIn;
use uuid::Uuid;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x00)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}
