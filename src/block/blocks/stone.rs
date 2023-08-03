use bevy::prelude::Component;

use crate::block::lib::*;
use crate::lib::Identifier::Identifier;
use std::collections::HashMap;

#[derive(Clone, Copy, Component)]
pub struct StoneBlock {
    current_state: Identifier,
}

impl Block for StoneBlock {
    fn block_id(&self) -> Identifier {
        Identifier {
            id: "block:{stone}".to_string(),
        }
    }
    fn states(&self) -> HashMap<Identifier, BlockState> {
        let mut out = HashMap::default();

        out.insert(
            Identifier {
                id: "blockstate:{stone:stone}".to_string(),
            },
            BlockState {
                state_image: "stone/stone.png".to_string(),
            },
        );

        out
    }
    fn state(&self) -> &Identifier {
        &self.current_state
    }
    fn set_state(&mut self, value: Identifier) {
        self.current_state = value
    }
}

impl Default for StoneBlock {
    fn default() -> Self {
        StoneBlock {
            current_state: Identifier {
                id: "blockstate:{stone:stone}".to_string(),
            },
        }
    }
}
