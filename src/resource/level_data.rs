use std::path::{Path, PathBuf};

use crate::block::{
    lib::Block,
    block_type::BlockType,
    blocks::air::AirBlock,
};


use crate::lib::USVec2::*;

use bevy::{asset::Asset, prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct LevelData {
    pub grid_unit: u8,
    pub world_size: USVec2,
    pub block_gird: Vec<Vec<BlockType>>,
}

impl Default for LevelData {
    fn default() -> Self {
        LevelData {
            grid_unit: 32,
            world_size: USVec2 { x: 90, y: 30 },
            block_gird: vec![vec![BlockType::Air(AirBlock {}); 90]; 30],
        }
    }
}

impl LevelData {
    pub fn grid_to_global_space_unit(&self, grid_pos: USVec2) -> USVec2 {
        USVec2 {
            x: grid_pos.x,
            y: LevelData::change_y_smallest(self.world_size.y, grid_pos.y),
        }
    }

    // changes between Bottom Smalles Y positioning and Top Smalles Y positioning
    pub fn change_y_smallest(world_size: usize, y: usize) -> usize {
        return world_size - y;
    }

    pub fn loop_block_grid<T>(&self, func: T)
    where
        T: Fn(usize, usize, &BlockType, &Vec<Vec<BlockType>>),
    {
        for (y, row) in self.block_gird.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                func(x, y, block, &self.block_gird)
            }
        }
    }
}
