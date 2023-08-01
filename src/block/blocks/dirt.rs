use crate::block::lib::*;
use crate::lib::Identifier::Identifier;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DirtBlock {
    current_state: Identifier,
}

impl Block for DirtBlock {
    fn block_id(&self) -> Identifier {
        Identifier {
            id: String::from("block:{dirt}}"),
        }
    }
    fn states(&self) -> HashMap<Identifier, BlockState> {
        let mut out = HashMap::default();

        out.insert(
            Identifier {
                id: "blockstate:{dirt:dirt}".to_string(),
            },
            BlockState {
                state_image: "dirt/dirt.png".to_string(),
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

impl Default for DirtBlock {
    fn default() -> Self {
        DirtBlock {
            current_state: Identifier {
                id: "blockstate:{dirt:dirt}".to_string(),
            },
        }
    }
}
