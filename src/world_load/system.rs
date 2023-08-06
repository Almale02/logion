use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::block::{block_type::*, blocks::*, lib::*};
use crate::lib::Identifier::Identifier;
use crate::lib::USVec2::USVec2;
use crate::resource::level_data::LevelData;

pub fn load_world(
    mut commands: Commands,
    mut level_data: ResMut<LevelData>,
    asset_server: Res<AssetServer>,
) {
    let mut block_grid = level_data.block_gird.clone();

    for (y, row) in level_data.gen_grid.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            let global_pos = level_data.grid_to_global_space_unit(USVec2 { x, y });

            let handle: Handle<Image> = match block.as_block().render_type() {
                BlockRenderType::None() => continue,
                BlockRenderType::BlockState(x) => asset_server.load(
                    block
                        .as_block()
                        .states()
                        .get(&Identifier { id: x.to_string() })
                        .unwrap()
                        .state_image
                        .clone(),
                ),
                BlockRenderType::Generated(x) => x.clone(),
            };

            let mut entity = commands.spawn((
                SpriteBundle {
                    texture: handle,
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

            match block {
                BlockType::Dirt(x) => entity.insert(x.clone()),
                BlockType::Stone(x) => entity.insert(x.clone()),
                _ => unreachable!(),
            };

            block_grid[y][x] = entity.id();
        }
    }
    level_data.block_gird = block_grid;
}
