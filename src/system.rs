use bevy_rapier2d::na::clamp;
use bevy_rapier2d::prelude::*;

use bevy::transform::commands;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::component::*;
use crate::resource::*;

pub fn init_rendering(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

pub fn move_camera(
    keys: Res<Input<KeyCode>>,
    mut q_camera: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>
) {
    if keys.pressed(KeyCode::A) {
        q_camera.single_mut().translation.x -= 0.5 * time.delta_seconds() *1000.;
        println!("{}", q_camera.single_mut().translation.x);
    }
    if keys.pressed(KeyCode::D) {
        q_camera.single_mut().translation.x += 0.5 * time.delta_seconds() *1000.;
        println!("{}", q_camera.single_mut().translation.x);
    }
}

pub fn move_ball(
    mut q_movement: Query<(&mut Velocity, &mut KinematicCharacterController), With<Ball>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    
    for (mut vel, mut controller) in q_movement.iter_mut() {
        if keyboard.pressed(KeyCode::Right) { vel.linvel.x = 1.4 * time.delta_seconds() * 100.}
        if keyboard.pressed(KeyCode::Left) { vel.linvel.x = -1.4 * time.delta_seconds() * 100.}
        if keyboard.just_pressed(KeyCode::Space) { vel.linvel.y += 18. }
        vel.linvel.y -= 1.;
        if vel.linvel.y < -1. {vel.linvel.y = -1.}
        if vel.linvel.x < 0. {vel.linvel += 0.1}
        if vel.linvel.x > 0. {vel.linvel -= 0.1}

        controller.translation = Some(vel.linvel)
    }
}

pub fn spawn_ball(
    mut commands: Commands
) {
    commands.spawn((
        Ball,
        TransformBundle::from(Transform::from_xyz(0., 300., 0.)),
        Collider::ball(15.),
        KinematicCharacterController::default(),
        GravityScale(2.5),
        Velocity::zero()
    ));
}
