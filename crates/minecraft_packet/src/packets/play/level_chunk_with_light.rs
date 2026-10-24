use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use protocol_version::protocol_version::ProtocolVersion;
use yoki_binutils::{BinaryError, BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt};

pub const WORLD_SECTION_COUNT: usize = 24;
pub const OVERWORLD_MIN_Y: i32 = -64;
pub const OVERWORLD_HEIGHT: i32 = 384;
pub const FLAT_SURFACE_Y: i32 = 63;
pub const HEIGHTMAP_MOTION_BLOCKING: i32 = 4;
pub const LIGHT_SECTION_COUNT: usize = WORLD_SECTION_COUNT + 2;

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

impl BitSet {
    fn write_for(
        &self,
        writer: &mut BinaryWriter,
        version: ProtocolVersion,
    ) -> Result<(), BinaryError> {
        if version >= ProtocolVersion::V26_3 {
            let mut bytes: Vec<u8> = self.0.iter().flat_map(|long| long.to_le_bytes()).collect();
            while bytes.last() == Some(&0) {
                bytes.pop();
            }
            VarInt::new(bytes.len() as i32).write(writer)?;
            writer.write_all(&bytes);
        } else {
            self.0.write(writer)?;
        }
        Ok(())
    }
}

impl WriteBytes for BitSet {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.0.write(writer)
    }
}
impl WriteBytes for LevelChunkWithLightPacket {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.chunk_x.write(writer)?;
        self.chunk_z.write(writer)?;
        self.heightmaps.write(writer)?;
        writer.write_byte_array(&self.chunk_data)?;
        VarInt::new(0).write(writer)?;
        self.sky_light_mask.write_for(writer, self.version)?;
        self.block_light_mask.write_for(writer, self.version)?;
        self.empty_sky_light_mask.write_for(writer, self.version)?;
        self.empty_block_light_mask
            .write_for(writer, self.version)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LevelChunkWithLightPacket {
    version: ProtocolVersion,
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
    pub fn empty(chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            version: ProtocolVersion::V26_3,
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

    pub fn flat(chunk_x: i32, chunk_z: i32, solid_block_state_id: i32) -> Self {
        Self::flat_for_version(
            ProtocolVersion::V26_3,
            chunk_x,
            chunk_z,
            solid_block_state_id,
        )
    }

    pub fn flat_for_version(
        version: ProtocolVersion,
        chunk_x: i32,
        chunk_z: i32,
        solid_block_state_id: i32,
    ) -> Self {
        Self {
            version,
            chunk_x,
            chunk_z,
            heightmaps: vec![Heightmap::motion_blocking(
                FLAT_SURFACE_Y - OVERWORLD_MIN_Y + 1,
            )],
            chunk_data: flat_world_chunk_data(Some(solid_block_state_id)),
            sky_light_mask: full_bitset(LIGHT_SECTION_COUNT),
            block_light_mask: BitSet::default(),
            empty_sky_light_mask: BitSet::default(),
            empty_block_light_mask: full_bitset(LIGHT_SECTION_COUNT),
            sky_light_arrays: vec![full_bright_array(); LIGHT_SECTION_COUNT],
            block_light_arrays: Vec::new(),
        }
    }
}

fn full_bitset(count: usize) -> BitSet {
    let longs_needed = count.div_ceil(64);
    let mut longs = vec![0i64; longs_needed];
    for i in 0..count {
        longs[i / 64] |= 1i64 << (i % 64);
    }
    BitSet(longs)
}

fn full_bright_array() -> [u8; 2048] {
    [0xFF; 2048]
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
        self.sky_light_mask.write_for(writer, self.version)?;
        self.block_light_mask.write_for(writer, self.version)?;
        self.empty_sky_light_mask.write_for(writer, self.version)?;
        self.empty_block_light_mask
            .write_for(writer, self.version)?;
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
    let bits = usize::from(bits_per_entry);
    let values_per_long = 64 / bits;
    let long_count = values.len().div_ceil(values_per_long);
    let mut data = vec![0u64; long_count];

    for (index, value) in values.iter().enumerate() {
        let long_index = index / values_per_long;
        let shift = (index % values_per_long) * bits;
        data[long_index] |= u64::from(*value) << shift;
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
    fn dump_flat_chunk() {
        let mut w = BinaryWriter::new();
        LevelChunkWithLightPacket::flat(0, 0, 1)
            .encode_payload(&mut w)
            .unwrap();
        let bytes = w.into_inner();
        println!("len = {}", bytes.len());
        println!("{:02x?}", &bytes[..540.min(bytes.len())]);
    }
}
