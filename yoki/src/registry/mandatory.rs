use protocol_version::protocol_version::ProtocolVersion;

/// Registries that must be synchronized during configuration.
///
/// Mirrors PicoLimbo's mandatory `RegistryKeys` set: vanilla clients already
/// know the rest via the `minecraft:core` known pack.
pub fn registry_ids_for_version(version: ProtocolVersion) -> &'static [&'static str] {
    match version {
        ProtocolVersion::V26_2 => &[
            "minecraft:banner_pattern",
            "minecraft:block",
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
        ],
        ProtocolVersion::V26_1 => &[
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
        ],
        ProtocolVersion::Any | ProtocolVersion::Unsupported => &[],
    }
}

/// Registries whose tags must be sent during configuration (since 1.21.6).
pub const TAGGED_REGISTRY_IDS: &[&str] = &[
    "minecraft:banner_pattern",
    "minecraft:block",
    "minecraft:damage_type",
    "minecraft:dialog",
    "minecraft:timeline",
];

pub fn should_omit_known_pack_entry_data(
    version: ProtocolVersion,
    client_accepted_known_pack: bool,
) -> bool {
    client_accepted_known_pack
        && matches!(version.data(), ProtocolVersion::V26_1 | ProtocolVersion::V26_2)
}
