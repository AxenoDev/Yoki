use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use crate::packets::play::data::DeathLocation;
use protocol_version::protocol_version::ProtocolVersion;
use yoki_binutils::{
    BinaryWriter, ProtocolError, WriteBytes,
    data_types::VarInt,
};
use yoki_identifier::Identifier;

#[derive(Debug, Clone)]
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
    /// Present for protocol 26.2+ (`online_mode` field).
    online_mode: Option<bool>,
    enforces_secure_chat: bool,
}

impl Default for LoginPacket {
    fn default() -> Self {
        Self::for_protocol_version(ProtocolVersion::V26_2)
    }
}

impl LoginPacket {
    pub fn for_protocol_version(version: ProtocolVersion) -> Self {
        let online_mode = matches!(version.data(), ProtocolVersion::V26_2).then_some(false);

        Self {
            entity_id: 1,
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
            online_mode,
            enforces_secure_chat: false,
        }
    }
}

impl PacketMeta for LoginPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for LoginPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.entity_id.write(writer)?;
        self.is_hardcore.write(writer)?;
        self.dimension_names.write(writer)?;
        self.max_players.write(writer)?;
        self.view_distance.write(writer)?;
        self.simulation_distance.write(writer)?;
        self.reduced_debug_info.write(writer)?;
        self.enable_respawn_screen.write(writer)?;
        self.do_limited_crafting.write(writer)?;
        self.dimension_type.write(writer)?;
        self.dimension_name.write(writer)?;
        self.hashed_seed.write(writer)?;
        self.game_mode.write(writer)?;
        self.previous_game_mode.write(writer)?;
        self.is_debug.write(writer)?;
        self.is_flat.write(writer)?;
        self.death_location.write(writer)?;
        self.portal_cooldown.write(writer)?;
        self.sea_level.write(writer)?;
        if let Some(online_mode) = self.online_mode {
            online_mode.write(writer)?;
        }
        self.enforces_secure_chat.write(writer)?;
        Ok(())
    }
}
