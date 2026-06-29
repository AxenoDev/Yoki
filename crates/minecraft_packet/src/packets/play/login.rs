use crate::packets::play::data::DeathLocation;
use yoki_binutils::data_types::VarInt;
use yoki_identifier::Identifier;
use yoki_macros::PacketOut;

#[derive(Debug, Clone, PacketOut)]
pub struct LoginPacket {
    entity_id: i32,
    is_hardcore: bool,
    dimension_names: Vec<Identifier>,
    max_players: VarInt,
    view_distance: VarInt,
    simulation_distance: VarInt,
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
    do_limited_crafting: bool,
    dimension_type: VarInt,
    dimension_name: Identifier,
    hashed_seed: i64,
    game_mode: u8,
    previous_game_mode: i8,
    is_debug: bool,
    is_flat: bool,
    death_location: Option<DeathLocation>,
    portal_cooldown: VarInt,
    v1_21_2_sea_level: VarInt,
    v26_2_online_mode: bool,
    v1_20_5_enforces_secure_chat: bool,
}
