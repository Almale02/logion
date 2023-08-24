use bevy::prelude::Component;

use crate::block::lib::*;
use crate::lib::Identifier::Identifier;
use crate::material::lib::{MaterialGenList, MaterialType};
use crate::material::materials::m_dirt::DirtMaterial;
use crate::material::materials::m_stone::StoneMaterial;
use std::collections::HashMap;

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash)]
pub struct StoneBlock {
    render_type: BlockRenderType,
    materials: MaterialGenList,
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
                state_image: "image/stone/stone.png".to_string(),
            },
        );

        out
    }
    fn render_type(&self) -> &BlockRenderType {
        &self.render_type
    }
    fn set_rendertype(&mut self, value: BlockRenderType) {
        self.render_type = value
    }
    fn gen_materials(&mut self, _x: usize, _y: usize, multiplyer: f32) -> &MaterialGenList {
        self.materials
            .add_material(MaterialType::Stone(StoneMaterial::default()), 80, true);

        &self.materials
    }
    fn get_materials(&self) -> &MaterialGenList {
        &self.materials
    }
}

impl Default for StoneBlock {
    fn default() -> Self {
        StoneBlock {
            render_type: BlockRenderType::BlockState("image/stone/stone.png".to_string()),
            materials: MaterialGenList::default(),
        }
    }
}
