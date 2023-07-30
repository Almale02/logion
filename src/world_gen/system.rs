#![allow(unused_mut)]

use std::sync::{Arc, Mutex};

use bevy_rapier2d::prelude::*;

use bevy::{prelude::*};

use crate::resource::{
    block_texture::BlockTexture,
    level_data::LevelData
};
use crate::lib::USVec2::USVec2;

pub fn generate_world(
    mut level_data: ResMut<LevelData>, 
) {
    let world_size: usize = level_data.world_size.y.try_into().unwrap();
    
    let mut grid = level_data.block_gird.clone();

    for (y, row) in level_data.block_gird.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {

            if y == LevelData::change_y_smallest(world_size, 11) {
                if rand::random::<bool>() {
                    grid[y][x] = 3;
                }
            }

            if y == LevelData::change_y_smallest(world_size, 10) {
                grid[y][x] = 3;
                println!("{}", grid[y +1][x]);
                if grid[y -1][x] == 3 {
                    grid[y][x] = 1;
                }
            }

            if y > LevelData::change_y_smallest(world_size, 10) {
                grid[y][x] = 1;
            }

            if y >= LevelData::change_y_smallest(world_size, 3) {
                grid[y][x] = 2;
            }
        }
    }

    level_data.block_gird = grid;
}
