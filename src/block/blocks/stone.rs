use std::collections::HashMap;
use crate::block::lib::*;
use crate::lib::Identifier::Identifier;

#[derive(Clone)]
pub struct StoneBlock {
    current_state: Identifier
}

impl Block for StoneBlock {
    fn block_id() -> Identifier {
        Identifier {id: "block:{stone}".to_string()}
    }
}
impl BlockStateable for StoneBlock {
    fn states() -> HashMap<Identifier, BlockState> {
        let mut out = HashMap::default();
        
        out.insert(Identifier {id: "blockstate:{stone:stone}".to_string()},
        BlockState {state_image: "stone/stone.png".to_string()});         

        out
    }
    fn value(&self) -> Identifier {
        self.current_state
    }
    fn set_value(&mut self, value: Identifier) {
        self.current_state = value
    }
}

