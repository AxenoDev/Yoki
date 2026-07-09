mod block_states;
mod known_packs;
mod mandatory;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use minecraft_packet::packets::configuration::{
    registry_data::{RegistryDataEntry, RegistryDataPacket},
    update_tags::{RegistryTags, TagEntry, UpdateTagsPacket},
};
use protocol_version::protocol_version::ProtocolVersion;
use serde::Deserialize;
use serde_json::Value;
use yoki_binutils::data_types::VarInt;
use yoki_binutils::data_types::{NbtTag, json_to_nbt};
use yoki_identifier::Identifier;

#[derive(Debug)]
pub struct RegistryStore {
    versions: HashMap<ProtocolVersion, VersionRegistries>,
}

#[derive(Debug)]
struct VersionRegistries {
    all_registry_data: Vec<RegistryDataPacket>,
    tags: UpdateTagsPacket,
    block_state_ids: BlockStateIds,
    player_entity_type_id: i32,
}

#[derive(Debug, Deserialize)]
struct DatapackReport {
    registries: HashMap<String, RegistryInfo>,
}

#[derive(Debug, Deserialize)]
struct RegistryInfo {
    elements: bool,
    #[allow(dead_code)]
    stable: bool,
    #[allow(dead_code)]
    tags: bool,
}

#[derive(Debug, Deserialize)]
struct RegistriesReport {
    #[serde(flatten)]
    registries: HashMap<String, RegistryValue>,
}

#[derive(Debug, Deserialize)]
struct RegistryValue {
    entries: HashMap<String, EntryValue>,
}

#[derive(Debug, Deserialize)]
struct EntryValue {
    protocol_id: i32,
}

#[derive(Debug, Deserialize)]
struct TagFile {
    values: Vec<String>,
}

impl RegistryStore {
    pub fn load() -> Result<Self, RegistryError> {
        let generated_dir = find_data_generated_dir()?;
        let mut versions = HashMap::new();

        for version in ProtocolVersion::ALL_VERSION {
            if matches!(version, ProtocolVersion::Any | ProtocolVersion::Unsupported) {
                continue;
            }

            let version_dir = generated_dir.join(data_folder(*version));
            if !version_dir.is_dir() {
                continue;
            }

            let version_registries = load_version(&version_dir)?;
            versions.insert(*version, version_registries);
        }

        if versions.is_empty() {
            return Err(RegistryError::NoVersions(generated_dir));
        }

        let mut loaded: Vec<_> = versions.keys().map(|v| v.humanize()).collect();
        loaded.sort();
        tracing::info!("loaded registry data for {}", loaded.join(", "));

        Ok(Self { versions })
    }

    pub fn registry_data_packets(
        &self,
        version: ProtocolVersion,
        omit_known_pack_entry_data: bool,
    ) -> impl Iterator<Item = RegistryDataPacket> + '_ {
        self.versions
            .get(&version.data())
            .into_iter()
            .flat_map(move |data| {
                data.all_registry_data.iter().map(move |packet| {
                    if !omit_known_pack_entry_data {
                        return packet.clone();
                    }

                    RegistryDataPacket::new(
                        packet.registry_id.clone(),
                        packet
                            .entries
                            .iter()
                            .map(|entry| RegistryDataEntry {
                                entry_id: entry.entry_id.clone(),
                                data: None,
                            })
                            .collect(),
                    )
                })
            })
    }

    pub fn tags_packet(&self, version: ProtocolVersion) -> UpdateTagsPacket {
        let Some(data) = self.versions.get(&version.data()) else {
            return UpdateTagsPacket {
                registries: Vec::new(),
            };
        };

        data.tags.clone()
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
}

