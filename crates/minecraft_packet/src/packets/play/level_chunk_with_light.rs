use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt, BinaryError};

pub const WORLD_SECTION_COUNT: usize = 24;

/// Types de heightmap standards (voir protocole).
pub const HEIGHTMAP_MOTION_BLOCKING: i32 = 5;

#[derive(Debug, Clone)]
pub struct Heightmap {
    kind: VarInt,
    data: Vec<i64>,
}

impl Heightmap {
    /// Heightmap vide (tout à zéro) pour un chunk plat/air.
    pub fn empty(kind: i32) -> Self {
        Self {
            kind: VarInt::new(kind),
            // ceil(16*16 * bits_per_entry / 64), bits_per_entry dépend de la
            // hauteur du monde. 37 est une approximation correcte pour
            // un monde de 384 blocs de haut (9 bits/entrée).
            data: vec![0i64; 37],
        }
    }
}

impl WriteBytes for Heightmap {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.kind.write(writer)?;
        self.data.write(writer)?;
        Ok(())
    }
}

/// BitSet façon protocole Minecraft: VarInt de longueur + tableau de i64.
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
    /// Chunk minimal vide (que de l'air), sans lumière — pour débloquer
    /// le client de l'écran "Loading terrain...".
    pub fn empty(chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            chunk_x,
            chunk_z,
            heightmaps: vec![Heightmap::empty(HEIGHTMAP_MOTION_BLOCKING)],
            chunk_data: empty_chunk_sections(),
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

        // Heightmaps: Prefixed Array of Heightmap.
        self.heightmaps.write(writer)?;

        // Data: Prefixed Array of Byte.
        writer.write_byte_array(&self.chunk_data)?;

        // Block Entities: aucun.
        VarInt::new(0).write(writer)?;

        // Light Data.
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

/// Sérialise WORLD_SECTION_COUNT sections vides (que de l'air).
fn empty_chunk_sections() -> Vec<u8> {
    let mut w = BinaryWriter::new();
    for _ in 0..WORLD_SECTION_COUNT {
        // Block count: 0 (aucun bloc non-air).
        w.write_all(&0i16.to_be_bytes());

        // Block states paletted container: bits_per_entry=0 -> single value.
        w.write_byte(0);
        VarInt::new(0).write(&mut w).ok(); // palette = air (id 0)
        VarInt::new(0).write(&mut w).ok(); // data array length = 0

        // Biomes paletted container: bits_per_entry=0 -> single value.
        w.write_byte(0);
        VarInt::new(0).write(&mut w).ok(); // id de biome (ex: plains)
        VarInt::new(0).write(&mut w).ok(); // data array length = 0
    }
    w.into_inner()
}