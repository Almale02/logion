use std::path::{Path, PathBuf};

use crate::block::{block_type::BlockType, blocks::air::AirBlock, lib::Block};

use crate::lib::USVec2::*;

use bevy::reflect::Array;
use bevy::{asset::Asset, prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct LevelData {
    pub grid_unit: u8,
    pub world_size: USVec2,
    pub block_gird: [[Entity; 90]; 30],
    pub gen_grid: Vec<Vec<BlockType>>,
}

impl Default for LevelData {
    fn default() -> Self {
        LevelData {
            grid_unit: 32,
            world_size: USVec2 { x: 90, y: 30 },
            gen_grid: vec![vec![BlockType::Air(AirBlock::default()); 90]; 30],
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

    // changes between Bottom Smalles Y positioning and Top Smalles Y positioning
    pub fn change_y_smallest(&self, y: usize) -> usize {
        return self.world_size.y - y;
    }
}
