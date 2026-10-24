use minecraft_packet::packets::configuration::{
    known_packs::KnownPack,
    registry_data::{RegistryDataEntry, RegistryDataPacket},
    update_tags::{RegistryTags, TagEntry, UpdateTagsPacket},
};
use protocol_version::protocol_version::ProtocolVersion;
use yoki_binutils::data_types::VarInt;

pub use yoki_registries::RegistryStore;

pub fn known_packs_for_version(version: ProtocolVersion) -> Vec<KnownPack> {
    version
        .data()
        .known_packs()
        .iter()
        .map(|pack_version| KnownPack {
            namespace: "minecraft".to_string(),
            id: "core".to_string(),
            version: (*pack_version).to_string(),
        })
        .collect()
}

pub fn client_accepted_known_pack(client_packs: &[KnownPack], version: ProtocolVersion) -> bool {
    let expected_versions = version.data().known_packs();
    client_packs.iter().any(|pack| {
        pack.namespace == "minecraft"
            && pack.id == "core"
            && expected_versions.contains(&pack.version.as_str())
    })
}

pub fn should_omit_known_pack_entry_data(
    version: ProtocolVersion,
    client_accepted_known_pack: bool,
) -> bool {
    client_accepted_known_pack
        && matches!(
            version.data(),
            ProtocolVersion::V26_1 | ProtocolVersion::V26_2 | ProtocolVersion::V26_3
        )
}

pub trait RegistryStorePackets {
    fn registry_data_packets(
        &self,
        version: ProtocolVersion,
        omit_known_pack_entry_data: bool,
    ) -> Vec<RegistryDataPacket>;

    fn tags_packet(&self, version: ProtocolVersion) -> UpdateTagsPacket;
}

impl RegistryStorePackets for RegistryStore {
    fn registry_data_packets(
        &self,
        version: ProtocolVersion,
        omit_known_pack_entry_data: bool,
    ) -> Vec<RegistryDataPacket> {
        self.registry_data(version, omit_known_pack_entry_data)
            .map(|registry| {
                RegistryDataPacket::new(
                    registry.registry_id,
                    registry
                        .entries
                        .into_iter()
                        .map(|entry| RegistryDataEntry {
                            entry_id: entry.entry_id,
                            data: entry.data,
                        })
                        .collect(),
                )
            })
            .collect()
    }

    fn tags_packet(&self, version: ProtocolVersion) -> UpdateTagsPacket {
        UpdateTagsPacket {
            registries: self
                .tags(version)
                .into_iter()
                .map(|registry| RegistryTags {
                    registry: registry.registry,
                    tags: registry
                        .tags
                        .into_iter()
                        .map(|tag| TagEntry {
                            name: tag.name,
                            entries: tag.entries.into_iter().map(VarInt::new).collect(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
