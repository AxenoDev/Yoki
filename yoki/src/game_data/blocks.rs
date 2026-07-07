use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BlockEntry {
    states: Vec<BlockState>,
}

#[derive(Debug, Deserialize)]
struct BlockState {
    id: i32,
    #[serde(default)]
    default: bool,
}

pub struct BlockRegistry {
    default_state_ids: HashMap<String, i32>,
}

impl BlockRegistry {
    pub fn load_from_file(path: &str) -> Self {
        let raw = std::fs::read_to_string(path).expect("blocks.json introuvable");
        let parsed: HashMap<String, BlockEntry> =
            serde_json::from_str(&raw).expect("blocks.json invalide");

        let mut default_state_ids = HashMap::new();
        for (name, entry) in parsed {
            let default_id = entry.states.iter()
                .find(|s| s.default)
                .map(|s| s.id)
                .or_else(|| {
                    if entry.states.len() == 1 {
                        Some(entry.states[0].id)
                    } else {
                        None
                    }
                });
            if let Some(id) = default_id {
                default_state_ids.insert(name, id);
            }
        }
        Self { default_state_ids }
    }

    pub fn default_state_id(&self, block_name: &str) -> i32 {
        *self
            .default_state_ids
            .get(block_name)
            .unwrap_or_else(|| panic!("bloc inconnu: {block_name}"))
    }
}