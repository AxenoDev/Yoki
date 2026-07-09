mod messages;

use minecraft_packet::packets::play::level_chunk_with_light::{
    FLAT_SURFACE_Y, LevelChunkWithLightPacket,
};
use minecraft_packet::packets::play::{
    add_entity::AddEntityPacket,
    animate::{ANIMATION_SWING_MAIN_ARM, ANIMATION_SWING_OFFHAND, AnimatePacket},
    entity_position_sync::EntityPositionSyncPacket,
    game_event::GameEventPacket,
    keep_alive::ServerboundKeepAlivePacket,
    login::LoginPacket,
    move_player::{
        FLAG_ON_GROUND, MovePlayerPosPacket, MovePlayerPosRotPacket, MovePlayerRotPacket,
        MovePlayerStatusOnlyPacket,
    },
    player_abilities::PlayerAbilitiesPacket,
    player_info_remove::PlayerInfoRemovePacket,
    player_info_update::{PlayerInfoEntry, PlayerInfoUpdatePacket},
    remove_entities::RemoveEntitiesPacket,
    rotate_head::RotateHeadPacket,
    set_chunk_cache_center::SetChunkCacheCenterPacket,
    swing::{HAND_OFF, SwingPacket},
    synchronize_player_position::SynchronizePlayerPositionPacket,
};
use protocol_version::protocol_version::ProtocolVersion;
use std::sync::Arc;

use crate::{
    ServerState,
    batch::Batch,
    server::packet_registry::PacketRegistry,
    server::players::{ConnectedPlayer, PlayerPosition},
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

const SPAWN_X: f64 = 0.5;
const SPAWN_Y: f64 = (FLAT_SURFACE_Y + 1) as f64;
const SPAWN_Z: f64 = 0.5;
const CHUNK_RADIUS: i32 = 4;

pub const fn spawn_position() -> PlayerPosition {
    PlayerPosition {
        x: SPAWN_X,
        y: SPAWN_Y,
        z: SPAWN_Z,
        yaw: 0.0,
        pitch: 0.0,
        on_ground: true,
    }
}

pub fn queue_join_play_packets(
    batch: &mut Batch,
    protocol_version: ProtocolVersion,
    server_state: &ServerState,
    entity_id: i32,
) {
    let stone_id = server_state
        .registry_store()
        .block_state_id(protocol_version, "minecraft:stone")
        .expect("stone block state id should exist");

    batch.queue_packet(PacketRegistry::Login(LoginPacket::for_protocol_version(
        protocol_version,
        entity_id,
    )));
    batch.queue_packet(PacketRegistry::PlayerAbilities(
        PlayerAbilitiesPacket::default(),
    ));
    batch.queue_packet(PacketRegistry::GameEvent(
        GameEventPacket::start_waiting_for_chunks(),
    ));
    batch.queue_packet(PacketRegistry::SetChunkCacheCenter(
        SetChunkCacheCenterPacket::new(0, 0),
    ));
    for chunk_x in -CHUNK_RADIUS..=CHUNK_RADIUS {
        for chunk_z in -CHUNK_RADIUS..=CHUNK_RADIUS {
            batch.queue_packet(PacketRegistry::LevelChunkWithLight(
                LevelChunkWithLightPacket::flat(chunk_x, chunk_z, stone_id),
            ));
        }
    }
    batch.queue_packet(PacketRegistry::SynchronizePlayerPosition(
        SynchronizePlayerPositionPacket::spawn(SPAWN_X, SPAWN_Y, SPAWN_Z, 0.0, 0.0),
    ));
}

fn info_entry(player: &ConnectedPlayer) -> PlayerInfoEntry {
    PlayerInfoEntry {
        uuid: player.profile().uuid(),
        name: player.profile().username().to_string(),
    }
}

fn spawn_packet(
    server_state: &ServerState,
    player: &ConnectedPlayer,
    recipient_version: ProtocolVersion,
) -> PacketRegistry {
    let entity_type = server_state
        .registry_store()
        .player_entity_type_id(recipient_version)
        .expect("player entity type id should exist");
    let position = player.position();
    PacketRegistry::AddEntity(AddEntityPacket::player(
        player.entity_id(),
        player.profile().uuid(),
        entity_type,
        position.x,
        position.y,
        position.z,
        position.yaw,
        position.pitch,
    ))
}

pub fn queue_player_spawns(
    batch: &mut Batch,
    server_state: &ServerState,
    new_player: &ConnectedPlayer,
    others: &[Arc<ConnectedPlayer>],
) {
    batch.queue_packet(PacketRegistry::PlayerInfoUpdate(
        PlayerInfoUpdatePacket::add_players(vec![info_entry(new_player)]),
    ));

    if others.is_empty() {
        return;
    }

    let entries = others.iter().map(|other| info_entry(other)).collect();
    batch.queue_packet(PacketRegistry::PlayerInfoUpdate(
        PlayerInfoUpdatePacket::add_players(entries),
    ));
    for other in others {
        batch.queue_packet(spawn_packet(
            server_state,
            other,
            new_player.protocol_version(),
        ));
    }

    for other in others {
        other.send_packet(PacketRegistry::PlayerInfoUpdate(
            PlayerInfoUpdatePacket::add_players(vec![info_entry(new_player)]),
        ));
        other.send_packet(spawn_packet(
            server_state,
            new_player,
            other.protocol_version(),
        ));
    }
}

pub fn handle_player_disconnect(server_state: &ServerState, entity_id: i32) {
    let Some(player) = server_state.players().unregister(entity_id) else {
        return;
    };

    tracing::info!(
        "{} left the game ({} online)",
        player.profile().username(),
        server_state.players().get_size()
    );

    let uuid = player.profile().uuid();
    server_state.broadcast(format!("§e{} left the game!", player.profile().username()));
    for other in server_state.players().others(entity_id) {
        other.send_packet(PacketRegistry::RemoveEntities(RemoveEntitiesPacket::new([
            entity_id,
        ])));
        other.send_packet(PacketRegistry::PlayerInfoRemove(
            PlayerInfoRemovePacket::new(vec![uuid]),
        ));
    }
}

fn broadcast_movement(
    client_state: &ClientState,
    server_state: &ServerState,
    update: impl FnOnce(&mut PlayerPosition),
    rotated: bool,
) {
    let Some(entity_id) = client_state.entity_id() else {
        return;
    };
    let Some(player) = server_state.players().get(entity_id) else {
        return;
    };

    let position = player.update_position(update);
    tracing::trace!(
        "entity {entity_id} moved to ({:.2}, {:.2}, {:.2}) yaw {:.1} pitch {:.1}",
        position.x,
        position.y,
        position.z,
        position.yaw,
        position.pitch
    );
    for other in server_state.players().others(entity_id) {
        other.send_packet(PacketRegistry::EntityPositionSync(
            EntityPositionSyncPacket::new(
                entity_id,
                position.x,
                position.y,
                position.z,
                position.yaw,
                position.pitch,
                position.on_ground,
            ),
        ));
        if rotated {
            other.send_packet(PacketRegistry::RotateHead(RotateHeadPacket::new(
                entity_id,
                position.yaw,
            )));
        }
    }
}

impl PacketHandler for MovePlayerPosPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        broadcast_movement(
            client_state,
            server_state,
            |position| {
                position.x = self.x;
                position.y = self.y;
                position.z = self.z;
                position.on_ground = self.flags & FLAG_ON_GROUND != 0;
            },
            false,
        );
        Ok(Batch::new())
    }
}

