use std::collections::HashMap;

use crate::block::lib::*;
use crate::lib::Identifier::Identifier;
use crate::material::lib::{MaterialGenList, MaterialType};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    fn set_rendertype(&mut self, _value: BlockRenderType) {
        unreachable!()
    }
    fn get_materials(&self) -> &MaterialGenList {
        unreachable!()
    }
    fn gen_materials(&mut self, _x: usize, _y: usize, multiplyer: f32) -> &MaterialGenList {
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