fn load_version(version_dir: &Path) -> Result<VersionRegistries, RegistryError> {
    let datapack: DatapackReport = read_json(&version_dir.join("reports/datapack.json"))?;
    let registries_report: RegistriesReport =
        read_json(&version_dir.join("reports/registries.json"))?;
    let mandatory_registry_ids: HashSet<String> = mandatory::SYNCED_REGISTRY_IDS
        .iter()
        .map(|registry_id| (*registry_id).to_string())
        .collect();

    let mut id_lookup: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for (registry_id, registry) in &registries_report.registries {
        let registry_ids = registry
            .entries
            .iter()
            .map(|(entry_name, entry_value)| (entry_name.clone(), entry_value.protocol_id))
            .collect();
        id_lookup.insert(registry_id.clone(), registry_ids);
    }

    for (registry_id, info) in &datapack.registries {
        if !mandatory_registry_ids.contains(registry_id) || !info.elements {
            continue;
        }

        let entries = match list_element_entries(version_dir, registry_id) {
            Ok(entries) => entries,
            Err(RegistryError::MissingRegistryDir(_)) => continue,
            Err(err) => return Err(err),
        };

        let registry_ids = id_lookup.entry(registry_id.clone()).or_default();
        for (index, (full_name, _)) in entries.iter().enumerate() {
            registry_ids
                .entry(full_name.clone())
                .or_insert(index as i32);
        }
    }

    let mut all_registry_data = Vec::new();

    for (registry_id, info) in &datapack.registries {
        if !mandatory_registry_ids.contains(registry_id) {
            continue;
        }

        let entries = if info.elements {
            match build_element_registry(version_dir, registry_id, &id_lookup) {
                Ok(entries) => entries,
                Err(
                    RegistryError::MissingRegistryDir(_)
                    | RegistryError::EmptyRegistry(_)
                    | RegistryError::MissingRegistry(_),
                ) => continue,
                Err(err) => return Err(err),
            }
        } else {
            match build_static_registry(registry_id, &registries_report) {
                Ok(entries) => entries,
                Err(RegistryError::MissingRegistry(_)) => continue,
                Err(err) => return Err(err),
            }
        };

        let registry_identifier = Identifier::parse(registry_id)
            .map_err(|err| RegistryError::InvalidIdentifier(err.to_string()))?;
        let packet = RegistryDataPacket::new(registry_identifier, entries);
        all_registry_data.push(packet);
    }

    let tags = load_tags(version_dir, &id_lookup)?;

    let block_state_ids = BlockStateIds::load(version_dir)?;

    let player_entity_type_id = id_lookup
        .get("minecraft:entity_type")
        .and_then(|entries| entries.get("minecraft:player"))
        .copied()
        .ok_or_else(|| {
            RegistryError::MissingProtocolId("minecraft:entity_type/minecraft:player".to_string())
        })?;

    Ok(VersionRegistries {
        all_registry_data,
        tags,
        block_state_ids,
        player_entity_type_id,
    })
}

fn build_static_registry(
    registry_id: &str,
    registries_report: &RegistriesReport,
) -> Result<Vec<RegistryDataEntry>, RegistryError> {
    let registry = registries_report
        .registries
        .get(registry_id)
        .ok_or_else(|| RegistryError::MissingRegistry(registry_id.to_string()))?;

    let mut sorted_entries: Vec<_> = registry.entries.iter().collect();
    sorted_entries.sort_by_key(|(_, entry_value)| entry_value.protocol_id);

    let mut entries = Vec::with_capacity(sorted_entries.len());
    for (entry_name, entry_value) in sorted_entries {
        let mut entry_nbt = BTreeMap::new();
        entry_nbt.insert("name".to_string(), NbtTag::string(entry_name));
        entry_nbt.insert("id".to_string(), NbtTag::Int(entry_value.protocol_id));
        entries.push(RegistryDataEntry {
            entry_id: Identifier::parse(entry_name)
                .map_err(|err| RegistryError::InvalidIdentifier(err.to_string()))?,
            data: Some(NbtTag::Compound(entry_nbt)),
        });
    }

    Ok(entries)
}

