use crate::block::lib::*;
use crate::lib::Identifier::Identifier;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DirtBlock {
    current_state: Identifier,
    grass_facings: [GrassFacing; 4]
}
impl DirtBlock {
    pub fn make_grassy(&mut self) {
        self.current_state = Identifier {id: "blockstate:{grass:dirt_grass_t}".to_string()}
    }
}

impl Block for DirtBlock {
    fn block_id(&self) -> Identifier {
        Identifier {
            id: String::from("block:{dirt}}"),
        }
    }
    fn states(&self) -> HashMap<Identifier, BlockState> {
        let mut out = HashMap::default();

        add_block_state(&mut out, "dirt", "dirt", "dirt/dirt.png");
        add_grass_state(&mut out, "dirt", "dirt_grass_t", &self.grass_facings[0]);
        add_grass_state(&mut out, "dirt", "dirt_grass_tl", &self.grass_facings[1]);
        add_grass_state(&mut out, "dirt", "dirt_grass_tr", &self.grass_facings[2]);
        add_grass_state(&mut out, "dirt", "dirt_grass_tlr", &self.grass_facings[3]);
                
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
            grass_facings: [
                GrassFacing::Top("dirt/grass_state/dirt_grass_t.png".to_string()),
                GrassFacing::TopLeft("dirt/grass_state/dirt_grass_tl.png".to_string()),
                GrassFacing::TopRight("dirt/grass_state/dirt_grass_tr.png".to_string()),
                GrassFacing::TopLeftRight("dirt/grass_state/dirt_grass_tlr.png".to_string()),
            ]
        }
    }
}