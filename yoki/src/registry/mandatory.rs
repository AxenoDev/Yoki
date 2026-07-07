use protocol_version::protocol_version::ProtocolVersion;

pub const SYNCED_REGISTRY_IDS: &[&str] = &[
    "minecraft:banner_pattern",
    "minecraft:cat_sound_variant",
    "minecraft:cat_variant",
    "minecraft:chicken_sound_variant",
    "minecraft:chicken_variant",
    "minecraft:cow_sound_variant",
    "minecraft:cow_variant",
    "minecraft:damage_type",
    "minecraft:dialog",
    "minecraft:dimension_type",
    "minecraft:frog_variant",
    "minecraft:instrument",
    "minecraft:jukebox_song",
    "minecraft:painting_variant",
    "minecraft:pig_sound_variant",
    "minecraft:pig_variant",
    "minecraft:timeline",
    "minecraft:trim_material",
    "minecraft:wolf_sound_variant",
    "minecraft:wolf_variant",
    "minecraft:world_clock",
    "minecraft:worldgen/biome",
    "minecraft:zombie_nautilus_variant",
];

pub fn should_omit_known_pack_entry_data(
    version: ProtocolVersion,
    client_accepted_known_pack: bool,
) -> bool {
    client_accepted_known_pack
        && matches!(
            version.data(),
            ProtocolVersion::V26_1 | ProtocolVersion::V26_2
        )
}
