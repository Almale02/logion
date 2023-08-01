 use crate::block::lib::*;
 use crate::lib::Identifier::Identifier;

 #[derive(Clone)]
 pub struct AirBlock {}

 impl Block for AirBlock {
    fn block_id() -> Identifier {
        Identifier {id: String::from("block:{air}}")}
    }
 }