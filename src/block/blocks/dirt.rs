use bevy::prelude::Component;

use crate::block::lib::*;
use crate::lib::identifier::Identifier;
use crate::material::lib::{MaterialGenList, MaterialType};
use crate::material::materials::m_dirt::DirtMaterial;

use crate::material::materials::m_stone::StoneMaterial;
use std::collections::HashMap;

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash)]
pub struct DirtBlock {
    render_type: BlockRenderType,
    grass_facings: [GrassFacing; 4],
    materials: MaterialGenList,
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
        Identifier::new(Identifier::BLOCK, "game:dirt")
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
    fn gen_materials(&mut self, _x: usize, _y: usize, multiplyer: f32) -> &MaterialGenList {
        /*if let BlockRenderType::BlockState(x) = self.render_type() {
            if x != "image/dirt/dirt.png" {
                self.materials
                    .add_material(MaterialType::Dirt(DirtMaterial::default()), 100, true);
                return &self.materials;
            }
        }*/
        // TODO: make it so you dont need to specify the material count for default materials.
        self.materials
            .add_material(
                MaterialType::Stone(StoneMaterial::default()),
                (15. * multiplyer) as u8,
                false,
            )
            //.add_material(MaterialType::Iron(IronMaterial::default()), 11, false)
            .add_material(MaterialType::Dirt(DirtMaterial::default()), 0, true);

        &self.materials
    }
    fn get_materials(&self) -> &MaterialGenList {
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
            materials: MaterialGenList::default(),
        }
    }
}
