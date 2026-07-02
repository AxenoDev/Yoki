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
    sea_level: VarInt,
    enforces_secure_chat: bool,
}

impl Default for LoginPacket {
    fn default() -> Self {
        Self {
            entity_id: 0,
            is_hardcore: false,
            dimension_names: vec![
                Identifier::minecraft("overworld").expect("valid identifier"),
                Identifier::minecraft("the_nether").expect("valid identifier"),
                Identifier::minecraft("the_end").expect("valid identifier"),
            ],
            max_players: VarInt::new(20),
            view_distance: VarInt::new(10),
            simulation_distance: VarInt::new(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt::new(0),
            dimension_name: Identifier::minecraft("overworld").expect("valid identifier"),
            hashed_seed: 0,
            game_mode: 0,
            previous_game_mode: -1,
            is_debug: false,
            is_flat: false,
            death_location: None,
            portal_cooldown: VarInt::new(0),
            sea_level: VarInt::new(63),
            enforces_secure_chat: true,
        }
    }
}

impl LoginPacket {
    pub fn new(
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
        sea_level: VarInt,
        enforces_secure_chat: bool,
    ) -> Self {
        Self {
            entity_id,
            is_hardcore,
            dimension_names,
            max_players,
            view_distance,
            simulation_distance,
            reduced_debug_info,
            enable_respawn_screen,
            do_limited_crafting,
            dimension_type,
            dimension_name,
            hashed_seed,
            game_mode,
            previous_game_mode,
            is_debug,
            is_flat,
            death_location,
            portal_cooldown,
            sea_level,
            enforces_secure_chat,
        }
    }
}
