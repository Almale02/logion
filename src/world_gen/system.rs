#![allow(unused_mut)]

use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::block::block_type::BlockConvertible;
use crate::block::{block_type::BlockType, blocks::*, lib::*};
use crate::resource::level_data::LevelData;

pub fn generate_world(mut level_data: ResMut<LevelData>) {
    let mut grid = level_data.gen_grid.clone();

    for (y, row) in level_data.gen_grid.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            if y == level_data.change_y_smallest(11) {
                if rand::random::<bool>() {
                    grid[y][x] = BlockType::Dirt(dirt::DirtBlock::default())
                }
            }

            if y >= level_data.change_y_smallest(10) {
                println!("{}", true);
                grid[y][x] = BlockType::Dirt(dirt::DirtBlock::default())
            }
            if y >= level_data.change_y_smallest(3) {
                println!("{}", true);
                grid[y][x] = BlockType::Stone(stone::StoneBlock::default());
            }
        }
    }

    level_data.gen_grid = grid;
}
pub fn generate_grass(mut level_data: ResMut<LevelData>) {
    let world = level_data.gen_grid.clone();

    for (y, row) in level_data.gen_grid.iter_mut().enumerate() {
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
pub fn materialize(
    asset_server: Res<AssetServer>,
    mut level_data: ResMut<LevelData>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut image_assets: ResMut<Assets<Image>>,
) {
    let world = level_data.gen_grid.clone();

    for (y, row) in level_data.gen_grid.iter_mut().enumerate() {
        for (x, block) in row.iter_mut().enumerate() {
            let base_state = match block.as_block().render_type() {
                BlockRenderType::None() => continue,
                BlockRenderType::BlockState(x) => x.to_owned(),
                BlockRenderType::Generated(_) => unreachable!(),
            };
            let image_handle: Handle<Image> = asset_server.load("path/to/your/image.png");

            let image_data = image_assets.get(&image_handle).unwrap();

            let a = &image_data.data; // INFO: me have default image data!
        }
    }
}

fn png_to_rgb_image(handle: Handle<Image>) {}
