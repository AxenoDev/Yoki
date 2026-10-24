use protocol_version::protocol_version::ProtocolVersion;

#[derive(Debug, Clone, Copy)]
pub struct SyncedRegistry {
    pub id: &'static str,
    pub min_version: ProtocolVersion,
}

const fn reg(id: &'static str, min_version: ProtocolVersion) -> SyncedRegistry {
    SyncedRegistry { id, min_version }
}

use ProtocolVersion::{V26_1, V26_3};

pub const SYNCED_REGISTRIES: &[SyncedRegistry] = &[
    reg("minecraft:banner_pattern", V26_1),
    reg("minecraft:block_transformer", V26_3),
    reg("minecraft:cat_sound_variant", V26_1),
    reg("minecraft:cat_variant", V26_1),
    reg("minecraft:chicken_sound_variant", V26_1),
    reg("minecraft:chicken_variant", V26_1),
    reg("minecraft:cow_sound_variant", V26_1),
    reg("minecraft:cow_variant", V26_1),
    reg("minecraft:damage_type", V26_1),
    reg("minecraft:decorated_pot_pattern", V26_3),
    reg("minecraft:dialog", V26_1),
    reg("minecraft:dimension_type", V26_1),
    reg("minecraft:frog_variant", V26_1),
    reg("minecraft:instrument", V26_1),
    reg("minecraft:jukebox_song", V26_1),
    reg("minecraft:painting_variant", V26_1),
    reg("minecraft:pig_sound_variant", V26_1),
    reg("minecraft:pig_variant", V26_1),
    reg("minecraft:timeline", V26_1),
    reg("minecraft:trim_material", V26_1),
    reg("minecraft:wolf_sound_variant", V26_1),
    reg("minecraft:wolf_variant", V26_1),
    reg("minecraft:world_clock", V26_1),
    reg("minecraft:worldgen/biome", V26_1),
    reg("minecraft:zombie_nautilus_variant", V26_1),
];

pub fn synced_registries_for(
    version: ProtocolVersion,
) -> impl Iterator<Item = &'static SyncedRegistry> {
    let number = version.data().protocol_number();
    SYNCED_REGISTRIES
        .iter()
        .filter(move |registry| number >= registry.min_version.protocol_number())
}
