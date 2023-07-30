use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_rapier2d::prelude::*;

use crate::component::*;

pub fn move_ball(
    mut q_movement: Query<(&mut Velocity, &mut KinematicCharacterController, &KinematicCharacterControllerOutput), With<Ball>>,
    mut q_transform: Query<&mut Transform, With<Ball>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let move_speed = 3.5;
    let fall_speed = 2.5;

    for (mut y_vel, mut controller, controller_info) in q_movement.iter_mut() {
        let mut x_vel = 0.;
        if keyboard.pressed(KeyCode::Right) {x_vel = move_speed * time.delta_seconds() * 100.}
        if keyboard.pressed(KeyCode::Left) {x_vel = -move_speed * time.delta_seconds() * 100.}

        if controller_info.grounded {
            if keyboard.just_pressed(KeyCode::Space) { y_vel.linvel.y += 18. * (fall_speed / 2.)}
        }

        y_vel.linvel.y -= fall_speed;
        if y_vel.linvel.y < -fall_speed {y_vel.linvel.y = -fall_speed}

        controller.translation = Some(Vect {x: x_vel, y: y_vel.linvel.y});
        
        for mut transform in q_transform.iter_mut() {
            transform.rotate_z(x_vel);
        }
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>
    
) {
    commands.spawn((
        Ball,
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_scale(
                Vec3 {x: 2., y: 2., z: 0.}
            ).with_translation(
                Vec3 {x: 0., y: 360., z: 0.}
            ),
            ..default()
        },

        Collider::ball(16. /2. * 2./3.),
        KinematicCharacterController::default(),
        KinematicCharacterControllerOutput::default(),
        Velocity::zero()
    ));
}