impl PacketHandler for MovePlayerPosRotPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        broadcast_movement(
            client_state,
            server_state,
            |position| {
                position.x = self.x;
                position.y = self.y;
                position.z = self.z;
                position.yaw = self.yaw;
                position.pitch = self.pitch;
                position.on_ground = self.flags & FLAG_ON_GROUND != 0;
            },
            true,
        );
        Ok(Batch::new())
    }
}

impl PacketHandler for MovePlayerRotPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        broadcast_movement(
            client_state,
            server_state,
            |position| {
                position.yaw = self.yaw;
                position.pitch = self.pitch;
                position.on_ground = self.flags & FLAG_ON_GROUND != 0;
            },
            true,
        );
        Ok(Batch::new())
    }
}

impl PacketHandler for MovePlayerStatusOnlyPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let flags = self.flags;
        if let Some(entity_id) = client_state.entity_id()
            && let Some(player) = server_state.players().get(entity_id)
        {
            player.update_position(|position| position.on_ground = flags & FLAG_ON_GROUND != 0);
        }
        Ok(Batch::new())
    }
}

impl PacketHandler for SwingPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        if let Some(entity_id) = client_state.entity_id() {
            let animation = if self.hand == HAND_OFF {
                ANIMATION_SWING_OFFHAND
            } else {
                ANIMATION_SWING_MAIN_ARM
            };
            tracing::trace!("entity {entity_id} swings (hand {})", self.hand);
            for other in server_state.players().others(entity_id) {
                other.send_packet(PacketRegistry::Animate(AnimatePacket::new(
                    entity_id, animation,
                )));
            }
        }
        Ok(Batch::new())
    }
}

impl PacketHandler for ServerboundKeepAlivePacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        tracing::trace!("keep alive echo (id {})", self.id);
        Ok(Batch::new())
    }
}
