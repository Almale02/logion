use std::collections::HashMap;

use crate::block::lib::*;
use crate::lib::Identifier::Identifier;

#[derive(Clone)]
pub struct AirBlock {
    current_state: Identifier,
}
impl Block for AirBlock {
    fn block_id(&self) -> Identifier {
        Identifier {
            id: String::from("block:{air}}"),
        }
    }
    fn states(&self) -> HashMap<Identifier, BlockState> {
        let mut out = HashMap::default();

        out.insert(
            Identifier {
                id: "air".to_string(),
            },
            BlockState {
                state_image: "air".to_string(),
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
impl Default for AirBlock {
    fn default() -> Self {
        AirBlock {
            current_state: Identifier {
                id: "air".to_string(),
            },
        }
    }
}
