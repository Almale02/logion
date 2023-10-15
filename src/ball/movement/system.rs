use bevy::prelude::*;

use crate::ball::component::Ball;

pub fn move_camera(
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera2d>>,
        Query<&Transform, With<Ball>>,
    )>,
    window: Query<&Window>,
    _time: Res<Time>,
) {
    if set.p1().is_empty() {
        return;
    }
    let player_x = set.p1().single().translation.x as i32;
    let camera_x = set.p0().single().translation.x as i32;
    let player_y = set.p1().single().translation.y as i32;
    let _camera_y = set.p0().single().translation.y as i32;

    let window = window.single();
    let width = window.width();

    let diff: i32 = (width / 2. * 0.5) as i32;
    if player_x.abs_diff(camera_x) > diff as u32 {
        if player_x - camera_x > 0 {
            set.p0().single_mut().translation.x += (player_x - camera_x - diff) as f32;
        }

        if player_x - camera_x < 0 {
            set.p0().single_mut().translation.x += (player_x - camera_x + diff) as f32;
        }
    }

    set.p0().single_mut().translation.y = player_y as f32;
}
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, States)]
pub enum CameraFollowState {
    None,
    #[default]
    PlayerMain,
}
