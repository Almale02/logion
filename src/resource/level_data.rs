
use crate::material::lib::MaterialType;
use crate::material::materials::m_dirt::DirtMaterial;
use crate::material::materials::m_stone::StoneMaterial;

use std::collections::HashMap;

use ordered_float::OrderedFloat;


use crate::block::{block_type::BlockType, blocks::air::AirBlock};

use crate::lib::USVec2::*;

use bevy::prelude::*;

#[derive(Resource)]
pub struct LevelData {
    pub grid_unit: u8,
    pub world_size: USVec2,
    pub block_gird: [[Entity; 90]; 30],
    pub terrain_list: Vec<USVec2>,
    pub gen_grid: Vec<Vec<GenTileGridData>>,
}
impl Default for LevelData {
    fn default() -> Self {
        LevelData {
            grid_unit: 32,
            world_size: USVec2 { x: 90, y: 30 },
            gen_grid: vec![vec![GenTileGridData::default(); 90]; 30],
            terrain_list: Vec::default(),
            block_gird: [[Entity::PLACEHOLDER; 90]; 30],
        }
    }
}
impl LevelData {
    pub fn grid_to_global_space_unit(&self, grid_pos: USVec2) -> USVec2 {
        USVec2 {
            x: grid_pos.x,
            y: self.change_y_smallest(grid_pos.y),
        }
    }

    // INFO: changes between Bottom Smalles Y positioning and Top Smalles Y positioning
    pub fn change_y_smallest(&self, y: usize) -> usize {
        return self.world_size.y - y;
    }
}
// SECTION: GET_TILE_GRID_DATA
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenTileGridData {
    pub block: BlockType,
    pub material_multiplier: OrderedFloat<f32>,
}
impl Default for GenTileGridData {
    fn default() -> Self {
        GenTileGridData {
            block: BlockType::Air(AirBlock::default()),
            material_multiplier: OrderedFloat(0.),
        }
    }
}

fn default_material_hotspots() -> HashMap<MaterialType, [USVec2; 15]> {
    let mut map: HashMap<MaterialType, [USVec2; 15]> = HashMap::new();

    map.insert(
        MaterialType::Dirt(DirtMaterial::default()),
        [USVec2::default(); 15],
    );
    map.insert(
        MaterialType::Stone(StoneMaterial::default()),
        [USVec2::default(); 15],
    );
    map
}
