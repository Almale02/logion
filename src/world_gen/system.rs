#![allow(unused_mut)]

use std::sync::{Arc, Mutex};

use bevy_rapier2d::prelude::*;

use bevy::{prelude::*};

use crate::resource::{
    block_texture::BlockTexture,
    level_data::LevelData
};
use crate::block::{
    lib::*,
    block_type::BlockType,
    blocks::*
};
use crate::lib::USVec2::USVec2;

pub fn generate_world(
    mut level_data: ResMut<LevelData>, 
) {
    let world_size: usize = level_data.world_size.y.try_into().unwrap();
    
    let mut grid = level_data.block_gird.clone();

    for (y, row) in level_data.block_gird.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {                                    
            if y >= LevelData::change_y_smallest(world_size, 10) {
                grid[y][x] = BlockType::Dirt(dirt::DirtBlock::default()) 
            }

            if y >= LevelData::change_y_smallest(world_size, 3) {
                grid[y][x] = BlockType::Stone(stone::StoneBlock::default());
            }
        }
    }

    level_data.block_gird = grid;
}
pub fn generate_grass(
    mut level_data: ResMut<LevelData>,
) {
    let world_size: usize = level_data.world_size.y.try_into().unwrap();
    
    let mut grid = level_data.block_gird.clone();
    

    for (y, row) in level_data.block_gird.iter_mut().enumerate() {
        for (x, block) in row.iter_mut().enumerate() {
            if y == LevelData::change_y_smallest(world_size, 10) {
                match block {
                    BlockType::Dirt(x) => {
                        x.make_grassy()
                    },
                    _ => ()
                }
            }
        }
    }
}