fn build_element_registry(
    version_dir: &Path,
    registry_id: &str,
    id_lookup: &HashMap<String, HashMap<String, i32>>,
) -> Result<Vec<RegistryDataEntry>, RegistryError> {
    let entries = list_element_entries(version_dir, registry_id)?;
    let entry_ids = id_lookup
        .get(registry_id)
        .ok_or_else(|| RegistryError::MissingRegistry(registry_id.to_string()))?;

    if entries.is_empty() {
        return Err(RegistryError::EmptyRegistry(registry_id.to_string()));
    }

    let mut registry_entries = Vec::with_capacity(entries.len());
    for (full_name, path) in entries {
        entry_ids
            .get(&full_name)
            .ok_or_else(|| RegistryError::MissingProtocolId(full_name.clone()))?;

        let json: Value = read_json(&path)?;
        let entry_nbt = json_to_nbt(&json)
            .and_then(|tag| match tag {
                NbtTag::Compound(map) => Some(map),
                _ => None,
            })
            .unwrap_or_default();
        registry_entries.push(RegistryDataEntry {
            entry_id: Identifier::parse(&full_name)
                .map_err(|err| RegistryError::InvalidIdentifier(err.to_string()))?,
            data: Some(NbtTag::Compound(entry_nbt)),
        });
    }

    Ok(registry_entries)
}

fn list_element_entries(
    version_dir: &Path,
    registry_id: &str,
) -> Result<Vec<(String, PathBuf)>, RegistryError> {
    let registry_path = registry_id
        .strip_prefix("minecraft:")
        .unwrap_or(registry_id);
    let registry_dir = version_dir.join("data/minecraft").join(registry_path);
    if !registry_dir.is_dir() {
        return Err(RegistryError::MissingRegistryDir(registry_id.to_string()));
    }

    let mut entries = Vec::new();
    collect_element_paths(&registry_dir, &registry_dir, &mut entries)?;
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(entries)
}

fn collect_element_paths(
    base_dir: &Path,
    current_dir: &Path,
    entries: &mut Vec<(String, PathBuf)>,
) -> Result<(), RegistryError> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_element_paths(base_dir, &path, entries)?;
            continue;
        }

        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }

        let relative = path
            .strip_prefix(base_dir)
            .map_err(|_| RegistryError::InvalidEntry(path.display().to_string()))?;
        let entry_suffix = relative
            .with_extension("")
            .to_string_lossy()
            .replace('\\', "/");
        let full_name = format!("minecraft:{entry_suffix}");
        entries.push((full_name, path));
    }

    Ok(())
}

fn load_tags(
    version_dir: &Path,
    id_lookup: &HashMap<String, HashMap<String, i32>>,
) -> Result<UpdateTagsPacket, RegistryError> {
    let tags_root = version_dir.join("data/minecraft/tags");
    if !tags_root.is_dir() {
        return Ok(UpdateTagsPacket {
            registries: Vec::new(),
        });
    }

    let mut raw_tags: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for (registry_id, registry_dir) in tag_registry_dirs(&tags_root, id_lookup)? {
        let registry_tags = raw_tags.entry(registry_id).or_default();
        collect_tag_files(&registry_dir, "", registry_tags)?;
    }

    let mut registries = Vec::new();
    for (registry_id, tags) in raw_tags {
        let entry_ids = id_lookup.get(&registry_id);
        let mut resolved_tags = Vec::new();

        for (tag_name, values) in &tags {
            let mut entries = Vec::new();
            for protocol_id in resolve_tag_values(values, &tags, entry_ids)
                .into_iter()
                .flatten()
            {
                entries.push(VarInt::new(protocol_id));
            }

            resolved_tags.push(TagEntry {
                name: Identifier::parse(tag_name)
                    .map_err(|err| RegistryError::InvalidIdentifier(err.to_string()))?,
                entries,
            });
        }

        if resolved_tags.is_empty() {
            continue;
        }

        registries.push(RegistryTags {
            registry: Identifier::parse(&registry_id)
                .map_err(|err| RegistryError::InvalidIdentifier(err.to_string()))?,
            tags: resolved_tags,
        });
    }

    Ok(UpdateTagsPacket { registries })
}

fn tag_registry_dirs(
    tags_root: &Path,
    id_lookup: &HashMap<String, HashMap<String, i32>>,
) -> Result<Vec<(String, PathBuf)>, RegistryError> {
    let mut dirs = Vec::new();
    for entry in fs::read_dir(tags_root)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let registry_id = format!("minecraft:{name}");
        if id_lookup.contains_key(&registry_id) {
            dirs.push((registry_id, entry.path()));
            continue;
        }

        for sub_entry in fs::read_dir(entry.path())? {
            let sub_entry = sub_entry?;
            if !sub_entry.file_type()?.is_dir() {
                continue;
            }

            let sub_name = sub_entry.file_name().to_string_lossy().to_string();
            let registry_id = format!("minecraft:{name}/{sub_name}");
            if id_lookup.contains_key(&registry_id) {
                dirs.push((registry_id, sub_entry.path()));
            }
        }
    }
    Ok(dirs)
}

