use std::sync::{Arc, Mutex};

use bevy::transform::commands;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::component::*;
use crate::resource::*;

pub fn generate_world(
    mut level_data: ResMut<LevelData>, 
) {
    level_data.loop_block_grid_mut(|x, y, block|  {
        if y > 24 {
            *block = 1
        }
    });
}

pub fn gen_world_from_level_data(
    mut commands: Commands,
    mut level_data: ResMut<LevelData>, 
) {
    let mut cmd = Arc::new(Mutex::new(commands));    
    
        level_data.loop_block_grid(|x, y, block| {
        let mut cmd = cmd.lock().unwrap();
        let global_pos = level_data.grid_to_global_space_unit(
            UVec2 {x: x as u32, y: y as u32}
        );
        let mut color = Color::rgb(0., 0., 0.);

        if *block == 0 {
            color = Color::rgb(0., 140. /255., 1.);
        }
        if *block == 1 {
            color = Color::rgb(0., 140. /255., 0.);
        }
    
        cmd.spawn(SpriteBundle {
            sprite: Sprite {
                color: color,
                custom_size: Some(Vec2::new(16., 16.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                global_pos.x *level_data.grid_unit as f32,
                global_pos.y *level_data.grid_unit as f32 - 500.,
                0.)),
            ..default()
        });
    })
}
