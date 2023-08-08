use bevy::prelude::Component;

use crate::block::lib::*;
use crate::lib::Identifier::Identifier;
use crate::material::lib::MaterialType;
use crate::material::materials::m_dirt::DirtMaterial;
use std::collections::HashMap;

#[derive(Clone, Component, Debug)]
pub struct DirtBlock {
    render_type: BlockRenderType,
    grass_facings: [GrassFacing; 4],
    materials: HashMap<MaterialType, u8>,
}
impl DirtBlock {
    pub fn make_grassy(&mut self, left: bool, right: bool, top: bool) {
        if top && !left && !right {
            self.set_rendertype(BlockRenderType::BlockState(
                "image/dirt/grass_state/dirt_grass_t.png".to_string(),
            ))
        }
        if top && left && !right {
            self.set_rendertype(BlockRenderType::BlockState(
                "image/dirt/grass_state/dirt_grass_tl.png".to_string(),
            ))
        }
        if top && !left && right {
            self.set_rendertype(BlockRenderType::BlockState(
                "image/dirt/grass_state/dirt_grass_tr.png".to_string(),
            ))
        }
        if top && left && right {
            self.set_rendertype(BlockRenderType::BlockState(
                "image/dirt/grass_state/dirt_grass_tlr.png".to_string(),
            ))
        }
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

        add_block_state(&mut out, "dirt", "dirt", "image/dirt/dirt.png");
        add_grass_state(&mut out, "dirt", "dirt_grass_t", &self.grass_facings[0]);
        add_grass_state(&mut out, "dirt", "dirt_grass_tl", &self.grass_facings[1]);
        add_grass_state(&mut out, "dirt", "dirt_grass_tr", &self.grass_facings[2]);
        add_grass_state(&mut out, "dirt", "dirt_grass_tlr", &self.grass_facings[3]);

        out
    }
    fn render_type(&self) -> &BlockRenderType {
        &self.render_type
    }
    fn set_rendertype(&mut self, value: BlockRenderType) {
        self.render_type = value
    }
    fn gen_materials(&mut self, _x: usize, _y: usize) -> &HashMap<MaterialType, u8> {
        let mut map: HashMap<MaterialType, u8> = HashMap::default();

        map.insert(MaterialType::Dirt(DirtMaterial::default()), 80);
        map.insert(MaterialType::Dirt(DirtMaterial::default()), 20);
        self.materials = map;

        &self.materials
    }
    fn get_materials(&self) -> &HashMap<MaterialType, u8> {
        &self.materials
    }
}

impl Default for DirtBlock {
    fn default() -> Self {
        DirtBlock {
            render_type: BlockRenderType::BlockState("image/dirt/dirt.png".to_string()),
            grass_facings: [
                GrassFacing::Top("image/dirt/grass_state/dirt_grass_t.png".to_string()),
                GrassFacing::TopLeft("image/dirt/grass_state/dirt_grass_tl.png".to_string()),
                GrassFacing::TopRight("image/dirt/grass_state/dirt_grass_tr.png".to_string()),
                GrassFacing::TopLeftRight("image/dirt/grass_state/dirt_grass_tlr.png".to_string()),
            ],
            materials: default_mat(),
        }
    }
}
fn default_mat() -> HashMap<MaterialType, u8> {
    let mut map: HashMap<MaterialType, u8> = HashMap::default();

    map.insert(MaterialType::Dirt(DirtMaterial::default()), 100);

    map
}
