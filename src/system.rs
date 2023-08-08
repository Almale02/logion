use bevy::prelude::*;

use crate::component::*;

pub fn init_rendering(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3 {
            x: (0.),
            y: (360.),
            z: (100.),
        }),
        ..default()
    });
}

pub fn move_camera(
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera2d>>,
        Query<&Transform, With<Ball>>,
    )>,
) {
    if set.p1().is_empty() {
        return;
    }
    set.p0().single_mut().translation.x = set.p1().single().translation.x;
    set.p0().single_mut().translation.y = set.p1().single().translation.y;
}
