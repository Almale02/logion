use bevy::prelude::*;

use std::collections::HashMap;
use crate::lib::Identifier::{Identifier};

// SECTION: BLOCK_STATE
pub struct BlockState {
    pub state_image: String,
}
// SECTION: BLOCK
pub trait Block {
    fn block_id() -> Identifier;
}
// SECTION: BLOCK_STATEABLE
pub trait BlockStateable {
    fn states() -> HashMap<Identifier, BlockState>;
    fn state(&self) -> Identifier;
    fn set_state(&mut self, value: Identifier);

}