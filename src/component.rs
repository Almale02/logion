use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Xp {
    pub xp_level: u8,
}
#[derive(Component, Default)]
pub struct Vel2D {
    pub v_x: f32,
    pub v_y: f32,
}
#[derive(Component)]
pub struct Ball;
