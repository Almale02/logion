use bevy::prelude::*;

pub fn cursor_to_global_pos(
    window: &Query<&Window>,
    camera: &Query<(&Camera, &GlobalTransform), With<Camera>>,
) -> Option<Transform> {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    if let Some(pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        return Some(Transform::from_xyz(pos.x, pos.y, 0.));
    }
    None
}
