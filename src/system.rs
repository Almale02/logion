use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::component::*;
use crate::resource::*;

pub fn move_by_velocity(
    mut transforms: Query<(&mut Transform, &Vel2D)>,
    time: Res<Time>
) {
    println!("{}", 1./ time.delta_seconds());
    for (mut transform, vel) in &mut transforms {        
        transform.translation.x += vel.v_x * time.delta_seconds() * 100.;
        transform.translation.y += vel.v_y * time.delta_seconds() * 100.;
    }
}

pub fn change_dir_at_edge(
    mut q: Query<(&mut Vel2D, &Transform)>,
    q_window: Query<&Window>
) {
    let window = q_window.single();
    let width = window.resolution.width() - 7.5;
    let height = window.resolution.height() - 7.5;

    for (mut vel, transform) in &mut q {
        if transform.translation.x >= width /2. || transform.translation.x <= -width /2. {vel.v_x *= -1.}
        if transform.translation.y >= height /2. || transform.translation.y <= -height /2. {vel.v_y *= -1.}
    }
}

pub fn intit_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());

    for i in 1 ..= 1500 {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(15.).into()).into(),            
                transform: Transform::from_xyz(-154., 0., 0.),
                material: materials.add(ColorMaterial::from(Color::RED)),       
                ..default()
            },
            //Transform::from_xyz(-154., 0., 0.),
            Xp::default(),
            Vel2D {v_x: 11. /2. + rand::random::<f32>() * 15., v_y: 5. / 2. - rand::random::<f32>() * 15.}
            //Vel2D {v_x: 11. /2., v_y: 5. /2.}
        ));
    }    
}

pub fn init_word_grid(
    //mut commands: Commands,
    mut level_data: ResMut<LevelData>
) {
    let block_grid = &mut level_data.block_gird;
    
    for (y, row) in block_grid.iter_mut().enumerate() {
        for (x, data) in row.iter_mut().enumerate() {
            *data = 5 as u8;
        }
    }
}