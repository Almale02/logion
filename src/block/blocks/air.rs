use std::collections::HashMap;

use crate::block::lib::*;
use crate::lib::Identifier::Identifier;
use crate::material::lib::MaterialType;

#[derive(Clone, Debug)]
pub struct AirBlock {
    render_type: BlockRenderType,
}
impl Block for AirBlock {
    fn block_id(&self) -> Identifier {
        Identifier {
            id: String::from("block:{air}}"),
        }
    }
    fn states(&self) -> HashMap<Identifier, BlockState> {
        unreachable!()
    }
    fn render_type(&self) -> &BlockRenderType {
        &self.render_type
    }
    fn set_rendertype(&mut self, value: BlockRenderType) {
        unreachable!()
    }
    fn get_materials(&self) -> &HashMap<MaterialType, u8> {
        unreachable!()
    }
    fn gen_materials(&mut self, x: usize, y: usize) -> &HashMap<MaterialType, u8> {
        unreachable!()
    }
}
impl Default for AirBlock {
    fn default() -> Self {
        AirBlock {
            render_type: BlockRenderType::None(),
        }
    }
}
