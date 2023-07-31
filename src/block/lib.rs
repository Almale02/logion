use bevy::prelude::*;

use crate::lib::Identifier::{Identifiable, Identifier};

// INFO: BLOCK_STATE // easier to find then in this way
pub struct BlockState {
    pub state_image: Handle<Image>,
    pub state_id: Identifier<BlockState>,
}
impl Identifiable for BlockState {
    fn id_prefix() -> String {
        String::from("block_state:")
    }
}

// INFO: BLOCK
pub struct Block {
    pub state_list: Vec<BlockState>,
    pub default_state: Identifier<Block>,
}
impl Identifiable for Block {
    fn id_prefix() -> String {
        String::from("block:")
    }
}
