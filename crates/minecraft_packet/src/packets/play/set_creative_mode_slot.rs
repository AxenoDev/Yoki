use crate::packet::{IncomingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{data_types::VarInt, BinaryReader, ProtocolError, ReadBytes};

#[derive(Debug, Clone, Copy)]
pub struct SetCreativeModeSlotPacket {
    pub slot: i16,
    pub item_id: Option<i32>,
}

impl PacketMeta for SetCreativeModeSlotPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for SetCreativeModeSlotPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        let slot = u16::read(reader)? as i16;
        let count = VarInt::read(reader)?.inner();
        let item_id = if count > 0 {
            Some(VarInt::read(reader)?.inner())
        } else {
            None
        };
        Ok(Self { slot, item_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yoki_binutils::BinaryReader;

    #[test]
    fn empty_slot_has_no_item() {
        // slot 36 (0x00 0x24), count 0
        let bytes = [0x00, 0x24, 0x00];
        let mut reader = BinaryReader::new(&bytes);
        let packet = SetCreativeModeSlotPacket::decode_payload(&mut reader).expect("decode");
        assert_eq!(packet.slot, 36);
        assert_eq!(packet.item_id, None);
    }

    #[test]
    fn filled_slot_reads_the_item_id() {
        // slot 36, count 1, item id 1 (stone)
        let bytes = [0x00, 0x24, 0x01, 0x01, 0x00, 0x00];
        let mut reader = BinaryReader::new(&bytes);
        let packet = SetCreativeModeSlotPacket::decode_payload(&mut reader).expect("decode");
        assert_eq!(packet.slot, 36);
        assert_eq!(packet.item_id, Some(1));
    }
}
