use bevy::prelude::*;

use crate::component::*;

pub fn init(
    mut commands: Commands,
    time: Res<Time>
) {
    for _ in 1..=5 {
        commands.spawn(Pos {pos: Vec2{x: 1.0, y: 5.0}});
    }
}
