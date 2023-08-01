use std::collections::HashMap;
use crate::block::lib::*;
use crate::lib::Identifier::Identifier;

#[derive(Clone)]
pub struct StoneBlock {}

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
    fn current_state() -> Identifier {
        Identifier {id: "blockstate:{stone:stone}".to_string()}
    }
}

