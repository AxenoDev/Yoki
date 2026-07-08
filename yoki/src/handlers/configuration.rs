use minecraft_packet::packets::configuration::{
    acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket,
    client_information::ClientInformationPacket, finish_configuration::FinishConfigurationPacket,
    known_packs::SelectKnownPacksPacket, plugin_message::PluginMessagePacket,
    update_enabled_features::UpdateEnabledFeaturesPacket,
};
use minecraft_protocol::State;
use yoki_identifier::Identifier;

use std::sync::Arc;

use crate::{
    ServerState,
    batch::Batch,
    handlers::play::{queue_join_play_packets, queue_player_spawns, spawn_position},
    registry::{client_accepted_known_pack, should_omit_known_pack_entry_data},
    server::packet_registry::PacketRegistry,
    server::players::ConnectedPlayer,
    server::{ClientState, PacketHandler, packet_handler::PacketHandlerError},
};

impl PacketHandler for ClientInformationPacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let _ = self;
        Ok(Batch::new())
    }
}

impl PacketHandler for PluginMessagePacket {
    fn handle(
        &self,
        _client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let _ = self;
        Ok(Batch::new())
    }
}

impl PacketHandler for SelectKnownPacksPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        let protocol_version = client_state.protocol_version();
        let accepted_known_pack = client_accepted_known_pack(&self.known_packs, protocol_version);
        let omit_registry_data =
            should_omit_known_pack_entry_data(protocol_version, accepted_known_pack);

        let registry_packets: Vec<_> = server_state
            .registry_store()
            .registry_data_packets(protocol_version, omit_registry_data)
            .map(PacketRegistry::RegistryData)
            .collect();
        batch.chain_iter(registry_packets);

        batch.queue_packet(PacketRegistry::UpdateEnabledFeatures(
            UpdateEnabledFeaturesPacket {
                features: vec![Identifier::minecraft("vanilla").expect("valid identifier")],
            },
        ));
        batch.queue_packet(PacketRegistry::UpdateTags(
            server_state.registry_store().tags_packet(protocol_version),
        ));
        batch.queue_packet(PacketRegistry::FinishConfiguration(
            FinishConfigurationPacket,
        ));

        Ok(batch)
    }
}

impl PacketHandler for AcknowledgeFinishConfigurationPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();
        let protocol_version = client_state.protocol_version();

        let Some(profile) = client_state.game_profile() else {
            return Err(PacketHandlerError::disconnect(
                "missing game profile before entering play",
            ));
        };
        let Some(sender) = client_state.sender() else {
            return Err(PacketHandlerError::disconnect("missing connection writer"));
        };

        let entity_id = server_state.players().allocate_entity_id();
        client_state.set_entity_id(entity_id);

        batch.queue_both_state_change(State::Play);
        queue_join_play_packets(&mut batch, protocol_version, server_state, entity_id);

        let player = Arc::new(ConnectedPlayer::new(
            entity_id,
            profile,
            protocol_version,
            spawn_position(),
            sender,
        ));
        let others = server_state.players().register(Arc::clone(&player));
        queue_player_spawns(&mut batch, server_state, &player, &others);

        Ok(batch)
    }
}
