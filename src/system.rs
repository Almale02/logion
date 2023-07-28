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
    mut q_controller: Query<&mut KinematicCharacterController, With<Ball>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut vel = Vec2::ZERO;

    if keyboard.pressed(KeyCode::Right) { vel.x = 1.4 * time.delta_seconds() * 100.}
    if keyboard.pressed(KeyCode::Left) { vel.x = -1.4 * time.delta_seconds() * 100.}
    if keyboard.just_pressed(KeyCode::Space) { vel.y = 70.5 }
    
    vel.y -= 2.;
    for mut controller in q_controller.iter_mut() {
        controller.translation = Some(vel)
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
    ));
}
