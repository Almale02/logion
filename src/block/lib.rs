use bevy::prelude::*;

use crate::lib::Identifier::Identifier;
use std::collections::HashMap;

// SECTION: BLOCK_STATE
pub struct BlockState {
    pub state_image: String,
}
// SECTION: BLOCK
pub trait Block {
    fn block_id(&self) -> Identifier;
    fn states(&self) -> HashMap<Identifier, BlockState>;
    fn state(&self) -> &Identifier;
    fn set_state(&mut self, value: Identifier);
}
