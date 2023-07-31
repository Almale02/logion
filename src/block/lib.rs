use bevy::prelude::*;

use crate::lib::Identifier::{Identifiable, Identifier};

// SECTION: BLOCK_STATE
pub struct BlockState {
    pub state_image: String,
    pub state_id: Identifier<BlockState>,
}
impl Identifiable for BlockState {
    fn id_prefix() -> String {
        String::from("block_state:")
    }
}

// SECTION: BLOCK
pub struct Block {
    pub state_list: Vec<BlockState>,
    pub default_state: Identifier<BlockState>,
    pub block_id: Identifier<Block>,
}
impl Identifiable for Block {
    fn id_prefix() -> String {
        String::from("block:")
    }
}
