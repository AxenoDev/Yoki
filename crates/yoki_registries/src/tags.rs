use crate::error::RegistryError;
use crate::loader::{IdLookup, parse_identifier};
use crate::report::read_json;
use crate::types::{RegistryTags, TagEntry};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct TagFile {
    values: Vec<String>,
}

pub fn load_tags(
    version_dir: &Path,
    id_lookup: &IdLookup,
) -> Result<Vec<RegistryTags>, RegistryError> {
    let tags_root = version_dir.join("data/minecraft/tags");
    if !tags_root.is_dir() {
        return Ok(Vec::new());
    }

    let mut raw: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    for (registry_id, dir) in tag_registry_dirs(&tags_root, id_lookup)? {
        collect_tag_files(&dir, "", raw.entry(registry_id).or_default())?;
    }

    let mut registries = Vec::new();
    for (registry_id, tags) in raw {
        let entry_ids = id_lookup.get(&registry_id);
        let mut resolved = Vec::new();
        for (name, values) in &tags {
            let mut seen = HashSet::new();
            let entries = resolve_tag_values(values, &tags, entry_ids, &mut seen);
            resolved.push(TagEntry {
                name: parse_identifier(name)?,
                entries,
            });
        }
        if resolved.is_empty() {
            continue;
        }
        resolved.sort_by(|a, b| a.name.to_string().cmp(&b.name.to_string()));
        registries.push(RegistryTags {
            registry: parse_identifier(&registry_id)?,
            tags: resolved,
        });
    }
    registries.sort_by(|a, b| a.registry.to_string().cmp(&b.registry.to_string()));
    Ok(registries)
}

fn tag_registry_dirs(
    tags_root: &Path,
    id_lookup: &IdLookup,
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
        for sub in fs::read_dir(entry.path())? {
            let sub = sub?;
            if !sub.file_type()?.is_dir() {
                continue;
            }
            let registry_id = format!("minecraft:{name}/{}", sub.file_name().to_string_lossy());
            if id_lookup.contains_key(&registry_id) {
                dirs.push((registry_id, sub.path()));
            }
        }
    }
    Ok(dirs)
}

fn collect_tag_files(
    dir: &Path,
    prefix: &str,
    tags: &mut HashMap<String, Vec<String>>,
) -> Result<(), RegistryError> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let segment = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .ok_or_else(|| RegistryError::InvalidEntry(path.display().to_string()))?;
            let next = if prefix.is_empty() {
                segment
            } else {
                format!("{prefix}/{segment}")
            };
            collect_tag_files(&path, &next, tags)?;
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|n| n.to_str())
            .ok_or_else(|| RegistryError::InvalidEntry(path.display().to_string()))?;
        let full = if prefix.is_empty() {
            format!("minecraft:{stem}")
        } else {
            format!("minecraft:{prefix}/{stem}")
        };
        let file: TagFile = read_json(&path)?;
        tags.insert(full, file.values);
    }
    Ok(())
}

fn resolve_tag_values(
    values: &[String],
    all_tags: &HashMap<String, Vec<String>>,
    entry_ids: Option<&HashMap<String, i32>>,
    seen_tags: &mut HashSet<String>,
) -> Vec<i32> {
    let mut resolved = Vec::new();
    for value in values {
        if let Some(tag) = value.strip_prefix('#') {
            let key = qualify(tag);
            if seen_tags.insert(key.clone())
                && let Some(nested) = all_tags.get(&key)
            {
                resolved.extend(resolve_tag_values(nested, all_tags, entry_ids, seen_tags));
            }
            continue;
        }
        if let Some(id) = entry_ids.and_then(|ids| ids.get(&qualify(value))) {
            resolved.push(*id);
        }
    }
    resolved
}

fn qualify(name: &str) -> String {
    if name.contains(':') {
        name.to_string()
    } else {
        format!("minecraft:{name}")
    }
}
