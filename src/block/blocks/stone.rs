use bevy::prelude::Component;

use crate::block::lib::*;
use crate::lib::Identifier::Identifier;
use crate::material::lib::MaterialType;
use crate::material::materials::m_stone::StoneMaterial;
use std::collections::HashMap;

#[derive(Clone, Component, Debug)]
pub struct StoneBlock {
    render_type: BlockRenderType,
    materials: HashMap<MaterialType, u8>,
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
    fn gen_materials(
        &mut self,
        _x: usize,
        _y: usize,
    ) -> &HashMap<crate::material::lib::MaterialType, u8> {
        let mut map: HashMap<MaterialType, u8> = HashMap::default();

        map.insert(MaterialType::Stone(StoneMaterial::default()), 100);

        self.materials = map;

        &self.materials
    }
    fn get_materials(&self) -> &HashMap<MaterialType, u8> {
        &self.materials
    }
}

impl Default for StoneBlock {
    fn default() -> Self {
        StoneBlock {
            render_type: BlockRenderType::BlockState("image/stone/stone.png".to_string()),
            materials: default_mat(),
        }
    }
}

fn default_mat() -> HashMap<MaterialType, u8> {
    let mut map: HashMap<MaterialType, u8> = HashMap::default();

    map.insert(MaterialType::Stone(StoneMaterial::default()), 100);

    map
}
