use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::block::blocks::air::AirBlock;
use crate::block::{block_type::*, blocks::*, lib::*};
use crate::lib::USVec2::USVec2;
use crate::resource::{block_texture::BlockTexture, level_data::LevelData};

pub fn load_world(
    mut commands: Commands,
    level_data: ResMut<LevelData>,
    asset_server: Res<AssetServer>,
    block_texture: Res<BlockTexture>,
) {
    let cmd = Mutex::new(commands);

    level_data.loop_block_grid(|x, y, block, _| {
        let mut cmd = cmd.lock().unwrap();
        let global_pos = level_data.grid_to_global_space_unit(USVec2 { x, y });
        match block {
            BlockType::Air(_) => return,
            _ => (),
        }

        let states = block.as_block().states();
        let texture = states.get(&block.as_block().state()).unwrap();

        cmd.spawn((
            SpriteBundle {
                texture: asset_server.load(&texture.state_image),
                transform: Transform::from_translation(Vec3::new(
                    global_pos.x as f32 * level_data.grid_unit as f32,
                    global_pos.y as f32 * level_data.grid_unit as f32,
                    0.,
                ))
                .with_scale(Vec3 {
                    x: 2.,
                    y: 2.,
                    z: 0.,
                }),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(8., 8.),
        ));
    })
}
