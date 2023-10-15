

use crate::material::lib::MaterialType;
use crate::material::materials::m_dirt::DirtMaterial;
use crate::material::materials::m_stone::StoneMaterial;
use crate::world_gen::lib::WorldGenData;
use crate::world_gen::ore_gen::lib::OrePatchData;

use std::collections::HashMap;



use crate::block::{block_type::BlockType, blocks::air::AirBlock};

use crate::lib::math::usvec2::USVec2;

use bevy::prelude::*;

#[derive(Resource)]
pub struct LevelData {
    pub grid_unit: u8,
    pub world_size: USVec2,
    pub block_gird: [[Entity; 200 * 3]; 90],
    pub gen_grid: Vec<Vec<GenTileGridData>>,
    pub terrain_map: HashMap<USVec2, bool>,
    pub terrain_height: [usize; 200 * 3],
    pub generation_data: WorldGenData,
}
impl Default for LevelData {
    fn default() -> Self {
        const x: usize = 200 * 3;
        const y: usize = 90;
        LevelData {
            grid_unit: 16 * 2,
            world_size: USVec2 { x, y },
            block_gird: [[Entity::PLACEHOLDER; x]; y],
            gen_grid: vec![vec![GenTileGridData::default(); x]; y],
            terrain_map: HashMap::default(),
            terrain_height: [0; x],
            generation_data: WorldGenData::default(),
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
    pub ore_patch: OrePatchData,
}
impl Default for GenTileGridData {
    fn default() -> Self {
        GenTileGridData {
            block: BlockType::Air(AirBlock::default()),
            ore_patch: OrePatchData::default(),
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
