#![allow(unused_mut)]

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, Texture, TextureDimension, TextureFormat};
use bevy::render::texture::{CompressedImageFormats, ImageType};
use image::buffer::ConvertBuffer;
use image::{ImageOutputFormat, Rgb, RgbImage, Rgba, RgbaImage};

use crate::block::block_type::BlockConvertible;
use crate::block::blocks::dirt::DirtBlock;
use crate::block::{block_type::BlockType, blocks::*, lib::*};
use crate::resource::level_data::LevelData;

pub fn generate_world(mut level_data: ResMut<LevelData>) {
    let mut grid = level_data.gen_grid.clone();

    for (y, row) in level_data.gen_grid.iter().enumerate() {
        for (x, _block) in row.iter().enumerate() {
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

    level_data.gen_grid = grid;
}
pub fn generate_grass(mut level_data: ResMut<LevelData>) {
    let world = level_data.gen_grid.clone();

    for (y, row) in level_data.gen_grid.iter_mut().enumerate() {
        for (x, block) in row.iter_mut().enumerate() {
            match block {
                BlockType::Dirt(dirt) => {
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
    mut image_assets: ResMut<Assets<Image>>,
) {
    let _world = level_data.gen_grid.clone();

    for (_y, row) in level_data.gen_grid.iter_mut().enumerate() {
        for (_x, mut block) in row.iter_mut().enumerate() {
            let base_state = match block.as_block().render_type() {
                BlockRenderType::None() => continue,
                BlockRenderType::BlockState(x) => x.to_owned(),
                BlockRenderType::Generated(_) => unreachable!(),
            };
            dbg!(&base_state);
            let default_handle = asset_server.load(base_state);

            let image_data = image_assets.get(&default_handle).unwrap();
            let width = image_data.size().x as u8;
            let height = image_data.size().y as u8;

            let mut rgba_image = create_rgba_image(image_data, width, height);

            let new_image = Image::new(
                Extent3d {
                    width: 16,
                    height: 16,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                rgba_image.clone().into_raw(),
                TextureFormat::Rgba8UnormSrgb,
            );

            let image_handle = image_assets.add(new_image);
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
            */
            let rgb = Rgba([rgba[0], rgba[1], rgba[2], rgba[3]]);

            rgba_image.put_pixel(x.into(), y.into(), rgb)
        }
    }
    rgba_image
}
