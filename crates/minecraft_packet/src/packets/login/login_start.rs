use uuid::Uuid;
use yoki_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
#[packet(id = 0x00)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}
