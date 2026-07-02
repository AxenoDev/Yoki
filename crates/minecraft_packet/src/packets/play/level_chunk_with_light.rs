use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{
    BinaryError, BinaryWriter, ProtocolError, WriteBytes,
    data_types::VarInt,
};

pub const WORLD_SECTION_COUNT: usize = 24;
pub const OVERWORLD_MIN_Y: i32 = -64;
pub const OVERWORLD_HEIGHT: i32 = 384;
pub const FLAT_SURFACE_Y: i32 = 63;
pub const HEIGHTMAP_MOTION_BLOCKING: i32 = 4;

#[derive(Debug, Clone)]
pub struct Heightmap {
    type_id: VarInt,
    data: Vec<i64>,
}

impl Heightmap {
    pub fn motion_blocking(surface_height_index: i32) -> Self {
        let bits_per_entry = heightmap_bits_per_entry(OVERWORLD_HEIGHT);
        Self {
            type_id: VarInt::new(HEIGHTMAP_MOTION_BLOCKING),
            data: pack_heightmap_uniform(surface_height_index as u16, bits_per_entry),
        }
    }
}

impl WriteBytes for Heightmap {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.type_id.write(writer)?;
        self.data.write(writer)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct BitSet(pub Vec<i64>);

impl WriteBytes for BitSet {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.0.write(writer)
    }
}

#[derive(Debug, Clone)]
pub struct LevelChunkWithLightPacket {
    chunk_x: i32,
    chunk_z: i32,
    heightmaps: Vec<Heightmap>,
    chunk_data: Vec<u8>,
    sky_light_mask: BitSet,
    block_light_mask: BitSet,
    empty_sky_light_mask: BitSet,
    empty_block_light_mask: BitSet,
    sky_light_arrays: Vec<[u8; 2048]>,
    block_light_arrays: Vec<[u8; 2048]>,
}

impl LevelChunkWithLightPacket {
    /// Chunk 100% air, utile pour les tests / débug uniquement.
    pub fn empty(chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            chunk_x,
            chunk_z,
            heightmaps: vec![Heightmap::motion_blocking(0)],
            chunk_data: flat_world_chunk_data(None),
            sky_light_mask: BitSet::default(),
            block_light_mask: BitSet::default(),
            empty_sky_light_mask: BitSet::default(),
            empty_block_light_mask: BitSet::default(),
            sky_light_arrays: Vec::new(),
            block_light_arrays: Vec::new(),
        }
    }

    /// Chunk plat avec un sol solide fait du block_state_id fourni.
    pub fn flat(chunk_x: i32, chunk_z: i32, solid_block_state_id: i32) -> Self {
        let surface_height_index = FLAT_SURFACE_Y - OVERWORLD_MIN_Y;
        Self {
            chunk_x,
            chunk_z,
            heightmaps: vec![Heightmap::motion_blocking(surface_height_index)],
            chunk_data: flat_world_chunk_data(Some(solid_block_state_id)),
            sky_light_mask: BitSet::default(),
            block_light_mask: BitSet::default(),
            empty_sky_light_mask: BitSet::default(),
            empty_block_light_mask: BitSet::default(),
            sky_light_arrays: Vec::new(),
            block_light_arrays: Vec::new(),
        }
    }
}

impl PacketMeta for LevelChunkWithLightPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for LevelChunkWithLightPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.chunk_x.write(writer)?;
        self.chunk_z.write(writer)?;
        self.heightmaps.write(writer)?;
        writer.write_byte_array(&self.chunk_data)?;
        VarInt::new(0).write(writer)?;
        self.sky_light_mask.write(writer)?;
        self.block_light_mask.write(writer)?;
        self.empty_sky_light_mask.write(writer)?;
        self.empty_block_light_mask.write(writer)?;
        VarInt::new(self.sky_light_arrays.len() as i32).write(writer)?;
        for arr in &self.sky_light_arrays {
            writer.write_byte_array(arr)?;
        }
        VarInt::new(self.block_light_arrays.len() as i32).write(writer)?;
        for arr in &self.block_light_arrays {
            writer.write_byte_array(arr)?;
        }
        Ok(())
    }
}

fn heightmap_bits_per_entry(world_height: i32) -> u8 {
    let possible_values = (world_height + 1) as u32;
    let bits = 32 - possible_values.leading_zeros();
    bits.max(1) as u8
}

fn pack_heightmap_uniform(value: u16, bits_per_entry: u8) -> Vec<i64> {
    let values = [value; 256];
    pack_heightmap(&values, bits_per_entry)
}

fn pack_heightmap(values: &[u16], bits_per_entry: u8) -> Vec<i64> {
    let bits = bits_per_entry as usize;
    let long_count = (values.len() * bits + 63) / 64;
    let mut data = vec![0u64; long_count];

    for (index, value) in values.iter().enumerate() {
        let value = *value as u64;
        let bit_index = index * bits;
        let long_index = bit_index / 64;
        let bit_offset = bit_index % 64;
        data[long_index] |= value << bit_offset;
        if bit_offset + bits > 64 {
            let overflow = bit_offset + bits - 64;
            data[long_index + 1] |= value >> (bits - overflow);
        }
    }

    data.into_iter().map(|value| value as i64).collect()
}

fn solid_section(block_state_id: i32) -> Vec<u8> {
    let mut w = BinaryWriter::new();
    w.write_all(&4096i16.to_be_bytes());
    w.write_all(&0i16.to_be_bytes());
    write_single_valued_palette(&mut w, block_state_id);
    write_single_valued_palette(&mut w, 0);
    w.into_inner()
}

fn air_section() -> Vec<u8> {
    let mut w = BinaryWriter::new();
    w.write_all(&0i16.to_be_bytes());
    w.write_all(&0i16.to_be_bytes());
    write_single_valued_palette(&mut w, 0);
    write_single_valued_palette(&mut w, 0);
    w.into_inner()
}

fn write_single_valued_palette(writer: &mut BinaryWriter, registry_id: i32) {
    writer.write_byte(0);
    VarInt::new(registry_id).write(writer).ok();
}

fn flat_world_chunk_data(solid_block_state_id: Option<i32>) -> Vec<u8> {
    let mut data = Vec::new();
    for section_index in 0..WORLD_SECTION_COUNT {
        let section = match solid_block_state_id {
            Some(id) if section_index < 8 => solid_section(id),
            _ => air_section(),
        };
        data.extend_from_slice(&section);
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_chunk_section_count_matches_overworld_height() {
        let chunk = LevelChunkWithLightPacket::flat(0, 0, 1);
        let section_size = air_section().len();
        assert_eq!(section_size, 8);
        assert_eq!(chunk.chunk_data.len(), section_size * WORLD_SECTION_COUNT);
    }

    #[test]
    fn motion_blocking_heightmap_has_expected_long_count() {
        let heightmap = Heightmap::motion_blocking(127);
        assert_eq!(heightmap.type_id.inner(), HEIGHTMAP_MOTION_BLOCKING);
        assert_eq!(heightmap.data.len(), 36);
    }

    #[test]
    fn heightmaps_are_written_as_prefixed_array_not_nbt() {
        let packet = LevelChunkWithLightPacket::flat(0, 0, 1);
        let mut writer = BinaryWriter::new();
        packet.encode_payload(&mut writer).unwrap();
        let bytes = writer.into_inner();
        // chunk x,z = 8 bytes, then heightmap array length varint (1)
        assert_eq!(bytes[8], 1);
        // heightmap type MOTION_BLOCKING = 4
        assert_eq!(bytes[9], 4);
        // long array length = 36
        assert_eq!(bytes[10], 36);
    }
}
