use std::collections::HashMap;
use std::path::Path;
use serde::Deserialize;
use crate::registry::RegistryError;

#[derive(Debug, Deserialize)]
struct BlockEntry {
    states: Vec<BlockStateEntry>,
}

#[derive(Debug, Deserialize)]
struct BlockStateEntry {
    id: i32,
    #[serde(default)]
    default: bool,
}

#[derive(Debug)]
pub struct BlockStateIds {
    default_state_ids: HashMap<String, i32>,
}

impl BlockStateIds {
    pub fn load(version_dir: &Path) -> Result<Self, RegistryError> {
        let path = version_dir.join("reports/blocks.json");
        let raw = std::fs::read_to_string(&path)
            .map_err(|err| RegistryError::Io(path.display().to_string(), err))?;
        let parsed: HashMap<String, BlockEntry> = serde_json::from_str(&raw)
            .map_err(|err| RegistryError::Json(path.display().to_string(), err))?;

        let mut default_state_ids = HashMap::new();
        for (name, entry) in parsed {
            if let Some(default) = entry.states.iter().find(|s| s.default) {
                default_state_ids.insert(name, default.id);
            }
        }

        Ok(Self { default_state_ids })
    }

    pub fn default_state_id(&self, block_name: &str) -> Option<i32> {
        self.default_state_ids.get(block_name).copied()
    }
}