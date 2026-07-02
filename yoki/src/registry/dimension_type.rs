use std::collections::BTreeMap;

use minecraft_packet::packets::configuration::registry_data::{RegistryDataEntry, RegistryDataPacket};
use yoki_binutils::data_types::NbtTag;
use yoki_identifier::Identifier;

pub fn overworld_dimension_type_nbt() -> NbtTag {
    let mut entries = BTreeMap::new();

    entries.insert("height".to_string(), NbtTag::Int(384));
    entries.insert("min_y".to_string(), NbtTag::Int(-64));
    entries.insert("has_skylight".to_string(), NbtTag::byte(true));
    entries.insert("has_ceiling".to_string(), NbtTag::byte(false));
    entries.insert("ambient_light".to_string(), NbtTag::Float(0.0));
    entries.insert("piglin_safe".to_string(), NbtTag::byte(false));
    entries.insert("respawn_anchor_works".to_string(), NbtTag::byte(false));
    entries.insert("bed_works".to_string(), NbtTag::byte(true));
    entries.insert("natural".to_string(), NbtTag::byte(true));
    entries.insert("coordinate_scale".to_string(), NbtTag::Double(1.0));
    entries.insert("ultrawarm".to_string(), NbtTag::byte(false));
    entries.insert("has_raids".to_string(), NbtTag::byte(true));
    entries.insert("logical_height".to_string(), NbtTag::Int(384));
    entries.insert(
        "infiniburn".to_string(),
        NbtTag::string("#minecraft:infiniburn_overworld"),
    );
    entries.insert(
        "effects".to_string(),
        NbtTag::string("minecraft:overworld"),
    );
    entries.insert("monster_spawn_light_level".to_string(), NbtTag::Int(0));
    entries.insert(
        "monster_spawn_block_light_limit".to_string(),
        NbtTag::Int(0),
    );

    NbtTag::compound(entries)
}

pub fn overworld_dimension_type_registry() -> RegistryDataPacket {
    RegistryDataPacket::new(
        Identifier::minecraft("dimension_type").expect("valid identifier"),
        vec![RegistryDataEntry {
            entry_id: Identifier::minecraft("overworld").expect("valid identifier"),
            data: Some(overworld_dimension_type_nbt()),
        }],
    )
}