#![allow(unused_mut)]




use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};


use image::{Rgba, RgbaImage};

use crate::block::block_type::BlockConvertible;

use crate::block::{block_type::BlockType, blocks::*, lib::*};
use crate::lib::USVec2::USVec2;

use crate::resource::level_data::LevelData;

pub fn generate_world(mut level_data: ResMut<LevelData>) {
    let mut grid = level_data.gen_grid.clone();

    for (y, row) in level_data.gen_grid.iter().enumerate() {
        for (x, _block) in row.iter().enumerate() {
            if y == level_data.change_y_smallest(11) {
                if rand::random::<bool>() {
                    grid[y][x].block = BlockType::Dirt(dirt::DirtBlock::default())
                }
            }
            if y == level_data.change_y_smallest(15) && x == 10 {
                grid[y][x].block = BlockType::Dirt(dirt::DirtBlock::default())
            }

            if y >= level_data.change_y_smallest(10) {
                grid[y][x].block = BlockType::Dirt(dirt::DirtBlock::default())
            }
            if y >= level_data.change_y_smallest(3) {
                grid[y][x].block = BlockType::Stone(stone::StoneBlock::default());
            }
        }
    }

    level_data.gen_grid = grid;
}
pub fn generate_grass(mut level_data: ResMut<LevelData>) {
    let world = level_data.gen_grid.clone();

    for (y, row) in level_data.gen_grid.iter_mut().enumerate() {
        for (x, block) in row.iter_mut().enumerate() {
            match &mut block.block {
                BlockType::Dirt(dirt) => {
                    if x == 0 || x == world[0].len() - 1 {
                        continue;
                    }
                    let left = match world[y][x - 1].block {
                        BlockType::Air(_) => true,
                        _ => false,
                    };
                    let right = match world[y][x + 1].block {
                        BlockType::Air(_) => true,
                        _ => false,
                    };
                    let top = match world[y - 1][x].block {
                        BlockType::Air(_) => true,
                        _ => false,
                    };
                    if y == 15 && x == 10 {
                        continue;
                    }
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
    mut image_assets: ResMut<Assets<Image>>,
) {
    let _world = level_data.gen_grid.clone();

    for (y, row) in level_data.gen_grid.iter_mut().enumerate() {
        for (x, mut block) in row.iter_mut().enumerate() {
            let mat_mulitplyer = block.material_multiplier;
            let block = &mut block.block;

            let base_state = match block.as_block().render_type() {
                BlockRenderType::None() => continue,
                BlockRenderType::BlockState(x) => x.to_owned(),
                BlockRenderType::Generated(_) => unreachable!(),
            };
            let block_ref = block.clone();
            let default_handle = asset_server.load(base_state);

            let image_data = image_assets.get(&default_handle).unwrap();
            let width = image_data.size().x as u8;
            let height = image_data.size().y as u8;

            let mut rgba_image = &mut create_rgba_image(image_data, width, height);

            let material_map = block.as_mut_block().gen_materials(x, y, *mat_mulitplyer);

            // SECTION: GENERATE PIXLES
            let used_matrix: &mut [[bool; 16]; 16] = &mut [[false; 16]; 16];

            for (m_type, count) in material_map.list().iter() {
                let material = m_type.as_mateiral().clone();

                if material.base_block() == block_ref.as_block().block_id() {
                    continue;
                }
                material.write_pixles(rgba_image, used_matrix, count.clone());
            }

            // SECTION: write back the generated image
            let image_handle: Handle<Image> = image_assets.add(rgba_to_image(rgba_image.clone()));

            block
                .as_mut_block()
                .set_rendertype(BlockRenderType::Generated(image_handle));
        }
    }
}

fn create_rgba_image(data: &Image, width: u8, height: u8) -> RgbaImage {
    let mut rgba_image = RgbaImage::new(width.into(), height.into());

    for y in 0..width {
        for x in 0..height {
            let index = ((y as u32 * width as u32 + x as u32) * 4) as usize;

            let rgba = &data.data[index..index + 4];

            /*  INFO: randomized Rgba channels look very good!
                let rgb = Rgba([rand::random::<u8>(), rgba[1], rgba[2], rgba[3]]);
                let rgb = Rgba([rgba[0], rand::random::<u8>(), rgba[2], rgba[3]]);
                let rgb = Rgba([rgba[0], rgba[1], rand::random::<u8>(), rgba[3]]);
                let rgb = Rgba([rgba[0], rgba[1], rgba[2], rgba[3]]);
            */
            let rgb = Rgba([rgba[0], rgba[1], rgba[2], rgba[3]]);

            rgba_image.put_pixel(x.into(), y.into(), rgb)
        }
    }
    rgba_image
}
fn rgba_to_image(image: RgbaImage) -> Image {
    let new_image = Image::new(
        Extent3d {
            width: 16,
            height: 16,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        image.clone().into_raw(),
        TextureFormat::Rgba8UnormSrgb,
    );

    new_image
}
pub fn fill_terrain_list(mut level_data: ResMut<LevelData>) {
    let mut t_list = level_data.terrain_list.clone();
    for (y, row) in level_data.gen_grid.iter_mut().enumerate() {
        for (x, mut block) in row.iter_mut().enumerate() {
            if let BlockType::Air(_) = block.block {
                continue;
            }

            t_list.push(USVec2 { x, y })
        }
    }

    level_data.terrain_list = t_list;
    dbg!(&level_data.terrain_list);
}
