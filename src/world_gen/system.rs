#![allow(unused_mut)]

use std::sync::{Arc, Mutex};

use bevy_rapier2d::prelude::*;

use bevy::prelude::*;

use crate::block::{block_type::BlockType, blocks::*, lib::*};
use crate::lib::USVec2::USVec2;
use crate::resource::{block_texture::BlockTexture, level_data::LevelData};

pub fn generate_world(mut level_data: ResMut<LevelData>) {
    let world_size: usize = level_data.world_size.y.try_into().unwrap();

    let mut grid = level_data.block_gird.clone();

    for (y, row) in level_data.block_gird.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            if y == level_data.change_y_smallest(11) {
                if rand::random::<bool>() {
                    grid[y][x] = BlockType::Dirt(dirt::DirtBlock::default())
                }
            }

            if y >= level_data.change_y_smallest(10) {
                grid[y][x] = BlockType::Dirt(dirt::DirtBlock::default())
            }
            if y >= level_data.change_y_smallest(3) {
                grid[y][x] = BlockType::Stone(stone::StoneBlock::default());
            }
        }
    }

    level_data.block_gird = grid;
}
pub fn generate_grass(mut level_data: ResMut<LevelData>) {
    let world = level_data.block_gird.clone();

    for (y, row) in level_data.block_gird.iter_mut().enumerate() {
        for (x, block) in row.iter_mut().enumerate() {
            match block {
                BlockType::Dirt(dirt) => {
                    println!("x: {}, y: {}", x, y);
                    if x == 0 || x == world[0].len() - 1 {
                        continue;
                    }
                    let left = match world[y][x - 1] {
                        BlockType::Air(_) => true,
                        _ => false,
                    };
                    let right = match world[y][x + 1] {
                        BlockType::Air(_) => true,
                        _ => false,
                    };
                    let top = match world[y - 1][x] {
                        BlockType::Air(_) => true,
                        _ => false,
                    };
                    dirt.make_grassy(left, right, top)
                }
                _ => (),
            }
        }
    }
}
