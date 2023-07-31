use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::lib::USVec2::USVec2;
use crate::resource::{
    level_data::LevelData,
    block_texture::BlockTexture
};


pub fn load_world(
    mut commands: Commands,
    level_data: ResMut<LevelData>, 
    asset_server: Res<AssetServer>,
    block_texture: Res<BlockTexture>,
) {
    let cmd = Arc::new(Mutex::new(commands));
    
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
            Collider::cuboid(8., 8.),
        ));
    })}
