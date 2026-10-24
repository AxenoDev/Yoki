use crate::block_states::BlockStateIds;
use crate::error::RegistryError;
use crate::loader::{
    IdLookup, build_element_registry, build_id_lookup, build_static_registry, list_element_files,
};
use crate::locate::find_data_generated_dir;
use crate::report::{DatapackReport, RegistriesReport, read_json};
use crate::synced::synced_registries_for;
use crate::tags::load_tags;
use crate::types::{RegistryData, RegistryEntry, RegistryTags};
use protocol_version::protocol_version::ProtocolVersion;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RegistryStore {
    versions: HashMap<ProtocolVersion, VersionRegistries>,
}

#[derive(Debug)]
struct VersionRegistries {
    registry_data: Vec<RegistryData>,
    tags: Vec<RegistryTags>,
    block_state_ids: BlockStateIds,
    player_entity_type_id: i32,
    command_argument_type_ids: HashMap<String, i32>,
    item_names: HashMap<i32, String>,
}

impl RegistryStore {
    pub fn load() -> Result<Self, RegistryError> {
        Self::load_from(&find_data_generated_dir()?)
    }

    pub fn load_from(generated_dir: &Path) -> Result<Self, RegistryError> {
        let mut versions = HashMap::new();

        for version in ProtocolVersion::ALL_VERSION {
            if matches!(version, ProtocolVersion::Any | ProtocolVersion::Unsupported) {
                continue;
            }
            if version.data() != *version {
                continue;
            }
            let version_dir = generated_dir.join(data_folder(*version));
            if !version_dir.is_dir() {
                continue;
            }
            versions.insert(*version, load_version(*version, &version_dir)?);
        }

        if versions.is_empty() {
            return Err(RegistryError::NoVersions(generated_dir.to_path_buf()));
        }

        let mut loaded: Vec<_> = versions.keys().map(|v| v.humanize()).collect();
        loaded.sort();
        tracing::info!("loaded registry data for {}", loaded.join(", "));
        Ok(Self { versions })
    }

    pub fn is_loaded(&self, version: ProtocolVersion) -> bool {
        self.versions.contains_key(&version.data())
    }

    pub fn registry_data(
        &self,
        version: ProtocolVersion,
        omit_known_pack_entry_data: bool,
    ) -> impl Iterator<Item = RegistryData> + '_ {
        self.versions
            .get(&version.data())
            .into_iter()
            .flat_map(move |data| {
                data.registry_data.iter().map(move |registry| {
                    if !omit_known_pack_entry_data {
                        return registry.clone();
                    }
                    RegistryData {
                        registry_id: registry.registry_id.clone(),
                        entries: registry
                            .entries
                            .iter()
                            .map(|entry| RegistryEntry {
                                entry_id: entry.entry_id.clone(),
                                data: None,
                            })
                            .collect(),
                    }
                })
            })
    }

    pub fn tags(&self, version: ProtocolVersion) -> Vec<RegistryTags> {
        self.versions
            .get(&version.data())
            .map(|data| data.tags.clone())
            .unwrap_or_default()
    }

    pub fn block_state_id(&self, version: ProtocolVersion, block_name: &str) -> Option<i32> {
        self.versions
            .get(&version.data())
            .and_then(|data| data.block_state_ids.default_state_id(block_name))
    }

    pub fn player_entity_type_id(&self, version: ProtocolVersion) -> Option<i32> {
        self.versions
            .get(&version.data())
            .map(|data| data.player_entity_type_id)
    }

    pub fn command_argument_type_id(&self, version: ProtocolVersion, name: &str) -> Option<i32> {
        self.versions
            .get(&version.data())
            .and_then(|data| data.command_argument_type_ids.get(name))
            .copied()
    }

    pub fn item_block_state_id(&self, version: ProtocolVersion, item_id: i32) -> Option<i32> {
        let data = self.versions.get(&version.data())?;
        let name = data.item_names.get(&item_id)?;
        data.block_state_ids.default_state_id(name)
    }
}

fn load_version(
    version: ProtocolVersion,
    version_dir: &Path,
) -> Result<VersionRegistries, RegistryError> {
    let datapack: DatapackReport = read_json(&version_dir.join("reports/datapack.json"))?;
    let report: RegistriesReport = read_json(&version_dir.join("reports/registries.json"))?;
    let mut id_lookup: IdLookup = build_id_lookup(&report);

    let mut registry_data = Vec::new();
    for synced in synced_registries_for(version) {
        let is_element_registry = datapack
            .registries
            .get(synced.id)
            .is_some_and(|info| info.elements);

        let built = if is_element_registry {
            build_element_registry(version_dir, synced.id)
        } else {
            build_static_registry(synced.id, &report)
        };

        match built {
            Ok(data) => registry_data.push(data),
            Err(
                RegistryError::MissingRegistryDir(_)
                | RegistryError::EmptyRegistry(_)
                | RegistryError::MissingRegistry(_),
            ) => {
                return Err(RegistryError::MandatoryRegistryMissing {
                    registry: synced.id.to_string(),
                    version: version.humanize().to_string(),
                });
            }
            Err(err) => return Err(err),
        }

        if is_element_registry && let Ok(files) = list_element_files(version_dir, synced.id) {
            let ids = id_lookup.entry(synced.id.to_string()).or_default();
            for (index, (name, _)) in files.iter().enumerate() {
                ids.entry(name.clone()).or_insert(index as i32);
            }
        }
    }

    let tags = load_tags(version_dir, &id_lookup)?;
    let block_state_ids = BlockStateIds::load(version_dir)?;

    let player_entity_type_id = id_lookup
        .get("minecraft:entity_type")
        .and_then(|entries| entries.get("minecraft:player"))
        .copied()
        .ok_or_else(|| {
            RegistryError::MissingProtocolId("minecraft:entity_type/minecraft:player".into())
        })?;

    let command_argument_type_ids = id_lookup
        .get("minecraft:command_argument_type")
        .cloned()
        .ok_or_else(|| RegistryError::MissingRegistry("minecraft:command_argument_type".into()))?;

    let item_names = id_lookup
        .get("minecraft:item")
        .map(|entries| {
            entries
                .iter()
                .map(|(name, id)| (*id, name.clone()))
                .collect()
        })
        .ok_or_else(|| RegistryError::MissingRegistry("minecraft:item".into()))?;

    Ok(VersionRegistries {
        registry_data,
        tags,
        block_state_ids,
        player_entity_type_id,
        command_argument_type_ids,
        item_names,
    })
}

fn data_folder(version: ProtocolVersion) -> PathBuf {
    PathBuf::from(format!("V{}", version.humanize().replace('.', "_")))
}