fn collect_tag_files(
    dir: &Path,
    tag_prefix: &str,
    tags: &mut HashMap<String, Vec<String>>,
) -> Result<(), RegistryError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let segment = path.file_name().unwrap().to_string_lossy();
            let next_prefix = if tag_prefix.is_empty() {
                segment.to_string()
            } else {
                format!("{tag_prefix}/{segment}")
            };
            collect_tag_files(&path, &next_prefix, tags)?;
            continue;
        }

        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }

        let tag_name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| RegistryError::InvalidEntry(path.display().to_string()))?;
        let full_tag_name = if tag_prefix.is_empty() {
            format!("minecraft:{tag_name}")
        } else {
            format!("minecraft:{tag_prefix}/{tag_name}")
        };

        let tag_file: TagFile = read_json(&path)?;
        tags.insert(full_tag_name, tag_file.values);
    }

    Ok(())
}

fn resolve_tag_values(
    values: &[String],
    all_tags: &HashMap<String, Vec<String>>,
    entry_ids: Option<&HashMap<String, i32>>,
) -> Vec<Option<i32>> {
    let mut resolved = Vec::new();
    let mut pending: Vec<String> = values.to_vec();

    while let Some(value) = pending.pop() {
        if let Some(tag_name) = value.strip_prefix('#') {
            let tag_key = if tag_name.contains(':') {
                tag_name.to_string()
            } else {
                format!("minecraft:{tag_name}")
            };
            if let Some(nested_values) = all_tags.get(&tag_key) {
                pending.extend(nested_values.iter().cloned());
            }
            continue;
        }

        let entry_name = if value.contains(':') {
            value
        } else {
            format!("minecraft:{value}")
        };

        if let Some(ids) = entry_ids
            && let Some(protocol_id) = ids.get(&entry_name)
        {
            resolved.push(Some(*protocol_id));
        }
    }

    resolved
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, RegistryError> {
    let raw = fs::read_to_string(path)
        .map_err(|err| RegistryError::Io(path.display().to_string(), err))?;
    serde_json::from_str(&raw).map_err(|err| RegistryError::Json(path.display().to_string(), err))
}

fn find_data_generated_dir() -> Result<PathBuf, RegistryError> {
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd);
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(exe_dir) = exe.parent()
    {
        roots.push(exe_dir.to_path_buf());
    }
    roots.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")));

    for root in roots {
        let mut current = root;
        loop {
            let candidate = current.join("data/generated");
            if candidate.is_dir() {
                return Ok(candidate);
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
    }

    Err(RegistryError::MissingGeneratedData)
}

fn data_folder(version: ProtocolVersion) -> String {
    format!("V{}", version.humanize().replace('.', "_"))
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("could not find data/generated directory")]
    MissingGeneratedData,
    #[error("no registry versions found in {0}")]
    NoVersions(PathBuf),
    #[error("io error while reading {0}: {1}")]
    Io(String, #[source] std::io::Error),
    #[error("json error in {0}: {1}")]
    Json(String, #[source] serde_json::Error),
    #[error("missing registry {0}")]
    MissingRegistry(String),
    #[error("missing registry directory for {0}")]
    MissingRegistryDir(String),
    #[error("empty registry {0}")]
    EmptyRegistry(String),
    #[error("missing protocol id for {0}")]
    MissingProtocolId(String),
    #[error("invalid registry entry {0}")]
    InvalidEntry(String),
    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),
}

impl From<std::io::Error> for RegistryError {
    fn from(err: std::io::Error) -> Self {
        Self::Io("registry io".into(), err)
    }
}

use crate::registry::block_states::BlockStateIds;
pub use known_packs::{client_accepted_known_pack, known_packs_for_version};
pub use mandatory::should_omit_known_pack_entry_data;
