use uuid::Uuid;
use yoki_macros::PacketIn;

#[derive(Debug, Clone, PacketIn)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}
