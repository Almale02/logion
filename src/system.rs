use std::sync::{Arc, Mutex};

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
    println!("{}", 1./ time.delta_seconds());
    if keys.pressed(KeyCode::A) {
        q_camera.single_mut().translation.x -= 0.5 * time.delta_seconds() *1000.;
        println!("{}", q_camera.single_mut().translation.x);
    }
    if keys.pressed(KeyCode::D) {
        q_camera.single_mut().translation.x += 0.5 * time.delta_seconds() *1000.;
        println!("{}", q_camera.single_mut().translation.x);
    }
}