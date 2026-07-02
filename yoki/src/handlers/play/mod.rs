use minecraft_packet::packets::play::{
    game_event::GameEventPacket,
    keep_alive::{ClientboundKeepAlivePacket, ServerboundKeepAlivePacket},
    login::LoginPacket,
    player_abilities::PlayerAbilitiesPacket,
    set_chunk_cache_center::SetChunkCacheCenterPacket,
    synchronize_player_position::SynchronizePlayerPositionPacket,
};
use minecraft_packet::packets::play::level_chunk_with_light::LevelChunkWithLightPacket;
use protocol_version::protocol_version::ProtocolVersion;
use crate::{
    ServerState,
    batch::Batch,
    server::packet_registry::PacketRegistry,
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

const SPAWN_X: f64 = 0.5;
const SPAWN_Y: f64 = 65.0;
const SPAWN_Z: f64 = 0.5;

pub fn queue_join_play_packets(batch: &mut Batch, protocol_version: ProtocolVersion, server_state: &ServerState) {

    let stone_id = server_state
        .registry_store()
        .block_state_id(protocol_version, "minecraft:stone")
        .expect("stone block state id should exist");

    batch.queue_packet(PacketRegistry::Login(LoginPacket::for_protocol_version(
        protocol_version,
    )));
    batch.queue_packet(PacketRegistry::PlayerAbilities(PlayerAbilitiesPacket::default()));
    batch.queue_packet(PacketRegistry::SynchronizePlayerPosition(
        SynchronizePlayerPositionPacket::spawn(SPAWN_X, SPAWN_Y, SPAWN_Z, 0.0, 0.0),
    ));
    batch.queue_packet(PacketRegistry::SetChunkCacheCenter(SetChunkCacheCenterPacket::new(
        0, 0,
    )));
    batch.queue_packet(PacketRegistry::LevelChunkWithLight(LevelChunkWithLightPacket::flat(0, 0, stone_id)));
    batch.queue_packet(PacketRegistry::GameEvent(GameEventPacket::start_waiting_for_chunks()));
}

impl PacketHandler for ServerboundKeepAlivePacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        batch.queue_packet(PacketRegistry::ClientboundKeepAlive(
            ClientboundKeepAlivePacket::new(self.id),
        ));
        Ok(batch)
    }
}
