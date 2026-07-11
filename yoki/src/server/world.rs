use std::collections::HashMap;
use std::sync::RwLock;

use minecraft_packet::packets::play::data::BlockPos;

pub const AIR_STATE: i32 = 0;

#[derive(Debug, Default)]
pub struct World {
    blocks: RwLock<HashMap<BlockPos, i32>>,
}

impl World {
    pub fn set_block(&self, pos: BlockPos, block_state_id: i32) {
        self.blocks
            .write()
            .expect("world lock poisoned")
            .insert(pos, block_state_id);
    }

    pub fn get_block(&self, pos: BlockPos) -> Option<i32> {
        self.blocks
            .read()
            .expect("world lock poisoned")
            .get(&pos)
            .copied()
    }

    pub fn modifications(&self) -> Vec<(BlockPos, i32)> {
        self.blocks
            .read()
            .expect("world lock poisoned")
            .iter()
            .map(|(pos, state)| (*pos, *state))
            .collect()
    }
}
