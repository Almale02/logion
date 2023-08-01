use std::collections::HashMap;
use crate::block::lib::*;
use crate::lib::Identifier::Identifier;

#[derive(Clone)]
pub struct DirtBlock {}

impl Block for DirtBlock {
    fn block_id() -> Identifier {
        Identifier{id: String::from("block:{dirt}}")}
    }    
}
impl BlockStateable for DirtBlock {
    fn states() -> HashMap<Identifier, BlockState> {
        let mut out = HashMap::default();
        
        out.insert(Identifier {id: "blockstate:{dirt:dirt}".to_string()},
        BlockState {state_image: "dirt/dirt.png".to_string()});

        out
    }
    fn current_state() -> Identifier {
        Identifier {id: "blockstate:{dirt:dirt}".to_string()}
    }
}

