use minecraft_packet::packets::configuration::{
    acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket,
    client_information::ClientInformationPacket,
    finish_configuration::FinishConfigurationPacket,
    known_packs::SelectKnownPacksPacket,
    plugin_message::PluginMessagePacket,
    update_enabled_features::UpdateEnabledFeaturesPacket,
};
use minecraft_protocol::State;
use yoki_identifier::Identifier;

use crate::{
    ServerState,
    batch::Batch,
    handlers::play::queue_join_play_packets,
    registry::{client_accepted_known_pack, should_omit_known_pack_entry_data},
    server::packet_registry::PacketRegistry,
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
        batch.queue_packet(PacketRegistry::FinishConfiguration(FinishConfigurationPacket));

        Ok(batch)
    }
}

impl PacketHandler for AcknowledgeFinishConfigurationPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        _server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError> {
        let mut batch = Batch::new();

        batch.queue_both_state_change(State::Play);
        queue_join_play_packets(&mut batch, client_state.protocol_version());

        Ok(batch)
    }
}
