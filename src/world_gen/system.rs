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

pub fn gen_world_from_level_data(
    mut commands: Commands,
    mut level_data: ResMut<LevelData>, 
    asset_server: Res<AssetServer>,
    block_texture: Res<BlockTexture>,
) {
    let mut cmd = Arc::new(Mutex::new(commands));    
    
    level_data.loop_block_grid(|x, y, block, _| {
        let mut cmd = cmd.lock().unwrap();
        let global_pos = level_data.grid_to_global_space_unit(
            USVec2 {x, y}
        );
        if *block == 0 {return}

        let texture = block_texture.texture_map.get(block);
        cmd.spawn((
            SpriteBundle {
                texture: asset_server.load(texture.unwrap()),
                transform: Transform::from_translation(
                    Vec3::new(
                        global_pos.x as f32 *level_data.grid_unit as f32,
                        global_pos.y as f32*level_data.grid_unit as f32,
                        0.
                    )
                ).with_scale(
                    Vec3 {x: 2., y: 2., z: 0.}
                ),
                ..default()
            },
            RigidBody::Fixed,
            Velocity {
                linvel: Vec2::new(1.0, 5.0),
                angvel: 0.4
            },
            Collider::cuboid(8., 8.),
        ));
    })
}
