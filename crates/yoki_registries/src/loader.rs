use crate::error::RegistryError;
use crate::report::{RegistriesReport, read_json};
use crate::types::{RegistryData, RegistryEntry};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use yoki_binutils::data_types::{NbtTag, json_to_nbt};
use yoki_identifier::Identifier;

pub type IdLookup = HashMap<String, HashMap<String, i32>>;

pub fn parse_identifier(value: &str) -> Result<Identifier, RegistryError> {
    Identifier::parse(value).map_err(|err| RegistryError::InvalidIdentifier(err.to_string()))
}

pub fn build_id_lookup(report: &RegistriesReport) -> IdLookup {
    report
        .registries
        .iter()
        .map(|(registry_id, registry)| {
            let ids = registry
                .entries
                .iter()
                .map(|(name, entry)| (name.clone(), entry.protocol_id))
                .collect();
            (registry_id.clone(), ids)
        })
        .collect()
}

pub fn build_element_registry(
    version_dir: &Path,
    registry_id: &str,
) -> Result<RegistryData, RegistryError> {
    let files = list_element_files(version_dir, registry_id)?;
    if files.is_empty() {
        return Err(RegistryError::EmptyRegistry(registry_id.to_string()));
    }

    let mut entries = Vec::with_capacity(files.len());
    for (full_name, path) in files {
        let json: serde_json::Value = read_json(&path)?;
        let nbt = json_to_nbt(&json)
            .ok_or_else(|| RegistryError::InvalidNbt(path.display().to_string()))?;
        entries.push(RegistryEntry {
            entry_id: parse_identifier(&full_name)?,
            data: Some(nbt),
        });
    }

    Ok(RegistryData {
        registry_id: parse_identifier(registry_id)?,
        entries,
    })
}

pub fn build_static_registry(
    registry_id: &str,
    report: &RegistriesReport,
) -> Result<RegistryData, RegistryError> {
    let registry = report
        .registries
        .get(registry_id)
        .ok_or_else(|| RegistryError::MissingRegistry(registry_id.to_string()))?;

    let mut sorted: Vec<_> = registry.entries.iter().collect();
    sorted.sort_by_key(|(_, entry)| entry.protocol_id);

    let mut entries = Vec::with_capacity(sorted.len());
    for (name, entry) in sorted {
        let mut compound = std::collections::BTreeMap::new();
        compound.insert("name".to_string(), NbtTag::string(name));
        compound.insert("id".to_string(), NbtTag::Int(entry.protocol_id));
        entries.push(RegistryEntry {
            entry_id: parse_identifier(name)?,
            data: Some(NbtTag::Compound(compound)),
        });
    }

    Ok(RegistryData {
        registry_id: parse_identifier(registry_id)?,
        entries,
    })
}

pub fn list_element_files(
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
        let path = entry?.path();

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
        let suffix = relative
            .with_extension("")
            .to_string_lossy()
            .replace('\\', "/");
        entries.push((format!("minecraft:{suffix}"), path));
    }
    Ok(())
}
